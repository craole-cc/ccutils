use std::fmt::{self, Display, Formatter};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Default, PartialEq, Clone)]
pub enum Config {
  TRACE,
  DEBUG,
  INFO,
  WARN,
  ERROR,
  OFF,
  #[default]
  ENV
}

impl Display for Config {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let level_str = match self {
      Self::TRACE => "TRACE",
      Self::DEBUG => "DEBUG",
      Self::INFO => "INFO",
      Self::WARN => "WARN",
      Self::ERROR => "ERROR",
      Self::OFF => "OFF",
      Self::ENV => "ENV"
    };

    write!(f, "{level_str}")
  }
}

impl Config {
  pub fn new() -> Self {
    Self::default()
  }

  // pub fn filter(&self) -> EnvFilter {
  // 	match self {
  // 		Self::TRACE => EnvFilter::new("TRACE"),
  // 		Self::DEBUG => EnvFilter::new("DEBUG"),
  // 		Self::INFO => EnvFilter::new("INFO"),
  // 		Self::WARN => EnvFilter::new("WARN"),
  // 		Self::ERROR => EnvFilter::new("ERROR"),
  // 		Self::OFF => EnvFilter::new("OFF"),
  // 		Self::ENV => EnvFilter::from_default_env(),
  // 	}
  // }

  pub fn filter(&self) -> EnvFilter {
    if *self == Self::ENV {
      return EnvFilter::from_default_env();
    }
    EnvFilter::new(self.to_string())
  }
}
