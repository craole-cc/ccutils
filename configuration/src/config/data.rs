use anyhow::{anyhow, Result};
use std::{
	any::{Any, TypeId},
	collections::HashMap,
	fmt::{Debug, Display},
	future::Future,
	sync::{Arc, OnceLock},
};
use tokio::sync::Mutex;

pub trait ConfigTrait: Any + Send + Sync + Sized + 'static {
	const NAME: &'static str;
	fn load_data() -> impl Future<Output = Result<Self>> + Send;
}

#[derive(Default)]
pub struct ConfigStore {
	configs: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl ConfigStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub async fn get_or_init<T: ConfigTrait + Debug + Display>(
		&mut self,
	) -> Result<Arc<T>> {
		if let Some(config) = self.configs.get(&TypeId::of::<T>()) {
			config
				.clone()
				.downcast::<T>()
				.map_err(|_| anyhow!("Failed to downcast config"))
		} else {
			let new_config = Arc::new(T::load_data().await?);
			self.configs
				.insert(TypeId::of::<T>(), new_config.clone());
			Ok(new_config)
		}
	}

	pub async fn reload<T: ConfigTrait + Debug + Display>(
		&mut self,
	) -> Result<Arc<T>> {
		let new_config = Arc::new(T::load_data().await?);
		self.configs.insert(TypeId::of::<T>(), new_config.clone());
		Ok(new_config)
	}
}

pub static CONFIG_STORE: OnceLock<Mutex<ConfigStore>> =
	OnceLock::new();
