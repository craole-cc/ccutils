use crate::{Input, Numeral};

impl Numeral {
	pub fn new<N: Input>(
		input: N,
	) -> Result<Self, String> {
		let numeral = input.into_number()?;
		let integer = numeral.floor() as usize;
		let fractional = numeral.fract() as usize;
		let rounding = crate::Rounding::default();
		let cardinal = crate::cardinal_to_integer(&integer.to_string()).to_string();
		let ordinal = format!("{}th", integer);
		let ordinal_abbrev = format!("{}th", integer);
		let decimal = format!("{}", numeral);
		let fraction = format!("{}/{}", integer, fractional);
		let percentage = format!("{}%", numeral * 100.0);
		let percentage_abbrev = format!("{}%", numeral * 100.0);

		Ok(Self {
			
		})
	}
}
