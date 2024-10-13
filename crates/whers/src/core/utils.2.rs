use super::error::{CommandError, PathError};
use std::collections::HashSet;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use whereismybin::whereismybin;
use which::which;

fn get_path_env() -> Vec<PathBuf> {
	env::var_os("PATH")
		.map(|paths| env::split_paths(&paths).collect())
		.unwrap_or_default()
}

fn search_in_path(command: &str) -> Vec<PathBuf> {
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
fn get_shell_builtin_commands() -> Vec<String> {
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
fn get_shell_builtin_commands() -> Vec<String> {
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

pub fn pathof_cmd(
	command: &str,
) -> Result<Vec<PathBuf>, CommandError> {
	let mut paths = HashSet::new();

	// Check if it's a shell builtin
	if get_shell_builtin_commands().contains(&command.to_lowercase())
	{
		paths.insert(PathBuf::from("[shell builtin]"));
	}

	// Try whereismybin
	if let Some(path) = whereismybin(command) {
		paths.insert(path);
	}

	// Try which
	if let Ok(path) = which(command) {
		paths.insert(path);
	}

	// Search in PATH
	paths.extend(search_in_path(command));

	// If no paths found, return an error
	if paths.is_empty() {
		Err(CommandError::CommandNotFound(command.to_string()))
	} else {
		Ok(paths.into_iter().collect())
	}
}

// Helper function to check if a path is executable
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
