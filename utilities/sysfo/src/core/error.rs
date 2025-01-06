use thiserror::Error;
use super::process;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Process error: {0}")]
    Process(#[from] process::Error),
}
