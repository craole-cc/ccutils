//! Internal imports shared across all modules.
//!
//! Import with `use crate::prelude::*;` in internal modules.

// Optional convenience macros
#[cfg(feature = "macros")]
pub use crate::macros::*;
// Optional tracing support
#[cfg(feature = "tracing")]
pub use tracing::{debug, error, info, trace, warn};
pub use {
  crate::{
    core::*, infrastructure::prelude::*, metadata::prelude::*, package::prelude::*,
    workspace::prelude::*,
  },
  dotenvy::dotenv,
  std::{
    env::{current_dir, var},
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    fs::{create_dir_all, metadata, read_to_string, write},
    io::{stderr, stdout, Error as IOError, ErrorKind as IOErrorKind, Write as IOWrite},
    path::{Path, PathBuf},
    result::Result as StdResult,
    str::FromStr,
    sync::{Arc, OnceLock},
  },
  toml::{
    from_str as from_toml_str, map::Map as TomlMap, to_string as to_toml_string,
    to_string_pretty as to_toml_string_pretty, Table as TomlTable, Value as TomlValue,
  },
};

/// Type alias for Cargo.toml representation as a TOML map.
///
/// Used when reading and parsing Cargo.toml files to represent
/// package or workspace metadata sections.
///
/// # Examples
/// ```ignore
/// use crate::prelude::*;
///
/// let cargo_toml: CargoToml = from_toml_str(contents)?;
/// if let Some(name) = cargo_toml.get("name") {
///     println!("Package: {}", name);
/// }
/// ```
pub type CargoToml = TomlMap<String, TomlValue>;

/// Result type for project operations.
pub type Result<T> = StdResult<T, IOError>; // TODO: Why not StdError here?

/// Generic result type for operations with custom error types
pub type GenericResult<T, E> = StdResult<T, E>;
