use crate::{parse::*, Error, Value};

impl Value {
	pub fn parse<T>(value: T) -> Result<Value, Error>
	where
		T: Into<Value>,
	{
		let value = value.into();

		match value {
			Self::Words(ref input) => {
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
					return Ok(Self::Cardinal(float_value));
				}

				// Try parsing as ordinal symbolic
				if let Some(int_value) = ordinal_symbolic(words) {
					return Ok(Self::OrdinalSymbolic(
						int_value,
						words.to_string(),
					));
				}

				// Try parsing as ordinal words
				if let Some(int_value) = ordinal_worded(words) {
					return Ok(Self::Ordinal(int_value));
				}

				// Try parsing as percentage symbol
				if let Some((percent, _)) = percentage_symbol(words) {
					return Ok(Self::PercentageSymbol(
						percent,
						words.to_string(),
					));
				}

				// Try parsing as percentage words
				if let Some((percent, base)) =
					percentage_worded(words)
				{
					return Ok(Self::Percentage(percent, base));
				}

				// Try parsing as fraction symbolic
				if let Some((numerator, denominator)) =
					fraction_symbolic(words)
				{
					return Ok(Self::Fraction(
						numerator,
						"/".to_string(),
						denominator,
					));
				}

				// Try parsing as fraction words
				if let Some((numerator, denominator)) =
					fraction_worded(words)
				{
					return Ok(Self::Fraction(
						numerator,
						"over".to_string(),
						denominator,
					));
				}

				// Try parsing as roman numeral
				if let Some(_) = roman_symbolic(words) {
					return Ok(Self::Roman(words.to_string()));
				}

				// If parsing fails, return an error
				Err(Error::UnknownNumeral(words.to_string()))
			}
			_ => Ok(value),
		}
	}

	pub fn as_f64(&self) -> f64 {
		match self {
			Self::Cardinal(v) => *v,
			Self::OrdinalSymbolic(v, _) | Self::Ordinal(v) => {
				*v as f64
			}
			Self::PercentageSymbol(v, _) | Self::Percentage(v, _) => {
				v / 100.0
			}
			Self::Fraction(n, _, d) => *n as f64 / *d as f64,
			Self::Roman(s) => roman_symbolic(s).unwrap_or(0) as f64,
		}
	}

	pub fn as_usize(&self) -> usize {
		match self {
			Self::Cardinal(v) => *v as usize,
			Self::OrdinalSymbolic(v, _) | Self::Ordinal(v) => *v,
			Self::PercentageSymbol(v, _) | Self::Percentage(v, _) => {
				(*v / 100.0) as usize
			}
			Self::Fraction(n, _, d) => *n / *d,
			Self::Roman(s) => roman_symbolic(s).unwrap_or(0),
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

impl From<f64> for Value {
	fn from(value: f64) -> Self {
		Self::Cardinal(value)
	}
}

impl From<usize> for Value {
	fn from(value: usize) -> Self {
		Self::Ordinal(value)
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
