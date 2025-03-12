// Corresponds to internal/ast/diagnostic.go in the Go implementation

use std::fmt;

/// Diagnostic severity
/// Corresponds to diagnostics.Diagnostic.Severity in Go
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Suggestion,
}

/// Diagnostic message codes
/// Corresponds to diagnostics.Messages in Go (compiler/diagnostics/diagnostics.go)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCode {
    UnknownError,
    SyntaxError,
    UndefinedVariable,
    UndefinedFunction,
    TypeMismatch,
    ArgumentCountMismatch,
    InvalidCallTarget,
    InvalidBinaryOperation,
    UnsupportedOperator,
    UnsupportedExpression,
    MissingSemicolon,
    UnterminatedString,
    // Add more specific error codes as needed
}

/// Diagnostic represents a compiler error or warning
/// Corresponds to ast.Diagnostic in Go
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub message: String,
    pub file: String,
    pub pos: usize,
    pub len: usize,
    pub line: usize,
    pub column: usize,
    pub severity: DiagnosticSeverity,
}

impl Diagnostic {
    pub fn new(
        code: DiagnosticCode,
        message: &str,
        file: &str,
        pos: usize,
        len: usize,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            code,
            message: message.to_owned(),
            file: file.to_owned(),
            pos,
            len,
            line,
            column,
            severity: DiagnosticSeverity::Error,
        }
    }

    // Simpler constructor for type checking that doesn't require line/column info
    pub fn simple(
        code: DiagnosticCode,
        message: String,
        pos: usize,
        end: usize,
    ) -> Self {
        Self {
            code,
            message,
            file: "".to_owned(),
            pos,
            len: end - pos,
            line: 0,
            column: 0,
            severity: DiagnosticSeverity::Error,
        }
    }

    pub fn with_severity(mut self, severity: DiagnosticSeverity) -> Self {
        self.severity = severity;
        self
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} TS{:04}: {} (at {}:{}:{})",
            self.severity_text(),
            self.code as u32,
            self.message,
            self.file,
            self.line,
            self.column
        )
    }
}

impl Diagnostic {
    fn severity_text(&self) -> &'static str {
        match self.severity {
            DiagnosticSeverity::Error => "error",
            DiagnosticSeverity::Warning => "warning",
            DiagnosticSeverity::Information => "info",
            DiagnosticSeverity::Suggestion => "hint",
        }
    }
}

/// Result type for operations that can fail with a diagnostic
pub type Result<T> = std::result::Result<T, Diagnostic>;

/// Creates a syntax error diagnostic
pub fn syntax_error(
    message: &str,
    file: &str,
    pos: usize,
    len: usize,
    line: usize,
    column: usize,
) -> Diagnostic {
    Diagnostic::new(
        DiagnosticCode::SyntaxError,
        message,
        file,
        pos,
        len,
        line,
        column,
    )
}
