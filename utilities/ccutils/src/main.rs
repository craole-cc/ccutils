use anyhow::{Context, Result, bail};
use directories::UserDirs;
use std::{
  env, fs,
  path::{Path, PathBuf},
  process::Command,
  time::SystemTime
};

fn main() -> Result<()> {
  //{ Find the workspace root directory and toml file }
  let workspace_toml = find_workspace_toml(
    &env::current_dir().context("Failed to get current dir")?
  )?;

  let binary_crates = find_binary_crates(&workspace_toml)?;

  let cargo_bin_dir = get_cargo_bin_dir()?;

  let mut to_rebuild = Vec::new();
  println!("\n--- Checking for updated crates ---");
  for member in &binary_crates {
    if needs_rebuild(member, &cargo_bin_dir)? {
      to_rebuild.push(member.clone());
    }
  }

  if to_rebuild.is_empty() {
    println!("\nAll binary crates are up to date. Nothing to do.");
    return Ok(());
  }

  println!(
    "\n--- Building {} updated binary crates ---",
    to_rebuild.len()
  );
  for member in &to_rebuild {
    build_binary(member)
      .with_context(|| format!("Failed to build '{member}'"))?;
  }

  println!(
    "\n--- Installing {} updated binary crates ---",
    to_rebuild.len()
  );
  for member in &to_rebuild {
    install_binary(member)
      .with_context(|| format!("Failed to install '{member}'"))?;
  }
  Ok(())
}

fn find_workspace_toml(start: &Path) -> Result<PathBuf> {
  let mut path = start.to_path_buf();
  loop {
    let cargo_toml = path.join("Cargo.toml");
    if cargo_toml.exists() {
      let content = fs::read_to_string(&cargo_toml)?;
      if content.contains("[workspace]") {
        return Ok(cargo_toml);
      }
    }
    if !path.pop() {
      break;
    }
  }
  bail!("Could not find workspace root from '{}'", start.display())
}

fn find_binary_crates(cargo_toml: &Path) -> Result<Vec<String>> {
  //{ Read the main Cargo.toml }
  let content = fs::read_to_string(cargo_toml).with_context(|| {
    format!(
      "Failed to read workspace Cargo.toml at '{}'",
      cargo_toml.display()
    )
  })?;
  let parsed: toml::Value = content.parse().with_context(|| {
    format!(
      "Failed to parse workspace Cargo.toml at '{}'",
      cargo_toml.display()
    )
  })?;

  //{ Extract workspace members }
  let members = parsed["workspace"]["members"]
    .as_array()
    .context("No `[workspace.members]` array found in workspace Cargo.toml")?
    .iter()
    .filter_map(|m| m.as_str());

  //{ Initialize the list of binary crates }
  let mut binary_crates = Vec::new();

  //{ Update the list of binary crates of the workspace members }
  for member in members {
    let cargo_path = Path::new(member).join("Cargo.toml");
    if !cargo_path.exists() {
      continue;
    }

    let member_toml = fs::read_to_string(&cargo_path).with_context(|| {
      format!("Failed to read member Cargo.toml at '{:?}'", cargo_path)
    })?;
    let member_parsed: toml::Value =
      member_toml.parse().with_context(|| {
        format!("Failed to parse member Cargo.toml at '{:?}'", cargo_path)
      })?;

    //{  Find crates with `[[bin]]` section or a `src/main.rs` file }
    let has_explicit_bin_target = member_parsed
      .get("bin")
      .and_then(|v| v.as_array())
      .is_some();
    let is_binary_by_convention =
      Path::new(member).join("src/main.rs").exists();

    if has_explicit_bin_target || is_binary_by_convention {
      binary_crates.push(member.to_string());
    }
  }
  Ok(binary_crates)
}

fn get_cargo_bin_dir() -> Result<PathBuf> {
  // Cargo installs to $CARGO_HOME/bin. If $CARGO_HOME is not set, it defaults
  // to ~/.cargo.
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
fn get_latest_mtime(path: &Path) -> Result<SystemTime> {
  let mut latest_mtime = SystemTime::UNIX_EPOCH;

  // If the path is a file, return its mtime.
  if !path.is_dir() {
    return if path.exists() {
      Ok(fs::metadata(path)?.modified()?)
    } else {
      Ok(latest_mtime)
    };
  }

  // If it's a directory, recurse.
  for entry in fs::read_dir(path)? {
    let entry = entry?;
    let path = entry.path();
    let mtime = if path.is_dir() {
      get_latest_mtime(&path)?
    } else {
      fs::metadata(&path)?.modified()?
    };
    if mtime > latest_mtime {
      latest_mtime = mtime;
    }
  }

  Ok(latest_mtime)
}

/// Checks if a binary crate needs to be rebuilt by comparing the modification
/// times of its source files against the installed binary.
fn needs_rebuild(member: &str, cargo_bin_dir: &Path) -> Result<bool> {
  let binary_name = Path::new(member)
    .file_name()
    .and_then(|name| name.to_str())
    .with_context(|| {
      format!("Could not determine binary name from path: {member}")
    })?;

  // On Windows, cargo install adds .exe. On Linux/macOS, it does not.
  // `env::consts::EXE_EXTENSION` is "" on non-Windows, so this works for all.
  let installed_binary_path = cargo_bin_dir
    .join(binary_name)
    .with_extension(env::consts::EXE_EXTENSION);

  if !installed_binary_path.exists() {
    println!("-> Crate '{binary_name}' needs install (not found).");
    return Ok(true);
  }

  let binary_mtime = fs::metadata(&installed_binary_path)?.modified()?;
  let latest_src_mtime = get_latest_mtime(&Path::new(member))?;

  if latest_src_mtime > binary_mtime {
    println!("-> Crate '{binary_name}' needs rebuild (source is newer).");
    Ok(true)
  } else {
    println!("-> Crate '{binary_name}' is up to date.");
    Ok(false)
  }
}

fn build_binary(member: &str) -> Result<()> {
  // Assumes the binary name is the same as the final component of the member
  // path. e.g., for a member path "utilities/wallter", the binary name is
  // "wallter".
  let binary_name = Path::new(member)
    .file_name()
    .and_then(|name| name.to_str())
    .with_context(|| {
      format!("Could not determine binary name from path: {member}")
    })?;

  println!("Building binary '{binary_name}' in release mode...");

  let status = Command::new("cargo")
    .arg("build")
    .arg("--release")
    .arg("--bin")
    .arg(binary_name)
    .status()?;

  if !status.success() {
    bail!("Cargo build failed for '{binary_name}' with status: {status}");
  }

  Ok(())
}

fn install_binary(member: &str) -> Result<()> {
  println!("Installing {member}...");
  let status = Command::new("cargo")
    .arg("install")
    .arg("--path")
    .arg(member)
    .status()?;

  if !status.success() {
    bail!("Cargo install failed for '{member}' with status: {status}");
  }

  Ok(())
}
