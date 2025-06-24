use tracing_subscriber::EnvFilter;

#[derive(Debug, Default, PartialEq, Clone)]
pub enum Level {
  TRACE,
  DEBUG,
  INFO,
  WARN,
  ERROR,
  OFF,
  #[default]
  ENV
}

impl std::fmt::Display for Level {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let level_str = match self {
      Level::TRACE => "TRACE",
      Level::DEBUG => "DEBUG",
      Level::INFO => "INFO",
      Level::WARN => "WARN",
      Level::ERROR => "ERROR",
      Level::OFF => "OFF",
      Level::ENV => "ENV"
    };

    write!(f, "{}", level_str)
  }
}

impl Level {
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
