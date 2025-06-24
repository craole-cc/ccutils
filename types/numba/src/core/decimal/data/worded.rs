#[derive(Debug)]
pub enum Worded {
	Small(rust_decimal::Decimal),
	#[cfg(feature = "big-decimal")]
	Large(bigdecimal::BigDecimal),
}
