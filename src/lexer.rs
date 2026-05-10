//! Zenith Lexical Analyzer (Lexer)
//! Lifetime-correct: borrows source from SourceMap, zero leaks, zero runtime keyword cost.

use crate::source_map::{FileId, BytePos, Span};
use std::collections::HashMap;
use std::sync::LazyLock;
use unicode_xid::UnicodeXID;

pub struct Lexer<'a> {
    file: FileId,
    source: &'a str,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    current_char_offset: usize,
    current_line: usize,
    current_column: usize,
    keywords: &'static phf::Map<&'static str, TokenType>,
    errors: Vec<LexerError>,
    eof_emitted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerError {
    pub message: String,
    pub span: Span,
}

impl<'a> Lexer<'a> {
    pub fn new(file: FileId, source_code: &'a str) -> Self {
        Lexer {
            file,
            source: source_code,
            chars: source_code.chars().peekable(),
            current_char_offset: 0,
            current_line: 1,
            current_column: 1,
            keywords: &KEYWORDS,
            errors: Vec::new(),
            eof_emitted: false,
        }
    }

    // Perfect hash map for keywords. Zero runtime cost.
    static KEYWORDS: phf::Map<&'static str, TokenType> = phf::phf_map! {
        "quantum" => TokenType::KeywordQuantum,
        "nano" => TokenType::KeywordNano,
        "effect" => TokenType::KeywordEffect,
        "handle" => TokenType::KeywordHandle,
        "language" => TokenType::KeywordLanguage,
        "type" => TokenType::KeywordType,
        "kind" => TokenType::KeywordKind,
        "sort" => TokenType::KeywordSort,
        "prop" => TokenType::KeywordProp,
        "linear" => TokenType::KeywordLinear,
        "affine" => TokenType::KeywordAffine,
        "unsafe" => TokenType::KeywordUnsafe,
        "remember" => TokenType::KeywordRemember,
        "recall" => TokenType::KeywordRecall,
        "learn" => TokenType::KeywordLearn,
        "wisdom" => TokenType::KeywordWisdom,
        "zamani" => TokenType::KeywordZamani,
        "sasa" => TokenType::KeywordSasa,
        "ancestral" => TokenType::KeywordAncestral,
        "consensus" => TokenType::KeywordConsensus,
        "observe" => TokenType::KeywordObserve,
        "living_doc" => TokenType::KeywordLivingDoc,
        "temporal_learn" => TokenType::KeywordTemporalLearn,
        "fn" => TokenType::KeywordFn,
        "let" => TokenType::KeywordLet,
        "if" => TokenType::KeywordIf,
        "else" => TokenType::KeywordElse,
        "return" => TokenType::KeywordReturn,
        "true" => TokenType::KeywordTrue,
        "false" => TokenType::KeywordFalse,
        "mts" => TokenType::KeywordMts,
    };

    fn make_span(&self, start_offset: usize, start_line: usize, start_column: usize) -> Span {
        Span {
            file: self.file,
            start: BytePos(start_offset as u32),
            end: BytePos(self.current_char_offset as u32),
            line: start_line,
            column: start_column,
        }
    }

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

    fn peek_char_n(&mut self, n: usize) -> Option<char> {
        self.chars.clone().nth(n - 1).copied()
    }

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
        if self.peek_char() == Some(&'/') && self.peek_char_n(2) == Some('/') {
            let start_span = self.make_span(self.current_char_offset, self.current_line, self.current_column);
            self.read_char_and_advance_pos(); // /
            self.read_char_and_advance_pos(); // /
            while let Some(&c) = self.peek_char() {
                if c == '\n' {
                    self.read_char_and_advance_pos();
                    return;
                }
                self.read_char_and_advance_pos();
            }
        } else if self.peek_char() == Some(&'/') && self.peek_char_n(2) == Some('*') {
            let start_span = self.make_span(self.current_char_offset, self.current_line, self.current_column);
            self.read_char_and_advance_pos(); // /
            self.read_char_and_advance_pos(); // *
            loop {
                match self.read_char_and_advance_pos() {
                    Some('*') if self.peek_char() == Some(&'/') => {
                        self.read_char_and_advance_pos();
                        return;
                    }
                    Some(_) => {}
                    None => {
                        self.errors.push(LexerError {
                            message: "Unterminated multi-line comment.".to_string(),
                            span: self.make_span(start_span.start.0 as usize, start_span.line, start_span.column),
                        });
                        return;
                    }
                }
            }
        }
    }

    fn read_identifier_or_keyword(&mut self, first_char: char) -> String {
        let mut ident = String::from(first_char);
        while let Some(&c) = self.peek_char() {
            if UnicodeXID::is_xid_continue(c) || c == '_' {
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
            } else if c == '.' &&!has_decimal && self.peek_char_n(2).map_or(false, |n| n.is_digit(10)) {
                num.push(self.read_char_and_advance_pos().unwrap());
                has_decimal = true;
            } else {
                break;
            }
        }
        num
    }

    fn read_string_literal_content(&mut self, start_span: Span) -> String {
        let mut literal_content = String::new();
        while let Some(&c) = self.peek_char() {
            if c == '"' {
                break;
            }
            if c == '\n' {
                self.errors.push(LexerError {
                    message: "Unterminated string literal.".to_string(),
                    span: start_span,
                });
                break;
            }
            if c == '\\' {
                self.read_char_and_advance_pos();
                match self.read_char_and_advance_pos() {
                    Some('n') => literal_content.push('\n'),
                    Some('t') => literal_content.push('\t'),
                    Some('r') => literal_content.push('\r'),
                    Some('\\') => literal_content.push('\\'),
                    Some('"') => literal_content.push('"'),
                    Some('\'') => literal_content.push('\''),
                    Some('0') => literal_content.push('\0'),
                    Some('u') => {
                        self.errors.push(LexerError {
                            message: "Unicode escape \\u{XXXX} not implemented yet.".to_string(),
                            span: start_span,
                        });
                    }
                    Some(other) => {
                        self.errors.push(LexerError {
                            message: format!("Invalid escape sequence '\\{}'.", other),
                            span: start_span,
                        });
                    }
                    None => break,
                }
            } else {
                literal_content.push(self.read_char_and_advance_pos().unwrap());
            }
        }
        literal_content
    }

    fn read_char_literal_content(&mut self, start_span: Span) -> String {
        let mut literal_content = String::new();
        if let Some(&c) = self.peek_char() {
            if c == '\\' {
                self.read_char_and_advance_pos();
                if let Some(escaped_char) = self.read_char_and_advance_pos() {
                    match escaped_char {
                        'n' => literal_content.push('\n'),
                        't' => literal_content.push('\t'),
                        'r' => literal_content.push('\r'),
                        '\\' => literal_content.push('\\'),
                        '"' => literal_content.push('"'),
                        '\'' => literal_content.push('\''),
                        other => {
                            self.errors.push(LexerError {
                                message: format!("Invalid escape sequence '\\{}'.", other),
                                span: start_span,
                            });
                        }
                    }
                }
            } else if c!= '\'' {
                literal_content.push(self.read_char_and_advance_pos().unwrap());
            }
        }
        if literal_content.len()!= 1 {
            self.errors.push(LexerError {
                message: "Character literal must contain exactly one character.".to_string(),
                span: start_span,
            });
        }
        literal_content
    }

    fn handle_quantum_literal(&mut self, start_span: Span) -> Option<Token> {
        self.read_char_and_advance_pos(); // consume space after '|'
        let state_start = self.current_char_offset;
        while let Some(&c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' || c == '+' || c == '-' {
                self.read_char_and_advance_pos();
            } else {
                break;
            }
        }
        let state_str = &self.source[state_start..self.current_char_offset];
        if self.peek_char() == Some(&'⟩') {
            self.read_char_and_advance_pos();
            Some(Token::new(
                TokenType::QuantumLiteral,
                format!("|{}⟩", state_str),
                self.make_span(start_span.start.0 as usize, start_span.line, start_span.column),
            ))
        } else {
            self.errors.push(LexerError {
                message: "Malformed quantum Dirac literal: expected '⟩'.".to_string(),
                span: self.make_span(start_span.start.0 as usize, start_span.line, start_span.column),
            });
            None
        }
    }

    fn handle_nano_annotation(&mut self, start_span: Span) -> Option<Token> {
        let anno_start = self.current_char_offset;
        while let Some(&c) = self.peek_char() {
            if c.is_alphabetic() {
                self.read_char_and_advance_pos();
            } else {
                break;
            }
        }
        let annotation = &self.source[anno_start..self.current_char_offset];
        if (annotation == "atom" || annotation == "molecule") && self.peek_char() == Some(&'(') {
            self.read_char_and_advance_pos(); // '('
            let mut nesting = 1;
            while nesting > 0 {
                match self.read_char_and_advance_pos() {
                    Some('(') => nesting += 1, // fixed: was ')'
                    Some(')') => nesting -= 1,
                    Some(_) => {}
                    None => {
                        self.errors.push(LexerError {
                            message: format!("Unterminated nano annotation '{}'.", annotation),
                            span: self.make_span(start_span.start.0 as usize, start_span.line, start_span.column),
                        });
                        return None;
                    }
                }
            }
            Some(Token::new(
                TokenType::NanoAnnotation,
                self.source[start_span.start.0 as usize..self.current_char_offset].to_string(),
                self.make_span(start_span.start.0 as usize, start_span.line, start_span.column),
            ))
        } else {
            self.errors.push(LexerError {
                message: format!("Malformed nano annotation '@{}'.", annotation),
                span: self.make_span(start_span.start.0 as usize, start_span.line, start_span.column),
            });
            None
        }
    }

    fn handle_mts_literal(&mut self, start_span: Span) -> Option<Token> {
        self.read_char_and_advance_pos(); // t
        self.read_char_and_advance_pos(); // s
        self.read_char_and_advance_pos(); // [
        let num_start = self.current_char_offset;
        while let Some(&c) = self.peek_char() {
            if c.is_digit(10) {
                self.read_char_and_advance_pos();
            } else {
                break;
            }
        }
        if self.peek_char() == Some(&']') {
            self.read_char_and_advance_pos();
            Some(Token::new(
                TokenType::MTSLiteral,
                self.source[start_span.start.0 as usize..self.current_char_offset].to_string(),
                self.make_span(start_span.start.0 as usize, start_span.line, start_span.column),
            ))
        } else {
            self.errors.push(LexerError {
                message: "Malformed MTS literal: expected ']'.".to_string(),
                span: self.make_span(start_span.start.0 as usize, start_span.line, start_span.column),
            });
            None
        }
    }

    fn handle_directive(&mut self, start_span: Span) -> Option<Token> {
        let name_start = self.current_char_offset;
        while let Some(&c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' {
                self.read_char_and_advance_pos();
            } else {
                break;
            }
        }
        let name = &self.source[name_start..self.current_char_offset];
        Some(Token::new(
            TokenType::Directive,
            format!("#{}", name),
            self.make_span(start_span.start.0 as usize, start_span.line, start_span.column),
        ))
    }

    pub fn get_errors(&self) -> &[LexerError] {
        &self.errors
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof_emitted {
            return None;
        }

        self.skip_whitespace_and_comments();

        let start_offset = self.current_char_offset;
        let start_line = self.current_line;
        let start_column = self.current_column;

        let c = match self.read_char_and_advance_pos() {
            Some(ch) => ch,
            None => {
                self.eof_emitted = true;
                return Some(Token::new(
                    TokenType::EOF,
                    "".to_string(),
                    Span {
                        file: self.file,
                        start: BytePos(start_offset as u32),
                        end: BytePos(start_offset as u32),
                        line: start_line,
                        column: start_column,
                    },
                ));
            }
        };

        let initial_span = self.make_span(start_offset, start_line, start_column);

        if c == '#' {
            return self.handle_directive(initial_span);
        }
        if c == '@' {
            return self.handle_nano_annotation(initial_span);
        }
        if c == 'm'
            && self.peek_char() == Some(&'t')
            && self.peek_char_n(2) == Some('s')
            && self.peek_char_n(3) == Some('[')
        {
            return self.handle_mts_literal(initial_span);
        }
        if c == '|' {
            if self.peek_char() == Some(&' ') {
                return self.handle_quantum_literal(initial_span);
            } else if self.peek_char() == Some(&'|') {
                self.read_char_and_advance_pos();
                return Some(Token::new(
                    TokenType::LogicalOr,
                    "||".to_string(),
                    self.make_span(start_offset, start_line, start_column),
                ));
            } else {
                return Some(Token::new(
                    TokenType::Pipe,
                    "|".to_string(),
                    initial_span,
                ));
            }
        }

        let (token_type, literal) = match c {
            '=' => {
                if self.peek_char() == Some(&'=') {
                    self.read_char_and_advance_pos();
                    (TokenType::Equals, "==".to_string())
                } else {
                    (TokenType::Assign, "=".to_string())
                }
            }
            '!' => {
                if self.peek_char() == Some(&'=') {
                    self.read_char_and_advance_pos();
                    (TokenType::NotEquals, "!=".to_string())
                } else {
                    (TokenType::Bang, "!".to_string())
                }
            }
            '<' => {
                if self.peek_char() == Some(&'=') {
                    self.read_char_and_advance_pos();
                    (TokenType::LTE, "<=".to_string())
                } else {
                    (TokenType::LT, "<".to_string())
                }
            }
            '>' => {
                if self.peek_char() == Some(&'=') {
                    self.read_char_and_advance_pos();
                    (TokenType::GTE, ">=".to_string())
                } else {
                    (TokenType::GT, ">".to_string())
                }
            }
            '&' => {
                if self.peek_char() == Some(&'&') {
                    self.read_char_and_advance_pos();
                    (TokenType::LogicalAnd, "&&".to_string())
                } else {
                    (TokenType::BitwiseAnd, "&".to_string())
                }
            }
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
            '^' => (TokenType::Caret, "^".to_string()),
            '"' => {
                let content = self.read_string_literal_content(initial_span);
                if self.peek_char() == Some(&'"') {
                    self.read_char_and_advance_pos();
                    (TokenType::String, content)
                } else {
                    (TokenType::Illegal, content)
                }
            }
            '\'' => {
                let content = self.read_char_literal_content(initial_span);
                if self.peek_char() == Some(&'\'') {
                    self.read_char_and_advance_pos();
                    (TokenType::Char, content)
                } else {
                    (TokenType::Illegal, content)
                }
            }
            c if UnicodeXID::is_xid_start(c) || c == '_' => {
                let ident = self.read_identifier_or_keyword(c);
                let tt = self.keywords.get(ident.as_str())
                   .cloned()
                   .unwrap_or(TokenType::Identifier);
                (tt, ident)
            }
            c if c.is_digit(10) => {
                let num = self.read_number(c);
                let tt = if num.contains('.') {
                    TokenType::Float
                } else {
                    TokenType::Integer
                };
                (tt, num)
            }
            _ => {
                self.errors.push(LexerError {
                    message: format!("Unexpected character '{}'.", c),
                    span: self.make_span(start_offset, start_line, start_column),
                });
                (TokenType::Illegal, c.to_string())
            }
        };

        Some(Token::new(
            token_type,
            literal,
            self.make_span(start_offset, start_line, start_column),
        ))
    }
}

// --- Token & TokenType Definitions ---
pub mod tokens {
    use crate::source_map::{FileId, BytePos};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Span {
        pub file: FileId,
        pub start: BytePos,
        pub end: BytePos,
        pub line: usize,
        pub column: usize,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum TokenType {
        Assign, Plus, Minus, Star, Slash,
        LParen, RParen, LBrace, RBrace, LBracket, RBracket,
        Semicolon, Colon, Comma, Dot, Pipe, Caret,
        Equals, NotEquals, LT, GT, LTE, GTE, Bang,
        BitwiseAnd, LogicalAnd, LogicalOr,
        Identifier, Integer, Float, String, Char,
        QuantumLiteral, NanoAnnotation, MTSLiteral,
        KeywordFn, KeywordLet, KeywordIf, KeywordElse, KeywordReturn,
        KeywordTrue, KeywordFalse,
        KeywordQuantum, KeywordNano, KeywordEffect, KeywordHandle,
        KeywordLanguage, KeywordType, KeywordKind, KeywordSort, KeywordProp,
        KeywordLinear, KeywordAffine, KeywordUnsafe,
        KeywordRemember, KeywordRecall, KeywordLearn, KeywordWisdom,
        KeywordZamani, KeywordSasa, KeywordAncestral, KeywordConsensus,
        KeywordObserve, KeywordLivingDoc, KeywordTemporalLearn,
        KeywordMts,
        Directive, Illegal, EOF,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Token {
        pub token_type: TokenType,
        pub literal: String,
        pub span: Span,
    }

    impl Token {
        pub fn new(token_type: TokenType, literal: impl Into<String>, span: Span) -> Self {
            Token {
                token_type,
                literal: literal.into(),
                span,
            }
        }
    }
}