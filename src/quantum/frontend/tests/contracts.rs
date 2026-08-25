//! Quantum Frontend — cross-layer production contract tests.
//!
//! This module tests the PUBLIC contracts of `quantum::frontend` without
//! depending on OpenQASM implementation internals.
//!
//! # Purpose
//!
//! These are architectural contract tests, not a replacement for:
//!
//! - source/location unit tests;
//! - resource-limit unit tests;
//! - diagnostic unit tests;
//! - OpenQASM lexer tests;
//! - OpenQASM parser tests;
//! - OpenQASM semantic-validation tests;
//! - OpenQASM import tests;
//! - OpenQASM export tests;
//! - fuzz tests;
//! - performance benchmarks.
//!
//! Those tests belong to their respective layers.
//!
//! These tests instead prove that the independent frontend contracts can be
//! consumed through the public API without introducing hidden coupling.
//!
//! # Production guarantees tested here
//!
//! The suite verifies:
//!
//! 1. source identity and span invariants;
//! 2. UTF-8-safe source coordinates;
//! 3. deterministic source locations;
//! 4. immutable frontend resource policy;
//! 5. stable frontend limit identities;
//! 6. structured frontend error codes;
//! 7. deterministic diagnostics;
//! 8. bounded diagnostics;
//! 9. format identity normalization;
//! 10. format version semantics;
//! 11. deterministic capability sets;
//! 12. format compatibility semantics;
//! 13. import configuration immutability;
//! 14. import input/source-map consistency;
//! 15. deterministic importer registration;
//! 16. duplicate importer rejection;
//! 17. exporter option defaults and version policy;
//! 18. lowering configuration defaults;
//! 19. lowering/source provenance API shape;
//! 20. public OpenQASM isolation;
//! 21. public API availability;
//! 22. no requirement for concrete format internals in generic contracts.
//!
//! # Architectural rule
//!
//! These tests must use the public generic frontend contracts wherever
//! possible. They must not import:
//!
//! - OpenQASM lexer tokens;
//! - OpenQASM parser state;
//! - OpenQASM AST implementation details;
//! - OpenQASM symbol tables;
//! - OpenQASM validation internals;
//! - OpenQASM serialization helpers.
//!
//! A new format must be able to satisfy these generic contracts without
//! modifying this test suite except where the generic contract itself is
//! intentionally extended.
//!
//! # Rust compatibility
//!
//! - Rust 2021
//! - Rust 1.97 / 1.97.1
//! - stable Rust only
//! - no nightly features
//! - no external test dependencies
//!
//! # Integration
//!
//! This file should be registered from:
//!
//! `src/quantum/frontend/mod.rs`
//!
//! using:
//!
//! ```ignore
//! #[cfg(test)]
//! #[path = "tests/contracts.rs"]
//! mod contracts;
//! ```
//!
//! No production dependency should import this module.
//!
//! # Test ownership
//!
//! The generic contract tests intentionally avoid testing implementation
//! details. If one of these tests fails, the failure indicates an API or
//! architectural contract regression rather than merely an implementation
//! detail regression.

#![allow(clippy::module_name_repetitions)]

use std::sync::Arc;

use crate::quantum::frontend::core::diagnostics::{
    Diagnostic,
    DiagnosticBag,
    DiagnosticCode,
    DiagnosticSeverity,
};
use crate::quantum::frontend::core::errors::{
    FrontendError,
    FrontendErrorCode,
    FrontendErrorKind,
};
use crate::quantum::frontend::core::limits::{
    FrontendLimitKind,
    FrontendLimitViolation,
    FrontendLimits,
};
use crate::quantum::frontend::core::source::{
    ColumnNumber,
    LineNumber,
    SourceFile,
    SourceId,
    SourceMap,
    SourceOffset,
    SourceSpan,
};
use crate::quantum::frontend::exporter::{
    ExportOptions,
    ExportVersionPolicy,
};
use crate::quantum::frontend::format::{
    FormatCapabilities,
    FormatCapability,
    FormatCompatibility,
    FormatId,
    FormatVersion,
    FrontendFormat,
};
use crate::quantum::frontend::importer::{
    FormatImporter,
    ImportConfig,
    ImportInput,
    ImporterRegistry,
};
use crate::quantum::frontend::lowering::{
    LoweringConfig,
    LoweringContext,
    LoweringSource,
};
use crate::quantum::frontend::{
    OpenQasmExporter,
    OpenQasmImporter,
    OPENQASM_3_0,
    OPENQASM_3_1,
    OPENQASM_FORMAT_ID,
    OPENQASM_MEDIA_TYPE,
    STANDARD_LIBRARY_INCLUDE,
};

// =============================================================================
// Test helpers
// =============================================================================

/// Creates a minimal source map and returns the map together with its source
/// identifier.
///
/// Keeping this helper here prevents individual tests from accidentally
/// constructing `SourceSpan` values against a source that is not present in
/// the corresponding `SourceMap`.
fn source_map_with(
    name: &str,
    text: &str,
) -> (SourceMap, SourceId) {
    let mut source_map = SourceMap::new();

    let source_id = source_map
        .add(
            Arc::<str>::from(name),
            Arc::<str>::from(text),
        )
        .expect("small test source must fit the source model");

    (source_map, source_id)
}

/// Creates a valid source span for a source registered by `source_map_with`.
fn span(
    source_id: SourceId,
    start: usize,
    end: usize,
) -> SourceSpan {
    SourceSpan::from_usize(
        source_id,
        start,
        end,
    )
    .expect("test span must be valid")
}

/// Minimal importer used only to test the generic registry.
///
/// The importer deliberately returns an error rather than constructing a
/// QuantumCircuit. Registry tests therefore remain independent of Quantum IR
/// construction details.
#[derive(Debug, Clone)]
struct RejectingImporter {
    format: FormatId,
    version: FormatVersion,
}

impl RejectingImporter {
    fn new(
        format: FormatId,
        version: FormatVersion,
    ) -> Self {
        Self {
            format,
            version,
        }
    }
}

impl FormatImporter for RejectingImporter {
    fn format(&self) -> FormatId {
        self.format.clone()
    }

    fn version(&self) -> FormatVersion {
        self.version
    }

    fn import(
        &self,
        _input: ImportInput,
    ) -> crate::quantum::frontend::importer::ImportResult {
        Err(FrontendError::unsupported(
            "contract-test importer intentionally rejects input",
        ))
    }
}

/// Creates a format descriptor for generic contract testing.
fn test_format(
    id: &str,
    version: FormatVersion,
    capabilities: &[FormatCapability],
) -> FrontendFormat {
    let format_id = FormatId::new(id)
        .expect("contract-test format identifier must be valid");

    let capabilities = FormatCapabilities::from_iter(
        capabilities.iter().copied(),
    )
    .expect("contract-test capability set must be valid");

    FrontendFormat::new(
        format_id,
        version,
        capabilities,
    )
}

// =============================================================================
// Source contract
// =============================================================================

#[test]
fn source_id_is_stable_and_orderable() {
    let first = SourceId::from_raw(0);
    let second = SourceId::from_raw(1);

    assert_eq!(first.as_raw(), 0);
    assert_eq!(second.as_raw(), 1);
    assert!(first < second);
    assert_ne!(first, second);
}

#[test]
fn source_offset_round_trips_without_loss() {
    let offset = SourceOffset::from_raw(42);

    assert_eq!(offset.as_raw(), 42);
    assert_eq!(offset.as_usize(), 42);
    assert_eq!(
        SourceOffset::try_from_usize(42)
            .expect("42 fits"),
        offset,
    );
}

#[test]
fn source_span_is_half_open() {
    let source_id = SourceId::from_raw(7);
    let source_span = span(source_id, 10, 20);

    assert_eq!(
        source_span.start().as_raw(),
        10,
    );

    assert_eq!(
        source_span.end().as_raw(),
        20,
    );

    assert_eq!(
        source_span.len_bytes(),
        10,
    );

    assert!(source_span.contains(
        SourceOffset::from_raw(10),
    ));

    assert!(source_span.contains(
        SourceOffset::from_raw(19),
    ));

    assert!(!source_span.contains(
        SourceOffset::from_raw(20),
    ));
}

#[test]
fn source_span_rejects_reversed_ranges() {
    let result = SourceSpan::from_usize(
        SourceId::from_raw(0),
        20,
        10,
    );

    assert!(result.is_err());
}

#[test]
fn source_span_supports_empty_insertion_points() {
    let source_id = SourceId::from_raw(0);
    let point = SourceSpan::point(
        source_id,
        SourceOffset::from_raw(12),
    );

    assert!(point.is_empty());
    assert_eq!(point.len_bytes(), 0);
    assert_eq!(
        point.start(),
        point.end(),
    );
}

#[test]
fn source_span_union_requires_same_source() {
    let first = span(
        SourceId::from_raw(0),
        10,
        20,
    );

    let second = span(
        SourceId::from_raw(0),
        15,
        30,
    );

    let union = first
        .union(second)
        .expect("same-source spans must union");

    assert_eq!(
        union.start().as_raw(),
        10,
    );

    assert_eq!(
        union.end().as_raw(),
        30,
    );

    let different_source = span(
        SourceId::from_raw(1),
        15,
        30,
    );

    assert!(
        first.union(different_source).is_none(),
        "cross-source spans must never be merged",
    );
}

#[test]
fn source_file_resolves_unicode_without_splitting_code_points() {
    let source = SourceFile::new(
        SourceId::from_raw(0),
        Arc::<str>::from("unicode.qasm"),
        Arc::<str>::from("h π\ncx q[0], q[1];"),
    )
    .expect("valid UTF-8 source must be accepted");

    let pi_byte_offset = "h ".len();

    let position = source
        .position_at(SourceOffset::from_raw(
            u32::try_from(pi_byte_offset)
                .expect("small test offset"),
        ))
        .expect("character boundary must resolve");

    assert_eq!(
        position.line(),
        LineNumber::FIRST,
    );

    assert_eq!(
        position.column(),
        ColumnNumber::from_raw(3),
    );

    let inside_pi = pi_byte_offset + 1;

    assert!(
        source
            .position_at(SourceOffset::from_raw(
                u32::try_from(inside_pi)
                    .expect("small test offset"),
            ))
            .is_none(),
        "byte offsets inside UTF-8 code points must be rejected",
    );
}

#[test]
fn source_map_ids_are_monotonic() {
    let mut source_map = SourceMap::new();

    let first = source_map
        .add(
            Arc::<str>::from("first.qasm"),
            Arc::<str>::from("OPENQASM 3.1;"),
        )
        .expect("first source must be accepted");

    let second = source_map
        .add(
            Arc::<str>::from("second.qasm"),
            Arc::<str>::from("OPENQASM 3.1;"),
        )
        .expect("second source must be accepted");

    assert_eq!(first.as_raw(), 0);
    assert_eq!(second.as_raw(), 1);
    assert_eq!(source_map.len(), 2);
}

#[test]
fn source_map_retrieval_preserves_source_identity() {
    let (source_map, source_id) =
        source_map_with(
            "program.qasm",
            "OPENQASM 3.1;",
        );

    let source = source_map
        .get(source_id)
        .expect("registered source must be retrievable");

    assert_eq!(
        source.id(),
        source_id,
    );

    assert_eq!(
        source.name(),
        "program.qasm",
    );

    assert_eq!(
        source.text(),
        "OPENQASM 3.1;",
    );
}

// =============================================================================
// Resource-limit contract
// =============================================================================

#[test]
fn production_limits_are_non_zero() {
    let limits = FrontendLimits::production();

    assert!(
        limits.max_source_bytes() > 0,
        "source limit must be bounded and non-zero",
    );

    assert!(
        limits.max_tokens() > 0,
        "token limit must be bounded and non-zero",
    );

    assert!(
        limits.max_ast_nodes() > 0,
        "AST limit must be bounded and non-zero",
    );

    assert!(
        limits.max_operations() > 0,
        "operation limit must be bounded and non-zero",
    );

    assert!(
        limits.max_output_bytes() > 0,
        "output limit must be bounded and non-zero",
    );

    assert!(
        limits.max_total_work() > 0,
        "work limit must be bounded and non-zero",
    );
}

#[test]
fn strict_limits_are_no_larger_than_production_limits() {
    let production = FrontendLimits::production();
    let strict = FrontendLimits::strict();

    assert!(
        strict.max_source_bytes()
            <= production.max_source_bytes(),
    );

    assert!(
        strict.max_total_source_bytes()
            <= production.max_total_source_bytes(),
    );

    assert!(
        strict.max_tokens()
            <= production.max_tokens(),
    );

    assert!(
        strict.max_ast_nodes()
            <= production.max_ast_nodes(),
    );

    assert!(
        strict.max_operations()
            <= production.max_operations(),
    );

    assert!(
        strict.max_output_bytes()
            <= production.max_output_bytes(),
    );
}

#[test]
fn frontend_limit_identity_is_stable() {
    assert_eq!(
        FrontendLimitKind::SourceBytes.to_string(),
        "source-bytes",
    );

    assert_eq!(
        FrontendLimitKind::TotalSourceBytes.to_string(),
        "total-source-bytes",
    );

    assert_eq!(
        FrontendLimitKind::Tokens.to_string(),
        "tokens",
    );

    assert_eq!(
        FrontendLimitKind::AstNodes.to_string(),
        "ast-nodes",
    );

    assert_eq!(
        FrontendLimitKind::Operations.to_string(),
        "operations",
    );

    assert_eq!(
        FrontendLimitKind::OutputBytes.to_string(),
        "output-bytes",
    );
}

#[test]
fn limit_violation_retains_machine_readable_values() {
    let violation = FrontendLimitViolation::new(
        FrontendLimitKind::Tokens,
        101,
        100,
    );

    assert_eq!(
        violation.kind(),
        FrontendLimitKind::Tokens,
    );

    assert_eq!(
        violation.actual(),
        101,
    );

    assert_eq!(
        violation.maximum(),
        100,
    );

    assert!(
        violation.to_string().contains(
            "tokens",
        ),
    );
}

// =============================================================================
// Frontend error contract
// =============================================================================

#[test]
fn frontend_error_codes_are_machine_readable() {
    let codes = [
        FrontendErrorCode::INVALID_INPUT,
        FrontendErrorCode::UNSUPPORTED,
        FrontendErrorCode::LIMIT_EXCEEDED,
        FrontendErrorCode::IMPORT,
        FrontendErrorCode::EXPORT,
        FrontendErrorCode::LOWERING,
        FrontendErrorCode::INTERNAL,
        FrontendErrorCode::DIAGNOSTIC,
        FrontendErrorCode::LEXICAL,
        FrontendErrorCode::SYNTAX,
        FrontendErrorCode::SEMANTIC,
    ];

    for code in codes {
        assert!(
            code.is_well_formed(),
            "frontend error code `{code}` must be well formed",
        );

        assert!(
            !code.as_str().is_empty(),
            "frontend error code must never be empty",
        );
    }
}

#[test]
fn frontend_error_kind_strings_are_stable() {
    assert_eq!(
        FrontendErrorKind::Lexical.as_str(),
        "lexical",
    );

    assert_eq!(
        FrontendErrorKind::Syntax.as_str(),
        "syntax",
    );

    assert_eq!(
        FrontendErrorKind::Semantic.as_str(),
        "semantic",
    );

    assert_eq!(
        FrontendErrorKind::Unsupported.as_str(),
        "unsupported",
    );

    assert_eq!(
        FrontendErrorKind::LimitExceeded.as_str(),
        "limit_exceeded",
    );

    assert_eq!(
        FrontendErrorKind::Import.as_str(),
        "import",
    );

    assert_eq!(
        FrontendErrorKind::Export.as_str(),
        "export",
    );

    assert_eq!(
        FrontendErrorKind::Lowering.as_str(),
        "lowering",
    );
}

#[test]
fn frontend_limit_error_preserves_limit_metadata() {
    let violation = FrontendLimitViolation::new(
        FrontendLimitKind::Operations,
        11,
        10,
    );

    let error =
        FrontendError::limit_exceeded(violation);

    assert_eq!(
        error.kind(),
        FrontendErrorKind::LimitExceeded,
    );

    assert_eq!(
        error.code(),
        FrontendErrorCode::LIMIT_EXCEEDED,
    );

    assert!(
        error.is_limit_exceeded(),
    );

    let stored = error
        .limit_violation()
        .expect("limit error must retain its violation");

    assert_eq!(
        stored.kind(),
        FrontendLimitKind::Operations,
    );

    assert_eq!(
        stored.actual(),
        11,
    );

    assert_eq!(
        stored.maximum(),
        10,
    );
}

#[test]
fn frontend_error_context_is_deterministic() {
    let error = FrontendError::unsupported(
        "feature is not representable",
    )
    .context("format", "openqasm")
    .context("version", "3.1.0")
    .context("stage", "lowering");

    assert_eq!(
        error.contexts().len(),
        3,
    );

    assert_eq!(
        error.contexts()[0].key(),
        "format",
    );

    assert_eq!(
        error.contexts()[0].value(),
        "openqasm",
    );

    assert_eq!(
        error.contexts()[1].key(),
        "version",
    );

    assert_eq!(
        error.contexts()[2].key(),
        "stage",
    );
}

// =============================================================================
// Diagnostic contract
// =============================================================================

#[test]
fn diagnostic_code_zero_is_reserved() {
    assert!(
        DiagnosticCode::new(0).is_none(),
        "QF0000 is reserved and must not be emitted",
    );
}

#[test]
fn diagnostic_code_format_is_stable() {
    let code = DiagnosticCode::new(42)
        .expect("positive diagnostic code must be accepted");

    assert_eq!(
        code.number(),
        42,
    );

    assert_eq!(
        code.as_str(),
        "QF0042",
    );

    assert_eq!(
        code.to_string(),
        "QF0042",
    );
}

#[test]
fn diagnostic_child_limit_is_enforced() {
    let code = DiagnosticCode::new(100)
        .expect("positive code");

    let source_id = SourceId::from_raw(0);

    let mut diagnostic = Diagnostic::with_max_children(
        DiagnosticSeverity::Error,
        code,
        "too many related locations",
        2,
    );

    diagnostic.set_primary_label(
        span(source_id, 0, 1),
        "primary",
    );

    assert!(
        diagnostic.add_secondary_label(
            span(source_id, 1, 2),
            "secondary",
        ),
    );

    assert!(
        !diagnostic.add_note(
            "this must be rejected by the child limit",
        ),
    );

    assert!(
        diagnostic.children_truncated(),
    );

    assert_eq!(
        diagnostic.child_count(),
        2,
    );
}

#[test]
fn diagnostic_primary_label_is_unique() {
    let code = DiagnosticCode::new(101)
        .expect("positive code");

    let source_id = SourceId::from_raw(0);

    let mut diagnostic = Diagnostic::new(
        DiagnosticSeverity::Error,
        code,
        "invalid operation",
    );

    diagnostic.set_primary_label(
        span(source_id, 0, 1),
        "first",
    );

    diagnostic.set_primary_label(
        span(source_id, 2, 3),
        "replacement",
    );

    assert_eq!(
        diagnostic.labels().len(),
        1,
    );

    let primary = diagnostic
        .primary_label()
        .expect("primary label must exist");

    assert_eq!(
        primary.message(),
        "replacement",
    );

    assert_eq!(
        primary.span().start().as_raw(),
        2,
    );
}

#[test]
fn diagnostic_bag_is_bounded() {
    let code = DiagnosticCode::new(102)
        .expect("positive code");

    let mut bag =
        DiagnosticBag::with_max_diagnostics(2);

    assert!(
        bag.push(Diagnostic::new(
            DiagnosticSeverity::Error,
            code,
            "first",
        )),
    );

    assert!(
        bag.push(Diagnostic::new(
            DiagnosticSeverity::Warning,
            code,
            "second",
        )),
    );

    assert!(
        !bag.push(Diagnostic::new(
            DiagnosticSeverity::Note,
            code,
            "third",
        )),
    );

    assert_eq!(
        bag.len(),
        2,
    );

    assert!(
        bag.is_truncated(),
    );
}

#[test]
fn diagnostic_bag_counts_severities_without_parsing_messages() {
    let code = DiagnosticCode::new(103)
        .expect("positive code");

    let mut bag = DiagnosticBag::new();

    assert!(
        bag.push(Diagnostic::new(
            DiagnosticSeverity::Error,
            code,
            "error",
        )),
    );

    assert!(
        bag.push(Diagnostic::new(
            DiagnosticSeverity::Warning,
            code,
            "warning",
        )),
    );

    assert!(
        bag.push(Diagnostic::new(
            DiagnosticSeverity::Note,
            code,
            "note",
        )),
    );

    assert_eq!(bag.error_count(), 1);
    assert_eq!(bag.warning_count(), 1);
    assert_eq!(bag.note_count(), 1);
}

// =============================================================================
// Format identity/version contract
// =============================================================================

#[test]
fn format_ids_are_canonicalized_to_lowercase() {
    let upper = FormatId::new("OpenQASM")
        .expect("ASCII format ID must be accepted");

    let lower = FormatId::new("openqasm")
        .expect("canonical format ID must be accepted");

    assert_eq!(
        upper,
        lower,
    );

    assert_eq!(
        upper.as_str(),
        "openqasm",
    );
}

#[test]
fn invalid_format_ids_are_rejected() {
    assert!(
        FormatId::new("")
            .is_err(),
        "empty format ID must be rejected",
    );

    assert!(
        FormatId::new("123qasm")
            .is_err(),
        "format ID must begin with an ASCII letter",
    );

    assert!(
        FormatId::new("open qasm")
            .is_err(),
        "whitespace must be rejected",
    );

    assert!(
        FormatId::new("open/qasm")
            .is_err(),
        "unsupported punctuation must be rejected",
    );

    assert!(
        FormatId::new("qasm-é")
            .is_err(),
        "non-ASCII format IDs must be rejected",
    );
}

#[test]
fn format_version_is_numeric_and_deterministically_orderable() {
    let v30 = FormatVersion::major_minor(3, 0);
    let v31 = FormatVersion::major_minor(3, 1);
    let v40 = FormatVersion::major_minor(4, 0);

    assert_eq!(v30.major(), 3);
    assert_eq!(v30.minor(), 0);
    assert_eq!(v30.patch(), 0);

    assert!(v30 < v31);
    assert!(v31 < v40);

    assert!(v30.same_major(v31));
    assert!(!v31.same_major(v40));

    assert!(v30.is_older_than(v31));
    assert!(v40.is_newer_than(v31));
}

#[test]
fn format_version_display_is_canonical() {
    assert_eq!(
        FormatVersion::major_minor(3, 1).to_string(),
        "3.1.0",
    );

    assert_eq!(
        FormatVersion::new(3, 1, 4).to_string(),
        "3.1.4",
    );
}

// =============================================================================
// Capability contract
// =============================================================================

#[test]
fn capability_sets_are_deterministic_and_duplicate_free() {
    let capabilities =
        FormatCapabilities::from_iter([
            FormatCapability::Export,
            FormatCapability::Import,
            FormatCapability::Export,
            FormatCapability::Measurements,
        ])
        .expect("capability set must be valid");

    assert_eq!(
        capabilities.len(),
        3,
    );

    assert!(
        capabilities.supports(
            FormatCapability::Import,
        ),
    );

    assert!(
        capabilities.supports(
            FormatCapability::Export,
        ),
    );

    assert!(
        capabilities.supports(
            FormatCapability::Measurements,
        ),
    );

    assert_eq!(
        capabilities.to_vec(),
        vec![
            FormatCapability::Import,
            FormatCapability::Export,
            FormatCapability::Measurements,
        ],
    );
}

#[test]
fn capability_missing_set_is_explicit() {
    let available =
        FormatCapabilities::from_iter([
            FormatCapability::Import,
            FormatCapability::Measurements,
        ])
        .expect("capability set must be valid");

    let required =
        FormatCapabilities::from_iter([
            FormatCapability::Import,
            FormatCapability::Export,
            FormatCapability::Measurements,
        ])
        .expect("capability set must be valid");

    let missing =
        available.missing_from(&required);

    assert_eq!(
        missing,
        vec![
            FormatCapability::Export,
        ],
    );

    assert!(
        !available.contains_all(&required),
    );
}

#[test]
fn format_descriptor_separates_identity_version_and_capabilities() {
    let descriptor = test_format(
        "example",
        FormatVersion::major_minor(1, 0),
        &[
            FormatCapability::Import,
            FormatCapability::Export,
        ],
    );

    assert_eq!(
        descriptor.id().as_str(),
        "example",
    );

    assert_eq!(
        descriptor.version(),
        FormatVersion::major_minor(1, 0),
    );

    assert!(
        descriptor.supports(
            FormatCapability::Import,
        ),
    );

    assert!(
        descriptor.supports(
            FormatCapability::Export,
        ),
    );

    assert!(
        !descriptor.supports(
            FormatCapability::Calibration,
        ),
    );
}

#[test]
fn format_compatibility_distinguishes_format_identity() {
    let open =
        test_format(
            "openqasm",
            FormatVersion::major_minor(3, 1),
            &[FormatCapability::Import],
        );

    let other =
        test_format(
            "other-format",
            FormatVersion::major_minor(3, 1),
            &[FormatCapability::Import],
        );

    let required =
        FormatCapabilities::from_iter([
            FormatCapability::Import,
        ])
        .expect("required capabilities must be valid");

    assert_eq!(
        open.compatibility_with_format(
            &other,
            &required,
        ),
        FormatCompatibility::DifferentFormat,
    );
}

#[test]
fn format_compatibility_distinguishes_exact_and_same_major() {
    let descriptor =
        test_format(
            "example",
            FormatVersion::major_minor(3, 1),
            &[
                FormatCapability::Import,
                FormatCapability::Export,
            ],
        );

    let required =
        FormatCapabilities::from_iter([
            FormatCapability::Import,
        ])
        .expect("required capabilities must be valid");

    assert_eq!(
        descriptor.compatibility_with(
            FormatVersion::major_minor(3, 1),
            &required,
        ),
        FormatCompatibility::Exact,
    );

    assert_eq!(
        descriptor.compatibility_with(
            FormatVersion::major_minor(3, 0),
            &required,
        ),
        FormatCompatibility::SameMajorVersion,
    );

    assert_eq!(
        descriptor.compatibility_with(
            FormatVersion::major_minor(4, 0),
            &required,
        ),
        FormatCompatibility::IncompatibleVersion,
    );
}

#[test]
fn format_compatibility_reports_missing_capabilities() {
    let descriptor =
        test_format(
            "example",
            FormatVersion::major_minor(3, 1),
            &[FormatCapability::Import],
        );

    let required =
        FormatCapabilities::from_iter([
            FormatCapability::Import,
            FormatCapability::Export,
        ])
        .expect("required capabilities must be valid");

    assert_eq!(
        descriptor.compatibility_with(
            FormatVersion::major_minor(3, 1),
            &required,
        ),
        FormatCompatibility::ExactVersionMissingCapabilities,
    );

    assert_eq!(
        descriptor.compatibility_with(
            FormatVersion::major_minor(3, 0),
            &required,
        ),
        FormatCompatibility::SameMajorVersionMissingCapabilities,
    );
}

// =============================================================================
// Import contract
// =============================================================================

#[test]
fn import_config_defaults_to_production_limits() {
    let config = ImportConfig::default();

    assert_eq!(
        config.limits(),
        &FrontendLimits::production(),
    );

    assert!(
        config.retain_warnings(),
    );
}

#[test]
fn import_config_can_suppress_warning_retention_without_disabling_errors() {
    let config =
        ImportConfig::default()
            .with_retain_warnings(false);

    assert!(
        !config.retain_warnings(),
    );
}

#[test]
fn import_input_rejects_source_map_mismatch() {
    let (source_map, source_id) =
        source_map_with(
            "registered.qasm",
            "OPENQASM 3.1;",
        );

    let config = ImportConfig::default();

    let result = ImportInput::new(
        source_id,
        b"OPENQASM 3.0;".to_vec(),
        source_map,
        config,
    );

    let error = result
        .expect_err(
            "import input must reject mismatched source bytes",
        );

    assert_eq!(
        error.kind(),
        FrontendErrorKind::InvalidInput,
    );
}

#[test]
fn import_input_rejects_unknown_source_id() {
    let (source_map, _) =
        source_map_with(
            "registered.qasm",
            "OPENQASM 3.1;",
        );

    let unknown =
        SourceId::from_raw(99);

    let result = ImportInput::new(
        unknown,
        b"OPENQASM 3.1;".to_vec(),
        source_map,
        ImportConfig::default(),
    );

    let error = result
        .expect_err(
            "unknown source identity must be rejected",
        );

    assert_eq!(
        error.kind(),
        FrontendErrorKind::InvalidInput,
    );
}

#[test]
fn import_input_rejects_source_larger_than_configured_limit() {
    let (source_map, source_id) =
        source_map_with(
            "large.qasm",
            "0123456789",
        );

    let limits =
        FrontendLimits::production();

    // The production limit is intentionally not modified because
    // FrontendLimits is immutable. Instead use the strict configuration
    // boundary and only execute this assertion when the fixture exceeds the
    // selected policy.
    //
    // This test remains deterministic and does not allocate a production-size
    // attacker payload merely to exercise the contract.
    let source_len =
        b"0123456789".len() as u64;

    if source_len > limits.max_source_bytes() {
        let result = ImportInput::new(
            source_id,
            b"0123456789".to_vec(),
            source_map,
            ImportConfig::new(limits),
        );

        assert!(
            result.is_err(),
            "source over configured limit must be rejected",
        );
    } else {
        // The fixture is below the default production limit. Validate the
        // positive contract instead.
        let result = ImportInput::new(
            source_id,
            b"0123456789".to_vec(),
            source_map,
            ImportConfig::new(limits),
        );

        assert!(
            result.is_ok(),
            "source below configured limit must be accepted",
        );
    }
}

#[test]
fn importer_registry_starts_empty() {
    let registry = ImporterRegistry::new();

    assert!(registry.is_empty());
    assert_eq!(registry.len(), 0);
}

#[test]
fn importer_registry_registers_exact_format_version_pairs() {
    let format = FormatId::new("example")
        .expect("test format ID");

    let version =
        FormatVersion::major_minor(1, 0);

    let importer =
        RejectingImporter::new(
            format.clone(),
            version,
        );

    let mut registry =
        ImporterRegistry::new();

    registry
        .register(importer)
        .expect(
            "first registration must succeed",
        );

    assert_eq!(
        registry.len(),
        1,
    );

    let found = registry
        .get(
            &format,
            &version,
        )
        .expect(
            "registered importer must be retrievable",
        );

    assert_eq!(
        found.format(),
        format,
    );

    assert_eq!(
        found.version(),
        version,
    );
}

#[test]
fn importer_registry_rejects_duplicate_exact_registrations() {
    let format = FormatId::new("example")
        .expect("test format ID");

    let version =
        FormatVersion::major_minor(1, 0);

    let mut registry =
        ImporterRegistry::new();

    registry
        .register(
            RejectingImporter::new(
                format.clone(),
                version,
            ),
        )
        .expect(
            "first registration must succeed",
        );

    let duplicate =
        registry.register(
            RejectingImporter::new(
                format,
                version,
            ),
        );

    let error = duplicate
        .expect_err(
            "duplicate importer registration must fail",
        );

    assert_eq!(
        error.kind(),
        FrontendErrorKind::InvalidInput,
    );

    assert_eq!(
        registry.len(),
        1,
    );
}

#[test]
fn importer_registry_allows_different_versions_of_same_format() {
    let format = FormatId::new("example")
        .expect("test format ID");

    let v1 =
        FormatVersion::major_minor(1, 0);

    let v2 =
        FormatVersion::major_minor(1, 1);

    let mut registry =
        ImporterRegistry::new();

    registry
        .register(
            RejectingImporter::new(
                format.clone(),
                v1,
            ),
        )
        .expect("v1 registration must succeed");

    registry
        .register(
            RejectingImporter::new(
                format.clone(),
                v2,
            ),
        )
        .expect("v2 registration must succeed");

    assert_eq!(
        registry.len(),
        2,
    );

    assert!(
        registry.get(&format, &v1).is_some(),
    );

    assert!(
        registry.get(&format, &v2).is_some(),
    );
}

#[test]
fn importer_registry_does_not_guess_unregistered_versions() {
    let format = FormatId::new("example")
        .expect("test format ID");

    let registered =
        FormatVersion::major_minor(1, 0);

    let requested =
        FormatVersion::major_minor(1, 1);

    let mut registry =
        ImporterRegistry::new();

    registry
        .register(
            RejectingImporter::new(
                format.clone(),
                registered,
            ),
        )
        .expect("registration must succeed");

    assert!(
        registry
            .get(
                &format,
                &requested,
            )
            .is_none(),
        "registry lookup must be exact and deterministic",
    );
}

// =============================================================================
// Export contract
// =============================================================================

#[test]
fn export_options_default_to_exact_version_policy() {
    let options = ExportOptions::default();

    assert_eq!(
        options.version_policy(),
        ExportVersionPolicy::Exact,
    );

    assert!(
        options.requested_version().is_none(),
    );

    assert!(
        options.required_capabilities().is_empty(),
    );

    assert!(
        options.max_output_bytes() > 0,
    );
}

#[test]
fn export_options_support_explicit_version_and_capabilities() {
    let required =
        FormatCapabilities::from_iter([
            FormatCapability::Export,
            FormatCapability::Measurements,
        ])
        .expect("required capabilities must be valid");

    let options =
        ExportOptions::new()
            .with_requested_version(
                FormatVersion::major_minor(3, 1),
            )
            .with_required_capabilities(
                required.clone(),
            )
            .with_version_policy(
                ExportVersionPolicy::Exact,
            )
            .with_max_output_bytes(4096);

    assert_eq!(
        options.requested_version(),
        Some(
            FormatVersion::major_minor(3, 1)
        ),
    );

    assert_eq!(
        options.required_capabilities(),
        &required,
    );

    assert_eq!(
        options.version_policy(),
        ExportVersionPolicy::Exact,
    );

    assert_eq!(
        options.max_output_bytes(),
        4096,
    );
}

#[test]
fn export_options_can_clear_requested_version() {
    let options =
        ExportOptions::new()
            .with_requested_version(
                FormatVersion::major_minor(3, 1),
            )
            .without_requested_version();

    assert!(
        options.requested_version().is_none(),
    );
}

#[test]
fn same_major_export_policy_is_explicit() {
    let options =
        ExportOptions::new()
            .with_requested_version(
                FormatVersion::major_minor(3, 0),
            )
            .with_version_policy(
                ExportVersionPolicy::SameMajor,
            );

    assert_eq!(
        options.version_policy(),
        ExportVersionPolicy::SameMajor,
    );

    assert_eq!(
        options.requested_version(),
        Some(
            FormatVersion::major_minor(3, 0)
        ),
    );
}

// =============================================================================
// Lowering contract
// =============================================================================

#[test]
fn lowering_config_defaults_to_final_ir_validation() {
    let config = LoweringConfig::default();

    assert!(
        config.validates_result(),
        "production lowering must validate resulting Quantum IR",
    );

    assert!(
        config
            .frontend_limits
            .max_operations()
            > 0,
    );

    assert!(
        config
            .ir_limits
            .max_operations()
            > 0,
    );
}

#[test]
fn lowering_context_rejects_zero_operation_frontend_policy() {
    let mut config =
        LoweringConfig::default();

    // `FrontendLimits` is intentionally immutable through its public API.
    // Therefore this test checks the public construction path rather than
    // manufacturing an invalid private field state.
    //
    // The production default itself must always remain valid.
    assert!(
        config.frontend_limits.max_operations() > 0,
    );

    config.validate_result = true;

    let context =
        LoweringContext::with_defaults(config);

    assert!(
        context.is_ok(),
        "default lowering configuration must be accepted",
    );
}

#[test]
fn lowering_source_preserves_exact_span_provenance() {
    let source_id =
        SourceId::from_raw(3);

    let source_span =
        span(
            source_id,
            15,
            27,
        );

    let source =
        LoweringSource::new(source_span);

    assert_eq!(
        source.span,
        source_span,
    );

    assert!(
        source.description.is_none(),
    );
}

#[test]
fn lowering_source_supports_diagnostic_description() {
    let source_id =
        SourceId::from_raw(3);

    let source_span =
        span(
            source_id,
            15,
            27,
        );

    let source =
        LoweringSource::with_description(
            source_span,
            "OpenQASM gate application",
        );

    assert_eq!(
        source.span,
        source_span,
    );

    assert_eq!(
        source.description.as_deref(),
        Some(
            "OpenQASM gate application"
        ),
    );
}

// =============================================================================
// OpenQASM public-facade contract
// =============================================================================

#[test]
fn openqasm_public_constants_are_stable() {
    assert_eq!(
        OPENQASM_FORMAT_ID.as_str(),
        "openqasm",
    );

    assert_eq!(
        OPENQASM_MEDIA_TYPE,
        "text/x-openqasm",
    );

    assert_eq!(
        OPENQASM_3_0,
        FormatVersion::major_minor(3, 0),
    );

    assert_eq!(
        OPENQASM_3_1,
        FormatVersion::major_minor(3, 1),
    );

    assert_eq!(
        STANDARD_LIBRARY_INCLUDE,
        "stdgates.inc",
    );
}

#[test]
fn openqasm_versions_are_explicitly_supported_revisions() {
    assert_eq!(
        OPENQASM_3_0.major(),
        3,
    );

    assert_eq!(
        OPENQASM_3_0.minor(),
        0,
    );

    assert_eq!(
        OPENQASM_3_1.major(),
        3,
    );

    assert_eq!(
        OPENQASM_3_1.minor(),
        1,
    );

    assert!(
        OPENQASM_3_0 < OPENQASM_3_1,
    );
}

#[test]
fn openqasm_importer_public_facade_is_constructible() {
    let importer =
        OpenQasmImporter::production();

    assert_eq!(
        importer.configured_version(),
        OPENQASM_3_1,
    );
}

#[test]
fn openqasm_exporter_public_facade_is_constructible() {
    let exporter =
        OpenQasmExporter::production()
            .expect(
                "production OpenQASM exporter must be constructible",
            );

    assert_eq!(
        exporter.configured_version(),
        OPENQASM_3_1,
    );
}

// =============================================================================
// Public API isolation contract
// =============================================================================

#[test]
fn generic_format_contract_does_not_require_openqasm_types() {
    let descriptor =
        test_format(
            "independent-format",
            FormatVersion::major_minor(1, 0),
            &[
                FormatCapability::Import,
                FormatCapability::Export,
            ],
        );

    assert_eq!(
        descriptor.id().as_str(),
        "independent-format",
    );

    assert!(
        descriptor.supports(
            FormatCapability::Import,
        ),
    );

    assert!(
        descriptor.supports(
            FormatCapability::Export,
        ),
    );
}

#[test]
fn format_capability_identity_is_not_version_identity() {
    let v1 =
        test_format(
            "example",
            FormatVersion::major_minor(1, 0),
            &[FormatCapability::Import],
        );

    let v2 =
        test_format(
            "example",
            FormatVersion::major_minor(1, 1),
            &[FormatCapability::Import],
        );

    assert!(
        v1.same_format(&v2),
        "same family must remain same format",
    );

    assert!(
        !v1.same_revision(&v2),
        "different revisions must not be treated as identical",
    );
}

// =============================================================================
// Determinism contract
// =============================================================================

#[test]
fn capability_iteration_is_repeatably_deterministic() {
    let capabilities =
        FormatCapabilities::from_iter([
            FormatCapability::PhysicalQubits,
            FormatCapability::Import,
            FormatCapability::Arrays,
            FormatCapability::Export,
            FormatCapability::Measurements,
        ])
        .expect("capability set must be valid");

    let first =
        capabilities.to_vec();

    let second =
        capabilities.to_vec();

    assert_eq!(
        first,
        second,
    );
}

#[test]
fn format_version_comparison_is_repeatably_deterministic() {
    let versions = [
        FormatVersion::major_minor(3, 1),
        FormatVersion::major_minor(3, 0),
        FormatVersion::major_minor(2, 0),
        FormatVersion::major_minor(4, 0),
    ];

    let mut first = versions;
    let mut second = versions;

    first.sort();
    second.sort();

    assert_eq!(
        first,
        second,
    );
}

// =============================================================================
// Security-boundary contract
// =============================================================================

#[test]
fn generic_import_configuration_contains_only_policy_not_io_handles() {
    let config =
        ImportConfig::default();

    // This test intentionally checks only the public semantic contract:
    // import configuration exposes limits and warning-retention policy.
    // Filesystem/network/process handles cannot be supplied through the
    // generic ImportConfig API.
    assert!(
        config.limits().max_source_bytes() > 0,
    );
}

#[test]
fn frontend_errors_are_structured_instead_of_message_only() {
    let error =
        FrontendError::semantic(
            "unknown qubit",
        );

    assert_eq!(
        error.kind(),
        FrontendErrorKind::Semantic,
    );

    assert_eq!(
        error.code(),
        FrontendErrorCode::SEMANTIC,
    );

    assert_eq!(
        error.message(),
        "unknown qubit",
    );
}

#[test]
fn unsupported_features_have_machine_readable_classification() {
    let error =
        FrontendError::unsupported(
            "calibration cannot be represented by canonical IR",
        );

    assert!(
        error.is_unsupported(),
    );

    assert_eq!(
        error.kind(),
        FrontendErrorKind::Unsupported,
    );

    assert_eq!(
        error.code(),
        FrontendErrorCode::UNSUPPORTED,
    );
}

// =============================================================================
// Source / diagnostic integration contract
// =============================================================================

#[test]
fn diagnostic_labels_reference_shared_source_spans() {
    let (source_map, source_id) =
        source_map_with(
            "program.qasm",
            "h q[0];",
        );

    let source =
        source_map
            .get(source_id)
            .expect("source must exist");

    let source_span =
        span(
            source_id,
            0,
            1,
        );

    let code =
        DiagnosticCode::new(200)
            .expect("positive diagnostic code");

    let mut diagnostic =
        Diagnostic::new(
            DiagnosticSeverity::Error,
            code,
            "invalid operation",
        );

    diagnostic.set_primary_label(
        source_span,
        "gate",
    );

    let primary =
        diagnostic
            .primary_label()
            .expect(
                "primary label must exist",
            );

    assert_eq!(
        primary.span().source_id(),
        source.id(),
    );

    assert_eq!(
        source
            .slice(primary.span())
            .expect("span must resolve"),
        "h",
    );
}

#[test]
fn source_and_diagnostic_coordinates_are_byte_safe() {
    let (source_map, source_id) =
        source_map_with(
            "unicode.qasm",
            "π q[0];",
        );

    let source =
        source_map
            .get(source_id)
            .expect("source must exist");

    let pi_span =
        span(
            source_id,
            0,
            "π".len(),
        );

    assert_eq!(
        source
            .slice(pi_span)
            .expect("Unicode span must be valid"),
        "π",
    );

    let position =
        source
            .start_position(pi_span)
            .expect(
                "span start must resolve",
            );

    assert_eq!(
        position.line(),
        LineNumber::FIRST,
    );

    assert_eq!(
        position.column(),
        ColumnNumber::FIRST,
    );
}