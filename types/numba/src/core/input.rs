#[derive(Debug, Clone, PartialEq)]
pub enum Input {
	Cardinal(f64),
	OrdinalSymbolic(usize, String),
	Ordinal(usize),
	PercentageSymbolic(f64, String),
	Percentage(f64, usize),
	Fraction(usize, String, usize),
	RomanSymbolic(String),
}

impl Default for Value {
	fn default() -> Self {
		Self::Cardinal(0.0)
	}
}
