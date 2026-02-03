//! Metadata caching using OnceLock.
//!
//! Provides static cache for workspace metadata to avoid repeated file I/O.

use {
  super::{
    Metadata,
    load_workspace_metadata,
  },
  crate::_prelude::*,
};

/// Static cache for workspace metadata.
static WORKSPACE_METADATA: OnceLock<Metadata> = OnceLock::new();

/// Get cached workspace metadata (lazy-loaded).
///
/// First call loads from Cargo.toml, subsequent calls return cached value.
///
/// # Examples
/// ```no_run
/// use craole_cc_project::metadata::*;
///
/// let metadata = get_cached_workspace();
/// println!("Workspace: {}", metadata.display_name());
/// ```
#[cfg(not(feature = "tracing"))]
pub fn get_cached_workspace() -> &'static Metadata {
  WORKSPACE_METADATA.get_or_init(load_workspace_metadata)
}

#[cfg(feature = "tracing")]
#[tracing::instrument(level = "trace", name = "get_workspace_metadata")]
pub fn get_cached_workspace() -> &'static Metadata {
  WORKSPACE_METADATA.get_or_init(|| {
    trace!("Loading workspace metadata");
    let metadata = load_workspace_metadata();
    debug!("Workspace metadata cached: {}", metadata.display_name());
    metadata
  })
}

/// Set workspace metadata cache manually.
///
/// Useful for testing or custom initialization.
///
/// # Note
/// If the cache is already initialized, this returns the existing value
/// and ignores the provided metadata (idempotent behavior).
///
/// # Examples
/// ```no_run
/// use craole_cc_project::metadata::*;
///
/// let custom = Metadata::new()
///   .with_name("test-workspace")
///   .with_version("0.0.0");
///
/// set_cached_workspace(custom);
/// ```
pub fn set_cached_workspace(metadata: Metadata) -> &'static Metadata {
  WORKSPACE_METADATA.get_or_init(|| metadata)
}

/// Try to get cached metadata without initializing.
///
/// Returns `None` if cache hasn't been initialized yet.
///
/// # Examples
/// ```no_run
/// use craole_cc_project::metadata::*;
///
/// if let Some(metadata) = try_get_cached_workspace() {
///   println!("Already loaded: {}", metadata.display_name());
/// } else {
///   println!("Not yet loaded");
/// }
/// ```
pub fn try_get_cached_workspace() -> Option<&'static Metadata> {
  WORKSPACE_METADATA.get()
}

#[cfg(test)]
mod tests {
  use super::*;

  /// For tests, just use a fresh process or accept that cache persists.
  ///
  /// Since OnceLock is process-global and doesn't support clearing,
  /// tests that need isolation should use `cargo test -- --test-threads=1`
  /// or run in separate processes.
  ///
  /// Alternatively, use dependency injection in your code instead of
  /// relying on the global cache for testable code paths.
  #[test]
  fn test_cache_is_idempotent() {
    let first = set_cached_workspace(Metadata::new().with_name("first"));

    let second = set_cached_workspace(Metadata::new().with_name("second"));

    // Second call returns the first value (cache already set)
    assert_eq!(first.name, second.name);
    assert_eq!(first.name, "first"); // or whatever was already cached
  }

  #[test]
  fn test_try_get_before_init() {
    // This test might fail if other tests already initialized the cache
    // Run with --test-threads=1 for isolation
    if let Some(_metadata) = try_get_cached_workspace() {
      println!("Cache already initialized by another test");
    }
  }
}
