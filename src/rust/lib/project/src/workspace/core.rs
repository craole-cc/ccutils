//! Workspace project environment configuration.
//!
//! This module defines the `Project` environment struct that combines workspace metadata,
//! server/runtime configuration, and project paths into a single cohesive structure.
//!
//! The `Project` struct is the workspace-level equivalent of `Package` - it represents
//! the workspace root configuration rather than the individual crate.
//!
//! # Relationship to Global Environment
//!
//! The global `Environment` struct (in `core.rs`) contains both:
//! - **Project**: Workspace-level configuration (this module)
//! - **Package**: Individual crate configuration (in `package` module)
//!
//! # Initialization Flow
//!
//! ```text
//! Environment::default()
//! └── Project::default()
//!     ├── Metadata::default() [reads workspace Cargo.toml]
//!     ├── Paths::default() [discovers workspace root and paths]
//!     └── Configuration::default() [reads env vars]
//! ```
//!
//! # Examples
//!
//! ```no_run
//! use env::project::prelude::*;
//!
//! let project = Project::new();
//! println!("Name: {}", project.metadata.name);
//! println!("Version: {}", project.metadata.version);
//! println!("Database: {}", project.configuration.db);
//! println!("Assets: {}", project.paths.assets.display());
//! ```

use super::{
  Configuration,
  Metadata,
  Paths,
};

/// Workspace project environment configuration.
///
/// Combines three aspects of the workspace into a single struct:
/// 1. **Metadata** - Project name, version, description from Cargo.toml
/// 2. **Paths** - Workspace root and standard directory locations
/// 3. **Configuration** - Server settings and env var configuration
///
/// This represents the entire workspace-level configuration, as opposed to
/// `Package` which represents the individual running crate.
///
/// # Fields
/// - `metadata` - Parsed from workspace Cargo.toml
/// - `paths` - Discovered from filesystem
/// - `configuration` - Loaded from environment variables
///
/// # Builder Pattern
/// All `with_*` methods return `Self` for method chaining:
/// ```no_run
/// use env::project::prelude::*;
/// let project = Project::new()
///   .with_name("my-workspace")
///   .with_port(8080)
///   .with_ip("0.0.0.0");
/// ```
///
/// # Defaults
/// Uses sensible defaults for all fields. Environment variables override defaults,
/// and builder methods override environment variables.
///
/// # Thread Safety
/// Safe to clone and share; all fields are cloneable (`String`, `PathBuf`, `u16`).
///
/// # Examples
/// ```no_run
/// use env::project::prelude::*;
/// let project = Project::default();
/// println!(
///   "Project: {} v{}",
///   project.metadata.name, project.metadata.version
/// );
/// println!("Database: {}", project.configuration.db);
/// ```
#[derive(Debug, Clone)]
pub struct Environment {
  /// Project metadata (name, version, description).
  ///
  /// Read from workspace Cargo.toml and cached. Override using `with_name()`,
  /// `with_version()`, or `with_description()`.
  pub metadata: Metadata,

  /// Project paths (root, assets, database).
  ///
  /// Auto-discovered from workspace root. Immutable after creation;
  /// modify by creating new Paths instance if needed.
  pub paths: Paths,

  /// Project configuration (database, server settings).
  ///
  /// Loaded from environment variables with defaults. Override using
  /// `with_db()`, `with_port()`, or `with_ip()`.
  pub configuration: Configuration,
}

impl Default for Environment {
  /// Creates project environment with auto-discovered workspace configuration.
  ///
  /// # Initialization Order
  /// 1. Create `Metadata` from workspace Cargo.toml
  /// 2. Create `Paths` by discovering workspace root
  /// 3. Create `Configuration` from environment variables
  /// 4. **Special handling**: If `DATABASE_URL` env var is empty,
  ///    override to use `{workspace}/assets/db`
  ///
  /// # Database Path Fallback
  /// This is a key feature: if no `DATABASE_URL` is set, the database
  /// automatically defaults to the discovered workspace assets directory.
  /// This provides sensible out-of-the-box behavior without requiring
  /// configuration for simple `SQLite` setups.
  ///
  /// # Performance
  /// ~5-50ms on first call (workspace discovery + file I/O)
  /// <1µs on subsequent calls (all values cached)
  ///
  /// # Examples
  /// ```no_run
  /// use env::project::prelude::*;
  /// let project = Project::default();
  /// // If DATABASE_URL not set, project.configuration.db will be
  /// // something like "/home/user/project/assets/db"
  /// ```
  fn default() -> Self {
    let metadata = Metadata::default();
    let paths = Paths::default();
    let configuration = if Configuration::default().db.is_empty() {
      Configuration {
        db: paths.database.to_string_lossy().into_owned(),
        ..Default::default()
      }
    } else {
      Configuration::default()
    };

    Self {
      metadata,
      paths,
      configuration,
    }
  }
}

impl Environment {
  /// Creates a new default project environment.
  ///
  /// Equivalent to `Project::default()`. Use builder methods to customize.
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the project name, overriding the Cargo.toml value.
  ///
  /// # Examples
  /// ```no_run
  /// use env::project::prelude::*;
  /// let project = Project::new().with_name("custom-name");
  /// ```
  #[must_use]
  pub fn with_name(mut self, name: impl Into<String>) -> Self {
    self.metadata = self.metadata.with_name(name);
    self
  }

  /// Sets the project version, overriding the Cargo.toml value.
  ///
  /// # Examples
  /// ```no_run
  /// use env::project::prelude::*;
  /// let project = Project::new().with_version("2.0.0");
  /// ```
  #[must_use]
  pub fn with_version(mut self, version: impl Into<String>) -> Self {
    self.metadata = self.metadata.with_version(version);
    self
  }

  /// Sets the project description, overriding the Cargo.toml value.
  #[must_use]
  pub fn with_description(mut self, description: impl Into<String>) -> Self {
    self.metadata = self.metadata.with_description(description);
    self
  }

  /// Sets the database URL/path, overriding the `DATABASE_URL` environment variable.
  ///
  /// # Examples
  /// ```no_run
  /// use env::project::prelude::*;
  /// let project = Project::new().with_db("postgres://localhost/mydb");
  /// ```
  #[must_use]
  pub fn with_db(mut self, database_url: impl Into<String>) -> Self {
    self.configuration = self.configuration.with_db(database_url);
    self
  }

  /// Sets the server port, overriding the `PORT` environment variable.
  ///
  /// # Examples
  /// ```no_run
  /// use env::project::prelude::*;
  /// let project = Project::new().with_port(8080_u16);
  /// ```
  #[must_use]
  pub fn with_port(mut self, port: impl Into<u16>) -> Self {
    self.configuration = self.configuration.with_port(port);
    self
  }

  /// Sets the server bind IP address, overriding the `IP` environment variable.
  ///
  /// # Examples
  /// ```no_run
  /// use env::project::prelude::*;
  /// let project = Project::new().with_ip("0.0.0.0");
  /// ```
  #[must_use]
  pub fn with_ip(mut self, ip: impl Into<String>) -> Self {
    self.configuration = self.configuration.with_ip(ip);
    self
  }
}
