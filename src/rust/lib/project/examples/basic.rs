//! Basic usage - no extra dependencies needed
//!
//! cargo run --example basic

use craole_cc_project::prelude::*;
use std::path::Path;

fn main() {
    println!("=== craole-cc-project Basic Test ===");

    // Find workspace root
    let root = find_cargo_root();
    println!("📁 Workspace root: {}", root.display());

    // Check if it's a workspace
    let cargo_toml = root.join("Cargo.toml");
    println!("🏠 Is workspace: {}", is_workspace_toml(&cargo_toml));

    // Scaffold library package
    let lib_pkg = PackageBuilder::new("example-lib")
        .version("0.1.0")
        .description("Example library package")
        .scaffold(".");
    println!("📚 Library: {:?}", lib_pkg);

    // Scaffold binary package
    let bin_pkg = PackageBuilder::new("example-bin")
        .version("0.1.0")
        .description("Example binary package")
        .binary()
        .scaffold(".");
    println!("⚙️  Binary: {:?}", bin_pkg);

    println!("\n✅ SUCCESS! Check ./example-lib/ and ./example-bin/");
}
