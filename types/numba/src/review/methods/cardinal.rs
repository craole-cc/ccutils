use crate::{traits::cardinal::Cast, types::Cardinal};

impl Default for Cardinal {
	fn default() -> Self {
		Cardinal {
			numeric: 0.0,
			worded: "zero".to_string(),
		}
	}
}

// Implement From for Cardinal for various types using Cast
impl<T: Cast> From<T> for Cardinal {
	fn from(value: T) -> Self {
		let (numeric, worded) =
			value.cast().unwrap_or((0.0, String::from("")));
		Cardinal { numeric, worded }
	}
}

impl Cardinal {
	pub fn new<T: Cast>(value: T) -> Self {
		Self::from(value)
	}
}
