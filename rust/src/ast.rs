use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeAnnotation {
    String,
    Number,
}

#[derive(Debug, Clone)]
pub enum Expression {
    StringLiteral(String),
    NumberLiteral(f64),
    BinaryExpression {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    FunctionCall {
        callee: String,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Plus,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: TypeAnnotation,
}

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: TypeAnnotation,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Return(Expression),
    Expression(Expression),
}

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<FunctionDeclaration>,
    pub statements: Vec<Statement>,
}

impl fmt::Display for TypeAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeAnnotation::String => write!(f, "string"),
            TypeAnnotation::Number => write!(f, "number"),
        }
    }
}