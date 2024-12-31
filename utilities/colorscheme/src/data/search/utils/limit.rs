use crate::parse::to_usize;
use std::str::FromStr;

impl crate::Search {
	pub fn with_limit<L>(mut self, limit: L) -> Self
	where
		L: Into<LimitInput>,
	{
		let limit_input: LimitInput = limit.into();
		self.limit = match limit_input {
			LimitInput::Number(n) if n > 0 => Some(n as usize),
			LimitInput::Word(word) => {
				if to_usize(&word) > 0 {
					Some(to_usize(&word))
				} else {
					None
				}
			}
			_ => None,
		};
		self
	}
}

pub enum LimitInput {
	Number(isize),
	Word(String),
}

impl From<isize> for LimitInput {
	fn from(n: isize) -> Self {
		LimitInput::Number(n)
	}
}

impl From<&str> for LimitInput {
	fn from(s: &str) -> Self {
		if let Ok(n) = isize::from_str(s) {
			LimitInput::Number(n)
		} else {
			LimitInput::Word(s.to_lowercase())
		}
	}
}

impl From<String> for LimitInput {
	fn from(s: String) -> Self {
		if let Ok(n) = isize::from_str(&s) {
			LimitInput::Number(n)
		} else {
			LimitInput::Word(s.to_lowercase())
		}
	}
}

impl From<Option<isize>> for LimitInput {
	fn from(opt: Option<isize>) -> Self {
		match opt {
			Some(n) => LimitInput::Number(n),
			None => LimitInput::Number(-1), // Use a negative number to represent None
		}
	}
}

fn word_to_limit(word: &str) -> Option<usize> {
	match word {
		"one" | "first" => Some(1),
		"two" | "second" => Some(2),
		"three" | "third" => Some(3),
		"four" | "fourth" => Some(4),
		"five" | "fifth" => Some(5),
		// Add more words as needed
		_ => None,
	}
}
