use super::error::Error;
use bigdecimal::{BigDecimal, ParseBigDecimalError, Zero};
use std::{borrow::Cow, num::ParseFloatError, str::FromStr};

/// A trait for types that can be converted to a string reference efficiently
pub trait ToStringRef {
  /// Converts the value to a string reference
  fn to_string_ref(&self) -> Cow<'_, str>;
}

// Implement for string slice - zero copy
impl ToStringRef for str {
  #[inline]
  fn to_string_ref(&self) -> Cow<'_, str> {
    Cow::Borrowed(self)
  }
}

// Implement for String - zero copy
impl ToStringRef for String {
  #[inline]
  fn to_string_ref(&self) -> Cow<'_, str> {
    Cow::Borrowed(self.as_str())
  }
}

// Implement for all numeric types using a blanket implementation
impl<T: std::fmt::Display> ToStringRef for T {
  #[inline]
  fn to_string_ref(&self) -> Cow<'_, str> {
    Cow::Owned(self.to_string())
  }
}

/// Parses any compatible type into a `BigDecimal`.
///
/// # Type Support
/// - Strings and string slices (&str, String)
/// - Integers (i8, i16, i32, i64, i128, isize)
/// - Unsigned integers (u8, u16, u32, u64, u128, usize)
/// - Floating point numbers (f32, f64)
/// - Any type implementing Display
///
/// # Examples
/// ```
/// let dec1 = big_decimal(123)?;            // From integer
/// let dec2 = big_decimal(123.45)?;         // From float
/// let dec3 = big_decimal("123.45")?;       // From string slice
/// let dec4 = big_decimal(&String::from("123.45"))?;  // From String
/// ```
#[inline]
pub fn big_decimal<T>(input: T) -> Result<BigDecimal, Error<'static>>
where
  T: ToStringRef
{
  let input_cow = input.to_string_ref();
  let trimmed = input_cow.trim();

  // Handle empty/whitespace input early
  if trimmed.is_empty() {
    return Ok(BigDecimal::zero());
  }

  // Only allocate a new string if we actually find commas
  let cleaned = if trimmed.contains(',') {
    Cow::Owned(trimmed.chars().filter(|&c| c != ',').collect())
  } else {
    match input_cow {
      Cow::Borrowed(s) => Cow::Borrowed(trimmed),
      Cow::Owned(_) => Cow::Owned(trimmed.to_owned())
    }
  };

  Ok(
    BigDecimal::from_str(&cleaned)
      .map_err(|err| match err {
        ParseBigDecimalError::Empty => {
          // This case is unreachable due to our empty check above
          Ok(BigDecimal::zero())
        }
        ParseBigDecimalError::ParseDecimal(float_err) =>
          if cleaned.contains(['e', 'E']) {
            Err(Error::InvalidScientificNotation(float_err, cleaned.into_owned().into()))
          } else {
            Err(Error::InvalidBigDecimal(
              ParseBigDecimalError::ParseDecimal(float_err),
              cleaned.into_owned().into()
            ))
          },
        ParseBigDecimalError::ParseInt(int_err) => Err(Error::InvalidMantissa(
          ParseBigDecimalError::ParseInt(int_err),
          cleaned.into_owned().into()
        )),
        ParseBigDecimalError::ParseBigInt(bigint_err) =>
          Err(Error::InvalidBigInt(bigint_err, cleaned.into_owned().into())),
        ParseBigDecimalError::Other(err_msg) => Err(Error::InvalidBigDecimal(
          ParseBigDecimalError::Other(err_msg),
          cleaned.into_owned().into()
        ))
      })
      .unwrap_or_else(|err| err)
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_various_types() {
    // Integer types
    assert_eq!(big_decimal(123i32).unwrap().to_string(), "123");
    assert_eq!(big_decimal(456i64).unwrap().to_string(), "456");
    assert_eq!(big_decimal(789usize).unwrap().to_string(), "789");

    // Floating point types
    assert_eq!(big_decimal(123.45f64).unwrap().to_string(), "123.45");
    assert_eq!(big_decimal(456.78f32).unwrap().to_string(), "456.78");

    // String types
    assert_eq!(big_decimal("123.45").unwrap().to_string(), "123.45");
    assert_eq!(big_decimal(&String::from("456.78")).unwrap().to_string(), "456.78");

    // With commas
    assert_eq!(big_decimal("1,234.56").unwrap().to_string(), "1234.56");
  }

  #[test]
  fn test_edge_cases() {
    // Empty and whitespace
    assert_eq!(big_decimal("").unwrap().to_string(), "0");
    assert_eq!(big_decimal(" ").unwrap().to_string(), "0");

    // Max values
    assert!(big_decimal(i64::MAX).is_ok());
    assert!(big_decimal(u64::MAX).is_ok());

    // Scientific notation
    assert!(matches!(
      big_decimal("1.23e+invalid"),
      Err(Error::InvalidScientificNotation(_, _))
    ));
  }

  #[test]
  fn test_numeric_limits() {
    // Test various integer types
    assert!(big_decimal(i8::MAX).is_ok());
    assert!(big_decimal(i16::MAX).is_ok());
    assert!(big_decimal(i32::MAX).is_ok());
    assert!(big_decimal(i64::MAX).is_ok());
    assert!(big_decimal(i128::MAX).is_ok());

    assert!(big_decimal(u8::MAX).is_ok());
    assert!(big_decimal(u16::MAX).is_ok());
    assert!(big_decimal(u32::MAX).is_ok());
    assert!(big_decimal(u64::MAX).is_ok());
    assert!(big_decimal(u128::MAX).is_ok());
  }
}
