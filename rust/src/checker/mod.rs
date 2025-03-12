// Corresponds to internal/checker in the Go implementation
// This is the root module for type checking functionality

pub mod checker;
pub mod relater;
pub mod scope;
pub mod types;

// Re-export key types for public API
pub use self::checker::TypeChecker;
pub use self::scope::TypeContext;
pub use self::types::{FunctionSignature, Type};
