use crate::Duration;
use chrono::{DateTime, Local, TimeZone};
use std::fmt::{Display, Formatter, Result};
use sysinfo::System;

#[derive(Debug)]
pub struct Time {
  now: DateTime<Local>,
  booted: DateTime<Local>,
  active: Duration,
  timezone: Local
}

impl Default for Time {
  fn default() -> Self {
    let now = Local::now();
    let booted = Local
      .timestamp_opt(System::boot_time() as i64, 0)
      .single()
      .unwrap_or_default();
    let active = Duration::until_now(booted);
    let timezone = now.timezone();

    Self {
      now,
      booted,
      active,
      timezone
    }
  }
}

impl Time {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn all() -> Self {
    Self::default()
  }
  pub fn now() -> DateTime<Local> {
    Self::new().now
  }

  pub fn booted() -> DateTime<Local> {
    Self::new().booted
  }

  pub fn active() -> Duration {
    Self::new().active
  }
}

impl Display for Time {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let dtfmt = "%Y-%m-%d %H:%M:%S";
    let header = "Time {";
    let active = format!("{:>16}: {}", "Active", self.active);
    let booted = format!("{:>16}: {}", "Booted", self.booted.format(dtfmt));
    let now = format!("{:>16}: {}", "Now", self.now.format(dtfmt));
    let timezone = format!("{:>16}: {:?}", "Timezone", self.timezone);
    let footer = "}";

    write!(
      f,
      "{}\n{}\n{}\n{}\n{}\n{}",
      header, active, booted, now, timezone, footer
    )
  }
}
