use std::{error, fmt, process};
use sysinfo::{Pid, System};

/// Custom error type for process-related operations.
#[derive(Debug)]
pub enum Error {
  /// Error indicating the current or specified process could not be found.
  ProcessNotFound,
  /// Error indicating the parent process could not be found.
  ParentNotFound
}

impl error::Error for Error {}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ProcessNotFound => write!(f, "Process not found"),
      Self::ParentNotFound => write!(f, "Parent process not found")
    }
  }
}

/// Struct to encapsulate system process operations.
#[derive(Debug)]
pub struct Process {
  /// The underlying `sysinfo::System` instance used to query process
  /// information.
  sys: System
}

impl Default for Process {
  /// Creates a new `Process` instance and refreshes system information.
  fn default() -> Self {
    let mut sys = System::new_all();
    sys.refresh_all();
    Self { sys }
  }
}

impl Process {
  /// Creates a new `Process` instance.
  ///
  /// This is a convenience method that uses the default implementation.
  pub fn new() -> Self {
    Self::default()
  }

  /// Refreshes the system's process information.
  ///
  /// Call this method to ensure the process information is up-to-date
  /// before performing any queries.
  pub fn refresh(&mut self) {
    self.sys.refresh_all();
  }

  /// Retrieves the current process's PID and name.
  ///
  /// # Returns
  /// - `Ok((u32, String))`: A tuple containing the current process's PID and name.
  /// - `Err(Error::ProcessNotFound)`: If the current process could not be found.
  pub fn get_current_proc(&mut self) -> Result<(u32, String), Error> {
    let id = process::id();
    let name = self.get_process_name(id)?;
    Ok((id, name))
  }

  /// Retrieves the parent process's PID and name.
  ///
  /// # Returns
  /// - `Ok((u32, String))`: A tuple containing the parent process's PID and name.
  /// - `Err(Error::ParentNotFound)`: If the parent process could not be found.
  pub fn get_parent_proc(&mut self) -> Result<(u32, String), Error> {
    let current_pid = self.get_current_proc()?.0;
    let parent_pid = self.get_parent_pid(current_pid)?;
    let parent_name = self.get_process_name(parent_pid)?;
    Ok((parent_pid, parent_name))
  }

  /// Retrieves the parent PID of a given process.
  ///
  /// # Arguments
  /// - `pid`: The PID of the process whose parent PID is to be retrieved.
  ///
  /// # Returns
  /// - `Ok(u32)`: The parent PID.
  /// - `Err(Error::ParentNotFound)`: If the parent process could not be found.
  pub fn get_parent_pid(&mut self, pid: u32) -> Result<u32, Error> {
    self
      .sys
      .process(Pid::from_u32(pid))
      .and_then(|process| process.parent())
      .map(|pid| pid.as_u32())
      .ok_or(Error::ParentNotFound)
  }

  /// Retrieves the name of a process given its PID.
  ///
  /// # Arguments
  /// - `pid`: The PID of the process whose name is to be retrieved.
  ///
  /// # Returns
  /// - `Ok(String)`: The name of the process.
  /// - `Err(Error::ProcessNotFound)`: If the process could not be found.
  pub fn get_process_name(&mut self, pid: u32) -> Result<String, Error> {
    self
      .sys
      .process(Pid::from_u32(pid))
      .map(|process| process.name().to_string_lossy().to_string())
      .ok_or(Error::ProcessNotFound)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  /// Test the retrieval of the current process's PID and name.
  #[test]
  fn test_get_current_proc() {
    let result = Process::new().get_current_proc();
    println!("Current Process: {result:#?}");
    assert!(result.is_ok(), "Failed to get current process");
  }

  /// Test the retrieval of the parent process's PID and name.
  #[test]
  fn test_get_parent_proc() {
    let result = Process::new().get_parent_proc();
    println!("Parent Process: {result:#?}");
    assert!(result.is_ok(), "Failed to get parent process");
  }

  /// Test the retrieval of a process's parent PID.
  #[test]
  fn test_get_parent_pid() {
    let mut proc = Process::new();
    let current_pid = proc.get_current_proc().unwrap().0;
    let result = proc.get_parent_pid(current_pid);
    println!("Parent PID: {result:#?}");
    assert!(result.is_ok(), "Failed to get parent PID");
  }

  /// Test the retrieval of a process's name by PID.
  #[test]
  fn test_get_process_name() {
    let mut proc = Process::new();
    let current_pid = proc.get_current_proc().unwrap().0;
    let result = proc.get_process_name(current_pid);
    println!("Process Name: {result:#?}");
    assert!(result.is_ok(), "Failed to get process name");
  }
}
