use crate::{error::cardinal::Error, traits::cardinal::Cast};

const SMALL: [&str; 21] = [
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

const LARGE: [&str; 21] = [
  "thousand",
  "million",
  "billion",
  "trillion",
  "quadrillion",
  "quintillion",
  "sextillion",
  "septillion",
  "octillion",
  "nonillion",
  "decillion",
  "undecillion",
  "duodecillion",
  "tredecillion",
  "quattuordecillion",
  "quindecillion",
  "sexdecillion",
  "septendecillion",
  "octodecillion",
  "novemdecillion",
  "vigintillion"
];

pub fn cast<T: Cast>(value: T) -> Result<(f64, String), Error> {
  value.cast()
}

pub fn numeric_to_worded(input: &str) -> Result<f64, Error> {
  let mut result = 0i128;
  let mut current_number = 0i128;
  let mut is_negative = false;
  let mut is_fractional = false;
  let mut fractional_part = 0u64;
  let mut fractional_digits = 0;
  let mut last_multiplier = i128::MAX;

  let words: Vec<&str> = input
    .split(|c: char| c.is_whitespace() || c == '-')
    .map(|w| w.trim())
    .filter(|&w| !w.is_empty() && w != "and")
    .collect();

  let mut i = 0;
  while i < words.len() {
    let word = words[i].to_lowercase();

    if i == 0 && (word == "minus" || word == "negative") {
      is_negative = true;
      i += 1;
      continue;
    }

    if word == "point" {
      result += current_number;
      current_number = 0;
      is_fractional = true;
      i += 1;
      continue;
    }

    if word == "hundred" {
      if i > 0 {
        current_number *= 100;
      }
      i += 1;
      continue;
    }

    if let Some(value) = word_to_value(&word) {
      let value = i128::try_from(value).map_err(|_| Error::InvalidScale)?;

      if is_fractional {
        fractional_part = fractional_part * 10 + u64::try_from(value).unwrap_or(0);
        fractional_digits += 1;
      } else {
        // Check if it's a large number (thousand or greater)
        if value >= 1000 {
          let scale = value;

          // If current_number is 0, use 1 as the multiplier
          let multiplier = if current_number == 0 { 1 } else { current_number };

          // Add the scaled value to result
          result += multiplier * scale;

          // Reset current_number and update last_multiplier
          current_number = 0;
          last_multiplier = scale;
        } else {
          // Handle compound numbers (like twenty-one)
          if i + 1 < words.len() && words[i + 1].to_lowercase() != "hundred" {
            if let Some(next_value) = word_to_value(&words[i + 1].to_lowercase()) {
              let next_value = i128::try_from(next_value).map_err(|_| Error::InvalidScale)?;
              if next_value < 10 {
                current_number += value + next_value;
                i += 1;
              } else {
                current_number += value;
              }
            } else {
              current_number += value;
            }
          } else {
            current_number += value;
          }
        }
      }
    } else {
      return Err(Error::FailedToParseWords(word.to_string()));
    }

    i += 1;
  }

  // Add any remaining current_number to result
  result += current_number;

  let final_result = result as f64 + (fractional_part as f64 / 10f64.powi(fractional_digits));

  Ok(if is_negative { -final_result } else { final_result })
}

pub fn worded_from_numeric(value: f64) -> Result<String, Error> {
  let (integer, fraction) = split_float(value);
  let integer_worded = worded_from_isize(integer)?;

  if fraction.is_empty() {
    Ok(integer_worded)
  } else {
    Ok(format!("{} point {}", integer_worded, fraction_to_words(&fraction)?))
  }
}

fn split_float(value: f64) -> (i128, String) {
  let value_str = format!("{:.}", value);
  let parts: Vec<&str> = value_str.split('.').collect();
  let integer = parts[0].parse().unwrap_or(0);
  let fraction = parts.get(1).map(|&s| s.to_string()).unwrap_or_default();
  (integer, fraction)
}

fn fraction_to_words(fraction: &str) -> Result<String, Error> {
  fraction
    .chars()
    .map(|c| worded_from_digit(c).ok_or(Error::InvalidDigit))
    .collect::<Result<Vec<_>, _>>()
    .map(|v| v.join(" "))
}

fn worded_from_digit(digit: char) -> Option<String> {
  digit.to_digit(10).map(|d| SMALL[d as usize].to_string())
}

fn worded_from_isize(value: i128) -> Result<String, Error> {
  if value == 0 {
    return Ok("zero".to_string());
  }

  let is_negative = value < 0;
  let mut abs_value = value.unsigned_abs();

  let mut result = Vec::new();
  let mut group_index = 0;

  while abs_value > 0 {
    let group = (abs_value % 1000) as isize;
    abs_value /= 1000;

    if group != 0 {
      let mut group_str = three_digit_group(group)?;
      if group_index > 0 {
        group_str.push(' ');
        group_str.push_str(LARGE.get(group_index).ok_or(Error::InvalidScale)?);
      }
      result.push(group_str);
    }

    group_index += 1;
  }

  result.reverse();
  let words = result.join(" ");

  Ok(if is_negative { format!("minus {}", words) } else { words })
}

fn three_digit_group(value: isize) -> Result<String, Error> {
  let hundreds = value / 100;
  let remainder = value % 100;

  let mut result = Vec::new();

  if hundreds > 0 {
    result.push(format!(
      "{} hundred",
      SMALL.get(hundreds as usize).ok_or(Error::InvalidScale)?
    ));
  }

  if remainder > 0 {
    if hundreds > 0 {
      result.push("and".to_string());
    }
    result.push(tens_and_ones(remainder)?);
  }

  Ok(result.join(" "))
}

fn tens_and_ones(value: isize) -> Result<String, Error> {
  match value {
    0..=20 => Ok(SMALL.get(value as usize).ok_or(Error::InvalidScale)?.to_string()),
    21..=99 => {
      let tens = value / 10;
      let ones = value % 10;
      if ones == 0 {
        Ok(TENS.get(tens as usize - 2).ok_or(Error::InvalidScale)?.to_string())
      } else {
        Ok(format!(
          "{}-{}",
          TENS.get(tens as usize - 2).ok_or(Error::InvalidScale)?,
          SMALL.get(ones as usize).ok_or(Error::InvalidScale)?
        ))
      }
    }
    _ => Err(Error::InvalidScale)
  }
}

fn word_to_value(word: &str) -> Option<u128> {
  SMALL
    .iter()
    .position(|&w| w == word)
    .map(|i| i as u128)
    .or_else(|| TENS.iter().position(|&w| w == word).map(|i| (i as u128 + 2) * 10))
    .or_else(|| {
      LARGE
        .iter()
        .position(|&w| w == word)
        .map(|i| if i == 0 { 0 } else { 1000u128.pow(i as u32) })
    })
}
