use crate::{Decimal, error::decimal::Error};

pub trait ToDecimal {
  /// Convert the value into a Decimal
  fn to_decimal(&self) -> Result<Decimal, Error<'static>>;
}

// Implement for common types
impl<T: ToString + ?Sized + std::fmt::Display> ToDecimal for T {
  fn to_decimal(&self) -> Result<Decimal, Error<'static>> {
    Decimal::parse(self)
  }
}

// Example usage in tests:
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_to_decimal_trait() {
    // String
    assert!("123.45".to_decimal().is_ok());
    assert!("not_a_number".to_decimal().is_err());

    // &str
    assert!(("123.45").to_decimal().is_ok());

    // Numbers
    assert!((123.45).to_decimal().is_ok());
    assert!((123_i64).to_decimal().is_ok());

    // From String
    assert!(String::from("123.45").to_decimal().is_ok());
  }
}
