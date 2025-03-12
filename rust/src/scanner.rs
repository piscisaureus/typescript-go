use crate::error::{Result, TsError};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Function,
    Return,
    
    // Identifiers and literals
    Identifier,
    StringLiteral,
    NumberLiteral,
    
    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Colon,
    Semicolon,
    Comma,
    Plus,
    
    // End of file
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }
    
    pub fn scan_tokens(&mut self) -> Result<Vec<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        
        self.tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: String::from(""),
            line: self.line,
            column: self.column,
        });
        
        Ok(self.tokens.clone())
    }
    
    fn scan_token(&mut self) -> Result<()> {
        let c = self.advance();
        
        match c {
            '(' => self.add_token(TokenKind::LeftParen),
            ')' => self.add_token(TokenKind::RightParen),
            '{' => self.add_token(TokenKind::LeftBrace),
            '}' => self.add_token(TokenKind::RightBrace),
            ':' => self.add_token(TokenKind::Colon),
            ';' => self.add_token(TokenKind::Semicolon),
            ',' => self.add_token(TokenKind::Comma),
            '+' => self.add_token(TokenKind::Plus),
            
            '"' => self.string()?,
            
            ' ' | '\r' | '\t' => {
                // Ignore whitespace
            },
            '\n' => {
                self.line += 1;
                self.column = 1;
            },
            
            c if self.is_digit(c) => self.number()?,
            c if self.is_alpha(c) => self.identifier()?,
            
            _ => return Err(TsError::SyntaxError(format!("Unexpected character: {}", c))),
        }
        
        Ok(())
    }
    
    fn identifier(&mut self) -> Result<()> {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }
        
        let text = self.source[self.start..self.current].iter().collect::<String>();
        
        let kind = match text.as_str() {
            "function" => TokenKind::Function,
            "return" => TokenKind::Return,
            _ => TokenKind::Identifier,
        };
        
        self.add_token(kind);
        Ok(())
    }
    
    fn string(&mut self) -> Result<()> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            self.advance();
        }
        
        if self.is_at_end() {
            return Err(TsError::SyntaxError("Unterminated string.".to_string()));
        }
        
        // The closing "
        self.advance();
        
        // Get the string contents without the quotes
        let text_with_quotes = self.source[self.start..self.current].iter().collect::<String>();
        let value = &text_with_quotes[1..text_with_quotes.len() - 1];
        
        self.add_token_with_lexeme(TokenKind::StringLiteral, value.to_string());
        Ok(())
    }
    
    fn number(&mut self) -> Result<()> {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        
        // Look for a decimal point
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();
            
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        
        let value = self.source[self.start..self.current].iter().collect::<String>();
        self.add_token_with_lexeme(TokenKind::NumberLiteral, value);
        Ok(())
    }
    
    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        self.column += 1;
        c
    }
    
    fn add_token(&mut self, kind: TokenKind) {
        let lexeme = self.source[self.start..self.current].iter().collect::<String>();
        self.add_token_with_lexeme(kind, lexeme);
    }
    
    fn add_token_with_lexeme(&mut self, kind: TokenKind, lexeme: String) {
        self.tokens.push(Token {
            kind,
            lexeme,
            line: self.line,
            column: self.column - (self.current - self.start),
        });
    }
    
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }
    
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }
    
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    
    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }
    
    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }
    
    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }
}