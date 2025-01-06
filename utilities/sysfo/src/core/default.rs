use super::{process, time};
use sysinfo::System;

#[derive(Debug)]
pub struct Fetcher {
    pub time: time::Info,
    pub process: process::Info,
    // pub shell: shell::Info,
}

impl Default for Fetcher {
    fn default() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        let time = time::Info::default();
        // let process = process::Info::default();
        let process = process::Info::new(&system);
        // let shell = shell::Info::new(&process);
        // let shell = shell::Info::default();

        Self {
            time,
            process,
            // shell,
        }
    }
}

pub fn init() -> Fetcher {
    Fetcher::default()
}

pub fn test() {
    let info = init();
    let msg = "Testing Fetcher";
    logline::debug!("{}", msg);
    logline::debug!("{}", info.time.fetch());
    logline::debug!("{}", info.process.fetch());
    // logline::debug!("{}", info.shell.fetch());

    // crate::core::shell::test();
}
