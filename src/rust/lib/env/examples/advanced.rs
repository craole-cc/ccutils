//! Advanced usage example demonstrating all features
//!
//! Run with: cargo run --example advanced --features full

#![cfg(feature = "full")]

use prjenv::prelude::*;

fn main() {
  println!("🚀 prjenv - Advanced Example (with all features)\n");

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 1: Macro-Based Initialization                     ║
  //╚═══════════════════════════════════════════════════════════╝
  #[cfg(feature = "macros")]
  {
    println!("📦 Macro-Based Initialization:");
    setenv!();

    let pkg_name = getenv!(pkg_name);
    let pkg_version = getenv!(pkg_version);
    let port = getenv!(port);

    println!("  Package: {} v{}", pkg_name, pkg_version);
    println!("  Port: {}", port);
    println!();
  }

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 2: Full Environment Inspection                    ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("🔍 Full Environment Inspection:");

  let env = get();
  println!("  Kind: {:?}", env.kind);
  println!("  Summary: {}", env.summary());
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 3: Workspace Management                           ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("📚 Workspace with Multiple Packages:");

  let workspace = Workspace::new()
    .with_name("advanced-workspace")
    .with_version("2.0.0")
    .with_description("Demonstration workspace with multiple packages")
    .with_package(
      Package::new()
        .with_name("api")
        .with_version("1.0.0")
        .with_description("REST API service"),
    )
    .with_package(
      Package::new()
        .with_name("cli")
        .with_version("1.0.0")
        .with_description("Command-line interface"),
    )
    .with_package(
      Package::new()
        .with_name("web")
        .with_version("1.0.0")
        .with_description("Web frontend"),
    );

  println!("  Workspace: {}", workspace.metadata.display_name());
  println!("  Description: {}", workspace.metadata.description);
  println!("  Package count: {}", workspace.package_count());
  println!("  Packages:");

  for pkg in workspace.packages() {
    println!(
      "    - {} v{}: {}",
      pkg.name(),
      pkg.version(),
      pkg.description()
    );
  }

  if let Some(api) = workspace.find_package("api") {
    println!(
      "\n  ✅ Found 'api' package: {}",
      api.metadata.display_name()
    );
  }

  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 4: Complex Configuration                          ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("⚙️  Complex Configuration:");

  let config = Configuration::new()
    .with_db("postgresql://localhost:5432/production_db")
    .with_ip("0.0.0.0")
    .with_port(443_u16)
    .with_rust_log("api=debug,web=info,cli=trace");

  println!("  Database: {}", config.db);
  println!("  Server: {}:{}", config.ip, config.port);
  println!("  Logging: {}", config.rust_log);
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 5: Environment Kinds                              ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("🏗️  Different Environment Kinds:");

  let kinds = [Kind::Workspace, Kind::Standalone, Kind::Library];

  for kind in &kinds {
    println!("  {} mode:", kind.as_str());
    println!(
      "    - Can access filesystem: {}",
      kind.can_access_filesystem()
    );
    println!(
      "    - Should discover workspace: {}",
      kind.should_discover_workspace()
    );
  }

  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 6: Package Scaffolding                            ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("🏗️  Package Scaffolding:");

  let microservice = PackageScaffold::new("user-service")
    .version("1.0.0")
    .description("Microservice for user management")
    .author("Development Team")
    .dependency("tokio", "1.0")
    .dependency("serde", "1.0")
    .binary();

  println!("  Name: {}", microservice.name);
  println!("  Version: {}", microservice.version);
  println!(
    "  Type: {}",
    if microservice.is_binary {
      "Binary"
    } else {
      "Library"
    }
  );
  println!("  Dependencies: {:?}", microservice.dependencies);
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 7: Full Environment Builder                       ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("🎯 Complete Environment Configuration:");

  let production_env = Environment::workspace()
    .with_workspace_name("production-cluster")
    .with_workspace_version("3.0.0")
    .with_workspace_description("Production deployment environment")
    .with_workspace_package(
      Package::new()
        .with_name("load-balancer")
        .with_version("1.5.0"),
    )
    .with_workspace_package(
      Package::new()
        .with_name("api-gateway")
        .with_version("2.1.0"),
    )
    .with_pkg_name("api-gateway")
    .with_pkg_version("2.1.0")
    .with_pkg_description("Main API gateway service")
    .with_db("postgresql://prod-db.example.com:5432/api")
    .with_ip("0.0.0.0")
    .with_port(443_u16);

  println!("  {}", production_env.summary());
  println!(
    "  Server: {}:{}",
    production_env.config.ip, production_env.config.port
  );
  println!("  Database: {}", production_env.config.db);
  println!(
    "  Workspace packages: {}",
    production_env.workspace.package_count()
  );
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 8: Metadata Operations                            ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("📝 Metadata Operations:");

  let metadata = Metadata::from_parts("my-service", "1.2.3", "A microservice example");

  println!("  Display name: {}", metadata.display_name());
  println!("  Has name: {}", metadata.has_name());
  println!("  Is empty: {}", metadata.is_empty());
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 9: Path Management                                ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("📂 Path Management:");

  let paths = Paths::default();
  println!("  Project root: {}", paths.project.display());
  println!("  Package path: {}", paths.package.display());
  println!("  Assets: {}", paths.assets.display());
  println!("  Database: {}", paths.database.display());
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 10: Environment Detection                         ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("🔎 Environment Detection:");

  let detected_kind = Kind::detect();
  println!("  Detected kind: {}", detected_kind);
  println!("  From string parsing:");

  let test_strings = ["workspace", "standalone", "library", "invalid"];
  for s in &test_strings {
    match Kind::parse(s) {
      Some(k) => println!("    '{}' → {}", s, k),
      None => println!("    '{}' → Invalid", s),
    }
  }

  println!();
  println!("✅ Advanced example completed successfully!");
  println!("\n💡 This example demonstrates:");
  println!("   • Workspace and package management");
  println!("   • Complex configuration scenarios");
  println!("   • Environment kind detection");
  println!("   • Package scaffolding");
  println!("   • Metadata manipulation");
  println!("   • Path management");
}
