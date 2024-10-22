#![cfg_attr(feature = "simd", feature(portable_simd))]

mod config;
mod error;
mod traits;
mod utilities;

pub use config::{allocator, cache, simd};
pub use traits::ToStringRef;
pub use utilities::parse::parse_numeral;

pub fn test() {
	// parse::tests();
	let integer = <num::BigInt as num::FromPrimitive>::from_u64(10)
		.unwrap()
		.pow(100);
	let fractional: usize = isize::MAX as usize;
	let sign = "-";
	match parse_numeral(format!("{}{}.{}", sign, integer, fractional))
	{
		Ok(value) => {
			logline::trace!("   Integer {:?}", integer);
			logline::trace!("Fractional {:?}", fractional);
			logline::trace!("{:?}", value);
			logline::info!("{}", value);
		}
		Err(err) => {
			logline::error!("{}", err);
		}
	}
}
