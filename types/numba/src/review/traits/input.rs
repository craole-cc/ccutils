use crate::Input;

impl From<f64> for Input {
  fn from(value: f64) -> Self {
      Self::Decimal(value)
  }
}

impl From<usize> for Input {
  fn from(value: usize) -> Self {
      Self::Integer(value)
  }
}

impl From<&str> for Input {
  fn from(value: &str) -> Self {
      Self::Words(value.to_string())
  }
}

impl From<String> for Input {
  fn from(value: String) -> Self {
      Self::Words(value)
  }
}
