use logline::debug;
use wallter::prelude::*;
mod api;
use api::wallhaven::search::*;

#[tokio::main]
async fn main() -> Result<()> {
  log::init()?;
  cli::init()?;
  features::init()?;
  config::init()?;
  let params = Url::default();
  let params = Purity::default();
  let params = Category::default();
  let params = Resolution::validate_format("1920x1080").is_ok();
  debug!("{:#?}", params);
  Ok(())
}
