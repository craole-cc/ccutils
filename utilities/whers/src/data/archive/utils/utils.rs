use super::{PathType, Search};

impl<P: AsRef<str> + Default> Search<P> {
	pub fn new(pattern: P) -> Self {
		Self {
			pattern,
			..Default::default()
		}
	}

	pub fn with_pattern(mut self, pattern: P) -> Self {
		self.pattern = pattern;
		self
	}

	pub fn with_type<T: Into<PathType>>(
		mut self,
		path_type: T,
	) -> Self {
		self.path_type = path_type.into();
		self
	}
}
