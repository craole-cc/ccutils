fn main() {
	logline::init();

	let info = sysfo::Fetcher::default();
	// println!("{}", info.battery);
	println!("{:#?}", info.time);
	logline::trace!("{:#?}", info.time);
	logline::error!("{:#?}", info.time);
}
