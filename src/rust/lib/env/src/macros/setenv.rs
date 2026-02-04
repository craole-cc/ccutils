//! Environment initialization macro.

/// Initializes the project environment with package metadata.
///
/// Extracts metadata from compile-time cargo environment variables or accepts
/// a pre-configured [`Environment`](crate::prelude::Environment) object.
///
/// # Variants
///
/// ## No Arguments - Auto-Initialize
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
/// ## With Environment Object - Custom Initialization
///
/// ```ignore
/// use prjenv::*;
/// let env = Environment::new()
///   .with_pkg_name("my_app")
///   .with_pkg_version("1.0.0");
/// setenv!(env);
/// ```
///
/// # Idempotency
///
/// This macro (and the underlying [`set()`](crate::prelude::set) function) is idempotent.
/// Only the **first call** sets the environment; subsequent calls return the
/// already-initialized value.
///
/// ```ignore
/// setenv!(); // Initializes
/// setenv!(); // Returns existing, doesn't reinitialize
/// ```
///
/// # Examples
///
/// ## Typical Application Startup
/// ```ignore
/// fn main() {
///     // Initialize environment at startup
///     prjenv::setenv!();
///
///     // Access throughout the app
///     let version = prjenv::getenv!(pkg_version);
///     println!("Running version: {}", version);
/// }
/// ```
///
/// ## With Custom Configuration
/// ```ignore
/// fn main() {
///     let env = Environment::new()
///       .with_pkg_name(env!("CARGO_PKG_NAME"))
///       .with_pkg_version(env!("CARGO_PKG_VERSION"))
///       .with_port(8080)
///       .with_db("postgres://localhost/mydb");
///
///     setenv!(env);
///
///     let port = getenv!(port);
///     println!("Server running on port {}", port);
/// }
/// ```
///
/// # Implementation Note
///
/// This macro expands to a call to [`set()`](crate::prelude::set), which uses an
/// `OnceLock` for thread-safe, one-time initialization.
///
/// # See Also
///
/// - [`getenv!`](crate::prelude::getenv) - Access initialized environment
/// - [`get()`](crate::prelude::get) - Core API for retrieving environment
/// - [`set()`](crate::prelude::set) - Core API for setting environment
#[macro_export]
macro_rules! setenv {
  () => {
    $crate::prelude::set(
      $crate::prelude::Environment::new()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
        .with_description(env!("CARGO_PKG_DESCRIPTION")),
    )
  };

  ($env:expr) => {
    $crate::prelude::set($env)
  };
}
