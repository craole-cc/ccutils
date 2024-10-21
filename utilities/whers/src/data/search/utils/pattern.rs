impl crate::Search {
	/// Adds a pattern to the search.
	///
	/// # Arguments
	///
	/// * `pattern` - The pattern to add. This can be any type that can be converted into a String.
	///
	/// # Returns
	///
	/// The updated Search instance.
	pub fn with_pattern<S: Into<String>>(
		mut self,
		pattern: S,
	) -> Self {
		self.pattern.push(pattern.into());
		self
	}

	/// Adds multiple patterns to the search.
	///
	/// # Arguments
	///
	/// * `patterns` - An iterator of patterns to add. Each pattern can be any type that can be converted into a String.
	///
	/// # Returns
	///
	/// The updated Search instance.
	pub fn with_patterns<I, S>(mut self, patterns: I) -> Self
	where
		I: IntoIterator<Item = S>,
		S: Into<String>,
	{
		self.pattern.extend(patterns.into_iter().map(Into::into));
		self
	}
}
