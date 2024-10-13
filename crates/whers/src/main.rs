mod core;
use erks::{AnyhowResult, Context};
use std::path::PathBuf;

fn main() -> AnyhowResult<()> {
	logline::init();

	test_pathof_cmd("rustc");
	test_pathof_cmd("type");
	test_pathof_cmd("ls");
	test_pathof_cmd("mv");
	test_pathof_cmd("ver3n");
	test_pathof_cmd("wal");
	test_pathof_cmd("fd");
	test_pathof_cmd("find");
	test_pathof_cmd("pwsh.exe");
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
