use crate::decimal::Error;
#[cfg(feature = "big-decimal")]
use bigdecimal::ParseBigDecimalError;
use num::bigint::ParseBigIntError;
use std::{borrow::Cow, num::ParseIntError};

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
