//! Project metadata loaded from workspace Cargo.toml.
//!
//! Reads and caches package metadata (name, version, description) from the workspace
//! root's Cargo.toml file. Handles both workspace-level `[workspace.package]` and
//! regular package `[package]` sections automatically.
//!
//! # Caching
//!
//! Metadata is cached in a static `OnceLock` to avoid repeated file I/O.
//! First access reads and parses the Cargo.toml, subsequent accesses return cached data.
//!
//! # Examples
//!
//! ```no_run
//! use craole_cc_project::prelude::*;
//!
//! let metadata = WorkspaceMetadata::default();
//! println!("Project: {} v{}", metadata.name, metadata.version);
//! println!("Description: {}", metadata.description);
//! ```

use crate::_prelude::*;

/// Static cache for project metadata TOML table.
///
/// Uses `OnceLock` to ensure metadata is read and parsed exactly once,
/// even if accessed from multiple threads or multiple times.
static METADATA: OnceLock<CargoToml> = OnceLock::new();

/// Get cached workspace metadata (without tracing).
#[cfg(not(feature = "tracing"))]
pub fn get() -> &'static CargoToml {
  METADATA.get_or_init(load_metadata)
}

/// Get cached workspace metadata (with tracing).
#[cfg(feature = "tracing")]
#[tracing::instrument(level = "trace", name = "get_workspace_metadata")]
pub fn get() -> &'static CargoToml {
  METADATA.get_or_init(|| {
    trace!("Loading workspace metadata from Cargo.toml");
    let result = load_metadata();
    debug!("Workspace metadata loaded successfully");
    result
  })
}

/// Set metadata cache (without tracing).
#[cfg(not(feature = "tracing"))]
pub fn set(data: CargoToml) -> &'static CargoToml {
  METADATA.get_or_init(|| data)
}

/// Set metadata cache (with tracing).
#[cfg(feature = "tracing")]
#[tracing::instrument(level = "trace", name = "set_workspace_metadata", skip(data))]
pub fn set(data: CargoToml) -> &'static CargoToml {
  trace!("Setting workspace metadata cache");
  METADATA.get_or_init(|| {
    debug!("Workspace metadata cache initialized");
    data
  })
}

/// Load metadata from workspace Cargo.toml using your utils.
fn load_metadata() -> CargoToml {
  let root = find_cargo_root();
  let toml = root.join("Cargo.toml");

  read_cargo_metadata(&toml).unwrap_or_else(|| {
    #[cfg(feature = "tracing")]
    warn!("Failed to read workspace metadata, using empty map");

    TomlMap::new()
  })
}

/// Project metadata extracted from workspace Cargo.toml.
///
/// Contains package-level information (name, version, description) discovered
/// from the workspace root's Cargo.toml file.
///
/// # Fields
/// - `name` - Package name (from `[package]` or `[workspace.package]`)
/// - `version` - Package version (same section)
/// - `description` - Package description (same section)
///
/// # Defaults
/// If metadata is not found or parsing fails:
/// - `name`: "craole-cc" (workspace name, since workspaces don't have [package] names)
/// - `version`: empty string
/// - `description`: empty string
///
/// # Thread Safety
/// Safe to clone and share; all fields are `String` types.
///
/// # Examples
/// ```no_run
/// use craole_cc_project::prelude::*;
///
/// let metadata = WorkspaceMetadata::new();
/// if !metadata.name.is_empty() {
///   println!("Project: {}", metadata.name);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Metadata {
  pub name: String,
  pub version: String,
  pub description: String,
}

impl Default for Metadata {
  /// Creates metadata by reading and parsing the workspace Cargo.toml.
  ///
  /// # Process
  /// 1. Calls `read_project_toml()` which:
  ///    - Finds the project root using `find_cargo_root()`
  ///    - Loads and parses `{root}/Cargo.toml`
  ///    - Caches the result in `METADATA` `OnceLock`
  /// 2. Extracts name/version/description from appropriate section:
  ///    - Workspace toml: `[workspace.package]`
  ///    - Package toml: `[package]`
  /// 3. Returns with fallback defaults if extraction fails
  ///
  /// # Performance
  /// - First call: 5-50ms (file read, TOML parse, workspace discovery)
  /// - Subsequent calls: <1µs (cache lookup)
  ///
  /// # Failures (non-fatal, uses defaults)
  /// - Cargo.toml not found
  /// - TOML parsing error
  /// - Expected section missing
  fn default() -> Self {
    let meta = Self::read_project_toml();

    // Workspace does not have name
    let fallback_name = String::from("craole-cc");

    // Try to get package metadata from project [package] section
    let name = meta
      .get("package")
      .and_then(|p| p.get("name"))
      .and_then(|n| n.as_str())
      .map_or(fallback_name, String::from);

    let version = meta
      .get("version")
      .and_then(|v| v.as_str())
      .map(String::from)
      .unwrap_or_default();

    let description = meta
      .get("description")
      .and_then(|d| d.as_str())
      .map(String::from)
      .unwrap_or_default();

    Self {
      name,
      version,
      description,
    }
  }
}

impl Metadata {
  /// Creates a new default metadata instance.
  ///
  /// Equivalent to `Metadata::default()`. Use builder methods to customize fields.
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the metadata name, overriding the Cargo.toml value.
  ///
  /// # Examples
  /// ```no_run
  /// use craole_cc_project::prelude::*;
  /// let metadata = WorkspaceMetadata::new().with_name("my-project");
  /// ```
  #[must_use]
  pub fn with_name(mut self, name: impl Into<String>) -> Self {
    self.name = name.into();
    self
  }

  /// Sets the metadata version, overriding the Cargo.toml value.
  #[must_use]
  pub fn with_version(mut self, version: impl Into<String>) -> Self {
    self.version = version.into();
    self
  }

  /// Sets the metadata description, overriding the Cargo.toml value.
  #[must_use]
  pub fn with_description(mut self, description: impl Into<String>) -> Self {
    self.description = description.into();
    self
  }

  /// Load and cache the project Cargo.toml metadata.
  ///
  /// Uses the static `METADATA` cache to ensure the file is only read once.
  /// Calls `read_cargo_metadata()` from `project::utils` to do the actual work.
  ///
  /// # Returns
  /// `&'static CargoToml` - The parsed [package] or [workspace.package] section
  ///
  /// # Performance
  /// - First call: ~5-15ms (file I/O + TOML parsing)
  /// - Subsequent calls: <1µs (static cache lookup)
  ///
  /// # Caching
  /// The result is cached in `METADATA` static `OnceLock`.
  /// Thread-safe; multiple threads coordinate safely during first initialization.
  fn read_project_toml() -> &'static CargoToml {
    METADATA.get_or_init(|| {
      let project_root = find_cargo_root();
      let cargo_toml_path = project_root.join("Cargo.toml");

      read_cargo_metadata(&cargo_toml_path).unwrap_or_else(|| {
        eprintln!(
          "Failed to read metadata from Cargo.toml at {}",
          cargo_toml_path.display()
        );
        TomlMap::new()
      })
    })
  }
}
