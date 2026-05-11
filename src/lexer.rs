
//! Zenith Universal Meta-Compiler (UMC) Lexer
//!
//! This module implements the lexical analysis phase of the Zenith compiler.
//! It takes a raw Zenith source code string and breaks it down into a stream
//! of tokens (lexemes), each representing a meaningful unit in the language.
//!
//! The lexer is responsible for:
//! - Recognizing keywords, identifiers, operators, and literals.
//! - Handling whitespace and comments.
//! - Identifying special Zenith-specific tokens (e.g., quantum literals, nano annotations).
//! - Reporting lexical errors (e.g., illegal characters, unclosed strings).
//! - Attaching source location information (Span) to each token for precise error reporting.

use crate::source_map::{FileId, Span, BytePos, SourceFile};
use std::sync::Arc; // For shared ownership of SourceFile

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens
    LParen, RParen, LBrace, RBrace, LBracket, RBracket, Comma, Dot, Semicolon, Colon, Arrow,
    Plus, Minus, Star, Slash, Modulo, Assign, Not, BitAnd, BitOr, Caret, LessThan, GreaterThan,
    Ampersand, Pipe, Question, Exclamation, Tilde, Hash, At,

    // Two-character tokens
    Equals, NotEquals, LessThanEqual, GreaterThanEqual,
    LogicalAnd, LogicalOr, LeftShift, RightShift, DoubleColon, ThinArrow, FatArrow,

    // Literals
    Identifier,
    String,
    Integer,
    Float,
    Char,
    Boolean,
    QuantumLiteral, // e.g., |0⟩, |+⟩
    NanoAnnotation, // e.g., @atom, @molecule
    MTSLiteral,     // e.g., mts[timestamp]

    // Keywords (Alphabetical)
    KeywordAffine, KeywordAgent, KeywordAnd, KeywordAncestor, KeywordAs, KeywordAsync, KeywordAwait,
    KeywordBreak, KeywordCase, KeywordCatch, KeywordCircuit, KeywordClass, KeywordConst,
    KeywordContinue, KeywordEffect, KeywordElse, KeywordEnum, KeywordExtends, KeywordFalse,
    KeywordFn, KeywordFor, KeywordFrom, KeywordHandle, KeywordIf, KeywordImpl, KeywordImport,
    KeywordIn, KeywordInterface, KeywordIs, KeywordLearn, KeywordLet, KeywordLinear, KeywordMatch,
    KeywordModule, KeywordMove, KeywordMts, KeywordMut, KeywordNano, KeywordNew, KeywordNot,
    KeywordOr, KeywordPerform, KeywordPrivate, KeywordPublic, KeywordQuantum, KeywordRecall,
    KeywordRemember, KeywordReturn, KeywordSasa, KeywordSelf, KeywordSigma, KeywordStatic,
    KeywordStruct, KeywordSuper, KeywordSwitch, KeywordSync, KeywordThen, KeywordThrow,
    KeywordTrait, KeywordTrue, KeywordType, KeywordUnsafe, KeywordUse, KeywordVar,
    KeywordWhere, KeywordWhile, KeywordWith, KeywordWisdom, KeywordYield, KeywordZamani,
    KeywordPi, // For dependent types: Π

    // Special Zenith/Sankofa/Nimbus tokens
    SigmaSymbol, // Σ
    PiSymbol,    // Π

    // End of file
    EOF,
    // Error token
    Illegal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerError {
    pub message: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Lexer {
    file_id: FileId,
    source_file_arc: Arc<SourceFile>, // Reference to the SourceFile for line/column info
    input: Arc<String>, // Shared ownership of the input string
    position: BytePos,       // current position in input (points to current char)
    read_position: BytePos,  // current reading position in input (after current char)
    ch: Option<char>,        // current char under examination
    errors: Vec<LexerError>,
}

impl Lexer {
    pub fn new(file_id: FileId, source_file_arc: Arc<SourceFile>) -> Self { // Modified signature
        let input = Arc::clone(&source_file_arc.content);
        let mut lexer = Lexer {
            file_id,
            source_file_arc,
            input,
            position: BytePos(0),
            read_position: BytePos(0),
            ch: None,
            errors: Vec::new(),
        };
        lexer.read_char(); // Initialize ch
        lexer
    }

    pub fn get_errors(&self) -> &Vec<LexerError> {
        &self.errors
    }

    fn read_char(&mut self) {
        if self.read_position.0 >= self.input.len() as u32 {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position.0 as usize);
        }
        self.position = self.read_position;
        self.read_position.0 += 1; // Move to the next byte position for reading
    }

    fn peek_char(&self) -> Option<char> {
        if self.read_position.0 >= self.input.len() as u32 {
            None
        } else {
            self.input.chars().nth(self.read_position.0 as usize)
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.ch {
            if c.is_ascii_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn skip_comments(&mut self) {
        loop {
            // Single-line comment
            if self.ch == Some('/') && self.peek_char() == Some('/') {
                while self.ch != Some('\n') && self.ch.is_some() {
                    self.read_char();
                }
                self.read_char(); // Consume newline
            }
            // Multi-line comment
            else if self.ch == Some('/') && self.peek_char() == Some('*') {
                self.read_char(); // Consume /
                self.read_char(); // Consume *
                loop {
                    if self.ch == Some('*') && self.peek_char() == Some('/') {
                        self.read_char(); // Consume *
                        self.read_char(); // Consume /
                        break;
                    }
                    if self.ch.is_none() {
                        self.errors.push(LexerError {
                            message: "Unterminated multi-line comment".to_string(),
                            span: self.make_token_span(self.position, self.position), // Approximate error location
                        });
                        return;
                    }
                    self.read_char();
                }
            } else {
                break;
            }
            self.skip_whitespace(); // Comments can be followed by whitespace
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position.0;
        while let Some(c) = self.ch {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.read_char();
            } else {
                break;
            }
        }
        self.input.get(position as usize..self.position.0 as usize).unwrap_or("").to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position.0;
        let mut is_float = false;
        while let Some(c) = self.ch {
            if c.is_ascii_digit() {
                self.read_char();
            } else if c == '.' && self.peek_char().map_or(false, |p| p.is_ascii_digit()) {
                is_float = true;
                self.read_char(); // Consume '.'
            } else {
                break;
            }
        }
        let num_str = self.input.get(position as usize..self.position.0 as usize).unwrap_or("").to_string();
        if is_float {
            // No need to store in token type directly, TokenType::Float implies it.
        }
        num_str
    }

    fn read_string(&mut self) -> String {
        let position = self.position.0 + 1; // Skip opening quote
        self.read_char(); // Consume opening quote
        while let Some(c) = self.ch {
            if c == '"' {
                break;
            }
            if c == '\\'.to_string().chars().next().unwrap() { // Handle escape sequences conceptually
                self.read_char(); // Consume backslash
                self.read_char(); // Consume escaped char
            } else {
                self.read_char();
            }
        }
        let str_literal = self.input.get(position as usize..self.position.0 as usize).unwrap_or("").to_string();
        self.read_char(); // Consume closing quote
        str_literal
    }

    fn read_char_literal(&mut self) -> String {
        let position = self.position.0 + 1; // Skip opening quote
        self.read_char(); // Consume opening quote
        // Expect exactly one character, or an escaped sequence
        if self.ch == Some('\') { // Handle backslash escape explicitly
            self.read_char(); // Consume backslash
            self.read_char(); // Consume escaped char
        } else if self.ch.is_some() && self.ch != Some(''') {
            self.read_char(); // Consume char
        }

        let char_literal = self.input.get(position as usize..self.position.0 as usize).unwrap_or("").to_string();
        
        if self.ch == Some(''') {
            self.read_char(); // Consume closing quote
            char_literal
        } else {
            self.errors.push(LexerError {
                message: "Unterminated character literal".to_string(),
                span: self.make_token_span(self.position, self.position), // Approximate error location
            });
            "".to_string()
        }
    }

    // Helper to create a Span using the Lexer's current SourceFile
    fn make_token_span(&self, start_pos: BytePos, end_pos: BytePos) -> Span {
        let (start_line, start_column) = self.source_file_arc.get_line_info(start_pos);
        Span::new(self.file_id, start_pos, end_pos, start_line, start_column)
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.skip_comments();
        self.skip_whitespace();

        let start_pos = self.position;
        let token_type;
        let mut literal = "".to_string();

        match self.ch {
            Some('(') => token_type = TokenType::LParen,
            Some(')') => token_type = TokenType::RParen,
            Some('{') => token_type = TokenType::LBrace,
            Some('}') => token_type = TokenType::RBrace,
            Some('[') => token_type = TokenType::LBracket,
            Some(']') => token_type = TokenType::RBracket,
            Some(',') => token_type = TokenType::Comma,
            Some('.') => token_type = TokenType::Dot,
            Some(';') => token_type = TokenType::Semicolon,
            Some(':') => {
                if self.peek_char() == Some(':') {
                    token_type = TokenType::DoubleColon;
                    self.read_char();
                } else {
                    token_type = TokenType::Colon;
                }
            }
            Some('+') => token_type = TokenType::Plus,
            Some('-') => {
                if self.peek_char() == Some('>') {
                    token_type = TokenType::ThinArrow;
                    self.read_char();
                } else {
                    token_type = TokenType::Minus;
                }
            }
            Some('*') => token_type = TokenType::Star,
            Some('/') => token_type = TokenType::Slash, // Comments handled before
            Some('%') => token_type = TokenType::Modulo,
            Some('=') => {
                if self.peek_char() == Some('=') {
                    token_type = TokenType::Equals;
                    self.read_char();
                } else {
                    token_type = TokenType::Assign;
                }
            }
            Some('!') => {
                if self.peek_char() == Some('=') {
                    token_type = TokenType::NotEquals;
                    self.read_char();
                } else {
                    token_type = TokenType::Not;
                }
            }
            Some('<') => {
                if self.peek_char() == Some('=') {
                    token_type = TokenType::LessThanEqual;
                    self.read_char();
                } else if self.peek_char() == Some('<') {
                    token_type = TokenType::LeftShift;
                    self.read_char();
                } else {
                    token_type = TokenType::LessThan;
                }
            }
            Some('>') => {
                if self.peek_char() == Some('=') {
                    token_type = TokenType::GreaterThanEqual;
                    self.read_char();
                } else if self.peek_char() == Some('>') {
                    token_type = TokenType::RightShift;
                    self.read_char();
                } else {
                    token_type = TokenType::GreaterThan;
                }
            }
            Some('&') => {
                if self.peek_char() == Some('&') {
                    token_type = TokenType::LogicalAnd;
                    self.read_char();
                } else {
                    token_type = TokenType::BitAnd;
                }
            }
            Some('|') => {
                if self.peek_char() == Some('|') {
                    token_type = TokenType::LogicalOr;
                    self.read_char();
                } else if self.peek_char() == Some('⟩') { // Quantum Literal |0⟩
                    // Consume '|', then '0' or '1' or '+', then '⟩'
                    self.read_char(); // Consumes '|'
                    self.read_char(); // Consumes inner char (e.g., '0')
                    self.read_char(); // Consumes '⟩'
                    literal = self.input.get(start_pos.0 as usize..self.position.0 as usize).unwrap_or("").to_string();
                    token_type = TokenType::QuantumLiteral;
                } else {
                    token_type = TokenType::Pipe;
                }
            }
            Some('^') => token_type = TokenType::Caret,
            Some('~') => token_type = TokenType::Tilde,
            Some('#') => token_type = TokenType::Hash,
            Some('@') => { // For @atom(...), @molecule, etc.
                self.read_char(); // Consume @
                if let Some(c) = self.ch {
                    if c.is_ascii_alphabetic() { // Start of an identifier for annotation
                        let id_start_pos = self.position;
                        let id = self.read_identifier();
                        literal = format!("@{}", id);
                        token_type = TokenType::NanoAnnotation;
                    } else {
                        // If it's just '@' not followed by an identifier, treat as TokenType::At
                        token_type = TokenType::At;
                        literal = "@".to_string();
                    }
                } else {
                    token_type = TokenType::At;
                    literal = "@".to_string();
                }
            }
            Some('"') => {
                literal = self.read_string();
                token_type = TokenType::String;
            }
            Some(''') => {
                literal = self.read_char_literal();
                token_type = TokenType::Char;
            }
            Some('Π') => token_type = TokenType::PiSymbol,
            Some('Σ') => token_type = TokenType::SigmaSymbol,
            Some(c) if c.is_ascii_alphanumeric() || c == '_' => {
                literal = self.read_identifier();
                token_type = match literal.as_str() {
                    "let" => TokenType::KeywordLet,
                    "fn" => TokenType::KeywordFn,
                    "return" => TokenType::KeywordReturn,
                    "if" => TokenType::KeywordIf,
                    "else" => TokenType::KeywordElse,
                    "true" => TokenType::Boolean,
                    "false" => TokenType::Boolean,
                    "quantum" => TokenType::KeywordQuantum,
                    "circuit" => TokenType::KeywordCircuit,
                    "nano" => TokenType::KeywordNano,
                    "agent" => TokenType::KeywordAgent,
                    "remember" => TokenType::KeywordRemember,
                    "recall" => TokenType::KeywordRecall,
                    "learn" => TokenType::KeywordLearn,
                    "wisdom" => TokenType::KeywordWisdom,
                    "zamani" => TokenType::KeywordZamani,
                    "sasa" => TokenType::KeywordSasa,
                    "ancestor" => TokenType::KeywordAncestor,
                    "linear" => TokenType::KeywordLinear,
                    "affine" => TokenType::KeywordAffine,
                    "handle" => TokenType::KeywordHandle,
                    "effect" => TokenType::KeywordEffect,
                    "perform" => TokenType::KeywordPerform,
                    "unsafe" => TokenType::KeywordUnsafe,
                    "type" => TokenType::KeywordType,
                    // ... other keywords
                    _ => TokenType::Identifier,
                };
            }
            Some(c) if c.is_ascii_digit() => {
                literal = self.read_number();
                token_type = if literal.contains('.') {
                    TokenType::Float
                } else {
                    TokenType::Integer
                };
            }
            None => token_type = TokenType::EOF,
            _ => {
                token_type = TokenType::Illegal;
                let err_span = self.make_token_span(self.position, self.position);
                self.errors.push(LexerError {
                    message: format!("Illegal character: '{}'", self.ch.unwrap_or(' ')),
                    span: err_span,
                });
            }
        }

        let end_pos = self.position; // This is the start of the *next* character.
        // If the `match` arm consumed chars, self.position is already updated. 
        // If not, we need to manually advance for single-char tokens. 
        if token_type != TokenType::QuantumLiteral && token_type != TokenType::NanoAnnotation && !token_type.is_keyword_or_literal_with_read_ahead() && self.ch.is_some() {
            self.read_char(); // Advance for single character tokens and unhandled multi-char. 
        }

        // Special handling for QuantumLiteral, NanoAnnotation, MTSLiteral to capture full literal
        // The literal might have been set already by read_string/read_number/read_identifier
        if literal.is_empty() {
             literal = self.input.get(start_pos.0 as usize .. end_pos.0 as usize).unwrap_or("").to_string();
        }
        
        let span = self.make_token_span(start_pos, end_pos); // Span goes from start_pos to the char *before* the current self.position

        Token { token_type, literal, span }
    }
}

impl TokenType {
    // Helper to identify token types that automatically advance position when their literal is fully read.
    fn is_keyword_or_literal_with_read_ahead(&self) -> bool {
        matches!(self, TokenType::Identifier | TokenType::Integer | TokenType::Float | TokenType::String | TokenType::Char)
    }
}

// Iterator implementation
impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if token.token_type == TokenType::EOF {
            None
        } else {
            Some(token)
        }
    }
}
