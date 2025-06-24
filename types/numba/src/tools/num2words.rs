use num_bigfloat::BigFloat;

/// Error type returned by the builder
#[derive(Debug, PartialEq)]
pub enum Num2Err {
	/// General error, language cannot convert number
	///
	/// It is likely that the language does not support the number because
	/// it is too large.
	///
	/// Example:
	/// ```
	/// use num2words::{Num2Err, Num2Words};
	/// assert_eq!(
	///     Num2Words::new(1e100).to_words(),
	///     Err(Num2Err::CannotConvert)
	/// );
	/// ```
	CannotConvert,
	/// Request of a negative ordinal
	///
	/// Example:
	/// ```
	/// use num2words::{Num2Err, Num2Words};
	/// assert_eq!(
	///     Num2Words::new(-42).ordinal().to_words(),
	///     Err(Num2Err::NegativeOrdinal)
	/// );
	/// ```
	NegativeOrdinal,
	/// Request of a float ordinal
	///
	/// Example:
	/// ```
	/// use num2words::{Num2Err, Num2Words};
	/// assert_eq!(
	///     Num2Words::new(42.01).ordinal().to_words(),
	///     Err(Num2Err::FloatingOrdinal)
	/// );
	/// ```
	FloatingOrdinal,
	/// Request of a float year
	///
	/// Example:
	/// ```
	/// use num2words::{Num2Err, Num2Words};
	/// assert_eq!(
	///     Num2Words::new(42.01).year().to_words(),
	///     Err(Num2Err::FloatingYear)
	/// );
	/// ```
	FloatingYear,
	/// Request of an infinite ordinal
	///
	/// Example:
	/// ```
	/// use num2words::{Num2Err, Num2Words};
	/// assert_eq!(
	///     Num2Words::parse("inf").unwrap().ordinal().to_words(),
	///     Err(Num2Err::InfiniteOrdinal)
	/// );
	/// ```
	InfiniteOrdinal,
	/// Request of an infinite year
	///
	/// Example:
	/// ```
	/// use num2words::{Num2Err, Num2Words};
	/// assert_eq!(
	///     Num2Words::parse("inf").unwrap().year().to_words(),
	///     Err(Num2Err::InfiniteYear)
	/// );
	/// ```
	InfiniteYear,
}

impl std::fmt::Display for Num2Err {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Num2Err::CannotConvert => "cannot convert number",
				Num2Err::NegativeOrdinal =>
					"cannot treat negative number as ordinal",
				Num2Err::FloatingOrdinal =>
					"cannot treat float as ordinal",
				Num2Err::FloatingYear => "cannot treat float as year",
				Num2Err::InfiniteOrdinal =>
					"cannot treat infinity as ordinal",
				Num2Err::InfiniteYear =>
					"cannot treat infinity as year",
			}
		)
	}
}

const UNITS: [&str; 9] = [
	"one", "two", "three", "four", "five", "six", "seven", "eight",
	"nine",
];

const TENS: [&str; 9] = [
	"ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy",
	"eighty", "ninety",
];

const TEENS: [&str; 10] = [
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
];

const MASSIVE: [&str; 21] = [
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
	"vigintillion",
];

fn split_thousands(mut num: BigFloat) -> Vec<u64> {
	let mut thousands = Vec::new();
	let bf_1000 = BigFloat::from(1000);

	while !num.is_zero() {
		thousands.push((num % bf_1000).to_u64().unwrap());
		num /= bf_1000;
	}

	thousands
}

fn int_to_cardinal(mut num: BigFloat) -> Result<String, Num2Err> {
	// special case zero
	if num.is_zero() {
		return Ok(String::from("zero"));
	}

	// handling negative values
	let mut words = vec![];
	if num.is_negative() {
		words.push(String::from("minus"));
		num = -num;
	}

	// iterate over thousands
	let mut first_elem = true;
	for (i, triplet) in split_thousands(num).iter().enumerate().rev()
	{
		let hundreds = (triplet / 100 % 10) as usize;
		let tens = (triplet / 10 % 10) as usize;
		let units = (triplet % 10) as usize;

		if hundreds > 0 {
			words.push(String::from(UNITS[hundreds - 1]));
			words.push(String::from("hundred"));
		}

		if tens != 0 || units != 0 {
			if i == 0 && !first_elem {
				words.push(String::from("and"));
			} else {
				first_elem = false;
			}

			match tens {
				0 => {
					// case 102 => [one hundred] two
					words.push(String::from(UNITS[units - 1]));
				}
				1 => {
					// case 112 => [one hundred] twelve
					words.push(String::from(TEENS[units]));
				}
				_ => {
					// case 142 => [one hundred] forty-two
					let ten: String = String::from(TENS[tens - 1]);
					words.push(match units {
						0 => ten,
						_ => format!("{}-{}", ten, UNITS[units - 1]),
					});
				}
			}
		}

		if i != 0 && triplet != &0 {
			if i > MASSIVE.len() {
				return Err(Num2Err::CannotConvert);
			}
			words.push(String::from(MASSIVE[i - 1]));
		}
	}

	Ok(words.join(" "))
}

fn float_to_cardinal(num: BigFloat) -> Result<String, Num2Err> {
	let integral_part = num.int();
	let mut words: Vec<String> = vec![];

	if !integral_part.is_zero() {
		let integral_word = int_to_cardinal(integral_part)?;
		words.push(integral_word);
	}

	let mut ordinal_part = num.frac();
	if !ordinal_part.is_zero() {
		words.push(String::from("point"));
	}
	while !ordinal_part.is_zero() {
		let digit = (ordinal_part * BigFloat::from(10)).int();
		ordinal_part = (ordinal_part * BigFloat::from(10)).frac();
		words.push(match digit.to_u64().unwrap() {
			0 => String::from("zero"),
			i => String::from(UNITS[i as usize - 1]),
		});
	}
	Ok(words.join(" "))
}

pub fn to_cardinal(num: BigFloat) -> Result<String, Num2Err> {
	if num.is_inf_pos() {
		Ok(String::from("infinity"))
	} else if num.is_inf_neg() {
		Ok(String::from("minus infinity"))
	} else if num.frac().is_zero() {
		int_to_cardinal(num)
	} else {
		float_to_cardinal(num)
	}
}

pub fn to_ordinal(num: BigFloat) -> Result<String, Num2Err> {
	let cardinal_word = to_cardinal(num)?;

	let mut words: Vec<String> = vec![];
	let mut split = cardinal_word.split_whitespace().peekable();

	while let Some(w) = split.next() {
		if split.peek().is_some() {
			// not last word, no modification needed
			words.push(String::from(w));
		} else {
			// last word, needs to be processed
			let mut prefix = String::from("");
			let mut suffix = String::from(w);

			if w.contains('-') {
				// e.g. forty-two => forty-second
				let mut w_split = w.split('-');

				if let Some(pre) = w_split.next() {
					prefix = format!("{}-", pre);
				}

				if let Some(suf) = w_split.next() {
					suffix = String::from(suf);
				}
			}

			suffix = match suffix.as_str() {
				"one" => String::from("first"),
				"two" => String::from("second"),
				"three" => String::from("third"),
				"four" => String::from("fourth"),
				"five" => String::from("fifth"),
				"six" => String::from("sixth"),
				"seven" => String::from("seventh"),
				"eight" => String::from("eighth"),
				"nine" => String::from("ninth"),
				"ten" => String::from("tenth"),
				"eleven" => String::from("eleventh"),
				"twelve" => String::from("twelfth"),
				_ => {
					if suffix.ends_with('y') {
						format!("{}ieth", &suffix[..suffix.len() - 1])
					} else {
						format!("{}th", suffix)
					}
				}
			};

			words.push(format!("{}{}", prefix, suffix))
		}
	}

	Ok(words.join(" "))
}

fn to_ordinal_num(num: BigFloat) -> Result<String, Num2Err> {
	let tail = (num % BigFloat::from(100)).to_u64().unwrap();
	let last = tail % 10;
	Ok(format!(
		"{}{}",
		num.to_u128().unwrap(),
		match (tail / 10 != 1, last) {
			(true, 1) => "st",
			(true, 2) => "nd",
			(true, 3) => "rd",
			_ => "th",
		}
	))
}
