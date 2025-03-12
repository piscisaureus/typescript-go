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
    /// In the Go implementation, these are likely loaded from lib.es5.d.ts and other declaration files
    /// instead of being hardcoded. We're implementing basic support here.
    pub fn add_built_ins(&mut self) {
        eprintln!("[DEBUG] Adding built-in types and functions to TypeContext");
        
        // Add the String function that converts values to strings
        // Similar to Go's String() builtin
        let string_params = vec![Type::Any];
        let string_signature = FunctionSignature {
            parameters: string_params,
            return_type: Type::String,
        };
        self.add_function("String".to_string(), string_signature);
        eprintln!("[DEBUG] Added String function");

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
        eprintln!("[DEBUG] Added console object with log method");
        
        // Register Array interface and its methods (similar to lib.es5.d.ts)
        // In Go, this would be loaded from the .d.ts files
        eprintln!("[DEBUG] Adding Array prototype methods");
        
        // Array.prototype.push method - takes element type, returns number
        let push_signature = FunctionSignature {
            parameters: vec![Type::Any], // Will be replaced with actual element type when used
            return_type: Type::Number,
        };
        eprintln!("[DEBUG] Created push signature: params={:?}, return={:?}", 
                 push_signature.parameters, push_signature.return_type);
        
        // Array.prototype.pop method - returns element type
        let pop_signature = FunctionSignature {
            parameters: vec![],
            return_type: Type::Any, // Will be replaced with actual element type when used
        };
        
        // Array.prototype.join method - takes string separator, returns string
        let join_signature = FunctionSignature {
            parameters: vec![Type::String],
            return_type: Type::String,
        };
        
        // Add all of these to a registry for array methods that can be referenced
        // when checking property access on arrays
        self.add_type("Array.prototype.push".to_string(), Type::Function(Box::new(push_signature)));
        eprintln!("[DEBUG] Added Array.prototype.push to types registry");
        
        self.add_type("Array.prototype.pop".to_string(), Type::Function(Box::new(pop_signature)));
        eprintln!("[DEBUG] Added Array.prototype.pop to types registry");
        
        self.add_type("Array.prototype.join".to_string(), Type::Function(Box::new(join_signature)));
        eprintln!("[DEBUG] Added Array.prototype.join to types registry");
        
        // Check if these types are really in our registry
        if let Some(push_type) = self.get_type("Array.prototype.push") {
            eprintln!("[DEBUG] Verified Array.prototype.push is in registry");
        } else {
            eprintln!("[DEBUG] ERROR: Array.prototype.push is NOT in registry!");
        }
        
        // Dump all types in registry for debugging
        eprintln!("[DEBUG] All registered types:");
        for (name, _) in &self.types {
            eprintln!("[DEBUG]   - {}", name);
        }
    }
}
