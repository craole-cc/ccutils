use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Error {
	InvalidInput,
}

#[derive(Default, Debug, PartialEq)]
pub enum Numver {
	#[default]
	Cardinal(usize),
	Ordinal(usize),
}

impl FromStr for Numver {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Numver::to_numver(s)
	}
}

impl fmt::Display for Numver {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.to_words())
	}
}

impl Numver {
	pub fn as_usize(&self) -> usize {
		match self {
			Self::Cardinal(n) | Self::Ordinal(n) => *n,
		}
	}

	pub fn to_numver(word: &str) -> Result<Self, Error> {
		match word.to_lowercase().as_str() {
			"zero" => Ok(Self::Cardinal(0)),
			"one" => Ok(Self::Cardinal(1)),
			"two" => Ok(Self::Cardinal(2)),
			"three" => Ok(Self::Cardinal(3)),
			"four" => Ok(Self::Cardinal(4)),
			"five" => Ok(Self::Cardinal(5)),
			"six" => Ok(Self::Cardinal(6)),
			"seven" => Ok(Self::Cardinal(7)),
			"eight" => Ok(Self::Cardinal(8)),
			"nine" => Ok(Self::Cardinal(9)),
			"ten" => Ok(Self::Cardinal(10)),
			"eleven" => Ok(Self::Cardinal(11)),
			"twelve" => Ok(Self::Cardinal(12)),
			"thirteen" => Ok(Self::Cardinal(13)),
			"fourteen" => Ok(Self::Cardinal(14)),
			"fifteen" => Ok(Self::Cardinal(15)),
			"sixteen" => Ok(Self::Cardinal(16)),
			"seventeen" => Ok(Self::Cardinal(17)),
			"eighteen" => Ok(Self::Cardinal(18)),
			"nineteen" => Ok(Self::Cardinal(19)),
			"twenty" => Ok(Self::Cardinal(20)),
			"thirty" => Ok(Self::Cardinal(30)),
			"forty" => Ok(Self::Cardinal(40)),
			"fifty" => Ok(Self::Cardinal(50)),
			"sixty" => Ok(Self::Cardinal(60)),
			"seventy" => Ok(Self::Cardinal(70)),
			"eighty" => Ok(Self::Cardinal(80)),
			"ninety" => Ok(Self::Cardinal(90)),
			"hundred" => Ok(Self::Cardinal(100)),
			"thousand" => Ok(Self::Cardinal(1000)),

			"first" => Ok(Self::Ordinal(1)),
			"second" => Ok(Self::Ordinal(2)),
			"third" => Ok(Self::Ordinal(3)),
			"fourth" => Ok(Self::Ordinal(4)),
			"fifth" => Ok(Self::Ordinal(5)),
			"sixth" => Ok(Self::Ordinal(6)),
			"seventh" => Ok(Self::Ordinal(7)),
			"eighth" => Ok(Self::Ordinal(8)),
			"ninth" => Ok(Self::Ordinal(9)),
			"tenth" => Ok(Self::Ordinal(10)),
			"eleventh" => Ok(Self::Ordinal(11)),
			"twelfth" => Ok(Self::Ordinal(12)),
			"thirteenth" => Ok(Self::Ordinal(13)),
			"fourteenth" => Ok(Self::Ordinal(14)),
			"fifteenth" => Ok(Self::Ordinal(15)),
			"sixteenth" => Ok(Self::Ordinal(16)),
			"seventeenth" => Ok(Self::Ordinal(17)),
			"eighteenth" => Ok(Self::Ordinal(18)),
			"nineteenth" => Ok(Self::Ordinal(19)),
			"twentieth" => Ok(Self::Ordinal(20)),
			"thirtieth" => Ok(Self::Ordinal(30)),
			"fortieth" => Ok(Self::Ordinal(40)),
			"fiftieth" => Ok(Self::Ordinal(50)),
			"sixtieth" => Ok(Self::Ordinal(60)),
			"seventieth" => Ok(Self::Ordinal(70)),
			"eightieth" => Ok(Self::Ordinal(80)),
			"ninetieth" => Ok(Self::Ordinal(90)),
			"hundredth" => Ok(Self::Ordinal(100)),
			"thousandth" => Ok(Self::Ordinal(1000)),

			_ => {
				if let Some((tens, ones)) = word.split_once('-') {
					let tens = Self::to_numver(tens)?.as_usize();
					let ones = Self::to_numver(ones)?.as_usize();
					if word.contains("first")
						|| word.contains("second")
						|| word.contains("third")
					{
						Ok(Self::Ordinal(tens + ones))
					} else {
						Ok(Self::Cardinal(tens + ones))
					}
				} else {
					Err(Error::InvalidInput)
				}
			}
		}
	}

	pub fn to_usize(phrase: &str) -> Result<usize, Error> {
		let words: Vec<Self> = phrase
			.split_whitespace()
			.map(|word| word.parse::<Self>())
			.collect::<Result<_, _>>()?;

		let mut result = 0;
		let mut current = 0;

		for word in words {
			match word {
				Self::Cardinal(100) => current *= 100,
				Self::Cardinal(1000) => {
					result += current * 1000;
					current = 0;
				}
				_ => current += word.as_usize(),
			}
		}

		Ok(result + current)
	}

	pub fn to_words(&self) -> String {
		match self {
			Self::Cardinal(n) => Self::number_to_words(*n),
			Self::Ordinal(n) => Self::number_to_ordinal_words(*n),
		}
	}

	fn number_to_words(n: usize) -> String {
		match n {
			0 => "zero".to_string(),
			1 => "one".to_string(),
			2 => "two".to_string(),
			3 => "three".to_string(),
			4 => "four".to_string(),
			5 => "five".to_string(),
			6 => "six".to_string(),
			7 => "seven".to_string(),
			8 => "eight".to_string(),
			9 => "nine".to_string(),
			10 => "ten".to_string(),
			11 => "eleven".to_string(),
			12 => "twelve".to_string(),
			13 => "thirteen".to_string(),
			14 => "fourteen".to_string(),
			15 => "fifteen".to_string(),
			16 => "sixteen".to_string(),
			17 => "seventeen".to_string(),
			18 => "eighteen".to_string(),
			19 => "nineteen".to_string(),
			20..=90 if n % 10 == 0 => match n {
				20 => "twenty",
				30 => "thirty",
				40 => "forty",
				50 => "fifty",
				60 => "sixty",
				70 => "seventy",
				80 => "eighty",
				90 => "ninety",
				_ => unreachable!(),
			}
			.to_string(),
			21..=99 => {
				format!(
					"{}-{}",
					Self::number_to_words(n - n % 10),
					Self::number_to_words(n % 10)
				)
			}
			100..=999 => {
				let hundreds = n / 100;
				let remainder = n % 100;
				if remainder == 0 {
					format!(
						"{} hundred",
						Self::number_to_words(hundreds)
					)
				} else {
					format!(
						"{} hundred {}",
						Self::number_to_words(hundreds),
						Self::number_to_words(remainder)
					)
				}
			}
			1000..=9999 => {
				let thousands = n / 1000;
				let remainder = n % 1000;
				if remainder == 0 {
					format!(
						"{} thousand",
						Self::number_to_words(thousands)
					)
				} else {
					format!(
						"{} thousand {}",
						Self::number_to_words(thousands),
						Self::number_to_words(remainder)
					)
				}
			}
			_ => "number out of range".to_string(),
		}
	}

	fn number_to_ordinal_words(n: usize) -> String {
		match n {
			1 => "first".to_string(),
			2 => "second".to_string(),
			3 => "third".to_string(),
			4 => "fourth".to_string(),
			5 => "fifth".to_string(),
			6 => "sixth".to_string(),
			7 => "seventh".to_string(),
			8 => "eighth".to_string(),
			9 => "ninth".to_string(),
			10 => "tenth".to_string(),
			11 => "eleventh".to_string(),
			12 => "twelfth".to_string(),
			13 => "thirteenth".to_string(),
			14..=19 => format!("{}th", Self::number_to_words(n)),
			20 => "twentieth".to_string(),
			30 => "thirtieth".to_string(),
			40 => "fortieth".to_string(),
			50 => "fiftieth".to_string(),
			60 => "sixtieth".to_string(),
			70 => "seventieth".to_string(),
			80 => "eightieth".to_string(),
			90 => "ninetieth".to_string(),
			21..=99 => {
				let tens = n / 10 * 10;
				let ones = n % 10;
				if ones == 0 {
					Self::number_to_ordinal_words(tens)
				} else {
					format!(
						"{}-{}",
						Self::number_to_words(tens),
						Self::number_to_ordinal_words(ones)
					)
				}
			}
			100 => "hundredth".to_string(),
			1000 => "thousandth".to_string(),
			_ => {
				let words = Self::number_to_words(n);
				if words.ends_with("y") {
					format!("{}ieth", &words[..words.len() - 1])
				} else {
					format!("{}th", words)
				}
			}
		}
	}
}

trait ToNumver {
	fn to_numver(&self) -> Result<Numver, Error>;
}

impl ToNumver for str {
	fn to_numver(&self) -> Result<Numver, Error> {
		Numver::to_numver(self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_cardinal() {
		assert_eq!("zero".parse(), Ok(Numver::Cardinal(0)));
		assert_eq!("one".parse(), Ok(Numver::Cardinal(1)));
		assert_eq!("twenty".parse(), Ok(Numver::Cardinal(20)));
	}

	#[test]
	fn test_parse_ordinal() {
		assert_eq!("first".parse(), Ok(Numver::Ordinal(1)));
		assert_eq!("third".parse(), Ok(Numver::Ordinal(3)));
		assert_eq!("twentieth".parse(), Ok(Numver::Ordinal(20)));
	}

	#[test]
	fn test_parse_compound() {
		assert_eq!("twenty-first".parse(), Ok(Numver::Ordinal(21)));
		assert_eq!("fifty-second".parse(), Ok(Numver::Ordinal(52)));
	}

	#[test]
	fn test_to_usize() {
		assert_eq!(
			Numver::to_usize("one hundred twenty-three"),
			Ok(123)
		);
		assert_eq!(Numver::to_usize("first"), Ok(1));
		assert_eq!(
			Numver::to_usize("one thousand two hundred thirty-four"),
			Ok(1234)
		);
	}

	#[test]
	fn test_invalid_input() {
		assert_eq!(
			"invalid".parse::<Numver>(),
			Err(Error::InvalidInput)
		);
	}
}
