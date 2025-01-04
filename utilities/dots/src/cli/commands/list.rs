use super::ListCommands;
use crate::utilities::{detect_package_managers, PackageManager};
use anyhow::{Context, Result};
use logline::{debug, info};
use std::process::Command;

pub fn list(cmd: &ListCommands) -> Result<()> {
    match cmd {
        ListCommands::Installed => installed(),
        ListCommands::Available { query } => available(query),
        ListCommands::Outdated => outdated(),
        ListCommands::Managers => managers(),
    }
}

fn installed() -> Result<()> {
    let package_managers = detect_package_managers()?;
    if package_managers.is_empty() {
        anyhow::bail!("No package managers found");
    }

    for pm in package_managers.iter().filter(|pm| pm.available) {
        debug!("Listing installed packages for {}", pm.name);
        match list_installed_for_manager(pm) {
            Ok(_) => (),
            Err(e) => info!("Failed to list packages for {}: {}", pm.name, e),
        }
    }

    Ok(())
}

fn list_installed_for_manager(pm: &PackageManager) -> Result<()> {
    let (cmd, args) = match pm.name.as_str() {
        "winget" => ("winget", vec!["list"]),
        "chocolatey" => ("choco", vec!["list", "--local-only"]),
        "scoop" => ("scoop", vec!["list"]),
        "apt" => ("apt", vec!["list", "--installed"]),
        "dnf" => ("dnf", vec!["list", "installed"]),
        "pacman" => ("pacman", vec!["-Q"]),
        "paru" => ("paru", vec!["-Q"]),
        "yay" => ("yay", vec!["-Q"]),
        "nix-env" => ("nix-env", vec!["--query"]),
        "flatpak" => ("flatpak", vec!["list"]),
        _ => anyhow::bail!("Unsupported package manager: {}", pm.name),
    };

    info!("Installed packages from {}:", pm.name);

    let output = Command::new(cmd)
        .args(args)
        .output()
        .with_context(|| format!("Failed to execute {} command", pm.name))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to list packages: {}", error);
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{}", output_str);

    Ok(())
}

fn available(query: &Option<String>) -> Result<()> {
    // Implementation for listing/searching available packages
    todo!()
}

fn outdated() -> Result<()> {
    // Implementation for listing outdated packages
    todo!()
}

fn managers() -> Result<()> {
    // Implementation for listing available package managers
    todo!()
}
