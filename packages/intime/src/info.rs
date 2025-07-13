// use battery::units::Time as BatTime;
// use chrono::{DateTime, Duration as ChronoDuration, Local};
// use std::fmt::{self, Display, Formatter};
// use uom::si::{f64::Time as SiTime, time::second};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Info {
  pub years: i64,
  pub months: i64,
  pub weeks: i64,
  pub days: i64,
  pub hours: i64,
  pub minutes: i64,
  pub seconds: i64,
  pub options: Options
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Options {
  pub hide_years: bool,
  pub hide_months: bool,
  pub hide_weeks: bool,
  pub hide_days: bool,
  pub hide_hours: bool,
  pub hide_minutes: bool,
  pub hide_seconds: bool
}
