use thiserror::Error;

/// Custom error type for process-related operations.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Current or specified process could not be found")]
    ProcessNotFound,
    #[error("Parent process could not be found")]
    ParentNotFound,
}
