use wallter::{prelude::*, *};

fn main() -> Result<()> {
  log::init()?;
  cli::init()?;
  config::init()?;
  features::init()?;
  Ok(())
}
