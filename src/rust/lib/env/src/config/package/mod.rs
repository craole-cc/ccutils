//! Package (current running crate) environment and metadata management.
//!
//! This module provides types for accessing metadata about the currently executing
//! crate/binary. It complements the `project` module which handles workspace-level
//! configuration.
//!
//! # Overview
//!
//! The package module contains two main types:
//! - **`Environment`** - Container for package-level metadata (this is called `Package` in prelude)
//! - **`Metadata`** - The actual name, version, and description fields
//!
//! # Relationship to Project Module
//!
//! In a Rust workspace, you have:
//! - **One workspace** with workspace-level Cargo.toml (`project` module handles this)
//! - **Multiple packages/crates** with individual Cargo.toml files (`package` module handles this)
//!
//! The global `Environment` struct combines both:
//! ```text
//! Environment {
//!     project: Project { ... },  // Workspace-level (from workspace Cargo.toml)
//!     package: Package { ... },  // Current crate (from env! macros or builder)
//! }
//! ```
//!
//! # Typical Usage
//!
//! ## Option 1: Using setenv! macro (recommended)
//! ```no_run
//! # #[cfg(feature = "macros")]
//! # {
//! use env::prelude::*;
//!
//! fn main() {
//!   setenv!(); // Auto-populates package metadata from env! macros
//!   let pkg_name = getenv!(pkg_name);
//!   let pkg_version = getenv!(pkg_version);
//!   println!("{} v{}", pkg_name, pkg_version);
//! }
//! # }
//! ```
//!
//! ## Option 2: Manual builder pattern
//! ```no_run
//! use env::prelude::*;
//!
//! set_env(
//!   Environment::new()
//!     .with_pkg_name(env!("CARGO_PKG_NAME"))
//!     .with_pkg_version(env!("CARGO_PKG_VERSION"))
//!     .with_pkg_description(env!("CARGO_PKG_DESCRIPTION")),
//! );
//! ```
//!
//! ## Option 3: Direct access
//! ```no_run
//! use env::prelude::*;
//!
//! let env = get_env();
//! println!("Running: {}", env.package.metadata.name);
//! ```
//!
//! # Compile-Time vs Runtime Metadata
//!
//! The recommended approach is using Rust's compile-time `env!()` macros, which are
//! evaluated at compile time and baked into the binary. This has several advantages:
//! - **Zero I/O at runtime** - No need to read Cargo.toml
//! - **No mismatches** - Version in binary always matches Cargo.toml
//! - **Performance** - Compile-time cost, zero runtime cost
//!
//! See individual module documentation for detailed examples and explanations.

mod core;
mod metadata;

pub use {
  core::*,
  metadata::*,
};

/// Public prelude for package module.
///
/// Import with `use env::package::prelude::*;` to get:
/// - `Environment` (aliased as `Package`)
/// - `Metadata` (aliased as `PackageMetadata`)
///
/// # Examples
/// ```no_run
/// use env::package::prelude::*;
///
/// let package = Package::new()
///   .with_name(env!("CARGO_PKG_NAME"))
///   .with_version(env!("CARGO_PKG_VERSION"));
/// ```
pub mod prelude {
  pub use super::{
    Environment as Package,
    Metadata as PackageMetadata,
  };
}
