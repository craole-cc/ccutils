use wallter::{Error, Result};

fn main() -> Result<()> {
  println!("Welcome to {}!", env!("CARGO_PKG_NAME"));

  // nightlight::toggle()?;
  // nightlight::enable()
  // let config = wallter::config::Config::default();
  let config = wallter::config::init()?;
  println!("Config: {config}");

  Ok(())
}
