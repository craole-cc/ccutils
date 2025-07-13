// mod config;
// mod init;
mod pacman;

// pub use config::Config;
// pub use init::init;
pub use pacman::{PackageManager, detect_package_managers, get_default_managers};
