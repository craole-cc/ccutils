use bigdecimal::BigDecimal;
use logline::{error, info, warn};
use num::BigInt;
use std::str::FromStr;

pub fn test() {
  warn!("Testing BigDecimal");
  let int_str = "314005";
  let scale = 2;

  let decimal_from_sci = BigDecimal::from_str("5.67e2").unwrap();
  info!(
    "{:?} | {:#?} | {}",
    decimal_from_sci, decimal_from_sci, decimal_from_sci
  );

  let decimal = BigDecimal::new(
    match BigInt::from_str(int_str) {
      Ok(int) => int,
      Err(err) => {
        error!("Failed to parse BigInt: {}", err);
        return;
      }
    },
    scale as i64
  );

  info!("{:?} | {:#?} | {}", decimal, decimal, decimal);
  info!("{:>24} => {}", "Is Integer", decimal.is_integer());
  info!("{:>24} => {}", "Digits", decimal.digits());
  info!(
    "{:>24} => {}",
    "Fractional Digits",
    decimal.fractional_digit_count()
  );
  info!(
    "{:>24} => {:?}",
    "BigInt and Exponent",
    decimal.as_bigint_and_exponent()
  );
  info!(
    "{:>24} => {}",
    "Scientific Notation",
    decimal.to_scientific_notation()
  );
  info!(
    "{:>24} => {}",
    "Engineering Notation",
    decimal.to_engineering_notation()
  );
  info!("{:>24} => {}", "Absolute", decimal.abs());
  info!("{:>24} => {}", "Normalized", decimal.normalized());
  info!("{:>24} => {}", "Half", decimal.half());
  info!("{:>24} => {}", "Double", decimal.double());
  info!("{:>24} => {}", "Cube", decimal.cube());
  info!("{:>24} => {}", "Square", decimal.square());
  info!("{:>24} => {}", "Root Square", decimal.sqrt().unwrap());
  info!("{:>24} => {}", "Root Cube", decimal.cbrt());
  info!("{:>24} => {}", "Inverse", decimal.inverse());
}

pub fn test_errors() {
  warn!("Testing BigDecimal Errors");
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
    "271576662.__E4"
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
    "0xCafeBeef"
  ];

  for good in good_strings {
    let decimal = BigDecimal::from_str(good).unwrap();
    info!("{:?} => {:#?} => {}", decimal, decimal, decimal);
  }

  for bad in bad_strings {
    let decimal = BigDecimal::from_str(bad);
    error!(
      "
		{:>24} => {:?}",
      bad,
      decimal.err().unwrap()
    );
  }
}
