# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2025-02-XX

### Changed

- Improved README documentation with better examples and structure
- Enhanced code examples with proper error handling patterns
- Clarified feature documentation and configuration precedence
- Updated documentation paths and internal links

### Fixed

- Corrected relative paths in documentation
- Fixed license badge linking to proper section
- Improved code formatting consistency (2-space indentation)

## [0.1.0] - 2025-02-04

### Added

- Initial release of `prjenv`
- Environment detection (Workspace/Standalone/Library modes)
- Workspace and package metadata access
- Configuration management from environment variables
- Automatic workspace root discovery with multiple fallback strategies
- Package scaffolding utilities
- Workspace management operations
- Thread-safe static initialization with `OnceLock`
- Optional `macros` feature for `setenv!()` and `getenv!()` convenience macros
- Optional `tracing` feature for instrumentation
- Comprehensive documentation and examples
- Examples: `basic`, `macros`, `tracing`, `advanced`

### Features

- Zero-configuration defaults for rapid development
- Builder pattern for custom configurations
- Cached metadata loading for performance
- Path discovery with sensible defaults
- Support for database URL, server IP/port configuration
- TOML parsing utilities for Cargo.toml manipulation

### Documentation

- Comprehensive API documentation
- Four example programs demonstrating different use cases
- README with quick start guide
- In-code documentation with examples

[Unreleased]: https://github.com/craole-cc/ccutils/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/craole-cc/ccutils/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/craole-cc/ccutils/releases/tag/v0.1.0
