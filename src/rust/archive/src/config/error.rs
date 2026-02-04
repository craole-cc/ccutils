use super::_prelude::*;

pub type Result<T> = StdResult<T, Error>;

pub mod prelude {
  pub use super::{
    Error as EnvError,
    Result as EnvResult,
  };
}

#[derive(Debug, ThisError, Diagnostic)]
pub enum Error {
  #[error("{0}")]
  #[diagnostic(
    code(env::generic),
    help("Consider converting this to a more descriptive error message.")
  )]
  Generic(String),
}
