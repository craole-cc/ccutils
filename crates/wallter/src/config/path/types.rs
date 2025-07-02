use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub enum Config {
  #[default]
  Toml,
  Json
}

impl Config {
  /// Returns the file extension for this config type (without dot).
  pub fn extension(self) -> &'static str {
    match self {
      Config::Toml => "toml",
      Config::Json => "json"
    }
  }

  /// Detect config type from file extension
  pub fn from_extension(path: &Path) -> Result<Self> {
    path
      .extension()
      .and_then(|ext| ext.to_str())
      .map(|ext| match ext.to_lowercase().as_str() {
        "toml" => Config::Toml,
        "json" => Config::Json,
        _ => Config::default()
      })
      .ok_or_else(|| Error::Config("Unknown config file format".into()))
  }
}
