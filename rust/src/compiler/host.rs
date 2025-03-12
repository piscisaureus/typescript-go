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
        // Following the Go implementation's priority: .ts, .d.ts, then .js
        let base_dir = std::path::Path::new(base_path).parent()?;
        
        // First try .ts extension
        let ts_path = base_dir.join(format!("{}.ts", module_name));
        if ts_path.exists() {
            return Some(ts_path.to_string_lossy().to_string());
        }
        
        // Then try .d.ts extension (declaration files)
        let dts_path = base_dir.join(format!("{}.d.ts", module_name));
        if dts_path.exists() {
            return Some(dts_path.to_string_lossy().to_string());
        }
        
        // Finally try .js extension (only if allowJs is true, but we'll allow it for now)
        let js_path = base_dir.join(format!("{}.js", module_name));
        if js_path.exists() {
            return Some(js_path.to_string_lossy().to_string());
        }
        
        // If still not found, default to .ts for error messages
        Some(ts_path.to_string_lossy().to_string())
    }
}
