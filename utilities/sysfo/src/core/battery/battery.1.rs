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
	pub time_left: Duration,
	pub technology: Technology,
	pub cycles: Option<u32>,
	pub brand: Option<String>,
	// energy: 42577.2 m^2 kg^1 s^-2,
	// energy_full: 182577.6 m^2 kg^1 s^-2,
	// energy_full_design: 182577.6 m^2 kg^1 s^-2,
	// energy_rate: 36.775 m^2 kg^1 s^-3,
	// voltage: 12.239 m^2 kg^1 s^-3 A^-1,
}

#[derive(clap::Parser)]
#[command(about = "Get information about the system battery")]
pub enum BatteryCommands {
	#[command(
		about = "Display a natural language statement about the battery",
		long_about = "Provides a detailed, human-readable statement regarding the current battery status."
	)]
	Statement,

	#[command(
		about = "Show all battery information",
		long_about = "Displays comprehensive information about the battery, including level, status, and more."
	)]
	All,

	#[command(
		about = "Show battery level percentage",
		long_about = "Indicates the current battery level as a percentage."
	)]
	Level,

	#[command(
		about = "Show battery charging status",
		long_about = "Reveals whether the battery is currently charging, discharging, or fully charged."
	)]
	Status,

	#[command(
		about = "Show time remaining for charge/discharge",
		long_about = "Estimates the remaining time for the battery to fully charge or discharge based on current usage."
	)]
	Time,

	#[command(
		about = "Show battery technology",
		long_about = "Provides details about the battery's technology, such as Li-ion or NiMH."
	)]
	Technology,

	#[command(
		about = "Show battery cycle count",
		long_about = "Displays the number of charge cycles the battery has undergone."
	)]
	Cycle,

	#[command(
		about = "Show battery manufacturer",
		long_about = "Identifies the company that manufactured the battery."
	)]
	Brand,
}
pub trait PrettyBattery {
	fn pretty_level(&self) -> String;
	fn pretty_status(&self) -> String;
	fn pretty_time_left(&self) -> String;
	fn pretty_technology(&self) -> String;
	fn pretty_cycles(&self) -> (u32, &str);
	fn pretty_brand(&self) -> &str;
	fn statement(&self) -> String;
	fn all(&self) -> String;
}

impl Default for Battery {
	fn default() -> Self {
		pub fn get_battery_info() -> battery::Battery {
			// TODO: Handle errors properly
			BatteryManager::new()
				.expect("Failed to create battery manager")
				.batteries()
				.expect("Failed to get batteries")
				.next()
				.expect("Failed to get battery information")
				.expect("Failed to get battery information")
		}
		let battery = get_battery_info();
		let level = battery.state_of_charge().value;
		let status = battery.state();
		let time_to_empty = battery.time_to_empty();
		let time_to_full = battery.time_to_full();
		let time_left = if time_to_empty.is_some() {
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
			time_left,
			technology,
			cycles,
			brand,
		}
	}
}

impl PrettyBattery for Battery {
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
impl Battery {
	pub fn handle_command(
		&self,
		command: Option<&BatteryCommands>,
	) -> String {
		match command {
			None => self.all(),
			Some(cmd) => match cmd {
				BatteryCommands::Statement => self.statement(),
				BatteryCommands::All => self.all(),
				BatteryCommands::Level => self.pretty_level(),
				BatteryCommands::Status => self.pretty_status(),
				BatteryCommands::Time => {
					self.pretty_time_left()
				}
				BatteryCommands::Technology => {
					self.pretty_technology()
				}
				BatteryCommands::Cycle => {
					self.pretty_cycles().0.to_string()
				}
				BatteryCommands::Brand => {
					self.pretty_brand().to_string()
				}
			},
		}
	}
}
