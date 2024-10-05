use std::{
	env::var,
	fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub struct General {
	username: String,
}

impl Default for General {
	fn default() -> Self {
		fn get_current_user() -> String {
			var("USER")
				.or_else(|_| var("USERNAME"))
				.or_else(|_| var("LOGNAME"))
				.unwrap_or_else(|_| "Unknown".to_string())
		}

		let username = get_current_user();
		Self { username }
	}
}

impl General {
	pub fn all() -> Self {
		Self::default()
	}
}

impl Display for General {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		let header = "General {";
		let username =
			format!("{:>16}: {}", "Username", self.username);
		let footer = "}";

		write!(f, "{}\n{}\n{}", header, username, footer)
	}
}
