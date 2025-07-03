use crate::decimal::{Error, Numeral};

pub trait ToNumeral {
  /// Convert the value into a Numeral
  fn to_numeral(&self) -> Result<Numeral, Error<'static>>;
}

// Implement for common types
impl<T: ToString + ?Sized + std::fmt::Display> ToNumeral for T {
  fn to_numeral(&self) -> Result<Numeral, Error<'static>> {
    Numeral::parse(self)
  }
}

// Example usage in tests:
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_to_numeral_trait() {
    // String
    assert!("123.45".to_numeral().is_ok());
    assert!("not_a_number".to_numeral().is_err());

    // &str
    assert!(("123.45").to_numeral().is_ok());

    // Numbers
    assert!((123.45).to_numeral().is_ok());
    assert!((123_i64).to_numeral().is_ok());

    // From String
    assert!(String::from("123.45").to_numeral().is_ok());
  }
}
