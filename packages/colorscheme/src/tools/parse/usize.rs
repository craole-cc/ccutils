use std::{collections::HashMap, convert::From};

#[derive(Debug)]
pub struct OrdinalNumber(usize);

impl OrdinalNumber {
  // The value can be accessed via this method if needed
  pub fn value(&self) -> usize {
    self.0
  }
}

impl From<&str> for OrdinalNumber {
  fn from(s: &str) -> Self {
    let value = to_usize(s);
    OrdinalNumber(value)
  }
}

impl From<String> for OrdinalNumber {
  fn from(s: String) -> Self {
    OrdinalNumber::from(s.as_str())
  }
}

/// Converts a word representing a number into a usize.
pub fn to_usize<S: AsRef<str>>(word: S) -> usize {
  let word = word.as_ref();

  // Define mappings for cardinal numbers
  let mut cardinal_map: HashMap<&str, usize> = HashMap::new();

  // Populate the HashMap with unit values
  cardinal_map.insert("one", 1);
  cardinal_map.insert("two", 2);
  cardinal_map.insert("three", 3);
  cardinal_map.insert("four", 4);
  cardinal_map.insert("five", 5);
  cardinal_map.insert("six", 6);
  cardinal_map.insert("seven", 7);
  cardinal_map.insert("eight", 8);
  cardinal_map.insert("nine", 9);
  cardinal_map.insert("ten", 10);
  cardinal_map.insert("eleven", 11);
  cardinal_map.insert("twelve", 12);
  cardinal_map.insert("thirteen", 13);
  cardinal_map.insert("fourteen", 14);
  cardinal_map.insert("fifteen", 15);
  cardinal_map.insert("sixteen", 16);
  cardinal_map.insert("seventeen", 17);
  cardinal_map.insert("eighteen", 18);
  cardinal_map.insert("nineteen", 19);

  // Populate the HashMap with tens values
  cardinal_map.insert("twenty", 20);
  cardinal_map.insert("thirty", 30);
  cardinal_map.insert("forty", 40);
  cardinal_map.insert("fifty", 50);
  cardinal_map.insert("sixty", 60);
  cardinal_map.insert("seventy", 70);
  cardinal_map.insert("eighty", 80);
  cardinal_map.insert("ninety", 90);

  // Populate the HashMap with hundreds values
  cardinal_map.insert("hundred", 100);
  cardinal_map.insert("thousand", 1000);

  // Define mappings for ordinal numbers
  let mut ordinal_map: HashMap<&str, usize> = HashMap::new();

  // Populate the HashMap with ordinal values
  ordinal_map.insert("first", 1);
  ordinal_map.insert("second", 2);
  ordinal_map.insert("third", 3);
  ordinal_map.insert("fourth", 4);
  ordinal_map.insert("fifth", 5);
  ordinal_map.insert("sixth", 6);
  ordinal_map.insert("seventh", 7);
  ordinal_map.insert("eighth", 8);
  ordinal_map.insert("ninth", 9);
  ordinal_map.insert("tenth", 10);
  ordinal_map.insert("eleventh", 11);
  ordinal_map.insert("twelfth", 12);
  ordinal_map.insert("thirteenth", 13);
  ordinal_map.insert("fourteenth", 14);
  ordinal_map.insert("fifteenth", 15);
  ordinal_map.insert("sixteenth", 16);
  ordinal_map.insert("seventeenth", 17);
  ordinal_map.insert("eighteenth", 18);
  ordinal_map.insert("nineteenth", 19);

  // Populate the ordinal values for tens
  ordinal_map.insert("twentieth", 20);
  ordinal_map.insert("thirtieth", 30);
  ordinal_map.insert("fortieth", 40);
  ordinal_map.insert("fiftieth", 50);
  ordinal_map.insert("sixtieth", 60);
  ordinal_map.insert("seventieth", 70);
  ordinal_map.insert("eightieth", 80);
  ordinal_map.insert("ninetieth", 90);

  // Handle hyphenated ordinals (like "fifty-first")
  if let Some(pos) = word.find('-') {
    let (left, right) = word.split_at(pos);
    let right = &right[1..]; // Skip the hyphen

    if let Some(left_value) = cardinal_map.get(left.trim())
      && let Some(right_value) = ordinal_map.get(right.trim())
    {
      return left_value + right_value; // Combine the values
    }
  }

  // Split the input to check for combinations of numbers
  let parts: Vec<&str> = word.split_whitespace().collect();
  let mut total = 0;
  let mut current_value = 0;

  for part in parts {
    if let Some(&value) = cardinal_map.get(part) {
      if part == "hundred" {
        current_value *= value; // Multiply current value by 100
      } else if part == "thousand" {
        total += current_value * value; // Add current value multiplied by 1000
        current_value = 0; // Reset current value
      } else {
        current_value += value; // Add the current unit, ten, or hundreds
      }
    } else if let Some(&value) = ordinal_map.get(part) {
      current_value += value; // Ordinals treated as their cardinal equivalent
    }
  }

  total += current_value; // Add any remaining value

  // Return the result
  total
}
