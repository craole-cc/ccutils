fn main() {
	logline::Options::new()
		.with_max_level(logline::Level::TRACE)
		.init();

	let info = sysfo::Fetcher::default();
	// println!("{}", info.battery);
	println!("{:#?}", info.time);
	println!("{}", info.time);
}
