use data::shell;
use dots::*;

fn main() -> anyhow::Result<()> {
  logline::init_trace();
  logline::trace!("Starting up");

  // Shell Definitions
  logline::trace!(
    "Identified installed shells and their associated rc files (bash, zsh, pwsh, powershell, fish, nushell, etc.)"
  );
  utils::test();

  // Define directories that should be added to path

  // Update the config rc for each shell

  Ok(())
}
