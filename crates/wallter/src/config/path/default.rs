use super::types;
use crate::{Error, Result, config::Monitor};
use serde::{Deserialize, Serialize};
use std::{
  fmt::{self, Display, Formatter},
  fs::{File, create_dir_all},
  io::Write,
  path::{Path, PathBuf}
};
use winit::monitor;

/// Holds paths specific to a single monitor.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonitorPaths {
  /// The name of the monitor (e.g., "DP-1").
  pub name: String,
  /// The directory where wallpapers for this monitor's resolution are stored.
  pub download_dir: PathBuf,
  /// The path to the file currently set as the wallpaper for this monitor.
  pub current_wallpaper: PathBuf
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
  /// Home directory for the config and wallpapers
  pub home_dir: PathBuf,

  /// All wallpaper downloads
  pub downloads_dir: PathBuf,

  /// User-defined wallpaper favorites
  pub favorites_dir: PathBuf,

  /// This directory houses the current wallpaper for each monitor
  pub wallpaper_dir: PathBuf,

  /// The name of the configuration file
  pub config_name: String,

  /// The format type of the configuration file
  pub config_type: types::Config,

  /// The constructed path to the config file
  pub config_file: PathBuf,

  /// Paths specific to each detected monitor.
  #[serde(default)]
  pub monitor_paths: Vec<MonitorPaths>
}

impl Display for Config {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    printf!(f, "Home Directory", self.home_dir.display())?;
    printf!(f, "Downloads Directory", self.downloads_dir.display())?;
    printf!(f, "Favorites Directory", self.favorites_dir.display())?;
    printf!(f, "Wallpaper Directory", self.wallpaper_dir.display())?;
    printf!(f, "Config File", self.config_file.display())?;

    // for paths in &self.monitor_paths {
    //   printf!(
    //     f,
    //     &format!("Wallpapers [{}]", paths.name),
    //     paths.download_dir.display()
    //   )?;
    //   printf!(
    //     f,
    //     &format!("Wallpaper [{}]", paths.name),
    //     paths.current_wallpaper.display()
    //   )?;
    // }

    Ok(())
  }
}

impl Default for Config {
  fn default() -> Self {
    let title = env!("CARGO_PKG_NAME")
      .chars()
      .next()
      .unwrap()
      .to_uppercase()
      .chain(env!("CARGO_PKG_NAME").chars().skip(1))
      .collect::<String>();
    let home_dir = directories::UserDirs::new()
      .expect("Could not determine home directory")
      .home_dir()
      .to_path_buf()
      .join("Pictures")
      .join(title);
    let downloads_dir = home_dir.join("downloads");
    let favorites_dir = home_dir.join("favorites");
    let wallpaper_dir = home_dir.join("wallpaper");
    let config_name = "config".to_string();
    let config_type = types::Config::default();
    let config_file =
      home_dir.join(format!("{}.{}", config_name, config_type.extension()));

    Self {
      home_dir,
      downloads_dir,
      favorites_dir,
      wallpaper_dir,
      config_name,
      config_file,
      config_type,
      monitor_paths: Vec::new()
    }
  }
}

impl Config {
  pub fn new() -> Self {
    Self::default()
  }

  /// Returns the path to the monitor-specific wallpaper download directory.
  pub fn get_download_dir(&self, monitor: &Monitor) -> PathBuf {
    let monitor = &monitor.size;
    let ratio_dir = monitor.ratio_str();
    let resolution_dir = monitor.resolution_str();
    self.downloads_dir.join(ratio_dir).join(resolution_dir)
  }

  /// Create all necessary directories (home, downloads, favorites, wallpaper,
  /// monitor-specific) and the config file.
  pub fn create_all(&mut self, monitors: &[Monitor]) -> Result<()> {
    create_dir_all(&self.home_dir)?;
    create_dir_all(&self.downloads_dir)?;
    create_dir_all(&self.favorites_dir)?;
    create_dir_all(&self.wallpaper_dir)?;

    //{ Clear old paths and create monitor-specific paths }
    self.monitor_paths.clear();
    for monitor in monitors {
      let download_dir = self.get_download_dir(monitor);
      create_dir_all(&download_dir)?;

      // The path for the active wallpaper for this monitor.
      // We assume a default extension for now; the `set` command will manage
      // the actual file.
      let current_wallpaper =
        self.wallpaper_dir.join(format!("{}.png", monitor.name));

      self.monitor_paths.push(MonitorPaths {
        name: monitor.name.clone(),
        download_dir,
        current_wallpaper
      });
    }

    self.create_config_file(None)?;
    Ok(())
  }

  /// Create the config file if it does not exist.
  pub fn create_config_file(
    &self,
    default_content: Option<&str>
  ) -> Result<()> {
    if !self.config_exists() {
      let mut file = File::create(&self.config_file)?;
      if let Some(content) = default_content {
        file.write_all(content.as_bytes())?;
      }
    }
    Ok(())
  }

  /// Check if the config file exists.
  pub fn config_exists(&self) -> bool {
    self.config_file.exists()
  }

  /// Builder method to set the config file name.
  pub fn with_config_name<S: Into<String>>(mut self, name: S) -> Self {
    self.config_name = name.into();
    self.update_config_file();
    self
  }

  /// Builder method to set the config file type.
  pub fn with_type(mut self, config_type: types::Config) -> Self {
    self.config_type = config_type;
    self.update_config_file();
    self
  }

  /// Private helper to update the config_file path.
  fn update_config_file(&mut self) {
    self.config_file = self.home_dir.join(format!(
      "{}.{}",
      self.config_name,
      self.config_type.extension()
    ));
  }
}
