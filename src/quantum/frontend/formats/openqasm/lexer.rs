//! Zamani Quantum Frontend — OpenQASM lexer.
//!
//! Production lexical boundary for OpenQASM 3.
//!
//! Architectural boundary:
//!
//! ```text
//! untrusted UTF-8 source
//!         │
//!         ▼
//!     OpenQasmLexer
//!         │
//!         ▼
//!   Token<'src> stream
//!         │
//!         ▼
//!      parser.rs
//!         │
//!         ▼
//!     OpenQASM AST
//! ```
//!
//! This module:
//!
//! - performs lexical analysis only;
//! - does not perform semantic validation;
//! - does not construct Quantum IR;
//! - does not resolve symbols;
//! - does not resolve includes;
//! - does not access the filesystem;
//! - does not access the network;
//! - does not execute code;
//! - does not invoke a QPU;
//! - preserves exact source lexemes;
//! - preserves byte-accurate source spans;
//! - enforces explicit resource limits;
//! - guarantees progress for every successful lexical step;
//! - is deterministic;
//! - is intended to be panic-free for untrusted source;
//! - is compatible with Rust 1.97.1 / Rust 2021.
//!
//! OpenQASM lexical authority:
//!
//! <https://openqasm.com/versions/3.1/grammar/index.html>
//!
//! Important:
//!
//! OpenQASM's reference grammar uses lexical modes for:
//!
//! - the version specifier after `OPENQASM`;
//! - include / defcalgrammar strings;
//! - annotation/pragma remaining-line content;
//! - calibration prelude/block handling.
//!
//! This implementation keeps those context transitions inside the lexer,
//! without performing semantic work or external I/O.

use std::fmt;

// =============================================================================
// Token
// =============================================================================

/// A lexical token borrowed from the original source.
///
/// The lexer never copies token text into the token. `lexeme` is an exact
/// slice of the original source.
///
/// The source must remain alive while the token is used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'src> {
    kind: TokenKind,
    span: Span,
    lexeme: &'src str,
}

impl<'src> Token<'src> {
    /// Creates a token.
    #[must_use]
    pub const fn new(
        kind: TokenKind,
        span: Span,
        lexeme: &'src str,
    ) -> Self {
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

// =============================================================================
// Source span
// =============================================================================

/// Half-open byte span `[start, end)`.
///
/// Byte offsets are the canonical representation used by the lexer. The
/// frontend source subsystem can convert these offsets into line/column
/// information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    /// Inclusive byte offset.
    pub start: usize,

    /// Exclusive byte offset.
    pub end: usize,
}

impl Span {
    /// Creates a span.
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Returns the span length.
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

/// OpenQASM lexical token categories.
///
/// The names used here intentionally preserve the existing Zamani parser
/// vocabulary while covering the OpenQASM 3 lexical contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // -------------------------------------------------------------------------
    // Special
    // -------------------------------------------------------------------------

    Eof,
    Identifier,
    HardwareQubit,

    IntegerLiteral,
    FloatLiteral,
    ImaginaryLiteral,
    DurationLiteral,

    /// A string accepted by an include/defcalgrammar lexical mode.
    StringLiteral,

    /// A `"0101"` OpenQASM bitstring literal.
    BitstringLiteral,

    /// Version specifier.
    ///
    /// The current Zamani parser historically consumes the version spelling
    /// through its existing integer-literal path. The lexer therefore emits
    /// the version spelling as one token while preserving this semantic
    /// category in the implementation state.
    VersionSpecifier,

    /// Annotation token beginning with `@`.
    Annotation,

    /// Pragma / remaining-line directive.
    Pragma,

    /// Remaining text after an annotation/pragma.
    RemainingLineContent,

    /// Calibration-language body.
    CalibrationBlock,

    // -------------------------------------------------------------------------
    // Language keywords
    // -------------------------------------------------------------------------

    KwOpenQasm,
    KwInclude,
    KwDefcalGrammar,

    KwDef,
    KwCal,
    KwDefcal,

    KwGate,
    KwExtern,
    KwBox,
    KwLet,

    KwBreak,
    KwContinue,
    KwIf,
    KwElse,
    KwEnd,
    KwReturn,
    KwFor,
    KwWhile,
    KwIn,

    KwSwitch,
    KwCase,
    KwDefault,

    // -------------------------------------------------------------------------
    // Declarations / types
    // -------------------------------------------------------------------------

    KwInput,
    KwOutput,
    KwConst,
    KwReadonly,
    KwMutable,

    KwQreg,
    KwQubit,

    KwCreg,
    KwBit,
    KwBool,
    KwInt,
    KwUInt,
    KwFloat,
    KwAngle,
    KwComplex,
    KwArray,
    KwVoid,

    KwDuration,
    KwStretch,

    // -------------------------------------------------------------------------
    // Built-ins / quantum operations
    // -------------------------------------------------------------------------

    KwGphase,
    KwInv,
    KwPow,
    KwCtrl,
    KwNegctrl,

    /// `#dim`.
    KwDim,

    KwDurationof,

    KwDelay,
    KwReset,
    KwMeasure,
    KwBarrier,

    KwTrue,
    KwFalse,

    // Kept for backwards compatibility with the existing lexer API.
    // These are intentionally not OpenQASM-reserved keywords.
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
    Increment,

    Minus,
    Decrement,

    Star,
    Power,

    Slash,
    Percent,

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

    AmpersandEqual,
    PipeEqual,
    TildeEqual,
    CaretEqual,

    ShiftLeftEqual,
    ShiftRightEqual,

    PowerEqual,

    Arrow,

    At,
    Hash,
}

// =============================================================================
// Token classification
// =============================================================================

impl TokenKind {
    /// Classifies an ordinary identifier spelling as a keyword.
    ///
    /// OpenQASM is case-sensitive.
    #[must_use]
    pub fn keyword(lexeme: &str) -> Option<Self> {
        Some(match lexeme {
            "OPENQASM" => Self::KwOpenQasm,
            "include" => Self::KwInclude,
            "defcalgrammar" => Self::KwDefcalGrammar,

            "def" => Self::KwDef,
            "cal" => Self::KwCal,
            "defcal" => Self::KwDefcal,

            "gate" => Self::KwGate,
            "extern" => Self::KwExtern,
            "box" => Self::KwBox,
            "let" => Self::KwLet,

            "break" => Self::KwBreak,
            "continue" => Self::KwContinue,
            "if" => Self::KwIf,
            "else" => Self::KwElse,
            "end" => Self::KwEnd,
            "return" => Self::KwReturn,
            "for" => Self::KwFor,
            "while" => Self::KwWhile,
            "in" => Self::KwIn,

            // Kept because the current Zamani AST/parser already models
            // switch statements and the reference grammar contains them.
            "switch" => Self::KwSwitch,
            "case" => Self::KwCase,
            "default" => Self::KwDefault,

            "input" => Self::KwInput,
            "output" => Self::KwOutput,
            "const" => Self::KwConst,
            "readonly" => Self::KwReadonly,
            "mutable" => Self::KwMutable,

            "qreg" => Self::KwQreg,
            "qubit" => Self::KwQubit,

            "creg" => Self::KwCreg,
            "bit" => Self::KwBit,
            "bool" => Self::KwBool,
            "int" => Self::KwInt,
            "uint" => Self::KwUInt,
            "float" => Self::KwFloat,
            "angle" => Self::KwAngle,
            "complex" => Self::KwComplex,
            "array" => Self::KwArray,
            "void" => Self::KwVoid,

            "duration" => Self::KwDuration,
            "stretch" => Self::KwStretch,

            "gphase" => Self::KwGphase,
            "inv" => Self::KwInv,
            "pow" => Self::KwPow,
            "ctrl" => Self::KwCtrl,
            "negctrl" => Self::KwNegctrl,

            "durationof" => Self::KwDurationof,

            "delay" => Self::KwDelay,
            "reset" => Self::KwReset,
            "measure" => Self::KwMeasure,
            "barrier" => Self::KwBarrier,

            "true" => Self::KwTrue,
            "false" => Self::KwFalse,

            // `pi` and `euler` are not reserved by the OpenQASM lexical
            // grammar. They therefore remain ordinary identifiers.
            _ => return None,
        })
    }

    /// Returns whether this token is identifier-like.
    #[must_use]
    pub const fn is_identifier_like(self) -> bool {
        matches!(
            self,
            Self::Identifier | Self::HardwareQubit
        )
    }

    /// Returns whether this token is a literal.
    #[must_use]
    pub const fn is_literal(self) -> bool {
        matches!(
            self,
            Self::IntegerLiteral
                | Self::FloatLiteral
                | Self::ImaginaryLiteral
                | Self::DurationLiteral
                | Self::StringLiteral
                | Self::BitstringLiteral
                | Self::KwTrue
                | Self::KwFalse
                | Self::HardwareQubit
        )
    }

    /// Returns whether this is a compound-assignment operator.
    #[must_use]
    pub const fn is_compound_assignment(self) -> bool {
        matches!(
            self,
            Self::PlusEqual
                | Self::MinusEqual
                | Self::StarEqual
                | Self::SlashEqual
                | Self::PercentEqual
                | Self::AmpersandEqual
                | Self::PipeEqual
                | Self::TildeEqual
                | Self::CaretEqual
                | Self::ShiftLeftEqual
                | Self::ShiftRightEqual
                | Self::PowerEqual
        )
    }
}

// =============================================================================
// Lexer limits
// =============================================================================

/// Resource limits for lexical analysis.
///
/// These limits protect the frontend from pathological source without
/// constraining downstream Quantum IR resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LexerLimits {
    /// Maximum source size.
    pub max_source_bytes: usize,

    /// Maximum emitted token count.
    pub max_tokens: usize,

    /// Maximum individual token size.
    pub max_lexeme_bytes: usize,

    /// Maximum include/ordinary string size.
    pub max_string_bytes: usize,

    /// Maximum bitstring size.
    pub max_bitstring_bytes: usize,

    /// Maximum comment size.
    pub max_comment_bytes: usize,

    /// Maximum identifier scalar count.
    pub max_identifier_scalars: usize,

    /// Maximum numeric digit count.
    pub max_numeric_digits: usize,

    /// Maximum annotation/pragma remaining-line size.
    pub max_directive_bytes: usize,

    /// Maximum calibration-block size.
    pub max_calibration_bytes: usize,

    /// Maximum calibration nesting.
    pub max_calibration_nesting: usize,
}

impl Default for LexerLimits {
    fn default() -> Self {
        Self {
            max_source_bytes: 16 * 1024 * 1024,
            max_tokens: 4_000_000,
            max_lexeme_bytes: 1 * 1024 * 1024,
            max_string_bytes: 1 * 1024 * 1024,
            max_bitstring_bytes: 1 * 1024 * 1024,
            max_comment_bytes: 1 * 1024 * 1024,
            max_identifier_scalars: 16 * 1024,
            max_numeric_digits: 1_000_000,
            max_directive_bytes: 1 * 1024 * 1024,
            max_calibration_bytes: 4 * 1024 * 1024,
            max_calibration_nesting: 4096,
        }
    }
}

/// Lexer configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LexerConfig {
    /// Lexer resource limits.
    pub limits: LexerLimits,

    /// Emit comments as tokens.
    ///
    /// Production parser mode leaves this disabled.
    pub emit_comments: bool,

    /// Preserve blank directive lines as `RemainingLineContent`.
    pub preserve_empty_directives: bool,
}

impl Default for LexerConfig {
    fn default() -> Self {
        Self {
            limits: LexerLimits::default(),
            emit_comments: false,
            preserve_empty_directives: false,
        }
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Lexer result.
pub type LexerResult<T> = Result<T, LexError>;

/// Stable lexical error category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LexErrorKind {
    SourceTooLarge,
    TokenLimitExceeded,
    LexemeTooLarge,
    StringTooLarge,
    BitstringTooLarge,
    CommentTooLarge,
    DirectiveTooLarge,
    CalibrationTooLarge,
    CalibrationNestingTooDeep,

    IdentifierTooLong,
    NumericLiteralTooLong,

    UnexpectedCharacter,
    InvalidUtf8,
    InvalidSourceBoundary,

    UnterminatedString,
    InvalidString,
    UnterminatedComment,

    InvalidHardwareQubit,

    InvalidNumber,
    InvalidNumericSeparator,
    InvalidDuration,
    InvalidImaginaryLiteral,

    InvalidVersion,
    InvalidBitstring,

    InvalidAnnotation,
    InvalidPragma,

    UnterminatedCalibrationBlock,
}

impl LexErrorKind {
    /// Stable machine-readable code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::SourceTooLarge => "QASM-L001",
            Self::TokenLimitExceeded => "QASM-L002",
            Self::LexemeTooLarge => "QASM-L003",
            Self::StringTooLarge => "QASM-L004",
            Self::BitstringTooLarge => "QASM-L005",
            Self::CommentTooLarge => "QASM-L006",
            Self::DirectiveTooLarge => "QASM-L007",
            Self::CalibrationTooLarge => "QASM-L008",
            Self::CalibrationNestingTooDeep => "QASM-L009",

            Self::IdentifierTooLong => "QASM-L010",
            Self::NumericLiteralTooLong => "QASM-L011",

            Self::UnexpectedCharacter => "QASM-L012",
            Self::InvalidUtf8 => "QASM-L013",
            Self::InvalidSourceBoundary => "QASM-L014",

            Self::UnterminatedString => "QASM-L015",
            Self::InvalidString => "QASM-L016",
            Self::UnterminatedComment => "QASM-L017",

            Self::InvalidHardwareQubit => "QASM-L018",

            Self::InvalidNumber => "QASM-L019",
            Self::InvalidNumericSeparator => "QASM-L020",
            Self::InvalidDuration => "QASM-L021",
            Self::InvalidImaginaryLiteral => "QASM-L022",

            Self::InvalidVersion => "QASM-L023",
            Self::InvalidBitstring => "QASM-L024",

            Self::InvalidAnnotation => "QASM-L025",
            Self::InvalidPragma => "QASM-L026",

            Self::UnterminatedCalibrationBlock => "QASM-L027",
        }
    }
}

impl fmt::Display for LexErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

/// Structured lexical failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    kind: LexErrorKind,
    span: Span,
    message: String,
}

impl LexError {
    fn new(
        kind: LexErrorKind,
        span: Span,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            span,
            message: message.into(),
        }
    }

    /// Error category.
    #[must_use]
    pub const fn kind(&self) -> LexErrorKind {
        self.kind
    }

    /// Stable error code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.kind.code()
    }

    /// Error span.
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Error message.
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
// Lexer mode
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LexerMode {
    Default,
    Version,
    ArbitraryString,
    RemainingLine,
    CalibrationPrelude,
    CalibrationBlock,
}

// =============================================================================
// Lexer
// =============================================================================

/// Production OpenQASM lexer.
pub struct OpenQasmLexer<'src> {
    source: &'src str,
    config: LexerConfig,

    offset: usize,
    token_count: usize,

    mode: LexerMode,

    /// Set after `cal` / `defcal`.
    calibration_depth: usize,

    /// Whether EOF has already been emitted.
    finished: bool,
}

impl<'src> OpenQasmLexer<'src> {
    /// Creates a lexer using production defaults.
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
                    "source contains {} bytes; configured maximum is {}",
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
            mode: LexerMode::Default,
            calibration_depth: 0,
            finished: false,
        })
    }

    /// Returns the source.
    #[must_use]
    pub const fn source(&self) -> &'src str {
        self.source
    }

    /// Returns current byte offset.
    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Returns configuration.
    #[must_use]
    pub const fn config(&self) -> LexerConfig {
        self.config
    }

    /// Tokenizes the entire source.
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

    /// Produces one token.
    pub fn next_token(&mut self) -> LexerResult<Token<'src>> {
        if self.finished {
            return self.eof_token();
        }

        loop {
            match self.mode {
                LexerMode::Version => {
                    return self.lex_version_token();
                }

                LexerMode::ArbitraryString => {
                    return self.lex_arbitrary_string();
                }

                LexerMode::RemainingLine => {
                    return self.lex_remaining_line();
                }

                LexerMode::CalibrationBlock => {
                    return self.lex_calibration_block();
                }

                LexerMode::CalibrationPrelude => {
                    self.skip_whitespace_and_comments()?;

                    if self.current_char() == Some('{') {
                        let start = self.offset;
                        self.advance_char();

                        self.mode = LexerMode::CalibrationBlock;
                        self.calibration_depth = 1;

                        return self.make_token(
                            start,
                            TokenKind::LBrace,
                        );
                    }

                    return self.next_default_token();
                }

                LexerMode::Default => {
                    return self.next_default_token();
                }
            }
        }
    }

    // =========================================================================
    // Default mode
    // =========================================================================

    fn next_default_token(&mut self) -> LexerResult<Token<'src>> {
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
                "invalid UTF-8 source boundary",
            )
        })?;

        let kind = match ch {
            '$' => self.lex_hardware_qubit(start)?,

            c if is_identifier_start(c) => {
                self.lex_identifier(start)?
            }

            c if c.is_ascii_digit() => {
                self.lex_number(start)?
            }

            '.' => self.lex_dot_or_float(start)?,

            '"' => self.lex_bitstring(start)?,

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

                if self.mode == LexerMode::CalibrationBlock {
                    self.calibration_depth =
                        self.calibration_depth.saturating_sub(1);

                    if self.calibration_depth == 0 {
                        self.mode = LexerMode::Default;
                    }
                }

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

            '+' => self.lex_plus(start)?,
            '-' => self.lex_minus(start)?,
            '*' => self.lex_star(start)?,
            '/' => self.lex_slash(start)?,
            '%' => self.lex_percent(start)?,
            '&' => self.lex_ampersand(start)?,
            '|' => self.lex_pipe(start)?,
            '^' => self.lex_caret(start)?,
            '~' => self.lex_tilde(start)?,
            '!' => self.lex_bang(start)?,
            '=' => self.lex_equal(start)?,
            '<' => self.lex_less(start)?,
            '>' => self.lex_greater(start)?,
            '@' => self.lex_annotation(start)?,
            '#' => self.lex_hash(start)?,

            _ => {
                let end = start.saturating_add(ch.len_utf8());

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

            if ch == '/'
                && self.peek_char() == Some('/')
            {
                self.skip_line_comment()?;
                continue;
            }

            if ch == '/'
                && self.peek_char() == Some('*')
            {
                self.skip_block_comment()?;
                continue;
            }

            return Ok(());
        }
    }

    fn skip_line_comment(&mut self) -> LexerResult<()> {
        let start = self.offset;

        self.advance_char();
        self.advance_char();

        while let Some(ch) = self.current_char() {
            if ch == '\r' || ch == '\n' {
                break;
            }

            self.advance_char();

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

        self.advance_char();
        self.advance_char();

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

            if ch == '*'
                && self.peek_char() == Some('/')
            {
                self.advance_char();
                self.advance_char();
                return Ok(());
            }

            self.advance_char();
        }
    }

    // =========================================================================
    // Version mode
    // =========================================================================

    fn lex_version_token(&mut self) -> LexerResult<Token<'src>> {
        self.skip_whitespace_and_comments()?;

        let start = self.offset;

        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() || ch == '.' {
                self.advance_char();
            } else {
                break;
            }
        }

        let end = self.offset;

        let raw = self.source.get(start..end).ok_or_else(|| {
            LexError::new(
                LexErrorKind::InvalidSourceBoundary,
                Span::new(start, end),
                "version token is not on a valid UTF-8 boundary",
            )
        })?;

        if !is_valid_version(raw) {
            return Err(LexError::new(
                LexErrorKind::InvalidVersion,
                Span::new(start, end),
                "OpenQASM version must be `major` or `major.minor`",
            ));
        }

        self.mode = LexerMode::Default;

        /*
         * Compatibility with the current Zamani parser:
         *
         * parser.rs currently reads the version token's `lexeme()` directly
         * and therefore requires the complete `3.0` spelling to be one token.
         *
         * `IntegerLiteral` is retained here so the existing parser contract
         * remains valid without forcing lexer/parser churn.
         */
        self.make_token(start, TokenKind::IntegerLiteral)
    }

    // =========================================================================
    // Include / defcalgrammar string mode
    // =========================================================================

    fn lex_arbitrary_string(&mut self) -> LexerResult<Token<'src>> {
        self.skip_horizontal_and_newline_whitespace()?;

        let start = self.offset;

        let quote = match self.current_char() {
            Some('"') => '"',
            Some('\'') => '\'',
            _ => {
                return Err(LexError::new(
                    LexErrorKind::InvalidString,
                    Span::new(start, start),
                    "include/defcalgrammar requires a quoted string",
                ));
            }
        };

        self.advance_char();

        let content_start = self.offset;

        loop {
            let Some(ch) = self.current_char() else {
                return Err(LexError::new(
                    LexErrorKind::UnterminatedString,
                    Span::new(start, self.offset),
                    "unterminated include/defcalgrammar string",
                ));
            };

            if ch == '\r' || ch == '\n' {
                return Err(LexError::new(
                    LexErrorKind::InvalidString,
                    Span::new(start, self.offset),
                    "newline is not permitted inside an include string",
                ));
            }

            if ch == quote {
                let content_len =
                    self.offset.saturating_sub(content_start);

                if content_len == 0 {
                    return Err(LexError::new(
                        LexErrorKind::InvalidString,
                        Span::new(start, self.offset),
                        "include/defcalgrammar string must not be empty",
                    ));
                }

                self.advance_char();
                self.mode = LexerMode::Default;

                return self.make_token(
                    start,
                    TokenKind::StringLiteral,
                );
            }

            self.advance_char();

            if self.offset.saturating_sub(content_start)
                > self.config.limits.max_string_bytes
            {
                return Err(LexError::new(
                    LexErrorKind::StringTooLarge,
                    Span::new(start, self.offset),
                    "string exceeds configured maximum size",
                ));
            }
        }
    }

    // =========================================================================
    // Annotation / pragma line mode
    // =========================================================================

    fn lex_annotation(
        &mut self,
        start: usize,
    ) -> LexerResult<TokenKind> {
        self.advance_char(); // @

        if !is_identifier_start(
            self.current_char().unwrap_or('\0'),
        ) {
            return Err(LexError::new(
                LexErrorKind::InvalidAnnotation,
                Span::new(start, self.offset),
                "annotation requires an identifier after `@`",
            ));
        }

        while let Some(ch) = self.current_char() {
            if is_identifier_continue(ch)
                || ch == '.'
            {
                self.advance_char();
            } else {
                break;
            }
        }

        self.mode = LexerMode::RemainingLine;

        self.check_directive_size(start)?;

        Ok(TokenKind::Annotation)
    }

    fn lex_hash(
        &mut self,
        start: usize,
    ) -> LexerResult<TokenKind> {
        if self.source[start..].starts_with("#dim") {
            let after = start + 4;

            let boundary = self
                .source
                .get(after..)
                .and_then(|s| s.chars().next());

            if boundary
                .is_none_or(|c| !is_identifier_continue(c))
            {
                self.offset = after;
                return Ok(TokenKind::KwDim);
            }
        }

        let rest = &self.source[start..];

        if rest.starts_with("#pragma") {
            let after = start + "#pragma".len();

            let boundary = self
                .source
                .get(after..)
                .and_then(|s| s.chars().next());

            if boundary.is_none_or(|c| c.is_whitespace()) {
                self.offset = after;
                self.mode = LexerMode::RemainingLine;
                return Ok(TokenKind::Pragma);
            }
        }

        if self.source[start..].starts_with("pragma") {
            self.offset = start + "pragma".len();
            self.mode = LexerMode::RemainingLine;
            return Ok(TokenKind::Pragma);
        }

        self.advance_char();
        Ok(TokenKind::Hash)
    }

    fn lex_remaining_line(&mut self) -> LexerResult<Token<'src>> {
        while matches!(
            self.current_char(),
            Some(' ' | '\t')
        ) {
            self.advance_char();
        }

        let start = self.offset;

        while let Some(ch) = self.current_char() {
            if ch == '\r' || ch == '\n' {
                break;
            }

            self.advance_char();

            if self.offset.saturating_sub(start)
                > self.config.limits.max_directive_bytes
            {
                return Err(LexError::new(
                    LexErrorKind::DirectiveTooLarge,
                    Span::new(start, self.offset),
                    "directive content exceeds configured maximum size",
                ));
            }
        }

        self.mode = LexerMode::Default;

        if start == self.offset {
            if self.config.preserve_empty_directives {
                return self.make_token(
                    start,
                    TokenKind::RemainingLineContent,
                );
            }

            self.skip_newline();
            return self.next_token();
        }

        self.make_token(
            start,
            TokenKind::RemainingLineContent,
        )
    }

    // =========================================================================
    // Identifier
    // =========================================================================

    fn lex_identifier(
        &mut self,
        start: usize,
    ) -> LexerResult<TokenKind> {
        let mut scalar_count = 0usize;

        while let Some(ch) = self.current_char() {
            if !is_identifier_continue(ch) {
                break;
            }

            scalar_count = scalar_count
                .checked_add(1)
                .ok_or_else(|| {
                    LexError::new(
                        LexErrorKind::IdentifierTooLong,
                        Span::new(start, self.offset),
                        "identifier scalar counter overflow",
                    )
                })?;

            if scalar_count
                > self.config.limits.max_identifier_scalars
            {
                return Err(LexError::new(
                    LexErrorKind::IdentifierTooLong,
                    Span::new(start, self.offset),
                    "identifier exceeds configured scalar limit",
                ));
            }

            self.advance_char();
        }

        let end = self.offset;

        self.check_lexeme_size(start, end)?;

        let lexeme = self
            .source
            .get(start..end)
            .ok_or_else(|| {
                LexError::new(
                    LexErrorKind::InvalidSourceBoundary,
                    Span::new(start, end),
                    "identifier is not aligned to UTF-8 boundaries",
                )
            })?;

        let kind = TokenKind::keyword(lexeme)
            .unwrap_or(TokenKind::Identifier);

        if kind == TokenKind::KwOpenQasm {
            self.mode = LexerMode::Version;
        } else if kind == TokenKind::KwInclude
            || kind == TokenKind::KwDefcalGrammar
        {
            self.mode = LexerMode::ArbitraryString;
        } else if kind == TokenKind::KwCal
            || kind == TokenKind::KwDefcal
        {
            self.mode = LexerMode::CalibrationPrelude;
        }

        Ok(kind)
    }

    // =========================================================================
    // Hardware qubits
    // =========================================================================

    fn lex_hardware_qubit(
        &mut self,
        start: usize,
    ) -> LexerResult<TokenKind> {
        self.advance_char(); // $

        let digit_start = self.offset;

        while let Some(ch) = self.current_char() {
            if !ch.is_ascii_digit() {
                break;
            }

            self.advance_char();
        }

        if digit_start == self.offset {
            return Err(LexError::new(
                LexErrorKind::InvalidHardwareQubit,
                Span::new(start, self.offset),
                "hardware-qubit identifier requires decimal digits",
            ));
        }

        let digits = self
            .source
            .get(digit_start..self.offset)
            .ok_or_else(|| {
                LexError::new(
                    LexErrorKind::InvalidSourceBoundary,
                    Span::new(digit_start, self.offset),
                    "hardware-qubit digits are not valid UTF-8",
                )
            })?;

        if digits.len()
            > self.config.limits.max_numeric_digits
        {
            return Err(LexError::new(
                LexErrorKind::NumericLiteralTooLong,
                Span::new(digit_start, self.offset),
                "hardware-qubit index exceeds configured digit limit",
            ));
        }

        Ok(TokenKind::HardwareQubit)
    }

    // =========================================================================
    // Numbers
    // =========================================================================

    fn lex_number(
        &mut self,
        start: usize,
    ) -> LexerResult<TokenKind> {
        if self.current_char() == Some('0') {
            match self.peek_char() {
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

                Some('x') | Some('X') => {
                    self.advance_char();
                    self.advance_char();
                    return self.lex_based_integer(start, 16);
                }

                _ => {}
            }
        }

        self.lex_decimal_or_float(start)
    }

    fn lex_based_integer(
        &mut self,
        start: usize,
        radix: u32,
    ) -> LexerResult<TokenKind> {
        let digits_start = self.offset;
        let mut digits = 0usize;
        let mut previous_was_digit = false;
        let mut previous_was_underscore = false;

        while let Some(ch) = self.current_char() {
            let is_digit = match radix {
                2 => matches!(ch, '0' | '1'),
                8 => matches!(ch, '0'..='7'),
                16 => ch.is_ascii_hexdigit(),
                _ => false,
            };

            if is_digit {
                digits = digits.saturating_add(1);
                previous_was_digit = true;
                previous_was_underscore = false;
                self.advance_char();
                continue;
            }

            if ch == '_' {
                if !previous_was_digit
                    || previous_was_underscore
                {
                    return Err(LexError::new(
                        LexErrorKind::InvalidNumericSeparator,
                        Span::new(start, self.offset + 1),
                        "numeric separators must occur between digits",
                    ));
                }

                previous_was_underscore = true;
                self.advance_char();
                continue;
            }

            break;
        }

        if digits == 0 || previous_was_underscore {
            return Err(LexError::new(
                LexErrorKind::InvalidNumber,
                Span::new(start, self.offset),
                "based integer contains no valid digit sequence",
            ));
        }

        if digits > self.config.limits.max_numeric_digits {
            return Err(LexError::new(
                LexErrorKind::NumericLiteralTooLong,
                Span::new(start, self.offset),
                "numeric literal exceeds configured digit limit",
            ));
        }

        let _ = digits_start;

        Ok(TokenKind::IntegerLiteral)
    }

    fn lex_decimal_or_float(
        &mut self,
        start: usize,
    ) -> LexerResult<TokenKind> {
        let mut saw_digit = false;
        let mut previous_was_digit = false;
        let mut previous_was_underscore = false;

        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() {
                saw_digit = true;
                previous_was_digit = true;
                previous_was_underscore = false;
                self.advance_char();
                continue;
            }

            if ch == '_' {
                if !previous_was_digit
                    || previous_was_underscore
                {
                    return Err(LexError::new(
                        LexErrorKind::InvalidNumericSeparator,
                        Span::new(start, self.offset + 1),
                        "numeric separators must occur between digits",
                    ));
                }

                previous_was_underscore = true;
                self.advance_char();
                continue;
            }

            break;
        }

        if !saw_digit || previous_was_underscore {
            return Err(LexError::new(
                LexErrorKind::InvalidNumber,
                Span::new(start, self.offset),
                "invalid decimal literal",
            ));
        }

        let mut is_float = false;

        // Fractional part.
        if self.current_char() == Some('.') {
            is_float = true;
            self.advance_char();

            let mut fraction_digits = 0usize;
            let mut previous_digit = false;
            let mut previous_underscore = false;

            while let Some(ch) = self.current_char() {
                if ch.is_ascii_digit() {
                    fraction_digits =
                        fraction_digits.saturating_add(1);
                    previous_digit = true;
                    previous_underscore = false;
                    self.advance_char();
                    continue;
                }

                if ch == '_' {
                    if !previous_digit
                        || previous_underscore
                    {
                        return Err(LexError::new(
                            LexErrorKind::InvalidNumericSeparator,
                            Span::new(start, self.offset + 1),
                            "numeric separators must occur between digits",
                        ));
                    }

                    previous_underscore = true;
                    self.advance_char();
                    continue;
                }

                break;
            }

            if previous_underscore {
                return Err(LexError::new(
                    LexErrorKind::InvalidNumericSeparator,
                    Span::new(start, self.offset),
                    "numeric literal cannot end a fractional part with `_`",
                ));
            }

            // `123.` is explicitly valid according to the OpenQASM grammar.
            let _ = fraction_digits;
        }

        // Exponent.
        if matches!(
            self.current_char(),
            Some('e' | 'E')
        ) {
            is_float = true;
            self.advance_char();

            if matches!(
                self.current_char(),
                Some('+' | '-')
            ) {
                self.advance_char();
            }

            let exponent_start = self.offset;
            let mut exponent_digits = 0usize;
            let mut previous_digit = false;
            let mut previous_underscore = false;

            while let Some(ch) = self.current_char() {
                if ch.is_ascii_digit() {
                    exponent_digits =
                        exponent_digits.saturating_add(1);
                    previous_digit = true;
                    previous_underscore = false;
                    self.advance_char();
                    continue;
                }

                if ch == '_' {
                    if !previous_digit
                        || previous_underscore
                    {
                        return Err(LexError::new(
                            LexErrorKind::InvalidNumericSeparator,
                            Span::new(start, self.offset + 1),
                            "numeric separators must occur between exponent digits",
                        ));
                    }

                    previous_underscore = true;
                    self.advance_char();
                    continue;
                }

                break;
            }

            if exponent_digits == 0
                || previous_underscore
                || self.offset == exponent_start
            {
                return Err(LexError::new(
                    LexErrorKind::InvalidNumber,
                    Span::new(start, self.offset),
                    "exponent requires at least one decimal digit",
                ));
            }
        }

        self.validate_numeric_length(start)?;

        // OpenQASM uses `im`, not `i`.
        if self.source[self.offset..].starts_with("im") {
            let after = self.offset + 2;

            let boundary = self
                .source
                .get(after..)
                .and_then(|s| s.chars().next());

            if boundary.is_none_or(|c| !is_identifier_continue(c)) {
                self.offset = after;
                return Ok(TokenKind::ImaginaryLiteral);
            }
        }

        if self.current_char().is_some_and(is_duration_start) {
            return self.lex_duration(start);
        }

        if is_float {
            Ok(TokenKind::FloatLiteral)
        } else {
            Ok(TokenKind::IntegerLiteral)
        }
    }

    fn lex_duration(
        &mut self,
        start: usize,
    ) -> LexerResult<TokenKind> {
        let suffix_start = self.offset;

        let suffix = if self.source[suffix_start..]
            .starts_with("µs")
        {
            self.offset += "µs".len();
            "µs"
        } else if self.source[suffix_start..]
            .starts_with("dt")
        {
            self.offset += 2;
            "dt"
        } else if self.source[suffix_start..]
            .starts_with("ns")
        {
            self.offset += 2;
            "ns"
        } else if self.source[suffix_start..]
            .starts_with("us")
        {
            self.offset += 2;
            "us"
        } else if self.source[suffix_start..]
            .starts_with("ms")
        {
            self.offset += 2;
            "ms"
        } else if self.current_char() == Some('s') {
            self.advance_char();
            "s"
        } else {
            return Err(LexError::new(
                LexErrorKind::InvalidDuration,
                Span::new(start, self.offset),
                "invalid OpenQASM timing suffix",
            ));
        };

        let _ = suffix;

        self.check_lexeme_size(start, self.offset)?;

        Ok(TokenKind::DurationLiteral)
    }

    fn validate_numeric_length(
        &self,
        start: usize,
    ) -> LexerResult<()> {
        let text = self
            .source
            .get(start..self.offset)
            .ok_or_else(|| {
                LexError::new(
                    LexErrorKind::InvalidSourceBoundary,
                    Span::new(start, self.offset),
                    "numeric literal has invalid source boundaries",
                )
            })?;

        let digits = text
            .chars()
            .filter(char::is_ascii_digit)
            .count();

        if digits > self.config.limits.max_numeric_digits {
            return Err(LexError::new(
                LexErrorKind::NumericLiteralTooLong,
                Span::new(start, self.offset),
                "numeric literal exceeds configured digit limit",
            ));
        }

        self.check_lexeme_size(start, self.offset)
    }

    // =========================================================================
    // Bitstring
    // =========================================================================

    fn lex_bitstring(
        &mut self,
        start: usize,
    ) -> LexerResult<TokenKind> {
        self.advance_char(); // "

        let content_start = self.offset;
        let mut bit_count = 0usize;
        let mut previous_was_bit = false;

        loop {
            let Some(ch) = self.current_char() else {
                return Err(LexError::new(
                    LexErrorKind::UnterminatedString,
                    Span::new(start, self.offset),
                    "unterminated bitstring literal",
                ));
            };

            if ch == '"' {
                if bit_count == 0 {
                    return Err(LexError::new(
                        LexErrorKind::InvalidBitstring,
                        Span::new(start, self.offset + 1),
                        "bitstring literal must contain at least one bit",
                    ));
                }

                if !previous_was_bit {
                    return Err(LexError::new(
                        LexErrorKind::InvalidBitstring,
                        Span::new(start, self.offset),
                        "bitstring literal cannot end with `_`",
                    ));
                }

                self.advance_char();

                let size =
                    self.offset.saturating_sub(start);

                if size
                    > self.config.limits.max_bitstring_bytes
                {
                    return Err(LexError::new(
                        LexErrorKind::BitstringTooLarge,
                        Span::new(start, self.offset),
                        "bitstring exceeds configured maximum size",
                    ));
                }

                return self.make_token(
                    start,
                    TokenKind::BitstringLiteral,
                );
            }

            match ch {
                '0' | '1' => {
                    bit_count = bit_count.saturating_add(1);

                    if bit_count
                        > self.config.limits.max_bitstring_bytes
                    {
                        return Err(LexError::new(
                            LexErrorKind::BitstringTooLarge,
                            Span::new(start, self.offset),
                            "bitstring exceeds configured maximum size",
                        ));
                    }

                    previous_was_bit = true;
                    self.advance_char();
                }

                '_' => {
                    if !previous_was_bit {
                        return Err(LexError::new(
                            LexErrorKind::InvalidBitstring,
                            Span::new(start, self.offset + 1),
                            "bitstring separator must follow a bit",
                        ));
                    }

                    previous_was_bit = false;
                    self.advance_char();
                }

                _ => {
                    return Err(LexError::new(
                        LexErrorKind::InvalidBitstring,
                        Span::new(content_start, self.offset + ch.len_utf8()),
                        "bitstring literal may contain only `0`, `1`, and `_`",
                    ));
                }
            }
        }
    }

    // =========================================================================
    // Operators
    // =========================================================================

    fn lex_dot_or_float(
        &mut self,
        start: usize,
    ) -> LexerResult<TokenKind> {
        if !self
            .peek_char()
            .is_some_and(|c| c.is_ascii_digit())
        {
            self.advance_char();
            return Ok(TokenKind::Dot);
        }

        self.advance_char();

        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() || ch == '_' {
                self.advance_char();
            } else {
                break;
            }
        }

        if matches!(
            self.current_char(),
            Some('e' | 'E')
        ) {
            self.advance_char();

            if matches!(
                self.current_char(),
                Some('+' | '-')
            ) {
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
                    "exponent requires decimal digits",
                ));
            }
        }

        self.validate_numeric_length(start)?;

        if self.source[self.offset..].starts_with("im") {
            self.offset += 2;
            return Ok(TokenKind::ImaginaryLiteral);
        }

        Ok(TokenKind::FloatLiteral)
    }

    fn lex_plus(
        &mut self,
        _start: usize,
    ) -> LexerResult<TokenKind> {
        self.advance_char();

        match self.current_char() {
            Some('+') => {
                self.advance_char();
                Ok(TokenKind::Increment)
            }

            Some('=') => {
                self.advance_char();
                Ok(TokenKind::PlusEqual)
            }

            _ => Ok(TokenKind::Plus),
        }
    }

    fn lex_minus(
        &mut self,
        _start: usize,
    ) -> LexerResult<TokenKind> {
        self.advance_char();

        match self.current_char() {
            Some('-') => {
                self.advance_char();
                Ok(TokenKind::Decrement)
            }

            Some('=') => {
                self.advance_char();
                Ok(TokenKind::MinusEqual)
            }

            Some('>') => {
                self.advance_char();
                Ok(TokenKind::Arrow)
            }

            _ => Ok(TokenKind::Minus),
        }
    }

    fn lex_star(
        &mut self,
        _start: usize,
    ) -> LexerResult<TokenKind> {
        self.advance_char();

        match self.current_char() {
            Some('*') => {
                self.advance_char();

                if self.current_char() == Some('=') {
                    self.advance_char();
                    Ok(TokenKind::PowerEqual)
                } else {
                    Ok(TokenKind::Power)
                }
            }

            Some('=') => {
                self.advance_char();
                Ok(TokenKind::StarEqual)
            }

            _ => Ok(TokenKind::Star),
        }
    }

    fn lex_slash(
        &mut self,
        _start: usize,
    ) -> LexerResult<TokenKind> {
        self.advance_char();

        if self.current_char() == Some('=') {
            self.advance_char();
            Ok(TokenKind::SlashEqual)
        } else {
            Ok(TokenKind::Slash)
        }
    }

    fn lex_percent(
        &mut self,
        _start: usize,
    ) -> LexerResult<TokenKind> {
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

        match self.current_char() {
            Some('&') => {
                self.advance_char();
                Ok(TokenKind::LogicalAnd)
            }

            Some('=') => {
                self.advance_char();
                Ok(TokenKind::AmpersandEqual)
            }

            _ => Ok(TokenKind::BitAnd),
        }
    }

    fn lex_pipe(
        &mut self,
        _start: usize,
    ) -> LexerResult<TokenKind> {
        self.advance_char();

        match self.current_char() {
            Some('|') => {
                self.advance_char();
                Ok(TokenKind::LogicalOr)
            }

            Some('=') => {
                self.advance_char();
                Ok(TokenKind::PipeEqual)
            }

            _ => Ok(TokenKind::BitOr),
        }
    }

    fn lex_caret(
        &mut self,
        _start: usize,
    ) -> LexerResult<TokenKind> {
        self.advance_char();

        if self.current_char() == Some('=') {
            self.advance_char();
            Ok(TokenKind::CaretEqual)
        } else {
            Ok(TokenKind::BitXor)
        }
    }

    fn lex_tilde(
        &mut self,
        _start: usize,
    ) -> LexerResult<TokenKind> {
        self.advance_char();

        if self.current_char() == Some('=') {
            self.advance_char();
            Ok(TokenKind::TildeEqual)
        } else {
            Ok(TokenKind::BitNot)
        }
    }

    fn lex_bang(
        &mut self,
        _start: usize,
    ) -> LexerResult<TokenKind> {
        self.advance_char();

        if self.current_char() == Some('=') {
            self.advance_char();
            Ok(TokenKind::NotEqual)
        } else {
            Ok(TokenKind::LogicalNot)
        }
    }

    fn lex_equal(
        &mut self,
        _start: usize,
    ) -> LexerResult<TokenKind> {
        self.advance_char();

        if self.current_char() == Some('=') {
            self.advance_char();
            Ok(TokenKind::EqualEqual)
        } else {
            Ok(TokenKind::Equal)
        }
    }

    fn lex_less(
        &mut self,
        _start: usize,
    ) -> LexerResult<TokenKind> {
        self.advance_char();

        match self.current_char() {
            Some('=') => {
                self.advance_char();
                Ok(TokenKind::LessEqual)
            }

            Some('<') => {
                self.advance_char();

                if self.current_char() == Some('=') {
                    self.advance_char();
                    Ok(TokenKind::ShiftLeftEqual)
                } else {
                    Ok(TokenKind::ShiftLeft)
                }
            }

            _ => Ok(TokenKind::Less),
        }
    }

    fn lex_greater(
        &mut self,
        _start: usize,
    ) -> LexerResult<TokenKind> {
        self.advance_char();

        match self.current_char() {
            Some('=') => {
                self.advance_char();
                Ok(TokenKind::GreaterEqual)
            }

            Some('>') => {
                self.advance_char();

                if self.current_char() == Some('=') {
                    self.advance_char();
                    Ok(TokenKind::ShiftRightEqual)
                } else {
                    Ok(TokenKind::ShiftRight)
                }
            }

            _ => Ok(TokenKind::Greater),
        }
    }

    // =========================================================================
    // Calibration block
    // =========================================================================

    fn lex_calibration_block(
        &mut self,
    ) -> LexerResult<Token<'src>> {
        let start = self.offset;

        let mut depth = 0usize;
        let mut in_string: Option<char> = None;
        let mut escaped = false;

        while let Some(ch) = self.current_char() {
            if let Some(quote) = in_string {
                if escaped {
                    escaped = false;
                    self.advance_char();
                    continue;
                }

                if ch == '\\' {
                    escaped = true;
                    self.advance_char();
                    continue;
                }

                if ch == quote {
                    in_string = None;
                }

                self.advance_char();
                continue;
            }

            if ch == '"' || ch == '\'' {
                in_string = Some(ch);
                self.advance_char();
                continue;
            }

            if ch == '/'
                && self.peek_char() == Some('/')
            {
                self.skip_line_comment()?;
                continue;
            }

            if ch == '/'
                && self.peek_char() == Some('*')
            {
                self.skip_block_comment()?;
                continue;
            }

            match ch {
                '{' => {
                    depth = depth
                        .checked_add(1)
                        .ok_or_else(|| {
                            LexError::new(
                                LexErrorKind::CalibrationNestingTooDeep,
                                Span::new(start, self.offset),
                                "calibration nesting counter overflow",
                            )
                        })?;

                    if depth
                        > self.config.limits.max_calibration_nesting
                    {
                        return Err(LexError::new(
                            LexErrorKind::CalibrationNestingTooDeep,
                            Span::new(start, self.offset),
                            "calibration nesting exceeds configured maximum",
                        ));
                    }

                    self.advance_char();
                }

                '}' => {
                    if depth == 0 {
                        break;
                    }

                    depth -= 1;
                    self.advance_char();

                    if depth == 0 {
                        self.mode = LexerMode::Default;

                        return self.make_token(
                            start,
                            TokenKind::CalibrationBlock,
                        );
                    }
                }

                _ => {
                    self.advance_char();
                }
            }

            if self.offset.saturating_sub(start)
                > self.config.limits.max_calibration_bytes
            {
                return Err(LexError::new(
                    LexErrorKind::CalibrationTooLarge,
                    Span::new(start, self.offset),
                    "calibration block exceeds configured maximum size",
                ));
            }
        }

        Err(LexError::new(
            LexErrorKind::UnterminatedCalibrationBlock,
            Span::new(start, self.offset),
            "unterminated calibration block",
        ))
    }

    // =========================================================================
    // Token creation / limits
    // =========================================================================

    fn make_token(
        &mut self,
        start: usize,
        kind: TokenKind,
    ) -> LexerResult<Token<'src>> {
        let end = self.offset;

        self.check_lexeme_size(start, end)?;

        let lexeme = self
            .source
            .get(start..end)
            .ok_or_else(|| {
                LexError::new(
                    LexErrorKind::InvalidSourceBoundary,
                    Span::new(start, end),
                    "token boundaries do not align with UTF-8",
                )
            })?;

        self.token_count =
            self.token_count.checked_add(1).ok_or_else(|| {
                LexError::new(
                    LexErrorKind::TokenLimitExceeded,
                    Span::new(start, end),
                    "token counter overflow",
                )
            })?;

        if self.token_count
            > self.config.limits.max_tokens
        {
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
            Span::new(
                self.source.len(),
                self.source.len(),
            ),
            "",
        ))
    }

    fn ensure_token_capacity(&self) -> LexerResult<()> {
        if self.token_count
            >= self.config.limits.max_tokens
        {
            return Err(LexError::new(
                LexErrorKind::TokenLimitExceeded,
                Span::new(self.offset, self.offset),
                "configured token limit has been reached",
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

        if size
            > self.config.limits.max_lexeme_bytes
        {
            return Err(LexError::new(
                LexErrorKind::LexemeTooLarge,
                Span::new(start, end),
                "lexeme exceeds configured maximum size",
            ));
        }

        Ok(())
    }

    fn check_directive_size(
        &self,
        start: usize,
    ) -> LexerResult<()> {
        if self.offset.saturating_sub(start)
            > self.config.limits.max_directive_bytes
        {
            return Err(LexError::new(
                LexErrorKind::DirectiveTooLarge,
                Span::new(start, self.offset),
                "directive exceeds configured maximum size",
            ));
        }

        Ok(())
    }

    // =========================================================================
    // Source traversal
    // =========================================================================

    fn current_char(&self) -> Option<char> {
        self.source
            .get(self.offset..)?
            .chars()
            .next()
    }

    fn peek_char(&self) -> Option<char> {
        let current = self.current_char()?;

        let next = self
            .offset
            .checked_add(current.len_utf8())?;

        self.source
            .get(next..)?
            .chars()
            .next()
    }

    fn advance_char(&mut self) {
        if let Some(ch) = self.current_char() {
            self.offset += ch.len_utf8();
        }
    }

    fn skip_horizontal_and_newline_whitespace(
        &mut self,
    ) -> LexerResult<()> {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.advance_char();
            } else {
                break;
            }
        }

        Ok(())
    }

    fn skip_newline(&mut self) {
        if self.current_char() == Some('\r') {
            self.advance_char();

            if self.current_char() == Some('\n') {
                self.advance_char();
            }
        } else if self.current_char() == Some('\n') {
            self.advance_char();
        }
    }
}

// =============================================================================
// Lexical helpers
// =============================================================================

/// OpenQASM identifier start.
///
/// The specification permits `_`, ASCII letters, and Unicode letters in the
/// Unicode categories used by its `ValidUnicode` fragment.
fn is_identifier_start(ch: char) -> bool {
    ch == '_'
        || ch.is_alphabetic()
        || is_unicode_number_letter(ch)
}

/// OpenQASM identifier continuation.
fn is_identifier_continue(ch: char) -> bool {
    is_identifier_start(ch) || ch.is_ascii_digit()
}

/// Approximation of Unicode `Nl` identifier characters using stable standard
/// library ranges. The major relevant ranges are included without pulling in
/// an additional Unicode-category dependency.
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

fn is_duration_start(ch: char) -> bool {
    matches!(
        ch,
        'd' | 'n' | 'u' | 'm' | 's' | 'µ'
    )
}

fn is_valid_version(value: &str) -> bool {
    let mut parts = value.split('.');

    let Some(major) = parts.next() else {
        return false;
    };

    if major.is_empty()
        || !major.chars().all(|c| c.is_ascii_digit())
    {
        return false;
    }

    match parts.next() {
        Some(minor) => {
            !minor.is_empty()
                && minor.chars().all(|c| c.is_ascii_digit())
                && parts.next().is_none()
        }

        None => true,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(source: &str) -> Vec<Token<'_>> {
        OpenQasmLexer::new(source)
            .expect("lexer construction")
            .tokenize()
            .expect("lexing")
    }

    fn kinds(source: &str) -> Vec<TokenKind> {
        lex(source)
            .into_iter()
            .map(Token::kind)
            .collect()
    }

    #[test]
    fn empty_source() {
        assert_eq!(
            kinds(""),
            vec![TokenKind::Eof]
        );
    }

    #[test]
    fn openqasm_version_is_one_token() {
        let tokens = lex("OPENQASM 3.0;");

        assert_eq!(
            tokens[0].kind(),
            TokenKind::KwOpenQasm
        );

        assert_eq!(
            tokens[1].kind(),
            TokenKind::IntegerLiteral
        );

        assert_eq!(
            tokens[1].lexeme(),
            "3.0"
        );
    }

    #[test]
    fn rejects_invalid_version() {
        let result =
            OpenQasmLexer::new("OPENQASM 3.;")
                .and_then(OpenQasmLexer::tokenize);

        assert!(matches!(
            result,
            Err(LexError {
                kind: LexErrorKind::InvalidVersion,
                ..
            })
        ));
    }

    #[test]
    fn include_uses_arbitrary_string_mode() {
        let tokens =
            lex(r#"include "stdgates.inc";"#);

        assert_eq!(
            tokens[0].kind(),
            TokenKind::KwInclude
        );

        assert_eq!(
            tokens[1].kind(),
            TokenKind::StringLiteral
        );

        assert_eq!(
            tokens[1].lexeme(),
            r#""stdgates.inc""#
        );
    }

    #[test]
    fn include_accepts_single_quotes() {
        let tokens =
            lex("include 'stdgates.inc';");

        assert_eq!(
            tokens[1].kind(),
            TokenKind::StringLiteral
        );
    }

    #[test]
    fn default_quotes_are_bitstrings() {
        let tokens = lex(r#""0101_0011""#);

        assert_eq!(
            tokens[0].kind(),
            TokenKind::BitstringLiteral
        );
    }

    #[test]
    fn rejects_invalid_bitstring() {
        let result =
            OpenQasmLexer::new(r#""012""#)
                .and_then(OpenQasmLexer::tokenize);

        assert!(matches!(
            result,
            Err(LexError {
                kind: LexErrorKind::InvalidBitstring,
                ..
            })
        ));
    }

    #[test]
    fn identifiers_are_case_sensitive() {
        assert_eq!(
            kinds("qubit QUBIT"),
            vec![
                TokenKind::KwQubit,
                TokenKind::Identifier,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn unicode_identifier() {
        let tokens = lex("αβγ");

        assert_eq!(
            tokens[0].kind(),
            TokenKind::Identifier
        );

        assert_eq!(
            tokens[0].lexeme(),
            "αβγ"
        );
    }

    #[test]
    fn hardware_qubits() {
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
    fn rejects_bare_hardware_qubit() {
        let result =
            OpenQasmLexer::new("$")
                .and_then(OpenQasmLexer::tokenize);

        assert!(matches!(
            result,
            Err(LexError {
                kind: LexErrorKind::InvalidHardwareQubit,
                ..
            })
        ));
    }

    #[test]
    fn integer_bases() {
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
    fn floats() {
        assert_eq!(
            kinds("1.0 123. .25 1e3 1.5e-2"),
            vec![
                TokenKind::FloatLiteral,
                TokenKind::FloatLiteral,
                TokenKind::FloatLiteral,
                TokenKind::FloatLiteral,
                TokenKind::FloatLiteral,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn imaginary_uses_im_suffix() {
        assert_eq!(
            kinds("1im 2.5im"),
            vec![
                TokenKind::ImaginaryLiteral,
                TokenKind::ImaginaryLiteral,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn bare_i_is_not_imaginary() {
        assert_eq!(
            kinds("1i"),
            vec![
                TokenKind::IntegerLiteral,
                TokenKind::Identifier,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn timing_units() {
        assert_eq!(
            kinds("10dt 2ns 3us 4µs 5ms 6s"),
            vec![
                TokenKind::DurationLiteral,
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
    fn operators() {
        assert_eq!(
            kinds(
                "+ ++ += - -- -= -> \
                 * ** *= **= / /= % %= \
                 & &= | |= ^ ^= ~ ~= \
                 << <<= >> >>= \
                 = == ! != < <= > >="
            ),
            vec![
                TokenKind::Plus,
                TokenKind::Increment,
                TokenKind::PlusEqual,
                TokenKind::Minus,
                TokenKind::Decrement,
                TokenKind::MinusEqual,
                TokenKind::Arrow,
                TokenKind::Star,
                TokenKind::Power,
                TokenKind::StarEqual,
                TokenKind::PowerEqual,
                TokenKind::Slash,
                TokenKind::SlashEqual,
                TokenKind::Percent,
                TokenKind::PercentEqual,
                TokenKind::BitAnd,
                TokenKind::AmpersandEqual,
                TokenKind::BitOr,
                TokenKind::PipeEqual,
                TokenKind::BitXor,
                TokenKind::CaretEqual,
                TokenKind::BitNot,
                TokenKind::TildeEqual,
                TokenKind::ShiftLeft,
                TokenKind::ShiftLeftEqual,
                TokenKind::ShiftRight,
                TokenKind::ShiftRightEqual,
                TokenKind::Equal,
                TokenKind::EqualEqual,
                TokenKind::LogicalNot,
                TokenKind::NotEqual,
                TokenKind::Less,
                TokenKind::LessEqual,
                TokenKind::Greater,
                TokenKind::GreaterEqual,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn comments_are_skipped() {
        assert_eq!(
            kinds(
                "// comment\n\
                 /* block */\n\
                 qubit q;"
            ),
            vec![
                TokenKind::KwQubit,
                TokenKind::Identifier,
                TokenKind::Semicolon,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn unterminated_block_comment() {
        let result =
            OpenQasmLexer::new("/*")
                .and_then(OpenQasmLexer::tokenize);

        assert!(matches!(
            result,
            Err(LexError {
                kind: LexErrorKind::UnterminatedComment,
                ..
            })
        ));
    }

    #[test]
    fn annotation_and_remaining_line() {
        let tokens =
            lex("@foo.bar hello world\nqubit q;");

        assert_eq!(
            tokens[0].kind(),
            TokenKind::Annotation
        );

        assert_eq!(
            tokens[1].kind(),
            TokenKind::RemainingLineContent
        );

        assert_eq!(
            tokens[1].lexeme(),
            "hello world"
        );

        assert_eq!(
            tokens[2].kind(),
            TokenKind::KwQubit
        );
    }

    #[test]
    fn pragma_and_remaining_line() {
        let tokens =
            lex("#pragma optimize foo\nqubit q;");

        assert_eq!(
            tokens[0].kind(),
            TokenKind::Pragma
        );

        assert_eq!(
            tokens[1].kind(),
            TokenKind::RemainingLineContent
        );

        assert_eq!(
            tokens[1].lexeme(),
            "optimize foo"
        );
    }

    #[test]
    fn hash_dim() {
        assert_eq!(
            kinds("#dim"),
            vec![
                TokenKind::KwDim,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn declarations() {
        assert_eq!(
            kinds(
                "input output const readonly mutable \
                 qreg qubit creg bit bool int uint float \
                 angle complex array void duration stretch"
            ),
            vec![
                TokenKind::KwInput,
                TokenKind::KwOutput,
                TokenKind::KwConst,
                TokenKind::KwReadonly,
                TokenKind::KwMutable,
                TokenKind::KwQreg,
                TokenKind::KwQubit,
                TokenKind::KwCreg,
                TokenKind::KwBit,
                TokenKind::KwBool,
                TokenKind::KwInt,
                TokenKind::KwUInt,
                TokenKind::KwFloat,
                TokenKind::KwAngle,
                TokenKind::KwComplex,
                TokenKind::KwArray,
                TokenKind::KwVoid,
                TokenKind::KwDuration,
                TokenKind::KwStretch,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn control_flow_keywords() {
        assert_eq!(
            kinds(
                "break continue if else end return \
                 for while in switch case default"
            ),
            vec![
                TokenKind::KwBreak,
                TokenKind::KwContinue,
                TokenKind::KwIf,
                TokenKind::KwElse,
                TokenKind::KwEnd,
                TokenKind::KwReturn,
                TokenKind::KwFor,
                TokenKind::KwWhile,
                TokenKind::KwIn,
                TokenKind::KwSwitch,
                TokenKind::KwCase,
                TokenKind::KwDefault,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn malformed_numeric_separator_is_rejected() {
        let result =
            OpenQasmLexer::new("1__2")
                .and_then(OpenQasmLexer::tokenize);

        assert!(matches!(
            result,
            Err(LexError {
                kind: LexErrorKind::InvalidNumericSeparator,
                ..
            })
        ));
    }

    #[test]
    fn trailing_numeric_separator_is_rejected() {
        let result =
            OpenQasmLexer::new("123_")
                .and_then(OpenQasmLexer::tokenize);

        assert!(matches!(
            result,
            Err(LexError {
                kind: LexErrorKind::InvalidNumericSeparator,
                ..
            })
        ));
    }

    #[test]
    fn source_limit_is_enforced() {
        let config = LexerConfig {
            limits: LexerLimits {
                max_source_bytes: 2,
                ..LexerLimits::default()
            },
            ..LexerConfig::default()
        };

        let result =
            OpenQasmLexer::with_config("123", config);

        assert!(matches!(
            result,
            Err(LexError {
                kind: LexErrorKind::SourceTooLarge,
                ..
            })
        ));
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

        let result =
            OpenQasmLexer::with_config(
                "qubit q;",
                config,
            )
            .and_then(OpenQasmLexer::tokenize);

        assert!(matches!(
            result,
            Err(LexError {
                kind: LexErrorKind::TokenLimitExceeded,
                ..
            })
        ));
    }

    #[test]
    fn token_spans_are_byte_accurate() {
        let tokens = lex("α q;");

        assert_eq!(
            tokens[0].span(),
            Span::new(0, 2)
        );

        assert_eq!(
            tokens[0].lexeme(),
            "α"
        );

        assert_eq!(
            tokens[1].span(),
            Span::new(3, 4)
        );
    }

    #[test]
    fn eof_span_is_source_end() {
        let source = "qubit q;";
        let tokens = lex(source);
        let eof = tokens.last().expect("EOF");

        assert_eq!(
            eof.span(),
            Span::new(
                source.len(),
                source.len()
            )
        );
    }

    #[test]
    fn lexer_never_needs_filesystem_or_network() {
        // Architectural test: construction/tokenization requires only source
        // bytes and configuration.
        let result =
            OpenQasmLexer::new(
                "OPENQASM 3.0;\nqubit q;"
            )
            .and_then(OpenQasmLexer::tokenize);

        assert!(result.is_ok());
    }
}