use crate::{Error, Result};
use dark_light::{Mode, detect};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

pub trait Manager {
  fn set(&self, config: Config) -> Result<()>;
  fn notify(&self) -> Result<()>;
}

#[derive(
  Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone, Copy,
)]
pub enum Config {
  Light,
  Dark,
  #[default]
  Auto
}

impl Config {
  /// Creates a new `Config` instance using the default color configuration.
  /// The default mode is determined based on the system's current theme.
  /// Returns a `Result` containing the newly created `Config` instance.
  pub fn new() -> Result<Self> {
    Ok(Self::default())
  }

  fn get_current() -> Self {
    let fallback = Self::Dark;
    let detected = detect();
    match detected {
      Ok(Mode::Dark) => Self::Dark,
      Ok(Mode::Light) => Self::Light,
      Ok(Mode::Unspecified) => {
        eprintln!(
          "System color mode is unspecified. Using default mode: {fallback}"
        );
        fallback
      }
      Err(e) => {
        eprintln!(
          "Failed to detect the system's color mode: {e}. Using default mode: {fallback}"
        );
        fallback
      }
    }
  }

  /// Toggles the current color mode between `Light` and `Dark`.
  /// This function detects the current mode using the default detection logic,
  /// switches to the opposite mode, and applies the change.
  /// Returns the new mode upon successful application.
  pub fn toggle() -> Result<Self> {
    let current = Self::get_current();
    let desired = match current {
      Self::Light => Self::Dark,
      Self::Dark => Self::Light,
      Self::Auto => unreachable!("get_current always returns Light or Dark")
    };
    desired.apply().map(|_| desired)
  }

  pub fn apply(&self) -> Result<()> {
    let current = Self::get_current();
    // let desired = *self;
    let desired = match *self {
      // Self::Light => Self::Light,
      // Self::Dark => Self::Dark,
      Self::Auto => current,
      _ => *self
    };

    //{ Early return if mode is already set }
    if current == desired {
      println!("System mode is already {desired:?}");
      return Ok(());
    };

    //{ Set the system mode using the necessary platform-specific manager }
    println!("Setting system mode to {desired:?}");
    let manager: Box<dyn self::Manager> = {
      #[cfg(target_os = "windows")]
      {
        Box::new(super::windows::Manager::new_default())
      }
      #[cfg(target_os = "linux")]
      {
        Box::new(super::linux::Manager)
      }
      #[cfg(not(any(target_os = "windows", target_os = "linux")))]
      {
        // Define and implement UnsupportedManager directly here
        struct UnsupportedManager;
        impl self::Manager for UnsupportedManager {
          fn set(&self, _config: Config) -> Result<()> {
            eprintln!(
              "System theme setting is not supported on this platform."
            );
            Ok(())
          }

          fn notify(&self) -> Result<()> {
            // No-op for unsupported platforms
            Ok(())
          }
        }
        Box::new(UnsupportedManager)
      }
    };
    manager.set(desired);
    Ok(())
  }
}

impl Display for Config {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Light => write!(f, "Light"),
      Self::Dark => write!(f, "Dark"),
      Self::Auto => write!(f, "Auto")
    }
  }
}
