// -- Macros --
#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate embellish;
#[macro_use]
extern crate logline;

// -- Modules --
#[cfg(feature = "config")]
pub mod config;
pub mod custom;
pub mod default;
#[cfg(feature = "glob")]
pub mod glob;
#[cfg(feature = "http")]
pub mod http;
pub mod io;
#[cfg(any(feature = "json", feature = "toml"))]
pub mod parse;
pub mod utils;

// -- Exports --
pub use anyhow::{
  self, Context as AnyhowContext, Error as AnyhowError, Result as AnyhowResult
};
#[cfg(feature = "config")]
pub use config::Error as ConfigError;
pub use custom::Error as CustomError;
pub use default::{
  Context as ErksContext, Error as ErksError, Result as ErksResult,
  Severity as ErksSeverity, *
};
#[cfg(feature = "glob")]
pub use glob::Error as GlobError;
#[cfg(feature = "http")]
pub use http::{
  Error as HttpError, Error as HttpRequestError, Error as ReqwestError
};
pub use io::{Error as IoError, Error as IOError, Error as SystemError};
#[cfg(any(feature = "json", feature = "toml"))]
pub use parse::Error as ParseError;
pub use thiserror::{self, Error as ThisError};
