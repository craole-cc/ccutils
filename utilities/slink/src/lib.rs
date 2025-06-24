pub mod config;
pub mod error;
pub mod symlink;

pub use config::Config;
pub use error::SymlinkError;
pub use symlink::process_links;

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs::{self, File};
  use std::io::Write;
  use tempfile::tempdir;

  fn setup_test_env() -> (tempfile::TempDir, Config) {
    let temp_dir = tempdir().unwrap();
    let src_dir = temp_dir.path().join("src");
    let link_dir = temp_dir.path().join("link");
    fs::create_dir_all(&src_dir).unwrap();
    fs::create_dir_all(&link_dir).unwrap();

    let config = Config::new(false, false, vec![], link_dir);

    (temp_dir, config)
  }

  #[test]
  fn test_single_file_symlink() -> Result<(), Box<dyn std::error::Error>> {
    let (temp_dir, mut config) = setup_test_env();
    let src_file = temp_dir.path().join("src/test_file");
    File::create(&src_file)?.write_all(b"test content")?;

    config.sources = vec![src_file.clone()];
    process_links(&config)?;

    let link_path = config.link_base.join("test_file");
    assert!(link_path.exists());
    assert!(link_path.is_symlink());
    assert_eq!(fs::read_to_string(link_path)?, "test content");

    Ok(())
  }

  // ... [Include all other tests from the previous version]
}
