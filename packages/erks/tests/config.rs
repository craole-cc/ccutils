#[cfg(feature = "config")]
use erks::{Code, Context, Severity, config::Error};
use logline::info;

#[test]
fn test_file_not_found_error() {
  let error = Error::file_not_found("config.toml");
  assert_eq!(
    error.to_string(),
    "Configuration file not found: config.toml"
  );
  assert_eq!(error.error_code(), Code::ConfigNotFound);
  assert_eq!(error.severity(), Severity::Error);
  assert!(error.is_recoverable());

  if let Some(meta) = error.metadata() {
    info!("Metadata: {:#?}", meta);
    assert_eq!(meta.component.unwrap(), "config");
    assert_eq!(meta.context.get("path").unwrap(), "config.toml");
  } else {
    panic!("Metadata should exist for FileNotFound");
  }
}

#[test]
fn test_invalid_format_error() {
  let error = Error::invalid_format("config.toml", "Invalid syntax");
  assert_eq!(
    error.to_string(),
    "Invalid configuration format in config.toml: Invalid syntax"
  );
  assert_eq!(error.error_code(), Code::ConfigInvalid);
  assert_eq!(error.severity(), Severity::Critical);
  assert!(!error.is_recoverable());

  if let Some(meta) = error.metadata() {
    assert_eq!(meta.component.unwrap(), "config");
    assert_eq!(meta.context.get("file").unwrap(), "config.toml");
  } else {
    panic!("Metadata should exist for InvalidFormat");
  }
}

#[test]
fn test_missing_key_error() {
  let error = Error::missing_key("database.url");
  assert_eq!(
    error.to_string(),
    "Missing required configuration key: database.url"
  );
  assert_eq!(error.error_code(), Code::ConfigMissingKey);
  assert_eq!(error.severity(), Severity::Critical);
  assert!(!error.is_recoverable());
}

#[test]
fn test_invalid_value_error() {
  let error = Error::invalid_value("server.port", "must be a number");
  assert_eq!(
    error.to_string(),
    "Invalid value for server.port: must be a number"
  );
  assert_eq!(error.error_code(), Code::ConfigInvalid);
  assert_eq!(error.severity(), Severity::Critical);
  assert!(!error.is_recoverable());
}

#[test]
fn test_environment_error() {
  let error = Error::Environment {
    message: "VAR not found".to_string()
  };
  assert_eq!(
    error.to_string(),
    "Environment variable error: VAR not found"
  );
  assert_eq!(error.error_code(), Code::IoError);
  assert_eq!(error.severity(), Severity::Warning);
  assert!(error.is_recoverable());
}

#[test]
fn test_generic_config_error() {
  // This requires creating a `config::ConfigError` which is complex to do
  // directly. We can simulate it by wrapping it in our top-level error.
  let config_lib_error =
    config::ConfigError::Message("a generic error".to_string());
  let error = Error::from(config_lib_error);
  eprintln!("Testing config error: {error:?}");
  assert!(error.to_string().contains("a generic error"));
  assert_eq!(error.error_code(), Code::ConfigInvalid);
  assert_eq!(error.severity(), Severity::Error);
  assert!(!error.is_recoverable());
  assert!(error.metadata().is_none());
}
