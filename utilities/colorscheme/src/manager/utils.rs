use crate::manager::windows::Windows;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Mode {
  Dark,
  Light
}

pub trait Manager {
  fn get(&self) -> Option<Mode>;
  fn set(&self, theme: &Mode) -> Result<(), Box<dyn std::error::Error>>;
}

pub fn manager() -> Option<Box<dyn Manager>> {
  #[cfg(target_os = "windows")]
  {
    Some(Box::new(Windows::new()))
  }
  #[cfg(target_os = "linux")]
  {
    Some(Box::new(linux::Manager::new()))
  }
  #[cfg(not(any(target_os = "windows", target_os = "linux")))]
  {
    None
  }
}

pub fn toggle(current: &Mode) -> Mode {
  match current {
    Mode::Dark => Mode::Light,
    Mode::Light => Mode::Dark
  }
}
