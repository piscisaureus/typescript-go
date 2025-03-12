// Corresponds to internal/ast/kind.go in the Go implementation

use std::fmt;

/// Represents different kinds of AST nodes and tokens.
/// In Go this is defined as:
/// ```go
/// type Kind int16
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    // Special tokens
    Unknown,
    EndOfFile,
    SingleLineCommentTrivia,
    MultiLineCommentTrivia,
    NewLineTrivia,
    WhitespaceTrivia,

    // Literals
    NumericLiteral,
    StringLiteral,

    // Punctuation
    OpenBraceToken,    // {
    CloseBraceToken,   // }
    OpenParenToken,    // (
    CloseParenToken,   // )
    OpenBracketToken,  // [
    CloseBracketToken, // ]
    DotToken,          // .
    DotDotDotToken,    // ...
    SemicolonToken,    // ;
    CommaToken,        // ,
    LessThanToken,     // <
    GreaterThanToken,  // >
    PlusToken,         // +
    MinusToken,        // -
    AsteriskToken,     // *
    SlashToken,        // /
    EqualsToken,       // =
    ColonToken,        // :

    // Keywords
    FunctionKeyword,
    ReturnKeyword,
    VarKeyword,
    LetKeyword,
    ConstKeyword,
    IfKeyword,
    ForKeyword,
    WhileKeyword,
    StringKeyword,
    NumberKeyword,
    TrueKeyword,
    FalseKeyword,

    // Identifiers
    Identifier,

    // Nodes
    SourceFile,
    FunctionDeclaration,
    FunctionExpression,
    Parameter,
    Block,
    ReturnStatement,
    ExpressionStatement,
    BinaryExpression,
    CallExpression,
    ArrayLiteralExpression,
    PropertyAccessExpression,
    ObjectLiteralExpression,
    PropertyAssignment,
    SpreadAssignment,
    VariableDeclaration,
    VariableDeclarationList,
    VariableStatement,
    TypeReference,
    TypeLiteral,
    PropertySignature,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
