use super::Error;
use sysinfo::{Pid, System};

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
