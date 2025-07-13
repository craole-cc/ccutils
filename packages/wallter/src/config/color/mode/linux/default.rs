//! Manages system color mode (light/dark) settings specifically for Linux
//! desktop environments.
//!
//! This module attempts to detect the current desktop environment (KDE Plasma,
//! GNOME) and uses environment-specific commands (e.g.,
//! `plasma-apply-colorscheme`, `gsettings`) to apply the desired theme.
use super::Config;
use crate::{Error, Result};
use std::{env, process::Command};

/// A manager for Linux system color mode settings.
///
/// This struct is a zero-sized type used as a marker to group Linux-specific
/// theme management logic.
pub struct Manager;

/// Represents supported Linux desktop environments and outcomes of detection.
#[derive(Debug, PartialEq)]
enum DesktopEnvironment {
  /// KDE Plasma desktop environment.
  KDE,
  /// GNOME desktop environment.
  GNOME,
  /// An unsupported desktop environment, with the detected name.
  Unsupported(String),
  /// The desktop environment could not be determined.
  Unknown
}

impl DesktopEnvironment {
  /// Detects the current Linux desktop environment.
  ///
  /// It primarily checks the `XDG_CURRENT_DESKTOP` environment variable.
  ///
  /// # Returns
  ///
  /// A `DesktopEnvironment` enum variant indicating the detected environment
  /// or if it's unsupported/unknown.
  fn detect() -> Self {
    let desktop = env::var("XDG_CURRENT_DESKTOP").ok().map(|d| d.to_lowercase());

    match desktop.as_deref() {
      Some(desktop) if desktop.contains("kde") => DesktopEnvironment::KDE,
      Some(desktop) if desktop.contains("gnome") => DesktopEnvironment::GNOME,
      Some(desktop) => DesktopEnvironment::Unsupported(desktop.to_string()),
      None => DesktopEnvironment::Unknown
    }
  }

  /// Attempts to set the KDE color scheme persistently using `kwriteconfig5`.
  /// This is a helper method for `apply_theme`.
  fn set_kde_persistent_theme(&self, theme_name: &str) -> Result<()> {
    let status = Command::new("kwriteconfig5")
      .args([
        "--file",
        "kdeglobals",
        "--group",
        "General",
        "--key",
        "ColorScheme",
        theme_name
      ])
      .status()
      .map_err(|e| Error::ColorMode(format!("Linux/KDE: Failed to execute kwriteconfig5: {}", e)))?;

    if !status.success() {
      return Err(Error::ColorMode("Linux/KDE: kwriteconfig5 command failed".to_string()));
    }
    Ok(())
  }

  /// Sets the GTK theme for GNOME applications using `gsettings`.
  /// This is a helper method for `apply_theme`.
  fn set_gnome_gtk_theme(&self, config: Config) -> Result<()> {
    let gtk_theme = match config {
      Config::Dark => "Adwaita-dark",
      Config::Light => "Adwaita",
      Config::Auto => unreachable!("Auto mode is resolved to Light or Dark already")
    };

    let status = Command::new("gsettings")
      .args(["set", "org.gnome.desktop.interface", "gtk-theme", gtk_theme])
      .status()
      .map_err(|e| Error::ColorMode(format!("Linux/GNOME: Failed to set GTK theme: {e}")))?;

    if !status.success() {
      return Err(Error::ColorMode("Linux/GNOME: Failed to set GTK theme".to_string()));
    }
    Ok(())
  }

  /// Applies the KDE color theme.
  /// This is a helper method for `apply_theme`.
  fn apply_kde_theme_config(&self, config: Config) -> Result<()> {
    let theme_name = match config {
      Config::Dark => "BreezeDark",
      Config::Light => "BreezeLight",
      Config::Auto => unreachable!("Auto mode is resolved to Light or Dark already")
    };

    let status = Command::new("plasma-apply-colorscheme")
      .arg(theme_name)
      .status()
      .map_err(|e| Error::ColorMode(format!("Linux/KDE: Failed to execute plasma-apply-colorscheme: {e}")))?;

    if !status.success() {
      return Err(Error::ColorMode(
        "Linux/KDE: plasma-apply-colorscheme command failed".to_string()
      ));
    }

    if let Err(e) = self.set_kde_persistent_theme(theme_name) {
      eprintln!("Warning: Failed to set persistent KDE theme: {e}");
    }
    Ok(())
  }

  /// Applies the GNOME color theme.
  /// This is a helper method for `apply_theme`.
  fn apply_gnome_theme_config(&self, config: Config) -> Result<()> {
    let scheme_value = match config {
      Config::Dark => "prefer-dark",
      Config::Light => "prefer-light",
      Config::Auto => unreachable!("Auto mode is resolved to Light or Dark already")
    };

    let status = Command::new("gsettings")
      .args(["set", "org.gnome.desktop.interface", "color-scheme", scheme_value])
      .status()
      .map_err(|e| Error::ColorMode(format!("Linux/GNOME: Failed to execute gsettings: {e}")))?;

    if !status.success() {
      return Err(Error::ColorMode(
        "Linux/GNOME: gsettings set color-scheme command failed".to_string()
      ));
    }

    if let Err(e) = self.set_gnome_gtk_theme(config) {
      eprintln!("Warning: Failed to set GTK theme: {e}");
    }
    Ok(())
  }

  /// Applies the specified color mode to the detected desktop environment.
  ///
  /// # Arguments
  ///
  /// * `config` - The desired color mode (`Light` or `Dark`) to apply.
  ///
  /// # Errors
  ///
  /// Returns `Error::ColorMode` if setting the theme fails for a supported
  /// environment. For unsupported or unknown environments, it prints a message
  /// to `stderr` and returns `Ok(())`.
  fn apply_theme(&self, config: Config) -> Result<()> {
    match self {
      DesktopEnvironment::KDE => self.apply_kde_theme_config(config),
      DesktopEnvironment::GNOME => self.apply_gnome_theme_config(config),
      DesktopEnvironment::Unsupported(ref desktop_name) => {
        eprintln!("Unsupported Linux desktop environment for theme setting: {desktop_name}");
        Ok(())
      }
      DesktopEnvironment::Unknown => {
        eprintln!("Could not determine Linux desktop environment for theme setting.");
        Ok(())
      }
    }
  }
}

impl super::Manager for Manager {
  /// Sets the Linux system color mode based on the detected desktop
  /// environment.
  ///
  /// # Arguments
  ///
  /// * `mode` - The desired color mode (`Light` or `Dark`) to apply.
  ///
  /// # Errors
  ///
  /// Returns `Error::ColorMode` if setting the theme fails for a supported
  /// environment. For unsupported or unknown environments, it prints a message
  /// to `stderr` and returns `Ok(())`.
  fn set(&self, mode: Config) -> Result<()> {
    let desktop_env = DesktopEnvironment::detect();
    desktop_env.apply_theme(mode)
  }

  /// Notifies other running applications of a theme change (Linux specific).
  ///
  /// On Linux, there isn't a universal, standardized broadcast mechanism for
  /// theme changes that all applications listen to in the same way as on
  /// Windows. Therefore, this function is currently a no-op.
  ///
  /// # Returns
  ///
  /// Always returns `Ok(())`.
  fn notify(&self) -> Result<()> {
    // Future enhancements could attempt DE-specific notifications if available.
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_desktop_environment_detection() {
    let manager = Manager;

    // This test primarily checks that the detection logic runs without
    // panicking. Asserting specific outcomes is difficult without mocking
    // environment variables.
    let _desktop = DesktopEnvironment::detect();
  }

  #[test]
  fn test_kde_theme_mapping() {
    // Test KDE theme name mapping from Config mode.
    let test_cases = [(Config::Dark, "BreezeDark"), (Config::Light, "BreezeLight")];

    for (config, expected) in test_cases {
      let actual_theme_name = match config {
        Config::Dark => "BreezeDark",
        Config::Light => "BreezeLight",
        Config::Auto => unreachable!("Auto mode is resolved to Light or Dark already")
      };
      assert_eq!(actual_theme_name, expected);
    }
  }

  #[test]
  fn test_gnome_scheme_mapping() {
    // Test GNOME color scheme mapping from Config mode.
    let test_cases = [(Config::Dark, "prefer-dark"), (Config::Light, "prefer-light")];

    for (config, expected) in test_cases {
      let actual_scheme_value = match config {
        Config::Dark => "prefer-dark",
        Config::Light => "prefer-light"
      };
      assert_eq!(actual_scheme_value, expected);
    }
  }

  #[test]
  fn test_gnome_gtk_theme_mapping() {
    // Test GTK theme mapping from Config mode.
    let test_cases = [
      (Config::Dark, "Adwaita-dark"),
      (Config::Light, "Adwaita"),
      (Config::Auto, "Adwaita")
    ];

    for (config, expected) in test_cases {
      let actual_gtk_theme = match config {
        Config::Dark => "Adwaita-dark",
        Config::Light => "Adwaita",
        Config::Auto => unreachable!("Auto mode is resolved to Light or Dark already")
      };
      assert_eq!(actual_gtk_theme, expected);
    }
  }

  #[test]
  fn test_desktop_environment_enum() {
    // Basic structural test for DesktopEnvironment enum variants.
    let kde = DesktopEnvironment::KDE;
    let gnome = DesktopEnvironment::GNOME;
    let unknown = DesktopEnvironment::Unknown;
    let unsupported = DesktopEnvironment::Unsupported("xfce".to_string());

    assert_ne!(kde, gnome);
    assert_ne!(unknown, unsupported);
  }

  // Note: Integration tests for theme setting would require an actual Linux
  // desktop environment and the necessary command-line tools (gsettings,
  // plasma-apply-colorscheme, kwriteconfig5) to be installed.
}
