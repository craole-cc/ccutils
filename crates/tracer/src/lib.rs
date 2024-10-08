mod default;
pub use default::Tracer;

// Re-export the tracing and tracing_subscriber crates
pub use tracing::{self, debug, error, info, trace, warn, Level};
pub use tracing_subscriber;
