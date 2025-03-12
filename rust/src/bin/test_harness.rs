// Test harness for TypeScript file checking
// Runs both our Rust compiler and Deno on the same files and compares output

use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

// Import from our main crate
use typescript_rust::compiler;
use typescript_rust::error::Diagnostic;
use typescript_rust::parser;

fn main() {
    // Get test directory (use absolute path)
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let test_dir = current_dir.join("test");

    // Ensure the test directory exists
    if !test_dir.exists() {
        eprintln!("Test directory does not exist: {:?}", test_dir);
        std::process::exit(1);
    }

    println!("Running TypeScript test harness...\n");

    // Find all TypeScript files in the test directory
    let entries = match fs::read_dir(test_dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error reading test directory: {}", e);
            std::process::exit(1);
        }
    };

    let mut test_files = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "ts") {
                test_files.push(path);
            }
        }
    }

    // Sort files so we have a predictable order
    test_files.sort();

    println!("Found {} test files:", test_files.len());
    for file in &test_files {
        println!("  - {}", file.display());
    }
    println!();

    let mut passed = 0;
    let mut failed = 0;

    // Run each test file
    for file_path in test_files {
        let result = run_test(&file_path);

        if result {
            passed += 1;
            println!("✅ PASS: {}", file_path.display());
        } else {
            failed += 1;
            println!("❌ FAIL: {}", file_path.display());
        }
        println!();
    }

    // Print summary
    println!("Test summary:");
    println!("  - Passed: {}", passed);
    println!("  - Failed: {}", failed);
    println!("  - Total: {}", passed + failed);

    if failed > 0 {
        std::process::exit(1);
    }
}

/// Run a single test file through both our compiler and Deno
fn run_test(file_path: &Path) -> bool {
    println!("Testing file: {}", file_path.display());

    // Step 1: Run through our Rust implementation
    let rust_results = run_rust_compiler(file_path);

    // Step 2: Run through Deno
    let deno_results = run_deno(file_path);

    // Step 3: Compare results
    compare_results(file_path, &rust_results, &deno_results)
}

/// Run our Rust compiler on a TypeScript file
fn run_rust_compiler(file_path: &Path) -> Vec<Diagnostic> {
    println!("Running Rust compiler...");
    let start = Instant::now();

    // Read the file
    let source_text = match fs::read_to_string(file_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path.display(), e);
            return vec![];
        }
    };

    // Get the file name
    let file_name = file_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    // Parse the file
    let source_file = match parser::parse_source_file(&file_name, &source_text) {
        Ok(source_file) => source_file,
        Err(e) => {
            // Return the error as a diagnostic
            return vec![e];
        }
    };

    // Create a program and perform type checking
    let mut program = compiler::create_program(source_file);
    if let Err(e) = program.type_check() {
        // Return the error as a diagnostic
        return vec![e];
    }

    // Get all diagnostics
    let diagnostics = program.get_diagnostics().to_vec();

    let elapsed = start.elapsed();
    println!("  Completed in {:?}", elapsed);
    println!("  Found {} diagnostics", diagnostics.len());

    diagnostics
}

/// Run Deno on a TypeScript file
fn run_deno(file_path: &Path) -> String {
    println!("Running Deno...");
    let start = Instant::now();

    // Make sure we use the absolute path
    let abs_path = if file_path.is_absolute() {
        file_path.to_path_buf()
    } else {
        std::env::current_dir()
            .expect("Failed to get current directory")
            .join(file_path)
    };

    // Run deno check command
    let output = Command::new("deno").arg("check").arg(&abs_path).output();

    let result = match output {
        Ok(output) => {
            // Combine stdout and stderr
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            // Include the command's status for debugging
            let status_info = if output.status.success() {
                "Deno check succeeded"
            } else {
                "Deno check failed"
            };

            format!("[{}]\n{}{}", status_info, stdout, stderr)
        }
        Err(e) => {
            eprintln!("Error running Deno: {}", e);
            "[Error running Deno]".to_string()
        }
    };

    let elapsed = start.elapsed();
    println!("  Completed in {:?}", elapsed);

    // Extract error count if present
    if let Some(index) = result.find("Found") {
        if let Some(end) = result[index..].find("\n") {
            println!("  {}", &result[index..index + end]);
        }
    }

    result
}

/// Compare the results from our compiler and Deno
fn compare_results(file_path: &Path, rust_diagnostics: &[Diagnostic], deno_output: &str) -> bool {
    println!("Comparing results...");

    // Check if Deno found errors (look for error message or "Found X errors")
    let deno_has_errors = deno_output.contains("ERROR")
        || deno_output.contains("error:")
        || deno_output.contains("Found") && deno_output.contains("error");

    // Our compiler has errors if diagnostics is not empty
    let rust_has_errors = !rust_diagnostics.is_empty();

    // For pass.ts, we expect no diagnostics from our compiler
    // and no error message from Deno
    if file_path.file_name().unwrap_or_default() == "pass.ts" {
        if !rust_has_errors && !deno_has_errors {
            println!("  Both compilers reported no errors, as expected.");
            return true;
        } else if rust_has_errors && !deno_has_errors {
            println!("  ❌ Mismatch: Deno reported no errors, but our compiler found errors:");
            for diag in rust_diagnostics {
                println!("    - {}", diag);
            }
            return false;
        } else if !rust_has_errors && deno_has_errors {
            println!("  ❌ Mismatch: Our compiler reported no errors, but Deno found errors:");
            println!("  Deno output:");
            for line in deno_output.lines() {
                if line.contains("ERROR") || line.contains("error:") {
                    println!("    {}", line);
                }
            }
            return false;
        } else {
            // Both found errors for a file that should pass
            println!("  ❌ Both compilers reported errors, but the file should pass:");

            // Display diagnostics from our compiler
            println!("  Our compiler diagnostics:");
            for diag in rust_diagnostics {
                println!("    - {}", diag);
            }

            // Display Deno errors
            println!("  Deno output:");
            for line in deno_output.lines() {
                if line.contains("ERROR") || line.contains("error:") {
                    println!("    {}", line);
                }
            }

            return false;
        }
    }

    // For fail.ts, we expect both to report errors
    // Ideally we would check that the specific errors match,
    // but for now we'll just check that both find errors
    if file_path.file_name().unwrap_or_default() == "fail.ts" {
        if rust_has_errors && deno_has_errors {
            println!("  Both compilers reported errors, as expected.");

            // Display diagnostics from our compiler
            println!("  Our compiler diagnostics:");
            for diag in rust_diagnostics {
                println!("    - {}", diag);
            }

            // Display Deno errors
            println!("  Deno output:");
            for line in deno_output.lines() {
                if line.contains("ERROR") || line.contains("error:") {
                    println!("    {}", line);
                }
            }

            return true;
        } else if !rust_has_errors && deno_has_errors {
            println!("  ❌ Mismatch: Deno reported errors, but our compiler found none:");
            println!("  Deno output:");
            for line in deno_output.lines() {
                if line.contains("ERROR") || line.contains("error:") {
                    println!("    {}", line);
                }
            }
            return false;
        } else if rust_has_errors && !deno_has_errors {
            println!("  ❌ Mismatch: Our compiler reported errors, but Deno found none:");
            for diag in rust_diagnostics {
                println!("    - {}", diag);
            }
            return false;
        } else {
            // Neither found errors for a file that should fail
            println!("  ❌ Neither compiler reported errors, but the file should fail.");
            return false;
        }
    }

    // For any other files, just compare if they both report errors or both pass
    if rust_has_errors == deno_has_errors {
        println!("  Both compilers agree on error status.");

        // Display details if there are errors
        if rust_has_errors {
            // Display diagnostics from our compiler
            println!("  Our compiler diagnostics:");
            for diag in rust_diagnostics {
                println!("    - {}", diag);
            }

            // Display Deno errors
            println!("  Deno output:");
            for line in deno_output.lines() {
                if line.contains("ERROR") || line.contains("error:") {
                    println!("    {}", line);
                }
            }
        }

        return true;
    } else {
        println!("  ❌ Compilers disagree on error status:");

        if rust_has_errors {
            println!("  Our compiler diagnostics:");
            for diag in rust_diagnostics {
                println!("    - {}", diag);
            }
        } else {
            println!("  Our compiler reported no errors.");
        }

        if deno_has_errors {
            println!("  Deno output:");
            for line in deno_output.lines() {
                if line.contains("ERROR") || line.contains("error:") {
                    println!("    {}", line);
                }
            }
        } else {
            println!("  Deno reported no errors.");
        }

        return false;
    }
}
