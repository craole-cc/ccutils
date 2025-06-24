#[derive(Debug, Clone)]
pub struct Battery {
    pub level: f32,
    pub status: battery::State,
    pub time_to_full: Option<battery::units::Time>,
    pub time_to_empty: Option<battery::units::Time>,
    pub time_left: crate::Duration,
    pub technology: battery::Technology,
    pub cycles: Option<u32>,
    pub brand: Option<String>,
    // energy: 42577.2 m^2 kg^1 s^-2,
    // energy_full: 182577.6 m^2 kg^1 s^-2,
    // energy_full_design: 182577.6 m^2 kg^1 s^-2,
    // energy_rate: 36.775 m^2 kg^1 s^-3,
    // voltage: 12.239 m^2 kg^1 s^-3 A^-1,
}
