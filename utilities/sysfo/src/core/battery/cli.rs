use super::{Display, Info};

#[derive(clap::Parser)]
#[command(about = "Get information about the system battery")]
pub enum Commands {
    #[command(
        about = "Display a natural language statement about the battery",
        long_about = "Provides a detailed, human-readable statement regarding the current battery status."
    )]
    Statement,

    #[command(
        about = "Show all battery information",
        long_about = "Displays comprehensive information about the battery, including level, status, and more."
    )]
    All,

    #[command(
        about = "Show battery level percentage",
        long_about = "Indicates the current battery level as a percentage."
    )]
    Level,

    #[command(
        about = "Show battery charging status",
        long_about = "Reveals whether the battery is currently charging, discharging, or fully charged."
    )]
    Status,

    #[command(
        about = "Show time remaining for charge/discharge",
        long_about = "Estimates the remaining time for the battery to fully charge or discharge based on current usage."
    )]
    Time,

    #[command(
        about = "Show battery technology",
        long_about = "Provides details about the battery's technology, such as Li-ion or NiMH."
    )]
    Technology,

    #[command(
        about = "Show battery cycle count",
        long_about = "Displays the number of charge cycles the battery has undergone."
    )]
    Cycle,

    #[command(
        about = "Show battery manufacturer",
        long_about = "Identifies the company that manufactured the battery."
    )]
    Brand,
}

impl Default for Commands {
    fn default() -> Self {
        Self::All
    }
}

impl Commands {
    pub fn handle_command(&self, info: &Info) -> String {
        match self {
            Self::Statement => info.statement(),
            Self::All => info.all(),
            Self::Level => info.pretty_level(),
            Self::Status => info.pretty_status(),
            Self::Time => info.pretty_time_left(),
            Self::Technology => info.pretty_technology(),
            Self::Cycle => info.pretty_cycles().0.to_string(),
            Self::Brand => info.pretty_brand().to_string(),
        }
    }
}
