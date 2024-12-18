const ONES: [&str; 10] = [
	"zero", "one", "two", "three", "four", "five", "six", "seven",
	"eight", "nine",
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

const TENS: [&str; 10] = [
	"", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy",
	"eighty", "ninety",
];

const HUGE: [&str; 6] = [
	"",
	"thousand",
	"million",
	"billion",
	"trillion",
	"quadrillion",
];

pub fn to_cardinal(number: usize) -> String {
	if number == 0 {
		return "zero".to_string();
	}
	let mut result = String::new();
	let mut number = number;
	let mut mega_idx = 0;

	while number > 0 {
		let chunk = (number % 1000);
		if chunk > 0 {
			let chunk_str = to_cardinal_below_thousand(chunk);
			if !result.is_empty() {
				result = format!(
					"{} {} {}",
					chunk_str, HUGE[mega_idx], result
				)
				.trim()
				.replace(" ,", ",");
			} else {
				result = format!("{} {}", chunk_str, HUGE[mega_idx])
					.trim()
					.to_string();
			}
		}
		number /= 1000;
		mega_idx += 1;
	}
	result.trim().replace(" ,", ",").to_string()
}

fn to_cardinal_below_thousand(number: usize) -> String {
	let hundreds = number / 100;
	let remainder = number % 100;
	let mut result = String::new();

	if hundreds > 0 {
		result.push_str(ONES[hundreds]);
		result.push_str(" hundred");
		if remainder > 0 {
			result.push_str(" and ");
		}
	}

	if remainder > 0 {
		result.push_str(&to_cardinal_below_hundred(remainder));
	}

	result
}

fn to_cardinal_below_hundred(number: usize) -> String {
	match number {
		0..=9 => ONES[number].to_string(),
		10..=19 => TEENS[number - 10].to_string(),
		20..=99 => {
			let tens = number / 10;
			let units = number % 10;
			if units == 0 {
				TENS[tens].to_string()
			} else {
				format!("{}-{}", TENS[tens], ONES[units])
			}
		}
		_ => unreachable!(),
	}
}
