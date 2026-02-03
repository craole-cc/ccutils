//! Environment operation mode.

use crate::_prelude::*;

/// Environment operation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
  /// Workspace with multiple packages.
  ///
  /// Discovers workspace root, reads workspace `Cargo.toml`,
  /// sets up paths relative to workspace root.
  Workspace,

  /// Standalone single package.
  ///
  /// No workspace - package is the root. Reads package `Cargo.toml` only.
  Standalone,

  /// Library mode - minimal initialization.
  ///
  /// Used when imported as a dependency. No filesystem discovery,
  /// only uses compile-time env vars.
  Library,
}

impl Default for Kind {
  fn default() -> Self {
    Self::detect()
  }
}

impl Kind {
  /// Auto-detect environment kind from cargo environment variables.
  ///
  /// # Detection Strategy
  /// 1. Check for `CARGO_WORKSPACE_DIR` → `Workspace`
  /// 2. Check for `CARGO_MANIFEST_DIR` → `Standalone`
  /// 3. Fallback → `Library`
  ///
  /// # Examples
  /// ```
  /// use craole_cc_project::prelude::*;
  ///
  /// let kind = Kind::detect();
  /// match kind {
  ///   Kind::Workspace => println!("Running in workspace"),
  ///   Kind::Standalone => println!("Standalone package"),
  ///   Kind::Library => println!("Library mode"),
  /// }
  /// ```
  #[must_use]
  pub fn detect() -> Self {
    if var("CARGO_WORKSPACE_DIR").is_ok() {
      Self::Workspace
    } else if var("CARGO_MANIFEST_DIR").is_ok() {
      Self::Standalone
    } else {
      Self::Library
    }
  }

  /// Check if this is workspace mode.
  ///
  /// # Examples
  /// ```
  /// use craole_cc_project::prelude::*;
  ///
  /// if Kind::Workspace.is_workspace() {
  ///   println!("Multi-package workspace");
  /// }
  /// ```
  #[must_use]
  pub const fn is_workspace(self) -> bool {
    matches!(self, Self::Workspace)
  }

  /// Check if this is standalone mode.
  ///
  /// # Examples
  /// ```
  /// use craole_cc_project::prelude::*;
  ///
  /// if Kind::Standalone.is_standalone() {
  ///   println!("Single package");
  /// }
  /// ```
  #[must_use]
  pub const fn is_standalone(self) -> bool {
    matches!(self, Self::Standalone)
  }

  /// Check if this is library mode.
  ///
  /// # Examples
  /// ```
  /// use craole_cc_project::prelude::*;
  ///
  /// if Kind::Library.is_library() {
  ///   println!("Running as imported library");
  /// }
  /// ```
  #[must_use]
  pub const fn is_library(self) -> bool {
    matches!(self, Self::Library)
  }

  /// Returns a human-readable string representation.
  ///
  /// # Examples
  /// ```
  /// use craole_cc_project::prelude::*;
  ///
  /// assert_eq!(Kind::Workspace.as_str(), "workspace");
  /// assert_eq!(Kind::Standalone.as_str(), "standalone");
  /// assert_eq!(Kind::Library.as_str(), "library");
  /// ```
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Workspace => "workspace",
      Self::Standalone => "standalone",
      Self::Library => "library",
    }
  }

  /// Create from string (case-insensitive).
  ///
  /// # Examples
  /// ```
  /// use craole_cc_project::prelude::*;
  ///
  /// assert_eq!(Kind::parse("workspace"), Some(Kind::Workspace));
  /// assert_eq!(Kind::parse("STANDALONE"), Some(Kind::Standalone));
  /// assert_eq!(Kind::parse("invalid"), None);
  /// ```
  #[must_use]
  pub fn parse(s: &str) -> Option<Self> {
    match s.to_lowercase().as_str() {
      "workspace" => Some(Self::Workspace),
      "standalone" | "binary" | "bin" => Some(Self::Standalone),
      "library" | "lib" => Some(Self::Library),
      _ => None,
    }
  }

  /// Check if workspace features should be enabled.
  ///
  /// Returns `true` for `Workspace` and `Standalone`, `false` for `Library`.
  ///
  /// # Examples
  /// ```
  /// use craole_cc_project::prelude::*;
  ///
  /// assert!(Kind::Workspace.should_discover_workspace());
  /// assert!(Kind::Standalone.should_discover_workspace());
  /// assert!(!Kind::Library.should_discover_workspace());
  /// ```
  #[must_use]
  pub const fn should_discover_workspace(self) -> bool {
    !matches!(self, Self::Library)
  }

  /// Check if filesystem operations should be performed.
  ///
  /// Returns `false` only for `Library` mode.
  ///
  /// # Examples
  /// ```
  /// use craole_cc_project::prelude::*;
  ///
  /// let kind = Kind::Workspace;
  /// if kind.can_access_filesystem() {
  ///   // Safe to read Cargo.toml, discover paths, etc.
  /// }
  /// ```
  #[must_use]
  pub const fn can_access_filesystem(self) -> bool {
    self.should_discover_workspace()
  }
}

impl Display for Kind {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.as_str())
  }
}

impl FromStr for Kind {
  type Err = String;

  fn from_str(s: &str) -> StdResult<Self, Self::Err> {
    Self::parse(s).ok_or_else(|| {
      format!("Invalid environment kind: '{s}'. Expected: workspace, standalone, or library")
    })
  }
}
