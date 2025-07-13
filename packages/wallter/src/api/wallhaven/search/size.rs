//! # Wallhaven API Parameters Builder
//!
//! This crate provides a type-safe, builder-pattern API for constructing
//! Wallhaven search parameters. It supports both web and API endpoints with
//! comprehensive parameter validation.
//!
//! ## Features
//!
//! - Type-safe parameter building using the builder pattern
//! - Bitflag-based category and purity filtering
//! - Resolution validation and mutual exclusion handling
//! - Color filtering with validation against predefined palettes
//! - Comprehensive error handling and validation
//!
//! ## Quick Start
//!
//! ```rust
//! use wallhaven_params::*;
//!
//! let params = Parameters::new()
//!     .with_query(vec!["+nature", "-landscape"])
//!     .with_categories(Category::General | Category::Anime)
//!     .with_purity(Purity::Sfw | Purity::Sketchy)
//!     .with_atleast("1920x1080")?
//!     .with_colors(vec!["#663399", "#000000"])
//!     .with_page(2);
//!
//! let url = params.build_url();
//! ```

use enumflags2::{BitFlags, bitflags};
use serde::{Deserialize, Serialize};
use std::{
  collections::HashSet,
  fmt::{self, Display, Formatter}
};
use urlencoding::encode;

/// Resolution specification for filtering wallpapers.
///
/// Provides type-safe handling of resolution constraints with mutual exclusion
/// between minimum resolution and exact resolution matching.
///
/// # Examples
///
/// ```rust
/// // Minimum resolution
/// let resolution = Resolution::AtLeast("1920x1080".to_string());
///
/// // Exact resolutions
/// let resolution = Resolution::Exact(vec![
///     "1920x1080".to_string(),
///     "2560x1440".to_string()
/// ]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resolution {
  /// Minimum resolution constraint (e.g., "1920x1080")
  ///
  /// Finds wallpapers with resolution greater than or equal to the specified
  /// value.
  AtLeast(String),
  /// Exact resolution matching (e.g., ["1920x1080", "2560x1440"])
  ///
  /// Finds wallpapers that exactly match one of the specified resolutions.
  Exact(Vec<String>)
}

impl Resolution {
  /// Validates that a resolution string follows the correct format.
  ///
  /// Resolution strings must follow the format "WIDTHxHEIGHT" where both
  /// WIDTH and HEIGHT are positive integers.
  ///
  /// # Arguments
  ///
  /// * `resolution` - The resolution string to validate
  ///
  /// # Returns
  ///
  /// * `Ok(())` if the resolution format is valid
  /// * `Err(String)` with a descriptive error message if invalid
  ///
  /// # Examples
  ///
  /// ```rust
  /// // Valid formats
  /// assert!(Resolution::validate_format("1920x1080").is_ok());
  /// assert!(Resolution::validate_format("3840x2160").is_ok());
  ///
  /// // Invalid formats
  /// assert!(Resolution::validate_format("invalid").is_err());
  /// assert!(Resolution::validate_format("1920x").is_err());
  /// assert!(Resolution::validate_format("x1080").is_err());
  /// ```
  pub fn validate_format(resolution: &str) -> Result<(), String> {
    let parts: Vec<&str> = resolution.split('x').collect();
    if parts.len() != 2 {
      return Err(format!(
        "Invalid resolution format: {resolution}. Expected format: WIDTHxHEIGHT"
      ));
    }

    for part in parts {
      part.parse::<u32>().map_err(|_| {
                format!(
                    "Invalid resolution format: {resolution}. Width and height must be positive numbers"
                )
            })?;
    }

    Ok(())
  }
}
