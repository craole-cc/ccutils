//! Global environment management and static initialization.

mod env;
mod kind;

#[cfg(feature = "macros")]
mod macros;

pub use {
  env::*,
  kind::*,
};
