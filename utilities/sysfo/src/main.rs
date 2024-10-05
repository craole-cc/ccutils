use clap::Parser;
use sysfo::{Battery, BatteryCommands};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Battery {
        #[clap(subcommand)]
        command: Option<BatteryCommands>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = sysfo::SystemInfoManager::new()?;
    manager.refresh();

    let battery = Battery::default();
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Battery { command }) => {
            println!("{}", battery.handle_command(command.as_ref()));
        }
        None => println!("No command provided"),
    }

    Ok(())
}
