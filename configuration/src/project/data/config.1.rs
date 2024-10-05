use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use std::{
	env::var_os,
	fmt::{self, Display, Formatter},
	path::PathBuf,
	sync::Arc,
};
use tword_util::{path::dir_of, string::capitalize_words};

// #[derive(Debug, Clone)]
// pub struct Project {
// 	pub root_path: PathBuf,
// 	pub name: String,
// 	pub pretty_name: String,
// 	pub assets_path: PathBuf,
// }

pub struct Config {
	pub paths: Paths,
	pub names: Names,
}

pub struct Paths {
	pub root_path: PathBuf,
	pub assets_path: PathBuf,
}

pub struct Names {
	pub name: String,
	pub pretty_name: String,
}

static CONFIG: Lazy<Arc<Result<Config>>> =
	Lazy::new(|| Arc::new(Config::load()));

impl Config {
	pub fn init() -> &'static Arc<Result<Config>> {
		&CONFIG
	}

	fn load() -> Result<Self> {
		let root_path = Self::get_root_path()?;
		let name = Self::get_name()?;
		let pretty_name = Self::get_pretty_name()?;
		let assets_path = Self::get_assets_path()?;

		Ok(Self {
			paths: Paths {
				root_path,
				assets_path,
			},
			names: Names { name, pretty_name },
			// root_path,
			// name,
			// pretty_name,
			// assets_path,
		})
	}

	fn get_root_path() -> Result<PathBuf> {
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

// impl Display for Config {
// 	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
// 		writeln!(f, "\tInfo:")?;
// 		writeln!(f, "  Name: {}", self.name)?;
// 		writeln!(f, "  Pretty Name: {}", self.pretty_name)?;
// 		writeln!(f, "\tPaths:")?;
// 		writeln!(f, "\t  Root Path: {}", self.root_path.display())?;
// 		writeln!(f, "  Assets Path: {}", self.assets_path.display())
// 	}
// }

impl Display for Paths {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		writeln!(f, "\tPaths:")?;
		writeln!(f, "\t  Root Path: {}", self.root_path.display())?;
		writeln!(f, "  Assets Path: {}", self.assets_path.display())
	}
}

impl Display for Names {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		writeln!(f, "\tNames:")?;
		writeln!(f, "  Name: {}", self.name)?;
		writeln!(f, "  Pretty Name: {}", self.pretty_name)
	}
}
