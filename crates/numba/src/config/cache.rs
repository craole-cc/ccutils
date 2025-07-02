use dashmap::DashMap;
use lru::LruCache;
use once_cell::sync::Lazy;
use std::{num::NonZeroUsize, sync::Mutex};

const CACHE_SIZE: usize = 10_000;
pub static CACHE: Lazy<Cache> = Lazy::new(Cache::new);

#[derive(Debug, Default)]
pub struct Cache {}

impl Cache {
  pub fn new() -> Self {
    Self::default()
  }
}
