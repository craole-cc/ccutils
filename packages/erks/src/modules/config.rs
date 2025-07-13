use crate::{Code, Context, Metadata, Severity};

/// Configuration loading and parsing errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Configuration file not found
  #[error("Configuration file not found: {path}")]
  FileNotFound { path: String },

  /// Invalid configuration format
  #[error("Invalid configuration format in {file}: {message}")]
  InvalidFormat {
    file: String,
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>
  },

  /// Missing required configuration key
  #[error("Missing required configuration key: {key}")]
  MissingKey { key: String },

  /// Invalid configuration value
  #[error("Invalid value for {key}: {message}")]
  InvalidValue { key: String, message: String },

  /// Environment variable error
  #[error("Environment variable error: {message}")]
  Environment { message: String },

  /// Generic configuration error
  #[error("Configuration error: {0}")]
  Generic(#[from] config::ConfigError)
}

impl Error {
  /// Create a file not found error
  pub fn file_not_found<T: Into<String>>(path: T) -> Self {
    Self::FileNotFound { path: path.into() }
  }

  /// Create an invalid format error
  pub fn invalid_format<T: Into<String>, F: Into<String>>(file: F, msg: T) -> Self {
    Self::InvalidFormat {
      file: file.into(),
      message: msg.into(),
      source: None
    }
  }

  /// Create a missing key error
  pub fn missing_key<T: Into<String>>(key: T) -> Self {
    Self::MissingKey { key: key.into() }
  }

  /// Create an invalid value error
  pub fn invalid_value<K: Into<String>, T: Into<String>>(key: K, msg: T) -> Self {
    Self::InvalidValue {
      key: key.into(),
      message: msg.into()
    }
  }
}

impl Context for Error {
  fn description(&self) -> String {
    format!("{self}")
  }

  fn is_recoverable(&self) -> bool {
    match self {
      Error::FileNotFound { .. } => true,
      Error::InvalidFormat { .. } => false,
      Error::MissingKey { .. } => false,
      Error::InvalidValue { .. } => false,
      Error::Environment { .. } => true,
      Error::Generic(_) => false
    }
  }

  fn severity(&self) -> Severity {
    match self {
      Error::FileNotFound { .. } => Severity::Error,
      Error::InvalidFormat { .. } => Severity::Critical,
      Error::MissingKey { .. } => Severity::Critical,
      Error::InvalidValue { .. } => Severity::Critical,
      Error::Environment { .. } => Severity::Warning,
      Error::Generic(_) => Severity::Error
    }
  }

  fn error_code(&self) -> Code {
    match self {
      Error::FileNotFound { .. } => Code::ConfigNotFound,
      Error::InvalidFormat { .. } => Code::ConfigInvalid,
      Error::MissingKey { .. } => Code::ConfigMissingKey,
      Error::InvalidValue { .. } => Code::ConfigInvalid,
      Error::Environment { .. } => Code::IoError, //? Environment errors are  often I/O related
      Error::Generic(_) => Code::ConfigInvalid
    }
  }

  fn metadata(&self) -> Option<Metadata> {
    let metadata = Metadata::new(self.error_code()).with_component("config");

    match self {
      Error::FileNotFound { path } => Some(metadata.with_context("path", path.clone())),
      Error::InvalidFormat { file, .. } => Some(metadata.with_context("file", file.clone())),
      Error::MissingKey { key } => Some(metadata.with_context("key", key.clone())),
      Error::InvalidValue { key, .. } => Some(metadata.with_context("key", key.clone())),
      Error::Environment { message } => Some(metadata.with_context("message", message.clone())),
      Error::Generic(_) => None /* Generic config errors might not have
                                 * specific metadata */
    }
  }
}
