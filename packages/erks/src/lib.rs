// -- Imports --
// #[macro_use]
// extern crate cfg_if;
// #[macro_use]
// extern crate embellish;
#[macro_use]
extern crate logline;
#[macro_use]
mod macros;
mod modules;
mod utils;

// -- Exports && Aliases --
pub use anyhow::{
  self, Context as AnyhowContext, Error as AnyhowError, Result as AnyhowResult
};
#[cfg(feature = "config")]
pub use modules::config::{self, Error as ConfigError};
#[cfg(feature = "glob")]
pub use modules::glob::{self, Error as GlobError};
#[cfg(feature = "http")]
pub use modules::http::{
  self, Error as HttpError, Error as HttpRequestError, Error as ReqwestError
};
#[cfg(any(feature = "json", feature = "toml"))]
pub use modules::parse::{self, Error as ParseError};
pub use modules::{
  core::{
    Context as ErksContext, Error as ErksError, Result as ErksResult,
    Severity as ErksSeverity, *
  },
  custom::{self, Error as CustomError},
  io::{self, Error as IoError, Error as IOError, Error as SystemError}
};
pub use thiserror::{self, Error as ThisError};
pub use utils::*;
