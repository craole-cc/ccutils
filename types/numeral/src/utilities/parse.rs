use crate::{
	cache::PARSE_CACHE, error::decimal::Error,
	simd::remove_commas_simd,
};
use bigdecimal::BigDecimal;
use rust_decimal::Decimal;
use std::str::FromStr;

/// Convert BigDecimal to Decimal safely
#[cfg(feature = "bigdecimal")]
fn convert_big_decimal_to_decimal(
	bd: BigDecimal,
) -> Result<Decimal, Error<'static>> {
	// Convert through string representation as a safe fallback
	Decimal::from_str(&bd.to_string())
		.map_err(|_| Error::Overflow("Number too large for Decimal"))
}

/// Fast path for parsing common numeric types
#[inline]
fn parse_decimal<T: ToString>(
	input: T,
) -> Result<Decimal, Error<'static>> {
	let input_str = input.to_string();

	// Check cache first
	if let Some(cached) = PARSE_CACHE.get_decimal(&input_str) {
		return Ok(cached);
	}

	// Fast path for integers
	if let Ok(i) = input_str.parse::<i64>() {
		let decimal = Decimal::from(i);
		PARSE_CACHE.insert_decimal(input_str, decimal);
		return Ok(decimal);
	}

	// Handle commas and parse
	let cleaned = if input_str.contains(',') {
		remove_commas_simd(&input_str)
	} else {
		input_str
	};

	match Decimal::from_str(&cleaned) {
		Ok(result) => {
			PARSE_CACHE.insert_decimal(cleaned.clone(), result);
			Ok(result)
		}
		Err(_) => Err(Error::Decimal("Invalid decimal".into())),
	}
}

#[cfg(feature = "bigdecimal")]
#[inline]
fn parse_big_decimal(
	input: &str,
) -> Result<BigDecimal, Error<'static>> {
	// Check cache first
	if let Some(cached) = PARSE_CACHE.get_big_decimal(input) {
		return Ok(cached);
	}

	let result =
		BigDecimal::from_str(input).map_err(Error::BigDecimal)?;

	PARSE_CACHE.insert_big_decimal(input.to_owned(), result.clone());
	Ok(result)
}

/// Parse number with fallback to BigDecimal if needed
#[inline]
pub fn parse_numeral<T: ToString>(
	input: T,
) -> Result<BigDecimal, Error<'static>> {
	let input_str = input.to_string();

	// Try parsing as Decimal first
	match parse_decimal(&input_str) {
		Ok(decimal) => {
			#[cfg(feature = "bigdecimal")]
			{
				logline::warn!("Converting Decimal to BigDecimal_1");
				// Convert Decimal to BigDecimal explicitly if needed
				return parse_big_decimal(&decimal.to_string());
			}
			// If bigdecimal feature is not enabled, return an error or handle accordingly
			Err(Error::Decimal(
				"BigDecimal feature not enabled".into(),
			))
		}
		Err(Error::Decimal(_)) => {
			#[cfg(feature = "bigdecimal")]
			{
				logline::warn!("Converting Decimal to BigDecimal_2");
				// If parsing as Decimal fails, try BigDecimal directly
				return parse_big_decimal(&input_str);
			}
			// If bigdecimal feature is not enabled, propagate the error
			Err(Error::Decimal(
				"BigDecimal feature not enabled".into(),
			))
		}
		Err(err) => Err(err), // Propagate other errors
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_decimal() {
		assert!(parse_decimal("123.45").is_ok());
		assert!(parse_decimal("not_a_number").is_err());
	}

	#[test]
	#[cfg(feature = "bigdecimal")]
	fn test_parse_big_decimal() {
		assert!(parse_big_decimal("123.45").is_ok());
		assert!(parse_big_decimal("not_a_number").is_err());
	}

	#[test]
	fn test_parse_number() {
		assert!(parse_numeral("123.45").is_ok());
		assert!(parse_numeral("not_a_number").is_err());
	}

	#[test]
	pub fn test_large_decimal() {
		// Initialize test values
		let integer =
			<num::BigInt as num::FromPrimitive>::from_u64(10)
				.unwrap()
				.pow(100);
		let fractional: usize = isize::MAX as usize;
		let sign = "-";
		let input = format!("{}{}.{}", sign, integer, fractional);

		// Expected value (stored as string for comparison)
		let expected = "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.9223372036854775807";

		// Log input values
		println!("Test input parameters:");
		println!("      Sign: {}", sign);
		println!("   Integer: {}", integer);
		println!("Fractional: {}", fractional);
		println!("Full input: {}", input);

		match parse_numeral(&input) {
			Ok(value) => {
				println!("Successfully parsed decimal");
				println!("Parsed value: {}", value);

				// Convert both to strings for comparison
				let value_str = value.to_string();

				// Log comparison values
				println!("Expected: {}", expected);
				println!("  Actual: {}", value_str);

				assert_eq!(
                value_str,
                expected,
                "Parsed value doesn't match expected value\nExpected: {}\nActual: {}",
                expected,
                value_str
            );
			}
			Err(err) => {
				println!("Failed to parse decimal: {}", err);
				println!("Input was: {}", input);
				panic!("Test failed due to parse error: {}", err);
			}
		}
	}
}
