use crate::Input;
use std::{collections::HashMap, str::FromStr};

impl Input {
	/// Convert `Input` enum variants into an `f64`.
	pub fn into_f64(self) -> Result<f64, String> {
		match self {
			Input::Fractional(v) => Ok(v),
			Input::Integer(v) => Ok(v as f64),
			Input::Words(s) => s.parse::<f64>().map_err(|_| {
				format!("Failed to parse '{}' as a number", s)
			}),
		}
	}
}

impl From<f64> for Input {
	fn from(value: f64) -> Self {
		Input::Fractional(value)
	}
}

impl From<usize> for Input {
	fn from(value: usize) -> Self {
		Input::Integer(value)
	}
}

impl From<&str> for Input {
	fn from(value: &str) -> Self {
		Input::Words(value.to_string())
	}
}

impl From<String> for Input {
	fn from(value: String) -> Self {
		Input::Words(value)
	}
}

fn parse_to_f64<T: AsRef<str>>(input: T) -> Result<f64, String> {
	let input_str = input.as_ref().to_lowercase();

	// First, try parsing as a regular number
	if let Ok(num) = f64::from_str(&input_str) {
		return Ok(num);
	}

	// If that fails, try parsing as a word form
	let word_to_number = create_word_to_number_map();

	if let Some(num) = word_to_number.get(&input_str) {
		return Ok(*num as f64);
	}

	// If it's an ordinal number word, try converting to cardinal and then to number
	let ordinal_suffix = ["st", "nd", "rd", "th"];
	if ordinal_suffix
		.iter()
		.any(|&suffix| input_str.ends_with(suffix))
	{
		let without_suffix =
			input_str.trim_end_matches(char::is_alphabetic);
		if let Some(num) = word_to_number.get(without_suffix) {
			return Ok(*num as f64);
		}
	}

	// If all parsing attempts fail, return an error
	Err(format!("Failed to parse '{}' to a number", input_str))
}

fn create_word_to_number_map() -> HashMap<String, u64> {
	let mut map = HashMap::new();
	let words = [
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
		"twenty",
		"thirty",
		"forty",
		"fifty",
		"sixty",
		"seventy",
		"eighty",
		"ninety",
		"hundred",
		"thousand",
		"million",
		"billion",
		"trillion",
	];
	let values = [
		0,
		1,
		2,
		3,
		4,
		5,
		6,
		7,
		8,
		9,
		10,
		11,
		12,
		13,
		14,
		15,
		16,
		17,
		18,
		19,
		20,
		30,
		40,
		50,
		60,
		70,
		80,
		90,
		100,
		1000,
		1_000_000,
		1_000_000_000,
		1_000_000_000_000,
	];
	words.iter().zip(values.iter()).for_each(|(word, &value)| {
		map.insert(word.to_string(), value);
	});
	map
}
