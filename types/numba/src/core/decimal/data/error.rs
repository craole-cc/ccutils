#[cfg(feature = "big-decimal")]
use bigdecimal::ParseBigDecimalError;
use num::bigint::ParseBigIntError;
use erks::thiserror;
use std::{borrow::Cow, num::ParseIntError};

/// An enumeration of errors that can occur during parsing operations.
///
/// This enum encapsulates various error types that may arise when parsing
/// strings into numeric types, including issues with formatting, invalid input,
/// and parsing failures.
#[derive(Debug, thiserror::Error)]
pub enum Error<'a> {
	/// Indicates that there are too many decimal points in the input.
	#[error("Too many decimal points: {0}")]
	TooManyDecimalPoints(usize),

	/// Indicates that the fractional part exceeds the allowed size.
	///
	/// # Parameters
	/// - `fractional`: The fractional part that caused the overflow.
	/// - `difference`: The number of digits by which it exceeds the allowed size.
	/// - `max`: The maximum allowed size in digits for the fractional part.
	#[error(
        "The fractional part provided ('{0}') exceeds the allowed size by {1} digits (Allowed: {2})."
    )]
	FractionalOverflow(Cow<'a, str>, usize, usize),

	/// Indicates that the fractional part is problematic due to its size.
	///
	/// # Parameters
	/// - `provided_len`: The length of the provided fractional part.
	/// - `max`: The maximum allowed size in digits for the fractional part.
	#[error("The fractional part provided ({0}) exceeds the allowed size ({1}).")]
	FractionalProblematic(usize, usize),

	/// Indicates that a negative fractional part was provided, which is not allowed.
	#[error("The fractional part provided is negative, which is not allowed.")]
	FractionalNegative,

	/// Indicates a failure to parse a `BigInt` from a string.
	///
	/// # Parameters
	/// - `ParseBigIntError`: The underlying error from the parsing attempt.
	/// - `input`: The original input string that caused the error.
	#[error(
		"Failed to parse BigInt from '{1}'\nParseBigIntError: {0}"
	)]
	InvalidBigInt(ParseBigIntError, Cow<'a, str>),

	/// Indicates a failure to parse an integer from a string.
	///
	/// # Parameters
	/// - `ParseIntError`: The underlying error from the parsing attempt.
	/// - `input`: The original input string that caused the error.
	#[error("Failed to parse Int from '{1}'\nParseIntError: {0}")]
	InvalidInt(ParseIntError, Cow<'a, str>),

	/// Indicates a failure to parse a valid fractional number from a string.
	///
	/// # Parameters
	/// - `ParseIntError`: The underlying error from the parsing attempt.
	/// - `input`: The original input string that caused the error.
	#[error("Failed to parse valid fractional from '{1}'\nInvalidFractional: {0}")]
	InvalidFractional(ParseIntError, Cow<'a, str>),

	#[cfg(feature = "big-decimal")]
	#[error("Failed to parse valid mantissa from '{1}'\nInvalidFractional: {0}")]
	InvalidMantissa(ParseBigDecimalError, Cow<'a, str>),

	#[error("Failed to parse valid exponent from '{1}'\nInvalidFractional: {0}")]
	InvalidExponent(ParseIntError, Cow<'a, str>),

	#[cfg(feature = "big-decimal")]
	#[error(
		"Failed to parse BigDecimal from '{1}'\nParseIntError: {0}"
	)]
	InvalidBigDecimal(ParseBigDecimalError, Cow<'a, str>),

	#[error("Failed to parse scientific notation from '{1}'\nParseFloatError: {0}")]
	InvalidScientificNotation(
		std::num::ParseFloatError,
		Cow<'a, str>,
	),
	#[error("Invalid decimal format: {0}")]
	InvalidFormat(&'a str),
	#[error("Number too large: {0}")]
	Overflow(&'a str),
	#[error("Invalid scientific notation: {0}")]
	InvalidScientific(&'a str),
	#[cfg(feature = "big-decimal")]
	#[error("BigDecimal error: {0}")]
	BigDecimal(#[from] bigdecimal::ParseBigDecimalError),
	#[error("Decimal error: {0}")]
	Decimal(#[from] rust_decimal::Error),
}
