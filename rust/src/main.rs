// Main entry point for the TypeScript compiler in Rust
// Corresponds to cmd/tsgo/main.go in the Go implementation

use std::fs;
use std::path::Path;
use std::process;

mod ast;
mod compiler;
mod error;
mod parser;
mod scanner;

fn main() {
    // Temporary implementation that just processes the demo.ts file
    let file_path = "../demo.ts";

    // Read the file
    let source_text = match fs::read_to_string(file_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            process::exit(1);
        }
    };

    // Parse the file
    let file_name = Path::new(file_path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    println!("Parsing file: {}", file_name); // DEBUG: not in Go
    println!("Source code:\n{}", source_text); // DEBUG: not in Go

    // Create scanner to debug tokens
    // DEBUG: not in Go - This whole token logging block is for debug only
    let mut scanner = scanner::Scanner::new(&source_text);
    println!("\nTokens:");
    loop {
        let token = scanner.scan();
        let text = scanner.token_text();
        println!("  {:?}: {}", token, text);
        if token == ast::Kind::EndOfFile {
            break;
        }
    }

    match parser::parse_source_file(&file_name, &source_text) {
        Ok(source_file) => {
            println!("\nParsing successful!"); // DEBUG: not in Go

            // Check for parsing diagnostics
            if !source_file.diagnostics.is_empty() {
                println!("\nParsing Diagnostics:"); // DEBUG: not in Go
                for diag in &source_file.diagnostics {
                    println!("{}", diag); // DEBUG: not in Go
                }
            }

            // Print a simple representation of the AST
            // DEBUG: not in Go - This whole AST visualization block is for debug only
            println!("\nProgram structure:");
            println!("- Source file: {}", source_file.file_name);

            // Print functions
            let mut found_functions = false;
            for stmt in &source_file.statements {
                if stmt.kind() == ast::Kind::FunctionDeclaration {
                    found_functions = true;
                    println!("  - Function declaration");
                }
            }

            if !found_functions {
                println!("  (No functions found)");
            }

            // Print statements that are not functions
            let mut found_statements = false;
            for stmt in &source_file.statements {
                if stmt.kind() != ast::Kind::FunctionDeclaration {
                    found_statements = true;
                    println!("  - Statement: {:?}", stmt.kind());
                }
            }

            if !found_statements {
                println!("  (No non-function statements found)");
            }

            // Create a program and perform type checking
            println!("\nPerforming type checking..."); // DEBUG: not in Go
            let mut program = compiler::create_program(source_file);

            match program.type_check() {
                Ok(_) => {
                    println!("Type checking completed!"); // DEBUG: not in Go

                    // Print any type checking diagnostics
                    let diagnostics = program.get_diagnostics();
                    if !diagnostics.is_empty() {
                        println!("\nType Checking Diagnostics:"); // DEBUG: not in Go
                        for diag in diagnostics {
                            println!("{}", diag); // DEBUG: not in Go
                        }
                    } else {
                        println!("No type errors found."); // DEBUG: not in Go
                    }
                }
                Err(e) => {
                    eprintln!("Error during type checking: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            process::exit(1);
        }
    }
}
