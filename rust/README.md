# TypeScript Compiler in Rust

This is a Rust implementation of a TypeScript compiler. It's currently focused on parsing and type-checking a minimal subset of TypeScript for demonstration purposes.

## Current Features

- Lexical analysis (scanner) that tokenizes TypeScript code
- Parser that builds an Abstract Syntax Tree (AST)
- Type checker that verifies types are used correctly
- Support for:
  - Function definitions
  - Function calls
  - String and number literals
  - Basic type annotations (string, number)
  - Binary operations (currently only +)

## Example

The compiler can successfully parse and type-check this simple TypeScript program:

```typescript
function demo(a: string, b: number): string {
  return a + b;
}

demo("hello", 42);
```

## Building and Running

To build and run the project:

```bash
cd rust
cargo build
cargo run
```

The program will read the demo.ts file from the parent directory, parse it, and type-check it.

## Project Structure

- `main.rs`: The entry point of the program, handles file reading and coordinating the compilation phases.
- `scanner.rs`: Performs lexical analysis to convert the source code into tokens.
- `parser.rs`: Parses the tokens into an Abstract Syntax Tree (AST).
- `ast.rs`: Defines the AST data structures.
- `types.rs`: Implements the type checker.
- `error.rs`: Defines the error types used throughout the compiler.

## Future Enhancements

- Support for more TypeScript features
- Code generation
- Source maps
- Error recovery
- IDE support (code completion, diagnostics, etc.)