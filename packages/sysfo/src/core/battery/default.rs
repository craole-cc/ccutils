use crate::{Duration, battery::Display};
use battery::{Battery, Manager, State, Technology, units::Time};

#[derive(Debug, Clone)]
pub struct Info {
  pub level: f32,
  pub status: State,
  pub time_to_full: Option<Time>,
  pub time_to_empty: Option<Time>,
  pub time_left: crate::Duration,
  pub technology: Technology,
  pub cycles: Option<u32>,
  pub brand: Option<String> /* energy: 42577.2 m^2 kg^1 s^-2,
                             * energy_full: 182577.6 m^2 kg^1 s^-2,
                             * energy_full_design: 182577.6 m^2 kg^1 s^-2,
                             * energy_rate: 36.775 m^2 kg^1 s^-3,
                             * voltage: 12.239 m^2 kg^1 s^-3 A^-1, */
}

impl Default for Info {
  fn default() -> Self {
    pub fn get_battery_info() -> Battery {
      // TODO: Handle errors properly
      Manager::new()
        .expect("Failed to create battery manager")
        .batteries()
        .expect("Failed to get batteries")
        .next()
        .expect("Failed to get battery information")
        .expect("Failed to get battery information")
    }
    let battery = get_battery_info();
    let level = battery.state_of_charge().value;
    let status = battery.state();
    let time_to_empty = battery.time_to_empty();
    let time_to_full = battery.time_to_full();
    let time_left = if time_to_empty.is_some() {
      Duration::from_battery_time(time_to_empty)
    } else if time_to_full.is_some() {
      Duration::from_battery_time(time_to_full)
    } else {
      Duration::default()
    }
    .above_seconds()
    .clone();
    let technology = battery.technology();
    let cycles = battery.cycle_count();
    let brand = battery.vendor().map(|s| s.to_string());

    Self {
      level,
      status,
      time_to_full,
      time_to_empty,
      time_left,
      technology,
      cycles,
      brand
    }
  }
}

impl std::fmt::Display for Info {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.fetch())
  }
}
