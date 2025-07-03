# erks

`erks` is a Rust library designed to simplify error handling by providing a unified error type that consolidates common error kinds from various sources like I/O, configuration, parsing, and HTTP requests. It's built on top of popular crates like `anyhow` and `thiserror`, offering a structured and extensible approach to error management.

The core idea is to wrap different error types into a single, comprehensive `erks::Error` enum. This enum is decorated with a `Context` trait, which provides additional metadata like severity, error codes, and recoverability status, making it easier to log, monitor, and handle errors programmatically.

## Features

- **Unified Error Type**: A single `erks::Error` to handle errors from different modules.
- **Structured Metadata**: Each error comes with severity, an error code, and context.
- **Categorization**: Errors are grouped into categories like `system`, `config`, `http`, `custom`, etc.
- **Feature-gated**: Functionality is modular and can be enabled via Cargo features (e.g., `http`, `config`, `json`).
- **Extensible**: Easily define and integrate your own custom application-specific errors.
- **Helper Utilities**: Includes macros like `error!`, `bail!`, and `ensure!` for ergonomic error creation and propagation, plus a `retry_with_backoff` utility.

## Core Concepts

- **`erks::Error`**: The main enum that wraps all other specific error types.
- **`erks::Context`**: A trait implemented by all error types, providing methods like `severity()`, `error_code()`, `is_recoverable()`, and `metadata()`.
- **`erks::Code`**: An enum of programmatic error codes for reliable error matching.
- **`erks::Severity`**: An enum (`Info`, `Warning`, `Error`, `Critical`) to classify the impact of an error.
- **`erks::Metadata`**: A struct for attaching arbitrary key-value context, component, and operation names to an error.

## Installation

Add `erks` to your `Cargo.toml`:

```toml
[dependencies]
# Enable features as needed.
# Use a version from crates.io or a local path if in a workspace.
erks = { version = "0.1.0", features = ["full"] }
```

### Available Features

- `full`: Enables all features below.
- `config`: For configuration file errors (integrates `config-rs`).
- `http`: For HTTP request errors (integrates `reqwest`).
- `glob`: For glob pattern errors.
- `json`: For JSON parsing errors (integrates `serde_json`).
- `toml`: For TOML parsing errors.
- `retry`: Includes the `retry_with_backoff` async utility.
- `structured-logging`: Enables serialization for structured error logging.

## Usage

### Basic Error Handling

```rust
use erks::{Context, Error, Result, io, utils};
use std::fs;

fn read_file(path: &str) -> Result<String> {
    fs::read_to_string(path).map_err(|e| {
        // Convert the std::io::Error into our custom IO error type,
        // then into the main ErksError.
        io::Error::from(e).into()
    })
}

fn main() {
    match read_file("nonexistent.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(e) => {
            eprintln!("An error occurred!");
            eprintln!("  Message: {}", e);
            eprintln!("  Category: {}", e.category());
            eprintln!("  Severity: {}", e.severity());
            eprintln!("  Code: {}", e.error_code());
            eprintln!("  Recoverable: {}", e.is_recoverable());

            // You can also log the structured version
            utils::log_error(&e);
        }
    }
}
```

### Creating Custom Errors

```rust
use erks::{bail, ensure, CustomError, ErksError, ErksResult};

fn process_user(user_id: u32) -> ErksResult<()> {
    if user_id == 0 {
        // Use the `error!` macro for simple, formatted errors
        return Err(erks::error!("User ID cannot be zero"));
    }

    // Use `bail!` to exit early with an error
    if user_id > 1000 {
        bail!("User ID {} is out of range", user_id);
    }

    // Use `ensure!` to validate a condition
    let is_active = get_user_status(user_id)?;
    ensure!(is_active, "User {} is not active", user_id);

    Ok(())
}

fn get_user_status(user_id: u32) -> ErksResult<bool> {
    // ... database logic that might fail ...
    Ok(true) // simplified
}
```

## Testing

The crate includes a comprehensive test suite. To run the tests, ensure you have the dev dependencies installed and run:

```sh
cargo test --all-features
```

This command will run all unit and integration tests for every module, enabling all available features to ensure complete coverage.

## License

This project is licensed under the terms of the LICENSE file.
