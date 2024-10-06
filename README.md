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
├─ .gitignore
├─ Cargo.lock
├─ Cargo.toml             # Workspace manifest
├─ configuration          # Crate for managing configurations
│  ├─ config.toml         # Example configuration file
│  ├─ src
│  │  ├─ config           # Configuration-related logic
│  │  └─ project          # Project-specific utilities
├─ documentation          # Documentation for API, code, and utilities
│  ├─ api                 # API references
│  └─ utilities           # Documentation for each utility
├─ LICENSE-APACHE
├─ LICENSE-MIT
├─ README.md              # The document you're reading
└─ utilities              # Individual utilities as separate crates
```

### Utilities

- [ ] **[embellish](documentation/utilities/embellish.md)**: Formatting text in various ways.
- [ ] **[geet](utilities/geet)**: Simplifies common Git operations to enhance workflow.
- [x] **[intime](./utilities/intime)**: Displays time in natural language.
- [ ] **[pathof](utilities/pathof)**: Resolves and normalizes file paths, handling symbolic links, redundant paths, and more.
- [ ] **[slink](utilities/slink)**: Simplifies common tasks related to symlink creation.
- [ ] **[sysfo](utilities/sysfo)**: Displays system information, such as hardware, software, username, hostname, battery status, and more.

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
