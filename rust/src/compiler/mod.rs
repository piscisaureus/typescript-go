// Corresponds to internal/compiler/program.go in the Go implementation

pub mod diagnostics;
pub mod fileloader;
pub mod host;
pub mod program_options;
pub mod utilities;

use crate::ast;
use crate::checker::TypeChecker;
use crate::error::Diagnostic;
use diagnostics::format_diagnostic;
use fileloader::FileLoader;
use host::{CompilerHost, DefaultCompilerHost};
use program_options::ProgramOptions;
use std::rc::Rc;
use utilities::sort_and_deduplicate_diagnostics;

/// Compiler manages compilation of TypeScript files
/// Corresponds to Program in internal/compiler/program.go
pub struct Program {
    root_files: Vec<Rc<ast::SourceFile>>, // Corresponds to rootFiles in Go implementation
    diagnostics: Vec<Diagnostic>,         // Corresponds to program.diagnostics in Go
    options: ProgramOptions,              // Corresponds to program.options in Go
    host: Box<dyn CompilerHost>,          // Corresponds to program.host in Go
    file_loader: FileLoader,              // Corresponds to program.fileLoader in Go
}

impl Program {
    /// Create a new program
    /// Corresponds to NewProgram() in internal/compiler/program.go
    pub fn new(options: ProgramOptions) -> Self {
        Self {
            root_files: Vec::new(),
            diagnostics: Vec::new(),
            options,
            host: Box::new(DefaultCompilerHost::new()),
            file_loader: FileLoader::new(),
        }
    }

    /// Create a new program with default options
    /// Helper constructor that's not in the Go implementation
    pub fn with_defaults() -> Self {
        Self::new(ProgramOptions::default())
    }

    /// Add a source file to the program
    /// Corresponds to part of program.Load() in internal/compiler/program.go
    pub fn add_source_file(&mut self, source_file: Rc<ast::SourceFile>) {
        // Clone diagnostics before moving the source file
        let diagnostics = source_file.diagnostics.clone();

        // Add the source file to the root files list
        self.root_files.push(source_file);

        // Add any diagnostics from the source file
        self.diagnostics.extend(diagnostics);
    }

    /// Load source files by path
    /// Corresponds to program.Load() in internal/compiler/program.go
    pub fn load(&mut self, file_paths: Vec<String>) -> std::result::Result<(), String> {
        for path in file_paths {
            // Read file content
            let content = match self.host.read_file(&path) {
                Ok(content) => content,
                Err(e) => {
                    return Err(format!("Error reading file {}: {}", path, e));
                }
            };

            // Parse the source file
            let source_file = match self.file_loader.load_source_file(&path, &content) {
                Ok(file) => file,
                Err(diag) => {
                    // Add the diagnostic and continue (don't abort on parse errors)
                    self.diagnostics.push(diag);
                    continue;
                }
            };

            // Add it to the program
            self.add_source_file(source_file);
        }

        Ok(())
    }

    /// Type-check the program
    /// Corresponds to part of program.Process() in internal/compiler/program.go
    pub fn type_check(&mut self) -> std::result::Result<(), String> {
        for source_file in &self.root_files {
            let mut checker = TypeChecker::new();
            match checker.check_source_file(Rc::clone(source_file)) {
                Ok(_) => {}
                Err(diag) => {
                    // Add the diagnostic and continue
                    self.diagnostics.push(diag);
                }
            }
            self.diagnostics.extend(checker.diagnostics);
        }
        Ok(())
    }

    /// Process (parse and type-check) the program
    /// Corresponds to program.Process() in internal/compiler/program.go
    pub fn process(&mut self) -> std::result::Result<(), String> {
        // Type checking is the only processing we currently do
        self.type_check()
    }

    /// Get the root source files
    /// Corresponds to GetSourceFiles() in internal/compiler/program.go
    pub fn get_source_files(&self) -> &[Rc<ast::SourceFile>] {
        &self.root_files
    }

    /// Get the diagnostics from the program
    /// Corresponds to GetDiagnostics() in internal/compiler/program.go
    pub fn get_diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Get sorted and deduplicated diagnostics
    /// Corresponds to GetSortedAndDeduplicatedDiagnostics() in internal/compiler/program.go
    pub fn get_sorted_diagnostics(&self) -> Vec<Diagnostic> {
        let mut sorted = self.diagnostics.clone();
        sort_and_deduplicate_diagnostics(&mut sorted);
        sorted
    }

    /// Format diagnostics for display
    /// Similar to what's done in the Go implementation
    pub fn format_diagnostics(&self) -> Vec<String> {
        let diagnostics = self.get_sorted_diagnostics();
        diagnostics
            .iter()
            .map(|diag| format_diagnostic(diag))
            .collect()
    }

    /// Get the program options
    /// Corresponds to program.Options() in internal/compiler/program.go
    pub fn options(&self) -> &ProgramOptions {
        &self.options
    }
}

/// Create a program from a source file
/// Helper function for tests and examples
/// Not directly in Go implementation, but similar to helper functions
/// in internal/testutil/harnessutil/harnessutil.go
pub fn create_program(source_file: Rc<ast::SourceFile>) -> Program {
    let mut program = Program::with_defaults();
    program.add_source_file(source_file);
    program
}
