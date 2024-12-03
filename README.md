# Rust Coverage Testing Example

This is an example of how to use the `cargo-llvm-cov` tool to generate coverage reports for a Rust project.

## Setup

```bash
cargo install cargo-binstall
```

We need to use nightly Rust to run the tests and generate the coverage report.

```bash
rustup install nightly
```

Once you have `binstall` installed, you can install the other tools with the following command:

```bash
cargo binstall -y cargo-nextest cargo-llvm-cov cargo-mutants just cargo-insta
```

## Running the tests

```bash
just test
```

## Updating the snapshot

```bash
cargo insta review
```

## See the coverage report in your browser

```bash
just coverage
```
