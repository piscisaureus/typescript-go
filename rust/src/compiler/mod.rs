// Corresponds to internal/compiler/program.go in the Go implementation

use crate::ast;
use crate::error::{Diagnostic, Result};
use std::rc::Rc;

/// Compiler manages compilation of TypeScript files
/// This is a simplified version that corresponds to Program in Go
pub struct Program {
    root_file: Option<Rc<ast::SourceFile>>,
    diagnostics: Vec<Diagnostic>,
}

impl Program {
    /// Create a new program
    /// Corresponds to NewProgram in Go
    pub fn new() -> Self {
        Self {
            root_file: None,
            diagnostics: Vec::new(),
        }
    }

    /// Add a source file to the program
    /// Corresponds to part of program.Load in Go
    pub fn add_source_file(&mut self, source_file: Rc<ast::SourceFile>) {
        // Clone diagnostics before moving the source file
        let diagnostics = source_file.diagnostics.clone();

        // For now, we just support a single file
        self.root_file = Some(source_file);

        // Add any diagnostics from the source file
        self.diagnostics.extend(diagnostics);
    }

    /// Type-check the program
    /// Corresponds to part of program.Process in Go
    pub fn type_check(&mut self) -> Result<()> {
        // This would be a more complete implementation for type checking
        // For now, we just return Ok
        Ok(())
    }

    /// Get the diagnostics from the program
    /// Corresponds to GetDiagnostics in Go
    pub fn get_diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}

/// Create a program from a source file
/// Helper function that corresponds to parts of various Go functions
pub fn create_program(source_file: Rc<ast::SourceFile>) -> Program {
    let mut program = Program::new();
    program.add_source_file(source_file);
    program
}
