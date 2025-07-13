use super::{error::Error, traits::ToStringRef};
use bigdecimal::{BigDecimal, ParseBigDecimalError, Zero};
use rust_decimal::Decimal;
use std::str::FromStr;

/// Try to parse as a regular decimal first, fall back to BigDecimal if needed
#[inline]
pub fn parse_number<T: ToStringRef>(input: T) -> Result<BigDecimal, Error<'static>> {
  let input_cow = input.to_string_ref();
  let trimmed = input_cow.trim();

  // Handle empty/whitespace input early
  if trimmed.is_empty() {
    return Ok(BigDecimal::zero());
  }

  // Only allocate a new string if we actually find commas
  let cleaned = if trimmed.contains(',') {
    trimmed.chars().filter(|&c| c != ',').collect::<String>()
  } else {
    trimmed.to_owned()
  };

  // Try parsing as regular Decimal first
  match Decimal::from_str(&cleaned) {
    Ok(decimal) => Ok(BigDecimal::from_str(decimal.to_string().as_str())),
    Err(_) => {
      // If that fails, try BigDecimal
      parse_big_decimal(&cleaned)
    }
  }
}

#[inline]
fn parse_big_decimal(input: &str) -> Result<BigDecimal, Error<'static>> {
  BigDecimal::from_str(input).map_err(|err| match err {
    ParseBigDecimalError::Empty => Error::InvalidBigDecimal(err, input.to_owned().into()),
    ParseBigDecimalError::ParseDecimal(float_err) =>
      if input.contains(['e', 'E']) {
        Error::InvalidScientificNotation(float_err, input.to_owned().into())
      } else {
        Error::InvalidBigDecimal(ParseBigDecimalError::ParseDecimal(float_err), input.to_owned().into())
      },
    ParseBigDecimalError::ParseInt(int_err) =>
      Error::InvalidMantissa(ParseBigDecimalError::ParseInt(int_err), input.to_owned().into()),
    ParseBigDecimalError::ParseBigInt(bigint_err) => Error::InvalidBigInt(bigint_err, input.to_owned().into()),
    ParseBigDecimalError::Other(err_msg) =>
      Error::InvalidBigDecimal(ParseBigDecimalError::Other(err_msg), input.to_owned().into()),
  })
}
