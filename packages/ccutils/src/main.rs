// --- Imports ---
mod cli;
mod commands;
mod config;
mod utilities;

// --- Exports ---
pub use cli::Cli;
pub use config::Workspace;
pub use logline::{debug, error, info, trace, warn};

fn main() -> anyhow::Result<()> {
  //{ Initialize logging }
  logline::init();

  //{ Parse command line arguments }
  let cli = Cli::parse();
  debug!("{:#?}", cli);

  //{ Identify the current configuration }
  let bin = Workspace::define()?;
  debug!("{:#?}", bin);

  //{ Execute the parsed command }
  bin.execute_command(&cli)?; // No change here, as the term_size is now handled within execute_command.

  //{ Finish }
  Ok(())
}
