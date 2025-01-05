use super::{process, time};
use anyhow::Ok;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use sysinfo::System;

pub static SYSTEM_INFO: Lazy<Mutex<System>> = Lazy::new(|| {
    let mut sys = System::new_all();
    sys.refresh_all();
    Mutex::new(sys)
});

#[derive(Debug)]
pub struct Fetcher {
    pub time: time::Info,
    pub process: process::Info,
}

impl Default for Fetcher {
    fn default() -> Self {
        let mut info = SYSTEM_INFO.lock().unwrap();
        info.refresh_all();

        let time = time::Info::new();
        let process = process::Info::new(&info).expect("Failed to get process info");

        Self { time, process }
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

    logline::debug!("{}", info.time.fetch());
    logline::debug!("{}", info.process.fetch());
    // logline::debug!("{}", info..fetch());

    crate::core::shell::test();

    Ok(())
}
