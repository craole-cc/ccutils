use std::fmt::{Display, Formatter, Result};
use sysinfo::System;

#[derive(Debug)]
pub struct Software {
	label: String,
	name: String,
	kernel: String,
	version: String,
	distro: String,
}

impl Default for Software {
	fn default() -> Self {
		Self {
			label: System::name().unwrap_or_default(),
			name: System::long_os_version().unwrap_or_default(),
			kernel: System::kernel_version().unwrap_or_default(),
			version: System::os_version().unwrap_or_default(),
			distro: System::distribution_id(),
		}
	}
}

impl Software {
	pub fn init() -> Self {
		Self::default()
	}

	pub fn all() -> Self {
		Self::default()
	}

	pub fn label() -> String {
		Self::default().label
	}

	pub fn name() -> String {
		Self::default().name
	}

	pub fn kernel() -> String {
		Self::default().kernel
	}

	pub fn version() -> String {
		Self::default().version
	}

	pub fn distro() -> String {
		Self::default().distro
	}
}

impl Display for Software {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		let header = "Software {";
		let label = format!("{:>16}: {}", "Label", self.label);
		let name = format!("{:>16}: {}", "Name", self.name);
		let kernel = format!("{:>16}: {}", "Kernel", self.kernel);
		let version = format!("{:>16}: {}", "Version", self.version);
		let distro = format!("{:>16}: {}", "Distro", self.distro);
		let footer = "}";

		write!(
			f,
			"{}\n{}\n{}\n{}\n{}\n{}\n{}",
			header, label, name, kernel, version, distro, footer
		)
	}
}
