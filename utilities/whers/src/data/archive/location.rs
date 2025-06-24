use crate::tools::locate::{
	get_shell_builtin_commands, locate_executable,
};
use std::{collections::HashSet, path::PathBuf};

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Location {
	Executable(PathBuf),
	ShellBuiltin,
	ShellAlias(String),
	ShellFunction(String),
	File(PathBuf),
	Directory(PathBuf),
}

impl Location {
	pub fn locate(name: &str) -> Result<HashSet<Self>, Error> {
		let mut locations = HashSet::new();

		// Check if it's a shell builtin
		if get_shell_builtin_commands().contains(&name.to_lowercase())
		{
			locations.insert(Self::ShellBuiltin);
		}

		// Try whereismybin
		if let Some(path) = locate_executable(name) {
			locations.insert(Self::Executable(path));
		}

		// Search for files and directories
		for path in Self::search_files_and_directories(name) {
			if path.is_file() {
				locations.insert(Self::File(path));
			} else if path.is_dir() {
				locations.insert(Self::Directory(path));
			}
		}

		// If no locations found, return an error
		if locations.is_empty() {
			Err(Error::NotFound(name.to_string()))
		} else {
			Ok(locations)
		}
	}
}
