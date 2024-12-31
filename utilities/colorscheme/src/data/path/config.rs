use std::fs::{self, Metadata};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PathTypeError {
	#[error("IO error: {0}")]
	Io(#[from] std::io::Error),
	#[error("Failed to get file type")]
	FileTypeUnavailable,
	#[error("Failed to get metadata")]
	MetadataUnavailable,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Path {
	Executable,
	Empty,
	BlockDevice,
	CharacterDevice,
	Directory,
	Pipe,
	File,
	SymbolicLink,
	Socket,
	#[default]
	Any,
}

impl From<&str> for PathType {
	fn from(s: &str) -> Self {
		match s.to_lowercase().as_str() {
			"b" | "block" | "block-device" => Self::BlockDevice,
			"c" | "char" | "char-device" => Self::CharacterDevice,
			"d" | "dir" | "directory" => Self::Directory,
			"p" | "pipe" => Self::Pipe,
			"f" | "file" => Self::File,
			"l" | "symlink" | "symbolic" => Self::SymbolicLink,
			"s" | "socket" => Self::Socket,
			"x" | "executable" => Self::Executable,
			"e" | "empty" => Self::Empty,
			_ => Self::Any,
		}
	}
}

impl PathType {
	pub fn matches(
		&self,
		path: &Path,
	) -> Result<bool, PathTypeError> {
		match self {
			Self::Any => Ok(true),
			Self::Empty => Ok(path.metadata()?.len() == 0),
			Self::Executable => {
				Self::is_executable(&path.metadata()?)
			}
			_ => {
				let metadata = path.metadata()?;
				let file_type = metadata.file_type();
				Ok(match self {
					Self::BlockDevice => {
						Self::is_block_device(&metadata)
					}
					Self::CharacterDevice => {
						Self::is_char_device(&metadata)
					}
					Self::Directory => file_type.is_dir(),
					Self::Pipe => Self::is_fifo(&metadata),
					Self::File => file_type.is_file(),
					Self::SymbolicLink => file_type.is_symlink(),
					Self::Socket => Self::is_socket(&metadata),
					_ => unreachable!(),
				})
			}
		}
	}

	pub fn get_type(path: &Path) -> Result<Self, PathTypeError> {
		let metadata = fs::metadata(path)?;
		let file_type = metadata.file_type();

		if file_type.is_dir() {
			Ok(Self::Directory)
		} else if file_type.is_file() {
			if Self::is_executable(&metadata)? {
				Ok(Self::Executable)
			} else if metadata.len() == 0 {
				Ok(Self::Empty)
			} else {
				Ok(Self::File)
			}
		} else if file_type.is_symlink() {
			Ok(Self::SymbolicLink)
		} else if Self::is_block_device(&metadata) {
			Ok(Self::BlockDevice)
		} else if Self::is_char_device(&metadata) {
			Ok(Self::CharacterDevice)
		} else if Self::is_fifo(&metadata) {
			Ok(Self::Pipe)
		} else if Self::is_socket(&metadata) {
			Ok(Self::Socket)
		} else {
			Ok(Self::Any)
		}
	}

	fn is_executable(
		metadata: &Metadata,
	) -> Result<bool, PathTypeError> {
		#[cfg(unix)]
		{
			use std::os::unix::fs::PermissionsExt;
			Ok(metadata.permissions().mode() & 0o111 != 0)
		}
		#[cfg(not(unix))]
		{
			Ok(true) // On non-unix systems, we can't easily check for executable permissions
		}
	}

	#[cfg(unix)]
	fn is_block_device(metadata: &Metadata) -> bool {
		use std::os::unix::fs::FileTypeExt;
		metadata.file_type().is_block_device()
	}

	#[cfg(not(unix))]
	fn is_block_device(_metadata: &Metadata) -> bool {
		false // Block devices are not supported on non-unix systems
	}

	#[cfg(unix)]
	fn is_char_device(metadata: &Metadata) -> bool {
		use std::os::unix::fs::FileTypeExt;
		metadata.file_type().is_char_device()
	}

	#[cfg(not(unix))]
	fn is_char_device(_metadata: &Metadata) -> bool {
		false // Character devices are not supported on non-unix systems
	}

	#[cfg(unix)]
	fn is_fifo(metadata: &Metadata) -> bool {
		use std::os::unix::fs::FileTypeExt;
		metadata.file_type().is_fifo()
	}

	#[cfg(not(unix))]
	fn is_fifo(_metadata: &Metadata) -> bool {
		false // FIFOs are not supported on non-unix systems
	}

	#[cfg(unix)]
	fn is_socket(metadata: &Metadata) -> bool {
		use std::os::unix::fs::FileTypeExt;
		metadata.file_type().is_socket()
	}

	#[cfg(not(unix))]
	fn is_socket(_metadata: &Metadata) -> bool {
		false // Sockets are not supported on non-unix systems
	}
}
