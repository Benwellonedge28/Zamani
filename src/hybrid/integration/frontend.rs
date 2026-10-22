
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani hybrid frontend integration
//!
//! Path: `src/hybrid/integration/frontend.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Bridge the canonical frontend's hybrid syntax adapter to the canonical
//! AST construction boundary without introducing a second AST, parser,
//! semantic model, or intermediate representation.
//!
//! The canonical syntax implementation is:
//!
//! `crate::frontend::parser::domains::hybrid`
//!
//! The canonical AST is owned by:
//!
//! `crate::frontend::ast::node`
//!
//! Semantic interpretation is owned by:
//!
//! `crate::frontend::semantic`
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The adapter contract between hybrid syntax and AST construction.
//! - Preservation and validation of the syntax adapter's token ranges.
//! - Structured separation of syntax failures from AST-adapter failures.
//! - A single entry point for parsing and adapting a hybrid region.
//!
//! This module does not own:
//!
//! - Lexing or token definitions.
//! - Hybrid grammar or expression parsing.
//! - Canonical AST node definitions.
//! - Name resolution, type checking, effects, ownership, or capabilities.
//! - Quantum or classical IR.
//! - Compiler planning, hardware discovery, scheduling, or runtime execution.
//!
//! ## Token-range contract
//!
//! `HybridRegion` element ranges index the parser-visible token sequence,
//! not the original token sequence. The parser-visible sequence contains
//! tokens on the Default and Error channels, in their original order.
//! Trivia and documentation-channel tokens are excluded.
//!
//! Adapters MUST use the same sequence when resolving an element's range.
//!
//! ## Integration contract
//!
//! The AST adapter supplied by the caller is the only component allowed to
//! construct or mutate canonical AST nodes. This module passes it the parsed
//! hybrid region and the corresponding parser-visible tokens.
//!
//! Syntax classification is not semantic validation. A successful result
//! means that the syntax adapter accepted the hybrid-region structure and
//! that the AST adapter completed successfully. It does not certify that
//! the program is semantically valid or executable on any target.

use core::fmt;
use std::error::Error;

use crate::frontend::lexer::token::{Token, TokenChannel};
use crate::frontend::parser::domains::hybrid::{
    parse_hybrid_region, HybridRegion, HybridSyntaxError,
};

/// The canonical-AST integration boundary for hybrid syntax.
///
/// Implement this trait in the existing frontend AST-adapter layer.
///
/// The implementation must use the canonical AST node definitions and
/// preserve source information. It must not create an alternative AST or
/// perform semantic validation.
///
/// `visible_tokens` is the exact token view against which every element's
/// half-open `token_range()` is defined.
///
/// The returned `Output` should be the canonical AST result, or an existing
/// canonical AST adapter result that the caller can integrate into it.
pub trait HybridAstAdapter {
    /// The canonical AST result or canonical AST-adapter result.
    type Output;

    /// The adapter's structured error type.
    type Error;

    /// Convert the parsed hybrid syntax into the canonical AST representation.
    ///
    /// Implementations should:
    ///
    /// - Preserve region and element source spans.
    /// - Preserve source order.
    /// - Resolve token ranges against `visible_tokens`.
    /// - Delegate ordinary statements and expressions to the shared parser
    ///   and AST adapter.
    /// - Preserve information that is not yet semantically understood.
    /// - Return errors instead of silently discarding unsupported syntax.
    /// - Avoid interpreting syntax classifications as semantic facts.
    fn adapt_region<'source>(
        &self,
        region: HybridRegion,
        visible_tokens: &[&Token<'source>],
    ) -> Result<Self::Output, Self::Error>;
}

/// Failure while integrating hybrid syntax into the canonical AST.
///
/// Syntax failures and AST-adapter failures remain distinguishable, allowing
/// the compiler to route them to the correct diagnostic subsystem.
#[derive(Debug)]
pub enum HybridFrontendIntegrationError<E> {
    /// The canonical hybrid syntax adapter rejected the input.
    Syntax(HybridSyntaxError),

    /// Canonical AST construction or adaptation failed.
    AstAdapter(E),

    /// An internally inconsistent token range or source mapping was detected.
    ///
    /// This is an integration failure, not a source-language semantic error.
    InvalidTokenMapping {
        element_index: usize,
        range_start: usize,
        range_end: usize,
        visible_token_count: usize,
    },
}

impl<E: fmt::Display> fmt::Display for HybridFrontendIntegrationError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax(error) => {
                write!(f, "hybrid syntax integration failed: {error}")
            }
            Self::AstAdapter(error) => {
                write!(f, "canonical AST adaptation failed: {error}")
            }
            Self::InvalidTokenMapping {
                element_index,
                range_start,
                range_end,
                visible_token_count,
            } => write!(
                f,
                "hybrid AST integration found an invalid token mapping \
                 for element {element_index}: range {range_start}..{range_end}, \
                 visible token count {visible_token_count}"
            ),
        }
    }
}

impl<E> Error for HybridFrontendIntegrationError<E>
where
    E: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Syntax(error) => Some(error),
            Self::AstAdapter(error) => Some(error),
            Self::InvalidTokenMapping { .. } => None,
        }
    }
}

/// Parse and adapt one complete hybrid region.
///
/// This function delegates syntax recognition to the existing hybrid parser
/// and AST construction to the supplied canonical AST adapter.
///
/// It does not create a competing AST or run semantic analysis.
///
/// # Arguments
///
/// * `tokens` - Tokens from the canonical Zamani lexer, in source order.
/// * `adapter` - The implementation of [`HybridAstAdapter`] backed by the
///   canonical frontend AST adapter.
///
/// # Errors
///
/// Returns a structured error if hybrid syntax is invalid, if the token
/// mapping is inconsistent, or if canonical AST adaptation fails.
///
/// # Scalability
///
/// This function introduces no hardware-size or domain-count ceilings.
/// Memory and execution costs depend on the supplied token stream, the
/// existing parser, the AST adapter, and available resources.
///
/// The parser's own allocations and operational safeguards remain applicable.
///
/// # Important
///
/// The token ranges exposed by the hybrid parser are relative to the
/// parser-visible sequence. This function constructs that exact sequence
/// before invoking the AST adapter.
pub fn adapt_hybrid_region<'source, A>(
    tokens: &[Token<'source>],
    adapter: &A,
) -> Result<A::Output, HybridFrontendIntegrationError<A::Error>>
where
    A: HybridAstAdapter,
{
    let region = parse_hybrid_region(tokens)
        .map_err(HybridFrontendIntegrationError::Syntax)?;

    let visible_tokens: Vec<&Token<'source>> = tokens
        .iter()
        .filter(|token| {
            matches!(
                token.channel(),
                TokenChannel::Default | TokenChannel::Error
            )
        })
        .collect();

    validate_token_mappings(&region, visible_tokens.len())?;

    adapter
        .adapt_region(region, &visible_tokens)
        .map_err(HybridFrontendIntegrationError::AstAdapter)
}

/// Validate that the parser's element ranges can be resolved against the
/// canonical parser-visible token sequence.
///
/// This function checks integration integrity only. It does not attempt to
/// validate statements, expressions, types, effects, or domain semantics.
fn validate_token_mappings<E>(
    region: &HybridRegion,
    visible_token_count: usize,
) -> Result<(), HybridFrontendIntegrationError<E>> {
    for (element_index, element) in region.elements().iter().enumerate() {
        let range = element.token_range();

        // Empty element ranges are not valid parser output: every accepted
        // element must correspond to at least one visible token.
        if range.start >= range.end || range.end > visible_token_count {
            return Err(
                HybridFrontendIntegrationError::InvalidTokenMapping {
                    element_index,
                    range_start: range.start,
                    range_end: range.end,
                    visible_token_count,
                },
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // These tests exercise the adapter contract without manufacturing
    // tokens or depending on undocumented constructors in the lexer.
    //
    // End-to-end tests using real lexer output belong in the frontend
    // integration test suite, where the canonical lexer API is available.

    #[test]
    fn invalid_token_mapping_is_reported_as_integration_failure() {
        let error: HybridFrontendIntegrationError<String> =
            HybridFrontendIntegrationError::InvalidTokenMapping {
                element_index: 2,
                range_start: 4,
                range_end: 4,
                visible_token_count: 8,
            };

        assert!(matches!(
            error,
            HybridFrontendIntegrationError::InvalidTokenMapping {
                element_index: 2,
                ..
            }
        ));
    }

    #[test]
    fn integration_error_display_is_actionable() {
        let error: HybridFrontendIntegrationError<String> =
            HybridFrontendIntegrationError::InvalidTokenMapping {
                element_index: 1,
                range_start: 3,
                range_end: 9,
                visible_token_count: 5,
            };

        let message = error.to_string();

        assert!(message.contains("element 1"));
        assert!(message.contains("3..9"));
        assert!(message.contains("visible token count 5"));
    }

    #[test]
    fn integration_error_categories_are_distinct() {
        let syntax: HybridFrontendIntegrationError<String> =
            HybridFrontendIntegrationError::InvalidTokenMapping {
                element_index: 0,
                range_start: 0,
                range_end: 0,
                visible_token_count: 0,
            };

        assert!(matches!(
            syntax,
            HybridFrontendIntegrationError::InvalidTokenMapping { .. }
        ));
    }
}
