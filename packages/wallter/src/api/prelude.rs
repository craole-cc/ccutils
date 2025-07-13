use crate::{Error, Result, api::wallhaven::Wallpaper};

#[derive(Debug, Default)]
pub struct Api {
  pub wallhaven: crate::api::wallhaven::Api
}

impl Api {
  pub fn new() -> Self {
    Self::default()
  }

  pub async fn pull_wallpapers(
    &self,
    config: &crate::config::Config
  ) -> Result<Vec<Wallpaper>> {
    // For now, we'll only consider Wallhaven as the source.
    // In the future, this should iterate through config.source.ordered
    // and use the appropriate API client.
    if let Some(wallhaven_config) = config
      .source
      .sources
      .iter()
      .find(|s| s.name == "wallhaven")
      .and_then(|s| s.wallhaven.as_ref())
    {
      let params = wallhaven_config.to_search_params();
      let response = self.wallhaven.search(&params).await?;
      Ok(response.data)
    } else {
      Err(Error::Api("Wallhaven configuration not found".into()))
    }
  }
}
