pub mod error;
pub mod from_str;

use error::Error;

pub fn test() {
	let sign = "-";
	let integer = <num::BigInt as num::FromPrimitive>::from_u64(10)
		.unwrap()
		.pow(100);
	let fractional = isize::MAX;
	let decimal_str = format!("{}{}.{}", sign, integer, fractional);
	let decimal = match from_str::float(decimal_str.as_str()).ok() {
		Some(value) => value,
		None => return println!("Invalid number format"),
	};
	logline::trace!("   Integer {:?}", integer);
	logline::trace!("Fractional {:?}", fractional);
	logline::trace!("{:?}", decimal);
	println!("{}", decimal);
}
