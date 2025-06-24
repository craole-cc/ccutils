fn cardinal_words_from_numeric(value: f64) -> String {
  let (integer, fraction) = split_float(value);
  let integer_worded = cardinal_words_from_isize(integer);

  if fraction.is_empty() {
    integer_worded
  } else {
    format!("{} point {}", integer_worded, fraction_to_words(&fraction))
  }
}

fn split_float(value: f64) -> (isize, String) {
  let value_str = format!("{:.}", value);
  let parts: Vec<&str> = value_str.split('.').collect();
  let integer = parts[0].parse().unwrap_or(0);
  let fraction = parts.get(1).map(|&s| s.to_string()).unwrap_or_default();
  (integer, fraction)
}

fn fraction_to_words(fraction: &str) -> String {
  fraction
    .chars()
    .map(cardinal_words_from_digit)
    .collect::<Vec<_>>()
    .join(" ")
}

fn cardinal_words_from_isize(value: isize) -> String {
  let abs_value = value.abs();
  let words = match abs_value {
    0 => "zero".to_string(),
    1..=20 => SMALL_NUMBERS[abs_value as usize].to_string(),
    21..=99 => {
      let tens = abs_value / 10;
      let ones = abs_value % 10;
      if ones == 0 {
        TENS[tens as usize - 2].to_string()
      } else {
        format!(
          "{}-{}",
          TENS[tens as usize - 2],
          SMALL_NUMBERS[ones as usize]
        )
      }
    }
    _ => abs_value.to_string() // Fallback for larger numbers
  };

  if value < 0 {
    format!("minus {}", words)
  } else {
    words
  }
}

fn cardinal_words_from_digit(digit: char) -> String {
  SMALL_NUMBERS[digit.to_digit(10).unwrap_or(0) as usize].to_string()
}

fn parse_worded_number(input: &str) -> Result<f64, &'static str> {
  let mut result = 0.0;
  let mut current_number = 0.0;
  let mut is_negative = false;
  let mut is_fractional = false;
  let mut fractional_multiplier = 0.1;

  for (i, word) in input.split([' ', '-']).enumerate() {
    match word.to_lowercase().as_str() {
      "minus" | "negative" if i == 0 => is_negative = true,
      "point" => {
        result += current_number;
        current_number = 0.0;
        is_fractional = true;
      }
      _ =>
        if let Some(digit) = word_to_digit(word) {
          if is_fractional {
            result += digit as f64 * fractional_multiplier;
            fractional_multiplier *= 0.1;
          } else {
            current_number = current_number * 10.0 + digit as f64;
          }
        } else {
          return Err("Invalid word in number");
        },
    }
  }

  result += current_number;
  Ok(if is_negative { -result } else { result })
}

fn word_to_digit(word: &str) -> Option<u8> {
  SMALL_NUMBERS
    .iter()
    .position(|&w| w == word)
    .map(|i| i as u8)
    .or_else(|| {
      TENS
        .iter()
        .position(|&w| w == word)
        .map(|i| (i + 2) as u8 * 10)
    })
}

const SMALL_NUMBERS: [&str; 21] = [
  "zero",
  "one",
  "two",
  "three",
  "four",
  "five",
  "six",
  "seven",
  "eight",
  "nine",
  "ten",
  "eleven",
  "twelve",
  "thirteen",
  "fourteen",
  "fifteen",
  "sixteen",
  "seventeen",
  "eighteen",
  "nineteen",
  "twenty"
];

const TENS: [&str; 8] = [
  "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"
];
