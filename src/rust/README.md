# Rust Workspace

A Rust workspace with multiple crates.

## Structure

```sh
.
├── Cargo.toml          # Workspace configuration
├── crates/             # Workspace members
│   ├── crate-one/
│   └── crate-two/
└── ...
```

## Quick Start

```sh
# Create a new crate
mise run new-crate my-crate

# Run tests for all crates
cargo test --workspace

# Run a specific crate
cargo run -p crate-name
```

## Development

- **Watch mode**: `mise run dev`
- **Test all**: `mise run test`
- **Format**: `mise run fmt`
- **Check**: `mise run check`

## License

MIT OR Apache-2.0
