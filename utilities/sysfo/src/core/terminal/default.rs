use crate::core::traits::FormatHelpers;
use crossterm::terminal::size;

// TODO: make width and height min-max structs or traits
// TODO: make section struct or trait

#[derive(Debug, Clone, Copy)]
pub struct Info {
  pub width: u16,
  pub height: u16,
  pub max_width: u16,
  pub section_width: u16
}

impl Default for Info {
  fn default() -> Self {
    let (width, height) = size().unwrap_or((80, 24)); // Fallback to standard terminal size
    let max_width = width.min(120); // Cap maximum width
    let section_width = (max_width as f32 * 0.5) as u16; // Sections are half of max width

    Self {
      width,
      height,
      max_width,
      section_width
    }
  }
}

impl Info {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn separator_line(&self) -> String {
    "=".repeat(self.max_width as usize)
  }

  pub fn section_separator(&self) -> String {
    "-".repeat(self.section_width as usize)
  }

  // Get dynamic width for content padding
  pub fn content_padding(&self) -> usize {
    12 // Base padding for labels
  }
}

impl FormatHelpers for Info {
  fn format_section(&self, title: &str) -> String {
    format!("\n{}\n{}\n", title, self.section_separator())
  }

  fn format_field(&self, label: &str, value: &str) -> String {
    format!(
      "{:width$}: {}\n",
      label,
      value,
      width = self.content_padding()
    )
  }
}
