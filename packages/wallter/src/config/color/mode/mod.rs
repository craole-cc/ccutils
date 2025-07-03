mod default;
pub use default::{Config, Manager};

cfg_if! {
  if #[cfg(target_os = "windows")] {
    mod windows;
    pub use windows::*;
  } else if #[cfg(target_os = "linux")] {
    pub mod linux;
    pub use linux::*;
  } else { }
}
