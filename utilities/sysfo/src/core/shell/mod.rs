// mod cli;
// mod default;
// mod error;
mod utils;

// mod display;

// pub use cli::Commands;
// pub use default::Info;
// pub use error::Error;
// pub use utils::*;

// pub use display::Display;

pub fn test() -> Result<(), anyhow::Error> {
    let shell = utils::Shell::current();
    println!("Detected shell: {:?}", shell);
    Ok(())
}
