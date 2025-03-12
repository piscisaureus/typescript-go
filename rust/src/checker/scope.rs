// Corresponds to internal/checker/scope.go in the Go implementation

use crate::checker::types::*;
use std::collections::HashMap;

/// TypeContext manages variable and function types
/// Corresponds to scope.Scope in internal/checker/scope.go
#[derive(Debug, Clone)]
pub struct TypeContext {
    // Each scope level has its own HashMap of names to types
    // In Go, this is managed differently with a linked list of scopes
    scopes: Vec<HashMap<String, Type>>, // Corresponds to vars map in scope.Scope

    // Functions have their own scope
    functions: HashMap<String, FunctionSignature>, // Corresponds to funcs map in scope.Scope

    // Types for interfaces, etc.
    types: HashMap<String, Type>, // Corresponds to types map in scope.Scope

    // Map of interface names to their properties
    interfaces: HashMap<String, Vec<(String, Type)>>, // Corresponds to interfaces map in scope.Scope
}

impl TypeContext {
    // Create a new type context
    // Corresponds to NewScope() in internal/checker/scope.go
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()], // Start with one global scope
            functions: HashMap::new(),
            types: HashMap::new(),
            interfaces: HashMap::new(),
        }
    }

    // Add a variable to the current scope
    // Corresponds to scope.AddVariable() in internal/checker/scope.go
    pub fn add_variable(&mut self, name: String, typ: Type) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, typ);
        }
    }

    // Look up a variable's type
    // Corresponds to scope.LookupVariable() in internal/checker/scope.go
    pub fn get_variable(&self, name: &str) -> Option<Type> {
        // Look through scopes from innermost to outermost
        // This matches the Go implementation's linked list traversal
        for scope in self.scopes.iter().rev() {
            if let Some(typ) = scope.get(name) {
                return Some(typ.clone());
            }
        }
        None
    }

    // Add a function to the context
    // Corresponds to scope.AddFunction() in internal/checker/scope.go
    pub fn add_function(&mut self, name: String, signature: FunctionSignature) {
        self.functions.insert(name, signature);
    }

    // Look up a function's signature
    // Corresponds to scope.LookupFunction() in internal/checker/scope.go
    pub fn get_function(&self, name: &str) -> Option<FunctionSignature> {
        self.functions.get(name).cloned()
    }

    // Add a type to the context
    // Corresponds to scope.AddType() in internal/checker/scope.go
    pub fn add_type(&mut self, name: String, typ: Type) {
        self.types.insert(name, typ);
    }

    // Look up a type
    // Corresponds to scope.LookupType() in internal/checker/scope.go
    pub fn get_type(&self, name: &str) -> Option<Type> {
        self.types.get(name).cloned()
    }

    // Add an interface to the context
    // Corresponds to scope.AddInterface() in internal/checker/scope.go
    pub fn add_interface(&mut self, name: String, properties: Vec<(String, Type)>) {
        self.interfaces.insert(name, properties);
    }

    // Look up an interface's properties
    // Corresponds to scope.LookupInterface() in internal/checker/scope.go
    pub fn get_interface(&self, name: &str) -> Option<Vec<(String, Type)>> {
        self.interfaces.get(name).cloned()
    }

    // Push a new scope onto the stack
    // Corresponds to scope.PushScope() in internal/checker/scope.go
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    // Pop the innermost scope from the stack
    // Corresponds to scope.PopScope() in internal/checker/scope.go
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    /// Initialize with built-in functions and types
    /// Corresponds to scope.initializeBuiltins() in internal/checker/scope.go
    /// Extended with console object support which isn't in the Go implementation
    /// but is useful for testing purposes
    pub fn add_built_ins(&mut self) {
        // Add the String function that converts values to strings
        // Similar to Go's String() builtin
        let string_params = vec![Type::Any];
        let string_signature = FunctionSignature {
            parameters: string_params,
            return_type: Type::String,
        };
        self.add_function("String".to_string(), string_signature);

        // Add the console object for logging
        // Note: This is not in the Go implementation, added for testing purposes
        let log_signature = FunctionSignature {
            parameters: vec![Type::Any], // console.log can take any arguments
            return_type: Type::Any,
        };

        // Make console.log function available
        let console_log_func = Type::Function(Box::new(log_signature));

        // Create console object properties
        let console_props = vec![("log".to_string(), console_log_func)];

        // Create console object
        let console_type = Type::Object(Some(console_props));

        // Register console in global scope
        self.add_variable("console".to_string(), console_type);
    }
}
