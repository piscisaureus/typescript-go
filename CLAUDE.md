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

When you add debugging code (eprintln statements etc) that doesn't appear in the
Go source code, mark them as such, so we can remove them more easily later. Feel
free to remove debug statements that you added but no longer find useful.

Document implementation progress in rust/PROGRESS.md. You should write down
which parts of the Go codebase have been (partially) translated to rust and any
major deviations you made. Estimate progress as a percentage.
