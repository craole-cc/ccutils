// lib.rs
pub mod duration;
mod error;
mod general;
mod hardware;
mod power;
mod software;
mod time;

pub use duration::Duration;
pub use error::SystemInfoError;
pub use general::General;
pub use hardware::Hardware;
pub use power::{get_battery_info, Battery};
pub use software::Software;
pub use time::Time;

use sysinfo::System;

pub struct SystemInfoManager {
	system: System,
	battery: Battery,
}

impl SystemInfoManager {
	pub fn new() -> Result<Self, SystemInfoError> {
		Ok(Self {
			system: System::new_all(),
			battery: Battery::default(),
		})
	}

	pub fn refresh(&mut self) {
		self.system.refresh_all();
		self.battery = Battery::default();
	}

	pub fn time(&self) -> time::Time {
		time::Time::all()
	}

	pub fn software(&self) -> Software {
		Software::all()
	}

	pub fn hardware(&self) -> Hardware {
		Hardware::all()
	}

	pub fn general(&self) -> General {
		General::all()
	}

	pub fn battery(&self) -> &Battery {
		&self.battery
	}
}
