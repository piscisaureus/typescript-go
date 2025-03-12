// Corresponds to utility functions in internal/compiler/program.go

use crate::error::Diagnostic;
use std::path::{Path, PathBuf};

/// Sort and deduplicate a list of diagnostics
/// Corresponds to SortAndDeduplicateDiagnostics in internal/compiler/program.go
pub fn sort_and_deduplicate_diagnostics(diagnostics: &mut Vec<Diagnostic>) {
    // Sort diagnostics by position
    diagnostics.sort_by_key(|diag| (diag.pos, diag.pos + diag.len));

    // Deduplicate diagnostics with the same position and message
    if diagnostics.len() > 1 {
        let mut i = 0;
        while i < diagnostics.len() - 1 {
            if diagnostics[i].pos == diagnostics[i + 1].pos
                && diagnostics[i].pos + diagnostics[i].len
                    == diagnostics[i + 1].pos + diagnostics[i + 1].len
                && diagnostics[i].message == diagnostics[i + 1].message
            {
                // Remove the duplicate
                diagnostics.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }
}

/// Check if a line in the source is a comment or blank line
/// Corresponds to isCommentOrBlankLine in internal/compiler/program.go
pub fn is_comment_or_blank_line(line: &str) -> bool {
    let line = line.trim();
    line.is_empty() || line.starts_with("//") || line.starts_with("/*")
}

/// Compute the common source directory of a list of file paths
/// Corresponds to computeCommonSourceDirectoryOfFilenames in internal/compiler/program.go
pub fn compute_common_source_directory(file_paths: &[String]) -> String {
    if file_paths.is_empty() {
        return String::new();
    }

    // Convert all paths to absolute paths
    let abs_paths: Vec<PathBuf> = file_paths
        .iter()
        .filter_map(|path| Path::new(path).canonicalize().ok())
        .collect();

    if abs_paths.is_empty() {
        return String::new();
    }

    // Start with the first path's components
    let mut common_components: Vec<String> = abs_paths[0]
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .collect();

    // Compare with each other path
    for path in &abs_paths[1..] {
        let components: Vec<String> = path
            .components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .collect();

        // Find the common prefix
        let mut common_len = 0;
        for (i, (a, b)) in common_components.iter().zip(components.iter()).enumerate() {
            if a == b {
                common_len = i + 1;
            } else {
                break;
            }
        }

        // Truncate to the common prefix
        common_components.truncate(common_len);

        if common_components.is_empty() {
            break;
        }
    }

    // Construct the common path
    let common_path = PathBuf::from("/").join(common_components.join("/"));
    common_path.to_string_lossy().to_string()
}
