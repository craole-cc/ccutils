use std::process::{id, Command};
use sysinfo::{Pid, System};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    CommandPrompt,
    Powershell,
    Nushell,
    Unsupported,
}

impl Shell {
    pub fn current() -> Self {
        let system = System::new_all();
        let current_pid = Pid::from_u32(id());

        // Define shell configurations with their detection logic
        let shells = [
            (
                Shell::Bash,
                &["bash", "bash.exe"][..],
                Some("$BASH_VERSION"),
            ),
            (Shell::Zsh, &["zsh", "zsh.exe"][..], Some("$ZSH_VERSION")),
            (Shell::Fish, &["fish", "fish.exe"][..], None),
            (
                Shell::Powershell,
                &["pwsh", "pwsh.exe", "powershell", "powershell.exe"][..],
                None,
            ),
            (Shell::Nushell, &["nu", "nu.exe"][..], None),
            (Shell::CommandPrompt, &["cmd", "cmd.exe"][..], None),
        ];

        for (shell, names, version_check) in shells {
            if is_shell_parent(&system, current_pid, names) {
                if let Some(version_arg) = version_check {
                    if !check_shell_version(version_arg) {
                        continue;
                    }
                }
                return shell;
            }
        }

        Shell::Unsupported
    }
}

fn is_shell_parent(system: &System, pid: Pid, shell_names: &[&str]) -> bool {
    let mut current_pid = pid;
    while let Some(process) = system.process(current_pid) {
        let process_name = process.name().to_string_lossy().to_lowercase();
        if shell_names.iter().any(|&name| process_name == name) {
            return true;
        }
        current_pid = match process.parent() {
            Some(parent_pid) => parent_pid,
            None => break,
        };
    }
    false
}

fn check_shell_version(arg: &str) -> bool {
    Command::new("echo")
        .arg(arg)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
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
