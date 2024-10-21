use std::{error::Error, fmt};

#[derive(Debug)]
pub enum SystemInfoError {
    BatteryError(battery::Error),
    NoBatteryFound,
}

impl fmt::Display for SystemInfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SystemInfoError::BatteryError(e) => {
                write!(f, "Battery error: {}", e)
            }
            SystemInfoError::NoBatteryFound => {
                write!(f, "No battery found")
            }
        }
    }
}

impl Error for SystemInfoError {}

impl From<battery::Error> for SystemInfoError {
    fn from(err: battery::Error) -> Self {
        SystemInfoError::BatteryError(err)
    }
}
