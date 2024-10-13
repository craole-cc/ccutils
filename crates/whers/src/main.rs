mod core;
use erks::{AnyhowResult, Context};
use std::path::PathBuf;

fn main() -> AnyhowResult<()> {
	logline::init();
	logline::info!("{}", "Tracing initialized via logline!");
	// init_tracing()
	// cli::init();

	// let pathof = core::pathof_cmd("ls").context(
	// 	"Failed to find the path"
	// )?;

	test_pathof_cmd("rustc");
	test_pathof_cmd("pathof");
	test_pathof_cmd("type");
	test_pathof_cmd("ls");
	test_pathof_cmd("whereis");

	Ok(())
}

fn test_pathof_cmd(command: &str) {
	match core::pathof_cmd(command) {
		Ok(path) => {
			logline::info!("{:#?}", path);
		}
		Err(e) => {
			logline::error!("{}", e);
		}
	};
}
