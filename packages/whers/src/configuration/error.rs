use erks::{IOError, ThisError};

#[derive(ThisError, Debug)]
pub enum Error {
  #[error("Failure to locate command: {0}")]
  CommandNotFound(String)
}
