use crate::core::process::Error;
use std::{
    fmt::{self, Debug, Formatter},
    path::PathBuf,
};
use sysinfo::{Pid, System};

#[derive(Clone)]
pub struct Process {
    pub id: u32,
    pub name: String,
    pub path: PathBuf,
    pub cmd: Vec<String>,
    pub cwd: PathBuf,
}

impl Debug for Process {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({}) [path: {}, cwd: {}]",
            self.name,
            self.id,
            self.path.display(),
            self.cwd.display()
        )
    }
}

impl Default for Process {
    fn default() -> Self {
        let system=
        Self {
            id: 0,
            name: String::new(),
            path: PathBuf::new(),
            cmd: Vec::new(),
            cwd: PathBuf::new(),
        }
    }
}

// impl Process {
//     pub fn from_sysinfo(process: &System) -> Self {
//         Self {
//             id: process.pid().as_u32(),
//             name: process.name().to_lossy.to_lowercase(),
//             path: process.exe().to_path_buf(),
//             cmd: process.cmd().to_vec(),
//             cwd: process.cwd().to_path_buf(),
//         }
//     }
// }

/// Retrieves the name of a process given its PID.
///
/// # Arguments
/// - `pid`: The PID of the process whose name is to be retrieved.
///
/// # Returns
/// - `Ok(String)`: The name of the process.
/// - `Err(Error::ProcessNotFound)`: If the process could not be found.
pub fn get_process_name(system: &System, pid: u32) -> Result<String, Error> {
    system
        .process(Pid::from_u32(pid))
        .map(|process| process.name().to_string_lossy().to_string())
        .ok_or(Error::ProcessNotFound)
}

/// Retrieves the parent PID of a given process.
///
/// # Arguments
/// - `pid`: The PID of the process whose parent PID is to be retrieved.
///
/// # Returns
/// - `Ok(u32)`: The parent PID.
/// - `Err(Error::ParentNotFound)`: If the parent process could not be found.
pub fn get_parent_id(system: &System, pid: u32) -> Result<u32, Error> {
    system
        .process(Pid::from_u32(pid))
        .and_then(|process| process.parent())
        .map(|pid| pid.as_u32())
        .ok_or(Error::ParentNotFound)
}
