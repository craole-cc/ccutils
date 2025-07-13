use super::Error;
use bigdecimal::{BigDecimal, Zero};
use num::{BigInt, FromPrimitive, bigint};
use std::str::FromStr;

/// Parses a string into a `BigDecimal` that can handle scientific notation.
///
/// This function cleans the input string by removing leading/trailing
/// whitespace and commas, then attempts to manually validate and parse it as a
/// `BigDecimal`. It supports both regular decimal notation and scientific
/// notation (e.g., "1.23e4").
///
/// # Arguments
///
/// * `input` - A type that can be converted into a string slice (e.g., &str or String).
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
/// - `Error::InvalidScientificNotation`: If the input does not match valid scientific notation.
/// - `Error::InvalidMantissa`: If the mantissa part of the input is invalid.
/// - `Error::InvalidExponent`: If the exponent part of the input is invalid.
pub fn scientific_notation<T: AsRef<str>>(input: T) -> Result<BigDecimal, Error<'static>> {
  // Clean the string by removing leading/trailing whitespace and commas
  let cleaned = input.as_ref().trim().replace(",", "");

  // Check if the cleaned string is empty
  if cleaned.is_empty() {
    return Ok(BigDecimal::zero());
  }

  // Check for scientific notation by looking for 'e' or 'E'
  let (mantissa_part, exponent_part) = match cleaned.find(|c: char| c == 'e' || c == 'E') {
    Some(index) => {
      // Split the string into mantissa and exponent parts
      let (mantissa, exponent) = cleaned.split_at(index);
      let exponent = &exponent[1..]; // Skip the 'e'/'E'
      (mantissa, Some(exponent))
    }
    None => (cleaned.as_str(), None)
  };

  // Validate the mantissa part (it should be a valid decimal)
  let mantissa =
    BigDecimal::from_str(mantissa_part).map_err(|err| Error::InvalidMantissa(err, mantissa_part.into()))?;

  // If there's no exponent, return the mantissa as the result
  if exponent_part.is_none() {
    return Ok(mantissa);
  }

  // Validate the exponent part (it should be a valid integer)
  let exponent_str = exponent_part.unwrap();
  let exponent = isize::from_str(exponent_str).map_err(|err| Error::InvalidExponent(err, exponent_str.into()))?;

  // Multiply the mantissa by 10^exponent to get the final result
  let scale = BigInt::from_u64(10).unwrap().pow(exponent.abs() as u32);
  let result = if exponent < 0 {
    mantissa / scale
  } else {
    mantissa * scale
  };

  Ok(result)
}
