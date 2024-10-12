use tracing::Level;

use crate::Info;
use std::fmt::Display;

impl Display for Info {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		todo!()
	}
}

impl Info {
	pub fn print_statement(&self) {
		let level = format!(
			" for {}",
			match self.level {
				Level::TRACE => "all log levels",
				Level::DEBUG => "debugging",
				Level::INFO => "info, warnings, and errors",
				Level::WARN => "errors and warnings",
				Level::ERROR => "errors only",
			}
		);

		// println!(
		// 	"level_filter: {}",
		// 	self.level_filter()
		// 		.to_string()
		// 		.split(',')
		// 		.last()
		// 		.map(|s| s.trim())
		// 		.unwrap_or("")
		// );

		println!("LogLine initialized{}", level);
	}
}
