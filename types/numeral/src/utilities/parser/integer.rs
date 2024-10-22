use super::Error;
use bigdecimal::{BigDecimal, Zero};
use num::BigInt;
use std::str::FromStr;

/// Parses a string into a `BigInt`.
///
/// This function cleans the input string by removing leading/trailing whitespace
/// and commas, then attempts to parse it as a `BigInt`.
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
pub fn big_integer<T: AsRef<str>>(
	integer: T,
) -> Result<BigInt, Error<'static>> {
	// Clean the string by removing leading/trailing whitespace and commas
	let cleaned = integer.as_ref().trim().replace(",", "");

	// Check if the cleaned string is empty
	if cleaned.is_empty() {
		// return Err(Error::EmptyString);
		Ok(BigInt::zero())
	} else {
		// Parse the BigInt from the cleaned string
		BigInt::from_str(&cleaned)
			.map_err(|err| Error::InvalidBigInt(err, cleaned.into()))
	}
}

/// Parses a string into an `isize`.
///
/// This function cleans the input string by removing leading/trailing whitespace
/// and commas, then attempts to parse it as an `isize`.
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
pub fn integer<T: AsRef<str>>(
	integer: T,
) -> Result<isize, Error<'static>> {
	// Clean the string by removing leading/trailing whitespace and commas
	let cleaned = integer.as_ref().trim().replace(",", "");

	// Check if the cleaned string is empty
	if cleaned.is_empty() {
		// return Err(Error::EmptyString);
		Ok(isize::zero())
	} else {
		// Parse the BigInt from the cleaned string
		isize::from_str(&cleaned)
			.map_err(|err| Error::InvalidInt(err, cleaned.into()))
	}
}
