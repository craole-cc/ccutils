pub trait Display {
    fn fetch(&self) -> String;
    fn all(&self) -> String;
    fn statement(&self) -> String;
    fn now_fmt(&self) -> String;
    fn booted_fmt(&self) -> String;
    fn timezone_fmt(&self) -> String;
    fn active_fmt(&self) -> String;
}

impl Display for super::Info {
    fn fetch(&self) -> String {
        format!(
            "Time {{\n\
			{:>16}: {}\n\
			{:>16}: {}\n\
			{:>16}: {}\n\
			{:>16}: {}\n\
			}}",
            "Active",
            self.active_fmt(),
            "Booted",
            self.booted_fmt(),
            "Now",
            self.now_fmt(),
            "Timezone",
            self.timezone_fmt()
        )
    }

    fn all(&self) -> String {
        format!(
            "Now: {}\nBooted: {}\nZone: {}\nActive: {}",
            self.now_fmt(),
            self.booted_fmt(),
            self.timezone_fmt(),
            self.active_fmt()
        )
    }

    fn statement(&self) -> String {
        "time statement".to_string()
    }

    fn now_fmt(&self) -> String {
        format!("{}", self.now.format(self.dtfmt))
    }

    fn booted_fmt(&self) -> String {
        format!("{}", self.booted.format(self.dtfmt))
    }

    fn active_fmt(&self) -> String {
        format!("{}", "self.active")
    }

    fn timezone_fmt(&self) -> String {
        format!("{:?}", self.timezone)
    }
}
