//! Current running crate (package) environment.
//!
//! This module defines the `Package` environment struct that holds metadata about
//! the currently executing binary or library crate.
//!
//! # Relationship to Project
//!
//! The global `Environment` struct (in `core.rs`) contains both:
//! - **Project**: Workspace-level configuration (static, workspace-wide)
//! - **Package**: Individual crate configuration (dynamic, current crate only)
//!
//! In a workspace, multiple packages might exist, but only one is running at a time.
//! The `Package` struct represents the metadata of that currently executing crate.
//!
//! # Initialization
//!
//! `Package` metadata is typically set via compile-time env vars and builder methods:
//! ```no_run
//! use env::prelude::*;
//!
//! let package = Package::new()
//!   .with_name(env!("CARGO_PKG_NAME"))
//!   .with_version(env!("CARGO_PKG_VERSION"))
//!   .with_description(env!("CARGO_PKG_DESCRIPTION"));
//! ```
//!
//! Or using the `setenv!()` macro:
//! ```no_run
//! # #[cfg(feature = "macros")]
//! # {
//! use env::prelude::*;
//! setenv!(); // Auto-populates from env! macros
//!
//! # }
//! ```
//!
//! # Examples
//!
//! ```no_run
//! use env::prelude::*;
//!
//! let env = get_env();
//! let pkg_name = &env.package.metadata.name;
//! let pkg_version = &env.package.metadata.version;
//! println!("{} v{}", pkg_name, pkg_version);
//! ```

use super::Metadata;

/// Current running package (crate) environment.
///
/// Contains metadata about the currently executing binary or library.
/// This is distinct from `Project` which represents the entire workspace.
///
/// # Single Field
/// - `metadata` - The package name, version, and description
///
/// # Builder Pattern
/// All `with_*` methods return `Self` for method chaining:
/// ```no_run
/// use env::package::prelude::*;
/// let package = Package::new()
///   .with_name("my-cli")
///   .with_version("1.0.0")
///   .with_description("A CLI application");
/// ```
///
/// # Thread Safety
/// Safe to clone and share; contains only `String` fields.
///
/// # Defaults
/// By default, Package metadata is cloned from Project metadata.
/// This provides sensible defaults if not explicitly set, but typically you'll
/// override these values with the current crate's `env!()` macros.
///
/// # Examples
/// ```no_run
/// use env::package::prelude::*;
///
/// let package = Package::default();
/// println!("Running: {}", package.metadata.name);
///
/// // Override for specific needs
/// let custom = Package::new()
///   .with_name("my-special-name")
///   .with_version("2.0.0");
/// ```
#[derive(Debug, Clone)]
pub struct Environment {
  /// Package metadata (name, version, description).
  ///
  /// Represents the currently executing crate's information.
  /// Set via builder methods or by cloning from Project metadata by default.
  pub metadata: Metadata,
}

impl Default for Environment {
  /// Creates a package environment with default metadata.
  ///
  /// # Default Behavior
  /// By default, package metadata is cloned from the project metadata.
  /// This means if not explicitly set, the package will have the same
  /// name/version/description as the workspace.
  ///
  /// This is typically overridden immediately via builder methods or
  /// the `setenv!()` macro using compile-time `env!()` values.
  ///
  /// # Examples
  /// ```no_run
  /// use env::package::prelude::*;
  /// let package = Package::default();
  /// // Will have project's metadata
  /// ```
  fn default() -> Self {
    let metadata = Metadata::default();

    Self { metadata }
  }
}

impl Environment {
  /// Creates a new default package environment.
  ///
  /// Equivalent to `Package::default()`. Use builder methods to customize.
  ///
  /// # Examples
  /// ```no_run
  /// use env::package::prelude::*;
  /// let package = Package::new();
  /// ```
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the package name, overriding the default.
  ///
  /// # Examples
  /// ```no_run
  /// use env::package::prelude::*;
  ///
  /// let package = Package::new().with_name(env!("CARGO_PKG_NAME"));
  /// ```
  #[must_use]
  pub fn with_name(mut self, name: impl Into<String>) -> Self {
    self.metadata = self.metadata.with_name(name);
    self
  }

  /// Sets the package version, overriding the default.
  ///
  /// # Examples
  /// ```no_run
  /// use env::package::prelude::*;
  ///
  /// let package = Package::new().with_version(env!("CARGO_PKG_VERSION"));
  /// ```
  #[must_use]
  pub fn with_version(mut self, version: impl Into<String>) -> Self {
    self.metadata = self.metadata.with_version(version);
    self
  }

  /// Sets the package description, overriding the default.
  ///
  /// # Examples
  /// ```no_run
  /// use env::package::prelude::*;
  ///
  /// let package = Package::new().with_description(env!("CARGO_PKG_DESCRIPTION"));
  /// ```
  #[must_use]
  pub fn with_description(mut self, description: impl Into<String>) -> Self {
    self.metadata = self.metadata.with_description(description);
    self
  }
}
