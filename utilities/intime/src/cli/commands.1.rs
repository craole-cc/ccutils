use clap::{CommandFactory, Parser, Subcommand};

use super::{fetch, greet};

#[derive(Parser)]
pub struct Cli {
	/// Duration in seconds (optionally followed by 's', 'm', 'h', or 'd')
	#[arg(required = false)]
	pub duration: Option<String>,

	#[command(subcommand)]
	pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
	Greet(greet::Command),
	Fetch(fetch::Command),
}

impl Cli {
	pub fn init(self) {
		if let Some(duration) = self.duration {
			// If duration is provided, execute fetch with it
			let default = fetch::Command { duration };
			default.execute();
		} else if let Some(command) = self.command {
			match command {
				Commands::Greet(cmd) => cmd.execute(),
				Commands::Fetch(cmd) => cmd.execute(),
			}
		} else {
			// Handle the case where neither duration nor command is provided
			if let Err(err) = Cli::command().print_help() {
				eprintln!("Failed to print help: {}", err);
			}
		}
	}
}

pub fn init() {
	Cli::parse().init();
}
