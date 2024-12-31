use directories::{BaseDirs, UserDirs};
use std::{
	env, io, path::{Path, PathBuf}
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DirectoryError {
	#[error("Failed to get current directory: {0}")]
	CurrentDir(#[from] io::Error),
	#[error("Failed to get environment variable: {0}")]
	EnvVar(#[from] env::VarError),
	#[error("Base directories not available")]
	BaseDirsNotAvailable,
	#[error("User directories not available")]
	UserDirsNotAvailable,
	#[error("Failed to get directory: {0}")]
	Simple(String),
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Directory {
	Executable,
	#[default]
	Current,
	Parent,
	Root,
	Config,
	Data,
	Cache,
	Home,
	Fonts,
	Downloads,
	Documents,
	Path(PathBuf),
	Env(String),
}

impl From<&str> for Directory {
	fn from(s: &str) -> Self {
		match s.to_lowercase().as_str() {
			"executable" => Directory::Executable,
			"current" | "pwd" => Directory::Current,
			"parent" => Directory::Parent,
			"root" | "/" => Directory::Root,
			"config" | ".config" => Directory::Config,
			"data" | ".local" => Directory::Data,
			"cache" | ".cache" => Directory::Cache,
			"home" => Directory::Home,
			"fonts" => Directory::Fonts,
			"downloads" => Directory::Downloads,
			"documents" => Directory::Documents,
			_ if s.starts_with('/') => {
				Directory::Path(PathBuf::from(s))
			}
			_ if s.starts_with('$') => {
				Directory::Env(s[1..].to_string())
			}
			_ => Directory::Current,
		}
	}
}

impl Directory {
	pub fn to_path_buf(
		&self,
	) -> Result<Vec<PathBuf>, DirectoryError> {
		match self {
			Self::Executable => Ok(get_env_path("PATH")),
			Self::Current => Ok(vec![env::current_dir()?]),
			Self::Parent => Ok(vec![env::current_dir()?
				.parent()
				.unwrap_or_else(|| Path::new(""))
				.to_path_buf()]),
			Self::Root => Ok(get_root_path()?),
			Self::Config => {
				get_base_dir(|dirs| dirs.config_dir().to_path_buf())
			}
			Self::Data => {
				get_base_dir(|dirs| dirs.data_dir().to_path_buf())
			}
			Self::Cache => {
				get_base_dir(|dirs| dirs.cache_dir().to_path_buf())
			}
			Self::Home => {
				get_base_dir(|dirs| dirs.home_dir().to_path_buf())
			}
			Self::Downloads => get_user_dir(|dirs| {
				dirs.download_dir().map(|p| p.to_path_buf())
			}),
			Self::Documents => get_user_dir(|dirs| {
				dirs.document_dir().map(|p| p.to_path_buf())
			}),
			Self::Fonts => get_user_dir(|dirs| {
				dirs.font_dir().map(|p| p.to_path_buf())
			}),
			Self::Path(path) => Ok(vec![path.clone()]),
			Self::Env(var) => Ok(get_env_path(var)),
		}
	}
}

fn get_env_path(env: &str) -> Vec<PathBuf> {
	env::var_os(env)
		.map(|paths| env::split_paths(&paths).collect())
		.unwrap_or_default()
}

fn get_root_path() -> Result<Vec<PathBuf>, DirectoryError> {
	if cfg!(target_os = "windows") {
		Ok(vec![PathBuf::from(format!(
			"{}\\",
			env::var("SystemDrive")
				.unwrap_or_else(|_| "C:".to_string())
		))])
	} else {
		Ok(vec![PathBuf::from("/")])
	}
}

fn get_base_dir<F>(f: F) -> Result<Vec<PathBuf>, DirectoryError>
where
	F: FnOnce(&BaseDirs) -> PathBuf,
{
	BaseDirs::new()
		.map(|dirs| Ok(vec![f(&dirs)]))
		.unwrap_or(Err(DirectoryError::BaseDirsNotAvailable))
}

fn get_user_dir<F>(f: F) -> Result<Vec<PathBuf>, DirectoryError>
where
	F: FnOnce(&UserDirs) -> Option<PathBuf>,
{
	UserDirs::new()
		.map(|dirs| {
			Ok(f(&dirs)
				.or_else(|| Some(dirs.home_dir().to_path_buf()))
				.map(|path| vec![path])
				.unwrap_or_default())
		})
		.unwrap_or(Err(DirectoryError::UserDirsNotAvailable))
}
