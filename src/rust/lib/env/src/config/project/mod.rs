//! Workspace project configuration and management.
//!
//! This module provides comprehensive workspace-level configuration including:
//! - **Metadata** - Project name, version, description from workspace Cargo.toml
//! - **Paths** - Workspace root and standard directory locations
//! - **Configuration** - Server and runtime settings from environment variables
//! - **Utilities** - Workspace detection and TOML parsing helpers
//!
//! # Relationship to Package Module
//!
//! The global `Environment` struct (in `core.rs`) contains both:
//! - **Project**: Workspace-level configuration (this module)
//! - **Package**: Current crate configuration (in `package` module)
//!
//! In a workspace:
//! - One workspace (handled by `project` module)
//! - Multiple crates/packages (individual crate info in `package` module)
//!
//! # Module Structure
//!
//! ```text
//! project/
//! ├── config.rs      - Configuration from environment variables
//! ├── core.rs        - Main Project Environment struct
//! ├── metadata.rs    - Workspace metadata from Cargo.toml
//! ├── paths.rs       - Workspace paths and directories
//! ├── utils.rs       - Workspace discovery and TOML parsing
//! └── mod.rs         - This file, re-exports and prelude
//! ```
//!
//! # Typical Usage
//!
//! ## Using the Prelude
//! ```no_run
//! use env::project::prelude::*;
//!
//! let project = Project::default();
//! println!("Project: {}", project.metadata.name);
//! println!("Database: {}", project.configuration.db);
//! println!(
//!   "Listening on {}:{}",
//!   project.configuration.ip, project.configuration.port
//! );
//! ```
//!
//! ## Accessing Specific Components
//! ```no_run
//! use env::project::prelude::*;
//!
//! let project = Project::default();
//! let config = &project.configuration;
//! let paths = &project.paths;
//! let metadata = &project.metadata;
//!
//! println!("Config: {:?}", config);
//! println!("Assets at: {}", paths.assets.display());
//! ```
//!
//! ## Builder Pattern
//! ```no_run
//! use env::project::prelude::*;
//!
//! let project = Project::new()
//!   .with_port(8080_u16)
//!   .with_ip("0.0.0.0")
//!   .with_db("postgres://localhost/mydb");
//! ```
//!
//! # Initialization Process
//!
//! When `Project::default()` is called, it:
//! 1. Discovers the workspace root via `find_project_path()` (~5-50ms)
//! 2. Reads and parses `workspace_root/Cargo.toml` for metadata (~5-15ms)
//! 3. Sets up path structure relative to workspace root (instant)
//! 4. Loads environment variables for configuration (~1ms)
//! 5. Falls back `DATABASE_URL` to `{workspace}/assets/db` if not set
//!
//! After initialization, all data is cached and subsequent accesses are instant.
//!
//! # Workspace Discovery Strategy
//!
//! The `project` module locates the workspace root using:
//! 1. **Environment variables** - `PRJ_ROOT`, `PROJECT_ROOT`, `WORKSPACE_ROOT`, `CARGO_WORKSPACE_DIR`
//! 2. **Walk up from `CARGO_MANIFEST_DIR`** - Looks for `[workspace]` section
//! 3. **Walk up from current directory** - With workspace markers
//! 4. **`cargo_metadata` fallback** - Guaranteed correct but slower
//! 5. **Last resort** - Falls back to `CARGO_MANIFEST_DIR` or current directory
//!
//! See `utils::find_project_path()` for detailed strategy documentation.
//!
//! # Common Patterns
//!
//! ## Getting Configuration for Server Startup
//! ```no_run
//! use env::prelude::*;
//!
//! let env = get_env();
//! let config = &env.project.configuration;
//! let address = format!("{}:{}", config.ip, config.port);
//! println!("Starting server on {}", address);
//! ```
//!
//! ## Accessing Assets Directory
//! ```no_run
//! use env::prelude::*;
//!
//! let assets_dir = &get_env().project.paths.assets;
//! let config_file = assets_dir.join("config.toml");
//! ```
//!
//! ## Database Connection Setup
//! ```no_run
//! use env::prelude::*;
//!
//! let db_url = &get_env().project.configuration.db;
//! println!("Connecting to: {}", db_url);
//! ```
//!
//! # Thread Safety
//!
//! All project components are thread-safe and can be cloned and shared safely.
//! The global `Environment` is static and provides `&'static` references.

mod config;
mod core;
mod metadata;
mod paths;
mod utils;

pub use {
  config::*,
  core::*,
  metadata::*,
  paths::*,
  utils::*,
};

/// Public prelude for project module.
///
/// Import with `use env::project::prelude::*;` to get commonly used types:
/// - `Configuration` (aliased as `ProjectConfig`) - Server/runtime settings
/// - `Environment` (aliased as `Project`) - Main project struct
/// - `Metadata` (aliased as `ProjectMetadata`) - Project metadata
/// - `Paths` (aliased as `ProjectPaths`) - Project directory structure
/// - `*` from utils - Workspace detection utilities
///
/// # Examples
/// ```no_run
/// use env::project::prelude::*;
///
/// // Access the project configuration
/// let project = Project::default();
/// let db = project.configuration.db.as_str();
/// let port = project.configuration.port;
/// println!("Database: {}", db);
/// println!("Port: {}", port);
///
/// // Build custom configuration
/// let config = ProjectConfig::new().with_port(8080_u16).with_ip("0.0.0.0");
///
/// // Check workspace root
/// let root = find_project_path();
/// println!("Workspace root: {}", root.display());
///
/// // Access project metadata
/// let metadata = ProjectMetadata::default();
/// println!("Project: {} v{}", metadata.name, metadata.version);
/// ```
///
/// # What's Included
/// - **Types**: Configuration, Environment, Metadata, Paths
/// - **Functions**: `find_project_path()`, `is_workspace_toml()`(), `read_toml_metadata()`
/// - **Type aliases**: `ProjectConfig`, `Project`, `ProjectMetadata`, `ProjectPaths`
pub mod prelude {
  pub use super::{
    config::Configuration as ProjectConfig,
    core::Environment as Project,
    metadata::Metadata as ProjectMetadata,
    paths::Paths as ProjectPaths,
    utils::*,
  };
}
