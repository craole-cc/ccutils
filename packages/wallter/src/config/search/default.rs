use super::{Source, wallhaven::Params as Wallhaven};
use crate::api::wallhaven::Sorting;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// Global API configuration for all wallpaper sources.
/// This acts as the main configuration struct for the `api` module.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
  /// List of configured wallpaper sources
  pub sources: Vec<Source>,

  /// The ordered list of source names by priority. When fetching, the
  /// application will attempt to use sources in this order until a wallpaper
  /// is successfully retrieved.
  pub ordered: Vec<String>
}

impl Display for Config {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    for source in self.sources.iter() {
      //{ Determine and display rank }
      if let Some(rank) =
        self.ordered.iter().position(|name| name == &source.name)
      {
        printf!(f, "Rank", rank + 1)?;
      }

      //{ Display source information }
      writeln!(f, "{source}")?;
    }
    Ok(())
  }
}

impl Default for Config {
  /// Creates a new `Config` instance with default values.
  /// By default, it initializes with a common set of wallpaper sources.
  fn default() -> Self {
    //{ Define default sources directly here, including specific parameters }
    let wallhaven_source = Source {
      name: "wallhaven".into(),
      //? base_url is not used by our new API client, so it's empty.
      base_url: "".into(),
      requires_api_key: false,
      wallhaven: Some(Wallhaven {
        categories: Some((true, true, false)), // General & Anime
        purity: Some((true, true, false)),     // SFW & Sketchy
        sorting: Some(Sorting::Random),
        ..Default::default()
      }),
      ..Default::default()
    };

    let unsplash_source = Source {
      name: "unsplash".into(),
      base_url: "https://api.unsplash.com/".into(),
      requires_api_key: true,
      ..Default::default()
    };

    let pixabay_source = Source {
      name: "pixabay".into(),
      base_url: "https://pixabay.com/api/".into(),
      requires_api_key: true,
      ..Default::default()
    };

    let default_sources =
      vec![wallhaven_source, unsplash_source, pixabay_source];

    //{ Define default rank order based on the default sources' names }
    let default_rank_names: Vec<String> = default_sources
      .iter()
      .map(|source| source.name.clone())
      .collect();

    Self {
      sources: default_sources,
      ordered: default_rank_names
    }
  }
}

impl Config {
  pub fn new() -> Self {
    Self::default()
  }
}
