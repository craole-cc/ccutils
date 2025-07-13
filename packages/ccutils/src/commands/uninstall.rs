use anyhow::{Context, Result};
use std::{
  env::consts,
  fs::remove_file,
  path::{Path, PathBuf}
};

pub struct Config {
  // No change here.
  workspace_name: String,
  cargo_bin_dir: PathBuf
}

impl Config {
  pub fn new(workspace_name: String, cargo_bin_dir: PathBuf) -> Self {
    Self {
      workspace_name,
      cargo_bin_dir
    }
  }

  pub fn uninstall_crates(
    &self,
    crates: &[String],
    // all_variants: bool, // Removed as both variants are always uninstalled
    verbose: bool
  ) -> Result<()> {
    println!("Uninstalling {} binary crates...", crates.len());

    for crate_path in crates {
      let binary_name = Path::new(crate_path)
        .file_name()
        .and_then(|name| name.to_str())
        .context("Could not determine binary name")?;
      self.uninstall_binary(binary_name, verbose)?;
    }
    Ok(())
  }

  fn uninstall_binary(
    &self,
    binary_name: &str,
    // _all_variants: bool, // Removed as both variants are always uninstalled
    verbose: bool
  ) -> Result<()> {
    let mut removed_count = 0;

    //{ Always try to remove the unprefixed version }
    removed_count += self.remove_if_exists(binary_name, verbose)?;

    //{ Always remove prefixed version }
    let prefixed_name = format!("{}-{}", self.workspace_name, binary_name);
    removed_count += self.remove_if_exists(&prefixed_name, verbose)?;
    if removed_count == 0 {
      println!("Binary '{binary_name}' was not installed");
    } else if verbose {
      println!("Removed {removed_count} variant(s) of '{binary_name}'");
    }

    Ok(())
  }

  fn remove_if_exists(
    &self,
    binary_name: &str,
    verbose: bool
  ) -> Result<usize> {
    let path_with_ext = self
      .cargo_bin_dir
      .join(binary_name)
      .with_extension(consts::EXE_EXTENSION);

    let path_without_ext = self.cargo_bin_dir.join(binary_name); // On Unix, EXE_EXTENSION is empty, so this might be the same as path_with_ext

    // Check for existence of the path itself (symlink or file), not its target,
    // to ensure broken symlinks can also be removed.
    let binary_path = if path_with_ext.symlink_metadata().is_ok() {
      Some(path_with_ext)
    } else if path_without_ext.symlink_metadata().is_ok() {
      // Only check path_without_ext if path_with_ext didn't exist
      Some(path_without_ext)
    } else {
      None
    };

    if let Some(path) = binary_path {
      if verbose {
        println!("Removing '{}'", path.display());
      }
      remove_file(&path).with_context(|| {
        format!("Failed to remove binary '{}'", path.display())
      })?;
      Ok(1)
    } else {
      Ok(0)
    }
  }
}
