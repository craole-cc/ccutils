use crate::types::Cardinal;
use std::fmt;

impl fmt::Display for Cardinal {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} ({})", self.worded, self.numeric)
	}
}
