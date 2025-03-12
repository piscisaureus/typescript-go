// Corresponds to internal/compiler/diagnostics.go in the Go implementation

use crate::error::{Diagnostic, DiagnosticCode};
use std::fmt;

/// Diagnostic category determines severity level of the diagnostic
/// Corresponds to Category in internal/compiler/diagnostics.go
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCategory {
    /// Warnings don't prevent compilation
    Warning,
    /// Errors prevent successful compilation
    Error,
    /// Messages are informational only
    Message,
    /// Suggestions provide hints for improvement
    Suggestion,
}

impl fmt::Display for DiagnosticCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Warning => write!(f, "warning"),
            Self::Error => write!(f, "error"),
            Self::Message => write!(f, "info"),
            Self::Suggestion => write!(f, "hint"),
        }
    }
}

/// Get the category of a diagnostic based on its code
/// Corresponds to GetCategoryFromCode in internal/compiler/diagnostics.go
pub fn get_category_from_code(code: DiagnosticCode) -> DiagnosticCategory {
    match code {
        // Error diagnostics
        DiagnosticCode::SyntaxError
        | DiagnosticCode::TypeMismatch
        | DiagnosticCode::UndefinedVariable
        | DiagnosticCode::UndefinedFunction
        | DiagnosticCode::InvalidCallTarget
        | DiagnosticCode::InvalidBinaryOperation
        | DiagnosticCode::InvalidMethodCall
        | DiagnosticCode::NonExistentProperty
        | DiagnosticCode::PropertyTypeMismatch
        | DiagnosticCode::MissingProperty
        | DiagnosticCode::ArgumentCountMismatch
        | DiagnosticCode::ExtraProperty
        | DiagnosticCode::UnsupportedExpression
        | DiagnosticCode::UnsupportedOperator => DiagnosticCategory::Error,

        // Suggestions (no current diagnostic codes are suggestions)
        _ => DiagnosticCategory::Error, // Default to error for now
    }
}

/// Format a diagnostic message for display
/// Corresponds to FormatDiagnostic in internal/compiler/diagnostics.go
pub fn format_diagnostic(diagnostic: &Diagnostic) -> String {
    let category = get_category_from_code(diagnostic.code);
    let code_str = format!("TS{:04}", diagnostic.code as u32);

    // Format using TypeScript compiler's format: file(line,col): category TS#### : message
    if !diagnostic.file.is_empty() && diagnostic.line > 0 {
        format!(
            "{}({}:{}): {} {} : {}",
            diagnostic.file,
            diagnostic.line,
            diagnostic.column,
            category,
            code_str,
            diagnostic.message
        )
    } else {
        // Format without location information
        format!("{} {} : {}", category, code_str, diagnostic.message)
    }
}
