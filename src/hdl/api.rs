
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Public API
//!
//! Path: `src/hdl/api.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provide a stable public entry point to the existing HDL parser and
//! semantic validator without creating competing implementations.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/hdl/hdl.g4` and applicable delegate grammars
//! - `grammar/spec/hdl.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar owns syntax. The canonical frontend owns tokenization,
//! parsing, AST construction, source locations, and semantic analysis.
//! ZUIR owns the universal computational representation. HDLIR owns the
//! hardware-domain intermediate representation.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The public HDL API identity and contract version.
//! - Deliberate re-exports of canonical HDL contracts.
//! - A stateless facade for parser-session creation and validation.
//!
//! This module does not own:
//!
//! - Lexing, parsing algorithms, or AST construction.
//! - Source-level name resolution or a competing semantic model.
//! - HDLIR construction, verification, transformation, or serialization.
//! - ZUIR implementation or representation conversion.
//! - Elaboration, synthesis, scheduling, placement, or routing.
//! - Target discovery, physical allocation, or device handles.
//!
//! All HDLIR implementation must remain under `src/hdl/hdlir/`.
//!
//! ## Scalability
//!
//! This facade imposes no universal hardware-capacity limits. Parser
//! safety budgets remain configurable operational policies and do not
//! redefine source-language semantics.
//!
//! Finite compiler processes remain subject to available resources,
//! representational limits, supported semantics, and explicit budgets.
//! Incomplete work must never be reported as completed validation.
//!
//! The canonical validator currently represents concrete HDL widths
//! with `u128`. This API preserves that existing contract; it does not
//! claim arbitrary-precision concrete-width support.
//!
//! ## Parsing contract
//!
//! `parse_session` borrows an existing canonical token stream. It does
//! not tokenize source text, own the source, or construct the canonical
//! AST. The shared frontend remains responsible for parser orchestration,
//! AST adaptation, and whole-program parse-result reporting.
//!
//! A successful `parse_next` operation on the returned parser means
//! that one construct was recognized by that domain parser. It does
//! not establish that the entire source file parsed successfully.
//!
//! ## Validation contract
//!
//! `validate` forwards the existing semantic validator's result without
//! upgrading its status or discarding its diagnostics.
//!
//! A `Valid` result means only that the checks implemented by the
//! canonical HDL validator succeeded. It does not certify all language
//! semantics, elaboration, target feasibility, synthesis, or timing
//! closure.
//!
//! ## Freeze criteria
//!
//! - The canonical parser and semantic validator remain authoritative.
//! - Public re-exports compile through the registered module tree.
//! - Parsing and validation errors remain distinct.
//! - Incomplete validation is never treated as success.
//! - No HDLIR or target-realization implementation is introduced here.
//! - Public API tests and repository integration checks pass.

/// Stable identifier for the Zamani HDL public API.
pub const HDL_API_NAME: &str = "zamani.hdl.api";

/// Version of this public API contract.
///
/// This version is independent of the language, grammar, frontend
/// semantic model, crate, HDLIR artifact, and target versions.
pub const HDL_API_SCHEMA_VERSION: u16 = 1;

// -----------------------------------------------------------------------------
// Canonical parser contracts
// -----------------------------------------------------------------------------

/// Canonical HDL parser contracts.
///
/// These types are re-exported without replacement implementations.
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

/// Canonical HDL semantic-validation contracts.
///
/// The public facade forwards these types and functions without defining
/// a second semantic representation or diagnostic system.
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

/// Stateless entry point to the canonical HDL parser and validator.
///
/// `HdlApi` owns no design, token stream, parser cursor, global mutable
/// state, target handle, or cached semantic result.
///
/// Each parser session owns its own cursor and borrows its token stream.
/// Validation borrows caller-owned semantic input.
///
/// Because the facade contains no mutable state, callers may construct
/// independent instances wherever needed.
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

    /// Return the version of this API contract.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        HDL_API_SCHEMA_VERSION
    }

    /// Create an HDL parser session over canonical frontend tokens.
    ///
    /// The returned parser borrows `tokens`. The original token stream
    /// remains owned by its caller and is not re-tokenized or removed.
    ///
    /// `limits` configures parser safety budgets. These budgets are
    /// operational safeguards, not language-level hardware ceilings.
    ///
    /// The caller remains responsible for invoking the parser and
    /// integrating its loss-preserving syntax with the canonical AST
    /// through the appropriate frontend parser adapter.
    ///
    /// # Errors
    ///
    /// Returns a structured parser error if the parser configuration is
    /// invalid.
    pub fn parse_session<'tokens, 'src>(
        &self,
        tokens: &'tokens [
            crate::frontend::lexer::token::Token<'src>
        ],
        limits: HdlParserLimits,
    ) -> Result<HdlDomainParser<'tokens, 'src>, HdlParseError> {
        HdlDomainParser::new(tokens, limits)
    }

    /// Validate normalized HDL semantic input.
    ///
    /// This method forwards the canonical validator's result unchanged.
    /// The program remains owned by the caller and is not modified.
    ///
    /// The result describes only the checks implemented by
    /// `validate_hdl`. It does not establish whole-program semantic
    /// validity, target feasibility, synthesis success, or timing closure.
    #[must_use]
    pub fn validate(&self, program: &HdlProgram) -> HdlValidationResult {
        validate_hdl(program)
    }
}

// -----------------------------------------------------------------------------
// Convenience functions
// -----------------------------------------------------------------------------

/// Validate normalized HDL semantic input.
///
/// This function is equivalent to `HdlApi::new().validate(program)`.
/// It forwards the canonical result without altering its status or
/// diagnostics.
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
// Public API contract tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_identity_is_stable() {
        let api = HdlApi::new();

        assert_eq!(api.name(), HDL_API_NAME);
        assert_eq!(api.schema_version(), HDL_API_SCHEMA_VERSION);
        assert_eq!(api_name(), api.name());
        assert_eq!(api_schema_version(), api.schema_version());
    }

    #[test]
    fn api_is_stateless_and_default_constructible() {
        assert_eq!(HdlApi::new(), HdlApi::default());
        assert_eq!(HdlApi::new(), HdlApi);
    }

    #[test]
    fn default_parser_limits_are_accepted() {
        let api = HdlApi::new();
        let tokens = [];

        let parser = api.parse_session(&tokens, HdlParserLimits::default());

        assert!(parser.is_ok());
    }

    #[test]
    fn zero_nesting_budget_is_rejected() {
        let api = HdlApi::new();
        let tokens = [];
        let limits = HdlParserLimits {
            max_nesting_depth: 0,
            ..HdlParserLimits::default()
        };

        let result = api.parse_session(&tokens, limits);

        assert!(matches!(
            result,
            Err(HdlParseError {
                kind: HdlParseErrorKind::InvalidConfiguration,
                ..
            })
        ));
    }

    #[test]
    fn zero_construct_token_budget_is_rejected() {
        let api = HdlApi::new();
        let tokens = [];
        let limits = HdlParserLimits {
            max_tokens_per_construct: 0,
            ..HdlParserLimits::default()
        };

        let result = api.parse_session(&tokens, limits);

        assert!(matches!(
            result,
            Err(HdlParseError {
                kind: HdlParseErrorKind::InvalidConfiguration,
                ..
            })
        ));
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
    fn validation_status_and_diagnostics_are_forwarded_unchanged() {
        let program = HdlProgram::default();

        let actual = HdlApi::new().validate(&program);
        let expected = validate_hdl(&program);

        assert_eq!(actual.status, expected.status);
        assert_eq!(actual.complete, expected.complete);
        assert_eq!(actual.diagnostics, expected.diagnostics);
    }

    #[test]
    fn validation_preserves_invalid_input_results() {
        let id = HdlId::new("signal_a")
            .expect("a non-empty HDL identity should be accepted");

        let program = HdlProgram {
            signals: vec![
                HdlSignal {
                    id: id.clone(),
                    name: "a".to_owned(),
                    direction: HdlDirection::Internal,
                    value_type: HdlValueType::Bit,
                    location: None,
                },
                HdlSignal {
                    id,
                    name: "b".to_owned(),
                    direction: HdlDirection::Internal,
                    value_type: HdlValueType::Bit,
                    location: None,
                },
            ],
            ..HdlProgram::default()
        };

        let actual = HdlApi::new().validate(&program);
        let expected = validate_hdl(&program);

        assert_eq!(actual, expected);
        assert_eq!(actual.status, HdlValidationStatus::Invalid);
        assert!(actual.complete);
        assert!(actual.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == HdlDiagnosticCode::DuplicateIdentity
        }));
    }
}
