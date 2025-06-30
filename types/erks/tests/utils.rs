// We are separating the imports into distinct groups (modules, macros, types)
// to improve readability and avoid potential parsing issues with some tooling.
use erks::utils::{log_error, retry_with_backoff, wrap_error};
use erks::{
  Code, CustomError, ErksError, ErksResult, IoError, bail, ensure, error
};
use std::sync::{
  Arc,
  atomic::{AtomicUsize, Ordering}
};

#[test]
fn test_wrap_error() {
  let original_err =
    std::io::Error::new(std::io::ErrorKind::NotFound, "original");
  let wrapped_err = wrap_error(original_err);
  assert!(matches!(wrapped_err, ErksError::Anyhow(_)));
  assert!(wrapped_err.to_string().contains("original"));
}

#[tokio::test]
#[cfg(feature = "retry")]
async fn test_retry_with_backoff_success() {
  let counter = Arc::new(AtomicUsize::new(0));
  let counter_clone = counter.clone();

  let operation = move || {
    let counter_ref = counter_clone.clone();
    async move {
      counter_ref.fetch_add(1, Ordering::SeqCst);
      let result: ErksResult<&'static str> = Ok("success");
      result
    }
  };

  let result = retry_with_backoff(operation, 3, 10).await;
  assert_eq!(result.unwrap(), "success");
  assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[tokio::test]
#[cfg(feature = "retry")]
async fn test_retry_with_backoff_recoverable_failure() {
  let counter = Arc::new(AtomicUsize::new(0));
  let counter_clone = counter.clone();

  let operation = move || {
    let counter_ref = counter_clone.clone();
    async move {
      let count = counter_ref.fetch_add(1, Ordering::SeqCst);
      let result: ErksResult<&'static str> = if count < 2 {
        // A recoverable IO error
        Err(ErksError::from(IoError::network("connection reset")))
      } else {
        Ok("success after retries")
      };
      result
    }
  };

  let result = retry_with_backoff(operation, 3, 10).await;
  assert_eq!(result.unwrap(), "success after retries");
  assert_eq!(counter.load(Ordering::SeqCst), 3);
}

#[tokio::test]
#[cfg(feature = "retry")]
async fn test_retry_with_backoff_unrecoverable_failure() {
  let counter = Arc::new(AtomicUsize::new(0));
  let counter_clone = counter.clone();

  let operation = move || {
    let counter_ref = counter_clone.clone();
    async move {
      counter_ref.fetch_add(1, Ordering::SeqCst);
      // An unrecoverable custom error
      let result: ErksResult<()> =
        Err(ErksError::from(CustomError::invalid_state("unrecoverable")));
      result
    }
  };

  let result = retry_with_backoff(operation, 3, 10).await;
  assert!(result.is_err());
  let err = result.unwrap_err();
  assert_eq!(err.error_code(), Code::InvalidState);
  assert_eq!(counter.load(Ordering::SeqCst), 1); // Should not retry
}

#[tokio::test]
#[cfg(feature = "retry")]
async fn test_retry_exhausted() {
  let counter = Arc::new(AtomicUsize::new(0));
  let counter_clone = counter.clone();

  let operation = move || {
    let counter_ref = counter_clone.clone();
    async move {
      counter_ref.fetch_add(1, Ordering::SeqCst);
      Err(ErksError::from(IoError::network("always fails"))) as ErksResult<()>
    }
  };

  let result: ErksResult<()> = retry_with_backoff(operation, 2, 10).await;
  assert!(result.is_err());
  assert_eq!(counter.load(Ordering::SeqCst), 3); // 1 initial + 2 retries
}

#[test]
fn test_log_error() {
  // This test just ensures the function runs without panicking.
  // Verifying output would require capturing stderr.
  let error = ErksError::custom("test log");
  log_error(&error);
}

// --- Macro Tests ---

fn returns_error() -> ErksResult<()> {
  Err(error!("This is a formatted error: {}", 123))
}

#[test]
fn test_error_macro() {
  let err = returns_error().unwrap_err();
  assert!(matches!(err, ErksError::Custom(_)));
  assert_eq!(
    err.to_string(),
    "Application error: This is a formatted error: 123"
  );
}

fn bails_out() -> ErksResult<()> {
  bail!("Bailing out with a message");
}

#[test]
fn test_bail_macro() {
  let err = bails_out().unwrap_err();
  assert!(matches!(err, ErksError::Custom(_)));
  assert_eq!(
    err.to_string(),
    "Application error: Bailing out with a message"
  );
}

fn ensure_condition(value: i32) -> ErksResult<()> {
  ensure!(value > 10, "Value must be greater than 10, was {}", value);
  Ok(())
}

#[test]
fn test_ensure_macro() {
  assert!(ensure_condition(11).is_ok());

  let err = ensure_condition(5).unwrap_err();
  assert!(matches!(err, ErksError::Custom(_)));
  assert_eq!(
    err.to_string(),
    "Application error: Value must be greater than 10, was 5"
  );
}
