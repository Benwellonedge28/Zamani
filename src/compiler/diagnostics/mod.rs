
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Compiler Diagnostics
//!
//! Path: `src/compiler/diagnostics/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provide the canonical compiler-level diagnostic integration boundary.
//!
//! This module connects the existing frontend diagnostic authority to
//! compiler-level aggregation, human-readable rendering, and
//! machine-readable reporting.
//!
//! It supports diagnostics originating from:
//!
//! - source decoding;
//! - lexical analysis;
//! - parsing;
//! - AST validation;
//! - name and module resolution;
//! - type analysis;
//! - ownership and borrowing;
//! - effect analysis;
//! - capability and requirement analysis;
//! - resource analysis;
//! - classical computation;
//! - quantum computation;
//! - AI and tensor computation;
//! - HDL and hardware compilation;
//! - hybrid compilation;
//! - distributed compilation;
//! - IR verification;
//! - optimization;
//! - lowering;
//! - backend generation;
//! - linking;
//! - artifact validation;
//! - target realization.
//!
//! New computational domains must be able to use this diagnostic
//! infrastructure without requiring a new diagnostic architecture.
//!
//! ## Architectural authority
//!
//! Canonical diagnostic records:
//!
//! `crate::frontend::semantic::diagnostics`
//!
//! Canonical source coordinates:
//!
//! `crate::frontend::ast::node::source::span`
//!
//! Canonical diagnostic ordering:
//!
//! `crate::frontend::semantic::diagnostics::ordering`
//!
//! Compiler diagnostic aggregation:
//!
//! `aggregation`
//!
//! Human-readable presentation:
//!
//! `rendering`
//!
//! Machine-readable reporting:
//!
//! `reporting`
//!
//! Normative specifications:
//!
//! - `grammar/spec/diagnostics.md`
//! - `grammar/spec/source-spans.md`
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - compiler diagnostic module composition;
//! - compiler-facing diagnostic exports;
//! - integration boundaries between diagnostic subsystems;
//! - convenience construction of compiler diagnostic aggregators;
//! - compiler-level diagnostic API documentation.
//!
//! It does not own:
//!
//! - semantic diagnostic construction;
//! - diagnostic code registration;
//! - diagnostic severity definitions;
//! - source-file storage;
//! - source-map construction;
//! - source-span construction;
//! - semantic validation;
//! - diagnostic ordering algorithms;
//! - diagnostic deduplication algorithms;
//! - parser recovery;
//! - compiler pipeline execution;
//! - backend error semantics;
//! - runtime execution.
//!
//! ## Scalability
//!
//! No fixed diagnostic-count ceiling is introduced.
//!
//! No hardware-count ceiling is introduced.
//!
//! No computational-domain enumeration is required.
//!
//! No source-size ceiling is introduced.
//!
//! Aggregation policies may impose configurable operational
//! retention budgets.
//!
//! Rendering policies may impose configurable output budgets.
//!
//! These budgets must never be interpreted as restrictions on
//! valid Zamani programs.
//!
//! Incomplete diagnostic aggregation must remain explicitly
//! distinguishable from complete aggregation.
//!
//! ## Determinism
//!
//! This module introduces no independent diagnostic ordering.
//!
//! Canonical ordering and deduplication remain delegated to
//! the frontend diagnostic authority.
//!
//! Compiler aggregation preserves deterministic reporting.
//!
//! JSON reporting preserves the canonical aggregate order.
//!
//! No hash-map iteration order is exposed as diagnostic order.
//!
//! ## Security
//!
//! No unsafe code.
//!
//! No global mutable diagnostic state.
//!
//! No implicit filesystem access.
//!
//! No implicit network access.
//!
//! No execution of diagnostic suggestions or fix-its.
//!
//! No interpretation of diagnostic metadata as executable instructions.
//!
//! Diagnostic rendering must escape untrusted presentation text.
//!
//! ## Integration policy
//!
//! Compiler stages must convert their stage-specific failures
//! into the canonical frontend Diagnostic record through
//! explicit adapters.
//!
//! An adapter must preserve:
//!
//! - diagnostic code;
//! - severity;
//! - category;
//! - compilation phase;
//! - primary source span;
//! - secondary labels;
//! - notes;
//! - help information;
//! - fix-it information;
//! - recovery classification;
//! - structured metadata.
//!
//! Information that cannot be represented must be handled
//! explicitly rather than silently discarded.


// -----------------------------------------------------------------------------
// Diagnostic implementation modules
// -----------------------------------------------------------------------------

/// Compiler-level aggregation and retention policies.
///
/// Owns collection of canonical diagnostics emitted by multiple
/// compiler stages.
pub mod aggregation;

/// Human-readable diagnostic presentation.
///
/// Does not own diagnostic construction, ordering, or source storage.
pub mod rendering;

/// Machine-readable diagnostic reporting.
///
/// Provides the versioned diagnostic report format.
pub mod reporting;


// -----------------------------------------------------------------------------
// Canonical frontend diagnostic authority
// -----------------------------------------------------------------------------

/// Canonical frontend diagnostic definitions.
///
/// This is a re-export of the existing frontend implementation.
/// It is not a second diagnostic implementation.
pub use crate::frontend::semantic::diagnostics;


// -----------------------------------------------------------------------------
// Canonical diagnostic record exports
// -----------------------------------------------------------------------------

pub use crate::frontend::semantic::diagnostics::{
    Diagnostic,
    DiagnosticCategory,
    DiagnosticCode,
    DiagnosticLabel,
    DiagnosticPhase,
    DiagnosticSeverity,
    DiagnosticValue,
    FixIt,
    FixItEdit,
    RecoveryClassification,
};


// -----------------------------------------------------------------------------
// Canonical diagnostic registry exports
// -----------------------------------------------------------------------------

pub use crate::frontend::semantic::diagnostics::{
    BuiltinDiagnosticMetadata,
    CategoryError,
    DiagnosticCodeError,
    RegisteredDiagnosticCategory,
    RegisteredDiagnosticCode,
    RegisteredDiagnosticSeverity,
    BUILTIN_DIAGNOSTICS,
};


// -----------------------------------------------------------------------------
// Canonical source-label exports
// -----------------------------------------------------------------------------

pub use crate::frontend::semantic::diagnostics::{
    DiagnosticLabels,
    LabelInsertError,
    LabelRole,
    SourceDiagnosticLabel,
};


// -----------------------------------------------------------------------------
// Canonical diagnostic ordering exports
// -----------------------------------------------------------------------------

pub use crate::frontend::semantic::diagnostics::{
    compare_diagnostics,
    deduplicate_diagnostics,
    sort_diagnostics,
};


// -----------------------------------------------------------------------------
// Compiler aggregation exports
// -----------------------------------------------------------------------------

pub use aggregation::{
    AggregatedDiagnostics,
    AggregationError,
    AggregationPolicy,
    AggregationStatistics,
    AggregationStatus,
    AllocationCollection,
    DiagnosticAggregator,
    InsertOutcome,
};


// -----------------------------------------------------------------------------
// Human-readable rendering exports
// -----------------------------------------------------------------------------

pub use rendering::{
    DisplayWidth,
    RenderingError,
    RenderingOptions,
    ScalarDisplayWidth,
    SourceFileRef,
    SourceProvider,
};


// -----------------------------------------------------------------------------
// Machine-readable reporting exports
// -----------------------------------------------------------------------------

pub use reporting::{
    DiagnosticReport,
    ReportError,
    ReportStatus,
    ReportSummary,
    DIAGNOSTIC_REPORT_SCHEMA,
};


// -----------------------------------------------------------------------------
// Compiler diagnostic construction
// -----------------------------------------------------------------------------

/// Construct a compiler diagnostic aggregator using an explicit policy.
///
/// This function does not modify diagnostic meaning or severity.
///
/// The supplied policy controls retention and duplicate handling.
///
/// # Arguments
///
/// * `policy` - Compiler diagnostic aggregation policy.
///
/// # Returns
///
/// A new independent diagnostic aggregator.
///
/// # Example
///
/// ```ignore
/// use zamani::compiler::diagnostics::{
///     AggregationPolicy,
///     DiagnosticAggregator,
///     new_aggregator,
/// };
///
/// let aggregator = new_aggregator(
///     AggregationPolicy::unlimited()
/// );
/// ```
#[must_use]
pub fn new_aggregator(
    policy: AggregationPolicy,
) -> DiagnosticAggregator {
    DiagnosticAggregator::new(policy)
}


/// Construct an aggregator without an explicit diagnostic
/// retention ceiling.
///
/// This does not promise infinite physical memory.
///
/// Actual resource availability remains authoritative.
///
/// # Returns
///
/// A new aggregator using the default unlimited-retention policy.
#[must_use]
pub fn unlimited_aggregator() -> DiagnosticAggregator {
    new_aggregator(AggregationPolicy::unlimited())
}


// -----------------------------------------------------------------------------
// Module-level tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiler_diagnostics_use_canonical_frontend_records() {
        let code = DiagnosticCode::new(
            "compiler.integration.test"
        );

        assert_eq!(
            code.as_str(),
            "compiler.integration.test"
        );
    }

    #[test]
    fn unlimited_aggregator_has_no_explicit_retention_limit() {
        let aggregator = unlimited_aggregator();

        assert_eq!(
            aggregator.policy(),
            AggregationPolicy::unlimited()
        );

        assert_eq!(
            aggregator.status(),
            AggregationStatus::Complete
        );

        assert!(aggregator.is_empty());
    }

    #[test]
    fn explicit_aggregation_policy_is_preserved() {
        use std::num::NonZeroUsize;

        let limit = NonZeroUsize::new(128)
            .expect("128 is nonzero");

        let policy = AggregationPolicy::with_limit(limit);

        let aggregator = new_aggregator(policy);

        assert_eq!(aggregator.policy(), policy);
    }

    #[test]
    fn diagnostic_report_schema_is_stable() {
        assert_eq!(
            DIAGNOSTIC_REPORT_SCHEMA,
            "zamani.diagnostics.v1"
        );
    }

    #[test]
    fn report_status_distinguishes_complete_and_incomplete() {
        assert_eq!(
            ReportStatus::Complete.as_str(),
            "complete"
        );

        assert_eq!(
            ReportStatus::Incomplete.as_str(),
            "incomplete"
        );
    }

    #[test]
    fn frontend_ordering_is_reexported() {
        let _: fn(
            &Diagnostic,
            &Diagnostic,
        ) -> std::cmp::Ordering = compare_diagnostics;
    }
}
