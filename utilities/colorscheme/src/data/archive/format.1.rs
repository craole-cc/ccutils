use super::Location;
use std::fmt;

#[derive(Default)]
pub enum Format {
	#[default]
	Plain,
	Fetch,
	Verbose,
}

impl fmt::Display for Location {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match f.alternate() {
			true => match self {
				Self::Executable(path) => {
					write!(f, "Executable: {}", path.display())
				}
				Self::ShellBuiltin => write!(f, "Shell Builtin"),
				Self::ShellAlias(info) => {
					write!(f, "Shell Alias: {}", info)
				}
				Self::ShellFunction(info) => {
					write!(f, "Shell Function: {}", info)
				}
				Self::File(path) => {
					write!(f, "File: {}", path.display())
				}
				Self::Directory(path) => {
					write!(f, "Directory: {}", path.display())
				}
			},
			false => match self {
				Self::Executable(path) => {
					write!(f, "{}", path.display())
				}
				Self::ShellBuiltin => write!(f, "builtin"),
				Self::ShellAlias(info) => write!(f, "alias:{}", info),
				Self::ShellFunction(info) => {
					write!(f, "function:{}", info)
				}
				Self::File(path) => {
					write!(f, "file:{}", path.display())
				}
				Self::Directory(path) => {
					write!(f, "dir:{}", path.display())
				}
			},
		}
	}
}
