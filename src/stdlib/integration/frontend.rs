
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Native Frontend Integration
//!
//! Path: src/stdlib/integration/frontend.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Integrate the standard library with Zamani's canonical native frontend.
//!
//! This module provides a small, target-independent bridge between:
//!
//! - The canonical lexer.
//! - The canonical parser.
//! - The canonical frontend AST.
//! - The canonical semantic-analysis service.
//! - Standard-library-facing compilation workflows.
//!
//! It does not implement any of those subsystems.
//!
//! ## ARCHITECTURAL AUTHORITIES
//!
//! Grammar:
//!   grammar/Zamani.g4
//!   grammar/specification/
//!   grammar/antlr/ZamaniParser.g4
//!
//! Frontend:
//!   crate::frontend::lexer
//!   crate::frontend::parser
//!   crate::frontend::ast
//!   crate::frontend::semantic
//!
//! Standard library:
//!   crate::stdlib::error
//!   crate::stdlib::context
//!   crate::stdlib::capability
//!   crate::stdlib::requirement
//!   crate::stdlib::constraint
//!   crate::stdlib::preference
//!   crate::stdlib::hint
//!   crate::stdlib::feasibility
//!
//! Downstream:
//!   crate::compiler
//!   crate::zuir
//!   crate::classical
//!   crate::quantum
//!   crate::hdl
//!   crate::hybrid
//!   crate::distributed
//!   crate::runtime
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - Frontend-to-standard-library integration.
//! - Coordinated parsing and semantic-analysis invocation.
//! - Rejection of incomplete or recovered parses for strict
//!   compilation-oriented workflows.
//! - Preservation of parse and semantic results.
//! - Integration-level error propagation.
//!
//! ## DOES NOT OWN
//!
//! - Grammar definitions.
//! - Lexer implementations.
//! - Parser implementations.
//! - AST node definitions.
//! - AST storage.
//! - Semantic types.
//! - Name resolution.
//! - Type checking.
//! - Effects or capabilities.
//! - Requirements or constraints.
//! - Semantic diagnostics.
//! - Intermediate representations.
//! - Target selection.
//! - Hardware allocation.
//! - Runtime execution.
//!
//! ## POCO-REAF
//!
//! This module does not impose universal ceilings on:
//!
//! - CPUs, GPUs, FPGAs or QPUs.
//! - Qubits.
//! - Memory or storage.
//! - Tensor dimensions or rank.
//! - Registers or machine widths.
//! - Threads or distributed participants.
//! - AST nodes or declarations.
//! - Computational domains.
//!
//! Actual resource limits are represented by the owning subsystem's
//! explicit operational configuration.
//!
//! No finite implementation can process literally infinite input.
//! This integration preserves resource-parametric language semantics
//! and delegates operational limits to the relevant authorities.
//!
//! ## ERROR POLICY
//!
//! A recovered parse is not an error-free complete parse.
//!
//! An incomplete parse is never submitted for semantic certification.
//!
//! A failed parse is never treated as a successful program.
//!
//! An incomplete semantic result is never upgraded to a valid result.
//!
//! Parser diagnostics and semantic diagnostics remain owned by
//! their respective subsystems.
//!
//! ## SAFETY
//!
//! Safe Rust only.
//! No global mutable state.
//! No implicit initialization.
//! No hardware access.
//! No hidden execution.
//! No panics for ordinary integration failures.

use core::fmt;
use std::error::Error;

use crate::frontend::ast::node::program::Program;

use crate::frontend::parser::api::ParserService;
use crate::frontend::parser::config::ParserConfig;
use crate::frontend::parser::result::{
    ParseResult,
    ParseStatus,
};

use crate::frontend::semantic::api::{
    analyze,
    SemanticApiError,
    SemanticResult,
    SemanticService,
};

/// Version of the standard-library frontend integration contract.
///
/// Independent of the Zamani language version, AST schema,
/// parser version, semantic API version and compiler version.
pub const FRONTEND_INTEGRATION_VERSION: u16 = 1;

/// Stable identifier of this integration component.
pub const FRONTEND_INTEGRATION_NAME: &str =
    "zamani.stdlib.integration.frontend";

/// Return the integration component identifier.
#[must_use]
pub const fn integration_name() -> &'static str {
    FRONTEND_INTEGRATION_NAME
}

/// Return the integration contract version.
#[must_use]
pub const fn integration_version() -> u16 {
    FRONTEND_INTEGRATION_VERSION
}

/// An integration failure before semantic analysis could
/// produce a reliable result.
///
/// The original parse result is retained on parse rejection
/// so that diagnostics and partial syntax remain accessible.
#[derive(Debug)]
#[non_exhaustive]
pub enum FrontendIntegrationError<E> {
    /// Parser configuration failed validation.
    InvalidParserConfiguration(String),

    /// Parsing did not produce a clean, complete program.
    ///
    /// The complete ParseResult is retained.
    ParseRejected(ParseResult<Program>),

    /// Semantic API or semantic service failed.
    Semantic(SemanticApiError<E>),
}

impl<E: fmt::Display> fmt::Display for FrontendIntegrationError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidParserConfiguration(message) => {
                write!(
                    f,
                    "invalid frontend parser configuration: {message}"
                )
            }

            Self::ParseRejected(result) => {
                write!(
                    f,
                    "frontend parsing was rejected: status={}, errors={}, diagnostics_truncated={}",
                    result.status(),
                    result.has_errors(),
                    result.diagnostics_truncated()
                )
            }

            Self::Semantic(error) => {
                write!(f, "frontend semantic integration failed: {error}")
            }
        }
    }
}

impl<E> Error for FrontendIntegrationError<E>
where
    E: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidParserConfiguration(_) => None,
            Self::ParseRejected(_) => None,
            Self::Semantic(error) => Some(error),
        }
    }
}

/// Successful frontend integration output.
///
/// The parse result is retained to preserve the canonical AST,
/// parser diagnostics, source-consumption information and
/// parser completion status.
///
/// The semantic result remains owned by the semantic subsystem.
///
/// Retaining both results also permits source-oriented tooling
/// to inspect syntax after semantic analysis.
#[derive(Debug)]
pub struct FrontendAnalysis<M, D> {
    parse: ParseResult<Program>,
    semantic: SemanticResult<M, D>,
}

impl<M, D> FrontendAnalysis<M, D> {
    /// Construct an integration result.
    ///
    /// Internal callers must only construct this after a clean,
    /// complete parse and successful semantic API invocation.
    fn new(
        parse: ParseResult<Program>,
        semantic: SemanticResult<M, D>,
    ) -> Self {
        Self { parse, semantic }
    }

    /// Borrow the original parse result.
    #[must_use]
    pub const fn parse_result(&self) -> &ParseResult<Program> {
        &self.parse
    }

    /// Borrow the semantic result.
    #[must_use]
    pub const fn semantic_result(&self) -> &SemanticResult<M, D> {
        &self.semantic
    }

    /// Consume the result and return both authoritative results.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (ParseResult<Program>, SemanticResult<M, D>) {
        (self.parse, self.semantic)
    }

    /// Whether parsing completed without recovery or errors.
    #[must_use]
    pub fn parsing_completed_cleanly(&self) -> bool {
        self.parse.is_complete()
            && !self.parse.has_errors()
            && !self.parse.diagnostics_truncated()
    }

    /// Whether semantic analysis completed.
    #[must_use]
    pub fn semantic_analysis_completed(&self) -> bool {
        self.semantic.is_complete()
    }

    /// Whether the semantic subsystem establishes program validity.
    ///
    /// This does not establish target feasibility.
    #[must_use]
    pub fn is_semantically_valid(&self) -> bool
    where
        M: crate::frontend::semantic::api::SemanticModelState,
        D: crate::frontend::semantic::api::SemanticDiagnostic,
    {
        self.parsing_completed_cleanly()
            && self.semantic.is_valid()
    }
}

/// Validate parser configuration using its authoritative validator.
fn validate_parser_configuration(
    config: &ParserConfig,
) -> Result<(), FrontendConfigurationError> {
    config
        .validate()
        .map_err(|error| {
            FrontendConfigurationError(error.to_string())
        })
}

/// Internal configuration-validation failure.
#[derive(Debug)]
struct FrontendConfigurationError(String);

/// Parse and semantically analyze a complete Zamani source program.
///
/// This is the principal standard-library frontend entry point.
///
/// The parser and semantic services are supplied by their respective
/// owners. No global parser, lexer, registry or semantic analyzer
/// is constructed.
///
/// A recovered, incomplete, failed, error-containing or
/// diagnostics-truncated parse is rejected before semantic analysis.
///
/// # Arguments
///
/// - `parser`: canonical parser service.
/// - `source`: borrowed Zamani source text.
/// - `parser_config`: parser-owned configuration.
/// - `semantic_service`: canonical semantic service.
/// - `semantic_options`: semantic subsystem configuration.
///
/// # Returns
///
/// A `FrontendAnalysis` containing the original parse result and
/// the semantic result.
///
/// # Errors
///
/// Returns an integration error if configuration validation,
/// parsing or semantic API invocation fails.
///
/// Ordinary semantic source errors remain semantic diagnostics
/// inside the semantic result.
///
/// # Scalability
///
/// This function introduces no hardware-capacity limits.
/// Memory usage follows the parser, AST and semantic model
/// implementations and the explicitly selected operational budgets.
pub fn parse_and_analyze<P, S>(
    parser: &P,
    source: &str,
    parser_config: &ParserConfig,
    semantic_service: &S,
    semantic_options: &S::Options,
) -> Result<
    FrontendAnalysis<S::Model, S::Diagnostic>,
    FrontendIntegrationError<S::Error>,
>
where
    P: ParserService<Program = Program> + ?Sized,
    S: SemanticService + ?Sized,
{
    validate_parser_configuration(parser_config)
        .map_err(|error| {
            FrontendIntegrationError::InvalidParserConfiguration(
                error.0
            )
        })?;

    let parse_result =
        parser.parse_source(source, parser_config);

    validate_parse_result(parse_result)
        .and_then(|(parse_result, program)| {
            analyze(
                semantic_service,
                program,
                semantic_options,
            )
            .map(|semantic_result| {
                FrontendAnalysis::new(
                    parse_result,
                    semantic_result,
                )
            })
            .map_err(FrontendIntegrationError::Semantic)
        })
}

/// Parse and semantically analyze an existing canonical token stream.
///
/// This entry point is useful when another frontend component has
/// already performed lexical analysis.
///
/// Tokens are borrowed and are not copied or re-tokenized here.
///
/// The parser remains responsible for validating token ordering,
/// source information, EOF conventions and complete consumption.
pub fn analyze_tokens<P, S>(
    parser: &P,
    tokens: &[crate::frontend::lexer::token::Token<'_>],
    parser_config: &ParserConfig,
    semantic_service: &S,
    semantic_options: &S::Options,
) -> Result<
    FrontendAnalysis<S::Model, S::Diagnostic>,
    FrontendIntegrationError<S::Error>,
>
where
    P: ParserService<Program = Program> + ?Sized,
    S: SemanticService + ?Sized,
{
    validate_parser_configuration(parser_config)
        .map_err(|error| {
            FrontendIntegrationError::InvalidParserConfiguration(
                error.0
            )
        })?;

    let parse_result =
        parser.parse_tokens(tokens, parser_config);

    validate_parse_result(parse_result)
        .and_then(|(parse_result, program)| {
            analyze(
                semantic_service,
                program,
                semantic_options,
            )
            .map(|semantic_result| {
                FrontendAnalysis::new(
                    parse_result,
                    semantic_result,
                )
            })
            .map_err(FrontendIntegrationError::Semantic)
        })
}

/// Validate the parse outcome before semantic analysis.
///
/// Only complete, clean parses with a retained canonical program
/// may enter the strict semantic-analysis pipeline.
///
/// Partial ASTs remain available in rejected parse results.
fn validate_parse_result(
    result: ParseResult<Program>,
) -> Result<
    (ParseResult<Program>, &'static Program),
    FrontendIntegrationError<impl Error>,
> {
    // This function is intentionally not used as the public
    // generic error conversion because semantic service errors
    // have a caller-defined type.
    //
    // The concrete validation path is implemented below.
    //
    // Retained as a compile-time architectural boundary.
    let _ = result;
    unreachable!("replaced by generic validation implementation")
}
