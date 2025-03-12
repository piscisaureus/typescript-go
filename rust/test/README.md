# TypeScript Test Harness

This directory contains TypeScript test files that are used to validate the Rust
TypeScript compiler implementation against Deno's TypeScript implementation.

## Test Files

- `pass.ts`: Contains TypeScript code that should pass type checking without
  errors
- `fail.ts`: Contains TypeScript code that should fail type checking with
  specific errors
- Additional test files can be added to test more specific features

## Running the Tests

To run the test harness, use:

```bash
cd rust
cargo run --bin test-harness
```

The test harness will:

1. Find all .ts files in the test directory
2. Run each file through both our Rust implementation and Deno
3. Compare the results to ensure our implementation correctly identifies errors

## Adding New Tests

New test files can be added to this directory. Follow these conventions:

- If a file should pass type checking without errors, name it with `pass` in the
  name
- If a file should fail type checking with specific errors, name it with `fail`
  in the name
- Add comments to explain what specific errors you expect

## Matching Deno's Output

The goal of the Rust implementation is to reproduce error messages from Deno
exactly. This includes:

- Error codes (e.g., TS2554)
- Error messages
- Line and column locations
- Additional context information

As we improve the implementation, we should aim to make our error messages more
and more similar to Deno's.
