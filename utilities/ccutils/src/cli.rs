use crate::commands::{clean, install};
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
    crates: Vec<String>,
    /// Installation mode for binary names
    #[arg(short, long, value_enum, default_value_t = install::Target::Both)]
    mode: install::Target
  },
  /// Build and install binary crates (default)
  #[command(name = "build-install")]
  BuildInstall {
    /// Specific crates to build and install
    crates: Vec<String>,
    /// Installation mode for binary names
    #[arg(short, long, value_enum, default_value_t = install::Target::Both)]
    mode: install::Target
  },
  /// Clean target directories and/or installed binaries
  Clean {
    /// Specific crates to clean (empty = all)
    crates: Vec<String>,
    /// What to clean
    #[arg(short, long, value_enum, default_value_t = clean::Target::Dir)]
    target: clean::Target
  },
  /// Uninstall binary crates
  Uninstall {
    /// Specific crates to uninstall
    crates: Vec<String>,
  },
  /// List workspace crates and their status
  List {
    /// Show detailed information
    #[arg(short, long)]
    detailed: bool,
    /// Only show binary crates
    #[arg(short, long)]
    bins_only: bool,
    /// Only show library crates
    #[arg(short, long)]
    libs_only: bool
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

  /// Installation mode for binary names (when no subcommand is provided)
  #[arg(long, value_enum, default_value_t = install::Target::Both)]
  pub install_mode: install::Target,

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
    self
      .command
      .clone()
      .unwrap_or_else(|| Command::BuildInstall {
        crates: self.crates.clone(),
        mode: self.install_mode.clone()
      })
  }
}
