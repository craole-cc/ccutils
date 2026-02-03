//! Environment initialization and metadata access macros.
//!
//! This module provides macros for setting up project environment and accessing
//! configuration, package metadata, and project paths throughout the application lifecycle.

/// Initializes the project environment with package metadata.
///
/// Extracts metadata from compile-time cargo environment variables or accepts
/// a pre-configured [`Project`] object.
///
/// # Variants
///
/// ## No Arguments
///
/// ```ignore
/// setenv!();
/// ```
///
/// Automatically initializes using:
/// - `CARGO_PKG_NAME` - Current package name
/// - `CARGO_PKG_VERSION` - Current package version
/// - `CARGO_PKG_DESCRIPTION` - Current package description
///
/// ## With Environment Object
///
/// ```ignore
/// use craole_cc_project::*;
/// let prj = Project::new()
///   .with_pkg_name("my_app")
///   .with_pkg_version("1.0.0");
/// setenv!(prj);
/// ```
///
/// # Examples
///
/// ```ignore
/// // Typical usage at application startup
/// fn main() {
///     macros::setenv!();
///
///     let version = macros::getenv!(pkg_version);
///     println!("Running version: {}", version);
/// }
/// ```
///
/// # Panics
///
/// May panic if environment initialization fails or if called multiple times with conflicting data.
#[macro_export]
macro_rules! setenv {
  () => {
    craole_cc_project::prelude::set(
      craole_cc_project::prelude::Project::new()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
        .with_description(env!("CARGO_PKG_DESCRIPTION")),
    )
  };

  ($env:expr) => {
    craole_cc_project::prelude::set($prj)
  };
}

/// Provides access to initialized environment configuration and metadata.
///
/// Must be called after [`setenv!`] has been invoked. Returns references to
/// nested environment data through various accessor patterns.
///
/// # Project Metadata Accessors
///
/// Access project-level metadata:
///
/// - `getenv!(prj_name)` → `&String` - Project name
/// - `getenv!(prj_version)` → `&String` - Project version
/// - `getenv!(prj_desc)` → `&String` - Project description
///
/// # Package Metadata Accessors
///
/// Access current package information:
///
/// - `getenv!(pkg_name)` → `&String` - Package name
/// - `getenv!(pkg_version)` → `&String` - Package version
/// - `getenv!(pkg_desc)` → `&String` - Package description
///
/// # Project Configuration Accessors
///
/// Access runtime configuration values:
///
/// - `getenv!(db)` → `&DbConfig` - Database configuration
/// - `getenv!(ip)` → `&IpAddr` - Server IP address
/// - `getenv!(port)` → `&u16` - Server port number
/// - `getenv!(rust_log)` → `&String` - `RUST_LOG` environment variable
///
/// # Project Paths Accessors
///
/// Access filesystem paths:
///
/// - `getenv!(prj_path)` → `&PathBuf` - Project root directory
/// - `getenv!(pkg_path)` → `&PathBuf` - Current package directory
/// - `getenv!(assets_path)` → `&PathBuf` - Assets directory
/// - `getenv!(db_path)` → `&PathBuf` - Database directory
///
/// # Full Object Access
///
/// Access complete structures:
///
/// - `getenv!()` → `&Environment` - Full environment object
/// - `getenv!(workspace)` → `&WorkspaceConfig` - Complete project structure
/// - `getenv!(package)` → `&PackageConfig` - Complete package structure
///
/// # Examples
///
/// ```ignore
/// // Retrieve specific configuration
/// let app_name = getenv!(pkg_name);
/// let port = getenv!(port);
/// let db_config = getenv!(db);
///
/// // Retrieve complete project structure
/// let project = getenv!(project);
///
/// // Access paths
/// let project_root = getenv!(prj_path);
/// let assets = getenv!(assets_path);
/// ```
///
/// # Panics
///
/// Panics if called before [`setenv!`] has been invoked, or if requested
/// configuration keys don't exist in the environment.
#[macro_export]
macro_rules! getenv {
  () => {
    craole_cc_project::prelude::get()
  };

  // Workspace metadata accessors
  (ws_name) => {
    &craole_cc_project::prelude::get().workspace.metadata.name
  };
  (ws_version) => {
    &craole_cc_project::prelude::get().workspace.metadata.version
  };
  (ws_desc) => {
    &craole_cc_project::prelude::get()
      .workspace
      .metadata
      .description
  };

  // Package metadata accessors
  (pkg_name) => {
    &craole_cc_project::prelude::get().package.metadata.name
  };
  (pkg_version) => {
    &craole_cc_project::prelude::get().package.metadata.version
  };
  (pkg_desc) => {
    &craole_cc_project::prelude::get()
      .package
      .metadata
      .description
  };

  // Project configuration accessors
  (db) => {
    &craole_cc_project::prelude::get().workspace.configuration.db
  };
  (ip) => {
    &craole_cc_project::prelude::get().workspace.configuration.ip
  };
  (port) => {
    &craole_cc_project::prelude::get()
      .workspace
      .configuration
      .port
  };
  (rust_log) => {
    &craole_cc_project::prelude::get()
      .workspace
      .configuration
      .rust_log
  };

  // Project paths accessors
  (prj_path) => {
    &craole_cc_project::prelude::get().workspace.paths.project
  };
  (pkg_path) => {
    &craole_cc_project::prelude::get().workspace.paths.package
  };
  (assets_path) => {
    &craole_cc_project::prelude::get().workspace.paths.assets
  };
  (db_path) => {
    &craole_cc_project::prelude::get().workspace.paths.database
  };

  // Full environment access
  (workspace) => {
    &craole_cc_project::prelude::get().workspace
  };
  (package) => {
    &craole_cc_project::prelude::get().package
  };
}
