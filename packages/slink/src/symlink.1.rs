use crate::config::Config;
use crate::error::SymlinkError;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use tracing::{debug, info, warn};

pub fn process_links(config: &Config) -> Result<(), SymlinkError> {
  for src in &config.sources {
    if !src.exists() {
      warn!("Skipping non-existent source: {}", src.display());
      continue;
    }

    let link_path = config.resolve_link_path(src)?;

    debug!("Processing: Source: {}, Link: {}", src.display(), link_path.display());

    if link_path.exists() {
      if config.force {
        info!("Removing existing link: {}", link_path.display());
        if !config.debug {
          fs::remove_file(&link_path).or_else(|_| fs::remove_dir(&link_path))?;
        }
      } else {
        info!("Link already exists: {}", link_path.display());
        if !prompt_user_for_overwrite()? {
          info!("Skipping existing link: {}", link_path.display());
          continue;
        }
        if !config.debug {
          fs::remove_file(&link_path).or_else(|_| fs::remove_dir(&link_path))?;
        }
      }
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

fn prompt_user_for_overwrite() -> Result<bool, SymlinkError> {
  print!("Do you want to overwrite it? [y/N] ");
  io::stdout().flush()?;

  let mut input = String::new();
  io::stdin().read_line(&mut input)?;

  Ok(input.trim().eq_ignore_ascii_case("y"))
}

fn prompt_user_for_backup() -> Result<Option<String>, SymlinkError> {
  print!("Do you want to make a backup? [Y/n] ");
  io::stdout().flush()?;

  let mut input = String::new();
  io::stdin().read_line(&mut input)?;

  if input.trim().eq_ignore_ascii_case("n") {
    return Ok(None);
  }

  print!("Enter backup name (leave empty for default): ");
  io::stdout().flush()?;

  let mut backup_name = String::new();
  io::stdin().read_line(&mut backup_name)?;

  let backup_name = backup_name.trim();
  if backup_name.is_empty() {
    Ok(Some(".bac".to_string()))
  } else {
    Ok(Some(backup_name.to_string()))
  }
}

// ...

fn ensure_parent_directory_exists(path: &Path, debug_mode: bool) -> Result<(), SymlinkError> {
  if let Some(parent) = path.parent() {
    if !parent.exists() {
      debug!("Creating parent directory: {}", parent.display());
      if !debug_mode {
        fs::create_dir_all(parent)?;
      }
    }
  }
  Ok(())
}

#[cfg(unix)]
fn create_symlink(src: &Path, dst: &Path) -> Result<(), std::io::Error> {
  std::os::unix::fs::symlink(src, dst)
}

#[cfg(windows)]
fn create_symlink(src: &Path, dst: &Path) -> Result<(), std::io::Error> {
  if src.is_dir() {
    std::os::windows::fs::symlink_dir(src, dst)
  } else {
    std::os::windows::fs::symlink_file(src, dst)
  }
}
