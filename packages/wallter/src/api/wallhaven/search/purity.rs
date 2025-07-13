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

/// Purity levels for filtering wallpapers.
///
/// Can be combined using bitwise operations. Note that NSFW content requires
/// a valid API key to access.
///
/// # Examples
///
/// ```rust
/// // Safe for work only
/// let purity = Purity::Sfw;
///
/// // SFW and sketchy content
/// let purity = Purity::Sfw | Purity::Sketchy;
///
/// // All content types (requires API key for NSFW)
/// let purity = Purity::Sfw | Purity::Sketchy | Purity::Nsfw;
#[bitflags]
#[repr(u8)]
#[derive(Debug, Default, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Purity {
  #[default]
  /// Safe for work content
  Sfw = 0b100,
  /// Sketchy content (suggestive but not explicit)
  Sketchy = 0b010,
  /// Not safe for work content (requires API key)
  Nsfw = 0b001
}

impl Purity {
  pub fn default() -> BitFlags<Self> {
    Purity::sfw()
  }

  pub fn all() -> BitFlags<Self> {
    Purity::Sfw | Purity::Sketchy | Purity::Nsfw
  }

  pub fn no_sketchy() -> BitFlags<Self> {
    Purity::Sfw | Purity::Nsfw
  }

  pub fn no_sfw() -> BitFlags<Self> {
    Purity::Sketchy | Purity::Nsfw
  }

  pub fn no_nsfw() -> BitFlags<Self> {
    Purity::Sfw | Purity::Sketchy
  }

  pub fn sfw() -> BitFlags<Self> {
    Purity::Sfw.into()
  }

  pub fn sketchy() -> BitFlags<Self> {
    Purity::Sketchy.into()
  }

  pub fn nsfw() -> BitFlags<Self> {
    Purity::Nsfw.into()
  }
}
