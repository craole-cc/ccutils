// mod functions;
mod error;
// mod methods;
// mod output;
// mod tests;
// mod traits;
// mod types;
mod utilities;
use erks::{anyhow::anyhow, AnyhowResult};

// pub use functions::*;

// pub use traits::*;
// pub use types::*;
// pub use utilities::*;

// pub fn init() -> Numeral {
// 	Numeral::default()
// }

use bigdecimal::BigDecimal;
use num::{bigint::Sign, BigInt};
use std::str::FromStr;
use utilities::{cardinal::*, num2words::*, parse};

mod parser {
	use bigdecimal::{BigDecimal, ParseBigDecimalError, Zero};
	use erks::thiserror;
	use num::bigint::{BigInt, ParseBigIntError};
	use std::{num::ParseIntError, str::FromStr};

	#[derive(thiserror::Error, Debug)]
	pub enum Error {
		#[error("Empty string after cleaning")]
		EmptyString,
		#[error("Too many decimal points {0}")]
		TooManyDecimalPoints(usize),
		#[error("Error parsing BigInt: {0}")]
		InvalidBigInt(#[from] ParseBigIntError),
		#[error("Error parsing isize: {0}")]
		InvalidInt(#[from] ParseIntError),
		#[error("Error parsing BigDecimal: {0}")]
		InvalidFloat(#[from] ParseBigDecimalError),
	}
	fn big_integer_from_str(integer: &str) -> Result<BigInt, Error> {
		// Clean the string by removing leading/trailing whitespace and commas
		let cleaned = integer.trim().replace(",", "");

		// Check if the cleaned string is empty
		if cleaned.is_empty() {
			return Err(Error::EmptyString);
		}

		// Parse the BigInt from the cleaned string
		BigInt::from_str(&cleaned).map_err(Error::InvalidBigInt)
	}

	fn integer_from_str(integer: &str) -> Result<isize, Error> {
		// Clean the string by removing leading/trailing whitespace and commas
		let cleaned = integer.trim().replace(",", "");

		// Check if the cleaned string is empty
		if cleaned.is_empty() {
			return Err(Error::EmptyString);
		}

		// Parse the BigInt from the cleaned string
		isize::from_str(&cleaned).map_err(Error::InvalidInt)
	}

	fn float_from_str(float: &str) -> Result<BigDecimal, Error> {
		// Clean the string by removing leading/trailing whitespace and commas
		let cleaned = float.trim().replace(",", "");

		// Check if the cleaned string is empty
		if cleaned.is_empty() {
			return Err(Error::EmptyString);
		}

		// Split the string into the integer and fractional parts
		let parts: Vec<&str> = cleaned.split('.').collect();

		// Ensure there are no more than two parts (integer and fractional)
		if parts.len() > 2 {
			return Err(Error::TooManyDecimalPoints(parts.len()));
		}

		// Check if value is zero
		if cleaned == "0" {
			return Ok(BigDecimal::zero());
		}

		// Parse the integer part
		let digits = match BigInt::from_str(parts[0]) {
			Ok(digits) => digits,
			Err(err) => return Err(Error::InvalidBigInt(err)),
		};

		// Determine the scale (number of digits after the decimal point)
		let scale = if parts.len() == 2 {
			let fractional_part = &parts[1];

			// Check if the fractional part is empty or zero
			if fractional_part.is_empty() || *fractional_part == "0" {
				0 // No digits after the decimal point
			} else {
				// Try to parse the scale from the fractional part
				i64::from_str(fractional_part)
					.map_err(Error::InvalidInt)?
			}
		} else {
			0 // No fractional part, scale is 0
		};

		// Return the parsed BigDecimal
		Ok(BigDecimal::new(digits, scale))
	}

	pub fn main() {
		let sign = "-";
		let integer = <BigInt as num::FromPrimitive>::from_u64(10)
			.unwrap()
			.pow(100);
		let fractional = isize::MAX;
		let decimal_str =
			format!("{}{}.{}", sign, integer, fractional);
		let decimal = match float_from_str(decimal_str.as_str()).ok()
		{
			Some(value) => value,
			None => return println!("Invalid number format"),
		};
		logline::trace!("   Integer {:?}", integer);
		logline::trace!("Fractional {:?}", fractional);
		logline::trace!("{:?}", decimal);
		println!("{}", decimal);
	}
}

pub fn test() -> AnyhowResult<()> {
	logline::Logline::default()
		.with_level(logline::TRACE)
		.init();

	// parse::test();
	let result = parse::from_str::float("1.9223372036854775807")?;
	logline::trace!("{:?} => {:}", result, result);
	let result = parse::from_str::float(String::from(
		"1.9999999999999999999",
	))?;
	// let result = parse::from_str::float("1.92233720368547758")?;
	logline::trace!("{:?} => {:}", result, result);

	Ok(())
}
