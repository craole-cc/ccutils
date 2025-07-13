//! Enhanced Windows theme manager with additional registry keys and refresh
//! methods.
//!
//! This module extends the basic theme switching with additional registry
//! locations and more robust notification methods to ensure all Windows
//! elements update properly.

#![cfg_attr(feature = "windows-broadcast", allow(unsafe_code))]
// use super::NightlightState;
use crate::{
  Error, Result,
  config::color::mode::{Config as Mode, Manager as ModeManager, windows::nightlight},
  utils::registry
};
use std::{io, process::Command};
use winreg::{RegKey, enums::*};

/// Enhanced theme switching strategy with proper night-light support
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
  /// Set key values directly, may miss some componentsAuto
  FastMode,

  /// Set only Night Light settings, compatible with `Auto Dark Mode`
  Nightlight,

  /// Comprehensive option targeting all system components
  SystemComponents,

  /// Nuclear option with full system refresh
  ForceRefresh
}

impl Default for Strategy {
  fn default() -> Self {
    if registry::value_exists(
      HKEY_CURRENT_USER,
      r"Software\Microsoft\Windows\CurrentVersion\Run",
      "AutoDarkMode"
    ) || registry::key_exists(HKEY_CURRENT_USER, r"Software\AutoDarkMode\Installed")
      || registry::key_exists(HKEY_LOCAL_MACHINE, r"SOFTWARE\AutoDarkMode")
    {
      eprintln!("[DEBUG] Default Strategy: Auto Dark Mode detected, setting to Nightlight.");

      Self::Nightlight
    } else {
      eprintln!("[DEBUG] Default Strategy: Auto Dark Mode not detected, setting to SystemComponents.");
      Self::SystemComponents
    }
  }
}

/// Enhanced manager for Windows system color mode settings.
pub struct Manager {
  strategy: Strategy
}

impl Manager {
  /// Primary registry paths
  const REGISTRY_PATH: &str = r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize";
  const DWM_PATH: &str = r"Software\Microsoft\Windows\DWM";

  /// Registry key names
  const APPS_THEME_KEY: &str = "AppsUseLightTheme";
  const SYSTEM_THEME_KEY: &str = "SystemUsesLightTheme";
  const COLORPREVALENCE_KEY: &str = "ColorPrevalence";
  const ENABLETRANSPARENCY_KEY: &str = "EnableTransparency";

  /// DWM-specific keys for desktop window manager
  const DWM_COLORIZATIONCOLOR_KEY: &str = "ColorizationColor";
  const DWM_COLORIZATIONAFTERGLOW_KEY: &str = "ColorizationAfterglowBalance";
  const DWM_COLORIZATIONBLURBALANCE_KEY: &str = "ColorizationBlurBalance";

  /// Registry values
  const LIGHT_MODE_REG_VALUE: u32 = 1;
  const DARK_MODE_REG_VALUE: u32 = 0;

  /// DWM color values for light/dark theme
  const LIGHT_DWM_COLOR: u32 = 0xC40078D4; // Light blue accent //todo: use accent color
  const DARK_DWM_COLOR: u32 = 0xC4000000; // Dark theme colort found by reverse engineering for Win 11 24H2

  /// Create a new manager with the specified theme strategy
  pub fn new(strategy: Strategy) -> Self {
    Self { strategy }
  }

  /// Create a new manager with default strategy
  pub fn new_default() -> Self {
    Self::new(Strategy::default())
  }

  /// Set only the night-mode registry keys (most compatible with Auto Dark
  /// Mode)
  /// Set only Night Light (blue light filter) settings
  fn set_night_light(&self, mode: Mode) -> Result<()> {
    // No println! here, use logging if needed

    // Use the functions from the dedicated nightlight module.
    // This encapsulates all the complex byte parsing and registry logic.
    let changed = match mode {
      Mode::Dark => nightlight::enable()?, // Enable night light for dark mode
      Mode::Light => nightlight::disable()?, /* Disable night light for light */ // Use info! for logging
      // mode
      Mode::Auto => unreachable!()
    };

    // if changed {
    //   // No println! here, use logging if needed
    // } else {
    //   // No println! here, use logging if needed
    // }
    Ok(())
  }

  /// Set the primary theme registry keys
  fn set_primary_theme_keys(&self, value: u32) -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu
      .open_subkey_with_flags(Self::REGISTRY_PATH, KEY_ALL_ACCESS)
      .map_err(|e| {
        Error::ColorMode(format!(
          "Windows: Failed to open registry key '{}': {e}",
          Self::REGISTRY_PATH
        ))
      })?;

    // Set application theme
    key
      .set_value(Self::APPS_THEME_KEY, &value)
      .map_err(|e| Error::ColorMode(format!("Windows: Failed to set application theme registry value: {e}")))?;

    // Set system theme
    key
      .set_value(Self::SYSTEM_THEME_KEY, &value)
      .map_err(|e| Error::ColorMode(format!("Windows: Failed to set system theme registry value: {e}")))?;

    Ok(())
  }

  /// Set additional registry keys that might help with complete theme switching
  fn set_additional_theme_keys(&self, _value: u32, config: Mode) -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // Try to set additional keys in the personalize section
    if let Ok(key) = hkcu.open_subkey_with_flags(Self::REGISTRY_PATH, KEY_ALL_ACCESS) {
      // These might help with accent colors and transparency effects
      let _ = key.set_value(Self::COLORPREVALENCE_KEY, &0u32);
      let _ = key.set_value(Self::ENABLETRANSPARENCY_KEY, &1u32);
    }

    // Try to set DWM colorization keys
    if let Ok(dwm_key) = hkcu.open_subkey_with_flags(Self::DWM_PATH, KEY_ALL_ACCESS) {
      let dwm_color = match config {
        Mode::Light => Self::LIGHT_DWM_COLOR,
        Mode::Dark => Self::DARK_DWM_COLOR,
        Mode::Auto => unreachable!()
      };

      let _ = dwm_key.set_value(Self::DWM_COLORIZATIONCOLOR_KEY, &dwm_color);
      let _ = dwm_key.set_value(Self::DWM_COLORIZATIONAFTERGLOW_KEY, &0u32);
      let _ = dwm_key.set_value(Self::DWM_COLORIZATIONBLURBALANCE_KEY, &1u32);
    }

    Ok(())
  }

  /// Enhanced notification method with optimized timing
  fn notify_theme_change(&self) -> Result<()> {
    #[cfg(feature = "windows-broadcast")]
    {
      use std::ffi::CString;
      use std::ptr;
      use winapi::um::winuser::{
        HWND_BROADCAST, SMTO_ABORTIFHUNG, SendMessageTimeoutW, WM_DWMCOLORIZATIONCOLORCHANGED, WM_SETTINGCHANGE
      };

      // Optimized message sequence - most important first
      let messages = [
        ("ImmersiveColorSet", WM_SETTINGCHANGE),
        ("WindowsThemeElement", WM_SETTINGCHANGE),
        ("", WM_DWMCOLORIZATIONCOLORCHANGED)
      ];

      for (param_str, message_type) in &messages {
        let message = if param_str.is_empty() {
          None
        } else {
          Some(CString::new(*param_str).unwrap())
        };

        unsafe {
          SendMessageTimeoutW(
            HWND_BROADCAST,
            *message_type,
            0,
            match &message {
              Some(msg) => msg.as_ptr() as isize,
              None => 0
            },
            SMTO_ABORTIFHUNG,
            1000, // Reduced timeout for faster response
            ptr::null_mut()
          );
        }

        // Shorter delay between messages
        std::thread::sleep(std::time::Duration::from_millis(50));
      }
    }
    Ok(())
  }

  /// Gentle refresh methods that don't restart explorer
  fn refresh_explorer(&self) -> Result<()> {
    // Method 1: Update per-user system parameters (gentle refresh)
    let _ = Command::new("rundll32.exe")
      .args(["user32.dll,UpdatePerUserSystemParameters"])
      .output();

    // Method 2: Alternative gentle refresh using shell32
    let _ = Command::new("rundll32.exe")
      .args(["shell32.dll,SHChangeNotify"])
      .output();

    Ok(())
  }

  /// Targeted approach for stubborn system components (taskbar, Windows
  /// Terminal) This tries additional registry locations and specific refresh
  /// methods
  fn set_system_components(&self, config: Mode) -> Result<()> {
    let value = match config {
      Mode::Light => Self::LIGHT_MODE_REG_VALUE,
      Mode::Dark => Self::DARK_MODE_REG_VALUE,
      Mode::Auto => unreachable!()
    };

    // Set primary keys first
    self.set_primary_theme_keys(value)?;

    // Additional registry locations that might affect system components
    self.set_system_specific_keys(value, config)?;

    // Specific notifications for system components
    self.notify_system_components()?;

    // Taskbar-specific refresh
    self.refresh_taskbar()?;

    Ok(())
  }

  /// Set registry keys that specifically affect system components
  fn set_system_specific_keys(&self, value: u32, config: Mode) -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // Windows Terminal and Console settings
    let console_paths = [
      r"Console",
      r"Console\%SystemRoot%_system32_WindowsPowerShell_v1.0_powershell.exe",
      r"Console\%SystemRoot%_System32_cmd.exe"
    ];

    for path in &console_paths {
      if let Ok(console_key) = hkcu.open_subkey_with_flags(path, KEY_ALL_ACCESS) {
        // Set console color scheme based on theme
        let color_table = match config {
          Mode::Light => 0x00F0F0F0u32, // Light background
          Mode::Dark => 0x00000000u32,  // Dark background
          Mode::Auto => unreachable!()
        };
        let _ = console_key.set_value("ColorTable00", &color_table);
        let _ = console_key.set_value("ColorTable07", &(!color_table & 0x00FFFFFF));
      }
    }

    // Windows Explorer settings
    if let Ok(explorer_key) = hkcu.open_subkey_with_flags(
      r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
      KEY_ALL_ACCESS
    ) {
      // Force explorer to use the theme
      let _ = explorer_key.set_value("AppsUseLightTheme", &value);
      let _ = explorer_key.set_value("SystemUsesLightTheme", &value);
      // Additional explorer-specific settings
      let _ = explorer_key.set_value("EnableTransparency", &1u32);
      let _ = explorer_key.set_value("ColorPrevalence", &0u32);
    }

    // Taskbar-specific registry settings
    if let Ok(taskbar_key) = hkcu.open_subkey_with_flags(
      r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
      KEY_ALL_ACCESS
    ) {
      // Force taskbar transparency/theme awareness
      let taskbar_theme = match config {
        Mode::Light => 0u32,
        Mode::Dark => 1u32,
        Mode::Auto => unreachable!()
      };
      let _ = taskbar_key.set_value("UseColorization", &taskbar_theme);
      let _ = taskbar_key.set_value("ColorPrevalence", &0u32);
    }

    Ok(())
  }

  /// Send notifications specifically targeting system components
  fn notify_system_components(&self) -> Result<()> {
    #[cfg(feature = "windows-broadcast")]
    {
      use std::ffi::CString;
      use std::ptr;
      use winapi::um::winuser::{HWND_BROADCAST, SMTO_ABORTIFHUNG, SendMessageTimeoutW, WM_SETTINGCHANGE};

      // System-specific notification messages
      let system_messages = ["Environment", "Policy", "Windows", "ShellState"];

      for message_param in &system_messages {
        let message = CString::new(*message_param).unwrap();

        unsafe {
          SendMessageTimeoutW(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            0,
            message.as_ptr() as isize,
            SMTO_ABORTIFHUNG,
            500,
            ptr::null_mut()
          );
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
      }
    }

    Ok(())
  }

  /// Specific taskbar refresh methods
  fn refresh_taskbar(&self) -> Result<()> {
    // Method 1: Refresh taskbar specifically
    let _ = Command::new("powershell")
      .args([
        "-Command",
        "Stop-Process -Name explorer -Force; Start-Sleep 1; Start-Process explorer"
      ])
      .output();

    // Method 2: Alternative taskbar refresh
    let _ = Command::new("taskkill").args(["/f", "/im", "explorer.exe"]).output();

    std::thread::sleep(std::time::Duration::from_millis(500));

    let _ = Command::new("explorer.exe").spawn();

    Ok(())
  }

  /// Alternative method: Use registry-only approach with better notifications
  /// This is faster and doesn't cause shell issues
  fn set_fast_mode(&self, config: Mode) -> Result<()> {
    let value = match config {
      Mode::Light => Self::LIGHT_MODE_REG_VALUE,
      Mode::Dark => Self::DARK_MODE_REG_VALUE,
      Mode::Auto => unreachable!()
    };

    // Set primary theme keys
    self.set_primary_theme_keys(value)?;

    // Send optimized notifications
    self.send_optimized_notifications()?;

    // Wait a moment for changes to propagate
    std::thread::sleep(std::time::Duration::from_millis(200));

    Ok(())
  }

  /// Send only the most effective notifications without delays
  fn send_optimized_notifications(&self) -> Result<()> {
    #[cfg(feature = "windows-broadcast")]
    {
      use std::ffi::CString;
      use std::ptr;
      use winapi::um::winuser::{
        HWND_BROADCAST, SMTO_ABORTIFHUNG, SendMessageTimeoutW, WM_DWMCOLORIZATIONCOLORCHANGED, WM_SETTINGCHANGE
      };

      // Just the essential messages
      let immersive_msg = CString::new("ImmersiveColorSet").unwrap();

      unsafe {
        // Primary theme change notification
        SendMessageTimeoutW(
          HWND_BROADCAST,
          WM_SETTINGCHANGE,
          0,
          immersive_msg.as_ptr() as isize,
          SMTO_ABORTIFHUNG,
          500, // Very short timeout
          ptr::null_mut()
        );

        // DWM color change notification
        SendMessageTimeoutW(
          HWND_BROADCAST,
          WM_DWMCOLORIZATIONCOLORCHANGED,
          0,
          0,
          SMTO_ABORTIFHUNG,
          500,
          ptr::null_mut()
        );
      }

      // Add a broader WM_SETTINGCHANGE notification
      unsafe {
        SendMessageTimeoutW(
          HWND_BROADCAST,
          WM_SETTINGCHANGE,
          0,
          0, // NULL for lParam
          SMTO_ABORTIFHUNG,
          500,
          ptr::null_mut()
        );
      }
    }

    Ok(())
  }

  /// Nuclear option: Force complete system refresh
  /// Use this only if other methods fail
  fn force_system_refresh(&self, config: Mode) -> Result<()> {
    // Set all registry keys
    self.set_system_components(config)?;

    // Wait for registry changes to propagate
    std::thread::sleep(std::time::Duration::from_millis(1000));

    // Force explorer restart (this will cause temporary desktop disruption)
    let _ = Command::new("taskkill").args(["/f", "/im", "explorer.exe"]).output();

    std::thread::sleep(std::time::Duration::from_millis(2000));

    let _ = Command::new("explorer.exe").spawn();

    // Wait for explorer to fully restart
    std::thread::sleep(std::time::Duration::from_millis(3000));

    Ok(())
  }

  /// Check current theme state
  pub fn get_current_theme(&self) -> Result<Mode> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu
      .open_subkey(Self::REGISTRY_PATH)
      .map_err(|e| Error::ColorMode(format!("Failed to read theme state: {e}")))?;

    let system_light: u32 = key
      .get_value(Self::SYSTEM_THEME_KEY)
      .unwrap_or(Self::LIGHT_MODE_REG_VALUE);

    Ok(if system_light == Self::LIGHT_MODE_REG_VALUE {
      Mode::Light
    } else {
      Mode::Dark
    })
  }

  /// Wait for theme change to take effect (polling method)
  pub fn wait_for_theme_change(&self, expected: Mode, timeout_ms: u64) -> Result<bool> {
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_millis(timeout_ms);

    while start.elapsed() < timeout {
      if let Ok(current) = self.get_current_theme()
        && std::mem::discriminant(&current) == std::mem::discriminant(&expected)
      {
        return Ok(true);
      }
      std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(false)
  }
}

impl Default for Manager {
  fn default() -> Self {
    Self::new_default()
  }
}

impl ModeManager for Manager {
  fn set(&self, config: Mode) -> Result<()> {
    // Use the configured strategy to set the theme
    match self.strategy {
      Strategy::Nightlight => {
        self.set_night_light(config)?;
        self.notify()?;
      }
      Strategy::FastMode => {
        // Fast with optimized notifications, may miss some components
        self.set_fast_mode(config)?;
      }
      Strategy::SystemComponents => {
        // Comprehensive but slower, may conflict with Windhawk
        self.set_system_components(config)?;
        // Multiple notification attempts with different methods
        self.notify_theme_change()?;
        // Gentle refresh without restarting explorer (log warning if it fails)
        if let Err(e) = self.refresh_explorer() {
          // Use error! for warnings
          eprintln!("[DEBUG] Failed to refresh desktop: {e}");
        }
      }
      Strategy::ForceRefresh => {
        // Nuclear option - slow and causes temporary disruption
        self.force_system_refresh(config)?;
      }
    }

    Ok(())
  }

  fn notify(&self) -> Result<()> {
    match self.strategy {
      Strategy::Nightlight => self.send_optimized_notifications(),
      Strategy::FastMode => self.send_optimized_notifications(),
      _ => self.notify_theme_change()
    }
  }
}
