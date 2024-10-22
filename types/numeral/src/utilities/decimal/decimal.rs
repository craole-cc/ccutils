// traits.rs
use std::borrow::Cow;

/// A trait for types that can be converted to a string reference efficiently
pub trait ToStringRef {
    /// Converts the value to a string reference
    fn to_string_ref(&self) -> Cow<'_, str>;
}

// string_impls.rs
use super::traits::ToStringRef;
use std::borrow::Cow;

impl ToStringRef for str {
    #[inline]
    fn to_string_ref(&self) -> Cow<'_, str> {
        Cow::Borrowed(self)
    }
}

impl ToStringRef for String {
    #[inline]
    fn to_string_ref(&self) -> Cow<'_, str> {
        Cow::Borrowed(self.as_str())
    }
}

impl ToStringRef for &str {
    #[inline]
    fn to_string_ref(&self) -> Cow<'_, str> {
        Cow::Borrowed(*self)
    }
}

// numeric_impls.rs
use super::traits::ToStringRef;
use std::borrow::Cow;

macro_rules! impl_to_string_ref_numeric {
    ($($t:ty),*) => {
        $(
            impl ToStringRef for $t {
                #[inline]
                fn to_string_ref(&self) -> Cow<'_, str> {
                    // For integers and small floats, we can pre-allocate
                    // a reasonable buffer size
                    let mut buffer = String::with_capacity(20);
                    use std::fmt::Write;
                    write!(buffer, "{}", self).unwrap();
                    Cow::Owned(buffer)
                }
            }
        )*
    }
}

impl_to_string_ref_numeric!(
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64
);

// parse.rs
use super::error::Error;
use bigdecimal::{BigDecimal, ParseBigDecimalError, Zero};
use std::str::FromStr;
use rust_decimal::Decimal;

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
        Ok(decimal) => Ok(BigDecimal::from(decimal)),
        Err(_) => {
            // If that fails, try BigDecimal
            parse_big_decimal(&cleaned)
        }
    }
}

#[inline]
fn parse_big_decimal(input: &str) -> Result<BigDecimal, Error<'static>> {
    BigDecimal::from_str(input).map_err(|err| match err {
        ParseBigDecimalError::Empty => {
            Error::InvalidBigDecimal(err, input.to_owned().into())
        }
        ParseBigDecimalError::ParseDecimal(float_err) => {
            if input.contains(['e', 'E']) {
                Error::InvalidScientificNotation(float_err, input.to_owned().into())
            } else {
                Error::InvalidBigDecimal(
                    ParseBigDecimalError::ParseDecimal(float_err),
                    input.to_owned().into(),
                )
            }
        }
        ParseBigDecimalError::ParseInt(int_err) => {
            Error::InvalidMantissa(
                ParseBigDecimalError::ParseInt(int_err),
                input.to_owned().into(),
            )
        }
        ParseBigDecimalError::ParseBigInt(bigint_err) => {
            Error::InvalidBigInt(bigint_err, input.to_owned().into())
        }
        ParseBigDecimalError::Other(err_msg) => {
            Error::InvalidBigDecimal(
                ParseBigDecimalError::Other(err_msg),
                input.to_owned().into(),
            )
        }
    })
}

// benches/parse.rs
#[cfg(test)]
mod benches {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    use super::*;

    pub fn bench_parsing(c: &mut Criterion) {
        c.bench_function("parse regular decimal", |b| {
            b.iter(|| parse_number(black_box("123.45")))
        });

        c.bench_function("parse large number", |b| {
            b.iter(|| parse_number(black_box("123456789123456789.123456789")))
        });
    }

    criterion_group!(benches, bench_parsing);
    criterion_main!(benches);
}
