#[cfg(feature = "glob")]
use erks::{Code, Context, Severity, glob::Error};

#[test]
fn test_glob_pattern_error() {
  let pattern_err = glob::Pattern::new("[").expect_err("Should be a pattern error");
  let error = Error::from(pattern_err);

  assert!(error.to_string().contains("Invalid glob pattern"));
  assert_eq!(error.error_code(), Code::ParseError);
  assert_eq!(error.severity(), Severity::Warning);
  assert!(error.is_recoverable());
  assert!(error.metadata().is_none());
}

#[test]
fn test_glob_iteration_error() {
  let error = Error::iteration("Failed to read directory entry");

  assert_eq!(
    error.to_string(),
    "Glob iteration error: Failed to read directory entry"
  );
  assert_eq!(error.error_code(), Code::IoError);
  assert_eq!(error.severity(), Severity::Warning);
  assert!(error.is_recoverable());

  let meta = error.metadata().unwrap();
  assert_eq!(meta.component.unwrap(), "glob");
  assert_eq!(meta.context.get("message").unwrap(), "Failed to read directory entry");
}
