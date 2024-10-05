fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut manager = sysfo::SystemInfoManager::new()?;
	manager.refresh();

	println!("{:#?}", manager.battery());
	println!("{}", manager.battery());
	println!("{:#?}", sysfo::get_battery_info());
	println!("{}", manager.hardware());
	println!("{}", manager.software());
	println!("{}", manager.time());
	// println!("{}", manager.battery());
	println!("{}", manager.general());

	// match manager.battery() {
	// 	Ok(battery) => println!("{}", battery),
	// 	Err(e) => println!("Failed to get battery info: {}", e),
	// }

	Ok(())
}
