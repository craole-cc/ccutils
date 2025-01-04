use crate::cli::commands::{add, list, remove, update, Commands, Default, ListCommands};
use anyhow::Result;
use clap::Parser;
use logline::info;

pub fn init() -> Result<()> {
    let mut cli = Default::parse();

    info!("Config: {:#?}", cli.command);

    match cli.command {
        Some(Commands::Add { packages, file }) => {
            info!("Adding packages: {:?}, file: {:?}", packages, file);
            add(&packages, &file);
        }
        Some(Commands::Update { packages, file }) => {
            info!("Updating packages: {:?}, file: {:?}", packages, file);
            update(&packages, &file);
        }
        Some(Commands::Remove { packages }) => {
            info!("Removing packages: {:?}", packages);
            remove(&packages);
        }
        Some(Commands::List { command }) => {
            info!("List command: {:?}", command);
            list(&command);
        }
        None => {
            // Default to list available if a query is provided
            if let Some(query) = cli.query {
                info!("Default list available with query: {}", query);
                list(&ListCommands::Available { query: Some(query) });
            } else {
                info!("No command or query provided");
            }
        }
    }

    Ok(())
}
