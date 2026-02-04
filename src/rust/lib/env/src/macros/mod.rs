//! Convenience macros for environment management.
//!
//! This module provides ergonomic macros for initializing and accessing
//! the global environment. All macros are thin wrappers around the core
//! API (`get()`, `set()`) for better usability.
//!
//! # Feature Gate
//! Only available when the `macros` feature is enabled.
//!
//! # Available Macros
//!
//! - [`setenv!`] - Initialize the global environment
//! - [`getenv!`] - Access environment configuration and metadata
//!
//! # Examples
//!
//! ## Basic Usage
//! ```ignore
//! use prjenv::prelude::*;
//!
//! // Initialize from CARGO_PKG_* env vars
//! setenv!();
//!
//! // Access package metadata
//! let name = getenv!(pkg_name);
//! let version = getenv!(pkg_version);
//!
//! // Access configuration
//! let port = getenv!(port);
//! let db = getenv!(db);
//! ```
//!
//! ## Custom Initialization
//! ```ignore
//! use prjenv::prelude::*;
//!
//! let custom = Environment::new()
//!   .with_pkg_name("my-app")
//!   .with_port(8080);
//!
//! setenv!(custom);
//! ```
//!
//! # Relationship to Core API
//!
//! These macros are purely syntactic sugar. They compile to:
//! - `setenv!()` → `prjenv::set(Environment::new().with_...)`
//! - `getenv!(field)` → `&prjenv::get().path.to.field`
//!
//! You can always use the core API directly if you prefer:
//! ```ignore
//! let env = prjenv::get();
//! println!("Port: {}", env.config.port);
//! ```

mod getenv;
mod setenv;

// Re-export macros (they're already at crate root due to #[macro_export])
// This allows: use prjenv::macros::*;
pub use crate::{
  getenv,
  setenv,
};
