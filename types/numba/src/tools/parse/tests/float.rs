use logline::{error, info, trace, warn};
use num::{BigInt, FromPrimitive};

use crate::parse::{big_decimal, from_str};

pub fn test() {
	warn!("Testing Float");

	max();
	multiple_decimal_points();
	overflow_fractional();
	exceeded_precision();
	negative_fractional();

	fn max() {
		let integer = BigInt::from_u64(10).unwrap().pow(100);
		let fractional: usize = isize::MAX as usize;
		let sign = "-";
		match from_str::float(format!(
			"{}{}.{}",
			sign, integer, fractional
		)) {
			Ok(value) => {
				trace!("   Integer {:?}", integer);
				trace!("Fractional {:?}", fractional);
				trace!("{:?}", value);
				info!("{}", value);
			}
			Err(err) => {
				error!("{}", err);
			}
		}
	}

	fn overflow_fractional() {
		let numeral="10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001.99999";
		// let numeral="1.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000E-9223372036854775707";
		// match from_str::float("1.999999999999999999") {
		match from_str::float(numeral) {
			Ok(result) => {
				info!("{:?} => {:}", result, result);
			}
			Err(err) => {
				error!("{}", err);
			}
		};
	}

	fn exceeded_precision() {
		match from_str::float("1.99999999999999999990") {
			Ok(result) => {
				trace!("{:?} => {:}", result, result);
			}
			Err(err) => {
				error!("{}", err);
			}
		};
	}

	fn negative_fractional() {
		match from_str::float("1.-1") {
			Ok(result) => {
				info!("{:?} => {:}", result, result);
			}
			Err(err) => {
				error!("{}", err);
			}
		};
	}

	fn multiple_decimal_points() {
		match from_str::float("1.2.3") {
			Ok(result) => {
				info!("{:?} => {:}", result, result);
			}
			Err(err) => {
				error!("{}", err);
			}
		};
	}

	println!("...Done");
}

pub fn test_big_decimal() {
	warn!("Testing BigDecimal in Float");

	let good_strings = [
		"1331.107",
		"1.0",
		"2e1",
		"0.00123",
		"-123",
		"-1230",
		"12.3",
		"123e-1",
		"1.23e+1",
		"1.23E+3",
		"1.23E-8",
		"-1.23E-10",
		"123_",
		"31_862_140.830_686_979",
		"-1_1.2_2",
		"999.521_939",
		"679.35_84_03E-2",
		"271576662.__E4",
	];

	let bad_strings = [
		"",
		"123.123E",
		".",
		".e4",
		"_._",
		"hello",
		"nan",
		"12z3.12",
		"123.123eg",
		"123.12.45",
		"0xCafeBeef",
	];

	let num = "12e9.p";
	let float = match big_decimal(num) {
		Ok(result) => result,
		Err(err) => {
			error!("{}", err);
			return;
		}
	};
	warn!("{:?} => {:#?} => {:?}", num, float, float);
	for good in good_strings {
		let decimal = big_decimal(good).unwrap();
		// info!("{:?} => {:#?} => {}", decimal, decimal, decimal);
	}

	for bad in bad_strings {
		let decimal = big_decimal(bad);
		// error!(
		// 	"
		// {:>24} => {:?}",
		// 	bad, decimal
		// );
	}
}
