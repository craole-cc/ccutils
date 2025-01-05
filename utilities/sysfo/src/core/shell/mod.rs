// mod cli;
// mod default;
// mod error;
// mod display;
mod default;
mod utils;

pub use default::{Info, Name};

// pub use cli::Commands;
// pub use default::Info;
// pub use error::Error;
// pub use utils::*;

// pub use display::Display;

pub fn test() -> Result<(), anyhow::Error> {
    let shell = utils::Shell::current();
    logline::debug!("Shell: {:?}", shell);
    Ok(())
}
