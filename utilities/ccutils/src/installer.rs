use anyhow::{Result, bail};
use std::{path::Path, process::Command};

pub struct Installer;

impl Installer {
  pub fn new() -> Self {
    Self
  }

  pub fn install_crates(&self, crates: &[String]) -> Result<()> {
    println!("\n--- Installing {} binary crates ---", crates.len());
    for member in crates {
      self.install_binary(member)?;
    }
    Ok(())
  }

  fn install_binary(&self, member: &str) -> Result<()> {
    let binary_name = Path::new(member)
      .file_name()
      .and_then(|name| name.to_str())
      .unwrap_or(member);

    println!("Installing '{binary_name}'...");
    let status = Command::new("cargo")
      .arg("install")
      .arg("--path")
      .arg(member)
      .status()?;

    if !status.success() {
      bail!("Cargo install failed for '{member}' with status: {status}");
    }

    Ok(())
  }
}
