// Corresponds to internal/checker/types.go in the Go implementation
// This module defines the TypeScript type system

// Type represents a TypeScript type
// Corresponds to types.Type interface in internal/checker/types.go
// In Go, this is defined as an interface with different implementations for each type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Any,                                 // Corresponds to types.AnyType in Go
    Error,                               // Corresponds to types.ErrorType in Go
    String,                              // Corresponds to types.StringType in Go
    Number,                              // Corresponds to types.NumberType in Go
    Boolean,                             // Corresponds to types.BooleanType in Go
    _Void,                               // Corresponds to types.VoidType in Go (not currently used)
    _Undefined,       // Corresponds to types.UndefinedType in Go (not currently used)
    _Null,            // Corresponds to types.NullType in Go (not currently used)
    Array(Box<Type>), // Corresponds to types.ArrayType in Go
    Object(Option<Vec<(String, Type)>>), // Corresponds to types.ObjectType in Go
    // None for empty object, Some for object with properties
    Interface(String, Vec<(String, Type)>), // Corresponds to types.InterfaceType in Go
    // String is interface name, Vec contains properties
    TypeParameter(String), // Type parameter like T in Array<T>
    GenericInterface(String, Vec<String>, Vec<(String, Type)>), // Generic interface like Array<T>
    // Fields: name, type_parameters, properties
    Function(Box<FunctionSignature>), // Corresponds to types.FunctionType in Go
    Union(Vec<Type>),                 // Corresponds to types.UnionType in Go
                                      // Union type (e.g., string | number)
}

// FunctionSignature represents a function's type signature
// Corresponds to types.Signature in internal/checker/types.go
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionSignature {
    pub parameters: Vec<Type>,         // Corresponds to params field in types.Signature
    pub return_type: Type,             // Corresponds to result field in types.Signature
    pub type_parameters: Vec<String>,  // For generic functions like function foo<T>(param: T): T
}
