#![cfg_attr(feature = "simd", feature(portable_simd))]

mod config;
pub use config::allocator;

mod core;
pub use core::*;

mod tools;
pub use tools::*;

pub fn test() {
	decimal::test();
}
