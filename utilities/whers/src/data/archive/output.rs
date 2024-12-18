use crate::Error;

#[derive(Default)]
pub enum Output {
	#[default]
	Plain,
	Fetch,
	Verbose,
}

impl Output {
	pub fn output(
		command: &str,
		format: &Self,
	) -> Result<String, Error> {
		let locations = locate(command)?;

		if locations.is_empty() {
			return Err(Error::CommandNotFound(command.to_string()));
		}

		Ok(match format {
			Self::Plain => locations
				.into_iter()
				.next()
				.map(|loc| loc.to_string())
				.unwrap_or_default(),
			Self::Fetch => locations
				.into_iter()
				.map(|loc| format!("{:#}", loc))
				.collect::<Vec<_>>()
				.join("\n"),
			Self::Verbose => {
				let mut output = format!(
					"'{}' found in {} location(s):",
					command,
					locations.len()
				);
				for location in locations {
					output.push_str(&format!("\n  {:#}", location));
				}
				output
			}
		})
	}
}
