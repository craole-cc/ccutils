use crate::{Code, Context, Metadata, Severity};

/// HTTP and network related errors, wrapping `reqwest::Error`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// An error from the `reqwest` crate.
  #[error(transparent)]
  Request(#[from] reqwest::Error),

  /// A custom error message for an HTTP-related issue.
  #[error("HTTP error: {message}")]
  Custom { message: String }
}

impl Error {
  /// Create a new custom HTTP error.
  pub fn custom<T: Into<String>>(msg: T) -> Self {
    Self::Custom {
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
      Error::Request(e) => {
        // Timeouts and connection errors are often transient and can be
        // retried.
        if e.is_timeout() || e.is_connect() {
          return true;
        }
        // Server-side errors (5xx) might be temporary.
        if let Some(status) = e.status() {
          return status.is_server_error();
        }
        // Builder errors, redirect loops, client errors (4xx), and body/decode
        // errors are generally not recoverable without changing the request.
        false
      }
      // Custom errors are conservatively marked as not recoverable.
      Error::Custom { .. } => false
    }
  }

  fn severity(&self) -> Severity {
    match self {
      Error::Request(e) => {
        if e.is_timeout() {
          //? A timeout is often less severe and retriable.
          return Severity::Warning;
        }
        if e.is_connect() {
          //? Failure to connect is a more significant error.
          return Severity::Error;
        }
        if let Some(status) = e.status() {
          if status.is_client_error() {
            //? 4xx errors indicate a problem with the client's request.
            return Severity::Error;
          }
          if status.is_server_error() {
            //? 5xx errors indicate a problem on the server side.
            return Severity::Critical;
          }
        }
        //? Other errors (builder, redirect, body, decode) are likely programming errors.
        Severity::Error
      }
      Error::Custom { .. } => Severity::Error
    }
  }

  fn error_code(&self) -> Code {
    // TODO: Implement more specific HTTP error codes
    Code::NetworkError
  }

  fn metadata(&self) -> Option<Metadata> {
    None
  }
}
