// -- Macros --
#[macro_use]
extern crate cfg_if;

#[macro_use]
extern crate embellish;

#[macro_use]
extern crate logline;

// -- Modules --
mod utils;
mod api;
pub mod cli;
pub mod config;
mod consts;
mod error;
pub mod features;

// -- Exports --
pub use api::Api;
pub use config::Config;
pub use error::{Error, Result};
pub mod prelude;
