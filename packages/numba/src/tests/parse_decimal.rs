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
