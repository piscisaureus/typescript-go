// Corresponds to internal/ast/kind.go in the Go implementation
// Go implementation defines this as type Kind int16

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
    RegularExpressionLiteral,
    NoSubstitutionTemplateLiteral,
    TemplateHead,
    TemplateMiddle,
    TemplateTail,
    BooleanLiteral,
    NullLiteral,

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
    QuestionToken,     // ?
    QuestionQuestionToken, // ??
    QuestionDotToken,  // ?.
    ExclamationToken,  // !
    ColonToken,        // :
    BarToken,          // |
    BarBarToken,       // ||
    AmpersandToken,    // &
    AmpersandAmpersandToken, // &&
    CaretToken,        // ^
    AtToken,           // @
    BacktickToken,     // `
    
    // Operators
    PlusToken,       // +
    PlusEqualsToken, // +=
    PlusPlusToken,   // ++

    MinusToken,       // -
    MinusEqualsToken, // -=
    MinusMinusToken,  // --

    AsteriskToken,               // *
    AsteriskEqualsToken,         // *=
    AsteriskAsteriskToken,       // **
    AsteriskAsteriskEqualsToken, // **=

    SlashToken,       // /
    SlashEqualsToken, // /=

    PercentToken,     // %
    PercentEqualsToken, // %=
    
    LessThanLessThanToken,       // <<
    LessThanLessThanEqualsToken, // <<=
    GreaterThanGreaterThanToken,       // >>
    GreaterThanGreaterThanEqualsToken, // >>=
    GreaterThanGreaterThanGreaterThanToken,       // >>>
    GreaterThanGreaterThanGreaterThanEqualsToken, // >>>=

    EqualsToken,             // =
    EqualsEqualsToken,       // ==
    EqualsEqualsEqualsToken, // ===
    EqualsGreaterThanToken,  // =>
    ExclamationEqualsToken,       // !=
    ExclamationEqualsEqualsToken, // !==
    
    LessThanEqualsToken,     // <=
    GreaterThanEqualsToken,  // >=

    AmpersandEqualsToken,    // &=
    BarEqualsToken,          // |=
    CaretEqualsToken,        // ^=

    // Keywords
    FunctionKeyword,
    ClassKeyword,
    VarKeyword,
    LetKeyword,
    ConstKeyword,
    IfKeyword,
    ElseKeyword,
    ForKeyword,
    WhileKeyword,
    DoKeyword,
    SwitchKeyword,
    CaseKeyword,
    DefaultKeyword,
    BreakKeyword,
    ContinueKeyword,
    ReturnKeyword,
    YieldKeyword,
    NewKeyword,
    DeleteKeyword,
    TypeofKeyword,
    VoidKeyword,
    ThrowKeyword,
    TryKeyword,
    CatchKeyword,
    FinallyKeyword,
    DebuggerKeyword,
    ImportKeyword,
    ExportKeyword,
    FromKeyword,
    AsKeyword,
    OfKeyword,
    InKeyword,
    InstanceofKeyword,
    ThisKeyword,
    SuperKeyword,
    EnumKeyword,
    ImplementsKeyword,
    InterfaceKeyword,
    NamespaceKeyword,
    ModuleKeyword,
    TypeKeyword,
    DeclareKeyword,
    PublicKeyword,
    PrivateKeyword,
    ProtectedKeyword,
    AbstractKeyword,
    StaticKeyword,
    ReadonlyKeyword,
    AnyKeyword,
    NumberKeyword,
    BigIntKeyword,
    BooleanKeyword,
    StringKeyword,
    SymbolKeyword,
    UnknownKeyword,
    NeverKeyword,
    UndefinedKeyword,
    ObjectKeyword,
    TrueKeyword,
    FalseKeyword,
    NullKeyword,
    AsyncKeyword,
    AwaitKeyword,
    RequireKeyword,
    GetKeyword,
    SetKeyword,
    ExtendsKeyword,
    SatisfiesKeyword,
    
    // Identifiers
    Identifier,

    // Nodes - Expressions
    ParenthesizedExpression,
    ArrayLiteralExpression,
    ObjectLiteralExpression,
    PropertyAccessExpression,
    ElementAccessExpression,
    NewExpression,
    CallExpression,
    BinaryExpression,
    ConditionalExpression,
    TemplateExpression,
    TaggedTemplateExpression,
    TypeAssertionExpression,
    AsExpression,
    NonNullExpression,
    MetaProperty,
    SpreadElement,
    YieldExpression,
    AwaitExpression,
    ClassExpression,
    OmittedExpression,
    FunctionExpression,
    ArrowFunction,
    DeleteExpression,
    TypeOfExpression,
    VoidExpression,
    PrefixUnaryExpression,
    PostfixUnaryExpression,
    SatisfiesExpression,
    
    // Nodes - Statements
    Block,
    VariableStatement,
    ExpressionStatement,
    IfStatement,
    DoStatement,
    WhileStatement,
    ForStatement,
    ForInStatement,
    ForOfStatement,
    ContinueStatement,
    BreakStatement,
    ReturnStatement,
    WithStatement,
    SwitchStatement,
    LabeledStatement,
    ThrowStatement,
    TryStatement,
    DebuggerStatement,
    EmptyStatement,
    
    // Nodes - Declaration
    VariableDeclaration,
    VariableDeclarationList,
    FunctionDeclaration,
    ClassDeclaration,
    InterfaceDeclaration,
    TypeAliasDeclaration,
    EnumDeclaration,
    ModuleDeclaration,
    NamespaceExportDeclaration,
    ImportEqualsDeclaration,
    ImportDeclaration,
    ExportDeclaration,
    ExportAssignment,
    
    // Nodes - Class Elements
    Constructor,
    PropertyDeclaration,
    MethodDeclaration,
    GetAccessor,
    SetAccessor,
    ClassStaticBlockDeclaration,

    // Nodes - Interface and Type Elements
    TypeParameter,
    Parameter,
    PropertySignature,
    PropertyAssignment,
    ShorthandPropertyAssignment,
    SpreadAssignment,
    MethodSignature,
    CallSignature,
    ConstructSignature,
    IndexSignature,
    TypePredicate,
    TypeReference,
    FunctionType,
    ConstructorType,
    TypeQuery,
    TypeLiteral,
    ArrayType,
    TupleType,
    OptionalType,
    RestType,
    UnionType,
    IntersectionType,
    ConditionalType,
    InferType,
    ParenthesizedType,
    ThisType,
    TypeOperator,
    IndexedAccessType,
    MappedType,
    LiteralType,
    ImportType,
    TemplateLiteralType,
    NamedTupleMember,
    
    // Nodes - Module Related
    ImportClause,
    NamespaceImport,
    NamedImports,
    ImportSpecifier,
    ExportSpecifier,
    
    // Nodes - JSX
    JsxElement,
    JsxSelfClosingElement,
    JsxOpeningElement,
    JsxClosingElement,
    JsxFragment,
    JsxOpeningFragment,
    JsxClosingFragment,
    JsxAttribute,
    JsxAttributes,
    JsxSpreadAttribute,
    JsxExpression,
    JsxText,
    
    // Binding Patterns
    ObjectBindingPattern,
    ArrayBindingPattern,
    BindingElement,
    ComputedPropertyName,
    
    // Other
    SourceFile,
    Bundle,
    Decorator,
    CaseBlock,
    CaseClause,
    DefaultClause,
    CatchClause,
    SyntaxList,
    TemplateSpan,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
