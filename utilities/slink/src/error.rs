use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum SymlinkError {
    #[error("Source path does not exist: {0}")]
    SourceNotFound(PathBuf),
    #[error("Link path already exists: {0}")]
    LinkExists(PathBuf),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not determine config directory")]
    NoConfigDir,
    #[error("Could not determine source file name")]
    NoSourceFileName,
    #[error("Glob pattern error: {0}")]
    GlobPattern(#[from] glob::PatternError),
    #[error("No files matched the given pattern")]
    NoMatchingFiles,
    #[error("User aborted the operation")]
    UserAbort,
    #[error("IO error: {0}")]
    IoError(String),
    #[error("Path error: {0}")]
    PathError(String),
    #[error("Insufficient privileges to create symlink. Try running the program as administrator.")]
    InsufficientPrivileges,
}