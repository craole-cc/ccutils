//! Environment access macro.

/// Provides access to initialized environment configuration and metadata.
///
/// Must be called after [`setenv!`](crate::prelude::setenv) has been invoked. Returns references to
/// nested environment data through various accessor patterns.
///
/// # Workspace Metadata Accessors
///
/// Access workspace-level metadata:
///
/// - `getenv!(ws_name)` → `&String` - Workspace name
/// - `getenv!(ws_version)` → `&String` - Workspace version
/// - `getenv!(ws_desc)` → `&String` - Workspace description
///
/// # Package Metadata Accessors
///
/// Access current package information:
///
/// - `getenv!(pkg_name)` → `&String` - Package name
/// - `getenv!(pkg_version)` → `&String` - Package version
/// - `getenv!(pkg_desc)` → `&String` - Package description
///
/// # Runtime Configuration Accessors
///
/// Access runtime configuration values:
///
/// - `getenv!(db)` → `&String` - Database URL/path
/// - `getenv!(ip)` → `&String` - Server bind IP address
/// - `getenv!(port)` → `&u16` - Server bind port
/// - `getenv!(rust_log)` → `&String` - `RUST_LOG` environment variable
///
/// # Filesystem Path Accessors
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
/// - `getenv!(workspace)` → `&Workspace` - Complete workspace structure
/// - `getenv!(package)` → `&Package` - Complete package structure
///
/// # Examples
///
/// ## Accessing Specific Fields
/// ```ignore
/// // Retrieve specific configuration
/// let app_name = getenv!(pkg_name);
/// let port = getenv!(port);
/// let db_config = getenv!(db);
///
/// println!("Starting {} on port {}", app_name, port);
/// println!("Database: {}", db_config);
/// ```
///
/// ## Accessing Paths
/// ```ignore
/// let project_root = getenv!(prj_path);
/// let assets = getenv!(assets_path);
///
/// println!("Project: {}", project_root.display());
/// println!("Assets: {}", assets.display());
/// ```
///
/// ## Accessing Complete Environment
/// ```ignore
/// let env = getenv!();
/// println!("{}", env.summary());
/// ```
///
/// # Return Types
///
/// All accessors return references (`&T`) to avoid cloning. The underlying
/// data is stored in a static `OnceLock`, so references are valid for the
/// entire program lifetime.
///
/// # Panics
///
/// Panics if called before [`setenv!`](crate::prelude::setenv) has been invoked.
/// Always initialize the environment at application startup.
///
/// # Implementation Note
///
/// This macro expands to calls to [`get()`](crate::prelude::get), which retrieves
/// the static environment instance. Each accessor then dereferences the
/// appropriate field path.
///
/// # See Also
///
/// - [`setenv!`](crate::prelude::setenv) - Initialize environment
/// - [`get()`](crate::prelude::get) - Core API for retrieving environment
#[macro_export]
macro_rules! getenv {
  () => {
    $crate::prelude::get()
  };

  // Workspace metadata accessors
  (ws_name) => {
    &$crate::prelude::get().workspace.metadata.name
  };
  (ws_version) => {
    &$crate::prelude::get().workspace.metadata.version
  };
  (ws_desc) => {
    &$crate::prelude::get().workspace.metadata.description
  };

  // Package metadata accessors
  (pkg_name) => {
    &$crate::prelude::get().package.metadata.name
  };
  (pkg_version) => {
    &$crate::prelude::get().package.metadata.version
  };
  (pkg_desc) => {
    &$crate::prelude::get().package.metadata.description
  };

  // Runtime configuration accessors
  (db) => {
    &$crate::prelude::get().config.db
  };
  (ip) => {
    &$crate::prelude::get().config.ip
  };
  (port) => {
    &$crate::prelude::get().config.port
  };
  (rust_log) => {
    &$crate::prelude::get().config.rust_log
  };

  // Filesystem path accessors
  (prj_path) => {
    &$crate::prelude::get().paths.project
  };
  (pkg_path) => {
    &$crate::prelude::get().paths.package
  };
  (assets_path) => {
    &$crate::prelude::get().paths.assets
  };
  (db_path) => {
    &$crate::prelude::get().paths.database
  };

  // Full structure access
  (workspace) => {
    &$crate::prelude::get().workspace
  };
  (package) => {
    &$crate::prelude::get().package
  };
}
