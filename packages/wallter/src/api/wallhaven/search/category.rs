use enumflags2::{BitFlags, bitflags};
use serde::{Deserialize, Serialize};
use std::{
  collections::HashSet,
  fmt::{self, Display, Formatter}
};
use urlencoding::encode;

// -- Enums for Type-Safe Search Parameters --

/// Categories for filtering wallpapers.
///
/// Can be combined using bitwise operations (e.g., `Category::General |
/// Category::Anime`).
///
/// # Examples
///
/// ```rust
/// // Single category
/// let cats = Category::General;
///
/// // Multiple categories
/// let cats = Category::General | Category::Anime;
///
/// // All categories
/// let cats = Category::General | Category::Anime | Category::People;
#[bitflags]
#[repr(u8)]
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Category {
  /// General wallpapers (landscapes, abstract, etc.)
  General = 0b100,
  /// Anime/manga wallpapers
  Anime = 0b010,
  /// People/portraits wallpapers
  People = 0b001
}

impl Category {
  pub fn default() -> BitFlags<Self> {
    Category::all()
  }

  pub fn all() -> BitFlags<Self> {
    Category::General | Category::Anime | Category::People
  }

  pub fn no_people() -> BitFlags<Self> {
    Category::General | Category::Anime
  }

  pub fn no_anime() -> BitFlags<Self> {
    Category::General | Category::People
  }

  pub fn no_general() -> BitFlags<Self> {
    Category::Anime | Category::People
  }

  pub fn general() -> BitFlags<Self> {
    Category::General.into()
  }

  pub fn anime() -> BitFlags<Self> {
    Category::Anime.into()
  }

  pub fn people() -> BitFlags<Self> {
    Category::People.into()
  }
}
