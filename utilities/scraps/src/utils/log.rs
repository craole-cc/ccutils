use tracing::{subscriber, Level};
use tracing_subscriber::FmtSubscriber;

pub fn init(level: Level) {
    subscriber::set_global_default(
        FmtSubscriber::builder()
            .without_time()
            .with_max_level(level)
            .finish(),
    )
    .expect("setting default subscriber failed");
}
