use crate::ConfigTrait;
use anyhow::{anyhow, Result};
use std::{
	env::var_os,
	fmt::{self, Display, Formatter},
	path::PathBuf,
	sync::Arc,
};
use tword_util::{path::dir_of, string::capitalize_words};

#[derive(Debug)]
pub struct Config {
	pub name: String,
	pub pretty_name: String,
	pub root_path: PathBuf,
	pub assets_path: PathBuf,
}

impl Display for Config {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		writeln!(f, "\tInfo:")?;
		writeln!(f, "\t  Name: {}", self.name)?;
		writeln!(f, "\t  Pretty Name: {}", self.pretty_name)?;
		writeln!(f, "\tPaths:")?;
		writeln!(f, "\t  Root Path: {:#?}", self.root_path)?;
		writeln!(f, "\t  Assets Path: {:#?}", self.assets_path)?;
		Ok(())
	}
}

impl ConfigTrait for Config {
	const NAME: &'static str = "Project";
	async fn load_data() -> Result<Self> {
		let root_path = Self::get_root_path()?;
		let name = Self::get_name()?;
		let pretty_name = Self::get_pretty_name()?;
		let assets_path = Self::get_assets_path()?;

		Ok(Self {
			name,
			pretty_name,
			root_path,
			assets_path,
		})
	}
}

impl Config {
	pub fn get_root_path() -> Result<PathBuf> {
		let expected_children = ["Cargo.lock", ".git", "flake.nix"];

		match dir_of(expected_children) {
			Ok(path_via_children) => Ok(path_via_children),
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

				Ok(path_via_env)
			}
		}
	}

	pub fn get_name() -> Result<String> {
		let project_name =
			Self::get_root_path()?
				.file_name()
				.ok_or_else(|| {
					anyhow!("Failed to get project name from the root path")
				})?
				.to_string_lossy()
				.into_owned();

		Ok(project_name)
	}

	pub fn get_pretty_name() -> Result<String> {
		let name = Self::get_name()?;
		let pretty_name = capitalize_words(name.replace("_", " "));
		Ok(pretty_name)
	}

	pub fn get_assets_path() -> Result<PathBuf> {
		let root_path = Self::get_root_path()?;
		let assets_path = root_path.join("assets");
		Ok(assets_path)
	}
}
