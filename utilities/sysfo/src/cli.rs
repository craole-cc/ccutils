use clap::Parser;
use sysfo::{battery, SystemInfoManager};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Battery {
        #[clap(subcommand)]
        command: Option<battery::Commands>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = Manager::new()?;
    manager.refresh();

    println!("{:#?}", manager);

    // let battery = Battery::default(); // TODO: I don't want to have to do this here
    let cli = Cli::parse();

    // match &cli.command {
    //     Some(Commands::Battery { command }) => {
    //         println!("{}", battery.handle_command(command.as_ref()));
    //     }
    //     None => println!("No command provided"),
    // }

    Ok(())
}
