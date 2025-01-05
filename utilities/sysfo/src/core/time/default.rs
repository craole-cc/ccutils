use chrono::{DateTime, Local, TimeDelta, TimeZone};
use std::fmt::{self, Display, Formatter};
use sysinfo::System;

#[derive(Debug)]
pub struct Info {
    pub current: DateTime<Local>,
    pub boot: DateTime<Local>,
    pub uptime: TimeDelta,
    pub timezone: String,
    pub dtfmt: &'static str,
}

impl Display for Info {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.fetch())
    }
}

impl Default for Info {
    fn default() -> Self {
        let current = Local::now();
        let boot = Local
            .timestamp_opt(System::boot_time() as i64, 0)
            .single()
            .unwrap_or_else(Local::now);
        let uptime = current.signed_duration_since(boot);
        let timezone =
            iana_time_zone::get_timezone().unwrap_or_else(|_| boot.format("%Z").to_string());
        let dtfmt = "%Y-%m-%d %H:%M";

        Self {
            current,
            boot,
            uptime,
            timezone,
            dtfmt,
        }
    }
}
impl Info {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn refresh(&mut self) {
        self.current = Local::now();
        self.boot = Local
            .timestamp_opt(System::boot_time() as i64, 0)
            .single()
            .unwrap_or_else(Local::now);
        self.uptime = self.current.signed_duration_since(self.boot);
    }

    pub fn set_dtfmt(&mut self, dtfmt: &'static str) {
        self.dtfmt = dtfmt;
    }

    pub fn current_fmt(&self) -> String {
        self.current.format(self.dtfmt).to_string()
    }

    pub fn boot_fmt(&self) -> String {
        self.boot.format(self.dtfmt).to_string()
    }

    pub fn uptime_fmt(&self) -> String {
        let uptime = self.current.signed_duration_since(self.boot);
        format!("{} mins", uptime.num_minutes())
    }

    pub fn statement(&self) -> String {
        "This is the statement for time".to_string()
    }

    pub fn fetch(&self) -> String {
        format!(
            "Time {{\n\
            {:>16}: {}\n\
            {:>16}: {}\n\
            {:>16}: {}\n\
            {:>16}: {}\n\
            {:>16}: {}\n\
            }}",
            "Active",
            self.uptime_fmt(),
            "Boot",
            self.boot_fmt(),
            "Current",
            self.current_fmt(),
            "Timezone",
            self.timezone,
            "Statement",
            self.statement()
        )
    }
}
