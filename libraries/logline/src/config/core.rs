use super::{Level, Options, Time};

#[derive(Debug, Clone)]
pub struct Logline {
	pub level: Level,
	pub time: Time,
	pub display: Options,
}

impl Default for Logline {
	fn default() -> Self {
		let display = Options::default();
		let time = if display.time {
			Time::Datetime
		} else if display.uptime {
			Time::Uptime
		} else {
			Time::None
		};
		let level = Level::default();

		Self {
			level,
			time,
			display,
		}
	}
}

impl Logline {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn init(&mut self) {
		let timer = tracing_subscriber::fmt::time::uptime();
		let filter = self.level.filter();
		let mut subscriber =
			tracing_subscriber::fmt().with_env_filter(filter);

		subscriber = subscriber
			.with_level(self.display.level)
			.with_file(self.display.file)
			.with_target(self.display.target)
			.with_thread_names(self.display.thread)
			.with_thread_ids(self.display.thread_id)
			.with_line_number(self.display.line);

		match self.display.pretty {
			true => match self.time {
				Time::None => {
					subscriber.without_time().pretty().init()
				}
				Time::Datetime => subscriber.pretty().init(),
				Time::Uptime => {
					subscriber.with_timer(timer).pretty().init()
				}
			},
			false => match self.time {
				Time::None => subscriber.without_time().init(),
				Time::Datetime => subscriber.init(),
				Time::Uptime => subscriber.with_timer(timer).init(),
			},
		}
		// subscriber.init();
	}

	pub fn with_level(mut self, level: Level) -> Self {
		self.level = level;
		self
	}

	pub fn with_time(mut self, time: Time) -> Self {
		self.time = time;
		self
	}

	pub fn use_env(mut self) -> Self {
		self.level = Level::ENV;
		self
	}

	pub fn hide_level(mut self) -> Self {
		self.display.level = false;
		self
	}

	pub fn show_line(mut self) -> Self {
		self.display.line = true;
		self
	}

	pub fn show_target(mut self) -> Self {
		self.display.target = true;
		self
	}

	pub fn ugly(mut self) -> Self {
		self.display.pretty = false;
		self
	}
}
