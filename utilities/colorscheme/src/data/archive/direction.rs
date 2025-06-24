use super::PathType;
use logline::debug;
use std::path::{Path, PathBuf};
use std::{fs, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DirectionError {
	#[error("IO error: {0}")]
	Io(#[from] std::io::Error),
	#[error("No results found")]
	NoResults,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
	Up,
	Down,
	#[default]
	Both,
}

impl From<&str> for Direction {
	fn from(s: &str) -> Self {
		match s.to_lowercase().as_str() {
			"up" => Direction::Up,
			"down" => Direction::Down,
			"both" => Direction::Both,
			_ => Direction::Both, // Default case or handle error as needed
		}
	}
}

impl Direction {
	pub fn search<F>(
		&self,
		start: &Path,
		condition: F,
	) -> Result<Vec<PathBuf>, DirectionError>
	where
		F: Fn(&Path) -> bool + Clone,
	{
		match self {
			Direction::Up => self.search_up(start, condition),
			Direction::Down => self.search_down(start, condition),
			Direction::Both => {
				let mut results =
					self.search_down(start, condition.clone())?;
				if let Ok(up_results) =
					self.search_up(start, &condition)
				{
					results.extend(up_results);
				}
				if results.is_empty() {
					Err(DirectionError::NoResults)
				} else {
					Ok(results)
				}
			}
		}
	}

	fn search_up<F>(
		&self,
		start: &Path,
		condition: F,
	) -> Result<Vec<PathBuf>, DirectionError>
	where
		F: Fn(&Path) -> bool,
	{
		let mut current = start.to_path_buf();
		let mut results = Vec::new();

		loop {
			debug!("Checking directory: {:?}", current);

			// Get directory entries
			let entries = fs::read_dir(&current)?;

			for entry in entries {
				let entry = entry?; // Unwrap each DirEntry
				let path = entry.path();
				if condition(&path) {
					results.push(path);
				}
			}

			if let Some(parent) = current.parent() {
				current = parent.to_path_buf();
			} else {
				debug!("Reached root directory");
				break;
			}
		}

		if results.is_empty() {
			Err(DirectionError::NoResults)
		} else {
			Ok(results)
		}
	}

	fn search_down<F>(
		&self,
		start: &Path,
		condition: F,
	) -> Result<Vec<PathBuf>, DirectionError>
	where
		F: Fn(&Path) -> bool,
	{
		let mut results = Vec::new();
		self.search_down_recursive(start, &condition, &mut results)?;

		if results.is_empty() {
			Err(DirectionError::NoResults)
		} else {
			Ok(results)
		}
	}

	fn search_down_recursive<F>(
		&self,
		dir: &Path,
		condition: &F,
		results: &mut Vec<PathBuf>,
	) -> Result<(), DirectionError>
	where
		F: Fn(&Path) -> bool,
	{
		debug!("Searching directory: {:?}", dir);

		for entry in fs::read_dir(dir)? {
			let entry = entry?;
			let path = entry.path();

			if condition(&path) {
				results.push(path.clone());
			}

			if path.is_dir() {
				self.search_down_recursive(
					&path, condition, results,
				)?;
			}
		}

		Ok(())
	}
}

/// Helper function to create a condition function that checks if a path matches a given `PathType`.
pub fn create_path_type_condition(
	path_type: &PathType,
) -> impl Fn(&Path) -> bool + '_ {
	move |path| path_type.matches(path).unwrap_or(false)
}
