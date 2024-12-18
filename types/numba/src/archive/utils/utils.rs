impl Number {
	pub fn new(value: f64) -> Self {
		Self {
			numeral: value,
			round: Round::new(
				value.round() as usize,
				RoundDirection::Nearest,
				1,
			),
			floor: value.floor() as usize,
			ceil: value.ceil() as usize,
			numerator: floor,
			denominator: 100, // Assuming two decimal places for now
			cardinal: Self::to_cardinal(floor),
			ordinal: Self::to_ordinal(floor),
			ordinal_abbrev: format!("{}th", floor), // Simplified for now
			decimal: format!("{}", value),          // Simplified for now
			fraction: format!("{}/{}", floor, 100), // Simplified for now
			percentage: format!(
				"{}%",
				(value * 100.0).round() / 100.0
			),
			percentage_abbrev: format!("{:.1}%", value),
		}
	}

	pub const fn to_cardinal(n: usize) -> &'static str {
		match n {
			0 => "zero",
			1 => "one",
			2 => "two",
			3 => "three",
			4 => "four",
			5 => "five",
			6 => "six",
			7 => "seven",
			8 => "eight",
			9 => "nine",
			10 => "ten",
			11 => "eleven",
			12 => "twelve",
			13 => "thirteen",
			14 => "fourteen",
			15 => "fifteen",
			16 => "sixteen",
			17 => "seventeen",
			18 => "eighteen",
			19 => "nineteen",
			20 => "twenty",
			30 => "thirty",
			40 => "forty",
			50 => "fifty",
			60 => "sixty",
			70 => "seventy",
			80 => "eighty",
			90 => "ninety",
			_ => "number out of range",
		}
	}

	pub const fn to_ordinal(n: usize) -> &'static str {
		match n {
			1 => "first",
			2 => "second",
			3 => "third",
			4 => "fourth",
			5 => "fifth",
			6 => "sixth",
			7 => "seventh",
			8 => "eighth",
			9 => "ninth",
			10 => "tenth",
			11 => "eleventh",
			12 => "twelfth",
			13 => "thirteenth",
			20 => "twentieth",
			30 => "thirtieth",
			40 => "fortieth",
			50 => "fiftieth",
			60 => "sixtieth",
			70 => "seventieth",
			80 => "eightieth",
			90 => "ninetieth",
			_ => "th", // This is a simplification
		}
	}

	pub fn full_cardinal(n: usize) -> String {
		match n {
			0..=20 => Self::to_cardinal(n).to_string(),
			21..=99 => {
				let tens = n / 10;
				let ones = n % 10;
				if ones == 0 {
					Self::to_cardinal(n).to_string()
				} else {
					format!(
						"{}-{}",
						Self::to_cardinal(tens * 10),
						Self::to_cardinal(ones)
					)
				}
			}
			100..=999 => {
				let hundreds = n / 100;
				let remainder = n % 100;
				if remainder == 0 {
					format!("{} hundred", Self::to_cardinal(hundreds))
				} else {
					format!(
						"{} hundred and {}",
						Self::to_cardinal(hundreds),
						Self::full_cardinal(remainder)
					)
				}
			}
			// TODO: handle larger numbers
			_ => "number out of range".to_string(),
		}
	}

	pub fn full_ordinal(n: usize) -> String {
		if n <= 20 || n % 10 == 0 {
			Self::to_ordinal(n).to_string()
		} else {
			let cardinal = Self::full_cardinal(n);
			if cardinal.ends_with('y') {
				format!("{}ieth", &cardinal[..cardinal.len() - 1])
			} else {
				format!("{}th", cardinal)
			}
		}
	}
}
