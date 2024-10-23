use crate::{Error, Numeral, Value};

impl Numeral {
	pub fn new<T>(value: T) -> Result<Self, Error>
	where
		T: Into<Value>,
	{
		let input = Value::parse(value.into())?;

		let mut numeral = Self::default();
		numeral.input = input;

		Ok(numeral)
	}

	pub fn from<T>(value: T) -> Result<Self, Error>
	where
		T: Into<Value>,
	{
		Self::new(value) // Directly call new with converted input
	}
}
