use super::{utils::*, Error};
use std::{fmt::format, process::id};
use sysinfo::{Pid, System};

#[derive(Debug)]
pub struct ProcessInfo {
    pub id: u32,
    pub name: String,
}

/// Struct to encapsulate system process operations.
#[derive(Debug)]
pub struct Info {
    pub current: ProcessInfo,
    pub parent: ProcessInfo,
}

impl Info {
    pub fn new(system: &System) -> Result<Self, Error> {
        let id = id();
        let name = get_process_name(system, id)?;
        let parent_id = get_parent_id(system, id)?;
        let parent_name = get_process_name(system, parent_id)?;
        // let shell_id = get_parent_id(system, parent_id)?; // TODO: Get this from the Shell struct
        // let shell_name = get_process_name(system, shell_id)?; // TODO: Get this from the Shell struct

        // logline::debug!(
        //     "Process {} ({}) has parent {} ({}). The shell is {} ({})",
        //     name,
        //     id,
        //     parent_name,
        //     parent_id,
        //     // shell_id,
        //     // shell_name
        // );

        Ok(Self {
            current: ProcessInfo { id, name },
            parent: ProcessInfo {
                id: parent_id,
                name: parent_name,
            },
            // shell: (0, String::new()),
        })
    }

    pub fn fetch(&self) -> String {
        format!("{:#?}", self)
        // format!(
        //     "{:?} ({:?}) -> {:?} ({:?})",
        //     self.current.name, self.current.id, self.parent.name, self.parent.id,
        // )
    }
}
