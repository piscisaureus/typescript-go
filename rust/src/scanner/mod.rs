// Corresponds to internal/scanner/scanner.go in the Go implementation

use crate::ast::Kind;
use crate::error::Diagnostic;

/// A token from the scanner
/// In Go, this is represented by Kind in the scanner context
pub struct Token {
    pub kind: Kind,
    pub pos: usize,
    pub end: usize,
    pub text: String,
}

/// Scanner reads characters and produces tokens
/// In Go, this is the Scanner struct
pub struct Scanner {
    text: String,
    pos: usize,
    token_pos: usize,
    token: Kind,
    token_value: String,
    token_flags: u32,

    // Additional fields from Go scanner
    scan_error: Option<Diagnostic>,
    line: usize,
    line_start: usize,
    skip_trivia: bool,
    is_jsx_context: bool,
}

impl Scanner {
    /// Creates a new scanner
    /// Corresponds to NewScanner in the Go code
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_owned(),
            pos: 0,
            token_pos: 0,
            token: Kind::Unknown,
            token_value: String::new(),
            token_flags: 0,
            scan_error: None,
            line: 0,
            line_start: 0,
            skip_trivia: true,
            is_jsx_context: false,
        }
    }

    /// Get the current character
    /// Corresponds to char() in the Go code
    fn char(&self) -> char {
        if self.pos >= self.text.len() {
            '\0'
        } else {
            self.text.chars().nth(self.pos).unwrap_or('\0')
        }
    }

    /// Advances the scanner and returns the next character
    /// Corresponds to nextChar() in the Go code
    fn next_char(&mut self) -> char {
        let c = self.char();
        if c != '\0' {
            self.pos += 1;
        }
        c
    }

    /// Scan a token
    /// Corresponds to scan() in the Go code
    pub fn scan(&mut self) -> Kind {
        self.token_pos = self.pos;
        self.reset_token_value();

        if self.pos >= self.text.len() {
            self.token = Kind::EndOfFile;
            return self.token;
        }

        // Process the current character
        let c = self.next_char();
        match c {
            // Whitespace and newlines
            '\t' | '\u{000B}' | '\u{000C}' | ' ' => {
                while matches!(self.char(), '\t' | '\u{000B}' | '\u{000C}' | ' ') {
                    self.next_char();
                }
                if self.skip_trivia {
                    return self.scan();
                }
                self.token = Kind::WhitespaceTrivia;
            }
            '\n' | '\r' => {
                if c == '\r' && self.char() == '\n' {
                    self.next_char();
                }
                self.line += 1;
                self.line_start = self.pos;
                if self.skip_trivia {
                    return self.scan();
                }
                self.token = Kind::NewLineTrivia;
            }

            // Comments
            '/' => {
                match self.char() {
                    '/' => {
                        // Single-line comment
                        self.next_char();
                        while !matches!(self.char(), '\0' | '\n' | '\r') {
                            self.next_char();
                        }
                        if self.skip_trivia {
                            return self.scan();
                        }
                        self.token = Kind::SingleLineCommentTrivia;
                    }
                    '*' => {
                        // Multi-line comment
                        self.next_char();
                        let mut closed = false;
                        while !closed && self.pos < self.text.len() {
                            let ch = self.next_char();
                            if ch == '*' && self.char() == '/' {
                                self.next_char();
                                closed = true;
                            }
                            if ch == '\n' {
                                self.line += 1;
                                self.line_start = self.pos;
                            }
                        }
                        if self.skip_trivia {
                            return self.scan();
                        }
                        self.token = Kind::MultiLineCommentTrivia;
                    }
                    '=' => {
                        self.next_char();
                        self.token = Kind::SlashToken; // Should be SlashEqualsToken in a full implementation
                    }
                    _ => {
                        self.token = Kind::SlashToken;
                    }
                }
            }

            // Punctuation
            '{' => self.token = Kind::OpenBraceToken,
            '}' => self.token = Kind::CloseBraceToken,
            '(' => self.token = Kind::OpenParenToken,
            ')' => self.token = Kind::CloseParenToken,
            '[' => self.token = Kind::OpenBracketToken,
            ']' => self.token = Kind::CloseBracketToken,
            '.' => {
                // Check for ellipsis/spread operator ('...')
                // Corresponds to scan() in internal/scanner/scanner.go
                // but our implementation directly handles '...' while Go uses token sequences
                if self.char() == '.' {
                    self.next_char();
                    if self.char() == '.' {
                        self.next_char();
                        self.token = Kind::DotDotDotToken;
                    } else {
                        self.token = Kind::DotToken;
                    }
                } else {
                    self.token = Kind::DotToken;
                }
            }
            ';' => self.token = Kind::SemicolonToken,
            ',' => self.token = Kind::CommaToken,
            '<' => self.token = Kind::LessThanToken,
            '>' => self.token = Kind::GreaterThanToken,
            '+' => self.token = Kind::PlusToken,
            '-' => self.token = Kind::MinusToken,
            '*' => self.token = Kind::AsteriskToken,
            '=' => self.token = Kind::EqualsToken,
            ':' => self.token = Kind::ColonToken,

            // String literals
            '"' | '\'' => {
                let quote = c;
                let mut s = String::new();
                let mut terminated = false;

                while !terminated && self.pos < self.text.len() {
                    let ch = self.char();
                    if ch == '\0' || ch == '\n' || ch == '\r' {
                        break;
                    }
                    self.next_char();
                    if ch == quote {
                        terminated = true;
                    } else {
                        s.push(ch);
                    }
                }

                self.token_value = s;
                self.token = Kind::StringLiteral;
            }

            // Numbers
            '0'..='9' => {
                let mut s = c.to_string();

                while self.pos < self.text.len() {
                    let ch = self.char();
                    if !ch.is_ascii_digit() && ch != '.' {
                        break;
                    }
                    self.next_char();
                    s.push(ch);
                }

                self.token_value = s;
                self.token = Kind::NumericLiteral;
            }

            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut s = c.to_string();

                while self.pos < self.text.len() {
                    let ch = self.char();
                    if !ch.is_ascii_alphanumeric() && ch != '_' {
                        break;
                    }
                    self.next_char();
                    s.push(ch);
                }

                self.token_value = s.clone();

                // Check for keywords
                self.token = match s.as_str() {
                    "function" => Kind::FunctionKeyword,
                    "return" => Kind::ReturnKeyword,
                    "var" => Kind::VarKeyword,
                    "let" => Kind::LetKeyword,
                    "const" => Kind::ConstKeyword,
                    "if" => Kind::IfKeyword,
                    "for" => Kind::ForKeyword,
                    "while" => Kind::WhileKeyword,
                    "string" => Kind::StringKeyword,
                    "number" => Kind::NumberKeyword,
                    _ => Kind::Identifier,
                };
            }

            _ => {
                // Unknown character, treat as unknown token
                self.token = Kind::Unknown;
            }
        }

        self.token
    }

    /// Get the text of the current token
    /// Corresponds to getTokenText in Go
    pub fn token_text(&self) -> &str {
        // For tokens that have explicit values (like literals), return the stored value
        if !self.token_value.is_empty() {
            &self.token_value
        } else if self.token_pos < self.pos {
            // Otherwise, return the slice of text that corresponds to this token
            &self.text[self.token_pos..self.pos]
        } else {
            ""
        }
    }

    /// Reset the token value
    fn reset_token_value(&mut self) {
        self.token_value = String::new();
    }

    /// Skip trivia (whitespace, comments) and return the position
    /// Corresponds to skipTrivia in the Go code
    pub fn skip_trivia(&mut self, start_pos: usize) -> usize {
        let mut pos = start_pos;

        while pos < self.text.len() {
            let c = self.text.chars().nth(pos).unwrap_or('\0');

            match c {
                '\t' | '\u{000B}' | '\u{000C}' | ' ' => {
                    pos += 1;
                }
                '\n' | '\r' => {
                    pos += 1;
                    if c == '\r'
                        && pos < self.text.len()
                        && self.text.chars().nth(pos).unwrap_or('\0') == '\n'
                    {
                        pos += 1;
                    }
                }
                '/' => {
                    if pos + 1 < self.text.len() {
                        let next = self.text.chars().nth(pos + 1).unwrap_or('\0');
                        if next == '/' {
                            // Single-line comment
                            pos += 2;
                            while pos < self.text.len() {
                                let ch = self.text.chars().nth(pos).unwrap_or('\0');
                                if ch == '\n' || ch == '\r' {
                                    break;
                                }
                                pos += 1;
                            }
                        } else if next == '*' {
                            // Multi-line comment
                            pos += 2;
                            let mut closed = false;
                            while !closed && pos < self.text.len() {
                                let ch = self.text.chars().nth(pos).unwrap_or('\0');
                                pos += 1;
                                if ch == '*'
                                    && pos < self.text.len()
                                    && self.text.chars().nth(pos).unwrap_or('\0') == '/'
                                {
                                    pos += 1;
                                    closed = true;
                                }
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
        }

        pos
    }
}
