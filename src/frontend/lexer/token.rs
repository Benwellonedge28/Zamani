
#![forbid(unsafe_code)]

//! Canonical Zamani frontend token representation.
//!
//! Intended path: `src/frontend/lexer/token.rs`
//!
//! # Responsibilities
//!
//! - Represent one lexically recognized source unit.
//! - Preserve the exact lexeme without requiring a copy for ordinary scanning.
//! - Attach a source span and a token channel.
//! - Support synthetic tokens used by recovery and end-of-input handling.
//! - Provide stable accessors for parsers, diagnostics, formatters and tooling.
//!
//! # Does not own
//!
//! - Token identities or keyword spelling: `token_kind.rs` and registries.
//! - Source coordinate calculations: `span.rs`.
//! - Scanning, Unicode classification or recovery algorithms.
//! - Semantic interpretation of classical, quantum, HDL or other constructs.
//! - Target discovery, hardware capacity or resource allocation.
//!
//! # Integration contract
//!
//! `token_kind.rs` must export `TokenKind`, including `Eof`.
//! `span.rs` must export the canonical `Span` type.
//! `token_metadata.rs` may derive metadata from `TokenKind`; this type does
//! not duplicate that metadata.
//! `scanner.rs` constructs tokens, `api.rs` exposes them, and the parser
//! consumes their kind, lexeme and span.
//!
//! The legacy lexer/parser boundary should use an explicit adapter rather
//! than duplicating token definitions in this module.
//!
//! # Scalability
//!
//! Borrowed lexemes avoid an allocation per token. Owned lexemes are available
//! for synthetic tokens. Actual maximum source size remains bounded by
//! available address space, memory, and the configured frontend resource
//! policy; this type imposes no language-level token-count or hardware limit.

use std::borrow::Cow;
use std::fmt;

use super::span::Span;
use super::token_kind::TokenKind;

/// The destination or purpose of a token in the lexical stream.
///
/// The channel is distinct from `TokenKind`: for example, a comment's
/// lexical identity remains a comment even when the parser ignores its
/// channel.
///
/// The scanner and configuration determine channel policy. This enum does
/// not decide whether a particular spelling is a comment or whitespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum TokenChannel {
    /// Ordinary tokens consumed by the parser.
    Default,

    /// Whitespace and comments retained for tooling but normally skipped
    /// by parser-facing token iteration.
    Trivia,

    /// Documentation material retained for documentation tooling.
    Documentation,

    /// A token representing malformed or unrecognized source.
    Error,
}

impl Default for TokenChannel {
    fn default() -> Self {
        Self::Default
    }
}

/// One token emitted by the Zamani lexical frontend.
///
/// `lexeme` stores the original source spelling whenever the scanner can
/// preserve it. It is deliberately not normalized or semantically decoded:
/// escape processing, numeric conversion and identifier normalization
/// belong to their respective frontend stages.
///
/// The lifetime `'src` permits zero-copy tokens that borrow slices of the
/// source text. Use `Token::owned` for generated or synthetic lexemes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token<'src> {
    kind: TokenKind,
    lexeme: Cow<'src, str>,
    span: Span,
    channel: TokenChannel,
}

impl<'src> Token<'src> {
    /// Construct a token whose lexeme borrows from the input source.
    ///
    /// The caller must ensure the supplied lexeme is the exact source slice
    /// represented by `span`. This constructor does not guess source offsets
    /// or rewrite the spelling.
    #[must_use]
    pub fn borrowed(kind: TokenKind, lexeme: &'src str, span: Span) -> Self {
        Self {
            kind,
            lexeme: Cow::Borrowed(lexeme),
            span,
            channel: TokenChannel::Default,
        }
    }

    /// Construct a token with an owned lexeme.
    ///
    /// Use this for synthetic tokens, generated text, or when the source
    /// spelling cannot safely be borrowed for the required lifetime.
    #[must_use]
    pub fn owned(kind: TokenKind, lexeme: String, span: Span) -> Self {
        Self {
            kind,
            lexeme: Cow::Owned(lexeme),
            span,
            channel: TokenChannel::Default,
        }
    }

    /// Construct a token from any value convertible to `Cow<str>`.
    ///
    /// This is useful when the caller already manages borrowing versus
    /// ownership. Prefer `borrowed` or `owned` when the choice is known.
    #[must_use]
    pub fn new(
        kind: TokenKind,
        lexeme: impl Into<Cow<'src, str>>,
        span: Span,
    ) -> Self {
        Self {
            kind,
            lexeme: lexeme.into(),
            span,
            channel: TokenChannel::Default,
        }
    }

    /// Construct a synthetic token with no source spelling.
    ///
    /// Typically used for EOF or parser recovery. The caller supplies the
    /// zero-width or otherwise appropriate span.
    #[must_use]
    pub fn synthetic(kind: TokenKind, span: Span) -> Self {
        Self {
            kind,
            lexeme: Cow::Borrowed(""),
            span,
            channel: TokenChannel::Default,
        }
    }

    /// Set the token channel.
    ///
    /// This builder does not change the token's kind, lexeme or span.
    #[must_use]
    pub fn with_channel(mut self, channel: TokenChannel) -> Self {
        self.channel = channel;
        self
    }

    /// Return the canonical token identity.
    #[must_use]
    pub const fn kind(&self) -> TokenKind {
        self.kind
    }

    /// Return the exact source spelling or synthetic lexeme.
    #[must_use]
    pub fn lexeme(&self) -> &str {
        self.lexeme.as_ref()
    }

    /// Return the lexeme as a `Cow`, preserving whether it is borrowed.
    #[must_use]
    pub fn lexeme_cow(&self) -> &Cow<'src, str> {
        &self.lexeme
    }

    /// Return the source span.
    #[must_use]
    pub const fn span(&self) -> &Span {
        &self.span
    }

    /// Return the token channel.
    #[must_use]
    pub const fn channel(&self) -> TokenChannel {
        self.channel
    }

    /// Return true when the token belongs to the default parser channel.
    #[must_use]
    pub const fn is_default_channel(&self) -> bool {
        matches!(self.channel, TokenChannel::Default)
    }

    /// Return true when the token is retained as trivia.
    #[must_use]
    pub const fn is_trivia_channel(&self) -> bool {
        matches!(self.channel, TokenChannel::Trivia)
    }

    /// Return true when the token is documentation material.
    #[must_use]
    pub const fn is_documentation_channel(&self) -> bool {
        matches!(self.channel, TokenChannel::Documentation)
    }

    /// Return true when the scanner classified this token as an error.
    #[must_use]
    pub const fn is_error_channel(&self) -> bool {
        matches!(self.channel, TokenChannel::Error)
    }

    /// Return true when the token is the canonical EOF token.
    #[must_use]
    pub const fn is_eof(&self) -> bool {
        matches!(self.kind, TokenKind::Eof)
    }

    /// Return true when this token carries no lexeme.
    ///
    /// This is not equivalent to EOF: recovery may synthesize other
    /// zero-lexeme tokens.
    #[must_use]
    pub fn has_empty_lexeme(&self) -> bool {
        self.lexeme.is_empty()
    }

    /// Return true when the lexeme is borrowed from source text.
    #[must_use]
    pub const fn borrows_source(&self) -> bool {
        matches!(self.lexeme, Cow::Borrowed(_))
    }

    /// Consume the token and return its owned lexeme.
    ///
    /// Borrowed lexemes are copied only when this method is called.
    #[must_use]
    pub fn into_owned_lexeme(self) -> String {
        self.lexeme.into_owned()
    }

    /// Consume the token and produce an owned-lifetime-independent token.
    ///
    /// The token kind, span and channel are preserved.
    #[must_use]
    pub fn into_owned(self) -> Token<'static> {
        Token {
            kind: self.kind,
            lexeme: Cow::Owned(self.lexeme.into_owned()),
            span: self.span,
            channel: self.channel,
        }
    }

    /// Change the channel while preserving all other token data.
    ///
    /// Provided for code that prefers mutation over builder-style creation.
    pub fn set_channel(&mut self, channel: TokenChannel) {
        self.channel = channel;
    }
}

impl fmt::Display for Token<'_> {
    /// Display a stable, human-readable token summary.
    ///
    /// This representation is intended for diagnostics and debugging, not
    /// as a serialization format or a replacement for source reconstruction.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:?}({:?})",
            self.kind,
            self.lexeme.as_ref()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Token, TokenChannel};
    use crate::frontend::lexer::span::Span;
    use crate::frontend::lexer::token_kind::TokenKind;

    fn test_span() -> Span {
        Span::default()
    }

    #[test]
    fn borrowed_token_preserves_lexeme() {
        let source = "qubit";
        let token = Token::borrowed(TokenKind::Identifier, source, test_span());

        assert_eq!(token.kind(), TokenKind::Identifier);
        assert_eq!(token.lexeme(), "qubit");
        assert!(token.borrows_source());
    }

    #[test]
    fn owned_token_preserves_lexeme() {
        let token = Token::owned(
            TokenKind::Identifier,
            String::from("generated_name"),
            test_span(),
        );

        assert_eq!(token.lexeme(), "generated_name");
        assert!(!token.borrows_source());
    }

    #[test]
    fn synthetic_token_has_empty_lexeme() {
        let token = Token::synthetic(TokenKind::Eof, test_span());

        assert!(token.is_eof());
        assert!(token.has_empty_lexeme());
    }

    #[test]
    fn channel_builder_preserves_token_identity() {
        let token = Token::borrowed(
            TokenKind::Identifier,
            "commented_name",
            test_span(),
        )
        .with_channel(TokenChannel::Trivia);

        assert_eq!(token.kind(), TokenKind::Identifier);
        assert_eq!(token.lexeme(), "commented_name");
        assert!(token.is_trivia_channel());
    }

    #[test]
    fn owned_conversion_removes_source_lifetime_dependency() {
        let token = {
            let source = String::from("temporary");
            Token::borrowed(TokenKind::Identifier, &source, test_span()).into_owned()
        };

        assert_eq!(token.lexeme(), "temporary");
    }

    #[test]
    fn display_contains_kind_and_lexeme() {
        let token = Token::borrowed(
            TokenKind::Identifier,
            "sample",
            test_span(),
        );

        let rendered = token.to_string();

        assert!(rendered.contains("Identifier"));
        assert!(rendered.contains("sample"));
    }
}
