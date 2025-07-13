use std::{
  path::PathBuf,
  process::{Command, id}
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Info {
  pub name: Name,
  pub config: Vec<PathBuf>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Name {
  Bash,
  Zsh,
  Fish,
  CommandPrompt,
  Powershell,
  Nushell,
  Unsupported
}

impl Name {
  pub fn current(system: &System) -> (Info, Process) {
    let current_pid = Pid::from_u32(id());
    let home = dirs::home_dir().unwrap_or_default();

    // Define shell configurations with their detection logic and config paths
    let shells = [
      (
        Name::Bash,
        &["bash", "bash.exe"][..],
        Some("$BASH_VERSION"),
        vec![home.join(".bashrc"), home.join(".bash_profile"), home.join(".profile")]
      ),
      (
        Name::Zsh,
        &["zsh", "zsh.exe"][..],
        Some("$ZSH_VERSION"),
        vec![home.join(".zshrc"), home.join(".zprofile")]
      ),
      (
        Name::Fish,
        &["fish", "fish.exe"][..],
        None,
        vec![home.join(".config/fish/config.fish")]
      ),
      (
        Name::Powershell,
        &["pwsh", "pwsh.exe", "powershell", "powershell.exe"][..],
        None,
        vec![home.join("Documents/PowerShell/Microsoft.PowerShell_profile.ps1")]
      ),
      (
        Name::Nushell,
        &["nu", "nu.exe"][..],
        None,
        vec![home.join(".config/nushell/config.nu")]
      ),
      (Name::CommandPrompt, &["cmd", "cmd.exe"][..], None, vec![])
    ];

    if let Some((shell_type, process, config)) = Self::detect_shell(system, current_pid, &shells) {
      (ShellInfo { shell_type, config }, process)
    } else {
      (
        ShellInfo {
          shell_type: Name::Unsupported,
          config: vec![]
        },
        Process {
          id: 0,
          name: String::from("unknown"),
          path: PathBuf::new(),
          cmd: vec![],
          cwd: PathBuf::new()
        }
      )
    }
  }

  fn detect_shell(
    system: &System,
    pid: Pid,
    shell_configs: &[(Shell, &[&str], Option<&str>, Vec<PathBuf>)]
  ) -> Option<(Shell, Process, Vec<PathBuf>)> {
    for (shell_type, names, version_check, config) in shell_configs {
      if let Some(process) = find_shell_process(system, pid, names) {
        if let Some(version_arg) = version_check {
          if !check_shell_version(version_arg) {
            continue;
          }
        }
        return Some((*shell_type, process, config.clone()));
      }
    }
    None
  }
}

fn find_shell_process(system: &System, pid: Pid, shell_names: &[&str]) -> Option<Process> {
  let mut current_pid = pid;
  while let Some(process) = system.process(current_pid) {
    let process_name = process.name().to_lowercase();
    if shell_names.iter().any(|&name| process_name == name) {
      return Some(Process::from_sysinfo(process));
    }
    current_pid = process.parent()?;
  }
  None
}

fn check_shell_version(arg: &str) -> bool {
  Command::new("echo")
    .arg(arg)
    .output()
    .map(|output| output.status.success())
    .unwrap_or(false)
}
