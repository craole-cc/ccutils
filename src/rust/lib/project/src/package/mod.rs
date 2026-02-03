//! Package module.
//!
//! Provides domain model for packages and scaffolding utilities.

mod core;
mod scaffold;

pub use {
  core::*,
  scaffold::*,
};

pub mod prelude {
  pub use super::{
    Package,
    Scaffold as PackageScaffold,
  };
}
