// Corresponds to internal/checker/types.go in the Go implementation

use std::collections::HashMap;

/// Basic TypeScript type representation
/// Corresponds to the Type struct in Go
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Any,        // Any type (corresponds to 'any' in TypeScript)
    Error,      // Error type (used internally for type checking errors)
    String,     // String type
    Number,     // Number type
    Boolean,    // Boolean type
    Void,       // Void type
    Undefined,  // Undefined type
    Null,       // Null type
    Function,   // Function type (simplified, would have signature in full implementation)
}

/// Represents a function signature with parameters and return type
/// Corresponds to Signature in Go
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub parameters: Vec<Type>,
    pub return_type: Type,
}

/// Scope represents a variable and function scope during type checking
/// Similar to scope handling in Go's checker
#[derive(Debug, Default)]
struct Scope {
    variables: HashMap<String, Type>,
    functions: HashMap<String, FunctionSignature>,
}

/// TypeContext maintains type information during checking
/// Similar to checker context in Go
#[derive(Debug)]
pub struct TypeContext {
    scopes: Vec<Scope>,
}

impl TypeContext {
    /// Create a new type context
    pub fn new() -> Self {
        // Start with one global scope
        Self {
            scopes: vec![Scope::default()],
        }
    }

    /// Push a new scope onto the stack
    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    /// Pop the current scope off the stack
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    /// Add a variable to the current scope
    pub fn add_variable(&mut self, name: String, typ: Type) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.variables.insert(name, typ);
        }
    }

    /// Get a variable's type, searching from innermost to outermost scope
    pub fn get_variable(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(typ) = scope.variables.get(name) {
                return Some(typ.clone());
            }
        }
        None
    }

    /// Add a function to the current scope
    pub fn add_function(&mut self, name: String, signature: FunctionSignature) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.functions.insert(name, signature);
        }
    }

    /// Get a function's signature, searching from innermost to outermost scope
    pub fn get_function(&self, name: &str) -> Option<FunctionSignature> {
        for scope in self.scopes.iter().rev() {
            if let Some(signature) = scope.functions.get(name) {
                return Some(signature.clone());
            }
        }
        None
    }
}