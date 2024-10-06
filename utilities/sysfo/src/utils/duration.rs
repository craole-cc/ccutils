use battery::units::Time as BatTime;
use chrono::{DateTime, Duration as ChronoDuration, Local};
use std::fmt::{self, Display, Formatter};
use uom::si::{f64::Time as SiTime, time::second};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Duration {
	years: i64,
	months: i64,
	weeks: i64,
	days: i64,
	hours: i64,
	minutes: i64,
	seconds: i64,
	display_options: DisplayOptions,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct DisplayOptions {
	pub hide_years: bool,
	pub hide_months: bool,
	pub hide_weeks: bool,
	pub hide_days: bool,
	pub hide_hours: bool,
	pub hide_minutes: bool,
	pub hide_seconds: bool,
}

impl Duration {
	pub fn set_display_options(&mut self, options: DisplayOptions) {
		self.display_options = options;
	}

	pub fn with_display_options(
		mut self,
		options: DisplayOptions,
	) -> Self {
		self.display_options = options;
		self
	}
	pub fn from_battery_time(time: Option<BatTime>) -> Self {
		match time {
			Some(time) => {
				let seconds = time.value as f64;
				Self::from_secs_f64(seconds)
			}
			None => Self::default(),
		}
	}

	pub fn from_si_time(si_time: SiTime) -> Self {
		Self::from_secs_f64(si_time.get::<second>())
	}

	pub fn from_secs_f64(duration: f64) -> Self {
		let total_minutes = (duration / 60.0).floor() as i64;
		let remaining_seconds = (duration % 60.0).floor() as i64;

		let chrono_duration =
			ChronoDuration::seconds(duration as i64);
		let mut duration = Self::from_delta(chrono_duration);
		duration.seconds = remaining_seconds;
		duration
	}

	pub fn from_delta(duration: ChronoDuration) -> Self {
		let total_minutes = duration.num_minutes();
		let total_hours = duration.num_hours();
		let total_days = duration.num_days();

		let years = total_days / 365;
		let remaining_days = total_days % 365;

		let months = remaining_days / 30;
		let remaining_days = remaining_days % 30;

		let weeks = remaining_days / 7;
		let days = remaining_days % 7;

		let hours = total_hours % 24;
		let minutes = total_minutes % 60;
		let seconds = duration.num_seconds() % 60;

		let display_options = DisplayOptions::default();

		Self {
			years,
			months,
			weeks,
			days,
			hours,
			minutes,
			seconds,
			display_options,
		}
	}

	pub fn until_now(start: DateTime<Local>) -> Self {
		Self::start_to_finish(start, Local::now())
	}

	pub fn start_to_finish(
		start: DateTime<Local>,
		end: DateTime<Local>,
	) -> Self {
		let duration = end.signed_duration_since(start);
		Self::from_delta(duration)
	}

	pub fn above_seconds(&mut self) -> &Self {
		self.display_options.hide_seconds = true;
		self
	}

	pub fn in_minutes(&mut self) -> &Self {
		self.display_options = DisplayOptions {
			hide_years: true,
			hide_months: true,
			hide_weeks: true,
			hide_days: true,
			hide_hours: true,
			hide_seconds: true,
			..Default::default()
		};
		self
	}

	pub fn in_hours(&mut self) -> &Self {
		self.display_options = DisplayOptions {
			hide_years: true,
			hide_months: true,
			hide_weeks: true,
			hide_days: true,
			hide_minutes: true,
			hide_seconds: true,
			..Default::default()
		};
		self
	}

	pub fn in_days(&mut self) -> &Self {
		self.display_options = DisplayOptions {
			hide_years: true,
			hide_months: true,
			hide_weeks: true,
			hide_hours: true,
			hide_minutes: true,
			hide_seconds: true,
			..Default::default()
		};
		self
	}

	pub fn is_zero(&self) -> bool {
		self.years == 0
			&& self.months == 0
			&& self.weeks == 0
			&& self.days == 0
			&& self.hours == 0
			&& self.minutes == 0
			&& self.seconds == 0
	}
}

impl Display for Duration {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let mut parts = Vec::new();

		let units = [
			("year", self.years, self.display_options.hide_years),
			("month", self.months, self.display_options.hide_months),
			("week", self.weeks, self.display_options.hide_weeks),
			("day", self.days, self.display_options.hide_days),
			("hour", self.hours, self.display_options.hide_hours),
			(
				"minute",
				self.minutes,
				self.display_options.hide_minutes,
			),
			(
				"second",
				self.seconds,
				self.display_options.hide_seconds,
			),
		];

		for (unit, value, hide) in units {
			if !hide && value > 0 {
				let plural = if value == 1 { "" } else { "s" };
				let value_str = if unit == "second" {
					format!("{:.3}", value)
				} else {
					format!("{}", value)
				};
				parts.push(format!(
					"{} {}{}",
					value_str, unit, plural
				));
			}
		}

		match parts.len() {
			0 => write!(f, "{}", 0),
			1 => write!(f, "{}", parts[0]),
			2 => write!(f, "{} and {}", parts[0], parts[1]),
			_ => {
				let last = parts.pop().unwrap();
				write!(f, "{}, and {}", parts.join(", "), last)
			}
		}
	}
}
