pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("API error: {0}")]
  API(String),

  #[error("Configuration error: {0}")]
  Config(String),

  #[error("I/O error: {0}")]
  System(#[from] std::io::Error),

  #[error("Network error: {0}")]
  Network(#[from] reqwest::Error),

  #[error("Image processing error: {0}")]
  Image(String),

  #[error("Monitor detection error: {0}")]
  Monitor(#[from] crate::config::monitor::Error),

  #[error("Invalid settings: {0}")]
  Settings(String),

  #[error("Color mode error: {0}")]
  ColorMode(String),

  #[error("Parse error: {0}")]
  Parse(#[from] crate::utils::parse::Error),

  #[cfg(target_os = "windows")]
  #[error("Night Light error: {0}")]
  NightLight(
    #[from] Box<crate::Error> /* #[from] Box<crate::config::color::mode::windows::nightlight::Error> */
  )
}
