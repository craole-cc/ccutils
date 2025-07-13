use crate::{
  cli::{Cli, Command},
  commands::{build, clean, install, list, uninstall},
  utilities::get
};
use anyhow::{Context, Result, anyhow, bail};
use std::{
  env::current_dir,
  fs::read_to_string,
  path::{Path, PathBuf}
};

#[derive(Debug)]
pub struct Workspace {
  pub name: String,
  pub members: Vec<String>,
  pub binaries: Vec<String>
}

impl Workspace {
  pub fn define() -> Result<Self> {
    let current_dir = current_dir().context("Failed to get current dir")?;
    let root_toml = Self::find_workspace_toml(&current_dir)?;
    let name = Self::extract_workspace_name(&root_toml)?;
    let members = Self::find_all_members(&root_toml)?;
    let binaries = Self::find_binary_crates(&members)?;

    Ok(Self {
      name,
      members,
      binaries
    })
  }

  pub fn execute_command(&self, cli: &Cli) -> Result<()> {
    let cargo_bin = get::cargo_bin_dir()?;

    match cli.effective_command() {
      Command::Build { crates } => {
        let target_crates = self.filter_target_crates(&crates)?;
        let builder = build::Config::new(&cargo_bin);
        builder.build_crates(&target_crates, cli.force, cli.verbose)?;
      }
      Command::Install { crates, mode } => {
        let target_crates = self.filter_target_crates(&crates)?;
        let installer = install::Config::new(self.name.clone(), cargo_bin);
        installer.install_crates(
          &target_crates,
          &mode,
          cli.force,
          cli.verbose
        )?;
      }
      Command::BuildInstall { crates, mode } => {
        let target_crates = self.filter_target_crates(&crates)?;
        let builder = build::Config::new(&cargo_bin);
        let installer = install::Config::new(self.name.clone(), cargo_bin);

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
        installer.install_crates(&to_rebuild, &mode, cli.force, cli.verbose)?;
      }
      Command::Clean { crates, target } => {
        let target_crates = if crates.is_empty() {
          Vec::new() //? Empty means clean all
        } else {
          self.filter_target_crates(&crates)?
        };
        let cleaner = clean::Config::new(self.name.clone())?;
        cleaner.clean_crates(&target_crates, &target, cli.verbose)?;
      }
      Command::Uninstall { crates } => {
        let target_crates = self.filter_target_crates(&crates)?;
        let uninstaller = uninstall::Config::new(self.name.clone(), cargo_bin);
        uninstaller.uninstall_crates(
          &target_crates,
          // all_variants is removed as both variants are always uninstalled
          cli.verbose
        )?;
      }
      Command::List {
        detailed,
        bins_only,
        libs_only
      } => {
        let lister = list::Config::new(self.name.clone(), cargo_bin);
        lister.list_crates(
          &self.members,
          &self.binaries,
          detailed,
          bins_only,
          libs_only
        )?;
      }
    }

    Ok(())
  }

  fn filter_target_crates(
    &self,
    specified_crates: &[String]
  ) -> Result<Vec<String>> {
    if specified_crates.is_empty() {
      return Ok(self.binaries.clone());
    }

    let mut target_crates = Vec::new();
    for crate_name in specified_crates {
      //{ Check if the specified crate exists in the workspace }
      let member_path = self
        .binaries
        .iter()
        .find(|member| {
          Path::new(member)
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name == crate_name)
        })
        .ok_or_else(|| {
          anyhow!(
            "Crate '{}' not found in workspace binary crates. Available: {}",
            crate_name,
            self.binaries.join(", ")
          )
        })?;

      target_crates.push(member_path.clone());
    }

    Ok(target_crates)
  }

  fn find_workspace_toml(start: &Path) -> Result<PathBuf> {
    let mut path = start.to_path_buf();
    loop {
      let cargo_toml = path.join("Cargo.toml");
      if cargo_toml.exists() {
        let content = read_to_string(&cargo_toml).with_context(|| {
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

  fn extract_workspace_name(cargo_toml: &Path) -> Result<String> {
    let content = read_to_string(cargo_toml).with_context(|| {
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

    //{ Try to get the package name first (if this is also a package) }
    if let Some(package_name) = parsed
      .get("package")
      .and_then(|p| p.get("name"))
      .and_then(|n| n.as_str())
    {
      return Ok(package_name.to_string());
    }

    //{ Fallback to workspace directory name }
    let workspace_dir = cargo_toml
      .parent()
      .context("Failed to get workspace directory")?;
    let dir_name = workspace_dir
      .file_name()
      .and_then(|name| name.to_str())
      .context("Failed to get workspace directory name")?;

    Ok(dir_name.to_string())
  }

  fn find_all_members(cargo_toml: &Path) -> Result<Vec<String>> {
    let content = read_to_string(cargo_toml).with_context(|| {
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

    let members = parsed["workspace"]["members"]
      .as_array()
      .context("No `[workspace.members]` array found in workspace Cargo.toml")?
      .iter()
      .filter_map(|m| m.as_str())
      .map(|s| s.to_string())
      .collect();

    Ok(members)
  }

  fn find_binary_crates(members: &[String]) -> Result<Vec<String>> {
    let mut binary_crates = Vec::new();

    for member in members {
      let cargo_path = Path::new(member).join("Cargo.toml");
      if !cargo_path.exists() {
        continue;
      }

      let member_toml = read_to_string(&cargo_path).with_context(|| {
        format!(
          "Failed to read member Cargo.toml at '{}'",
          cargo_path.display()
        )
      })?;

      let member_parsed: toml::Value =
        member_toml.parse().with_context(|| {
          format!(
            "Failed to parse member Cargo.toml at '{}'",
            cargo_path.display()
          )
        })?;

      let has_explicit_bin_target = member_parsed
        .get("bin")
        .and_then(|v| v.as_array())
        .is_some();
      let is_binary_by_convention =
        Path::new(member).join("src/main.rs").exists();

      if has_explicit_bin_target || is_binary_by_convention {
        binary_crates.push(member.clone());
      }
    }
    Ok(binary_crates)
  }
}
