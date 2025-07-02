// use super::{parse_rust_decimal, DecimalNumeral, Error};
use crate::decimal::{Error, Numeral, parse_big_decimal, parse_rust_decimal};

impl Numeral {
  pub fn from<T: ToString>(input: T) -> Result<Self, Error<'static>> {
    Self::parse(input)
  }

  /// Parse number with fallback to BigDecimal if needed
  pub fn parse<T: ToString>(input: T) -> Result<Self, Error<'static>> {
    let input_str = input.to_string();

    // Try parsing as rust_decimal first
    match parse_rust_decimal(&input_str) {
      Ok(decimal) => Ok(Self::Small(decimal)),
      #[cfg(feature = "big-decimal")]
      Err(Error::Decimal(_)) => {
        logline::warn!("Falling back to BigDecimal");
        parse_big_decimal(&input_str).map(Self::Large)
      }
      #[cfg(not(feature = "big-decimal"))]
      Err(Error::Decimal(_)) =>
        Err(Error::Decimal("BigDecimal feature not enabled".into())),
      Err(err) => Err(err) // Propagate other errors
    }
  }
}
