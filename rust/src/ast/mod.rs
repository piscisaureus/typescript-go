// Corresponds to internal/ast/ast.go in the Go implementation

mod kind;
mod node_flags;

pub use kind::Kind;
pub use node_flags::NodeFlags;

use crate::error::Diagnostic;
use std::any::Any;
use std::rc::Rc;

/// Represents a text range/position in source code
/// In Go this would be the core.TextRange type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextRange {
    pub pos: usize,
    pub end: usize,
}

impl TextRange {
    pub fn new(pos: usize, end: usize) -> Self {
        Self { pos, end }
    }

    pub fn undefined() -> Self {
        Self { pos: 0, end: 0 }
    }
}

/// Base interface for all AST nodes
/// In Go, this is represented as the Node type
pub trait Node: std::fmt::Debug {
    fn kind(&self) -> Kind;
    fn flags(&self) -> NodeFlags;
    fn pos(&self) -> usize;
    fn end(&self) -> usize;
    fn loc(&self) -> TextRange;
    fn set_flags(&mut self, flags: NodeFlags);
    
    /// Allows downcasting to concrete node types
    /// This is needed for type checking to examine specific node properties
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Base implementation for all AST nodes
/// In Go, this is the NodeBase struct embedded in all node types
#[derive(Debug, Clone)]
pub struct NodeBase {
    pub kind: Kind,
    pub flags: NodeFlags,
    pub loc: TextRange,
}

impl NodeBase {
    pub fn new(kind: Kind) -> Self {
        Self {
            kind,
            flags: NodeFlags::None,
            loc: TextRange::undefined(),
        }
    }
}

// Implement the Node trait for NodeBase
impl Node for NodeBase {
    fn kind(&self) -> Kind {
        self.kind
    }

    fn flags(&self) -> NodeFlags {
        self.flags
    }

    fn pos(&self) -> usize {
        self.loc.pos
    }

    fn end(&self) -> usize {
        self.loc.end
    }

    fn loc(&self) -> TextRange {
        self.loc
    }

    fn set_flags(&mut self, flags: NodeFlags) {
        self.flags = flags;
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// Implement Node for each AST node type
impl Node for SourceFile {
    fn kind(&self) -> Kind {
        self.base.kind()
    }
    fn flags(&self) -> NodeFlags {
        self.base.flags()
    }
    fn pos(&self) -> usize {
        self.base.pos()
    }
    fn end(&self) -> usize {
        self.base.end()
    }
    fn loc(&self) -> TextRange {
        self.base.loc()
    }
    fn set_flags(&mut self, flags: NodeFlags) {
        self.base.set_flags(flags)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Node for Identifier {
    fn kind(&self) -> Kind {
        self.base.kind()
    }
    fn flags(&self) -> NodeFlags {
        self.base.flags()
    }
    fn pos(&self) -> usize {
        self.base.pos()
    }
    fn end(&self) -> usize {
        self.base.end()
    }
    fn loc(&self) -> TextRange {
        self.base.loc()
    }
    fn set_flags(&mut self, flags: NodeFlags) {
        self.base.set_flags(flags)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Node for StringLiteral {
    fn kind(&self) -> Kind {
        self.base.kind()
    }
    fn flags(&self) -> NodeFlags {
        self.base.flags()
    }
    fn pos(&self) -> usize {
        self.base.pos()
    }
    fn end(&self) -> usize {
        self.base.end()
    }
    fn loc(&self) -> TextRange {
        self.base.loc()
    }
    fn set_flags(&mut self, flags: NodeFlags) {
        self.base.set_flags(flags)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Node for NumericLiteral {
    fn kind(&self) -> Kind {
        self.base.kind()
    }
    fn flags(&self) -> NodeFlags {
        self.base.flags()
    }
    fn pos(&self) -> usize {
        self.base.pos()
    }
    fn end(&self) -> usize {
        self.base.end()
    }
    fn loc(&self) -> TextRange {
        self.base.loc()
    }
    fn set_flags(&mut self, flags: NodeFlags) {
        self.base.set_flags(flags)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Node for BinaryExpression {
    fn kind(&self) -> Kind {
        self.base.kind()
    }
    fn flags(&self) -> NodeFlags {
        self.base.flags()
    }
    fn pos(&self) -> usize {
        self.base.pos()
    }
    fn end(&self) -> usize {
        self.base.end()
    }
    fn loc(&self) -> TextRange {
        self.base.loc()
    }
    fn set_flags(&mut self, flags: NodeFlags) {
        self.base.set_flags(flags)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Node for FunctionDeclaration {
    fn kind(&self) -> Kind {
        self.base.kind()
    }
    fn flags(&self) -> NodeFlags {
        self.base.flags()
    }
    fn pos(&self) -> usize {
        self.base.pos()
    }
    fn end(&self) -> usize {
        self.base.end()
    }
    fn loc(&self) -> TextRange {
        self.base.loc()
    }
    fn set_flags(&mut self, flags: NodeFlags) {
        self.base.set_flags(flags)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Node for ParameterDeclaration {
    fn kind(&self) -> Kind {
        self.base.kind()
    }
    fn flags(&self) -> NodeFlags {
        self.base.flags()
    }
    fn pos(&self) -> usize {
        self.base.pos()
    }
    fn end(&self) -> usize {
        self.base.end()
    }
    fn loc(&self) -> TextRange {
        self.base.loc()
    }
    fn set_flags(&mut self, flags: NodeFlags) {
        self.base.set_flags(flags)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Node for Block {
    fn kind(&self) -> Kind {
        self.base.kind()
    }
    fn flags(&self) -> NodeFlags {
        self.base.flags()
    }
    fn pos(&self) -> usize {
        self.base.pos()
    }
    fn end(&self) -> usize {
        self.base.end()
    }
    fn loc(&self) -> TextRange {
        self.base.loc()
    }
    fn set_flags(&mut self, flags: NodeFlags) {
        self.base.set_flags(flags)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Node for ReturnStatement {
    fn kind(&self) -> Kind {
        self.base.kind()
    }
    fn flags(&self) -> NodeFlags {
        self.base.flags()
    }
    fn pos(&self) -> usize {
        self.base.pos()
    }
    fn end(&self) -> usize {
        self.base.end()
    }
    fn loc(&self) -> TextRange {
        self.base.loc()
    }
    fn set_flags(&mut self, flags: NodeFlags) {
        self.base.set_flags(flags)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Node for ExpressionStatement {
    fn kind(&self) -> Kind {
        self.base.kind()
    }
    fn flags(&self) -> NodeFlags {
        self.base.flags()
    }
    fn pos(&self) -> usize {
        self.base.pos()
    }
    fn end(&self) -> usize {
        self.base.end()
    }
    fn loc(&self) -> TextRange {
        self.base.loc()
    }
    fn set_flags(&mut self, flags: NodeFlags) {
        self.base.set_flags(flags)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Node for CallExpression {
    fn kind(&self) -> Kind {
        self.base.kind()
    }
    fn flags(&self) -> NodeFlags {
        self.base.flags()
    }
    fn pos(&self) -> usize {
        self.base.pos()
    }
    fn end(&self) -> usize {
        self.base.end()
    }
    fn loc(&self) -> TextRange {
        self.base.loc()
    }
    fn set_flags(&mut self, flags: NodeFlags) {
        self.base.set_flags(flags)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Represents a source file
/// In Go, this is the SourceFile struct
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub base: NodeBase,
    pub text: String,
    pub file_name: String,
    pub language_version: u8,
    pub statements: Vec<Rc<dyn Node>>,
    pub line_map: Vec<usize>,
    pub diagnostics: Vec<Diagnostic>,
}

/// Represents an identifier
/// In Go, this is the Identifier struct
#[derive(Debug, Clone)]
pub struct Identifier {
    pub base: NodeBase,
    pub text: String,
}

/// Represents a string literal
/// In Go, this is the StringLiteral struct
#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub base: NodeBase,
    pub text: String,
}

/// Represents a numeric literal
/// In Go, this is the NumericLiteral struct
#[derive(Debug, Clone)]
pub struct NumericLiteral {
    pub base: NodeBase,
    pub text: String,
    pub value: f64,
}

/// Represents a binary expression (e.g. a + b)
/// In Go, this is the BinaryExpression struct
#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub base: NodeBase,
    pub left: Rc<dyn Node>,
    pub operator_token: Kind,
    pub right: Rc<dyn Node>,
}

/// Represents a function declaration
/// In Go, this is the FunctionDeclaration struct
#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub base: NodeBase,
    pub name: Option<Rc<Identifier>>,
    pub parameters: Vec<Rc<ParameterDeclaration>>,
    pub body: Option<Rc<Block>>,
    pub return_type: Option<Rc<dyn Node>>,
}

/// Represents a parameter declaration
/// In Go, this is the ParameterDeclaration struct
#[derive(Debug, Clone)]
pub struct ParameterDeclaration {
    pub base: NodeBase,
    pub name: Rc<Identifier>,
    pub type_annotation: Option<Rc<dyn Node>>,
}

/// Represents a block of statements
/// In Go, this is the Block struct
#[derive(Debug, Clone)]
pub struct Block {
    pub base: NodeBase,
    pub statements: Vec<Rc<dyn Node>>,
}

/// Represents a return statement
/// In Go, this is the ReturnStatement struct
#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub base: NodeBase,
    pub expression: Option<Rc<dyn Node>>,
}

/// Represents an expression statement
/// In Go, this is the ExpressionStatement struct
#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
}

/// Represents a call expression (e.g. func(arg1, arg2))
/// In Go, this is the CallExpression struct
#[derive(Debug, Clone)]
pub struct CallExpression {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
    pub arguments: Vec<Rc<dyn Node>>,
}

/// Represents a type reference (e.g. 'string', 'number')
/// This is similar to TypeReference in the Go code
#[derive(Debug, Clone)]
pub struct TypeReference {
    pub base: NodeBase,
    pub type_name: Rc<Identifier>,
}

impl Node for TypeReference {
    fn kind(&self) -> Kind {
        self.base.kind()
    }
    fn flags(&self) -> NodeFlags {
        self.base.flags()
    }
    fn pos(&self) -> usize {
        self.base.pos()
    }
    fn end(&self) -> usize {
        self.base.end()
    }
    fn loc(&self) -> TextRange {
        self.base.loc()
    }
    fn set_flags(&mut self, flags: NodeFlags) {
        self.base.set_flags(flags)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Node factory creates and manages AST nodes
/// In Go, this is the NodeFactory struct
pub struct NodeFactory {
    // Add fields as needed for pooling and hooks
}

impl NodeFactory {
    pub fn new() -> Self {
        Self {
            // Initialize fields as needed
        }
    }

    // Add methods for creating various AST nodes
    pub fn create_identifier(&self, text: String) -> Rc<Identifier> {
        Rc::new(Identifier {
            base: NodeBase::new(Kind::Identifier),
            text,
        })
    }

    pub fn create_string_literal(&self, text: String) -> Rc<StringLiteral> {
        Rc::new(StringLiteral {
            base: NodeBase::new(Kind::StringLiteral),
            text,
        })
    }

    pub fn create_numeric_literal(&self, text: String, value: f64) -> Rc<NumericLiteral> {
        Rc::new(NumericLiteral {
            base: NodeBase::new(Kind::NumericLiteral),
            text,
            value,
        })
    }

    pub fn create_type_reference(&self, type_name: Rc<Identifier>) -> Rc<TypeReference> {
        Rc::new(TypeReference {
            base: NodeBase::new(Kind::TypeReference),
            type_name,
        })
    }
}
