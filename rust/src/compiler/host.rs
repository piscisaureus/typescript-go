// Corresponds to internal/compiler/host.go in the Go implementation

use std::io;
use std::path::PathBuf;

/// CompilerHost defines an interface for interacting with the host environment
/// This corresponds to CompilerHost in internal/compiler/host.go
pub trait CompilerHost {
    /// Get the current working directory
    fn get_current_directory(&self) -> String;

    /// Check if a file exists
    fn file_exists(&self, path: &str) -> bool;

    /// Read a file from the filesystem
    fn read_file(&self, path: &str) -> io::Result<String>;

    /// Resolve a module name to a path
    fn resolve_module_name(&self, module_name: &str, base_path: &str) -> Option<String>;
}

/// DefaultCompilerHost provides default implementations of CompilerHost methods
pub struct DefaultCompilerHost {
    current_directory: String,
}

impl DefaultCompilerHost {
    /// Create a new default compiler host
    pub fn new() -> Self {
        let current_dir = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .to_string_lossy()
            .to_string();

        Self {
            current_directory: current_dir,
        }
    }
}

impl CompilerHost for DefaultCompilerHost {
    fn get_current_directory(&self) -> String {
        self.current_directory.clone()
    }

    fn file_exists(&self, path: &str) -> bool {
        std::path::Path::new(path).exists()
    }

    fn read_file(&self, path: &str) -> io::Result<String> {
        std::fs::read_to_string(path)
    }

    fn resolve_module_name(&self, module_name: &str, base_path: &str) -> Option<String> {
        // Simplified module resolution - just append .ts to the module name
        // In a real implementation, this would handle paths, node_modules, etc.
        let base_dir = std::path::Path::new(base_path).parent()?;
        let module_path = base_dir.join(format!("{}.ts", module_name));
        Some(module_path.to_string_lossy().to_string())
    }
}
