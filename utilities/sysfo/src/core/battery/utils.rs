pub trait Display {
	fn pretty_level(&self) -> String;
	fn pretty_status(&self) -> String;
	fn pretty_time_left(&self) -> String;
	fn pretty_technology(&self) -> String;
	fn pretty_cycles(&self) -> (u32, &str);
	fn pretty_brand(&self) -> &str;
	fn statement(&self) -> String;
	fn all(&self) -> String;
	fn fetch(&self) -> String;
}

impl Display for super::Info {
	fn all(&self) -> String {
		format!("Level: {}\nStatus: {}\nTime: {}\nTechnology: {}\nCycles: {}\nBrand: {}",
			self.pretty_level(),
			self.pretty_status(),
			self.pretty_time_left(),
			self.pretty_technology(),
			self.pretty_cycles().0,
			self.pretty_brand()
		)
	}
	fn statement(&self) -> String {
		use battery::State;
		let has_cycles = self.pretty_cycles().0 > 0;
		let status = match self.status {
			State::Charging | State::Discharging => format!(
				"The battery is currently at {} and {}, with {} remaining {}",
				self.pretty_level(),
				self.status,
				self.pretty_time_left(),
				if has_cycles {
					format!(
						"on this its {}{} charge cycle.",
						self.pretty_cycles().0,
						self.pretty_cycles().1
					)
				} else {
					".".to_string()
				}
			),
			State::Full => {
				format!(
					"The battery is {}{}",
					self.status,
					if has_cycles {
						format!(
							", having being cycled {} times.",
							self.pretty_cycles().0,
						)
					} else {
						".".to_string()
					}
				)
			}
			_ => format!("The battery is {}", self.pretty_level()),
		};

		let make = format!(
			"It is made of {} materials by {}.",
			self.technology,
			if self.pretty_brand() != "unknown" {
				self.pretty_brand()
			} else {
				"an unknown manufacturer"
			}
		);

		format!("{} {}", status, make)
	}

	fn pretty_level(&self) -> String {
		format!("{}%", (self.level * 100.0).round() as i32)
	}

	fn pretty_status(&self) -> String {
		format!("{:?}", self.status)
	}

	fn pretty_time_left(&self) -> String {
		format!("{}", self.time_left)
	}

	fn pretty_technology(&self) -> String {
		format!("{:?}", self.technology)
	}

	fn pretty_cycles(&self) -> (u32, &str) {
		if self.cycles.is_some() {
			let cycle_number = self.cycles.unwrap();
			let ordinal_suffix = match cycle_number % 10 {
				1 if cycle_number % 100 != 11 => "st",
				2 if cycle_number % 100 != 12 => "nd",
				3 if cycle_number % 100 != 13 => "rd",
				_ => "th",
			};
			(cycle_number, ordinal_suffix)
		} else {
			(0, "")
		}
	}

	fn pretty_brand(&self) -> &str {
		match &self.brand {
			Some(brand) => brand,
			None => "unknown",
		}
	}

	fn fetch(&self) -> String {
		format!(
			"Battery {{\n\
			{:>16}: {}\n\
			{:>16}: {}\n\
			{:>16}: {}\n\
			{:>16}: {}\n\
			{:>16}: {}\n\
			{:>16}: {}\n\
			}}",
			"Level",
			self.pretty_level(),
			"Status",
			self.pretty_status(),
			"Time",
			self.pretty_time_left(),
			"Technology",
			self.pretty_technology(),
			"Cycles",
			self.pretty_cycles().0,
			"Brand",
			self.pretty_brand()
		)
	}
}
