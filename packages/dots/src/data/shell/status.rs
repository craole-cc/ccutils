use std::env;
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
enum Status {
  Active,
  Inactive,
  Unavailable,
  Available
}

#[derive(Debug)]
enum Shell {
  Bash,
  Zsh,
  Fish,
  PowerShell,
  Cmd,
  Unknown
}

impl Default for Status {
  fn default() -> Self {
    let current_shell = detect_current_shell();

    match current_shell {
      Shell::Unknown => Status::Unavailable,
      shell =>
        if is_shell_available(&shell) {
          if is_shell_active(&shell) {
            Status::Active
          } else {
            Status::Available
          }
        } else {
          Status::Unavailable
        },
    }
  }
}

fn detect_current_shell() -> Shell {
  // Check various environment variables and process information

  // 1. Check PowerShell specific variables
  if env::var("PSModulePath").is_ok() {
    return Shell::PowerShell;
  }

  // 2. Check fish specific variables
  if env::var("FISH_VERSION").is_ok() {
    return Shell::Fish;
  }

  // 3. Check for bash/zsh specific variables
  if let Ok(version) = env::var("BASH_VERSION") {
    return Shell::Bash;
  }

  if let Ok(version) = env::var("ZSH_VERSION") {
    return Shell::Zsh;
  }

  // 4. On Windows, check if we're in CMD
  #[cfg(target_family = "windows")]
  {
    if env::var("ComSpec").is_ok() && env::var("PSModulePath").is_err() {
      return Shell::Cmd;
    }
  }

  // 5. Check process name as fallback
  let process_name = get_process_name(get_parent_pid()).to_lowercase();

  match process_name.as_str() {
    name if name.contains("bash") => Shell::Bash,
    name if name.contains("zsh") => Shell::Zsh,
    name if name.contains("fish") => Shell::Fish,
    name if name.contains("powershell") || name.contains("pwsh") => Shell::PowerShell,
    name if name.contains("cmd.exe") => Shell::Cmd,
    _ => Shell::Unknown
  }
}

fn is_shell_available(shell: &Shell) -> bool {
  match shell {
    Shell::Unknown => false,
    _ => {
      let command = match shell {
        Shell::PowerShell => "powershell",
        Shell::Bash => "bash",
        Shell::Zsh => "zsh",
        Shell::Fish => "fish",
        Shell::Cmd => "cmd",
        Shell::Unknown => return false
      };

      #[cfg(target_family = "unix")]
      {
        Command::new("which")
          .arg(command)
          .output()
          .map(|output| output.status.success())
          .unwrap_or(false)
      }

      #[cfg(target_family = "windows")]
      {
        Command::new("where")
          .arg(command)
          .output()
          .map(|output| output.status.success())
          .unwrap_or(false)
      }
    }
  }
}

fn is_shell_active(shell: &Shell) -> bool {
  let ppid = get_parent_pid();
  let parent_process = get_process_name(ppid).to_lowercase();

  match shell {
    Shell::PowerShell => parent_process.contains("powershell") || parent_process.contains("pwsh"),
    Shell::Bash => parent_process.contains("bash"),
    Shell::Zsh => parent_process.contains("zsh"),
    Shell::Fish => parent_process.contains("fish"),
    Shell::Cmd => parent_process.contains("cmd.exe"),
    Shell::Unknown => false
  }
}

fn get_parent_pid() -> u32 {
  #[cfg(target_family = "unix")]
  {
    unsafe { libc::getppid() as u32 }
  }

  #[cfg(target_family = "windows")]
  {
    use windows::Win32::System::ProcessStatus::{GetCurrentProcessId, GetParentPid};
    unsafe {
      let current_pid = GetCurrentProcessId();
      GetParentPid(current_pid)
    }
  }
}

fn get_process_name(pid: u32) -> String {
  #[cfg(target_family = "unix")]
  {
    let output = Command::new("ps")
      .args(["-p", &pid.to_string(), "-o", "comm="])
      .output()
      .unwrap_or_default();
    String::from_utf8(output.stdout).unwrap_or_default()
  }

  #[cfg(target_family = "windows")]
  {
    let output = Command::new("tasklist")
      .args(["/FI", &format!("PID eq {}", pid), "/NH", "/FO", "CSV"])
      .output()
      .unwrap_or_default();
    String::from_utf8(output.stdout)
      .unwrap_or_default()
      .lines()
      .next()
      .unwrap_or("")
      .split(',')
      .next()
      .unwrap_or("")
      .trim_matches('"')
      .to_string()
  }
}
