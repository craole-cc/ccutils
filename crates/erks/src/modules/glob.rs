use crate::{Code, Context, Metadata, Severity};

/// Glob pattern related errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Invalid glob pattern
  #[error("Invalid glob pattern: {0}")]
  Pattern(#[from] glob::PatternError),

  /// Glob iteration error
  #[error("Glob iteration error: {message}")]
  Iteration { message: String }
}

impl Error {
  /// Create a glob iteration error
  pub fn iteration<T: Into<String>>(msg: T) -> Self {
    Self::Iteration {
      message: msg.into()
    }
  }
}

impl Context for Error {
  fn description(&self) -> String {
    format!("{self}")
  }

  fn is_recoverable(&self) -> bool {
    true
  }

  fn severity(&self) -> Severity {
    Severity::Warning
  }

  fn error_code(&self) -> Code {
    match self {
      Error::Pattern(_) => Code::ParseError, //? Pattern errors are a form of parsing error
      Error::Iteration { .. } => Code::IoError //? Iteration errors might be due to underlying I/O issues
    }
  }

  fn metadata(&self) -> Option<Metadata> {
    let metadata = Metadata::new(self.error_code()).with_component("glob");
    match self {
      Error::Iteration { message } =>
        Some(metadata.with_context("message", message.clone())),
      _ => None
    }
  }
}
