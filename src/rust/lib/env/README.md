# prjenv

> Cargo workspace/package environment detection and configuration management

[![Crates.io](https://img.shields.io/crates/v/prjenv.svg)](https://crates.io/crates/prjenv)
[![Documentation](https://docs.rs/prjenv/badge.svg)](https://docs.rs/prjenv)
[![License](https://img.shields.io/crates/l/prjenv.svg)](../../../../README.md)

`prjenv` provides a unified API for accessing workspace and package metadata,
runtime configuration, and filesystem paths in Rust projects. It automatically
detects your environment (workspace/standalone/library) and provides sensible
defaults with zero configuration.

## Features

- 🚀 **Zero-config** - Works out of the box with sensible defaults
- 🏗️ **Environment Detection** - Automatically detects workspace, standalone, or
  library mode
- 📦 **Metadata Access** - Easy access to package and workspace metadata
- ⚙️ **Configuration Management** - Centralized runtime configuration from
  environment variables
- 📂 **Path Discovery** - Automatic workspace root and standard directory
  discovery
- 🔧 **Builder Pattern** - Fluent API for custom configurations
- 🧵 **Thread-Safe** - Static initialization with `OnceLock` for zero-cost
  access
- 📝 **Optional Macros** - Convenient macros for common operations (with
  `macros` feature)
- 🔍 **Tracing Support** - Built-in instrumentation (with `tracing` feature)

## Installation

Add `prjenv` to your project either by:

1. Using the command - `cargo add`

   ```sh
   # Default features
   cargo add prjenv

   # Single feature
   cargo add prjenv --features "macros"

   # Multiple features
   cargo add prjenv --features "macros tracing"

   # All features
   cargo add prjenv --features "full"
   ```

2. Editing the `Cargo.toml` file manually

   ```toml
   [dependencies]
   prjenv = "0.1.0"

   # Single feature
   prjenv = { version = "0.1.0", features = ["macros"] }

   # Multiple features
   prjenv = { version = "0.1.0", features = ["macros", "tracing"] }

   # All features
   prjenv = { version = "0.1.0", features = ["full"] }
   ```

## Usage

### Basic Usage

```rust
use prjenv::prelude::*;

fn main() {
  let env = get();

  println!("Workspace: {}", env.workspace.metadata.name);
  println!("Package: {}", env.package.metadata.name);
  println!("Database: {}", env.config.db);
  println!("Server: {}:{}", env.config.ip, env.config.port);
}
```

### With Macros

Enable the `macros` feature:

```toml
[dependencies]
prjenv = { version = "0.1", features = ["macros"] }
```

```rust
use prjenv::prelude::*;

fn main() {
  // Initialize from compile-time environment variables
  setenv!();

  // Access configuration
  let name = getenv!(pkg_name);
  let version = getenv!(pkg_version);
  let port = getenv!(port);

  println!("{} v{} running on port {}", name, version, port);
}
```

### Custom Configuration

```rust
use prjenv::prelude::*;

fn main() {
  let env = Environment::workspace()
    .with_pkg_name(env!("CARGO_PKG_NAME"))
    .with_pkg_version(env!("CARGO_PKG_VERSION"))
    .with_db("postgres://localhost/mydb")
    .with_port(8080)
    .with_ip("0.0.0.0");

  set(env);

  // Now accessible globally
  let cfg = get();
  println!("Server: {}:{}", cfg.config.ip, cfg.config.port);
}
```

## Environment Variables

`prjenv` reads the following environment variables with sensible defaults:

| Variable       | Default                 | Description                          |
| -------------- | ----------------------- | ------------------------------------ |
| `DATABASE_URL` | `{workspace}/assets/db` | Database connection URL or file path |
| `IP`           | `localhost`             | Server bind address                  |
| `PORT`         | `3000`                  | Server bind port                     |
| `RUST_LOG`     | (empty)                 | Tracing filter directives            |

## Features

### Default Features

None - minimal footprint by default.

### Optional Features

- **`macros`** - Enables `setenv!()` and `getenv!()` convenience macros
- **`tracing`** - Adds instrumentation for debugging and monitoring
- **`full`** - Enables all features: `macros` + `tracing`

```toml
[dependencies]
prjenv = { version = "0.1", features = ["full"] }
```

## Examples

### Auto-detection

```rust
use prjenv::prelude::*;

let env = get();

match env.kind {
  Kind::Workspace => println!("Running in workspace with {} packages",
    env.workspace.package_count()),
  Kind::Standalone => println!("Standalone package: {}",
    env.package.metadata.name),
  Kind::Library => println!("Library mode - minimal initialization"),
}
```

### Package Scaffolding

```rust
use prjenv::prelude::*;

let package = PackageScaffold::new("my-service")
  .version("1.0.0")
  .description("My microservice")
  .author("Development Team")
  .dependency("tokio", "1.0")
  .binary()
  .create("packages")?;

println!("Created package at: {}", package.display());
```

### Workspace Management

```rust
use prjenv::prelude::*;

let workspace = Workspace::new()
  .with_name("my-workspace")
  .with_package_name("api")
  .with_package_name("cli")
  .with_package_name("web");

println!("Workspace has {} packages", workspace.package_count());

if let Some(api) = workspace.find_package("api") {
  println!("Found API package: {}", api.metadata.display_name());
}
```

## Architecture

`prjenv` separates concerns into distinct layers:

- **Core** - Environment types and global state management
- **Metadata** - Package/workspace metadata (name, version, description)
- **Infrastructure** - Configuration (env vars) and paths (filesystem)
- **Domain** - Workspace and package models
- **Macros** - Optional convenience macros

## Performance

- **Initialization**: ~5-50ms on first call (workspace discovery + file I/O)
- **Subsequent access**: <1µs (static `OnceLock` cache)
- **Metadata loading**: ~5-15ms (file read + TOML parse, cached)
- **Path discovery**: ~1-2ms typical (directory walking, cached)

## Thread Safety

All public APIs are thread-safe:

- Uses `OnceLock` for one-time initialization
- Multiple threads can safely call `get()` or `set()` concurrently
- First caller wins (idempotent behavior)

## Examples

Run the included examples:

```bash
# Basic usage
cargo run --example basic

# With macros
cargo run --example macros --features macros

# With tracing
RUST_LOG=trace cargo run --example tracing --features tracing

# Advanced (all features)
cargo run --example advanced --features full
```

## Documentation

Full API documentation is available at [docs.rs/prjenv](https://docs.rs/prjenv).

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../../../LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../../../../LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless explicitly stated otherwise, any contribution intentionally submitted for
inclusion in the work, as defined in the Apache-2.0 license, shall be dual
licensed as above, without any additional terms or conditions.

Contributions are welcome! Please feel free to submit a Pull Request.

## Similar Projects

- [`cargo_metadata`](https://crates.io/crates/cargo_metadata) - Low-level cargo
  metadata queries
- [`project-root`](https://crates.io/crates/project-root) - Simple project root
  detection

`prjenv` provides a higher-level, more ergonomic API with additional features
like configuration management and package scaffolding.
