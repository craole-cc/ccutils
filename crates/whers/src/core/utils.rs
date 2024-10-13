use super::error::{CommandError, PathError};
use std::path::PathBuf;
use whereismybin::whereismybin;
use which::which;

pub fn pathof_cmd(command: &str) -> Result<PathBuf, CommandError> {
	let via_which = which(command);
	let via_wherismybin = whereismybin(command);

	// Check if both failed (via_wherismybin is None and via_which is an Err)
	if via_wherismybin.is_none() && via_which.is_err() {
		return Err(CommandError::CommandNotFound(
			command.to_string(),
		));
	}

	// Handle both cases, prioritize via_wherismybin if it exists
	match (via_wherismybin, via_which) {
		(Some(path), _) => Ok(path), // Return path from whereismybin
		(None, Ok(path)) => Ok(path), // Return path from which if whereismybin is None
		(None, Err(e)) => {
			Err(CommandError::PathError(PathError::WhichError(e)))
		} // Convert which::Error to PathError
	}
}
