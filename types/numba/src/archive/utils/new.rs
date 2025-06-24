use crate::{to_cardinal, Input, Number, Round, RoundDirection};

impl Number {
	pub fn new(input: Input) -> Result<Self, String> {
		let numeral = input.into_f64()?;
		let floor = numeral.floor() as usize;
		let ceil = numeral.ceil() as usize;
		let round = Round::default();
		// let round = Round {
		// 	value: numeral.round() as usize,
		// 	direction: RoundDirection::Nearest,
		// 	unit: 1,
		// 	precision: Some(2),
		// };

		let integer = numeral.floor() as usize;
		let fractional = fractional_part_as_usize(
			numeral,
			round.precision.unwrap_or(0), // The default is whatever the fractional part is
		);

		Ok(Self {
			numeral,
			round,
			integer,
			fractional,
			cardinal: to_cardinal(floor),
			ordinal: format!("{}th", floor),
			ordinal_abbrev: format!("{}th", floor), // Simplified for now
			decimal: format!("{}", numeral),        // Simplified for now
			fraction: format!("{}/{}", floor, 100), // Simplified for now
			percentage: format!(
				"{}%",
				(numeral * 100.0).round() / 100.0
			),
			percentage_abbrev: format!("{:}%", numeral),
		})
	}
}

fn fractional_part_as_usize(value: f64, precision: usize) -> usize {
	// if precision == 0 {
	// 	return 0;

	// } else if precision > 0 {
	// 	return value.fract().round() as usize;
	// }
	(value.fract() * 10_f64.powi(precision.try_into().unwrap()))
		.round() as usize
}
