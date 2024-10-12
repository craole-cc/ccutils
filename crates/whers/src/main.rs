use logline::{debug, error, info, trace, warn, Info, Level, Time};
// mod cli;

fn main() {
	Info::default()
		// .with_max_level(Level::DEBUG)
		// .use_env()
		// .with_time(Time::Datetime)
		.show_line()
		.init();

	// cli::init();
	let message = "Hello, world!";
	// println!("{}", message);
	trace!("{}", message);
	debug!("{}", message);
	info!("{}", message);
	warn!("{}", message);
	error!("{}", message);
}
