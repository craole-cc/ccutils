#[derive(Debug, Clone, PartialEq)]
pub enum Value {
	Decimal(f64),
	Integer(usize),
	Words(String),
}

impl Default for Value {
	fn default() -> Self {
		Self::Decimal(0.0)
	}
}
