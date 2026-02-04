//! Infrastructure layer.
//!
//! Technical concerns: configuration from env vars and filesystem paths.

mod config;
mod paths;

pub use {
  config::*,
  paths::*,
};

pub mod prelude {
  pub use super::{
    Configuration,
    Paths,
  };
}
