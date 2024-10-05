// power.rs
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
	level: f32,
	status: State,
	time_remaining: Duration,
	technology: Technology,
	cycles: u32,
	brand: String,
	// vendor: Some(
	// 		"HP",
	// ),
	// model: Some(
	// 		"Primary",
	// ),
	// serial_number: Some(
	// 		"SerialNumber",
	// ),
	// technology: LithiumIon,
	// capacity: 1.0,
	// temperature: None,
	// percentage: 0.23320056,
	// cycle_count: Some(
	// 		78,
	// ),
	// energy: 42577.2 m^2 kg^1 s^-2,
	// energy_full: 182577.6 m^2 kg^1 s^-2,
	// energy_full_design: 182577.6 m^2 kg^1 s^-2,
	// energy_rate: 36.775 m^2 kg^1 s^-3,
	// voltage: 12.239 m^2 kg^1 s^-3 A^-1,
	// time_to_full: Some(
	// 		3806.9446 s^1,
	// ),
	// time_to_empty: None,
}

impl Default for Battery {
	fn default() -> Self {
		let battery = get_battery_info();
		let level = battery.state_of_charge().value * 100.0;
		let health = battery.state_of_health();
		let cycles = battery
			.cycle_count()
			.expect("Unable to get battery cycle count");
		let brand = battery
			.vendor()
			.map(|s| s.to_string())
			.unwrap_or_default();
		let status = battery.state();

		let time_remaining = if battery.time_to_empty().is_some() {
			Duration::from_battery_time(battery.time_to_empty())
		} else if battery.time_to_full().is_some() {
			Duration::from_battery_time(battery.time_to_full())
		} else {
			Duration::default()
		};

		let technology = battery.technology();

		Self {
			level,
			status,
			time_remaining,
			technology,
			cycles,
			brand,
		}
	}
}

impl Battery {
	pub fn fetch() {
		Self::default();
	}
}

impl Display for Battery {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let header = "Battery {";
		let level = format!("{:>16}: {}", "Level", self.level);
		let status = format!("{:>16}: {}", "Status", self.status);
		let time_remaining = format!(
			"{:>16}: {}",
			"Time Remaining", self.time_remaining
		);
		let footer = "}";

		write!(
			f,
			"{}\n{}\n{}\n{}\n{}",
			header, level, status, time_remaining, footer
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

// fn extract_seconds(time_str: Option<String>) -> f64 {
// 	// Split by space and extract the first part (numeric value)
// 	let seconds = time_str
// 		.split_whitespace()
// 		.next()
// 		.and_then(|s| s.parse::<f64>().ok())
// 		.unwrap_or(0.0);

// 	seconds
// }
