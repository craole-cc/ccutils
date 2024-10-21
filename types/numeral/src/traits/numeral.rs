use crate::error::numeral::Error;
use std::convert::TryFrom;

pub trait Cast {
	fn to_i128(self) -> i128;
	fn from_i128(value: i128) -> Self;
}

impl Cast for i32 {
	fn to_i128(self) -> i128 {
		self as i128
	}
	fn from_i128(value: i128) -> Self {
		value as i32
	}
}

impl Cast for i64 {
	fn to_i128(self) -> i128 {
		self as i128
	}
	fn from_i128(value: i128) -> Self {
		value as i64
	}
}

impl Cast for i128 {
	fn to_i128(self) -> i128 {
		self
	}
	fn from_i128(value: i128) -> Self {
		value
	}
}

// Implement for other numeric types as needed (u32, u64, f32, f64, etc.)
