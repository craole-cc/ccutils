use directories::BaseDirs;
use std::{path::PathBuf, process::Command};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
  Bash,
  Zsh,
  Fish,
  CommandPrompt,
  PowerShell,
  Nushell,
  #[default]
  Unsupported
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Info {
  pub id: u32,
  pub name: String,
  pub path: PathBuf,
  pub conf: Vec<PathBuf>,
  pub version: Option<String>
}

impl Info {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn fetch(&self) -> String {
    format!(
      "Time {{\n\
            {:>16}: {}\n\
            {:>16}: {}\n\
            {:>16}: {}\n\
            {:>16}: {}\n\
            }}",
      "Name",
      self.name,
      "Path",
      self.path.display(),
      "Version",
      self.version.as_deref().unwrap_or("Unknown"),
      "Configurations ",
      self
        .conf
        .iter()
        .map(|p| p.display().to_string())
        .collect::<Vec<_>>()
        .join(", ")
    )
  }
}

pub fn get_config_paths(kind: &Kind) -> Vec<PathBuf> {
  let dirs = match BaseDirs::new() {
    Some(dirs) => dirs,
    None => return vec![] /* Return an empty vector if base directories
                           * cannot be determined. */
  };

  let home = dirs.home_dir();

  match kind {
    Kind::Bash => vec![home.join(".bashrc"), home.join(".bash_profile")],
    Kind::Zsh => vec![home.join(".zshrc"), home.join(".zprofile")],
    Kind::Fish => vec![home.join(".config/fish/config.fish")],
    Kind::PowerShell => {
      vec![home.join("Documents/PowerShell/Microsoft.PowerShell_profile.ps1")]
    }
    Kind::Nushell => vec![home.join(".config/nushell/config.nu")],
    _ => vec![]
  }
}

pub fn get_version(kind: &Kind) -> Option<String> {
  let (cmd, args) = match kind {
    Kind::Bash => ("bash", vec!["--version"]),
    Kind::Zsh => ("zsh", vec!["--version"]),
    Kind::Fish => ("fish", vec!["--version"]),
    Kind::PowerShell => ("pwsh", vec!["-Version"]),
    Kind::Nushell => ("nu", vec!["--version"]),
    Kind::CommandPrompt => return None,
    Kind::Unsupported => return None
  };

  Command::new(cmd)
    .args(&args)
    .output()
    .ok()
    .and_then(|output| String::from_utf8(output.stdout).ok())
    .map(|version| version.lines().next().unwrap_or("").trim().to_string())
}
