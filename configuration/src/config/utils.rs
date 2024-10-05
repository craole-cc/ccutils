use crate::{ConfigStore, ConfigTrait, CONFIG_STORE};
use anyhow::Result;
use std::{
	fmt::{Debug, Display},
	sync::Arc,
};
use tokio::sync::Mutex;

pub async fn config_init<T: ConfigTrait + Debug + Display>() -> Result<Arc<T>>
{
	let mut store = CONFIG_STORE
		.get_or_init(|| Mutex::new(ConfigStore::new()))
		.lock()
		.await;
	let config = store.get_or_init::<T>().await?;
	pout(&config, "initialized");
	Ok(config)
}

pub async fn reload<T: ConfigTrait + Debug + Display>(
) -> Result<Arc<T>> {
	let mut store = CONFIG_STORE
		.get_or_init(|| Mutex::new(ConfigStore::new()))
		.lock()
		.await;
	let config = store.reload::<T>().await?;
	pout(&config, "reloaded");
	Ok(config)
}

fn pout<T: ConfigTrait + Debug + Display>(
	config: &Arc<T>,
	action: &str,
) {
	tracing::debug!("{} configuration {}", T::NAME, action);
	tracing::trace!("{} ⦫\n{}", T::NAME, config);
}
