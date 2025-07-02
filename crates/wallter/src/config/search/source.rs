use super::wallhaven::Params as Wallhaven;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// Configuration for an individual wallpaper source API.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Source {
  pub name: String,
  pub api_key: Option<String>,
  pub base_url: String,
  pub requires_api_key: bool,
  pub enabled: bool,
  pub valid: bool,

  /// Wallhaven-specific default parameters.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub wallhaven: Option<Wallhaven>
}

impl Source {
  /// Creates a new `Source` instance with essential fields.
  pub fn new(
    name: impl Into<String>,
    base_url: impl Into<String>,
    requires_api_key: bool
  ) -> Self {
    let mut s = Self {
      name: name.into(),
      base_url: base_url.into(),
      requires_api_key,
      ..Default::default()
    };
    //{ Determine validity based on `requires_api_key` and `api_key`. }
    s.valid = !s.requires_api_key || s.api_key.is_some();
    s
  }

  /// Sets the API key for the source and updates validity.
  pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
    self.api_key = Some(api_key.into());
    self.valid = true;
    self
  }

  /// Sets the Wallhaven-specific parameters.
  pub fn with_wallhaven_params(mut self, params: Wallhaven) -> Self {
    self.wallhaven = Some(params);
    self
  }

  /// Sets the enabled status of the source.
  pub fn with_enabled(mut self, enabled: bool) -> Self {
    self.enabled = enabled;
    self
  }
}

impl Display for Source {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    printf!(f, "Source Name", &self.name)?;

    //? Only show base_url if it's relevant
    if !self.base_url.is_empty() {
      printf!(f, "Base URL", &self.base_url)?;
    }

    printf!(f, "Enabled (User)", self.enabled)?;
    printf!(f, "Valid (Runtime)", self.valid)?;
    printf!(f, "Requires API Key", self.requires_api_key)?;
    printf!(f, "API Key", self.api_key.as_deref().unwrap_or("[Not Set]"))?;
    if let Some(params) = &self.wallhaven {
      printh!(f, "API Parameters:", 4)?;
      writeln!(f, "{params}")?;
    }
    Ok(())
  }
}
