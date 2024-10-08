mod cli;

// #[derive(Parser, Debug)]
// #[command(name="WHErs",version, about, long_about = None)]
// struct Cli {
// 	/// The pattern to search for
// 	pattern: Option<String>,

// 	/// Turn debugging information on
// 	#[arg(short, long, action = clap::ArgAction::Count)]
// 	debug: u8,

// 	/// The path to search in
// 	#[arg(short, long, default_value = ".")]
// 	path: String,

// 	/// Maximum search depth
// 	#[arg(long)]
// 	depth: Option<usize>,

// 	/// Type of path to search for
// 	/// `file` - search for files
// 	/// `dir` - search for directories
// 	#[arg(long, value_enum, default_value_t = PathType::File)]
// 	path_type: PathType,

// 	/// Patterns to ignore
// 	#[arg(long, value_delimiter = ',')]
// 	ignore: Vec<String>,

// 	/// Limit the number of results
// 	#[arg(long)]
// 	limit: Option<usize>,

// 	/// Enable case-sensitive search
// 	#[arg(long)]
// 	case_sensitive: bool,

// 	/// Include hidden files in the search
// 	#[arg(short = 'H', long)]
// 	hidden: bool,
// }

// #[derive(Debug, Clone, PartialEq, clap::ValueEnum)]
// enum PathType {
// 	File,
// 	Dir,
// 	All,
// }

fn main() {
	init_tracing();
	cli::init();
}

fn init_tracing() {
	tracing::subscriber::set_global_default(
		tracing_subscriber::FmtSubscriber::builder()
			.with_max_level(tracing::Level::INFO)
			.finish(),
	)
	.expect("setting default subscriber failed");
}
