The top level directory contains typescript-go, a go implementation of the
typescript compiler.

We are going to translate the entire project to Rust. The rust code lives in the
rust/ subdirectory. We don't make any changes outside the rust/ directory.

The current goal is to build a project structure with the same functionality as
the go project, without implementing the complete TypeScript language. The only
thing it should be able to compile is this toy program:

```typescript
// demo.ts
function demo(a: string, b: number): string {
  return a + b;
}

demo("hello", 42);
```

Try to follow the structure of to the go code as closely as possible.

Whenever you create Rust files, structures, functions, etc, add comments to them
that reference the go implementation.
