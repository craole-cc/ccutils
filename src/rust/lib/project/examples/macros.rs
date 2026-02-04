//! Macro usage example for prjenv
//!
//! Demonstrates the `setenv!` and `getenv!` macros when the `macros` feature is enabled.
//!
//! Run with: cargo run --example with_macros --features macros

#![cfg(feature = "macros")]

use prjenv::prelude::*;

fn main() {
  println!("🚀 Craole CC Project - Macro Example\n");

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 1: Auto-Initialize with setenv!()                 ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("📦 Auto-Initialization:");

  // Initialize environment using compile-time metadata
  setenv!();

  println!("  ✅ Environment initialized from CARGO_PKG_* variables");
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 2: Access Package Metadata                        ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("📝 Package Metadata Access:");

  let name = getenv!(pkg_name);
  let version = getenv!(pkg_version);
  let desc = getenv!(pkg_desc);

  println!("  Package: {}", name);
  println!("  Version: {}", version);
  println!("  Description: {}", desc);
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 3: Access Configuration                           ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("⚙️  Runtime Configuration:");

  let db = getenv!(db);
  let ip = getenv!(ip);
  let port = getenv!(port);

  println!("  Database: {}", db);
  println!("  Server: {}:{}", ip, port);
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 4: Access Paths                                   ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("📂 Filesystem Paths:");

  let project_root = getenv!(prj_path);
  let assets = getenv!(assets_path);
  let database = getenv!(db_path);

  println!("  Project: {}", project_root.display());
  println!("  Assets: {}", assets.display());
  println!("  Database: {}", database.display());
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 5: Access Whole Environment                       ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("🌍 Full Environment:");

  let env = getenv!();
  println!("  {}", env.summary());
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 6: Custom Initialization                          ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("🔧 Custom Initialization (note: setenv! is idempotent):");

  // This won't actually replace the environment since we already called setenv!()
  // But it demonstrates the syntax
  let custom_env = Environment::new()
    .with_pkg_name("custom-app")
    .with_pkg_version("2.0.0")
    .with_port(8080);

  // This returns the already-initialized environment
  let _ = setenv!(custom_env);

  // So these still show the original values
  println!("  Name (still original): {}", getenv!(pkg_name));
  println!("  Port (still original): {}", getenv!(port));
  println!("  (setenv! only sets once - first call wins)");
  println!();

  println!("✅ Macro example completed!");
  println!("\n💡 Tip: The macros are just convenient shortcuts.");
  println!("   You can also use get() and set() directly:");
  println!("   let env = prjenv::get();");
  println!("   println!(\"Port: {{}}\", env.config.port);");
}
