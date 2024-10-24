#[derive(Debug, Clone)]
pub struct Options {
	pub file: bool,
	pub target: bool,
	pub line: bool,
	pub level: bool,
	pub thread: bool,
	pub thread_id: bool,
	pub time: bool,
	pub uptime: bool,
	pub pretty: bool,
}

impl Default for Options {
	fn default() -> Self {
		Self {
			file: false,
			target: false,
			line: false,
			level: true,
			thread: false,
			thread_id: false,
			time: false,
			uptime: false,
			pretty: true,
		}
	}
}

impl Options {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn verbose(mut self) -> Self {
		self = self.show_uptime();
		self = self.show_level();
		self = self.show_target();
		self = self.show_line();
		self = self.show_thread();
		self
	}

	pub fn pretty(mut self) -> Self {
		self.pretty = true;
		self.level = true;
		// self.file = true;
		self.target = true;
		self.line = true;
		self.thread = true;
		self.uptime = true;
		self
	}

	pub fn show_level(mut self) -> Self {
		self.level = true;
		self
	}

	pub fn hide_level(mut self) -> Self {
		self.level = false;
		self
	}

	pub fn show_time(mut self) -> Self {
		self.time = true;
		self
	}

	pub fn hide_time(mut self) -> Self {
		self.time = false;
		self
	}

	pub fn show_uptime(mut self) -> Self {
		self.uptime = true;
		self
	}

	pub fn hide_uptime(mut self) -> Self {
		self.uptime = false;
		self
	}

	pub fn show_target(mut self) -> Self {
		self.target = true;
		self
	}

	pub fn hide_target(mut self) -> Self {
		self.target = false;
		self
	}

	pub fn show_file(mut self) -> Self {
		self.file = true;
		self
	}

	pub fn hide_file(mut self) -> Self {
		self.file = false;
		self
	}

	pub fn show_thread(mut self) -> Self {
		self.thread = true;
		self
	}

	pub fn hide_thread(mut self) -> Self {
		self.thread = false;
		self
	}

	pub fn show_thread_id(mut self) -> Self {
		self.thread_id = true;
		self
	}

	pub fn hide_thread_id(mut self) -> Self {
		self.thread_id = false;
		self
	}

	pub fn show_line(mut self) -> Self {
		self.line = true;
		self
	}
}
