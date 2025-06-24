use std::{
	env,
	path::{Path, PathBuf},
};
use whereismybin::whereismybin;
use which::which;


pub fn locate_executable(command: &str) -> Option<PathBuf> {
	// Try whereismybin
	if let Some(path) = whereismybin(command) {
		path;
	}

	// Try which
	if let Ok(path) = which(command) {
		path;
	}

	// Search in PATH
	for path in search_in_path(command) {
		path;
	}

	None
}


pub fn get_path_env() -> Vec<PathBuf> {
	env::var_os("PATH")
		.map(|paths| env::split_paths(&paths).collect())
		.unwrap_or_default()
}

pub fn search_in_path(command: &str) -> Vec<PathBuf> {
	get_path_env()
		.into_iter()
		.filter_map(|dir| {
			let full_path = dir.join(command);
			if full_path.is_file() && is_executable(&full_path) {
				Some(full_path)
			} else {
				None
			}
		})
		.collect()
}

#[cfg(target_family = "unix")]
pub fn get_shell_builtin_commands() -> Vec<String> {
	// This list is not exhaustive and may vary depending on the shell
	vec![
		"alias", "bg", "cd", "command", "echo", "eval", "exec",
		"exit", "export", "fg", "jobs", "kill", "pwd", "read", "set",
		"source", "type", "umask", "unalias", "wait",
	]
	.into_iter()
	.map(String::from)
	.collect()
}

#[cfg(target_family = "windows")]
pub fn get_shell_builtin_commands() -> Vec<String> {
	// This list is not exhaustive and may vary depending on the shell (cmd.exe or PowerShell)
	vec![
		"cd", "chdir", "cls", "copy", "del", "dir", "echo", "exit",
		"md", "mkdir", "move", "path", "ren", "rename", "rmdir",
		"set", "type",
	]
	.into_iter()
	.map(String::from)
	.collect()
}

#[cfg(unix)]
pub fn is_executable(path: &Path) -> bool {
	use std::os::unix::fs::PermissionsExt;
	path.metadata()
		.map(|m| m.permissions().mode() & 0o111 != 0)
		.unwrap_or(false)
}

#[cfg(windows)]
pub fn is_executable(path: &Path) -> bool {
	use std::path::Path;

	path.extension().map_or(false, |ext| {
		ext.eq_ignore_ascii_case("exe")
			|| ext.eq_ignore_ascii_case("cmd")
			|| ext.eq_ignore_ascii_case("bat")
	})
}
