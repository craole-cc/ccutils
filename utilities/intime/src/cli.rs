use intime::Info;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
	/// Optional name to operate on
	name: Option<String>,
}
// pub enum Commands {
// 	#[command(
// 		about = "Show all information using a 16-tab inline buffer"
// 	)]
// 	Fetch,

// 	#[command(about = "List all information")]
// 	All,
// 	// #[command(
// 	// 	about = "Display a natural language statement about the information"
// 	// )]
// 	// Statement,
// }

// impl Default for Commands {
// 	fn default() -> Self {
// 		Self::All
// 	}
// }

// impl Commands {
// 	pub fn handle_command(&self, info: &Info) -> String {
// 		match self {
// 			Self::Fetch => info.fetch(),
// 			Self::All => info.fetch(),
// 			// Self::Statement => info.statement(),
// 		}
// 	}
// }

impl Cli {
	pub fn init() -> Self {
		Self { name: None }
	}

	pub fn handle_command(&self, info: &Info) -> String {
		info.fetch()
	}
}
