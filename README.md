# Jack Compiler Frontend

This is a Jack compiler frontend implementation, which takes Jack source code as input and outputs its AST. To build and run this project, please read the sections [Prerequisite](#prerequisite) and [Usage](#run-the-project).

## Prerequisite

The whole project is written in Rust, so Rust toolchain is required to build and run it.

Rust toolchain can installed [here](https://rust-lang.org/tools/install/).

## Usage

If the prerequisite is satisfied, the project can be built and run using the following command:

```
cargo run --release -- [OPTIONS] <Jack source code>
```

**Options:**

- `-f, --format <format>`: The output format. It can be `xml` or `debug`, if not specified, `xml` is used.
  - `xml`: XML format, which is used by the test cases.
  - `debug`: Rust debug formatting with pretty-print. It would show the whole structure of the AST, including its data. This is usually the output format we'll see while using a debugger.

- `-o, --output <output>`: The path to the output file, if not specified, the output would be written to stdout. Note that if any parent directory is missing in `<output>`, it would be created automatically, but it is users' responsibility to ensure that they have necessary permission to do that.
