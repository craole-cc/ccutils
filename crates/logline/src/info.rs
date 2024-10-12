use core::time;
use std::str::FromStr;
use tracing::Level;
use tracing_subscriber::EnvFilter;

/// Represents the information that can be logged.
#[derive(Debug, Clone)]
pub struct Info {
	/// The maximum level of the log message.
	///
	/// This field determines the severity of the log message.
	/// The available levels are Trace, Debug, Info, Warn, and Error.
	// pub max_level: MaxLevel,
	pub level: Level,

	/// The time format to be used in the log message.
	///
	/// This field determines how the timestamp will be displayed in the log message.
	/// The available options are None (no timestamp), Duration (time elapsed since the start of the program),
	/// and Datetime (the current date and time).
	pub time: Time,
	/// The options that determine the format of the log message.
	///
	/// This field is a struct that contains boolean values for each formatting option.
	/// The available options are:
	/// - use_env: Whether to use environment variables to configure the logger.
	/// - show_file: Whether to include the file name in the log message.
	/// - show_target: Whether to include the target (module path) in the log message.
	/// - show_lines: Whether to include the line number in the log message.
	/// - show_time: Whether to include the timestamp in the log message.
	/// - show_level: Whether to include the log level in the log message.
	pub options: Options,
}

impl Default for Info {
	fn default() -> Self {
		Self {
			level: Level::INFO,
			time: Time::default(),
			options: Options {
				show_level: true,
				..Default::default()
			},
		}
	}
}

/// The time format to be used in the log message.
///
/// This enum represents the timestamp format that will be used in the log message.
/// The available options are None (no timestamp), Duration (time elapsed since the start of the program),
/// and Datetime (the current date and time).
#[derive(Debug, Default, PartialEq, Clone)]
pub enum Time {
	#[default]
	None,
	Datetime,
	Duration,
}

/// The options that determine the format of the log message.
///
/// This struct contains boolean values for each formatting option.
/// The available options are:
/// - use_env: Whether to use environment variables to configure the logger.
/// - show_file: Whether to include the file name in the log message.
/// - show_target: Whether to include the target (module path) in the log message.
/// - show_lines: Whether to include the line number in the log message.
/// - show_level: Whether to include the log level in the log message.
#[derive(Default, Debug, Clone)]
pub struct Options {
	pub use_env: bool,
	pub show_file: bool,
	pub show_target: bool,
	pub show_line: bool,
	pub show_level: bool,
}
