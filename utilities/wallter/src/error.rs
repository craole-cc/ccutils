use crate::{config::color::mode::windows::nightlight, utils::parse};
use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("API error: {0}")]
  API(String),

  #[error("Configuration error: {0}")]
  Config(String),

  #[error("IO error: {0}")]
  IO(#[from] io::Error),

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
  Parse(#[from] parse::Error) /* #[error("Night Light error: {0}")]
                               * NightLight(#[from] Box<nightlight::Error>) */
}
