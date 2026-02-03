# craole-cc-project

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
craole-cc-project = "0.1"
```

## Quick Start

```rust
use craole_cc_project::prelude::*;

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

## Crates in the Craole CC Dev Toolkit

The Craole CC toolkit provides a suite of Rust development utilities designed to work seamlessly together:

### Published Crates

- 🏗️ **[`craole-cc-project`](https://crates.io/crates/craole-cc-project)** - Workspace scaffolding and package management *(you are here)*

### Coming Soon

- 📝 **[`craole-cc-log`](https://crates.io/crates/craole-cc-log)** - Logging with sane defaults
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
craole-cc-project = { version = "0.1", features = ["full"] }
```

### `tracing`

Adds instrumented logging for debugging scaffolding operations.

```toml
[dependencies]
craole-cc-project = { version = "0.1", features = ["tracing"] }
craole-cc-log = "0.1"  # Recommended for tracing support
```

## Examples

### Find Workspace Root

```rust
use craole_cc_project::prelude::*;

let root = find_cargo_root();
println!("Workspace: {}", root.display());
```

### Create and Add a Package

```rust
use craole_cc_project::prelude::*;

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
use craole_cc_project::prelude::*;

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

- Apache License, Version 2.0 ([LICENSE-APACHE](../../../../LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](../../../../LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
