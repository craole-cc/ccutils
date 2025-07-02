use crate::prelude::*;

pub fn init() -> Result<()> {
  nightlight()?;

  Ok(())
}

pub fn nightlight() -> Result<()> {
  #[cfg(all(target_os = "windows", feature = "nightlight"))]
  {
    if nightlight::enable()? {
      trace!("Activating the {APP} nightlight feature");
    } else {
      trace!("Skipping activation of the {APP} nightlight feature.");
    }
  }

  Ok(())
}
