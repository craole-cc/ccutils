use clap::Parser;
use erks::{AnyhowResult, Context};
use std::{
	io::{stdout, Write},
	process,
};
use whers::data::{Location, Output::*};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	#[clap(required = true)]
	command: String,

	#[arg(short, long, action = clap::ArgAction::Count)]
	verbose: u8,
}

pub fn init() -> AnyhowResult<()> {
	let cli = Cli::parse();

	let format = match cli.verbose {
		0 => Plain,
		1 => Fetch,
		_ => Verbose,
	};

	match format_output(&cli.command, &format) {
		Ok(output) => {
			if !output.is_empty() {
				print!("{}", output);
				stdout().flush().context("Failed to flush stdout")?;
			}
			Ok(())
		}
		Err(e) => match format {
			Plain => {
				process::exit(1);
			}
			Fetch => {
				eprint!("Command not found: {}", cli.command);
				process::exit(1);
			}
			Verbose => {
				eprint!("Error: {}", e);
				process::exit(1);
			}
		},
	}
}
