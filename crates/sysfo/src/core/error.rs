use super::process;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
  #[error("Process error: {0}")]
  Process(#[from] process::Error)
}
