use clap::{Parser, ValueEnum};
use std::process::{exit, Command};

#[derive(Parser, Debug)]
#[command(name = "Rust Search Tool")]
#[command(version = "1.0")]
#[command(about = "Search for files matching a pattern")]
struct Config {
	/// The path to search in
	#[arg(short, long, default_value = ".")]
	path: String,

	/// The pattern to search for
	#[arg(short = 'g', long)]
	pattern: String,

	/// Maximum search depth
	#[arg(long)]
	depth: Option<u32>,

	/// Patterns to ignore
	#[arg(long, value_delimiter = ',')]
	ignore: Vec<String>,

	/// Limit the number of results
	#[arg(long)]
	limit: Option<u32>,

	/// Enable case-sensitive search
	#[arg(long)]
	case_sensitive: bool,

	/// Include hidden files in the search
	#[arg(long)]
	hidden: bool,
}

fn main() {
	let config = Config::parse();

	execute_process(&config);
}

fn execute_process(config: &Config) {
	let mut command = Command::new("fd");

	if let Some(depth) = config.depth {
		command.arg("--max-depth").arg(depth.to_string());
	}

	if !config.ignore.is_empty() {
		for ignore in &config.ignore {
			command.arg("--ignore").arg(ignore);
		}
	}

	if config.hidden {
		command.arg("--hidden");
	}

	if config.case_sensitive {
		command.arg("--case-sensitive");
	} else {
		command.arg("--ignore-case");
	}

	command.arg(&config.pattern).arg(&config.path);

	let output = command.output().expect("Failed to execute process");

	if output.status.success() {
		let result = String::from_utf8_lossy(&output.stdout);
		println!("{}", result);
	} else {
		eprintln!("No results found");
		exit(1);
	}
}
