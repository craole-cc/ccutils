// tracer/src/lib.rs
pub use tracing; // Re-export the entire tracing module
use tracing::{info, Level};
use tracing_subscriber::{fmt, EnvFilter, FmtSubscriber};

pub struct Tracer {
    use_env: bool,
    max_level: Option<Level>,
    with_target: bool,
    without_time: bool,
}

impl Tracer {
    pub fn new() -> Self {
        Self {
            use_env: false,
            max_level: None,
            with_target: true,
            without_time: false,
        }
    }

    pub fn with_env(mut self) -> Self {
        self.use_env = true;
        self
    }

    pub fn with_max_level(mut self, level: Level) -> Self {
        self.max_level = Some(level);
        self
    }

    pub fn without_time(mut self) -> Self {
        self.without_time = true;
        self
    }

    pub fn with_target(mut self, target: bool) -> Self {
        self.with_target = target;
        self
    }

    pub fn init(self) {
        if self.use_env {
            fmt()
                .with_env_filter(EnvFilter::from_default_env())
                .with_timer(fmt::time::uptime())
                .without_time()
                .with_target(self.with_target)
                .init();

            info!("Tracing initialized successfully with env filter!");
        } else {
            let mut builder = FmtSubscriber::builder()
                .with_max_level(self.max_level.unwrap_or(Level::INFO))
                .without_time()
                .with_target(self.with_target);

            let subscriber = builder.finish();

            tracing::subscriber::set_global_default(subscriber)
                .expect("Failed to set global subscriber");

            info!("Tracing initialized successfully");
        }
    }
}
