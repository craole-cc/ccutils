//! Workspace-level operations and metadata.

mod core;
mod manage;
mod utils;

pub use {
  core::*,
  manage::*,
  utils::*,
};

/// Public prelude for workspace module.
pub mod prelude {
  pub use super::{
    Workspace,
    WorkspaceManager,
    find_cargo_root,
    read_cargo_metadata,
  };
}
