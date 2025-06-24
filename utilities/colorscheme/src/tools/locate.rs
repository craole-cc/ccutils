use std::{
	env, fs,
	path::{Path, PathBuf},
};


pub fn locate_executables(command: &str) -> Vec<PathBuf> {
	getenv_path()
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

pub fn getenv_path() -> Vec<PathBuf> {
	env::var_os("PATH")
		.map(|paths| env::split_paths(&paths).collect())
		.unwrap_or_default()
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
	path.extension().map_or(false, |ext| {
		ext.eq_ignore_ascii_case("exe")
			|| ext.eq_ignore_ascii_case("cmd")
			|| ext.eq_ignore_ascii_case("bat")
	})
}

fn locate_dirs(name: &str, dir: &Path) -> Vec<PathBuf> {
	let mut results = Vec::new();

	if let Ok(entries) = fs::read_dir(dir) {
		for entry in entries.filter_map(Result::ok) {
			let path = entry.path();
			if path.is_dir()
				&& path.file_name().and_then(|n| n.to_str())
					== Some(name)
			{
				results.push(path);
			}
		}
	}

	results
}

fn locate_files(name: &str, dir: &Path) -> Vec<PathBuf> {
	let mut results = Vec::new();

	if let Ok(entries) = fs::read_dir(dir) {
		for entry in entries.filter_map(Result::ok) {
			let path = entry.path();
			if path.is_file()
				&& path.file_name().and_then(|n| n.to_str())
					== Some(name)
			{
				results.push(path);
			}
		}
	}

	results
}

pub fn locate_exes(name: &str) -> Vec<PathBuf> {
	let mut results = Vec::new();

	// Try whereismybin
	if let Some(path) = whereismybin(name) {
		if is_executable(&path) {
			results.push(path);
		}
	}

	// Try which
	if let Ok(path) = which(name) {
		if is_executable(&path) {
			results.push(path);
		}
	}

	// Search in PATH
	results.extend(
		search_in_path(name)
			.into_iter()
			.filter(|path| is_executable(path)),
	);

	// Search downward and upward
	let current_dir =
		env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
	results.extend(search_files_in_dirs(name, &current_dir));
	results
}

pub fn search_files_and_directories(name: &str) -> Vec<PathBuf> {
	let current_dir =
		env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
	let mut results = Vec::new();

	results.extend(locate_dirs(name, &current_dir));
	results.extend(locate_files(name, &current_dir));

	results
}
