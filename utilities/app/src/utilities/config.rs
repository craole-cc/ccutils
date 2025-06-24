use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub package_managers: HashMap<String, Vec<String>>,
  pub sources: HashMap<String, Vec<String>>
}

impl Config {
  pub fn load() -> Result<Self> {
    // Implementation for loading config
    todo!()
  }
}
