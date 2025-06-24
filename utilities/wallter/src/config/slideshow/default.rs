use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Serialize, Deserialize)]
pub enum Unit {
  #[serde(rename = "seconds")]
  Seconds,
  #[serde(rename = "minutes")]
  Minutes,
  #[serde(rename = "hours")]
  Hours,
  #[serde(rename = "days")]
  Days
}

impl Display for Unit {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Unit::Seconds => write!(f, "seconds"),
      Unit::Minutes => write!(f, "minutes"),
      Unit::Hours => write!(f, "hours"),
      Unit::Days => write!(f, "days")
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Interval {
  pub value: u32,
  pub unit: Unit
}

impl Default for Interval {
  fn default() -> Self {
    Self {
      value: 60,
      unit: Unit::Seconds
    }
  }
}

impl Interval {
  pub fn with_seconds(value: u32) -> Self {
    Self {
      value,
      unit: Unit::Seconds
    }
  }

  pub fn with_minutes(value: u32) -> Self {
    Self {
      value,
      unit: Unit::Minutes
    }
  }

  pub fn with_hours(value: u32) -> Self {
    Self {
      value,
      unit: Unit::Hours
    }
  }

  pub fn with_days(value: u32) -> Self {
    Self {
      value,
      unit: Unit::Days
    }
  }
}

impl Display for Interval {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{} {}", self.value, self.unit)
  }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
  pub interval: Interval,
  pub enabled: bool,
  pub sources: Vec<String>
}

impl Display for Config {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    writeln!(f, "Slideshow Settings:")?;
    writeln!(f, "  Change Interval: {}", self.interval)?;
    writeln!(f, "  Enabled: {}", self.enabled)?;
    writeln!(f, "  Sources: {}", self.sources.join(", "))
  }
}
