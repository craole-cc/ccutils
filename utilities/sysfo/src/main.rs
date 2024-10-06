fn main() {
	let info = sysfo::Fetcher::default();
	// println!("{}", info.battery);
	println!("{:#?}", info.time);
	println!("{}", info.time);

	// Get the current time zone as a string.
	let tz_str = iana_time_zone::get_timezone().expect("msg");
	println!("The current time zone is: {}", tz_str);
}
