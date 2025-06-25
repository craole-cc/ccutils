use anyhow::{Result, bail};
use std::{env, path::Path, process::Command};

pub struct Installer;

impl Installer {
  pub fn new() -> Self {
    Self
  }

  pub fn install_crates(&self, crates: &[String], force: bool) -> Result<()> {
    println!("\n--- Installing {} binary crates ---", crates.len());

    for member in crates {
      let binary_name = Path::new(member)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(member);

      // Check if we're trying to install ourselves
      if self.is_self_install(binary_name)? {
        if !force {
          println!(
            "Skipping self-install of '{binary_name}' (would cause file lock)."
          );
          println!(
            "To update ccutils, use --force or run: cargo install --path {member} --force"
          );
          continue;
        } else {
          println!(
            "Force installing '{binary_name}' (self-install with --force)..."
          );
        }
      } else {
        println!(
          "Installing '{binary_name}'{}...",
          if force { " (forced)" } else { "" }
        );
      }

      let mut cmd = Command::new("cargo");
      cmd.arg("install").arg("--path").arg(member);

      // Always use --force for self-installs or when force flag is set
      if force || self.is_self_install(binary_name)? {
        cmd.arg("--force");
      }

      let status = cmd.status()?;

      if !status.success() {
        if self.is_self_install(binary_name)? {
          bail!(
            "Self-install failed for '{binary_name}'. This can happen due to file locks. \
            Try: taskkill /f /im {}.exe && cargo install --path {} --force",
            binary_name,
            member
          );
        } else {
          bail!("Cargo install failed for '{member}' with status: {status}");
        }
      }
    }
    Ok(())
  }

  /// Check if we're trying to install the same binary that's currently running
  fn is_self_install(&self, binary_name: &str) -> Result<bool> {
    if let Ok(current_exe) = env::current_exe()
      && let Some(current_name) =
        current_exe.file_stem().and_then(|n| n.to_str())
    {
      return Ok(current_name == binary_name);
    }
    Ok(false)
  }
}
