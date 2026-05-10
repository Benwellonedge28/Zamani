//! Zenith Lexical Analyzer (Lexer)
//!
//! This module implements the lexical analysis phase of the Zenith compiler.
//! It converts the input source code into a stream of tokens based on the
//! NIMBUS Grammar v2.0 Trinity Edition rules. This lexer handles the unified
//! grammar across NIMBUS, Zenith, and Sankofa, including advanced literals
//! for quantum, nano, and multi-timeline systems, and tokens for algebraic effects.

use crate::tokens::{Token, TokenType};
use crate::source::SourceCode;
use std::collections::HashMap;

pub struct Lexer {
    source: SourceCode,
    chars: std::str::Chars<'static>, // Iterator over characters
    current_char: Option<char>,
    position: usize,
    keywords: HashMap<&'static str, TokenType>,
}

impl Lexer {
    pub fn new(source_code: &str) -> Self {
        let mut lexer = Lexer {
            source: SourceCode::new(source_code.to_string()),
            chars: "".chars(), // Will be re-initialized in init_chars
            current_char: None,
            position: 0,
            keywords: Self::build_keywords_map(),
        };
        lexer.init_chars();
        lexer
    }

    fn init_chars(&mut self) {
        // This is a workaround for the 'static lifetime of `chars`
        // In a real Rust project, `SourceCode` would hold the string, and `chars`
        // would be an iterator over its contents with the correct lifetime.
        // For conceptual purposes, we cast to 'static.
        let raw_src: &'static str = Box::leak(self.source.content.clone().into_boxed_str());
        self.chars = raw_src.chars();
        self.advance(); // Set initial current_char
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
        // ... many more keywords (140 total)
        map
    }

    fn advance(&mut self) {
        self.current_char = self.chars.next();
        self.position += 1;
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_identifier_or_keyword(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }
        identifier
    }

    fn read_number(&mut self) -> String {
        let mut number = String::new();
        while let Some(c) = self.current_char {
            if c.is_digit(10) {
                number.push(c);
                self.advance();
            } else {
                break;
            }
        }
        number
    }

    // Handles specific literals like quantum Dirac notation, nano agent syntax
    fn handle_special_literals(&mut self) -> Option<Token> {
        if self.current_char == Some('|') && self.peek() == Some(' ') { // '| QUBIT_STATE ⟩'
            // Conceptual logic to parse quantum qubit state
            self.advance(); // Skip '|'
            self.advance(); // Skip space
            let mut state = String::new();
            while let Some(c) = self.current_char {
                if c.is_alphanumeric() || c == '_' {
                    state.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
            if self.current_char == Some('⟩') {
                self.advance(); // Skip '⟩'
                return Some(Token::new(TokenType::QuantumLiteral, format!("|{}⟩", state), self.position));
            }
        } else if self.current_char == Some('@') { // @atom(...) or @molecule(...)
            self.advance(); // Skip '@'
            let annotation = self.read_identifier_or_keyword();
            if annotation == "atom" || annotation == "molecule" {
                // Conceptual logic to parse the content within parentheses
                // (ELEMENT:ORBITAL) or (FORMULA)
                return Some(Token::new(TokenType::NanoAnnotation, format!("@{}", annotation), self.position));
            }
        }
        None
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let start_pos = self.position;

        if let Some(c) = self.current_char {
            // Handle special literals first
            if let Some(token) = self.handle_special_literals() {
                return Some(token);
            }

            match c {
                '=' => { self.advance(); Some(Token::new(TokenType::Assign, "=", start_pos)) }
                '+' => { self.advance(); Some(Token::new(TokenType::Plus, "+", start_pos)) }
                '-' => { self.advance(); Some(Token::new(TokenType::Minus, "-", start_pos)) }
                '*' => { self.advance(); Some(Token::new(TokenType::Star, "*", start_pos)) }
                '/' => { self.advance(); Some(Token::new(TokenType::Slash, "/", start_pos)) }
                '(' => { self.advance(); Some(Token::new(TokenType::LParen, "(", start_pos)) }
                ')' => { self.advance(); Some(Token::new(TokenType::RParen, ")", start_pos)) }
                '{' => { self.advance(); Some(Token::new(TokenType::LBrace, "{", start_pos)) }
                '}' => { self.advance(); Some(Token::new(TokenType::RBrace, "}", start_pos)) }
                '[' => { self.advance(); Some(Token::new(TokenType::LBracket, "[", start_pos)) }
                ']' => { self.advance(); Some(Token::new(TokenType::RBracket, "]", start_pos)) }
                ';' => { self.advance(); Some(Token::new(TokenType::Semicolon, ";", start_pos)) }
                ':' => { self.advance(); Some(Token::new(TokenType::Colon, ":", start_pos)) }
                ',' => { self.advance(); Some(Token::new(TokenType::Comma, ",", start_pos)) }
                '<' => { self.advance(); Some(Token::new(TokenType::LT, "<", start_pos)) }
                '>' => { self.advance(); Some(Token::new(TokenType::GT, ">", start_pos)) }
                // ... handle other single-character tokens
                '"' => {
                    self.advance(); // Skip opening quote
                    let mut literal = String::new();
                    while let Some(ch) = self.current_char {
                        if ch == '"' {
                            self.advance(); // Skip closing quote
                            return Some(Token::new(TokenType::String, literal, start_pos));
                        }
                        literal.push(ch);
                        self.advance();
                    }
                    Some(Token::new(TokenType::Illegal, "Unterminated string", start_pos))
                }
                c if c.is_alphabetic() || c == '_' => {
                    let literal = self.read_identifier_or_keyword();
                    let token_type = self.keywords.get(literal.as_str()).cloned().unwrap_or(TokenType::Identifier);
                    Some(Token::new(token_type, literal, start_pos))
                }
                c if c.is_digit(10) => {
                    let literal = self.read_number();
                    Some(Token::new(TokenType::Integer, literal, start_pos))
                }
                _ => {
                    self.advance();
                    Some(Token::new(TokenType::Illegal, c.to_string(), start_pos))
                }
            }
        } else {
            None // End of input
        }
    }
}

// Placeholder types for the lexer to compile conceptually
pub mod tokens {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum TokenType {
        // Keywords
        KeywordQuantum, KeywordNano, KeywordEffect, KeywordHandle,
        KeywordRemember, KeywordRecall, KeywordLearn, KeywordWisdom,
        KeywordZamani, KeywordSasa, KeywordUnsafe,
        // Operators
        Assign, Plus, Minus, Star, Slash,
        // Delimiters
        LParen, RParen, LBrace, RBrace, LBracket, RBracket,
        Semicolon, Colon, Comma, LT, GT,
        // Literals
        Identifier, Integer, String, QuantumLiteral, NanoAnnotation,
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
