use anyhow::{anyhow, Result};
use std::{
	env::var_os,
	fmt::{self, Display, Formatter},
	path::PathBuf,
};
use tword_util::path::dir_of;

#[derive(Debug, Clone)]
pub struct Paths {
	pub root_path: PathBuf,
	pub assets_path: PathBuf,
}

impl Paths {
	// Initialize the struct by setting root_path and assets_path
	pub fn init() -> Self {
		let mut paths = Paths {
			root_path: PathBuf::new(),
			assets_path: PathBuf::new(),
		};

		// Set root_path
		paths.get_root_path().expect("Failed to get root path");

		// Set assets_path based on root_path
		paths.get_assets_path().expect("Failed to get assets path");

		paths
	}

	fn get_root_path(&mut self) -> Result<()> {
		let expected_children = ["Cargo.lock", ".git", "flake.nix"];

		match dir_of(expected_children) {
			Ok(path_via_children) => {
				self.root_path = path_via_children;
				Ok(())
			}
			Err(_) => {
				let path_via_env = var_os("CARGO_MANIFEST_DIR")
					.map(PathBuf::from)
					.ok_or_else(|| {
						anyhow!("Environment variable CARGO_MANIFEST_DIR not found.")
					})?;

				tracing::trace!(
                    "No directory found that contains any of {:?}\n\tFalling back to CARGO_MANIFEST_DIR: {:?}.",
                    expected_children,
                    path_via_env
                );

				self.root_path = path_via_env;
				Ok(())
			}
		}
	}

	// This is private since init() will handle calling it
	fn get_assets_path(&mut self) -> Result<()> {
		self.assets_path = self.root_path.join("assets");
		Ok(())
	}
}

impl Display for Paths {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		writeln!(f, "\tPaths:")?;
		writeln!(f, "\t  Root Path: {:#?}", self.root_path)?;
		writeln!(f, "\t  Assets Path: {:#?}", self.assets_path)
	}
}
