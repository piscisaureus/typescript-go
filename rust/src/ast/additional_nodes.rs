// Additional AST node type implementations
// These are the AST nodes that exist in the Go code but were missing in the Rust implementation

use super::{Kind, Node, NodeBase, NodeFlags, TextRange, Identifier};
use std::rc::Rc;

// ---------------------------------------------------------------------
// Expression Nodes
// ---------------------------------------------------------------------

/// Represents a parenthesized expression (e.g. (a + b))
#[derive(Debug, Clone)]
pub struct ParenthesizedExpression {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
}

impl Node for ParenthesizedExpression {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents an element access expression (e.g. arr[0])
#[derive(Debug, Clone)]
pub struct ElementAccessExpression {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
    pub argument_expression: Rc<dyn Node>,
}

impl Node for ElementAccessExpression {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a new expression (e.g. new MyClass())
#[derive(Debug, Clone)]
pub struct NewExpression {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
    pub arguments: Option<Vec<Rc<dyn Node>>>,
}

impl Node for NewExpression {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a conditional (ternary) expression (e.g. cond ? true_expr : false_expr)
#[derive(Debug, Clone)]
pub struct ConditionalExpression {
    pub base: NodeBase,
    pub condition: Rc<dyn Node>,
    pub when_true: Rc<dyn Node>,
    pub when_false: Rc<dyn Node>,
}

impl Node for ConditionalExpression {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a template expression (e.g. `Hello ${name}`)
#[derive(Debug, Clone)]
pub struct TemplateExpression {
    pub base: NodeBase,
    pub head: Rc<TemplateHead>,
    pub template_spans: Vec<Rc<TemplateSpan>>,
}

impl Node for TemplateExpression {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents the head part of a template literal
#[derive(Debug, Clone)]
pub struct TemplateHead {
    pub base: NodeBase,
    pub text: String,
}

impl Node for TemplateHead {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a template span (part of a template literal)
#[derive(Debug, Clone)]
pub struct TemplateSpan {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
    pub literal: Rc<TemplateMiddleOrTemplateTail>,
}

impl Node for TemplateSpan {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a middle or tail part of a template literal
#[derive(Debug, Clone)]
pub struct TemplateMiddleOrTemplateTail {
    pub base: NodeBase,
    pub text: String,
}

impl Node for TemplateMiddleOrTemplateTail {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a tagged template expression (e.g. tag`Hello ${name}`)
#[derive(Debug, Clone)]
pub struct TaggedTemplateExpression {
    pub base: NodeBase,
    pub tag: Rc<dyn Node>,
    pub template: Rc<dyn Node>, // Can be TemplateExpression or NoSubstitutionTemplateLiteral
}

impl Node for TaggedTemplateExpression {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a type assertion expression (e.g. expr as Type)
#[derive(Debug, Clone)]
pub struct TypeAssertionExpression {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
    pub type_: Rc<dyn Node>,
}

impl Node for TypeAssertionExpression {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents an 'as' expression (e.g. expr as Type)
#[derive(Debug, Clone)]
pub struct AsExpression {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
    pub type_: Rc<dyn Node>,
}

impl Node for AsExpression {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a non-null expression (e.g. obj!)
#[derive(Debug, Clone)]
pub struct NonNullExpression {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
}

impl Node for NonNullExpression {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a meta property (e.g. new.target)
#[derive(Debug, Clone)]
pub struct MetaProperty {
    pub base: NodeBase,
    pub key_expression: Rc<Identifier>,
    pub name: Rc<Identifier>,
}

impl Node for MetaProperty {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a spread element (e.g. ...arr)
#[derive(Debug, Clone)]
pub struct SpreadElement {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
}

impl Node for SpreadElement {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a yield expression (e.g. yield 5)
#[derive(Debug, Clone)]
pub struct YieldExpression {
    pub base: NodeBase,
    pub expression: Option<Rc<dyn Node>>,
    pub asterisk_token: bool, // For yield*
}

impl Node for YieldExpression {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents an await expression (e.g. await promise)
#[derive(Debug, Clone)]
pub struct AwaitExpression {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
}

impl Node for AwaitExpression {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents an arrow function (e.g. () => expr or (x) => { return x; })
#[derive(Debug, Clone)]
pub struct ArrowFunction {
    pub base: NodeBase,
    pub parameters: Vec<Rc<dyn Node>>,
    pub body: Rc<dyn Node>, // Can be Block or Expression
    pub return_type: Option<Rc<dyn Node>>,
    pub type_parameters: Option<Vec<Rc<dyn Node>>>,
}

impl Node for ArrowFunction {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a delete expression (e.g. delete obj.prop)
#[derive(Debug, Clone)]
pub struct DeleteExpression {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
}

impl Node for DeleteExpression {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a typeof expression (e.g. typeof x)
#[derive(Debug, Clone)]
pub struct TypeOfExpression {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
}

impl Node for TypeOfExpression {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a void expression (e.g. void 0)
#[derive(Debug, Clone)]
pub struct VoidExpression {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
}

impl Node for VoidExpression {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a prefix unary expression (e.g. !expr, ++x)
#[derive(Debug, Clone)]
pub struct PrefixUnaryExpression {
    pub base: NodeBase,
    pub operator: Kind,
    pub operand: Rc<dyn Node>,
}

impl Node for PrefixUnaryExpression {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a postfix unary expression (e.g. x++, x--)
#[derive(Debug, Clone)]
pub struct PostfixUnaryExpression {
    pub base: NodeBase,
    pub operand: Rc<dyn Node>,
    pub operator: Kind,
}

impl Node for PostfixUnaryExpression {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a satisfies expression (e.g. value satisfies Type)
#[derive(Debug, Clone)]
pub struct SatisfiesExpression {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
    pub type_: Rc<dyn Node>,
}

impl Node for SatisfiesExpression {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

// ---------------------------------------------------------------------
// Statement Nodes
// ---------------------------------------------------------------------

/// Represents an if statement
#[derive(Debug, Clone)]
pub struct IfStatement {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
    pub then_statement: Rc<dyn Node>,
    pub else_statement: Option<Rc<dyn Node>>,
}

impl Node for IfStatement {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a do...while statement
#[derive(Debug, Clone)]
pub struct DoStatement {
    pub base: NodeBase,
    pub statement: Rc<dyn Node>,
    pub expression: Rc<dyn Node>,
}

impl Node for DoStatement {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a while statement
#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
    pub statement: Rc<dyn Node>,
}

impl Node for WhileStatement {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a for statement (e.g. for (init; cond; incr) statement)
#[derive(Debug, Clone)]
pub struct ForStatement {
    pub base: NodeBase,
    pub initializer: Option<Rc<dyn Node>>,
    pub condition: Option<Rc<dyn Node>>,
    pub incrementor: Option<Rc<dyn Node>>,
    pub statement: Rc<dyn Node>,
}

impl Node for ForStatement {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a for...in statement (e.g. for (key in obj) statement)
#[derive(Debug, Clone)]
pub struct ForInStatement {
    pub base: NodeBase,
    pub initializer: Rc<dyn Node>,
    pub expression: Rc<dyn Node>,
    pub statement: Rc<dyn Node>,
}

impl Node for ForInStatement {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a for...of statement (e.g. for (value of iterable) statement)
#[derive(Debug, Clone)]
pub struct ForOfStatement {
    pub base: NodeBase,
    pub await_modifier: bool,
    pub initializer: Rc<dyn Node>,
    pub expression: Rc<dyn Node>,
    pub statement: Rc<dyn Node>,
}

impl Node for ForOfStatement {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a continue statement
#[derive(Debug, Clone)]
pub struct ContinueStatement {
    pub base: NodeBase,
    pub label: Option<Rc<Identifier>>,
}

impl Node for ContinueStatement {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a break statement
#[derive(Debug, Clone)]
pub struct BreakStatement {
    pub base: NodeBase,
    pub label: Option<Rc<Identifier>>,
}

impl Node for BreakStatement {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a switch statement
#[derive(Debug, Clone)]
pub struct SwitchStatement {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
    pub case_block: Rc<CaseBlock>,
}

impl Node for SwitchStatement {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a case block in a switch statement
#[derive(Debug, Clone)]
pub struct CaseBlock {
    pub base: NodeBase,
    pub clauses: Vec<Rc<dyn Node>>, // Contains CaseClause and DefaultClause
}

impl Node for CaseBlock {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a case clause in a switch statement
#[derive(Debug, Clone)]
pub struct CaseClause {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
    pub statements: Vec<Rc<dyn Node>>,
}

impl Node for CaseClause {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a default clause in a switch statement
#[derive(Debug, Clone)]
pub struct DefaultClause {
    pub base: NodeBase,
    pub statements: Vec<Rc<dyn Node>>,
}

impl Node for DefaultClause {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a labeled statement (e.g. label: statement)
#[derive(Debug, Clone)]
pub struct LabeledStatement {
    pub base: NodeBase,
    pub label: Rc<Identifier>,
    pub statement: Rc<dyn Node>,
}

impl Node for LabeledStatement {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a throw statement (e.g. throw error)
#[derive(Debug, Clone)]
pub struct ThrowStatement {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
}

impl Node for ThrowStatement {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a try statement (e.g. try {...} catch {...} finally {...})
#[derive(Debug, Clone)]
pub struct TryStatement {
    pub base: NodeBase,
    pub try_block: Rc<Block>,
    pub catch_clause: Option<Rc<CatchClause>>,
    pub finally_block: Option<Rc<Block>>,
}

impl Node for TryStatement {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a catch clause in a try statement
#[derive(Debug, Clone)]
pub struct CatchClause {
    pub base: NodeBase,
    pub variable_declaration: Option<Rc<dyn Node>>, // Can be VariableDeclaration or BindingPattern
    pub block: Rc<Block>,
}

impl Node for CatchClause {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a with statement (e.g. with (obj) statement)
#[derive(Debug, Clone)]
pub struct WithStatement {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
    pub statement: Rc<dyn Node>,
}

impl Node for WithStatement {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents an empty statement (just a semicolon)
#[derive(Debug, Clone)]
pub struct EmptyStatement {
    pub base: NodeBase,
}

impl Node for EmptyStatement {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a debugger statement (e.g. debugger;)
#[derive(Debug, Clone)]
pub struct DebuggerStatement {
    pub base: NodeBase,
}

impl Node for DebuggerStatement {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

// ---------------------------------------------------------------------
// Declaration Nodes
// ---------------------------------------------------------------------

/// Represents a class declaration
#[derive(Debug, Clone)]
pub struct ClassDeclaration {
    pub base: NodeBase,
    pub name: Option<Rc<Identifier>>,
    pub type_parameters: Option<Vec<Rc<dyn Node>>>,
    pub heritage_clauses: Vec<Rc<dyn Node>>, // Extends and Implements clauses
    pub members: Vec<Rc<dyn Node>>,
}

impl Node for ClassDeclaration {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a class expression (e.g. const MyClass = class {...})
#[derive(Debug, Clone)]
pub struct ClassExpression {
    pub base: NodeBase,
    pub name: Option<Rc<Identifier>>,
    pub type_parameters: Option<Vec<Rc<dyn Node>>>,
    pub heritage_clauses: Vec<Rc<dyn Node>>, // Extends and Implements clauses
    pub members: Vec<Rc<dyn Node>>,
}

impl Node for ClassExpression {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a class constructor
#[derive(Debug, Clone)]
pub struct Constructor {
    pub base: NodeBase,
    pub parameters: Vec<Rc<dyn Node>>,
    pub body: Option<Rc<Block>>,
}

impl Node for Constructor {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a property declaration in a class
#[derive(Debug, Clone)]
pub struct PropertyDeclaration {
    pub base: NodeBase,
    pub name: Rc<dyn Node>, // Can be Identifier or ComputedPropertyName
    pub type_annotation: Option<Rc<dyn Node>>,
    pub initializer: Option<Rc<dyn Node>>,
    pub is_static: bool,
    pub is_readonly: bool,
}

impl Node for PropertyDeclaration {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a method declaration in a class
#[derive(Debug, Clone)]
pub struct MethodDeclaration {
    pub base: NodeBase,
    pub name: Rc<dyn Node>, // Can be Identifier or ComputedPropertyName
    pub parameters: Vec<Rc<dyn Node>>,
    pub body: Option<Rc<Block>>,
    pub return_type: Option<Rc<dyn Node>>,
    pub is_static: bool,
    pub is_async: bool,
    pub is_generator: bool,
}

impl Node for MethodDeclaration {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a getter accessor in a class or object
#[derive(Debug, Clone)]
pub struct GetAccessor {
    pub base: NodeBase,
    pub name: Rc<dyn Node>, // Can be Identifier or ComputedPropertyName
    pub parameters: Vec<Rc<dyn Node>>,
    pub body: Option<Rc<Block>>,
    pub return_type: Option<Rc<dyn Node>>,
    pub is_static: bool,
}

impl Node for GetAccessor {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a setter accessor in a class or object
#[derive(Debug, Clone)]
pub struct SetAccessor {
    pub base: NodeBase,
    pub name: Rc<dyn Node>, // Can be Identifier or ComputedPropertyName
    pub parameters: Vec<Rc<dyn Node>>,
    pub body: Option<Rc<Block>>,
    pub is_static: bool,
}

impl Node for SetAccessor {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a computed property name (e.g. [expr])
#[derive(Debug, Clone)]
pub struct ComputedPropertyName {
    pub base: NodeBase,
    pub expression: Rc<dyn Node>,
}

impl Node for ComputedPropertyName {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a type alias declaration (e.g. type Foo = Bar)
#[derive(Debug, Clone)]
pub struct TypeAliasDeclaration {
    pub base: NodeBase,
    pub name: Rc<Identifier>,
    pub type_parameters: Option<Vec<Rc<dyn Node>>>,
    pub type_: Rc<dyn Node>,
}

impl Node for TypeAliasDeclaration {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents an enum declaration
#[derive(Debug, Clone)]
pub struct EnumDeclaration {
    pub base: NodeBase,
    pub name: Rc<Identifier>,
    pub members: Vec<Rc<EnumMember>>,
}

impl Node for EnumDeclaration {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents an enum member
#[derive(Debug, Clone)]
pub struct EnumMember {
    pub base: NodeBase,
    pub name: Rc<dyn Node>, // Can be Identifier or StringLiteral
    pub initializer: Option<Rc<dyn Node>>,
}

impl Node for EnumMember {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

// ---------------------------------------------------------------------
// Type Nodes
// ---------------------------------------------------------------------

/// Represents a type parameter (e.g. T in Array<T>)
#[derive(Debug, Clone)]
pub struct TypeParameter {
    pub base: NodeBase,
    pub name: Rc<Identifier>,
    pub constraint: Option<Rc<dyn Node>>,
    pub default: Option<Rc<dyn Node>>,
}

impl Node for TypeParameter {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents an array type (e.g. number[], Array<string>)
#[derive(Debug, Clone)]
pub struct ArrayType {
    pub base: NodeBase,
    pub element_type: Rc<dyn Node>,
}

impl Node for ArrayType {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a tuple type (e.g. [number, string])
#[derive(Debug, Clone)]
pub struct TupleType {
    pub base: NodeBase,
    pub element_types: Vec<Rc<dyn Node>>,
}

impl Node for TupleType {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents an intersection type (e.g. A & B & C)
#[derive(Debug, Clone)]
pub struct IntersectionType {
    pub base: NodeBase,
    pub types: Vec<Rc<dyn Node>>,
}

impl Node for IntersectionType {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a function type (e.g. (a: number, b: string) => void)
#[derive(Debug, Clone)]
pub struct FunctionType {
    pub base: NodeBase,
    pub parameters: Vec<Rc<dyn Node>>,
    pub return_type: Rc<dyn Node>,
    pub type_parameters: Option<Vec<Rc<dyn Node>>>,
}

impl Node for FunctionType {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a constructor type (e.g. new (a: number) => MyClass)
#[derive(Debug, Clone)]
pub struct ConstructorType {
    pub base: NodeBase,
    pub parameters: Vec<Rc<dyn Node>>,
    pub return_type: Rc<dyn Node>,
    pub type_parameters: Option<Vec<Rc<dyn Node>>>,
}

impl Node for ConstructorType {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a method signature in an interface
#[derive(Debug, Clone)]
pub struct MethodSignature {
    pub base: NodeBase,
    pub name: Rc<dyn Node>, // Can be Identifier or ComputedPropertyName
    pub parameters: Vec<Rc<dyn Node>>,
    pub return_type: Option<Rc<dyn Node>>,
    pub type_parameters: Option<Vec<Rc<dyn Node>>>,
    pub is_optional: bool,
}

impl Node for MethodSignature {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a call signature in an interface (e.g. (x: number): void)
#[derive(Debug, Clone)]
pub struct CallSignature {
    pub base: NodeBase,
    pub parameters: Vec<Rc<dyn Node>>,
    pub return_type: Option<Rc<dyn Node>>,
    pub type_parameters: Option<Vec<Rc<dyn Node>>>,
}

impl Node for CallSignature {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a construct signature in an interface (e.g. new (x: number): MyClass)
#[derive(Debug, Clone)]
pub struct ConstructSignature {
    pub base: NodeBase,
    pub parameters: Vec<Rc<dyn Node>>,
    pub return_type: Option<Rc<dyn Node>>,
    pub type_parameters: Option<Vec<Rc<dyn Node>>>,
}

impl Node for ConstructSignature {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents an index signature in an interface (e.g. [key: string]: any)
#[derive(Debug, Clone)]
pub struct IndexSignature {
    pub base: NodeBase,
    pub parameters: Vec<Rc<dyn Node>>,
    pub return_type: Rc<dyn Node>,
    pub is_readonly: bool,
}

impl Node for IndexSignature {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

// ---------------------------------------------------------------------
// Binding Patterns
// ---------------------------------------------------------------------

/// Represents an array binding pattern (e.g. [a, b, ...rest] = arr)
#[derive(Debug, Clone)]
pub struct ArrayBindingPattern {
    pub base: NodeBase,
    pub elements: Vec<Rc<dyn Node>>, // Can contain BindingElement or OmittedExpression
}

impl Node for ArrayBindingPattern {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

// ---------------------------------------------------------------------
// Module-Related Nodes
// ---------------------------------------------------------------------

/// Represents an import declaration (e.g. import { a, b } from 'module')
#[derive(Debug, Clone)]
pub struct ImportDeclaration {
    pub base: NodeBase,
    pub import_clause: Option<Rc<dyn Node>>, // ImportClause
    pub module_specifier: Rc<dyn Node>, // StringLiteral
}

impl Node for ImportDeclaration {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents an import clause (e.g. the part between import and from)
#[derive(Debug, Clone)]
pub struct ImportClause {
    pub base: NodeBase,
    pub name: Option<Rc<Identifier>>, // Default import
    pub named_bindings: Option<Rc<dyn Node>>, // NamedImports or NamespaceImport
}

impl Node for ImportClause {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents a namespace import (e.g. import * as ns from 'module')
#[derive(Debug, Clone)]
pub struct NamespaceImport {
    pub base: NodeBase,
    pub name: Rc<Identifier>,
}

impl Node for NamespaceImport {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents named imports (e.g. import { a, b as c } from 'module')
#[derive(Debug, Clone)]
pub struct NamedImports {
    pub base: NodeBase,
    pub elements: Vec<Rc<ImportSpecifier>>,
}

impl Node for NamedImports {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents an import specifier (e.g. a or b as c in import { a, b as c })
#[derive(Debug, Clone)]
pub struct ImportSpecifier {
    pub base: NodeBase,
    pub name: Rc<Identifier>, // The imported name
    pub property_name: Option<Rc<Identifier>>, // The name in the module, if renamed
}

impl Node for ImportSpecifier {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents an export declaration (e.g. export { a, b } or export * from 'module')
#[derive(Debug, Clone)]
pub struct ExportDeclaration {
    pub base: NodeBase,
    pub export_clause: Option<Rc<NamedExports>>,
    pub module_specifier: Option<Rc<dyn Node>>, // StringLiteral if exporting from another module
    pub is_type_only: bool,
}

impl Node for ExportDeclaration {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents named exports (e.g. export { a, b as c })
#[derive(Debug, Clone)]
pub struct NamedExports {
    pub base: NodeBase,
    pub elements: Vec<Rc<ExportSpecifier>>,
}

impl Node for NamedExports {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// Represents an export specifier (e.g. a or b as c in export { a, b as c })
#[derive(Debug, Clone)]
pub struct ExportSpecifier {
    pub base: NodeBase,
    pub name: Rc<Identifier>, // The exported name
    pub property_name: Option<Rc<Identifier>>, // The local name, if renamed
}

impl Node for ExportSpecifier {
    fn kind(&self) -> Kind { self.base.kind() }
    fn flags(&self) -> NodeFlags { self.base.flags() }
    fn pos(&self) -> usize { self.base.pos() }
    fn end(&self) -> usize { self.base.end() }
    fn loc(&self) -> TextRange { self.base.loc() }
    fn set_flags(&mut self, flags: NodeFlags) { self.base.set_flags(flags) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}