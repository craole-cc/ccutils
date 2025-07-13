mod config;
pub mod utils;

pub use config::{
  Config as Logline, Config,
  Level::{DEBUG, ERROR, INFO, TRACE, WARN},
  Time::{Datetime, Uptime},
  *
};
pub use tracing::{self, debug, error, info, trace, warn};
pub use tracing_subscriber;
pub use utils::*;

//TODO Use embellish printf macro for the display
