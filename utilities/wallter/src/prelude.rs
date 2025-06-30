pub use crate::api::Api;
pub use crate::config::Config;
pub use crate::consts::*;
pub use crate::error::{Error, Result};
pub use crate::utils::*;

// -- OS-specific Exports --
cfg_if! {
    if #[cfg(target_os = "windows")] {
      pub use crate::config::color::mode::windows::nightlight;
    } else if #[cfg(target_os = "linux")] {
    } else { }
}
