use std::{env::var, os::windows, path::Path};

#[derive(Debug)]
enum Shell {
    Bash,
    GitBash,
    Zsh,
    Fish,
    Cmd,
    PowerShell,
    Nushell,
    Unknown,
}

impl Default for Shell {
    fn default() -> Self {
        // TODO: Implement default shell detection logic to use the active shell on the system, fall back to unknown
        Self::Unknown
    }
}

impl Shell {
    fn current() -> Self {
        // Try multiple detection methods in order of reliability
        Self::from_combined_checks()
            .or_else(|| Self::from_parent_process())
            .or_else(|| Self::from_shell_env())
            .unwrap_or(Shell::Unknown)
    }

    fn from_combined_checks() -> Option<Self> {
        // Check for MINGW/Git Bash first
        let is_mingw = var("MSYSTEM")
            .map(|ms| ms.contains("MINGW"))
            .unwrap_or(false);
        let term_program = var("TERM_PROGRAM").unwrap_or_default();

        if is_mingw || term_program == "mintty" {
            return Some(Shell::GitBash);
        }

        // PowerShell detection (needs multiple signals)
        if var("PSModulePath").is_ok() {
            let ps_edition = var("PSEdition").is_ok();
            let ps_version = var("PSVersionTable").is_ok();

            if ps_edition || ps_version {
                return Some(Shell::PowerShell);
            }
        }

        // CMD detection (needs multiple signals)
        if let Ok(comspec) = var("COMSPEC") {
            if comspec.to_lowercase().contains("cmd.exe") {
                // Additional CMD-specific checks
                let is_cmd = var("PROMPT").is_ok() &&
                          // Check if we're not in MINGW
                          !is_mingw &&
                          // Check common CMD environment variables
                          var("CMDEXTVERSION").is_ok() &&
                          // Make sure we're not in PowerShell
                          var("PSModulePath").is_err();

                if is_cmd {
                    return Some(Shell::Cmd);
                }
            }
        }

        // Fish detection
        if var("FISH_VERSION").is_ok() {
            return Some(Shell::Fish);
        }

        // Zsh detection
        if var("ZSH_VERSION").is_ok() {
            return Some(Shell::Zsh);
        }

        // Bash detection (when not in Git Bash)
        if var("BASH_VERSION").is_ok() && !is_mingw {
            return Some(Shell::Bash);
        }

        // Nushell detection
        if var("NU_VERSION").is_ok() {
            return Some(Shell::Nushell);
        }

        None
    }

    fn from_parent_process() -> Option<Self> {
        let ppid = get_parent_pid();
        let process_name = get_process_name(ppid).to_lowercase();
        let is_mingw = var("MSYSTEM")
            .map(|ms| ms.contains("MINGW"))
            .unwrap_or(false);

        match process_name.as_str().trim() {
            name if name.contains("mintty") && is_mingw => Some(Shell::GitBash),
            name if name.contains("cmd.exe") => Some(Shell::Cmd),
            name if name.contains("powershell") || name.contains("pwsh") => Some(Shell::PowerShell),
            name if name.contains("fish") => Some(Shell::Fish),
            name if name.contains("zsh") => Some(Shell::Zsh),
            name if name.contains("bash") && !is_mingw => Some(Shell::Bash),
            name if name.contains("nu") => Some(Shell::Nushell),
            _ => None,
        }
    }

    fn from_shell_env() -> Option<Self> {
        let is_mingw = var("MSYSTEM")
            .map(|ms| ms.contains("MINGW"))
            .unwrap_or(false);

        if is_mingw {
            return Some(Shell::GitBash);
        }

        var("SHELL")
            .ok()
            .and_then(|shell_path| {
                let path = Path::new(&shell_path);
                path.file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| name.to_lowercase())
            })
            .and_then(|shell_name| match shell_name.as_str() {
                name if name.contains("mintty") && is_mingw => Some(Shell::GitBash),
                "cmd" | "cmd.exe" => Some(Shell::Cmd),
                "powershell" | "pwsh" => Some(Shell::PowerShell),
                "fish" => Some(Shell::Fish),
                "zsh" => Some(Shell::Zsh),
                "bash" => {
                    if is_mingw {
                        Some(Shell::GitBash)
                    } else {
                        Some(Shell::Bash)
                    }
                }
                "nu" => Some(Shell::Nushell),
                _ => None,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_detection() {
        let shell = Shell::current();
        println!("Detected shell: {:?}", shell);
    }
}
