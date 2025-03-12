// Corresponds to internal/scanner/scanner.go in the Go implementation

use crate::ast::Kind;
use crate::error::Diagnostic;
use std::collections::HashMap;
use std::sync::OnceLock;

/// A token from the scanner
/// In Go, this is represented by Kind in the scanner context
pub struct Token {
    pub kind: Kind,
    pub pos: usize,
    pub end: usize,
    pub text: String,
}

/// Map of identifier text to keyword token kinds
/// Corresponds to textToKeyword in Go
static TEXT_TO_KEYWORD: OnceLock<HashMap<&'static str, Kind>> = OnceLock::new();

fn get_text_to_keyword() -> &'static HashMap<&'static str, Kind> {
    TEXT_TO_KEYWORD.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert("function", Kind::FunctionKeyword);
        map.insert("return", Kind::ReturnKeyword);
        map.insert("var", Kind::VarKeyword);
        map.insert("let", Kind::LetKeyword);
        map.insert("const", Kind::ConstKeyword);
        map.insert("if", Kind::IfKeyword);
        map.insert("for", Kind::ForKeyword);
        map.insert("while", Kind::WhileKeyword);
        map.insert("string", Kind::StringKeyword);
        map.insert("number", Kind::NumberKeyword);
        map.insert("true", Kind::TrueKeyword);
        map.insert("false", Kind::FalseKeyword);
        map.insert("interface", Kind::InterfaceKeyword);
        // Add more keywords as needed to match Go implementation
        map
    })
}

/// Scanner reads characters and produces tokens
/// Corresponds to the Scanner struct in internal/scanner/scanner.go
pub struct Scanner {
    // Main scanner state
    text: String,
    pos: usize,
    token_pos: usize,
    token: Kind,
    token_value: String,
    token_flags: u32,

    // Error tracking
    _scan_error: Option<Diagnostic>,

    // Tracking position in source
    line: usize,
    line_start: usize,

    // Configuration
    skip_trivia: bool,
    language_version: u8, // Corresponds to ScriptTarget in Go
    language_variant: u8, // Corresponds to LanguageVariant in Go
    _is_jsx_context: bool,
}

impl Scanner {
    /// Creates a new scanner with default settings
    /// Corresponds to defaultScanner in the Go code
    fn default_scanner() -> Self {
        Self {
            text: String::new(),
            pos: 0,
            token_pos: 0,
            token: Kind::Unknown,
            token_value: String::new(),
            token_flags: 0,
            _scan_error: None,
            line: 0,
            line_start: 0,
            skip_trivia: true,
            language_version: 99, // Latest version, similar to ScriptTargetLatest
            language_variant: 0,  // Standard variant
            _is_jsx_context: false,
        }
    }

    /// Creates a new scanner
    /// Corresponds to NewScanner in the Go code
    pub fn new(text: &str) -> Self {
        let mut scanner = Self::default_scanner();
        scanner.set_text(text);
        // In the Go implementation, we don't scan the first token in NewScanner
        scanner
    }

    /// A more direct clone of Go's NewScanner implementation which returns a pointer to a new Scanner
    pub fn new_scanner() -> Box<Self> {
        Box::new(Self::default_scanner())
    }

    /// Reset the scanner to its initial state
    /// Corresponds to Reset in the Go code
    pub fn reset(&mut self) {
        *self = Self::default_scanner();
    }

    /// Set the text to scan
    /// Corresponds to SetText in the Go code
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_owned();
        self.pos = 0;
        self.token_pos = 0;
        self.token = Kind::Unknown;
        self.token_value = String::new();
        self.token_flags = 0;
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

    /// Get character at offset from current position
    /// Corresponds to charAt() in the Go code
    fn char_at(&self, offset: usize) -> char {
        if self.pos + offset >= self.text.len() {
            '\0'
        } else {
            self.text.chars().nth(self.pos + offset).unwrap_or('\0')
        }
    }

    /// Get current character and its UTF-8 size
    /// Corresponds to charAndSize() in the Go code
    fn _char_and_size(&self) -> (char, usize) {
        if self.pos >= self.text.len() {
            return ('\0', 0);
        }
        let c = self.text[self.pos..].chars().next().unwrap_or('\0');
        let size = c.len_utf8();
        (c, size)
    }

    /// Advances the scanner and returns the next character
    /// Corresponds to nextChar() in the Go code (but not used in Go implementation)
    fn _next_char(&mut self) -> char {
        let c = self.char();
        if c != '\0' {
            self.pos += 1;
        }
        c
    }

    /// Scan a token
    /// Corresponds to Scan() in the Go code
    pub fn scan(&mut self) -> Kind {
        self.token_pos = self.pos;
        self.reset_token_value();
        self.token_flags = 0; // Reset token flags for each scan

        // Main scanning loop to handle trivia (whitespace, comments, etc.)
        loop {
            // If at end of input, return EOF
            if self.pos >= self.text.len() {
                self.token = Kind::EndOfFile;
                return self.token;
            }

            // Start token at current position
            self.token_pos = self.pos;

            // Get current character and process it
            let c = self.char();

            match c {
                // Whitespace
                '\t' | '\u{000B}' | '\u{000C}' | ' ' => {
                    self.pos += 1;
                    while matches!(self.char(), '\t' | '\u{000B}' | '\u{000C}' | ' ') {
                        self.pos += 1;
                    }
                    if self.skip_trivia {
                        continue; // Skip and scan again
                    }
                    self.token = Kind::WhitespaceTrivia;
                }

                // Line breaks
                '\n' | '\r' => {
                    self.token_flags |= 1; // Set preceding line break flag
                    self.pos += 1;
                    if c == '\r' && self.char() == '\n' {
                        self.pos += 1;
                    }
                    self.line += 1;
                    self.line_start = self.pos;
                    if self.skip_trivia {
                        continue; // Skip and scan again
                    }
                    self.token = Kind::NewLineTrivia;
                }

                // Comments
                '/' => {
                    match self.char_at(1) {
                        '/' => {
                            // Single-line comment
                            self.pos += 2;
                            while self.pos < self.text.len() {
                                let ch = self.char();
                                if ch == '\r' || ch == '\n' {
                                    break;
                                }
                                self.pos += 1;
                            }
                            if self.skip_trivia {
                                continue; // Skip and scan again
                            }
                            self.token = Kind::SingleLineCommentTrivia;
                        }
                        '*' => {
                            // Multi-line comment
                            self.pos += 2;
                            let mut _closed = false;
                            while !_closed && self.pos < self.text.len() {
                                let ch = self.char();
                                if ch == '*' && self.char_at(1) == '/' {
                                    self.pos += 2;
                                    _closed = true;
                                    break;
                                }
                                if ch == '\r' || ch == '\n' {
                                    self.token_flags |= 1; // Set preceding line break flag
                                    if ch == '\r' && self.char_at(1) == '\n' {
                                        self.pos += 2;
                                    } else {
                                        self.pos += 1;
                                    }
                                    self.line += 1;
                                    self.line_start = self.pos;
                                } else {
                                    self.pos += 1;
                                }
                            }
                            if self.skip_trivia {
                                continue; // Skip and scan again
                            }
                            self.token = Kind::MultiLineCommentTrivia;
                        }
                        '=' => {
                            self.pos += 2;
                            self.token = Kind::SlashEqualsToken;
                        }
                        _ => {
                            self.pos += 1;
                            self.token = Kind::SlashToken;
                        }
                    }
                }

                // Punctuation
                '{' => {
                    self.pos += 1;
                    self.token = Kind::OpenBraceToken;
                }
                '}' => {
                    self.pos += 1;
                    self.token = Kind::CloseBraceToken;
                }
                '(' => {
                    self.pos += 1;
                    self.token = Kind::OpenParenToken;
                }
                ')' => {
                    self.pos += 1;
                    self.token = Kind::CloseParenToken;
                }
                '[' => {
                    self.pos += 1;
                    self.token = Kind::OpenBracketToken;
                }
                ']' => {
                    self.pos += 1;
                    self.token = Kind::CloseBracketToken;
                }
                '.' => {
                    if self.char_at(1) == '.' && self.char_at(2) == '.' {
                        self.pos += 3;
                        self.token = Kind::DotDotDotToken;
                    } else {
                        self.pos += 1;
                        self.token = Kind::DotToken;
                    }
                }
                ';' => {
                    self.pos += 1;
                    self.token = Kind::SemicolonToken;
                }
                ',' => {
                    self.pos += 1;
                    self.token = Kind::CommaToken;
                }
                '<' => {
                    self.pos += 1;
                    self.token = Kind::LessThanToken;
                }
                '>' => {
                    self.pos += 1;
                    self.token = Kind::GreaterThanToken;
                }
                '+' => {
                    if self.char_at(1) == '=' {
                        self.pos += 2;
                        self.token = Kind::PlusEqualsToken;
                    } else if self.char_at(1) == '+' {
                        self.pos += 2;
                        self.token = Kind::PlusPlusToken;
                    } else {
                        self.pos += 1;
                        self.token = Kind::PlusToken;
                    }
                }
                '-' => {
                    if self.char_at(1) == '=' {
                        self.pos += 2;
                        self.token = Kind::MinusEqualsToken;
                    } else if self.char_at(1) == '-' {
                        self.pos += 2;
                        self.token = Kind::MinusMinusToken;
                    } else {
                        self.pos += 1;
                        self.token = Kind::MinusToken;
                    }
                }
                '*' => {
                    if self.char_at(1) == '=' {
                        self.pos += 2;
                        self.token = Kind::AsteriskEqualsToken;
                    } else if self.char_at(1) == '*' {
                        if self.char_at(2) == '=' {
                            self.pos += 3;
                            self.token = Kind::AsteriskAsteriskEqualsToken;
                        } else {
                            self.pos += 2;
                            self.token = Kind::AsteriskAsteriskToken;
                        }
                    } else {
                        self.pos += 1;
                        self.token = Kind::AsteriskToken;
                    }
                }
                '=' => {
                    if self.char_at(1) == '=' {
                        if self.char_at(2) == '=' {
                            self.pos += 3;
                            self.token = Kind::EqualsEqualsEqualsToken;
                        } else {
                            self.pos += 2;
                            self.token = Kind::EqualsEqualsToken;
                        }
                    } else if self.char_at(1) == '>' {
                        self.pos += 2;
                        self.token = Kind::EqualsGreaterThanToken;
                    } else {
                        self.pos += 1;
                        self.token = Kind::EqualsToken;
                    }
                }
                ':' => {
                    self.pos += 1;
                    self.token = Kind::ColonToken;
                }

                // String literals
                '"' | '\'' => {
                    let quote = c;
                    self.pos += 1;
                    let start = self.pos;
                    let mut terminated = false;

                    while !terminated && self.pos < self.text.len() {
                        let ch = self.char();
                        if ch == '\0' || ch == '\n' || ch == '\r' {
                            break;
                        }
                        self.pos += 1;
                        if ch == quote {
                            terminated = true;
                        }
                    }

                    if terminated {
                        // Don't include the quotes in the value
                        self.token_value = self.text[start..self.pos - 1].to_string();
                    } else {
                        self.token_value = self.text[start..self.pos].to_string();
                        // TODO: Handle unterminated string error like in Go scanner
                    }

                    self.token = Kind::StringLiteral;
                }

                // Numbers
                '0'..='9' => {
                    let start = self.pos;
                    self.pos += 1;

                    // Scan digits and decimal point
                    while self.pos < self.text.len() {
                        let ch = self.char();
                        if !ch.is_ascii_digit() && ch != '.' {
                            break;
                        }
                        self.pos += 1;
                    }

                    self.token_value = self.text[start..self.pos].to_string();
                    self.token = Kind::NumericLiteral;
                }

                // Identifiers and keywords
                'a'..='z' | 'A'..='Z' | '_' => {
                    let start = self.pos;
                    self.pos += 1;

                    // Fast path for ASCII identifiers
                    while self.pos < self.text.len() {
                        let ch = self.char();
                        if !ch.is_ascii_alphanumeric() && ch != '_' {
                            break;
                        }
                        self.pos += 1;
                    }

                    self.token_value = self.text[start..self.pos].to_string();

                    // Check for keywords using the map lookup like in Go
                    let text = self.token_value.as_str();
                    self.token = match get_text_to_keyword().get(text) {
                        Some(&kind) => kind,
                        None => Kind::Identifier,
                    };
                }

                // Handle unknown characters
                _ => {
                    self.pos += 1;
                    self.token = Kind::Unknown;
                }
            }

            return self.token;
        }
    }

    /// Get the text of the current token
    /// Corresponds to TokenText in Go
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

    /// Get the token value
    /// Corresponds to TokenValue in Go
    pub fn token_value(&self) -> &str {
        &self.token_value
    }

    /// Get the token flags
    /// Corresponds to TokenFlags in Go
    pub fn token_flags(&self) -> u32 {
        self.token_flags
    }

    /// Get token position (start position)
    /// Corresponds to TokenStart in Go
    pub fn token_pos(&self) -> usize {
        self.token_pos
    }

    /// Get token end position
    /// Corresponds to TokenEnd in Go
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Get token range
    /// Corresponds to TokenRange in Go
    pub fn token_range(&self) -> (usize, usize) {
        (self.token_pos, self.pos)
    }

    /// Check if the current token has a preceding line break
    /// Corresponds to HasPrecedingLineBreak in Go
    pub fn has_preceding_line_break(&self) -> bool {
        (self.token_flags & 1) != 0
    }

    /// Get the current line number (1-based)
    /// Corresponds to getLinePos in the Go code
    pub fn get_line_number(&self) -> usize {
        self.line + 1 // Convert to 1-based line number
    }

    /// Get the current column number (1-based)
    /// Corresponds to getColPos in the Go code
    pub fn get_column_number(&self) -> usize {
        self.token_pos - self.line_start + 1 // Convert to 1-based column
    }

    /// Set the language version for scanning
    /// Corresponds to SetScriptTarget in Go
    pub fn set_language_version(&mut self, version: u8) {
        self.language_version = version;
    }

    /// Set the language variant for scanning
    /// Corresponds to SetLanguageVariant in Go
    pub fn set_language_variant(&mut self, variant: u8) {
        self.language_variant = variant;
    }

    /// Reset the token value
    fn reset_token_value(&mut self) {
        self.token_value = String::new();
    }

    /// Skip trivia (whitespace, comments) and return the position
    /// Corresponds to SkipTrivia in the Go code
    pub fn skip_trivia(&mut self, start_pos: usize) -> usize {
        let mut pos = start_pos;

        // Skip trivia logic similar to the Go implementation
        while pos < self.text.len() {
            let c = self.text.chars().nth(pos).unwrap_or('\0');

            match c {
                // Whitespace
                '\t' | '\u{000B}' | '\u{000C}' | ' ' => {
                    pos += 1;
                }

                // Line breaks
                '\n' | '\r' => {
                    pos += 1;
                    if c == '\r'
                        && pos < self.text.len()
                        && self.text.chars().nth(pos).unwrap_or('\0') == '\n'
                    {
                        pos += 1;
                    }
                }

                // Comments
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

                // Any other character ends trivia
                _ => {
                    break;
                }
            }
        }

        pos
    }

    /// Re-scan a token
    /// Helper function similar to various ReScan methods in Go
    pub fn rescan_token(&mut self, token_type: Kind) -> Kind {
        self.pos = self.token_pos;
        self.token = token_type;
        self.scan()
    }
}
