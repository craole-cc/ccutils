use crate::prelude::*;

pub fn init() -> Result<()> {
  nightlight()?;

  Ok(())
}

pub fn nightlight() -> Result<()> {
  #[cfg(all(target_os = "windows", feature = "nightlight"))]
  {
    if crate::config::color::mode::nightlight::enable()? {
      info!("Activating the {APP} nightlight feature");
    } else {
      trace!("Skipping activation of the {APP} nightlight feature.");
    }
  }

  Ok(())
}
