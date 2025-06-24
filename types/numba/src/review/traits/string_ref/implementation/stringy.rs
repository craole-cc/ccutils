use std::borrow::Cow;
use crate::ToStringRef;

impl ToStringRef for str {
	#[inline]
	fn to_string_ref(&self) -> Cow<'_, str> {
		Cow::Borrowed(self)
	}
}

impl ToStringRef for String {
	#[inline]
	fn to_string_ref(&self) -> Cow<'_, str> {
		Cow::Borrowed(self.as_str())
	}
}

impl ToStringRef for &str {
	#[inline]
	fn to_string_ref(&self) -> Cow<'_, str> {
		Cow::Borrowed(*self)
	}
}
