// -- Modules --
pub mod api;
pub mod cli;
pub mod config;
mod consts;
mod error;
pub mod features;
pub mod utils;

// -- Macros --
#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate embellish;
#[macro_use]
extern crate logline;

// -- Exports --
pub mod prelude {
  pub use crate::{
    // api::Api,
    cli,
    config::{self, Config},
    consts::*,
    error::{Error, Result},
    features,
    utils::{self, *}
  };
  pub use colored::*;

  cfg_if! {
      if #[cfg(test)] {
          pub use log::testing::*;
          pub use crate::*;
      }
  }
  // pub use colored::*;
}

pub use prelude::{
  // Api,
  Config,
  Error,
  Result // utils::log
};
