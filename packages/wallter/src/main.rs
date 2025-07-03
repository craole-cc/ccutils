use wallter::prelude::*;
use wallter::*;

fn main() -> Result<()> {
  log::init()?;
  cli::init()?;
  config::init()?;
  features::init()?;
  Ok(())
}
