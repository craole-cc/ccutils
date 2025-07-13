use crate::{Code, Context, Metadata, Severity};
use std::collections::HashMap;

/// Application-specific error types
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Business logic validation failed
  #[error("Validation error: {message}")]
  Validation {
    message: String,
    field: Option<String>,
    value: Option<String>,
    constraints: Vec<String>
  },

  /// Application state is inconsistent
  #[error("Invalid state: {message}")]
  InvalidState {
    message: String,
    current_state: Option<String>,
    expected_state: Option<String>,
    transition: Option<String>
  },

  /// Resource limit exceeded
  #[error("Resource limit exceeded: {message}")]
  ResourceLimit {
    message: String,
    resource_type: String,
    limit: Option<usize>,
    current: Option<usize>,
    unit: Option<String>
  },

  /// Business logic error
  #[error("Business logic error: {message}")]
  BusinessLogic {
    message: String,
    operation: Option<String>,
    context: HashMap<String, String>
  },

  /// Generic custom error
  #[error("Application error: {message}")]
  Generic {
    message: String,
    context: HashMap<String, String>
  }
}

impl Error {
  /// Create a new validation error
  pub fn validation<T: Into<String>>(msg: T) -> Self {
    Self::Validation {
      message: msg.into(),
      field: None,
      value: None,
      constraints: Vec::new()
    }
  }

  /// Create a validation error for a specific field
  pub fn validation_field<T: Into<String>, F: Into<String>>(msg: T, field: F) -> Self {
    Self::Validation {
      message: msg.into(),
      field: Some(field.into()),
      value: None,
      constraints: Vec::new()
    }
  }

  /// Create a detailed validation error
  pub fn validation_detailed<T: Into<String>, F: Into<String>, V: Into<String>>(
    msg: T,
    field: F,
    value: V,
    constraints: Vec<String>
  ) -> Self {
    Self::Validation {
      message: msg.into(),
      field: Some(field.into()),
      value: Some(value.into()),
      constraints
    }
  }

  /// Create an invalid state error
  pub fn invalid_state<T: Into<String>>(msg: T) -> Self {
    Self::InvalidState {
      message: msg.into(),
      current_state: None,
      expected_state: None,
      transition: None
    }
  }

  /// Create a detailed invalid state error
  pub fn invalid_state_transition<T: Into<String>, C: Into<String>, E: Into<String>, Tr: Into<String>>(
    msg: T,
    current: C,
    expected: E,
    transition: Tr
  ) -> Self {
    Self::InvalidState {
      message: msg.into(),
      current_state: Some(current.into()),
      expected_state: Some(expected.into()),
      transition: Some(transition.into())
    }
  }

  /// Create a resource limit error
  pub fn resource_limit<T: Into<String>, R: Into<String>>(
    msg: T,
    resource_type: R,
    limit: usize,
    current: usize
  ) -> Self {
    Self::ResourceLimit {
      message: msg.into(),
      resource_type: resource_type.into(),
      limit: Some(limit),
      current: Some(current),
      unit: None
    }
  }

  /// Create a resource limit error with units
  pub fn resource_limit_with_unit<T: Into<String>, R: Into<String>, U: Into<String>>(
    msg: T,
    resource_type: R,
    limit: usize,
    current: usize,
    unit: U
  ) -> Self {
    Self::ResourceLimit {
      message: msg.into(),
      resource_type: resource_type.into(),
      limit: Some(limit),
      current: Some(current),
      unit: Some(unit.into())
    }
  }

  /// Create a business logic error
  pub fn business_logic<T: Into<String>>(msg: T) -> Self {
    Self::BusinessLogic {
      message: msg.into(),
      operation: None,
      context: HashMap::new()
    }
  }

  /// Create a business logic error with operation context
  pub fn business_logic_with_operation<T: Into<String>, O: Into<String>>(msg: T, operation: O) -> Self {
    Self::BusinessLogic {
      message: msg.into(),
      operation: Some(operation.into()),
      context: HashMap::new()
    }
  }

  /// Create a business logic error with full context
  pub fn business_logic_with_context<T: Into<String>, O: Into<String>>(
    msg: T,
    operation: O,
    context: HashMap<String, String>
  ) -> Self {
    Self::BusinessLogic {
      message: msg.into(),
      operation: Some(operation.into()),
      context
    }
  }

  /// Create a generic custom error
  pub fn new<T: Into<String>>(msg: T) -> Self {
    Self::Generic {
      message: msg.into(),
      context: HashMap::new()
    }
  }

  /// Create a generic custom error with context
  pub fn new_with_context<T: Into<String>>(msg: T, context: HashMap<String, String>) -> Self {
    Self::Generic {
      message: msg.into(),
      context
    }
  }
}

impl Context for Error {
  fn description(&self) -> String {
    format!("{self}")
  }

  fn is_recoverable(&self) -> bool {
    match self {
      Error::Validation { .. } => true,
      Error::InvalidState { .. } => false,
      Error::ResourceLimit { .. } => true,
      Error::BusinessLogic { .. } => false,
      Error::Generic { .. } => false
    }
  }

  fn severity(&self) -> Severity {
    match self {
      Error::Validation { .. } => Severity::Warning,
      Error::InvalidState { .. } => Severity::Critical,
      Error::ResourceLimit { .. } => Severity::Error,
      Error::BusinessLogic { .. } => Severity::Error,
      Error::Generic { .. } => Severity::Error
    }
  }

  fn error_code(&self) -> Code {
    match self {
      Error::Validation { .. } => Code::ValidationError,
      Error::InvalidState { .. } => Code::InvalidState,
      Error::ResourceLimit { .. } => Code::ResourceLimit,
      Error::BusinessLogic { .. } => Code::BusinessLogic,
      Error::Generic { .. } => Code::Unknown
    }
  }

  fn metadata(&self) -> Option<Metadata> {
    let metadata = Metadata::new(self.error_code()).with_component("application");

    match self {
      Error::Validation {
        field: Some(f),
        value: Some(v),
        constraints,
        ..
      } => {
        let mut meta = metadata
          .with_context("field", f.clone())
          .with_context("value", v.clone());

        if !constraints.is_empty() {
          meta = meta.with_context("constraints", constraints.join(", "));
        }

        Some(meta)
      }
      Error::Validation { field: Some(f), .. } => Some(metadata.with_context("field", f.clone())),
      Error::InvalidState {
        current_state: Some(c),
        expected_state: Some(e),
        transition: Some(t),
        ..
      } => Some(
        metadata
          .with_context("current_state", c.clone())
          .with_context("expected_state", e.clone())
          .with_context("transition", t.clone())
      ),
      Error::ResourceLimit {
        resource_type,
        limit: Some(l),
        current: Some(c),
        unit,
        ..
      } => {
        let mut meta = metadata
          .with_context("resource_type", resource_type.clone())
          .with_context("limit", l.to_string())
          .with_context("current", c.to_string());

        if let Some(u) = unit {
          meta = meta.with_context("unit", u.clone());
        }

        Some(meta)
      }
      Error::BusinessLogic {
        operation: Some(op),
        context,
        ..
      } => {
        let mut meta = metadata.with_operation(op.clone());

        for (key, value) in context {
          meta = meta.with_context(key.clone(), value.clone());
        }

        Some(meta)
      }
      Error::Generic { context, .. } if !context.is_empty() => {
        let mut meta = metadata;

        for (key, value) in context {
          meta = meta.with_context(key.clone(), value.clone());
        }

        Some(meta)
      }
      _ => None
    }
  }
}
