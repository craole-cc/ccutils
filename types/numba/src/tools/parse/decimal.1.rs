use super::error::Error;
use bigdecimal::{BigDecimal, ParseBigDecimalError, Zero};
use std::{borrow::Cow, num::ParseFloatError, str::FromStr};

/// A trait for types that can be converted to a string reference efficiently
pub trait ToStringRef {
	/// Converts the value to a string reference
	fn to_string_ref(&self) -> Cow<'_, str>;
}

// Implement for string slice - zero copy
impl ToStringRef for str {
	#[inline]
	fn to_string_ref(&self) -> Cow<'_, str> {
		Cow::Borrowed(self)
	}
}

// Implement for String - zero copy
impl ToStringRef for String {
	#[inline]
	fn to_string_ref(&self) -> Cow<'_, str> {
		Cow::Borrowed(self.as_str())
	}
}

// Implement for references to str
impl ToStringRef for &str {
	#[inline]
	fn to_string_ref(&self) -> Cow<'_, str> {
		Cow::Borrowed(*self)
	}
}

// Implement for numeric types
macro_rules! impl_to_string_ref_numeric {
    ($($t:ty),*) => {
        $(
            impl ToStringRef for $t {
                #[inline]
                fn to_string_ref(&self) -> Cow<'_, str> {
                    Cow::Owned(self.to_string())
                }
            }
        )*
    }
}

// Implement for all numeric types we want to support
impl_to_string_ref_numeric!(
	i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,
	f32, f64
);

#[inline]
pub fn big_decimal<T>(input: T) -> Result<BigDecimal, Error<'static>>
where
	T: ToStringRef,
{
	let input_cow = input.to_string_ref();
	let trimmed = input_cow.trim();

	// Handle empty/whitespace input early
	if trimmed.is_empty() {
		return Ok(BigDecimal::zero());
	}

	// Only allocate a new string if we actually find commas
	let cleaned = if trimmed.contains(',') {
		Cow::Owned(trimmed.chars().filter(|&c| c != ',').collect())
	} else {
		match input_cow {
			Cow::Borrowed(s) => Cow::Borrowed(trimmed),
			Cow::Owned(_) => Cow::Owned(trimmed.to_owned()),
		}
	};

	BigDecimal::from_str(&cleaned).map_err(|err| match err {
		ParseBigDecimalError::Empty => {
			Error::InvalidBigDecimal(err, cleaned.into_owned().into())
		}
		ParseBigDecimalError::ParseDecimal(float_err) => {
			if cleaned.contains(['e', 'E']) {
				Error::InvalidScientificNotation(
					float_err,
					cleaned.into_owned().into(),
				)
			} else {
				Error::InvalidBigDecimal(
					ParseBigDecimalError::ParseDecimal(float_err),
					cleaned.into_owned().into(),
				)
			}
		}
		ParseBigDecimalError::ParseInt(int_err) => {
			Error::InvalidMantissa(
				ParseBigDecimalError::ParseInt(int_err),
				cleaned.into_owned().into(),
			)
		}
		ParseBigDecimalError::ParseBigInt(bigint_err) => {
			Error::InvalidBigInt(
				bigint_err,
				cleaned.into_owned().into(),
			)
		}
		ParseBigDecimalError::Other(err_msg) => {
			Error::InvalidBigDecimal(
				ParseBigDecimalError::Other(err_msg),
				cleaned.into_owned().into(),
			)
		}
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_various_types() {
		// Integer types
		assert_eq!(big_decimal(123i32).unwrap().to_string(), "123");
		assert_eq!(big_decimal(456i64).unwrap().to_string(), "456");
		assert_eq!(big_decimal(789usize).unwrap().to_string(), "789");

		// Floating point types
		assert_eq!(
			big_decimal(123.45f64).unwrap().to_string(),
			"123.45"
		);
		assert_eq!(
			big_decimal(456.78f32).unwrap().to_string(),
			"456.78"
		);

		// String types
		assert_eq!(
			big_decimal("123.45").unwrap().to_string(),
			"123.45"
		);
		assert_eq!(
			big_decimal(String::from("456.78")).unwrap().to_string(),
			"456.78"
		);

		// With commas
		assert_eq!(
			big_decimal("1,234.56").unwrap().to_string(),
			"1234.56"
		);
	}

	#[test]
	fn test_edge_cases() {
		// Empty and whitespace
		assert_eq!(big_decimal("").unwrap().to_string(), "0");
		assert_eq!(big_decimal(" ").unwrap().to_string(), "0");

		// Max values
		assert!(big_decimal(i64::MAX).is_ok());
		assert!(big_decimal(u64::MAX).is_ok());
	}
}
