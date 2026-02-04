//! Basic usage example for prjenv
//!
//! Demonstrates core functionality without macros or optional features.

use prjenv::prelude::*;

fn main() {
  println!("🚀 Craole CC Project - Basic Example\n");

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 1: Auto-detected Environment                      ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("📦 Auto-detected Environment:");
  let env = get();

  println!("  Kind: {:?}", env.kind);

  match env.kind {
    Kind::Workspace => {
      println!("  Workspace: {}", env.workspace.metadata.display_name());
      println!("  Packages: {}", env.workspace.package_count());
      println!("  Current package: {}", env.package.metadata.display_name());
    }
    Kind::Standalone => {
      println!("  Standalone: {}", env.package.metadata.display_name());
      println!("  (no workspace detected)");
    }
    Kind::Library => {
      println!("  Library: {}", env.package.metadata.display_name());
      println!("  (library mode - minimal filesystem access)");
    }
  }

  println!("  Database: {}", env.config.db);
  println!("  Server: {}:{}", env.config.ip, env.config.port);
  println!("  Project root: {}", env.paths.project.display());
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 2: Explicit Custom Environment                    ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("⚙️  Explicit Configuration:");

  let custom = Environment::workspace()
    .with_workspace_name("demo-workspace")
    .with_workspace_version("1.0.0")
    .with_pkg_name("my-custom-app")
    .with_pkg_version("1.0.0")
    .with_db("postgres://localhost/mydb")
    .with_port(8080)
    .with_ip("0.0.0.0");

  println!("  Workspace: {}", custom.workspace.metadata.display_name());
  println!("  Package: {}", custom.package.metadata.display_name());
  println!("  Server: {}:{}", custom.config.ip, custom.config.port);
  println!("  Database: {}", custom.config.db);
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 3: Package Scaffolding                            ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("🏗️  Package Scaffolding:");

  let lib_pkg = PackageScaffold::new("example-lib")
    .description("An example library package")
    .version("0.1.0")
    .library();

  println!("  📚 Library: {}", lib_pkg.name);

  let bin_pkg = PackageScaffold::new("example-bin")
    .description("An example binary package")
    .version("0.1.0")
    .binary();

  println!("  ⚙️  Binary: {}", bin_pkg.name);
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 4: Workspace Management                           ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("📚 Workspace Management:");

  let workspace = Workspace::new()
    .with_name("my-workspace")
    .with_version("1.0.0")
    .with_package_name("api")
    .with_package_name("cli")
    .with_package_name("web");

  println!("  Workspace: {}", workspace.metadata.display_name());
  println!("  Package count: {}", workspace.package_count());
  println!("  Packages: {:?}", workspace.package_names());

  if let Some(api_pkg) = workspace.find_package("api") {
    println!("  ✅ Found 'api': {}", api_pkg.metadata.name);
  } else {
    println!("  ❌ 'api' not found");
  }
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 5: Pure Metadata                                  ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("📝 Pure Metadata:");

  let metadata = Metadata::new()
    .with_name("example-project")
    .with_version("2.0.0")
    .with_description("A demonstration project");

  println!("  Project: {}", metadata.display_name());
  println!("  Description: {}", metadata.description);
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 6: Standalone Configuration                       ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("⚡ Standalone Configuration:");

  let config = Configuration::new()
    .with_db("sqlite:///data/app.db")
    .with_port(3000)
    .with_ip("localhost")
    .with_rust_log("debug");

  println!("  Database: {}", config.db);
  println!("  Server: {}:{}", config.ip, config.port);
  println!("  Logging: {}", config.rust_log);
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 7: Real App Entry Point                           ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("🚀 Real App Entry Point:");

  let app_env = Environment::new()
    .with_pkg_name(env!("CARGO_PKG_NAME"))
    .with_pkg_version(env!("CARGO_PKG_VERSION"))
    .with_pkg_description(env!("CARGO_PKG_DESCRIPTION"))
    .with_port(3000)
    .with_ip("0.0.0.0");

  println!("  App: {}", app_env.package.metadata.display_name());
  println!(
    "  Running at: {}:{}",
    app_env.config.ip, app_env.config.port
  );
  println!();

  println!("✅ All examples completed successfully!");
}
