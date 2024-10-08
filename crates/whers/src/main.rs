use tracer::{Level, Tracer};

// mod cli;
// cli::init();

fn main() {
    // Initialize tracing with environment filter and custom options
    Tracer::new().init();

    // .with_max_level(Level::DEBUG) // Set desired log level
    // .without_time() // Optionally disable timestamps
    // .with_target(false)
    // .init();
    let message = "Hello, world!";
    println!("{}", message);
    tracer::info!("{}", message);
    tracer::error!("{}", message);
    tracer::debug!("{}", message);
    tracer::warn!("{}", message);
    tracer::trace!("{}", message);
}
