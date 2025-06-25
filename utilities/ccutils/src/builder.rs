use crate::utils::get_latest_mtime;
use anyhow::{Context, Result, bail};
use std::{
  env, fs,
  path::{Path, PathBuf},
  process::Command
};

pub struct Builder {
  cargo_bin_dir: PathBuf
}

impl Builder {
  pub fn new(cargo_bin_dir: &Path) -> Self {
    Self {
      cargo_bin_dir: cargo_bin_dir.to_path_buf()
    }
  }

  pub fn build_crates(
    &self,
    crates: &[String],
    force: bool,
    verbose: bool
  ) -> Result<()> {
    let to_build = if force {
      crates.to_vec()
    } else {
      self.filter_outdated_crates(crates, force, verbose)?
    };

    if to_build.is_empty() {
      println!("All specified binary crates are up to date. Nothing to build.");
      return Ok(());
    }

    self.build_only(&to_build)
  }

  pub fn build_only(&self, crates: &[String]) -> Result<()> {
    println!("\n--- Building {} binary crates ---", crates.len());
    for member in crates {
      self
        .build_binary(member)
        .with_context(|| format!("Failed to build '{member}'"))?;
    }
    Ok(())
  }

  pub fn filter_outdated_crates(
    &self,
    crates: &[String],
    force: bool,
    verbose: bool
  ) -> Result<Vec<String>> {
    if force {
      return Ok(crates.to_vec());
    }

    let mut to_rebuild = Vec::new();

    if verbose {
      println!("\n--- Checking for updated crates ---");
    }

    for member in crates {
      if self.needs_rebuild(member, verbose)? {
        to_rebuild.push(member.clone());
      }
    }

    Ok(to_rebuild)
  }

  /// Checks if a binary crate needs to be rebuilt by comparing the modification
  /// times of its source files against the installed binary.
  fn needs_rebuild(&self, member: &str, verbose: bool) -> Result<bool> {
    let binary_name = Path::new(member)
      .file_name()
      .and_then(|name| name.to_str())
      .with_context(|| {
        format!("Could not determine binary name from path: {member}")
      })?;

    //{ On Windows, cargo install adds .exe. On Linux/macOS, it does not }
    let installed_binary_path = self
      .cargo_bin_dir
      .join(binary_name)
      .with_extension(env::consts::EXE_EXTENSION);

    if !installed_binary_path.exists() {
      if verbose {
        println!("-> Crate '{binary_name}' needs install (not found).");
      }
      return Ok(true);
    }

    let binary_mtime = fs::metadata(&installed_binary_path)?.modified()?;
    let latest_src_mtime = get_latest_mtime(Path::new(member))?;

    if latest_src_mtime > binary_mtime {
      if verbose {
        println!("-> Crate '{binary_name}' needs rebuild (source is newer).");
      }
      Ok(true)
    } else {
      if verbose {
        println!("-> Crate '{binary_name}' is up to date.");
      }
      Ok(false)
    }
  }

  fn build_binary(&self, member: &str) -> Result<()> {
    //{ Assumes the binary name is the same as the final component of the member
    //{ path }
    let binary_name = Path::new(member)
      .file_name()
      .and_then(|name| name.to_str())
      .with_context(|| {
        format!("Could not determine binary name from path: {member}")
      })?;

    println!("Building binary '{binary_name}' in release mode...");

    let status = Command::new("cargo")
      .arg("build")
      .arg("--release")
      .arg("--bin")
      .arg(binary_name)
      .status()?;

    if !status.success() {
      bail!("Cargo build failed for '{binary_name}' with status: {status}");
    }

    Ok(())
  }
}
