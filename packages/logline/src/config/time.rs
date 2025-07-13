#[derive(Debug, Default, PartialEq, Clone)]
pub enum Config {
  #[default]
  None,
  Datetime,
  Uptime
}

impl Config {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn datetime(&self) -> Self {
    Self::Datetime
  }

  pub fn uptime(&self) -> Self {
    Self::Uptime
  }

  pub fn off(&self) -> Self {
    Self::None
  }
}
