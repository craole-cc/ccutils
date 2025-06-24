use crate::Numeral;

impl Numeral {
  pub fn fraction_from_float(value: f64) -> (usize, usize) {
    // Logic to convert float to fractional representation
    // Example: 0.75 -> (3, 4)
    let whole_part = value.trunc() as usize;
    let fractional_part = value.fract();

    // Convert fractional part to fraction
    // This is a placeholder; you'll need actual logic here.
    if fractional_part == 0.0 {
      return (0, 1); // Represents whole number
    }

    // Example conversion logic; this could be improved
    let denominator = 100; // Assuming two decimal places for simplicity
    let numerator = (fractional_part * denominator as f64).round() as usize;

    (whole_part * denominator + numerator, denominator)
  }

  pub fn update_representations(&mut self) {
    self.cardinal_worded = self.to_cardinal_worded(self.cardinal_symbol);
    self.ordinal_worded = self.to_ordinal_worded(self.whole);
    self.ordinal_symbol = self.to_ordinal_symbol(self.whole);
    self.fraction_symbol =
      format!("{}/{}", self.fractional.0, self.fractional.1);
    self.fraction_worded = self.to_fraction_worded(self.whole, self.fractional);
    self.percentage_symbol = format!("{:.1}%", self.cardinal_symbol * 100.0);
    self.percentage_worded = self.to_percentage_worded(self.cardinal_symbol);
    self.roman_symbol = self.to_roman_numeral(self.whole);
  }

  // Implement methods for converting numbers to word forms...

  pub fn parse_words_to_f64(words: &str) -> Option<f64> {
    // Implement logic to convert words to f64
    match words.trim() {
      "zero" => Some(0.0),
      "one" => Some(1.0),
      "two" => Some(2.0),
      "three" => Some(3.0),
      "three point five" => Some(3.5),
      _ => None
    }
  }

  fn to_cardinal_worded(&self, value: f64) -> String {
    // Implement logic to convert cardinal number to word form
    format!("Cardinal word for {}", value) // Placeholder
  }

  fn to_ordinal_worded(&self, whole: usize) -> String {
    // Implement logic to convert whole number to ordinal word form
    format!("Ordinal word for {}", whole) // Placeholder
  }

  fn to_ordinal_symbol(&self, whole: usize) -> String {
    // Implement logic to convert whole number to ordinal symbol
    format!(
      "{}{}",
      whole,
      match whole % 10 {
        1 if whole % 100 != 11 => "st",
        2 if whole % 100 != 12 => "nd",
        3 if whole % 100 != 13 => "rd",
        _ => "th"
      }
    )
  }

  fn to_fraction_worded(
    &self,
    whole: usize,
    fractional: (usize, usize)
  ) -> String {
    // Implement logic to convert fraction into word form
    format!(
      "Fraction word for {} {}/{}",
      whole, fractional.0, fractional.1
    ) // Placeholder
  }

  fn to_percentage_worded(&self, cardinal: f64) -> String {
    // Implement logic to convert cardinal number into percentage word form
    format!("Percentage word for {}", cardinal * 100.0) // Placeholder
  }

  fn to_roman_numeral(&self, whole: usize) -> String {
    // Implement logic to convert whole number into Roman numerals
    format!("Roman numeral for {}", whole) // Placeholder
  }
}
