use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to fetch data: {0}")]
    Fetch(String),

    #[error("Invalid duration: {0}")]
    InvalidDuration(String),

    #[error("Network error: {0}")]
    Network(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(clap::Parser)]
pub struct Command {
    /// Duration in seconds (optionally followed by 's', 'm', 'h', or 'd')
    pub duration: String,
}

impl Command {
    pub fn execute(&self) -> Result<()> {
        let seconds = self.parse_duration(&self.duration)?;
        println!("Fetching for {} seconds...", seconds);
        // Implement fetching logic here
        Ok(())
    }

    fn parse_duration(&self, duration: &str) -> Result<u32> {
        let num_str = duration.trim_end_matches(|c: char| !c.is_ascii_digit());
        let unit_str = duration.trim_start_matches(num_str);
        let num: u32 = num_str
            .parse()
            .map_err(|_| Error::InvalidDuration("Invalid number".to_string()))?;

        match unit_str {
            "s" | "" => Ok(num),
            "m" => Ok(num * 60),
            "h" => Ok(num * 3600),
            "d" => Ok(num * 86400),
            _ => Err(Error::InvalidDuration("Invalid unit".to_string())),
        }
    }
}
