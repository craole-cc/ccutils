// use tracing_subscriber::{
// 	fmt::{self, time::uptime},
// 	EnvFilter,
// };

use crate::{Info, Level, Time};
use tracing::Subscriber;
use tracing_subscriber::{
	fmt::{time::uptime, SubscriberBuilder},
	FmtSubscriber,
};

impl Info {
	/// Creates a new logline with default settings.
	pub fn new() -> Self {
		Self::default()
	}

	/// Sets the maximum log level.
	pub fn with_max_level(mut self, level: Level) -> Self {
		self.level = level;
		self
	}

	pub fn show_level(mut self) -> Self {
		self.options.show_level = true;
		self
	}

	pub fn with_time(mut self, time: Time) -> Self {
		self.time = time;
		self
	}

	pub fn hide_time(mut self) -> Self {
		self.time = Time::None;
		self
	}

	pub fn show_time(mut self) -> Self {
		self.time = Time::Datetime;
		self
	}

	pub fn show_duration(mut self) -> Self {
		self.time = Time::Duration;
		self
	}

	pub fn use_env(mut self) -> Self {
		self.options.use_env = true;
		self
	}

	pub fn show_target(mut self) -> Self {
		self.options.show_target = true;
		self
	}

	/// Configures whether to include the file in logs.
	pub fn show_file(mut self) -> Self {
		self.options.show_file = true;
		self
	}

	/// Configures whether to include the line number in logs.
	pub fn show_line(mut self) -> Self {
		self.options.show_line = true;
		self
	}

	pub fn init_with_time(&self) {
		let mut builder =
			tracing_subscriber::FmtSubscriber::builder()
				.with_level(self.options.show_level)
				.with_file(self.options.show_file)
				.with_line_number(self.options.show_line)
				.with_target(self.options.show_target)
				.with_max_level(self.level);

		let subscriber = builder.finish();

		tracing::subscriber::set_global_default(subscriber)
			.expect("Failed to set global subscriber")
	}

	pub fn init_with_duration(&self) {
		tracing_subscriber::FmtSubscriber::builder()
			.with_timer(uptime())
			.with_level(self.options.show_level)
			.with_file(self.options.show_file)
			.with_line_number(self.options.show_line)
			.with_target(self.options.show_target)
			.with_max_level(self.level)
			.finish();
	}

	pub fn init_without_time(&self) {
		let mut builder =
			tracing_subscriber::FmtSubscriber::builder()
				.without_time()
				.with_level(self.with_level)
				.with_file(self.with_file)
				.with_line_number(self.with_lines)
				.with_target(self.with_target)
				.with_max_level(self.max_level);

		let subscriber = builder.finish();

		tracing::subscriber::set_global_default(subscriber)
			.expect("Failed to set global subscriber")
	}

	pub fn init_via_env_with_time(&self) {
		tracing_subscriber::fmt()
			.with_env_filter(EnvFilter::from_default_env())
			.with_level(self.with_level)
			.with_file(self.with_file)
			.with_line_number(self.with_lines)
			.with_target(self.with_target)
			.with_max_level(self.max_level)
			.init()
	}

	pub fn init_via_env_with_duration(&self) {
		tracing_subscriber::fmt()
			.with_env_filter(EnvFilter::from_default_env())
			.with_timer(uptime())
			.with_level(self.with_level)
			.with_file(self.with_file)
			.with_line_number(self.with_lines)
			.with_target(self.with_target)
			.with_max_level(self.max_level)
			.init()
	}

	pub fn init_via_env_without_time(&self) {
		tracing_subscriber::fmt()
			.with_env_filter(EnvFilter::from_default_env())
			.without_time()
			.with_level(self.with_level)
			.with_file(self.with_file)
			.with_line_number(self.with_lines)
			.with_target(self.with_target)
			.with_max_level(self.max_level)
			.init()
	}

	pub fn init(&self) {
		match self.options.use_env {
			true => match self.time {
				Time::None => {
					self.init_via_env_without_time();
				}
				Time::Duration => self.init_via_env_with_duration(),
				Time::Datetime => {
					self.init_via_env_with_time();
				}
			},
			false => {
				let builder = match self.time {
					Time::Duration => {
						self.init_with_duration();
					}
					Time::Datetime => {
						self.init_with_time();
					}
					Time::None => {
						self.init_without_time();
					}
				};

				let subscriber = builder.finish();

				tracing::subscriber::set_global_default(subscriber)
					.expect("Failed to set global subscriber")
			}
		}
	}
}
