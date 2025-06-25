use crate::{Config, SymlinkError};
use chrono::Local;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

pub fn process_links(config: &Config) -> Result<(), SymlinkError> {
  debug!("Configuration:\n{:#?}", config);

  for src in &config.sources {
    if !src.exists() {
      warn!("Skipping non-existent source: {}", src.display());
      continue;
    }

    let link_path = config.resolve_link_path(src)?;

    debug!(
      "Processing: Source: {}, Link: {}",
      src.display(),
      link_path.display()
    );

    if link_path.exists() {
      if is_correct_symlink(&link_path, src)? {
        info!(
          "Symlink already exists and is correct: {}",
          link_path.display()
        );
        continue;
      }
      handle_existing_link(&link_path, config)?;
    }

    ensure_parent_directory_exists(&link_path, config.debug)?;

    info!(
      "Creating symbolic link from '{}' to '{}'",
      src.display(),
      link_path.display()
    );

    if !config.debug {
      create_symlink(src, &link_path)?;
    }
  }

  Ok(())
}

fn is_correct_symlink(
  link_path: &Path,
  src: &Path
) -> Result<bool, SymlinkError> {
  match fs::read_link(link_path) {
    Ok(target) => Ok(target == src),
    Err(e) => {
      if e.kind() == std::io::ErrorKind::InvalidInput {
        // Path exists but is not a symlink
        Ok(false)
      } else {
        Err(SymlinkError::Io(e))
      }
    }
  }
}

fn handle_existing_link(
  link_path: &Path,
  config: &Config
) -> Result<(), SymlinkError> {
  if config.force {
    backup_existing_path(link_path, config, None)?;
  } else {
    if !prompt_user_for_overwrite()? {
      info!("Skipping existing link: {}", link_path.display());
      return Ok(());
    }

    let mut backup_name = String::new();
    print!("Enter backup name (press Enter for timestamp): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut backup_name)?;
    let backup_name = if backup_name.trim().is_empty() {
      None
    } else {
      Some(backup_name.trim().to_string())
    };

    backup_existing_path(link_path, config, backup_name)?;
  }
  Ok(())
}

fn backup_existing_path(
  link_path: &Path,
  config: &Config,
  backup_name: Option<String>
) -> Result<(), SymlinkError> {
  if config.debug {
    return Ok(());
  }

  let backup_path = generate_backup_path(link_path, backup_name)?;
  info!(
    "Moving existing path '{}' to backup location '{}'",
    link_path.display(),
    backup_path.display()
  );

  fs::rename(link_path, &backup_path).map_err(|e| {
    SymlinkError::IoError(format!(
      "Failed to move '{}' to backup location '{}': {}",
      link_path.display(),
      backup_path.display(),
      e
    ))
  })?;

  Ok(())
}

fn generate_backup_path(
  original_path: &Path,
  backup_name: Option<String>
) -> Result<PathBuf, SymlinkError> {
  let parent = original_path.parent().ok_or_else(|| {
    SymlinkError::PathError(
      "Cannot determine parent directory for backup".to_string()
    )
  })?;

  let original_name = original_path.file_name().ok_or_else(|| {
    SymlinkError::PathError("Cannot determine file name for backup".to_string())
  })?;

  let backup_name = match backup_name {
    Some(name) => name,
    None => {
      let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
      format!("{}_backup_{}", original_name.to_string_lossy(), timestamp)
    }
  };

  Ok(parent.join(backup_name))
}

fn prompt_user_for_overwrite() -> Result<bool, SymlinkError> {
  print!("Do you want to overwrite it? [y/N] ");
  io::stdout().flush()?;

  let mut input = String::new();
  io::stdin().read_line(&mut input)?;

  Ok(input.trim().eq_ignore_ascii_case("y"))
}

fn ensure_parent_directory_exists(
  path: &Path,
  debug_mode: bool
) -> Result<(), SymlinkError> {
  if let Some(parent) = path.parent()
    && !parent.exists() {
      debug!("Creating parent directory: {}", parent.display());
      if !debug_mode {
        fs::create_dir_all(parent)?;
      }
    }
  Ok(())
}

#[cfg(unix)]
fn create_symlink(src: &Path, dst: &Path) -> Result<(), std::io::Error> {
  std::os::unix::fs::symlink(src, dst)
}

#[cfg(windows)]
fn create_symlink(src: &Path, dst: &Path) -> Result<(), SymlinkError> {
  let result = if src.is_dir() {
    std::os::windows::fs::symlink_dir(src, dst)
  } else {
    std::os::windows::fs::symlink_file(src, dst)
  };

  match result {
    Ok(_) => Ok(()),
    Err(e) if e.raw_os_error() == Some(1314) =>
      Err(SymlinkError::InsufficientPrivileges),
    Err(e) => Err(SymlinkError::Io(e))
  }
}
