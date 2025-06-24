use crate::{Error, Value, Numeral};

impl Value {
	pub fn parse<T>(value: T) -> Result<Value, Error>
	where
		T: Into<Value>,
	{
		let value = value.into();

		if let Value::Words(ref words) = value {
  				if let Ok(value) = words.parse::<f64>() {
  					return Ok(Value::Decimal(value));
  				} else if let Ok(value) = words.parse::<usize>() {
  					return Ok(Value::Integer(value));
  				}
  			}

		Ok(value)
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
