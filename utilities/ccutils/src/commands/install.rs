use anyhow::{Context, Result, bail};
use clap::ValueEnum;
use std::{
  env::{consts::EXE_EXTENSION, current_exe},
  fs::{copy, remove_file},
  path::{Path, PathBuf},
  process::Command
};

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum Target {
  /// Install with default name only (e.g., 'wallter')
  Unprefixed,
  /// Install with workspace-prefixed name only (e.g., 'ccutils-wallter')
  Prefixed,
  /// Install both unprefixed and prefixed versions (default)
  #[default]
  Both
}

pub struct Config {
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

  pub fn install_crates(
    &self,
    crates: &[String],
    mode: &Target,
    force: bool,
    verbose: bool
  ) -> Result<()> {
    println!("\n--- Installing {} binary crates ---", crates.len());

    for member in crates {
      let binary_name = Path::new(member)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(member);

      // Check if we're trying to install ourselves
      let is_self_install = self.is_self_install(binary_name)?;

      if is_self_install && !force {
        println!(
          "Skipping self-install of '{binary_name}' (would cause file lock)."
        );
        println!(
          "To update {binary_name}, use --force or run: cargo install --path {member} --force"
        );
        continue;
      }

      self.install_with_mode(
        member,
        binary_name,
        mode,
        force,
        is_self_install,
        verbose
      )?;
    }
    Ok(())
  }

  fn install_with_mode(
    &self,
    member_path: &str,
    binary_name: &str,
    mode: &Target,
    force: bool,
    is_self_install: bool,
    verbose: bool
  ) -> Result<()> {
    match mode {
      Target::Unprefixed => {
        self.install_binary(
          member_path,
          binary_name,
          None,
          force,
          is_self_install,
          verbose
        )?;
      }
      Target::Prefixed => {
        let prefixed_name = format!("{}-{}", self.workspace_name, binary_name);
        self.install_binary(
          member_path,
          binary_name,
          Some(&prefixed_name),
          force,
          is_self_install,
          verbose
        )?;
      }
      Target::Both => {
        //{ Install unprefixed version first }
        self.install_binary(
          member_path,
          binary_name,
          None,
          force,
          is_self_install,
          verbose
        )?;

        //{ Create alias for prefixed version }
        let prefixed_name = format!("{}-{}", self.workspace_name, binary_name);
        self.create_alias(binary_name, &prefixed_name, verbose)?;
      }
    }
    Ok(())
  }

  fn install_binary(
    &self,
    member_path: &str,
    binary_name: &str,
    install_name: Option<&str>,
    force: bool,
    is_self_install: bool,
    verbose: bool
  ) -> Result<()> {
    let effective_name = install_name.unwrap_or(binary_name);
    let force_needed = force || is_self_install;

    if verbose || install_name.is_some() {
      println!(
        "Installing '{}' as '{}'{}...",
        binary_name,
        effective_name,
        if force_needed { " (forced)" } else { "" }
      );
    } else {
      println!(
        "Installing '{}'{}...",
        effective_name,
        if force_needed { " (forced)" } else { "" }
      );
    }

    let mut cmd = Command::new("cargo");
    cmd.arg("install").arg("--path").arg(member_path);

    if let Some(_name) = install_name {
      cmd.arg("--bin").arg(binary_name);
      //TODO For custom names, we'll need to handle this differently
      //TODO cargo install doesn't directly support renaming, so we install
      // normally TODO and then create the alias
    }

    if force_needed {
      cmd.arg("--force");
    }

    let status = cmd.status()?;

    if !status.success() {
      if is_self_install {
        bail!(
          "Self-install failed for '{}'. This can happen due to file locks. \
                    Try: taskkill /f /im {}.exe && cargo install --path {} --force",
          binary_name,
          binary_name,
          member_path
        );
      } else {
        bail!(
          "Cargo install failed for '{}' with status: {}",
          member_path,
          status
        );
      }
    }

    // If we specified a custom install name and it's different from binary
    // name, create an alias after installation
    if let Some(custom_name) = install_name
      && custom_name != binary_name
    {
      self.create_alias(binary_name, custom_name, verbose)?;
    }
    Ok(())
  }

  fn create_alias(
    &self,
    source_name: &str,
    alias_name: &str,
    verbose: bool
  ) -> Result<()> {
    let src = self
      .cargo_bin_dir
      .join(source_name)
      .with_extension(EXE_EXTENSION);
    let alias = self
      .cargo_bin_dir
      .join(alias_name)
      .with_extension(EXE_EXTENSION);

    if !src.exists() {
      bail!(
        "Source binary '{}' not found at '{}'",
        source_name,
        src.display()
      );
    }

    if verbose {
      println!("Creating alias '{alias_name}' -> '{source_name}'");
    }

    // Remove the alias if it already exists
    if alias.exists() {
      remove_file(&alias).with_context(|| {
        format!("Failed to remove existing alias '{}'", alias.display())
      })?;
    }

    #[cfg(unix)]
    {
      std::os::unix::fs::symlink(&src, &alias).with_context(|| {
        format!(
          "Failed to create symlink from '{}' to '{}'",
          src.display(),
          alias.display()
        )
      })?;
    }

    #[cfg(windows)]
    {
      //{ Windows: try symlink first, fallback to copy if not allowed }
      match std::os::windows::fs::symlink_file(&src, &alias) {
        Ok(_) =>
          if verbose {
            println!("Created symlink for '{alias_name}'");
          },
        Err(_) => {
          copy(&src, &alias).with_context(|| {
            format!(
              "Failed to copy '{}' to '{}'",
              src.display(),
              alias.display()
            )
          })?;
          if verbose {
            println!("Created copy for '{alias_name}' (symlink not available)");
          }
        }
      }
    }

    Ok(())
  }

  /// Check if we're trying to install the same binary that's currently running
  fn is_self_install(&self, binary_name: &str) -> Result<bool> {
    if let Ok(current_exe) = current_exe()
      && let Some(current_name) =
        current_exe.file_stem().and_then(|n| n.to_str())
    {
      return Ok(current_name == binary_name);
    }
    Ok(false)
  }
}
