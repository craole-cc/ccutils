use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to greet: {0}")]
    GreetingFailed(String),

    #[error("Invalid name provided: {0}")]
    InvalidName(String),

    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(clap::Parser)]
#[command(about = "A greeter")]
pub struct Command {
    /// Name of the person to greet
    pub name: String,
}

impl Command {
    pub fn execute(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(Error::InvalidName("Name cannot be empty".to_string()));
        }
        println!("Hello, {}!", self.name);
        Ok(())
    }
}
