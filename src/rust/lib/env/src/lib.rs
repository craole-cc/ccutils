#[macro_use]
pub mod config;
pub mod constants;
pub mod time;
pub mod traits;
pub mod prelude {
  pub use {
    crate::{
      config::prelude::*,
      constants::prelude::*,
      time::prelude::*,
      traits::*,
    },
    bitflags::bitflags,
    cfg_if::cfg_if,
    miette::{
      Diagnostic,
      Result as MietteResult,
    },
    std::{
      borrow::Cow,
      env::{
        current_dir,
        var,
      },
      error::Error as StdError,
      fmt::{
        Debug,
        Display,
        Formatter,
        Result as FmtResult,
      },
      fs::{
        create_dir_all,
        metadata,
        read_to_string,
      },
      io::{
        Error as IOError,
        ErrorKind,
        Write as IOWrite,
        stderr,
        stdout,
      },
      panic::Location,
      path::{
        Path,
        PathBuf,
      },
      result::Result as StdResult,
      str::FromStr,
      sync::{
        Arc,
        OnceLock,
      },
    },
    thiserror::Error as ThisError,
  };
}
