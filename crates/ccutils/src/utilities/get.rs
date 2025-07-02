use anyhow::{Context, Result};
use directories::UserDirs;
use std::{
  env::var,
  fs::{metadata, read_dir},
  path::{Path, PathBuf},
  time::SystemTime
};

pub fn cargo_bin_dir() -> Result<PathBuf> {
  //{ Cargo installs to $CARGO_HOME/bin. If $CARGO_HOME is not set, it defaults
  //{ to ~/.cargo }
  if let Ok(cargo_home) = var("CARGO_HOME") {
    return Ok(PathBuf::from(cargo_home).join("bin"));
  }

  let home_dir = UserDirs::new()
    .context("Could not determine user's home directory")?
    .home_dir()
    .to_path_buf();
  Ok(home_dir.join(".cargo").join("bin"))
}

/// Recursively finds the most recent modification timestamp in a directory.
pub fn latest_mtime(path: &Path) -> Result<SystemTime> {
  let mut system_time = SystemTime::UNIX_EPOCH;

  //{ If the path is a file, return its mtime }
  if !path.is_dir() {
    return if path.exists() {
      Ok(metadata(path)?.modified()?)
    } else {
      Ok(system_time)
    };
  }

  //{ If it's a directory, recurse }
  for entry in read_dir(path)? {
    let entry = entry?;
    let entry_path = entry.path();

    //{ Skip target directories and other build artifacts }
    if let Some(name) = entry_path.file_name().and_then(|n| n.to_str())
      && (name == "target" || name.starts_with('.'))
    {
      continue;
    }

    let modification_time = if entry_path.is_dir() {
      latest_mtime(&entry_path)?
    } else {
      metadata(&entry_path)?.modified()?
    };

    if modification_time > system_time {
      system_time = modification_time;
    }
  }

  Ok(system_time)
}
