//! Metadata domain model.
//!
//! Pure value object representing name, version, and description.
//! Used by both Workspace and Package.

use crate::prelude::*;

/// Metadata for workspace or package.
///
/// Pure domain model - no infrastructure concerns.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Metadata {
  pub name: String,
  pub version: String,
  pub description: String,
}

impl Metadata {
  /// Creates empty metadata.
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  /// Creates metadata with all fields.
  #[must_use]
  pub fn from_parts(
    name: impl Into<String>,
    version: impl Into<String>,
    description: impl Into<String>,
  ) -> Self {
    Self {
      name: name.into(),
      version: version.into(),
      description: description.into(),
    }
  }

  //╔═══════════════════════════════════════════════════════════╗
  //║ Builders                                                  ║
  //╚═══════════════════════════════════════════════════════════╝

  /// Sets the name.
  #[must_use]
  pub fn with_name(mut self, name: impl Into<String>) -> Self {
    self.name = name.into();
    self
  }

  /// Sets the version.
  #[must_use]
  pub fn with_version(mut self, version: impl Into<String>) -> Self {
    self.version = version.into();
    self
  }

  /// Sets the description.
  #[must_use]
  pub fn with_description(mut self, description: impl Into<String>) -> Self {
    self.description = description.into();
    self
  }

  //╔═══════════════════════════════════════════════════════════╗
  //║ Queries                                                   ║
  //╚═══════════════════════════════════════════════════════════╝

  /// Checks if metadata is empty (all fields empty).
  #[must_use]
  pub const fn is_empty(&self) -> bool {
    // self.name.is_empty() && self.version.is_empty() && self.description.is_empty()
    self.name.len() == 0 && self.version.len() == 0 && self.description.len() == 0
  }

  /// Checks if metadata has a name.
  #[must_use]
  pub const fn has_name(&self) -> bool {
    !self.name.is_empty()
  }

  /// Returns a display string: "name v.version".
  #[must_use]
  pub fn display_name(&self) -> String {
    if self.version.is_empty() {
      self.name.clone()
    } else {
      format!("{} v{}", self.name, self.version)
    }
  }
}

impl Display for Metadata {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.display_name())
  }
}
