use logline::{debug, error};
use regex::Regex;
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::data::directory::DirectoryError;

use super::{
	direction::create_path_type_condition, Direction, Directory,
	PathType, Sort,
};

#[derive(Error, Debug)]
pub enum SearchError {
	#[error("IO error: {0}")]
	Io(#[from] std::io::Error),
	#[error("Search reached root directory")]
	ReachedRoot,
	#[error("No valid paths to search")]
	NoValidPaths,
	#[error("No results found")]
	NoResults,
	#[error("Directory error: {0}")]
	DirectoryError(String),
	#[error("Regex error: {0}")]
	RegexError(#[from] regex::Error),
}

#[derive(Default, Debug)]
pub struct Search<P: AsRef<str> + Default> {
	pattern: P,
	path_type: PathType,
	directory: Directory,
	direction: Direction,
	depth: usize,
	limit: usize,
	sort: Sort,
	case_sensitive: bool,
	include_hidden: bool,
	exclude: Vec<String>,
	ignore_files: Vec<String>,
}

impl<P: AsRef<str> + Default> Search<P> {
	pub fn new(pattern: P) -> Self {
		Self {
			pattern,
			..Default::default()
		}
	}

	pub fn with_pattern(mut self, pattern: P) -> Self {
		self.pattern = pattern;
		self
	}

	pub fn with_type<T: Into<PathType>>(
		mut self,
		path_type: T,
	) -> Self {
		self.path_type = path_type.into();
		self
	}

	pub fn with_directory<D: Into<Directory>>(
		mut self,
		directory: D,
	) -> Self {
		self.directory = directory.into();
		self
	}

	pub fn with_direction<D: Into<Direction>>(
		mut self,
		direction: D,
	) -> Self {
		self.direction = direction.into();
		self
	}

	pub fn with_depth(mut self, depth: usize) -> Self {
		self.depth = depth;
		self
	}

	pub fn with_limit(mut self, limit: usize) -> Self {
		self.limit = limit;
		self
	}

	pub fn with_sort<S: Into<Sort>>(mut self, sort: S) -> Self {
		self.sort = sort.into();
		self
	}

	pub fn case_sensitive(mut self, case_sensitive: bool) -> Self {
		self.case_sensitive = case_sensitive;
		self
	}

	pub fn include_hidden(mut self, include_hidden: bool) -> Self {
		self.include_hidden = include_hidden;
		self
	}

	pub fn exclude(mut self, exclude: Vec<String>) -> Self {
		self.exclude = exclude;
		self
	}

	pub fn ignore_files(mut self, ignore_files: Vec<String>) -> Self {
		self.ignore_files = ignore_files;
		self
	}

	pub fn execute(&self) -> Result<Vec<PathBuf>, SearchError> {
		debug!("Starting search execution");
		let start_paths = match self.directory.to_path_buf() {
			Ok(paths) => paths,
			Err(e) => {
				error!("Failed to get start paths: {:?}", e);
				return Err(SearchError::DirectoryError(e.to_string()));
			}
		};

		debug!("Start paths: {:?}", start_paths);

		if start_paths.is_empty() {
			error!("No valid start paths found");
			return Err(SearchError::NoValidPaths);
		}

		let regex = match self.create_regex() {
			Ok(r) => r,
			Err(e) => {
				error!("Failed to create regex: {:?}", e);
				return Err(e);
			}
		};

		let path_type_condition =
			create_path_type_condition(&self.path_type);

		let mut results = Vec::new();
		for start_path in start_paths {
			debug!("Searching in path: {:?}", start_path);
			let condition = |path: &Path| {
				self.matches_criteria(path, &regex)
					&& path_type_condition(path)
			};

			match self.direction.search(&start_path, condition) {
				Ok(mut paths) => {
					debug!(
						"Found {} results in {:?}",
						paths.len(),
						start_path
					);
					results.append(&mut paths);
				}
				Err(e) => {
					error!(
						"Error searching in {:?}: {:?}",
						start_path, e
					);
					// TODO: Optionally, decide whether to continue with other paths or return the error
				}
			}

			if self.limit > 0 && results.len() >= self.limit {
				results.truncate(self.limit);
				break;
			}
		}

		if results.is_empty() {
			error!("No results found");
			return Err(SearchError::NoResults);
		}

		if !self.sort.is_none() {
			self.sort.sort(&mut results);
		}

		Ok(results)
	}

	fn create_regex(&self) -> Result<Regex, SearchError> {
		let mut pattern = regex::escape(self.pattern.as_ref());
		if !self.case_sensitive {
			pattern = format!("(?i){}", pattern);
		}
		Regex::new(&pattern).map_err(|_| SearchError::ReachedRoot) // Use a more appropriate error type if available
	}

	fn matches_criteria(&self, path: &Path, regex: &Regex) -> bool {
		let file_name =
			path.file_name().and_then(|s| s.to_str()).unwrap_or("");

		if !self.include_hidden && file_name.starts_with('.') {
			return false;
		}

		if self
			.exclude
			.iter()
			.any(|p| path.to_str().map_or(false, |s| s.contains(p)))
		{
			return false;
		}

		if self.ignore_files.iter().any(|f| path.ends_with(f)) {
			return false;
		}

		regex.is_match(file_name)
	}
}
