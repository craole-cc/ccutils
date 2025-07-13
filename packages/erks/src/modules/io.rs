use crate::{Code, Context, Metadata, Severity};

/// I/O operation errors with enhanced context
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// File system operation failed
  #[error("File system error: {message}")]
  FileSystem {
    message: String,
    path: Option<String>,
    #[source]
    source: Option<std::io::Error>
  },

  /// Network operation failed
  #[error("Network error: {message}")]
  Network {
    message: String,
    host: Option<String>,
    port: Option<u16>,
    #[source]
    source: Option<std::io::Error>
  },

  /// Permission denied
  #[error("Permission denied: {message}")]
  Permission {
    message: String,
    resource: Option<String>,
    required_permission: Option<String>,
    #[source]
    source: Option<std::io::Error>
  },

  /// Resource not found
  #[error("Resource not found: {message}")]
  NotFound {
    message: String,
    resource_type: Option<String>,
    searched_paths: Vec<String>,
    #[source]
    source: Option<std::io::Error>
  },

  /// Generic I/O error
  #[error("I/O error: {0}")]
  Generic(#[from] std::io::Error)
}

impl Error {
  /// Create a new file system error
  pub fn file_system<T: Into<String>>(msg: T) -> Self {
    Self::FileSystem {
      message: msg.into(),
      path: None,
      source: None
    }
  }

  /// Create a file system error with path context
  pub fn file_system_with_path<T: Into<String>, P: Into<String>>(
    msg: T,
    path: P
  ) -> Self {
    Self::FileSystem {
      message: msg.into(),
      path: Some(path.into()),
      source: None
    }
  }

  /// Create a new network error
  pub fn network<T: Into<String>>(msg: T) -> Self {
    Self::Network {
      message: msg.into(),
      host: None,
      port: None,
      source: None
    }
  }

  /// Create a network error with host/port context
  pub fn network_with_endpoint<T: Into<String>, H: Into<String>>(
    msg: T,
    host: H,
    port: Option<u16>
  ) -> Self {
    Self::Network {
      message: msg.into(),
      host: Some(host.into()),
      port,
      source: None
    }
  }

  /// Create a new permission error
  pub fn permission<T: Into<String>>(msg: T) -> Self {
    Self::Permission {
      message: msg.into(),
      resource: None,
      required_permission: None,
      source: None
    }
  }

  /// Create a permission error with resource context
  pub fn permission_denied<
    T: Into<String>,
    R: Into<String>,
    P: Into<String>
  >(
    msg: T,
    resource: R,
    permission: P
  ) -> Self {
    Self::Permission {
      message: msg.into(),
      resource: Some(resource.into()),
      required_permission: Some(permission.into()),
      source: None
    }
  }

  /// Create a new not found error
  pub fn not_found<T: Into<String>>(msg: T) -> Self {
    Self::NotFound {
      message: msg.into(),
      resource_type: None,
      searched_paths: Vec::new(),
      source: None
    }
  }

  /// Create a not found error with search context
  pub fn not_found_with_paths<T: Into<String>, R: Into<String>>(
    msg: T,
    resource_type: R,
    paths: Vec<String>
  ) -> Self {
    Self::NotFound {
      message: msg.into(),
      resource_type: Some(resource_type.into()),
      searched_paths: paths,
      source: None
    }
  }

  /// Create a generic I/O error with custom message
  pub fn new<T: Into<String>>(msg: T) -> Self {
    Self::Generic(std::io::Error::other(msg.into()))
  }
}

impl Context for Error {
  fn description(&self) -> String {
    format!("{self}")
  }

  fn is_recoverable(&self) -> bool {
    match self {
      Error::Permission { .. } => false,
      Error::NotFound { .. } => true,
      Error::Network { .. } => true,
      Error::FileSystem {
        source: Some(io_err),
        ..
      } => !matches!(
        io_err.kind(),
        std::io::ErrorKind::PermissionDenied
          | std::io::ErrorKind::OutOfMemory
          | std::io::ErrorKind::InvalidData
      ),
      Error::FileSystem { .. } => true,
      Error::Generic(e) => !matches!(
        e.kind(),
        std::io::ErrorKind::PermissionDenied
          | std::io::ErrorKind::OutOfMemory
          | std::io::ErrorKind::InvalidData
      )
    }
  }

  fn severity(&self) -> Severity {
    match self {
      Error::Permission { .. } => Severity::Critical,
      Error::NotFound { .. } => Severity::Warning,
      Error::Network { .. } => Severity::Error,
      Error::FileSystem {
        source: Some(io_err),
        ..
      } => match io_err.kind() {
        std::io::ErrorKind::OutOfMemory => Severity::Critical,
        std::io::ErrorKind::PermissionDenied => Severity::Critical,
        _ => Severity::Error
      },
      Error::FileSystem { .. } => Severity::Error,
      Error::Generic(e) => match e.kind() {
        std::io::ErrorKind::OutOfMemory => Severity::Critical,
        std::io::ErrorKind::PermissionDenied => Severity::Critical,
        _ => Severity::Error
      }
    }
  }

  fn error_code(&self) -> Code {
    match self {
      Error::Permission { .. } => Code::PermissionDenied,
      Error::NotFound { .. } => Code::NotFound,
      Error::Network { .. } => Code::NetworkError,
      Error::FileSystem { .. } => Code::IoError,
      Error::Generic(_) => Code::IoError
    }
  }

  fn metadata(&self) -> Option<Metadata> {
    let metadata = Metadata::new(self.error_code()).with_component("io");

    match self {
      Error::FileSystem { path: Some(p), .. } =>
        Some(metadata.with_context("path", p.clone())),
      Error::Network {
        host: Some(h),
        port: Some(p),
        ..
      } => Some(
        metadata
          .with_context("host", h.clone())
          .with_context("port", p.to_string())
      ),
      Error::Network { host: Some(h), .. } =>
        Some(metadata.with_context("host", h.clone())),
      Error::Permission {
        resource: Some(r),
        required_permission: Some(p),
        ..
      } => Some(
        metadata
          .with_context("resource", r.clone())
          .with_context("required_permission", p.clone())
      ),
      Error::NotFound {
        resource_type: Some(rt),
        searched_paths,
        ..
      } if !searched_paths.is_empty() => {
        let mut meta = metadata
          .with_context("resource_type", rt.clone())
          .with_context(
            "searched_paths_count",
            searched_paths.len().to_string()
          );

        // Add first few paths to avoid overwhelming the context
        for (i, path) in searched_paths.iter().take(3).enumerate() {
          meta = meta.with_context(format!("searched_path_{i}"), path.clone());
        }

        Some(meta)
      }
      _ => None
    }
  }
}
