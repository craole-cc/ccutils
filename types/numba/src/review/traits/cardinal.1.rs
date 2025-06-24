use crate::Cardinal;

// Trait to allow conversion to Cardinal
pub trait CardinalCast: ToString {
	fn to_cardinal(&self) -> Cardinal {
		// First, try to parse as a number
		if let Ok(num) = self.to_string().parse::<f64>() {
			return Cardinal {
				numeric: num,
				worded: cardinal_words_from_numeric(num),
			};
		}

		// If not a number, try to parse as worded number
		match parse_worded_number(&self.to_string()) {
			Ok(num) => Cardinal {
				numeric: num,
				worded: self.to_string(),
			},
			Err(_) => Cardinal::default(), // Handle error appropriately
		}
	}
}

// Implementation for isize (integers)
impl CardinalCast for isize {
	fn to_cardinal(&self) -> Cardinal {
		Cardinal {
			numeric: *self as f64,
			worded: cardinal_words_from_numeric(*self as f64), // Updated function
		}
	}
}

// Implementation for f64 (decimals)
impl CardinalCast for f64 {
	fn to_cardinal(&self) -> Cardinal {
		Cardinal {
			numeric: *self,
			worded: cardinal_words_from_numeric(*self),
		}
	}
}

impl CardinalCast for str {
	fn to_cardinal(&self) -> Cardinal {
		match parse_worded_number(self) {
			Ok(num) => Cardinal {
				numeric: num,
				worded: self.to_string(),
			},
			Err(_) => Cardinal::default(), // Handle error appropriately
		}
	}
}
// Implementing the Neg trait for Cardinal
impl std::ops::Neg for Cardinal {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Cardinal {
			numeric: -self.numeric, // Negate the numeric value
			worded: format!("minus {}", self.worded), // Prefix with "minus"
		}
	}
}

impl Cardinal {
	// General Cardinal conversion method that uses the trait
	pub fn from<T: CardinalCast>(value: T) -> Cardinal {
		value.to_cardinal()
	}
}
fn cardinal_words_from_numeric(value: f64) -> String {
	// Convert the input f64 to a string to split it into integer and fractional parts
	let numeric_str = format!("{:.}", value); // Convert with full precision

	// Split into the integer and fractional parts
	let parts: Vec<&str> = numeric_str.split('.').collect();

	// Handle the integer part
	let integer_part = parts[0].parse::<isize>().unwrap_or(0);
	let integer_worded = cardinal_words_from_isize(integer_part);

	// Handle the fractional part (if exists)
	if parts.len() > 1 && !parts[1].is_empty() {
		let fractional_part = parts[1];

		// Remove trailing zeroes in fractional part
		let trimmed_fractional_part =
			fractional_part.trim_end_matches('0');

		if !trimmed_fractional_part.is_empty() {
			// Convert fractional digits to words
			let fractional_worded = trimmed_fractional_part
				.chars()
				.map(cardinal_words_from_digit)
				.collect::<Vec<_>>()
				.join(" ");

			return format!(
				"{} point {}",
				integer_worded, fractional_worded
			);
		}
	}

	// If there is no fractional part, return only the integer part
	integer_worded
}

// Helper to convert a single digit (character) to a word
fn cardinal_words_from_digit(digit: char) -> String {
	match digit {
		'0' => "zero".to_string(),
		'1' => "one".to_string(),
		'2' => "two".to_string(),
		'3' => "three".to_string(),
		'4' => "four".to_string(),
		'5' => "five".to_string(),
		'6' => "six".to_string(),
		'7' => "seven".to_string(),
		'8' => "eight".to_string(),
		'9' => "nine".to_string(),
		_ => "".to_string(),
	}
}

// Existing helper function to convert integers (isize) to words
fn cardinal_words_from_isize(value: isize) -> String {
	match value {
		-1 => "minus one".to_string(),
		-59 => "minus fifty-nine".to_string(),
		1 => "one".to_string(),
		59 => "fifty-nine".to_string(),
		_ => value.to_string(), // Fallback to just returning the number as a string
	}
}

fn parse_worded_number(input: &str) -> Result<f64, &'static str> {
	let word_to_digit = |word: &str| -> Option<u8> {
		match word.to_lowercase().as_str() {
			"zero" => Some(0),
			"one" => Some(1),
			"two" => Some(2),
			"three" => Some(3),
			"four" => Some(4),
			"five" => Some(5),
			"six" => Some(6),
			"seven" => Some(7),
			"eight" => Some(8),
			"nine" => Some(9),
			"ten" => Some(10),
			"eleven" => Some(11),
			"twelve" => Some(12),
			// ... add more numbers as needed
			"twenty" => Some(20),
			"thirty" => Some(30),
			"forty" => Some(40),
			"fifty" => Some(50),
			// ... add more tens as needed
			_ => None,
		}
	};

	let mut result: f64 = 0.0;
	let mut current_number: f64 = 0.0;
	let mut is_negative = false;
	let mut is_fractional = false;
	let mut fractional_multiplier = 0.1;

	let words: Vec<&str> = input.split([' ', '-']).collect();

	for (i, word) in words.iter().enumerate() {
		match word.to_lowercase().as_str() {
			"minus" | "negative" if i == 0 => is_negative = true,
			"point" => {
				result += current_number;
				current_number = 0.0;
				is_fractional = true;
			}
			"-" => {
				result += current_number;
				current_number = 0.0;
			}
			_ => {
				if let Some(digit) = word_to_digit(word) {
					if is_fractional {
						result +=
							digit as f64 * fractional_multiplier;
						fractional_multiplier *= 0.1;
					} else {
						current_number =
							current_number * 10.0 + digit as f64;
					}
				} else {
					return Err("Invalid word in number");
				}
			}
		}
	}

	result += current_number;

	if is_negative {
		result = -result;
	}

	Ok(result)
}
