use super::{Process, Shell, ShellConfig};
use once_cell::sync::Lazy;
use std::{process::id, sync::Mutex};
use sysinfo::{Pid, System};

pub static INFO: Lazy<Mutex<System>> = Lazy::new(|| {
    let mut system = System::new_all();
    system.refresh_all();
    Mutex::new(system)
});

#[derive(Debug)]
pub struct Info {
    pub current: Process,
    pub parent: Process,
    pub shell_info: (ShellConfig, Process),
}

impl Info {
    pub fn new(system: &System) -> Result<Self, Error> {
        let current_pid = Pid::from_u32(id());
        let current_process = system
            .process(current_pid)
            .map(Process::from_sysinfo)
            .ok_or_else(|| Error("Failed to get current process".to_string()))?;

        let parent_process = system
            .process(current_pid)
            .and_then(|p| p.parent())
            .and_then(|pid| system.process(pid))
            .map(Process::from_sysinfo)
            .ok_or_else(|| Error("Failed to get parent process".to_string()))?;

        let shell_info = Shell::current(system);

        Ok(Self {
            current: current_process,
            parent: parent_process,
            shell_info,
        })
    }

    pub fn fetch(&self) -> String {
        format!(
            "{:?} -> {:?} -> {:?} ({:?}) [configs: {:?}]",
            self.current,
            self.parent,
            self.shell_info.1,
            self.shell_info.0.shell_type,
            self.shell_info.0.config_paths
        )
    }
}
