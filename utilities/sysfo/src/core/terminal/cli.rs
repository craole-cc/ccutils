use super::Info;

#[derive(clap::Parser)]
#[command(about = "Get information about the system battery")]
pub enum Commands {
  #[command(
    about = "Display a natural language statement about the battery",
    long_about = "Provides a detailed, human-readable statement regarding the current battery status."
  )]
  Statement,

  #[command(
    about = "Show all battery information",
    long_about = "Displays comprehensive information about the battery, including level, status, and more."
  )]
  All
}

impl Default for Commands {
  fn default() -> Self {
    Self::All
  }
}

impl Commands {
  pub fn handle_command(&self, info: &Info) -> String {
    match self {
      Self::Statement => info.statement(),
      Self::All => info.all()
    }
  }
}
