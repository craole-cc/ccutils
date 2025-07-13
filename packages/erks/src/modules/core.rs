use crate::*;
use std::{
  collections::HashMap,
  fmt::{self, Display, Formatter}
};

/// Alias for our custom error type
pub type Result<T> = std::result::Result<T, Error>;

/// Error codes for programmatic error handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Code {
  //? System errors
  IoError,
  PermissionDenied,
  NotFound,
  NetworkError,

  //? Configuration errors
  ConfigNotFound,
  ConfigInvalid,
  ConfigMissingKey,

  //? Data errors
  ParseError,
  ValidationError,

  //? Application errors
  InvalidState,
  ResourceLimit,
  BusinessLogic,

  //? Generic
  Unknown
}

impl Display for Code {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::IoError => write!(f, "IO_ERROR"),
      Self::PermissionDenied => write!(f, "PERMISSION_DENIED"),
      Self::NotFound => write!(f, "NOT_FOUND"),
      Self::NetworkError => write!(f, "NETWORK_ERROR"),
      Self::ConfigNotFound => write!(f, "CONFIG_NOT_FOUND"),
      Self::ConfigInvalid => write!(f, "CONFIG_INVALID"),
      Self::ConfigMissingKey => write!(f, "CONFIG_MISSING_KEY"),
      Self::ParseError => write!(f, "PARSE_ERROR"),
      Self::ValidationError => write!(f, "VALIDATION_ERROR"),
      Self::InvalidState => write!(f, "INVALID_STATE"),
      Self::ResourceLimit => write!(f, "RESOURCE_LIMIT"),
      Self::BusinessLogic => write!(f, "BUSINESS_LOGIC"),
      Self::Unknown => write!(f, "UNKNOWN")
    }
  }
}

/// Additional metadata for errors
#[derive(Debug, Clone)]
pub struct Metadata {
  pub error_code: Code,
  pub component: Option<String>,
  pub operation: Option<String>,
  pub context: HashMap<String, String>
}

impl Metadata {
  pub fn new(code: Code) -> Self {
    Self {
      error_code: code,
      component: None,
      operation: None,
      context: HashMap::new()
    }
  }

  pub fn with_component<T: Into<String>>(mut self, component: T) -> Self {
    self.component = Some(component.into());
    self
  }

  pub fn with_operation<T: Into<String>>(mut self, operation: T) -> Self {
    self.operation = Some(operation.into());
    self
  }

  pub fn with_context<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
    self.context.insert(key.into(), value.into());
    self
  }
}

/// Common trait for all error types in the system
pub trait Context {
  /// Get a human-readable description of the error
  fn description(&self) -> String;

  /// Check if this error is likely recoverable
  fn is_recoverable(&self) -> bool;

  /// Get error severity level
  fn severity(&self) -> Severity;

  /// Get error code for programmatic handling
  fn error_code(&self) -> Code;

  /// Get error metadata
  fn metadata(&self) -> Option<Metadata>;
}

/// Error severity levels for logging and handling decisions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
  /// Informational - not really an error
  Info,
  /// Warning - something unusual but not critical
  Warning,
  /// Error - something went wrong but application can continue
  Error,
  /// Critical - something went wrong that requires immediate attention
  Critical
}

impl Display for Severity {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Severity::Info => write!(f, "INFO"),
      Severity::Warning => write!(f, "WARN"),
      Severity::Error => write!(f, "ERROR"),
      Severity::Critical => write!(f, "CRITICAL")
    }
  }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// I/O related errors (file system, network, etc.)
  #[error(transparent)]
  System(#[from] io::Error),

  /// Application-specific custom errors
  #[error(transparent)]
  Custom(#[from] custom::Error),

  #[cfg(feature = "http")]
  #[error(transparent)]
  Http(#[from] http::Error),

  /// Configuration loading and parsing errors
  #[cfg(feature = "config")]
  #[error(transparent)]
  Config(#[from] config::Error),

  /// Data parsing and serialization errors
  #[cfg(any(feature = "json", feature = "toml"))]
  #[error(transparent)]
  Parse(#[from] parse::Error),

  /// Glob pattern matching errors
  #[cfg(feature = "glob")]
  #[error(transparent)]
  Glob(#[from] glob::Error),

  /// Generic anyhow errors for cases that don't fit other categories
  #[error(transparent)]
  Anyhow(#[from] anyhow::Error)
}

impl Error {
  /// Create a new custom error
  pub fn custom<T: Into<String>>(msg: T) -> Self {
    Self::Custom(custom::Error::new(msg))
  }

  /// Create a new I/O error
  pub fn io<T: Into<String>>(msg: T) -> Self {
    Self::System(io::Error::new(msg))
  }

  /// Create a validation error
  pub fn validation<T: Into<String>>(msg: T) -> Self {
    Self::Custom(custom::Error::validation(msg))
  }

  /// Create an invalid state error
  pub fn invalid_state<T: Into<String>>(msg: T) -> Self {
    Self::Custom(custom::Error::invalid_state(msg))
  }

  /// Get the error category as a string
  pub fn category(&self) -> &'static str {
    match self {
      Error::System(_) => "system",
      Error::Custom(_) => "custom",
      #[cfg(feature = "config")]
      Error::Config(_) => "config",
      #[cfg(any(feature = "json", feature = "toml"))]
      Error::Parse(_) => "parse",
      #[cfg(feature = "http")]
      Error::Http(_) => "http",
      #[cfg(feature = "glob")]
      Error::Glob(_) => "glob",
      Error::Anyhow(_) => "generic"
    }
  }

  /// Check if this is a recoverable error
  pub fn is_recoverable(&self) -> bool {
    match self {
      Error::System(e) => e.is_recoverable(),
      Error::Custom(e) => e.is_recoverable(),
      #[cfg(feature = "config")]
      Error::Config(e) => e.is_recoverable(),
      #[cfg(feature = "glob")]
      Error::Glob(e) => e.is_recoverable(),
      #[cfg(feature = "http")]
      Error::Http(e) => e.is_recoverable(),
      #[cfg(any(feature = "json", feature = "toml"))]
      Error::Parse(e) => e.is_recoverable(),
      Error::Anyhow(_) => false // Conservative default for unstructured errors
    }
  }

  /// Get the severity of the error
  pub fn severity(&self) -> Severity {
    match self {
      Error::System(e) => e.severity(),
      Error::Custom(e) => e.severity(),
      #[cfg(feature = "config")]
      Error::Config(e) => e.severity(),
      #[cfg(feature = "glob")]
      Error::Glob(e) => e.severity(),
      #[cfg(feature = "http")]
      Error::Http(e) => e.severity(),
      #[cfg(any(feature = "json", feature = "toml"))]
      Error::Parse(e) => e.severity(),
      Error::Anyhow(_) => Severity::Error /* Conservative default for
                                           * unstructured errors */
    }
  }

  /// Get error code for programmatic handling
  pub fn error_code(&self) -> Code {
    match self {
      Error::System(e) => e.error_code(),
      Error::Custom(e) => e.error_code(),
      #[cfg(feature = "config")]
      Error::Config(e) => e.error_code(),
      #[cfg(feature = "glob")]
      Error::Glob(e) => e.error_code(),
      #[cfg(feature = "http")]
      Error::Http(e) => e.error_code(),
      #[cfg(any(feature = "json", feature = "toml"))]
      Error::Parse(e) => e.error_code(),
      Error::Anyhow(_) => Code::Unknown
    }
  }

  /// Get error metadata
  pub fn metadata(&self) -> Option<Metadata> {
    match self {
      Error::System(e) => e.metadata(),
      Error::Custom(e) => e.metadata(),
      #[cfg(feature = "config")]
      Error::Config(e) => e.metadata(),
      #[cfg(feature = "glob")]
      Error::Glob(e) => e.metadata(),
      #[cfg(feature = "http")]
      Error::Http(e) => e.metadata(),
      #[cfg(any(feature = "json", feature = "toml"))]
      Error::Parse(e) => e.metadata(),
      Error::Anyhow(_) => None
    }
  }

  /// Convert to structured error data for serialization/logging
  pub fn to_structured(&self) -> StructuredError {
    StructuredError {
      message: self.to_string(),
      category: self.category().to_string(),
      code: self.error_code(),
      severity: self.severity(),
      recoverable: self.is_recoverable(),
      metadata: self.metadata()
    }
  }
}

/// Structured representation of an error for serialization/logging
#[derive(Debug, Clone)]
pub struct StructuredError {
  pub message: String,
  pub category: String,
  pub code: Code,
  pub severity: Severity,
  pub recoverable: bool,
  pub metadata: Option<Metadata>
}
