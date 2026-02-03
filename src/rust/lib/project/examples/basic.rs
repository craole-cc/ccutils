//! Basic usage example for craole-cc-project

use {
  craole_cc_project::prelude::*,
  std::path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("🚀 Craole CC Project - Basic Example\n");

  // ═══════════════════════════════════════════════════════════
  // Example 1: Get environment information
  // ═══════════════════════════════════════════════════════════
  println!("📦 Environment Information:");
  let env = get();

  println!("  Kind: {:?}", env.kind);
  println!("  Workspace: {}", env.workspace.metadata.display_name());
  println!("  Package: {}", env.package.metadata.display_name());
  println!("  Database: {}", env.config.db);
  println!("  Server: {}:{}", env.config.ip, env.config.port);
  println!("  Project root: {}", env.paths.project.display());
  println!();

  // ═══════════════════════════════════════════════════════════
  // Example 2: Custom environment configuration
  // ═══════════════════════════════════════════════════════════
  println!("⚙️  Custom Configuration:");

  let custom_env = Environment::workspace()
    .with_pkg_name("my-custom-app")
    .with_pkg_version("1.0.0")
    .with_db("postgres://localhost/mydb")
    .with_port(8080)
    .with_ip("0.0.0.0");

  println!("  Package: {}", custom_env.package.metadata.display_name());
  println!(
    "  Server: {}:{}",
    custom_env.config.ip, custom_env.config.port
  );
  println!("  Database: {}", custom_env.config.db);
  println!();

  // ═══════════════════════════════════════════════════════════
  // Example 3: Package scaffolding
  // ═══════════════════════════════════════════════════════════
  println!("🏗️  Package Scaffolding:");

  let lib_scaffold = PackageScaffold::new("example-lib")
    .description("An example library package")
    .version("0.1.0")
    .library();

  println!("  Library scaffold created: {}", lib_scaffold.name);

  let bin_scaffold = PackageScaffold::new("example-bin")
    .description("An example binary package")
    .version("0.1.0")
    .binary();

  println!("  Binary scaffold created: {}", bin_scaffold.name);
  println!();

  // ═══════════════════════════════════════════════════════════
  // Example 4: Workspace package management
  // ═══════════════════════════════════════════════════════════
  println!("📚 Workspace Packages:");

  let workspace = Workspace::new()
    .with_name("my-workspace")
    .with_package_name("api")
    .with_package_name("cli")
    .with_package_name("web");

  println!("  Workspace: {}", workspace.metadata.name);
  println!("  Package count: {}", workspace.package_count());
  println!("  Packages: {:?}", workspace.package_names());

  if let Some(api) = workspace.find_package("api") {
    println!("  Found package: {}", api.metadata.name);
  }
  println!();

  // ═══════════════════════════════════════════════════════════
  // Example 5: Metadata operations
  // ═══════════════════════════════════════════════════════════
  println!("📝 Metadata:");

  let metadata = Metadata::new()
    .with_name("example-project")
    .with_version("2.0.0")
    .with_description("A demonstration project");

  println!("  {}", metadata.display_name());
  println!("  Description: {}", metadata.description);
  println!();

  // ═══════════════════════════════════════════════════════════
  // Example 6: Configuration
  // ═══════════════════════════════════════════════════════════
  println!("⚡ Configuration:");

  let config = Configuration::new()
    .with_db("sqlite:///data/app.db")
    .with_port(3000)
    .with_ip("localhost")
    .with_rust_log("debug");

  println!("  Database: {}", config.db);
  println!("  Server: {}:{}", config.ip, config.port);
  println!("  Logging: {}", config.rust_log);
  println!();

  println!("✅ All examples completed successfully!");

  Ok(())
}
