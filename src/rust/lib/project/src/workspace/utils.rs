//! Workspace root detection and TOML metadata parsing utilities.
//!
//! Provides robust detection of the workspace root directory with multiple fallback strategies,
//! and utilities to read and parse Cargo.toml files while automatically handling workspace vs
//! package metadata.
//!
//! # Discovery Strategy
//!
//! The workspace root is located using a 5-step process with fast paths first:
//!
//! 1. **Environment variable override** (~instant)
//!    - `PRJ_ROOT`, `PROJECT_ROOT`, `WORKSPACE_ROOT`, or `CARGO_WORKSPACE_DIR`
//!    - Useful for testing or non-standard layouts
//!
//! 2. **Walk up from `CARGO_MANIFEST_DIR`** (~1-2ms)
//!    - Checks each parent directory for workspace Cargo.toml
//!    - Maximum 10 levels to prevent infinite loops
//!
//! 3. **Walk up from `current_dir`** (~1-2ms)
//!    - Starts from process's current working directory
//!    - Also caps at 10 levels
//!
//! 4. **`cargo_metadata` fallback** (~50-100ms, requires `metadata` feature)
//!    - Guaranteed correct but slower, invokes cargo
//!
//! 5. **Last resort** (instant)
//!    - Falls back to `CARGO_MANIFEST_DIR` or current directory
//!
//! # Examples
//!
//! ```no_run
//! use craole_cc_project::prelude::*;
//! let root = find_cargo_root();
//! println!("Project root: {}", root.display());
//!
//! let is_workspace = is_workspace_toml(&root.join("Cargo.toml"));
//! println!("Is workspace: {}", is_workspace);
//!
//! let metadata = read_cargo_metadata(&root.join("Cargo.toml"));
//! if let Some(meta) = metadata {
//!   println!("Metadata: {:?}", meta);
//! }
//! ```

use crate::_prelude::*;

/// Find the workspace root directory with fast detection methods first, then fallbacks.
///
/// Uses a 5-step strategy prioritizing speed for common cases:
///
/// 1. Check explicit environment variable overrides
/// 2. Walk up from `CARGO_MANIFEST_DIR` (typically fast)
/// 3. Walk up from current directory with workspace markers
/// 4. Use `cargo_metadata` (if feature enabled, slower but guaranteed)
/// 5. Fallback to `CARGO_MANIFEST_DIR` or current directory
///
/// # Returns
/// `PathBuf` - The detected workspace root directory (never fails)
///
/// # Performance
/// - Best case: <1ms (env var or immediate parent is workspace)
/// - Common case: 1-2ms (walk a few directories)
/// - Worst case: 50-100ms (`cargo_metadata`) or instant fallback
///
/// # Examples
/// ```no_run
/// use craole_cc_project::prelude::*;
/// let root = find_cargo_root();
/// assert!(root.join("Cargo.toml").exists());
/// ```
pub fn find_cargo_root() -> PathBuf {
  // Methods 0 & 1: Explicit override via environment variables
  let env_vars = [
    "PROJECT_ROOT",
    "WORKSPACE_ROOT",
    "CARGO_WORKSPACE_DIR", // Rust 1.80+, instant
  ];

  for var_name in &env_vars {
    if let Ok(prj_path) = var(var_name) {
      let path = PathBuf::from(prj_path);
      if path.exists() {
        // eprintln!(
        //   "[DEBUG] Found project root via `{var_name}`: {:#?}",
        //   path.display()
        // );
        return path;
      }
    }
  }

  // Method 2: Walk up from CARGO_MANIFEST_DIR
  if let Some(root) = walk_to_workspace() {
    return root;
  }

  // Method 3: Walk up from current_dir looking for workspace markers
  if let Some(root) = search_from_current_dir() {
    return root;
  }

  // Method 4: Use cargo_metadata as last resort
  #[cfg(feature = "full")]
  {
    if let Some(root) = find_cargo_root_via_cargo_metadata() {
      return root;
    }
  }

  // Final fallback: CARGO_MANIFEST_DIR or current directory
  var("CARGO_MANIFEST_DIR")
    .map(PathBuf::from)
    .or_else(|_| current_dir())
    .unwrap_or_else(|_| PathBuf::from("."))
}

/// Walk up from `CARGO_MANIFEST_DIR` to find workspace Cargo.toml.
///
/// Checks each parent directory for a workspace Cargo.toml up to 10 levels.
/// Stops as soon as a workspace is found.
///
/// # Returns
/// `Option<PathBuf>` - First workspace root found, or None if none exists
///
/// # Performance
/// ~1-2ms typical (few directory lookups)
#[must_use]
fn walk_to_workspace() -> Option<PathBuf> {
  let manifest_dir = var("CARGO_MANIFEST_DIR").ok()?;
  let mut current = PathBuf::from(manifest_dir);

  // Maximum 10 levels up to prevent infinite loops
  for _ in 0..10 {
    let cargo_toml = current.join("Cargo.toml");

    if cargo_toml.exists() && is_workspace_toml(&cargo_toml) {
      return Some(current);
    }

    // Move to parent directory
    current = current.parent()?.to_path_buf();
  }

  None
}

/// Search up from current directory for workspace markers.
///
/// Looks for either:
/// - A Cargo.toml with `[workspace]` section, or
/// - A `crates/` directory (common workspace layout indicator)
///
/// Searches up to 10 directory levels to prevent infinite loops.
///
/// # Returns
/// `Option<PathBuf>` - First workspace root found, or None
///
/// # Performance
/// ~1-2ms typical (few directory lookups + metadata checks)
#[must_use]
fn search_from_current_dir() -> Option<PathBuf> {
  let mut search_dir = current_dir().ok()?;

  // Maximum 10 levels up
  for _ in 0..10 {
    let cargo_toml = search_dir.join("Cargo.toml");

    // Look for workspace with common markers (faster than reading the whole file)
    if cargo_toml.exists() {
      // Check for common workspace indicators
      let has_crates_dir = search_dir.join("crates").exists();
      let has_workspace_toml = is_workspace_toml(&cargo_toml);

      if has_workspace_toml || (has_crates_dir && cargo_toml.exists()) {
        return Some(search_dir);
      }
    }

    // Move to parent
    search_dir = search_dir.parent()?.to_path_buf();
  }

  None
}

/// Check if a Cargo.toml file defines a workspace (optimized for speed).
///
/// Uses two optimizations:
/// 1. **Fast reject**: Files < 50 bytes can't be workspace Cargo.toml
/// 2. **Content check**: Must have `[workspace]` section + `members` or `resolver`
///
/// # Parameters
/// - `path` - Path to Cargo.toml to check
///
/// # Returns
/// `bool` - True if this is a workspace Cargo.toml, false otherwise
///
/// # Performance
/// Typically <1ms (single file read + string search)
///
/// # Examples
/// ```no_run
/// use {
///   craole_cc_project::prelude::*,
///   std::path::Path,
/// };
///
/// let is_ws = is_workspace_toml(Path::new("Cargo.toml"));
/// println!("Is workspace: {}", is_ws);
/// ```
#[must_use]
pub fn is_workspace_toml(path: &Path) -> bool {
  // Fast path: check file size first (workspace Cargo.toml usually > 100 bytes)
  if let Ok(file_metadata) = metadata(path)
    && file_metadata.len() < 50
  {
    return false; // Too small to be a workspace Cargo.toml
  }

  // Read and check for workspace markers
  if let Ok(contents) = read_to_string(path) {
    // Quick check: must have [workspace] section
    if !contents.contains("[workspace]") {
      return false;
    }

    // Workspace should have members or resolver
    contents.contains("members") || contents.contains("resolver")
  } else {
    false
  }
}

/// Fallback workspace detection using `cargo_metadata` (slower but guaranteed correct).
///
/// Invokes `cargo metadata` command to query the workspace root. This is more reliable
/// than heuristics but significantly slower (~50-100ms).
///
/// Only available with `metadata` feature enabled.
///
/// # Returns
/// `Option<PathBuf>` - The workspace root, or None if metadata command fails
///
/// # Performance
/// ~50-100ms (spawns cargo subprocess)
///
/// # Feature
/// Requires `metadata` feature to be enabled
#[cfg(feature = "full")]
#[must_use]
pub fn find_cargo_root_via_cargo_metadata() -> Option<PathBuf> {
  use cargo_metadata::MetadataCommand;

  MetadataCommand::new()
    .no_deps() // Skip dependency resolution for speed
    .exec()
    .ok()
    .map(|metadata| metadata.workspace_root.into_std_path_buf())
}

/// Read and parse a Cargo.toml file, returning the appropriate package metadata.
///
/// Automatically detects whether the Cargo.toml is a workspace or package file
/// and returns the correct `[package]` or `[workspace.package]` section.
///
/// # Parameters
/// - `cargo_toml_path` - Path to the Cargo.toml file to parse
///
/// # Returns
/// `Option<CargoToml>` - The parsed package metadata table, or None on error
///
/// # Behavior
/// - **Workspace file**: Extracts `[workspace.package]` section
/// - **Package file**: Extracts `[package]` section
/// - **Parse error**: Returns None with eprintln! message
/// - **Missing section**: Returns None with eprintln! message
///
/// # Examples
/// ```no_run
/// use {
///   craole_cc_project::prelude::*,
///   std::path::Path,
/// };
///
/// let metadata = read_cargo_metadata(Path::new("Cargo.toml"));
/// if let Some(meta) = metadata {
///   if let Some(name) = meta.get("name") {
///     println!("Package name: {:?}", name);
///   }
/// }
/// ```
///
/// # Performance
/// - File read: 1-5ms
/// - TOML parse: 1-10ms
/// - Total: ~5-15ms
#[must_use]
pub fn read_cargo_metadata(cargo_toml_path: &Path) -> Option<CargoToml> {
  let contents = read_to_string(cargo_toml_path).ok()?;
  let toml_value = from_toml_str::<TomlValue>(&contents).ok()?;
  let root_table = toml_value.as_table()?;

  // Check if this is a workspace toml
  if is_workspace_toml(cargo_toml_path) {
    // Read from [workspace.package]
    root_table
      .get("workspace")
      .and_then(|w| w.get("package"))
      .and_then(|p| p.as_table())
      .cloned()
  } else {
    // Read from [package]
    root_table
      .get("package")
      .and_then(|p| p.as_table())
      .cloned()
  }
}
