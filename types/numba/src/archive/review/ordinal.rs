use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Error {
	InvalidInput,
}

#[derive(Debug, PartialEq)]
pub enum Number {
	Cardinal(usize),
	Ordinal(usize),
}

impl FromStr for Number {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		parse_number_word(s)
	}
}

fn parse_number_word(word: &str) -> Result<Number, Error> {
	use Number::*;

	match word.to_lowercase().as_str() {
		// Cardinals
		"zero" => Ok(Cardinal(0)),
		"one" | "first" => Ok(Cardinal(1)),
		"two" | "second" => Ok(Cardinal(2)),
		"three" | "third" => Ok(Cardinal(3)),
		"four" | "fourth" => Ok(Cardinal(4)),
		"five" | "fifth" => Ok(Cardinal(5)),
		// ... add more numbers as needed
		"twenty" | "twentieth" => Ok(Cardinal(20)),
		"thirty" | "thirtieth" => Ok(Cardinal(30)),
		// ... add more tens as needed
		"hundred" => Ok(Cardinal(100)),
		"thousand" => Ok(Cardinal(1000)),
		// Ordinals (only for special cases, others are handled by cardinals)
		"first" => Ok(Ordinal(1)),
		"second" => Ok(Ordinal(2)),
		"third" => Ok(Ordinal(3)),
		// ... add more special ordinals as needed
		_ => {
			// Handle compound words (e.g., "twenty-first", "fifty-second")
			if let Some((tens, ones)) = word.split_once('-') {
				let tens = parse_number_word(tens)?.as_usize();
				let ones = parse_number_word(ones)?.as_usize();
				Ok(Ordinal(tens + ones))
			} else {
				Err(Error::InvalidInput)
			}
		}
	}
}

impl Number {
	pub fn as_usize(&self) -> usize {
		match self {
			Number::Cardinal(n) | Number::Ordinal(n) => *n,
		}
	}
}

pub fn parse_number_phrase(phrase: &str) -> Result<usize, Error> {
	let words: Vec<Number> = phrase
		.split_whitespace()
		.map(|word| word.parse::<Number>())
		.collect::<Result<_, _>>()?;

	let mut result = 0;
	let mut current = 0;

	for word in words {
		match word {
			Number::Cardinal(100) => current *= 100,
			Number::Cardinal(1000) => {
				result += current * 1000;
				current = 0;
			}
			_ => current += word.as_usize(),
		}
	}

	Ok(result + current)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_number_word() {
		assert_eq!("one".parse(), Ok(Number::Cardinal(1)));
		assert_eq!("first".parse(), Ok(Number::Ordinal(1)));
		assert_eq!("twenty".parse(), Ok(Number::Cardinal(20)));
		assert_eq!("thirtieth".parse(), Ok(Number::Cardinal(30)));
	}

	#[test]
	fn test_parse_number_phrase() {
		assert_eq!(parse_number_phrase("twenty-first"), Ok(21));
		assert_eq!(
			parse_number_phrase("one hundred twenty-three"),
			Ok(123)
		);
		assert_eq!(
			parse_number_phrase(
				"one thousand two hundred thirty-four"
			),
			Ok(1234)
		);
	}

	#[test]
	fn test_invalid_input() {
		assert_eq!(
			"invalid".parse::<Number>(),
			Err(Error::InvalidInput)
		);
		assert_eq!(
			parse_number_phrase("invalid"),
			Err(Error::InvalidInput)
		);
	}
}
