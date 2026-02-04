use super::_prelude::*;

#[must_use]
pub fn now() -> StdSysTime {
  StdSysTime::now()
}

#[cfg(feature = "time")]
#[must_use]
pub fn datetime() -> DateTime<Local> {
  Local::now()
}

#[cfg(feature = "time")]
#[must_use]
pub fn local_iso() -> String {
  datetime().to_rfc3339()
}

#[cfg(feature = "time")]
#[must_use]
pub fn local_long() -> String {
  datetime().to_rfc2822()
}

#[cfg(feature = "time")]
#[must_use]
pub fn local_date_only() -> String {
  datetime().format("%Y-%m-%d").to_string()
}

#[cfg(feature = "time")]
#[must_use]
pub fn local_time_only() -> String {
  datetime().format("%H:%M:%S").to_string()
}

#[cfg(feature = "time")]
#[must_use]
pub fn local_custom(fmt: &str) -> String {
  datetime().format(fmt).to_string()
}

#[cfg(feature = "time")]
#[must_use]
pub fn utc() -> DateTime<Utc> {
  Utc::now()
}

#[cfg(feature = "time")]
#[must_use]
pub fn utc_iso() -> String {
  utc().to_rfc3339()
}

#[cfg(feature = "time")]
#[must_use]
pub fn utc_long() -> String {
  utc().to_rfc2822()
}

#[cfg(feature = "time")]
#[must_use]
pub fn utc_date_only() -> String {
  utc().format("%Y-%m-%d").to_string()
}

#[cfg(feature = "time")]
#[must_use]
pub fn utc_time_only() -> String {
  utc().format("%H:%M:%S").to_string()
}

#[cfg(feature = "time")]
#[must_use]
pub fn utc_custom(fmt: &str) -> String {
  utc().format(fmt).to_string()
}
