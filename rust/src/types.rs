use crate::ast::{Expression, FunctionDeclaration, Program, Statement, TypeAnnotation};
use crate::error::{Result, TsError};

pub struct TypeChecker {
    program: Program,
}

impl TypeChecker {
    pub fn new(program: Program) -> Self {
        TypeChecker { program }
    }
    
    pub fn check(&self) -> Result<()> {
        for function in &self.program.functions {
            self.check_function(function)?;
        }
        
        for statement in &self.program.statements {
            self.check_statement(statement)?;
        }
        
        Ok(())
    }
    
    fn check_function(&self, function: &FunctionDeclaration) -> Result<()> {
        for statement in &function.body {
            match statement {
                Statement::Return(expr) => {
                    let expr_type = self.typeof_expression(expr)?;
                    if expr_type != function.return_type {
                        return Err(TsError::TypeError(format!(
                            "Function '{}' should return '{}' but returns '{}'",
                            function.name, function.return_type, expr_type
                        )));
                    }
                }
                _ => self.check_statement(statement)?,
            }
        }
        
        Ok(())
    }
    
    fn check_statement(&self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Return(expr) => {
                // Return statements are checked in check_function
                self.typeof_expression(expr)?;
            }
            Statement::Expression(expr) => {
                self.typeof_expression(expr)?;
            }
        }
        
        Ok(())
    }
    
    fn typeof_expression(&self, expr: &Expression) -> Result<TypeAnnotation> {
        match expr {
            Expression::StringLiteral(_) => Ok(TypeAnnotation::String),
            Expression::NumberLiteral(_) => Ok(TypeAnnotation::Number),
            
            Expression::BinaryExpression { left, operator, right } => {
                let left_type = self.typeof_expression(left)?;
                let right_type = self.typeof_expression(right)?;
                
                // Basic type checking for binary operators
                match operator {
                    crate::ast::BinaryOperator::Plus => {
                        if left_type == TypeAnnotation::String || right_type == TypeAnnotation::String {
                            Ok(TypeAnnotation::String)
                        } else if left_type == TypeAnnotation::Number && right_type == TypeAnnotation::Number {
                            Ok(TypeAnnotation::Number)
                        } else {
                            Err(TsError::TypeError(format!(
                                "Cannot apply '+' operator to types '{}' and '{}'",
                                left_type, right_type
                            )))
                        }
                    }
                }
            }
            
            Expression::FunctionCall { callee, arguments } => {
                // Find the function declaration
                let function = self.program.functions.iter().find(|f| f.name == *callee);
                
                match function {
                    Some(function) => {
                        // Check argument count
                        if arguments.len() != function.parameters.len() {
                            return Err(TsError::TypeError(format!(
                                "Function '{}' expects {} arguments but got {}",
                                callee,
                                function.parameters.len(),
                                arguments.len()
                            )));
                        }
                        
                        // Check argument types
                        for (i, (arg, param)) in arguments.iter().zip(function.parameters.iter()).enumerate() {
                            let arg_type = self.typeof_expression(arg)?;
                            if arg_type != param.type_annotation {
                                return Err(TsError::TypeError(format!(
                                    "Argument {} of function '{}' should be '{}' but got '{}'",
                                    i + 1,
                                    callee,
                                    param.type_annotation,
                                    arg_type
                                )));
                            }
                        }
                        
                        Ok(function.return_type.clone())
                    }
                    None => Err(TsError::TypeError(format!("Function '{}' not found", callee))),
                }
            }
        }
    }
}