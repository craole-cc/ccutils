use crate::{parse::*, Error, Value};

impl Value {
	pub fn parse<T>(value: T) -> Result<Value, Error>
	where
		T: Into<Value>,
	{
		let value = value.into();

		match value {
			Self::Words(ref words) => {
				// Try parsing as f64 first
				if let Ok(decimal_value) = words.parse::<f64>() {
					return Ok(Self::Decimal(decimal_value));
				}
				// Try parsing as usize if f64 fails
				if let Ok(int_value) = words.parse::<usize>() {
					return Ok(Self::Integer(int_value));
				}

				// Try parsing as cardinal words
				if let Some(float_value) = cardinal_worded(words) {
					return Ok(Self::Decimal(float_value));
				}

				// Try parsing as ordinal words
				if let Some(int_value) = ordinal_worded(words) {
					return Ok(Self::Integer(int_value));
				}

				// Try parsing as ordinal symbols
				if let Some(int_value) = ordinal_symbolic(words) {
					return Ok(Self::Integer(int_value));
				}

				// Try parsing as fraction symbols
				if let Some((numerator, denominator)) =
					fraction_symbolic(words)
				{
					return Ok(Self::Decimal(
						numerator as f64 / denominator as f64,
					));
				}

				// Try parsing as fraction words
				if let Some((numerator, denominator)) =
					fraction_worded(words)
				{
					return Ok(Self::Decimal(
						numerator as f64 / denominator as f64,
					));
				}

				// Try parsing as percentage symbol
				if let Some((percent, _)) = percentage_symbol(words) {
					return Ok(Self::Decimal(percent / 100.0));
				}

				// Try parsing as percentage words
				if let Some((percent, _)) = percentage_worded(words) {
					return Ok(Self::Decimal(percent / 100.0));
				}

				// Try parsing as roman numeral
				if let Some(int_value) = roman_symbolic(words) {
					return Ok(Self::Integer(int_value));
				}

				// If parsing fails, return an error
				Err(Error::FailedToParseWords(words.to_string()))
			}
			_ => Ok(value),
		}
	}
}

impl From<f64> for Value {
	fn from(value: f64) -> Self {
		Self::Decimal(value)
	}
}

impl From<usize> for Value {
	fn from(value: usize) -> Self {
		Self::Integer(value)
	}
}

impl From<&str> for Value {
	fn from(value: &str) -> Self {
		Self::Words(value.to_string())
	}
}

impl From<String> for Value {
	fn from(value: String) -> Self {
		Self::Words(value)
	}
}
