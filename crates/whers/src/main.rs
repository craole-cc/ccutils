// mod cli;
// cli::init();

fn main() {
	logline::Options::default().init();

	let message = "Hello, world!";
	println!("{}", message);
	logline::trace!("{}", message);
	logline::debug!("{}", message);
	logline::info!("{}", message);
	logline::warn!("{}", message);
	logline::error!("{}", message);
}
