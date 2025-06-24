# Embellish

A CLI utility for styling text displayed in the terminal

## Overview

Embellish is a powerful command-line utility for text styling and formatting in
the terminal. It enables you to emphasize and style your text with color,
alignment, offset, padding, and background color, making it an ideal choice for
creating visually appealing and readable CLI applications.

## Features

- **Emphasis**: Apply various emphasis styles to text, including bold, italic,
  underline, and more.

- **Color**: Set both foreground and background colors for text.

- **Alignment**: Control the alignment of text, both horizontally and
  vertically. (Defaults: "left" and "top")

- **Offset and Padding**: Add leading and trailing characters (single character)
  to shift and space text, providing customizable alignment and formatting.

- **Display Area**: Set the size of the print canvas to ensure consistent
  output, or adapt to the terminal's size dynamically.

- **Portability**: Works seamlessly on ANSI-capable terminals, falling back to
  plain text on non-ANSI terminals.

## Dependencies

Embellish relies on the following Rust crates to enhance its functionality:

- [clap](https://crates.io/crates/clap): Used for command-line argument parsing.
  It enables you to specify options and parameters when using Embellish.

- [colored](https://crates.io/crates/colored): Used for text formatting and
  applying color to the text. This crate allows you to style and emphasize your
  text effectively.

- [crossterm](https://crates.io/crates/term_size): Used to interact with the
  terminal, obtaining terminal dimensions dynamically and printing text.

- [thiserror](https://crates.io/crates/thiserror): Used for error handling,
  providing clear and concise error messages.

## Installation

You can install Embellish using Cargo, Rust's package manager:

```shell
cargo install embellish
```

## Usage

Embellish allows you to emphasize and style your text with ease. The basic
command structure is as follows:

```shell
embellish [OPTIONS] "text to embellish"
```

## Options

- `-e, --emphasis [STYLES]`: Apply emphasis styles to the text. You can specify
  multiple styles, separated by commas (e.g., "bold, italic").

- `-a, --align [ALIGNMENT]`: Align the text. You can specify one alignment value
  or a comma-separated tuple (e.g., "left, top" or "center, bottom"). (Defaults:
  "left" and "top")

- `-p, --padding [SIZE, CHARACTERS]`: Add padding characters (single character)
  around the text. The size comes before the character. (Optional)

- `-o, --offset [SIZE, CHARACTERS]`: Offset the text from the left (single
  character). The size comes before the character. (Optional)

- `-W, --width WIDTH`: Set the width of the print canvas. (Optional)

- `-H, --height HEIGHT`: Set the height of the print canvas. (Optional)

- `--vertical-align ALIGNMENT`: Vertically align the text ("top", "center",
  "bottom"). (Defaults: "top")

- `--color FG,BG`: Set both foreground and background colors for text and set
  background color for text.

- Additional color options can be set using `--#color_name` for text color and
  `--on-#color_name` for background color.

## Examples

- Print red, bold and italic text with a yellow background:

  ```shell
  embellish -e "bold,italic" --color "red,yellow" "Important Information"
  ```

- Center-align and add padding:

  ```shell
  embellish -a "center,bottom" -p "20,." "Centered Text"
  ```

- Customize print canvas size:

  ```shell
  embellish -w 60 -h 10 "Custom Size Text"
  ```

- Precise text alignment with offset and padding:

  ```shell
  embellish -o "5,>" -p "10,|" "Advanced Alignment"
  ```

## Library Usage

Embellish can also be used as a library in your Rust projects. Add it as a
dependency in your `Cargo.toml`:

```toml
[dependencies]
embellish = "0.1"
```

In your code:

```rust
extern crate embellish;

fn main() {
    let styled_text = embellish::style("Text to embellish")
        .with_emphasis("bold,italic")
        .with_padding("20,.")
        .to_string();
    println!("{}", styled_text);
}
```

## Compatibility

Embellish is compatible with terminals capable of ANSI styling and gracefully
falls back to plain text on non-ANSI terminals.

## Contributing

We welcome contributions to Embellish! Feel free to open issues or submit pull
requests on our [GitHub repository](https://github.com/craole-cc/embellish).

## License

Embellish is distributed under the Apache License 2.0. See
[LICENSE](docs/LICENSE) for more information.

## Contact

For questions or support, please contact
[craole@tuta.io](mailto:craole@tuta.io).

## Acknowledgments

We'd like to thank the Rust community and the authors of all the external crates
used in the development of this utility. As well as the developers of ChatGPT
which served as a good source of information.
