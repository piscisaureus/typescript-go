// Corresponds to internal/compiler/fileloader.go in the Go implementation

use crate::ast;
use crate::error::Result;
use crate::parser;
use std::path::Path;
use std::rc::Rc;

/// FileLoader handles loading and parsing source files
/// This is a simplified version of the FileLoader in the Go implementation
pub struct FileLoader {}

impl FileLoader {
    /// Create a new file loader
    /// Corresponds to NewFileLoader() in internal/compiler/fileloader.go
    pub fn new() -> Self {
        Self {}
    }

    /// Load a source file by path
    /// This is a simplified version of loadSourceFile in internal/compiler/fileloader.go
    pub fn load_source_file(
        &self,
        file_path: &str,
        source_text: &str,
    ) -> Result<Rc<ast::SourceFile>> {
        // Get the file name from the path
        let file_name = Path::new(file_path)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        // Parse the source file
        parser::parse_source_file(&file_name, source_text)
    }
}
