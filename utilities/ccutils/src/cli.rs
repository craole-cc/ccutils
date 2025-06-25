use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
  /// Build binary crates (without installing)
  Build {
    /// Specific crates to build
    crates: Vec<String>
  },
  /// Install binary crates (without building first)
  Install {
    /// Specific crates to install
    crates: Vec<String>
  },
  /// Build and install binary crates (default)
  #[command(name = "build-install")]
  BuildInstall {
    /// Specific crates to build and install
    crates: Vec<String>
  }
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
  /// The command to run
  #[command(subcommand)]
  pub command: Option<Command>,

  /// Skip checking modification times and force rebuild/install
  #[arg(short, long)]
  pub force: bool,

  /// Verbose output
  #[arg(short, long)]
  pub verbose: bool,

  /// Specific crates to build and install (when no subcommand is provided)
  pub crates: Vec<String>
}

impl Cli {
  pub fn parse() -> Self {
    <Self as Parser>::parse()
  }

  /// Get the effective command, defaulting to BuildInstall if no subcommand
  /// provided
  pub fn effective_command(&self) -> Command {
    match &self.command {
      Some(cmd) => cmd.clone(),
      None => Command::BuildInstall {
        crates: self.crates.clone()
      }
    }
  }
}
