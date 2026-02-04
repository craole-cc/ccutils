//! Global environment management and static initialization.
//!
//! This module provides thread-safe, static access to application environment configuration
//! through an `OnceLock`-backed global instance. All environment data is initialized once
//! and cached for the lifetime of the application.
//!
//! # Architecture
//!
//! The `Environment` struct contains:
//! - **Kind**: Environment operation mode (Workspace/Standalone/Library)
//! - **Workspace**: Workspace root configuration (metadata, paths, server config)
//! - **Package**: Current running crate metadata (name, version, description)
//!
//! # Initialization Strategies
//!
//! ## Option 1: Default Initialization (Auto-detect)
//! ```no_run
//! use prjenv::prelude::*;
//! let prj = get(); // Auto-detects kind, initializes with defaults
//! ```
//!
//! ## Option 2: Explicit Mode
//! ```no_run
//! use prjenv::prelude::*;
//! let prj = set(
//!   Environment::workspace()
//!     .with_name(env!("CARGO_PKG_NAME"))
//!     .with_version(env!("CARGO_PKG_VERSION"))
//!     .with_description(env!("CARGO_PKG_DESCRIPTION")),
//! );
//! ```
//!
//! ## Option 3: Macro-Based (with `macros` feature)
//! ```no_run
//! # #[cfg(feature = "macros")]
//! # {
//! use prjenv::prelude::*;
//! setenv!(); // Auto-populates from env! macros in calling crate
//!
//! # }
//! ```
//!
//! # Thread Safety
//!
//! Both `get()` and `set()` use `OnceLock::get_or_init()`, which is:
//! - **Thread-safe**: Multiple threads calling simultaneously will safely coordinate initialization
//! - **Idempotent**: Calling `set()` multiple times always returns the first set value
//! - **Zero-cost**: After initialization, access is just a pointer dereference
//!
//! # Lifetime
//!
//! Returns `&'static Project` - valid for the entire program lifetime. Perfect for use
//! in macros like `getenv!()` which need static references.

use crate::prelude::*;

/// Global static environment instance, protected by `OnceLock` for safe one-time initialization
pub static ENV: OnceLock<Environment> = OnceLock::new();

/// Retrieve the cached global environment.
///
/// On first call, initializes the environment with defaults by calling `Environment::new()`.
/// Subsequent calls return the cached instance immediately.
///
/// # Behavior
/// - **First call**: Loads project metadata from workspace Cargo.toml, reads env vars, discovers paths
/// - **Subsequent calls**: Returns cached `&'static Environment` (instant)
/// - **Thread-safe**: Multiple threads coordinate safely via `OnceLock`
///
/// # Returns
/// `&'static Environment` - valid for program lifetime
///
/// # Examples
/// ```no_run
/// use prjenv::prelude::*;
/// let prj = get();
/// println!("Workspace: {}", prj.workspace.metadata.name);
/// println!("Package: {}", prj.package.metadata.name);
/// ```
pub fn get() -> &'static Environment {
  // Run only once in the top-level environment.
  dotenv().ok();
  ENV.get_or_init(Environment::new)
}

/// Initialize the global environment with a custom `Environment` instance.
///
/// Stores the provided environment in the static `OnceLock`. If already initialized,
/// returns the previously cached instance (idempotent).
///
/// **Must be called before `get()`** for the custom environment to be used.
/// Subsequent calls to `get()` will return the first value passed here.
///
/// # Parameters
/// - `prj` - Pre-configured `Environment` with desired metadata and configuration
///
/// # Returns
/// `&'static Environment` - the initialized environment (yours on first call, cached on subsequent)
///
/// # Thread Safety
/// Safe to call from multiple threads; only the first caller's value is stored.
///
/// # Examples
/// ```no_run
/// use prjenv::prelude::*;
/// // Configure before any other code uses get()
/// set(
///   Environment::new()
///     .with_pkg_name(env!("CARGO_PKG_NAME"))
///     .with_pkg_version(env!("CARGO_PKG_VERSION"))
///     .with_pkg_description(env!("CARGO_PKG_DESCRIPTION")),
/// );
///
/// // Now get() returns our configuration
/// let prj = get();
/// println!("Package: {}", prj.package.metadata.name);
/// ```
pub fn set(env: Environment) -> &'static Environment {
  dotenv().ok();
  ENV.get_or_init(|| env)
}

pub fn try_get() -> Option<&'static Environment> {
  ENV.get()
}

/// Top-level application environment container.
///
/// Combines domain models (workspace, package) with infrastructure (config, paths).
///
/// # Fields
/// - `kind` - Environment operation mode
/// - `workspace` - Workspace domain model (metadata, packages)
/// - `package` - Current package domain model (metadata)
/// - `config` - Runtime configuration (database, server settings)
/// - `paths` - Filesystem paths (workspace root, assets, etc.)
///
/// # Builder Pattern
/// All `with_*` methods return `Self` for method chaining:
/// ```no_run
/// use prjenv::prelude::*;
/// let env = Environment::new()
///   .with_pkg_name("my-app")
///   .with_pkg_version("1.0.0")
///   .with_port(8080)
///   .with_db("postgres://localhost/mydb");
/// ```
#[derive(Debug, Clone)]
pub struct Environment {
  /// Environment operation mode
  pub kind: Kind,

  // ═══ DOMAIN ═══
  /// Workspace domain model
  pub workspace: Workspace,

  /// Current package domain model
  pub package: Package,

  // ═══ INFRASTRUCTURE ═══
  /// Runtime configuration (env vars)
  pub config: Configuration,

  /// Filesystem paths (discovered at runtime)
  pub paths: Paths,
}

impl Default for Environment {
  fn default() -> Self {
    let kind = Kind::detect();
    let workspace = Workspace::default();
    let package = Package::default();
    let paths = Paths::default();

    // Special handling: if DATABASE_URL is empty, use workspace/assets/db
    let mut config = Configuration::new();
    if config.db.is_empty() {
      config.db = paths.database.to_string_lossy().into_owned();
    }

    Self {
      kind,
      workspace,
      package,
      config,
      paths,
    }
  }
}

impl Environment {
  /// Creates a new default environment.
  ///
  /// Equivalent to `Environment::default()`. Use builder methods to customize.
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  /// Creates a workspace environment explicitly.
  #[must_use]
  pub fn workspace() -> Self {
    Self {
      kind: Kind::Workspace,
      ..Default::default()
    }
  }

  /// Creates a standalone package environment explicitly.
  #[must_use]
  pub fn standalone() -> Self {
    Self {
      kind: Kind::Standalone,
      ..Default::default()
    }
  }

  /// Creates a library-only environment (no filesystem access).
  #[must_use]
  pub fn library() -> Self {
    Self {
      kind: Kind::Library,
      ..Default::default()
    }
  }

  //╔═══════════════════════════════════════════════════════════╗
  //║ Package Shortcuts (convenience)                          ║
  //╚═══════════════════════════════════════════════════════════╝

  /// Sets the package name (shortcut for `with_pkg_name`).
  #[must_use]
  pub fn with_name(mut self, name: impl Into<String>) -> Self {
    self.package = self.package.with_name(name);
    self
  }

  /// Sets the package version (shortcut for `with_pkg_version`).
  #[must_use]
  pub fn with_version(mut self, version: impl Into<String>) -> Self {
    self.package = self.package.with_version(version);
    self
  }

  /// Sets the package description (shortcut for `with_pkg_description`).
  #[must_use]
  pub fn with_description(mut self, description: impl Into<String>) -> Self {
    self.package = self.package.with_description(description);
    self
  }

  //╔═══════════════════════════════════════════════════════════╗
  //║ Infrastructure Builders                                   ║
  //╚═══════════════════════════════════════════════════════════╝

  /// Sets the database URL/path.
  ///
  /// # Examples
  /// ```no_run
  /// use prjenv::prelude::*;
  /// let env = Environment::new().with_db("postgres://localhost/mydb");
  /// ```
  #[must_use]
  pub fn with_db(mut self, database_url: impl Into<String>) -> Self {
    self.config = self.config.with_db(database_url);
    self
  }

  /// Sets the server port.
  ///
  /// # Examples
  /// ```no_run
  /// use prjenv::prelude::*;
  /// let env = Environment::new().with_port(8080);
  /// ```
  #[must_use]
  pub fn with_port<P>(mut self, port: P) -> Self
  where
    P: TryInto<u16>,
    <P as TryInto<u16>>::Error: Debug,
  {
    self.config = self.config.with_port(port);
    self
  }

  /// Sets the server bind IP address.
  ///
  /// # Examples
  /// ```no_run
  /// use prjenv::prelude::*;
  /// let env = Environment::new().with_ip("0.0.0.0");
  /// ```
  #[must_use]
  pub fn with_ip(mut self, ip: impl Into<String>) -> Self {
    self.config = self.config.with_ip(ip);
    self
  }

  //╔═══════════════════════════════════════════════════════════╗
  //║ Workspace Delegation                                      ║
  //╚═══════════════════════════════════════════════════════════╝

  /// Sets the workspace name.
  #[must_use]
  pub fn with_workspace_name(mut self, name: impl Into<String>) -> Self {
    self.workspace = self.workspace.with_name(name);
    self
  }

  /// Sets the workspace version.
  #[must_use]
  pub fn with_workspace_version(mut self, version: impl Into<String>) -> Self {
    self.workspace = self.workspace.with_version(version);
    self
  }

  /// Sets the workspace description.
  #[must_use]
  pub fn with_workspace_description(mut self, description: impl Into<String>) -> Self {
    self.workspace = self.workspace.with_description(description);
    self
  }

  /// Adds a package to the workspace.
  #[must_use]
  pub fn with_workspace_package(mut self, package: Package) -> Self {
    self.workspace = self.workspace.with_package(package);
    self
  }

  /// Adds a package by name to the workspace.
  #[must_use]
  pub fn with_workspace_package_name(mut self, name: impl Into<String>) -> Self {
    self.workspace = self.workspace.with_package_name(name);
    self
  }

  //╔═══════════════════════════════════════════════════════════╗
  //║ Package Delegation                                        ║
  //╚═══════════════════════════════════════════════════════════╝

  /// Sets the package name (for the currently running crate).
  ///
  /// # Examples
  /// ```no_run
  /// use prjenv::prelude::*;
  ///
  /// let env = Environment::new()
  ///   .with_pkg_name(env!("CARGO_PKG_NAME"))
  ///   .with_pkg_version(env!("CARGO_PKG_VERSION"));
  /// ```
  #[must_use]
  pub fn with_pkg_name(mut self, name: impl Into<String>) -> Self {
    self.package = self.package.with_name(name);
    self
  }

  /// Sets the package version.
  #[must_use]
  pub fn with_pkg_version(mut self, version: impl Into<String>) -> Self {
    self.package = self.package.with_version(version);
    self
  }

  /// Sets the package description.
  #[must_use]
  pub fn with_pkg_description(mut self, description: impl Into<String>) -> Self {
    self.package = self.package.with_description(description);
    self
  }

  /// Returns a formatted summary of the environment.
  #[must_use]
  pub fn summary(&self) -> String {
    match self.kind {
      Kind::Workspace => {
        format!(
          "{} ({} packages, running {})",
          self.workspace.metadata.display_name(),
          self.workspace.package_count(),
          self.package.metadata.display_name()
        )
      }
      Kind::Standalone => self.package.metadata.display_name(),
      Kind::Library => format!("Library: {}", self.package.metadata.display_name()),
    }
  }
}
