use crate::prelude::*;
pub use nu_ansi_term::Color::{
  self,
  Black,
  Blue,
  Cyan,
  DarkGray,
  Default as DefaultColor,
  Green,
  LightBlue,
  LightCyan,
  LightGray,
  LightGreen,
  LightMagenta,
  LightPurple,
  LightRed,
  LightYellow,
  Magenta,
  Purple,
  Red,
  White,
  Yellow,
};

pub static TERMINAL_SUPPORTS_COLOR: OnceLock<bool> = OnceLock::new();
pub fn terminal_supports_color() -> bool {
  *TERMINAL_SUPPORTS_COLOR
    .get_or_init(|| supports_color::on(supports_color::Stream::Stdout).is_some())
}

pub const ERROR_ANSI: &str = "\x1b[1;31m"; // Bold red
pub const WARN_ANSI: &str = "\x1b[1;33m"; // Bold yellow
pub const INFO_ANSI: &str = "\x1b[32m"; // Green
pub const DEBUG_ANSI: &str = "\x1b[36m"; // Cyan
pub const TRACE_ANSI: &str = "\x1b[2m"; // Dimmed
pub const RESET_ANSI: &str = "\x1b[0m";
