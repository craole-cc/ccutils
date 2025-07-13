use crate::decimal::{CACHE_SIZE, Cache};
use dashmap::DashMap;
use lru::LruCache;
use once_cell::sync::Lazy;
use std::{num::NonZeroUsize, sync::Mutex};

impl Default for Cache {
  fn default() -> Self {
    Self {
      for_rust_decimal: DashMap::with_capacity(CACHE_SIZE),

      #[cfg(feature = "big-decimal")]
      for_big_decimal: Mutex::new(LruCache::new(NonZeroUsize::new(CACHE_SIZE).unwrap()))
    }
  }
}

impl Cache {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn get_rust_decimal(&self, key: &str) -> Option<rust_decimal::Decimal> {
    self.for_rust_decimal.get(key).map(|v| *v)
  }

  #[cfg(feature = "big-decimal")]
  pub fn get_big_decimal(&self, key: &str) -> Option<bigdecimal::BigDecimal> {
    self.for_big_decimal.lock().unwrap().get(key).cloned()
  }

  pub fn insert_rust_decimal(&self, key: String, value: rust_decimal::Decimal) {
    self.for_rust_decimal.insert(key, value);
  }

  #[cfg(feature = "big-decimal")]
  pub fn insert_big_decimal(&self, key: String, value: bigdecimal::BigDecimal) {
    self.for_big_decimal.lock().unwrap().put(key, value);
  }
}
