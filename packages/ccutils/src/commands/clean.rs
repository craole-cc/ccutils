use crate::utilities::get::cargo_bin_dir;
use anyhow::{Context, Result, bail};
use clap::ValueEnum;
use std::{
  env::consts::EXE_EXTENSION,
  fs::remove_file,
  path::{Path, PathBuf},
  process::Command
};

#[derive(Debug, Default, Clone, ValueEnum)] // No change here.
pub enum Target {
  /// Clean only target directories
  Dir,

  /// Clean only installed binaries
  Bin,

  /// Clean both target directories and installed binaries
  #[default]
  All
}

pub struct Config {
  workspace_name: String,
  cargo_bin: PathBuf
}

impl Config {
  pub fn new(workspace_name: String) -> Result<Self> {
    // No change here.
    Ok(Self {
      workspace_name,
      cargo_bin: cargo_bin_dir()?
    })
  }

  pub fn clean_crates(
    &self,
    crates: &[String],
    target: &Target,
    verbose: bool
  ) -> Result<()> {
    match target {
      Target::Dir => self.clean_target_dirs(crates, verbose)?,
      Target::Bin => self.clean_installed_binaries(crates, verbose)?,
      Target::All => {
        self.clean_target_dirs(crates, verbose)?;
        self.clean_installed_binaries(crates, verbose)?;
      }
    }
    Ok(())
  }

  fn clean_target_dirs(&self, crates: &[String], verbose: bool) -> Result<()> {
    if crates.is_empty() {
      println!("Cleaning workspace target directory...");
      let status = Command::new("cargo").arg("clean").status()?;
      if !status.success() {
        bail!("Failed to clean workspace target directory");
      }
    } else {
      println!("Cleaning target directories for {} crates...", crates.len());
      for crate_path in crates {
        if verbose {
          println!("Cleaning target for '{crate_path}'");
        }

        let status = Command::new("cargo")
          .arg("clean")
          .arg("--manifest-path")
          .arg(Path::new(crate_path).join("Cargo.toml"))
          .status()?;

        if !status.success() {
          bail!("Failed to clean target for '{}'", crate_path);
        }
      }
    }
    Ok(())
  }

  fn clean_installed_binaries(
    &self,
    crates: &[String],
    verbose: bool
  ) -> Result<()> {
    println!("Cleaning installed binaries for {} crates...", crates.len());

    for crate_path in crates {
      let binary_name = Path::new(crate_path)
        .file_name()
        .and_then(|name| name.to_str())
        .context("Could not determine binary name")?;

      self.remove_binary_variants(binary_name, verbose)?;
    }
    Ok(())
  }

  fn remove_binary_variants(
    &self,
    binary_name: &str,
    verbose: bool
  ) -> Result<()> {
    let variants = [
      binary_name.to_string(),
      format!("{}-{}", self.workspace_name, binary_name)
    ];

    for variant in &variants {
      let binary_path =
        self.cargo_bin.join(variant).with_extension(EXE_EXTENSION);

      if binary_path.exists() {
        if verbose {
          println!("Removing '{}'", binary_path.display());
        }
        remove_file(&binary_path).with_context(|| {
          format!("Failed to remove binary '{}'", binary_path.display())
        })?;
      }
    }
    Ok(())
  }
}
