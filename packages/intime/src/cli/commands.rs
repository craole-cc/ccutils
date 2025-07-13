use super::{fetch, greet};
use clap::{CommandFactory, Parser, Subcommand};
use std::process;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Clap error: {0}")]
  Clap(#[from] clap::Error),

  #[error("IO error: {0}")]
  IO(#[from] std::io::Error),

  #[error("Greet error: {0}")]
  Greet(#[from] greet::Error),

  #[error("Fetch error: {0}")]
  Fetch(#[from] fetch::Error)
  // #[error("Unknown error occurred")]
  // Unknown
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  /// Duration in seconds (optionally followed by 's', 'm', 'h', or 'd')
  #[arg(required = false)]
  pub duration: Option<String>,

  #[command(subcommand)]
  pub command: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
  Greet(greet::Command),
  Fetch(fetch::Command)
}

impl Cli {
  pub fn run(self) -> Result<()> {
    match (self.duration, self.command) {
      (Some(duration), _) => {
        fetch::Command { duration }.execute()?;
      }
      (_, Some(Commands::Greet(cmd))) => {
        cmd.execute()?;
      }
      (_, Some(Commands::Fetch(cmd))) => {
        cmd.execute()?;
      }
      (None, None) => {
        Cli::command().print_help()?;
        process::exit(1);
      }
    }
    Ok(())
  }
}

pub fn run() -> Result<()> {
  Cli::parse().run()
}
