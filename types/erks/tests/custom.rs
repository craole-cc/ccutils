use erks::{
  Code, Context, CustomError, ErksContext, ErksSeverity, Metadata, custom
};
use std::collections::HashMap;

#[test]
fn test_validation_error() {
  let error = CustomError::validation("Input is required");
  assert_eq!(error.to_string(), "Validation error: Input is required");
  assert_eq!(error.error_code(), Code::ValidationError);
  assert_eq!(error.severity(), ErksSeverity::Warning);
  assert!(error.is_recoverable());
  assert!(error.metadata().is_none());
}

#[test]
fn test_validation_field_error() {
  let error =
    CustomError::validation_field("Must be a valid email", "email_address");
  assert_eq!(error.to_string(), "Validation error: Must be a valid email");
  assert_eq!(error.error_code(), Code::ValidationError);
  let meta = error.metadata().unwrap();
  assert_eq!(meta.context.get("field").unwrap(), "email_address");
}

#[test]
fn test_validation_detailed_error() {
  let constraints = vec!["maxLength: 10".to_string()];
  let error = CustomError::validation_detailed(
    "Too long",
    "username",
    "a_very_long_username",
    constraints
  );
  assert_eq!(error.to_string(), "Validation error: Too long");
  let meta = error.metadata().unwrap();
  assert_eq!(meta.context.get("field").unwrap(), "username");
  assert_eq!(meta.context.get("value").unwrap(), "a_very_long_username");
  assert_eq!(meta.context.get("constraints").unwrap(), "maxLength: 10");
}

#[test]
fn test_invalid_state_error() {
  let error =
    CustomError::invalid_state("Cannot process order in 'shipped' state");
  assert_eq!(
    error.to_string(),
    "Invalid state: Cannot process order in 'shipped' state"
  );
  assert_eq!(error.error_code(), Code::InvalidState);
  assert_eq!(error.severity(), ErksSeverity::Critical);
  assert!(!error.is_recoverable());
}

#[test]
fn test_invalid_state_transition_error() {
  let error = CustomError::invalid_state_transition(
    "Cannot move from shipped to cancelled",
    "shipped",
    "pending",
    "cancel_order"
  );
  let meta = error.metadata().unwrap();
  assert_eq!(meta.context.get("current_state").unwrap(), "shipped");
  assert_eq!(meta.context.get("expected_state").unwrap(), "pending");
  assert_eq!(meta.context.get("transition").unwrap(), "cancel_order");
}

#[test]
fn test_resource_limit_error() {
  let error = CustomError::resource_limit(
    "API rate limit exceeded",
    "api_calls",
    100,
    101
  );
  assert_eq!(
    error.to_string(),
    "Resource limit exceeded: API rate limit exceeded"
  );
  assert_eq!(error.error_code(), Code::ResourceLimit);
  assert_eq!(error.severity(), ErksSeverity::Error);
  assert!(error.is_recoverable());
  let meta = error.metadata().unwrap();
  assert_eq!(meta.context.get("resource_type").unwrap(), "api_calls");
  assert_eq!(meta.context.get("limit").unwrap(), "100");
  assert_eq!(meta.context.get("current").unwrap(), "101");
}

#[test]
fn test_business_logic_error() {
  let error =
    CustomError::business_logic("User is not eligible for this action");
  assert_eq!(
    error.to_string(),
    "Business logic error: User is not eligible for this action"
  );
  assert_eq!(error.error_code(), Code::BusinessLogic);
  assert_eq!(error.severity(), ErksSeverity::Error);
  assert!(!error.is_recoverable());
}

#[test]
fn test_business_logic_with_context() {
  let mut context = HashMap::new();
  context.insert("user_id".to_string(), "123".to_string());
  let error = CustomError::business_logic_with_context(
    "Insufficient funds",
    "process_payment",
    context
  );
  let meta = error.metadata().unwrap();
  assert_eq!(meta.operation.unwrap(), "process_payment");
  assert_eq!(meta.context.get("user_id").unwrap(), "123");
}

#[test]
fn test_generic_custom_error() {
  let error = CustomError::new("Something unexpected happened");
  assert_eq!(
    error.to_string(),
    "Application error: Something unexpected happened"
  );
  assert_eq!(error.error_code(), Code::Unknown);
  assert!(!error.is_recoverable());
}
