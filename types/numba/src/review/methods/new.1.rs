use crate::{Error, Input, Numeral};

impl Numeral {
	pub fn new(input: Input) -> Result<Self, Error> {
		let mut numeral = Self::default();

		match input {
			Input::Decimal(value) => {
				numeral.input = input;
				numeral.cardinal_symbol = value;
				// numeral.whole = value.trunc() as usize;
				// numeral.fractional = Self::fraction_from_float(value);
			}
			Input::Integer(value) => {
				numeral.input = input;
				numeral.cardinal_symbol = value as f64;
				// numeral.whole = value;
				// numeral.fractional = (0, 1);
			}
			Input::Words(ref words) => {
				if let Some(value) = Self::parse_words_to_f64(words) {
					numeral.input = input;
					numeral.cardinal_symbol = value;
					// numeral.whole = value.trunc() as usize;
					// numeral.fractional =
					// 	Self::fraction_from_float(value);
				} else {
					return Err(Error::InvalidWordRepresentation);
				}
			}
		}

		numeral.whole = numeral.cardinal_symbol.trunc() as usize;
		numeral.fractional = Self::fraction_from_float(numeral.cardinal_symbol);
		// numeral.update_representations();
		Ok(numeral)
	}
}
