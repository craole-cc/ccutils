use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
	#[error("Failure to locate command: {0}")]
	CommandNotFound(String),

	#[error("Failed to get command path: {0}")]
	PathError(#[from] PathError),
}

#[derive(Error, Debug)]
pub enum PathError {
	#[error("I/O error: {0}")]
	IOError(#[from] io::Error),

	#[error("Error from `which`: {0}")]
	WhichError(#[from] which::Error),

	#[error("Invalid path: {0}")]
	InvalidPath(String),

	#[error("Permission denied for path: {0}")]
	PermissionDenied(String),
}
