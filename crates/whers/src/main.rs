// mod cli;

fn main() {
	logline::init();
	// init_tracing()
	// cli::init();

	let message = "Tracing initialized via logline!";
	logline::trace!("{}", message);
	logline::debug!("{}", message);
	logline::info!("{}", message);
	logline::warn!("{}", message);
	logline::error!("{}", message);
}
