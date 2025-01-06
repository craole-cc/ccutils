use super::Error;
use sysinfo::{Pid, System};

/// Retrieves the name of a process given its PID.
///
/// # Arguments
/// - `pid`: The PID of the process whose name is to be retrieved.
///
/// # Returns
/// - `Ok(String)`: The name of the process.
/// - `Err(Error::ProcessNotFound)`: If the process could not be found.
pub fn get_process_name(system: &System, pid: u32) -> Result<String, Error> {
    system
        .process(Pid::from_u32(pid))
        .map(|process| process.name().to_string_lossy().to_string())
        .ok_or(Error::ProcessNotFound)
}

/// Retrieves the parent PID of a given process.
///
/// # Arguments
/// - `pid`: The PID of the process whose parent PID is to be retrieved.
///
/// # Returns
/// - `Ok(u32)`: The parent PID.
/// - `Err(Error::ParentNotFound)`: If the parent process could not be found.
pub fn get_parent_id(system: &System, pid: u32) -> Result<u32, Error> {
    system
        .process(Pid::from_u32(pid))
        .and_then(|process| process.parent())
        .map(|pid| pid.as_u32())
        .ok_or(Error::ParentNotFound)
}

pub fn get_shell_config_paths(kind: &Kind) -> Vec<PathBuf> {
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

pub fn get_shell_version(kind: &Kind) -> Option<String> {
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
