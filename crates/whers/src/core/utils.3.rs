use super::error::{CommandError, PathError};
use std::collections::HashSet;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use whereismybin::whereismybin;
use which::which;

#[derive(Debug)]
pub enum CommandLocation {
	Executable(PathBuf),
	ShellBuiltin,
	ShellAlias(String),
	ShellFunction(String),
}

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

fn get_shell_builtin_commands() -> Vec<String> {
	if cfg!(target_family = "unix") {
		vec![
			"alias", "bg", "cd", "command", "echo", "eval", "exec",
			"exit", "export", "fg", "jobs", "kill", "pwd", "read",
			"set", "source", "type", "umask", "unalias", "wait",
		]
	} else {
		vec![
			"cd", "chdir", "cls", "copy", "del", "dir", "echo",
			"exit", "md", "mkdir", "move", "path", "ren", "rename",
			"rmdir", "set", "type",
		]
	}
	.into_iter()
	.map(String::from)
	.collect()
}

fn get_current_shell() -> String {
	env::var("SHELL")
		.or_else(|_| env::var("ComSpec"))
		.unwrap_or_else(|_| "Unknown".to_string())
}

fn check_shell_alias_or_function(
	command: &str,
) -> Option<CommandLocation> {
	let shell = get_current_shell();
	if shell.contains("powershell") || shell.contains("pwsh") {
		let output = Command::new("powershell")
			.args([
				"-Command",
				&format!(
					"Get-Alias {} -ErrorAction SilentlyContinue",
					command
				),
			])
			.output()
			.ok()?;
		if output.status.success() {
			return Some(CommandLocation::ShellAlias(
				String::from_utf8_lossy(&output.stdout).to_string(),
			));
		}

		let output = Command::new("powershell")
            .args(["-Command", &format!("Get-Command {} -CommandType Function -ErrorAction SilentlyContinue", command)])
            .output()
            .ok()?;
		if output.status.success() {
			return Some(CommandLocation::ShellFunction(
				String::from_utf8_lossy(&output.stdout).to_string(),
			));
		}
	} else if shell.contains("bash") || shell.contains("sh") {
		let output = Command::new("bash")
			.args(["-c", &format!("type -t {}", command)])
			.output()
			.ok()?;
		match String::from_utf8_lossy(&output.stdout).trim() {
			"alias" => {
				return Some(CommandLocation::ShellAlias(
					"Bash alias".to_string(),
				))
			}
			"function" => {
				return Some(CommandLocation::ShellFunction(
					"Bash function".to_string(),
				))
			}
			_ => {}
		}
	}
	None // Explicit return of None if no match is found
}

pub fn pathof_cmd(
	command: &str,
) -> Result<Vec<CommandLocation>, CommandError> {
	let mut locations = Vec::new();

	// Check if it's a shell builtin
	if get_shell_builtin_commands().contains(&command.to_lowercase())
	{
		locations.push(CommandLocation::ShellBuiltin);
	}

	// Check for shell alias or function
	if let Some(location) = check_shell_alias_or_function(command) {
		locations.push(location);
	}

	// Try whereismybin
	if let Some(path) = whereismybin(command) {
		locations.push(CommandLocation::Executable(path));
	}

	// Try which
	if let Ok(path) = which(command) {
		locations.push(CommandLocation::Executable(path));
	}

	// Search in PATH
	for path in search_in_path(command) {
		locations.push(CommandLocation::Executable(path));
	}

	// If no locations found, return an error
	if locations.is_empty() {
		Err(CommandError::CommandNotFound(command.to_string()))
	} else {
		Ok(locations)
	}
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
