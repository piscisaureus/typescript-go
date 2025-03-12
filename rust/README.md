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

The project is organized to mirror the Go implementation:

- `src/ast/` - AST definitions (corresponds to internal/ast in Go)
- `src/parser/` - Parser implementation (corresponds to internal/parser in Go)
- `src/scanner/` - Lexical scanner (corresponds to internal/scanner in Go)
- `src/error/` - Diagnostic and error handling (corresponds to internal/compiler/diagnostics in Go)
- `src/compiler/` - Compiler components (corresponds to internal/compiler in Go)

Each file contains comments referencing the corresponding Go files and types to make it easier to understand the relationship between the two implementations.

## Future Enhancements

- Support for more TypeScript features
- Code generation
- Source maps
- Error recovery
- IDE support (code completion, diagnostics, etc.)