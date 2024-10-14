use super::Error;
use crate::config::Format;
use std::{
	collections::HashSet,
	env, fmt,
	path::{Path, PathBuf},
	process::Command,
};
use whereismybin::whereismybin;
use which::which;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Location {
	Executable(PathBuf),
	ShellBuiltin,
	ShellAlias(String),
	ShellFunction(String),
}

impl fmt::Display for Location {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match f.alternate() {
			true => match self {
				Self::Executable(path) => {
					write!(f, "Executable: {}", path.display())
				}
				Self::ShellBuiltin => {
					write!(f, "Shell Builtin")
				}
				Self::ShellAlias(info) => {
					write!(f, "Shell Alias: {}", info)
				}
				Self::ShellFunction(info) => {
					write!(f, "Shell Function: {}", info)
				}
			},
			false => match self {
				Self::Executable(path) => {
					write!(f, "{}", path.display())
				}
				Self::ShellBuiltin => write!(f, "builtin"),
				Self::ShellAlias(info) => {
					write!(f, "alias:{}", info)
				}
				Self::ShellFunction(info) => {
					write!(f, "function:{}", info)
				}
			},
		}
	}
}

impl Location {
	pub fn find(command: &str) -> Result<HashSet<Self>, Error> {
		let mut locations = HashSet::new();

		// Check if it's a shell builtin
		if Self::get_shell_builtin_commands()
			.contains(&command.to_lowercase())
		{
			locations.insert(Self::ShellBuiltin);
		}

		// Try whereismybin
		if let Some(path) = whereismybin(command) {
			locations.insert(Self::Executable(path));
		}

		// Try which
		if let Ok(path) = which(command) {
			locations.insert(Self::Executable(path));
		}

		// Search in PATH
		for path in Self::search_in_path(command) {
			locations.insert(Self::Executable(path));
		}

		// If no locations found, return an error
		if locations.is_empty() {
			Err(Error::CommandNotFound(command.to_string()))
		} else {
			Ok(locations)
		}
	}

	fn get_path_env() -> Vec<PathBuf> {
		env::var_os("PATH")
			.map(|paths| env::split_paths(&paths).collect())
			.unwrap_or_default()
	}

	fn search_in_path(command: &str) -> Vec<PathBuf> {
		Self::get_path_env()
			.into_iter()
			.filter_map(|dir| {
				let full_path = dir.join(command);
				if full_path.is_file()
					&& Self::is_executable(&full_path)
				{
					Some(full_path)
				} else {
					None
				}
			})
			.collect()
	}

	#[cfg(target_family = "unix")]
	fn get_shell_builtin_commands() -> Vec<String> {
		// This list is not exhaustive and may vary depending on the shell
		vec![
			"alias", "bg", "cd", "command", "echo", "eval", "exec",
			"exit", "export", "fg", "jobs", "kill", "pwd", "read",
			"set", "source", "type", "umask", "unalias", "wait",
		]
		.into_iter()
		.map(String::from)
		.collect()
	}

	#[cfg(target_family = "windows")]
	fn get_shell_builtin_commands() -> Vec<String> {
		// This list is not exhaustive and may vary depending on the shell (cmd.exe or PowerShell)
		vec![
			"cd", "chdir", "cls", "copy", "del", "dir", "echo",
			"exit", "md", "mkdir", "move", "path", "ren", "rename",
			"rmdir", "set", "type",
		]
		.into_iter()
		.map(String::from)
		.collect()
	}

	#[cfg(unix)]
	fn is_executable(path: &Path) -> bool {
		use std::os::unix::fs::PermissionsExt;
		path.metadata()
			.map(|m| m.permissions().mode() & 0o111 != 0)
			.unwrap_or(false)
	}

	#[cfg(windows)]
	fn is_executable(path: &Path) -> bool {
		path.extension().map_or(false, |ext| {
			ext.eq_ignore_ascii_case("exe")
				|| ext.eq_ignore_ascii_case("cmd")
				|| ext.eq_ignore_ascii_case("bat")
		})
	}

	pub fn format_output(
		command: &str,
		format: &Format,
	) -> Result<String, Error> {
		let locations = Self::find(command)?;

		if locations.is_empty() {
			return Err(Error::CommandNotFound(command.to_string()));
		}

		Ok(match format {
			Format::Plain => locations
				.into_iter()
				.next()
				.map(|loc| loc.to_string())
				.unwrap_or_default(),
			Format::Fetch => locations
				.into_iter()
				.map(|loc| format!("{:#}", loc))
				.collect::<Vec<_>>()
				.join("\n"),
			Format::Verbose => {
				let mut output = format!(
					"Command '{}' found in {} location(s):",
					command,
					locations.len()
				);
				for location in locations {
					output.push_str(&format!("\n  {:#}", location));
				}
				output
			}
		})
	}
}
