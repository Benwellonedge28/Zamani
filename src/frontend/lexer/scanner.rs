#![forbid(unsafe_code)]

//! Zamani production lexical scanner.
//!
//! Responsibilities:
//! - Coordinate cursor-based lexical recognition.
//! - Apply configured dialects and token registries.
//! - Enforce configurable source, token, and diagnostic budgets.
//! - Preserve UTF-8-safe source spans.
//! - Provide deterministic recovery and exactly-once EOF emission.
//! - Implement the canonical TokenSource interface.

use super::api::TokenSource;
use super::config::LexerConfig;
use super::cursor::Cursor;
use super::keyword_registry::KeywordRegistry;
use super::operator_registry::OperatorRegistry;
use super::token::Token;

/// Stateful scanner over borrowed Zamani source text.
///
/// Registry ownership and diagnostic types must match the
/// final, integrated APIs of the corresponding modules.
pub struct Scanner<'src> {
    cursor: Cursor<'src>,
    config: LexerConfig,
    keywords: KeywordRegistry,
    operators: OperatorRegistry,
    eof_emitted: bool,
    emitted_tokens: usize,
}

impl<'src> Scanner<'src> {
    /// Construct a scanner using explicitly supplied lexical registries.
    ///
    /// The final implementation must validate configuration and
    /// registries before accepting source for scanning.
    pub fn new(
        source: &'src str,
        config: LexerConfig,
        keywords: KeywordRegistry,
        operators: OperatorRegistry,
    ) -> Self {
        Self {
            cursor: Cursor::new(source),
            config,
            keywords,
            operators,
            eof_emitted: false,
            emitted_tokens: 0,
        }
    }

    /// Return the current source position.
    pub const fn position(
        &self,
    ) -> super::cursor::CursorPosition {
        self.cursor.position()
    }

    /// Return the immutable scanner configuration.
    pub const fn config(&self) -> &LexerConfig {
        &self.config
    }

    /// Return the number of tokens emitted so far.
    pub const fn emitted_tokens(&self) -> usize {
        self.emitted_tokens
    }

    /// Return whether the explicit EOF token has been emitted.
    pub const fn eof_emitted(&self) -> bool {
        self.eof_emitted
    }

    // Integration requirement:
    //
    // Implement next_token() only after the canonical Span,
    // diagnostic, recovery, limit, and recognizer APIs are
    // available. Do not guess their constructors or error types.
}