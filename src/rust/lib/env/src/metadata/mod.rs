//! Metadata module.
//!
//! Provides domain model (`Metadata`) and infrastructure for loading/caching.

mod cache;
mod core;
mod fetch;

pub use {
  cache::*,
  core::*,
  fetch::*,
};

pub mod prelude {
  pub use super::{
    Metadata,
    get_cached_workspace as get_cached_workspace_metadata,
    load_from_file as load_metadata_from_file,
  };
}
