use erks::{ThisError, IOError};

#[derive(ThisError, Debug)]
pub enum Error {
	#[error("Failure to locate command: {0}")]
	CommandNotFound(String),
	

}
