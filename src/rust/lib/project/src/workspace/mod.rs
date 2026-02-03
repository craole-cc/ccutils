//! Workspace-level operations and metadata.

mod config;
mod core;
mod manage;
mod metadata;
mod paths;
mod utils;

// Re-export utility functions
pub use {
  config::*,
  core::*,
  manage::*,
  metadata::*,
  paths::*,
  utils::*,
};

/// Public prelude for workspace module.
pub mod prelude {
  pub use super::{
    config::Configuration as WorkspaceConfig,
    core::Environment as Workspace,
    manage::WorkspaceManager,
    metadata::Metadata as WorkspaceMetadata,
    paths::Paths as WorkspacePaths,
    utils::*,
  };
}
