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
