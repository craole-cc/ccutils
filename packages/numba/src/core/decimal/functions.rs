use super::{CACHE, Error};
use crate::remove_commas;
use std::str::FromStr;

pub fn parse_rust_decimal<T: ToString>(
  input: T
) -> Result<rust_decimal::Decimal, Error<'static>> {
  let input_str = input.to_string();

  // Check cache first
  if let Some(cached) = CACHE.get_rust_decimal(&input_str) {
    return Ok(cached);
  }

  // Fast path for integers
  if let Ok(i) = input_str.parse::<i64>() {
    let decimal = rust_decimal::Decimal::from(i);
    CACHE.insert_rust_decimal(input_str, decimal);
    return Ok(decimal);
  }

  // Handle commas and parse
  let cleaned = if input_str.contains(',') {
    remove_commas(&input_str)
  } else {
    input_str
  };

  match rust_decimal::Decimal::from_str(&cleaned) {
    Ok(result) => {
      CACHE.insert_rust_decimal(cleaned.clone(), result);
      Ok(result)
    }
    Err(_) => Err(Error::Decimal("Invalid decimal".into()))
  }
}

#[cfg(feature = "big-decimal")]
pub fn parse_big_decimal(
  input: &str
) -> Result<bigdecimal::BigDecimal, Error<'static>> {
  if let Some(cached) = CACHE.get_big_decimal(input) {
    return Ok(cached);
  }

  let result =
    bigdecimal::BigDecimal::from_str(input).map_err(Error::BigDecimal)?;

  CACHE.insert_big_decimal(input.to_owned(), result.clone());
  Ok(result)
}
