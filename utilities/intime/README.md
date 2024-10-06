# InTime - Time Duration Formatter

InTime is a versatile utility designed to format time durations into a human-readable format. As part of the **[ccutils](../../)** Rust project, it primarily functions as a library for integration into Rust applications, but it also includes a command-line interface (CLI) for ease of use. This dual functionality allows developers to leverage InTime's capabilities in various contexts, whether embedding it in larger projects or using it directly from the terminal.

## Features

- **Human-Readable Output**: Converts time durations into formats that are easy to understand.
- **Multiple Time Units Supported**: Handles seconds, minutes, hours, days, weeks, months, and years.
- **Library and CLI Tool**: Primarily a Rust library with an accompanying CLI for quick access.

## Dependencies

InTime relies on several external crates to provide its functionality:

### Library

- **battery**: Provides system battery information, allowing InTime to incorporate device power status into time calculations if needed.
- **chrono**: A comprehensive date and time library for Rust, essential for accurate time duration calculations and formatting.
- **uom**: Units of Measurement library, enabling precise handling of various time units and their conversions.

### CLI

- **clap**: A powerful Command Line Argument Parser, used to build InTime's CLI interface for easy interaction with the tool from the command line.

These dependencies are crucial for InTime's core functionality, ensuring accurate time calculations, proper unit handling, and a user-friendly command-line interface.

### Usage

To incorporate InTime into your Rust project, simply add the following line to your `Cargo.toml` file:

```toml
[dependencies]
intime = "0.1.0"
```

### Getting Started with the CLI

The CLI tool provides a straightforward way to format time durations directly from the command line. Users can input time durations in various units and receive formatted outputs. This feature is particularly useful for quick checks without needing to write additional code.

### Example Commands

Here are some example commands you might use with the InTime CLI:

```bash
# Format a duration of 3600 seconds
intime 3600

# Hide units below minutes
intime 3600 --above-seconds
```

### Conclusion

InTime serves as a powerful tool for developers needing to manage time durations efficiently. Its combination of library and CLI functionality makes it adaptable for various use cases in Rust projects. Whether you're building applications or simply need a quick formatting tool, InTime is designed to meet those needs effectively.
