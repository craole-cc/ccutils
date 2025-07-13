use crate::prelude::*;

// -- Main Application Logging --

/// Initialize logging for the main application
pub fn init() -> Result<()> {
  let mut logger = logline::Config::default();
  logger.init();
  // trace!("Logline \n{:#?}", &logger);
  // info!("Initialized logging for {APP} v.{VERSION}");
  Ok(())
}

// -- Test Logging --

#[cfg(test)]
pub mod testing {
  use std::sync::Once;

  static INIT: Once = Once::new();

  /// Initialize logging for tests (thread-safe, runs only once)
  pub fn init_logging() {
    INIT.call_once(|| {
      super::init().expect("Failed to initialize logging for tests");
    });
  }

  /// Convenience function that automatically initializes logging for tests
  pub fn init() {
    super::testing::init_logging();
  }
}

// -- Test Macros --

#[cfg(test)]
#[macro_export]
macro_rules! with_logging {
  ($name:ident, $body:block) => {
    #[test]
    fn $name() {
      $crate::utils::log::testing::init();
      $body
    }
  };
}

#[cfg(test)]
#[macro_export]
macro_rules! with_logging_async {
  ($name:ident, $body:block) => {
    #[tokio::test]
    async fn $name() {
      $crate::utils::log::testing::init();
      $body
    }
  };
}

#[cfg(test)]
mod tests {
  use super::*;
  use colored::*;

  #[test]
  fn test_logging_init() {
    testing::init();
    info!("Test logging initialization works!");
    eprintln!("{}", "[ERROR] This is an error message".red().bold());
    eprintln!("{}", "[WARN] This is an error message".yellow());
    eprintln!("{}", "[INFO] This is an error message".blue());
    eprintln!("{}", "[DEBUG] This is an error message".magenta().dimmed());
    eprintln!("{}", "[TRACE] This is an error message".green());
    debug!("Debug message");
    trace!("Trace message");
  }

  // Example using the macro
  with_logging!(test_macro_usage, {
    info!("This test uses the macro for automatic logging setup");
    assert_eq!(1, 1);
  });
}
