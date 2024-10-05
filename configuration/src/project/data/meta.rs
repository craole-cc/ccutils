use anyhow::{anyhow, Result};
use std::fmt::{self, Display, Formatter};
use tword_util::string::capitalize_words;

use super::Paths;

#[derive(Debug, Clone)]
pub struct Meta {
	pub name: String,
	pub pretty_name: String,
}

impl Meta {
	pub fn init() -> Self {
		let mut meta = Meta {
			name: String::new(),
			pretty_name: String::new(),
		};

		meta.get_name().expect("Failed to get project name");
		meta.get_pretty_name().expect("Failed to get pretty name");

		meta
	}

	fn get_name(&mut self) -> Result<()> {
		let project_path = Paths::init().root_path;
		let project_name =
			project_path
				.file_name()
				.ok_or_else(|| {
					anyhow!("Failed to get project name from the root path")
				})?
				.to_string_lossy()
				.into_owned();

		self.name = project_name.to_string();
		Ok(())
	}

	fn get_pretty_name(&mut self) -> Result<()> {
		let name = &self.name;
		let pretty_name = capitalize_words(name.replace("_", " "));
		self.pretty_name = pretty_name;
		Ok(())
	}
}

impl Display for Meta {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		writeln!(f, "\tInfo:")?;
		writeln!(f, "\t  Name: {:#?}", self.name)?;
		writeln!(f, "\t  Name [Pretty]: {:#?}", self.pretty_name)
	}
}
