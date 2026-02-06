# ccutils

> A collection of handy development toolkits across multiple programming
> languages. This suite contains utilities, libraries, and tools to enhance your
> development workflow.

## 📁 Project Structure

```sh
.
├── assets/          # Shared resources
├── src/             # Language-specific toolkits
│   ├── nix/
│   ├── rust/
│   ├── python/
│   ├── shellscript/
│   └── zig/
├── templates/       # Project templates
└── init.sh          # Environment initialization script
```

## 🚀 Quick Start

### Initialize Development Environment

The project includes an `init.sh` script that automatically detects and uses the
best available development environment:

```sh
#? Initialize default environment
./init.sh

#? Initialize for a specific language
./init.sh rust
./init.sh python
./init.sh nix

#? Or use flags
./init.sh --language rust
./init.sh -l python
```

**Environment Detection Priority:**

1. **Nix with Flakes** (preferred) - Full reproducible environment
2. **Nix** (legacy) - Fallback without flakes
3. **mise** - Fallback for non-Nix systems

### Options

```
Usage: init.sh [OPTIONS] [NAME]

Arguments:
  NAME   Language environment to initialize (e.g., 'rust', 'nix')

Options:
  -h, --help           Show help message
  -v, --verbose        Enable verbose output
  -d, --debug          Enable debug mode (very verbose)
  -l, --language NAME  Specify language explicitly
```

## 🛠️ Tools by Language

[![Nix](https://img.shields.io/badge/Nix-flakes-5277C3?logo=nixos)](src/nix/README.md)
[![Rust](https://img.shields.io/badge/Rust-1.87+-ce422b?logo=rust)](src/rust/README.md)
[![Python](https://img.shields.io/badge/Python-3.8+-3776ab?logo=python)](src/python/README.md)
[![Zig](https://img.shields.io/badge/Zig-latest-f7a41d?logo=zig)](src/zig/README.md)

Each language workspace contains its own README with specific tooling details.

## 📋 License

[![MIT License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
[![Apache License 2.0](https://img.shields.io/badge/License-Apache%202.0-orange.svg)](LICENSE-APACHE)

This software is dual-licensed under MIT or Apache 2.0; choose whichever works
best for you.

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for
guidelines on:

- Getting started
- Code style and testing
- Documentation requirements
- Commit message conventions
- Issue reporting

## ✨ Features

- **Multi-language**: Rust, Python, Nix, Shellscript and Zig tools in one
  repository
- **Well-organized**: Clean monorepo structure with independent workspaces
- **Well-tested**: Comprehensive testing across all crates
- **Type-safe**: Strict compiler settings and linting rules
- **Documented**: Inline documentation and README files per tool
