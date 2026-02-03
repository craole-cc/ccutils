//! Workspace domain model.
//!
//! Represents a Rust workspace containing multiple packages.
//! This is a pure domain model - no infrastructure concerns.
//!
//! # Relationship to Workspace
//!
//! The global `Workspace` struct composes:
//! - `Workspace` (this module) - Domain model
//! - `Paths` - Infrastructure (filesystem)
//! - `Configuration` - Infrastructure (env vars)
//!
//! # Examples
//!
//! ```no_run
//! use craole_cc_project::prelude::*;
//!
//! let workspace = Workspace::new()
//!   .with_name("my-workspace")
//!   .with_version("1.0.0")
//!   .with_package_name("api")
//!   .with_package_name("cli");
//!
//! println!(
//!   "Workspace: {} v{}",
//!   workspace.metadata.name, workspace.metadata.version
//! );
//! println!("Packages: {}", workspace.package_count());
//! ```

use crate::_prelude::*;

/// Workspace domain model.
///
/// Represents a Rust workspace with metadata and registered packages.
/// Does NOT contain infrastructure concerns like paths or configuration.
///
/// # Fields
/// - `metadata` - Workspace name, version, description
/// - `packages` - Packages registered in this workspace
///
/// # Builder Pattern
/// ```no_run
/// use craole_cc_project::prelude::*;
///
/// let workspace = Workspace::new()
///   .with_name("my-workspace")
///   .with_packages(vec![
///     Package::new().with_name("api"),
///     Package::new().with_name("cli"),
///   ]);
/// ```
#[derive(Debug, Clone, Default)]
pub struct Workspace {
  /// Workspace metadata (name, version, description)
  pub metadata: Metadata,

  /// Packages registered in this workspace
  pub packages: Vec<Package>,
}

impl Workspace {
  /// Creates a new empty workspace with default metadata.
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  //╔═══════════════════════════════════════════════════════════╗
  //║ Metadata Builders                                         ║
  //╚═══════════════════════════════════════════════════════════╝

  /// Sets the workspace name.
  ///
  /// # Examples
  /// ```no_run
  /// use craole_cc_project::prelude::*;
  /// let workspace = Workspace::new().with_name("my-workspace");
  /// ```
  #[must_use]
  pub fn with_name(mut self, name: impl Into<String>) -> Self {
    self.metadata = self.metadata.with_name(name);
    self
  }

  /// Sets the workspace version.
  ///
  /// # Examples
  /// ```no_run
  /// use craole_cc_project::prelude::*;
  /// let workspace = Workspace::new().with_version("2.0.0");
  /// ```
  #[must_use]
  pub fn with_version(mut self, version: impl Into<String>) -> Self {
    self.metadata = self.metadata.with_version(version);
    self
  }

  /// Sets the workspace description.
  #[must_use]
  pub fn with_description(mut self, description: impl Into<String>) -> Self {
    self.metadata = self.metadata.with_description(description);
    self
  }

  /// Sets all metadata at once.
  ///
  /// # Examples
  /// ```no_run
  /// use craole_cc_project::prelude::*;
  ///
  /// let metadata = Metadata::new()
  ///   .with_name("my-workspace")
  ///   .with_version("1.0.0");
  ///
  /// let workspace = Workspace::new().with_metadata(metadata);
  /// ```
  #[must_use]
  pub fn with_metadata(mut self, metadata: Metadata) -> Self {
    self.metadata = metadata;
    self
  }

  //╔═══════════════════════════════════════════════════════════╗
  //║ Package Management                                        ║
  //╚═══════════════════════════════════════════════════════════╝

  /// Registers multiple packages, replacing existing ones.
  ///
  /// # Examples
  /// ```no_run
  /// use craole_cc_project::prelude::*;
  ///
  /// let workspace = Workspace::new().with_packages(vec![
  ///   Package::new().with_name("api"),
  ///   Package::new().with_name("cli"),
  /// ]);
  /// ```
  #[must_use]
  pub fn with_packages(mut self, packages: impl IntoIterator<Item = Package>) -> Self {
    self.packages = packages.into_iter().collect();
    self
  }

  /// Adds a single package.
  ///
  /// # Examples
  /// ```no_run
  /// use craole_cc_project::prelude::*;
  ///
  /// let workspace = Workspace::new().with_package(Package::new().with_name("api"));
  /// ```
  #[must_use]
  pub fn with_package(mut self, package: Package) -> Self {
    self.packages.push(package);
    self
  }

  /// Convenience method to add a package by name only.
  ///
  /// # Examples
  /// ```no_run
  /// use craole_cc_project::prelude::*;
  ///
  /// let workspace = Workspace::new()
  ///   .with_package_name("api")
  ///   .with_package_name("cli");
  /// ```
  #[must_use]
  pub fn with_package_name(mut self, name: impl Into<String>) -> Self {
    self.packages.push(Package::new().with_name(name));
    self
  }

  //╔═══════════════════════════════════════════════════════════╗
  //║ Package Queries                                           ║
  //╚═══════════════════════════════════════════════════════════╝

  /// Finds a package by name.
  ///
  /// # Examples
  /// ```no_run
  /// use craole_cc_project::prelude::*;
  ///
  /// let workspace = Workspace::new().with_package_name("api");
  ///
  /// if let Some(pkg) = workspace.find_package("api") {
  ///   println!("Found: {}", pkg.metadata.name);
  /// }
  /// ```
  pub fn find_package(&self, name: &str) -> Option<&Package> {
    self.packages.iter().find(|p| p.metadata.name == name)
  }

  /// Returns mutable reference to a package by name.
  pub fn find_package_mut(&mut self, name: &str) -> Option<&mut Package> {
    self.packages.iter_mut().find(|p| p.metadata.name == name)
  }

  /// Returns the number of registered packages.
  #[must_use]
  pub fn package_count(&self) -> usize {
    self.packages.len()
  }

  /// Checks if a package is registered.
  #[must_use]
  pub fn has_package(&self, name: &str) -> bool {
    self.find_package(name).is_some()
  }

  /// Returns an iterator over all packages.
  pub fn packages(&self) -> impl Iterator<Item = &Package> {
    self.packages.iter()
  }

  /// Returns package names as a vector.
  pub fn package_names(&self) -> Vec<&str> {
    self
      .packages
      .iter()
      .map(|p| p.metadata.name.as_str())
      .collect()
  }
}

impl Display for Workspace {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(
      f,
      "{} v{} ({} packages)",
      self.metadata.name,
      self.metadata.version,
      self.package_count()
    )
  }
}
