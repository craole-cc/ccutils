//! Project directory structure and path management.
//!
//! Provides a centralized struct for accessing all important directories within the project,
//! automatically discovered from the workspace root.
//!
//! # Path Structure
//!
//! All paths are discovered relative to the workspace root:
//! ```text
//! {workspace_root}/
//! ├── Cargo.toml
//! ├── assets/
//! │   └── db/
//! ├── src/
//! └── ...
//! ```
//!
//! # Examples
//!
//! ```no_run
//! use env::project::prelude::*;
//!
//! let paths = ProjectPaths::default();
//! println!("Project root: {}", paths.project.display());
//! println!("Assets: {}", paths.assets.display());
//! println!("Database: {}", paths.database.display());
//! ```

use super::super::_prelude::*;

/// Project directory structure and path management.
///
/// Contains references to all important directories for the application.
/// All paths are discovered automatically from the workspace root via `find_project_path()`.
///
/// # Fields
/// - `project` - Workspace root directory (contains Cargo.toml)
/// - `package` - Currently set to same as project (can be customized if needed)
/// - `assets` - Static assets directory ({project}/assets)
/// - `database` - Database files directory ({project}/assets/db)
///
/// # Defaults
/// All paths are discovered from the workspace root on first access.
/// To customize, create a `Paths` instance and modify fields before use.
///
/// # Thread Safety
/// Safe to clone and share; all fields are `PathBuf` types.
///
/// # Examples
/// ```no_run
/// use {
///   env::project::prelude::*,
///   std::path::PathBuf,
/// };
///
/// let mut paths = ProjectPaths::default();
/// println!("Assets: {}", paths.assets.display());
///
/// // Customize if needed
/// paths.database = PathBuf::from("/custom/db/path");
/// ```
#[derive(Debug, Clone)]
pub struct Paths {
  /// Workspace root directory (contains workspace Cargo.toml).
  ///
  /// Discovered via `find_project_path()` which walks up from the current location
  /// or uses environment variable overrides.
  ///
  /// This is the definitive project root for all other path calculations.
  pub project: PathBuf,

  /// Package directory (typically same as project).
  ///
  /// Currently defaults to the same value as `project`. Provided for future flexibility
  /// in cases where the package and project might differ (e.g., monorepos with
  /// different package roots).
  pub package: PathBuf,

  /// Static assets directory.
  ///
  /// Standard location: `{project}/assets`
  ///
  /// Used for storing static files, configuration files, and other assets
  /// bundled with the application.
  pub assets: PathBuf,

  /// Database files directory.
  ///
  /// Standard location: `{project}/assets/db`
  ///
  /// Used for `SQLite` databases and other file-based database storage.
  /// Falls back to this path if `DATABASE_URL` environment variable is not set.
  ///
  /// # Note
  /// The actual database location can be overridden via `DATABASE_URL` env var
  /// in `Configuration::db`.
  pub database: PathBuf,
}

impl Default for Paths {
  /// Creates paths by discovering the workspace root and deriving standard locations.
  ///
  /// # Process
  /// 1. Call `find_project_path()` to locate workspace root
  /// 2. Set `package` to same as `project` (can be customized)
  /// 3. Derive `assets` = `project/assets`
  /// 4. Derive `database` = `assets/db`
  ///
  /// # Performance
  /// ~5-50ms on first call (workspace discovery)
  /// <1µs on subsequent calls (caching in `find_project_path`)
  ///
  /// # Examples
  /// ```no_run
  /// use env::project::prelude::*;
  /// let paths = ProjectPaths::default();
  /// assert!(paths.assets.ends_with("assets"));
  /// assert!(paths.database.ends_with("db"));
  /// ```
  fn default() -> Self {
    let project = find_project_path();
    let package = project.clone();
    let assets = project.join("assets");
    let database = assets.join("db");

    Self {
      project,
      package,
      assets,
      database,
    }
  }
}
