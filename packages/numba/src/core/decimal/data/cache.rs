use dashmap::DashMap;
use lru::LruCache;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub const CACHE_SIZE: usize = 10_000;
pub static CACHE: Lazy<Cache> = Lazy::new(Cache::new);

#[derive(Debug)]
pub struct Cache {
  pub for_rust_decimal: DashMap<String, rust_decimal::Decimal>,

  #[cfg(feature = "big-decimal")]
  pub for_big_decimal: Mutex<LruCache<String, bigdecimal::BigDecimal>>
}
