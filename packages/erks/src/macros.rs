/// Create a custom error with formatted message
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::Error::custom(format!($($arg)*))
    };
}

/// Bail with a custom error
#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {
        return Err($crate::error!($($arg)*))
    };
}

/// Ensure condition or return error
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $($arg:tt)*) => {
        if !$cond {
            $crate::bail!($($arg)*);
        }
    };
}
