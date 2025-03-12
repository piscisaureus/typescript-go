// Corresponds to internal/ast/ast.go in the Go implementation

mod kind;
mod node_flags;
mod additional_nodes;

pub use kind::Kind;
pub use node_flags::NodeFlags;
pub use additional_nodes::*;

use crate::error::Diagnostic;
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

    pub fn with_pos(kind: Kind, pos: usize, end: usize) -> Self {
        Self {
            kind,
            flags: NodeFlags::None,
            loc: TextRange::new(pos, end),
        }
    }

    pub fn set_pos(&mut self, pos: usize, end: usize) {
        self.loc = TextRange::new(pos, end);
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

impl Node for FunctionExpression {
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

impl Node for BooleanLiteral {
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

impl Node for ArrayLiteralExpression {
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

/// Represents a boolean literal (true/false)
/// In Go, this is the BooleanLiteral struct
#[derive(Debug, Clone)]
pub struct BooleanLiteral {
    pub base: NodeBase,
    pub value: bool,
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

/// Represents a function expression (anonymous function)
/// In Go, this is similar to FunctionExpression in internal/ast/ast.go
/// Note: The Go implementation uses a single FunctionLikeDeclarationBase for both
/// function declarations and expressions, but we use separate types for clarity
#[derive(Debug, Clone)]
pub struct FunctionExpression {
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

/// Represents an array literal (e.g. [1, 2, 3])
/// In Go, this is the ArrayLiteralExpression struct
#[derive(Debug, Clone)]
pub struct ArrayLiteralExpression {
    pub base: NodeBase,
    pub elements: Vec<Rc<dyn Node>>,
}

/// Represents a type reference (e.g. 'string', 'number', 'any[]')
/// This is similar to TypeReference in the Go code
#[derive(Debug, Clone)]
pub struct TypeReference {
    pub base: NodeBase,
    pub type_name: Rc<Identifier>,
    pub is_array_type: bool,
    pub type_arguments: Vec<Rc<dyn Node>>, // For handling cases like (string | number)[]
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

/// Represents a property in a type literal
/// In Go, this is similar to PropertySignature in internal/ast/ast.go
/// Used for object type literals in parameter and return type annotations
#[derive(Debug, Clone)]
pub struct PropertySignature {
    pub base: NodeBase,
    pub name: Rc<Identifier>,
    pub type_annotation: Rc<dyn Node>,
}

impl Node for PropertySignature {
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

/// Represents an object type literal (e.g. { prop: Type })
/// In Go, this is similar to TypeLiteralNode in internal/ast/ast.go
/// The Go implementation handles this through ObjectType, but we use TypeLiteral
/// to better match TypeScript's AST terminology
#[derive(Debug, Clone)]
pub struct TypeLiteral {
    pub base: NodeBase,
    pub members: Vec<Rc<dyn Node>>,
}

/// Represents a union type (e.g. string | number)
/// This is a new addition to support union types not present in the Go implementation
#[derive(Debug, Clone)]
pub struct UnionType {
    pub base: NodeBase,
    pub types: Vec<Rc<dyn Node>>,
}

impl Node for TypeLiteral {
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

impl Node for UnionType {
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

/// Represents an interface declaration (e.g. interface User {...})
/// In Go, this corresponds to InterfaceDeclaration
#[derive(Debug, Clone)]
pub struct InterfaceDeclaration {
    pub base: NodeBase,
    pub name: Rc<Identifier>,
    pub type_parameters: Vec<Rc<Identifier>>, // For generic interfaces like interface Array<T> {...}
    pub members: Vec<Rc<dyn Node>>,
}

impl Node for InterfaceDeclaration {
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

/// Represents a property access expression (e.g. obj.prop)
/// In Go, this is the PropertyAccessExpression struct
#[derive(Debug, Clone)]
pub struct PropertyAccessExpression {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
    pub name: Rc<Identifier>,
}

impl Node for PropertyAccessExpression {
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

/// Represents a property assignment in an object literal (e.g. prop: value)
/// In Go, this is the PropertyAssignment struct
#[derive(Debug, Clone)]
pub struct PropertyAssignment {
    pub base: NodeBase,
    pub name: Rc<Identifier>,
    pub initializer: Rc<dyn Node>,
}

impl Node for PropertyAssignment {
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

/// Represents a spread assignment in an object literal (e.g. ...obj)
/// In Go, this is similar to SpreadAssignment in internal/ast/ast.go
/// This enables the object spread syntax introduced in ES2018/TypeScript 2.1+
#[derive(Debug, Clone)]
pub struct SpreadAssignment {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
}

impl Node for SpreadAssignment {
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

/// Represents an object literal expression (e.g. {prop: value})
/// In Go, this is the ObjectLiteralExpression struct
#[derive(Debug, Clone)]
pub struct ObjectLiteralExpression {
    pub base: NodeBase,
    pub properties: Vec<Rc<dyn Node>>,
}

impl Node for ObjectLiteralExpression {
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

/// Represents a variable declaration (e.g. 'var x = 5', 'let y = "hello"', 'const z = true')
/// In Go, this is the VariableDeclaration struct
/// Enhanced to support destructuring patterns with binding_name vs. name
#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub base: NodeBase,
    pub name: Rc<Identifier>, // Kept for simple variable declarations
    pub binding_name: Option<Rc<dyn Node>>, // For destructuring patterns
    pub initializer: Option<Rc<dyn Node>>,
    pub type_annotation: Option<Rc<dyn Node>>,
}

impl Node for VariableDeclaration {
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

/// Represents an object binding pattern (destructuring) like {a, b, c} = obj
/// In Go, this would be ObjectBindingPattern
#[derive(Debug, Clone)]
pub struct ObjectBindingPattern {
    pub base: NodeBase,
    pub elements: Vec<Rc<BindingElement>>,
}

impl Node for ObjectBindingPattern {
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

/// Represents a binding element in an object or array binding pattern
/// In Go, this would be BindingElement
#[derive(Debug, Clone)]
pub struct BindingElement {
    pub base: NodeBase,
    pub name: Rc<Identifier>,
    pub property_name: Option<Rc<Identifier>>, // For renamed bindings like { prop: localName }
    pub initializer: Option<Rc<dyn Node>>,     // For default values
}

impl Node for BindingElement {
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

/// Represents a list of variable declarations (e.g. 'var x = 5, y = 10')
/// In Go, this is the VariableDeclarationList struct
#[derive(Debug, Clone)]
pub struct VariableDeclarationList {
    pub base: NodeBase,
    pub declarations: Vec<Rc<VariableDeclaration>>,
}

impl Node for VariableDeclarationList {
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

/// Represents a variable statement (e.g. 'var x = 5;', 'let y = "hello";', 'const z = true;')
/// In Go, this is the VariableStatement struct
#[derive(Debug, Clone)]
pub struct VariableStatement {
    pub base: NodeBase,
    pub declaration_list: Rc<VariableDeclarationList>,
    pub declaration_kind: Kind, // VarKeyword, LetKeyword, or ConstKeyword
}

impl Node for VariableStatement {
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
    // In Go, this has hooks and pools for different node types
    _hooks: Option<NodeFactoryHooks>,
}

/// Hooks for node creation, similar to NodeFactoryHooks in Go
pub struct NodeFactoryHooks {
    pub on_create: Option<Box<dyn Fn(&dyn Node)>>,
    pub on_update: Option<Box<dyn Fn(&dyn Node, &dyn Node)>>,
    pub on_clone: Option<Box<dyn Fn(&dyn Node, &dyn Node)>>,
}

impl NodeFactory {
    pub fn new() -> Self {
        Self { _hooks: None }
    }

    pub fn with_hooks(hooks: NodeFactoryHooks) -> Self {
        Self {
            _hooks: Some(hooks),
        }
    }

    // In Go, there's a hookNodeCreate function that calls OnCreate if it exists
    fn hook_node_create<T: Node>(&self, node: &T) {
        if let Some(hooks) = &self._hooks {
            if let Some(on_create) = &hooks.on_create {
                on_create(node);
            }
        }
    }

    // Add methods for creating various AST nodes
    pub fn create_identifier(&self, text: String) -> Rc<Identifier> {
        let node = Rc::new(Identifier {
            base: NodeBase::new(Kind::Identifier),
            text,
        });
        self.hook_node_create(&*node);
        node
    }

    pub fn create_string_literal(&self, text: String) -> Rc<StringLiteral> {
        let node = Rc::new(StringLiteral {
            base: NodeBase::new(Kind::StringLiteral),
            text,
        });
        self.hook_node_create(&*node);
        node
    }

    pub fn create_numeric_literal(&self, text: String, value: f64) -> Rc<NumericLiteral> {
        let node = Rc::new(NumericLiteral {
            base: NodeBase::new(Kind::NumericLiteral),
            text,
            value,
        });
        self.hook_node_create(&*node);
        node
    }

    pub fn create_type_reference(
        &self,
        type_name: Rc<Identifier>,
        is_array_type: bool,
    ) -> Rc<TypeReference> {
        let node = Rc::new(TypeReference {
            base: NodeBase::new(Kind::TypeReference),
            type_name,
            is_array_type,
            type_arguments: Vec::new(),
        });
        self.hook_node_create(&*node);
        node
    }
}
