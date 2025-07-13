// TODO: Make this a global lib, or update embellish
use std::fmt::{self, Display, Formatter};

/// Writes a formatted key-value pair to the provided `Formatter`.
///
/// The key is left-aligned and padded to the specified width (`pad`),
/// followed by an equals sign and a pipe character, then the value.
/// The entire line is indented by the given number of spaces (`indent`).
///
/// # Arguments
///
/// * `f` - A mutable reference to a `Formatter` where the output is written.
/// * `key` - The key to be displayed, left-aligned and padded.
/// * `value` - The value associated with the key.
/// * `pad` - The width to which the key is padded.
/// * `indent` - The number of spaces to indent the entire line.
///
/// # Returns
///
/// * `fmt::Result` - Result indicating success or failure of the write operation.
pub fn pout_field<T: Display>(f: &mut Formatter<'_>, key: &str, value: T, pad: usize, indent: usize) -> fmt::Result {
  writeln!(f, "{}{key:<pad$}=| {value}", " ".repeat(indent))
}

/// Writes a heading to the provided `Formatter`.
///
/// The heading is indented by the specified number of spaces (`indent`)
/// and appears as a plain text line.
///
/// # Arguments
///
/// * `f` - A mutable reference to a `Formatter` where the output is written.
/// * `text` - The text of the heading to be displayed.
/// * `indent` - The number of spaces to indent the heading.
///
/// # Returns
///
/// * `fmt::Result` - Result indicating success or failure of the write operation.
pub fn pout_heading(f: &mut Formatter<'_>, text: &str, indent: usize) -> fmt::Result {
  writeln!(f, "{}{}", " ".repeat(indent), text)
}

/// Macro for concise field printing, forwarding to `pout_field`.
#[macro_export]
macro_rules! printf {
  ($f:expr, $key:expr, $value:expr, $pad:expr, $indent:expr) => {
    $crate::print::pout_field($f, $key, $value, $pad, $indent)
  };
  ($f:expr, $key:expr, $value:expr, $pad:expr) => {
    $crate::print::pout_field($f, $key, $value, $pad, 4)
  };
  ($f:expr, $key:expr, $value:expr) => {
    $crate::print::pout_field($f, $key, $value, 24, 4)
  };
}

/// Macro for concise heading printing, forwarding to `pout_heading`.
#[macro_export]
macro_rules! printh {
  ($f:expr, $text:expr, $indent:expr) => {
    $crate::print::pout_heading($f, $text, $indent)
  };
  ($f:expr, $text:expr) => {
    $crate::print::pout_heading($f, $text, 2)
  };
}
