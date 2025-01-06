// mod cli;
// mod default;
// mod error;
// mod display;
// mod modules;
mod default;
mod utils;

pub use default::{Info, Kind};
pub use utils::*;

// pub use cli::Commands;
// pub use default::Info;
// pub use error::Error;
// pub use utils::*;

// pub use display::Display;

// pub fn test() -> Result<(), anyhow::Error> {
//     let shell = utils::Shell::current();
//     logline::debug!("Shell: {:?}", shell);
//     Ok(())
// }
