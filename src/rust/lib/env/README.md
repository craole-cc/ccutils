# prjenv

**Part of the [Craole CC Dev Toolkit](https://github.com/craole-cc/devtools)** - Development utilities by Craig "Craole" Cole.

Cargo workspace and package scaffolding and management toolkit.

## Features

- 🔍 **Read** workspace and package metadata
- 🏗️ **Scaffold** new packages with templates
- 📦 **Manage** workspace members (add/remove)
- ✏️ **Manipulate** Cargo.toml files programmatically

## Installation

```toml
[dependencies]
prjenv = "0.1"
```

## Quick Start

```rust
use prjenv::prelude::*;

// Find workspace root
let root = find_cargo_root();

// Scaffold a new package
PackageBuilder::new("my-lib")
    .description("A new library")
    .scaffold("./lib/my-lib")?;

// Add to workspace
let manager = WorkspaceManager::new(&root);
manager.add_member("lib/my-lib")?;
```

## Examples

The `examples/` folder contains working, copy-paste ready examples:

### Basic Usage (No Extra Dependencies)

```bash
cargo run --example basic
```

Creates library and binary packages with complete Cargo.toml files. **No additional dependencies needed** - just works!

**Output:**

```sh
📁 Workspace root: /path/to/workspace
🏠 Is workspace: true
📚 Library: Ok("./example-lib")
⚙️  Binary: Ok("./example-bin")
```

**Files created:**

```md
example-lib/
├── Cargo.toml
└── src/lib.rs

example-bin/
├── Cargo.toml
└── src/main.rs
```

### Full Workspace Management

```bash
cargo run --example workspace
```

Complete workspace operations: scaffold packages and add them to your workspace Cargo.toml.

### Advanced (With Tracing)

```bash
cargo run --example advanced --features=tracing
```

Production-ready example with proper error handling using `miette` and `thiserror`.

**View all examples:** [examples/](https://github.com/craole-cc/devtools/tree/main/src/rust/lib/project/examples)

## Crates in the Craole CC Dev Toolkit

The Craole CC toolkit provides a suite of Rust development utilities designed to work seamlessly together:

### Published Crates

- 🏗️ **[`prjenv`](https://crates.io/crates/prjenv)** - Workspace scaffolding and package management *(you are here)*

### Coming Soon

- 📝 **[`prjenv`](https://crates.io/crates/prjenv)** - Logging with sane defaults
  *Re-exports `tracing` with opinionated configuration, `miette` for beautiful error reporting, and utilities for structured logging*
  **Binary:** `ephelog` - Ephemeral log viewer and analyzer

- 💾 **[`craole-cc-cache`](https://crates.io/crates/craole-cc-cache)** - Intelligent caching utilities
  *Fast, type-safe caching with TTL, LRU, and persistence support*

- 🌍 **[`craole-cc-env`](https://crates.io/crates/craole-cc-env)** - Environment management
  *Runtime configuration, workspace paths, and compile-time metadata handling*

- ⚡ **[`craole-cc-cli`](https://crates.io/crates/craole-cc-cli)** - CLI framework
  *Opinionated CLI builder with clap integration and interactive prompts*

See all projects at [github.com/craole-cc/devtools](https://github.com/craole-cc/devtools)

## Feature Flags

### Default (minimal)

Basic TOML manipulation and metadata reading without external dependencies.

### `full`

Enables `cargo_metadata` for robust workspace detection via cargo's native metadata command (~50-100ms overhead).

```toml
[dependencies]
prjenv = { version = "0.1", features = ["full"] }
```

### `tracing`

Adds instrumented logging for debugging scaffolding operations.

```toml
[dependencies]
prjenv = { version = "0.1", features = ["tracing"] }
prjenv = "0.1"  # Recommended for tracing support
```

## API Examples

### Find Workspace Root

```rust
use prjenv::prelude::*;

let root = find_cargo_root();
println!("Workspace: {}", root.display());
```

### Create and Add a Package

```rust
use prjenv::prelude::*;

// Build a new library crate
let pkg_path = PackageBuilder::new("my-awesome-lib")
    .version("0.1.0")
    .description("My awesome library")
    .author("Your Name <you@example.com>")
    .dependency("serde", "1.0")
    .library()
    .scaffold("./lib")?;

// Add to workspace Cargo.toml
let root = find_cargo_root();
let manager = WorkspaceManager::new(&root);
manager.add_member("lib/my-awesome-lib")?;

println!("Created and registered: {}", pkg_path.display());
```

### Read Workspace Metadata

```rust
use prjenv::prelude::*;

let root = find_cargo_root();
let cargo_toml = root.join("Cargo.toml");

if let Some(metadata) = read_cargo_metadata(&cargo_toml) {
    if let Some(name) = metadata.get("name") {
        println!("Workspace: {}", name);
    }
}
```

## Why Craole CC?

The Craole CC toolkit is designed with these principles:

- 🎯 **Opinionated** - Sensible defaults that just work
- 🔧 **Composable** - Mix and match crates as needed
- 📦 **Zero-config** - Minimal setup, maximum productivity
- 🚀 **Fast** - Performance-conscious implementations
- 🦀 **Idiomatic Rust** - Follows ecosystem best practices

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](../../../../CONTRIBUTING.md) for guidelines.

## License

Licensed under either of:

- Apache License, Version 2.0 [LICENSE-APACHE](../../../../LICENSE-APACHE)
- MIT license [LICENSE-MIT](../../../../LICENSE-MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
