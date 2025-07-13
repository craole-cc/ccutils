use crate::core::process::Error;

use super::shell;
use std::{
  collections::{BTreeMap, HashMap},
  path::PathBuf,
  process::id
};
use sysinfo::{Pid, System, Users};

#[derive(Debug)]
pub struct Info {
  pub id: u32,
  pub name: String,
  pub path: PathBuf,
  pub user: String,
  pub time_started: u64,
  pub time_running: u64,
  pub cwd: PathBuf,
  pub dependencies: BTreeMap<u32, String>,
  pub env: HashMap<String, String>,
  pub shell: shell::Info
}

impl Default for Info {
  fn default() -> Self {
    let mut system = System::new_all();
    system.refresh_all();
    Self::new(&system).unwrap_or_else(|_| Self {
      id: 0,
      name: String::new(),
      path: PathBuf::new(),
      user: String::new(),
      time_started: 0,
      time_running: 0,
      cwd: PathBuf::new(),
      dependencies: BTreeMap::new(),
      env: HashMap::new(),
      shell: shell::Info::default()
    })
  }
}

impl Info {
  pub fn new(system: &System) -> Result<Self, Error> {
    let id = id();

    let process = match system.process(Pid::from_u32(id)) {
      Some(process) => process,
      None => return Err(Error::ProcessNotFound)
    };
    let name = process.name().to_string_lossy().to_string();
    let path = match process.exe() {
      Some(path) => path.to_path_buf(),
      None => return Err(Error::ProcessNotFound)
    };
    let users = Users::new_with_refreshed_list();
    let user_id = match process.user_id() {
      Some(user_id) => user_id,
      None => return Err(Error::ProcessNotFound)
    };
    let user = match users.get_user_by_id(user_id) {
      Some(user) => user.name().to_string(),
      None => return Err(Error::ProcessNotFound)
    };
    let time_started = process.start_time();
    let time_running = process.run_time();
    let cwd = match process.cwd() {
      Some(cwd) => cwd.to_path_buf(),
      None => return Err(Error::ProcessNotFound)
    };

    // Rest of the implementation remains the same
    let env: HashMap<String, String> = process
      .environ()
      .iter()
      .filter_map(|os_str| {
        let var = os_str.to_string_lossy();
        let parts: Vec<&str> = var.splitn(2, '=').collect();
        if parts.len() == 2 {
          Some((parts[0].to_string(), parts[1].to_string()))
        } else {
          None
        }
      })
      .collect();

    let mut dependencies: BTreeMap<u32, String> = BTreeMap::new();
    let mut pid = process.pid();

    while let Some(current_process) = system.process(pid) {
      if let Some(parent_pid) = current_process.parent() {
        if let Some(parent_process) = system.process(parent_pid) {
          dependencies.insert(parent_pid.as_u32(), parent_process.name().to_string_lossy().to_string());
        }
        pid = parent_pid;

        if dependencies.len() > 10 {
          break;
        }
      } else {
        break;
      }
    }

    let shell = dependencies
      .iter()
      .rev()
      .find_map(|(pid, name)| {
        let name = name.to_lowercase();
        let kind = match name.as_str() {
          n if n.contains("pwsh") || n.contains("powershell") => shell::Kind::PowerShell,
          n if n.contains("bash") => shell::Kind::Bash,
          n if n.contains("zsh") => shell::Kind::Zsh,
          n if n.contains("fish") => shell::Kind::Fish,
          n if n.contains("nu") => shell::Kind::Nushell,
          n if n.contains("cmd.exe") => shell::Kind::CommandPrompt,
          _ => return None
        };

        system.process(Pid::from_u32(*pid)).map(|process| shell::Info {
          id: *pid,
          name: process.name().to_string_lossy().to_string(),
          path: process.exe().map_or_else(PathBuf::new, |p| p.to_path_buf()),
          conf: shell::get_config_paths(&kind),
          version: shell::get_version(&kind)
        })
      })
      .unwrap_or_default();

    Ok(Self {
      id,
      name,
      path,
      user,
      time_started,
      time_running,
      cwd,
      dependencies,
      env,
      shell
    })
  }

  pub fn fetch(&self) -> String {
    let mut output = String::from("Process Information\n");
    output.push_str(&"=".repeat(80));
    output.push('\n');

    // Basic process info
    output.push_str(&format!("ID          : {}\n", self.id));
    output.push_str(&format!("Name        : {}\n", self.name));
    output.push_str(&format!("User        : {}\n", self.user));
    output.push_str(&format!("Path        : {}\n", self.path.display()));
    output.push_str(&format!("Working Dir : {}\n", self.cwd.display()));

    // Time information
    output.push_str("\nTime Information\n");
    output.push_str(&"-".repeat(40));
    output.push('\n');
    output.push_str(&format!("Started     : {}\n", self.time_started));
    output.push_str(&format!("Running     : {} seconds\n", self.time_running));

    // Shell information
    output.push_str("\nShell Information\n");
    output.push_str(&"-".repeat(40));
    output.push('\n');
    output.push_str(&format!("ID          : {}\n", self.shell.id));
    output.push_str(&format!("Name        : {}\n", self.shell.name));
    output.push_str(&format!("Path        : {}\n", self.shell.path.display()));
    output.push_str(&format!(
      "Version     : {}\n",
      self.shell.version.as_deref().unwrap_or("Unknown")
    ));

    // Shell configurations
    output.push_str("\nShell Configurations:\n");
    for path in &self.shell.conf {
      output.push_str(&format!("  - {}\n", path.display()));
    }

    // Optional: Add dependencies section if needed
    if !self.dependencies.is_empty() {
      output.push_str("\nProcess Dependencies\n");
      output.push_str(&"-".repeat(40));
      output.push('\n');
      for (pid, name) in &self.dependencies {
        output.push_str(&format!("  {pid} : {name}\n"));
      }
    }

    output
  }
}
