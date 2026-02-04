pub use {
  super::super::{
    core::prelude::*,
    error::prelude::*,
    // package::prelude::*,
    project::prelude::*,
  },
  crate::prelude::*,
  dotenv::dotenv,
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
