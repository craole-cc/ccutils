#[cfg(any(feature = "json", feature = "toml"))]
mod parse_tests {
  use erks::{Code, Context, ErksContext, ErksSeverity, ParseError};

  #[cfg(feature = "json")]
  #[test]
  fn test_json_parse_error() {
    let json_err = serde_json::from_str::<serde_json::Value>("{,}");
    assert!(json_err.is_err());
    let error = ParseError::from(json_err.unwrap_err());

    assert!(error.to_string().starts_with("JSON parsing error:"));
    assert_eq!(error.error_code(), Code::ParseError);
    assert_eq!(error.severity(), ErksSeverity::Error);
    assert!(error.is_recoverable());
    assert!(error.metadata().is_none());
  }

  #[cfg(feature = "toml")]
  #[test]
  fn test_toml_parse_error() {
    let toml_err = toml::from_str::<toml::Value>("key = 'value");
    assert!(toml_err.is_err());
    let error = ParseError::from(toml_err.unwrap_err());

    assert!(error.to_string().starts_with("TOML parsing error:"));
    assert_eq!(error.error_code(), Code::ParseError);
    assert_eq!(error.severity(), ErksSeverity::Error);
    assert!(error.is_recoverable());
    assert!(error.metadata().is_none());
  }

  #[test]
  fn test_generic_parse_error() {
    let error = ParseError::generic("Invalid character", "custom_format");
    assert_eq!(error.to_string(), "Parsing error: Invalid character");
    assert_eq!(error.error_code(), Code::ParseError);
  }
}
