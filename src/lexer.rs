//! Zenith Lexical Analyzer (Lexer)
//!
//! This module implements the lexical analysis phase of the Zenith compiler.
//! It converts the input source code into a stream of tokens based on the
//! NIMBUS Grammar v2.0 Trinity Edition rules. This lexer handles the unified
//! grammar across NIMBUS, Zenith, and Sankofa, including advanced literals
//! for quantum, nano, and multi-timeline systems, and tokens for algebraic effects.
//! This version incorporates detailed position tracking, conceptual error reporting,
//! and initial conceptual support for Unicode identifiers and string/char escape sequences.

use crate::tokens::{Token, TokenType, Span};
use crate::source::SourceCode;
use std::collections::HashMap;

// --- Lexer Structure ---
pub struct Lexer {
    source: SourceCode,
    chars: std::iter::Peekable<std::str::Chars<'static>>,
    current_char_offset: usize, // Byte offset of current character
    current_line: usize,
    current_column: usize,
    keywords: HashMap<&'static str, TokenType>,
    errors: Vec<LexerError>, // Conceptual list of accumulated errors
}

// --- LexerError Structure ---
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerError {
    pub message: String,
    pub span: Span,
}

impl Lexer {
    pub fn new(source_code: &str) -> Self {
        let mut lexer = Lexer {
            source: SourceCode::new(source_code.to_string()),
            chars: "".chars().peekable(),
            current_char_offset: 0,
            current_line: 1,
            current_column: 1,
            keywords: Self::build_keywords_map(),
            errors: Vec::new(), // Initialize error list
        };
        lexer.init_chars();
        lexer
    }

    fn init_chars(&mut self) {
        let raw_src: &'static str = Box::leak(self.source.content.clone().into_boxed_str());
        self.chars = raw_src.chars().peekable();
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
    fn read_char_and_advance_pos(&mut self) -> Option<char> {
        let c = self.chars.next();
        if let Some(ch) = c {
            if ch == '\n' {
                self.current_line += 1;
                self.current_column = 1;
            } else {
                self.current_column += 1;
            }
            self.current_char_offset += ch.len_utf8();
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
            self.skip_whitespace();
            if self.current_char_offset == start_offset {
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
            let start_span = Span::new(self.current_char_offset, self.current_line, self.current_column);
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
            let start_span = Span::new(self.current_char_offset, self.current_line, self.current_column);
            self.read_char_and_advance_pos(); // Consume '/'
            self.read_char_and_advance_pos(); // Consume '*'
            loop {
                if self.peek_char() == Some(&'*') && self.chars.clone().nth(1) == Some('/') {
                    self.read_char_and_advance_pos(); // Consume '*'
                    self.read_char_and_advance_pos(); // Consume '/'
                    return;
                }
                if self.peek_char().is_none() {
                    self.errors.push(LexerError {
                        message: "Unterminated multi-line comment.".to_string(),
                        span: Span { end: self.current_char_offset, ..start_span },
                    });
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
            // Allow Unicode characters that are valid in identifiers (e.g., in Rust)
            if c.is_alphanumeric() || c == '_' || c.is_xid_continue() { // is_xid_continue for Unicode
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

    fn read_string_literal_content(&mut self, start_span: Span) -> String {
        let start_offset = self.current_char_offset; // After opening quote
        let mut literal_content = String::new();
        while let Some(&c) = self.peek_char() {
            if c == '"' {
                break; // Found closing quote
            }
            if c == '\n' { // Unterminated string on new line
                self.errors.push(LexerError {
                    message: "Unterminated string literal.".to_string(),
                    span: Span { end: self.current_char_offset, ..start_span },
                });
                break;
            }
            if c == '\' { // Handle escape sequences
                self.read_char_and_advance_pos(); // Consume '\'
                if let Some(escaped_char) = self.read_char_and_advance_pos() {
                    match escaped_char {
                        'n' => literal_content.push('\n'),
                        't' => literal_content.push('\t'),
                        'r' => literal_content.push('\r'),
                        '\' => literal_content.push('\\'),
                        '"' => literal_content.push('"'),
                        ''' => literal_content.push('''),
                        '0' => literal_content.push('\0'),
                        'u' => { // Unicode escape sequence, e.g., \u{XXXX}
                            // Conceptual: read {XXXX} and convert to char
                            // For simplicity, just push 'u'
                            literal_content.push('u');
                            self.errors.push(LexerError {
                                message: "Conceptual: Unicode escape sequence \\u{XXXX} parsing needed.".to_string(),
                                span: Span { end: self.current_char_offset, ..start_span },
                            });
                        }
                        _ => {
                            self.errors.push(LexerError {
                                message: format!("Invalid escape sequence '\\{}'.", escaped_char),
                                span: Span { end: self.current_char_offset, ..start_span },
                            });
                            literal_content.push('\'); // Push back to avoid data loss in literal
                            literal_content.push(escaped_char);
                        }
                    }
                } else {
                    self.errors.push(LexerError {
                        message: "Incomplete escape sequence at end of string.".to_string(),
                        span: Span { end: self.current_char_offset, ..start_span },
                    });
                    literal_content.push('\');
                }
            } else {
                literal_content.push(self.read_char_and_advance_pos().unwrap());
            }
        }
        literal_content
    }

    fn read_char_literal_content(&mut self, start_span: Span) -> String {
        let start_offset = self.current_char_offset; // After opening single quote
        let mut literal_content = String::new();
        // Conceptual: handle escape sequences
        if let Some(&c) = self.peek_char() {
            if c == '\' { // Handle escape sequences
                self.read_char_and_advance_pos(); // Consume '\'
                if let Some(escaped_char) = self.read_char_and_advance_pos() {
                    match escaped_char {
                        'n' => literal_content.push('\n'),
                        't' => literal_content.push('\t'),
                        'r' => literal_content.push('\r'),
                        '\' => literal_content.push('\\'),
                        '"' => literal_content.push('"'),
                        ''' => literal_content.push('''),
                        _ => {
                            self.errors.push(LexerError {
                                message: format!("Invalid escape sequence '\\{}'.", escaped_char),
                                span: Span { end: self.current_char_offset, ..start_span },
                            });
                            literal_content.push('\');
                            literal_content.push(escaped_char);
                        }
                    }
                } else {
                    self.errors.push(LexerError {
                        message: "Incomplete escape sequence at end of char.".to_string(),
                        span: Span { end: self.current_char_offset, ..start_span },
                    });
                    literal_content.push('\');
                }
            } else if c != ''' { // Not an escape sequence and not the closing quote
                literal_content.push(self.read_char_and_advance_pos().unwrap());
            }
        }
        if literal_content.len() != 1 {
            self.errors.push(LexerError {
                message: "Character literal must contain exactly one character.".to_string(),
                span: Span { end: self.current_char_offset, ..start_span },
            });
        }

        literal_content
    }

    fn peek_char_n(&mut self, n: usize) -> Option<char> {
        self.chars.clone().nth(n - 1).copied()
    }

    // --- Special Literal Handlers ---
    fn handle_quantum_literal(&mut self, start_span: Span) -> Option<Token> {
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
        if self.peek_char() == Some(&'⟩') {
            self.read_char_and_advance_pos();
            Some(Token::new(TokenType::QuantumLiteral, format!("|{}⟩", state_str), Span { end: self.current_char_offset, ..start_span }))
        } else {
            self.errors.push(LexerError {
                message: "Malformed quantum Dirac literal: expected '⟩'.".to_string(),
                span: Span { end: self.current_char_offset, ..start_span },
            });
            Some(Token::new(TokenType::Illegal, self.source.content[start_span.start..self.current_char_offset].to_string(), Span { end: self.current_char_offset, ..start_span }))
        }
    }

    fn handle_nano_annotation(&mut self, start_span: Span) -> Option<Token> {
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
            if paren_nesting != 0 {
                self.errors.push(LexerError {
                    message: format!("Unterminated nano annotation '{}'.", annotation),
                    span: Span { end: self.current_char_offset, ..start_span },
                });
            }
            Some(Token::new(TokenType::NanoAnnotation, self.source.content[start_span.start..self.current_char_offset].to_string(), Span { end: self.current_char_offset, ..start_span }))
        } else {
            self.errors.push(LexerError {
                message: format!("Malformed nano annotation '@{}'.", annotation),
                span: Span { end: self.current_char_offset, ..start_span },
            });
            Some(Token::new(TokenType::Illegal, self.source.content[start_span.start..self.current_char_offset].to_string(), Span { end: self.current_char_offset, ..start_span }))
        }
    }

    fn handle_mts_literal(&mut self, start_span: Span) -> Option<Token> {
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
                self.errors.push(LexerError {
                    message: "Malformed MTS literal: expected ']'.".to_string(),
                    span: Span { end: self.current_char_offset, ..start_span },
                });
                Some(Token::new(TokenType::Illegal, self.source.content[start_span.start..self.current_char_offset].to_string(), Span { end: self.current_char_offset, ..start_span }))
            }
        } else {
            None
        }
    }

    fn handle_directive(&mut self, start_span: Span) -> Option<Token> {
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

    pub fn get_errors(&self) -> &[LexerError] {
        &self.errors
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

        let current_char = self.read_char_and_advance_pos();

        if let Some(c) = current_char {
            let initial_span = Span::new(start_offset, start_line, start_column);

            // Match complex tokens first
            if c == '#' { return self.handle_directive(initial_span); }
            if c == '@' { return self.handle_nano_annotation(initial_span); }
            if c == 'm' { // Check for 'mts' literal
                if self.peek_char() == Some(&'t') && self.peek_char_n(2) == Some('s') {
                    if let Some(token) = self.handle_mts_literal(initial_span) {
                        return Some(token);
                    }
                }
            }

            // Special handling for '|' to differentiate from quantum literal
            if c == '|' {
                if self.peek_char() == Some(&' ') { // This might be start of a quantum literal
                    return self.handle_quantum_literal(initial_span);
                } else if self.peek_char() == Some(&'|') { // Logical OR '||'
                    self.read_char_and_advance_pos();
                    return Some(Token::new(TokenType::LogicalOr, "||".to_string(), Span { end: self.current_char_offset, ..initial_span }));
                } else {
                    return Some(Token::new(TokenType::Pipe, "|".to_string(), initial_span));
                }
            }


            // --- Match single and multi-character operators/literals ---
            let (token_type, literal) = match c {
                // --- Multi-character operators and their single-char counterparts ---
                '=' => {
                    if self.peek_char() == Some(&'=') { self.read_char_and_advance_pos(); (TokenType::Equals, "==".to_string()) }
                    else { (TokenType::Assign, "=".to_string()) }
                }
                '!' => {
                    if self.peek_char() == Some(&'=') { self.read_char_and_advance_pos(); (TokenType::NotEquals, "!=".to_string()) }
                    else { (TokenType::Bang, "!".to_string()) }
                }
                '<' => {
                    if self.peek_char() == Some(&'=') { self.read_char_and_advance_pos(); (TokenType::LTE, "<=".to_string()) }
                    else { (TokenType::LT, "<".to_string()) }
                }
                '>' => {
                    if self.peek_char() == Some(&'=') { self.read_char_and_advance_pos(); (TokenType::GTE, ">=".to_string()) }
                    else { (TokenType::GT, ">".to_string()) }
                }
                '&' => {
                    if self.peek_char() == Some(&'&') { self.read_char_and_advance_pos(); (TokenType::LogicalAnd, "&&".to_string()) }
                    else { (TokenType::BitwiseAnd, "&".to_string()) }
                }
                '^' => (TokenType::Caret, "^".to_string()),

                // --- Single-character operators/delimiters ---
                '+' => (TokenType::Plus, "+".to_string()),
                '-' => (TokenType::Minus, "-".to_string()),
                '*' => (TokenType::Star, "*".to_string()),
                '/' => (TokenType::Slash, "/".to_string()),
                '(' => (TokenType::LParen, "(".to_string()),
                ')' => (TokenType::RParen, ")".to_string()),
                '{' => (TokenType::LBrace, "{".to_string()),
                '}' => (TokenType::RBrace, "}".to_string()),
                '[' => (TokenType::LBracket, "[".to_string()),
                ']' => (TokenType::RBracket, "]".to_string()),
                ';' => (TokenType::Semicolon, ";".to_string()),
                ':' => (TokenType::Colon, ":".to_string()),
                ',' => (TokenType::Comma, ",".to_string()),
                '.' => (TokenType::Dot, ".".to_string()),

                // --- Literals ---
                '"' => {
                    let literal_content = self.read_string_literal_content(initial_span);
                    if self.peek_char() == Some(&'"') {
                         self.read_char_and_advance_pos(); // Consume the closing quote
                         (TokenType::String, literal_content)
                    } else {
                        (TokenType::Illegal, literal_content) // Error already logged by read_string_literal_content
                    }
                }
                ''' => {
                    let literal_content = self.read_char_literal_content(initial_span);
                    if self.peek_char() == Some(&''') {
                        self.read_char_and_advance_pos(); // Consume the closing quote
                        (TokenType::Char, literal_content)
                    } else {
                        (TokenType::Illegal, literal_content) // Error already logged
                    }
                }
                c if c.is_xid_start() || c == '_' => { // is_xid_start for Unicode identifiers
                    let literal = self.read_identifier_or_keyword(c);
                    let token_type = self.keywords.get(literal.as_str()).cloned().unwrap_or(TokenType::Identifier);
                    (token_type, literal)
                }
                c if c.is_digit(10) => {
                    let literal = self.read_number(c);
                    let token_type = if literal.contains('.') { TokenType::Float } else { TokenType::Integer };
                    (token_type, literal)
                }
                _ => { // Fallback for any other unexpected character
                    self.errors.push(LexerError {
                        message: format!("Unexpected character '{}'.", c),
                        span: initial_span,
                    });
                    (TokenType::Illegal, c.to_string())
                },
            };
            Some(Token::new(token_type, literal, Span { end: self.current_char_offset, ..initial_span }))
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

    impl Span {
        pub fn new(start: usize, line: usize, column: usize) -> Self {
            Span { start, end: start, line, column } // End will be updated later
        }
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
        EOF,        // End Of File
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
