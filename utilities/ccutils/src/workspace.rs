use crate::builder::Builder;
use crate::cli::{Cli, Command};
use crate::installer::Installer;
use crate::utils::get_cargo_bin_dir;
use anyhow::{Context, Result, bail};
use std::{
  env, fs,
  path::{Path, PathBuf}
};

pub struct Workspace {
  pub binary_crates: Vec<String>,
  pub cargo_bin_dir: PathBuf
}

impl Workspace {
  pub fn find_current() -> Result<Self> {
    let current_dir =
      env::current_dir().context("Failed to get current dir")?;
    let root_toml = Self::find_workspace_toml(&current_dir)?;
    let binary_crates = Self::find_binary_crates(&root_toml)?;
    let cargo_bin_dir = get_cargo_bin_dir()?;

    Ok(Self {
      binary_crates,
      cargo_bin_dir
    })
  }

  pub fn execute_command(&self, cli: &Cli) -> Result<()> {
    if self.binary_crates.is_empty() {
      println!("No binary crates found in workspace.");
      return Ok(());
    }

    match cli.effective_command() {
      Command::Build { crates } => {
        let target_crates = self.filter_target_crates(&crates)?;
        let builder = Builder::new(&self.cargo_bin_dir);
        builder.build_crates(&target_crates, cli.force, cli.verbose)?;
      }
      Command::Install { crates } => {
        let target_crates = self.filter_target_crates(&crates)?;
        let installer = Installer::new();
        installer.install_crates(&target_crates, cli.force)?;
      }
      Command::BuildInstall { crates } => {
        let target_crates = self.filter_target_crates(&crates)?;
        let builder = Builder::new(&self.cargo_bin_dir);
        let installer = Installer::new();

        let to_rebuild = builder.filter_outdated_crates(
          &target_crates,
          cli.force,
          cli.verbose
        )?;

        if to_rebuild.is_empty() {
          println!(
            "All specified binary crates are up to date. Nothing to do."
          );
          return Ok(());
        }

        builder.build_only(&to_rebuild)?;
        installer.install_crates(&to_rebuild, cli.force)?;
      }
    }

    Ok(())
  }

  fn filter_target_crates(
    &self,
    specified_crates: &[String]
  ) -> Result<Vec<String>> {
    if specified_crates.is_empty() {
      return Ok(self.binary_crates.clone());
    }

    let mut target_crates = Vec::new();
    for crate_name in specified_crates {
      //{ Check if the specified crate exists in the workspace }
      let found = self.binary_crates.iter().any(|member| {
        Path::new(member)
          .file_name()
          .and_then(|name| name.to_str())
          .map(|name| name == crate_name)
          .unwrap_or(false)
      });

      if !found {
        bail!(
          "Crate '{}' not found in workspace binary crates. Available: {}",
          crate_name,
          self.binary_crates.join(", ")
        );
      }

      //{ Find the full member path for this crate }
      let member_path = self
        .binary_crates
        .iter()
        .find(|member| {
          Path::new(member)
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| name == crate_name)
            .unwrap_or(false)
        })
        .unwrap(); // Safe because we just checked it exists

      target_crates.push(member_path.clone());
    }

    Ok(target_crates)
  }

  fn find_workspace_toml(start: &Path) -> Result<PathBuf> {
    let mut path = start.to_path_buf();
    loop {
      let cargo_toml = path.join("Cargo.toml");
      if cargo_toml.exists() {
        let content = fs::read_to_string(&cargo_toml).with_context(|| {
          format!("Failed to read Cargo.toml at '{}'", cargo_toml.display())
        })?;
        if content.contains("[workspace]") {
          return Ok(cargo_toml);
        }
      }
      if !path.pop() {
        break;
      }
    }
    bail!("Could not find workspace root from '{}'", start.display())
  }

  fn find_binary_crates(cargo_toml: &Path) -> Result<Vec<String>> {
    //{ Read the main Cargo.toml }
    let content = fs::read_to_string(cargo_toml).with_context(|| {
      format!(
        "Failed to read workspace Cargo.toml at '{}'",
        cargo_toml.display()
      )
    })?;
    let parsed: toml::Value = content.parse().with_context(|| {
      format!(
        "Failed to parse workspace Cargo.toml at '{}'",
        cargo_toml.display()
      )
    })?;

    //{ Extract workspace members }
    let members = parsed["workspace"]["members"]
      .as_array()
      .context("No `[workspace.members]` array found in workspace Cargo.toml")?
      .iter()
      .filter_map(|m| m.as_str());

    //{ Initialize the list of binary crates }
    let mut binary_crates = Vec::new();

    //{ Update the list of binary crates of the workspace members }
    for member in members {
      let cargo_path = Path::new(member).join("Cargo.toml");
      if !cargo_path.exists() {
        continue;
      }

      let member_toml = fs::read_to_string(&cargo_path).with_context(|| {
        format!("Failed to read member Cargo.toml at '{cargo_path:?}'")
      })?;
      let member_parsed: toml::Value =
        member_toml.parse().with_context(|| {
          format!("Failed to parse member Cargo.toml at '{cargo_path:?}'")
        })?;

      //{ Find crates with `[[bin]]` section or a `src/main.rs` file }
      let has_explicit_bin_target = member_parsed
        .get("bin")
        .and_then(|v| v.as_array())
        .is_some();
      let is_binary_by_convention =
        Path::new(member).join("src/main.rs").exists();

      if has_explicit_bin_target || is_binary_by_convention {
        binary_crates.push(member.to_string());
      }
    }
    Ok(binary_crates)
  }
}
