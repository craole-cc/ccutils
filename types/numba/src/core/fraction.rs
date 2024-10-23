#[derive(Default, Debug, Clone, PartialEq)]
pub struct Fraction {
	numeric: f64,     // Numeric representation, e.g., 0.75
	worded: String,   // Worded form, e.g., "three fourths"
	symbolic: String, // Symbolic form, e.g., "3/4"
}

impl Fraction {
	// Constructor to initialize from numeric value with precision
	pub fn from_numeric(numeric: f64, precision: usize) -> Self {
		let (numerator, denominator) =
			Self::numeric_to_fraction(numeric, precision);
		let symbolic = format!("{}/{}", numerator, denominator);
		let worded = Self::fraction_to_words(numerator, denominator);
		Fraction {
			numeric,
			worded,
			symbolic,
		}
	}

	// Helper: Convert numeric to fraction with specified precision
	fn numeric_to_fraction(
		numeric: f64,
		precision: usize,
	) -> (isize, isize) {
		let denominator = 10_f64.powi(precision as i32) as isize; // Precision as denominator
		let numerator =
			(numeric * denominator as f64).round() as isize;
		(numerator, denominator)
	}

	// Convert symbolic string (e.g., "3/4") to a fraction
	fn symbolic_to_fraction(symbolic: &str) -> (isize, isize) {
		let parts: Vec<&str> = symbolic.split('/').collect();
		let numerator: isize = parts[0].parse().unwrap_or(1);
		let denominator: isize = parts[1].parse().unwrap_or(1);
		(numerator, denominator)
	}

	// Convert worded string to a fraction
	fn words_to_fraction(worded: &str) -> (isize, isize) {
		match worded {
			"one half" => (1, 2),
			"three fourths" => (3, 4),
			"one over nine" => (1, 9),
			"one and a half" => (3, 2), // 1.5 as fraction
			"three and one over 11" => (34, 11), // 3 + 1/11
			_ => (1, 1),                // Default fallback
		}
	}

	// Convert a fraction (numerator/denominator) to a worded string
	fn fraction_to_words(
		numerator: isize,
		denominator: isize,
	) -> String {
		let numerator_word = Self::number_to_words(numerator);
		let denominator_word =
			Self::denominator_to_words(denominator);
		format!("{} {}", numerator_word, denominator_word)
	}

	// Convert number to words (expandable)
	fn number_to_words(num: isize) -> String {
		match num {
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
			_ => num.to_string(),
		}
	}

	// Convert denominator to word representation
	fn denominator_to_words(den: isize) -> String {
		match den {
			2 => "half".to_string(),
			3 => "thirds".to_string(),
			4 => "fourths".to_string(),
			_ => format!("{}ths", den),
		}
	}
}

// Implementing From trait for Fraction
impl From<f64> for Fraction {
	fn from(numeric: f64) -> Self {
		Fraction::from_numeric(numeric, 2) // Default precision can be 2
	}
}

impl From<isize> for Fraction {
	fn from(numeric: isize) -> Self {
		Fraction::from_numeric(numeric as f64, 2) // Default precision can be 2
	}
}

impl From<String> for Fraction {
	fn from(worded: String) -> Self {
		Fraction::from(worded.as_str()) // Reuse the &str implementation
	}
}

impl From<&str> for Fraction {
	fn from(symbolic: &str) -> Self {
		let (numerator, denominator) =
			Fraction::symbolic_to_fraction(symbolic);
		let numeric = numerator as f64 / denominator as f64;
		let worded =
			Fraction::fraction_to_words(numerator, denominator);
		Fraction {
			numeric,
			worded,
			symbolic: symbolic.to_string(),
		}
	}
}
