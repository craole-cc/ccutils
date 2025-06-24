#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		error::cardinal::Error,
		utilities::cardinal::{
			numeric_to_worded, worded_from_numeric,
		},
	};

	#[test]
	fn test_numeric_to_worded() {
		// Positive integers
		assert_eq!(numeric_to_worded("zero").unwrap(), 0.0);
		assert_eq!(numeric_to_worded("one").unwrap(), 1.0);
		assert_eq!(numeric_to_worded("twenty").unwrap(), 20.0);
		assert_eq!(numeric_to_worded("ninety-nine").unwrap(), 99.0);
		assert_eq!(numeric_to_worded("one hundred").unwrap(), 100.0);
		assert_eq!(
			numeric_to_worded("one thousand").unwrap(),
			1000.0
		);
		assert_eq!(
			numeric_to_worded("one million").unwrap(),
			1_000_000.0
		);
		assert_eq!(
			numeric_to_worded("three billion").unwrap(),
			3_000_000_000.0
		);

		// Negative integers
		assert_eq!(numeric_to_worded("minus one").unwrap(), -1.0);
		assert_eq!(numeric_to_worded("negative two").unwrap(), -2.0);
		assert_eq!(
			numeric_to_worded("minus one hundred twenty-three")
				.unwrap(),
			-123.0
		);

		// Fractions
		assert_eq!(numeric_to_worded("one point five").unwrap(), 1.5);
		assert_eq!(
			numeric_to_worded("two point zero one").unwrap(),
			2.01
		);
		assert_eq!(
			numeric_to_worded("three point fourteen").unwrap(),
			3.14
		);
		assert_eq!(
			numeric_to_worded("negative two point five").unwrap(),
			-2.5
		);

		// Mixed numbers
		assert_eq!(
			numeric_to_worded("four hundred twenty-one").unwrap(),
			421.0
		);
		assert_eq!(
			numeric_to_worded("seven thousand one hundred").unwrap(),
			7100.0
		);
		assert_eq!(
			numeric_to_worded(
				"eleven thousand two hundred thirty-four"
			)
			.unwrap(),
			11234.0
		);

		// Edge cases
		assert!(numeric_to_worded("and").is_err());
		assert!(numeric_to_worded("minus").is_err());
		assert!(numeric_to_worded("three hundred and").is_err());
		assert!(numeric_to_worded(
			"three hundred and fourteen point"
		)
		.is_err());
		assert!(numeric_to_worded(
			"one hundred twenty-three point forty-five"
		)
		.is_ok());
	}

	#[test]
	fn test_worded_from_numeric() {
		// Positive integers
		assert_eq!(worded_from_numeric(0.0).unwrap(), "zero");
		assert_eq!(worded_from_numeric(1.0).unwrap(), "one");
		assert_eq!(worded_from_numeric(20.0).unwrap(), "twenty");
		assert_eq!(worded_from_numeric(99.0).unwrap(), "ninety-nine");
		assert_eq!(
			worded_from_numeric(100.0).unwrap(),
			"one hundred"
		);
		assert_eq!(
			worded_from_numeric(1000.0).unwrap(),
			"one thousand"
		);
		assert_eq!(
			worded_from_numeric(1_000_000.0).unwrap(),
			"one million"
		);
		assert_eq!(
			worded_from_numeric(3_000_000_000.0).unwrap(),
			"three billion"
		);

		// Negative integers
		assert_eq!(worded_from_numeric(-1.0).unwrap(), "minus one");
		assert_eq!(
			worded_from_numeric(-123.0).unwrap(),
			"minus one hundred twenty-three"
		);

		// Fractions
		assert_eq!(
			worded_from_numeric(1.5).unwrap(),
			"one point five"
		);
		assert_eq!(
			worded_from_numeric(2.01).unwrap(),
			"two point zero one"
		);
		assert_eq!(
			worded_from_numeric(-2.5).unwrap(),
			"minus two point five"
		);

		// Mixed numbers
		assert_eq!(
			worded_from_numeric(421.0).unwrap(),
			"four hundred twenty-one"
		);
		assert_eq!(
			worded_from_numeric(7100.0).unwrap(),
			"seven thousand one hundred"
		);
		assert_eq!(
			worded_from_numeric(11234.0).unwrap(),
			"eleven thousand two hundred thirty-four"
		);

		// Edge cases
		assert!(worded_from_numeric(f64::NAN).is_err());
		assert!(worded_from_numeric(f64::INFINITY).is_err());
		assert!(worded_from_numeric(f64::NEG_INFINITY).is_err());
	}
}
