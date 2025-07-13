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

  pub fn datetime(mut self) -> Self {
    self = Self::Datetime;
    self
  }

  pub fn uptime(mut self) -> Self {
    self = Self::Uptime;
    self
  }

  pub fn off(mut self) -> Self {
    self = Self::None;
    self
  }
}
