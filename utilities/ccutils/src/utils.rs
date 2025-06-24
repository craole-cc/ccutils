use anyhow::{Context, Result};
use directories::UserDirs;
use std::{env, fs, path::{Path, PathBuf}, time::SystemTime};

pub fn get_cargo_bin_dir() -> Result<PathBuf> {
    //{ Cargo installs to $CARGO_HOME/bin. If $CARGO_HOME is not set, it defaults to ~/.cargo }
    if let Ok(cargo_home) = env::var("CARGO_HOME") {
        return Ok(PathBuf::from(cargo_home).join("bin"));
    }

    let home_dir = UserDirs::new()
        .context("Could not determine user's home directory")?
        .home_dir()
        .to_path_buf();
    Ok(home_dir.join(".cargo").join("bin"))
}

/// Recursively finds the most recent modification timestamp in a directory.
pub fn get_latest_mtime(path: &Path) -> Result<SystemTime> {
    let mut latest_mtime = SystemTime::UNIX_EPOCH;

    //{ If the path is a file, return its mtime }
    if !path.is_dir() {
        return if path.exists() {
            Ok(fs::metadata(path)?.modified()?)
        } else {
            Ok(latest_mtime)
        };
    }

    //{ If it's a directory, recurse }
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        //{ Skip target directories and other build artifacts }
        if let Some(name) = entry_path.file_name().and_then(|n| n.to_str()) {
            if name == "target" || name.starts_with('.') {
                continue;
            }
        }

        let mtime = if entry_path.is_dir() {
            get_latest_mtime(&entry_path)?
        } else {
            fs::metadata(&entry_path)?.modified()?
        };

        if mtime > latest_mtime {
            latest_mtime = mtime;
        }
    }

    Ok(latest_mtime)
}
