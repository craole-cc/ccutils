//! Metadata module.
//!
//! Provides domain model (`Metadata`) and infrastructure for loading/caching.

mod cache;
mod core;
mod fetch;

pub use {
  self::core::Metadata,
  cache::{
    get_cached_workspace,
    set_cached_workspace,
  },
  fetch::{
    // extract_metadata,
    load_from_file,
    load_workspace_metadata,
  },
};

pub mod prelude {
  pub use super::{
    Metadata,
    get_cached_workspace,
    load_from_file,
  };
}
