use std::env;
use std::fs;
use std::process::{exit, Command};

#[derive(Default)]
struct Config {
	path: String,
	pattern: String,
	depth: Option<u32>,
	ignore: Vec<String>,
	limit: Option<u32>,
	case_sensitive: bool,
	hidden: bool,
}

fn main() {
	let mut config = Config::default();

	// Parse command-line arguments
	parse_arguments(&mut config);

	// Print environment if debug is enabled
	if config.limit.is_some() {
		print_env(&config);
	} else {
		execute_process(&config);
	}
}

fn parse_arguments(config: &mut Config) {
	let args: Vec<String> = env::args().collect();

	for i in 1..args.len() {
		match args[i].as_str() {
			"-h" | "--help" => {
				println!("Usage: ...");
				exit(0);
			}
			"-v" | "--version" => {
				println!("Version: 1.0");
				exit(0);
			}
			"--path" => {
				if i + 1 < args.len() {
					config.path = args[i + 1].clone();
				}
			}
			"-p" | "--pattern" => {
				if i + 1 < args.len() {
					config.pattern = args[i + 1].clone();
				}
			}
			"--depth" => {
				if i + 1 < args.len() {
					config.depth = args[i + 1].parse().ok();
				}
			}
			"--ignore" => {
				if i + 1 < args.len() {
					config.ignore.push(args[i + 1].clone());
				}
			}
			"--limit" => {
				if i + 1 < args.len() {
					config.limit = args[i + 1].parse().ok();
				}
			}
			"--case-sensitive" => config.case_sensitive = true,
			"--hidden" => config.hidden = true,
			_ => {}
		}
	}

	// Set default values
	if config.path.is_empty() {
		config.path = env::current_dir()
			.unwrap()
			.to_string_lossy()
			.into_owned();
	}

	if config.pattern.is_empty() {
		eprintln!("ERROR: Pattern must be specified.");
		exit(1);
	}
}

fn print_env(config: &Config) {
	println!("PATH: {}", config.path);
	println!("PATTERN: {}", config.pattern);
	println!("DEPTH: {:?}", config.depth);
	println!("IGNORE: {:?}", config.ignore);
	println!("LIMIT: {:?}", config.limit);
	println!("HIDDEN: {}", config.hidden);
	println!("CASE SENSITIVE: {}", config.case_sensitive);
}

fn execute_process(config: &Config) {
	let mut command = Command::new("fd"); // You can switch to "find" based on your needs

	command
		.arg("--max-depth")
		.arg(config.depth.unwrap_or(2).to_string());

	if !config.ignore.is_empty() {
		for ignore in &config.ignore {
			command.arg("--ignore").arg(ignore);
		}
	}

	if !config.case_sensitive {
		command.arg("--case-sensitive");
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
