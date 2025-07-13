use crate::decimal::Error;
use num::bigint::ParseBigIntError;
use std::{borrow::Cow, num::ParseIntError};

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

  pub fn from_bigint_error<S: Into<Cow<'a, str>>>(input: S, err: ParseBigIntError) -> Self {
    Self::InvalidBigInt(err, input.into())
  }

  pub fn from_int_error<S: Into<Cow<'a, str>>>(input: S, err: ParseIntError) -> Self {
    Self::InvalidInt(err, input.into())
  }
}
