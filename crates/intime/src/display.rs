use crate::{Info, Options};
use std::fmt::{self, Display, Formatter};

impl Display for Info {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        let units = [
            ("year", self.years, self.options.hide_years),
            ("month", self.months, self.options.hide_months),
            ("week", self.weeks, self.options.hide_weeks),
            ("day", self.days, self.options.hide_days),
            ("hour", self.hours, self.options.hide_hours),
            ("minute", self.minutes, self.options.hide_minutes),
            ("second", self.seconds, self.options.hide_seconds),
        ];

        for (unit, value, hide) in units {
            if !hide && value > 0 {
                let plural = if value == 1 { "" } else { "s" };
                let value_str = if unit == "second" {
                    format!("{:.3}", value)
                } else {
                    format!("{}", value)
                };
                parts.push(format!("{} {}{}", value_str, unit, plural));
            }
        }

        match parts.len() {
            0 => write!(f, "no time"),
            1 => write!(f, "{}", parts[0]),
            2 => write!(f, "{} and {}", parts[0], parts[1]),
            _ => {
                let last = parts.pop().unwrap();
                write!(f, "{}, and {}", parts.join(", "), last)
            }
        }
    }
}

impl Display for Options {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "  Hide Years: {}", self.hide_years)?;
        writeln!(f, "  Hide Months: {}", self.hide_months)?;
        writeln!(f, "  Hide Weeks: {}", self.hide_weeks)?;
        writeln!(f, "  Hide Days: {}", self.hide_days)?;
        writeln!(f, "  Hide Hours: {}", self.hide_hours)?;
        writeln!(f, "  Hide Minutes: {}", self.hide_minutes)?;
        writeln!(f, "  Hide Seconds: {}", self.hide_seconds)
    }
}
