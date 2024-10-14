#[cfg(test)]
mod tests {
	use crate::config::cmd::Location;

	#[test]
	fn test_existing_executable() {
		let result = Location::find("rustc");
		assert!(result.is_ok());
		let locations = result.unwrap();
		assert!(!locations.is_empty());
		assert!(locations
			.iter()
			.any(|loc| matches!(loc, Location::Executable(_))));
	}

	#[test]
	fn test_shell_builtin() {
		let result = Location::find("cd");
		assert!(result.is_ok());
		let locations = result.unwrap();
		assert!(locations
			.iter()
			.any(|loc| matches!(loc, Location::ShellBuiltin)));
	}

	#[test]
	fn test_nonexistent_command() {
		let result = Location::find("this_command_does_not_exist");
		assert!(result.is_err());
	}

	#[test]
	fn test_multiple_locations() {
		let result = Location::find("python");
		if let Ok(locations) = result {
			if locations.len() > 1 {
				assert!(
					locations
						.iter()
						.filter(|loc| matches!(
							loc,
							Location::Executable(_)
						))
						.count() > 1
				);
			}
		}
	}

	#[test]
	fn test_case_insensitivity() {
		let lower_result = Location::find("rustc");
		let upper_result = Location::find("RUSTC");
		assert_eq!(lower_result.is_ok(), upper_result.is_ok());
	}

	#[cfg(windows)]
	#[test]
	fn test_windows_specific() {
		let result = Location::find("cmd");
		assert!(result.is_ok());
		let locations = result.unwrap();
		assert!(locations
			.iter()
			.any(|loc| matches!(loc, Location::Executable(_))));
	}

	#[cfg(unix)]
	#[test]
	fn test_unix_specific() {
		let result = Location::find("bash");
		assert!(result.is_ok());
		let locations = result.unwrap();
		assert!(locations
			.iter()
			.any(|loc| matches!(loc, Location::Executable(_))));
	}

	#[test]
	fn test_all_commands() {
		let commands = vec![
			"rustc", "pathof", "type", "ls", "fd", "find", "pwsh",
			"whereis",
		];
		for cmd in commands {
			let result = Location::find(cmd);
			match result {
				Ok(locations) => {
					println!(
						"Command '{}' found in {} location(s):",
						cmd,
						locations.len()
					);
					for loc in locations {
						match loc {
							Location::Executable(path) => {
								println!(
									"  Executable: {}",
									path.display()
								)
							}
							Location::ShellBuiltin => {
								println!("  Shell Builtin")
							}
							Location::ShellAlias(info) => {
								println!("  Shell Alias: {}", info)
							}
							Location::ShellFunction(info) => {
								println!("  Shell Function: {}", info)
							}
						}
					}
				}
				Err(e) => {
					println!("Error finding command '{}': {}", cmd, e)
				}
			}
		}
	}
}
