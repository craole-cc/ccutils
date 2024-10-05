use std::fmt::{Display, Formatter, Result};
use sysinfo::System;

#[derive(Debug)]
pub struct Hardware {
	hostname: String,
	arch: String,
	cores: usize,
	threads: usize,
	memory: f64,
	// battery: BatteryInfo,
}

impl Default for Hardware {
	fn default() -> Self {
		let mut system = System::new_all();
		system.refresh_all();
		// TODO: System needs to be made static

		// let battery =

		Self {
			hostname: System::host_name().unwrap_or_default(),
			arch: System::cpu_arch().unwrap_or_default(),
			cores: system.physical_core_count().unwrap_or(0),
			threads: system.cpus().len(),
			memory: system.total_memory() as f64 / 2_f64.powi(30),
			// battery,
		}
	}
}

impl Hardware {
	pub fn init() -> Self {
		Self::default()
	}

	pub fn all() -> Self {
		Self::default()
	}
}

impl Display for Hardware {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		let header = "Hardware {";
		let hostname =
			format!("{:>16}: {}", "Hostname", self.hostname);
		let arch = format!("{:>16}: {}", "Architecture", self.arch);
		let cores = format!(
			"{:>16}: {} [{} Threads]",
			"Cores", self.cores, self.threads
		);
		let memory =
			format!("{:>16}: {:.2} GB", "Memory", self.memory);
		// let battery = format!("{:>16}: {}", "Battery", self.battery);
		let battery = format!(
			"{:>16}: {}",
			"Battery",
			"BatteryInfo::default().model"
		);
		let footer = "}";

		write!(
			f,
			"{}\n{}\n{}\n{}\n{}\n{}\n{}",
			header, hostname, arch, cores, memory, battery, footer
		)
	}
}
