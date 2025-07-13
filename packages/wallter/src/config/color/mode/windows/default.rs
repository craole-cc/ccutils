//! Enhanced Windows theme manager with additional registry keys and refresh
//! methods.
//!
//! This module extends the basic theme switching with additional registry
//! locations and more robust notification methods to ensure all Windows
//! elements update properly.

#![cfg_attr(feature = "windows-broadcast", allow(unsafe_code, dead_code))]
use crate::{
  Error, Result,
  config::color::mode::{
    Config as Mode, Manager as ModeManager, windows::nightlight
  },
  utils::registry
};
use std::{
  mem::discriminant,
  process::Command,
  thread::sleep,
  time::{Duration, Instant}
};
use winreg::{RegKey, enums::*};

/// Enhanced theme switching strategy with proper night-light support
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
  /// Set only Night Light settings, compatible with `Auto Dark Mode`
  Nightlight,
  /// Set key values directly with optimized notifications
  FastMode,
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
    ) || registry::key_exists(
      HKEY_CURRENT_USER,
      r"Software\AutoDarkMode\Installed"
    ) || registry::key_exists(HKEY_LOCAL_MACHINE, r"SOFTWARE\AutoDarkMode")
    {
      eprintln!("[DEBUG] Auto Dark Mode detected, using Nightlight strategy");
      Self::Nightlight
    } else {
      eprintln!(
        "[DEBUG] Auto Dark Mode not detected, using SystemComponents strategy"
      );
      Self::SystemComponents
    }
  }
}

/// Enhanced manager for Windows system color mode settings.
pub struct Manager {
  strategy: Strategy
}

impl Manager {
  /// Registry paths
  const PERSONALIZE_PATH: &str =
    r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize";
  const DWM_PATH: &str = r"Software\Microsoft\Windows\DWM";
  const EXPLORER_ADVANCED_PATH: &str =
    r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced";

  /// Primary theme registry keys
  const APPS_THEME_KEY: &str = "AppsUseLightTheme";
  const SYSTEM_THEME_KEY: &str = "SystemUsesLightTheme";

  /// Registry values for light/dark mode
  const LIGHT_MODE_VALUE: u32 = 1;
  const DARK_MODE_VALUE: u32 = 0;

  /// DWM color values for light/dark theme
  const LIGHT_DWM_COLOR: u32 = 0xC40078D4;
  const DARK_DWM_COLOR: u32 = 0xC4000000;

  /// Create a new manager with the specified theme strategy
  pub fn new(strategy: Strategy) -> Self {
    Self { strategy }
  }

  /// Create a new manager with default strategy
  pub fn new_default() -> Self {
    Self::new(Strategy::default())
  }

  /// Get registry value for the given mode
  fn mode_to_registry_value(mode: Mode) -> u32 {
    match mode {
      Mode::Light => Self::LIGHT_MODE_VALUE,
      Mode::Dark => Self::DARK_MODE_VALUE,
      Mode::Auto =>
        unreachable!("Auto mode should be resolved before this point"),
    }
  }

  /// Get DWM color for the given mode
  fn mode_to_dwm_color(mode: Mode) -> u32 {
    match mode {
      Mode::Light => Self::LIGHT_DWM_COLOR,
      Mode::Dark => Self::DARK_DWM_COLOR,
      Mode::Auto =>
        unreachable!("Auto mode should be resolved before this point"),
    }
  }

  /// Set only Night Light (blue light filter) settings
  fn set_night_light(&self, mode: Mode) -> Result<()> {
    let changed = match mode {
      Mode::Dark => nightlight::enable()?,
      Mode::Light => nightlight::disable()?,
      Mode::Auto => unreachable!()
    };

    if changed {
      eprintln!("[DEBUG] Night light mode changed");
    }

    Ok(())
  }

  /// Check current theme state
  pub fn get_current_theme(&self) -> Result<Mode> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu.open_subkey(Self::PERSONALIZE_PATH).map_err(|e| {
      Error::ColorMode(format!("Failed to read theme state: {e}"))
    })?;

    let system_light: u32 = key
      .get_value(Self::SYSTEM_THEME_KEY)
      .unwrap_or(Self::LIGHT_MODE_VALUE);

    Ok(if system_light == Self::LIGHT_MODE_VALUE {
      Mode::Light
    } else {
      Mode::Dark
    })
  }

  /// Wait for theme change to take effect
  pub fn wait_for_theme_change(
    &self,
    expected: Mode,
    timeout_ms: u64
  ) -> Result<bool> {
    let start = Instant::now();
    let timeout = Duration::from_millis(timeout_ms);

    while start.elapsed() < timeout {
      if let Ok(current) = self.get_current_theme()
        && discriminant(&current) == discriminant(&expected)
      {
        return Ok(true);
      }
      sleep(Duration::from_millis(100));
    }

    Ok(false)
  }

  /// Set the primary theme registry keys
  fn set_primary_theme_keys(&self, mode: Mode) -> Result<()> {
    let value = Self::mode_to_registry_value(mode);
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu
      .open_subkey_with_flags(Self::PERSONALIZE_PATH, KEY_ALL_ACCESS)
      .map_err(|e| {
        Error::ColorMode(format!(
          "Failed to open personalize registry key: {e}"
        ))
      })?;

    key.set_value(Self::APPS_THEME_KEY, &value).map_err(|e| {
      Error::ColorMode(format!("Failed to set application theme: {e}"))
    })?;

    key.set_value(Self::SYSTEM_THEME_KEY, &value).map_err(|e| {
      Error::ColorMode(format!("Failed to set system theme: {e}"))
    })?;

    Ok(())
  }

  /// Set DWM colorization settings
  fn set_dwm_colors(&self, mode: Mode) -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(dwm_key) =
      hkcu.open_subkey_with_flags(Self::DWM_PATH, KEY_ALL_ACCESS)
    {
      let dwm_color = Self::mode_to_dwm_color(mode);
      let _ = dwm_key.set_value("ColorizationColor", &dwm_color);
      let _ = dwm_key.set_value("ColorizationAfterglowBalance", &0u32);
      let _ = dwm_key.set_value("ColorizationBlurBalance", &1u32);
    }
    Ok(())
  }

  /// Set system-specific registry keys for better component coverage
  fn set_system_specific_keys(&self, mode: Mode) -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // Set console color schemes
    self.set_console_colors(&hkcu, mode)?;

    // Set taskbar-specific settings
    self.set_taskbar_settings(&hkcu, mode)?;

    Ok(())
  }

  /// Configure console applications color schemes
  fn set_console_colors(&self, hkcu: &RegKey, mode: Mode) -> Result<()> {
    let console_paths = [
      r"Console",
      r"Console\%SystemRoot%_system32_WindowsPowerShell_v1.0_powershell.exe",
      r"Console\%SystemRoot%_System32_cmd.exe"
    ];

    let (bg_color, fg_color) = match mode {
      Mode::Light => (0x00F0F0F0u32, 0x00000000u32),
      Mode::Dark => (0x00000000u32, 0x00F0F0F0u32),
      Mode::Auto => unreachable!()
    };

    for path in &console_paths {
      if let Ok(console_key) = hkcu.open_subkey_with_flags(path, KEY_ALL_ACCESS)
      {
        let _ = console_key.set_value("ColorTable00", &bg_color);
        let _ = console_key.set_value("ColorTable07", &fg_color);
      }
    }

    Ok(())
  }

  /// Configure taskbar-specific settings
  fn set_taskbar_settings(&self, hkcu: &RegKey, mode: Mode) -> Result<()> {
    if let Ok(taskbar_key) =
      hkcu.open_subkey_with_flags(Self::EXPLORER_ADVANCED_PATH, KEY_ALL_ACCESS)
    {
      let taskbar_theme = Self::mode_to_registry_value(mode);
      let _ = taskbar_key.set_value("UseColorization", &taskbar_theme);
      let _ = taskbar_key.set_value("ColorPrevalence", &0u32);
    }
    Ok(())
  }

  /// Send Windows messages to notify about theme changes
  fn send_theme_notifications(&self) -> Result<()> {
    #[cfg(feature = "windows-broadcast")]
    {
      use std::ffi::CString;
      use std::ptr;
      use winapi::um::winuser::{
        HWND_BROADCAST, SMTO_ABORTIFHUNG, SendMessageTimeoutW,
        WM_DWMCOLORIZATIONCOLORCHANGED, WM_SETTINGCHANGE
      };

      let messages = [
        ("ImmersiveColorSet", WM_SETTINGCHANGE),
        ("WindowsThemeElement", WM_SETTINGCHANGE),
        ("", WM_DWMCOLORIZATIONCOLORCHANGED)
      ];

      for (param_str, message_type) in &messages {
        use std::thread::sleep;

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
            1000,
            ptr::null_mut()
          );
        }

        sleep(Duration::from_millis(50));
      }
    }
    Ok(())
  }

  /// Send optimized notifications for fast mode
  fn send_optimized_notifications(&self) -> Result<()> {
    #[cfg(feature = "windows-broadcast")]
    {
      use std::ffi::CString;
      use std::ptr;
      use winapi::um::winuser::{
        HWND_BROADCAST, SMTO_ABORTIFHUNG, SendMessageTimeoutW,
        WM_DWMCOLORIZATIONCOLORCHANGED, WM_SETTINGCHANGE
      };

      let immersive_msg = CString::new("ImmersiveColorSet").unwrap();

      unsafe {
        SendMessageTimeoutW(
          HWND_BROADCAST,
          WM_SETTINGCHANGE,
          0,
          immersive_msg.as_ptr() as isize,
          SMTO_ABORTIFHUNG,
          500,
          ptr::null_mut()
        );

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
    }
    Ok(())
  }

  /// Gentle refresh of system UI without restarting explorer
  fn refresh_system_ui(&self) -> Result<()> {
    let _ = Command::new("rundll32.exe")
      .args(["user32.dll,UpdatePerUserSystemParameters"])
      .output();

    let _ = Command::new("rundll32.exe")
      .args(["shell32.dll,SHChangeNotify"])
      .output();

    Ok(())
  }

  /// Restart explorer.exe (disruptive but effective)
  fn restart_explorer(&self) -> Result<()> {
    let _ = Command::new("taskkill")
      .args(["/f", "/im", "explorer.exe"])
      .output();

    sleep(Duration::from_millis(1000));
    let _ = Command::new("explorer.exe").spawn();
    sleep(Duration::from_millis(2000));

    Ok(())
  }

  /// Fast mode implementation
  fn apply_fast_mode(&self, mode: Mode) -> Result<()> {
    self.set_primary_theme_keys(mode)?;
    self.send_optimized_notifications()?;
    sleep(Duration::from_millis(200));
    Ok(())
  }

  /// System components mode implementation
  fn apply_system_components(&self, mode: Mode) -> Result<()> {
    self.set_primary_theme_keys(mode)?;
    self.set_dwm_colors(mode)?;
    self.set_system_specific_keys(mode)?;
    self.send_theme_notifications()?;

    if let Err(e) = self.refresh_system_ui() {
      eprintln!("[DEBUG] Failed to refresh system UI: {e}");
    }

    Ok(())
  }

  /// Force refresh implementation
  fn apply_force_refresh(&self, mode: Mode) -> Result<()> {
    self.apply_system_components(mode)?;
    sleep(Duration::from_millis(1000));
    self.restart_explorer()?;
    Ok(())
  }
}

impl Default for Manager {
  fn default() -> Self {
    Self::new_default()
  }
}

impl ModeManager for Manager {
  fn set(&self, mode: Mode) -> Result<()> {
    match self.strategy {
      Strategy::Nightlight => {
        self.set_night_light(mode)?;
        self.send_optimized_notifications()?;
      }
      Strategy::FastMode => {
        self.apply_fast_mode(mode)?;
      }
      Strategy::SystemComponents => {
        self.apply_system_components(mode)?;
      }
      Strategy::ForceRefresh => {
        self.apply_force_refresh(mode)?;
      }
    }

    Ok(())
  }

  fn notify(&self) -> Result<()> {
    match self.strategy {
      Strategy::Nightlight | Strategy::FastMode =>
        self.send_optimized_notifications(),
      Strategy::SystemComponents | Strategy::ForceRefresh =>
        self.send_theme_notifications(),
    }
  }
}
