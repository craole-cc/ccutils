use super::Size;
use serde::{Deserialize, Serialize};
use std::{
  cmp::Ordering,
  fmt::{self, Display, Formatter}
};

/// Represents the orientation of a monitor based on its resolution.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Config {
  /// Width > Height (e.g., 1920x1080)
  Landscape,
  /// Height > Width (e.g., 1080x1920)
  Portrait,
  /// Width == Height (e.g., 1024x1024)
  Square
}

impl Config {
  pub fn from_size(res: &Size) -> Self {
    match res.width.cmp(&res.height) {
      Ordering::Greater => Self::Landscape,
      Ordering::Less => Self::Portrait,
      Ordering::Equal => Self::Square
    }
  }
}

impl Display for Config {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Landscape => write!(f, "Landscape"),
      Self::Portrait => write!(f, "Portrait"),
      Self::Square => write!(f, "Square")
    }
  }
}
