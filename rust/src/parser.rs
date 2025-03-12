use crate::ast::{BinaryOperator, Expression, FunctionDeclaration, Parameter, Program, Statement, TypeAnnotation};
use crate::error::{Result, TsError};
use crate::scanner::{Scanner, Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(mut scanner: Scanner) -> Result<Self> {
        let tokens = scanner.scan_tokens()?;
        Ok(Parser {
            tokens,
            current: 0,
        })
    }
    
    pub fn parse(&mut self) -> Result<Program> {
        let mut program = Program {
            functions: Vec::new(),
            statements: Vec::new(),
        };
        
        while !self.is_at_end() {
            if self.match_token(&[TokenKind::Function]) {
                let function = self.function_declaration()?;
                program.functions.push(function);
            } else {
                let statement = self.statement()?;
                program.statements.push(statement);
            }
        }
        
        Ok(program)
    }
    
    fn function_declaration(&mut self) -> Result<FunctionDeclaration> {
        // Get the function name
        let name_token = self.consume(TokenKind::Identifier, "Expect function name.")?;
        let function_name = name_token.lexeme.clone();
        
        self.consume(TokenKind::LeftParen, "Expect '(' after function name.")?;
        
        // Parse parameters
        let mut parameters = Vec::new();
        if !self.check(&TokenKind::RightParen) {
            loop {
                let param_name_token = self.consume(TokenKind::Identifier, "Expect parameter name.")?;
                let param_name = param_name_token.lexeme.clone();
                
                self.consume(TokenKind::Colon, "Expect ':' after parameter name.")?;
                
                let type_name_token = self.consume(TokenKind::Identifier, "Expect type annotation.")?;
                let type_name = type_name_token.lexeme.clone();
                
                let type_annotation = match type_name.as_str() {
                    "string" => TypeAnnotation::String,
                    "number" => TypeAnnotation::Number,
                    _ => return Err(TsError::SyntaxError(format!("Unsupported type: {}", type_name))),
                };
                
                parameters.push(Parameter {
                    name: param_name,
                    type_annotation,
                });
                
                if !self.match_token(&[TokenKind::Comma]) {
                    break;
                }
            }
        }
        
        self.consume(TokenKind::RightParen, "Expect ')' after parameters.")?;
        
        // Parse return type
        self.consume(TokenKind::Colon, "Expect ':' after parameter list.")?;
        
        let return_type_token = self.consume(TokenKind::Identifier, "Expect return type annotation.")?;
        let return_type_name = return_type_token.lexeme.clone();
        
        let return_type = match return_type_name.as_str() {
            "string" => TypeAnnotation::String,
            "number" => TypeAnnotation::Number,
            _ => return Err(TsError::SyntaxError(format!("Unsupported return type: {}", return_type_name))),
        };
        
        // Parse function body
        self.consume(TokenKind::LeftBrace, "Expect '{' before function body.")?;
        
        let mut body = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            body.push(self.statement()?);
        }
        
        self.consume(TokenKind::RightBrace, "Expect '}' after function body.")?;
        
        Ok(FunctionDeclaration {
            name: function_name,
            parameters,
            return_type,
            body,
        })
    }
    
    fn statement(&mut self) -> Result<Statement> {
        if self.match_token(&[TokenKind::Return]) {
            self.return_statement()
        } else {
            self.expression_statement()
        }
    }
    
    fn return_statement(&mut self) -> Result<Statement> {
        let value = self.expression()?;
        self.consume(TokenKind::Semicolon, "Expect ';' after return value.")?;
        Ok(Statement::Return(value))
    }
    
    fn expression_statement(&mut self) -> Result<Statement> {
        let expr = self.expression()?;
        self.consume(TokenKind::Semicolon, "Expect ';' after expression.")?;
        Ok(Statement::Expression(expr))
    }
    
    fn expression(&mut self) -> Result<Expression> {
        self.binary()
    }
    
    fn binary(&mut self) -> Result<Expression> {
        let mut expr = self.primary()?;
        
        while self.match_token(&[TokenKind::Plus]) {
            let operator = BinaryOperator::Plus;
            let right = self.primary()?;
            expr = Expression::BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn primary(&mut self) -> Result<Expression> {
        if self.match_token(&[TokenKind::StringLiteral]) {
            let previous = self.previous();
            Ok(Expression::StringLiteral(previous.lexeme.clone()))
        } else if self.match_token(&[TokenKind::NumberLiteral]) {
            let previous = self.previous();
            let value = previous.lexeme.parse::<f64>().map_err(|_| {
                TsError::SyntaxError(format!("Could not parse number: {}", previous.lexeme))
            })?;
            Ok(Expression::NumberLiteral(value))
        } else if self.match_token(&[TokenKind::Identifier]) {
            let function_name = self.previous().lexeme.clone();
            
            // Check if it's a function call
            if self.match_token(&[TokenKind::LeftParen]) {
                let mut arguments = Vec::new();
                
                if !self.check(&TokenKind::RightParen) {
                    loop {
                        arguments.push(self.expression()?);
                        if !self.match_token(&[TokenKind::Comma]) {
                            break;
                        }
                    }
                }
                
                self.consume(TokenKind::RightParen, "Expect ')' after arguments.")?;
                
                Ok(Expression::FunctionCall {
                    callee: function_name,
                    arguments,
                })
            } else {
                // Here we handle simple variable references
                Ok(Expression::StringLiteral(function_name))
            }
        } else {
            Err(TsError::SyntaxError("Expected expression.".to_string()))
        }
    }
    
    fn match_token(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }
        false
    }
    
    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().kind == kind
        }
    }
    
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    
    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<&Token> {
        if self.check(&kind) {
            Ok(self.advance())
        } else {
            Err(TsError::SyntaxError(message.to_string()))
        }
    }
    
    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}