use crate::{Error, Input, Numeral};

impl Numeral {
  pub fn from<T>(value: T) -> Numeral
  where
    T: Into<Input>
  {
    Self::new(value.into()).unwrap()
  }

  pub fn from_f64(value: f64) -> Self {
    let mut numeral = Self::default();
    numeral.cardinal_symbol = value;
    numeral.whole = value.trunc() as usize;
    numeral.fractional = Self::fraction_from_float(value);
    numeral
  }

  pub fn from_usize(value: usize) -> Self {
    let mut numeral = Self::default();
    numeral.cardinal_symbol = value as f64;
    numeral.whole = value;
    numeral.fractional = (0, 1);
    numeral
  }

  pub fn from_str(value: &str) -> Result<Self, Error> {
    // Implement parsing logic for words here
    // For example, you might want to parse "three" into 3.0
    if let Some(parsed_value) = Self::parse_words_to_f64(value) {
      return Ok(Self::from_f64(parsed_value));
    }
    Err(Error::InvalidWordRepresentation)
  }
}
