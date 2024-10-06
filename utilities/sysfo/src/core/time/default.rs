use chrono::{DateTime, Local, TimeDelta, TimeZone};
use sysinfo::System;
// use iana_time_zone::{get_timezone, GetTimezoneError};

#[derive(Debug)]
pub struct Info {
	pub current: DateTime<Local>,
	pub boot: DateTime<Local>,
	pub uptime: TimeDelta,
	// pub active: Duration,
	pub timezone: String,
	pub dtfmt: &'static str,
}

impl Default for Info {
	fn default() -> Self {
		let current = Local::now();
		let boot = Local
			.timestamp_opt(System::boot_time() as i64, 0)
			.single()
			.unwrap_or_default();
		// let active = Duration::until_now(booted);
		let uptime = current.signed_duration_since(boot);
		let timezone = iana_time_zone::get_timezone()
			.unwrap_or(boot.format("%Z").to_string());
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

impl std::fmt::Display for Info {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.all())
	}
}

impl Info {
	pub fn set_dtfmt(&mut self, dtfmt: &'static str) {
		self.dtfmt = dtfmt
	}

	pub fn current_fmt(&self) -> String {
		self.current.format(self.dtfmt).to_string()
	}

	pub fn boot_fmt(&self) -> String {
		self.boot.format(self.dtfmt).to_string()
	}

	// pub fn active_fmt(&self) -> Duration {
	// 	Duration::until_now(self.booted);
	// }

	pub fn uptime_fmt(&self) -> String {
		let uptime = self.current.signed_duration_since(self.boot);
		format!("{} mins", uptime.num_minutes())
	}

	pub fn statement(&self) -> String {
		"Tis is the statement for time".to_string()
	}

	pub fn fetch(&self) -> String {
		format!(
			"Time {{\n\
			{:>16}: {}\n\
			{:>16}: {}\n\
			{:>16}: {}\n\
			{:>16}: {}\n\
			}}",
			"Active",
			self.uptime,
			"Booted",
			self.boot,
			"Now",
			self.current,
			"Timezone",
			self.timezone,
		)
	}

	pub fn all(&self) -> String {
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
