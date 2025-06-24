mod builder;
mod cli;
mod installer;
mod utils;
mod workspace;

use anyhow::Result;
use cli::Cli;
use workspace::Workspace;

fn main() -> Result<()> {
  let cli = Cli::parse();
  let workspace = Workspace::find_current()?;

  workspace.execute_command(&cli)?;

  Ok(())
}
