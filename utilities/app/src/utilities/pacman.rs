use anyhow::Result;
use std::collections::HashMap;

pub struct PackageManager {
  pub name: String,
  pub priority: i32,
  pub available: bool
}

pub fn detect_package_managers() -> Result<Vec<PackageManager>> {
  // Implementation for detecting available package managers
  todo!()
}

pub fn get_default_managers() -> HashMap<String, Vec<String>> {
  let mut defaults = HashMap::new();

  // Windows defaults
  defaults.insert(
    "windows".to_string(),
    vec!["winget", "chocolatey", "scoop"]
      .into_iter()
      .map(String::from)
      .collect()
  );

  // Various Linux distros
  defaults.insert(
    "ubuntu".to_string(),
    vec!["apt", "flatpak", "snap"]
      .into_iter()
      .map(String::from)
      .collect()
  );

  defaults.insert(
    "debian".to_string(),
    vec!["apt", "flatpak", "snap"]
      .into_iter()
      .map(String::from)
      .collect()
  );

  defaults.insert(
    "fedora".to_string(),
    vec!["dnf", "flatpak", "snap"]
      .into_iter()
      .map(String::from)
      .collect()
  );

  defaults.insert(
    "arch".to_string(),
    vec!["paru", "yay", "pacman", "yaourt", "trizen", "flatpak"]
      .into_iter()
      .map(String::from)
      .collect()
  );

  defaults.insert(
    "nixos".to_string(),
    vec!["nix-shell", "nix-env", "flatpak"]
      .into_iter()
      .map(String::from)
      .collect()
  );

  defaults.insert(
    "opensuse".to_string(),
    vec!["zypper", "flatpak", "snap"]
      .into_iter()
      .map(String::from)
      .collect()
  );

  defaults.insert(
    "centos".to_string(),
    vec!["dnf", "flatpak", "snap"]
      .into_iter()
      .map(String::from)
      .collect()
  );

  defaults.insert(
    "rhel".to_string(),
    vec!["dnf", "flatpak", "snap"]
      .into_iter()
      .map(String::from)
      .collect()
  );

  defaults
}
