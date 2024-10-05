use chrono::format;
use sysfo::{Battery, PrettyBattery};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut manager = sysfo::SystemInfoManager::new()?;
	manager.refresh();

	// println!("{:#?}", manager.battery());
	// println!("{}", manager.battery());
	// println!("{:#?}", sysfo::get_battery_info());
	// println!("{}", manager.hardware());
	// println!("{}", manager.software());
	// println!("{}", manager.time());
	// // println!("{}", manager.battery());
	// println!("{}", manager.general());

	let battery = Battery::default();

	println!("\n{:#?}", battery);
	// Print the entire battery info in a formatted way
	println!("Pretty{}\n", battery);

	// Access pretty-printed data
	println!("{}", battery.statement());
	println!("Level: {}", battery.pretty_level());
	println!("Status: {}", battery.pretty_status());
	println!("Time remaining: {}", battery.pretty_time_remaining());
	println!("Technology: {}", battery.pretty_technology());
	println!("Cycle: {}", battery.pretty_cycles().0);
	println!("Brand: {}", battery.pretty_brand());

	Ok(())
}
