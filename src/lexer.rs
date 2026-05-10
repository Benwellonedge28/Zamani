//! Zenith Lexical Analyzer (Lexer)
//!
//! This module implements the lexical analysis phase of the Zenith compiler.
//! It converts the input source code into a stream of tokens based on the
//! NIMBUS Grammar v2.0 Trinity Edition rules. This lexer handles the unified
//! grammar across NIMBUS, Zenith, and Sankofa, including advanced literals
//! for quantum, nano, and multi-timeline systems, and tokens for algebraic effects.
//! This version includes detailed position tracking for improved error reporting.

use crate::tokens::{Token, TokenType, Span}; // Import Span
use crate::source::SourceCode;
use std::collections::HashMap;

// --- Lexer Structure ---
pub struct Lexer {
    source: SourceCode,
    chars: std::iter::Peekable<std::str::Chars<'static>>, // Peekable iterator over characters
    current_char_offset: usize, // Byte offset of current character
    current_line: usize,
    current_column: usize,
    keywords: HashMap<&'static str, TokenType>,
}

impl Lexer {
    pub fn new(source_code: &str) -> Self {
        let mut lexer = Lexer {
            source: SourceCode::new(source_code.to_string()),
            chars: "".chars().peekable(), // Dummy init, will be replaced
            current_char_offset: 0,
            current_line: 1,
            current_column: 1,
            keywords: Self::build_keywords_map(),
        };
        lexer.init_chars(); // Proper initialization
        lexer
    }

    fn init_chars(&mut self) {
        let raw_src: &'static str = Box::leak(self.source.content.clone().into_boxed_str());
        self.chars = raw_src.chars().peekable();
        // Don't call read_char here, let the first `next()` of the iterator handle it.
        // Or, if we keep `read_char` consuming, then ensure it updates line/column.
    }

    fn build_keywords_map() -> HashMap<&'static str, TokenType> {
        let mut map = HashMap::new();
        // Zenith-specific keywords
        map.insert("quantum", TokenType::KeywordQuantum);
        map.insert("nano", TokenType::KeywordNano);
        map.insert("effect", TokenType::KeywordEffect);
        map.insert("handle", TokenType::KeywordHandle);
        map.insert("language", TokenType::KeywordLanguage);
        map.insert("type", TokenType::KeywordType);
        map.insert("kind", TokenType::KeywordKind);
        map.insert("sort", TokenType::KeywordSort);
        map.insert("prop", TokenType::KeywordProp);
        map.insert("linear", TokenType::KeywordLinear);
        map.insert("affine", TokenType::KeywordAffine);
        map.insert("unsafe", TokenType::KeywordUnsafe); // for evas_cert
        // Sankofa keywords
        map.insert("remember", TokenType::KeywordRemember);
        map.insert("recall", TokenType::KeywordRecall);
        map.insert("learn", TokenType::KeywordLearn);
        map.insert("wisdom", TokenType::KeywordWisdom);
        map.insert("zamani", TokenType::KeywordZamani);
        map.insert("sasa", TokenType::KeywordSasa);
        map.insert("ancestral", TokenType::KeywordAncestral);
        map.insert("consensus", TokenType::KeywordConsensus);
        map.insert("observe", TokenType::KeywordObserve);
        map.insert("living_doc", TokenType::KeywordLivingDoc);
        map.insert("temporal_learn", TokenType::KeywordTemporalLearn);
        // Common language keywords (examples)
        map.insert("fn", TokenType::KeywordFn);
        map.insert("let", TokenType::KeywordLet);
        map.insert("if", TokenType::KeywordIf);
        map.insert("else", TokenType::KeywordElse);
        map.insert("return", TokenType::KeywordReturn);
        map.insert("true", TokenType::KeywordTrue);
        map.insert("false", TokenType::KeywordFalse);
        map.insert("mts", TokenType::KeywordMts); // For mts_lit
        // ... (all 140 keywords would go here)
        map
    }

    // --- Character & Cursor Management ---
    // This `read_char` now advances the actual character iterator
    fn read_char_and_advance_pos(&mut self) -> Option<char> {
        let c = self.chars.next();
        if let Some(ch) = c {
            if ch == '\n' {
                self.current_line += 1;
                self.current_column = 1;
            } else {
                self.current_column += 1;
            }
            self.current_char_offset += ch.len_utf8(); // Advance byte offset
        }
        c
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    // --- Skipping Utilities ---
    fn skip_whitespace_and_comments(&mut self) {
        loop {
            let start_offset = self.current_char_offset;
            self.skip_whitespace();
            self.skip_comments();
            self.skip_whitespace(); // After skipping comments, might be more whitespace
            if self.current_char_offset == start_offset {
                // No more whitespace or comments were skipped
                break;
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if c.is_whitespace() {
                self.read_char_and_advance_pos();
            } else {
                break;
            }
        }
    }

    fn skip_comments(&mut self) {
        // Single-line comments: // ...
        if self.peek_char() == Some(&'/') && self.chars.clone().nth(1) == Some('/') {
            self.read_char_and_advance_pos(); // Consume first '/'
            self.read_char_and_advance_pos(); // Consume second '/'
            while let Some(&c) = self.peek_char() {
                if c == '\n' {
                    self.read_char_and_advance_pos(); // Consume newline
                    return;
                }
                self.read_char_and_advance_pos();
            }
            return; // Reached EOF in single-line comment
        }
        // Multi-line comments: /* ... */
        if self.peek_char() == Some(&'/') && self.chars.clone().nth(1) == Some('*') {
            self.read_char_and_advance_pos(); // Consume '/'
            self.read_char_and_advance_pos(); // Consume '*'
            loop {
                if self.peek_char() == Some(&'*') && self.chars.clone().nth(1) == Some('/') {
                    self.read_char_and_advance_pos(); // Consume '*'
                    self.read_char_and_advance_pos(); // Consume '/'
                    return;
                }
                if self.peek_char().is_none() {
                    // Conceptual: Report unterminated multi-line comment error (e.g., return an Illegal token with span)
                    return;
                }
                self.read_char_and_advance_pos();
            }
        }
    }

    // --- Reading Utilities ---
    fn read_identifier_or_keyword(&mut self, first_char: char) -> String {
        let mut ident = String::from(first_char);
        while let Some(&c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(self.read_char_and_advance_pos().unwrap());
            } else {
                break;
            }
        }
        ident
    }

    fn read_number(&mut self, first_digit: char) -> String {
        let mut num = String::from(first_digit);
        let mut has_decimal = false;

        while let Some(&c) = self.peek_char() {
            if c.is_digit(10) {
                num.push(self.read_char_and_advance_pos().unwrap());
            } else if c == '.' && !has_decimal && self.peek_char_n(2).map_or(false, |next_c| next_c.is_digit(10)) {
                num.push(self.read_char_and_advance_pos().unwrap()); // Consume '.'
                has_decimal = true;
            } else {
                break;
            }
        }
        num
    }

    fn read_string_literal_content(&mut self) -> String {
        let start_offset = self.current_char_offset; // After opening quote
        // Conceptual: handle escape sequences here, e.g., '\n', '\"'
        while let Some(&c) = self.peek_char() {
            if c == '"' {
                break; // Found closing quote
            }
            // Add more complex escape handling here for a real lexer,
            // consuming multiple chars for an escape sequence.
            self.read_char_and_advance_pos();
        }
        self.source.content[start_offset..self.current_char_offset].to_string()
    }

    fn read_char_literal_content(&mut self) -> String {
        let start_offset = self.current_char_offset; // After opening single quote
        // Conceptual: handle escape sequences
        while let Some(&c) = self.peek_char() {
            if c == ''' {
                break; // Found closing quote
            }
            self.read_char_and_advance_pos();
        }
        self.source.content[start_offset..self.current_char_offset].to_string()
    }

    fn peek_char_n(&mut self, n: usize) -> Option<char> {
        self.chars.clone().nth(n - 1).copied()
    }


    // --- Special Literal Handlers ---
    fn handle_quantum_literal(&mut self, start_span: Span) -> Option<Token> {
        // Assuming '|' and then ' ' have been peeked/consumed for initial check
        self.read_char_and_advance_pos(); // Consume space after '|'
        let state_start_offset = self.current_char_offset;
        while let Some(&c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' || c == '+' || c == '-' { // Allow for |+⟩ or |-⟩ states
                self.read_char_and_advance_pos();
            } else {
                break;
            }
        }
        let state_str = self.source.content[state_start_offset..self.current_char_offset].to_string();
        if self.peek_char() == Some(&'⟩') { // Dirac notation
            self.read_char_and_advance_pos(); // Consume '⟩'
            Some(Token::new(TokenType::QuantumLiteral, format!("|{}⟩", state_str), Span { end: self.current_char_offset, ..start_span }))
        } else {
            // Not a valid quantum literal, return an Illegal token spanning what was parsed
            Some(Token::new(TokenType::Illegal, self.source.content[start_span.start..self.current_char_offset].to_string(), Span { end: self.current_char_offset, ..start_span }))
        }
    }

    fn handle_nano_annotation(&mut self, start_span: Span) -> Option<Token> {
        // Assuming '@' has been consumed
        let annotation_start_offset = self.current_char_offset;
        while let Some(&c) = self.peek_char() {
            if c.is_alphabetic() { // read "atom" or "molecule"
                self.read_char_and_advance_pos();
            } else {
                break;
            }
        }
        let annotation = self.source.content[annotation_start_offset..self.current_char_offset].to_string();
        if (annotation == "atom" || annotation == "molecule") && self.peek_char() == Some(&'(') {
            self.read_char_and_advance_pos(); // Consume '('
            let mut paren_nesting = 1;
            while let Some(&c) = self.peek_char() {
                if c == '(' { paren_nesting += 1; }
                if c == ')' { paren_nesting -= 1; }
                self.read_char_and_advance_pos();
                if paren_nesting == 0 { break; }
            }
            // End span here
            Some(Token::new(TokenType::NanoAnnotation, self.source.content[start_span.start..self.current_char_offset].to_string(), Span { end: self.current_char_offset, ..start_span }))
        } else {
            // Not a valid nano annotation
            Some(Token::new(TokenType::Illegal, self.source.content[start_span.start..self.current_char_offset].to_string(), Span { end: self.current_char_offset, ..start_span }))
        }
    }

    fn handle_mts_literal(&mut self, start_span: Span) -> Option<Token> {
        // Assumes 'm' was current_char, and 't', 's' were peeked and matched
        // Need to explicitly consume 't' and 's' now
        self.read_char_and_advance_pos(); // Consume 't'
        self.read_char_and_advance_pos(); // Consume 's'

        if self.peek_char() == Some(&'[') {
            self.read_char_and_advance_pos(); // Consume '['
            let number_start_offset = self.current_char_offset;
            while let Some(&c) = self.peek_char() {
                if c.is_digit(10) {
                    self.read_char_and_advance_pos();
                } else {
                    break;
                }
            }
            let number_str = self.source.content[number_start_offset..self.current_char_offset].to_string();
            if self.peek_char() == Some(&']') {
                self.read_char_and_advance_pos(); // Consume ']'
                Some(Token::new(TokenType::MTSLiteral, self.source.content[start_span.start..self.current_char_offset].to_string(), Span { end: self.current_char_offset, ..start_span }))
            } else {
                // Unterminated MTS literal
                Some(Token::new(TokenType::Illegal, self.source.content[start_span.start..self.current_char_offset].to_string(), Span { end: self.current_char_offset, ..start_span }))
            }
        } else {
            // Not a valid MTS literal, was just 'mts' as an identifier
            None // Allow it to be re-lexed as an identifier
        }
    }

    fn handle_directive(&mut self, start_span: Span) -> Option<Token> {
        // Assuming '#' was consumed
        let directive_name_start_offset = self.current_char_offset;
        while let Some(&c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' {
                self.read_char_and_advance_pos();
            } else {
                break;
            }
        }
        let directive_name = self.source.content[directive_name_start_offset..self.current_char_offset].to_string();
        Some(Token::new(TokenType::Directive, format!("#{}", directive_name), Span { end: self.current_char_offset, ..start_span }))
    }
}

// --- Iterator Implementation for Lexer ---
impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace_and_comments();

        let start_offset = self.current_char_offset;
        let start_line = self.current_line;
        let start_column = self.current_column;

        let current_char = self.read_char_and_advance_pos(); // Read the character for the next token

        if let Some(c) = current_char {
            let initial_span = Span { start: start_offset, end: self.current_char_offset, line: start_line, column: start_column };

            // Try to match complex tokens first that consume multiple characters
            // and determine their own end_offset
            if c == '#' { return self.handle_directive(initial_span); }
            if c == '@' { return self.handle_nano_annotation(initial_span); }

            // Special handling for '|' to differentiate from quantum literal
            if c == '|' {
                if self.peek_char() == Some(&' ') { // This might be start of a quantum literal
                    return self.handle_quantum_literal(initial_span);
                } else if self.peek_char() == Some(&'|') { // Logical OR '||'
                    self.read_char_and_advance_pos();
                    let end_offset = self.current_char_offset;
                    return Some(Token::new(TokenType::LogicalOr, "||", Span { end: end_offset, ..initial_span }));
                } else {
                    // It's a regular pipe operator
                    return Some(Token::new(TokenType::Pipe, "|", initial_span));
                }
            }
            if c == 'm' {
                if self.peek_char() == Some(&'t') && self.peek_char_n(2) == Some('s') { // Check for 'mts' literal
                 if let Some(token) = self.handle_mts_literal(initial_span) {
                    return Some(token);
                }
            }
            }


            // --- Match single and multi-character operators/literals ---
            let (token_type, literal_str) = match c {
                // --- Multi-character operators and their single-char counterparts ---
                '=' => {
                    if self.peek_char() == Some(&'=') { self.read_char_and_advance_pos(); (TokenType::Equals, "==") }
                    else { (TokenType::Assign, "=") }
                }
                '!' => {
                    if self.peek_char() == Some(&'=') { self.read_char_and_advance_pos(); (TokenType::NotEquals, "!=") }
                    else { (TokenType::Bang, "!") }
                }
                '<' => {
                    if self.peek_char() == Some(&'=') { self.read_char_and_advance_pos(); (TokenType::LTE, "<=") }
                    else { (TokenType::LT, "<") }
                }
                '>' => {
                    if self.peek_char() == Some(&'=') { self.read_char_and_advance_pos(); (TokenType::GTE, ">=") }
                    else { (TokenType::GT, ">") }
                }
                '&' => {
                    if self.peek_char() == Some(&'&') { self.read_char_and_advance_pos(); (TokenType::LogicalAnd, "&&") }
                    else { (TokenType::BitwiseAnd, "&") }
                }
                // '|' already handled special case (quantum literal, logical OR, pipe)
                '^' => (TokenType::Caret, "^"),

                // --- Single-character operators/delimiters ---
                '+' => (TokenType::Plus, "+"),
                '-' => (TokenType::Minus, "-"),
                '*' => (TokenType::Star, "*"),
                '/' => (TokenType::Slash, "/"),
                '(' => (TokenType::LParen, "("),
                ')' => (TokenType::RParen, ")"),
                '{' => (TokenType::LBrace, "{"),
                '}' => (TokenType::RBrace, "}"),
                '[' => (TokenType::LBracket, "["),
                ']' => (TokenType::RBracket, "]"),
                ';' => (TokenType::Semicolon, ";"),
                ':' => (TokenType::Colon, ":"),
                ',' => (TokenType::Comma, ","),
                '.' => (TokenType::Dot, "."),

                // --- Literals ---
                '"' => {
                    let literal = self.read_string_literal_content();
                    let current_char_after_literal = self.read_char_and_advance_pos(); // Consume the closing quote
                    if current_char_after_literal == Some('"') {
                         (TokenType::String, literal.as_str())
                    } else {
                        // Conceptual: Handle unterminated string literal error
                        (TokenType::Illegal, literal.as_str())
                    }
                }
                ''' => {
                    let literal = self.read_char_literal_content();
                    let current_char_after_literal = self.read_char_and_advance_pos(); // Consume the closing quote
                    if current_char_after_literal == Some(''') {
                        (TokenType::Char, literal.as_str())
                    } else {
                        // Conceptual: Handle unterminated char literal error
                        (TokenType::Illegal, literal.as_str())
                    }
                }
                c if c.is_alphabetic() || c == '_' => {
                    let literal = self.read_identifier_or_keyword(c);
                    let token_type = self.keywords.get(literal.as_str()).cloned().unwrap_or(TokenType::Identifier);
                    return Some(Token::new(token_type, literal, Span { end: self.current_char_offset, ..initial_span }));
                }
                c if c.is_digit(10) => {
                    let literal = self.read_number(c);
                    let token_type = if literal.contains('.') { TokenType::Float } else { TokenType::Integer };
                    return Some(Token::new(token_type, literal, Span { end: self.current_char_offset, ..initial_span }));
                }
                _ => (TokenType::Illegal, c.to_string().as_str()),
            };
            Some(Token::new(token_type_and_literal.0, token_type_and_literal.1, Span { end: self.current_char_offset, ..initial_span }))
        } else {
            None // End of input
        }
    }
}


// --- Token & TokenType Definitions ---
pub mod tokens {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Span {
        pub start: usize,   // Byte offset start
        pub end: usize,     // Byte offset end (exclusive)
        pub line: usize,
        pub column: usize,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum TokenType {
        // --- Single-character operators/delimiters ---
        Assign, Plus, Minus, Star, Slash,
        LParen, RParen, LBrace, RBrace, LBracket, RBracket,
        Semicolon, Colon, Comma, Dot, Pipe, Caret,

        // --- Multi-character operators ---
        Equals, NotEquals, LT, GT, LTE, GTE, Bang,
        BitwiseAnd, LogicalAnd, LogicalOr, // Bitwise and Logical operators

        // --- Literals ---
        Identifier, Integer, Float, String, Char,
        QuantumLiteral, // e.g., |0⟩, |+⟩
        NanoAnnotation, // e.g., @atom(O:2s) @molecule(H2O)
        MTSLiteral,     // e.g., mts[5]

        // --- Keywords (examples from Zenith, Sankofa, Nimbus) ---
        KeywordFn, KeywordLet, KeywordIf, KeywordElse, KeywordReturn,
        KeywordTrue, KeywordFalse,
        KeywordQuantum, KeywordNano, KeywordEffect, KeywordHandle,
        KeywordLanguage, KeywordType, KeywordKind, KeywordSort, KeywordProp,
        KeywordLinear, KeywordAffine, KeywordUnsafe, // for evas_cert
        KeywordRemember, KeywordRecall, KeywordLearn, KeywordWisdom,
        KeywordZamani, KeywordSasa, KeywordAncestral, KeywordConsensus,
        KeywordObserve, KeywordLivingDoc, KeywordTemporalLearn,
        KeywordMts, // For Multi-Timeline System literal

        // --- Directives ---
        Directive, // e.g., #language, #pragma

        // --- Special ---
        Illegal,    // Represents an unrecognized token
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Token {
        pub token_type: TokenType,
        pub literal: String,
        pub span: Span,
    }

    impl Token {
        pub fn new(token_type: TokenType, literal: impl Into<String>, span: Span) -> Self {
            Token { token_type, literal: literal.into(), span }
        }
    }
}

// --- Source Code Structure ---
pub mod source {
    #[derive(Debug, Clone)]
    pub struct SourceCode {
        pub content: String,
    }
    impl SourceCode {
        pub fn new(content: String) -> Self { SourceCode { content } }
    }
}
