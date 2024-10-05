use super::Config;
use crate::{config, config_init};
use anyhow::Result;
use std::sync::Arc;

pub async fn init() -> Result<Arc<Config>> {
	// let cfg = config::init::<Config>().await?;
	config_init::<Config>().await
}

pub async fn reload() -> Result<Arc<Config>> {
	// let cfg = config::reload::<Config>().await?;
	config::reload::<Config>().await
}
