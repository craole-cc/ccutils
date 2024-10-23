use crate::{Input, Numeral};

impl From<f64> for Numeral {
	fn from(value: f64) -> Numeral {
		Self::new(Input::Decimal(value)).unwrap()
	}
}

impl From<usize> for Numeral {
	fn from(value: usize) -> Self {
		Self::new(Input::Integer(value)).unwrap()
	}
}

impl From<&str> for Numeral {
	fn from(value: &str) -> Self {
		Self::new(Input::Words(value.to_string())).unwrap()
	}
}

impl From<String> for Numeral {
	fn from(value: String) -> Self {
		Self::new(Input::Words(value)).unwrap()
	}
}
