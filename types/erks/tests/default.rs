use erks::{
  Code, Context as ErksContext, ErksError, ErksSeverity, IoError, Metadata,
  StructuredError
};
use std::collections::HashMap;

#[test]
fn test_error_code_display() {
  assert_eq!(format!("{}", Code::IoError), "IO_ERROR");
  assert_eq!(format!("{}", Code::ValidationError), "VALIDATION_ERROR");
  assert_eq!(format!("{}", Code::Unknown), "UNKNOWN");
}

#[test]
fn test_severity_display() {
  assert_eq!(format!("{}", ErksSeverity::Info), "INFO");
  assert_eq!(format!("{}", ErksSeverity::Warning), "WARN");
  assert_eq!(format!("{}", ErksSeverity::Error), "ERROR");
  assert_eq!(format!("{}", ErksSeverity::Critical), "CRITICAL");
}

#[test]
fn test_metadata_builder() {
  let meta = Metadata::new(Code::NetworkError)
    .with_component("networking")
    .with_operation("fetch_data")
    .with_context("url", "http://example.com");

  assert_eq!(meta.error_code, Code::NetworkError);
  assert_eq!(meta.component, Some("networking".to_string()));
  assert_eq!(meta.operation, Some("fetch_data".to_string()));
  assert_eq!(
    meta.context.get("url"),
    Some(&"http://example.com".to_string())
  );
}

#[test]
fn test_error_category() {
  let io_err = ErksError::from(IoError::new("file access failed"));
  assert_eq!(io_err.category(), "system");

  let custom_err = ErksError::custom("a custom error");
  assert_eq!(custom_err.category(), "custom");

  #[cfg(feature = "http")]
  {
    let http_err =
      ErksError::from(erks::HttpError::custom("http request failed"));
    assert_eq!(http_err.category(), "http");
  }

  let anyhow_err = ErksError::from(anyhow::anyhow!("generic anyhow error"));
  assert_eq!(anyhow_err.category(), "generic");
}

#[test]
fn test_to_structured() {
  let error = ErksError::from(IoError::file_system_with_path(
    "Could not open file",
    "/tmp/test.txt"
  ));
  let structured: StructuredError = error.to_structured();

  assert_eq!(structured.message, "File system error: Could not open file");
  assert_eq!(structured.category, "system");
  assert_eq!(structured.code, Code::IoError);
  assert_eq!(structured.severity, ErksSeverity::Error);
  assert!(structured.recoverable);

  let meta = structured.metadata.unwrap();
  assert_eq!(meta.component.unwrap(), "io");
  assert_eq!(meta.context.get("path").unwrap(), "/tmp/test.txt");
}

#[test]
fn test_error_convenience_constructors() {
  let custom_err = ErksError::custom("my error");
  assert!(matches!(custom_err, ErksError::Custom(_)));
  assert_eq!(custom_err.to_string(), "Application error: my error");

  let io_err = ErksError::io("disk full");
  assert!(matches!(io_err, ErksError::System(_)));
  assert_eq!(io_err.to_string(), "I/O error: disk full");

  let validation_err = ErksError::validation("bad input");
  assert!(matches!(
    validation_err,
    ErksError::Custom(erks::CustomError::Validation { .. })
  ));
  assert_eq!(validation_err.to_string(), "Validation error: bad input");

  let state_err = ErksError::invalid_state("wrong state");
  assert!(matches!(
    state_err,
    ErksError::Custom(erks::CustomError::InvalidState { .. })
  ));
  assert_eq!(state_err.to_string(), "Invalid state: wrong state");
}
