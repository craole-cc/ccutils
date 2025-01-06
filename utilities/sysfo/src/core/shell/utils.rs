use super::Kind;
use directories::BaseDirs;
use std::{path::PathBuf, process::Command};

pub fn get_config_paths(kind: &Kind) -> Vec<PathBuf> {
    let dirs = match BaseDirs::new() {
        Some(dirs) => dirs,
        None => return vec![], // Return an empty vector if base directories cannot be determined.
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
        _ => vec![],
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
        Kind::Unsupported => return None,
    };

    Command::new(cmd)
        .args(&args)
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|version| version.lines().next().unwrap_or("").trim().to_string())
}
