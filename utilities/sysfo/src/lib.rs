pub mod core;
// mod utils;
pub use core::*;

#[derive(Debug, Default)]
pub struct Fetcher {
	// system: System,
	// pub battery: core::battery::Info,
	pub time: time::Info,
}
