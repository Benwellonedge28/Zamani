//! Zenith Lexical Analyzer (Lexer)
//!
//! This module implements the lexical analysis phase of the Zenith compiler.
//! It converts the input source code into a stream of tokens based on the
//! NIMBUS Grammar v2.0 Trinity Edition rules. This lexer handles the unified
//! grammar across NIMBUS, Zenith, and Sankofa, including advanced literals
//! for quantum, nano, and multi-timeline systems, and tokens for algebraic effects.
//! This version includes conceptual handling for comments, multi-character operators,
//! and meta-compilation directives.

use crate::tokens::{Token, TokenType};
use crate::source::SourceCode;
use std::collections::HashMap;

pub struct Lexer {
    source: SourceCode,
    chars: std::str::Chars<'static>, // Iterator over characters
    current_char: Option<char>,
    position: usize,
    read_position: usize, // Next position to read
    keywords: HashMap<&'static str, TokenType>,
}

impl Lexer {
    pub fn new(source_code: &str) -> Self {
        let mut lexer = Lexer {
            source: SourceCode::new(source_code.to_string()),
            chars: "".chars(), // Will be re-initialized in init_chars
            current_char: None,
            position: 0,
            read_position: 0,
            keywords: Self::build_keywords_map(),
        };
        lexer.init_chars();
        lexer
    }

    fn init_chars(&mut self) {
        let raw_src: &'static str = Box::leak(self.source.content.clone().into_boxed_str());
        self.chars = raw_src.chars();
        self.read_char(); // Set initial current_char and read_position
    }

    fn build_keywords_map() -> HashMap<&'static str, TokenType> {
        let mut map = HashMap::new();
        // Zenith-specific keywords
        map.insert("quantum", TokenType::KeywordQuantum);
        map.insert("nano", TokenType::KeywordNano);
        map.insert("effect", TokenType::KeywordEffect);
        map.insert("handle", TokenType::KeywordHandle);
        map.insert("remember", TokenType::KeywordRemember); // Sankofa
        map.insert("recall", TokenType::KeywordRecall);     // Sankofa
        map.insert("learn", TokenType::KeywordLearn);       // Sankofa
        map.insert("wisdom", TokenType::KeywordWisdom);     // Sankofa
        map.insert("zamani", TokenType::KeywordZamani);     // Sankofa
        map.insert("sasa", TokenType::KeywordSasa);         // Sankofa
        map.insert("unsafe", TokenType::KeywordUnsafe);     // For evas_cert
        map.insert("language", TokenType::KeywordLanguage); // For meta-compilation
        // ... (remaining 140 total keywords)
        map
    }

    fn read_char(&mut self) {
        self.current_char = if self.read_position < self.source.content.len() {
            // Need to handle UTF-8 chars correctly, this is a conceptual single byte read
            self.source.content.chars().nth(self.read_position)
        } else {
            None
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> Option<char> {
        if self.read_position < self.source.content.len() {
            self.source.content.chars().nth(self.read_position)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn skip_comments(&mut self) {
        // Single-line comments: // ...
        if self.current_char == Some('/') && self.peek_char() == Some('/') {
            while let Some(c) = self.current_char {
                if c == '\n' {
                    self.read_char(); // Consume the newline
                    return;
                }
                self.read_char();
            }
        }
        // Multi-line comments: /* ... */
        if self.current_char == Some('/') && self.peek_char() == Some('*') {
            self.read_char(); // Consume '/'
            self.read_char(); // Consume '*'
            loop {
                if self.current_char == Some('*') && self.peek_char() == Some('/') {
                    self.read_char(); // Consume '*'
                    self.read_char(); // Consume '/'
                    return;
                }
                if self.current_char.is_none() {
                    // Unterminated multi-line comment error
                    return;
                }
                self.read_char();
            }
        }
    }

    fn read_identifier_or_keyword(&mut self) -> String {
        let start_pos = self.position;
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                self.read_char();
            } else {
                break;
            }
        }
        self.source.content[start_pos..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let start_pos = self.position;
        while let Some(c) = self.current_char {
            if c.is_digit(10) {
                self.read_char();
            } else {
                break;
            }
        }
        self.source.content[start_pos..self.position].to_string()
    }

    // Handles specific literals like quantum Dirac notation, nano agent syntax
    fn handle_special_literals(&mut self) -> Option<Token> {
        let start_literal_pos = self.position;
        if self.current_char == Some('|') { // '| QUBIT_STATE ⟩'
            // Need to check for ' ' after '|' to distinguish from bitwise OR
            if self.peek_char() == Some(' ') {
                self.read_char(); // Consume '|'
                self.read_char(); // Consume space
                let state_start = self.position;
                while let Some(c) = self.current_char {
                    if c.is_alphanumeric() || c == '_' {
                        self.read_char();
                    } else {
                        break;
                    }
                }
                let state_str = self.source.content[state_start..self.position].to_string();
                if self.current_char == Some('⟩') {
                    self.read_char(); // Consume '⟩'
                    return Some(Token::new(TokenType::QuantumLiteral, format!("|{}⟩", state_str), start_literal_pos));
                }
            }
        } else if self.current_char == Some('@') { // @atom(...) or @molecule(...)
            self.read_char(); // Consume '@'
            let annotation_start = self.position;
            while let Some(c) = self.current_char {
                if c.is_alphabetic() { // read "atom" or "molecule"
                    self.read_char();
                } else {
                    break;
                }
            }
            let annotation = self.source.content[annotation_start..self.position].to_string();
            if (annotation == "atom" || annotation == "molecule") && self.current_char == Some('(') {
                // Conceptual logic to parse the content within parentheses
                // (ELEMENT:ORBITAL) or (FORMULA)
                let paren_content_start = self.position;
                let mut paren_nesting = 0;
                while let Some(c) = self.current_char {
                    if c == '(' { paren_nesting += 1; }
                    if c == ')' { paren_nesting -= 1; }
                    self.read_char();
                    if paren_nesting == 0 && c == ')' { break; } // Found matching ')'
                }
                let full_literal = self.source.content[start_literal_pos..self.position].to_string();
                return Some(Token::new(TokenType::NanoAnnotation, full_literal, start_literal_pos));
            }
        }
        None
    }

    // Conceptual handling for meta-compilation directives like #language
    fn handle_directive(&mut self) -> Option<Token> {
        let start_directive_pos = self.position;
        if self.current_char == Some('#') {
            self.read_char(); // Consume '#'
            let directive_name = self.read_identifier_or_keyword();
            // This is where a real lexer would identify specific directives
            // e.g., #language, #pragma, #include (if Zenith had one)
            return Some(Token::new(TokenType::Directive, format!("#{}", directive_name), start_directive_pos));
        }
        None
    }


    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        self.skip_comments();
        self.skip_whitespace(); // Skip whitespace after comments

        let start_pos = self.position;

        if let Some(c) = self.current_char {
            // Handle directives first
            if let Some(token) = self.handle_directive() {
                return Some(token);
            }
            // Handle special literals
            if let Some(token) = self.handle_special_literals() {
                return Some(token);
            }

            let mut token = None;
            match c {
                '=' => {
                    self.read_char();
                    if self.current_char == Some('=') { self.read_char(); token = Some(Token::new(TokenType::Equals, "==", start_pos)); }
                    else { token = Some(Token::new(TokenType::Assign, "=", start_pos)); }
                }
                '!' => {
                    self.read_char();
                    if self.current_char == Some('=') { self.read_char(); token = Some(Token::new(TokenType::NotEquals, "!=", start_pos)); }
                    else { token = Some(Token::new(TokenType::Bang, "!", start_pos)); }
                }
                '<' => {
                    self.read_char();
                    if self.current_char == Some('=') { self.read_char(); token = Some(Token::new(TokenType::LTE, "<=", start_pos)); }
                    else { token = Some(Token::new(TokenType::LT, "<", start_pos)); }
                }
                '>' => {
                    self.read_char();
                    if self.current_char == Some('=') { self.read_char(); token = Some(Token::new(TokenType::GTE, ">=", start_pos)); }
                    else { token = Some(Token::new(TokenType::GT, ">", start_pos)); }
                }
                // ... handle other multi-character tokens
                '+' => { self.read_char(); token = Some(Token::new(TokenType::Plus, "+", start_pos)); }
                '-' => { self.read_char(); token = Some(Token::new(TokenType::Minus, "-", start_pos)); }
                '*' => { self.read_char(); token = Some(Token::new(TokenType::Star, "*", start_pos)); }
                '/' => { self.read_char(); token = Some(Token::new(TokenType::Slash, "/", start_pos)); } // Single slash after comment check
                '(' => { self.read_char(); token = Some(Token::new(TokenType::LParen, "(", start_pos)); }
                ')' => { self.read_char(); token = Some(Token::new(TokenType::RParen, ")", start_pos)); }
                '{' => { self.read_char(); token = Some(Token::new(TokenType::LBrace, "{", start_pos)); }
                '}' => { self.read_char(); token = Some(Token::new(TokenType::RBrace, "}", start_pos)); }
                '[' => { self.read_char(); token = Some(Token::new(TokenType::LBracket, "[", start_pos)); }
                ']' => { self.read_char(); token = Some(Token::new(TokenType::RBracket, "]", start_pos)); }
                ';' => { self.read_char(); token = Some(Token::new(TokenType::Semicolon, ";", start_pos)); }
                ':' => { self.read_char(); token = Some(Token::new(TokenType::Colon, ":", start_pos)); }
                ',' => { self.read_char(); token = Some(Token::new(TokenType::Comma, ",", start_pos)); }
                '.' => { self.read_char(); token = Some(Token::new(TokenType::Dot, ".", start_pos)); } // Added for completeness

                '"' => {
                    self.read_char(); // Skip opening quote
                    let mut literal = String::new();
                    let literal_start = self.position;
                    while let Some(ch) = self.current_char {
                        if ch == '"' {
                            self.read_char(); // Skip closing quote
                            token = Some(Token::new(TokenType::String, literal, literal_start));
                            break;
                        }
                        literal.push(ch);
                        self.read_char();
                    }
                    if token.is_none() { // Fallback for unterminated string
                        token = Some(Token::new(TokenType::Illegal, "Unterminated string", start_pos));
                    }
                }
                c if c.is_alphabetic() || c == '_' => {
                    let literal = self.read_identifier_or_keyword();
                    let token_type = self.keywords.get(literal.as_str()).cloned().unwrap_or(TokenType::Identifier);
                    token = Some(Token::new(token_type, literal, start_pos));
                }
                c if c.is_digit(10) => {
                    let literal = self.read_number();
                    token = Some(Token::new(TokenType::Integer, literal, start_pos));
                }
                _ => {
                    self.read_char();
                    token = Some(Token::new(TokenType::Illegal, c.to_string(), start_pos));
                }
            }
            token
        } else {
            None // End of input
        }
    }
}

// Placeholder types for the lexer to compile conceptually
pub mod tokens {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum TokenType {
        // Keywords (Expanded)
        KeywordQuantum, KeywordNano, KeywordEffect, KeywordHandle,
        KeywordRemember, KeywordRecall, KeywordLearn, KeywordWisdom,
        KeywordZamani, KeywordSasa, KeywordUnsafe, KeywordLanguage,
        // Operators (Expanded for multi-char)
        Assign, Plus, Minus, Star, Slash,
        Equals, NotEquals, LT, GT, LTE, GTE, Bang,
        // Delimiters
        LParen, RParen, LBrace, RBrace, LBracket, RBracket,
        Semicolon, Colon, Comma, Dot,
        // Literals
        Identifier, Integer, String, QuantumLiteral, NanoAnnotation,
        // Directives
        Directive,
        // Special
        Illegal,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Token {
        pub token_type: TokenType,
        pub literal: String,
        pub position: usize,
    }

    impl Token {
        pub fn new(token_type: TokenType, literal: impl Into<String>, position: usize) -> Self {
            Token { token_type, literal: literal.into(), position }
        }
    }
}

pub mod source {
    pub struct SourceCode {
        pub content: String,
    }
    impl SourceCode {
        pub fn new(content: String) -> Self { SourceCode { content } }
    }
}
