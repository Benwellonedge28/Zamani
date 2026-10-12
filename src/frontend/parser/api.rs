#![forbid(unsafe_code)]

//! Public API contract for the Zamani frontend parser.
//!
//! File: `src/frontend/parser/api.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Expose a stable, implementation-independent interface for parsing Zamani
//! source text and canonical lexer tokens into the repository's AST.
//!
//! # Ownership
//!
//! This module owns:
//! - the public parser-service contract;
//! - stable, documented parsing entry points;
//! - the boundary between callers and parser implementation details;
//! - API-level documentation and contract tests.
//!
//! This module does not own:
//! - lexical scanning or token definitions;
//! - grammar productions or precedence rules;
//! - parser-session state or recovery algorithms;
//! - AST node definitions or AST conversion;
//! - semantic analysis, type checking, or effect validation;
//! - capability negotiation, resource allocation, or target discovery;
//! - quantum execution, HDL elaboration, scheduling, or lowering.
//!
//! # Integration contract
//!
//! - `config.rs` owns `ParserConfig`.
//! - `result.rs` owns `ParseResult<T>` and its completion/diagnostic model.
//! - `parser.rs` implements the parsing engine behind `ParserService`.
//! - `session.rs` owns per-invocation mutable state.
//! - `grammar.rs` and `grammar/` own syntax recognition.
//! - `ast_adapter.rs` maps syntax to the existing canonical AST.
//! - `source.rs` coordinates source identity and source locations.
//! - `crate::frontend::lexer` owns canonical tokens and spans.
//!
//! `ParserService` is intentionally a trait. The API does not assume a
//! particular parser constructor, AST root type, scanner constructor, or
//! grammar implementation that has not yet been verified. The production
//! parser implements this contract once those module interfaces are frozen.
//!
//! # Parsing guarantees
//!
//! A conforming implementation must:
//! - use the canonical frontend lexer token model;
//! - distinguish complete parsing from recovered or incomplete parsing;
//! - preserve source spans and meaningful syntax in the canonical AST;
//! - report structured diagnostics rather than silently dropping errors;
//! - enforce complete input consumption for complete-program parsing;
//! - use deterministic behavior for identical source, configuration, and
//!   extension definitions;
//! - return structured failures for exhausted parser budgets;
//! - avoid panics on malformed user input;
//! - keep syntax recognition separate from semantic validation and lowering.
//!
//! # Scalability
//!
//! This API introduces no language-level maximum for tokens, declarations,
//! expressions, qubits, processors, devices, register widths, tensor rank,
//! memory, or distributed topology. Practical parser budgets belong to
//! `ParserConfig` and `ParserLimits`, not to hardware assumptions.
//!
//! Parsing an entire source file may require memory proportional to the
//! source, token stream, and AST. Implementations must report configured
//! budget exhaustion rather than silently truncating input.
//!
//! # Source and token entry points
//!
//! `parse_source` accepts source text and delegates lexing and parsing to the
//! configured service. `parse_tokens` accepts canonical lexer tokens and
//! does not re-tokenize them. Expression, type, and item entry points use the
//! same configuration, diagnostic, and AST contracts.
//!
//! These functions are façades: grammar logic must remain in the parser
//! implementation, not in this module.

use crate::frontend::lexer::token::Token;

use super::config::ParserConfig;
use super::result::ParseResult;

/// Stable interface implemented by the production parser service.
///
/// The service should be cheap to share and must not retain mutable state
/// between independent calls. Per-parse state belongs in a parser session.
///
/// Associated AST types intentionally allow this interface to integrate with
/// the repository's existing AST without introducing duplicate node models.
pub trait ParserService {
    /// Canonical AST type for a complete source program.
    type Program;

    /// Canonical AST type for an expression.
    type Expression;

    /// Canonical AST type for a type expression.
    type Type;

    /// Canonical AST type for a top-level or nested item.
    type Item;

    /// Parse source text as a complete Zamani program.
    ///
    /// Implementations are responsible for invoking the canonical lexer,
    /// forwarding lexical diagnostics, parsing the resulting token stream,
    /// and reporting whether the entire source was consumed.
    ///
    /// The source is borrowed for the duration of the call. Returned AST
    /// values must satisfy their own ownership/lifetime contract.
    fn parse_source(
        &self,
        source: &str,
        config: &ParserConfig,
    ) -> ParseResult<Self::Program>;

    /// Parse a canonical token stream as a complete program.
    ///
    /// The implementation must validate the stream's EOF convention,
    /// preserve token spans, and reject trailing non-EOF input. It must not
    /// silently reinterpret tokens from the legacy `crate::lexer` API.
    fn parse_tokens<'src>(
        &self,
        tokens: &[Token<'src>],
        config: &ParserConfig,
    ) -> ParseResult<Self::Program>;

    /// Parse one expression from source text.
    ///
    /// The implementation must apply the language's normative precedence
    /// and associativity rules. Whether trailing tokens are allowed must be
    /// explicit in the returned parse status and documented configuration.
    fn parse_expression(
        &self,
        source: &str,
        config: &ParserConfig,
    ) -> ParseResult<Self::Expression>;

    /// Parse one type expression from source text.
    ///
    /// This recognizes type syntax only. Name resolution, type checking,
    /// target-specific widths, and semantic constraints belong downstream.
    fn parse_type(
        &self,
        source: &str,
        config: &ParserConfig,
    ) -> ParseResult<Self::Type>;

    /// Parse one item from source text.
    ///
    /// An item may be a declaration or another top-level construct permitted
    /// by the active language edition and dialect configuration.
    fn parse_item(
        &self,
        source: &str,
        config: &ParserConfig,
    ) -> ParseResult<Self::Item>;
}

/// Parse a complete Zamani source program.
///
/// This free function is the preferred entry point for callers that already
/// have a configured parser service. It deliberately does not construct a
/// global parser or retain state between invocations.
pub fn parse_source<P>(
    service: &P,
    source: &str,
    config: &ParserConfig,
) -> ParseResult<P::Program>
where
    P: ParserService + ?Sized,
{
    service.parse_source(source, config)
}

/// Parse a complete program from canonical lexer tokens.
///
/// The tokens must come from the Zamani frontend lexer or a compatible
/// implementation that follows its token and EOF contracts. This function
/// does not copy tokens or re-run lexical recognition.
pub fn parse_tokens<P>(
    service: &P,
    tokens: &[Token<'_>],
    config: &ParserConfig,
) -> ParseResult<P::Program>
where
    P: ParserService + ?Sized,
{
    service.parse_tokens(tokens, config)
}

/// Parse an expression using the configured parser service.
pub fn parse_expression<P>(
    service: &P,
    source: &str,
    config: &ParserConfig,
) -> ParseResult<P::Expression>
where
    P: ParserService + ?Sized,
{
    service.parse_expression(source, config)
}

/// Parse a type expression using the configured parser service.
pub fn parse_type<P>(
    service: &P,
    source: &str,
    config: &ParserConfig,
) -> ParseResult<P::Type>
where
    P: ParserService + ?Sized,
{
    service.parse_type(source, config)
}

/// Parse one item using the configured parser service.
pub fn parse_item<P>(
    service: &P,
    source: &str,
    config: &ParserConfig,
) -> ParseResult<P::Item>
where
    P: ParserService + ?Sized,
{
    service.parse_item(source, config)
}

#[cfg(test)]
mod tests {
    use super::*;

    // The API module deliberately tests its contract through a mock service.
    // Concrete AST construction and grammar correctness are tested by the
    // parser and grammar integration suites.

    struct MockService;

    #[derive(Debug, PartialEq, Eq)]
    struct MockNode;

    // This mock's result constructors depend on the canonical ParseResult
    // contract. Add the implementation once result.rs freezes its public
    // constructors; do not duplicate ParseResult here.
    //
    // ParserService implementation tests belong in tests/api.rs, where the
    // production service and its real configuration can be exercised.
}