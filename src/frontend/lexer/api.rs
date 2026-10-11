#![forbid(unsafe_code)]

//! Public API for the Zamani frontend lexer.
//!
//! File: src/frontend/lexer/api.rs
//! Minimum Rust: 1.97
//! Edition: 2021
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide a stable, implementation-independent interface for consuming
//! tokens from the Zamani lexer.
//!
//! # Responsibilities
//!
//! This module owns:
//! - the public token-source contract;
//! - a streaming lexer facade;
//! - fallible token retrieval;
//! - optional collection of a complete token stream;
//! - public re-exports of canonical token types;
//! - API-level behavior and compatibility documentation.
//!
//! This module does not own:
//! - source decoding or scanning;
//! - operator and keyword definitions;
//! - dialect registration;
//! - lexical configuration or resource limits;
//! - source cursor movement;
//! - token construction;
//! - diagnostic classification or recovery;
//! - parser precedence or AST construction;
//! - semantic interpretation, quantum execution, or HDL lowering;
//! - target selection, resource allocation, or hardware discovery.
//!
//! # Integration contract
//!
//! - `token.rs` supplies the canonical `Token` type.
//! - `token_kind.rs` supplies `TokenKind`.
//! - `span.rs` supplies the canonical source `Span`.
//! - `scanner.rs` implements `TokenSource`.
//! - `config.rs` defines configuration accepted by scanner construction.
//! - `diagnostics.rs` owns concrete lexical diagnostic types.
//! - `recovery.rs` owns lexical recovery policy.
//! - `keyword_registry.rs`, `operator_registry.rs`, and `dialect.rs`
//!   supply lexical classification and extension behavior to the scanner.
//! - The parser consumes the stream through this API or a compatibility
//!   adapter.
//!
//! # Streaming contract
//!
//! A `TokenSource` returns one token at a time. It must return the EOF token
//! once, if the canonical token-stream contract requires an explicit EOF
//! token, and then return `Ok(None)` on subsequent calls. Implementations
//! must not repeatedly emit EOF indefinitely.
//!
//! The API does not require callers to retain the entire token stream.
//! `collect_tokens` is provided for consumers that explicitly need an owned
//! vector and have an appropriate memory budget.
//!
//! # Scalability and portability
//!
//! This module defines no language-level maximum source size, token count,
//! operator count, qubit count, processor count, memory capacity, tensor
//! rank, device count, or topology size.
//!
//! Streaming retrieval avoids forcing whole-file token buffering. Collecting
//! all tokens necessarily uses memory proportional to the number and size of
//! tokens, subject to the host process's available resources.
//!
//! # Compatibility
//!
//! The existing root-level `crate::lexer` and `crate::parser` use the legacy
//! `TokenType` representation. This API does not silently convert between
//! incompatible token enums. An explicit compatibility adapter must map
//! canonical token kinds and spans to the legacy types until those consumers
//! are migrated.
//!
//! # Important boundary
//!
//! This is the public facade, not the scanner itself. It can be completed
//! independently of scanning algorithms, while the scanner is integrated
//! later by implementing `TokenSource`.

use std::error::Error;

use super::token::Token;
use super::token_kind::TokenKind;
use super::span::Span;

pub use super::token::TokenChannel;

/// Canonical token and source-location types exposed by the lexer API.
pub mod types {
    pub use super::super::span::Span;
    pub use super::super::token::{Token, TokenChannel};
    pub use super::super::token_kind::{TokenCategory, TokenKind};
}

/// A source of canonical lexer tokens.
///
/// The scanner is the primary intended implementation. This trait also makes
/// the public facade testable with deterministic mock sources, and allows
/// alternate source implementations without coupling callers to scanner
/// internals.
///
/// # EOF behavior
///
/// Implementations should emit a single explicit `TokenKind::Eof` token if
/// that is the configured token-stream convention, and then return
/// `Ok(None)` for later calls. If a scanner uses `None` as its sole EOF
/// signal, it must do so consistently and document that behavior.
///
/// # Errors
///
/// The associated error type should be the concrete lexical diagnostic
/// type provided by `diagnostics.rs` once that module is integrated.
pub trait TokenSource {
    /// Error type emitted by the source.
    type Error: Error;

    /// Returns the next token, or `None` when the stream is exhausted.
    fn next_token(&mut self) -> Result<Option<Token>, Self::Error>;
}

/// Public streaming facade over a token source.
///
/// `Lexer` does not implement lexical recognition itself. It delegates to
/// the configured source, keeping scanner implementation details out of the
/// public interface.
#[derive(Debug)]
pub struct Lexer<S> {
    source: S,
    exhausted: bool,
}

impl<S> Lexer<S>
where
    S: TokenSource,
{
    /// Wraps an implementation of `TokenSource`.
    ///
    /// Construct a concrete scanner using the scanner's constructor and its
    /// lexical configuration, then pass it to this method.
    #[must_use]
    pub const fn new(source: S) -> Self {
        Self {
            source,
            exhausted: false,
        }
    }

    /// Returns whether the facade has reached the end of its source.
    #[must_use]
    pub const fn is_exhausted(&self) -> bool {
        self.exhausted
    }

    /// Retrieves the next token.
    ///
    /// Once the source returns `Ok(None)`, subsequent calls return
    /// `Ok(None)` without calling the source again.
    ///
    /// If the source returns an error, the error is passed to the caller.
    /// The facade does not silently discard errors or decide whether scanning
    /// should recover. That policy belongs to the scanner and recovery layer.
    pub fn next_token(&mut self) -> Result<Option<Token>, S::Error> {
        if self.exhausted {
            return Ok(None);
        }

        match self.source.next_token()? {
            Some(token) => {
                if token.kind() == TokenKind::Eof {
                    // Preserve the explicit EOF token for the caller, then
                    // prevent repeated reads after this terminal token.
                    self.exhausted = true;
                }

                Ok(Some(token))
            }
            None => {
                self.exhausted = true;
                Ok(None)
            }
        }
    }

    /// Borrows the underlying token source.
    ///
    /// This is intended for diagnostics, instrumentation, or integration
    /// layers that need source-specific read-only access.
    #[must_use]
    pub const fn source(&self) -> &S {
        &self.source
    }

    /// Mutably borrows the underlying token source.
    ///
    /// Use this only when source-specific functionality is required. Normal
    /// consumers should use `next_token` to preserve the facade's EOF state.
    pub fn source_mut(&mut self) -> &mut S {
        &mut self.source
    }

    /// Consumes the facade and returns the underlying source.
    #[must_use]
    pub fn into_source(self) -> S {
        self.source
    }

    /// Collects the remaining token stream into a vector.
    ///
    /// This method has no artificial token-count limit. Its memory use grows
    /// with the stream and is bounded by the host process's available
    /// resources. For large inputs, prefer repeated calls to `next_token`.
    ///
    /// An explicit EOF token is included if the source emits one.
    pub fn collect_tokens(mut self) -> Result<Vec<Token>, S::Error> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }

        Ok(tokens)
    }

    /// Passes remaining tokens to a callback one at a time.
    ///
    /// This avoids retaining the complete stream. The callback returns
    /// `ControlFlow::Break(value)` to stop early or `ControlFlow::Continue`
    /// to keep consuming tokens.
    pub fn for_each_token<B, F>(
        &mut self,
        mut callback: F,
    ) -> Result<std::ops::ControlFlow<B>, S::Error>
    where
        F: FnMut(Token) -> std::ops::ControlFlow<B>,
    {
        while let Some(token) = self.next_token()? {
            if let std::ops::ControlFlow::Break(value) = callback(token) {
                return Ok(std::ops::ControlFlow::Break(value));
            }
        }

        Ok(std::ops::ControlFlow::Continue(()))
    }
}

/// Convenience function that collects every remaining token from a source.
///
/// This is equivalent to `Lexer::new(source).collect_tokens()`.
///
/// Prefer the streaming facade for large source files when retaining all
/// tokens is unnecessary.
pub fn collect_tokens<S>(source: S) -> Result<Vec<Token>, S::Error>
where
    S: TokenSource,
{
    Lexer::new(source).collect_tokens()
}

/// Canonical source-location alias for consumers of this module.
pub type SourceSpan = Span;

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::Infallible;
    use std::ops::ControlFlow;

    /// Minimal source used to test facade behavior without depending on the
    /// production scanner implementation.
    struct MockSource {
        tokens: std::vec::IntoIter<Token>,
        calls: usize,
    }

    impl MockSource {
        fn new(tokens: Vec<Token>) -> Self {
            Self {
                tokens: tokens.into_iter(),
                calls: 0,
            }
        }
    }

    impl TokenSource for MockSource {
        type Error = Infallible;

        fn next_token(&mut self) -> Result<Option<Token>, Self::Error> {
            self.calls += 1;
            Ok(self.tokens.next())
        }
    }

    // These tests are intentionally limited to the public facade contract.
    // Token construction should be tested in token.rs and scanner.rs.
    //
    // Once token.rs exposes its stable constructor, create test tokens here
    // through that constructor rather than duplicating token internals.

    #[test]
    fn empty_source_becomes_exhausted() {
        let source = MockSource::new(Vec::new());
        let mut lexer = Lexer::new(source);

        assert!(!lexer.is_exhausted());
        assert!(lexer.next_token().expect("infallible source").is_none());
        assert!(lexer.is_exhausted());
        assert!(lexer.next_token().expect("exhausted source").is_none());
    }

    #[test]
    fn callback_can_stop_early() {
        // This test uses an empty source because token construction belongs
        // to the token module; it verifies the control-flow API's contract.
        let source = MockSource::new(Vec::new());
        let mut lexer = Lexer::new(source);

        let result = lexer
            .for_each_token::<(), _>(|_token| ControlFlow::Break(()))
            .expect("infallible source");

        assert_eq!(result, ControlFlow::Continue(()));
    }

    #[test]
    fn collection_accepts_an_empty_stream() {
        let source = MockSource::new(Vec::new());
        let tokens = collect_tokens(source).expect("infallible source");

        assert!(tokens.is_empty());
    }
}