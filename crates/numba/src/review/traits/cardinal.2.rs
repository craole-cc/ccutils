use crate::{
  error::cardinal::Error,
  utilities::cardinal::{numeric_to_worded, worded_from_numeric}
};
use std::convert::TryFrom;

pub trait Cast {
  fn cast(&self) -> Result<(f64, String), Error>;
}

// Implementation for &str
impl Cast for &str {
  fn cast(&self) -> Result<(f64, String), Error> {
    // First, try parsing as an f64
    if let Ok(num) = self.parse::<f64>() {
      return Ok((num, worded_from_numeric(num)));
    }

    // Try parsing as usize
    if let Ok(num) = self.parse::<usize>() {
      return Ok((num as f64, worded_from_numeric(num as f64)));
    }

    // Handle worded input
    if let Ok(num) = numeric_to_worded(self) {
      return Ok((num, worded_from_numeric(num)));
    }

    // If all parsing attempts fail, return an error
    Err(Error::FailedToParseWords(self.to_string()))
  }
}

// Implementation for isize
impl Cast for isize {
  fn cast(&self) -> Result<(f64, String), Error> {
    let num = *self;
    if let Ok(usize_num) = usize::try_from(num) {
      return Ok((usize_num as f64, worded_from_numeric(usize_num as f64)));
    }
    Ok((num as f64, worded_from_numeric(num as f64)))
  }
}

// Implementation for i32
// Implementation for i32
impl Cast for i32 {
  fn cast(&self) -> Result<(f64, String), Error> {
    let num = *self;

    // Try converting to usize if the value is positive and fits
    if let Ok(usize_num) = usize::try_from(num) {
      return Ok((usize_num as f64, worded_from_numeric(usize_num as f64)));
    }

    // Otherwise, use the original i32 value
    Ok((num as f64, worded_from_numeric(num as f64)))
  }
}

// Implementation for i64
impl Cast for i64 {
  fn cast(&self) -> Result<(f64, String), Error> {
    let num = *self;

    // Try converting to usize if it fits within the range
    if let Ok(usize_num) = usize::try_from(num) {
      return Ok((usize_num as f64, worded_from_numeric(usize_num as f64)));
    }

    // Otherwise, use the original i64 value
    Ok((num as f64, worded_from_numeric(num as f64)))
  }
}

// Implementation for u64
impl Cast for u64 {
  fn cast(&self) -> Result<(f64, String), Error> {
    let num = *self;

    // Try converting to usize if it fits within the range
    if let Ok(usize_num) = usize::try_from(num) {
      return Ok((usize_num as f64, worded_from_numeric(usize_num as f64)));
    }

    // Otherwise, use the original u64 value
    Ok((num as f64, worded_from_numeric(num as f64)))
  }
}

// Implementation for f64
impl Cast for f64 {
  fn cast(&self) -> Result<(f64, String), Error> {
    let num = *self;
    Ok((num, worded_from_numeric(num)))
  }
}
