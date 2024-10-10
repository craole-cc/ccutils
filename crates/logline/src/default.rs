use tracing::{info, Level};
use tracing_subscriber::{
	fmt::{self, time::uptime},
	EnvFilter,
};

/// A struct to configure and initialize tracing.
#[derive(Debug)]
pub struct Options {
	max_level: Level,
	time_type: TimeOption,
	use_env: bool,
	with_file: bool,
	with_target: bool,
	with_lines: bool,
	with_level: bool,
}

impl Default for Options {
	fn default() -> Self {
		Self {
			max_level: Level::INFO,
			time_type: TimeOption::default(),
			with_file: false,
			use_env: false,
			with_target: true,
			with_lines: false,
			with_level: true,
		}
	}
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum TimeOption {
	#[default]
	None,
	Duration,
	Time,
}

impl Options {
	/// Creates a new logline with default settings.
	pub fn new() -> Self {
		Self::default()
	}

	/// Enables environment variable filtering.
	pub fn with_env(mut self) -> Self {
		self.use_env = true;
		self
	}

	/// Sets the maximum log level.
	pub fn with_max_level(mut self, level: Level) -> Self {
		self.max_level = level;
		self
	}

	/// Configures the time option.
	pub fn with_time_option(mut self, option: TimeOption) -> Self {
		self.time_type = option.clone();
		self
	}

	pub fn without_time(mut self) -> Self {
		self.time_type = TimeOption::None;
		self
	}

	pub fn with_time(mut self) -> Self {
		self.time_type = TimeOption::Time;
		self
	}

	pub fn with_duration(mut self) -> Self {
		self.time_type = TimeOption::Duration;
		self
	}

	/// Configures whether to include the target in logs.
	pub fn with_target(mut self) -> Self {
		self.with_target = true;
		self
	}

	/// Configures whether to include the file in logs.
	pub fn with_file(mut self) -> Self {
		self.with_file = true;
		self
	}

	/// Configures whether to include the line number in logs.
	pub fn with_lines(mut self) -> Self {
		self.with_lines = true;
		self
	}
	pub fn init_via_subscriber_with_time(self) {
		let mut builder =
			tracing_subscriber::FmtSubscriber::builder()
				.with_level(self.with_level)
				.with_file(self.with_file)
				.with_line_number(self.with_lines)
				.with_target(self.with_target)
				.with_max_level(self.max_level);

		let subscriber = builder.finish();

		tracing::subscriber::set_global_default(subscriber)
			.expect("Failed to set global subscriber");

		info!("Tracing initialized successfully");
	}

	pub fn init_via_subscriber_with_duration(self) {
		let mut builder =
			tracing_subscriber::FmtSubscriber::builder()
				.with_timer(uptime())
				.with_level(self.with_level)
				.with_file(self.with_file)
				.with_line_number(self.with_lines)
				.with_target(self.with_target)
				.with_max_level(self.max_level);

		let subscriber = builder.finish();

		tracing::subscriber::set_global_default(subscriber)
			.expect("Failed to set global subscriber");

		info!("Tracing initialized successfully");
	}

	pub fn init_via_subscriber_without_time(self) {
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
			.expect("Failed to set global subscriber");

		info!("Tracing initialized successfully");
	}

	pub fn init_via_env_with_time(self) {
		tracing_subscriber::fmt()
			.with_env_filter(EnvFilter::from_default_env())
			.with_level(self.with_level)
			.with_file(self.with_file)
			.with_line_number(self.with_lines)
			.with_target(self.with_target)
			.with_max_level(self.max_level)
			.init();

		tracing::trace!("Tracing initialized successfully!");
	}

	pub fn init_via_env_with_duration(self) {
		tracing_subscriber::fmt()
			.with_env_filter(EnvFilter::from_default_env())
			.with_timer(uptime())
			.with_level(self.with_level)
			.with_file(self.with_file)
			.with_line_number(self.with_lines)
			.with_target(self.with_target)
			.with_max_level(self.max_level)
			.init();

		tracing::trace!("Tracing initialized successfully!");
	}

	pub fn init_via_env_with_without_time(self) {
		tracing_subscriber::fmt()
			.with_env_filter(EnvFilter::from_default_env())
			.without_time()
			.with_level(self.with_level)
			.with_file(self.with_file)
			.with_line_number(self.with_lines)
			.with_target(self.with_target)
			.with_max_level(self.max_level)
			.init();

		tracing::trace!("Tracing initialized successfully!");
	}

	pub fn init(self) {
		match self.use_env {
			true => match self.time_type {
				TimeOption::Duration => {
					self.init_via_env_with_duration()
				}
				TimeOption::Time => self.init_via_env_with_time(),
				TimeOption::None => {
					self.init_via_env_with_without_time()
				}
			},
			false => match self.time_type {
				TimeOption::Duration => {
					self.init_via_subscriber_with_duration()
				}
				TimeOption::Time => {
					self.init_via_subscriber_with_time()
				}
				TimeOption::None => {
					self.init_via_subscriber_without_time()
				}
			},
		}
	}
}
