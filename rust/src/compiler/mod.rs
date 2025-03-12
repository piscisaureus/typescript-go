// Corresponds to internal/compiler/program.go in the Go implementation

mod types;
pub use types::*;

use crate::ast::{self, Node};
use crate::error::{Diagnostic, DiagnosticCode, Result};
use std::rc::Rc;

/// Compiler manages compilation of TypeScript files
/// This is a simplified version that corresponds to Program in Go
pub struct Program {
    root_file: Option<Rc<ast::SourceFile>>,
    diagnostics: Vec<Diagnostic>,
}

impl Program {
    /// Create a new program
    /// Corresponds to NewProgram in Go
    pub fn new() -> Self {
        Self {
            root_file: None,
            diagnostics: Vec::new(),
        }
    }

    /// Add a source file to the program
    /// Corresponds to part of program.Load in Go
    pub fn add_source_file(&mut self, source_file: Rc<ast::SourceFile>) {
        // Clone diagnostics before moving the source file
        let diagnostics = source_file.diagnostics.clone();

        // For now, we just support a single file
        self.root_file = Some(source_file);

        // Add any diagnostics from the source file
        self.diagnostics.extend(diagnostics);
    }

    /// Type-check the program
    /// Corresponds to part of program.Process in Go
    pub fn type_check(&mut self) -> Result<()> {
        if let Some(source_file) = &self.root_file {
            let mut checker = TypeChecker::new();
            checker.check_source_file(Rc::clone(source_file))?;
            self.diagnostics.extend(checker.diagnostics);
        }
        Ok(())
    }

    /// Get the diagnostics from the program
    /// Corresponds to GetDiagnostics in Go
    pub fn get_diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}

/// Create a program from a source file
/// Helper function that corresponds to parts of various Go functions
pub fn create_program(source_file: Rc<ast::SourceFile>) -> Program {
    let mut program = Program::new();
    program.add_source_file(source_file);
    program
}

/// TypeChecker performs type checking on the AST
/// Corresponds to parts of checker.Checker in Go
pub struct TypeChecker {
    diagnostics: Vec<Diagnostic>,
}

impl TypeChecker {
    /// Create a new type checker
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    /// Check a source file for type errors
    pub fn check_source_file(&mut self, source_file: Rc<ast::SourceFile>) -> Result<()> {
        // Create a type context for this source file
        let mut context = TypeContext::new();

        // Process statements in the source file
        for statement in &source_file.statements {
            self.check_statement(&mut context, Rc::clone(statement))?;
        }

        Ok(())
    }

    /// Check a statement node
    fn check_statement(&mut self, context: &mut TypeContext, statement: Rc<dyn ast::Node>) -> Result<()> {
        match statement.kind() {
            ast::Kind::FunctionDeclaration => {
                // Downcast to FunctionDeclaration
                if let Some(func_decl) = statement.as_any().downcast_ref::<ast::FunctionDeclaration>() {
                    // Register function in the type context
                    let return_type = if let Some(return_type) = &func_decl.return_type {
                        self.get_type_from_node(return_type.clone())?
                    } else {
                        Type::Any
                    };

                    // Create parameter types
                    let mut param_types = Vec::new();
                    for param in &func_decl.parameters {
                        let param_type = if let Some(type_annotation) = &param.type_annotation {
                            self.get_type_from_node(type_annotation.clone())?
                        } else {
                            Type::Any
                        };
                        param_types.push(param_type);
                    }

                    // Create function signature
                    let signature = FunctionSignature {
                        parameters: param_types,
                        return_type: return_type,
                    };

                    // Add function to context
                    if let Some(name) = &func_decl.name {
                        context.add_function(name.text.clone(), signature.clone());
                    }

                    // Check function body if it exists
                    if let Some(body) = &func_decl.body {
                        // Create a new scope for function body
                        context.push_scope();

                        // Add parameters to scope
                        for (i, param) in func_decl.parameters.iter().enumerate() {
                            let param_type = signature.parameters.get(i).cloned().unwrap_or(Type::Any);
                            context.add_variable(param.name.text.clone(), param_type);
                        }

                        // Check body statements
                        for stmt in &body.statements {
                            self.check_statement(context, Rc::clone(stmt))?;
                        }

                        // Pop scope after processing function body
                        context.pop_scope();
                    }
                }
            }
            ast::Kind::ExpressionStatement => {
                // Downcast to ExpressionStatement
                if let Some(expr_stmt) = statement.as_any().downcast_ref::<ast::ExpressionStatement>() {
                    // Just check the expression
                    self.check_expression(context, Rc::clone(&expr_stmt.expression))?;
                }
            }
            ast::Kind::ReturnStatement => {
                // Downcast to ReturnStatement
                if let Some(return_stmt) = statement.as_any().downcast_ref::<ast::ReturnStatement>() {
                    // Check return expression if it exists
                    if let Some(expr) = &return_stmt.expression {
                        let expr_type = self.check_expression(context, Rc::clone(expr))?;
                        
                        // We would check that expr_type is compatible with function return type here
                        // For now, we'll just make sure it's not Error type
                        if expr_type == Type::Error {
                            self.diagnostics.push(Diagnostic::simple(
                                DiagnosticCode::TypeMismatch,
                                "Invalid return expression type".to_string(),
                                expr.pos(),
                                expr.end(),
                            ));
                        }
                    }
                }
            }
            _ => {
                // Unhandled statement type - for now, we do nothing
            }
        }

        Ok(())
    }

    /// Check an expression node and return its type
    fn check_expression(&mut self, context: &TypeContext, expression: Rc<dyn ast::Node>) -> Result<Type> {
        match expression.kind() {
            ast::Kind::CallExpression => {
                // Downcast to CallExpression
                if let Some(call_expr) = expression.as_any().downcast_ref::<ast::CallExpression>() {
                    // Get the function being called
                    let func_expr = &call_expr.expression;
                    
                    // For now, we only handle identifier function calls
                    if func_expr.kind() == ast::Kind::Identifier {
                        if let Some(ident) = func_expr.as_any().downcast_ref::<ast::Identifier>() {
                            // Look up function in context
                            if let Some(signature) = context.get_function(&ident.text) {
                                // Check argument count
                                if call_expr.arguments.len() != signature.parameters.len() {
                                    self.diagnostics.push(Diagnostic::simple(
                                        DiagnosticCode::ArgumentCountMismatch,
                                        format!(
                                            "Expected {} arguments but got {}",
                                            signature.parameters.len(),
                                            call_expr.arguments.len()
                                        ),
                                        call_expr.pos(),
                                        call_expr.end(),
                                    ));
                                    return Ok(Type::Error);
                                }

                                // Check each argument type
                                for (i, arg) in call_expr.arguments.iter().enumerate() {
                                    let arg_type = self.check_expression(context, Rc::clone(arg))?;
                                    let expected_type = &signature.parameters[i];
                                    
                                    if !self.is_assignable_to(&arg_type, expected_type) {
                                        self.diagnostics.push(Diagnostic::simple(
                                            DiagnosticCode::TypeMismatch,
                                            format!(
                                                "Argument of type {:?} is not assignable to parameter of type {:?}",
                                                arg_type,
                                                expected_type
                                            ),
                                            arg.pos(),
                                            arg.end(),
                                        ));
                                        return Ok(Type::Error);
                                    }
                                }

                                // Return function's return type
                                return Ok(signature.return_type);
                            } else {
                                self.diagnostics.push(Diagnostic::simple(
                                    DiagnosticCode::UndefinedFunction,
                                    format!("Cannot find function '{}'", ident.text),
                                    func_expr.pos(),
                                    func_expr.end(),
                                ));
                                return Ok(Type::Error);
                            }
                        }
                    }
                    
                    // For non-identifier function expressions
                    self.diagnostics.push(Diagnostic::simple(
                        DiagnosticCode::InvalidCallTarget,
                        "Invalid call target".to_string(),
                        func_expr.pos(),
                        func_expr.end(),
                    ));
                    return Ok(Type::Error);
                }
            }
            ast::Kind::BinaryExpression => {
                // Downcast to BinaryExpression
                if let Some(binary_expr) = expression.as_any().downcast_ref::<ast::BinaryExpression>() {
                    let left_type = self.check_expression(context, Rc::clone(&binary_expr.left))?;
                    let right_type = self.check_expression(context, Rc::clone(&binary_expr.right))?;

                    // Handle addition operator
                    if binary_expr.operator_token == ast::Kind::PlusToken {
                        if left_type == Type::String {
                            // String + anything is allowed (coerces to string)
                            return Ok(Type::String);
                        } else if left_type == Type::Number && right_type == Type::Number {
                            // Number + Number = Number
                            return Ok(Type::Number);
                        } else {
                            self.diagnostics.push(Diagnostic::simple(
                                DiagnosticCode::InvalidBinaryOperation,
                                format!(
                                    "Operator '+' cannot be applied to types {:?} and {:?}",
                                    left_type, right_type
                                ),
                                binary_expr.pos(),
                                binary_expr.end(),
                            ));
                            return Ok(Type::Error);
                        }
                    }

                    // Handle other operators as needed
                    // ...

                    // Default error case
                    self.diagnostics.push(Diagnostic::simple(
                        DiagnosticCode::UnsupportedOperator,
                        "Unsupported binary operator".to_string(),
                        binary_expr.pos(),
                        binary_expr.end(),
                    ));
                    return Ok(Type::Error);
                }
            }
            ast::Kind::Identifier => {
                // Downcast to Identifier
                if let Some(ident) = expression.as_any().downcast_ref::<ast::Identifier>() {
                    // Look up variable in context
                    if let Some(var_type) = context.get_variable(&ident.text) {
                        return Ok(var_type);
                    } else {
                        self.diagnostics.push(Diagnostic::simple(
                            DiagnosticCode::UndefinedVariable,
                            format!("Cannot find variable '{}'", ident.text),
                            ident.pos(),
                            ident.end(),
                        ));
                        return Ok(Type::Error);
                    }
                }
            }
            ast::Kind::StringLiteral => {
                return Ok(Type::String);
            }
            ast::Kind::NumericLiteral => {
                return Ok(Type::Number);
            }
            ast::Kind::TrueKeyword | ast::Kind::FalseKeyword => {
                return Ok(Type::Boolean);
            }
            ast::Kind::ArrayLiteralExpression => {
                if let Some(array_expr) = expression.as_any().downcast_ref::<ast::ArrayLiteralExpression>() {
                    // Check if array is empty
                    if array_expr.elements.is_empty() {
                        // Empty array - default to Any[]
                        return Ok(Type::Array(Box::new(Type::Any)));
                    }
                    
                    // Try to infer the element type from the first element
                    let first_elem_type = self.check_expression(context, Rc::clone(&array_expr.elements[0]))?;
                    
                    // Check if all elements have compatible types
                    let mut common_type = first_elem_type.clone();
                    for elem in &array_expr.elements[1..] {
                        let elem_type = self.check_expression(context, Rc::clone(elem))?;
                        
                        // If types don't match exactly, default to Any
                        if elem_type != common_type {
                            common_type = Type::Any;
                            break;
                        }
                    }
                    
                    return Ok(Type::Array(Box::new(common_type)));
                }
                
                // Fallback for any unexpected issues
                return Ok(Type::Array(Box::new(Type::Any)))
            }
            _ => {
                // Unhandled expression type
                self.diagnostics.push(Diagnostic::simple(
                    DiagnosticCode::UnsupportedExpression,
                    format!("Unsupported expression type: {:?}", expression.kind()),
                    expression.pos(),
                    expression.end(),
                ));
                return Ok(Type::Error);
            }
        }

        Ok(Type::Error)
    }

    /// Get a Type from a node representing a type annotation
    fn get_type_from_node(&self, node: Rc<dyn ast::Node>) -> Result<Type> {
        match node.kind() {
            ast::Kind::TypeReference => {
                if let Some(type_ref) = node.as_any().downcast_ref::<ast::TypeReference>() {
                    // Check if this is an array type (has [] at the end)
                    if type_ref.is_array_type {
                        // For any[] we create an Array type with Any as element type
                        let element_type = match type_ref.type_name.text.as_str() {
                            "string" => Type::String,
                            "number" => Type::Number,
                            "boolean" => Type::Boolean,
                            "any" => Type::Any,
                            _ => Type::Any, // Unknown type name - default to Any
                        };
                        return Ok(Type::Array(Box::new(element_type)));
                    }
                    
                    // Regular non-array types
                    match type_ref.type_name.text.as_str() {
                        "string" => Ok(Type::String),
                        "number" => Ok(Type::Number),
                        "boolean" => Ok(Type::Boolean),
                        "any" => Ok(Type::Any),
                        _ => Ok(Type::Any), // Unknown type name - default to Any
                    }
                } else {
                    Ok(Type::Any)
                }
            }
            _ => Ok(Type::Any), // Unhandled type annotation - default to Any
        }
    }

    /// Check if source_type is assignable to target_type
    fn is_assignable_to(&self, source_type: &Type, target_type: &Type) -> bool {
        // Any is assignable to and from anything
        if *source_type == Type::Any || *target_type == Type::Any {
            return true;
        }

        // Error is not assignable to anything
        if *source_type == Type::Error {
            return false;
        }

        // Same types are assignable
        if source_type == target_type {
            return true;
        }

        // Handle arrays
        match (source_type, target_type) {
            (Type::Array(src_elem_type), Type::Array(tgt_elem_type)) => {
                // Check element type compatibility
                self.is_assignable_to(src_elem_type, tgt_elem_type)
            }
            
            // Number is assignable to String due to coercion
            (Type::Number, Type::String) => true,
            
            // Boolean can be converted to String in JS
            (Type::Boolean, Type::String) => true,
            
            // Add more special cases as needed
            // ...
            
            // By default, different types are not assignable
            _ => false,
        }
    }
}
