use anyhow::{Context, Result};
use logline::{debug, info};
use std::{fs, path::Path, process::Command};

use crate::utilities::{PackageManager, detect_package_managers};

pub fn add(pkgs: &[String], file: &Option<String>) -> Result<()> {
  if let Some(file_path) = file {
    from_file(file_path)
  } else {
    from_list(pkgs)
  }
}

fn from_file(file_path: &str) -> Result<()> {
  let path = Path::new(file_path);
  let content = fs::read_to_string(path)
    .with_context(|| format!("Failed to read package list from {file_path}"))?;

  let packages: Vec<String> = content
    .lines()
    .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
    .map(|s| s.trim().to_string())
    .collect();

  if packages.is_empty() {
    info!("No packages found in {}", file_path);
    return Ok(());
  }

  from_list(&packages)
}

fn from_list(pkgs: &[String]) -> Result<()> {
  if pkgs.is_empty() {
    info!("No packages specified");
    return Ok(());
  }

  let package_managers = detect_package_managers()?;
  if package_managers.is_empty() {
    anyhow::bail!("No package managers found");
  }

  // Try each package manager in order of priority
  for pm in package_managers.iter().filter(|pm| pm.available) {
    debug!("Trying to install with {}", pm.name);
    // return Ok(());
    match install_with_manager(pm, pkgs) {
      Ok(_) => return Ok(()),
      Err(e) => info!("Failed to install with {}: {}", pm.name, e)
    }
  }

  anyhow::bail!("Failed to install packages with any available package manager")
}

fn install_with_manager(pm: &PackageManager, pkgs: &[String]) -> Result<()> {
  let (cmd, args) = match pm.name.as_str() {
    "winget" => ("winget", vec!["install", "--accept-package-agreements"]),
    "chocolatey" => ("choco", vec!["install", "-y"]),
    "scoop" => ("scoop", vec!["install"]),
    "apt" => ("apt", vec!["install", "-y"]),
    "dnf" => ("dnf", vec!["install", "-y"]),
    "pacman" => ("pacman", vec!["-S", "--noconfirm"]),
    "paru" => ("paru", vec!["-S", "--noconfirm"]),
    "yay" => ("yay", vec!["-S", "--noconfirm"]),
    "nix-env" => ("nix-env", vec!["-i"]),
    "flatpak" => ("flatpak", vec!["install", "-y"]),
    _ => anyhow::bail!("Unsupported package manager: {}", pm.name)
  };

  let mut command = Command::new(cmd);
  command.args(&args);
  command.args(pkgs);

  info!("Installing with {}: {:?}", pm.name, pkgs);

  let output = command
    .output()
    .with_context(|| format!("Failed to execute {} command", pm.name))?;

  if !output.status.success() {
    let error = String::from_utf8_lossy(&output.stderr);
    anyhow::bail!("Installation failed: {}", error);
  }

  info!("Successfully installed packages with {}", pm.name);
  Ok(())
}
