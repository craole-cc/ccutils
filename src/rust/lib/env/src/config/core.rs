//! Global environment management and static initialization.
//!
//! This module provides thread-safe, static access to application environment configuration
//! through an `OnceLock`-backed global instance. All environment data is initialized once
//! and cached for the lifetime of the application.
//!
//! # Architecture
//!
//! The `Environment` struct contains two sub-environments:
//! - **Project**: Workspace root configuration (metadata, paths, server config)
//! - **Package**: Current running crate metadata (name, version, description)
//!
//! # Initialization Strategies
//!
//! ## Option 1: Default Initialization
//! ```no_run
//! use env::prelude::*;
//! let env = get_env(); // Initializes with defaults, cached forever
//! ```
//!
//! ## Option 2: Custom Initialization
//! ```no_run
//! use env::prelude::*;
//! let env = set_env(
//!   Environment::new()
//!     .with_pkg_name(env!("CARGO_PKG_NAME"))
//!     .with_pkg_version(env!("CARGO_PKG_VERSION"))
//!     .with_pkg_description(env!("CARGO_PKG_DESCRIPTION")),
//! );
//! ```
//!
//! ## Option 3: Macro-Based (with `macros` feature)
//! ```no_run
//! # #[cfg(feature = "macros")]
//! # {
//! use env::prelude::*;
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
//! Returns `&'static Environment` - valid for the entire program lifetime. Perfect for use
//! in macros like `getenv!()` which need static references.

use super::_prelude::*;

pub mod prelude {
  pub use super::{
    ENV,
    Environment,
    // get,
    // init,
    // try_get,
    get as get_env,
    init as init_env,
  };
}

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
/// use env::prelude::*;
/// let env = get_env();
/// println!("Project: {}", env.project.metadata.name);
/// println!("Package: {}", env.package.metadata.name);
/// ```
pub fn get() -> &'static Environment {
  // Run only once in the top-level environment.
  dotenv().ok();
  (ENV.get_or_init(Environment::new)) as _
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
/// - `env` - Pre-configured `Environment` with desired metadata and configuration
///
/// # Returns
/// `&'static Environment` - the initialized environment (yours on first call, cached on subsequent)
///
/// # Thread Safety
/// Safe to call from multiple threads; only the first caller's value is stored.
///
/// # Examples
/// ```no_run
/// use env::prelude::*;
/// // Configure before any other code uses get_env()
/// set_env(
///   Environment::new()
///     .with_pkg_name(env!("CARGO_PKG_NAME"))
///     .with_pkg_version(env!("CARGO_PKG_VERSION"))
///     .with_pkg_description(env!("CARGO_PKG_DESCRIPTION")),
/// );
///
/// // Now get_env() returns our configuration
/// let pkg_name = getenv!(pkg_name);
/// ```
pub fn init(env: Environment) -> &'static Environment {
  dotenv().ok();
  ENV.get_or_init(|| env)
}

// pub fn try_get() -> Option<&'static Environment> {
//   ENV.get()
// }

/// Top-level application environment container.
///
/// Combines project-level configuration (workspace, paths, server config)
/// with package-level metadata (current crate info).
///
/// # Fields
/// - `project` - Workspace configuration: metadata, paths, server settings
/// - `package` - Running package metadata: name, version, description
///
/// # Builder Pattern
/// All `with_*` methods return `Self` for method chaining:
/// ```no_run
/// use env::prelude::*;
/// let env = Environment::new()
///   .with_pkg_name("my-app")
///   .with_pkg_version("1.0.0")
///   .with_port(8080);
/// ```
#[derive(Default, Debug, Clone)]
pub struct Environment {
  pub project: Project,
  pub package: Package,
}

impl Environment {
  /// Creates a new default environment.
  ///
  /// Equivalent to `Environment::default()`. Use builder methods to customize.
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the project name (stored in package metadata).
  ///
  /// **Note:** This sets package name, not project name. For the actual project/workspace name,
  /// modify `env.project.metadata.name` directly or use workspace-level builder methods.
  #[must_use]
  pub fn with_prj_name(mut self, name: impl Into<String>) -> Self {
    self.package = self.package.with_name(name);
    self
  }

  /// Sets the project version (stored in package metadata).
  #[must_use]
  pub fn with_prj_version(mut self, version: impl Into<String>) -> Self {
    self.package = self.package.with_version(version);
    self
  }

  /// Sets the project description (stored in package metadata).
  #[must_use]
  pub fn with_prj_description(mut self, description: impl Into<String>) -> Self {
    self.package = self.package.with_description(description);
    self
  }

  /// Sets the database URL/path for the project.
  ///
  /// Used to override the default database location (`{workspace}/assets/db`).
  #[must_use]
  pub fn with_db(mut self, database_url: impl Into<String>) -> Self {
    self.project = self.project.with_db(database_url);
    self
  }

  /// Sets the server port.
  ///
  /// Overrides the `PORT` environment variable (default: 3000).
  #[must_use]
  pub fn with_port(mut self, port: impl Into<u16>) -> Self {
    self.project = self.project.with_port(port);
    self
  }

  /// Sets the server bind IP address.
  ///
  /// Overrides the `IP` environment variable (default: "localhost").
  #[must_use]
  pub fn with_ip(mut self, ip: impl Into<String>) -> Self {
    self.project = self.project.with_ip(ip);
    self
  }

  /// Sets the package name (current running crate).
  ///
  /// This is the name of the binary/library being executed, distinct from workspace name.
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
}
