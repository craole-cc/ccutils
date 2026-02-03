# craole-cc-project

**Part of the [Craole CC Dev Toolkit](https://github.com/craole-cc/devtools)** - Development utilities by Craig "Craole" Cole.

Cargo workspace and package scaffolding and management toolkit.

## Features

- 🔍 Read workspace and package metadata
- 🏗️ Scaffold new packages with templates
- 📦 Manage workspace members (add/remove)
- ✏️ Manipulate Cargo.toml programmatically

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

## Other Crates in Craole CC Toolkit

- [`craole-cc-cache`](https://crates.io/crates/craole-cc-cache) - Caching utilities *(coming soon)*
- [`craole-cc-env`](https://crates.io/crates/craole-cc-env) - Environment management *(coming soon)*
- [`craole-cc-cli`](https://crates.io/crates/craole-cc-cli) - CLI framework *(coming soon)*

See all at [github.com/craole-cc](https://github.com/craole-cc)

## License

MIT OR Apache-2.0
