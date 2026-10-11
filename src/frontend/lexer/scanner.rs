
#![forbid(unsafe_code)]

//! Streaming lexical scanner for the Zamani frontend.
//!
//! Rust baseline: 1.97 or later
//! Edition: 2021
//!
//! # Responsibilities
//!
//! - Coordinate the existing lexical recognizers.
//! - Apply source, token, literal, and numeric-digit budgets.
//! - Classify identifiers through the keyword registry.
//! - Recognize registered operators using maximal munch.
//! - Preserve borrowed source spellings and UTF-8-safe source positions.
//! - Apply the configured trivia and EOF policies.
//! - Provide a streaming implementation of `TokenSource`.
//!
//! # Architectural boundary
//!
//! This module converts source text into tokens. It does not parse expressions,
//! interpret quantum operations, choose hardware targets, or lower code to IR.
//! New quantum operations, vendor names, and domain-specific identifiers must
//! not require changes to this scanner when the registries can represent them.
//!
//! # Integration dependencies
//!
//! This module uses:
//! - `api.rs`: `TokenSource`;
//! - `config.rs`: `LexerConfig`, `ResourceLimit`, trivia and EOF policies;
//! - `cursor.rs`: UTF-8-safe source traversal and checkpoints;
//! - `token.rs` and `token_kind.rs`: canonical token representation;
//! - `keyword_registry.rs` and `operator_registry.rs`: lexical classification;
//! - `identifier.rs`, `number.rs`, `string.rs`, `character.rs`: recognizers.
//!
//! The canonical `Span` API must provide a constructor that accepts start and
//! end `CursorPosition` values. This implementation uses `Span::new(start, end)`.
//! If the final `span.rs` contract differs, align that single boundary rather
//! than duplicating coordinate logic here.
//!
//! Comment and whitespace recognition are kept small and source-oriented here
//! until their dedicated recognizer modules are available on the same branch.
//! Their rules must remain aligned with `grammar/spec/lexical.md` and the
//! canonical component grammars.

use core::fmt;
use std::error::Error;

use super::api::TokenSource;
use super::character::{scan_character_with_limit, CharacterError};
use super::config::{
    EofPolicy, LexerConfig, RecoveryPolicy, ResourceLimit, TriviaPolicy,
};
use super::cursor::{Cursor, CursorPosition};
use super::identifier::scan_identifier;
use super::keyword_registry::{KeywordLookup, KeywordRegistry};
use super::number::{scan_number_with_limit, NumberError};
use super::operator_registry::OperatorRegistry;
use super::span::Span;
use super::string::{scan_string_with_limit, StringError};
use super::token::{Token, TokenChannel};
use super::token_kind::TokenKind;

/// The kind of failure encountered by the scanner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ScannerErrorKind {
    /// The entire source exceeds the configured source-byte budget.
    SourceBudgetExceeded,

    /// The emitted-token budget has been exhausted.
    TokenBudgetExceeded,

    /// An individual token exceeds the configured byte budget.
    TokenBudgetForLexemeExceeded,

    /// A literal or numeric recognizer rejected the source.
    InvalidLiteral,

    /// A block comment reached EOF before its closing delimiter.
    UnterminatedBlockComment,

    /// The source begins with a prohibited byte-order mark.
    InitialBomNotAllowed,

    /// A counter could not represent the next value.
    CounterOverflow,

    /// An internal scanner invariant was violated.
    InternalInvariant,
}

impl fmt::Display for ScannerErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceBudgetExceeded => {
                f.write_str("source exceeds the configured byte budget")
            }
            Self::TokenBudgetExceeded => {
                f.write_str("token stream exceeds the configured token budget")
            }
            Self::TokenBudgetForLexemeExceeded => {
                f.write_str("token exceeds the configured byte budget")
            }
            Self::InvalidLiteral => f.write_str("invalid literal"),
            Self::UnterminatedBlockComment => {
                f.write_str("unterminated block comment")
            }
            Self::InitialBomNotAllowed => {
                f.write_str("initial byte-order mark is not allowed")
            }
            Self::CounterOverflow => {
                f.write_str("scanner counter overflow")
            }
            Self::InternalInvariant => {
                f.write_str("scanner internal invariant was violated")
            }
        }
    }
}

/// Structured scanner failure.
///
/// Byte offsets are zero-based, UTF-8 byte offsets into the original source.
/// The source range is half-open. The message is intentionally separate from
/// `ScannerErrorKind` so callers can choose their own diagnostic presentation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScannerError {
    kind: ScannerErrorKind,
    start: CursorPosition,
    end: CursorPosition,
    message: String,
}

impl ScannerError {
    fn new(
        kind: ScannerErrorKind,
        start: CursorPosition,
        end: CursorPosition,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            start,
            end,
            message: message.into(),
        }
    }

    /// Returns the stable error category.
    #[must_use]
    pub const fn kind(&self) -> ScannerErrorKind {
        self.kind
    }

    /// Returns the beginning of the source range.
    #[must_use]
    pub const fn start(&self) -> CursorPosition {
        self.start
    }

    /// Returns the end of the source range.
    #[must_use]
    pub const fn end(&self) -> CursorPosition {
        self.end
    }

    /// Returns the half-open UTF-8 byte range.
    #[must_use]
    pub fn byte_range(&self) -> core::ops::Range<usize> {
        self.start.byte_offset()..self.end.byte_offset()
    }

    /// Returns the human-readable error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at {}: {}",
            self.kind, self.start, self.message
        )
    }
}

impl Error for ScannerError {}

impl From<NumberError> for ScannerError {
    fn from(error: NumberError) -> Self {
        Self::new(
            ScannerErrorKind::InvalidLiteral,
            error.start(),
            error.end(),
            error.to_string(),
        )
    }
}

impl From<StringError> for ScannerError {
    fn from(error: StringError) -> Self {
        Self::new(
            ScannerErrorKind::InvalidLiteral,
            error.start(),
            error.end(),
            error.to_string(),
        )
    }
}

impl From<CharacterError> for ScannerError {
    fn from(error: CharacterError) -> Self {
        Self::new(
            ScannerErrorKind::InvalidLiteral,
            error.start(),
            error.end(),
            error.to_string(),
        )
    }
}

/// Stateful streaming scanner over borrowed source text.
///
/// Tokens borrow their lexemes from `source`; ordinary scanning does not
/// allocate a new string for every token. The keyword and operator registries
/// are supplied by the caller so dialect resolution remains external.
pub struct Scanner<'src> {
    cursor: Cursor<'src>,
    config: LexerConfig,
    keywords: KeywordRegistry,
    operators: OperatorRegistry,
    eof_emitted: bool,
    emitted_tokens: usize,
    initialized: bool,
}

impl<'src> Scanner<'src> {
    /// Constructs a scanner using explicit configuration and registries.
    ///
    /// Configuration-level invariants are validated before scanning starts.
    /// Source-dependent limits are checked when the first token is requested.
    pub fn new(
        source: &'src str,
        config: LexerConfig,
        keywords: KeywordRegistry,
        operators: OperatorRegistry,
    ) -> Result<Self, ScannerError> {
        config.validate().map_err(|error| {
            ScannerError::new(
                ScannerErrorKind::InternalInvariant,
                CursorPosition::default(),
                CursorPosition::default(),
                error.to_string(),
            )
        })?;

        keywords.validate().map_err(|error| {
            ScannerError::new(
                ScannerErrorKind::InternalInvariant,
                CursorPosition::default(),
                CursorPosition::default(),
                error.to_string(),
            )
        })?;

        operators.validate().map_err(|error| {
            ScannerError::new(
                ScannerErrorKind::InternalInvariant,
                CursorPosition::default(),
                CursorPosition::default(),
                error.to_string(),
            )
        })?;

        Ok(Self {
            cursor: Cursor::new(source),
            config,
            keywords,
            operators,
            eof_emitted: false,
            emitted_tokens: 0,
            initialized: false,
        })
    }

    /// Returns the current source position.
    #[must_use]
    pub const fn position(&self) -> CursorPosition {
        self.cursor.position()
    }

    /// Returns the immutable scanner configuration.
    #[must_use]
    pub const fn config(&self) -> &LexerConfig {
        &self.config
    }

    /// Returns the number of tokens emitted so far.
    ///
    /// This counts emitted tokens, including EOF when configured to emit it.
    #[must_use]
    pub const fn emitted_tokens(&self) -> usize {
        self.emitted_tokens
    }

    /// Returns whether the explicit EOF token has been emitted.
    #[must_use]
    pub const fn eof_emitted(&self) -> bool {
        self.eof_emitted
    }

    /// Returns the original source text.
    #[must_use]
    pub const fn source(&self) -> &'src str {
        self.cursor.source()
    }

    fn initialize(&mut self) -> Result<(), ScannerError> {
        if self.initialized {
            return Ok(());
        }

        self.initialized = true;

        let source_len = self.cursor.source_len_bytes();
        if !self.config.limits.source_bytes.permits(source_len) {
            let start = CursorPosition::default();
            let end = self.cursor.position();
            return Err(ScannerError::new(
                ScannerErrorKind::SourceBudgetExceeded,
                start,
                end,
                format!(
                    "source has {source_len} bytes; configured budget is {}",
                    self.config.limits.source_bytes
                ),
            ));
        }

        if self.cursor.starts_with("\u{FEFF}") {
            if !self.config.allow_initial_bom {
                let start = self.cursor.position();
                let _ = self.cursor.consume();
                let end = self.cursor.position();
                return Err(ScannerError::new(
                    ScannerErrorKind::InitialBomNotAllowed,
                    start,
                    end,
                    "the source starts with U+FEFF",
                ));
            }

            // The accepted initial BOM is source encoding metadata, not a
            // language token. Cursor movement preserves source coordinates.
            let _ = self.cursor.consume();
        }

        Ok(())
    }

    fn check_token_budget(&self) -> Result<(), ScannerError> {
        if self.config.limits.tokens.permits(self.emitted_tokens) {
            Ok(())
        } else {
            let position = self.cursor.position();
            Err(ScannerError::new(
                ScannerErrorKind::TokenBudgetExceeded,
                position,
                position,
                format!(
                    "token budget {} exhausted after {} tokens",
                    self.config.limits.tokens, self.emitted_tokens
                ),
            ))
        }
    }

    fn check_lexeme_budget(
        &self,
        start: CursorPosition,
        end: CursorPosition,
    ) -> Result<(), ScannerError> {
        let bytes = end
            .byte_offset()
            .checked_sub(start.byte_offset())
            .ok_or_else(|| {
                ScannerError::new(
                    ScannerErrorKind::CounterOverflow,
                    start,
                    end,
                    "end offset precedes start offset",
                )
            })?;

        if self.config.limits.token_bytes.permits(bytes) {
            Ok(())
        } else {
            Err(ScannerError::new(
                ScannerErrorKind::TokenBudgetForLexemeExceeded,
                start,
                end,
                format!(
                    "token has {bytes} bytes; configured budget is {}",
                    self.config.limits.token_bytes
                ),
            ))
        }
    }

    fn make_token(
        &mut self,
        kind: TokenKind,
        start: CursorPosition,
        end: CursorPosition,
        channel: TokenChannel,
    ) -> Result<Token<'src>, ScannerError> {
        self.check_lexeme_budget(start, end)?;

        let range = start.byte_offset()..end.byte_offset();
        let lexeme = self.cursor.slice(range).map_err(|error| {
            ScannerError::new(
                ScannerErrorKind::InternalInvariant,
                start,
                end,
                error.to_string(),
            )
        })?;

        self.check_token_budget()?;

        let span = Span::new(start, end);
        let token = Token::borrowed(kind, lexeme, span).with_channel(channel);

        self.emitted_tokens = self.emitted_tokens.checked_add(1).ok_or_else(|| {
            ScannerError::new(
                ScannerErrorKind::CounterOverflow,
                start,
                end,
                "emitted token counter overflowed",
            )
        })?;

        Ok(token)
    }

    fn emit_eof(&mut self) -> Result<Option<Token<'src>>, ScannerError> {
        if self.eof_emitted || self.config.eof == EofPolicy::Omit {
            return Ok(None);
        }

        self.check_token_budget()?;

        let position = self.cursor.position();
        let token = Token::synthetic(TokenKind::Eof, Span::new(position, position));

        self.emitted_tokens = self.emitted_tokens.checked_add(1).ok_or_else(|| {
            ScannerError::new(
                ScannerErrorKind::CounterOverflow,
                position,
                position,
                "emitted token counter overflowed",
            )
        })?;

        self.eof_emitted = true;
        Ok(Some(token))
    }

    fn skip_whitespace(&mut self) {
        while self.cursor.peek().is_some_and(char::is_whitespace) {
            let _ = self.cursor.consume();
        }
    }

    fn scan_line_comment(&mut self) -> Result<Token<'src>, ScannerError> {
        let start = self.cursor.position();
        let _ = self.cursor.consume_str("//");

        while let Some(ch) = self.cursor.peek() {
            if matches!(ch, '\r' | '\n' | '\u{2028}' | '\u{2029}') {
                break;
            }
            let _ = self.cursor.consume();
        }

        let end = self.cursor.position();
        self.make_token(
            TokenKind::LineComment,
            start,
            end,
            TokenChannel::Trivia,
        )
    }

    fn scan_block_comment(&mut self) -> Result<Token<'src>, ScannerError> {
        let start = self.cursor.position();
        let _ = self.cursor.consume_str("/*");
        let mut depth: usize = 1;

        while depth > 0 {
            if self.cursor.is_eof() {
                let end = self.cursor.position();
                return Err(ScannerError::new(
                    ScannerErrorKind::UnterminatedBlockComment,
                    start,
                    end,
                    "block comment is missing its closing delimiter",
                ));
            }

            if self.cursor.starts_with("/*") {
                let next_depth = depth.checked_add(1).ok_or_else(|| {
                    ScannerError::new(
                        ScannerErrorKind::CounterOverflow,
                        start,
                        self.cursor.position(),
                        "nested comment depth overflowed",
                    )
                })?;

                if self
                    .config
                    .limits
                    .lexical_nesting_depth
                    .permits(next_depth)
                {
                    depth = next_depth;
                    let _ = self.cursor.consume_str("/*");
                } else {
                    return Err(ScannerError::new(
                        ScannerErrorKind::TokenBudgetForLexemeExceeded,
                        start,
                        self.cursor.position(),
                        "nested comment depth exceeds the configured lexical budget",
                    ));
                }
            } else if self.cursor.starts_with("*/") {
                depth -= 1;
                let _ = self.cursor.consume_str("*/");
            } else {
                let _ = self.cursor.consume();
            }
        }

        let end = self.cursor.position();
        self.make_token(
            TokenKind::BlockComment,
            start,
            end,
            TokenChannel::Trivia,
        )
    }

    fn scan_one(&mut self) -> Result<Option<Token<'src>>, ScannerError> {
        loop {
            if self.cursor.is_eof() {
                return self.emit_eof();
            }

            if self.cursor.starts_with("//") {
                let token = self.scan_line_comment()?;
                if self.config.trivia == TriviaPolicy::Preserve {
                    return Ok(Some(token));
                }
                continue;
            }

            if self.cursor.starts_with("/*") {
                let token = self.scan_block_comment()?;
                if self.config.trivia == TriviaPolicy::Preserve {
                    return Ok(Some(token));
                }
                continue;
            }

            if self.cursor.peek().is_some_and(char::is_whitespace) {
                let start = self.cursor.position();
                self.skip_whitespace();
                let end = self.cursor.position();

                if self.config.trivia == TriviaPolicy::Preserve {
                    return self
                        .make_token(
                            TokenKind::Whitespace,
                            start,
                            end,
                            TokenChannel::Trivia,
                        )
                        .map(Some);
                }

                continue;
            }

            break;
        }

        let start = self.cursor.position();
        let ch = self.cursor.peek().ok_or_else(|| {
            ScannerError::new(
                ScannerErrorKind::InternalInvariant,
                start,
                start,
                "scanner expected input after trivia processing",
            )
        })?;

        // Literal recognizers run before numeric, identifier, and operator
        // classification. Each recognizer preserves source spelling.
        if ch == '"' {
            let literal = scan_string_with_limit(
                &mut self.cursor,
                self.config.limits.literal_bytes,
            )?
            .ok_or_else(|| {
                ScannerError::new(
                    ScannerErrorKind::InternalInvariant,
                    start,
                    self.cursor.position(),
                    "string recognizer declined a double quote",
                )
            })?;

            return self
                .make_token(
                    TokenKind::StringLiteral,
                    literal.start(),
                    literal.end(),
                    TokenChannel::Default,
                )
                .map(Some);
        }

        if ch == '\'' {
            let literal = scan_character_with_limit(
                &mut self.cursor,
                self.config.limits.literal_bytes,
            )?
            .ok_or_else(|| {
                ScannerError::new(
                    ScannerErrorKind::InternalInvariant,
                    start,
                    self.cursor.position(),
                    "character recognizer declined an apostrophe",
                )
            })?;

            return self
                .make_token(
                    TokenKind::CharacterLiteral,
                    literal.start(),
                    literal.end(),
                    TokenChannel::Default,
                )
                .map(Some);
        }

        if ch.is_ascii_digit()
            || (ch == '.'
                && self
                    .cursor
                    .peek_n(1)
                    .is_some_and(|next| next.is_ascii_digit()))
        {
            let number = scan_number_with_limit(
                &mut self.cursor,
                self.config.limits.numeric_digits,
            )?
            .ok_or_else(|| {
                ScannerError::new(
                    ScannerErrorKind::InternalInvariant,
                    start,
                    self.cursor.position(),
                    "numeric recognizer declined a numeric prefix",
                )
            })?;

            let kind = match number.kind() {
                super::number::NumberKind::Integer => TokenKind::IntegerLiteral,
                super::number::NumberKind::Float => TokenKind::FloatLiteral,
                _ => {
                    return Err(ScannerError::new(
                        ScannerErrorKind::InternalInvariant,
                        number.start(),
                        number.end(),
                        "unrecognized numeric literal category",
                    ));
                }
            };

            return self
                .make_token(kind, number.start(), number.end(), TokenChannel::Default)
                .map(Some);
        }

        if let Some(identifier) = scan_identifier(&mut self.cursor) {
            let spelling = identifier.spelling();
            let end = identifier.end();

            let kind = self
                .keywords
                .lookup(
                    spelling,
                    KeywordLookup {
                        include_contextual: false,
                    },
                )
                .map(|definition| definition.kind())
                .unwrap_or(TokenKind::Identifier);

            return self
                .make_token(kind, start, end, TokenChannel::Default)
                .map(Some);
        }

        if let Some(operator) = self.operators.match_prefix(self.cursor.remaining()) {
            let end_offset = start
                .byte_offset()
                .checked_add(operator.byte_len())
                .ok_or_else(|| {
                    ScannerError::new(
                        ScannerErrorKind::CounterOverflow,
                        start,
                        start,
                        "operator end offset overflowed",
                    )
                })?;

            let matched = self.cursor.consume_str(operator.spelling());
            if !matched || self.cursor.byte_offset() != end_offset {
                return Err(ScannerError::new(
                    ScannerErrorKind::InternalInvariant,
                    start,
                    self.cursor.position(),
                    "operator registry match disagrees with cursor advancement",
                ));
            }

            return self
                .make_token(
                    operator.kind(),
                    start,
                    self.cursor.position(),
                    TokenChannel::Default,
                )
                .map(Some);
        }

        if let Some(kind) = punctuation_kind(ch) {
            let _ = self.cursor.consume();
            return self
                .make_token(kind, start, self.cursor.position(), TokenChannel::Default)
                .map(Some);
        }

        // The recovery policy must never leave an unrecognized scalar at the
        // cursor forever. Stop mode reports the error without consuming it.
        if self.config.recovery == RecoveryPolicy::Stop {
            return Err(ScannerError::new(
                ScannerErrorKind::InternalInvariant,
                start,
                start,
                format!("unrecognized source character {ch:?}"),
            ));
        }

        let _ = self.cursor.consume();
        self.make_token(
            TokenKind::Invalid,
            start,
            self.cursor.position(),
            TokenChannel::Error,
        )
        .map(Some)
    }
}

impl TokenSource for Scanner<'_> {
    type Error = ScannerError;

    fn next_token(&mut self) -> Result<Option<Token<'_>>, Self::Error> {
        self.initialize()?;

        if self.eof_emitted {
            return Ok(None);
        }

        self.scan_one()
    }
}

/// Maps punctuation spellings to canonical token identities.
///
/// Operators are deliberately handled by `OperatorRegistry` first. Extend
/// this mapping only when the normative punctuation vocabulary changes.
fn punctuation_kind(ch: char) -> Option<TokenKind> {
    Some(match ch {
        '(' => TokenKind::LeftParen,
        ')' => TokenKind::RightParen,
        '{' => TokenKind::LeftBrace,
        '}' => TokenKind::RightBrace,
        '[' => TokenKind::LeftBracket,
        ']' => TokenKind::RightBracket,
        ',' => TokenKind::Comma,
        ';' => TokenKind::Semicolon,
        ':' => TokenKind::Colon,
        '.' => TokenKind::Dot,
        '@' => TokenKind::At,
        '#' => TokenKind::Hash,
        '$' => TokenKind::Dollar,
        '?' => TokenKind::Question,
        '~' => TokenKind::Tilde,
        _ => return None,
    })
}
