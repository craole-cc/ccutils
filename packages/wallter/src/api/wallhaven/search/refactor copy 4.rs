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

/// URL struct for handling Wallhaven endpoints
///
/// Represents a Wallhaven URL with its interface type and full address.
/// Used to manage different endpoint URLs (web interface vs API) in a type-safe
/// way.
///
/// # Fields
///
/// * `interface` - The interface type (Web or API)
/// * `address` - The complete URL string
///
/// # Examples
///
/// ```rust
/// // Create a default web interface URL
/// let url = Url::default();
/// assert_eq!(url.address, "https://wallhaven.cc/search?");
///
/// // Create an API endpoint URL
/// let url = Url::new(Interface::Api);
/// assert_eq!(url.address, "https://wallhaven.cc/api/v1/search?");
/// ```

#[derive(Debug, Clone)]
pub struct Url {
  pub interface: Interface,
  pub address: String
}

impl Default for Url {
  fn default() -> Self {
    Self::new(Interface::default())
  }
}

impl Url {
  /// Creates a new URL instance with the specified interface type
  ///
  /// # Arguments
  ///
  /// * `interface` - The interface type to use (Web or API)
  ///
  /// # Returns
  ///
  /// A new Url instance with the corresponding address
  pub fn new(interface: Interface) -> Self {
    let address = interface.to_string();
    Self { interface, address }
  }

  /// Updates the URL with a new interface type
  ///
  /// # Arguments
  ///
  /// * `interface` - The new interface type to use
  ///
  /// # Returns
  ///
  /// Self with updated interface and address
  pub fn with_interface(mut self, interface: Interface) -> Self {
    self.interface = interface;
    self.address = interface.to_string();
    self
  }

  pub fn with_address(mut self, address: String) -> Self {
    self.address = address;
    self
  }
}

/// URL type for Wallhaven requests.
///
/// Determines whether to generate URLs for the web interface or API endpoints.
#[derive(
  Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize,
)]
pub enum Interface {
  /// Web interface URL (https://wallhaven.cc/search?)
  #[default]
  Web,
  /// API endpoint URL (https://wallhaven.cc/api/v1/search?)
  Api
}

impl Display for Interface {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Interface::Web => "https://wallhaven.cc/search?",
        Interface::Api => "https://wallhaven.cc/api/v1/search?"
      }
    )
  }
}

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

// -- Search Parameters Builder --

/// Main parameter builder for Wallhaven API searches.
///
/// Provides a fluent, type-safe interface for building Wallhaven search
/// parameters. Uses the builder pattern to allow method chaining and provides
/// sensible defaults.
///
/// # Default Values
///
/// - **Categories**: All categories enabled (General | Anime | People)
/// - **Purity**: Safe for work only
/// - **Sorting**: Toplist
/// - **Order**: Descending
/// - **URL Type**: Web interface
///
/// # Examples
///
/// ```rust
/// // Basic search
/// let params = Parameters::new()
///     .with_query(vec!["nature", "landscape"])
///     .with_categories(Category::General.into())
///     .with_page(1);
///
/// // Advanced search with multiple filters
/// let params = Parameters::new()
///     .with_query(vec!["+anime", "-girl"])
///     .with_categories(Category::Anime.into())
///     .with_purity(Purity::Sfw | Purity::Sketchy)
///     .with_atleast("1920x1080")?
///     .with_colors(vec!["#663399", "#000000"])
///     .with_sorting(Sorting::Favorites)
///     .with_order(Order::Desc)
///     .with_page(2);
///
/// let url = params.build_url();
/// ```
#[derive(Debug, Clone)]
pub struct Parameters {
  /// Search query terms
  pub query: Option<Vec<String>>,
  /// Category filter flags
  pub categories: BitFlags<Category>,
  /// Purity level filter flags
  pub purity: BitFlags<Purity>,
  /// Sorting method
  pub sorting: Sorting,
  /// Sort order
  pub order: Order,
  /// Time range for toplist sorting
  pub top_range: Option<ToplistRange>,
  /// Resolution constraints
  pub resolution: Option<Resolution>,
  /// Aspect ratio filters
  pub ratios: Option<Vec<String>>,
  /// Color filters
  pub colors: Option<Vec<String>>,
  /// Page number for pagination
  pub page: Option<u32>,
  /// Seed for consistent random sorting
  pub seed: Option<String>,
  /// URL interface and address
  pub url: Url
}

impl Default for Parameters {
  fn default() -> Self {
    // let url_type = Interface::default();
    // let url = format!("{url_type}");

    Self {
      query: None,
      categories: Category::default(),
      purity: Purity::default(),
      sorting: Sorting::default(),
      order: Order::default(),
      top_range: None,
      resolution: None,
      ratios: None,
      colors: None,
      page: None,
      seed: None,
      url: Url::default()
    }
  }
}

impl Parameters {
  /// Creates a new `Parameters` instance with default values.
  ///
  /// # Returns
  ///
  /// A new `Parameters` instance with sensible defaults:
  /// - All categories enabled
  /// - Safe for work content only
  /// - Toplist sorting in descending order
  /// - Web interface URL
  ///
  /// # Examples
  ///
  /// ```rust
  /// let params = Parameters::new();
  /// assert_eq!(params.sorting, Sorting::Toplist);
  /// assert_eq!(params.order, Order::Desc);
  /// ```
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the search query for Wallhaven.
  ///
  /// The query can be a single string or multiple parts that will be joined
  /// with spaces. Supports Wallhaven's advanced search syntax including:
  /// - `+term` - Must include term
  /// - `-term` - Must exclude term
  /// - `id:123` - Search by wallpaper ID
  /// - `@username` - Search by uploader username
  /// - `type:png` - Search by file type
  ///
  /// # Arguments
  ///
  /// * `query` - An iterable of search terms
  ///
  /// # Examples
  ///
  /// ```rust
  /// // Single term
  /// let params = Parameters::new().with_query(vec!["anime"]);
  ///
  /// // Multiple terms with operators
  /// let params = Parameters::new().with_query(vec!["+blue", "-girl"]);
  ///
  /// // Special searches
  /// let params = Parameters::new().with_query(vec!["id:123"]);
  /// let params = Parameters::new().with_query(vec!["@username"]);
  /// ```
  pub fn with_query<Q, S>(mut self, query: Q) -> Self
  where
    Q: IntoIterator<Item = S>,
    S: AsRef<str>
  {
    self.query =
      Some(query.into_iter().map(|s| s.as_ref().to_string()).collect());
    self
  }

  /// Sets the categories to search using BitFlags.
  ///
  /// Categories can be combined using bitwise OR operations to search
  /// multiple categories simultaneously.
  ///
  /// # Arguments
  ///
  /// * `categories` - BitFlags containing the desired categories
  ///
  /// # Examples
  ///
  /// ```rust
  /// // Single category
  /// let params = Parameters::new()
  ///     .with_categories(Category::General.into());
  ///
  /// // Multiple categories
  /// let params = Parameters::new()
  ///     .with_categories(Category::General | Category::Anime);
  ///
  /// // All categories
  /// let params = Parameters::new()
  ///     .with_categories(Category::General | Category::Anime | Category::People);
  /// ```
  pub fn with_categories(mut self, categories: BitFlags<Category>) -> Self {
    self.categories = categories;
    self
  }

  /// Sets the categories to search using a boolean tuple.
  ///
  /// Provides a more intuitive interface for category selection using
  /// a tuple of boolean values: (General, Anime, People).
  ///
  /// # Arguments
  ///
  /// * `categories` - A tuple of (General, Anime, People) boolean flags
  ///
  /// # Examples
  ///
  /// ```rust
  /// // Only General category
  /// let params = Parameters::new()
  ///     .with_categories_tuple((true, false, false));
  ///
  /// // General and Anime categories
  /// let params = Parameters::new()
  ///     .with_categories_tuple((true, true, false));
  ///
  /// // All categories
  /// let params = Parameters::new()
  ///     .with_categories_tuple((true, true, true));
  /// ```
  pub fn with_categories_tuple(
    mut self,
    categories: (bool, bool, bool)
  ) -> Self {
    let mut flags = BitFlags::empty();
    if categories.0 {
      flags |= Category::General;
    }
    if categories.1 {
      flags |= Category::Anime;
    }
    if categories.2 {
      flags |= Category::People;
    }
    self.categories = flags;
    self
  }

  /// Sets the purity levels to search using BitFlags.
  ///
  /// Purity levels can be combined using bitwise OR operations.
  /// Note that NSFW content requires a valid API key to access.
  ///
  /// # Arguments
  ///
  /// * `purity` - BitFlags containing the desired purity levels
  ///
  /// # Examples
  ///
  /// ```rust
  /// // Safe for work only
  /// let params = Parameters::new()
  ///     .with_purity(Purity::Sfw.into());
  ///
  /// // SFW and sketchy content
  /// let params = Parameters::new()
  ///     .with_purity(Purity::Sfw | Purity::Sketchy);
  ///
  /// // All content types (requires API key for NSFW)
  /// let params = Parameters::new()
  ///     .with_purity(Purity::Sfw | Purity::Sketchy | Purity::Nsfw);
  /// ```
  pub fn with_purity(mut self, purity: BitFlags<Purity>) -> Self {
    self.purity = purity;
    self
  }

  /// Sets the purity levels to search using a boolean tuple.
  ///
  /// Provides a more intuitive interface for purity selection using
  /// a tuple of boolean values: (SFW, Sketchy, NSFW).
  ///
  /// # Arguments
  ///
  /// * `purities` - A tuple of (SFW, Sketchy, NSFW) boolean flags
  ///
  /// # Examples
  ///
  /// ```rust
  /// // Safe for work only
  /// let params = Parameters::new()
  ///     .with_purity_tuple((true, false, false));
  ///
  /// // SFW and sketchy content
  /// let params = Parameters::new()
  ///     .with_purity_tuple((true, true, false));
  ///
  /// // All content types (requires API key for NSFW)
  /// let params = Parameters::new()
  ///     .with_purity_tuple((true, true, true));
  /// ```
  pub fn with_purity_tuple(mut self, purities: (bool, bool, bool)) -> Self {
    let mut flags = BitFlags::empty();
    if purities.0 {
      flags |= Purity::Sfw;
    }
    if purities.1 {
      flags |= Purity::Sketchy;
    }
    if purities.2 {
      flags |= Purity::Nsfw;
    }
    self.purity = flags;
    self
  }

  /// Sets the sorting method for search results.
  ///
  /// Different sorting methods affect how wallpapers are ordered.
  /// Some methods may benefit from additional parameters.
  ///
  /// # Arguments
  ///
  /// * `sorting` - The sorting method to use
  ///
  /// # Examples
  ///
  /// ```rust
  /// // Sort by relevance to search query
  /// let params = Parameters::new()
  ///     .with_sorting(Sorting::Relevance);
  ///
  /// // Sort by number of favorites
  /// let params = Parameters::new()
  ///     .with_sorting(Sorting::Favorites);
  ///
  /// // Random sorting (consider using with_seed for consistency)
  /// let params = Parameters::new()
  ///     .with_sorting(Sorting::Random)
  ///     .with_seed("my-seed");
  /// ```
  pub fn with_sorting(mut self, sorting: Sorting) -> Self {
    self.sorting = sorting;
    self
  }

  /// Sets the sorting order for search results.
  ///
  /// # Arguments
  ///
  /// * `order` - The sort order (ascending or descending)
  ///
  /// # Examples
  ///
  /// ```rust
  /// // Descending order (default)
  /// let params = Parameters::new()
  ///     .with_order(Order::Desc);
  ///
  /// // Ascending order
  /// let params = Parameters::new()
  ///     .with_order(Order::Asc);
  /// ```
  pub fn with_order(mut self, order: Order) -> Self {
    self.order = order;
    self
  }

  /// Sets the top list time range and automatically enables toplist sorting.
  ///
  /// This method automatically sets the sorting method to `Sorting::Toplist`
  /// and configures the time range for toplist calculation.
  ///
  /// # Arguments
  ///
  /// * `range` - The time range for toplist calculation
  ///
  /// # Examples
  ///
  /// ```rust
  /// // Top wallpapers from the last week
  /// let params = Parameters::new()
  ///     .with_top_range(ToplistRange::Week);
  ///
  /// // Top wallpapers from the last month
  /// let params = Parameters::new()
  ///     .with_top_range(ToplistRange::Month);
  ///
  /// // Top wallpapers from the last year
  /// let params = Parameters::new()
  ///     .with_top_range(ToplistRange::Year);
  /// ```
  pub fn with_top_range(mut self, range: ToplistRange) -> Self {
    self.sorting = Sorting::Toplist;
    self.top_range = Some(range);
    self
  }

  /// Sets the minimum resolution constraint.
  ///
  /// Finds wallpapers with resolution greater than or equal to the specified
  /// value. This method validates the resolution format and will return an
  /// error for invalid formats. Setting a minimum resolution will replace any
  /// exact resolution constraints.
  ///
  /// # Arguments
  ///
  /// * `resolution` - The minimum resolution in "WIDTHxHEIGHT" format
  ///
  /// # Returns
  ///
  /// * `Ok(Self)` if the resolution format is valid
  /// * `Err(String)` with a descriptive error message if invalid
  ///
  /// # Examples
  ///
  /// ```rust
  /// // Minimum 1080p resolution
  /// let params = Parameters::new()
  ///     .with_atleast("1920x1080")?;
  ///
  /// // Minimum 4K resolution
  /// let params = Parameters::new()
  ///     .with_atleast("3840x2160")?;
  ///
  /// // Invalid format will return an error
  /// let result = Parameters::new().with_atleast("invalid");
  /// assert!(result.is_err());
  /// ```
  pub fn with_atleast(
    mut self,
    resolution: impl Into<String>
  ) -> Result<Self, String> {
    let resolution = resolution.into();
    Resolution::validate_format(&resolution)?;
    self.resolution = Some(Resolution::AtLeast(resolution));
    Ok(self)
  }

  /// Sets exact resolution constraints.
  ///
  /// Finds wallpapers that exactly match one of the specified resolutions.
  /// This method validates all resolution formats and will return an error if
  /// any format is invalid. Setting exact resolutions will replace any
  /// minimum resolution constraint.
  ///
  /// # Arguments
  ///
  /// * `resolutions` - An iterable of exact resolutions in "WIDTHxHEIGHT"
  ///   format
  ///
  /// # Returns
  ///
  /// * `Ok(Self)` if all resolution formats are valid
  /// * `Err(String)` with a descriptive error message if any format is invalid
  ///
  /// # Examples
  ///
  /// ```rust
  /// // Common HD resolutions
  /// let params = Parameters::new()
  ///     .with_resolutions(vec!["1920x1080", "2560x1440"])?;
  ///
  /// // Multiple 4K variants
  /// let params = Parameters::new()
  ///     .with_resolutions(vec!["3840x2160", "4096x2160"])?;
  ///
  /// // Invalid format will return an error
  /// let result = Parameters::new().with_resolutions(vec!["invalid"]);
  /// assert!(result.is_err());
  /// ```
  pub fn with_resolutions<Res, Str>(
    mut self,
    resolutions: Res
  ) -> Result<Self, String>
  where
    Res: IntoIterator<Item = Str>,
    Str: AsRef<str>
  {
    let resolutions: Vec<String> = resolutions
      .into_iter()
      .map(|s| s.as_ref().to_string())
      .collect();

    // Validate all resolutions
    for resolution in &resolutions {
      Resolution::validate_format(resolution)?;
    }

    self.resolution = Some(Resolution::Exact(resolutions));
    Ok(self)
  }

  /// Sets aspect ratio constraints.
  ///
  /// Filters wallpapers by their aspect ratio. Common ratios include
  /// "16x9", "16x10", "4x3", "21x9", etc.
  ///
  /// # Arguments
  ///
  /// * `ratios` - An iterable of aspect ratios in "WIDTHxHEIGHT" format
  ///
  /// # Examples
  ///
  /// ```rust
  /// // Widescreen ratios
  /// let params = Parameters::new()
  ///     .with_ratios(vec!["16x9", "21x9"]);
  ///
  /// // Traditional ratios
  /// let params = Parameters::new()
  ///     .with_ratios(vec!["4x3", "5x4"]);
  ///
  /// // Ultra-wide ratio
  /// let params = Parameters::new()
  ///     .with_ratios(vec!["21x9"]);
  /// ```
  pub fn with_ratios<Rat, Str>(mut self, ratios: Rat) -> Self
  where
    Rat: IntoIterator<Item = Str>,
    Str: AsRef<str>
  {
    self.ratios =
      Some(ratios.into_iter().map(|s| s.as_ref().to_string()).collect());
    self
  }

  /// Sets color constraints for wallpaper filtering.
  ///
  /// Filters wallpapers by their dominant colors. Colors can be provided
  /// with or without the '#' prefix. Invalid colors are automatically
  /// filtered out and will not cause errors.
  ///
  /// # Arguments
  ///
  /// * `colors` - An iterable of color values in hex format
  ///
  /// # Examples
  ///
  /// ```rust
  /// // Colors with hash prefix
  /// let params = Parameters::new()
  ///     .with_colors(vec!["#663399", "#000000"]);
  ///
  /// // Colors without hash prefix
  /// let params = Parameters::new()
  ///     .with_colors(vec!["663399", "000000"]);
  ///
  /// // Mixed format (invalid colors are filtered out)
  /// let params = Parameters::new()
  ///     .with_colors(vec!["#663399", "invalid", "000000"]);
  /// ```
  pub fn with_colors<Clr, Str>(mut self, colors: Clr) -> Self
  where
    Clr: IntoIterator<Item = Str>,
    Str: AsRef<str>
  {
    let valid_colors: HashSet<&str> = COLORS.iter().copied().collect();

    let filtered_colors: Vec<String> = colors
      .into_iter()
      .filter_map(|s| {
        let color = s.as_ref();
        let normalized = if color.starts_with('#') {
          color.to_string()
        } else {
          format!("#{color}")
        };

        if valid_colors.contains(normalized.as_str()) {
          Some(
            normalized
              .strip_prefix('#')
              .unwrap_or(&normalized)
              .to_string()
          )
        } else {
          None
        }
      })
      .collect();

    if !filtered_colors.is_empty() {
      self.colors = Some(filtered_colors);
    }
    self
  }

  /// Sets the pagination page number.
  ///
  /// Used for navigating through multiple pages of search results.
  /// Page numbers start from 1.
  ///
  /// # Arguments
  ///
  /// * `page` - The page number (1-based)
  ///
  /// # Examples
  ///
  /// ```rust
  /// // First page (default)
  /// let params = Parameters::new()
  ///     .with_page(1);
  ///
  /// // Second page
  /// let params = Parameters::new()
  ///     .with_page(2);
  ///
  /// // Navigate to specific page
  /// let params = Parameters::new()
  ///     .with_page(10);
  /// ```
  pub fn with_page(mut self, page: u32) -> Self {
    self.page = Some(page);
    self
  }

  /// Provides a seed for random sorting to get consistent results across pages.
  pub fn with_seed(mut self, seed: impl Into<String>) -> Self {
    self.seed = Some(seed.into());
    self
  }

  /// Sets the URL type (Web or API).
  pub fn with_url_type(mut self, interface: Interface) -> Self {
    self.url = Url::new(interface);
    self
  }

  /// Builds the complete URL with all parameters.
  pub fn url(mut self) -> Self {
    // let mut url = self.url.address;
    let mut url = self.url.address.clone();

    if let Some(query) = &self.query {
      let joined = query.join(" ");
      let encoded = encode(&joined);
      url.push_str(&format!("q={encoded}&"));
    }

    url.push_str(&format!("categories={:0>3b}&", self.categories.bits()));
    url.push_str(&format!("purity={:0>3b}&", self.purity.bits()));
    url.push_str(&format!("sorting={}&", self.sorting));
    url.push_str(&format!("order={}&", self.order));

    match &self.resolution {
      Some(Resolution::AtLeast(res)) => {
        url.push_str(&format!("atleast={res}&"));
      }
      Some(Resolution::Exact(resolutions)) => {
        let joined = resolutions.join(",");
        let encoded = encode(&joined);
        url.push_str(&format!("resolutions={encoded}&"));
      }
      None => {}
    }

    if let Some(ratios) = &self.ratios {
      let joined = ratios.join(",");
      let encoded = encode(&joined);
      url.push_str(&format!("ratios={encoded}&"));
    }

    if let Some(top_range) = &self.top_range {
      url.push_str(&format!("topRange={top_range}&"));
    }

    if let Some(colors) = &self.colors {
      let joined = colors.join(",");
      let encoded = encode(&joined);
      url.push_str(&format!("colors={encoded}&"));
    }

    if let Some(page) = &self.page {
      url.push_str(&format!("page={page}&"));
    }

    if let Some(seed) = &self.seed {
      url.push_str(&format!("seed={seed}&"));
    }

    // Remove trailing '&' if present
    if url.ends_with('&') {
      url.pop();
    }

    self.url = self.url.with_address(url);
    self
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::prelude::*;

  #[test]
  fn test_params() {
    init_logging();
    let params = Parameters::new()
      // .with_query(vec!["+nature", "-landscape", "-sea"])
      .with_categories(Category::no_anime())
      .with_purity(Purity::default())
      .with_ratios(vec!["16x9"])
      .with_order(Order::Asc)
      .with_resolutions(vec![
        "1920x1080",
        "2560x1440",
        "2500x4200",
        "3840x1920",
      ])
      .expect("Invalid resolution format")
      // .with_atleast("1920x1080")
      // .expect("Invalid resolution format")
      .with_colors(vec![
        "#663399",
        "invalid",
        "#nonexistent",
        "000000",
        "#ff6600",
      ])
      .with_page(2)
      // .with_seed("m1seed")
      .url();
    debug!("{:#?}", &params);

    let url = params.url.address;
    // assert!(url.contains("q=%2Bnature%20-landscape%20-sea"));
    // assert!(url.contains("categories=101"));
    assert!(url.contains("purity=100"));
    assert!(url.contains("ratios=16x9"));
    assert!(url.contains("order=asc"));
    assert!(url.contains("page=2"));
    // assert!(url.contains("atleast=1920x1080"));
    assert!(url.contains(&format!(
      "resolutions={}",
      encode("1920x1080,2560x1440,2500x4200,3840x1920")
    )));
    assert!(
      url.contains(&format!("colors={}", encode("663399,000000,ff6600")))
    );
  }

  // with_logging!(test_params, {
  //   let params = Parameters::new()
  //     // .with_query(vec!["+nature", "-landscape", "-sea"])
  //     .with_categories(Category::no_anime())
  //     .with_purity(Purity::default())
  //     .with_ratios(vec!["16x9"])
  //     .with_order(Order::Asc)
  //     .with_resolutions(vec![
  //       "1920x1080",
  //       "2560x1440",
  //       "2500x4200",
  //       "3840x1920",
  //     ])
  //     .expect("Invalid resolution format")
  //     // .with_atleast("1920x1080")
  //     // .expect("Invalid resolution format")
  //     .with_colors(vec![
  //       "#663399",
  //       "invalid",
  //       "#nonexistent",
  //       "000000",
  //       "#ff6600",
  //     ])
  //     .with_page(2)
  //     // .with_seed("m1seed")
  //     .url();
  //   eprintln!("{:#?}", &params);

  //   let url = params.url.address;
  //   // assert!(url.contains("q=%2Bnature%20-landscape%20-sea"));
  //   // assert!(url.contains("categories=101"));
  //   assert!(url.contains("purity=110"));
  //   assert!(url.contains("ratios=16x9"));
  //   assert!(url.contains("order=asc"));
  //   assert!(url.contains("page=2"));
  //   // assert!(url.contains("atleast=1920x1080"));
  //   assert!(url.contains(&format!(
  //     "resolutions={}",
  //     encode("1920x1080,2560x1440,2500x4200,3840x1920")
  //   )));
  //   assert!(
  //     url.contains(&format!("colors={}", encode("663399,000000,ff6600")))
  //   );
  // });

  #[test]
  fn test_params_basic() {
    let params = Parameters::new()
      .with_query(vec!["+nature", "-landscape", "-sea"])
      .with_categories(Category::General.into())
      .with_ratios(vec!["16x9"])
      .url();

    let url = params.url.address;
    assert!(url.contains("q=%2Bnature%20-landscape%20-sea"));
    assert!(url.contains("categories=100"));
    assert!(url.contains("ratios=16x9"));
  }

  #[test]
  fn test_resolution_validation() {
    // Valid resolution
    let params = Parameters::new().with_atleast("1920x1080").unwrap();
    assert!(matches!(params.resolution, Some(Resolution::AtLeast(_))));

    // Invalid resolution
    let result = Parameters::new().with_atleast("invalid");
    assert!(result.is_err());
  }

  #[test]
  fn test_resolution_mutual_exclusion() {
    // Test that setting atleast replaces exact resolutions
    let params = Parameters::new()
      .with_resolutions(vec!["1920x1080", "2560x1440"])
      .unwrap()
      .with_atleast("1920x1080")
      .unwrap();

    assert!(matches!(params.resolution, Some(Resolution::AtLeast(_))));

    // Test that setting exact resolutions replaces atleast
    let params = Parameters::new()
      .with_atleast("1920x1080")
      .unwrap()
      .with_resolutions(vec!["1920x1080", "2560x1440"])
      .unwrap();

    assert!(matches!(params.resolution, Some(Resolution::Exact(_))));
  }

  #[test]
  fn test_color_filtering() {
    let params = Parameters::new().with_colors(vec![
      "#663399",
      "invalid",
      "000000",
      "#nonexistent",
    ]);

    // Should only contain valid colors
    assert!(params.colors.is_some());
    let colors = params.colors.unwrap();
    assert!(colors.contains(&"663399".to_string()));
    assert!(colors.contains(&"000000".to_string()));
    assert_eq!(colors.len(), 2); // Only valid colors should remain
  }

  #[test]
  fn test_bitflags_usage() {
    let params = Parameters::new()
      .with_categories(Category::General | Category::Anime)
      .with_purity(Purity::Sfw | Purity::Sketchy);

    assert_eq!(params.categories, Category::General | Category::Anime);
    assert_eq!(params.purity, Purity::Sfw | Purity::Sketchy);
  }

  #[test]
  fn test_default_values() {
    let params = Parameters::new();

    // Default should be all categories enabled
    assert_eq!(
      params.categories,
      Category::General | Category::Anime | Category::People
    );

    // Default should be only SFW enabled
    assert_eq!(params.purity, BitFlags::<Purity>::from_flag(Purity::Sfw));
  }

  #[test]
  fn test_url_building() {
    let params = Parameters::new()
      .with_query(vec!["test"])
      .with_page(2)
      .url();

    let url = params.url.address;
    assert!(url.starts_with("https://wallhaven.cc/search?"));
    assert!(url.contains("q=test"));
    assert!(url.contains("page=2"));
    assert!(!url.ends_with('&')); // Should not end with trailing ampersand
  }
}
