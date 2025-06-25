use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum InstallMode {
  /// Install with default name only (e.g., 'wallter')
  Unprefixed,
  /// Install with workspace-prefixed name only (e.g., 'ccutils-wallter')
  Prefixed,
  /// Install both unprefixed and prefixed versions (default)
  #[default]
  Both
}

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
    #[arg(short, long, value_enum, default_value_t = InstallMode::Both)]
    mode: InstallMode
  },
  /// Build and install binary crates (default)
  #[command(name = "build-install")]
  BuildInstall {
    /// Specific crates to build and install
    crates: Vec<String>,
    /// Installation mode for binary names
    #[arg(short, long, value_enum, default_value_t = InstallMode::Both)]
    mode: InstallMode
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
  #[arg(long, value_enum, default_value_t = InstallMode::Both)]
  pub install_mode: InstallMode,

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
        crates: self.crates.clone(),
        mode: self.install_mode.clone()
      }
    }
  }
}
