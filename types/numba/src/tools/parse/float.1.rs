use super::Error;
use bigdecimal::{BigDecimal, ParseBigDecimalError, Zero};
use num::BigInt;
use std::str::FromStr;

/// Parses a string into a `BigDecimal`.
///
/// This function cleans the input string by removing leading/trailing
/// whitespace and commas, then attempts to parse it as a `BigDecimal`. It
/// handles both integer and fractional parts of the number.
///
/// # Arguments
///
/// * `float` - A type that can be converted into a string slice (e.g., &str or
///   String).
///
/// # Returns
///
/// * `Ok(BigDecimal)` if the parsing is successful.
/// * `Err(Error)` if the input string is empty or if parsing fails.
///
/// # Errors
///
/// This function can return several errors:
/// - `Error::EmptyString`: If the cleaned string is empty.
/// - `Error::TooManyDecimalPoints`: If more than one decimal point is found in
///   the input.
/// - `Error::InvalidBigInt`: If parsing fails while converting to BigInt.
/// - `Error::InvalidFractional`: If there are issues with parsing fractional
///   parts.
pub fn float<T: AsRef<str>>(float: T) -> Result<BigDecimal, Error<'static>> {
  // Clean the string by removing leading/trailing whitespace and commas
  let cleaned = float.as_ref().trim().replace(",", "");

  // Check if the cleaned string is empty
  if cleaned.is_empty() {
    return Ok(BigDecimal::zero());
  }

  // Split the string into integer and fractional parts
  let parts: Vec<&str> = cleaned.split('.').collect();

  // Ensure there are no more than two parts (integer and fractional)
  if parts.len() > 2 {
    return Err(Error::TooManyDecimalPoints(parts.len()));
  }

  // Check if value is zero
  if cleaned == "0" {
    return Ok(BigDecimal::zero());
  }

  // Parse integer part
  let digits = if cleaned.starts_with(".") {
    BigInt::zero()
  } else {
    match BigInt::from_str(parts[0]) {
      Ok(digits) => digits,
      Err(err) => return Err(Error::InvalidBigInt(err, cleaned.into()))
    }
  };

  // Determine scale (number of digits after decimal point)
  const MAX_SCALE: usize = isize::MAX as usize;
  let scale = match parts.get(1) {
    Some(fractional_part) if !fractional_part.is_empty() => {
      // Check for negative fractional part
      if fractional_part.starts_with('-') {
        return Err(Error::FractionalNegative);
      }

      // Check length of fractional part against max size
      let provided_len = fractional_part.len();
      if provided_len > MAX_SCALE {
        return Err(Error::fractional_overflow(
          fractional_part.to_string(),
          MAX_SCALE
        ));
      }

      let fractional_value =
        usize::from_str(fractional_part).map_err(|err| {
          Error::InvalidFractional(err, fractional_part.to_string().into())
        })?;

      fractional_value as i64 // Return scale as i64 for BigDecimal creation.
    }
    _ => 0 // No fractional part or empty; scale is 0.
  };

  // Return parsed BigDecimal with digits and scale.
  Ok(BigDecimal::new(digits, scale))
}

/// Parses a string into a `BigDecimal` after converting the input to a string.
///
/// This function accepts any type that can be converted into a string,
/// cleans the input by removing leading/trailing whitespace and commas,
/// and attempts to parse it as a `BigDecimal`.
///
/// # Arguments
///
/// * `input` - Any type that implements `ToString` (e.g., &str, String, i32,
///   f64, etc.).
///
/// # Returns
///
/// * `Ok(BigDecimal)` if the parsing is successful.
/// * `Err(Error)` if the input string is empty or if parsing fails.
pub fn big_decimal<T: ToString>(
  input: T
) -> Result<BigDecimal, Error<'static>> {
  // Convert the input to a string and clean it
  let cleaned = input.to_string().trim().replace(",", "");

  // Check if the cleaned string is empty
  if cleaned.is_empty() {
    return Ok(BigDecimal::zero());
  }

  // Attempt to parse the cleaned string into a BigDecimal
  match BigDecimal::from_str(&cleaned) {
    Ok(value) => Ok(value),
    Err(err) => match err {
      ParseBigDecimalError::Empty => Ok(BigDecimal::zero()),
      ParseBigDecimalError::ParseDecimal(err) => {
        // Try parsing as scientific notation first
        if cleaned.contains(['e', 'E']) {
          Err(Error::InvalidScientificNotation(err, cleaned.into()))
        } else {
          Err(Error::InvalidBigDecimal(
            ParseBigDecimalError::ParseDecimal(err),
            cleaned.into()
          ))
        }
      }
      ParseBigDecimalError::ParseInt(err) => Err(Error::InvalidMantissa(
        ParseBigDecimalError::ParseInt(err),
        cleaned.into()
      )),
      ParseBigDecimalError::ParseBigInt(err) =>
        Err(Error::InvalidBigInt(err, cleaned.into())),
      ParseBigDecimalError::Other(err) => Err(Error::InvalidBigDecimal(
        ParseBigDecimalError::Other(err),
        cleaned.into()
      ))
    }
  }
}
