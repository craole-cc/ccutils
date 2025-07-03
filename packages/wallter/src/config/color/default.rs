//! Defines the configuration for user-specified color preferences,
//! including the system color mode (light/dark) and a list of
//! color tags for wallpaper filtering.

use super::Mode;
use rand::{prelude::SliceRandom, rng};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

const DEFAULT_RANDOM_COLOR_COUNT: usize = 5;

/// Predefined set of allowed color values in hex format
pub const ALLOWED_COLORS: &[&str] = &[
  "#660000", "#990000", "#cc0000", "#cc3333", "#ea4c88", "#993399", "#663399",
  "#333399", "#0066cc", "#0099cc", "#66cccc", "#77cc33", "#669900", "#336600",
  "#666600", "#999900", "#cccc33", "#ffff00", "#ffcc33", "#ff9900", "#ff6600",
  "#cc6633", "#996633", "#663300", "#000000", "#999999", "#cccccc", "#ffffff",
  "#424153"
];

/// Holds user-defined color preferences, including system mode and color tags.
///
/// This configuration manages:
/// 1. The desired system color mode (Light/Dark), which can be applied
///    system-wide.
/// 2. A list of color names or tags for filtering/tagging wallpapers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
  /// The desired system color mode (Light/Dark).
  pub mode: Mode,
  /// Color list validated against `ALLOWED_COLORS`
  pub colors: Vec<String>
}

impl Default for Config {
  fn default() -> Self {
    Self {
      mode: Mode::default(),
      colors: Self::randomize_colors(DEFAULT_RANDOM_COLOR_COUNT)
    }
  }
}

impl Config {
  /// Creates a new `Config` with a specified mode and list of colors.
  pub fn new(mode: Mode, colors: Vec<String>) -> Self {
    Self {
      mode,
      colors: Self::validate_colors(colors)
    }
  }

  // pub fn toggle_mode(&mut self) -> Result<()> {
  //   Ok(Mode::toggle())
  //   // self.mode=self.mode.
  // }

  /// Returns a new `Config` with the specified mode.
  #[must_use]
  pub fn with_mode(mut self, mode: Mode) -> Self {
    self.mode = mode;
    self
  }

  /// Returns a new `Config` with the specified colors.
  #[must_use]
  pub fn with_colors(mut self, colors: Vec<String>) -> Self {
    self.colors = Self::validate_colors(colors);
    self
  }

  /// Filters colors to only include those in the allowed list.
  fn validate_colors(colors: Vec<String>) -> Vec<String> {
    colors
      .into_iter()
      .filter(|color| ALLOWED_COLORS.contains(&color.as_str()))
      .collect()
  }

  /// Generates a list of `count` random colors from the `ALLOWED_COLORS` list.
  ///
  /// Returns an empty vector if `count` is 0. If `count` exceeds the number
  /// of allowed colors, returns all colors in random order. Otherwise,
  /// returns `count` unique randomly selected colors.
  pub fn randomize_colors(count: usize) -> Vec<String> {
    if count == 0 {
      return Vec::new();
    }

    let mut rng = rng();

    if count >= ALLOWED_COLORS.len() {
      let mut colors = ALLOWED_COLORS.to_vec();
      colors.shuffle(&mut rng);
      return colors.into_iter().map(ToString::to_string).collect();
    }

    // For selecting multiple unique items, we need to shuffle and take
    let mut colors = ALLOWED_COLORS.to_vec();
    colors.shuffle(&mut rng);
    colors
      .into_iter()
      .take(count)
      .map(ToString::to_string)
      .collect()
  }
}

impl Display for Config {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    printf!(f, "Mode", self.mode)?;

    let colors_display = if self.colors.is_empty() {
      "None specified".to_string()
    } else {
      self.colors.join(", ")
    };
    printf!(f, "Colors", colors_display)?;

    Ok(())
  }
}
