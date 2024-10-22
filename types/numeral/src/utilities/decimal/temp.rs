// Cargo.toml
[package]
name = "decimal-parse"
version = "0.1.0"
edition = "2021"

[dependencies]
rust_decimal = "1.32"
bigdecimal = { version = "0.4", optional = true }
mimalloc = { version = "0.1", optional = true }
lru = "0.12"
dashmap = "5.5"
simd-json = "0.13"
thiserror = "1.0"
cfg-if = "1.0"

[features]
default = ["std-alloc"]
big-decimal = ["dep:bigdecimal"]
std-alloc = []
mi-alloc = ["dep:mimalloc"]
simd = []

[dev-dependencies]
criterion = "0.5"
proptest = "1.3"
test-case = "3.3"

[[bench]]
name = "parse_bench"
harness = false

// src/lib.rs
#![cfg_attr(feature = "simd", feature(portable_simd))]

mod allocator;
mod cache;
mod error;
mod parse;
mod simd;
mod traits;

pub use error::Error;
pub use parse::{parse_decimal, parse_number};
pub use traits::ToStringRef;

#[cfg(feature = "mi-alloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error<'a> {
    #[error("Invalid decimal format: {0}")]
    InvalidFormat(&'a str),
    #[error("Number too large: {0}")]
    Overflow(&'a str),
    #[error("Invalid scientific notation: {0}")]
    InvalidScientific(&'a str),
    #[cfg(feature = "big-decimal")]
    #[error("BigDecimal error: {0}")]
    BigDecimal(#[from] bigdecimal::ParseBigDecimalError),
    #[error("Decimal error: {0}")]
    Decimal(#[from] rust_decimal::Error),
}

// src/cache.rs
use dashmap::DashMap;
use lru::LruCache;
use std::sync::Mutex;

const CACHE_SIZE: usize = 10_000;

pub struct ParseCache {
    fast_cache: DashMap<String, rust_decimal::Decimal>,
    #[cfg(feature = "big-decimal")]
    big_cache: Mutex<LruCache<String, bigdecimal::BigDecimal>>,
}

impl ParseCache {
    pub fn new() -> Self {
        Self {
            fast_cache: DashMap::with_capacity(CACHE_SIZE),
            #[cfg(feature = "big-decimal")]
            big_cache: Mutex::new(LruCache::new(CACHE_SIZE)),
        }
    }

    #[inline]
    pub fn get_decimal(&self, key: &str) -> Option<rust_decimal::Decimal> {
        self.fast_cache.get(key).map(|v| *v)
    }

    #[cfg(feature = "big-decimal")]
    #[inline]
    pub fn get_big_decimal(&self, key: &str) -> Option<bigdecimal::BigDecimal> {
        self.big_cache
            .lock()
            .unwrap()
            .get(key)
            .map(|v| v.clone())
    }

    #[inline]
    pub fn insert_decimal(&self, key: String, value: rust_decimal::Decimal) {
        self.fast_cache.insert(key, value);
    }

    #[cfg(feature = "big-decimal")]
    #[inline]
    pub fn insert_big_decimal(&self, key: String, value: bigdecimal::BigDecimal) {
        self.big_cache.lock().unwrap().put(key, value);
    }
}

lazy_static::lazy_static! {
    static ref PARSE_CACHE: ParseCache = ParseCache::new();
}

// src/simd.rs
#[cfg(feature = "simd")]
use std::simd::{u8x32, mask8x32, Simd};

#[cfg(feature = "simd")]
#[inline]
pub fn remove_commas_simd(input: &str) -> String {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut result = Vec::with_capacity(len);

    // Process 32 bytes at a time
    let chunks = bytes.chunks_exact(32);
    let remainder = chunks.remainder();

    let comma_vec = u8x32::splat(b',');

    for chunk in chunks {
        let v = u8x32::from_slice(chunk);
        let mask = v.ne(comma_vec);

        // Extract non-comma bytes
        for (i, &byte) in chunk.iter().enumerate() {
            if mask[i] {
                result.push(byte);
            }
        }
    }

    // Handle remaining bytes
    for &byte in remainder {
        if byte != b',' {
            result.push(byte);
        }
    }

    // Safe because we only included valid UTF-8 bytes
    unsafe { String::from_utf8_unchecked(result) }
}

#[cfg(not(feature = "simd"))]
#[inline]
pub fn remove_commas_simd(input: &str) -> String {
    input.chars().filter(|&c| c != ',').collect()
}

// src/parse.rs
use crate::{cache::PARSE_CACHE, error::Error, simd::remove_commas_simd};
use std::str::FromStr;

/// Fast path for parsing common numeric types
#[inline]
pub fn parse_decimal<T: ToString>(input: T) -> Result<rust_decimal::Decimal, Error<'static>> {
    let input_str = input.to_string();

    // Check cache first
    if let Some(cached) = PARSE_CACHE.get_decimal(&input_str) {
        return Ok(cached);
    }

    // Fast path for integers
    if let Ok(i) = input_str.parse::<i64>() {
        let decimal = rust_decimal::Decimal::from(i);
        PARSE_CACHE.insert_decimal(input_str, decimal);
        return Ok(decimal);
    }

    // Handle commas and parse
    let cleaned = if input_str.contains(',') {
        remove_commas_simd(&input_str)
    } else {
        input_str
    };

    let result = rust_decimal::Decimal::from_str(&cleaned)
        .map_err(Error::Decimal)?;

    PARSE_CACHE.insert_decimal(cleaned, result);
    Ok(result)
}

#[cfg(feature = "big-decimal")]
#[inline]
pub fn parse_big_decimal(input: &str) -> Result<bigdecimal::BigDecimal, Error<'static>> {
    // Check cache first
    if let Some(cached) = PARSE_CACHE.get_big_decimal(input) {
        return Ok(cached);
    }

    let result = bigdecimal::BigDecimal::from_str(input)
        .map_err(Error::BigDecimal)?;

    PARSE_CACHE.insert_big_decimal(input.to_owned(), result.clone());
    Ok(result)
}

/// Parse number with fallback to BigDecimal if needed
#[inline]
pub fn parse_number<T: ToString>(input: T) -> Result<rust_decimal::Decimal, Error<'static>> {
    let result = parse_decimal(input);

    #[cfg(feature = "big-decimal")]
    {
        if let Err(Error::Decimal(_)) = result {
            // Try BigDecimal and convert back if possible
            return parse_big_decimal(&input.to_string())
                .and_then(|bd| {
                    rust_decimal::Decimal::try_from(bd)
                        .map_err(|_| Error::Overflow("Number too large for Decimal"))
                });
        }
    }

    result
}

// tests/parse_tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("123" => Ok(rust_decimal::Decimal::new(123, 0)) ; "integer")]
    #[test_case("123.45" => Ok(rust_decimal::Decimal::new(12345, 2)) ; "decimal")]
    #[test_case("1,234.56" => Ok(rust_decimal::Decimal::new(123456, 2)) ; "with commas")]
    #[test_case("invalid" => Err(Error::InvalidFormat("invalid")) ; "invalid")]
    fn test_parse_decimal(input: &str) -> Result<rust_decimal::Decimal, Error> {
        parse_decimal(input)
    }

    #[cfg(feature = "big-decimal")]
    mod big_decimal_tests {
        use super::*;

        #[test]
        fn test_large_numbers() {
            let large = "1".repeat(100);
            assert!(parse_decimal(&large).is_err());
            assert!(parse_big_decimal(&large).is_ok());
        }
    }

    proptest::proptest! {
        #[test]
        fn doesnt_crash(s in "\\PC*") {
            let _ = parse_decimal(s);
        }
    }
}

// benches/parse_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use decimal_parse::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_decimal");

    group.bench_function("small integer", |b| {
        b.iter(|| parse_decimal(black_box("123")))
    });

    group.bench_function("decimal", |b| {
        b.iter(|| parse_decimal(black_box("123.45")))
    });

    group.bench_function("with commas", |b| {
        b.iter(|| parse_decimal(black_box("1,234,567.89")))
    });

    #[cfg(feature = "big-decimal")]
    group.bench_function("large number", |b| {
        b.iter(|| parse_decimal(black_box(&"1".repeat(100))))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
