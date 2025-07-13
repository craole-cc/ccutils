use crate::utilities::get::latest_mtime;
use anyhow::Result;
use std::{
  env::consts::EXE_EXTENSION,
  fs::{metadata, read_to_string},
  path::{Path, PathBuf}
};

#[derive(Debug)]
pub struct Metadata {
  pub name: String,
  pub path: String,
  pub crate_type: Crate,
  pub is_installed: bool,
  pub has_prefixed_install: bool,
  pub needs_rebuild: Option<bool>
}

#[derive(Debug, PartialEq)]
pub enum Crate {
  Binary,
  Library,
  Both
}

pub struct Config {
  workspace_name: String,
  cargo_bin: PathBuf
}

impl Config {
  pub fn new(workspace_name: String, cargo_bin: PathBuf) -> Self {
    Self {
      workspace_name,
      cargo_bin
    }
  }

  pub fn list_crates(
    &self,
    members: &[String],
    binaries: &[String],
    detailed: bool,
    bins_only: bool,
    libs_only: bool
  ) -> Result<()> {
    let crate_infos = self.gather_crate_info(members, binaries)?;
    let filtered_crates = self.filter_crates(crate_infos, bins_only, libs_only);

    if filtered_crates.is_empty() {
      println!("No crates match the specified criteria.");
      return Ok(());
    }

    if detailed {
      self.print_detailed_list(&filtered_crates)?;
    } else {
      self.print_simple_list(&filtered_crates);
    }

    Ok(())
  }

  fn gather_crate_info(&self, members: &[String], binaries: &[String]) -> Result<Vec<Metadata>> {
    let mut crate_info = Vec::new();

    for member in members {
      let name = Path::new(member)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(member)
        .to_string();

      let is_binary = binaries.contains(member);
      let crate_type = self.determine_crate_type(member, is_binary)?;

      let (is_installed, has_prefixed_install) = if is_binary {
        self.check_installation_status(&name)?
      } else {
        (false, false)
      };

      let needs_rebuild = if is_binary && is_installed {
        Some(self.needs_rebuild(member, &name)?)
      } else {
        None
      };

      crate_info.push(Metadata {
        name,
        path: member.clone(),
        crate_type,
        is_installed,
        has_prefixed_install,
        needs_rebuild
      });
    }

    Ok(crate_info)
  }

  fn determine_crate_type(&self, member_path: &str, is_binary: bool) -> Result<Crate> {
    let cargo_toml_path = Path::new(member_path).join("Cargo.toml");
    let lib_rs_path = Path::new(member_path).join("src/lib.rs");

    let has_lib = lib_rs_path.exists() || self.has_lib_target(&cargo_toml_path)?;

    match (is_binary, has_lib) {
      (true, true) => Ok(Crate::Both),
      (true, false) => Ok(Crate::Binary),
      (false, true) => Ok(Crate::Library),
      (false, false) => Ok(Crate::Library)
    }
  }

  fn has_lib_target(&self, cargo_toml_path: &Path) -> Result<bool> {
    if !cargo_toml_path.exists() {
      return Ok(false);
    }

    let content = read_to_string(cargo_toml_path)?;
    let parsed: toml::Value = content.parse()?;

    //{ Check for explicit [lib] section }
    Ok(parsed.get("lib").is_some())
  }

  fn check_installation_status(&self, binary_name: &str) -> Result<(bool, bool)> {
    let unprefixed_path = self.cargo_bin.join(binary_name).with_extension(EXE_EXTENSION);

    let prefixed_name = format!("{}-{}", self.workspace_name, binary_name);
    let prefixed_path = self.cargo_bin.join(&prefixed_name).with_extension(EXE_EXTENSION);

    Ok((unprefixed_path.exists(), prefixed_path.exists()))
  }

  fn needs_rebuild(&self, member_path: &str, binary_name: &str) -> Result<bool> {
    let binary_path = self.cargo_bin.join(binary_name).with_extension(EXE_EXTENSION);

    if !binary_path.exists() {
      return Ok(true);
    }

    let binary_mtime = metadata(&binary_path)?.modified()?;
    let latest_src_mtime = latest_mtime(Path::new(member_path))?;

    Ok(latest_src_mtime > binary_mtime)
  }

  fn filter_crates(&self, crates: Vec<Metadata>, bins_only: bool, libs_only: bool) -> Vec<Metadata> {
    crates
      .into_iter()
      .filter(|info| {
        if bins_only && libs_only {
          true //? Show all if both flags are set
        } else if bins_only {
          matches!(info.crate_type, Crate::Binary | Crate::Both)
        } else if libs_only {
          matches!(info.crate_type, Crate::Library | Crate::Both)
        } else {
          true //? Show all by default
        }
      })
      .collect()
  }

  fn print_simple_list(&self, crates: &[Metadata]) {
    let rows: Vec<String> = crates
      .iter()
      .map(|info| {
        let built = match info.needs_rebuild {
          Some(true) => "✗",  // Needs rebuild
          Some(false) => "✓", // Up-to-date
          None => "-"         // N/A
        };

        let installed = match info.crate_type {
          Crate::Library => "-", // N/A for libraries
          _ => {
            // For Binary or Both
            if info.is_installed {
              "✓" // Is installed
            } else {
              "✗" // Is not installed
            }
          }
        };

        let prefixed = match info.crate_type {
          Crate::Library => "-", // N/A for libraries
          _ => {
            // For Binary or Both
            if info.has_prefixed_install {
              "✓" // Is prefixed
            } else {
              "✗" // Is not prefixed
            }
          }
        };

        format!(" [B:{} I:{} P:{}] {}", built, installed, prefixed, info.path)
      })
      .collect();

    println!(
      "Workspace Crates:\n{}\nB = Built | I = Installed | P = Prefixed\n",
      rows.join("\n")
    );
  }

  fn print_detailed_list(&self, crates: &[Metadata]) -> Result<()> {
    //{ Check if there are any workspace crates }
    if crates.is_empty() {
      println!("No workspace crates found");
      return Ok(());
    }

    let rows: Vec<String> = crates
      .iter()
      .map(|info| {
        let type_str = match info.crate_type {
          Crate::Binary => "bin",
          Crate::Library => "lib",
          Crate::Both => "bin + lib"
        };

        let install_status = match info.crate_type {
          Crate::Library => "N/A".to_string(),
          _ => {
            let mut status = Vec::new();
            if info.is_installed {
              status.push("installed");
            }
            if info.has_prefixed_install {
              status.push("prefixed");
            }
            if status.is_empty() {
              "Not installed".to_string()
            } else {
              status.join(" + ")
            }
          }
        };

        let rebuild_status = match info.needs_rebuild {
          Some(true) => "Yes",
          Some(false) => "No",
          None => "N/A"
        };

        format!(
          "{:<24} {:<12} {:<8} {:<22} {:<24}",
          info.name, type_str, rebuild_status, install_status, info.path
        )
      })
      .collect();

    //{ Print the table header and separator line }
    println!(
      "{:<24} {:<12} {:<8} {:<22} {:<24}",
      "Crate", "Type", "Up-to-date", "Status", "Path"
    );
    println!("{}", "-".repeat(90));
    println!("{}", rows.join("\n"));

    Ok(())
  }
}
