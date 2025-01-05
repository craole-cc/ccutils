use super::{process, time};
use anyhow::Ok;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use sysinfo::System;

pub static SYSTEM_INFO: Lazy<Mutex<System>> = Lazy::new(|| {
    let mut system = System::new_all();
    system.refresh_all();
    Mutex::new(system)
});

#[derive(Debug)]
pub struct Fetcher {
    pub time: time::Info,
    pub process: process::Info,
}

impl Default for Fetcher {
    fn default() -> Self {
        let time = time::Info::default();
        let process = process::Info::default();

        Self {
            time,
            process,
        }
    }
}

impl Fetcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn refresh(&mut self) {
        SYSTEM_INFO.lock().unwrap().refresh_all();
        self.time.refresh();
    }
}

pub fn init() -> Fetcher {
    Fetcher::default()
}

pub fn test() -> anyhow::Result<()> {
    let info = init();

    // logline::debug!("{:#?}", info.system);
    logline::debug!("{}", info.time.fetch());
    logline::debug!("{}", info.process.fetch());
    // logline::debug!("{}", info.process.new());

    // crate::core::shell::test();

    Ok(())
}
