use crate::types::numeral::Numeral;
use astro_float::BigFloat;

impl Numeral {
	pub fn new<T>(numeral: T) -> Self
	where
		T: Into<BigFloat>,
	{
		Self {
			value: numeral.into(),
		}
	}
}
