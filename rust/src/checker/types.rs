// Corresponds to internal/checker/types.go in the Go implementation
// This module defines the types and type context for the type checker

use std::collections::HashMap;

// Type represents a TypeScript type
// Corresponds to types.Type in internal/checker/types.go
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Any,
    Error,
    String,
    Number,
    Boolean,
    _Void,      // Not currently used
    _Undefined, // Not currently used
    _Null,      // Not currently used
    Array(Box<Type>),
    Object(Option<Vec<(String, Type)>>), // None for empty object, Some for object with properties
    Interface(String, Vec<(String, Type)>), // Name, properties
    Function(Box<FunctionSignature>),
}

// FunctionSignature represents a function's type signature
// Corresponds to types.Signature in Go
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    pub parameters: Vec<Type>,
    pub return_type: Type,
}

// TypeContext manages variable and function types
// Loosely corresponds to scope.Scope in internal/checker/scope.go
pub struct TypeContext {
    // Each scope level has its own HashMap of names to types
    scopes: Vec<HashMap<String, Type>>,
    // Functions have their own scope
    functions: HashMap<String, FunctionSignature>,
    // Types for interfaces, etc.
    types: HashMap<String, Type>,
    // Map of interface names to their properties
    interfaces: HashMap<String, Vec<(String, Type)>>,
}

impl TypeContext {
    // Create a new type context
    // Corresponds to NewScope in Go
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()], // Start with one global scope
            functions: HashMap::new(),
            types: HashMap::new(),
            interfaces: HashMap::new(),
        }
    }

    // Add a variable to the current scope
    // Corresponds to scope.AddVariable in Go
    pub fn add_variable(&mut self, name: String, typ: Type) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, typ);
        }
    }

    // Look up a variable's type
    // Corresponds to scope.LookupVariable in Go
    pub fn get_variable(&self, name: &str) -> Option<Type> {
        // Look through scopes from innermost to outermost
        for scope in self.scopes.iter().rev() {
            if let Some(typ) = scope.get(name) {
                return Some(typ.clone());
            }
        }
        None
    }

    // Add a function to the context
    // Corresponds to scope.AddFunction in Go
    pub fn add_function(&mut self, name: String, signature: FunctionSignature) {
        self.functions.insert(name, signature);
    }

    // Look up a function's signature
    // Corresponds to scope.LookupFunction in Go
    pub fn get_function(&self, name: &str) -> Option<FunctionSignature> {
        self.functions.get(name).cloned()
    }

    // Add a type to the context
    // No direct correspondence in Go
    pub fn add_type(&mut self, name: String, typ: Type) {
        self.types.insert(name, typ);
    }

    // Look up a type
    // No direct correspondence in Go
    pub fn get_type(&self, name: &str) -> Option<Type> {
        self.types.get(name).cloned()
    }

    // Add an interface to the context
    // Corresponds to scope.AddInterface in Go
    pub fn add_interface(&mut self, name: String, properties: Vec<(String, Type)>) {
        self.interfaces.insert(name, properties);
    }

    // Look up an interface's properties
    // Corresponds to scope.LookupInterface in Go
    pub fn get_interface(&self, name: &str) -> Option<Vec<(String, Type)>> {
        self.interfaces.get(name).cloned()
    }

    // Push a new scope onto the stack
    // Corresponds to scope.PushScope in Go
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    // Pop the innermost scope from the stack
    // Corresponds to scope.PopScope in Go
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }
}
