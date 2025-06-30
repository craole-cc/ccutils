use erks::{Code, Context, ErksContext, ErksSeverity, IoError, Metadata};

#[test]
fn test_file_system_error() {
  let error = IoError::file_system("Disk is full");
  assert_eq!(error.to_string(), "File system error: Disk is full");
  assert_eq!(error.error_code(), Code::IoError);
  assert_eq!(error.severity(), ErksSeverity::Error);
  assert!(error.is_recoverable());
}

#[test]
fn test_file_system_with_path() {
  let error =
    IoError::file_system_with_path("Cannot write", "/var/log/app.log");
  let meta = error.metadata().unwrap();
  assert_eq!(meta.component.unwrap(), "io");
  assert_eq!(meta.context.get("path").unwrap(), "/var/log/app.log");
}

#[test]
fn test_network_error() {
  let error = IoError::network("Connection timed out");
  assert_eq!(error.to_string(), "Network error: Connection timed out");
  assert_eq!(error.error_code(), Code::NetworkError);
  assert_eq!(error.severity(), ErksSeverity::Error);
  assert!(error.is_recoverable());
}

#[test]
fn test_network_with_endpoint() {
  let error =
    IoError::network_with_endpoint("Host not found", "example.com", Some(443));
  let meta = error.metadata().unwrap();
  assert_eq!(meta.context.get("host").unwrap(), "example.com");
  assert_eq!(meta.context.get("port").unwrap(), "443");
}

#[test]
fn test_permission_error() {
  let error = IoError::permission("Read access denied");
  assert_eq!(error.to_string(), "Permission denied: Read access denied");
  assert_eq!(error.error_code(), Code::PermissionDenied);
  assert_eq!(error.severity(), ErksSeverity::Critical);
  assert!(!error.is_recoverable());
}

#[test]
fn test_permission_denied_with_context() {
  let error =
    IoError::permission_denied("Cannot execute", "/usr/bin/top", "execute");
  let meta = error.metadata().unwrap();
  assert_eq!(meta.context.get("resource").unwrap(), "/usr/bin/top");
  assert_eq!(meta.context.get("required_permission").unwrap(), "execute");
}

#[test]
fn test_not_found_error() {
  let error = IoError::not_found("User profile not found");
  assert_eq!(
    error.to_string(),
    "Resource not found: User profile not found"
  );
  assert_eq!(error.error_code(), Code::NotFound);
  assert_eq!(error.severity(), ErksSeverity::Warning);
  assert!(error.is_recoverable());
}

#[test]
fn test_not_found_with_paths() {
  let paths = vec!["/etc/app.conf".to_string(), "./app.conf".to_string()];
  let error = IoError::not_found_with_paths("Config file", "config", paths);
  let meta = error.metadata().unwrap();
  assert_eq!(meta.context.get("resource_type").unwrap(), "config");
  assert_eq!(meta.context.get("searched_paths_count").unwrap(), "2");
  assert_eq!(
    meta.context.get("searched_path_0").unwrap(),
    "/etc/app.conf"
  );
}

#[test]
fn test_generic_io_error() {
  let std_io_error =
    std::io::Error::new(std::io::ErrorKind::Interrupted, "syscall interrupted");
  let error = IoError::from(std_io_error);
  assert_eq!(error.to_string(), "I/O error: syscall interrupted");
  assert_eq!(error.error_code(), Code::IoError);
}
