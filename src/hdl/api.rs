
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Public API
//!
//! Path: `src/hdl/api.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97.
//!
//! ## Purpose
//!
//! Provide the deliberate public entry point for HDL parsing and source-level
//! HDL semantic validation. This module connects existing frontend
//! implementations without duplicating their types or behavior.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/*.g4`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/model/`
//!
//! The grammar owns syntax. The canonical lexer owns tokens and source spans.
//! The canonical AST owns parsed source structure. The semantic subsystem
//! owns source-level validation. Target realization and HDLIR remain
//! separate responsibilities.
//!
//! ## Owns
//!
//! - The stable public HDL API entry point.
//! - Public re-exports of the existing HDL parser and semantic contracts.
//! - A stateless service façade for parser-session creation and validation.
//! - The version of this API contract.
//!
//! ## Does not own
//!
//! - Lexing, tokenization, or a replacement parser.
//! - A replacement AST, source-span type, or source map.
//! - A second semantic model, width expression system, or diagnostic system.
//! - HDLIR, IR builders, IR verification, passes, serialization, or lowering.
//! - Elaboration, synthesis, scheduling, placement, routing, or allocation.
//! - Device discovery, target selection, or physical feasibility decisions.
//! - A guarantee that every target can realize every valid source program.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Scalability and portability
//!
//! This API imposes no universal maximum on design size, number of modules,
//! ports, signals, processes, clocks, memories, or hardware devices.
//! Operational limits belong to the owning parser or analysis session.
//! Exhausting such a limit must be reported by that subsystem; it must not
//! silently convert incomplete work into a successful result.
//!
//! A finite implementation cannot process literally infinite input. The
//! portability goal is to avoid artificial language-level hardware ceilings
//! and preserve program meaning across supported targets.
//!
//! The existing semantic width contract represents a concrete width as
//! `u128`. This API deliberately preserves that canonical contract rather
//! than introducing a competing representation. If the language requires
//! concrete widths beyond `u128`, the canonical semantic width contract and
//! its consumers must be evolved together before claiming arbitrary-precision
//! concrete-width support.
//!
//! ## Validation semantics
//!
//! `validate_hdl` checks only the rules implemented by the existing HDL
//! semantic validator. A `Valid` result is not proof of whole-program
//! semantic validity, complete elaboration, target feasibility, successful
//! synthesis, or physical timing closure.
//!
//! The parser API consumes canonical frontend tokens. It does not tokenize
//! source text or promise to construct the canonical AST. AST adaptation and
//! shared frontend orchestration remain owned by their existing modules.
//!
//! ## Freeze criteria
//!
//! - Existing parser and semantic modules are registered and reachable.
//! - Public exports resolve to the canonical implementations.
//! - Parser errors remain distinguishable from semantic diagnostics.
//! - Incomplete validation is never presented as completed validation.
//! - No IR or target-realization dependency is introduced.
//! - Public API, integration, determinism, and conformance tests pass.
//! - Formatting, compilation, tests, and lint checks pass on Rust 1.97+.

/// Stable identifier for the Zamani HDL API.
pub const HDL_API_NAME: &str = "zamani.hdl.api";

/// Version of this public API contract.
///
/// This is independent of the language version, grammar version, semantic
/// model version, HDLIR version, and package version.
pub const HDL_API_SCHEMA_VERSION: u16 = 1;

// -----------------------------------------------------------------------------
// Existing canonical parser API
// -----------------------------------------------------------------------------

/// Existing HDL parser contracts.
///
/// These types are implemented by the frontend parser. Re-exporting them
/// makes them accessible through the HDL API without defining another parser
/// or another syntax representation.
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
// Existing canonical HDL semantic API
// -----------------------------------------------------------------------------

/// Existing HDL semantic contracts.
///
/// These are re-exports, not replacement definitions. The semantic validator
/// remains authoritative for the rules it implements.
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
// Stateless public façade
// -----------------------------------------------------------------------------

/// Stateless entry point for the existing HDL parser and semantic validator.
///
/// `HdlApi` owns no design data, parser cursor, global state, target handle,
/// or cached semantic result. Each parser session is independent, and
/// validation consumes an immutable reference to the caller's semantic input.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HdlApi;

impl HdlApi {
    /// Create the stateless HDL API façade.
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
    /// The parser borrows the supplied token slice. It does not tokenize
    /// source text, modify the token stream, or construct the canonical AST.
    /// The caller selects parser-safety budgets through `limits`.
    ///
    /// # Errors
    ///
    /// Returns the existing parser's structured error when the parser
    /// configuration is invalid.
    pub fn parse_session<'tokens, 'src>(
        &self,
        tokens: &'tokens [crate::frontend::lexer::token::Token<'src>],
        limits: HdlParserLimits,
    ) -> Result<HdlDomainParser<'tokens, 'src>, HdlParseError> {
        HdlDomainParser::new(tokens, limits)
    }

    /// Validate a normalized HDL semantic input using the canonical validator.
    ///
    /// The input remains owned by the caller and is not modified.
    ///
    /// A `Valid` result means only that the checks implemented by
    /// `validate_hdl` completed without semantic errors. It does not certify
    /// all frontend semantics or target feasibility.
    #[must_use]
    pub fn validate(&self, program: &HdlProgram) -> HdlValidationResult {
        validate_hdl(program)
    }
}

/// Validate a normalized HDL semantic input using the canonical validator.
///
/// This free function is a convenience entry point equivalent to
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
    fn empty_semantic_input_uses_the_canonical_validator() {
        let program = HdlProgram::default();
        let api_result = HdlApi::new().validate(&program);
        let direct_result = validate_hdl(&program);

        assert_eq!(api_result, direct_result);
    }

    #[test]
    fn free_function_uses_the_canonical_validator() {
        let program = HdlProgram::default();

        assert_eq!(validate(&program), validate_hdl(&program));
    }

    #[test]
    fn incomplete_and_invalid_are_not_relabelled_by_the_api() {
        // The façade forwards the canonical validator's result unchanged.
        // It must not reinterpret diagnostics or manufacture target success.
        let program = HdlProgram::default();
        let result = HdlApi::new().validate(&program);

        assert_eq!(result, validate_hdl(&program));
        assert_eq!(result.complete, result.status != HdlValidationStatus::Incomplete);
    }
}
