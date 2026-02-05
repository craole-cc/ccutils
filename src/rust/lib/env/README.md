# prjenv

[![Crates.io](https://img.shields.io/crates/v/prjenv.svg)](https://crates.io/crates/prjenv)
[![Documentation](https://docs.rs/prjenv/badge.svg)](https://docs.rs/prjenv)
[![License](https://img.shields.io/crates/l/prjenv.svg)](#license)
[![Build Status](https://img.shields.io/github/actions/workflow/status/craole-cc/ccutils/ci.yml?branch=main)](https://github.com/craole-cc/ccutils/actions)

> **Environment detection and configuration management for Cargo workspaces and
> packages.**

`prjenv` provides a unified interface for working with Cargo project
environments:

- 🔍 Auto-detecting workspace vs standalone vs library environments
- 📦 Reading package and workspace metadata from `Cargo.toml`
- ⚙️ Managing runtime configuration via environment variables
- 📂 Discovering filesystem paths (project root, assets, database)
- 🏗️ Scaffolding new packages with templates
- 🔧 Macro-based initialization (optional `macros` feature)

## Quick Start

```rust
use prjenv::prelude::*;

fn main() {
  let env = get();

  println!("Running: {}", env.summary());
  println!("Server: {}:{}", env.config.ip, env.config.port);
}
```

## Installation

```bash
# Minimal installation (no optional features)
cargo add prjenv

# With specific features
cargo add prjenv --features macros
cargo add prjenv --features "macros,tracing"

# All features
cargo add prjenv --features full
```

Or add to `Cargo.toml`:

```toml
[dependencies]
prjenv = "0.1"

# With features
prjenv = { version = "0.1", features = ["macros"] }
```

## Features

| Feature   | Description                                 | Default |
| --------- | ------------------------------------------- | ------- |
| `full`    | Enables all features (`tracing` + `macros`) | ❌      |
| `tracing` | Adds instrumentation for debugging          | ❌      |
| `macros`  | Provides `setenv!()` and `getenv!()` macros | ❌      |

All features are **disabled by default** to minimize dependencies.

## Usage

### Basic Environment Detection

```rust
use prjenv::prelude::*;

fn main() {
  let env = get();

  match env.kind {
    Kind::Workspace => {
      println!("Workspace: {}", env.workspace.metadata.display_name());
      println!("Packages: {}", env.workspace.package_count());
      println!("Running: {}", env.package.metadata.display_name());
    }
    Kind::Standalone => {
      println!("Standalone: {}", env.package.metadata.display_name());
    }
    Kind::Library => {
      println!("Library mode");
    }
  }
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
  let env = get();
  println!("Server: {}:{}", env.config.ip, env.config.port);
}
```

### With Macros (requires `macros` feature)

```rust
use prjenv::prelude::*;

fn main() {
  // Initialize from CARGO_PKG_* environment variables
  setenv!();

  // Access via convenience macros
  let name = getenv!(pkg_name);
  let version = getenv!(pkg_version);
  let port = getenv!(port);

  println!("Starting {} v{} on port {}", name, version, port);
}
```

## Environment Variables

`prjenv` reads these environment variables with sensible defaults:

| Variable       | Type   | Default                 | Description                          |
| -------------- | ------ | ----------------------- | ------------------------------------ |
| `DATABASE_URL` | String | `{workspace}/assets/db` | Database connection URL or file path |
| `IP`           | String | `localhost`             | Server bind address                  |
| `PORT`         | u16    | `3000`                  | Server bind port                     |
| `RUST_LOG`     | String | _(empty)_               | Tracing filter directives            |

### Configuration Precedence

1. Explicitly set values (via builder)
2. Environment variables
3. Built-in defaults

```rust
use prjenv::prelude::*;

// Override defaults programmatically
let config = Configuration::new()
  .with_db("postgres://localhost/mydb")
  .with_ip("0.0.0.0")
  .with_port(8080)
  .with_rust_log("debug");
```

Or via environment:

```bash
DATABASE_URL=postgres://localhost/mydb \
IP=0.0.0.0 \
PORT=8080 \
RUST_LOG=debug \
cargo run
```

## Examples

### Package Scaffolding

```rust
use prjenv::prelude::*;

let scaffold = PackageScaffold::new("my-service")
  .version("1.0.0")
  .description("My microservice")
  .author("Development Team")
  .dependency("tokio", "1.0")
  .dependency("axum", "0.7")
  .binary();

let package_path = scaffold.create("packages")?;
println!("Created: {}", package_path.display());
# Ok::<(), std::io::Error>(())
```

This creates:

```sh
packages/my-service/
├── Cargo.toml
└── src/
    └── main.rs
```

### Workspace Management

```rust
use prjenv::prelude::*;

let workspace = Workspace::new()
  .with_name("my-workspace")
  .with_version("1.0.0")
  .with_package_name("api")
  .with_package_name("cli")
  .with_package_name("web");

println!("Workspace: {}", workspace.metadata.display_name());
println!("Packages: {}", workspace.package_count());

if let Some(api) = workspace.find_package("api") {
  println!("Found: {}", api.metadata.display_name());
}
```

### Accessing Filesystem Paths

```rust
use prjenv::prelude::*;

let env = get();
let paths = &env.paths;

println!("Project root: {}", paths.project.display());
println!("Assets dir:   {}", paths.assets.display());
println!("Database dir: {}", paths.database.display());

// Use in your application
let config_path = paths.assets.join("config.toml");
let db_path = paths.database.join("app.db");
```

### Complete Application Setup

```rust
use prjenv::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Initialize with compile-time metadata
  #[cfg(feature = "macros")]
  setenv!();

  #[cfg(not(feature = "macros"))]
  set(Environment::new()
    .with_pkg_name(env!("CARGO_PKG_NAME"))
    .with_pkg_version(env!("CARGO_PKG_VERSION")));

  let env = get();

  eprintln!("🚀 Starting: {}", env.summary());
  eprintln!("📂 Root: {}", env.paths.project.display());
  eprintln!("⚙️  Server: {}:{}", env.config.ip, env.config.port);
  eprintln!("💾 Database: {}", env.config.db);

  // Your application logic here

  Ok(())
}
```

## Running the Examples

The crate includes several examples demonstrating different features:

```bash
# Basic usage (no features required)
cargo run --example basic

# With macros
cargo run --example macros --features macros

# With tracing instrumentation
RUST_LOG=trace cargo run --example tracing --features tracing

# Advanced usage (all features)
cargo run --example advanced --features full
```

## Architecture

`prjenv` is organized into focused modules:

- **`core`** - Environment detection and global state management
- **`metadata`** - Package and workspace metadata (name, version, description)
- **`infrastructure`** - Runtime configuration and filesystem paths
- **`workspace`** - Workspace domain model and management
- **`package`** - Package domain model and scaffolding
- **`macros`** - Optional convenience macros (requires `macros` feature)

This separation ensures clear boundaries and makes testing easier.

## Performance Characteristics

- **First `get()` call**: 5-50ms (workspace discovery + file I/O)
- **Subsequent calls**: <1µs (static `OnceLock` cache)
- **Metadata loading**: 5-15ms (TOML parsing, cached)
- **Path discovery**: 1-2ms (directory traversal, cached)

All expensive operations are cached in static storage for zero-cost subsequent
access.

## Thread Safety

All public APIs are thread-safe:

- `OnceLock` ensures one-time initialization
- Multiple threads calling `get()` or `set()` coordinate safely
- First caller wins (idempotent behavior)
- No locks after initialization (zero-cost access)

```rust
use std::thread;
use prjenv::prelude::*;

// Safe to call from multiple threads
let handles: Vec<_> = (0..10)
  .map(|_| thread::spawn(|| {
    let env = get(); // All threads get the same cached instance
    env.package.metadata.name.clone()
  }))
  .collect();

for handle in handles {
  println!("{}", handle.join().unwrap());
}
```

## Testing

Override environment detection for tests:

```rust
#[cfg(test)]
mod tests {
  use prjenv::prelude::*;

  #[test]
  fn test_custom_environment() {
    let env = Environment::library()
      .with_name("test-package")
      .with_version("0.0.0")
      .with_db("sqlite::memory:");

    let env = set(env);
    assert_eq!(env.package.metadata.name, "test-package");
    assert_eq!(env.config.db, "sqlite::memory:");
  }
}
```

Or use environment variables:

```bash
# Override project root for testing
PROJECT_ROOT=/tmp/test cargo test

# Override configuration
DATABASE_URL=sqlite::memory: cargo test
```

## Troubleshooting

### "PORT must be a valid number" panic

The `PORT` environment variable must be a valid `u16` (0-65535):

```bash
# ❌ Will panic
PORT=invalid cargo run

# ✅ Correct
PORT=8080 cargo run
```

### Workspace not detected

Ensure your workspace `Cargo.toml` has:

```toml
[workspace]
members = ["packages/*"]
resolver = "2"
```

Or override detection:

```bash
WORKSPACE_ROOT=/path/to/workspace cargo run
```

### Database path not found

Create the assets directory:

```bash
mkdir -p assets/db
```

Or set explicitly:

```bash
DATABASE_URL=sqlite:///tmp/app.db cargo run
```

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing`)
3. Add tests for new functionality
4. Ensure tests pass (`cargo test --all-features`)
5. Run formatting (`cargo fmt`)
6. Run clippy (`cargo clippy --all-features -- -D warnings`)
7. Submit a pull request

See [CONTRIBUTING.md](../../../../CONTRIBUTING.md) for detailed guidelines.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../../../LICENSE-APACHE))
- MIT License ([LICENSE-MIT](../../../../LICENSE-MIT))

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Acknowledgments

Built with:

- [`toml`](https://crates.io/crates/toml) - TOML parsing
- [`dotenvy`](https://crates.io/crates/dotenvy) - `.env` file support
- [`tracing`](https://crates.io/crates/tracing) - Optional instrumentation (with
  `tracing` feature)

---

**Part of the [ccutils](https://github.com/craole-cc/ccutils) suite • Made with
❤️ by [Craole](https://github.com/craole-cc)**
