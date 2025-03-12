The top level directory contains typescript-go, a go implementation of the
typescript compiler.

We are going to translate the entire project to Rust. The rust code lives in the
rust/ subdirectory. We don't make any changes outside the rust/ directory.

The current goal is to build a project structure with the same functionality as
the go project, while incrementally implementing the complete TypeScript
language.

Try to follow the structure of to the go code as closely as possible.

Whenever you create Rust files, structures, functions, blocks of logic, etc, add
comments to them that reference the go implementation. Include the path to the
go source file. Add comments too if you implement control flow that deviates
from the original go source.

When you add debugging code (eprintln statements etc) that doesn't appear in the
Go source code, mark them as such, so we can remove them more easily later. Feel
free to remove debug statements that you added but no longer find useful.

Document implementation progress in rust/PROGRESS.md. You should write down
which parts of the Go codebase have been (partially) translated to rust and any
major deviations you made. Estimate progress as a percentage.

Some test typescript programs that were not part of the original Go codebase
have been placed in the rust/test directory. Some of them pass type checking
while others fail - you may run `deno check <filename>` to see which ones pass
and which ones fail. The Rust codebase should reproduce the error messages you
get from Deno exactly. As you implement more TypeScript features, be sure to add
tests for them as well. You can run these tests with
`cargo run --bin test-harness`.

There are unit tests in the code base as well, use `cargo test` to run them.
