use std::fmt::{self, Formatter};

/// Print a padded key-value field with a custom separator for uniform CLI
/// output.
///
/// # Example
/// ```
/// print_field(f, "Name", "DISPLAY1", 11)?;
/// ```
pub fn pout_field<T: fmt::Display>(
  f: &mut Formatter<'_>,
  key: &str,
  value: T,
  pad: usize,
  indent: usize
) -> fmt::Result {
  writeln!(f, "{}{key:<pad$}=| {value}", " ".repeat(indent))
}

/// Print an indented heading.
pub fn pout_heading(
  f: &mut Formatter<'_>,
  text: &str,
  indent: usize
) -> fmt::Result {
  writeln!(f, "{}{}", " ".repeat(indent), text)
}

/// Macro for concise field printing, forwarding to `pout_field`.
#[macro_export]
macro_rules! printf {
  ($f:expr, $key:expr, $value:expr, $pad:expr, $indent:expr) => {
    $crate::utils::print::pout_field($f, $key, $value, $pad, $indent)
  };
  ($f:expr, $key:expr, $value:expr, $pad:expr) => {
    $crate::utils::print::pout_field($f, $key, $value, $pad, 4)
  };
  ($f:expr, $key:expr, $value:expr) => {
    $crate::utils::print::pout_field($f, $key, $value, 24, 4)
  };
}

/// Macro for concise heading printing, forwarding to `pout_heading`.
#[macro_export]
macro_rules! printh {
  ($f:expr, $text:expr, $indent:expr) => {
    $crate::utils::print::pout_heading($f, $text, $indent)
  };
  ($f:expr, $text:expr) => {
    $crate::utils::print::pout_heading($f, $text, 2)
  };
}
