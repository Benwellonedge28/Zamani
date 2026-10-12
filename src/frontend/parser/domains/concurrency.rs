
#![forbid(unsafe_code)]

//! Zamani parser — concurrency-domain token adapter.
//!
//! Path: `src/frontend/parser/domains/concurrency.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Provide a loss-preserving boundary between the canonical lexer and the
//! concurrency grammar handlers. Recognized constructs retain their original
//! token range and source spans so that later grammar productions can parse
//! actors, tasks, futures, channels, synchronization, cancellation, and
//! parallel computation without re-tokenizing source text.
//!
//! # Ownership
//!
//! This module owns:
//! - concurrency construct categories used by parser dispatch;
//! - source-token range metadata;
//! - balanced-delimiter scanning for candidate construct boundaries;
//! - structured boundary errors;
//! - the token-preservation contract for concurrency parsing.
//!
//! This module does not own:
//! - lexical recognition or token spelling registries;
//! - authoritative grammar productions;
//! - canonical AST node construction;
//! - name or type resolution;
//! - effect, ownership, or capability checking;
//! - task scheduling or runtime execution;
//! - physical resource discovery or allocation;
//! - distributed placement or hardware selection.
//!
//! # Integration contract
//!
//! The canonical lexer owns `Token`, `TokenKind`, `TokenChannel`, and `Span`.
//! This module consumes those types directly. It does not define replacements.
//!
//! The grammar files under `grammar/concurrency/` own the actual language
//! syntax. This adapter recognizes candidate roots and token boundaries only;
//! it must not be treated as a substitute for those grammar productions.
//!
//! # Scalability
//!
//! There are no language-level limits on task count, actor count, channel
//! count, future count, parallel regions, or hardware capacity.
//!
//! The delimiter stack grows with input nesting and is bounded only by
//! available memory and any explicit resource budget imposed by the caller.
//! Callers processing untrusted input should apply the parser's shared
//! source, token, nesting, and work budgets.
//!
//! # Determinism
//!
//! The same ordered token stream produces the same candidate boundaries.
//! No hardware inspection, global mutable state, I/O, or runtime execution
//! occurs in this module.

use core::fmt;

use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::{Token, TokenChannel};
use crate::frontend::lexer::token_kind::TokenKind;

/// The concurrency grammar category associated with a candidate construct.
///
/// This is an open-ended dispatch classification, not a semantic model.
/// More specific syntax remains owned by its corresponding grammar file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ConcurrencyConstructKind {
    /// An asynchronous expression or declaration.
    Async,

    /// An await expression.
    Await,

    /// A spawn expression or statement.
    Spawn,

    /// An actor declaration, construction, command, or actor statement.
    Actor,

    /// A task-oriented construct.
    Task,

    /// A future-oriented construct.
    Future,

    /// Generic parallel computation.
    Parallel,

    /// A communication channel construct.
    Channel,

    /// A send operation.
    Send,

    /// A receive operation.
    Receive,

    /// A channel selection construct.
    Select,

    /// A synchronization construct.
    Synchronization,

    /// A cancellation construct.
    Cancellation,

    /// A concurrency construct not covered by a more specific category.
    Other,
}

/// A half-open range into the caller's original token slice.
///
/// `start` is inclusive and `end` is exclusive. The range retains the
/// original tokens rather than copying or normalizing their contents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenRange {
    start: usize,
    end: usize,
}

impl TokenRange {
    /// Construct a range if its bounds are ordered.
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Option<Self> {
        if start <= end {
            Some(Self { start, end })
        } else {
            None
        }
    }

    /// Inclusive starting token index.
    #[must_use]
    pub const fn start(self) -> usize {
        self.start
    }

    /// Exclusive ending token index.
    #[must_use]
    pub const fn end(self) -> usize {
        self.end
    }

    /// Number of tokens in the range.
    #[must_use]
    pub const fn len(self) -> usize {
        self.end - self.start
    }

    /// Whether the range contains no tokens.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }

    /// Borrow the original tokens covered by this range.
    ///
    /// Returns `None` when the range does not fit the supplied slice.
    #[must_use]
    pub fn get<'tokens, 'src>(
        self,
        tokens: &'tokens [Token<'src>],
    ) -> Option<&'tokens [Token<'src>]> {
        tokens.get(self.start..self.end)
    }
}

/// A candidate concurrency construct in the original token stream.
///
/// Both endpoint spans are retained separately. The adapter does not need
/// to invent a new source-span type or assume that spans can be merged.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcurrencyConstruct {
    kind: ConcurrencyConstructKind,
    tokens: TokenRange,
    start_span: Span,
    end_span: Span,
}

impl ConcurrencyConstruct {
    /// Construct metadata for a recognized candidate.
    #[must_use]
    pub const fn new(
        kind: ConcurrencyConstructKind,
        tokens: TokenRange,
        start_span: Span,
        end_span: Span,
    ) -> Self {
        Self {
            kind,
            tokens,
            start_span,
            end_span,
        }
    }

    /// Candidate category.
    #[must_use]
    pub const fn kind(&self) -> ConcurrencyConstructKind {
        self.kind
    }

    /// Original token range, with an exclusive end.
    #[must_use]
    pub const fn token_range(&self) -> TokenRange {
        self.tokens
    }

    /// Source span of the first token.
    #[must_use]
    pub const fn start_span(&self) -> &Span {
        &self.start_span
    }

    /// Source span of the final token in the candidate.
    #[must_use]
    pub const fn end_span(&self) -> &Span {
        &self.end_span
    }

    /// Borrow the original tokens for a grammar-specific parser.
    #[must_use]
    pub fn source_tokens<'tokens, 'src>(
        &self,
        tokens: &'tokens [Token<'src>],
    ) -> Option<&'tokens [Token<'src>]> {
        self.tokens.get(tokens)
    }
}

/// Reason a candidate boundary could not be determined safely.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConcurrencyBoundaryErrorKind {
    /// The requested starting index is outside the token slice.
    StartOutOfBounds,

    /// The starting token does not identify a concurrency candidate.
    NotConcurrencyConstruct,

    /// A closing delimiter appeared without a matching opening delimiter.
    UnexpectedClosingDelimiter,

    /// The input ended before an opened delimiter was closed.
    UnclosedDelimiter,

    /// The token stream ended without an EOF token while a construct
    /// still required a terminator.
    UnterminatedConstruct,
}

/// A structured boundary error with source location and token index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcurrencyBoundaryError {
    kind: ConcurrencyBoundaryErrorKind,
    token_index: usize,
    span: Option<Span>,
}

impl ConcurrencyBoundaryError {
    /// Create a boundary error.
    #[must_use]
    pub const fn new(
        kind: ConcurrencyBoundaryErrorKind,
        token_index: usize,
        span: Option<Span>,
    ) -> Self {
        Self {
            kind,
            token_index,
            span,
        }
    }

    /// Error classification.
    #[must_use]
    pub const fn kind(&self) -> &ConcurrencyBoundaryErrorKind {
        &self.kind
    }

    /// Index of the token at which the error was detected.
    #[must_use]
    pub const fn token_index(&self) -> usize {
        self.token_index
    }

    /// Source span when a token was available.
    #[must_use]
    pub const fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl fmt::Display for ConcurrencyBoundaryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "concurrency boundary error {:?} at token {}",
            self.kind, self.token_index
        )
    }
}

impl std::error::Error for ConcurrencyBoundaryError {}

/// Identify a concurrency candidate from its first token.
///
/// Dedicated token kinds are used where the canonical lexer provides them.
/// Other concurrency keywords remain spelling-driven until the lexical
/// contract supplies dedicated token kinds for them.
///
/// Spelling-based recognition is limited to identifier tokens, preventing
/// string and numeric literal contents from being mistaken for keywords.
#[must_use]
pub fn classify_start(
    token: &Token<'_>,
) -> Option<ConcurrencyConstructKind> {
    if !token.is_default_channel() {
        return None;
    }

    match token.kind() {
        TokenKind::KeywordAsync => Some(ConcurrencyConstructKind::Async),
        TokenKind::KeywordAwait => Some(ConcurrencyConstructKind::Await),
        TokenKind::KeywordSpawn => Some(ConcurrencyConstructKind::Spawn),
        TokenKind::Identifier => classify_identifier(token.lexeme()),
        _ => None,
    }
}

fn classify_identifier(spelling: &str) -> Option<ConcurrencyConstructKind> {
    // This table classifies syntax roots; it is not a closed list of
    // operations, runtime primitives, hardware devices, or domain names.
    match spelling {
        "actor" => Some(ConcurrencyConstructKind::Actor),
        "task" => Some(ConcurrencyConstructKind::Task),
        "future" => Some(ConcurrencyConstructKind::Future),
        "parallel" => Some(ConcurrencyConstructKind::Parallel),
        "channel" => Some(ConcurrencyConstructKind::Channel),
        "send" => Some(ConcurrencyConstructKind::Send),
        "receive" => Some(ConcurrencyConstructKind::Receive),
        "select" => Some(ConcurrencyConstructKind::Select),
        "synchronize" | "synchronization" => {
            Some(ConcurrencyConstructKind::Synchronization)
        }
        "cancel" | "cancellation" => {
            Some(ConcurrencyConstructKind::Cancellation)
        }
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Delimiter {
    Parenthesis,
    Bracket,
    Brace,
}

impl Delimiter {
    fn opening(kind: TokenKind) -> Option<Self> {
        match kind {
            TokenKind::LeftParen => Some(Self::Parenthesis),
            TokenKind::LeftBracket => Some(Self::Bracket),
            TokenKind::LeftBrace => Some(Self::Brace),
            _ => None,
        }
    }

    fn closing(kind: TokenKind) -> Option<Self> {
        match kind {
            TokenKind::RightParen => Some(Self::Parenthesis),
            TokenKind::RightBracket => Some(Self::Bracket),
            TokenKind::RightBrace => Some(Self::Brace),
            _ => None,
        }
    }
}

/// Scan one candidate construct beginning at `start`.
///
/// This routine determines a balanced token boundary; it does not validate
/// the internal syntax of actors, tasks, channels, or other constructs.
///
/// The candidate ends after:
/// - a top-level semicolon; or
/// - the closing brace that completes its outermost braced construct; or
/// - EOF, if no delimiter remains open.
///
/// A grammar handler must still validate the candidate against its canonical
/// production. For complete source parsing, the caller must separately
/// enforce the expected EOF/terminator policy for the selected production.
pub fn scan_candidate<'src>(
    tokens: &[Token<'src>],
    start: usize,
) -> Result<ConcurrencyConstruct, ConcurrencyBoundaryError> {
    let first = tokens.get(start).ok_or_else(|| {
        ConcurrencyBoundaryError::new(
            ConcurrencyBoundaryErrorKind::StartOutOfBounds,
            start,
            None,
        )
    })?;

    let kind = classify_start(first).ok_or_else(|| {
        ConcurrencyBoundaryError::new(
            ConcurrencyBoundaryErrorKind::NotConcurrencyConstruct,
            start,
            Some(first.span().clone()),
        )
    })?;

    let mut delimiters: Vec<(Delimiter, usize)> = Vec::new();
    let mut index = start;
    let mut saw_outer_brace = false;

    while let Some(token) = tokens.get(index) {
        if token.channel() != TokenChannel::Default {
            index += 1;
            continue;
        }

        let token_kind = token.kind();

        if token_kind == TokenKind::Eof {
            if let Some((_, opening_index)) = delimiters.last().copied() {
                let opening_span = tokens
                    .get(opening_index)
                    .map(|opening| opening.span().clone());

                return Err(ConcurrencyBoundaryError::new(
                    ConcurrencyBoundaryErrorKind::UnclosedDelimiter,
                    opening_index,
                    opening_span,
                ));
            }

            if index == start {
                return Err(ConcurrencyBoundaryError::new(
                    ConcurrencyBoundaryErrorKind::UnterminatedConstruct,
                    index,
                    Some(token.span().clone()),
                ));
            }

            let last = tokens.get(index.saturating_sub(1)).unwrap_or(first);
            return Ok(ConcurrencyConstruct::new(
                kind,
                TokenRange::new(start, index).expect("ordered token range"),
                first.span().clone(),
                last.span().clone(),
            ));
        }

        if let Some(opening) = Delimiter::opening(token_kind) {
            if opening == Delimiter::Brace && delimiters.is_empty() {
                saw_outer_brace = true;
            }

            delimiters.push((opening, index));
            index += 1;
            continue;
        }

        if let Some(closing) = Delimiter::closing(token_kind) {
            match delimiters.pop() {
                Some((opening, _)) if opening == closing => {
                    index += 1;

                    if delimiters.is_empty() && saw_outer_brace {
                        let last = &tokens[index - 1];
                        return Ok(ConcurrencyConstruct::new(
                            kind,
                            TokenRange::new(start, index)
                                .expect("ordered token range"),
                            first.span().clone(),
                            last.span().clone(),
                        ));
                    }

                    continue;
                }
                _ => {
                    return Err(ConcurrencyBoundaryError::new(
                        ConcurrencyBoundaryErrorKind::UnexpectedClosingDelimiter,
                        index,
                        Some(token.span().clone()),
                    ));
                }
            }
        }

        if token_kind == TokenKind::Semicolon && delimiters.is_empty() {
            let last = token;
            index += 1;

            return Ok(ConcurrencyConstruct::new(
                kind,
                TokenRange::new(start, index).expect("ordered token range"),
                first.span().clone(),
                last.span().clone(),
            ));
        }

        index += 1;
    }

    if let Some((_, opening_index)) = delimiters.last().copied() {
        let opening_span = tokens
            .get(opening_index)
            .map(|opening| opening.span().clone());

        return Err(ConcurrencyBoundaryError::new(
            ConcurrencyBoundaryErrorKind::UnclosedDelimiter,
            opening_index,
            opening_span,
        ));
    }

    let last_index = tokens.len().saturating_sub(1);
    let last = tokens.get(last_index).unwrap_or(first);

    Ok(ConcurrencyConstruct::new(
        kind,
        TokenRange::new(start, tokens.len()).expect("ordered token range"),
        first.span().clone(),
        last.span().clone(),
    ))
}

/// Discover candidate concurrency constructs in a token stream.
///
/// The scan advances monotonically. Each recognized candidate is preserved
/// as a token range; non-concurrency tokens are left untouched. Nested
/// constructs may be discovered by scanning their own token ranges in a
/// subsequent grammar pass.
///
/// This is a discovery helper, not a replacement for the canonical program,
/// statement, expression, or concurrency grammar.
pub fn discover_candidates<'src>(
    tokens: &[Token<'src>],
) -> Vec<ConcurrencyConstruct> {
    let mut candidates = Vec::new();
    let mut index = 0;

    while let Some(token) = tokens.get(index) {
        if token.kind() == TokenKind::Eof {
            break;
        }

        if let Some(_) = classify_start(token) {
            match scan_candidate(tokens, index) {
                Ok(candidate) => {
                    let end = candidate.token_range().end();
                    candidates.push(candidate);

                    // Continue inside the candidate as well, so nested
                    // await/spawn/task constructs can be discovered.
                    index = index.saturating_add(1);
                    if end <= index {
                        continue;
                    }
                }
                Err(_) => {
                    // Discovery is intentionally non-fatal. The owning
                    // grammar should call scan_candidate at its production
                    // boundary and report its structured error.
                    index = index.saturating_add(1);
                }
            }
        } else {
            index = index.saturating_add(1);
        }
    }

    candidates
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::lexer::span::{CursorPosition, Span};

    fn span_at(byte: usize) -> Span {
        let position = CursorPosition::new(byte, 1, byte.saturating_add(1));
        Span::at(position)
    }

    fn token(kind: TokenKind, lexeme: &'static str, byte: usize) -> Token<'static> {
        Token::borrowed(kind, lexeme, span_at(byte))
    }

    #[test]
    fn classifies_canonical_async_tokens() {
        let async_token = token(TokenKind::KeywordAsync, "async", 0);
        let await_token = token(TokenKind::KeywordAwait, "await", 6);
        let spawn_token = token(TokenKind::KeywordSpawn, "spawn", 12);

        assert_eq!(
            classify_start(&async_token),
            Some(ConcurrencyConstructKind::Async)
        );
        assert_eq!(
            classify_start(&await_token),
            Some(ConcurrencyConstructKind::Await)
        );
        assert_eq!(
            classify_start(&spawn_token),
            Some(ConcurrencyConstructKind::Spawn)
        );
    }

    #[test]
    fn classifies_identifier_based_actor_root() {
        let actor_token = token(TokenKind::Identifier, "actor", 0);

        assert_eq!(
            classify_start(&actor_token),
            Some(ConcurrencyConstructKind::Actor)
        );
    }

    #[test]
    fn does_not_classify_literal_contents_as_keywords() {
        let string_token = token(TokenKind::StringLiteral, "actor", 0);

        assert_eq!(classify_start(&string_token), None);
    }

    #[test]
    fn scans_a_balanced_block() {
        let tokens = vec![
            token(TokenKind::Identifier, "actor", 0),
            token(TokenKind::Identifier, "Worker", 6),
            token(TokenKind::LeftBrace, "{", 13),
            token(TokenKind::Identifier, "run", 15),
            token(TokenKind::LeftParen, "(", 18),
            token(TokenKind::RightParen, ")", 19),
            token(TokenKind::Semicolon, ";", 20),
            token(TokenKind::RightBrace, "}", 22),
            token(TokenKind::Eof, "", 23),
        ];

        let candidate = scan_candidate(&tokens, 0).expect("balanced block");
        assert_eq!(candidate.kind(), ConcurrencyConstructKind::Actor);
        assert_eq!(candidate.token_range().start(), 0);
        assert_eq!(candidate.token_range().end(), 8);
        assert_eq!(candidate.source_tokens(&tokens).unwrap().len(), 8);
    }

    #[test]
    fn rejects_a_mismatched_delimiter() {
        let tokens = vec![
            token(TokenKind::KeywordAwait, "await", 0),
            token(TokenKind::LeftParen, "(", 5),
            token(TokenKind::RightBracket, "]", 6),
            token(TokenKind::Eof, "", 7),
        ];

        let error = scan_candidate(&tokens, 0).expect_err("mismatch must fail");
        assert_eq!(
            error.kind(),
            &ConcurrencyBoundaryErrorKind::UnexpectedClosingDelimiter
        );
    }

    #[test]
    fn reports_an_unclosed_delimiter() {
        let tokens = vec![
            token(TokenKind::KeywordSpawn, "spawn", 0),
            token(TokenKind::LeftParen, "(", 5),
            token(TokenKind::Identifier, "work", 6),
            token(TokenKind::Eof, "", 10),
        ];

        let error = scan_candidate(&tokens, 0).expect_err("open delimiter");
        assert_eq!(
            error.kind(),
            &ConcurrencyBoundaryErrorKind::UnclosedDelimiter
        );
    }

    #[test]
    fn ignores_trivia_when_scanning_boundaries() {
        let tokens = vec![
            token(TokenKind::KeywordAwait, "await", 0),
            Token::borrowed(TokenKind::Whitespace, " ", span_at(5))
                .with_channel(TokenChannel::Trivia),
            token(TokenKind::Identifier, "operation", 6),
            token(TokenKind::Semicolon, ";", 15),
            token(TokenKind::Eof, "", 16),
        ];

        let candidate = scan_candidate(&tokens, 0).expect("candidate");
        assert_eq!(candidate.token_range().end(), 4);
    }

    #[test]
    fn token_ranges_are_half_open() {
        let range = TokenRange::new(2, 7).expect("ordered range");

        assert_eq!(range.start(), 2);
        assert_eq!(range.end(), 7);
        assert_eq!(range.len(), 5);
        assert!(!range.is_empty());
        assert!(TokenRange::new(4, 4).unwrap().is_empty());
        assert!(TokenRange::new(5, 4).is_none());
    }
}
