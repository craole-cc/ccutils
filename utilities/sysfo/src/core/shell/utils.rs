use std::{env::var, path::Path};

#[derive(Debug)]
pub enum Shell {
    Bash,
    GitBash,
    Zsh,
    Fish,
    Cmd,
    Pwsh,
    Nushell,
    Unknown,
}

impl Default for Shell {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Shell {
    pub fn current() -> Self {
        Self::from_combined_checks()
            .or_else(|| Self::from_shell_env())
            .unwrap_or(Shell::Unknown)
    }

    pub fn from_combined_checks() -> Option<Self> {
        let is_vscode = var("TERM_PROGRAM").map(|tp| tp == "vscode").unwrap_or(false);

        if is_vscode {
            return Self::detect_vscode_shell();
        }

        // Windows Terminal detection remains unchanged
        if let Ok(msystem) = var("MSYSTEM") {
            if msystem.contains("MINGW") {
                if let Ok(shell) = var("SHELL") {
                    if shell.contains("bash") || shell.contains("git") {
                        return Some(Shell::GitBash);
                    }
                }
            }
        }

        if let Ok(ps_path) = var("PSModulePath") {
            if ps_path.contains("Users") && ps_path.contains("WindowsPowerShell") {
                return Some(Shell::Pwsh);
            }
        }

        if let Ok(comspec) = var("COMSPEC") {
            if comspec.to_lowercase().contains("cmd.exe")
                && var("SHELL").is_err()
                && var("MSYSTEM").is_err() {
                return Some(Shell::Cmd);
            }
        }

        None
    }

    fn detect_vscode_shell() -> Option<Self> {
        // Check PSModulePath patterns which are unique for each shell
        if let Ok(ps_path) = var("PSModulePath") {
            // PowerShell has PowerShell/Modules in its path
            if ps_path.contains("PowerShell\\Modules") && ps_path.contains("Users") {
                return Some(Shell::Pwsh);
            }

            // CMD and Git Bash have simpler PSModulePath without user modules
            let is_simple_path = ps_path == "C:\\Program Files\\WindowsPowerShell\\Modules;C:\\windows\\system32\\WindowsPowerShell\\v1.0\\Modules";

            if is_simple_path {
                // If we have PS1 and MSYSTEM, it's Git Bash
                if var("PS1").is_ok() && var("MSYSTEM").is_ok() {
                    return Some(Shell::GitBash);
                }
                // Otherwise it's CMD
                return Some(Shell::Cmd);
            }
        }

        None
    }

    pub fn from_shell_env() -> Option<Self> {
        let is_vscode = var("TERM_PROGRAM").map(|tp| tp == "vscode").unwrap_or(false);

        if is_vscode {
            return None;
        }

        let shell_env = var("SHELL").ok()?;
        let is_mingw = var("MSYSTEM").map(|ms| ms.contains("MINGW")).unwrap_or(false);

        let shell_name = Path::new(&shell_env)
            .file_name()?
            .to_str()?
            .to_lowercase();

        match shell_name.as_str() {
            name if name.contains("powershell") || name.contains("pwsh") => Some(Shell::Pwsh),
            name if (name.contains("bash") || name.contains("git")) && is_mingw => Some(Shell::GitBash),
            "cmd" | "cmd.exe" => Some(Shell::Cmd),
            "fish" => Some(Shell::Fish),
            "zsh" => Some(Shell::Zsh),
            "bash" => Some(Shell::Bash),
            "nu" => Some(Shell::Nushell),
            _ => None,
        }
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
