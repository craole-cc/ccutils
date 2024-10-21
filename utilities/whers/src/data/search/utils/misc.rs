impl crate::Search {

	/// Sets the case sensitivity of the search.
	///
	/// # Arguments
	///
	/// * `case_sensitive` - A boolean value indicating whether the search should be case sensitive.
	///
	/// # Returns
	///
	/// The updated Search instance.
	pub fn with_case_sensitivity(
		mut self,
		case_sensitive: bool,
	) -> Self {
		self.case_sensitive = case_sensitive;
		self
	}

	/// Sets the search to be case sensitive.
	///
	/// # Returns
	///
	/// The updated Search instance.
	pub fn case_sensitive(mut self) -> Self {
		self.case_sensitive = true;
		self
	}

	/// Sets the search to be case insensitive.
	///
	/// # Returns
	///
	/// The updated Search instance.
	pub fn case_insensitive(mut self) -> Self {
		self.case_sensitive = false;
		self
	}

	// pub fn with_limit<L>(mut self, limit: L) -> Self
	// where
	// 	L: Into<Option<isize>>, // Accept signed integers for better flexibility
	// {
	// 	// Convert the input into Option<isize>
	// 	let opt_limit: Option<isize> = limit.into();

	// 	// Set limit to the value or to None if the value is None or non-positive
	// 	self.limit = match opt_limit {
	// 		Some(l) if l > 0 => Some(l as usize), // If the value is positive, use it
	// 		_ => None, // Otherwise, set limit to None
	// 	};
	// 	self
	// }
}
