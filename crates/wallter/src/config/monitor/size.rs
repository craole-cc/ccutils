use super::Orientation;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter, Write};

/// Represents the pixel dimensions of a monitor.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
  /// The width in pixels.
  pub width: u32,
  /// The height in pixels.
  pub height: u32
}

impl Config {
  pub fn new(width: &u32, height: &u32) -> Self {
    Self {
      width: *width,
      height: *height
    }
  }

  /// Calculates the ratio (width / height).
  pub fn ratio(&self) -> f32 {
    if self.height > 0 {
      self.width as f32 / self.height as f32
    } else {
      0.0
    }
  }

  /// Calculates the ratio (width / height).
  pub fn ratio_str(&self) -> &'static str {
    if self.height > 0 {
      let ratio = self.width as f32 / self.height as f32;
      let formatted = format!("{ratio:.2}");
      let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
      Box::leak(trimmed.to_string().into_boxed_str())
    } else {
      "0" // No trailing zero needed
    }
  }

  /// Returns the resolution as a Resolution struct.
  pub fn resolution(&self) -> Config {
    Self {
      width: self.width,
      height: self.height
    }
  }

  /// Returns the resolution as a formatted string (e.g., "1920x1080").
  pub fn resolution_str(&self) -> &'static str {
    Box::leak(format!("{}x{}", self.width, self.height).into_boxed_str())
  }

  /// Determines the orientation based on width and height.
  pub fn orientation(&self) -> Orientation {
    Orientation::from_size(&self.resolution())
  }
}

impl Display for Config {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{} [{}] - {}",
      self.resolution_str(),
      self.ratio_str(),
      self.orientation()
    )?;

    // printf!(f, "Height", self.height)?;
    // printf!(f, "Width", self.width)?;
    // printf!(f, "Resolution", &self.resolution())?;
    // printf!(f, "Ratio", format!("{:.2}", self.ratio()))?;
    // printf!(f, "Orientation", &self.orientation())?;

    Ok(())
  }
}
