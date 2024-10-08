use clap::{Parser, ValueEnum};
use rust_search_fork::SearchBuilder;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(name = "WHErs")]
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

	execute_search(&config);
}

fn execute_search(config: &Config) {
	let search =
		SearchBuilder::default().location(&config.path).build();

	// if let Some(depth) = config.depth {
	// 	options.max_depth(depth as usize);
	// }

	// if !config.ignore.is_empty() {
	// 	for ignore in &config.ignore {
	// 		options.ignore_pattern(ignore);
	// 	}
	// }

	// if config.hidden {
	// 	options.include_hidden(true);
	// }

	// options.case_sensitive(config.case_sensitive);

	// if let Some(limit) = config.limit {
	// 	options.max_results(limit as usize);
	// }

	// let results = Search::new(&config.pattern, &config.path, options)
	// 	.expect("Failed to execute search");

	// if !results.is_empty() {
	// 	for result in results {
	// 		println!("{}", result.path().display());
	// 	}
	// } else {
	// 	eprintln!("No results found");
	// 	exit(1);
	// }
}
