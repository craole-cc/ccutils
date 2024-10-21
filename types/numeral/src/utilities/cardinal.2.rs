// use crate::{error::cardinal::Error, traits::cardinal::Cast};
// use astro_float_num::BigFloat;
use num_bigfloat::BigFloat;

use crate::error::cardinal::Error;

const ONES: [&str; 9] = [
	"one", "two", "three", "four", "five", "six", "seven", "eight",
	"nine",
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

const TENS: [&str; 9] = [
	"ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy",
	"eighty", "ninety",
];

const HUGE: [&str; 21] = [
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

pub fn int_to_cardinal(
	mut numeral: BigFloat,
) -> Result<String, Error> {

	// Exit early if zero
	if numeral.is_zero() {
		return Ok(String::from("zero"));
	}

	// Tag negative values
	let mut words = vec![];
	if numeral.is_negative() {
		words.push(String::from("minus"));
		numeral = -numeral;
	}

	// Iterate thousand groups
	let mut first_elem = true;
	for (i, triplet) in
		group_by_thousands(numeral).iter().enumerate().rev()
	{
		let hundreds = (triplet / 100 % 10) as usize;
		let tens = (triplet / 10 % 10) as usize;
		let units = (triplet % 10) as usize;

		if hundreds > 0 {
			words.push(String::from(ONES[hundreds - 1]));
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
					words.push(String::from(ONES[units - 1]));
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
						_ => format!("{}-{}", ten, ONES[units - 1]),
					});
				}
			}
		}

		if i != 0 && triplet != &0 {
			if i > HUGE.len() {
				return Err(Error::KnownRangeExceeded);
			}
			words.push(String::from(HUGE[i - 1]));
		}
	}

	Ok(words.join(" "))
}

fn group_by_thousands(mut numeral: BigFloat) -> Vec<u64> {
	let mut thousands = Vec::new();
	let bf_1000 = BigFloat::from(1000);

	while !numeral.is_zero() {
		let remainder = numeral.rem(&bf_1000);
		thousands.push(remainder.to_u64().unwrap());
		numeral = numeral.div(&bf_1000);
	}

	thousands
}
