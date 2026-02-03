//! Internal imports shared across all modules.
//!
//! Import with `use crate::_prelude::*;` in internal modules.

// Optional tracing support
#[cfg(feature = "tracing")]
pub use tracing::{
  debug,
  error,
  info,
  trace,
  warn,
};
pub use {
  // crate::{
  //   // package::prelude::*,
  //   workspace::prelude::*,
  // },
  // Standard library
  std::{
    env::{
      current_dir,
      var,
    },
    fs::{
      create_dir_all,
      metadata,
      read_to_string,
      write,
    },
    io::{
      self,
      Error as IoError,
      ErrorKind,
      Result as IoResult,
    },
    path::{
      Path,
      PathBuf,
    },
    result::Result as StdResult,
    sync::OnceLock,
  },

  // TOML (always enabled - required for scaffolding)
  toml::{
    Table as TomlTable,
    Value as TomlValue,
    from_str as from_toml_str,
    map::Map as TomlMap,
    to_string as to_toml_string,
    to_string_pretty as to_toml_string_pretty,
  },
};

/// Type alias for Cargo.toml representation as a TOML map.
///
/// Used when reading and parsing Cargo.toml files to represent
/// package or workspace metadata sections.
///
/// # Examples
/// ```ignore
/// use crate::_prelude::*;
///
/// let cargo_toml: CargoToml = from_toml_str(contents)?;
/// if let Some(name) = cargo_toml.get("name") {
///     println!("Package: {}", name);
/// }
/// ```
pub type CargoToml = TomlMap<String, TomlValue>;

/// Result type for project operations.
pub type Result<T> = StdResult<T, IoError>;
