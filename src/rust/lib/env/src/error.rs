//! Error handling for prjenv.
//!
//! This module provides comprehensive error types using `thiserror` for error definitions
//! and `miette` for beautiful error reporting with diagnostic information.
//!
//! # Features
//!
//! - **Structured errors**: Type-safe error variants with `thiserror`
//! - **Rich diagnostics**: Beautiful error messages with `miette`
//! - **Source context**: File paths, line numbers, and helpful suggestions
//! - **Re-exports**: Both `thiserror` and `miette` are re-exported for convenience
//!
//! # Examples
//!
//! ```no_run
//! use prjenv::prelude::*;
//!
//! fn load_config() -> Result<Configuration> {
//!   let path = PathBuf::from("config.toml");
//!
//!   if !path.exists() {
//!     return Err(Error::ConfigNotFound {
//!       path: path.clone(),
//!       suggestion: "Run `cargo init` to create a new project".to_string(),
//!     });
//!   }
//!
//!   Ok(Configuration::new())
//! }
//! ```
//!
//! # Error Reporting
//!
//! For beautiful terminal output, use `miette`'s reporting:
//!
//! ```no_run
//! use {
//!   miette::{
//!     IntoDiagnostic,
//!     Result as MietteResult,
//!   },
//!   prjenv::prelude::*,
//! };
//!
//! fn main() -> MietteResult<()> {
//!   let env = get();
//!
//!   // Your application logic
//!
//!   Ok(())
//! }
//! ```

use crate::prelude::*;
pub mod prelude {
  pub use {
    super::{
      Error as EnvError,
      Result as EnvResult,
    },
    miette::{
      self,
      Diagnostic,
      Report as MietteReport,
      Result as MietteResult,
    },
    std::{
      error::Error as StdError,
      io::Error as IOError,
      result::Result as StdResult,
    },
    thiserror::{
      self,
      Error as ThisError,
    },
    toml::de::Error as TomlDeError,
  };
}

/// Main error type for prjenv operations.
///
/// Uses `thiserror` for ergonomic error definitions and `miette` for
/// rich diagnostic information.
///
/// # Variants
///
/// ## I/O Errors
/// - `IOError` - File system operations failed
///
/// ## Configuration Errors
/// - `ConfigNotFound` - Configuration file not found
/// - `InvalidPort` - Port number out of range or invalid
/// - `InvalidToml` - TOML parsing failed
///
/// ## Workspace Errors
/// - `WorkspaceNotFound` - Could not locate workspace root
/// - `InvalidWorkspace` - Workspace structure is invalid
/// - `PackageNotFound` - Package not found in workspace
///
/// ## Metadata Errors
/// - `MetadataNotFound` - Required metadata field missing
/// - `InvalidMetadata` - Metadata format is invalid
///
/// # Examples
///
/// ```no_run
/// use {
///   prjenv::error::*,
///   std::path::PathBuf,
/// };
///
/// let err = Error::ConfigNotFound {
///   path: PathBuf::from("config.toml"),
///   suggestion: "Create a config.toml file".to_string(),
/// };
///
/// println!("{}", err);
/// ```
#[derive(ThisError, Debug, Diagnostic)]
pub enum Error {
  /// I/O operation failed.
  ///
  /// Wraps standard library I/O errors with additional context.
  #[error("I/O error: {0}")]
  #[diagnostic(code(prjenv::io_error), help("Check file permissions and paths"))]
  IOError(#[from] IOError),

  /// Configuration file not found.
  ///
  /// Indicates that a required configuration file (typically Cargo.toml)
  /// could not be located.
  #[error("Configuration file not found: {path}")]
  #[diagnostic(code(prjenv::config_not_found), help("{suggestion}"))]
  ConfigNotFound {
    /// Path where the config file was expected
    path: PathBuf,
    /// Helpful suggestion for resolving the error
    suggestion: String,
  },

  /// Invalid port number.
  ///
  /// Port must be a valid u16 (0-65535).
  #[error("Invalid port: {value}")]
  #[diagnostic(
    code(prjenv::invalid_port),
    help("Port must be a number between 0 and 65535")
  )]
  InvalidPort {
    /// The invalid port value provided
    value: String,
  },

  /// TOML parsing failed.
  ///
  /// The TOML file exists but contains syntax errors or invalid structure.
  #[error("Failed to parse TOML file: {path}")]
  #[diagnostic(code(prjenv::invalid_toml), help("Check TOML syntax at {}", path.display()))]
  InvalidToml {
    /// Path to the malformed TOML file
    path: PathBuf,
    /// The underlying parsing error
    #[source]
    source: TomlDeError,
  },

  /// Workspace root not found.
  ///
  /// Could not locate the workspace root directory through any detection method.
  #[error("Workspace root not found")]
  #[diagnostic(
    code(prjenv::workspace_not_found),
    help(
      "Run this command from within a Cargo workspace, or set WORKSPACE_ROOT environment variable"
    )
  )]
  WorkspaceNotFound,

  /// Invalid workspace structure.
  ///
  /// The workspace Cargo.toml exists but doesn't have the expected structure.
  #[error("Invalid workspace structure in {path}")]
  #[diagnostic(
    code(prjenv::invalid_workspace),
    help("Ensure Cargo.toml has a [workspace] section with 'members' or 'resolver' fields")
  )]
  InvalidWorkspace {
    /// Path to the invalid workspace file
    path: PathBuf,
    /// Description of what's wrong
    reason: String,
  },

  /// Package not found in workspace.
  ///
  /// The requested package name doesn't exist in the workspace members.
  #[error("Package '{name}' not found in workspace")]
  #[diagnostic(
    code(prjenv::package_not_found),
    help("Available packages: {}", available.join(", "))
  )]
  PackageNotFound {
    /// Name of the package that wasn't found
    name: String,
    /// List of available package names
    available: Vec<String>,
  },

  /// Required metadata field missing.
  ///
  /// A required field (name, version, etc.) is missing from package metadata.
  #[error("Required metadata field '{field}' not found")]
  #[diagnostic(
    code(prjenv::metadata_not_found),
    help("Add '{field}' to the [package] section in Cargo.toml")
  )]
  MetadataNotFound {
    /// Name of the missing field
    field: String,
  },

  /// Invalid metadata format.
  ///
  /// Metadata field exists but has an invalid format or value.
  #[error("Invalid metadata: {field}")]
  #[diagnostic(code(prjenv::invalid_metadata), help("{suggestion}"))]
  InvalidMetadata {
    /// Name of the invalid field
    field: String,
    /// What's wrong with it
    reason: String,
    /// How to fix it
    suggestion: String,
  },

  /// Environment variable error.
  ///
  /// Required environment variable is missing or has an invalid value.
  #[error("Environment variable error: {var}")]
  #[diagnostic(code(prjenv::env_var_error), help("{suggestion}"))]
  EnvVarError {
    /// Name of the environment variable
    var: String,
    /// What went wrong
    reason: String,
    /// How to fix it
    suggestion: String,
  },

  /// Custom error with arbitrary message.
  ///
  /// For errors that don't fit other categories.
  #[error("{message}")]
  #[diagnostic(code(prjenv::custom))]
  Custom {
    /// Error message
    message: String,
  },
}

/// Specialized Result type for prjenv operations.
///
/// Uses the prjenv `Error` type for error cases.
///
/// # Examples
///
/// ```no_run
/// use prjenv::prelude::*;
///
/// fn load_metadata(path: &Path) -> Result<Metadata> {
///   if !path.exists() {
///     return Err(Error::ConfigNotFound {
///       path: path.to_path_buf(),
///       suggestion: "Create a Cargo.toml file".to_string(),
///     });
///   }
///
///   Ok(Metadata::new())
/// }
/// ```
pub type Result<T> = StdResult<T, Error>;

impl Error {
  /// Create a custom error with a message.
  ///
  /// # Examples
  ///
  /// ```
  /// use prjenv::error::Error;
  ///
  /// let err = Error::custom("Something went wrong");
  /// ```
  pub fn custom(message: impl Into<String>) -> Self {
    Self::Custom {
      message: message.into(),
    }
  }

  /// Create a config not found error.
  ///
  /// # Examples
  ///
  /// ```
  /// use {
  ///   prjenv::error::Error,
  ///   std::path::PathBuf,
  /// };
  ///
  /// let err = Error::config_not_found(
  ///   PathBuf::from("config.toml"),
  ///   "Run `cargo init` to create a new project",
  /// );
  /// ```
  pub fn config_not_found(path: PathBuf, suggestion: impl Into<String>) -> Self {
    Self::ConfigNotFound {
      path,
      suggestion: suggestion.into(),
    }
  }

  /// Create an invalid port error.
  ///
  /// # Examples
  ///
  /// ```
  /// use prjenv::error::Error;
  ///
  /// let err = Error::invalid_port("99999");
  /// ```
  pub fn invalid_port(value: impl Into<String>) -> Self {
    Self::InvalidPort {
      value: value.into(),
    }
  }

  /// Create a package not found error.
  ///
  /// # Examples
  ///
  /// ```
  /// use prjenv::error::Error;
  ///
  /// let err = Error::package_not_found("api", vec!["cli", "web"]);
  /// ```
  pub fn package_not_found(name: impl Into<String>, available: Vec<String>) -> Self {
    Self::PackageNotFound {
      name: name.into(),
      available,
    }
  }
}

/// Extension trait for converting Results to miette diagnostics.
///
/// Provides convenient methods for working with miette in applications.
///
/// # Examples
///
/// ```no_run
/// use {
///   miette::Result as MietteResult,
///   prjenv::prelude::*,
/// };
///
/// fn main() -> MietteResult<()> {
///   let env = get();
///
///   // Use into_diagnostic() to convert errors
///   std::fs::read_to_string("config.toml").into_diagnostic()?;
///
///   Ok(())
/// }
/// ```
pub trait ResultExt<T> {
  /// Convert to a miette Result for better error reporting.
  fn into_diagnostic(self) -> miette::Result<T>;
}

impl<T, E> ResultExt<T> for StdResult<T, E>
where
  E: StdError + Send + Sync + 'static,
{
  fn into_diagnostic(self) -> miette::Result<T> {
    self.map_err(|e| MietteReport::msg(e.to_string()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_error_display() {
    let err = Error::ConfigNotFound {
      path: PathBuf::from("Cargo.toml"),
      suggestion: "Run cargo init".to_string(),
    };

    let msg = err.to_string();
    assert!(msg.contains("Cargo.toml"));
  }

  #[test]
  fn test_custom_error() {
    let err = Error::custom("test message");
    assert!(err.to_string().contains("test message"));
  }

  #[test]
  fn test_invalid_port() {
    let err = Error::invalid_port("99999");
    assert!(err.to_string().contains("99999"));
  }

  #[test]
  fn test_package_not_found() {
    let err = Error::package_not_found("api", vec!["cli".to_string(), "web".to_string()]);
    assert!(err.to_string().contains("api"));
  }
}
