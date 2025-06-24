#[macro_use]
pub mod utils;

mod api;
pub use api::Api;

pub mod consts;

mod error;
pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub mod config;
pub use config::Config;
