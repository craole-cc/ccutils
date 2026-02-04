//! Cargo workspace and package scaffolding and management.
//!
//! Provides tools for:
//! - Reading workspace and package metadata
//! - Creating new packages with scaffolding
//! - Managing workspace members
//! - Manipulating Cargo.toml files

mod core;
pub mod infrastructure;
#[cfg(feature = "macros")]
pub mod macros;
pub mod metadata;
pub mod package;
pub mod prelude;
pub mod workspace;
