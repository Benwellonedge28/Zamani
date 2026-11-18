
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain Diagnostics
//!
//! Public composition root for the Zamani toolchain diagnostic system.
//!
//! Rust edition: 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provides one stable diagnostic interface across the Zamani toolchain.
//!
//! Diagnostics can originate from project management, workspace
//! discovery, manifests, dependency resolution, build orchestration,
//! task execution, packages, registries, external tools, targets,
//! security, deployment and future registered providers.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Diagnostic subsystem composition.
//! - Public diagnostic API exports.
//! - Public aggregation API exports.
//!
//! It does not own:
//!
//! - Diagnostic record implementation.
//! - Error classification.
//! - Frontend or compiler diagnostic definitions.
//! - Source maps or source-coordinate calculation.
//! - Aggregation algorithms.
//! - Deduplication or sorting algorithms.
//! - Diagnostic rendering.
//! - Report serialization.
//! - Operation execution or recovery.
//!
//! ## Canonical authorities
//!
//! Diagnostic records:
//! `diagnostic.rs`
//!
//! Diagnostic aggregation:
//! `aggregation.rs`
//!
//! Toolchain errors:
//! `crate::toolchain::errors`
//!
//! Toolchain results:
//! `crate::toolchain::result`
//!
//! Canonical source locations:
//! `crate::frontend::ast::node::source::span`
//!
//! Normative diagnostic specification:
//! `grammar/spec/diagnostics.md`
//!
//! Normative semantic model:
//! `grammar/specification/semantic-model.md`
//!
//! Semantic boundaries:
//! `grammar/validation/semantic-boundaries.md`
//!
//! ## Computing-domain integration
//!
//! The diagnostic API is open to classical, quantum, HDL, AI, tensor,
//! hybrid, distributed, networking, cryptographic and future providers.
//!
//! Adding a computing domain must not require modifying this module.
//!
//! Each originating subsystem remains responsible for its own
//! diagnostic meaning and provenance.
//!
//! ## Scalability
//!
//! This module introduces no artificial diagnostic-count,
//! source-size, project-size, domain-count, target-count,
//! processor-count, accelerator-count, qubit-count or
//! distributed-participant ceiling.
//!
//! The default aggregation policy and optional retention budgets
//! are owned by aggregation.rs.
//!
//! Operational budgets do not redefine language capabilities.
//!
//! ## Completeness
//!
//! Diagnostic collection completeness is independent of operation
//! completion.
//!
//! An empty diagnostic collection does not establish that a build,
//! package operation or deployment succeeded.
//!
//! Consumers must inspect both the operation result and aggregation
//! status.
//!
//! Allocation failures, dropped records and invalid diagnostics
//! must not be represented as complete aggregation.
//!
//! ## Determinism
//!
//! Deterministic ordering and exact-record deduplication are owned
//! by aggregation.rs.
//!
//! This module introduces no competing ordering or identity policy.
//!
//! ## Security
//!
//! Diagnostic metadata and external provider messages are untrusted
//! data and must be validated by their respective consumers.
//!
//! Credentials and other secrets must not be included in diagnostic
//! messages or metadata.
//!
//! Suggested fixes are informational and are never applied here.
//!
//! ## Integration
//!
//! Standard imports:
//!
//! `crate::toolchain::diagnostics::*`
//!
//! Advanced imports:
//!
//! `crate::toolchain::diagnostics::diagnostic::*`
//!
//! `crate::toolchain::diagnostics::aggregation::*`

// ============================================================================
// AUTHORITATIVE SUBMODULES
// ============================================================================

/// Canonical structured toolchain diagnostic records.
///
/// Owns diagnostic codes, severity, categories, phases, provenance,
/// source labels, notes, proposed fixes, metadata and validation.
///
/// Reuses the canonical frontend source Span.
pub mod diagnostic;

/// Canonical diagnostic aggregation infrastructure.
///
/// Owns aggregation policies, retention budgets, completeness,
/// statistics, exact duplicate detection and deterministic ordering.
pub mod aggregation;

// ============================================================================
// DIAGNOSTIC RECORD API
// ============================================================================

/// Stable machine-readable diagnostic code.
pub use self::diagnostic::DiagnosticCode;

/// Diagnostic severity.
pub use self::diagnostic::DiagnosticSeverity;

/// Open-world diagnostic category.
pub use self::diagnostic::DiagnosticCategory;

/// Toolchain lifecycle phase.
pub use self::diagnostic::DiagnosticPhase;

/// Diagnostic origin and provenance.
pub use self::diagnostic::DiagnosticOrigin;

/// Structured diagnostic metadata.
pub use self::diagnostic::DiagnosticValue;

/// Secondary source label.
pub use self::diagnostic::DiagnosticLabel;

/// Additional explanatory note.
pub use self::diagnostic::DiagnosticNote;

/// Proposed source correction.
pub use self::diagnostic::DiagnosticFix;

/// Individual proposed source edit.
pub use self::diagnostic::DiagnosticFixEdit;

/// Diagnostic record validation error.
pub use self::diagnostic::DiagnosticValidationError;

/// Canonical structured toolchain diagnostic.
pub use self::diagnostic::ToolchainDiagnostic;

// ============================================================================
// AGGREGATION API
// ============================================================================

/// Diagnostic aggregation policy.
pub use self::aggregation::AggregationPolicy;

/// Aggregation completeness status.
pub use self::aggregation::AggregationStatus;

/// Aggregation statistics.
pub use self::aggregation::AggregationStatistics;

/// Result of submitting a diagnostic.
pub use self::aggregation::InsertOutcome;

/// Collection associated with an allocation failure.
pub use self::aggregation::AllocationCollection;

/// Aggregation error.
pub use self::aggregation::AggregationError;

/// Completed diagnostic aggregation.
pub use self::aggregation::AggregatedToolchainDiagnostics;

/// Diagnostic aggregation service.
pub use self::aggregation::ToolchainDiagnosticAggregator;

/// Deterministic diagnostic ordering.
pub use self::aggregation::sort_diagnostics;

/// Canonical diagnostic comparison.
pub use self::aggregation::compare_diagnostics;

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_diagnostic(
        code: &'static str,
        severity: DiagnosticSeverity,
    ) -> ToolchainDiagnostic {
        ToolchainDiagnostic::new(
            code,
            severity,
            DiagnosticCategory::new("build"),
            DiagnosticPhase::new("build.validation"),
            "Test diagnostic",
            DiagnosticOrigin::new("toolchain.testing")
                .with_operation("diagnostic.test"),
        )
    }

    #[test]
    fn public_api_exposes_canonical_diagnostic_schema() {
        assert_eq!(
            ToolchainDiagnostic::schema_id(),
            "zamani.toolchain.diagnostic.v1"
        );
    }

    #[test]
    fn diagnostic_construction_uses_canonical_model() {
        let diagnostic = sample_diagnostic(
            "toolchain.test.failure",
            DiagnosticSeverity::Error,
        );

        assert!(diagnostic.validate().is_ok());

        assert_eq!(
            diagnostic.code().as_str(),
            "toolchain.test.failure"
        );
    }

    #[test]
    fn default_aggregator_accepts_valid_diagnostics() {
        let mut aggregator = ToolchainDiagnosticAggregator::new();

        let outcome = aggregator
            .push(sample_diagnostic(
                "toolchain.test.failure",
                DiagnosticSeverity::Error,
            ))
            .expect("valid diagnostic should be accepted");

        assert_eq!(outcome, InsertOutcome::Inserted);

        let result = aggregator.finish();

        assert!(result.is_complete());
        assert_eq!(result.len(), 1);
        assert_eq!(result.statistics().retained(), 1);
    }

    #[test]
    fn exact_duplicates_are_deduplicated() {
        let mut aggregator = ToolchainDiagnosticAggregator::new();

        let diagnostic = sample_diagnostic(
            "toolchain.test.duplicate",
            DiagnosticSeverity::Warning,
        );

        assert_eq!(
            aggregator.push(diagnostic.clone()).unwrap(),
            InsertOutcome::Inserted
        );

        assert_eq!(
            aggregator.push(diagnostic).unwrap(),
            InsertOutcome::Duplicate
        );

        let result = aggregator.finish();

        assert_eq!(result.len(), 1);
        assert_eq!(result.statistics().duplicates(), 1);
        assert!(result.is_complete());
    }

    #[test]
    fn retention_exhaustion_marks_aggregation_incomplete() {
        let policy = AggregationPolicy::with_max_diagnostics(0);

        let mut aggregator =
            ToolchainDiagnosticAggregator::with_policy(policy);

        assert_eq!(
            aggregator
                .push(sample_diagnostic(
                    "toolchain.test.retention",
                    DiagnosticSeverity::Warning,
                ))
                .unwrap(),
            InsertOutcome::DroppedByPolicy
        );

        let result = aggregator.finish();

        assert!(result.is_empty());
        assert!(!result.is_complete());
        assert_eq!(result.statistics().dropped(), 1);
    }

    #[test]
    fn invalid_diagnostics_are_rejected() {
        let diagnostic = ToolchainDiagnostic::new(
            "",
            DiagnosticSeverity::Error,
            "build",
            "build.validation",
            "Invalid diagnostic",
            DiagnosticOrigin::new("toolchain.testing"),
        );

        let mut aggregator = ToolchainDiagnosticAggregator::new();

        assert!(matches!(
            aggregator.push(diagnostic),
            Err(AggregationError::InvalidDiagnostic(
                DiagnosticValidationError::EmptyCode
            ))
        ));

        let result = aggregator.finish();

        assert!(!result.is_complete());
        assert_eq!(result.statistics().invalid(), 1);
    }

    #[test]
    fn canonical_sorting_is_exposed() {
        let mut diagnostics = vec![
            sample_diagnostic(
                "toolchain.test.warning",
                DiagnosticSeverity::Warning,
            ),
            sample_diagnostic(
                "toolchain.test.error",
                DiagnosticSeverity::Error,
            ),
            sample_diagnostic(
                "toolchain.test.fatal",
                DiagnosticSeverity::Fatal,
            ),
        ];

        sort_diagnostics(&mut diagnostics);

        assert_eq!(
            diagnostics[0].severity(),
            DiagnosticSeverity::Fatal
        );

        assert_eq!(
            diagnostics[1].severity(),
            DiagnosticSeverity::Error
        );

        assert_eq!(
            diagnostics[2].severity(),
            DiagnosticSeverity::Warning
        );
    }

    #[test]
    fn direct_submodule_imports_remain_available() {
        let diagnostic = diagnostic::ToolchainDiagnostic::new(
            "toolchain.test.module_path",
            DiagnosticSeverity::Information,
            "build",
            "build.validation",
            "Module path test",
            DiagnosticOrigin::new("toolchain.testing"),
        );

        let mut aggregator =
            aggregation::ToolchainDiagnosticAggregator::new();

        assert_eq!(
            aggregator.push(diagnostic).unwrap(),
            aggregation::InsertOutcome::Inserted
        );
    }
}
