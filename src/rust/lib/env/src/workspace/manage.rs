//! Workspace management operations (create, add/remove members).

use crate::_prelude::*;

/// Workspace manager for creation and member management.
pub struct WorkspaceManager {
  root: PathBuf,
}

impl WorkspaceManager {
  /// Create a new workspace manager for the given root.
  pub fn new(root: impl AsRef<Path>) -> Self {
    Self {
      root: root.as_ref().to_path_buf(),
    }
  }

  /// Create a new workspace at the given path.
  ///
  /// # Errors
  ///
  /// Returns an error if the workspace directory cannot be created or if the Cargo.toml file cannot be written.
  ///
  /// # Panics
  ///
  /// Panics if the TOML serialization fails (which should not happen with valid TOML structures).
  pub fn create(name: &str, path: &Path) -> Result<PathBuf> {
    let workspace_path = path.join(name);
    create_dir_all(&workspace_path)?;

    let cargo_toml = workspace_path.join("Cargo.toml");
    let mut toml = TomlTable::new();

    // Create [workspace] section
    let mut workspace = TomlTable::new();
    workspace.insert("members".to_string(), TomlValue::Array(vec![]));
    workspace.insert("resolver".to_string(), TomlValue::String("2".to_string()));

    toml.insert("workspace".to_string(), TomlValue::Table(workspace));

    // Write Cargo.toml
    write(&cargo_toml, to_toml_string_pretty(&toml).unwrap())?;

    #[cfg(feature = "tracing")]
    info!("Created workspace: {}", workspace_path.display());

    Ok(workspace_path)
  }

  /// Add a member to the workspace.
  ///
  /// # Errors
  ///
  /// Returns an error if the Cargo.toml file cannot be read or written, or if the workspace structure is invalid.
  ///
  /// # Panics
  ///
  /// Panics if the TOML serialization fails (which should not happen with valid TOML structures).
  pub fn add_member(&self, member_path: &str) -> Result<()> {
    let cargo_toml_path = self.root.join("Cargo.toml");
    let contents = read_to_string(&cargo_toml_path)?;
    let mut toml: TomlTable =
      from_toml_str(&contents).map_err(|e| IOError::new(IOErrorKind::InvalidData, e))?;

    // Get or create workspace.members array
    let members = toml
      .entry("workspace".to_string())
      .or_insert_with(|| TomlValue::Table(TomlTable::new()))
      .as_table_mut()
      .and_then(|w| {
        w.entry("members".to_string())
          .or_insert_with(|| TomlValue::Array(vec![]))
          .as_array_mut()
      })
      .ok_or_else(|| IOError::new(IOErrorKind::InvalidData, "Invalid workspace structure"))?;

    // Add member if not already present
    let member_value = TomlValue::String(member_path.to_string());
    if !members.contains(&member_value) {
      members.push(member_value);
    }

    // Write back
    write(&cargo_toml_path, to_toml_string_pretty(&toml).unwrap())?;

    #[cfg(feature = "tracing")]
    info!("Added member: {}", member_path);

    Ok(())
  }

  /// Remove a member from the workspace.
  ///
  /// # Errors
  ///
  /// Returns an error if the Cargo.toml file cannot be read or written, or if the workspace structure is invalid.
  ///
  /// # Panics
  ///
  /// Panics if the TOML serialization fails (which should not happen with valid TOML structures).
  pub fn remove_member(&self, member_path: &str) -> Result<()> {
    let cargo_toml_path = self.root.join("Cargo.toml");
    let contents = read_to_string(&cargo_toml_path)?;
    let mut toml: TomlTable =
      from_toml_str(&contents).map_err(|e| IOError::new(IOErrorKind::InvalidData, e))?;

    // Get workspace.members array
    let members = toml
      .get_mut("workspace")
      .and_then(|w| w.as_table_mut())
      .and_then(|w| w.get_mut("members"))
      .and_then(|m| m.as_array_mut())
      .ok_or_else(|| IOError::new(IOErrorKind::NotFound, "No workspace members found"))?;

    // Remove member
    members.retain(|m| m.as_str() != Some(member_path));

    // Write back
    write(&cargo_toml_path, to_toml_string_pretty(&toml).unwrap())?;

    #[cfg(feature = "tracing")]
    info!("Removed member: {}", member_path);

    Ok(())
  }
}
