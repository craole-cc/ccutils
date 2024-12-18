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
		to_number(s)
	}
}

fn to_number(word: &str) -> Result<Number, Error> {
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
				let tens = to_number(tens)?.as_usize();
				let ones = to_number(ones)?.as_usize();
				Ok(Ordinal(tens + ones))
			} else {
				Err(Error::InvalidInput)
			}
		}
	}
}
