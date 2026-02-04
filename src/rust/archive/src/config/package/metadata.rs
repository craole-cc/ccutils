//! Current package (crate) metadata.
//!
//! Provides metadata about the currently executing crate (name, version, description).
//! This metadata is typically populated from compile-time environment variables using
//! the `env!()` macro, allowing each crate to declare its own identity.
//!
//! # Relationship to Project Metadata
//!
//! There are two levels of metadata:
//! - **Project**: Workspace-level metadata (read from workspace Cargo.toml)
//! - **Package**: Individual crate metadata (this module, typically from `env!()` macros)
//!
//! By default, package metadata is initialized by cloning project metadata,
//! but this is typically overridden with the actual crate's compile-time values.
//!
//! # Compile-Time Integration
//!
//! The recommended way to populate package metadata is using Rust's `env!()` macros:
//! ```no_run
//! use env::prelude::*;
//!
//! let package = Package::new()
//!   .with_name(env!("CARGO_PKG_NAME"))
//!   .with_version(env!("CARGO_PKG_VERSION"))
//!   .with_description(env!("CARGO_PKG_DESCRIPTION"));
//!
//! println!("{} v{}", package.metadata.name, package.metadata.version);
//! ```
//!
//! Or more simply with the `setenv!()` macro:
//! ```no_run
//! # #[cfg(feature = "macros")]
//! # {
//! use env::prelude::*;
//!
//! fn main() {
//!   setenv!(); // Auto-populates package metadata from env! macros
//! }
//! # }
//! ```
//!
//! # Examples
//!
//! ```no_run
//! use env::package::prelude::*;
//!
//! let metadata = PackageMetadata::new()
//!   .with_name("my-cli")
//!   .with_version("1.2.3")
//!   .with_description("A command-line tool");
//!
//! println!(
//!   "{} v{}: {}",
//!   metadata.name, metadata.version, metadata.description
//! );
//! ```

use crate::prelude::*;

/// Package (crate) metadata.
///
/// Holds information about the currently executing crate:
/// - `name` - Crate name (e.g., "my-app", "my-cli")
/// - `version` - Semantic version (e.g., "1.0.0")
/// - `description` - Brief description of the crate
///
/// # Defaults
/// By default, package metadata is cloned from the project (workspace) metadata.
/// This provides sensible fallback values but is typically overridden with actual
/// crate metadata from compile-time `env!()` macros.
///
/// # Builder Pattern
/// Use `with_*` methods for method chaining:
/// ```no_run
/// use env::package::prelude::*;
///
/// let metadata = PackageMetadata::new()
///   .with_name(env!("CARGO_PKG_NAME"))
///   .with_version(env!("CARGO_PKG_VERSION"))
///   .with_description(env!("CARGO_PKG_DESCRIPTION"));
/// ```
///
/// # Thread Safety
/// Safe to clone and share; all fields are `String` types.
///
/// # Compile-Time vs Runtime
/// - **Compile-time** (recommended): Use `env!()` macros, values baked into binary
/// - **Runtime**: Set via builder methods, values loaded at startup
///
/// Compile-time is preferred as it requires zero I/O and prevents mismatches
/// between declared and actual versions.
///
/// # Examples
/// ```no_run
/// use env::package::prelude::*;
///
/// // Default (cloned from project)
/// let default_meta = PackageMetadata::default();
///
/// // With compile-time values
/// let meta = PackageMetadata::new()
///   .with_name(env!("CARGO_PKG_NAME"))
///   .with_version(env!("CARGO_PKG_VERSION"));
/// ```
#[derive(Debug, Clone)]
pub struct Metadata {
  /// Package name.
  ///
  /// Typically the `name` field from the crate's `Cargo.toml`.
  /// Examples: "cli", "web", "api", "my-app"
  ///
  /// # Source
  /// Usually populated from `env!("CARGO_PKG_NAME")` at compile time,
  /// providing the exact crate name baked into the binary.
  pub name: String,

  /// Package version.
  ///
  /// Semantic version string (e.g., "1.0.0", "2.1.3-beta.1").
  /// Follows semantic versioning conventions: MAJOR.MINOR.PATCH
  ///
  /// # Source
  /// Usually populated from `env!("CARGO_PKG_VERSION")` at compile time.
  ///
  /// # Why Compile-Time is Preferred
  /// Ensures the version in the binary matches the declared version,
  /// preventing accidental mismatches between code and version declaration.
  pub version: String,

  /// Package description.
  ///
  /// Brief description of what the crate does.
  /// Examples:
  /// - "Command-line interface for the Craole portfolio"
  /// - "Web server component"
  /// - "API implementation"
  ///
  /// # Source
  /// Usually populated from `env!("CARGO_PKG_DESCRIPTION")` at compile time.
  ///
  /// # Typical Length
  /// 1-2 sentences (50-200 characters), fits well in logging and help text.
  pub description: String,
}

impl Default for Metadata {
  /// Creates metadata with default values cloned from project metadata.
  ///
  /// # Behavior
  /// Reads the project (workspace) metadata and clones its name, version, and description.
  /// This serves as a fallback if package metadata is not explicitly set.
  ///
  /// In most cases, you'll want to override this with the actual crate's metadata
  /// using builder methods and compile-time `env!()` macros.
  ///
  /// # Performance
  /// - First call: ~5-50ms (discovers workspace, reads Cargo.toml)
  /// - Subsequent calls: <1µs (uses cached project metadata)
  ///
  /// # Examples
  /// ```no_run
  /// use env::package::prelude::*;
  /// let metadata = PackageMetadata::default();
  /// // Will have the workspace's name, version, description
  /// ```
  fn default() -> Self {
    let project = ProjectMetadata::default();
    Self {
      name: project.name,
      version: project.version,
      description: project.description,
    }
  }
}

impl Metadata {
  /// Creates a new default package metadata instance.
  ///
  /// Equivalent to `Metadata::default()`. Use builder methods to customize.
  ///
  /// # Examples
  /// ```no_run
  /// use env::package::prelude::*;
  /// let metadata = PackageMetadata::new();
  /// ```
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the package name, overriding the default value.
  ///
  /// # Examples
  /// ```no_run
  /// use env::package::prelude::*;
  ///
  /// // Using compile-time env var (recommended)
  /// let metadata = PackageMetadata::new().with_name(env!("CARGO_PKG_NAME"));
  ///
  /// // Or with a literal string
  /// let metadata = PackageMetadata::new().with_name("my-cli");
  /// ```
  ///
  /// # Builder Chaining
  /// Returns `Self` for method chaining:
  /// ```no_run
  /// use env::package::prelude::*;
  /// let metadata = PackageMetadata::new()
  ///   .with_name(env!("CARGO_PKG_NAME"))
  ///   .with_version(env!("CARGO_PKG_VERSION"))
  ///   .with_description(env!("CARGO_PKG_DESCRIPTION"));
  /// ```
  #[must_use]
  pub fn with_name(mut self, name: impl Into<String>) -> Self {
    self.name = name.into();
    self
  }

  /// Sets the package version, overriding the default value.
  ///
  /// # Examples
  /// ```no_run
  /// use env::package::prelude::*;
  ///
  /// // Using compile-time env var (recommended)
  /// let metadata = PackageMetadata::new().with_version(env!("CARGO_PKG_VERSION"));
  ///
  /// // Or with a literal semantic version
  /// let metadata = PackageMetadata::new().with_version("1.2.3");
  /// ```
  ///
  /// # Why Use Compile-Time Values
  /// Using `env!("CARGO_PKG_VERSION")` ensures the version embedded in the binary
  /// always matches the declared version in Cargo.toml. Manual strings risk
  /// accidental mismatches during releases.
  #[must_use]
  pub fn with_version(mut self, version: impl Into<String>) -> Self {
    self.version = version.into();
    self
  }

  /// Sets the package description, overriding the default value.
  ///
  /// # Examples
  /// ```no_run
  /// use env::package::prelude::*;
  ///
  /// // Using compile-time env var (recommended)
  /// let metadata = PackageMetadata::new().with_description(env!("CARGO_PKG_DESCRIPTION"));
  ///
  /// // Or with a literal string
  /// let metadata =
  ///   PackageMetadata::new().with_description("A command-line interface for managing Craole");
  /// ```
  ///
  /// # Typical Use Cases
  /// - Printed in help/version text
  /// - Logged during startup
  /// - Displayed in HTTP response headers
  /// - Used in CLI help output
  #[must_use]
  pub fn with_description(mut self, description: impl Into<String>) -> Self {
    self.description = description.into();
    self
  }
}
