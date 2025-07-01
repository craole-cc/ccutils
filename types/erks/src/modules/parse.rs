use crate::{Code, Context, Metadata, Severity};

/// Parsing and serialization errors for various data formats
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// JSON parsing error
  #[cfg(feature = "json")]
  #[error("JSON parsing error: {0}")]
  Json(#[from] serde_json::Error),

  /// TOML parsing error
  #[cfg(feature = "toml")]
  #[error("TOML parsing error: {0}")]
  Toml(#[from] toml::de::Error),

  /// Generic parsing error
  #[error("Parsing error: {message}")]
  Generic { message: String, format: String }
}

impl Error {
  /// Create a generic parsing error
  pub fn generic<T: Into<String>, F: Into<String>>(msg: T, format: F) -> Self {
    Self::Generic {
      message: msg.into(),
      format: format.into()
    }
  }
}

impl Context for Error {
  fn description(&self) -> String {
    format!("{self}")
  }

  fn is_recoverable(&self) -> bool {
    true // Most parsing errors are recoverable with different input
  }

  fn severity(&self) -> Severity {
    Severity::Error
  }

  fn error_code(&self) -> Code {
    match self {
      #[cfg(feature = "json")]
      Error::Json(_) => Code::ParseError,
      #[cfg(feature = "toml")]
      Error::Toml(_) => Code::ParseError,
      Error::Generic { .. } => Code::ParseError
    }
  }

  fn metadata(&self) -> Option<Metadata> {
    let metadata = Metadata::new(self.error_code()).with_component("parse");
    //? No specific metadata for parse errors beyond the message
    // None
    Some(metadata)
  }
}
