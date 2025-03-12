// Corresponds to internal/ast/diagnostic.go in the Go implementation

use std::fmt;

/// Diagnostic severity
/// Corresponds to diagnostics.Diagnostic.Severity in Go
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    // These variants are in the Go implementation but not used in Rust
    _Warning,
    _Information,
    _Suggestion,
}

/// Diagnostic message codes
/// Corresponds to diagnostics.Messages in Go (compiler/diagnostics/diagnostics.go)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCode {
    // The following are used in the Rust implementation
    // UnknownError, // Not currently used
    SyntaxError,
    UndefinedVariable,
    UndefinedFunction,
    TypeMismatch,
    ArgumentCountMismatch,
    InvalidCallTarget,
    InvalidBinaryOperation,
    UnsupportedOperator,
    UnsupportedExpression,
    // The following are in the Go implementation but not used in Rust
    _UnknownError,
    _MissingSemicolon,
    _UnterminatedString,
    MissingProperty,
    ExtraProperty,
    PropertyTypeMismatch,
    NonExistentProperty,
    InvalidMethodCall,
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
    // Accessor for end position (pos + len)
    // This corresponds to Go implementation but isn't used in Rust
    pub fn _end(&self) -> usize {
        self.pos + self.len
    }
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

    // Constructor for type checking that computes line/column from absolute position
    pub fn simple(code: DiagnosticCode, message: String, pos: usize, end: usize) -> Self {
        // Calculate line and column based on the position
        // This is a simplified implementation - in production, use a proper line map
        let (line, column) = Self::compute_line_column("", pos);

        Self {
            code,
            message,
            file: "".to_owned(), // File name needs to be set by the caller
            pos,
            len: end - pos,
            line,
            column,
            severity: DiagnosticSeverity::Error,
        }
    }

    // Add file information to a diagnostic
    // Corresponds to Go implementation, renamed to indicate it's used in tests
    pub fn with_file(mut self, file_name: &str) -> Self {
        self.file = file_name.to_owned();
        self
    }

    // Update line/column information using source text
    // Corresponds to Go implementation, renamed to indicate it's used in tests
    pub fn with_source_text(mut self, source_text: &str) -> Self {
        let (line, column) = Self::compute_line_column(source_text, self.pos);
        self.line = line;
        self.column = column;
        self
    }

    // Helper method to compute line and column from position and source text
    // In a real implementation, this would use the source file's line map
    pub fn compute_line_column(source: &str, pos: usize) -> (usize, usize) {
        // Default values if we can't compute
        if source.is_empty() {
            return (1, pos + 1); // 1-based line and column
        }

        // Safety check - if pos is out of bounds, return the last position
        if pos >= source.len() {
            // Calculate for the end of the file
            let mut line = 1;
            let mut line_start = 0;
            let mut last_pos = 0;

            for (i, c) in source.char_indices() {
                last_pos = i;
                if c == '\n' {
                    line += 1;
                    line_start = i + 1;
                }
            }

            // Position at the end of the last line
            let column = last_pos - line_start + 2; // +2 to account for the last character
            return (line, column);
        }

        let mut line = 1;
        let mut line_start = 0;

        for (i, c) in source.char_indices() {
            if i >= pos {
                break;
            }

            if c == '\n' {
                line += 1;
                line_start = i + 1;
            }
        }

        let column = pos - line_start + 1; // 1-based column
        (line, column)
    }

    // Corresponds to Go implementation but not used in Rust
    pub fn _with_severity(mut self, severity: DiagnosticSeverity) -> Self {
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
            DiagnosticSeverity::_Warning => "warning",
            DiagnosticSeverity::_Information => "info",
            DiagnosticSeverity::_Suggestion => "hint",
        }
    }
}

/// Result type for operations that can fail with a diagnostic
pub type Result<T> = std::result::Result<T, Diagnostic>;

/// Creates a syntax error diagnostic - corresponds to Go implementation but not used in Rust
pub fn _syntax_error(
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
