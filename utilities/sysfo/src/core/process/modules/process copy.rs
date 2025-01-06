use super::shell;
use std::{
    collections::{BTreeMap, HashMap},
    path::PathBuf,
    process::id,
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
    pub shell: shell::Info,
}

impl Default for Info {
    fn default() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self::new(&system)
    }
}

impl Info {
    pub fn new(system: &System) -> Self {
        let id = id();

        let process = match system.process(Pid::from_u32(id)) {
            Some(process) => process,
            None => return Self::default(), // TODO: This should return an error: "Could not get process with id: {id}"
        };
        let name = process.name().to_string_lossy().to_string();
        let path = match process.exe() {
            Some(path) => path.to_path_buf(),
            None => return Self::default(), // TODO: This should return an error: "Could not get process path"
        };
        let users = Users::new_with_refreshed_list();
        let user_id = match process.user_id() {
            Some(user_id) => user_id,
            None => return Self::default(), // TODO: This should return an error: "Could not get user id"
        };
        let user = match users.get_user_by_id(user_id) {
            Some(user) => user.name().to_string(),
            None => return Self::default(), // TODO: This should return an error: "Could not get user name"
        };
        let time_started = process.start_time();
        let time_running = process.run_time();
        let cwd = match process.cwd() {
            Some(cwd) => cwd.to_path_buf(),
            None => return Self::default(), // TODO: This should return an error: "Could not get process cwd"
        };

        // Convert environment variables from OsString
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

        // Collect all parent PIDs and their names in sorted order
        let mut dependencies: BTreeMap<u32, String> = BTreeMap::new();
        let mut pid = process.pid();

        while let Some(current_process) = system.process(pid) {
            if let Some(parent_pid) = current_process.parent() {
                // Get the parent process name
                if let Some(parent_process) = system.process(parent_pid) {
                    dependencies.insert(
                        parent_pid.as_u32(),
                        parent_process.name().to_string_lossy().to_string(),
                    );
                }
                pid = parent_pid;

                // Safety check to prevent infinite loops
                if dependencies.len() > 10 {
                    break;
                }
            } else {
                break;
            }
        }

        // Detect shell by traversing parent processes
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
                    _ => return None,
                };

                // Try to get the process info
                system
                    .process(Pid::from_u32(*pid))
                    .map(|process| shell::Info {
                        id: *pid,
                        name: process.name().to_string_lossy().to_string(),
                        path: process.exe().map_or_else(PathBuf::new, |p| p.to_path_buf()),
                        conf: shell::get_config_paths(&kind),
                        version: shell::get_version(&kind),
                    })
            })
            .unwrap_or_default();

        Self {
            id,
            name,
            path,
            user,
            time_started,
            time_running,
            cwd,
            dependencies,
            env,
            shell,
        }
    }

    pub fn fetch(&self) -> String {
        format!(
            "Time {{\n\
            {:>16}: {}\n\
            {:>16}: {}\n\
            {:>16}: {}\n\
            {:>16}: {}\n\
            {:>16}: {}\n\
            {:>16}: {}\n\
            {:>16}: {}\n\
            {:>16}: {:#?}\n\
            }}",
            "Id",
            self.id,
            "Name",
            self.name,
            "Path",
            self.path.display(),
            "CWD",
            self.cwd.display(),
            "User",
            self.user,
            "Time Started",
            self.time_started,
            "Time Running",
            self.time_running,
            "Shell",
            self.shell,
            // "Dependencies",
            // self.dependencies,
            // "Env",
            // self.env,
        )
    }
}
