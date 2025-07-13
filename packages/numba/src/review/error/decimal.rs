#[cfg(feature = "big-decimal")]
use bigdecimal::ParseBigDecimalError;
use erks::thiserror;
use num::bigint::ParseBigIntError;
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
  #[error("The fractional part provided ('{0}') exceeds the allowed size by {1} digits (Allowed: {2}).")]
  FractionalOverflow(Cow<'a, str>, usize, usize),

  /// Indicates that the fractional part is problematic due to its size.
  ///
  /// # Parameters
  /// - `provided_len`: The length of the provided fractional part.
  /// - `max`: The maximum allowed size in digits for the fractional part.
  #[error("The fractional part provided ({0}) exceeds the allowed size ({1}).")]
  FractionalProblematic(usize, usize),

  /// Indicates that a negative fractional part was provided, which is not
  /// allowed.
  #[error("The fractional part provided is negative, which is not allowed.")]
  FractionalNegative,

  /// Indicates a failure to parse a `BigInt` from a string.
  ///
  /// # Parameters
  /// - `ParseBigIntError`: The underlying error from the parsing attempt.
  /// - `input`: The original input string that caused the error.
  #[error("Failed to parse BigInt from '{1}'\nParseBigIntError: {0}")]
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
  #[error("Failed to parse BigDecimal from '{1}'\nParseIntError: {0}")]
  InvalidBigDecimal(ParseBigDecimalError, Cow<'a, str>),

  #[error("Failed to parse scientific notation from '{1}'\nParseFloatError: {0}")]
  InvalidScientificNotation(std::num::ParseFloatError, Cow<'a, str>),
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
  Decimal(#[from] rust_decimal::Error)
}

impl<'a> Error<'a> {
  /// Constructs a `FractionalOverflow` error by calculating the max size and
  /// difference
  ///
  /// # Arguments
  /// * `fractional`: The fractional part as a string slice or owned string that caused the overflow.
  /// * `max`: The maximum allowed size for the fractional part in digits.
  pub fn fractional_overflow<S: Into<Cow<'a, str>>>(fractional: S, max: usize) -> Self {
    let fractional_str = fractional.into();
    let provided_len = fractional_str.len();
    let difference = provided_len.saturating_sub(max.to_string().len());

    // Return the error with the fractional part, the maximum size, and the
    // difference calculated
    Self::FractionalOverflow(fractional_str, difference, max)
  }

  fn from_bigint_error<S: Into<Cow<'a, str>>>(input: S, err: ParseBigIntError) -> Self {
    Self::InvalidBigInt(err, input.into())
  }

  fn from_int_error<S: Into<Cow<'a, str>>>(input: S, err: ParseIntError) -> Self {
    Self::InvalidInt(err, input.into())
  }
}

// Implement From for (&str, ParseBigIntError)
impl<'a> From<(&'a str, ParseBigIntError)> for Error<'a> {
  fn from((input, err): (&'a str, ParseBigIntError)) -> Self {
    Error::from_bigint_error(input, err)
  }
}

// Implement From for (String, ParseBigIntError)
impl From<(String, ParseBigIntError)> for Error<'_> {
  fn from((input, err): (String, ParseBigIntError)) -> Self {
    Error::from_bigint_error(input, err)
  }
}

// Implement From for (&str, ParseIntError)
impl<'a> From<(&'a str, ParseIntError)> for Error<'a> {
  fn from((input, err): (&'a str, ParseIntError)) -> Self {
    Error::from_int_error(input, err)
  }
}

// Implement From for (String, ParseIntError)
impl From<(String, ParseIntError)> for Error<'_> {
  fn from((input, err): (String, ParseIntError)) -> Self {
    Error::from_int_error(input, err)
  }
}
