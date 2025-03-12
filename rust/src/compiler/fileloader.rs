// Corresponds to internal/compiler/fileloader.go in the Go implementation

use crate::ast;
use crate::ast::NodeFlags;
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

    /// Determines if a file is a declaration file by checking its extension
    /// Similar to isDeclarationFileName in the Go implementation
    fn is_declaration_file(&self, file_path: &str) -> bool {
        file_path.ends_with(".d.ts") || file_path.ends_with(".d.mts") || file_path.ends_with(".d.cts")
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

        // Check if this is a declaration file
        let is_declaration_file = self.is_declaration_file(file_path);
        
        // Parse the source file
        let mut source_file = parser::parse_source_file(&file_name, source_text)?;
        
        // If this is a declaration file, set the appropriate flags
        if is_declaration_file {
            println!("Loaded declaration file: {}", file_path);
            
            // Create a new SourceFile with the IsDeclarationFile and Ambient flags
            // Since Rc is immutable, we need to clone the SourceFile to modify it
            let mut cloned = (*source_file).clone();
            cloned.base.flags = cloned.base.flags | NodeFlags::IsDeclarationFile | NodeFlags::Ambient;
            
            // Replace our source_file with the modified one
            source_file = Rc::new(cloned);
        }
        
        Ok(source_file)
    }
    
    /// Loads standard library declaration files
    /// Similar to loadStandardLibraryFiles in the Go implementation
    pub fn load_standard_library(&self) -> Vec<Result<Rc<ast::SourceFile>>> {
        let mut result = Vec::new();
        
        // Check for lib.es5.d.ts in the current directory (simplification)
        let lib_path = Path::new("lib.es5.d.ts");
        if lib_path.exists() {
            match std::fs::read_to_string(lib_path) {
                Ok(content) => {
                    match self.load_source_file(lib_path.to_str().unwrap(), &content) {
                        Ok(source_file) => result.push(Ok(source_file)),
                        Err(err) => result.push(Err(err)),
                    }
                }
                Err(_) => {
                    // Silently ignore errors reading standard library files
                    // In a real implementation, we might want to log this
                }
            }
        }
        
        // Check for lib.array.simple.d.ts as well (our temporary implementation)
        let array_lib_path = Path::new("lib.array.simple.d.ts");
        if array_lib_path.exists() {
            match std::fs::read_to_string(array_lib_path) {
                Ok(content) => {
                    match self.load_source_file(array_lib_path.to_str().unwrap(), &content) {
                        Ok(source_file) => result.push(Ok(source_file)),
                        Err(err) => result.push(Err(err)),
                    }
                }
                Err(_) => {
                    // Silently ignore errors reading standard library files
                }
            }
        }
        
        result
    }
}
