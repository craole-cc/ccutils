
// Formatting helper trait
pub trait FormatHelpers {
  fn format_section(&self, title: &str) -> String;
  fn format_field(&self, label: &str, value: &str) -> String;
}
