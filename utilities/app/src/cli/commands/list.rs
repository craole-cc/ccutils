use super::default::ListCommands;
use anyhow::Result;

pub fn list(cmd: &ListCommands) -> Result<()> {
    match cmd {
        ListCommands::Installed => installed(),
        ListCommands::Available { query } => available(query),
        ListCommands::Outdated => outdated(),
        ListCommands::Managers => managers(),
    }
}

fn installed() -> Result<()> {
    // Implementation for listing installed packages
    todo!()
}

fn available(query: &Option<String>) -> Result<()> {
    // Implementation for listing/searching available packages
    todo!()
}

fn outdated() -> Result<()> {
    // Implementation for listing outdated packages
    todo!()
}

fn managers() -> Result<()> {
    // Implementation for listing available package managers
    todo!()
}
