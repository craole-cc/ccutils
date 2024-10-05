use crate::{duration, Duration};
use battery::{
	units::{
		time::{self, nanosecond},
		Time,
	},
	Manager as BatteryManager, State, Technology,
};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct Battery {
	pub level: f32,
	pub status: State,
	pub time_to_full: Option<Time>,
	pub time_to_empty: Option<Time>,
	pub time_remaining: Duration,
	pub technology: Technology,
	pub cycles: Option<u32>,
	pub brand: Option<String>,
	// energy: 42577.2 m^2 kg^1 s^-2,
	// energy_full: 182577.6 m^2 kg^1 s^-2,
	// energy_full_design: 182577.6 m^2 kg^1 s^-2,
	// energy_rate: 36.775 m^2 kg^1 s^-3,
	// voltage: 12.239 m^2 kg^1 s^-3 A^-1,
}

pub trait PrettyBattery {
	fn pretty_level(&self) -> String;
	fn pretty_status(&self) -> String;
	fn pretty_time_remaining(&self) -> String;
	fn pretty_technology(&self) -> String;
	fn pretty_cycles(&self) -> (u32, String);
	fn pretty_brand(&self) -> String;
	fn statement(&self) -> String;
}

impl Default for Battery {
	fn default() -> Self {
		let battery = get_battery_info();
		let level = battery.state_of_charge().value * 100.0;
		let status = battery.state();
		let time_to_empty = battery.time_to_empty();
		let time_to_full = battery.time_to_full();
		let time_remaining = if time_to_empty.is_some() {
			Duration::from_battery_time(time_to_empty)
		} else if time_to_full.is_some() {
			Duration::from_battery_time(time_to_full)
		} else {
			Duration::default()
		}
		.above_seconds()
		.clone();
		let technology = battery.technology();
		let cycles = battery.cycle_count();
		let brand = battery.vendor().map(|s| s.to_string());

		Self {
			level,
			status,
			time_to_full,
			time_to_empty,
			time_remaining,
			technology,
			cycles,
			brand,
		}
	}
}

impl PrettyBattery for Battery {
	fn statement(&self) -> String {
		let status = match self.time_remaining.is_zero() {
			true => format!("{:?}", self.status),
			false => format!(
				"The battery is currently at {} and {}, with {} remaining",
				self.pretty_level(),
				self.status,
				self.time_remaining
			),
		};

		let cycle = if self.pretty_cycles().0 > 0 {
			format!(
				"on this it's {:?}{:?} charge cycle.",
				self.pretty_cycles().0,
				self.pretty_cycles().1
			)
		} else {
			".".to_string()
		};

		let make = format!(
			"It is made of {} materials by {}.",
			self.technology,
			if self.pretty_brand() != "unknown" {
				self.pretty_brand()
			} else {
				"an unknown manufacturer".to_string()
			}
		);

		format!("{} {} {}", status, cycle, make)
	}

	fn pretty_level(&self) -> String {
		format!("{}%", self.level.round() as i32)
	}

	fn pretty_status(&self) -> String {
		format!("{:?}", self.status)
	}

	fn pretty_time_remaining(&self) -> String {
		format!("{}", self.time_remaining)
	}

	fn pretty_technology(&self) -> String {
		format!("{:?}", self.technology)
	}

	fn pretty_cycles(&self) -> (u32, String) {
		if self.cycles.is_some() {
			let cycle_number = self.cycles.unwrap();
			let ordinal_suffix = match cycle_number % 10 {
				1 if cycle_number % 100 != 11 => "st",
				2 if cycle_number % 100 != 12 => "nd",
				3 if cycle_number % 100 != 13 => "rd",
				_ => "th",
			};
			// format!("{}{}", cycle_number, ordinal_suffix)\
			(cycle_number, ordinal_suffix.to_string())
		} else {
			(0, "".to_string())
		}
	}

	fn pretty_brand(&self) -> String {
		let brand = match &self.brand {
			Some(brand) => brand,
			None => "unknown",
		};

		brand.to_string()
	}
}

impl Display for Battery {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(
			f,
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
			self.pretty_time_remaining(),
			"Technology",
			self.pretty_technology(),
			"Cycles",
			self.pretty_cycles().0,
			"Brand",
			self.pretty_brand()
		)
	}
}
pub fn get_battery_info() -> battery::Battery {
	BatteryManager::new()
		.expect("Failed to create battery manager")
		.batteries()
		.expect("Failed to get batteries")
		.next()
		.expect("Failed to get battery information")
		.expect("Failed to get battery information")
}
