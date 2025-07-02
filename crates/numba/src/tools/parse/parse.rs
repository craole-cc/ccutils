pub fn numeral<S: AsRef<str>>(words: S) -> Option<(f64, String)> {
  let words_ref = words.as_ref();

  // Attempt to parse as f64 directly
  if let Ok(value) = words_ref.parse::<f64>() {
    return Some((value, "f64".to_string()));
  }

  // Attempt to parse as usize directly
  if let Ok(value) = words_ref.parse::<usize>() {
    return Some((value as f64, "usize".to_string())); // Cast usize to f64
  }

  // Attempt to parse as f32 directly
  if let Ok(value) = words_ref.parse::<f32>() {
    return Some((value as f64, "f32".to_string())); // Cast f32 to f64
  }

  // Attempt to parse as a cardinal number
  if let Some(value) = cardinal_worded(words_ref) {
    return Some((value, "cardinal".to_string()));
  }

  // Attempt to parse as an ordinal number
  if let Some(value) = ordinal_worded(words_ref) {
    return Some((value as f64, "ordinal".to_string())); // Cast usize to f64
  }

  // Attempt to parse as a fraction (worded)
  if let Some((numerator, denominator)) = fraction_worded(words_ref) {
    return Some((
      numerator as f64 / denominator as f64,
      "fraction".to_string()
    ));
  }

  // Attempt to parse as a fraction (symbol)
  if let Some((numerator, denominator)) = fraction_symbolic(words_ref) {
    return Some((
      numerator as f64 / denominator as f64,
      "fraction_symbol".to_string()
    ));
  }

  // Attempt to parse as a percentage (worded)
  if let Some((percent, _)) = percentage_worded(words_ref) {
    return Some((percent, "percentage".to_string()));
  }

  // Attempt to parse as a percentage (symbol)
  if let Some((percent, _)) = percentage_symbol(words_ref) {
    return Some((percent, "percentage_symbol".to_string()));
  }

  // Attempt to parse as a Roman numeral
  if let Some(value) = roman_symbolic(words_ref) {
    return Some((value as f64, "roman".to_string())); // Cast usize to f64
  }

  None // Return None if no parsing was successful
}

pub fn cardinal_worded<S: AsRef<str>>(words: S) -> Option<f64> {
  match words.as_ref() {
    "zero" => Some(0.0),
    "one" => Some(1.0),
    "one point one" => Some(1.1),
    "one point zero one" => Some(1.01),
    "one point zero two" => Some(1.02),
    "one point zero three" => Some(1.03),
    "one point zero four" => Some(1.04),
    "one point zero five" => Some(1.05),
    "one point zero six" => Some(1.06),
    "one point zero seven" => Some(1.07),
    "one point zero eight" => Some(1.08),
    "one point zero nine" => Some(1.09),
    "one point one" => Some(1.1),
    "one point two" => Some(1.2),
    "one point three" => Some(1.3),
    "one point four" => Some(1.4),
    "one point five" => Some(1.5),
    "one point six" => Some(1.6),
    "one point seven" => Some(1.7),
    "one point eight" => Some(1.8),
    "one point nine" => Some(1.9),
    _ => None
  }
}

pub fn ordinal_worded<S: AsRef<str>>(words: S) -> Option<usize> {
  match words.as_ref() {
    "first" => Some(1),
    "second" => Some(2),
    "third" => Some(3),
    "fourth" => Some(4),
    "fifth" => Some(5),
    "sixth" => Some(6),
    "seventh" => Some(7),
    "eighth" => Some(8),
    "ninth" => Some(9),
    "tenth" => Some(10),
    "eleventh" => Some(11),
    "twelfth" => Some(12),
    "thirteenth" => Some(13),
    "fourteenth" => Some(14),
    "fifteenth" => Some(15),
    "sixteenth" => Some(16),
    "seventeenth" => Some(17),
    "eighteenth" => Some(18),
    "nineteenth" => Some(19),
    "twentieth" => Some(20),
    "thirtieth" => Some(30),
    "fortieth" => Some(40),
    "fiftieth" => Some(50),
    "sixtieth" => Some(60),
    "seventieth" => Some(70),
    "eightieth" => Some(80),
    "ninetieth" => Some(90),
    _ => None
  }
}

pub fn ordinal_symbolic<S: AsRef<str>>(words: S) -> Option<usize> {
  match words.as_ref() {
    "1st" => Some(1),
    "2nd" => Some(2),
    "3rd" => Some(3),
    "4th" => Some(4),
    _ => None
  }
}

pub fn fraction_symbolic<S: AsRef<str>>(words: S) -> Option<(usize, usize)> {
  match words.as_ref() {
    "1/2" => Some((1, 2)),
    "1/3" => Some((1, 3)),
    "1/4" => Some((1, 4)),
    _ => None
  }
}

pub fn fraction_worded<S: AsRef<str>>(words: S) -> Option<(usize, usize)> {
  match words.as_ref() {
    "one half" => Some((1, 2)),
    "one third" => Some((1, 3)),
    "one fourth" => Some((1, 4)),
    "one over five" => Some((1, 5)),
    "eight over six" => Some((8, 6)),
    _ => None
  }
}

pub fn percentage_symbol<S: AsRef<str>>(words: S) -> Option<(f64, usize)> {
  let percent = match words.as_ref() {
    "100%" => 100.0,
    "1.67%" => 1.67,
    _ => return None
  };

  Some((percent, 100))
}

pub fn percentage_worded<S: AsRef<str>>(words: S) -> Option<(f64, usize)> {
  let percent = match words.as_ref() {
    "one hundred percent" => 100.0,
    "one point six seven percent" => 1.67,
    _ => return None
  };

  Some((percent, 100))
}

pub fn roman_symbolic<S: AsRef<str>>(words: S) -> Option<usize> {
  match words.as_ref().to_lowercase().as_str() {
    "i" => Some(1),
    "ii" => Some(2),
    "iii" => Some(3),
    "iv" => Some(4),
    "v" => Some(5),
    "vi" => Some(6),
    "vii" => Some(7),
    "viii" => Some(8),
    "ix" => Some(9),
    _ => None
  }
}
