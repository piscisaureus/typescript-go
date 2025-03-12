// Corresponds to program options handling in internal/compiler/program.go

/// ProgramOptions defines options for the TypeScript compiler
/// This is a simplified version of options in the Go implementation
#[derive(Debug, Clone)]
pub struct ProgramOptions {
    /// Root file names to process
    pub root_files: Vec<String>,

    /// Whether to emit JavaScript output
    pub emit_js: bool,

    /// Whether to emit declaration files
    pub emit_declaration: bool,

    /// Target ECMAScript version
    pub target: TargetVersion,

    /// Module system to use
    pub module: ModuleKind,
    
    /// Disable loading of standard library files (lib.*.d.ts)
    pub no_lib: bool,
    
    /// List of specific library files to include (when no_lib is false)
    pub lib: Vec<String>,
}

/// Target ECMAScript version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetVersion {
    ES3,
    ES5,
    ES2015,
    ES2016,
    ES2017,
    ES2018,
    ES2019,
    ES2020,
    ESNext,
}

/// Module system to use
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleKind {
    None,
    CommonJS,
    AMD,
    UMD,
    System,
    ES2015,
    ESNext,
}

impl Default for ProgramOptions {
    fn default() -> Self {
        Self {
            root_files: Vec::new(),
            emit_js: true,
            emit_declaration: false,
            target: TargetVersion::ES2020,
            module: ModuleKind::None,
            no_lib: false,
            lib: Vec::new(), // Empty means use default libs for the target
        }
    }
}
