use crate::types::Decimal;
use std::fmt::{Display, Formatter, Result};

impl Display for Decimal {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
      Self::Small(rust_decimal) => {
        write!(f, "{}", rust_decimal)
      }
      #[cfg(feature = "big-decimal")]
      Self::Large(big_decimal) => write!(f, "{}", big_decimal)
    }
  }
}
