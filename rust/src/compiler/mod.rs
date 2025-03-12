// Corresponds to internal/compiler/program.go in the Go implementation

use crate::ast::{self, Node};
mod types;
use crate::error::{Diagnostic, DiagnosticCode, Result};
use std::rc::Rc;
pub use types::*;

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
    source_text: String,  // Source text for computing line/column info
    file_name: String,    // File name for diagnostics
    line_map: Vec<usize>, // Line starts map for computing locations
}

impl TypeChecker {
    /// Create a new type checker
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
            source_text: String::new(),
            file_name: String::new(),
            line_map: Vec::new(),
        }
    }

    /// Check an object binding pattern against the initializer type
    /// This validates that all properties in the pattern exist on the initializer type
    fn check_object_binding_pattern(
        &mut self,
        context: &mut TypeContext,
        pattern: &ast::ObjectBindingPattern,
        init_type: &Type,
    ) -> Result<()> {
        // Extract properties from the initializer type
        let properties = match init_type {
            Type::Object(Some(props)) => props.clone(),
            Type::Interface(_, props) => props.clone(),
            _ => {
                // Not an object type - error already reported
                return Ok(());
            }
        };

        // Check each binding element
        for element in &pattern.elements {
            // Get the property name from the pattern
            let property_name = if let Some(prop_name) = &element.property_name {
                // { x: y } form - property name is 'x'
                prop_name.text.clone()
            } else {
                // { x } form - property name is same as binding name
                element.name.text.clone()
            };

            // Check if property exists on the initializer type
            if let Some((_, prop_type)) = properties.iter().find(|(name, _)| name == &property_name)
            {
                // Property exists, add the binding to the context
                context.add_variable(element.name.text.clone(), prop_type.clone());

                // If element has its own initializer, check type compatibility
                if let Some(element_init) = &element.initializer {
                    let init_type = self.check_expression(context, Rc::clone(element_init))?;

                    if !self.is_assignable_to(&init_type, prop_type) {
                        self.diagnostics.push(self.create_diagnostic(
                            DiagnosticCode::TypeMismatch,
                            format!(
                                "Type '{}' is not assignable to type '{}'",
                                self.format_type(&init_type),
                                self.format_type(prop_type)
                            ),
                            element_init.pos(),
                            element_init.end(),
                        ));
                    }
                }
            } else {
                // Property doesn't exist on the type - report an error
                self.diagnostics.push(self.create_diagnostic(
                    DiagnosticCode::NonExistentProperty,
                    format!(
                        "Property '{}' does not exist on type '{}'",
                        property_name,
                        self.format_type(init_type)
                    ),
                    element.pos(),
                    element.end(),
                ));
            }
        }

        Ok(())
    }

    fn create_diagnostic(
        &self,
        code: DiagnosticCode,
        message: String,
        pos: usize,
        end: usize,
    ) -> Diagnostic {
        // Use the provided positions without any special case handling
        // This approach is cleaner and will apply generally rather than having special cases for tests
        let (adjusted_pos, adjusted_end) = (pos, end);

        // Create a basic diagnostic
        let mut diag = Diagnostic::simple(code, message, adjusted_pos, adjusted_end);

        // Add file name
        diag.file = self.file_name.clone();

        // Calculate line and column using the line map
        if !self.line_map.is_empty() {
            // Find the line by binary search
            let line_index = match self.line_map.binary_search(&adjusted_pos) {
                Ok(exact) => exact,
                Err(insertion) => {
                    if insertion == 0 {
                        0 // Handle positions before the first line start
                    } else {
                        insertion - 1
                    }
                }
            };

            // Calculate 1-based line and column
            let line = line_index + 1; // 1-based line number
            let column = adjusted_pos - self.line_map[line_index] + 1; // 1-based column

            diag.line = line;
            diag.column = column;
        } else if !self.source_text.is_empty() {
            // Fall back to the old algorithm if no line map
            let (line, column) = Diagnostic::compute_line_column(&self.source_text, adjusted_pos);
            diag.line = line;
            diag.column = column;
        }

        diag
    }

    /// Initialize with built-in functions and types
    /// Corresponds to scope.initializeBuiltins in internal/checker/scope.go
    /// Extended with console object support which isn't in the Go implementation
    fn add_built_ins(&self, context: &mut TypeContext) {
        // Add the String function that converts values to strings
        // Similar to Go's String() builtin
        let string_params = vec![Type::Any];
        let string_signature = FunctionSignature {
            parameters: string_params,
            return_type: Type::String,
        };
        context.add_function("String".to_string(), string_signature);

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
        context.add_variable("console".to_string(), console_type);
    }

    /// Check a source file for type errors
    pub fn check_source_file(&mut self, source_file: Rc<ast::SourceFile>) -> Result<()> {
        // Store source text and file name for diagnostics
        self.source_text = source_file.text.clone();
        self.file_name = source_file.file_name.clone();

        // Build a line map from the source text for better location computation
        self.line_map = if source_file.line_map.is_empty() {
            // Build our own if the source file doesn't provide one
            let mut map = vec![0];
            for (i, c) in self.source_text.char_indices() {
                if c == '\n' {
                    map.push(i + 1);
                }
            }
            map
        } else {
            source_file.line_map.clone()
        };

        // Create a type context for this source file
        let mut context = TypeContext::new();

        // Add built-in functions and types
        self.add_built_ins(&mut context);

        // First pass: Register interfaces
        self.register_interfaces(&mut context, &source_file)?;

        // Second pass: Create context with function declarations
        self.register_declarations(&mut context, &source_file)?;

        // Third pass: Check all statements
        for statement in &source_file.statements {
            self.check_statement(&mut context, Rc::clone(statement))?;
        }

        Ok(())
    }

    /// Register all interface declarations in the source file
    fn register_interfaces(
        &mut self,
        context: &mut TypeContext,
        source_file: &ast::SourceFile,
    ) -> Result<()> {
        // Register all interface declarations
        for statement in &source_file.statements {
            if statement.kind() == ast::Kind::InterfaceDeclaration {
                if let Some(interface_decl) = statement
                    .as_any()
                    .downcast_ref::<ast::InterfaceDeclaration>()
                {
                    let interface_name = interface_decl.name.text.clone();
                    let mut properties = Vec::new();

                    // Extract properties from the interface
                    for member in &interface_decl.members {
                        if let Some(prop_sig) =
                            member.as_any().downcast_ref::<ast::PropertySignature>()
                        {
                            let prop_name = prop_sig.name.text.clone();
                            let prop_type =
                                self.get_type_from_node(Rc::clone(&prop_sig.type_annotation))?;
                            properties.push((prop_name, prop_type));
                        }
                    }

                    // Add interface to type context
                    context.add_interface(interface_name.clone(), properties.clone());

                    // Also register it as a named type
                    let interface_type = Type::Interface(interface_name, properties);
                    context.add_type(interface_decl.name.text.clone(), interface_type);
                }
            }
        }

        Ok(())
    }

    /// Register all declarations in the source file
    fn register_declarations(
        &mut self,
        context: &mut TypeContext,
        source_file: &ast::SourceFile,
    ) -> Result<()> {
        // First pass: Register all function declarations
        for statement in &source_file.statements {
            if statement.kind() == ast::Kind::FunctionDeclaration {
                if let Some(func_decl) = statement
                    .as_any()
                    .downcast_ref::<ast::FunctionDeclaration>()
                {
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

                    // Get return type
                    let return_type = if let Some(return_type) = &func_decl.return_type {
                        self.get_type_from_node(return_type.clone())?
                    } else {
                        Type::Any
                    };

                    // Create function signature
                    let signature = FunctionSignature {
                        parameters: param_types,
                        return_type,
                    };

                    // Add function to context
                    if let Some(name) = &func_decl.name {
                        context.add_function(name.text.clone(), signature.clone());

                        // Also add the function as a variable of function type
                        context
                            .add_variable(name.text.clone(), Type::Function(Box::new(signature)));
                    }
                }
            }
        }

        Ok(())
    }

    /// Check a statement node
    fn check_statement(
        &mut self,
        context: &mut TypeContext,
        statement: Rc<dyn ast::Node>,
    ) -> Result<()> {
        match statement.kind() {
            ast::Kind::InterfaceDeclaration => {
                // Interface declarations are already handled in register_interfaces
                // No need for additional checking here
            }
            ast::Kind::FunctionDeclaration => {
                // Downcast to FunctionDeclaration
                if let Some(func_decl) = statement
                    .as_any()
                    .downcast_ref::<ast::FunctionDeclaration>()
                {
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
                            let param_type =
                                signature.parameters.get(i).cloned().unwrap_or(Type::Any);
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
                if let Some(expr_stmt) = statement
                    .as_any()
                    .downcast_ref::<ast::ExpressionStatement>()
                {
                    // Just check the expression
                    self.check_expression(context, Rc::clone(&expr_stmt.expression))?;
                }
            }
            ast::Kind::ReturnStatement => {
                // Downcast to ReturnStatement
                if let Some(return_stmt) = statement.as_any().downcast_ref::<ast::ReturnStatement>()
                {
                    // Check return expression if it exists
                    if let Some(expr) = &return_stmt.expression {
                        let expr_type = self.check_expression(context, Rc::clone(expr))?;

                        // We would check that expr_type is compatible with function return type here
                        // For now, we'll just make sure it's not Error type
                        if expr_type == Type::Error {
                            self.diagnostics.push(self.create_diagnostic(
                                DiagnosticCode::TypeMismatch,
                                "Invalid return expression type".to_string(),
                                expr.pos(),
                                expr.end(),
                            ));
                        }
                    }
                }
            }
            ast::Kind::VariableStatement => {
                // Downcast to VariableStatement
                if let Some(var_stmt) = statement.as_any().downcast_ref::<ast::VariableStatement>()
                {
                    // Check each declaration in the declaration list
                    if let Some(decl_list) = var_stmt
                        .declaration_list
                        .as_any()
                        .downcast_ref::<ast::VariableDeclarationList>()
                    {
                        for decl in &decl_list.declarations {
                            // Check if this is a destructuring pattern
                            if let Some(binding_pattern) = &decl.binding_name {
                                // Handle object binding pattern (destructuring)
                                if binding_pattern.kind() == ast::Kind::ObjectBindingPattern {
                                    if let Some(obj_pattern) = binding_pattern
                                        .as_any()
                                        .downcast_ref::<ast::ObjectBindingPattern>(
                                    ) {
                                        // Object binding patterns must have an initializer
                                        if let Some(initializer) = &decl.initializer {
                                            // Get type of the initializer
                                            let init_type = self.check_expression(
                                                context,
                                                Rc::clone(initializer),
                                            )?;

                                            // Check if the initializer has an object-like type
                                            match &init_type {
                                                Type::Object(_) | Type::Interface(_, _) => {
                                                    // Process the object binding pattern
                                                    self.check_object_binding_pattern(
                                                        context,
                                                        obj_pattern,
                                                        &init_type,
                                                    )?;
                                                }
                                                _ => {
                                                    // Invalid type for object destructuring
                                                    self.diagnostics.push(self.create_diagnostic(
                                                        DiagnosticCode::TypeMismatch,
                                                        format!("Cannot destructure type '{}' as it is not an object type", self.format_type(&init_type)),
                                                        initializer.pos(),
                                                        initializer.end(),
                                                    ));
                                                }
                                            }

                                            // Continue to next declaration
                                            continue;
                                        } else {
                                            // Error: Destructuring declarations require initializers
                                            self.diagnostics.push(self.create_diagnostic(
                                                DiagnosticCode::SyntaxError,
                                                "Object destructuring patterns require an initializer".to_string(),
                                                binding_pattern.pos(),
                                                binding_pattern.end(),
                                            ));
                                            continue;
                                        }
                                    }
                                }
                            }

                            // Regular variable declaration (not destructuring)
                            let mut var_type = Type::Any;

                            // Check type annotation if present
                            if let Some(type_anno) = &decl.type_annotation {
                                var_type = self.get_type_from_node(type_anno.clone())?;

                                // For TypeReference nodes, check if they reference an interface or other named type
                                if type_anno.kind() == ast::Kind::TypeReference {
                                    if let Some(type_ref) =
                                        type_anno.as_any().downcast_ref::<ast::TypeReference>()
                                    {
                                        let type_name = &type_ref.type_name.text;

                                        // Look up the type in context
                                        if let Some(resolved_type) = context.get_type(type_name) {
                                            var_type = resolved_type;
                                        }
                                    }
                                }
                            }

                            // Check initializer if present
                            if let Some(init) = &decl.initializer {
                                let init_type = self.check_expression(context, Rc::clone(init))?;

                                // For object-to-interface assignment, we need to handle extra properties separately
                                // since they won't be caught by the assignability check
                                if let (
                                    Type::Object(Some(init_props)),
                                    Type::Interface(interface_name, interface_props),
                                ) = (&init_type, &var_type)
                                {
                                    // Check for excess properties
                                    for (prop_name, _) in init_props {
                                        if !interface_props
                                            .iter()
                                            .any(|(name, _)| name == prop_name)
                                        {
                                            // Report excess property
                                            self.diagnostics.push(self.create_diagnostic(
                                                DiagnosticCode::ExtraProperty,
                                                format!(
                                                    "Object literal may only specify known properties, and '{}' does not exist in type '{}'",
                                                    prop_name,
                                                    interface_name
                                                ),
                                                init.pos(),
                                                init.end(),
                                            ));
                                        }
                                    }
                                }

                                // Save the fact that we're dealing with an object-to-interface assignment
                                // to avoid duplicate type mismatch errors later
                                let is_object_to_interface_assignment = matches!(
                                    (&init_type, &var_type),
                                    (Type::Object(Some(_)), Type::Interface(_, _))
                                );

                                // If we have both a type annotation and initializer, verify compatibility
                                if decl.type_annotation.is_some() {
                                    if !self.is_assignable_to(&init_type, &var_type) {
                                        // Special handling for object type errors
                                        match (&init_type, &var_type) {
                                            (
                                                Type::Object(Some(init_props)),
                                                Type::Interface(interface_name, target_props),
                                            ) => {
                                                // Check for missing properties
                                                let mut missing_props = Vec::new();
                                                for (target_name, _) in target_props {
                                                    if !init_props
                                                        .iter()
                                                        .any(|(name, _)| name == target_name)
                                                    {
                                                        missing_props.push(target_name.clone());
                                                    }
                                                }

                                                if !missing_props.is_empty() {
                                                    // We'll report just one missing property at a time, following Deno's behavior
                                                    let missing_prop = &missing_props[0];

                                                    // Format the object type as a string representation
                                                    let obj_type_str = format!(
                                                        "{{ {} }}",
                                                        init_props
                                                            .iter()
                                                            .map(|(name, typ)| format!(
                                                                "{}: {}",
                                                                name,
                                                                self.format_type(typ)
                                                            ))
                                                            .collect::<Vec<_>>()
                                                            .join("; ")
                                                    );

                                                    // Report missing property
                                                    self.diagnostics.push(self.create_diagnostic(
                                                        DiagnosticCode::MissingProperty,
                                                        format!(
                                                            "Property '{}' is missing in type '{}' but required in type '{}'",
                                                            missing_prop,
                                                            obj_type_str,
                                                            interface_name
                                                        ),
                                                        init.pos(),
                                                        init.end(),
                                                    ));
                                                }

                                                // Check for type mismatches in properties
                                                for (init_name, init_prop_type) in init_props {
                                                    if let Some((_, target_prop_type)) =
                                                        target_props
                                                            .iter()
                                                            .find(|(name, _)| name == init_name)
                                                    {
                                                        if !self.is_assignable_to(
                                                            init_prop_type,
                                                            target_prop_type,
                                                        ) {
                                                            // Find the position of the property value in the object literal
                                                            let property_start = self.source_text
                                                                [..init.end()]
                                                                .rfind(init_name)
                                                                .unwrap_or(init.pos());
                                                            let value_start = self.source_text
                                                                [property_start..init.end()]
                                                                .find(':')
                                                                .map(|pos| property_start + pos + 1)
                                                                .unwrap_or(init.pos());
                                                            let value_end = if let Some(pos) = self
                                                                .source_text
                                                                [value_start..init.end()]
                                                                .find(',')
                                                            {
                                                                value_start + pos
                                                            } else {
                                                                init.end() - 1
                                                            };

                                                            // Report property type mismatch - Format like Deno
                                                            // We know the exact location where the property value is defined
                                                            self.diagnostics.push(self.create_diagnostic(
                                                                DiagnosticCode::TypeMismatch, // Changed to TypeMismatch to be consistent
                                                                format!(
                                                                    "Type '{}' is not assignable to type '{}'",
                                                                    self.format_type(init_prop_type), self.format_type(target_prop_type)
                                                                ),
                                                                value_start,
                                                                value_end,
                                                            ));
                                                        }
                                                    } else if !target_props
                                                        .iter()
                                                        .any(|(name, _)| name == init_name)
                                                    {
                                                        // Find the position of the extra property in the object literal
                                                        let property_start = self.source_text
                                                            [..init.end()]
                                                            .rfind(init_name)
                                                            .unwrap_or(init.pos());
                                                        let property_end = if let Some(pos) = self
                                                            .source_text
                                                            [property_start..init.end()]
                                                            .find(',')
                                                        {
                                                            property_start + pos
                                                        } else {
                                                            init.end() - 1
                                                        };

                                                        // Report extra property (not in target interface)
                                                        self.diagnostics.push(self.create_diagnostic(
                                                            DiagnosticCode::ExtraProperty,
                                                            format!(
                                                                "Object literal may only specify known properties, and '{}' does not exist in type '{}'",
                                                                init_name, interface_name
                                                            ),
                                                            property_start,
                                                            property_end,
                                                        ));
                                                    }
                                                }
                                            }
                                            (
                                                Type::Object(Some(init_props)),
                                                Type::Object(Some(target_props)),
                                            ) => {
                                                // Similar checks for object to object assignment
                                                // Check for missing properties
                                                for (target_name, _) in target_props {
                                                    if !init_props
                                                        .iter()
                                                        .any(|(name, _)| name == target_name)
                                                    {
                                                        self.diagnostics.push(
                                                            self.create_diagnostic(
                                                                DiagnosticCode::MissingProperty,
                                                                format!(
                                                                "Property '{}' is missing in type",
                                                                target_name
                                                            ),
                                                                init.pos(),
                                                                init.end(),
                                                            ),
                                                        );
                                                    }
                                                }

                                                // Check for type mismatches
                                                for (init_name, init_prop_type) in init_props {
                                                    if let Some((_, target_prop_type)) =
                                                        target_props
                                                            .iter()
                                                            .find(|(name, _)| name == init_name)
                                                    {
                                                        if !self.is_assignable_to(
                                                            init_prop_type,
                                                            target_prop_type,
                                                        ) {
                                                            self.diagnostics.push(self.create_diagnostic(
                                                                DiagnosticCode::PropertyTypeMismatch,
                                                                format!(
                                                                    "Type '{}' is not assignable to type '{}' for property '{}'",
                                                                    self.format_type(init_prop_type), self.format_type(target_prop_type), init_name
                                                                ),
                                                                init.pos(),
                                                                init.end(),
                                                            ));
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {
                                                // General type mismatch error
                                                // Skip if we already reported property-specific errors
                                                // for object-to-interface assignments
                                                if !is_object_to_interface_assignment {
                                                    self.diagnostics.push(self.create_diagnostic(
                                                        DiagnosticCode::TypeMismatch,
                                                        format!(
                                                            "Type '{}' is not assignable to type '{}'",
                                                            self.format_type(&init_type),
                                                            self.format_type(&var_type)
                                                        ),
                                                        init.pos(),
                                                        init.end(),
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                    // Keep using the annotated type even if initializer doesn't match
                                } else {
                                    // Infer type from initializer if no type annotation
                                    var_type = init_type;
                                }
                            } else if var_stmt.declaration_kind == ast::Kind::ConstKeyword {
                                // Constants must have initializers
                                self.diagnostics.push(self.create_diagnostic(
                                    DiagnosticCode::SyntaxError,
                                    "const declarations must be initialized".to_string(),
                                    decl.pos(),
                                    decl.end(),
                                ));
                            }

                            // Add variable to context
                            context.add_variable(decl.name.text.clone(), var_type);
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
    fn check_expression(
        &mut self,
        context: &TypeContext,
        expression: Rc<dyn ast::Node>,
    ) -> Result<Type> {
        match expression.kind() {
            ast::Kind::FunctionExpression => {
                // Handle function expressions (e.g., function(n) { return n + 1; })
                // Corresponds to checking FunctionExpression in internal/checker/checker.go
                // But our implementation is simplified since we don't check the function body
                if let Some(func_expr) = expression
                    .as_any()
                    .downcast_ref::<ast::FunctionExpression>()
                {
                    // Create parameter types - don't try to create a new context
                    let mut param_types = Vec::new();
                    for param in &func_expr.parameters {
                        let param_type = if let Some(type_annotation) = &param.type_annotation {
                            self.get_type_from_node(type_annotation.clone())?
                        } else {
                            Type::Any
                        };
                        param_types.push(param_type);
                    }

                    // Get return type
                    let return_type = if let Some(return_type) = &func_expr.return_type {
                        self.get_type_from_node(return_type.clone())?
                    } else {
                        Type::Any
                    };

                    // Create function signature
                    let signature = FunctionSignature {
                        parameters: param_types,
                        return_type,
                    };

                    // Don't check function body for now, just return function type
                    // Note: The Go implementation would check the function body here
                    return Ok(Type::Function(Box::new(signature)));
                }

                return Ok(Type::Error);
            }

            ast::Kind::PropertyAccessExpression => {
                // Handle property access expression (e.g., d.join)
                if let Some(prop_access) = expression
                    .as_any()
                    .downcast_ref::<ast::PropertyAccessExpression>()
                {
                    // Check the object expression
                    let obj_type =
                        self.check_expression(context, Rc::clone(&prop_access.expression))?;

                    // Get the property name
                    let prop_name = &prop_access.name.text;

                    // Check for array type methods
                    if let Type::Array(_) = obj_type {
                        // For arrays, check common methods
                        if prop_name == "join" {
                            // Simplified handling - join method takes a string and returns a string
                            return Ok(Type::Function(Box::new(FunctionSignature {
                                parameters: vec![Type::String],
                                return_type: Type::String,
                            })));
                        }
                    }

                    // Check for object type properties
                    match &obj_type {
                        Type::Object(Some(props)) => {
                            // Look for property in props
                            if let Some((_, prop_type)) =
                                props.iter().find(|(name, _)| name == prop_name)
                            {
                                return Ok(prop_type.clone());
                            } else {
                                // Property not found in object
                                // Find the actual location in the source code where this property access happens
                                let pos = prop_access.name.pos();
                                let end = prop_access.name.end();

                                self.diagnostics.push(self.create_diagnostic(
                                    DiagnosticCode::NonExistentProperty,
                                    format!(
                                        "Property '{}' does not exist on type '{}'",
                                        prop_name,
                                        self.format_type(&obj_type)
                                    ),
                                    pos,
                                    end,
                                ));
                                // Continue to allow checking for more errors
                                return Ok(Type::Error);
                            }
                        }
                        Type::Interface(interface_name, props) => {
                            // Look for property in interface
                            if let Some((_, prop_type)) =
                                props.iter().find(|(name, _)| name == prop_name)
                            {
                                return Ok(prop_type.clone());
                            } else {
                                // Property not found in interface
                                self.diagnostics.push(self.create_diagnostic(
                                    DiagnosticCode::NonExistentProperty,
                                    format!(
                                        "Property '{}' does not exist on type '{}'",
                                        prop_name, interface_name
                                    ),
                                    prop_access.name.pos(),
                                    prop_access.name.end(),
                                ));
                                // Continue to allow checking for more errors
                                return Ok(Type::Error);
                            }
                        }
                        Type::Number => {
                            // Numbers don't have properties except for some standard ones
                            let pos = prop_access.name.pos();
                            let end = prop_access.name.end();

                            self.diagnostics.push(self.create_diagnostic(
                                DiagnosticCode::InvalidMethodCall,
                                format!("Property '{}' does not exist on type 'number'", prop_name),
                                pos,
                                end,
                            ));

                            // Return a dummy function type to avoid "Invalid call target" error
                            // This matches TypeScript's behavior of only reporting one error
                            return Ok(Type::Function(Box::new(FunctionSignature {
                                parameters: vec![],
                                return_type: Type::Any,
                            })));
                        }
                        _ => {
                            // For other types, we'll just return Any for now
                            return Ok(Type::Any);
                        }
                    }
                }

                return Ok(Type::Error);
            }

            ast::Kind::CallExpression => {
                // Downcast to CallExpression
                if let Some(call_expr) = expression.as_any().downcast_ref::<ast::CallExpression>() {
                    // Get the function being called
                    let func_expr = &call_expr.expression;

                    // Get the function type
                    let func_type = self.check_expression(context, Rc::clone(func_expr))?;

                    // Handle different types of function expressions
                    match func_expr.kind() {
                        ast::Kind::Identifier => {
                            // Direct function call (e.g., demo(...))
                            if let Some(ident) =
                                func_expr.as_any().downcast_ref::<ast::Identifier>()
                            {
                                // Look up function in context
                                if let Some(signature) = context.get_function(&ident.text) {
                                    // Check argument count
                                    if call_expr.arguments.len() != signature.parameters.len() {
                                        // Format Deno-style error message
                                        let msg = if call_expr.arguments.len()
                                            < signature.parameters.len()
                                        {
                                            format!(
                                                "Expected {} arguments, but got {}",
                                                signature.parameters.len(),
                                                call_expr.arguments.len()
                                            )
                                        } else {
                                            format!(
                                                "Expected {} arguments, but got {}",
                                                signature.parameters.len(),
                                                call_expr.arguments.len()
                                            )
                                        };

                                        // Create diagnostic with proper location info
                                        let diag = self.create_diagnostic(
                                            DiagnosticCode::ArgumentCountMismatch,
                                            msg,
                                            call_expr.pos(),
                                            call_expr.end(),
                                        );
                                        self.diagnostics.push(diag);
                                    }

                                    // Check each argument type, but only up to the minimum of arguments provided and parameters expected
                                    let param_count = signature.parameters.len();
                                    let arg_count = call_expr.arguments.len();
                                    let check_count = std::cmp::min(param_count, arg_count);

                                    for i in 0..check_count {
                                        let arg = &call_expr.arguments[i];
                                        let arg_type =
                                            self.check_expression(context, Rc::clone(arg))?;
                                        let expected_type = &signature.parameters[i];

                                        if !self.is_assignable_to(&arg_type, expected_type) {
                                            self.diagnostics.push(self.create_diagnostic(
                                                DiagnosticCode::TypeMismatch,
                                                format!(
                                                    "Argument of type '{}' is not assignable to parameter of type '{}'",
                                                    self.format_type(&arg_type),
                                                    self.format_type(expected_type)
                                                ),
                                                arg.pos(),
                                                arg.end(),
                                            ));
                                            // Don't return early, continue checking other arguments
                                        }
                                    }

                                    // Return function's return type
                                    return Ok(signature.return_type);
                                } else {
                                    self.diagnostics.push(self.create_diagnostic(
                                        DiagnosticCode::UndefinedFunction,
                                        format!("Cannot find function '{}'", ident.text),
                                        func_expr.pos(),
                                        func_expr.end(),
                                    ));
                                    // Continue to check for other errors
                                    return Ok(Type::Error);
                                }
                            }
                        }
                        ast::Kind::PropertyAccessExpression => {
                            // Method call (e.g., d.join(...))
                            if let Type::Function(signature) = func_type {
                                // Check argument count
                                if call_expr.arguments.len() != signature.parameters.len() {
                                    self.diagnostics.push(self.create_diagnostic(
                                        DiagnosticCode::ArgumentCountMismatch,
                                        format!(
                                            "Expected {} arguments but got {}",
                                            signature.parameters.len(),
                                            call_expr.arguments.len()
                                        ),
                                        call_expr.pos(),
                                        call_expr.end(),
                                    ));
                                }

                                // Check each argument, but only up to the minimum of arguments provided and parameters expected
                                let param_count = signature.parameters.len();
                                let arg_count = call_expr.arguments.len();
                                let check_count = std::cmp::min(param_count, arg_count);

                                for i in 0..check_count {
                                    let arg = &call_expr.arguments[i];
                                    let arg_type =
                                        self.check_expression(context, Rc::clone(arg))?;
                                    let expected_type = &signature.parameters[i];

                                    if !self.is_assignable_to(&arg_type, expected_type) {
                                        self.diagnostics.push(self.create_diagnostic(
                                            DiagnosticCode::TypeMismatch,
                                            format!(
                                                "Argument of type '{}' is not assignable to parameter of type '{}'",
                                                self.format_type(&arg_type),
                                                self.format_type(expected_type)
                                            ),
                                            arg.pos(),
                                            arg.end(),
                                        ));
                                        // Don't return early, continue checking other arguments
                                    }
                                }

                                // Return the function's return type
                                return Ok(signature.return_type);
                            }
                        }
                        _ => {}
                    }

                    // If we get here, it's an invalid call target
                    self.diagnostics.push(self.create_diagnostic(
                        DiagnosticCode::InvalidCallTarget,
                        "Invalid call target".to_string(),
                        func_expr.pos(),
                        func_expr.end(),
                    ));
                    // Continue to check for other errors
                    return Ok(Type::Error);
                }
            }
            ast::Kind::BinaryExpression => {
                // Downcast to BinaryExpression
                if let Some(binary_expr) =
                    expression.as_any().downcast_ref::<ast::BinaryExpression>()
                {
                    let left_type = self.check_expression(context, Rc::clone(&binary_expr.left))?;
                    let right_type =
                        self.check_expression(context, Rc::clone(&binary_expr.right))?;

                    // Handle addition operator
                    if binary_expr.operator_token == ast::Kind::PlusToken {
                        if left_type == Type::String {
                            // String + anything is allowed (coerces to string)
                            return Ok(Type::String);
                        } else if left_type == Type::Number && right_type == Type::Number {
                            // Number + Number = Number
                            return Ok(Type::Number);
                        } else {
                            self.diagnostics.push(self.create_diagnostic(
                                DiagnosticCode::InvalidBinaryOperation,
                                format!(
                                    "Operator '+' cannot be applied to types '{}' and '{}'",
                                    self.format_type(&left_type),
                                    self.format_type(&right_type)
                                ),
                                binary_expr.pos(),
                                binary_expr.end(),
                            ));
                            // Continue to check for other errors
                            return Ok(Type::Error);
                        }
                    }

                    // Handle other operators as needed
                    // ...

                    // Default error case
                    self.diagnostics.push(self.create_diagnostic(
                        DiagnosticCode::UnsupportedOperator,
                        "Unsupported binary operator".to_string(),
                        binary_expr.pos(),
                        binary_expr.end(),
                    ));
                    // Continue to check for other errors
                    return Ok(Type::Error);
                }
            }
            ast::Kind::Identifier => {
                // Downcast to Identifier
                if let Some(ident) = expression.as_any().downcast_ref::<ast::Identifier>() {
                    // Special case for the built-in String constructor/function
                    if ident.text == "String" {
                        let string_params = vec![Type::Any];
                        let string_signature = FunctionSignature {
                            parameters: string_params,
                            return_type: Type::String,
                        };
                        return Ok(Type::Function(Box::new(string_signature)));
                    }

                    // Look up variable in context
                    if let Some(var_type) = context.get_variable(&ident.text) {
                        return Ok(var_type);
                    } else {
                        self.diagnostics.push(self.create_diagnostic(
                            DiagnosticCode::UndefinedVariable,
                            format!("Cannot find variable '{}'", ident.text),
                            ident.pos(),
                            ident.end(),
                        ));
                        // Continue to check for other errors
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
                if let Some(array_expr) = expression
                    .as_any()
                    .downcast_ref::<ast::ArrayLiteralExpression>()
                {
                    // Check if array is empty
                    if array_expr.elements.is_empty() {
                        // Empty array - default to Any[]
                        return Ok(Type::Array(Box::new(Type::Any)));
                    }

                    // Try to infer the element type from the first element
                    let first_elem_type =
                        self.check_expression(context, Rc::clone(&array_expr.elements[0]))?;

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
                return Ok(Type::Array(Box::new(Type::Any)));
            }
            ast::Kind::ObjectLiteralExpression => {
                if let Some(obj_expr) = expression
                    .as_any()
                    .downcast_ref::<ast::ObjectLiteralExpression>()
                {
                    // Empty object case
                    if obj_expr.properties.is_empty() {
                        return Ok(Type::Object(None)); // Empty object
                    }

                    // Collect property types for non-empty objects
                    let mut properties = Vec::new();

                    for prop_node in &obj_expr.properties {
                        if let Some(prop_assignment) =
                            prop_node.as_any().downcast_ref::<ast::PropertyAssignment>()
                        {
                            let name = &prop_assignment.name.text;
                            let value_type = self.check_expression(
                                context,
                                Rc::clone(&prop_assignment.initializer),
                            )?;

                            properties.push((name.clone(), value_type));
                        }
                    }

                    return Ok(Type::Object(Some(properties)));
                }

                // Fallback for any unexpected issues
                return Ok(Type::Object(None));
            }
            _ => {
                // Unhandled expression type
                self.diagnostics.push(self.create_diagnostic(
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
    /// Corresponds to various type creation functions in internal/checker/types.go
    /// Extended with additional type handling including object type literals
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
            ast::Kind::TypeLiteral => {
                // Handle object type literals: { name: string; age: number }
                // In Go, object types are handled differently through ObjectType and PropertySignature
                // Our implementation uses a simpler approach with TypeLiteral and property maps
                if let Some(type_lit) = node.as_any().downcast_ref::<ast::TypeLiteral>() {
                    let mut props = Vec::new();

                    // Convert each property signature to a named type
                    for member in &type_lit.members {
                        if let Some(prop_sig) =
                            member.as_any().downcast_ref::<ast::PropertySignature>()
                        {
                            let name = prop_sig.name.text.clone();
                            let type_annotation =
                                self.get_type_from_node(Rc::clone(&prop_sig.type_annotation))?;
                            props.push((name, type_annotation));
                        }
                    }

                    if props.is_empty() {
                        Ok(Type::Object(None))
                    } else {
                        Ok(Type::Object(Some(props)))
                    }
                } else {
                    Ok(Type::Any)
                }
            }
            _ => Ok(Type::Any), // Unhandled type annotation - default to Any
        }
    }

    /// Format a type as a string for error messages
    /// This produces more readable type names for diagnostics
    fn format_type(&self, typ: &Type) -> String {
        match typ {
            Type::Any => "any".to_string(),
            Type::Error => "error".to_string(),
            Type::String => "string".to_string(),
            Type::Number => "number".to_string(),
            Type::Boolean => "boolean".to_string(),
            Type::_Void => "void".to_string(),
            Type::_Undefined => "undefined".to_string(),
            Type::_Null => "null".to_string(),
            Type::Array(elem_type) => format!("{}[]", self.format_type(elem_type)),
            Type::Object(None) => "{}".to_string(),
            Type::Object(Some(props)) => {
                if props.is_empty() {
                    "{}".to_string()
                } else {
                    let properties = props
                        .iter()
                        .map(|(name, typ)| format!("{}: {}", name, self.format_type(typ)))
                        .collect::<Vec<_>>()
                        .join("; ");
                    format!("{{ {} }}", properties)
                }
            }
            Type::Interface(name, _) => name.clone(),
            Type::Function(signature) => {
                let params = signature
                    .parameters
                    .iter()
                    .map(|param| self.format_type(param))
                    .collect::<Vec<_>>()
                    .join(", ");
                let return_type = self.format_type(&signature.return_type);
                format!("({}) => {}", params, return_type)
            }
        }
    }

    /// Check if source_type is assignable to target_type
    /// Corresponds to isTypeAssignableTo in internal/checker/relater.go
    fn is_assignable_to(&self, source_type: &Type, target_type: &Type) -> bool {
        let relater = self.create_relater();
        relater.is_related_to(source_type, target_type, RelationKind::Assignable)
    }

    /// Creates a new relater instance for type relationship checking
    /// Corresponds to the use of Relaters in the Go implementation
    fn create_relater(&self) -> Relater {
        Relater::new(self)
    }
}

/// RelationKind represents different kinds of type relations
/// Corresponds to the different relation constants (assignableRelation, etc.) in Go
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RelationKind {
    /// Types are assignable to each other
    Assignable,
    /// Types are identical
    Identity,
    /// One type is a subtype of another
    Subtype,
    /// Types are comparable with == or !=
    Comparable,
}

/// Relater handles type relation checking
/// This is a simplified version of the Relater struct in Go
pub struct Relater<'a> {
    checker: &'a TypeChecker,
}

impl<'a> Relater<'a> {
    /// Create a new Relater
    fn new(checker: &'a TypeChecker) -> Self {
        Self { checker }
    }

    /// Check if source is related to target according to the relation kind
    /// Simplified version of isRelatedTo in Go
    pub fn is_related_to(&self, source: &Type, target: &Type, relation: RelationKind) -> bool {
        // Any is related to anything
        if *source == Type::Any || *target == Type::Any {
            return true;
        }

        // Error is not related to anything except Any
        if *source == Type::Error {
            return false;
        }

        // Identical types are always related
        if source == target {
            return true;
        }

        // For identity relation, check must be more strict
        if relation == RelationKind::Identity {
            return self.is_identical_to(source, target);
        }

        // Use simple type relation check for primitive types
        if self.is_simple_type_related_to(source, target, relation) {
            return true;
        }

        // Handle structured types (objects, interfaces, arrays)
        match (source, target) {
            // Array types
            (Type::Array(src_elem), Type::Array(tgt_elem)) => {
                self.is_related_to(src_elem, tgt_elem, relation)
            }

            // Object to interface relation
            (Type::Object(src_props), Type::Interface(_, tgt_props)) => {
                self.properties_related_to(source, target)
            }

            // Object to object relation
            (Type::Object(_), Type::Object(_)) => self.properties_related_to(source, target),

            // Interface to interface relation
            (Type::Interface(_, _), Type::Interface(_, _)) => {
                self.properties_related_to(source, target)
            }

            // Interface to object relation
            (Type::Interface(_, _), Type::Object(_)) => self.properties_related_to(source, target),

            // Function types
            (Type::Function(src_sig), Type::Function(tgt_sig)) => {
                self.signatures_related_to(src_sig, tgt_sig)
            }

            // Default: not related
            _ => false,
        }
    }

    /// Check if two types are identical
    /// Simplified version of isTypeIdenticalTo in Go
    fn is_identical_to(&self, source: &Type, target: &Type) -> bool {
        match (source, target) {
            // Array types are identical if their element types are identical
            (Type::Array(src_elem), Type::Array(tgt_elem)) => {
                self.is_identical_to(src_elem, tgt_elem)
            }

            // Object types are identical if they have the same properties with identical types
            (Type::Object(src_props), Type::Object(tgt_props)) => {
                match (src_props, tgt_props) {
                    (None, None) => true,
                    (Some(src), Some(tgt)) if src.len() == tgt.len() => {
                        // For identity, all properties must exactly match in both directions
                        for (src_name, src_type) in src {
                            match tgt.iter().find(|(name, _)| name == src_name) {
                                Some((_, tgt_type)) => {
                                    if !self.is_identical_to(src_type, tgt_type) {
                                        return false;
                                    }
                                }
                                None => return false,
                            }
                        }
                        true
                    }
                    _ => false,
                }
            }

            // Interface types are identical if they have the same name and identical properties
            (Type::Interface(src_name, src_props), Type::Interface(tgt_name, tgt_props)) => {
                src_name == tgt_name
                    && src_props.len() == tgt_props.len()
                    && src_props.iter().all(|(src_name, src_type)| {
                        tgt_props.iter().any(|(tgt_name, tgt_type)| {
                            src_name == tgt_name && self.is_identical_to(src_type, tgt_type)
                        })
                    })
            }

            // Function types are identical if their signatures are identical
            (Type::Function(src_sig), Type::Function(tgt_sig)) => {
                src_sig.parameters.len() == tgt_sig.parameters.len()
                    && self.is_identical_to(&src_sig.return_type, &tgt_sig.return_type)
                    && src_sig
                        .parameters
                        .iter()
                        .zip(tgt_sig.parameters.iter())
                        .all(|(src_param, tgt_param)| self.is_identical_to(src_param, tgt_param))
            }

            // Default comparison: types are identical if they're the same variant
            _ => std::mem::discriminant(source) == std::mem::discriminant(target),
        }
    }

    /// Check if source type is related to target type for simple type cases
    /// Simplified version of isSimpleTypeRelatedTo in Go
    fn is_simple_type_related_to(
        &self,
        source: &Type,
        target: &Type,
        relation: RelationKind,
    ) -> bool {
        match (source, target) {
            // String-like types are related to string
            (_, Type::String) => {
                matches!(source, Type::String)
            }

            // Number-like types are related to number
            (_, Type::Number) => {
                matches!(source, Type::Number)
            }

            // Boolean-like types are related to boolean
            (_, Type::Boolean) => {
                matches!(source, Type::Boolean)
            }

            // For assignable relation, Any is assignable to anything
            (Type::Any, _) if relation == RelationKind::Assignable => true,

            // Default: not a simple relation
            _ => false,
        }
    }

    /// Check if source's properties are related to target's properties
    /// Simplified version of propertiesRelatedTo in Go
    fn properties_related_to(&self, source: &Type, target: &Type) -> bool {
        // Extract properties from source type
        let source_props = match source {
            Type::Object(Some(props)) => props,
            Type::Object(None) => return true, // Empty object is assignable to any object or interface
            Type::Interface(_, props) => props,
            _ => return false,
        };

        // Extract properties from target type
        let target_props = match target {
            Type::Object(Some(props)) => props,
            Type::Object(None) => return true, // Anything is assignable to empty object
            Type::Interface(_, props) => props,
            _ => return false,
        };

        // Check that all target properties exist in source with compatible types
        for (target_prop_name, target_prop_type) in target_props {
            match source_props
                .iter()
                .find(|(name, _)| name == target_prop_name)
            {
                Some((_, source_prop_type)) => {
                    // Property exists, check type compatibility
                    if !self.is_related_to(
                        source_prop_type,
                        target_prop_type,
                        RelationKind::Assignable,
                    ) {
                        return false;
                    }
                }
                None => {
                    // Required property missing in source
                    return false;
                }
            }
        }

        // All required properties exist with compatible types
        true
    }

    /// Check if source signature is related to target signature
    /// Simplified version of signatureRelatedTo in Go
    fn signatures_related_to(
        &self,
        source: &FunctionSignature,
        target: &FunctionSignature,
    ) -> bool {
        // Check return type - return type is covariant
        if !self.is_related_to(
            &source.return_type,
            &target.return_type,
            RelationKind::Assignable,
        ) {
            return false;
        }

        // Check parameters - parameters are contravariant
        // Parameter count must match
        if source.parameters.len() != target.parameters.len() {
            return false;
        }

        // Each parameter type must be compatible in reverse direction (contravariant)
        for (i, target_param) in target.parameters.iter().enumerate() {
            let source_param = &source.parameters[i];
            if !self.is_related_to(target_param, source_param, RelationKind::Assignable) {
                return false;
            }
        }

        true
    }
}
