//! Server and runtime configuration from environment variables.
//!
//! Provides centralized configuration for server binding, database connections,
//! and logging filters, all loaded from environment variables with sensible defaults.
//!
//! # Environment Variables
//!
//! | Variable | Type | Default | Purpose |
//! |----------|------|---------|---------|
//! | `DATABASE_URL` | String | (auto-set to `{workspace}/assets/db`) | Database connection URL or file path |
//! | `IP` | String | `localhost` | Server bind address (use `0.0.0.0` for production) |
//! | `PORT` | u16 | `3000` | Server bind port (must be valid u16, panics if invalid) |
//! | `RUST_LOG` | String | (empty by default) | Tracing filter directives |
//!
//! # Examples
//!
//! ## Runtime Override
//! ```bash
//! # Override configuration at runtime
//! PORT=8080 IP=0.0.0.0 DATABASE_URL="postgres://user:pass@localhost/mydb" RUST_LOG=debug ./app
//! ```
//!
//! ## Programmatic Access
//! ```no_run
//! use craole_cc_project::prelude::*;
//!
//! let env = get();
//! let port = env.config.port;
//! let db = &env.config.db;
//! let ip = &env.config.ip;
//! println!("Server: {}:{}", ip, port);
//! println!("Database: {}", db);
//! ```
//!
//! ## Builder Pattern
//! ```no_run
//! use craole_cc_project::infrastructure::*;
//!
//! let config = Configuration::new()
//!   .with_port(8080)
//!   .with_ip("0.0.0.0")
//!   .with_db("sqlite:///data/app.db");
//! ```

use crate::_prelude::*;

/// Server and runtime configuration loaded from environment variables.
///
/// Contains all settings needed for the running application:
/// - **Database**: Connection URL or file path
/// - **Server**: Bind IP and port
/// - **Logging**: Tracing filter directives
///
/// All fields have sensible defaults that apply if the corresponding environment
/// variable is not set. Use the builder pattern (`with_*` methods) to override
/// specific fields after construction.
///
/// # Defaults
/// - `db`: Empty (fallback to `{workspace}/assets/db` handled by `Environment::default`)
/// - `ip`: "localhost" (suitable for development)
/// - `port`: 3000 (IANA registered for commonly used services)
/// - `rust_log`: Empty (use env var or builder to set)
///
/// # Thread Safety
/// Can be cloned safely; all fields are `String` or primitive types.
///
/// # Validation
/// - **Port must be u16**: If `PORT` env var is set, it must parse as a valid u16 (0-65535).
///   Invalid values cause a panic in `Default::default()` to fail fast.
/// - **Other fields**: No validation; malformed URLs/IPs are stored as-is and
///   will fail at connection time (preferable to failing at startup).
///
/// # Examples
/// ```no_run
/// use craole_cc_project::infrastructure::*;
///
/// let config = Configuration::default();
/// println!(
///   "Connecting to {} at {}:{}",
///   config.db, config.ip, config.port
/// );
/// ```
#[derive(Debug, Clone, Default)]
pub struct Configuration {
  /// Database URL or file path.
  ///
  /// Set via `DATABASE_URL` environment variable.
  /// If empty, the `Environment` struct will fall back to `{workspace}/assets/db`.
  ///
  /// # Common Values
  /// - **SQLite** (file-based):
  ///   - `sqlite:///data/app.db` (absolute path)
  ///   - `sqlite://./data/app.db` (relative path)
  ///   - `/var/lib/app/app.db` (simple file path)
  ///
  /// - **PostgreSQL**:
  ///   - `postgres://user:password@localhost/dbname`
  ///   - `postgresql://user:password@host:5432/dbname`
  ///
  /// - **MySQL**:
  ///   - `mysql://user:password@localhost/dbname`
  ///   - `mysql://user:password@host:3306/dbname`
  ///
  /// # Connection Failures
  /// Invalid URLs don't cause startup panic; the failure occurs when the application
  /// attempts to connect, allowing environment misconfigurations to be caught at
  /// runtime (appropriate for deployed applications).
  pub db: String,

  /// Server bind IP address.
  ///
  /// Set via `IP` environment variable (default: "localhost").
  ///
  /// # Common Values
  /// - **"localhost"** or **"127.0.0.1"** (IPv4 loopback, development only)
  ///   - Accessible only from this machine
  ///   - Safe for development without network isolation
  ///
  /// - **"0.0.0.0"** (all interfaces, production)
  ///   - Binds to all IPv4 addresses
  ///   - Required for production behind reverse proxies
  ///   - Accessible from network (rely on firewall rules)
  ///
  /// - **"::1"** (IPv6 loopback)
  ///   - IPv6 equivalent of 127.0.0.1
  ///
  /// - **"::"** (all IPv6 interfaces)
  ///   - IPv6 equivalent of 0.0.0.0 (may require dual-stack config)
  pub ip: String,

  /// Server bind port.
  ///
  /// Set via `PORT` environment variable (default: 3000).
  /// Must be valid u16 (0-65535).
  ///
  /// # Common Values
  /// - **Development**: 3000, 8000, 8080, 5000
  /// - **Production**: 80 (HTTP), 443 (HTTPS)
  /// - **System services**: 1-1023 (require root/admin)
  /// - **User services**: 1024-65535 (unprivileged)
  ///
  /// # Port Binding Failures
  /// If the port is already in use or requires elevated privileges, the bind will
  /// fail at startup (appropriate to fail fast during setup).
  ///
  /// # Validation
  /// If `PORT` environment variable is set but not a valid u16, `Default::default()`
  /// panics with "PORT must be a valid number". This is intentional - port configuration
  /// errors should fail immediately during application startup.
  pub port: u16,

  /// Tracing filter directives for the `RUST_LOG` subscriber.
  ///
  /// Set via `RUST_LOG` environment variable.
  /// Defines which modules log at which levels, controlling verbosity per component.
  ///
  /// # Default
  /// Empty string by default. Set via `RUST_LOG` env var or builder methods.
  ///
  /// # Format: Comma-Separated Directives
  /// Each directive is `target=level`:
  /// - `target` - Module/crate name or `tokio`, `hyper`, etc.
  /// - `level` - TRACE, DEBUG, INFO, WARN, ERROR
  ///
  /// # Examples
  /// - **Development (verbose)**:
  ///   ```text
  ///   RUST_LOG=trace
  ///   ```
  ///   - All modules at TRACE level (very verbose)
  ///
  /// - **Development (specific crate)**:
  ///   ```text
  ///   RUST_LOG=myapp=debug,tokio=warn
  ///   ```
  ///   - Your app at DEBUG, tokio at WARN
  ///
  /// - **Production (info only)**:
  ///   ```text
  ///   RUST_LOG=info
  ///   ```
  ///   - All modules at INFO level (less verbose)
  ///
  /// - **Production (conservative)**:
  ///   ```text
  ///   RUST_LOG=warn
  ///   ```
  ///   - Only warnings and errors
  ///
  /// # Format Specification
  /// See `tracing_subscriber::filter::EnvFilter` for detailed syntax:
  /// - `target=level` - Single directive
  /// - `target::module=level` - Path-specific
  /// - `target/file.rs=level` - Source file specific
  /// - Multiple: comma-separated
  ///
  /// # Parsing Errors
  /// Invalid directives (typos, unknown levels) are logged as warnings and
  /// fallback to environment defaults. Non-fatal to avoid startup failures.
  pub rust_log: String,
}

impl Configuration {
  /// Creates configuration from environment variables with built-in defaults.
  ///
  /// # Reading Order
  /// 1. `RUST_LOG` → defaults to empty
  /// 2. `DATABASE_URL` → defaults to empty (fallback handled by Environment)
  /// 3. `IP` → defaults to "localhost"
  /// 4. `PORT` → defaults to "3000", parsed as u16
  ///
  /// # Panics
  /// If `PORT` environment variable is set but cannot be parsed as u16.
  /// This is intentional - invalid port configuration should fail immediately
  /// during startup rather than at first connection attempt.
  ///
  /// # Examples
  /// ```no_run
  /// use craole_cc_project::infrastructure::*;
  /// let config = Configuration::new();
  /// // If PORT="invalid", this panics with "PORT must be a valid number"
  /// ```
  ///
  /// # Performance
  /// ~1ms (reads 4 environment variables, parses one u16)
  #[must_use]
  pub fn new() -> Self {
    let rust_log = var("RUST_LOG").unwrap_or_default();
    let db = var("DATABASE_URL").unwrap_or_default();
    let ip = var("IP").unwrap_or_else(|_| String::from("localhost"));
    let port = var("PORT")
      .unwrap_or_else(|_| String::from("3000"))
      .parse::<u16>()
      .expect("PORT must be a valid number");

    Self {
      db,
      ip,
      port,
      rust_log,
    }
  }

  //╔═══════════════════════════════════════════════════════════╗
  //║ Builders                                                  ║
  //╚═══════════════════════════════════════════════════════════╝

  /// Sets the database URL/path, overriding the `DATABASE_URL` environment variable.
  ///
  /// # Examples
  /// ```no_run
  /// use craole_cc_project::infrastructure::*;
  ///
  /// // Use PostgreSQL in production
  /// let config = Configuration::new().with_db("postgres://user:pass@db.example.com/myapp");
  ///
  /// // Or keep SQLite in development
  /// let config = Configuration::new().with_db("sqlite:///./data/app.db");
  /// ```
  #[must_use]
  pub fn with_db(mut self, database_url: impl Into<String>) -> Self {
    self.db = database_url.into();
    self
  }

  /// Sets the server port, overriding the `PORT` environment variable.
  ///
  /// Accepts any integer type that can be converted to `u16`.
  ///
  /// # Examples
  /// ```no_run
  /// use craole_cc_project::infrastructure::*;
  ///
  /// // Integer literals (inferred as i32) work automatically
  /// let config = Configuration::new().with_port(3000);
  /// let config = Configuration::new().with_port(8080);
  ///
  /// // Explicit types also work
  /// let config = Configuration::new().with_port(3000_u16);
  /// let config = Configuration::new().with_port(80u8);
  /// ```
  ///
  /// # Panics
  /// Panics if the value is outside u16 range (0-65535).
  #[must_use]
  pub fn with_port<P>(mut self, port: P) -> Self
  where
    P: TryInto<u16>,
    <P as TryInto<u16>>::Error: std::fmt::Debug,
  {
    self.port = port.try_into().expect("Port must be 0-65535");
    self
  }

  /// Sets the server bind IP address, overriding the `IP` environment variable.
  ///
  /// # Examples
  /// ```no_run
  /// use craole_cc_project::infrastructure::*;
  ///
  /// // Development (localhost only)
  /// let config = Configuration::new().with_ip("localhost");
  ///
  /// // Production (all interfaces, with firewall rules)
  /// let config = Configuration::new().with_ip("0.0.0.0");
  ///
  /// // Docker container (all interfaces)
  /// let config = Configuration::new().with_ip("0.0.0.0").with_port(3000);
  ///
  /// // IPv6
  /// let config = Configuration::new().with_ip("::1"); // IPv6 loopback
  /// ```
  ///
  /// # Recommended Values
  /// - **Development**: "localhost" (safe, no network access)
  /// - **Docker**: "0.0.0.0" (expose to bridge network)
  /// - **Production**: "0.0.0.0" (behind reverse proxy/firewall)
  /// - **Testing**: "127.0.0.1" (explicit IPv4 loopback)
  #[must_use]
  pub fn with_ip(mut self, ip: impl Into<String>) -> Self {
    self.ip = ip.into();
    self
  }

  /// Sets the RUST_LOG filter directives.
  ///
  /// # Examples
  /// ```no_run
  /// use craole_cc_project::infrastructure::*;
  ///
  /// let config = Configuration::new().with_rust_log("debug");
  ///
  /// let config = Configuration::new().with_rust_log("myapp=trace,tokio=warn");
  /// ```
  #[must_use]
  pub fn with_rust_log(mut self, rust_log: impl Into<String>) -> Self {
    self.rust_log = rust_log.into();
    self
  }
}
