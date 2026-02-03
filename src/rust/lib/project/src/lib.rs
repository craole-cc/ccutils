//! Cargo workspace and package scaffolding and management.
//!
//! Provides tools for:
//! - Reading workspace and package metadata
//! - Creating new packages with scaffolding
//! - Managing workspace members
//! - Manipulating Cargo.toml files

mod core;
mod exports;
mod infrastructure;
mod metadata;
mod package;
mod workspace;

/// Internal prelude for use within this crate.
///
/// Import with `use crate::_prelude::*;` to get common std imports and TOML types.
pub(crate) mod _prelude {
  pub use super::exports::internal::*;
}

/// Public API prelude.
///
/// Import with `use devtools-project::prelude::*;` to get all public types
/// and TOML manipulation utilities.
pub mod prelude {
  pub use super::exports::external::*;
}
