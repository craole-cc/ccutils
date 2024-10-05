use super::{Meta, Paths};
use anyhow::Result;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Project {
	pub meta: Meta,
	pub paths: Paths,
}

impl Project {
	pub fn init() -> Self {
		let meta = Meta::init();
		let paths = Paths::init();

		let conf = Self { meta, paths };
		pout(&conf);
		conf
	}
}

impl Display for Project {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}{}", self.meta, self.paths)
	}
}

fn pout<T: Display>(payload: &T) {
	let type_name = std::any::type_name::<T>();
	let struct_name =
		type_name.split("::").last().unwrap_or(type_name);

		tracing::debug!("{} initialized", struct_name);
		tracing::info!("{} initialized", struct_name);
		let reps = 12;
		tracing::trace!(
		"{} {} {}\n{}",
		">".repeat(reps),
		struct_name,
		"<".repeat(reps),
		payload
	);
}
