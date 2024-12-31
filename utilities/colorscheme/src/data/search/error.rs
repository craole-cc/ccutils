#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Search reached root directory")]
    ReachedRoot,
    #[error("No valid paths to search")]
    NoValidPaths,
    #[error("No results found")]
    NoResults,
    #[error("Directory error: {0}")]
    Directory(String),
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
}
