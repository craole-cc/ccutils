use tracer::{Level, Tracer};

// mod cli;
// cli::init();

fn main() {
	// Initialize tracing with environment filter and custom options
	Tracer::new()
		.with_max_level(Level::TRACE)
		.with_lines()
		.with_time()
		.with_duration()
		.without_time()
		.init();

	let message = "Hello, world!";
	println!("{}", message);
	tracer::info!("{}", message);
	tracer::error!("{}", message);
	tracer::debug!("{}", message);
	tracer::warn!("{}", message);
	tracer::trace!("{}", message);
}
