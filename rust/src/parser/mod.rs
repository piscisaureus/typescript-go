// Corresponds to internal/parser/parser.go in the Go implementation

use crate::ast::{self, Kind, NodeFlags, TextRange};
use crate::error::{Diagnostic, DiagnosticCode, Result};
use crate::scanner::Scanner;
use std::path::Path;
use std::rc::Rc;

/// Parser context - corresponds to ParsingContext in Go
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsingContext {
    SourceElements,
    BlockStatements,
    SwitchClauses,
    SwitchClauseStatements,
    TypeMembers,
    ClassMembers,
    EnumMembers,
    HeritageClauseElement,
    VariableDeclarations,
    ObjectBindingElements,
    ArrayBindingElements,
    ArgumentExpressions,
    ObjectLiteralMembers,
    Parameters,
    TypeParameters,
    TypeArguments,
}

/// Parser for TypeScript source code
/// Corresponds to Parser struct in internal/parser/parser.go
pub struct Parser {
    // Scanner and token state
    scanner: Scanner,
    token: Kind,

    // Source information
    file_name: String,
    source_text: String,

    // Error tracking
    diagnostics: Vec<Diagnostic>,

    // Parsing state
    language_version: u8,    // Corresponds to languageVersion in Go
    language_variant: u8,    // Corresponds to languageVariant in Go
    _source_file_flags: u32, // Corresponds to sourceFileFlags in Go
    _parsing_context: Vec<ParsingContext>,
    // Node creation
    factory: ast::NodeFactory, // Corresponds to factory in Go
}

impl Parser {
    /// Helper method to create a diagnostic with proper location information
    fn error(&self, code: DiagnosticCode, message: &str) -> Diagnostic {
        let token_pos = self.scanner.token_pos();
        let pos = self.scanner.pos();
        Diagnostic::new(
            code,
            message,
            &self.file_name,
            token_pos,                        // Start position of the token
            pos - token_pos,                  // Length of the token
            self.scanner.get_line_number(),   // Line number (1-based)
            self.scanner.get_column_number(), // Column number (1-based)
        )
    }

    /// Helper method to create a NodeBase with the current token's position information
    fn create_node_base(&self, kind: Kind) -> ast::NodeBase {
        ast::NodeBase::with_pos(kind, self.scanner.token_pos(), self.scanner.pos())
    }

    /// Helper method to track a node's range from start_pos to current position
    fn finish_node(&self, start_pos: usize) -> (usize, usize) {
        (start_pos, self.scanner.pos())
    }

    /// Creates a new parser
    /// Corresponds to InitializeState in Go (but more closely models the Go constructor pattern)
    pub fn new(file_name: &str, source_text: &str) -> Self {
        let mut parser = Self {
            scanner: Scanner::new(source_text),
            token: Kind::Unknown,
            file_name: file_name.to_owned(),
            source_text: source_text.to_owned(),
            diagnostics: Vec::new(),
            language_version: 99, // Latest version (like ScriptTargetLatest)
            language_variant: 0,  // Standard language variant
            _source_file_flags: 0,
            _parsing_context: Vec::new(),
            factory: ast::NodeFactory::new(), // Initialize factory
        };

        parser.initialize_state(file_name, source_text);
        parser
    }

    /// Initialize parser state
    /// Corresponds to InitializeState in Go
    fn initialize_state(&mut self, _file_name: &str, _source_text: &str) {
        // Set scanner language settings
        self.scanner.set_language_version(self.language_version);
        self.scanner.set_language_variant(self.language_variant);

        // Advance to the first token
        self.next_token();
    }

    /// Advance to the next token
    /// Corresponds to nextToken in Go
    fn next_token(&mut self) -> Kind {
        self.token = self.scanner.scan();
        self.token
    }

    /// Parse a source file
    /// Corresponds to parseSourceFileWorker in Go
    pub fn parse_source_file(&mut self) -> Result<Rc<ast::SourceFile>> {
        // Start of parsing source file
        let start_pos = 0;
        let mut statements = Vec::new();

        // Parse statements until end of file
        // This follows the Go implementation's structure
        while self.token != Kind::EndOfFile {
            if let Some(statement) = self.parse_statement()? {
                statements.push(statement);
            }
        }

        // Extract just the filename without path
        let file_name_only = Path::new(&self.file_name)
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| self.file_name.clone());

        // Create a simple line map (in Go this is more complete)
        let mut line_map = vec![0];
        for (pos, ch) in self.source_text.char_indices() {
            if ch == '\n' {
                line_map.push(pos + 1);
            }
        }

        // Create source file node - more closely follows Go's factory pattern
        // In Go, this would use factory.createSourceFile()
        let source_file = Rc::new(ast::SourceFile {
            base: ast::NodeBase {
                kind: Kind::SourceFile,
                flags: NodeFlags::None,
                loc: TextRange::new(start_pos, self.source_text.len()),
            },
            text: self.source_text.clone(),
            file_name: file_name_only,
            language_version: self.language_version, // Use the parser's language version
            statements,
            line_map, // More complete line map implementation
            diagnostics: self.diagnostics.clone(),
        });

        Ok(source_file)
    }

    /// Parse a statement
    /// Corresponds to parseStatement in Go
    fn parse_statement(&mut self) -> Result<Option<Rc<dyn ast::Node>>> {
        match self.token {
            Kind::FunctionKeyword => self
                .parse_function_declaration()
                .map(|f| Some(f as Rc<dyn ast::Node>)),
            Kind::InterfaceKeyword => self
                .parse_interface_declaration()
                .map(|i| Some(i as Rc<dyn ast::Node>)),
            Kind::ReturnKeyword => self
                .parse_return_statement()
                .map(|r| Some(r as Rc<dyn ast::Node>)),
            Kind::VarKeyword | Kind::LetKeyword | Kind::ConstKeyword => self
                .parse_variable_statement(self.token)
                .map(|v| Some(v as Rc<dyn ast::Node>)),
            Kind::SemicolonToken => {
                // Skip empty statements
                self.next_token();
                Ok(None)
            }
            _ => {
                // Try to parse as an expression statement
                self.parse_expression_statement()
                    .map(|e| Some(e as Rc<dyn ast::Node>))
            }
        }
    }

    /// Parse a variable statement (var, let, const)
    /// Corresponds to parseVariableStatement in Go
    fn parse_variable_statement(
        &mut self,
        declaration_kind: Kind,
    ) -> Result<Rc<ast::VariableStatement>> {
        // Save the declaration kind (var, let, const)
        if declaration_kind != Kind::VarKeyword
            && declaration_kind != Kind::LetKeyword
            && declaration_kind != Kind::ConstKeyword
        {
            return Err(Diagnostic::new(
                DiagnosticCode::SyntaxError,
                "Expected 'var', 'let', or 'const'",
                &self.file_name,
                0, // TODO: Get actual position
                0, // TODO: Get actual length
                0, // TODO: Get actual line
                0, // TODO: Get actual column
            ));
        }

        // Consume the var/let/const keyword
        self.next_token();

        // Parse the variable declaration list
        let declaration_list = self.parse_variable_declaration_list()?;

        // Expect ';'
        if self.token == Kind::SemicolonToken {
            self.next_token();
        }

        // Create the variable statement
        let var_stmt = Rc::new(ast::VariableStatement {
            base: ast::NodeBase::new(Kind::VariableStatement),
            declaration_list,
            declaration_kind,
        });

        Ok(var_stmt)
    }

    /// Parse a variable declaration list
    /// Corresponds to parseVariableDeclarationList in Go
    fn parse_variable_declaration_list(&mut self) -> Result<Rc<ast::VariableDeclarationList>> {
        let mut declarations = Vec::new();

        // Parse the first declaration (required)
        let decl = self.parse_variable_declaration()?;
        declarations.push(decl);

        // Parse additional declarations separated by commas
        while self.token == Kind::CommaToken {
            self.next_token(); // Consume the comma
            let decl = self.parse_variable_declaration()?;
            declarations.push(decl);
        }

        // Create the declaration list
        let decl_list = Rc::new(ast::VariableDeclarationList {
            base: ast::NodeBase::new(Kind::VariableDeclarationList),
            declarations,
        });

        Ok(decl_list)
    }

    /// Parse a single variable declaration
    /// Corresponds to parseVariableDeclaration in Go
    /// Updated to support destructuring patterns
    fn parse_variable_declaration(&mut self) -> Result<Rc<ast::VariableDeclaration>> {
        // Check for object destructuring pattern
        if self.token == Kind::OpenBraceToken {
            // Parse object binding pattern
            let object_binding = self.parse_object_binding_pattern()?;

            // We don't support type annotations for destructuring patterns yet
            let type_annotation = None;

            // Parse initializer (required for destructuring)
            let initializer = if self.token == Kind::EqualsToken {
                self.next_token(); // Consume the equals

                // Parse the initializer expression
                let init_expr = self.parse_expression()?;
                Some(init_expr)
            } else {
                return Err(self.error(
                    DiagnosticCode::SyntaxError,
                    "Destructuring declarations must have an initializer",
                ));
            };

            // Create a placeholder name for compatibility
            let placeholder_name = Rc::new(ast::Identifier {
                base: ast::NodeBase::new(Kind::Identifier),
                text: "_destructured".to_owned(),
            });

            // Create the variable declaration with binding pattern
            let var_decl = Rc::new(ast::VariableDeclaration {
                base: ast::NodeBase::new(Kind::VariableDeclaration),
                name: placeholder_name,
                binding_name: Some(object_binding as Rc<dyn ast::Node>),
                initializer,
                type_annotation,
            });

            return Ok(var_decl);
        }

        // Regular variable declaration case - parse the variable name
        if self.token != Kind::Identifier {
            return Err(self.error(DiagnosticCode::SyntaxError, "Expected variable name"));
        }

        let name_text = self.scanner.token_text().to_owned();
        let name = Rc::new(ast::Identifier {
            base: ast::NodeBase::new(Kind::Identifier),
            text: name_text,
        });
        self.next_token();

        // Parse optional type annotation
        let mut type_annotation = None;
        if self.token == Kind::ColonToken {
            self.next_token(); // Consume the colon

            // Parse the type annotation
            if self.token == Kind::StringKeyword
                || self.token == Kind::NumberKeyword
                || self.token == Kind::Identifier
            {
                let type_text = self.scanner.token_text().to_owned();
                let type_ident = Rc::new(ast::Identifier {
                    base: ast::NodeBase::new(Kind::Identifier),
                    text: type_text,
                });
                self.next_token();

                // Check for array type (e.g., string[])
                let is_array_type = if self.token == Kind::OpenBracketToken {
                    self.next_token(); // Consume '['

                    if self.token != Kind::CloseBracketToken {
                        return Err(self.error(DiagnosticCode::SyntaxError, "Expected ']'"));
                    }

                    self.next_token(); // Consume ']'
                    true
                } else {
                    false
                };

                let type_ref = Rc::new(ast::TypeReference {
                    base: ast::NodeBase::new(Kind::TypeReference),
                    type_name: type_ident,
                    is_array_type,
                });

                type_annotation = Some(type_ref as Rc<dyn ast::Node>);
            } else {
                return Err(Diagnostic::new(
                    DiagnosticCode::SyntaxError,
                    "Expected type annotation after colon",
                    &self.file_name,
                    0, // TODO: Get actual position
                    0, // TODO: Get actual length
                    0, // TODO: Get actual line
                    0, // TODO: Get actual column
                ));
            }
        }

        // Parse optional initializer
        let mut initializer = None;
        if self.token == Kind::EqualsToken {
            self.next_token(); // Consume the equals

            // Parse the initializer expression
            let init_expr = self.parse_expression()?;
            initializer = Some(init_expr);
        }

        // Create the variable declaration
        let var_decl = Rc::new(ast::VariableDeclaration {
            base: ast::NodeBase::new(Kind::VariableDeclaration),
            name,
            binding_name: None,
            initializer,
            type_annotation,
        });

        Ok(var_decl)
    }

    /// Parse an object binding pattern (destructuring) like { a, b, c } or { x: y }
    /// New method, not directly corresponding to a single Go method
    fn parse_object_binding_pattern(&mut self) -> Result<Rc<ast::ObjectBindingPattern>> {
        // Save the start position
        let start_pos = self.scanner.token_pos();

        // Expect '{'
        if self.token != Kind::OpenBraceToken {
            return Err(self.error(
                DiagnosticCode::SyntaxError,
                "Expected '{' for object binding pattern",
            ));
        }
        self.next_token(); // Consume '{'

        let mut elements = Vec::new();

        // Parse binding elements until '}'
        while self.token != Kind::CloseBraceToken && self.token != Kind::EndOfFile {
            let element = self.parse_binding_element()?;
            elements.push(element);

            // Expect comma between elements
            if self.token == Kind::CommaToken {
                self.next_token(); // Consume ','
            } else if self.token != Kind::CloseBraceToken {
                return Err(self.error(
                    DiagnosticCode::SyntaxError,
                    "Expected ',' or '}' after binding element",
                ));
            }
        }

        // Expect '}'
        if self.token != Kind::CloseBraceToken {
            return Err(self.error(
                DiagnosticCode::SyntaxError,
                "Expected '}' to close object binding pattern",
            ));
        }

        // Create and return object binding pattern node
        let mut base = ast::NodeBase::new(Kind::ObjectBindingPattern);
        let end_pos = self.scanner.pos();
        base.set_pos(start_pos, end_pos);

        self.next_token(); // Consume '}'

        let binding_pattern = Rc::new(ast::ObjectBindingPattern { base, elements });

        Ok(binding_pattern)
    }

    /// Parse a binding element within an object binding pattern
    /// For example: 'a' or 'x: y' or 'z = defaultValue'
    fn parse_binding_element(&mut self) -> Result<Rc<ast::BindingElement>> {
        // Save the start position
        let start_pos = self.scanner.token_pos();

        // Parse property name
        if self.token != Kind::Identifier {
            return Err(self.error(
                DiagnosticCode::SyntaxError,
                "Expected identifier in binding pattern",
            ));
        }

        let property_text = self.scanner.token_text().to_owned();
        let property_name = Rc::new(ast::Identifier {
            base: ast::NodeBase::new(Kind::Identifier),
            text: property_text,
        });
        self.next_token();

        // Check for binding name (if different from property name)
        let (name, property_name_opt) = if self.token == Kind::ColonToken {
            self.next_token(); // Consume ':'

            // Parse the binding name
            if self.token != Kind::Identifier {
                return Err(self.error(
                    DiagnosticCode::SyntaxError,
                    "Expected identifier after ':' in binding pattern",
                ));
            }

            let name_text = self.scanner.token_text().to_owned();
            let name = Rc::new(ast::Identifier {
                base: ast::NodeBase::new(Kind::Identifier),
                text: name_text,
            });
            self.next_token();

            (name, Some(property_name))
        } else {
            // Property name is the same as binding name
            (property_name.clone(), None)
        };

        // Check for default value
        let initializer = if self.token == Kind::EqualsToken {
            self.next_token(); // Consume '='

            // Parse the initializer expression
            let init_expr = self.parse_expression()?;
            Some(init_expr)
        } else {
            None
        };

        // Create and return binding element node
        let mut base = ast::NodeBase::new(Kind::BindingElement);
        let end_pos = self.scanner.pos();
        base.set_pos(start_pos, end_pos);

        let binding_element = Rc::new(ast::BindingElement {
            base,
            name,
            property_name: property_name_opt,
            initializer,
        });

        Ok(binding_element)
    }

    /// Parse a function declaration
    /// Corresponds to parseFunctionDeclaration in Go
    fn parse_function_declaration(&mut self) -> Result<Rc<ast::FunctionDeclaration>> {
        // Expect 'function' keyword
        if self.token != Kind::FunctionKeyword {
            return Err(Diagnostic::new(
                DiagnosticCode::SyntaxError,
                "Expected 'function' keyword",
                &self.file_name,
                0, // TODO: Get actual position
                0, // TODO: Get actual length
                0, // TODO: Get actual line
                0, // TODO: Get actual column
            ));
        }
        self.next_token();

        // Parse function name
        let name = if self.token == Kind::Identifier {
            let name_text = self.scanner.token_text().to_owned();
            let identifier = Rc::new(ast::Identifier {
                base: ast::NodeBase::new(Kind::Identifier),
                text: name_text,
            });
            self.next_token();
            Some(identifier)
        } else {
            None
        };

        // Parse parameter list
        let (parameters, _) = self.parse_parameter_list()?;

        // Parse return type (if any)
        let return_type = if self.token == Kind::ColonToken {
            self.next_token();
            println!("Parsing return type, token: {:?}", self.token); // DEBUG: not in Go

            // Use the new parse_type function
            let type_node = self.parse_type()?;
            println!("Return type parsed, token now: {:?}", self.token); // DEBUG: not in Go
            Some(type_node)
        } else {
            None
        };

        // Parse function body
        let body = if self.token == Kind::OpenBraceToken {
            Some(self.parse_block()?)
        } else {
            // Function declaration requires a body
            return Err(Diagnostic::new(
                DiagnosticCode::SyntaxError,
                "Function implementation is missing or not immediately following the declaration",
                &self.file_name,
                0, // TODO: Get actual position
                0, // TODO: Get actual length
                0, // TODO: Get actual line
                0, // TODO: Get actual column
            ));
        };

        // Create function declaration
        let func_decl = Rc::new(ast::FunctionDeclaration {
            base: ast::NodeBase::new(Kind::FunctionDeclaration),
            name,
            parameters,
            return_type,
            body,
        });

        Ok(func_decl)
    }

    /// Parse a function expression (anonymous function)
    /// Similar to parse_function_declaration but returns a FunctionExpression
    fn parse_function_expression(&mut self) -> Result<Rc<dyn ast::Node>> {
        println!("Parsing function expression, token: {:?}", self.token); // DEBUG

        // Expect 'function' keyword
        if self.token != Kind::FunctionKeyword {
            return Err(Diagnostic::new(
                DiagnosticCode::SyntaxError,
                "Expected 'function' keyword",
                &self.file_name,
                0, // TODO: Get actual position
                0, // TODO: Get actual length
                0, // TODO: Get actual line
                0, // TODO: Get actual column
            ));
        }
        self.next_token();
        println!("After function keyword, token: {:?}", self.token); // DEBUG

        // Parse optional function name (for named function expressions)
        let name = if self.token == Kind::Identifier {
            let name_text = self.scanner.token_text().to_owned();
            println!("Function name: {}", name_text); // DEBUG
            let identifier = Rc::new(ast::Identifier {
                base: ast::NodeBase::new(Kind::Identifier),
                text: name_text,
            });
            self.next_token();
            Some(identifier)
        } else {
            None
        };

        // Parse parameter list
        println!("Before parameter list, token: {:?}", self.token); // DEBUG
        let (parameters, _) = self.parse_parameter_list()?;
        println!("After parameter list, token: {:?}", self.token); // DEBUG

        // Parse return type (if any)
        let return_type = if self.token == Kind::ColonToken {
            self.next_token();
            println!("After return type colon, token: {:?}", self.token); // DEBUG

            // Use the new parse_type function
            let type_node = self.parse_type()?;
            Some(type_node)
        } else {
            None
        };

        // Parse function body
        println!("Before function body, token: {:?}", self.token); // DEBUG
        let body = if self.token == Kind::OpenBraceToken {
            Some(self.parse_block()?)
        } else {
            None
        };
        println!("After function body, token: {:?}", self.token); // DEBUG

        // Create function expression
        let func_expr = Rc::new(ast::FunctionExpression {
            base: ast::NodeBase::new(Kind::FunctionExpression),
            name,
            parameters,
            return_type,
            body,
        });

        println!("Function expression complete, token: {:?}", self.token); // DEBUG
        Ok(func_expr as Rc<dyn ast::Node>)
    }

    /// Parse a parameter list
    /// Corresponds to parseParameterList in Go
    fn parse_parameter_list(&mut self) -> Result<(Vec<Rc<ast::ParameterDeclaration>>, bool)> {
        // Expect '('
        if self.token != Kind::OpenParenToken {
            return Err(Diagnostic::new(
                DiagnosticCode::SyntaxError,
                "Expected '('",
                &self.file_name,
                0, // TODO: Get actual position
                0, // TODO: Get actual length
                0, // TODO: Get actual line
                0, // TODO: Get actual column
            ));
        }
        self.next_token();

        let mut parameters = Vec::new();
        let has_rest_parameter = false;

        // Parse parameters
        println!("Parsing parameters, current token: {:?}", self.token); // DEBUG: not in Go
        while self.token != Kind::CloseParenToken && self.token != Kind::EndOfFile {
            let parameter = self.parse_parameter()?;
            parameters.push(parameter);

            println!("  After parameter parse, current token: {:?}", self.token); // DEBUG: not in Go
            if self.token == Kind::CommaToken {
                self.next_token();
                println!("  Found comma, next token: {:?}", self.token); // DEBUG: not in Go
            } else {
                println!("  No comma found, breaking parameter list"); // DEBUG: not in Go
                break;
            }
        }

        // Expect ')'
        println!("After param list, token: {:?}", self.token); // DEBUG: not in Go
        if self.token != Kind::CloseParenToken {
            return Err(Diagnostic::new(
                DiagnosticCode::SyntaxError,
                "Expected ')'",
                &self.file_name,
                0, // TODO: Get actual position
                0, // TODO: Get actual length
                0, // TODO: Get actual line
                0, // TODO: Get actual column
            ));
        }
        self.next_token();
        println!("After ')', token: {:?}", self.token); // DEBUG: not in Go

        Ok((parameters, has_rest_parameter))
    }

    /// Parse a parameter
    /// Corresponds to parseParameter in Go
    fn parse_parameter(&mut self) -> Result<Rc<ast::ParameterDeclaration>> {
        println!("  Parsing parameter, token: {:?}", self.token); // DEBUG: not in Go

        // Parse parameter name
        if self.token != Kind::Identifier {
            return Err(Diagnostic::new(
                DiagnosticCode::SyntaxError,
                "Expected parameter name",
                &self.file_name,
                0, // TODO: Get actual position
                0, // TODO: Get actual length
                0, // TODO: Get actual line
                0, // TODO: Get actual column
            ));
        }

        let name_text = self.scanner.token_text().to_owned();
        println!("  Parameter name: {}", name_text); // DEBUG: not in Go

        let identifier = Rc::new(ast::Identifier {
            base: ast::NodeBase::new(Kind::Identifier),
            text: name_text,
        });
        self.next_token();
        println!("  After identifier, token: {:?}", self.token); // DEBUG: not in Go

        // Parse type annotation (if any)
        let type_annotation = if self.token == Kind::ColonToken {
            self.next_token();
            println!("  Found colon, parsing type, token: {:?}", self.token); // DEBUG: not in Go

            // Parse the type
            let type_node = self.parse_type()?;
            Some(type_node)
        } else {
            None
        };

        // Create parameter declaration
        let param_decl = Rc::new(ast::ParameterDeclaration {
            base: ast::NodeBase::new(Kind::Parameter),
            name: identifier,
            type_annotation,
        });

        Ok(param_decl)
    }

    /// Parse a type annotation
    /// Corresponds to parts of parseTypeReference and parseType in internal/parser/parser.go
    /// Extended to handle object type literals which is implemented differently in the Go version
    fn parse_type(&mut self) -> Result<Rc<dyn ast::Node>> {
        println!("  Parsing type, token: {:?}", self.token); // DEBUG

        // Handle primitive types and type references
        if self.token == Kind::StringKeyword
            || self.token == Kind::NumberKeyword
            || self.token == Kind::Identifier
        {
            let type_text = self.scanner.token_text().to_owned();
            let type_identifier = Rc::new(ast::Identifier {
                base: ast::NodeBase::new(Kind::Identifier),
                text: type_text,
            });

            self.next_token();

            // Check if this is an array type (has [] at the end)
            let is_array_type = if self.token == Kind::OpenBracketToken {
                self.next_token(); // consume '['

                // Check for and consume ']'
                if self.token != Kind::CloseBracketToken {
                    return Err(Diagnostic::new(
                        DiagnosticCode::SyntaxError,
                        "Expected ']' after '['",
                        &self.file_name,
                        0, // TODO: Get actual position
                        0, // TODO: Get actual length
                        0, // TODO: Get actual line
                        0, // TODO: Get actual column
                    ));
                }

                self.next_token(); // consume ']'
                true
            } else {
                false
            };

            let type_reference = Rc::new(ast::TypeReference {
                base: ast::NodeBase::new(Kind::TypeReference),
                type_name: type_identifier,
                is_array_type,
            });

            println!("  Basic type parsed, token now: {:?}", self.token); // DEBUG
            Ok(type_reference as Rc<dyn ast::Node>)
        }
        // Handle object type literals: { name: string; age: number }
        else if self.token == Kind::OpenBraceToken {
            println!("  Parsing object type literal"); // DEBUG
            self.parse_type_literal()
        } else {
            return Err(Diagnostic::new(
                DiagnosticCode::SyntaxError,
                "Expected type annotation",
                &self.file_name,
                0, // TODO: Get actual position
                0, // TODO: Get actual length
                0, // TODO: Get actual line
                0, // TODO: Get actual column
            ));
        }
    }

    /// Parse a block of statements
    /// Corresponds to parseBlock in Go
    fn parse_block(&mut self) -> Result<Rc<ast::Block>> {
        // Expect '{'
        if self.token != Kind::OpenBraceToken {
            return Err(Diagnostic::new(
                DiagnosticCode::SyntaxError,
                "Expected '{'",
                &self.file_name,
                0, // TODO: Get actual position
                0, // TODO: Get actual length
                0, // TODO: Get actual line
                0, // TODO: Get actual column
            ));
        }
        self.next_token();

        let mut statements = Vec::new();

        // Parse statements in the block
        while self.token != Kind::CloseBraceToken && self.token != Kind::EndOfFile {
            if let Some(statement) = self.parse_statement()? {
                statements.push(statement);
            }
        }

        // Expect '}'
        if self.token != Kind::CloseBraceToken {
            return Err(Diagnostic::new(
                DiagnosticCode::SyntaxError,
                "Expected '}'",
                &self.file_name,
                0, // TODO: Get actual position
                0, // TODO: Get actual length
                0, // TODO: Get actual line
                0, // TODO: Get actual column
            ));
        }
        self.next_token();

        // Create block
        let block = Rc::new(ast::Block {
            base: ast::NodeBase::new(Kind::Block),
            statements,
        });

        Ok(block)
    }

    /// Parse a return statement
    /// Corresponds to parseReturnStatement in Go
    fn parse_return_statement(&mut self) -> Result<Rc<ast::ReturnStatement>> {
        // Expect 'return' keyword
        if self.token != Kind::ReturnKeyword {
            return Err(Diagnostic::new(
                DiagnosticCode::SyntaxError,
                "Expected 'return' keyword",
                &self.file_name,
                0, // TODO: Get actual position
                0, // TODO: Get actual length
                0, // TODO: Get actual line
                0, // TODO: Get actual column
            ));
        }
        self.next_token();

        // Parse return expression (if any)
        let expression = if self.token != Kind::SemicolonToken {
            Some(self.parse_expression()?)
        } else {
            None
        };

        // Expect ';'
        if self.token == Kind::SemicolonToken {
            self.next_token();
        }

        // Create return statement
        let return_stmt = Rc::new(ast::ReturnStatement {
            base: ast::NodeBase::new(Kind::ReturnStatement),
            expression,
        });

        Ok(return_stmt)
    }

    /// Parse an expression statement
    /// Corresponds to parseExpressionStatement in Go
    fn parse_expression_statement(&mut self) -> Result<Rc<ast::ExpressionStatement>> {
        let expression = self.parse_expression()?;

        // Expect ';'
        if self.token == Kind::SemicolonToken {
            self.next_token();
        }

        // Create expression statement
        let expr_stmt = Rc::new(ast::ExpressionStatement {
            base: ast::NodeBase::new(Kind::ExpressionStatement),
            expression,
        });

        Ok(expr_stmt)
    }

    /// Parse an expression
    /// Corresponds to parseExpression in Go
    fn parse_expression(&mut self) -> Result<Rc<dyn ast::Node>> {
        self.parse_binary_expression()
    }

    /// Parse a binary expression
    /// Corresponds to parseBinaryExpression in Go
    fn parse_binary_expression(&mut self) -> Result<Rc<dyn ast::Node>> {
        // Parse the left operand
        let mut left = self.parse_primary_expression()?;

        // Continue parsing binary operators as long as they appear
        while self.token == Kind::PlusToken {
            let operator = self.token;
            self.next_token();

            // Parse the right operand
            let right = self.parse_primary_expression()?;

            // Create a binary expression with the left and right operands
            left = Rc::new(ast::BinaryExpression {
                base: ast::NodeBase::new(Kind::BinaryExpression),
                left,
                operator_token: operator,
                right,
            }) as Rc<dyn ast::Node>;
        }

        Ok(left)
    }

    /// Parse a primary expression
    /// Corresponds to parsePrimaryExpression in Go
    fn parse_primary_expression(&mut self) -> Result<Rc<dyn ast::Node>> {
        let mut expression = match self.token {
            Kind::Identifier => {
                // Save the token position
                let token_pos = self.scanner.token_pos();
                let name_text = self.scanner.token_text().to_owned();

                // Special handling for boolean literals (true/false)
                if name_text == "true" || name_text == "false" {
                    let value = name_text == "true";
                    let kind = if value {
                        Kind::TrueKeyword
                    } else {
                        Kind::FalseKeyword
                    };

                    // Create the boolean literal with position information
                    let mut base = ast::NodeBase::new(kind);
                    let pos_end = self.scanner.pos();
                    base.set_pos(token_pos, pos_end);

                    let boolean_literal = Rc::new(ast::BooleanLiteral { base, value });
                    self.next_token();
                    return Ok(boolean_literal as Rc<dyn ast::Node>);
                }

                // Create identifier with position information
                let mut base = ast::NodeBase::new(Kind::Identifier);
                let pos_end = self.scanner.pos();
                base.set_pos(token_pos, pos_end);

                let identifier = Rc::new(ast::Identifier {
                    base,
                    text: name_text,
                });
                self.next_token();
                identifier as Rc<dyn ast::Node>
            }
            Kind::FunctionKeyword => {
                // Parse function expression (anonymous function)
                self.parse_function_expression()?
            }
            Kind::StringLiteral => {
                // Save the token position
                let token_pos = self.scanner.token_pos();
                let text = self.scanner.token_text().to_owned();

                // Create string literal with position information
                let mut base = ast::NodeBase::new(Kind::StringLiteral);
                let pos_end = self.scanner.pos();
                base.set_pos(token_pos, pos_end);

                let string_literal = Rc::new(ast::StringLiteral { base, text });
                self.next_token();
                string_literal as Rc<dyn ast::Node>
            }
            Kind::NumericLiteral => {
                // Save the token position
                let token_pos = self.scanner.token_pos();
                let text = self.scanner.token_text().to_owned();
                let value = text.parse::<f64>().unwrap_or(0.0);

                // Create numeric literal with position information
                let mut base = ast::NodeBase::new(Kind::NumericLiteral);
                let pos_end = self.scanner.pos();
                base.set_pos(token_pos, pos_end);

                let number_literal = Rc::new(ast::NumericLiteral { base, text, value });
                self.next_token();
                number_literal as Rc<dyn ast::Node>
            }
            Kind::TrueKeyword | Kind::FalseKeyword => {
                // Save the token position
                let token_pos = self.scanner.token_pos();
                let value = self.token == Kind::TrueKeyword;

                // Create boolean literal with position information
                let mut base = ast::NodeBase::new(self.token);
                let pos_end = self.scanner.pos();
                base.set_pos(token_pos, pos_end);

                let boolean_literal = Rc::new(ast::BooleanLiteral { base, value });
                self.next_token();
                boolean_literal as Rc<dyn ast::Node>
            }
            Kind::OpenBracketToken => {
                // Parse array literal
                self.parse_array_literal_expression()?
            }
            Kind::OpenBraceToken => {
                // Parse object literal
                self.parse_object_literal_expression()?
            }
            _ => {
                return Err(Diagnostic::new(
                    DiagnosticCode::SyntaxError,
                    &format!("Unexpected token: {:?}", self.token),
                    &self.file_name,
                    0, // TODO: Get actual position
                    0, // TODO: Get actual length
                    0, // TODO: Get actual line
                    0, // TODO: Get actual column
                ));
            }
        };

        // Handle post-fix expressions (function calls, property access)
        loop {
            match self.token {
                Kind::OpenParenToken => {
                    // Function call
                    expression = self.parse_call_expression(expression)?;
                }
                Kind::DotToken => {
                    // Property access
                    self.next_token(); // Consume '.'

                    // Expect property name (identifier)
                    if self.token != Kind::Identifier {
                        return Err(Diagnostic::new(
                            DiagnosticCode::SyntaxError,
                            "Expected identifier after '.'",
                            &self.file_name,
                            0, // TODO: Get actual position
                            0, // TODO: Get actual length
                            0, // TODO: Get actual line
                            0, // TODO: Get actual column
                        ));
                    }

                    let property_text = self.scanner.token_text().to_owned();
                    let property_name = Rc::new(ast::Identifier {
                        base: ast::NodeBase::new(Kind::Identifier),
                        text: property_text,
                    });
                    self.next_token();

                    // Create property access expression
                    expression = Rc::new(ast::PropertyAccessExpression {
                        base: ast::NodeBase::new(Kind::PropertyAccessExpression),
                        expression,
                        name: property_name,
                    }) as Rc<dyn ast::Node>;
                }
                _ => {
                    // No more post-fix operations, break the loop
                    break;
                }
            }
        }

        Ok(expression)
    }

    /// Parse a call expression
    /// Corresponds to parseCallExpression in Go
    fn parse_call_expression(
        &mut self,
        expression: Rc<dyn ast::Node>,
    ) -> Result<Rc<dyn ast::Node>> {
        // Save the start position (the position of the function expression)
        let start_pos = expression.pos();

        // Expect '('
        if self.token != Kind::OpenParenToken {
            return Err(self.error(DiagnosticCode::SyntaxError, "Expected '('"));
        }
        self.next_token();

        let mut arguments = Vec::new();

        // Parse arguments
        while self.token != Kind::CloseParenToken && self.token != Kind::EndOfFile {
            let argument = self.parse_expression()?;
            arguments.push(argument);

            if self.token == Kind::CommaToken {
                self.next_token();
            } else {
                break;
            }
        }

        // Expect ')'
        if self.token != Kind::CloseParenToken {
            return Err(self.error(DiagnosticCode::SyntaxError, "Expected ')'"));
        }
        self.next_token();

        // The end position is the current scanner position (after the closing paren)
        let end_pos = self.scanner.pos();

        // Create call expression with proper source range
        let mut base = ast::NodeBase::new(Kind::CallExpression);
        base.set_pos(start_pos, end_pos);

        let call_expr = Rc::new(ast::CallExpression {
            base,
            expression,
            arguments,
        });

        Ok(call_expr as Rc<dyn ast::Node>)
    }

    /// Parse an array literal expression
    /// Corresponds to parseArrayLiteralExpression in Go
    fn parse_array_literal_expression(&mut self) -> Result<Rc<dyn ast::Node>> {
        // Expect '['
        if self.token != Kind::OpenBracketToken {
            return Err(Diagnostic::new(
                DiagnosticCode::SyntaxError,
                "Expected '['",
                &self.file_name,
                0, // TODO: Get actual position
                0, // TODO: Get actual length
                0, // TODO: Get actual line
                0, // TODO: Get actual column
            ));
        }
        self.next_token();

        let mut elements = Vec::new();

        // Parse elements
        while self.token != Kind::CloseBracketToken && self.token != Kind::EndOfFile {
            let element = self.parse_expression()?;
            elements.push(element);

            if self.token == Kind::CommaToken {
                self.next_token();
            } else {
                break;
            }
        }

        // Expect ']'
        if self.token != Kind::CloseBracketToken {
            return Err(Diagnostic::new(
                DiagnosticCode::SyntaxError,
                "Expected ']'",
                &self.file_name,
                0, // TODO: Get actual position
                0, // TODO: Get actual length
                0, // TODO: Get actual line
                0, // TODO: Get actual column
            ));
        }
        self.next_token();

        // Create array literal expression with position tracking
        let array_expr = Rc::new(ast::ArrayLiteralExpression {
            base: self.create_node_base(Kind::ArrayLiteralExpression),
            elements,
        });

        Ok(array_expr as Rc<dyn ast::Node>)
    }

    /// Parse an object literal expression
    /// Corresponds to parseObjectLiteralExpression in Go
    fn parse_object_literal_expression(&mut self) -> Result<Rc<dyn ast::Node>> {
        // Expect '{'
        if self.token != Kind::OpenBraceToken {
            return Err(Diagnostic::new(
                DiagnosticCode::SyntaxError,
                "Expected '{'",
                &self.file_name,
                0, // TODO: Get actual position
                0, // TODO: Get actual length
                0, // TODO: Get actual line
                0, // TODO: Get actual column
            ));
        }
        self.next_token();

        let mut properties = Vec::new();

        // Parse properties
        while self.token != Kind::CloseBraceToken && self.token != Kind::EndOfFile {
            // Parse property
            let property = self.parse_property_assignment()?;
            properties.push(property);

            if self.token == Kind::CommaToken {
                self.next_token();
            } else {
                break;
            }
        }

        // Expect '}'
        if self.token != Kind::CloseBraceToken {
            return Err(Diagnostic::new(
                DiagnosticCode::SyntaxError,
                "Expected '}'",
                &self.file_name,
                0, // TODO: Get actual position
                0, // TODO: Get actual length
                0, // TODO: Get actual line
                0, // TODO: Get actual column
            ));
        }
        self.next_token();

        // Create object literal expression with position tracking
        let object_expr = Rc::new(ast::ObjectLiteralExpression {
            base: self.create_node_base(Kind::ObjectLiteralExpression),
            properties,
        });

        Ok(object_expr as Rc<dyn ast::Node>)
    }

    /// Parse an interface declaration
    /// Corresponds to parseInterfaceDeclaration in the Go implementation
    fn parse_interface_declaration(&mut self) -> Result<Rc<ast::InterfaceDeclaration>> {
        // Expect 'interface' keyword
        if self.token != Kind::InterfaceKeyword {
            return Err(self.error(DiagnosticCode::SyntaxError, "Expected 'interface' keyword"));
        }

        // Save the interface token position for later use
        let interface_pos = self.scanner.token_pos();
        self.next_token();

        // Parse interface name
        let name = if self.token == Kind::Identifier {
            let name_text = self.scanner.token_text().to_owned();
            let identifier = Rc::new(ast::Identifier {
                base: self.create_node_base(Kind::Identifier),
                text: name_text,
            });
            self.next_token();
            identifier
        } else {
            return Err(self.error(
                DiagnosticCode::SyntaxError,
                "Expected identifier after 'interface' keyword",
            ));
        };

        // Expect '{'
        if self.token != Kind::OpenBraceToken {
            return Err(self.error(
                DiagnosticCode::SyntaxError,
                "Expected '{' after interface name",
            ));
        }
        self.next_token();

        // Parse interface members
        let mut members = Vec::new();

        while self.token != Kind::CloseBraceToken && self.token != Kind::EndOfFile {
            let member = self.parse_property_signature()?;
            members.push(member);

            // Skip optional semicolon or comma
            if self.token == Kind::SemicolonToken || self.token == Kind::CommaToken {
                self.next_token();
            }
        }

        // Expect '}'
        if self.token != Kind::CloseBraceToken {
            return Err(self.error(
                DiagnosticCode::SyntaxError,
                "Expected '}' to close interface declaration",
            ));
        }
        self.next_token();

        // Create and return interface declaration node
        let mut base = ast::NodeBase::new(Kind::InterfaceDeclaration);
        let end_pos = self.scanner.pos();
        base.set_pos(interface_pos, end_pos);

        let interface_decl = Rc::new(ast::InterfaceDeclaration {
            base,
            name,
            members,
        });

        Ok(interface_decl)
    }

    /// Parse a property signature in an interface
    fn parse_property_signature(&mut self) -> Result<Rc<dyn ast::Node>> {
        // Save the property name position
        let prop_pos = self.scanner.token_pos();

        // Parse property name
        let name = if self.token == Kind::Identifier {
            let name_text = self.scanner.token_text().to_owned();
            let identifier = Rc::new(ast::Identifier {
                base: self.create_node_base(Kind::Identifier),
                text: name_text,
            });
            self.next_token();
            identifier
        } else {
            return Err(self.error(
                DiagnosticCode::SyntaxError,
                "Expected identifier as property name",
            ));
        };

        // Expect ':'
        if self.token != Kind::ColonToken {
            return Err(self.error(
                DiagnosticCode::SyntaxError,
                "Expected ':' after property name",
            ));
        }
        self.next_token();

        // Parse property type
        let type_annotation = self.parse_type()?;

        // Create and return property signature node
        let mut base = ast::NodeBase::new(Kind::PropertySignature);
        let end_pos = self.scanner.pos();
        base.set_pos(prop_pos, end_pos);

        let property_signature = Rc::new(ast::PropertySignature {
            base,
            name,
            type_annotation,
        });

        Ok(property_signature as Rc<dyn ast::Node>)
    }

    // This function was removed to resolve duplicate definition
    // The implementation from lines ~601-754 is used instead

    /// Parse a type literal (object type)
    fn parse_type_literal(&mut self) -> Result<Rc<dyn ast::Node>> {
        // Save start position
        let start_pos = self.scanner.token_pos();

        // Expect '{'
        if self.token != Kind::OpenBraceToken {
            return Err(self.error(
                DiagnosticCode::SyntaxError,
                "Expected '{' for object type literal",
            ));
        }
        self.next_token();

        // Parse members
        let mut members = Vec::new();

        while self.token != Kind::CloseBraceToken && self.token != Kind::EndOfFile {
            let member = self.parse_property_signature()?;
            members.push(member);

            // Skip optional semicolon or comma
            if self.token == Kind::SemicolonToken || self.token == Kind::CommaToken {
                self.next_token();
            }
        }

        // Expect '}'
        if self.token != Kind::CloseBraceToken {
            return Err(self.error(
                DiagnosticCode::SyntaxError,
                "Expected '}' to close object type literal",
            ));
        }
        self.next_token();

        // Create and return type literal node
        let mut base = ast::NodeBase::new(Kind::TypeLiteral);
        let end_pos = self.scanner.pos();
        base.set_pos(start_pos, end_pos);

        let type_literal = Rc::new(ast::TypeLiteral { base, members });

        Ok(type_literal as Rc<dyn ast::Node>)
    }

    /// Parse a property assignment in an object literal
    /// Corresponds to parsePropertyAssignment in internal/parser/parser.go
    /// Extended with spread operator (...) support which is handled differently in Go
    fn parse_property_assignment(&mut self) -> Result<Rc<dyn ast::Node>> {
        println!("Parsing property assignment, token: {:?}", self.token); // DEBUG

        // Check for spread operator (...)
        // Note: The Go implementation handles this through a separate parseSpreadAssignment function
        if self.token == Kind::DotDotDotToken {
            println!("Found spread operator"); // DEBUG
            self.next_token();

            // Parse the expression after the spread operator
            let expression = self.parse_expression()?;
            println!("After spread expression, token: {:?}", self.token); // DEBUG

            // Create spread assignment with position tracking
            let spread_assignment = Rc::new(ast::SpreadAssignment {
                base: self.create_node_base(Kind::SpreadAssignment),
                expression,
            });

            println!("Spread assignment complete, token: {:?}", self.token); // DEBUG
            return Ok(spread_assignment as Rc<dyn ast::Node>);
        }

        // Parse property name
        let name = if self.token == Kind::Identifier {
            let name_text = self.scanner.token_text().to_owned();
            println!("Property name: {}", name_text); // DEBUG
            let identifier = Rc::new(ast::Identifier {
                base: self.create_node_base(Kind::Identifier),
                text: name_text,
            });
            self.next_token();
            identifier
        } else if self.token == Kind::StringLiteral {
            let name_text = self.scanner.token_text().to_owned();
            let string_literal = Rc::new(ast::Identifier {
                // Using Identifier for simplicity
                base: ast::NodeBase::new(Kind::Identifier),
                text: name_text,
            });
            self.next_token();
            string_literal
        } else {
            return Err(Diagnostic::new(
                DiagnosticCode::SyntaxError,
                "Expected property name",
                &self.file_name,
                0, // TODO: Get actual position
                0, // TODO: Get actual length
                0, // TODO: Get actual line
                0, // TODO: Get actual column
            ));
        };

        // Expect ':'
        println!("After property name, token: {:?}", self.token); // DEBUG
        if self.token != Kind::ColonToken {
            return Err(Diagnostic::new(
                DiagnosticCode::SyntaxError,
                "Expected ':' in property assignment",
                &self.file_name,
                0, // TODO: Get actual position
                0, // TODO: Get actual length
                0, // TODO: Get actual line
                0, // TODO: Get actual column
            ));
        }
        self.next_token();
        println!("After colon, token: {:?}", self.token); // DEBUG

        // Parse property value
        let initializer = self.parse_expression()?;
        println!("After expression parse, token: {:?}", self.token); // DEBUG

        // Create property assignment with position tracking
        let property_assignment = Rc::new(ast::PropertyAssignment {
            base: self.create_node_base(Kind::PropertyAssignment),
            name,
            initializer,
        });

        println!("Property assignment complete, token: {:?}", self.token); // DEBUG
        Ok(property_assignment as Rc<dyn ast::Node>)
    }
}

/// Parse a source file with the given name and text
/// This function corresponds directly to ParseSourceFile in Go,
/// which creates a parser and delegates to parseSourceFileWorker
pub fn parse_source_file(file_name: &str, source_text: &str) -> Result<Rc<ast::SourceFile>> {
    let mut parser = Parser::new(file_name, source_text);
    parser.parse_source_file()
}

/// Create a Parser and parse a TypeScript file with specific language settings
/// Corresponds to ParseFile in Go
pub fn parse_file(
    file_name: &str,
    source_text: &str,
    language_version: u8,
    language_variant: u8,
) -> Result<Rc<ast::SourceFile>> {
    let mut parser = Parser::new(file_name, source_text);

    // Update settings to match requested configuration
    parser.language_version = language_version;
    parser.language_variant = language_variant;
    parser.scanner.set_language_version(language_version);
    parser.scanner.set_language_variant(language_variant);

    parser.parse_source_file()
}

#[cfg(test)]
mod tests;
