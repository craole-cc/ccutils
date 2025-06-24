use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use rust_search_fork::{FilterExt, SearchBuilder};

#[derive(Parser, Debug, Clone)]
#[command(name="whers",version, about, long_about = None)]
struct Cli {
    /// The pattern to search for
    query: Vec<String>,

    /// Sets a custom ignore file
    #[arg(short, long, value_name = "IGNORE_FILE")]
    ignore_file: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// The path to search in
    #[arg(short, long, default_value = ".")]
    path: String,

    /// Maximum search depth
    #[arg(long)]
    depth: Option<usize>,

    // /// Type of path to search for
    // /// `file` - search for files
    // /// `dir` - search for directories
    // #[arg(long, value_enum, default_value_t = PathType::File)]
    // path_type: PathType,
    /// Patterns to ignore
    #[arg(short, long, value_delimiter = ',')]
    exclude: Vec<String>,

    /// Limit the number of results
    #[arg(long)]
    limit: Option<usize>,

    /// Enable case-sensitive search
    #[arg(long)]
    case_sensitive: bool,

    /// Do not respect .(git|fd|whers)ignore files
    #[arg(short = 'I', long)]
    no_ignore: bool,

    /// Search hidden files and directories
    #[arg(short = 'H', long)]
    hidden: bool,
}

// impl Default for Cli {
// 	fn default() -> Self {
// 		Self {
// 			pattern: Some(".*".to_string()),
// 			debug: 0,
// 			path: "path".to_string(),
// 			depth: None,
// 			ignore: vec![],
// 			limit: None,
// 			case_sensitive: false,
// 			hidden: false,
// 		}
// 	}
// }

impl Cli {
    // fn init_cli() {
    // 		let cli = Cli::parse();
    // 		tracing::info!("Config: {:#?}", cli);

    // 		// Debugging
    // 		match cli.debug {
    // 			0 => println!("Debug mode is off"),
    // 			1 => println!("Debug mode is kind of on"),
    // 			2 => println!("Debug mode is on"),
    // 			_ => println!("Don't be crazy"),
    // 		}

    // 		execute_search(&cli);
    // 	}

    // 	fn execute_search(config: &Cli) {
    // 		let input = match &config.pattern {
    // 			Some(pattern) => pattern.to_string(),
    // 			None => ".*".to_string(),
    // 		};

    // 		let mut options = SearchBuilder::default()
    // 			.location(&config.path)
    // 			.search_input(input)
    // 			.strict();

    // 		if let Some(depth) = config.depth {
    // 			options = options.depth(depth);
    // 		}

    // 		if let Some(limit) = config.limit {
    // 			options = options.limit(limit);
    // 		}

    // 		if !config.case_sensitive {
    // 			options = options.ignore_case();
    // 		}

    // 		if config.hidden {
    // 			options = options.hidden();
    // 		}

    // 		// match config.path_type {
    // 		// 	PathType::File => {
    // 		// 		options = options.custom_filter(|dir| {
    // 		// 			dir.metadata().unwrap().is_file()
    // 		// 		})
    // 		// 	}
    // 		// 	PathType::Dir => {
    // 		// 		options = options
    // 		// 			.custom_filter(|dir| dir.metadata().unwrap().is_dir())
    // 		// 	}
    // 		// 	PathType::All => options,
    // 		// }

    // 		// for ignore in &config.ignore {
    // 		// 	options = options.ignore_pattern(ignore);
    // 		// }

    // 		let result: Vec<String> = options.build().collect();
    // 		tracing::info!("Results: {:#?}", result);

    // 		if !result.is_empty() {
    // 			for result in result {
    // 				println!("{}", result);
    // 			}
    // 		} else {
    // 			eprintln!("No results found");
    // 			exit(1);
    // 		}
    // 	}
}

pub fn init() {
    let mut cli = Cli::parse();
    println!("query: {:?}", cli.query);
    // cli.pattern = Some(match &cli.pattern {
    // 	Some(pattern) => pattern.to_string(),
    // 	None => ".*".to_string(),
    // });

    // if cli.path {
    // 	cli.path = "path".to_string();
    // }

    // Debugging
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // ::info!("Config: {:#?}", cli);
    // search(&cli);
}

fn search(config: &Cli) {
    // let input = match &config.pattern {
    // 	Some(pattern) => pattern.to_string(),
    // 	None => ".*".to_string(),
    // };

    let mut options = SearchBuilder::default()
        .location(&config.path)
        // .search_input(&config.pattern)
        // .search_input(config.pattern.clone().unwrap())
        .strict();

    if let Some(depth) = config.depth {
        options = options.depth(depth);
    }

    if let Some(limit) = config.limit {
        options = options.limit(limit);
    }

    if !config.case_sensitive {
        options = options.ignore_case();
    }

    if config.hidden {
        options = options.hidden();
    }

    // match config.path_type {
    // 	PathType::File => {
    // 		options = options.custom_filter(|dir| {
    // 			dir.metadata().unwrap().is_file()
    // 		})
    // 	}
    // 	PathType::Dir => {
    // 		options = options
    // 			.custom_filter(|dir| dir.metadata().unwrap().is_dir())
    // 	}
    // 	PathType::All => options,
    // }

    // for ignore in &config.ignore {
    // 	options = options.ignore_pattern(ignore);
    // }

    let result: Vec<String> = options.build().collect();
    // tracing::info!("Results: {:#?}", result);

    if !result.is_empty() {
        for result in result {
            println!("{}", result);
        }
    } else {
        eprintln!("No results found");
        std::process::exit(1);
    }
}
