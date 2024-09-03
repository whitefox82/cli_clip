# cli_clip# clip

`clip` is a simple command-line tool written in Rust that executes a given command with its arguments, captures the output (both stdout and stderr), and copies the output to the clipboard. The output is also displayed in the terminal.

## Features

- Executes any command with the provided arguments.
- Captures both stdout and stderr of the command.
- Copies the output to the clipboard automatically.
- Displays the output in the terminal.

## Usage

```bash
clip <command> [<args>...]
```

### Example

```bash
clip cargo build --release
```

This will run cargo build --release, copy the output of the command (including any errors) to the clipboard, and print the output in the terminal.

## Installation

To use `clip`, you need to have Rust installed. If you don't have it, you can install it from [rust-lang.org](https://www.rust-lang.org/).

Once Rust is installed, you can build the tool by running:

```bash
cargo build --release
```

This will create an executable in the `target/release` directory that you can use.

### Running without building

If you prefer not to build the tool, you can run it directly using `cargo`:

```bash
cargo run -- <command> [<args>...]
```

## Dependencies

`clip` uses the following Rust crates:

- `clipboard`: Provides clipboard access across different platforms.
- `std::process`: Handles process creation and management.
- `std::io`: Facilitates input/output operations.

## Error Handling

If the command fails to execute, the tool will print an error message and exit with a status code of `1`.

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](https://github.com/whitefox82/cli_clip/blob/main/LICENSE) file for details.