#[derive(Default, Debug, Clone, PartialEq)]
pub struct Rounding {
	/// The rounding direction
	pub target: Target,

	/// The unit to round to
	pub unit: usize,

	/// The number of decimal places to round to. Automatically calculated based on the fractional length of the value, for example if the fractional part is 0.06, the precision is 2
	pub precision: Option<usize>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Target {
	/// No rounding
	None,

	/// Round up to the nearest unit
	Up,

	/// Round down to the nearest unit
	Down,

	/// Round half toward zero
	ToZero,

	/// Round half away from zero
	FromZero,

	/// Round to the nearest even unit
	ToEven,

	/// Round to the nearest odd unit
	ToOdd,

	/// Round to the nearest unit
	#[default]
	Nearest,
}
