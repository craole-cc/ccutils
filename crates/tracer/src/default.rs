use tracing::{subscriber, Level};
use tracing_subscriber::{fmt, EnvFilter, FmtSubscriber};

/// A struct to configure and initialize tracing.
#[derive(Debug)]
pub struct Tracer {
    use_env: bool,
    max_level: Level,
    with_target: bool,
    without_time: bool,
    with_timer: bool,
}

impl Default for Tracer {
    fn default() -> Self {
        Self {
            use_env: false,         // Use environment variables
            max_level: Level::INFO, // The default log level
            with_target: true,      // Shows the name of the crate that logs the trace
            with_timer: false,      // Shows the time it took to execute
            without_time: false,    // Hides all timestamps and duration output
        }
    }
}

impl Tracer {
    /// Creates a new Tracer with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables environment variable filtering.
    pub fn with_env(mut self) -> Self {
        self.use_env = true;
        self
    }

    /// Sets the maximum log level.
    pub fn with_max_level(mut self, level: Level) -> Self {
        self.max_level = level;
        self
    }

    /// Disables timestamp in logs.
    pub fn without_time(mut self) -> Self {
        self.without_time = true;
        self
    }

    /// Configures whether to include the target in logs.
    pub fn with_target(mut self, target: bool) -> Self {
        self.with_target = target;
        self
    }

    pub fn init(&self) {
        // let tracer = Tracer::new();
        let tracer = self;
        let max_level = tracer.max_level;
        let with_target = tracer.with_target;
        let timer = tracing_subscriber::fmt::time::uptime();

        // match self.use_env {
        // 	true => {
        // 		todo!()
        // 	}
        // 	false => {
        // 		// Start building the subscriber
        let builder = FmtSubscriber::builder()
            .with_max_level(max_level)
            .with_target(with_target)
            .with_timer(timer);

        let subscriber = builder.finish();

        subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");
        // 	}
        // 	_ => todo!(),
        // }

        // if self.use_env {
        // 	fmt()
        // 		.with_env_filter(EnvFilter::from_default_env())
        // 		.with_target(self.with_target)
        // 		.init();
        // } else {
        // 	// Start building the subscriber
        // 	let builder = FmtSubscriber::builder()
        // 		.with_max_level(self.max_level.unwrap_or(Level::INFO))
        // 		.with_target(self.with_target);

        // 	let subscriber = builder.finish();

        // 	subscriber::set_global_default(subscriber)
        // 		.expect("Failed to set global subscriber");
        // }
    }
}
