use tracing::{info, Level};
use tracing_subscriber::{
	fmt::{self, time::uptime},
	EnvFilter,
};

/// A struct to configure and initialize tracing.
#[derive(Debug)]
pub struct Tracer {
	use_env: bool,
	with_time_type: TimeOption,
	without_time: bool,
	with_duration: bool,
	with_file: bool,
	with_target: bool,
	with_lines: bool,
	with_level: bool,
	max_level: Level,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum TimeOption {
	#[default]
	None,
	Duration,
	Time,
}

impl Default for Tracer {
	fn default() -> Self {
		Self {
			use_env: false, // Use environment variables
			with_time_type: TimeOption::default(), // Time option
			without_time: true, // Hides all timestamps and duration output
			with_duration: true, // Shows the time it took to execute
			with_file: false, // Shows the file name and line number
			with_target: true, // Shows the name of the crate that logs the trace
			with_lines: false, // Shows the line number
			with_level: true,  // Shows the log level
			max_level: Level::INFO, // The default log level
		}
	}
}

impl Tracer {
	/// Creates a new Tracer with default settings.
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
		self.with_time_type = option.clone();
		self
	}

	pub fn without_time(mut self) -> Self {
		self.with_time_type = TimeOption::None;
		self
	}

	pub fn with_time(mut self) -> Self {
		self.with_time_type = TimeOption::Time;
		self
	}

	pub fn with_duration(mut self) -> Self {
		self.with_time_type = TimeOption::Duration;
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
			true => match self.with_time_type {
				TimeOption::Duration => {
					self.init_via_env_with_duration()
				}
				TimeOption::Time => self.init_via_env_with_time(),
				TimeOption::None => {
					self.init_via_env_with_without_time()
				}
			},
			false => match self.with_time_type {
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
