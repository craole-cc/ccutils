use std::env;
use std::path::PathBuf;

fn find_location(name: &str) -> Option<PathBuf> {
	if let Ok(path) = env::var("PATH") {
		for dir in path.split(':') {
			let full_path = PathBuf::from(dir).join(name);
			if full_path.exists() {
				return Some(full_path);
			}
		}
	}
	None
}

fn main() {
	logline::init();
	logline::info!("{}", "Tracing initialized via logline!");

	if let Some(location) = find_location("your_file_or_binary") {
		logline::info!("Location: {}", location.display());
	} else {
		logline::warn!("File or binary not found in PATH");
	}
}
