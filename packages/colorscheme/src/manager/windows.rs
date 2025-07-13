use super::utils::{Manager, Mode};
use winreg::RegKey;
use winreg::enums::*;

pub struct Windows {
  hkcu: RegKey,
  path: &'static str
}

impl Windows {
  pub fn new() -> Self {
    Self {
      hkcu: RegKey::predef(HKEY_CURRENT_USER),
      path: r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"
    }
  }
}

impl Manager for Windows {
  fn get(&self) -> Option<Mode> {
    if let Ok(key) = self.hkcu.open_subkey(self.path) {
      if let Ok(value) = key.get_value::<u32, _>("AppsUseLightTheme") {
        return Some(if value == 1 { Mode::Light } else { Mode::Dark });
      }
    }
    None
  }

  fn set(&self, theme: &Mode) -> Result<(), Box<dyn std::error::Error>> {
    let key = self.hkcu.open_subkey_with_flags(self.path, KEY_ALL_ACCESS)?;
    let value = match theme {
      Mode::Dark => 0u32,
      Mode::Light => 1u32
    };
    key.set_value("AppsUseLightTheme", &value)?;
    Ok(())
  }
}
