use crate::*;

/// Convert any error implementing std::error::Error to our Error type
pub fn wrap_error<E>(err: E) -> Error
where
  E: std::error::Error + Send + Sync + 'static
{
  Error::Anyhow(anyhow::Error::new(err))
}

/// Create a chain of errors with context
pub fn error_chain<T: Into<String>>(msg: T) -> anyhow::Error {
  anyhow::Error::msg(msg.into())
}

/// Retry function with exponential backoff for recoverable errors
pub async fn retry_with_backoff<F, T, Fut>(
  mut operation: F,
  max_retries: usize,
  base_delay_ms: u64
) -> Result<T>
where
  F: FnMut() -> Fut,
  Fut: std::future::Future<Output = Result<T>>
{
  let mut delay = base_delay_ms;
  let mut last_error = None;

  for attempt in 0..=max_retries {
    match operation().await {
      Ok(result) => return Ok(result),
      Err(error) => {
        if !error.is_recoverable() || attempt == max_retries {
          return Err(error);
        }

        last_error = Some(error);

        // Sleep for exponential backoff
        tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
        delay *= 2; // Exponential backoff
      }
    }
  }

  Err(last_error.unwrap_or_else(|| Error::custom("Retry exhausted")))
}

/// Log error with structured data
pub fn log_error(error: &Error) {
  let structured = error.to_structured();

  debug!(
    "[{}][{}] {}: {} (recoverable: {})",
    structured.category.to_uppercase(),
    structured.severity,
    structured.code,
    structured.message,
    structured.recoverable
  );

  if let Some(metadata) = &structured.metadata {
    if let Some(component) = &metadata.component {
      debug!("  component: {component}");
    }
    if let Some(operation) = &metadata.operation {
      debug!("  operation: {operation}");
    }
    if !metadata.context.is_empty() {
      debug!("  context: {:?}", metadata.context);
    }
  }
}
