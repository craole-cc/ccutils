//! Wallhaven API v1 Client
//!
//! Provides a comprehensive and type-safe interface for interacting with the
//! Wallhaven.cc API. It handles authentication, parameter validation, and
//! deserialization of API responses.

use crate::{Error, Result};
use reqwest::Client;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{self, Display, Formatter};

// -- Data Structures for API Responses --

/// Represents the top-level structure for paginated responses (e.g., search,
/// collections).
#[derive(Debug, Deserialize)]
pub struct PaginatedResponse {
  pub data: Vec<Wallpaper>,
  pub meta: Meta
}

/// Represents the top-level structure for a single wallpaper details response.
#[derive(Debug, Deserialize)]
pub struct WallpaperDetailsResponse {
  pub data: Wallpaper
}

/// Represents a single wallpaper from the Wallhaven API.
/// This struct includes all fields from both search results and detailed views.
#[derive(Debug, Deserialize, Clone)]
pub struct Wallpaper {
  pub id: String,
  pub url: String,
  pub short_url: String,
  pub views: u32,
  pub favorites: u32,
  pub source: String,
  pub purity: String,
  pub category: String,
  pub dimension_x: u32,
  pub dimension_y: u32,
  pub resolution: String,
  pub ratio: String,
  pub file_size: u64,
  pub file_type: String,
  pub created_at: String,
  pub colors: Vec<String>,
  pub path: String,
  pub thumbs: Thumbnails,
  // The 'tags' field is only present in the detailed wallpaper view
  // (`/w/{id}`). It is optional to handle both search results and detailed
  // views with one struct.
  pub tags: Option<Vec<Tag>>
}

/// Represents the thumbnails for a wallpaper.
#[derive(Debug, Deserialize, Clone)]
pub struct Thumbnails {
  pub large: String,
  pub original: String,
  pub small: String
}

/// Represents a tag associated with a wallpaper.
#[derive(Debug, Deserialize, Clone)]
pub struct Tag {
  pub id: u32,
  pub name: String,
  pub alias: String,
  pub category_id: u32,
  pub category: String,
  pub purity: String,
  pub created_at: String
}

/// Represents metadata for a paginated API response.
#[derive(Debug, Deserialize)]
pub struct Meta {
  pub current_page: u32,
  pub last_page: u32,
  pub per_page: u32,
  pub total: u32,
  pub query: Option<String>,
  pub seed: Option<String>
}

// -- Enums for Type-Safe Search Parameters --

/// Categories for filtering wallpapers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
  General = 0,
  Anime = 1,
  People = 2
}

/// Purity levels for filtering wallpapers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Purity {
  Sfw = 0,
  Sketchy = 1,
  Nsfw = 2
}

/// Available sorting methods for search results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sorting {
  DateAdded,
  Relevance,
  Random,
  Views,
  Favorites,
  Toplist
}

impl Display for Sorting {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Sorting::DateAdded => "date_added",
        Sorting::Relevance => "relevance",
        Sorting::Random => "random",
        Sorting::Views => "views",
        Sorting::Favorites => "favorites",
        Sorting::Toplist => "toplist"
      }
    )
  }
}

/// Sorting order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Order {
  Desc,
  Asc
}

impl fmt::Display for Order {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Order::Desc => "desc",
        Order::Asc => "asc"
      }
    )
  }
}

/// Time range for `toplist` sorting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToplistRange {
  Day,     // 1d
  Days3,   // 3d
  Week,    // 1w
  Month,   // 1M
  Months3, // 3M
  Months6, // 6M
  Year     // 1y
}

impl fmt::Display for ToplistRange {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        ToplistRange::Day => "1d",
        ToplistRange::Days3 => "3d",
        ToplistRange::Week => "1w",
        ToplistRange::Month => "1M",
        ToplistRange::Months3 => "3M",
        ToplistRange::Months6 => "6M",
        ToplistRange::Year => "1y"
      }
    )
  }
}

// -- Search Parameters Builder --

/// Represents the parameters for a Wallhaven API search.
/// Use the builder methods to construct a search query.
#[derive(Debug, Default, Clone)]
pub struct SearchParams {
  pub query: Option<String>,
  pub categories: Option<(bool, bool, bool)>,
  pub purity: Option<(bool, bool, bool)>,
  pub sorting: Option<Sorting>,
  pub order: Option<Order>,
  pub top_range: Option<ToplistRange>,
  pub atleast: Option<String>,
  pub resolutions: Option<String>,
  pub ratios: Option<String>,
  pub colors: Option<String>,
  pub page: Option<u32>,
  pub seed: Option<String>
}

impl SearchParams {
  /// Creates a new, empty `SearchParams` instance.
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the search query.
  /// Example: `"anime"`, `"+blue -girl"`, `"id:123"`, `"@username"`
  pub fn with_query(mut self, query: impl Into<String>) -> Self {
    self.query = Some(query.into());
    self
  }

  /// Sets the categories to search. Tuple is (General, Anime, People).
  /// Example: `(true, true, false)` for General and Anime.
  pub fn with_categories(mut self, cats: (bool, bool, bool)) -> Self {
    self.categories = Some(cats);
    self
  }

  /// Sets the purity levels to search. Tuple is (SFW, Sketchy, NSFW).
  /// **Note:** NSFW requires a valid API key.
  pub fn with_purity(mut self, purities: (bool, bool, bool)) -> Self {
    self.purity = Some(purities);
    self
  }

  /// Sets the sorting method.
  pub fn with_sorting(mut self, sorting: Sorting) -> Self {
    self.sorting = Some(sorting);
    self
  }

  /// Sets the sorting order (asc/desc).
  pub fn with_order(mut self, order: Order) -> Self {
    self.order = Some(order);
    self
  }

  /// Sets the top list time range. **Requires `sorting` to be
  /// `Sorting::Toplist`**.
  pub fn with_top_range(mut self, range: ToplistRange) -> Self {
    self.top_range = Some(range);
    self
  }

  /// Sets the minimum resolution. Example: `"1920x1080"`.
  pub fn with_atleast(mut self, resolution: impl Into<String>) -> Self {
    self.atleast = Some(resolution.into());
    self
  }

  /// Sets a list of exact resolutions. Example: `"1920x1080,2560x1440"`.
  pub fn with_resolutions(mut self, resolutions: impl Into<String>) -> Self {
    self.resolutions = Some(resolutions.into());
    self
  }

  /// Sets a list of aspect ratios. Example: `"16x9,16x10"`.
  pub fn with_ratios(mut self, ratios: impl Into<String>) -> Self {
    self.ratios = Some(ratios.into());
    self
  }

  /// Sets a color to search by. Example: `"663399"`.
  pub fn with_colors(mut self, color: impl Into<String>) -> Self {
    self.colors = Some(color.into());
    self
  }

  /// Sets the pagination page number.
  pub fn with_page(mut self, page: u32) -> Self {
    self.page = Some(page);
    self
  }

  /// Provides a seed for random sorting to get consistent results across pages.
  pub fn with_seed(mut self, seed: impl Into<String>) -> Self {
    self.seed = Some(seed.into());
    self
  }
}

/// The main Wallhaven API client.
pub struct Api {
  client: Client,
  base_url: String,
  api_key: Option<String>
}

impl Api {
  /// Creates a new Wallhaven API client.
  ///
  /// # Arguments
  /// * `api_key` - An optional API key for authenticated requests.
  pub fn new(api_key: Option<String>) -> Self {
    Self {
      client: Client::new(),
      base_url: "https://wallhaven.cc/api/v1".to_string(),
      api_key
    }
  }

  /// Checks if an API key is configured.
  fn has_api_key(&self) -> bool {
    self.api_key.is_some()
  }

  /// Sends a request, handling authentication and error responses.
  async fn send_request<T: for<'de> Deserialize<'de>>(
    &self,
    url: String,
    params: &[(&str, String)]
  ) -> Result<T> {
    let mut request = self.client.get(&url).query(params);

    // Add API key to header if available.
    // The API also allows it as a query param `?apikey=...`, but header is
    // cleaner.
    if let Some(key) = &self.api_key {
      request = request.header("X-API-Key", key);
    }

    let response = request.send().await.map_err(Error::Network)?;

    if !response.status().is_success() {
      let status = response.status();
      let error_text = response
        .text()
        .await
        .unwrap_or_else(|_| "Could not read error body.".to_string());
      return Err(Error::API(format!(
        "API request failed with status {status}: {error_text}"
      )));
    }

    response
      .json::<T>()
      .await
      .map_err(|e| Error::API(e.to_string()))
  }

  /// Searches for wallpapers on Wallhaven.
  /// Returns a `PaginatedResponse` containing the wallpapers and metadata.
  pub async fn search(
    &self,
    params: &SearchParams
  ) -> Result<PaginatedResponse> {
    let url = format!("{}/search", self.base_url);
    let mut query_params = Vec::new();

    if let Some(q) = &params.query {
      query_params.push(("q", q.clone()));
    }

    if let Some(cats) = params.categories {
      let cat_str = format!(
        "{}{}{}",
        if cats.0 { '1' } else { '0' },
        if cats.1 { '1' } else { '0' },
        if cats.2 { '1' } else { '0' }
      );
      query_params.push(("categories", cat_str));
    }

    if let Some(mut purities) = params.purity {
      if purities.2 && !self.has_api_key() {
        eprintln!(
          "Warning: NSFW purity filter requires an API key. Disabling NSFW for this search."
        );
        purities.2 = false; // Disable NSFW
      }
      let purity_str = format!(
        "{}{}{}",
        if purities.0 { '1' } else { '0' },
        if purities.1 { '1' } else { '0' },
        if purities.2 { '1' } else { '0' }
      );
      query_params.push(("purity", purity_str));
    }

    if let Some(sorting) = params.sorting {
      query_params.push(("sorting", sorting.to_string()));
      if sorting == Sorting::Toplist {
        if let Some(range) = params.top_range {
          query_params.push(("topRange", range.to_string()));
        }
      } else if params.top_range.is_some() {
        eprintln!(
          "Warning: `top_range` is only effective when `sorting` is `Toplist`. It will be ignored."
        );
      }
    }

    if let Some(order) = params.order {
      query_params.push(("order", order.to_string()));
    }

    if let Some(atleast) = &params.atleast {
      query_params.push(("atleast", atleast.clone()));
    }

    if let Some(resolutions) = &params.resolutions {
      query_params.push(("resolutions", resolutions.clone()));
    }

    if let Some(ratios) = &params.ratios {
      query_params.push(("ratios", ratios.clone()));
    }

    if let Some(colors) = &params.colors {
      query_params.push(("colors", colors.clone()));
    }

    if let Some(page) = params.page {
      query_params.push(("page", page.to_string()));
    }

    if let Some(seed) = &params.seed {
      query_params.push(("seed", seed.clone()));
    }

    self.send_request(url, &query_params).await
  }

  /// Retrieves details for a specific wallpaper by its ID.
  /// An API key is required to view NSFW wallpapers.
  pub async fn get_wallpaper_details(&self, id: &str) -> Result<Wallpaper> {
    let url = format!("{}/w/{}", self.base_url, id);
    let response: WallpaperDetailsResponse =
      self.send_request(url, &[]).await?;
    Ok(response.data)
  }

  // NOTE: The following methods are not yet implemented in this example stub,
  // but this is where you would add them following the same pattern.
  // Examples:
  // pub async fn get_tag_details(&self, tag_id: u32) -> Result<Tag> { ... }
  // pub async fn get_user_settings(&self) -> Result<UserSettings> { ... }
  // pub async fn get_user_collections(&self, username: &str) ->
  // Result<Vec<Collection>> { ... }

  /// Downloads a wallpaper image from its direct URL (`wallpaper.path`).
  pub async fn download_wallpaper(
    &self,
    url: &str,
    path: &std::path::Path
  ) -> Result<()> {
    let response = self.client.get(url).send().await.map_err(Error::Network)?;

    if !response.status().is_success() {
      let status = response.status();
      return Err(Error::API(format!(
        "Failed to download wallpaper: Status {status}"
      )));
    }

    let bytes = response.bytes().await.map_err(Error::Network)?;
    tokio::fs::write(path, bytes).await.map_err(Error::IO)?;
    Ok(())
  }
}
