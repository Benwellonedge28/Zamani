//! Zenith Lexical Analyzer (Lexer)
//!
//! This module implements the lexical analysis phase of the Zenith compiler.
//! It converts the input source code into a stream of tokens based on the
//! NIMBUS Grammar v2.0 Trinity Edition rules. This lexer handles the unified
//! grammar across NIMBUS, Zenith, and Sankofa, including advanced literals
//! for quantum, nano, and multi-timeline systems, and tokens for algebraic effects.
//! This version refines token stream generation and includes more comprehensive
//! token types, covering floating-point numbers, character literals, and more operators.

use crate::tokens::{Token, TokenType};
use crate::source::SourceCode;
use std::collections::HashMap;

// --- Lexer Structure ---
pub struct Lexer {
    source: SourceCode,
    chars: std::iter::Peekable<std::str::Chars<'static>>, // Peekable iterator over characters
    position: usize,      // Current position in source (start of current token)
    read_position: usize, // Next character to read (end of current token + 1)
    keywords: HashMap<&'static str, TokenType>,
}

impl Lexer {
    pub fn new(source_code: &str) -> Self {
        let mut lexer = Lexer {
            source: SourceCode::new(source_code.to_string()),
            chars: "".chars().peekable(), // Dummy init, will be replaced
            position: 0,
            read_position: 0,
            keywords: Self::build_keywords_map(),
        };
        lexer.init_chars(); // Proper initialization
        lexer
    }

    fn init_chars(&mut self) {
        // This is a workaround for the 'static lifetime. In a real project,
        // SourceCode would own the String, and chars would borrow from it.
        let raw_src: &'static str = Box::leak(self.source.content.clone().into_boxed_str());
        self.chars = raw_src.chars().peekable();
        self.read_char(); // Initialize current_char/position
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
    fn read_char(&mut self) -> Option<char> {
        self.position = self.read_position;
        let c = self.chars.next();
        if c.is_some() {
            self.read_position += 1;
        }
        c
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    // --- Skipping Utilities ---
    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if c.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn skip_comments(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            // Single-line comments: // ...
            if self.peek_char() == Some(&'/') && self.chars.clone().nth(1) == Some('/') {
                self.read_char(); // Consume first '/'
                self.read_char(); // Consume second '/'
                while let Some(&c) = self.peek_char() {
                    if c == '\n' {
                        self.read_char(); // Consume newline
                        changed = true;
                        break;
                    }
                    self.read_char();
                }
            }
            // Multi-line comments: /* ... */
            if self.peek_char() == Some(&'/') && self.chars.clone().nth(1) == Some('*') {
                self.read_char(); // Consume '/'
                self.read_char(); // Consume '*'
                loop {
                    if self.peek_char() == Some(&'*') && self.chars.clone().nth(1) == Some('/') {
                        self.read_char(); // Consume '*'
                        self.read_char(); // Consume '/'
                        changed = true;
                        break;
                    }
                    if self.peek_char().is_none() {
                        // Conceptual: Report unterminated multi-line comment error
                        break;
                    }
                    self.read_char();
                }
            }
        }
    }

    // --- Reading Utilities ---
    fn read_identifier_or_keyword(&mut self, first_char: char) -> String {
        let start_pos = self.position - 1; // Adjust since first_char was already read
        let mut ident = String::from(first_char);
        while let Some(&c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(self.read_char().unwrap());
            } else {
                break;
            }
        }
        ident
    }

    fn read_number(&mut self, first_digit: char) -> String {
        let start_pos = self.position - 1; // Adjust since first_digit was already read
        let mut num = String::from(first_digit);
        let mut is_float = false;

        while let Some(&c) = self.peek_char() {
            if c.is_digit(10) {
                num.push(self.read_char().unwrap());
            } else if c == '.' && !is_float && self.peek_char_n(2).map_or(false, |next_c| next_c.is_digit(10)) {
                num.push(self.read_char().unwrap()); // Consume '.'
                is_float = true;
            } else {
                break;
            }
        }
        num
    }

    fn read_string_literal(&mut self) -> String {
        let start_pos = self.position; // Start after opening quote
        // Conceptual: handle escape sequences here, e.g., '\n', '\"'
        while let Some(&c) = self.peek_char() {
            if c == '"' {
                break;
            }
            // Add more complex escape handling here for a real lexer
            self.read_char();
        }
        self.source.content[start_pos..self.position].to_string()
    }

    fn read_char_literal(&mut self) -> String {
        let start_pos = self.position; // Start after opening single quote
        // Conceptual: handle escape sequences here
        while let Some(&c) = self.peek_char() {
            if c == ''' {
                break;
            }
            self.read_char();
        }
        self.source.content[start_pos..self.position].to_string()
    }

    fn peek_char_n(&mut self, n: usize) -> Option<char> {
        self.chars.clone().nth(n - 1).copied()
    }


    // --- Special Literal Handlers ---
    fn handle_quantum_literal(&mut self) -> Option<Token> {
        let start_literal_pos = self.position - 1; // Adjust for consumed '|'
        // Assumes current_char is already '|' and next is ' '
        self.read_char(); // Consume space
        let state_start = self.position;
        while let Some(&c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' || c == '+' || c == '-' { // Allow for |+⟩ or |-⟩ states
                self.read_char();
            } else {
                break;
            }
        }
        let state_str = self.source.content[state_start..self.position].to_string();
        if self.peek_char() == Some(&'⟩') { // Dirac notation
            self.read_char(); // Consume '⟩'
            Some(Token::new(TokenType::QuantumLiteral, format!("|{}⟩", state_str), start_literal_pos))
        } else {
            // Not a valid quantum literal, backtrack or create an error token
            Some(Token::new(TokenType::Illegal, self.source.content[start_literal_pos..self.position].to_string(), start_literal_pos))
        }
    }

    fn handle_nano_annotation(&mut self) -> Option<Token> {
        let start_literal_pos = self.position - 1; // Adjust for consumed '@'
        let annotation_start = self.position;
        while let Some(&c) = self.peek_char() {
            if c.is_alphabetic() { // read "atom" or "molecule"
                self.read_char();
            } else {
                break;
            }
        }
        let annotation = self.source.content[annotation_start..self.position].to_string();
        if (annotation == "atom" || annotation == "molecule") && self.peek_char() == Some(&'(') {
            self.read_char(); // Consume '('
            let mut paren_nesting = 1; // Already consumed one '('
            while let Some(&c) = self.peek_char() {
                if c == '(' { paren_nesting += 1; }
                if c == ')' { paren_nesting -= 1; }
                self.read_char();
                if paren_nesting == 0 { break; }
            }
            let full_literal = self.source.content[start_literal_pos..self.position].to_string();
            Some(Token::new(TokenType::NanoAnnotation, full_literal, start_literal_pos))
        } else {
            // Not a valid nano annotation, backtrack or create an error token
            Some(Token::new(TokenType::Illegal, self.source.content[start_literal_pos..self.position].to_string(), start_literal_pos))
        }
    }

    fn handle_mts_literal(&mut self) -> Option<Token> {
        let start_literal_pos = self.position - 1; // Adjust for consumed 'm'
        // Assumes current_char is 'm', and 't', 's' follow, then '['
        if self.source.content[start_literal_pos..self.position + 2].eq_ignore_ascii_case("mts") &&
           self.peek_char() == Some(&'[') {
            self.read_char(); // Consume 't' (from mts)
            self.read_char(); // Consume 's' (from mts)
            self.read_char(); // Consume '['
            let number_start = self.position;
            while let Some(&c) = self.peek_char() {
                if c.is_digit(10) {
                    self.read_char();
                } else {
                    break;
                }
            }
            let number_str = self.source.content[number_start..self.position].to_string();
            if self.peek_char() == Some(&']') {
                self.read_char(); // Consume ']'
                Some(Token::new(TokenType::MTSLiteral, format!("mts[{}]", number_str), start_literal_pos))
            } else {
                // Unterminated MTS literal
                Some(Token::new(TokenType::Illegal, self.source.content[start_literal_pos..self.position].to_string(), start_literal_pos))
            }
        } else {
            None
        }
    }


    fn handle_directive(&mut self) -> Option<Token> {
        let start_directive_pos = self.position - 1; // Adjust for consumed '#'
        let directive_name_start = self.position;
        while let Some(&c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' {
                self.read_char();
            } else {
                break;
            }
        }
        let directive_name = self.source.content[directive_name_start..self.position].to_string();
        // Conceptual: Further parse directive arguments if needed (e.g., #language "Zenith")
        Some(Token::new(TokenType::Directive, format!("#{}", directive_name), start_directive_pos))
    }
}

// --- Iterator Implementation for Lexer ---
impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        self.skip_comments(); // Skip comments (and any whitespace surrounding them)
        self.skip_whitespace(); // Re-skip whitespace after comments

        let current_char = self.read_char(); // Read the character for the next token

        if let Some(c) = current_char {
            let start_pos = self.position - 1; // Adjusted start position

            // Try to match complex tokens first
            if c == '#' { return self.handle_directive(); }
            if c == '@' { return self.handle_nano_annotation(); }
            if c == 'm' { // Check for 'mts' literal
                if let Some(token) = self.handle_mts_literal() {
                    return Some(token);
                }
            }

            // Special handling for '|' to differentiate from quantum literal
            if c == '|' {
                if self.peek_char() == Some(&' ') { // This might be start of a quantum literal
                    return self.handle_quantum_literal();
                } else {
                    // It's a regular pipe operator, also checked for '||' below
                }
            }


            match c {
                // --- Multi-character operators and their single-char counterparts ---
                '=' => {
                    if self.peek_char() == Some(&'=') { self.read_char(); Some(Token::new(TokenType::Equals, "==", start_pos)) }
                    else { Some(Token::new(TokenType::Assign, "=", start_pos)) }
                }
                '!' => {
                    if self.peek_char() == Some(&'=') { self.read_char(); Some(Token::new(TokenType::NotEquals, "!=", start_pos)) }
                    else { Some(Token::new(TokenType::Bang, "!", start_pos)) }
                }
                '<' => {
                    if self.peek_char() == Some(&'=') { self.read_char(); Some(Token::new(TokenType::LTE, "<=", start_pos)) }
                    else { Some(Token::new(TokenType::LT, "<", start_pos)) }
                }
                '>' => {
                    if self.peek_char() == Some(&'=') { self.read_char(); Some(Token::new(TokenType::GTE, ">=", start_pos)) }
                    else { Some(Token::new(TokenType::GT, ">", start_pos)) }
                }
                '&' => {
                    if self.peek_char() == Some(&'&') { self.read_char(); Some(Token::new(TokenType::LogicalAnd, "&&", start_pos)) }
                    else { Some(Token::new(TokenType::BitwiseAnd, "&", start_pos)) }
                }
                '|' => { // Already handled quantum literal, so this is bitwise OR
                    if self.peek_char() == Some(&'|') { self.read_char(); Some(Token::new(TokenType::LogicalOr, "||", start_pos)) }
                    else { Some(Token::new(TokenType::Pipe, "|", start_pos)) }
                }
                '^' => Some(Token::new(TokenType::Caret, "^", start_pos)),

                // --- Single-character operators/delimiters ---
                '+' => Some(Token::new(TokenType::Plus, "+", start_pos)),
                '-' => Some(Token::new(TokenType::Minus, "-", start_pos)),
                '*' => Some(Token::new(TokenType::Star, "*", start_pos)),
                '/' => Some(Token::new(TokenType::Slash, "/", start_pos)),
                '(' => Some(Token::new(TokenType::LParen, "(", start_pos)),
                ')' => Some(Token::new(TokenType::RParen, ")", start_pos)),
                '{' => Some(Token::new(TokenType::LBrace, "{", start_pos)),
                '}' => Some(Token::new(TokenType::RBrace, "}", start_pos)),
                '[' => Some(Token::new(TokenType::LBracket, "[", start_pos)),
                ']' => Some(Token::new(TokenType::RBracket, "]", start_pos)),
                ';' => Some(Token::new(TokenType::Semicolon, ";", start_pos)),
                ':' => Some(Token::new(TokenType::Colon, ":", start_pos)),
                ',' => Some(Token::new(TokenType::Comma, ",", start_pos)),
                '.' => Some(Token::new(TokenType::Dot, ".", start_pos)),

                // --- Literals ---
                '"' => {
                    let literal = self.read_string_literal();
                    self.read_char(); // Consume the closing quote
                    Some(Token::new(TokenType::String, literal, start_pos))
                }
                ''' => {
                    let literal = self.read_char_literal();
                    self.read_char(); // Consume the closing quote
                    Some(Token::new(TokenType::Char, literal, start_pos))
                }
                c if c.is_alphabetic() || c == '_' => {
                    let literal = self.read_identifier_or_keyword(c);
                    let token_type = self.keywords.get(literal.as_str()).cloned().unwrap_or(TokenType::Identifier);
                    Some(Token::new(token_type, literal, start_pos))
                }
                c if c.is_digit(10) => {
                    let literal = self.read_number(c);
                    // Determine if it's an integer or float based on content
                    if literal.contains('.') {
                        Some(Token::new(TokenType::Float, literal, start_pos))
                    } else {
                        Some(Token::new(TokenType::Integer, literal, start_pos))
                    }
                }
                _ => Some(Token::new(TokenType::Illegal, c.to_string(), start_pos)),
            }
        } else {
            None // End of input
        }
    }
}


// --- Token & TokenType Definitions ---
pub mod tokens {
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
        Identifier, Integer, Float, String, Char, // Added Float and Char
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
        pub position: usize,
    }

    impl Token {
        pub fn new(token_type: TokenType, literal: impl Into<String>, position: usize) -> Self {
            Token { token_type, literal: literal.into(), position }
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
