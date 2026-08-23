//! OpenQASM 3 lexical analyzer.
//!
//! This module converts OpenQASM source text into a deterministic stream of
//! lexical tokens. It intentionally performs NO semantic analysis and does
//! NOT construct Quantum IR.
//!
//! Architectural boundary:
//!
//! ```text
//! OpenQASM source
//!      │
//!      ▼
//!    lexer.rs
//!      │
//!      ▼
//! Token stream
//!      │
//!      ▼
//!   parser.rs
//!      │
//!      ▼
//! OpenQASM AST
//! ```
//!
//! This module must NOT depend on:
//!
//! - `crate::quantum::ir`;
//! - `GateKind`;
//! - `QuantumCircuit`;
//! - OpenQASM validation;
//! - lowering;
//! - exporters;
//! - other frontend formats;
//! - filesystem access;
//! - network access;
//! - code execution.
//!
//! The lexer is an untrusted-input boundary. All externally supplied source
//! is treated as hostile input and is processed under explicit resource
//! limits.
//!
//! Rust compatibility: Rust 1.97.1.
//! No nightly features.
//!
//! OpenQASM lexical reference:
//! <https://openqasm.com/versions/3.1/grammar/index.html>

use std::fmt;

// =============================================================================
// Public lexical token model
// =============================================================================

/// A lexical token produced by [`OpenQasmLexer`].
///
/// Token text is represented as a byte range into the original source rather
/// than as an owned `String`. This avoids unnecessary allocations and ensures
/// that the lexer remains deterministic and inexpensive.
///
/// The source supplied to the lexer must remain alive while the returned
/// tokens are used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'src> {
    kind: TokenKind,
    span: Span,
    lexeme: &'src str,
}

impl<'src> Token<'src> {
    /// Creates a token.
    #[must_use]
    pub const fn new(kind: TokenKind, span: Span, lexeme: &'src str) -> Self {
        Self {
            kind,
            span,
            lexeme,
        }
    }

    /// Returns the token kind.
    #[must_use]
    pub const fn kind(self) -> TokenKind {
        self.kind
    }

    /// Returns the source span.
    #[must_use]
    pub const fn span(self) -> Span {
        self.span
    }

    /// Returns the exact source spelling.
    #[must_use]
    pub const fn lexeme(self) -> &'src str {
        self.lexeme
    }

    /// Returns whether this is EOF.
    #[must_use]
    pub const fn is_eof(self) -> bool {
        matches!(self.kind, TokenKind::Eof)
    }
}

/// A half-open byte range in the source.
///
/// `start` is inclusive and `end` is exclusive.
///
/// Byte offsets are used as the canonical representation because they are
/// stable, efficient, and directly compatible with Rust string slicing.
///
/// Line and column information can be derived by the shared frontend source
/// infrastructure without making the lexer depend on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    /// Inclusive start byte offset.
    pub start: usize,

    /// Exclusive end byte offset.
    pub end: usize,
}

impl Span {
    /// Creates a new span.
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Returns the byte length of this span.
    #[must_use]
    pub const fn len(self) -> usize {
        self.end.saturating_sub(self.start)
    }

    /// Returns whether the span is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }
}

// =============================================================================
// Token kinds
// =============================================================================

/// All lexical token categories recognized by the OpenQASM frontend.
///
/// Keywords are represented individually because the parser needs to
/// distinguish language constructs without repeatedly comparing source text.
///
/// Identifier-like values remain identifiers; semantic classification belongs
/// to the parser/validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // -------------------------------------------------------------------------
    // Special tokens
    // -------------------------------------------------------------------------

    /// End of input.
    Eof,

    /// An ordinary OpenQASM identifier.
    Identifier,

    /// A hardware-qubit identifier such as `$0` or `$127`.
    HardwareQubit,

    /// An integer literal.
    IntegerLiteral,

    /// A floating-point literal.
    FloatLiteral,

    /// An imaginary literal.
    ImaginaryLiteral,

    /// A duration literal such as `10ns`.
    DurationLiteral,

    /// A string literal.
    StringLiteral,

    /// A compiler annotation beginning with `@`.
    Annotation,

    /// A pragma beginning with `#pragma`.
    Pragma,

    // -------------------------------------------------------------------------
    // Version / include
    // -------------------------------------------------------------------------

    KwOpenQasm,
    KwInclude,
    KwDefcalGrammar,

    // -------------------------------------------------------------------------
    // Quantum declarations
    // -------------------------------------------------------------------------

    KwQubit,
    KwQreg,
    KwBit,
    KwBool,
    KwInt,
    KwUInt,
    KwFloat,
    KwAngle,
    KwComplex,
    KwDuration,
    KwStretch,

    // -------------------------------------------------------------------------
    // Classical declarations / operations
    // -------------------------------------------------------------------------

    KwConst,
    KwLet,
    KwReadonly,
    KwInput,
    KwOutput,

    // -------------------------------------------------------------------------
    // Gate / subroutine definitions
    // -------------------------------------------------------------------------

    KwGate,
    KwDef,
    KwExtern,
    KwReturn,

    // -------------------------------------------------------------------------
    // Quantum operations
    // -------------------------------------------------------------------------

    KwMeasure,
    KwReset,
    KwBarrier,

    // -------------------------------------------------------------------------
    // Control flow
    // -------------------------------------------------------------------------

    KwIf,
    KwElse,
    KwFor,
    KwWhile,

    // -------------------------------------------------------------------------
    // Timing / scheduling
    // -------------------------------------------------------------------------

    KwDelay,
    KwBox,

    // -------------------------------------------------------------------------
    // Calibration
    // -------------------------------------------------------------------------

    KwDefcal,
    KwCal,
    KwCalGrammar,

    // -------------------------------------------------------------------------
    // Literals / built-ins
    // -------------------------------------------------------------------------

    KwTrue,
    KwFalse,

    KwDurationof,

    // -------------------------------------------------------------------------
    // Mathematical / language constants
    // -------------------------------------------------------------------------

    KwPi,
    KwEuler,

    // -------------------------------------------------------------------------
    // Delimiters
    // -------------------------------------------------------------------------

    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,

    Comma,
    Semicolon,
    Colon,
    Dot,

    // -------------------------------------------------------------------------
    // Operators
    // -------------------------------------------------------------------------

    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    Power,

    Equal,
    EqualEqual,
    NotEqual,

    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    LogicalAnd,
    LogicalOr,
    LogicalNot,

    BitAnd,
    BitOr,
    BitXor,
    BitNot,

    ShiftLeft,
    ShiftRight,

    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,

    Increment,
    Decrement,

    Arrow,

    // -------------------------------------------------------------------------
    // Miscellaneous punctuation
    // -------------------------------------------------------------------------

    At,
    Hash,
}

// =============================================================================
// Keyword classification
// =============================================================================

impl TokenKind {
    /// Classifies an identifier spelling as a keyword where applicable.
    ///
    /// The comparison is deliberately ASCII case-sensitive because OpenQASM
    /// keywords are case-sensitive.
    #[must_use]
    pub fn keyword(lexeme: &str) -> Option<Self> {
        Some(match lexeme {
            "OPENQASM" => Self::KwOpenQasm,
            "include" => Self::KwInclude,
            "defcalgrammar" => Self::KwDefcalGrammar,

            "qubit" => Self::KwQubit,
            "qreg" => Self::KwQreg,
            "bit" => Self::KwBit,
            "bool" => Self::KwBool,
            "int" => Self::KwInt,
            "uint" => Self::KwUInt,
            "float" => Self::KwFloat,
            "angle" => Self::KwAngle,
            "complex" => Self::KwComplex,
            "duration" => Self::KwDuration,
            "stretch" => Self::KwStretch,

            "const" => Self::KwConst,
            "let" => Self::KwLet,
            "readonly" => Self::KwReadonly,
            "input" => Self::KwInput,
            "output" => Self::KwOutput,

            "gate" => Self::KwGate,
            "def" => Self::KwDef,
            "extern" => Self::KwExtern,
            "return" => Self::KwReturn,

            "measure" => Self::KwMeasure,
            "reset" => Self::KwReset,
            "barrier" => Self::KwBarrier,

            "if" => Self::KwIf,
            "else" => Self::KwElse,
            "for" => Self::KwFor,
            "while" => Self::KwWhile,

            "delay" => Self::KwDelay,
            "box" => Self::KwBox,

            "defcal" => Self::KwDefcal,
            "cal" => Self::KwCal,
            "calibration" => Self::KwCalGrammar,

            "true" => Self::KwTrue,
            "false" => Self::KwFalse,

            "durationof" => Self::KwDurationof,

            "pi" => Self::KwPi,
            "euler" => Self::KwEuler,

            _ => return None,
        })
    }

    /// Returns whether this token is a literal token.
    #[must_use]
    pub const fn is_literal(self) -> bool {
        matches!(
            self,
            Self::IntegerLiteral
                | Self::FloatLiteral
                | Self::ImaginaryLiteral
                | Self::DurationLiteral
                | Self::StringLiteral
                | Self::KwTrue
                | Self::KwFalse
        )
    }

    /// Returns whether this token is an identifier-like token.
    #[must_use]
    pub const fn is_identifier_like(self) -> bool {
        matches!(self, Self::Identifier | Self::HardwareQubit)
    }
}

// =============================================================================
// Lexer configuration
// =============================================================================

/// Resource limits protecting the lexer against pathological or malicious
/// input.
///
/// These limits belong to lexical processing. They are deliberately separate
/// from `QuantumIrLimits`, which protect canonical IR resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LexerLimits {
    /// Maximum source length in bytes.
    pub max_source_bytes: usize,

    /// Maximum number of emitted tokens.
    pub max_tokens: usize,

    /// Maximum length of a single lexical item in bytes.
    pub max_lexeme_bytes: usize,

    /// Maximum length of a string literal in bytes.
    pub max_string_bytes: usize,

    /// Maximum length of a comment in bytes.
    pub max_comment_bytes: usize,

    /// Maximum number of Unicode scalar values in one identifier.
    pub max_identifier_scalars: usize,

    /// Maximum number of digits in a numeric literal.
    pub max_numeric_digits: usize,
}

impl Default for LexerLimits {
    fn default() -> Self {
        Self {
            max_source_bytes: 16 * 1024 * 1024,
            max_tokens: 4_000_000,
            max_lexeme_bytes: 1024 * 1024,
            max_string_bytes: 1024 * 1024,
            max_comment_bytes: 1024 * 1024,
            max_identifier_scalars: 16 * 1024,
            max_numeric_digits: 1_000_000,
        }
    }
}

/// Configuration for lexical analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LexerConfig {
    /// Resource limits.
    pub limits: LexerLimits,

    /// Whether comments should be emitted as tokens.
    ///
    /// Production parsing normally leaves this disabled. Keeping the option
    /// here makes the lexer useful for tooling without changing lexical
    /// semantics.
    pub emit_comments: bool,
}

impl Default for LexerConfig {
    fn default() -> Self {
        Self {
            limits: LexerLimits::default(),
            emit_comments: false,
        }
    }
}

// =============================================================================
// Lexer errors
// =============================================================================

/// Result returned by the OpenQASM lexer.
pub type LexerResult<T> = Result<T, LexError>;

/// Stable lexical error classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LexErrorKind {
    /// Input exceeded the maximum source size.
    SourceTooLarge,

    /// Too many tokens were emitted.
    TokenLimitExceeded,

    /// A lexical item exceeded its configured size.
    LexemeTooLarge,

    /// A string exceeded its configured size.
    StringTooLarge,

    /// A comment exceeded its configured size.
    CommentTooLarge,

    /// An identifier exceeded its configured scalar limit.
    IdentifierTooLong,

    /// A numeric literal exceeded its configured digit limit.
    NumericLiteralTooLong,

    /// Unexpected character.
    UnexpectedCharacter,

    /// Invalid UTF-8.
    ///
    /// This variant is retained for API completeness even though Rust `&str`
    /// input is already guaranteed to be valid UTF-8.
    InvalidUtf8,

    /// Unterminated string.
    UnterminatedString,

    /// Invalid escape sequence.
    InvalidEscape,

    /// Unterminated block comment.
    UnterminatedComment,

    /// Invalid hardware-qubit literal.
    InvalidHardwareQubit,

    /// Invalid numeric literal.
    InvalidNumber,

    /// Invalid duration literal.
    InvalidDuration,

    /// Integer conversion overflow.
    NumericOverflow,

    /// Internal source-boundary inconsistency.
    InvalidSourceBoundary,
}

impl LexErrorKind {
    /// Stable machine-readable error code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::SourceTooLarge => "QASM-L001",
            Self::TokenLimitExceeded => "QASM-L002",
            Self::LexemeTooLarge => "QASM-L003",
            Self::StringTooLarge => "QASM-L004",
            Self::CommentTooLarge => "QASM-L005",
            Self::IdentifierTooLong => "QASM-L006",
            Self::NumericLiteralTooLong => "QASM-L007",
            Self::UnexpectedCharacter => "QASM-L008",
            Self::InvalidUtf8 => "QASM-L009",
            Self::UnterminatedString => "QASM-L010",
            Self::InvalidEscape => "QASM-L011",
            Self::UnterminatedComment => "QASM-L012",
            Self::InvalidHardwareQubit => "QASM-L013",
            Self::InvalidNumber => "QASM-L014",
            Self::InvalidDuration => "QASM-L015",
            Self::NumericOverflow => "QASM-L016",
            Self::InvalidSourceBoundary => "QASM-L017",
        }
    }
}

impl fmt::Display for LexErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

/// Structured lexical error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    kind: LexErrorKind,
    span: Span,
    message: String,
}

impl LexError {
    /// Creates a lexical error.
    fn new(kind: LexErrorKind, span: Span, message: impl Into<String>) -> Self {
        Self {
            kind,
            span,
            message: message.into(),
        }
    }

    /// Returns the stable error kind.
    #[must_use]
    pub const fn kind(&self) -> LexErrorKind {
        self.kind
    }

    /// Returns the stable error code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.kind.code()
    }

    /// Returns the source span.
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Returns the human-readable diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at bytes {}..{}: {}",
            self.code(),
            self.span.start,
            self.span.end,
            self.message
        )
    }
}

impl std::error::Error for LexError {}

// =============================================================================
// Lexer
// =============================================================================

/// OpenQASM 3 lexer.
///
/// The lexer borrows the source and therefore performs no source-copying.
/// Returned token lexemes are slices into the original source.
///
/// # Determinism
///
/// Given identical source and configuration, this lexer always produces the
/// same token sequence or the same lexical error.
///
/// # Safety
///
/// No operation performed by this type:
///
/// - executes source code;
/// - opens files;
/// - accesses the network;
/// - invokes external processes;
/// - allocates based on attacker-controlled values without limits;
/// - panics on malformed source by design.
pub struct OpenQasmLexer<'src> {
    source: &'src str,
    config: LexerConfig,

    offset: usize,
    token_count: usize,

    /// Whether the lexer has already emitted EOF.
    finished: bool,
}

impl<'src> OpenQasmLexer<'src> {
    /// Creates a lexer using default production limits.
    pub fn new(source: &'src str) -> LexerResult<Self> {
        Self::with_config(source, LexerConfig::default())
    }

    /// Creates a lexer with explicit configuration.
    pub fn with_config(
        source: &'src str,
        config: LexerConfig,
    ) -> LexerResult<Self> {
        if source.len() > config.limits.max_source_bytes {
            return Err(LexError::new(
                LexErrorKind::SourceTooLarge,
                Span::new(0, source.len()),
                format!(
                    "source contains {} bytes but the configured maximum is {}",
                    source.len(),
                    config.limits.max_source_bytes
                ),
            ));
        }

        Ok(Self {
            source,
            config,
            offset: 0,
            token_count: 0,
            finished: false,
        })
    }

    /// Returns the original source.
    #[must_use]
    pub const fn source(&self) -> &'src str {
        self.source
    }

    /// Returns the current byte offset.
    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Returns the lexer configuration.
    #[must_use]
    pub const fn config(&self) -> LexerConfig {
        self.config
    }

    /// Tokenizes the complete source.
    pub fn tokenize(mut self) -> LexerResult<Vec<Token<'src>>> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            let eof = token.is_eof();

            tokens.push(token);

            if eof {
                break;
            }
        }

        Ok(tokens)
    }

    /// Returns the next token.
    pub fn next_token(&mut self) -> LexerResult<Token<'src>> {
        if self.finished {
            return self.eof_token();
        }

        self.skip_whitespace_and_comments()?;

        if self.offset >= self.source.len() {
            self.finished = true;
            return self.eof_token();
        }

        self.ensure_token_capacity()?;

        let start = self.offset;

        let ch = self.current_char().ok_or_else(|| {
            LexError::new(
                LexErrorKind::InvalidSourceBoundary,
                Span::new(start, start),
                "lexer reached an invalid UTF-8 source boundary",
            )
        })?;

        let kind = match ch {
            // -------------------------------------------------------------
            // Identifiers / keywords
            // -------------------------------------------------------------
            '$' => self.lex_hardware_qubit(start)?,

            c if is_identifier_start(c) => self.lex_identifier(start)?,

            // -------------------------------------------------------------
            // Numeric literals
            // -------------------------------------------------------------
            c if c.is_ascii_digit() => self.lex_number(start)?,

            // -------------------------------------------------------------
            // Strings
            // -------------------------------------------------------------
            '"' => self.lex_string(start)?,

            // -------------------------------------------------------------
            // Annotation
            // -------------------------------------------------------------
            '@' => self.lex_annotation(start)?,

            // -------------------------------------------------------------
            // Operators / punctuation
            // -------------------------------------------------------------
            '(' => {
                self.advance_char();
                TokenKind::LParen
            }

            ')' => {
                self.advance_char();
                TokenKind::RParen
            }

            '[' => {
                self.advance_char();
                TokenKind::LBracket
            }

            ']' => {
                self.advance_char();
                TokenKind::RBracket
            }

            '{' => {
                self.advance_char();
                TokenKind::LBrace
            }

            '}' => {
                self.advance_char();
                TokenKind::RBrace
            }

            ',' => {
                self.advance_char();
                TokenKind::Comma
            }

            ';' => {
                self.advance_char();
                TokenKind::Semicolon
            }

            ':' => {
                self.advance_char();
                TokenKind::Colon
            }

            '.' => self.lex_dot_or_number(start)?,

            '+' => self.lex_plus(start)?,

            '-' => self.lex_minus(start)?,

            '*' => self.lex_star(start)?,

            '/' => self.lex_slash(start)?,

            '%' => self.lex_percent(start)?,

            '^' => {
                self.advance_char();
                TokenKind::BitXor
            }

            '&' => self.lex_ampersand(start)?,

            '|' => self.lex_pipe(start)?,

            '~' => {
                self.advance_char();
                TokenKind::BitNot
            }

            '!' => self.lex_bang(start)?,

            '=' => self.lex_equal(start)?,

            '<' => self.lex_less(start)?,

            '>' => self.lex_greater(start)?,

            '#' => self.lex_hash(start)?,

            // -------------------------------------------------------------
            // Unknown
            // -------------------------------------------------------------
            _ => {
                let end = self.offset + ch.len_utf8();

                return Err(LexError::new(
                    LexErrorKind::UnexpectedCharacter,
                    Span::new(start, end),
                    format!("unexpected character `{ch}`"),
                ));
            }
        };

        self.make_token(start, kind)
    }

    // =========================================================================
    // Whitespace/comments
    // =========================================================================

    fn skip_whitespace_and_comments(&mut self) -> LexerResult<()> {
        loop {
            let Some(ch) = self.current_char() else {
                return Ok(());
            };

            if ch.is_whitespace() {
                self.advance_char();
                continue;
            }

            if ch == '/' && self.peek_char() == Some('/') {
                self.skip_line_comment()?;
                continue;
            }

            if ch == '/' && self.peek_char() == Some('*') {
                self.skip_block_comment()?;
                continue;
            }

            break;
        }

        Ok(())
    }

    fn skip_line_comment(&mut self) -> LexerResult<()> {
        let start = self.offset;

        self.advance_char(); // /
        self.advance_char(); // /

        while let Some(ch) = self.current_char() {
            self.advance_char();

            if ch == '\n' {
                break;
            }

            if self.offset.saturating_sub(start)
                > self.config.limits.max_comment_bytes
            {
                return Err(LexError::new(
                    LexErrorKind::CommentTooLarge,
                    Span::new(start, self.offset),
                    "line comment exceeds configured maximum size",
                ));
            }
        }

        Ok(())
    }

    fn skip_block_comment(&mut self) -> LexerResult<()> {
        let start = self.offset;

        self.advance_char(); // /
        self.advance_char(); // *

        loop {
            let Some(ch) = self.current_char() else {
                return Err(LexError::new(
                    LexErrorKind::UnterminatedComment,
                    Span::new(start, self.offset),
                    "unterminated block comment",
                ));
            };

            if self.offset.saturating_sub(start)
                > self.config.limits.max_comment_bytes
            {
                return Err(LexError::new(
                    LexErrorKind::CommentTooLarge,
                    Span::new(start, self.offset),
                    "block comment exceeds configured maximum size",
                ));
            }

            if ch == '*' && self.peek_char() == Some('/') {
                self.advance_char();
                self.advance_char();
                return Ok(());
            }

            self.advance_char();
        }
    }

    // =========================================================================
    // Identifier handling
    // =========================================================================

    fn lex_identifier(&mut self, start: usize) -> LexerResult<TokenKind> {
        let mut scalar_count = 0usize;

        while let Some(ch) = self.current_char() {
            if !is_identifier_continue(ch) {
                break;
            }

            scalar_count = scalar_count.saturating_add(1);

            if scalar_count > self.config.limits.max_identifier_scalars {
                return Err(LexError::new(
                    LexErrorKind::IdentifierTooLong,
                    Span::new(start, self.offset),
                    "identifier exceeds configured Unicode scalar limit",
                ));
            }

            self.advance_char();
        }

        let end = self.offset;

        self.check_lexeme_size(start, end)?;

        let lexeme = self.source.get(start..end).ok_or_else(|| {
            LexError::new(
                LexErrorKind::InvalidSourceBoundary,
                Span::new(start, end),
                "identifier does not align with UTF-8 boundaries",
            )
        })?;

        Ok(TokenKind::keyword(lexeme).unwrap_or(TokenKind::Identifier))
    }

    fn lex_hardware_qubit(&mut self, start: usize) -> LexerResult<TokenKind> {
        self.advance_char(); // $

        let digit_start = self.offset;

        while let Some(ch) = self.current_char() {
            if !ch.is_ascii_digit() {
                break;
            }

            self.advance_char();
        }

        if self.offset == digit_start {
            return Err(LexError::new(
                LexErrorKind::InvalidHardwareQubit,
                Span::new(start, self.offset),
                "hardware-qubit identifier requires at least one decimal digit",
            ));
        }

        let digits = &self.source[digit_start..self.offset];

        if digits.len() > self.config.limits.max_numeric_digits {
            return Err(LexError::new(
                LexErrorKind::NumericLiteralTooLong,
                Span::new(digit_start, self.offset),
                "hardware-qubit index exceeds configured digit limit",
            ));
        }

        // Reject a leading-zero multi-digit hardware index only if the
        // language/parser later requires that distinction. The lexer preserves
        // the spelling and deliberately leaves semantic range validation to the
        // parser/validator.
        Ok(TokenKind::HardwareQubit)
    }

    // =========================================================================
    // Numeric literals
    // =========================================================================

    fn lex_number(&mut self, start: usize) -> LexerResult<TokenKind> {
        // OpenQASM integer bases.
        if self.current_char() == Some('0') {
            match self.peek_char() {
                Some('x') | Some('X') => {
                    self.advance_char();
                    self.advance_char();

                    return self.lex_based_integer(start, 16);
                }

                Some('b') | Some('B') => {
                    self.advance_char();
                    self.advance_char();

                    return self.lex_based_integer(start, 2);
                }

                Some('o') | Some('O') => {
                    self.advance_char();
                    self.advance_char();

                    return self.lex_based_integer(start, 8);
                }

                _ => {}
            }
        }

        self.lex_decimal_number(start)
    }

    fn lex_based_integer(
        &mut self,
        start: usize,
        radix: u32,
    ) -> LexerResult<TokenKind> {
        let digit_start = self.offset;

        while let Some(ch) = self.current_char() {
            let valid = match radix {
                2 => matches!(ch, '0' | '1' | '_'),
                8 => matches!(ch, '0'..='7' | '_'),
                16 => ch.is_ascii_hexdigit() || ch == '_',
                _ => false,
            };

            if !valid {
                break;
            }

            self.advance_char();
        }

        if self.offset == digit_start {
            return Err(LexError::new(
                LexErrorKind::InvalidNumber,
                Span::new(start, self.offset),
                "based integer literal requires at least one digit",
            ));
        }

        self.validate_numeric_length(start)?;

        Ok(TokenKind::IntegerLiteral)
    }

    fn lex_decimal_number(
        &mut self,
        start: usize,
    ) -> LexerResult<TokenKind> {
        let mut saw_digits = false;

        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() || ch == '_' {
                saw_digits = saw_digits || ch.is_ascii_digit();
                self.advance_char();
            } else {
                break;
            }
        }

        if !saw_digits {
            return Err(LexError::new(
                LexErrorKind::InvalidNumber,
                Span::new(start, self.offset),
                "decimal literal requires at least one digit",
            ));
        }

        let mut is_float = false;

        // Fractional part.
        if self.current_char() == Some('.')
            && self.peek_char().is_some_and(|c| c.is_ascii_digit())
        {
            is_float = true;
            self.advance_char();

            while let Some(ch) = self.current_char() {
                if ch.is_ascii_digit() || ch == '_' {
                    self.advance_char();
                } else {
                    break;
                }
            }
        }

        // Exponent.
        if matches!(self.current_char(), Some('e' | 'E')) {
            is_float = true;
            self.advance_char();

            if matches!(self.current_char(), Some('+' | '-')) {
                self.advance_char();
            }

            let exponent_start = self.offset;

            while let Some(ch) = self.current_char() {
                if ch.is_ascii_digit() || ch == '_' {
                    self.advance_char();
                } else {
                    break;
                }
            }

            if self.offset == exponent_start {
                return Err(LexError::new(
                    LexErrorKind::InvalidNumber,
                    Span::new(start, self.offset),
                    "exponent requires at least one digit",
                ));
            }
        }

        // Imaginary suffix.
        if self.current_char() == Some('i') {
            self.advance_char();
            self.validate_numeric_length(start)?;
            return Ok(TokenKind::ImaginaryLiteral);
        }

        // OpenQASM duration suffixes.
        if self.current_char().is_some_and(is_duration_suffix_start) {
            return self.lex_duration_suffix(start);
        }

        self.validate_numeric_length(start)?;

        if is_float {
            Ok(TokenKind::FloatLiteral)
        } else {
            Ok(TokenKind::IntegerLiteral)
        }
    }

    fn lex_duration_suffix(
        &mut self,
        start: usize,
    ) -> LexerResult<TokenKind> {
        let suffix_start = self.offset;

        while let Some(ch) = self.current_char() {
            if ch.is_ascii_alphabetic() {
                self.advance_char();
            } else {
                break;
            }
        }

        let suffix = &self.source[suffix_start..self.offset];

        match suffix {
            "dt" | "ns" | "us" | "ms" | "s" => {
                self.validate_numeric_length(start)?;
                Ok(TokenKind::DurationLiteral)
            }

            _ => Err(LexError::new(
                LexErrorKind::InvalidDuration,
                Span::new(suffix_start, self.offset),
                format!("unknown duration suffix `{suffix}`"),
            )),
        }
    }

    fn validate_numeric_length(&self, start: usize) -> LexerResult<()> {
        let end = self.offset;

        let Some(text) = self.source.get(start..end) else {
            return Err(LexError::new(
                LexErrorKind::InvalidSourceBoundary,
                Span::new(start, end),
                "numeric literal does not align with UTF-8 boundaries",
            ));
        };

        let digits = text
            .chars()
            .filter(|c| c.is_ascii_digit())
            .count();

        if digits > self.config.limits.max_numeric_digits {
            return Err(LexError::new(
                LexErrorKind::NumericLiteralTooLong,
                Span::new(start, end),
                "numeric literal exceeds configured digit limit",
            ));
        }

        self.check_lexeme_size(start, end)
    }

    // =========================================================================
    // String literals
    // =========================================================================

    fn lex_string(&mut self, start: usize) -> LexerResult<TokenKind> {
        self.advance_char(); // opening "

        let content_start = self.offset;

        loop {
            let Some(ch) = self.current_char() else {
                return Err(LexError::new(
                    LexErrorKind::UnterminatedString,
                    Span::new(start, self.offset),
                    "unterminated string literal",
                ));
            };

            if self.offset.saturating_sub(content_start)
                > self.config.limits.max_string_bytes
            {
                return Err(LexError::new(
                    LexErrorKind::StringTooLarge,
                    Span::new(start, self.offset),
                    "string literal exceeds configured maximum size",
                ));
            }

            match ch {
                '"' => {
                    self.advance_char();
                    return Ok(TokenKind::StringLiteral);
                }

                '\\' => {
                    self.advance_char();
                    self.validate_escape(start)?;
                }

                '\n' | '\r' => {
                    return Err(LexError::new(
                        LexErrorKind::UnterminatedString,
                        Span::new(start, self.offset),
                        "newline is not permitted before the end of a string literal",
                    ));
                }

                _ => {
                    self.advance_char();
                }
            }
        }
    }

    fn validate_escape(&mut self, start: usize) -> LexerResult<()> {
        let Some(ch) = self.current_char() else {
            return Err(LexError::new(
                LexErrorKind::InvalidEscape,
                Span::new(start, self.offset),
                "unterminated escape sequence",
            ));
        };

        match ch {
            'n' | 'r' | 't' | '\\' | '"' | '\'' | '0' => {
                self.advance_char();
                Ok(())
            }

            'x' => {
                self.advance_char();

                for _ in 0..2 {
                    match self.current_char() {
                        Some(c) if c.is_ascii_hexdigit() => {
                            self.advance_char();
                        }

                        _ => {
                            return Err(LexError::new(
                                LexErrorKind::InvalidEscape,
                                Span::new(start, self.offset),
                                "hexadecimal escape requires exactly two hexadecimal digits",
                            ));
                        }
                    }
                }

                Ok(())
            }

            'u' => {
                self.advance_char();

                // OpenQASM strings use Unicode escapes. Accept the common
                // \u{...} form while preserving the source spelling for the
                // parser/consumer.
                if self.current_char() == Some('{') {
                    self.advance_char();

                    let digit_start = self.offset;
                    let mut digits = 0usize;

                    while let Some(c) = self.current_char() {
                        if c == '}' {
                            break;
                        }

                        if !c.is_ascii_hexdigit() {
                            return Err(LexError::new(
                                LexErrorKind::InvalidEscape,
                                Span::new(start, self.offset),
                                "Unicode escape contains a non-hexadecimal character",
                            ));
                        }

                        digits += 1;

                        if digits > 6 {
                            return Err(LexError::new(
                                LexErrorKind::InvalidEscape,
                                Span::new(start, self.offset),
                                "Unicode escape contains more than six hexadecimal digits",
                            ));
                        }

                        self.advance_char();
                    }

                    if self.offset == digit_start {
                        return Err(LexError::new(
                            LexErrorKind::InvalidEscape,
                            Span::new(start, self.offset),
                            "Unicode escape requires at least one hexadecimal digit",
                        ));
                    }

                    if self.current_char() != Some('}') {
                        return Err(LexError::new(
                            LexErrorKind::InvalidEscape,
                            Span::new(start, self.offset),
                            "unterminated Unicode escape",
                        ));
                    }

                    self.advance_char();
                    Ok(())
                } else {
                    for _ in 0..4 {
                        match self.current_char() {
                            Some(c) if c.is_ascii_hexdigit() => {
                                self.advance_char();
                            }

                            _ => {
                                return Err(LexError::new(
                                    LexErrorKind::InvalidEscape,
                                    Span::new(start, self.offset),
                                    "Unicode escape requires four hexadecimal digits",
                                ));
                            }
                        }
                    }

                    Ok(())
                }
            }

            _ => Err(LexError::new(
                LexErrorKind::InvalidEscape,
                Span::new(start, self.offset + ch.len_utf8()),
                format!("unsupported escape sequence `\\{ch}`"),
            )),
        }
    }

    // =========================================================================
    // Annotation / pragma
    // =========================================================================

    fn lex_annotation(
        &mut self,
        start: usize,
    ) -> LexerResult<TokenKind> {
        self.advance_char(); // @

        let identifier_start = self.offset;

        while let Some(ch) = self.current_char() {
            if is_annotation_continue(ch) {
                self.advance_char();
            } else {
                break;
            }
        }

        if self.offset == identifier_start {
            return Ok(TokenKind::At);
        }

        self.check_lexeme_size(start, self.offset)?;

        Ok(TokenKind::Annotation)
    }

    fn lex_hash(&mut self, start: usize) -> LexerResult<TokenKind> {
        if self.source[start..].starts_with("#pragma") {
            let after = start + "#pragma".len();

            if self
                .source
                .get(after..)
                .and_then(|rest| rest.chars().next())
                .is_none_or(|c| c.is_whitespace())
            {
                while let Some(ch) = self.current_char() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }

                    self.advance_char();

                    if self.offset.saturating_sub(start)
                        > self.config.limits.max_comment_bytes
                    {
                        return Err(LexError::new(
                            LexErrorKind::CommentTooLarge,
                            Span::new(start, self.offset),
                            "pragma exceeds configured maximum size",
                        ));
                    }
                }

                return Ok(TokenKind::Pragma);
            }
        }

        self.advance_char();
        Ok(TokenKind::Hash)
    }

    // =========================================================================
    // Operators
    // =========================================================================

    fn lex_dot_or_number(
        &mut self,
        start: usize,
    ) -> LexerResult<TokenKind> {
        if self
            .peek_char()
            .is_some_and(|ch| ch.is_ascii_digit())
        {
            self.advance_char();

            while let Some(ch) = self.current_char() {
                if ch.is_ascii_digit() || ch == '_' {
                    self.advance_char();
                } else {
                    break;
                }
            }

            if matches!(self.current_char(), Some('e' | 'E')) {
                self.advance_char();

                if matches!(self.current_char(), Some('+' | '-')) {
                    self.advance_char();
                }

                let exponent_start = self.offset;

                while let Some(ch) = self.current_char() {
                    if ch.is_ascii_digit() || ch == '_' {
                        self.advance_char();
                    } else {
                        break;
                    }
                }

                if self.offset == exponent_start {
                    return Err(LexError::new(
                        LexErrorKind::InvalidNumber,
                        Span::new(start, self.offset),
                        "exponent requires at least one digit",
                    ));
                }
            }

            if self.current_char() == Some('i') {
                self.advance_char();
                self.validate_numeric_length(start)?;
                return Ok(TokenKind::ImaginaryLiteral);
            }

            self.validate_numeric_length(start)?;
            return Ok(TokenKind::FloatLiteral);
        }

        self.advance_char();
        Ok(TokenKind::Dot)
    }

    fn lex_plus(&mut self, _start: usize) -> LexerResult<TokenKind> {
        self.advance_char();

        if self.current_char() == Some('+') {
            self.advance_char();
            Ok(TokenKind::Increment)
        } else if self.current_char() == Some('=') {
            self.advance_char();
            Ok(TokenKind::PlusEqual)
        } else {
            Ok(TokenKind::Plus)
        }
    }

    fn lex_minus(&mut self, _start: usize) -> LexerResult<TokenKind> {
        self.advance_char();

        if self.current_char() == Some('-') {
            self.advance_char();
            Ok(TokenKind::Decrement)
        } else if self.current_char() == Some('=') {
            self.advance_char();
            Ok(TokenKind::MinusEqual)
        } else if self.current_char() == Some('>') {
            self.advance_char();
            Ok(TokenKind::Arrow)
        } else {
            Ok(TokenKind::Minus)
        }
    }

    fn lex_star(&mut self, _start: usize) -> LexerResult<TokenKind> {
        self.advance_char();

        if self.current_char() == Some('*') {
            self.advance_char();
            Ok(TokenKind::Power)
        } else if self.current_char() == Some('=') {
            self.advance_char();
            Ok(TokenKind::StarEqual)
        } else {
            Ok(TokenKind::Star)
        }
    }

    fn lex_slash(&mut self, _start: usize) -> LexerResult<TokenKind> {
        self.advance_char();

        if self.current_char() == Some('=') {
            self.advance_char();
            Ok(TokenKind::SlashEqual)
        } else {
            Ok(TokenKind::Slash)
        }
    }

    fn lex_percent(&mut self, _start: usize) -> LexerResult<TokenKind> {
        self.advance_char();

        if self.current_char() == Some('=') {
            self.advance_char();
            Ok(TokenKind::PercentEqual)
        } else {
            Ok(TokenKind::Percent)
        }
    }

    fn lex_ampersand(
        &mut self,
        _start: usize,
    ) -> LexerResult<TokenKind> {
        self.advance_char();

        if self.current_char() == Some('&') {
            self.advance_char();
            Ok(TokenKind::LogicalAnd)
        } else {
            Ok(TokenKind::BitAnd)
        }
    }

    fn lex_pipe(&mut self, _start: usize) -> LexerResult<TokenKind> {
        self.advance_char();

        if self.current_char() == Some('|') {
            self.advance_char();
            Ok(TokenKind::LogicalOr)
        } else {
            Ok(TokenKind::BitOr)
        }
    }

    fn lex_bang(&mut self, _start: usize) -> LexerResult<TokenKind> {
        self.advance_char();

        if self.current_char() == Some('=') {
            self.advance_char();
            Ok(TokenKind::NotEqual)
        } else {
            Ok(TokenKind::LogicalNot)
        }
    }

    fn lex_equal(&mut self, _start: usize) -> LexerResult<TokenKind> {
        self.advance_char();

        if self.current_char() == Some('=') {
            self.advance_char();
            Ok(TokenKind::EqualEqual)
        } else {
            Ok(TokenKind::Equal)
        }
    }

    fn lex_less(&mut self, _start: usize) -> LexerResult<TokenKind> {
        self.advance_char();

        if self.current_char() == Some('=') {
            self.advance_char();
            Ok(TokenKind::LessEqual)
        } else if self.current_char() == Some('<') {
            self.advance_char();
            Ok(TokenKind::ShiftLeft)
        } else {
            Ok(TokenKind::Less)
        }
    }

    fn lex_greater(
        &mut self,
        _start: usize,
    ) -> LexerResult<TokenKind> {
        self.advance_char();

        if self.current_char() == Some('=') {
            self.advance_char();
            Ok(TokenKind::GreaterEqual)
        } else if self.current_char() == Some('>') {
            self.advance_char();
            Ok(TokenKind::ShiftRight)
        } else {
            Ok(TokenKind::Greater)
        }
    }

    // =========================================================================
    // Token construction / limits
    // =========================================================================

    fn make_token(
        &mut self,
        start: usize,
        kind: TokenKind,
    ) -> LexerResult<Token<'src>> {
        let end = self.offset;

        self.check_lexeme_size(start, end)?;

        let lexeme = self.source.get(start..end).ok_or_else(|| {
            LexError::new(
                LexErrorKind::InvalidSourceBoundary,
                Span::new(start, end),
                "token does not align with UTF-8 source boundaries",
            )
        })?;

        self.token_count = self
            .token_count
            .checked_add(1)
            .ok_or_else(|| {
                LexError::new(
                    LexErrorKind::TokenLimitExceeded,
                    Span::new(start, end),
                    "token counter overflow",
                )
            })?;

        if self.token_count > self.config.limits.max_tokens {
            return Err(LexError::new(
                LexErrorKind::TokenLimitExceeded,
                Span::new(start, end),
                format!(
                    "token count exceeds configured maximum of {}",
                    self.config.limits.max_tokens
                ),
            ));
        }

        Ok(Token::new(
            kind,
            Span::new(start, end),
            lexeme,
        ))
    }

    fn eof_token(&self) -> LexerResult<Token<'src>> {
        Ok(Token::new(
            TokenKind::Eof,
            Span::new(self.source.len(), self.source.len()),
            "",
        ))
    }

    fn ensure_token_capacity(&self) -> LexerResult<()> {
        if self.token_count >= self.config.limits.max_tokens {
            return Err(LexError::new(
                LexErrorKind::TokenLimitExceeded,
                Span::new(self.offset, self.offset),
                format!(
                    "token count reached configured maximum of {}",
                    self.config.limits.max_tokens
                ),
            ));
        }

        Ok(())
    }

    fn check_lexeme_size(
        &self,
        start: usize,
        end: usize,
    ) -> LexerResult<()> {
        let size = end.saturating_sub(start);

        if size > self.config.limits.max_lexeme_bytes {
            return Err(LexError::new(
                LexErrorKind::LexemeTooLarge,
                Span::new(start, end),
                format!(
                    "lexical item contains {} bytes but the configured maximum is {}",
                    size,
                    self.config.limits.max_lexeme_bytes
                ),
            ));
        }

        Ok(())
    }

    // =========================================================================
    // Source traversal
    // =========================================================================

    fn current_char(&self) -> Option<char> {
        self.source.get(self.offset..)?.chars().next()
    }

    fn peek_char(&self) -> Option<char> {
        let current = self.current_char()?;
        let next_offset = self.offset + current.len_utf8();

        self.source.get(next_offset..)?.chars().next()
    }

    fn advance_char(&mut self) {
        if let Some(ch) = self.current_char() {
            self.offset += ch.len_utf8();
        }
    }
}

// =============================================================================
// Unicode identifier rules
// =============================================================================

/// Returns whether `ch` may begin an OpenQASM identifier.
///
/// OpenQASM follows Unicode identifier categories rather than limiting
/// identifiers to ASCII.
///
/// Rust's standard library does not expose Unicode General Categories directly,
/// so the implementation below uses the stable `char` APIs available on the
/// repository's Rust version plus explicit ranges for the relevant Unicode
/// identifier classes.
///
/// The parser/semantic layer remains responsible for any future specification
/// refinements.
fn is_identifier_start(ch: char) -> bool {
    ch == '_'
        || ch.is_alphabetic()
        || is_unicode_number_letter(ch)
}

/// Returns whether `ch` may continue an OpenQASM identifier.
fn is_identifier_continue(ch: char) -> bool {
    is_identifier_start(ch) || ch.is_ascii_digit()
}

/// Unicode `Nl` characters include letter numbers such as Roman numerals.
///
/// The standard library does not expose `General_Category=Nl` directly.
/// These ranges cover the Unicode number-letter blocks relevant to identifier
/// processing without introducing another runtime dependency.
fn is_unicode_number_letter(ch: char) -> bool {
    matches!(
        ch,
        '\u{2160}'..='\u{2188}'
            | '\u{3007}'
            | '\u{3021}'..='\u{3029}'
            | '\u{3038}'..='\u{303A}'
            | '\u{A6E6}'..='\u{A6EF}'
    )
}

fn is_annotation_continue(ch: char) -> bool {
    is_identifier_continue(ch)
        || matches!(
            ch,
            '-'
                | '.'
                | ':'
                | '/'
                | '\\'
                | '+'
                | '='
                | '*'
                | '%'
                | '!'
                | '?'
                | ','
                | '('
                | ')'
                | '['
                | ']'
                | '{'
                | '}'
                | ' '
                | '\t'
        )
}

fn is_duration_suffix_start(ch: char) -> bool {
    matches!(ch, 'd' | 'n' | 'u' | 'm' | 's')
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(source: &str) -> Vec<Token<'_>> {
        OpenQasmLexer::new(source)
            .expect("lexer construction must succeed")
            .tokenize()
            .expect("lexing must succeed")
    }

    fn kinds(source: &str) -> Vec<TokenKind> {
        lex(source)
            .into_iter()
            .map(Token::kind)
            .collect()
    }

    #[test]
    fn empty_source_produces_eof() {
        assert_eq!(kinds(""), vec![TokenKind::Eof]);
    }

    #[test]
    fn lexes_openqasm_header() {
        assert_eq!(
            kinds("OPENQASM 3.0;"),
            vec![
                TokenKind::KwOpenQasm,
                TokenKind::IntegerLiteral,
                TokenKind::Dot,
                TokenKind::IntegerLiteral,
                TokenKind::Semicolon,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn lexes_include() {
        assert_eq!(
            kinds(r#"include "stdgates.inc";"#),
            vec![
                TokenKind::KwInclude,
                TokenKind::StringLiteral,
                TokenKind::Semicolon,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn lexes_quantum_declaration() {
        assert_eq!(
            kinds("qubit[5] q;"),
            vec![
                TokenKind::KwQubit,
                TokenKind::LBracket,
                TokenKind::IntegerLiteral,
                TokenKind::RBracket,
                TokenKind::Identifier,
                TokenKind::Semicolon,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn lexes_bit_declaration() {
        assert_eq!(
            kinds("bit[5] c;"),
            vec![
                TokenKind::KwBit,
                TokenKind::LBracket,
                TokenKind::IntegerLiteral,
                TokenKind::RBracket,
                TokenKind::Identifier,
                TokenKind::Semicolon,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn lexes_gate_call() {
        assert_eq!(
            kinds("cx q[0], q[1];"),
            vec![
                TokenKind::Identifier,
                TokenKind::Identifier,
                TokenKind::LBracket,
                TokenKind::IntegerLiteral,
                TokenKind::RBracket,
                TokenKind::Comma,
                TokenKind::Identifier,
                TokenKind::LBracket,
                TokenKind::IntegerLiteral,
                TokenKind::RBracket,
                TokenKind::Semicolon,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn lexes_keywords_case_sensitively() {
        assert_eq!(
            kinds("OPENQASM openqasm qubit QUBIT"),
            vec![
                TokenKind::KwOpenQasm,
                TokenKind::Identifier,
                TokenKind::KwQubit,
                TokenKind::Identifier,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn lexes_hardware_qubits() {
        assert_eq!(
            kinds("$0 $1 $127"),
            vec![
                TokenKind::HardwareQubit,
                TokenKind::HardwareQubit,
                TokenKind::HardwareQubit,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn rejects_bare_dollar() {
        let error = OpenQasmLexer::new("$")
            .expect("construction must succeed")
            .tokenize()
            .expect_err("bare $ must fail");

        assert_eq!(
            error.kind(),
            LexErrorKind::InvalidHardwareQubit
        );
    }

    #[test]
    fn lexes_unicode_identifier() {
        let tokens = lex("qubit q; αβγ;");
        assert_eq!(
            tokens[3].kind(),
            TokenKind::Identifier
        );
        assert_eq!(tokens[3].lexeme(), "αβγ");
    }

    #[test]
    fn lexes_integer_bases() {
        assert_eq!(
            kinds("42 0b1010 0o52 0x2a"),
            vec![
                TokenKind::IntegerLiteral,
                TokenKind::IntegerLiteral,
                TokenKind::IntegerLiteral,
                TokenKind::IntegerLiteral,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn lexes_float() {
        assert_eq!(
            kinds("1.0 1.5e-2 .25 10E+3"),
            vec![
                TokenKind::FloatLiteral,
                TokenKind::FloatLiteral,
                TokenKind::FloatLiteral,
                TokenKind::FloatLiteral,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn lexes_imaginary() {
        assert_eq!(
            kinds("1i 2.5i"),
            vec![
                TokenKind::ImaginaryLiteral,
                TokenKind::ImaginaryLiteral,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn lexes_duration_literals() {
        assert_eq!(
            kinds("10ns 2us 3ms 4s 5dt"),
            vec![
                TokenKind::DurationLiteral,
                TokenKind::DurationLiteral,
                TokenKind::DurationLiteral,
                TokenKind::DurationLiteral,
                TokenKind::DurationLiteral,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn rejects_unknown_duration() {
        let error = OpenQasmLexer::new("10foo")
            .expect("construction must succeed")
            .tokenize()
            .expect_err("invalid duration must fail");

        assert_eq!(
            error.kind(),
            LexErrorKind::InvalidDuration
        );
    }

    #[test]
    fn lexes_string() {
        let tokens = lex(r#""stdgates.inc""#);

        assert_eq!(
            tokens[0].kind(),
            TokenKind::StringLiteral
        );
        assert_eq!(tokens[0].lexeme(), r#""stdgates.inc""#);
    }

    #[test]
    fn rejects_unterminated_string() {
        let error = OpenQasmLexer::new(r#""stdgates.inc"#)
            .expect("construction must succeed")
            .tokenize()
            .expect_err("unterminated string must fail");

        assert_eq!(
            error.kind(),
            LexErrorKind::UnterminatedString
        );
    }

    #[test]
    fn rejects_invalid_escape() {
        let error = OpenQasmLexer::new(r#""bad\q""#)
            .expect("construction must succeed")
            .tokenize()
            .expect_err("invalid escape must fail");

        assert_eq!(
            error.kind(),
            LexErrorKind::InvalidEscape
        );
    }

    #[test]
    fn skips_line_comments() {
        assert_eq!(
            kinds("// comment\nqubit q;"),
            vec![
                TokenKind::KwQubit,
                TokenKind::Identifier,
                TokenKind::Semicolon,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn skips_block_comments() {
        assert_eq!(
            kinds("/* comment */ qubit q;"),
            vec![
                TokenKind::KwQubit,
                TokenKind::Identifier,
                TokenKind::Semicolon,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn rejects_unterminated_block_comment() {
        let error = OpenQasmLexer::new("/* comment")
            .expect("construction must succeed")
            .tokenize()
            .expect_err("unterminated comment must fail");

        assert_eq!(
            error.kind(),
            LexErrorKind::UnterminatedComment
        );
    }

    #[test]
    fn lexes_assignment_and_comparison() {
        assert_eq!(
            kinds("a = b == c != d <= e >= f"),
            vec![
                TokenKind::Identifier,
                TokenKind::Equal,
                TokenKind::Identifier,
                TokenKind::EqualEqual,
                TokenKind::Identifier,
                TokenKind::NotEqual,
                TokenKind::Identifier,
                TokenKind::LessEqual,
                TokenKind::Identifier,
                TokenKind::GreaterEqual,
                TokenKind::Identifier,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn lexes_logical_operators() {
        assert_eq!(
            kinds("a && b || !c"),
            vec![
                TokenKind::Identifier,
                TokenKind::LogicalAnd,
                TokenKind::Identifier,
                TokenKind::LogicalOr,
                TokenKind::LogicalNot,
                TokenKind::Identifier,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn lexes_bitwise_operators() {
        assert_eq!(
            kinds("a & b | c ^ ~d"),
            vec![
                TokenKind::Identifier,
                TokenKind::BitAnd,
                TokenKind::Identifier,
                TokenKind::BitOr,
                TokenKind::Identifier,
                TokenKind::BitXor,
                TokenKind::BitNot,
                TokenKind::Identifier,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn lexes_shift_operators() {
        assert_eq!(
            kinds("a << 1 a >> 1"),
            vec![
                TokenKind::Identifier,
                TokenKind::ShiftLeft,
                TokenKind::IntegerLiteral,
                TokenKind::Identifier,
                TokenKind::ShiftRight,
                TokenKind::IntegerLiteral,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn lexes_compound_assignment() {
        assert_eq!(
            kinds("a += 1 b -= 1 c *= 2 d /= 2 e %= 2"),
            vec![
                TokenKind::Identifier,
                TokenKind::PlusEqual,
                TokenKind::IntegerLiteral,
                TokenKind::Identifier,
                TokenKind::MinusEqual,
                TokenKind::IntegerLiteral,
                TokenKind::Identifier,
                TokenKind::StarEqual,
                TokenKind::IntegerLiteral,
                TokenKind::Identifier,
                TokenKind::SlashEqual,
                TokenKind::IntegerLiteral,
                TokenKind::Identifier,
                TokenKind::PercentEqual,
                TokenKind::IntegerLiteral,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn lexes_increment_decrement() {
        assert_eq!(
            kinds("a++ b--"),
            vec![
                TokenKind::Identifier,
                TokenKind::Increment,
                TokenKind::Identifier,
                TokenKind::Decrement,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn lexes_arrow() {
        assert_eq!(
            kinds("a -> b"),
            vec![
                TokenKind::Identifier,
                TokenKind::Arrow,
                TokenKind::Identifier,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn lexes_annotations() {
        assert_eq!(
            kinds("@foo @bar.baz"),
            vec![
                TokenKind::Annotation,
                TokenKind::Annotation,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn lexes_pragma() {
        assert_eq!(
            kinds("#pragma something here"),
            vec![TokenKind::Pragma, TokenKind::Eof]
        );
    }

    #[test]
    fn preserves_token_spans() {
        let tokens = lex("h q[0];");

        assert_eq!(tokens[0].span(), Span::new(0, 1));
        assert_eq!(tokens[0].lexeme(), "h");

        assert_eq!(tokens[1].span(), Span::new(2, 3));
        assert_eq!(tokens[1].lexeme(), "q");

        assert_eq!(tokens[2].span(), Span::new(3, 4));
        assert_eq!(tokens[2].lexeme(), "[");

        assert_eq!(tokens[3].span(), Span::new(4, 5));
        assert_eq!(tokens[3].lexeme(), "0");

        assert_eq!(tokens[4].span(), Span::new(5, 6));
        assert_eq!(tokens[4].lexeme(), "]");

        assert_eq!(tokens[5].span(), Span::new(6, 7));
        assert_eq!(tokens[5].lexeme(), ";");
    }

    #[test]
    fn unicode_spans_are_byte_based() {
        let tokens = lex("α q;");

        assert_eq!(tokens[0].lexeme(), "α");
        assert_eq!(tokens[0].span(), Span::new(0, 2));

        assert_eq!(tokens[1].lexeme(), "q");
        assert_eq!(tokens[1].span(), Span::new(3, 4));
    }

    #[test]
    fn token_limit_is_enforced() {
        let config = LexerConfig {
            limits: LexerLimits {
                max_tokens: 2,
                ..LexerLimits::default()
            },
            ..LexerConfig::default()
        };

        let error = OpenQasmLexer::with_config(
            "q q q",
            config,
        )
        .expect("construction must succeed")
        .tokenize()
        .expect_err("token limit must be enforced");

        assert_eq!(
            error.kind(),
            LexErrorKind::TokenLimitExceeded
        );
    }

    #[test]
    fn source_limit_is_enforced() {
        let config = LexerConfig {
            limits: LexerLimits {
                max_source_bytes: 3,
                ..LexerLimits::default()
            },
            ..LexerConfig::default()
        };

        let error = OpenQasmLexer::with_config(
            "qubit",
            config,
        )
        .expect_err("source limit must be enforced");

        assert_eq!(
            error.kind(),
            LexErrorKind::SourceTooLarge
        );
    }

    #[test]
    fn lexing_is_deterministic() {
        let source = r#"
            OPENQASM 3.0;
            include "stdgates.inc";

            qubit[2] q;
            bit[2] c;

            h q[0];
            cx q[0], q[1];
            c[0] = measure q[0];
        "#;

        let first = OpenQasmLexer::new(source)
            .expect("first lexer")
            .tokenize()
            .expect("first tokenization");

        let second = OpenQasmLexer::new(source)
            .expect("second lexer")
            .tokenize()
            .expect("second tokenization");

        assert_eq!(first, second);
    }

    #[test]
    fn no_ir_dependency_is_required() {
        // This test is intentionally simple. The important architectural
        // property is compile-time: lexer.rs does not import quantum::ir.
        let tokens = lex("h q[0];");

        assert_eq!(tokens[0].kind(), TokenKind::Identifier);
    }
}