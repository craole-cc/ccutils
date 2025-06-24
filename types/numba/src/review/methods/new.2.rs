use crate::{Error, Numeral, Value};

impl Numeral {
  pub fn new<T>(value: T) -> Result<Self, Error>
  where
    T: Into<Value>
  {
    let mut numeral = Self::default();
    let input = Value::parse(value.into())?;

    // Input &
    match input {
      Value::Decimal(value) => {
        numeral.input = input;
      }
      Value::Integer(value) => {
        numeral.input = input;
      }
      Value::Words(ref words) => {
        if let Some(value) = Self::parse_words_to_f64(words) {
          numeral.input = input;
        } else {
          return Err(Error::InvalidWordRepresentation);
        }
      }
    }

    Ok(numeral)
  }

  pub fn from<T>(value: T) -> Result<Self, Error>
  where
    T: Into<Value>
  {
    Self::new(value) // Directly call new with converted input
  }
}
