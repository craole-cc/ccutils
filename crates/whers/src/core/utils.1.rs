use super::error::{CommandError, PathError};
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::Command;
use whereismybin::whereismybin;
use which::which;

#[cfg(target_os = "windows")]
fn fallback_search(command: &str) -> Vec<PathBuf> {
	let output = Command::new("where").arg(command).output().ok();
	output
		.and_then(|o| String::from_utf8(o.stdout).ok())
		.map(|s| s.lines().map(PathBuf::from).collect())
		.unwrap_or_default()
}

#[cfg(not(target_os = "windows"))]
fn fallback_search(command: &str) -> Vec<PathBuf> {
	let output = Command::new("whereis").arg(command).output().ok();
	output
		.and_then(|o| String::from_utf8(o.stdout).ok())
		.map(|s| {
			s.split_whitespace().skip(1).map(PathBuf::from).collect()
		})
		.unwrap_or_default()
}

pub fn pathof_cmd(
	command: &str,
) -> Result<Vec<PathBuf>, CommandError> {
	let mut paths = HashSet::new();

	// Try whereismybin
	if let Some(path) = whereismybin(command) {
		paths.insert(path);
	}

	// Try which
	if let Ok(path) = which(command) {
		paths.insert(path);
	}

	// Try fallback search
	paths.extend(fallback_search(command));

	// If no paths found, return an error
	if paths.is_empty() {
		Err(CommandError::CommandNotFound(command.to_string()))
	} else {
		Ok(paths.into_iter().collect())
	}
}
