use crate::{core::process::Error, SYSTEM_INFO};
use std::{path::PathBuf, process::id};
use sysinfo::{Pid, System, Uid, Users};

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
}

impl Default for Info {
    fn default() -> Self {
        let system = SYSTEM_INFO.lock().unwrap();

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
        }
    }
}

impl Info {
    pub fn new(system: &System) -> Self {
        // let system = SYSTEM_INFO.lock().unwrap();

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
