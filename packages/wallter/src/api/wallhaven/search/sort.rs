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

/// Predefined set of allowed color values in hex format.
///
/// These are the only colors supported by the Wallhaven API.
/// Colors must be provided in hex format (with or without the # prefix).
pub const COLORS: &[&str] = &[
  "#660000", "#990000", "#cc0000", "#cc3333", "#ea4c88", "#993399", "#663399",
  "#333399", "#0066cc", "#0099cc", "#66cccc", "#77cc33", "#669900", "#336600",
  "#666600", "#999900", "#cccc33", "#ffff00", "#ffcc33", "#ff9900", "#ff6600",
  "#cc6633", "#996633", "#663300", "#000000", "#999999", "#cccccc", "#ffffff",
  "#424153"
];

// -- Enums for Type-Safe Search Parameters --

/// Available sorting methods for search results.
///
/// Different sorting methods affect how wallpapers are ordered in the results.
/// Some sorting methods may require additional parameters (e.g., `Toplist` can
/// use `ToplistRange`).
#[derive(
  Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize,
)]
pub enum Sorting {
  /// Sort by date added to Wallhaven
  DateAdded,
  /// Sort by relevance to search query
  Relevance,
  /// Random order (can use seed for consistency)
  Random,
  /// Sort by view count
  Views,
  /// Sort by number of favorites
  Favorites,
  /// Sort by toplist ranking (default)
  #[default]
  Toplist
}

impl Display for Sorting {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let s = match self {
      Sorting::DateAdded => "date_added",
      Sorting::Relevance => "relevance",
      Sorting::Random => "random",
      Sorting::Views => "views",
      Sorting::Favorites => "favorites",
      Sorting::Toplist => "toplist"
    };
    write!(f, "{s}")
  }
}

/// Sorting order for search results.
///
/// Determines whether results are sorted in ascending or descending order.
#[derive(
  Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize,
)]
pub enum Order {
  /// Descending order (highest to lowest)
  #[default]
  Desc,
  /// Ascending order (lowest to highest)
  Asc
}

impl Display for Order {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let s = match self {
      Order::Desc => "desc",
      Order::Asc => "asc"
    };
    write!(f, "{s}")
  }
}

/// Time range for toplist sorting.
///
/// When using `Sorting::Toplist`, this determines the time period
/// for calculating the toplist rankings.
///
/// # Examples
///
/// ```rust
/// // Get top wallpapers from the last week
/// let params = Parameters::new()
///     .with_top_range(ToplistRange::Week);
///
/// // Get top wallpapers from the last month (default)
/// let params = Parameters::new()
///     .with_top_range(ToplistRange::Month);
/// ```
#[derive(
  Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize,
)]
pub enum ToplistRange {
  /// Last 24 hours
  Day,
  /// Last 3 days
  Days3,
  /// Last week
  Week,
  /// Last month (default)
  #[default]
  Month,
  /// Last 3 months
  Months3,
  /// Last 6 months
  Months6,
  /// Last year
  Year
}

impl Display for ToplistRange {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let s = match self {
      ToplistRange::Day => "1d",
      ToplistRange::Days3 => "3d",
      ToplistRange::Week => "1w",
      ToplistRange::Month => "1M",
      ToplistRange::Months3 => "3M",
      ToplistRange::Months6 => "6M",
      ToplistRange::Year => "1y"
    };
    write!(f, "{s}")
  }
}
