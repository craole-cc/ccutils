pub enum Sign {
  Neg = -1,
  Pos = 1
}

impl Sign {
  pub fn to_char(self) -> char {
    match self {
      Sign::Neg => '-',
      Sign::Pos => '+'
    }
  }

  pub fn to_word(self) -> String {
    match self {
      Sign::Neg => "minus",
      Sign::Pos => ""
    }
  }
}
