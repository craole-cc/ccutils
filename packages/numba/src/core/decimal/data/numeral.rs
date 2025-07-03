#[derive(Debug)]
pub enum Numeral {
  Small(rust_decimal::Decimal),
  #[cfg(feature = "big-decimal")]
  Large(bigdecimal::BigDecimal)
}
