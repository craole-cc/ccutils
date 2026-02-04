//! Metadata loading from Cargo.toml (infrastructure).
//!
//! Handles reading and parsing Cargo.toml files to extract metadata.

use crate::_prelude::*;

/// Load metadata from a Cargo.toml file.
///
/// Attempts to read workspace-level `[workspace.package]` first,
/// then falls back to `[package]` section.
///
/// # Arguments
/// - `toml_path` - Path to Cargo.toml file
///
/// # Returns
/// `Metadata` with extracted fields, or defaults if parsing fails.
///
/// # Examples
/// ```no_run
/// use prjenv::prelude::*;
///
/// let metadata = load_metadata_from_file(Path::new("Cargo.toml"));
/// println!("{}", metadata.display_name());
/// ```
#[cfg(not(feature = "tracing"))]
#[must_use]
pub fn load_from_file(toml_path: &Path) -> Metadata {
  load_from_file_impl(toml_path)
}

#[cfg(feature = "tracing")]
#[tracing::instrument(level = "debug", name = "load_metadata")]
pub fn load_from_file(toml_path: &Path) -> Metadata {
  trace!("Loading metadata from {}", toml_path.display());
  let metadata = load_from_file_impl(toml_path);
  debug!("Loaded metadata: {}", metadata.display_name());
  metadata
}

/// Internal implementation (shared by traced/non-traced versions).
fn load_from_file_impl(toml_path: &Path) -> Metadata {
  let toml = read_cargo_metadata(toml_path).unwrap_or_else(|| {
    #[cfg(feature = "tracing")]
    warn!("Failed to read {}, using defaults", toml_path.display());

    TomlMap::new()
  });

  extract_metadata(&toml)
}

/// Extract metadata from parsed TOML map.
///
/// Tries `[workspace.package]` first, then `[package]`.
fn extract_metadata(toml: &CargoToml) -> Metadata {
  // Try workspace.package first
  let section = toml
    .get("workspace")
    .and_then(|w| w.get("package"))
    .or_else(|| toml.get("package"));

  let name = section
    .and_then(|s| s.get("name"))
    .and_then(|n| n.as_str())
    .map(String::from)
    .unwrap_or_default();

  let version = section
    .and_then(|s| s.get("version"))
    .and_then(|v| v.as_str())
    .map(String::from)
    .unwrap_or_default();

  let description = section
    .and_then(|s| s.get("description"))
    .and_then(|d| d.as_str())
    .map(String::from)
    .unwrap_or_default();

  Metadata::from_parts(name, version, description)
}

/// Load workspace metadata from discovered workspace root.
///
/// Combines workspace discovery with metadata loading.
#[must_use]
pub fn load_workspace_metadata() -> Metadata {
  let root = find_cargo_root();
  load_from_file(&root.join("Cargo.toml"))
}
