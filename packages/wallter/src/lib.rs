// -- Macros --
#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate embellish;
#[macro_use]
extern crate logline;

// -- Modules --
mod api;
pub mod cli;
pub mod config;
mod consts;
mod error;
pub mod features;
mod utils;

// -- Exports --
pub mod prelude {
  pub use crate::api::Api;
  pub use crate::config::Config;
  pub use crate::consts::*;
  pub use crate::error::{Error, Result};
  pub use crate::utils::*;
}
pub use prelude::{Api, Config, Error, Result};
