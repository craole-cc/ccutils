# CCutils

This project is a growing collection of CLI-based utilities, inspired by the versatility and power of tools like [coreutils](https://uutils.github.io/coreutils/docs/). The primary objective of this project is to enhance my Rust programming skills while migrating and improving the personal utilities I have developed over the years.

This migration is driven by a desire to increase speed, consistency, and ease of use across my dotfiles and scripts, ensuring better portability. I operate across various environments, including Linux (particularly NixOS), Windows (Git Bash, Powershell and NixWSL), and recently macOS (with Nix). By building these utilities in Rust, I ensure they will run seamlessly across all my machines.

In the future, I expect to package this project so it can be installed as a binary via popular package managers. For now, the way to install will be to clone the repository. If Nix is installed, you can use `nix develop`, which will set up the entire project in a temporary, isolated store. If Nix is not installed, the project can be built with `cargo build --release`, and the utilities will be located in the `target/release` folder.

One of the things I want to do is also automatically add the utilities to the PATH so they are available system-wide, regardless of the terminal or shell being used.

## Why Rust?

After years of scripting in POSIX sh, I found myself wanting a language that could provide better ergonomics, safety, and performance. Rust offers:

- **Memory safety**: No more worrying about subtle memory errors or segmentation faults.
- **Concurrency without fear**: Easily handle tasks that require parallelism or asynchronous operations.
- **Ease of deployment**: Compile to a single binary that runs on multiple platforms without extra dependencies.
- **Better tooling**: The Cargo package manager, along with a thriving ecosystem, simplifies the process of development and managing dependencies.

## Project Structure

This project is organized as a Cargo workspace with individual crates, each representing a different utility. Here's a breakdown of the main components:

```sh
ccutils
├── .cargo
│   └── config.toml
├── .gitattributes
├── .gitignore
├── .rust-analyzer.toml
├── .rustfmt.toml
├── .treefmt.toml
├── Cargo.toml
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── packages
│   ├── ccutils
│   ├── colorscheme
│   ├── dots
│   ├── embellish
│   ├── erks
│   ├── genna
│   ├── grit
│   ├── intime
│   ├── logline
│   ├── numba
│   ├── pankage
│   ├── scraps
│   ├── slink
│   ├── sysfo
│   ├── wallter
│   └── whers
├── flake.nix
├── LICENSE-APACHE-2.0
├── LICENSE-MIT
├── README
├── rust-toolchain.toml
└── SECURITY
```

### Utilities

- [ ] **[ccutils](crates/ccutils)**: Manages the ccutils workspace crates.
- [ ] **[colorscheme](crates/colorscheme)**: Manages the desktop color scheme.
- [ ] **[dots](crates/dots)**: Establishes dotfiles.
- [ ] **[embellish](crates/embellish)**: Formatting text in various ways.
- [ ] **[grit](crates/grit)**: Simplifies common Git operations to enhance workflow.
- [x] **[erks](./crates/erks)**: Simplifies error handling throughout the project.
- [x] **[intime](./crates/intime)**: Displays time in natural language.
- [x] **[logline](./crates/logline)**: Manages tracing/log messages internally.
- [ ] **[numba](crates/numba)**: Utilities for working with numbers.
- [ ] **[pankage](crates/pankage)**: Manages applications using available package manager.
- [ ] **[scraps](crates/scraps)**: Implementation of a web scraper.
- [ ] **[slink](crates/slink)**: Simplifies common tasks related to symlink management.
- [ ] **[sysfo](crates/sysfo)**: Displays system information.
- [ ] **[wallter](crates/wallter)**: Redefined dynamic desktop wallpaper manager.
- [ ] **[whers](crates/whers)**: Locate files or commands available locally.

### Configuration Management

The `configuration` crate handles the configuration logic for the utilities, which includes:

- Loading and parsing TOML-based configuration files.
- Managing project-specific settings for web or other environments.
- Utility functions to support configuration loading across different environments.

### Nix Flake Support

This project is also set up as a **Nix flake**, providing a unique development environment with all required tools and dependencies bundled within. This ensures consistency and ease of development across different systems.

To enter the development environment via Nix:

```bash
nix develop
```

This sets up a reproducible environment containing the Rust toolchain and other tools needed for `ccutils` development.

### Documentation

The `documentation` folder contains resources for understanding the utilities and their APIs. This project also aims to provide clear documentation for each utility to aid in further development and usage.

## Getting Started

To use `ccutils`, you'll need to have Rust or Nix installed. If you haven't already, you can install it by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

Once Rust is set up:

```bash
git clone https://github.com/yourusername/ccutils.git
cd ccutils
cargo build
```

Alternatively, if you're using Nix:

```bash
nix develop
```

## Contributing

This project is still a work in progress. Contributions are welcome as I continue to convert more of my shell scripts to Rust.

## License

This project is dual-licensed under the [Apache License 2.0](./LICENSE-APACHE-2.0) and [MIT License](./LICENSE-MIT).

All contributions submitted for inclusion in this project will be dual-licensed under both licenses mentioned above, unless explicitly specified otherwise. This dual-licensing approach ensures maximum flexibility and compatibility with other open-source projects.
