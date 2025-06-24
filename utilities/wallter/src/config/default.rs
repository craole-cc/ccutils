use super::{Color, ColorMode, ConfigType, Monitor, Path, Search, Slideshow};
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{
  fmt::{self, Display, Formatter},
  fs::{create_dir_all, read_to_string, write}
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
  pub path: Path,
  pub monitors: Vec<Monitor>,
  pub color: Color,
  pub slideshow: Slideshow,
  pub source: Search
}

impl Config {
  /// Initializes the config: creates all dirs, creates config file if missing,
  /// and loads or saves config.
  pub fn init(path_config: &mut Path) -> Result<Self> {
    //{ Always enumerate current monitors to have them ready for path creation }
    let detected_monitors = Monitor::get_info()?;

    //{ Ensure all necessary paths exist, including monitor-specific ones }
    path_config.create_all(&detected_monitors)?;

    //{ Try to load config from file, or fall back to default and save it }
    let mut config = match Self::load(path_config) {
      Ok(cfg) => cfg,
      Err(_) => {
        let mut default_cfg = Self::default();
        default_cfg.save(path_config)?;
        default_cfg
      }
    };

    //{ Apply color mode from config if it's explicit and differs from system }
    match config.color.mode {
      ColorMode::Light | ColorMode::Dark => {
        config.color.mode.apply()?;
      }
      ColorMode::Auto => { /* Do nothing, let the system control the theme */ }
    }

    //{ Update the config with the detected monitors and paths }
    config.monitors = detected_monitors;
    config.path = path_config.clone();

    //{ Return the initialized config }
    Ok(config)
  }

  /// Loads the configuration from the config file if it exists, otherwise
  /// returns default.
  pub fn load(path_config: &Path) -> Result<Self> {
    //{ Retrieve the contents of the config file }
    let content = read_to_string(&path_config.config_file)?;

    //{ Parse the contents of the config file based on the defined format }
    match path_config.config_type {
      ConfigType::Toml =>
        toml::from_str(&content).map_err(|e| Error::Config(e.to_string())),
      ConfigType::Json =>
        serde_json::from_str(&content).map_err(|e| Error::Config(e.to_string())),
    }
  }

  /// Saves the configuration to the config file
  pub fn save(&self, path_config: &Path) -> Result<()> {
    //{ Serialize to appropriate format }
    let contents = match path_config.config_type {
      ConfigType::Toml =>
        toml::to_string(self).map_err(|e| Error::Config(e.to_string()))?,
      ConfigType::Json => serde_json::to_string_pretty(self)
        .map_err(|e| Error::Config(e.to_string()))?
    };

    //{ Update the configuration file }
    write(&path_config.config_file, contents)?;
    Ok(())
  }
}

impl Display for Config {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    writeln!(f, "Configuration:")?;

    //|-> Paths Section
    writeln!(f, "  Paths:\n{}", self.path)?;

    //|-> Monitors Section
    if self.monitors.is_empty() {
      printh!(f, "Monitors: No monitors detected")?;
    } else {
      printh!(f, "Monitors:")?;
      for monitor in &self.monitors {
        //{ Print the monitor's own fields using its Display implementation }
        write!(f, "{monitor}")?;

        //{ To toggle path display, comment out the following line }
        printh!(f, "Wallpapers:", 4)?;
        monitor.display_wallpaper_paths(f, &self.path)?;

        //{ Add a blank line for separation before the next monitor }.
        writeln!(f)?;
      }
    }

    //|-> Color Section
    writeln!(f, "  Colors:\n{}", self.color)?;

    //|-> Source Section
    if self.source.sources.is_empty() {
      writeln!(f, "  Search: No sources configured")?;
    } else {
      writeln!(f, "  Search:")?;
      writeln!(f, "{}", self.source)?;
    }

    //|-> Slideshow Section
    if self.slideshow.sources.is_empty() {
      writeln!(f, "  Slideshow: No wallpaper sources configured")?;
    } else {
      writeln!(f, "  Slideshow:")?;
      writeln!(f, "{}", self.slideshow)?;
    }

    Ok(())
  }
}

/// Helper function to initialize the configuration with default path config.
pub fn init() -> crate::Result<Config> {
  let mut path_config = Path::default();
  Config::init(&mut path_config)
}
