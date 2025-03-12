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
            let diagnostic = e.with_file(&file_name).with_source_text(&source_text);
            return vec![diagnostic];
        }
    };

    // Create a program and perform type checking
    let mut program = compiler::create_program(source_file);
    if let Err(err_msg) = program.type_check() {
        // Create a synthetic diagnostic from the error message
        let diagnostic = Diagnostic::new(
            typescript_rust::error::DiagnosticCode::SyntaxError,
            &err_msg,
            &file_name,
            0,
            0,
            1,
            1,
        );
        return vec![diagnostic];
    }

    // Get all diagnostics
    let mut diagnostics = program.get_diagnostics().to_vec();

    // Ensure all diagnostics have proper file information
    for diag in &mut diagnostics {
        if diag.file.is_empty() {
            diag.file = file_name.clone();
        }

        // If the line is 1:1, try to recompute using the source text
        if diag.line == 1 && diag.column == 1 && diag.pos > 0 {
            let (line, column) = Diagnostic::compute_line_column(&source_text, diag.pos);
            diag.line = line;
            diag.column = column;
        }
    }

    let elapsed = start.elapsed();
    println!("  Completed in {:?}", elapsed);
    println!("  Found {} diagnostics", diagnostics.len());

    // Print location information for each diagnostic
    for (i, diag) in diagnostics.iter().enumerate() {
        println!(
            "  Diagnostic #{}: TS{:04} at {}:{}:{}",
            i + 1,
            diag.code as u32,
            diag.file,
            diag.line,
            diag.column
        );
    }

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

    // Run deno check command with NO_COLOR env var to disable colors
    let output = Command::new("deno")
        .env("NO_COLOR", "1")
        .arg("check")
        .arg(&abs_path)
        .output();

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

    // Extract Deno's error locations from output
    let mut deno_locations: Vec<(String, String, usize, usize)> = Vec::new();

    println!("  Extracting Deno error locations...");

    // Modern Deno (>1.0) format parser - parses output like:
    // TS2554 [ERROR]: Expected 4 arguments, but got 1.
    // demo("hello");
    // ~~~~
    //     at file:///Users/.../rust/test/fail.ts:5:1

    let mut lines = deno_output.lines().peekable();
    while let Some(line) = lines.next() {
        // Look for TS error codes with [ERROR] or error: patterns
        if line.contains("TS") && (line.contains("[ERROR]") || line.contains("error:")) {
            // Extract the TS code
            let ts_code = if let Some(ts_start) = line.find("TS") {
                if let Some(end) = line[ts_start..].find(" ") {
                    line[ts_start..ts_start + end].trim().to_string()
                } else {
                    "TS????".to_string()
                }
            } else {
                "TS????".to_string()
            };

            // Get error message - extract the message part after the TS code
            let error_msg = if let Some(ts_start) = line.find("TS") {
                if let Some(colon) = line[ts_start..].find(":") {
                    let start_pos = ts_start + colon + 1;
                    line[start_pos..].trim().to_string()
                } else {
                    line.trim().to_string()
                }
            } else {
                line.trim().to_string()
            };

            // Skip any number of lines until we find a line containing "at file://..."
            while let Some(loc_line) = lines.peek() {
                if loc_line.contains("at file://") && loc_line.contains(".ts:") {
                    // Parse file location from the line
                    if let Some(file_pos) = loc_line.rfind("file://") {
                        if let Some(ts_pos) = loc_line[file_pos..].rfind(".ts:") {
                            let loc_part = &loc_line[file_pos + ts_pos + 4..]; // Skip ".ts:"

                            // Parse line:column, format like "5:1"
                            if let Some(colon_pos) = loc_part.find(':') {
                                if let (Ok(line_num), Ok(col_num)) = (
                                    loc_part[..colon_pos].trim().parse::<usize>(),
                                    loc_part[colon_pos + 1..]
                                        .trim()
                                        .split(|c| !char::is_numeric(c))
                                        .next()
                                        .unwrap_or("1")
                                        .parse::<usize>(),
                                ) {
                                    // Extract filename
                                    let filename = if let Some(slash_pos) = loc_line.rfind('/') {
                                        let end_pos = loc_line[slash_pos..]
                                            .find(".ts:")
                                            .unwrap_or(loc_line.len() - slash_pos);
                                        loc_line[slash_pos + 1..slash_pos + end_pos].to_string()
                                    } else {
                                        "unknown.ts".to_string()
                                    };

                                    // Add to locations list
                                    deno_locations.push((
                                        ts_code.clone(),
                                        error_msg.clone(),
                                        line_num,
                                        col_num,
                                    ));
                                    println!(
                                        "    Found error at {}:{}:{}",
                                        filename, line_num, col_num
                                    );

                                    // Consume the location line
                                    lines.next();
                                    break;
                                }
                            }
                        }
                    }
                }

                // Move to next line
                lines.next();

                // If we've gone more than 10 lines without finding location, give up on this error
                if deno_locations.len() > 10 {
                    break;
                }
            }
        }
    }

    // If we couldn't extract any locations with the primary method, try the fallback approach
    if deno_locations.is_empty() && deno_has_errors {
        println!("  Using fallback location extraction...");

        // Look for patterns like "file.ts:5:1" anywhere in the output
        for line in deno_output.lines() {
            if line.contains(".ts:") {
                // For lines with file:///path/to/file.ts:5:1 format
                if line.contains("file://") {
                    let parts: Vec<&str> = line.split(".ts:").collect();
                    if parts.len() > 1 {
                        let location_part = parts[1];
                        if let Some(colon_pos) = location_part.find(':') {
                            if let (Ok(line_num), Ok(col_num)) = (
                                location_part[..colon_pos].trim().parse::<usize>(),
                                location_part[colon_pos + 1..]
                                    .trim()
                                    .split_whitespace()
                                    .next()
                                    .unwrap_or("1")
                                    .parse::<usize>(),
                            ) {
                                // Extract TS code if present on the same line or nearby
                                let (ts_code, error_msg) = if line.contains("TS") {
                                    if let Some(ts_pos) = line.find("TS") {
                                        if let Some(space_pos) = line[ts_pos..].find(' ') {
                                            let code =
                                                line[ts_pos..ts_pos + space_pos].trim().to_string();

                                            // Extract message part after the TS code
                                            let msg = if let Some(colon) = line[ts_pos..].find(':')
                                            {
                                                line[ts_pos + colon + 1..].trim().to_string()
                                            } else {
                                                line.trim().to_string()
                                            };

                                            (code, msg)
                                        } else {
                                            ("TS????".to_string(), line.trim().to_string())
                                        }
                                    } else {
                                        ("TS????".to_string(), line.trim().to_string())
                                    }
                                } else {
                                    ("TS????".to_string(), line.trim().to_string())
                                };

                                // Add to locations
                                deno_locations.push((ts_code, error_msg, line_num, col_num));
                                println!(
                                    "    Found error (fallback) at line {}:{}",
                                    line_num, col_num
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    println!("  Found {} Deno diagnostic locations", deno_locations.len());

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
    // Check that both find errors and compare location info
    if file_path.file_name().unwrap_or_default() == "fail.ts" {
        if rust_has_errors && deno_has_errors {
            println!("  Both compilers reported errors, as expected.");

            // Display diagnostics from our compiler
            println!("  Our compiler diagnostics:");
            for diag in rust_diagnostics {
                println!(
                    "    - TS{:04}: {} (at {}:{}:{})",
                    diag.code as u32,
                    diag.message,
                    if diag.file.is_empty() {
                        file_path.file_name().unwrap_or_default().to_string_lossy()
                    } else {
                        diag.file.clone().into()
                    },
                    diag.line,
                    diag.column
                );
            }

            // Display Deno errors with their locations
            println!("  Deno diagnostics:");
            if deno_locations.is_empty() {
                // If we couldn't parse the locations, just show the raw output
                for line in deno_output.lines() {
                    if line.contains("ERROR") || line.contains("error:") {
                        println!("    {}", line);
                    }
                }
            } else {
                // Show the parsed locations
                for (ts_code, error, line, column) in &deno_locations {
                    let filename = file_path.file_name().unwrap_or_default().to_string_lossy();

                    println!(
                        "    - {}: {} (at {}:{}:{})",
                        ts_code, error, filename, line, column
                    );
                }
            }

            // Compare location accuracy between Rust and Deno diagnostics
            if !rust_diagnostics.is_empty() && !deno_locations.is_empty() {
                println!("  Location comparison analysis:");

                // Sort the Deno locations by line number for easier comparison
                let deno_loc_only: Vec<(usize, usize)> = deno_locations
                    .iter()
                    .map(|(_, _, line, col)| (*line, *col))
                    .collect();

                // Group Rust errors by line/column
                let mut rust_loc_map: std::collections::HashMap<(usize, usize), usize> =
                    std::collections::HashMap::new();
                for diag in rust_diagnostics {
                    let key = (diag.line, diag.column);
                    *rust_loc_map.entry(key).or_insert(0) += 1;
                }

                // Check if all our errors are at 1:1 (which indicates inaccurate locations)
                let default_position_count = rust_loc_map.get(&(1, 1)).unwrap_or(&0);
                let total_errors = rust_diagnostics.len();

                if *default_position_count > 0 {
                    println!("    ⚠️  Warning: {}/{} Rust errors are at line 1:1, which is likely incorrect", 
                        default_position_count, total_errors);
                    println!(
                        "    ⚠️  Expected locations based on Deno: {:?}",
                        deno_loc_only
                    );
                }

                // Compare each Rust error with Deno errors at the same location
                let mut matches = 0;
                for diag in rust_diagnostics {
                    let rust_pos = (diag.line, diag.column);

                    // Check if there's a Deno error at the same location
                    if deno_loc_only.contains(&rust_pos) {
                        matches += 1;
                    }
                }

                let match_percentage = if total_errors > 0 {
                    (matches as f64 / total_errors as f64) * 100.0
                } else {
                    0.0
                };

                println!(
                    "    Location accuracy: {:.1}% ({}/{} errors at matching locations)",
                    match_percentage, matches, total_errors
                );
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
