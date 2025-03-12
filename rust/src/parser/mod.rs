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
/// Corresponds to Parser in Go
pub struct Parser {
    scanner: Scanner,
    token: Kind,

    file_name: String,
    source_text: String,

    diagnostics: Vec<Diagnostic>,

    parsing_context: Vec<ParsingContext>,
}

impl Parser {
    /// Creates a new parser
    /// Corresponds to InitializeState in Go
    pub fn new(file_name: &str, source_text: &str) -> Self {
        let mut parser = Self {
            scanner: Scanner::new(source_text),
            token: Kind::Unknown,
            file_name: file_name.to_owned(),
            source_text: source_text.to_owned(),
            diagnostics: Vec::new(),
            parsing_context: Vec::new(),
        };

        // Advance to the first token
        parser.next_token();

        parser
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
        let mut statements = Vec::new();

        // Parse statements
        while self.token != Kind::EndOfFile {
            if let Some(statement) = self.parse_statement()? {
                statements.push(statement);
            }
        }

        // Create source file node
        let file_name_only = Path::new(&self.file_name)
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| self.file_name.clone());

        let source_file = Rc::new(ast::SourceFile {
            base: ast::NodeBase {
                kind: Kind::SourceFile,
                flags: NodeFlags::None,
                loc: TextRange::new(0, self.source_text.len()),
            },
            text: self.source_text.clone(),
            file_name: file_name_only,
            language_version: 0, // ES2015 (ES6)
            statements,
            line_map: vec![0], // TODO: Build a proper line map
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
            Kind::ReturnKeyword => self
                .parse_return_statement()
                .map(|r| Some(r as Rc<dyn ast::Node>)),
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

            if self.token == Kind::StringKeyword || self.token == Kind::NumberKeyword {
                let type_text = self.scanner.token_text().to_owned();
                let type_identifier = Rc::new(ast::Identifier {
                    base: ast::NodeBase::new(Kind::Identifier),
                    text: type_text,
                });

                let type_reference = Rc::new(ast::TypeReference {
                    base: ast::NodeBase::new(Kind::TypeReference),
                    type_name: type_identifier,
                });

                self.next_token();
                println!("Return type parsed, token now: {:?}", self.token); // DEBUG: not in Go
                Some(type_reference as Rc<dyn ast::Node>)
            } else {
                None
            }
        } else {
            None
        };

        // Parse function body
        let body = if self.token == Kind::OpenBraceToken {
            Some(self.parse_block()?)
        } else {
            None
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
            if self.token == Kind::StringKeyword || self.token == Kind::NumberKeyword {
                let type_text = self.scanner.token_text().to_owned();
                let type_identifier = Rc::new(ast::Identifier {
                    base: ast::NodeBase::new(Kind::Identifier),
                    text: type_text,
                });

                let type_reference = Rc::new(ast::TypeReference {
                    base: ast::NodeBase::new(Kind::TypeReference),
                    type_name: type_identifier,
                });

                self.next_token();
                println!("  Type parsed, token now: {:?}", self.token); // DEBUG: not in Go
                Some(type_reference as Rc<dyn ast::Node>)
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
        let left = self.parse_primary_expression()?;

        if self.token == Kind::PlusToken {
            let operator = self.token;
            self.next_token();

            let right = self.parse_primary_expression()?;

            let binary_expr = Rc::new(ast::BinaryExpression {
                base: ast::NodeBase::new(Kind::BinaryExpression),
                left,
                operator_token: operator,
                right,
            });

            Ok(binary_expr as Rc<dyn ast::Node>)
        } else {
            Ok(left)
        }
    }

    /// Parse a primary expression
    /// Corresponds to parsePrimaryExpression in Go
    fn parse_primary_expression(&mut self) -> Result<Rc<dyn ast::Node>> {
        match self.token {
            Kind::Identifier => {
                let name_text = self.scanner.token_text().to_owned();
                let identifier = Rc::new(ast::Identifier {
                    base: ast::NodeBase::new(Kind::Identifier),
                    text: name_text,
                });
                self.next_token();

                // Check for function call
                if self.token == Kind::OpenParenToken {
                    self.parse_call_expression(identifier as Rc<dyn ast::Node>)
                } else {
                    Ok(identifier as Rc<dyn ast::Node>)
                }
            }
            Kind::StringLiteral => {
                let text = self.scanner.token_text().to_owned();
                let string_literal = Rc::new(ast::StringLiteral {
                    base: ast::NodeBase::new(Kind::StringLiteral),
                    text,
                });
                self.next_token();
                Ok(string_literal as Rc<dyn ast::Node>)
            }
            Kind::NumericLiteral => {
                let text = self.scanner.token_text().to_owned();
                let value = text.parse::<f64>().unwrap_or(0.0);
                let number_literal = Rc::new(ast::NumericLiteral {
                    base: ast::NodeBase::new(Kind::NumericLiteral),
                    text,
                    value,
                });
                self.next_token();
                Ok(number_literal as Rc<dyn ast::Node>)
            }
            _ => {
                Err(Diagnostic::new(
                    DiagnosticCode::SyntaxError,
                    &format!("Unexpected token: {:?}", self.token),
                    &self.file_name,
                    0, // TODO: Get actual position
                    0, // TODO: Get actual length
                    0, // TODO: Get actual line
                    0, // TODO: Get actual column
                ))
            }
        }
    }

    /// Parse a call expression
    /// Corresponds to parseCallExpression in Go
    fn parse_call_expression(
        &mut self,
        expression: Rc<dyn ast::Node>,
    ) -> Result<Rc<dyn ast::Node>> {
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

        // Create call expression
        let call_expr = Rc::new(ast::CallExpression {
            base: ast::NodeBase::new(Kind::CallExpression),
            expression,
            arguments,
        });

        Ok(call_expr as Rc<dyn ast::Node>)
    }
}

/// Parse a source file with the given name and text
/// Corresponds to ParseSourceFile in Go
pub fn parse_source_file(file_name: &str, source_text: &str) -> Result<Rc<ast::SourceFile>> {
    let mut parser = Parser::new(file_name, source_text);
    parser.parse_source_file()
}
