use crate::{Info, Options};
use battery::units::Time as BatTime;
use chrono::{DateTime, Duration, Local};
use uom::si::{f64::Time as SiTime64, time::second};

impl Info {
    pub fn fetch(&self) -> String {
        format!(
            "Duration {{\n\
			{:>16}: {}\n\
			{:>16}: {}\n\
			{:>16}: {}\n\
			{:>16}: {}\n\
			{:>16}: {}\n\
			{:>16}: {}\n\
			{:>16}: {}\n\
			{:>16}: {}\n\
			}}",
            "Years",
            self.years,
            "Months",
            self.months,
            "Weeks",
            self.weeks,
            "Days",
            self.days,
            "Hours",
            self.hours,
            "Minutes",
            self.minutes,
            "Seconds",
            self.seconds,
            "Options",
            self.options
        )
    }

    pub fn set_options(&mut self, options: Options) {
        self.options = options;
    }

    pub fn with_options(mut self, options: Options) -> Self {
        self.options = options;
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

    pub fn from_si_time(si_time: SiTime64) -> Self {
        Self::from_secs_f64(si_time.get::<second>())
    }

    pub fn from_secs_f64(duration: f64) -> Self {
        let total_minutes = (duration / 60.0).floor() as i64;
        let remaining_seconds = (duration % 60.0).floor() as i64;

        let chrono_duration = Duration::seconds(duration as i64);
        let mut duration = Self::from_delta(chrono_duration);
        duration.seconds = remaining_seconds;
        duration
    }

    pub fn from_delta(duration: Duration) -> Self {
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

        let options = Options::default();

        Self {
            years,
            months,
            weeks,
            days,
            hours,
            minutes,
            seconds,
            options,
        }
    }

    pub fn until_now(start: DateTime<Local>) -> Self {
        Self::start_to_finish(start, Local::now())
    }

    pub fn start_to_finish(start: DateTime<Local>, end: DateTime<Local>) -> Self {
        let duration = end.signed_duration_since(start);
        Self::from_delta(duration)
    }

    pub fn above_seconds(&mut self) -> &Self {
        self.options.hide_seconds = true;
        self
    }

    pub fn in_minutes(&mut self) -> &Self {
        self.options = Options {
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
        self.options = Options {
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
        self.options = Options {
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
