use super::Error;
use bigdecimal::{BigDecimal, Zero};
use num::BigInt;
use std::str::FromStr;

/// Parses a string into a `BigInt`.
///
/// This function cleans the input string by removing leading/trailing
/// whitespace and commas, then attempts to parse it as a `BigInt`.
///
/// # Arguments
///
/// * `integer` - A type that can be converted into a string slice (e.g., &str or String).
///
/// # Returns
///
/// * `Ok(BigInt)` if the parsing is successful.
/// * `Err(Error)` if the input string is empty or if parsing fails.
///
/// # Errors
///
/// This function can return the following errors:
/// - `Error::EmptyString`: If the cleaned string is empty.
/// - `Error::InvalidBigInt`: If parsing the cleaned string into a `BigInt` fails.
pub fn big_integer<T: AsRef<str>>(integer: T) -> Result<BigInt, Error<'static>> {
  // Clean the string by removing leading/trailing whitespace and commas
  let cleaned = integer.as_ref().trim().replace(",", "");

  // Check if the cleaned string is empty
  if cleaned.is_empty() {
    // return Err(Error::EmptyString);
    Ok(BigInt::zero())
  } else {
    // Parse the BigInt from the cleaned string
    BigInt::from_str(&cleaned).map_err(|err| Error::InvalidBigInt(err, cleaned.into()))
  }
}

/// Parses a string into an `isize`.
///
/// This function cleans the input string by removing leading/trailing
/// whitespace and commas, then attempts to parse it as an `isize`.
///
/// # Arguments
///
/// * `integer` - A type that can be converted into a string slice (e.g., &str or String).
///
/// # Returns
///
/// * `Ok(isize)` if the parsing is successful.
/// * `Err(Error)` if the input string is empty or if parsing fails.
///
/// # Errors
///
/// This function can return the following errors:
/// - `Error::EmptyString`: If the cleaned string is empty.
/// - `Error::InvalidInt`: If parsing the cleaned string into an `isize` fails.
pub fn integer<T: AsRef<str>>(integer: T) -> Result<isize, Error<'static>> {
  // Clean the string by removing leading/trailing whitespace and commas
  let cleaned = integer.as_ref().trim().replace(",", "");

  // Check if the cleaned string is empty
  if cleaned.is_empty() {
    // return Err(Error::EmptyString);
    Ok(isize::zero())
  } else {
    // Parse the BigInt from the cleaned string
    isize::from_str(&cleaned).map_err(|err| Error::InvalidInt(err, cleaned.into()))
  }
}

/// Parses a string into a `BigDecimal`.
///
/// This function cleans the input string by removing leading/trailing
/// whitespace and commas, then attempts to parse it as a `BigDecimal`. It
/// handles both integer and fractional parts of the number.
///
/// # Arguments
///
/// * `float` - A type that can be converted into a string slice (e.g., &str or String).
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
/// - `Error::TooManyDecimalPoints`: If more than one decimal point is found in the input.
/// - `Error::InvalidBigInt`: If parsing fails while converting to BigInt.
/// - `Error::InvalidFractional`: If there are issues with parsing fractional parts.
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
        return Err(Error::fractional_overflow(fractional_part.to_string(), MAX_SCALE));
      }

      let fractional_value = usize::from_str(fractional_part)
        .map_err(|err| Error::InvalidFractional(err, fractional_part.to_string().into()))?;

      fractional_value as i64 // Return scale as i64 for BigDecimal creation.
    }
    _ => 0 // No fractional part or empty; scale is 0.
  };

  // Return parsed BigDecimal with digits and scale.
  Ok(BigDecimal::new(digits, scale))
}
