use crate::core::process::Error;
use once_cell::sync::Lazy;
use std::{collections::HashMap, path::PathBuf, process::id, sync::Mutex};
use sysinfo::{Pid, System, Uid, Users};

pub static INFO: Lazy<Mutex<System>> = Lazy::new(|| {
    let mut system = System::new_all();
    system.refresh_all();
    Mutex::new(system)
});

#[derive(Debug)]
pub struct Info {
    pub id: u32,
    pub name: String,
    pub path: PathBuf,
    pub user: String,
    pub time_started: u64,
    pub time_running: u64,
    pub cwd: PathBuf,
    // pub shell: ShellConfig,
    // pub env: HashMap<String>,
    // pub usage_cpu: f32,
    // pub usage_mem: u64,
}

impl Default for Info {
    fn default() -> Self {
        let system = INFO.lock().unwrap();

        let id = id();
        let mut process = match system.process(Pid::from_u32(id)) {
            Some(process) => process,
            None => return Self::default(), // TODO: This should return an error: "Could not get process with id: {id}"
        };
        let pid = process.pid();
        let name = process.name().to_string_lossy().to_string();
        let path = match process.exe() {
            Some(path) => path.to_path_buf(),
            None => return Self::default(), // TODO: This should return an error: "Could not get process path"
        };
        let users = Users::new_with_refreshed_list();
        let user_id = match process.user_id() {
            Some(user_id) => (user_id),
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
        // let env: HashMap<String, String> = process.environ().iter()
        //     .map(|(k, v)| format!("{}={}", k.to_string_lossy(), v.to_string_lossy()))
        //     .collect();

        // let usage ={
        //     system.refresh_all();
        //     system.refresh_all();

        // \]

        // }

        Self {
            id,
            name,
            path,
            user,
            time_started,
            time_running,
            cwd,
            // env,
            // usage_cpu,
            // usage_mem,
        }
    }
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
            {:>16}: {:?}\n\
            {:>16}: {:?}\n\
            {:>16}: {:?}\n\
            {:>16}: {:?}\n\
            {:>16}: {:?}\n\
            }}",
            "Id",
            self.id,
            "Name",
            self.name,
            "Path",
            self.path,
            "User",
            self.user,
            "Time Started",
            self.time_started,
            "Time Running",
            self.time_running,
            "CWD",
            self.cwd,
            //     "CPU usage",
            //     self.usage_cpu,
            //     "Memory usage",
            //     self.usage_mem
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_info_default() -> Result<(), Box<dyn std::error::Error>> {
        let info = Info::new();

        logline::trace!("{:#?}", &info);
        assert_eq!(info.id, 0);
        assert_eq!(info.name, String::new());
        Ok(())
    }
}
