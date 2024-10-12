// use tracing_subscriber::{
// 	fmt::{self, time::uptime},
// 	EnvFilter,
// };

use crate::{Level, Time};
use std::str::FromStr;
use tracing::subscriber;
use tracing_subscriber::{
	filter,
	fmt::time::{uptime, Uptime},
	EnvFilter, FmtSubscriber,
};

impl crate::Info {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_max_level(mut self, level: Level) -> Self {
		self.level = level;
		self
	}

	pub fn show_level(mut self) -> Self {
		self.options.show_level = true;
		self
	}

	pub fn hide_level(mut self) -> Self {
		self.options.show_level = false;
		self
	}

	pub fn with_time(mut self, time: Time) -> Self {
		self.time = time;
		self
	}

	pub fn show_time(mut self) -> Self {
		self.time = Time::Datetime;
		self
	}

	pub fn hide_time(mut self) -> Self {
		self.time = Time::None;
		self
	}

	pub fn show_duration(mut self) -> Self {
		self.time = Time::Duration;
		self
	}

	pub fn hide_duration(mut self) -> Self {
		self.time = Time::None;
		self
	}

	pub fn show_target(mut self) -> Self {
		self.options.show_target = true;
		self
	}

	pub fn hide_target(mut self) -> Self {
		self.options.show_target = false;
		self
	}

	pub fn show_file(mut self) -> Self {
		self.options.show_file = true;
		self
	}

	pub fn hide_file(mut self) -> Self {
		self.options.show_file = false;
		self
	}

	pub fn show_line(mut self) -> Self {
		self.options.show_line = true;
		self
	}

	pub fn use_env(mut self) -> Self {
		self.options.use_env = true;
		self
	}
	
	fn init_via_env(&mut self) {
		let mut subscriber = tracing_subscriber::fmt()
			.with_level(self.options.show_level)
			.with_file(self.options.show_file)
			.with_target(self.options.show_target)
			.with_line_number(self.options.show_line)
			.with_env_filter(EnvFilter::from_default_env());

		match self.time {
			Time::None => subscriber.without_time().init(),
			Time::Datetime => subscriber.init(),
			Time::Duration => subscriber.with_timer(uptime()).init(),
		};

		self.print_statement();
	}

	fn init_via_max(&mut self) {
		let mut subscriber = tracing_subscriber::fmt()
			.with_level(self.options.show_level)
			.with_file(self.options.show_file)
			.with_target(self.options.show_target)
			.with_line_number(self.options.show_line)
			.with_max_level(self.level);

		match self.time {
			Time::None => subscriber.without_time().init(),
			Time::Datetime => subscriber.init(),
			Time::Duration => subscriber.with_timer(uptime()).init(),
		};

		self.print_statement();
	}

	pub fn init(&mut self) {
		if self.options.use_env {
			self.init_via_env()
		} else {
			self.init_via_max()
		}
	}
}
