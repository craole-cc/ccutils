use std::borrow::Cow;

/// A trait for types that can be converted to a string reference efficiently
pub trait ToStringRef {
	/// Converts the value to a string reference
	fn to_string_ref(&self) -> Cow<'_, str>;
}
