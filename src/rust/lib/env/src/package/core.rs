//! Package domain model.
//!
//! Represents a single package/crate in a workspace.
//! Pure domain model - uses shared Metadata from metadata module.
//!
//! # Examples
//!
//! ```no_run
//! use prjenv::prelude::*;
//!
//! let package = Package::new()
//!   .with_name(env!("CARGO_PKG_NAME"))
//!   .with_version(env!("CARGO_PKG_VERSION"))
//!   .with_description(env!("CARGO_PKG_DESCRIPTION"));
//!
//! println!("Package: {}", package.metadata.display_name());
//! ```

use crate::prelude::*;

/// Package domain model.
///
/// Represents a single package/crate with its metadata.
///
/// # Fields
/// - `metadata` - Package name, version, description
///
/// # Builder Pattern
/// ```no_run
/// use prjenv::prelude::*;
///
/// let package = Package::new()
///   .with_name("my-cli")
///   .with_version("1.0.0")
///   .with_description("A CLI application");
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Package {
  /// Package metadata (name, version, description)
  pub metadata: Metadata,
}

impl Package {
  /// Creates a new empty package.
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  /// Creates a package with metadata.
  #[must_use]
  pub const fn with_metadata(metadata: Metadata) -> Self {
    Self { metadata }
  }

  //╔═══════════════════════════════════════════════════════════╗
  //║ Metadata Builders (delegation)                            ║
  //╚═══════════════════════════════════════════════════════════╝

  /// Sets the package name.
  ///
  /// # Examples
  /// ```no_run
  /// use prjenv::prelude::*;
  ///
  /// let package = Package::new().with_name(env!("CARGO_PKG_NAME"));
  /// ```
  #[must_use]
  pub fn with_name(mut self, name: impl Into<String>) -> Self {
    self.metadata = self.metadata.with_name(name);
    self
  }

  /// Sets the package version.
  ///
  /// # Examples
  /// ```no_run
  /// use prjenv::prelude::*;
  ///
  /// let package = Package::new().with_version(env!("CARGO_PKG_VERSION"));
  /// ```
  #[must_use]
  pub fn with_version(mut self, version: impl Into<String>) -> Self {
    self.metadata = self.metadata.with_version(version);
    self
  }

  /// Sets the package description.
  ///
  /// # Examples
  /// ```no_run
  /// use prjenv::prelude::*;
  ///
  /// let package = Package::new().with_description(env!("CARGO_PKG_DESCRIPTION"));
  /// ```
  #[must_use]
  pub fn with_description(mut self, description: impl Into<String>) -> Self {
    self.metadata = self.metadata.with_description(description);
    self
  }

  //╔═══════════════════════════════════════════════════════════╗
  //║ Queries                                                   ║
  //╚═══════════════════════════════════════════════════════════╝

  /// Returns the package name.
  #[must_use]
  pub fn name(&self) -> &str {
    &self.metadata.name
  }

  /// Returns the package version.
  #[must_use]
  pub fn version(&self) -> &str {
    &self.metadata.version
  }

  /// Returns the package description.
  #[must_use]
  pub fn description(&self) -> &str {
    &self.metadata.description
  }
}

impl Display for Package {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.metadata)
  }
}

impl From<Metadata> for Package {
  fn from(metadata: Metadata) -> Self {
    Self { metadata }
  }
}
