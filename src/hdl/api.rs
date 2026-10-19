
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Public API
//!
//! Path: `src/hdl/api.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Expose the existing HDL parser and semantic validator through one
//! deliberate public API without duplicating their implementations.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/Zamani.g4`
//! - `grammar/hdl/hdl.g4` and its delegate grammars
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - The canonical frontend AST and semantic model.
//!
//! The grammar owns syntax. The frontend owns tokens, spans, AST adaptation,
//! and semantic validation. This API coordinates access to those services.
//!
//! ## Owns
//!
//! - The stable HDL API identifier and API schema version.
//! - Deliberate public re-exports of existing HDL contracts.
//! - A stateless facade for parser-session creation and validation.
//!
//! ## Does not own
//!
//! - Lexing, parsing algorithms, AST nodes, or source spans.
//! - A second semantic model, width system, or diagnostic system.
//! - HDLIR, IR construction, verification, transformations, or lowering.
//! - Elaboration, synthesis, scheduling, placement, routing, or allocation.
//! - Target discovery, target feasibility, or physical device handles.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Scalability and portability
//!
//! This API introduces no universal hardware-capacity limits. Parser safety
//! budgets are supplied to the canonical parser and remain operational
//! safeguards, not language-semantic restrictions.
//!
//! Finite software cannot process literally infinite input. Actual processing
//! is bounded by available resources, representable values, supported
//! semantics, and explicit operational budgets. Incomplete work must never
//! be reported as completed validation.
//!
//! The canonical semantic validator represents concrete widths with `u128`.
//! This facade preserves that contract instead of introducing a competing
//! width representation. Arbitrary-precision concrete widths would require
//! a coordinated evolution of the canonical semantic contract and consumers.
//!
//! ## Validation semantics
//!
//! `validate_hdl` checks only the rules implemented by the existing HDL
//! semantic validator. A valid result does not certify all frontend
//! semantics, elaboration, target feasibility, synthesis, or physical timing
//! closure.
//!
//! The parser session consumes canonical frontend tokens. It does not
//! tokenize source text or promise to construct the canonical AST.
//! Shared-parser orchestration and AST adaptation remain frontend concerns.
//!
//! ## Freeze criteria
//!
//! - Canonical parser and semantic contracts remain authoritative.
//! - Public exports compile through the registered HDL module.
//! - Parser errors and semantic diagnostics remain distinct.
//! - Incomplete validation is never reclassified as success.
//! - No HDLIR or target-realization implementation is introduced here.
//! - Public API and integration tests pass on the supported Rust toolchain.

/// Stable identifier for the Zamani HDL API.
pub const HDL_API_NAME: &str = "zamani.hdl.api";

/// Version of this public API contract.
///
/// This is independent of the language, grammar, semantic-model, HDLIR,
/// and package versions.
pub const HDL_API_SCHEMA_VERSION: u16 = 1;

// -----------------------------------------------------------------------------
// Canonical parser contracts
// -----------------------------------------------------------------------------

/// Existing HDL parser contracts, re-exported without replacement types.
pub use crate::frontend::parser::domains::hdl::{
    HdlConstruct,
    HdlConstructKind,
    HdlDomainParser,
    HdlParseError,
    HdlParseErrorKind,
    HdlParserLimits,
    HdlSyntaxToken,
};

// -----------------------------------------------------------------------------
// Canonical semantic contracts
// -----------------------------------------------------------------------------

/// Existing HDL semantic contracts, re-exported without replacement types.
pub use crate::frontend::semantic::domains::hdl::{
    validate_hdl,
    HdlAssignment,
    HdlClock,
    HdlDiagnostic,
    HdlDiagnosticCode,
    HdlDirection,
    HdlId,
    HdlInputError,
    HdlProcess,
    HdlProcessKind,
    HdlProgram,
    HdlReset,
    HdlResetBehavior,
    HdlResetPolarity,
    HdlSeverity,
    HdlSignal,
    HdlSignedness,
    HdlSourceLocation,
    HdlValidationResult,
    HdlValidationStatus,
    HdlValueType,
    HdlWidth,
};

// -----------------------------------------------------------------------------
// Stateless public facade
// -----------------------------------------------------------------------------

/// Stateless entry point to the canonical HDL parser and semantic validator.
///
/// `HdlApi` owns no design, token stream, parser cursor, global mutable state,
/// target handle, or cached semantic result. Parser sessions own their
/// individual cursors; validation borrows caller-owned semantic input.
///
/// The facade is safe to share because it contains no mutable state.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HdlApi;

impl HdlApi {
    /// Create the stateless HDL API facade.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Return the stable API identifier.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        HDL_API_NAME
    }

    /// Return this API contract's schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        HDL_API_SCHEMA_VERSION
    }

    /// Create an HDL parser session over canonical frontend tokens.
    ///
    /// The returned parser borrows `tokens`. The token stream is not
    /// re-tokenized, modified, or removed from its owner. `limits` configures
    /// parser-safety budgets, not language-level hardware capacities.
    ///
    /// The caller remains responsible for invoking the parser and adapting
    /// its loss-preserving syntax into the canonical AST through the
    /// appropriate frontend integration.
    ///
    /// # Errors
    ///
    /// Returns the canonical parser's structured error when the parser
    /// configuration is invalid.
    pub fn parse_session<'tokens, 'src>(
        &self,
        tokens: &'tokens [
            crate::frontend::lexer::token::Token<'src>
        ],
        limits: HdlParserLimits,
    ) -> Result<HdlDomainParser<'tokens, 'src>, HdlParseError> {
        HdlDomainParser::new(tokens, limits)
    }

    /// Validate normalized HDL semantic input using the canonical validator.
    ///
    /// The program remains owned by the caller and is not modified.
    ///
    /// The returned result is forwarded unchanged from `validate_hdl`.
    /// Its status describes only the checks implemented by that validator;
    /// it does not establish target feasibility or implementation success.
    #[must_use]
    pub fn validate(&self, program: &HdlProgram) -> HdlValidationResult {
        validate_hdl(program)
    }
}

/// Validate normalized HDL semantic input using the canonical validator.
///
/// This convenience function is equivalent to
/// `HdlApi::new().validate(program)`.
#[must_use]
pub fn validate(program: &HdlProgram) -> HdlValidationResult {
    validate_hdl(program)
}

/// Return the stable HDL API identifier.
#[must_use]
pub const fn api_name() -> &'static str {
    HDL_API_NAME
}

/// Return the HDL API schema version.
#[must_use]
pub const fn api_schema_version() -> u16 {
    HDL_API_SCHEMA_VERSION
}

// -----------------------------------------------------------------------------
// Public contract tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_identity_is_stable() {
        let api = HdlApi::new();

        assert_eq!(api.name(), "zamani.hdl.api");
        assert_eq!(api.schema_version(), 1);
        assert_eq!(api_name(), api.name());
        assert_eq!(api_schema_version(), api.schema_version());
    }

    #[test]
    fn api_is_stateless_and_default_constructible() {
        assert_eq!(HdlApi::new(), HdlApi::default());
        assert_eq!(HdlApi::new(), HdlApi);
    }

    #[test]
    fn empty_semantic_input_matches_the_canonical_validator() {
        let program = HdlProgram::default();

        assert_eq!(
            HdlApi::new().validate(&program),
            validate_hdl(&program)
        );
    }

    #[test]
    fn free_function_matches_the_canonical_validator() {
        let program = HdlProgram::default();

        assert_eq!(validate(&program), validate_hdl(&program));
    }

    #[test]
    fn api_preserves_incomplete_validation_status() {
        // The facade must not reinterpret or upgrade a canonical result.
        // The empty input's actual status is determined by the validator.
        let program = HdlProgram::default();
        let actual = HdlApi::new().validate(&program);
        let expected = validate_hdl(&program);

        assert_eq!(actual, expected);
        assert_eq!(actual.complete, expected.complete);
        assert_eq!(actual.status, expected.status);
        assert_eq!(actual.diagnostics, expected.diagnostics);
    }
}
