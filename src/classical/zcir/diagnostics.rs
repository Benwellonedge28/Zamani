
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR Structured Diagnostics Adapter
//!
//! Path: `src/classical/zcir/diagnostics.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Integrate classical intermediate-representation diagnostics with the
//! canonical ZUIR diagnostic system.
//!
//! ZCIR does not define a competing diagnostic representation. It reuses
//! canonical diagnostic codes, severity, phases, source labels, suggestions,
//! deterministic collections, JSON conversion, and source-map validation.
//!
//! ## AUTHORITY
//!
//! This module conforms to:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/diagnostics.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/errors.rs`
//! - `src/zuir/diagnostics.rs`
//! - `src/zuir/source_map.rs`
//! - `src/classical/diagnostics.rs`
//! - `src/classical/zcir/error.rs`
//!
//! ## OWNS
//!
//! - The ZCIR-to-ZUIR diagnostic adapter.
//! - Default mapping from canonical error phases to diagnostic phases.
//! - Convenience functions for converting and collecting ZCIR errors.
//! - Tests for phase mapping and canonical diagnostic integration.
//!
//! ## DOES NOT OWN
//!
//! - Diagnostic codes, severity definitions, labels, or fix-it structures.
//! - Source IDs, source spans, source text, or source-map storage.
//! - A competing diagnostic collection or JSON schema.
//! - Semantic analysis, IR verification, or lowering algorithms.
//! - Error categories or stable error-code registration.
//! - Compiler recovery policy or target-selection policy.
//!
//! ## PHASE MAPPING
//!
//! The mapping in this adapter is a default for ZCIR-originated errors.
//! Callers may supply an explicit diagnostic phase when the precise phase
//! is known. A diagnostic phase is reporting provenance, not a claim that
//! the underlying source program is semantically invalid.
//!
//! ## SCALABILITY
//!
//! This adapter defines no maximum diagnostic count, label count, source
//! length, IR size, operation count, or hardware population.
//!
//! Collection retention limits, when used, are explicit operational
//! policies supplied to the canonical `DiagnosticBag`. They are not
//! language-level or ZCIR representation limits.
//!
//! No finite process can guarantee literal infinity. Memory, address space,
//! allocation behavior, and caller-configured budgets remain operational
//! constraints.
//!
//! ## DEPENDENCY DIRECTION
//!
//! ```text
//! src/zuir/errors.rs -----------+
//!                               |
//! src/zuir/source_map.rs -------+--> src/zuir/diagnostics.rs
//!                               |              |
//! src/classical/zcir/error.rs --+              v
//!                                  src/classical/zcir/diagnostics.rs
//!                                               |
//!                                               v
//!                                   ZCIR builders and verifiers
//! ```
//!
//! The canonical diagnostic and source-map layers must not depend on ZCIR.
//!
//! ## INTEGRATION CONTRACT
//!
//! - `zcir/mod.rs` declares `pub mod diagnostics;` only after this file exists.
//! - ZCIR consumers use the re-exported canonical diagnostic types.
//! - Error conversion uses the canonical `ZcirError` alias from `error.rs`.
//! - Source locations use `crate::zuir::source_map::SourceSpan`.
//! - Source validation uses the canonical `SourceMap`.
//! - Diagnostic collection uses the canonical `DiagnosticBag`.
//! - The legacy `crate::classical::diagnostics` module remains a distinct
//!   compatibility boundary until a deliberate migration is implemented.
//! - Frontend, semantic, and target diagnostics must preserve their originating
//!   phase instead of being relabeled as ZCIR verification failures.
//!
//! ## FREEZE CRITERIA
//!
//! - All exported types resolve to canonical ZUIR definitions.
//! - Phase mapping is deterministic and tested.
//! - Error codes and structured context survive conversion.
//! - Source spans are not reconstructed or reinterpreted here.
//! - Duplicate suppression and retention limits use canonical behavior.
//! - No unsafe code or arbitrary universal capacity ceiling is introduced.
//! - The complete repository module graph, build, tests, and lint checks pass.

use crate::classical::zcir::error::ZcirError;
use crate::zuir::errors::ErrorPhase;
use crate::zuir::source_map::SourceSpan;

// Re-export the canonical diagnostic API rather than defining a competing
// ZCIR diagnostic model.
pub use crate::zuir::diagnostics::{
    Diagnostic,
    DiagnosticBag,
    DiagnosticBuildError,
    DiagnosticCode,
    DiagnosticFix,
    DiagnosticLabel,
    DiagnosticPhase,
    DiagnosticSeverity,
    DiagnosticSuggestion,
    DiagnosticValidationError,
    InsertOutcome,
    SuggestionApplicability,
};

// Re-export the canonical source-map types used by ZCIR diagnostic callers.
pub use crate::zuir::source_map::{SourceMap, SourceMapError};

/// Result type for ZCIR diagnostic construction.
///
/// The underlying diagnostic validation and construction errors are owned
/// by the canonical ZUIR diagnostic implementation.
pub type ZcirDiagnosticResult<T> =
    Result<T, DiagnosticBuildError>;

/// Convert a ZCIR error to a canonical diagnostic.
///
/// The phase is derived from the error's canonical context when present.
/// If the error has no phase context, the default is IR verification because
/// this adapter is intended for ZCIR-originated errors.
///
/// Call [`diagnostic_from_error_at_phase`] when the caller knows the precise
/// phase and wants to avoid the default mapping.
#[must_use]
pub fn diagnostic_from_error(
    error: &ZcirError,
    primary_span: Option<SourceSpan>,
) -> Diagnostic {
    let phase = diagnostic_phase_for_error(error.context().phase());

    Diagnostic::from_zui_error(error, phase, primary_span)
}

/// Convert a ZCIR error to a canonical diagnostic with an explicit phase.
///
/// This is the preferred entry point when the caller knows which ZCIR
/// operation produced the error. The canonical ZUIR diagnostic constructor
/// preserves the error's stable code, message, primary entity and context
/// notes. The source span is passed through unchanged.
#[must_use]
pub fn diagnostic_from_error_at_phase(
    error: &ZcirError,
    phase: DiagnosticPhase,
    primary_span: Option<SourceSpan>,
) -> Diagnostic {
    Diagnostic::from_zui_error(error, phase, primary_span)
}

/// Convert a ZCIR error and attempt to retain the diagnostic in a canonical
/// diagnostic bag.
///
/// The returned outcome explicitly indicates whether the diagnostic was
/// inserted, was already present, or was rejected by the bag's configured
/// retention budget.
pub fn record_error(
    bag: &mut DiagnosticBag,
    error: &ZcirError,
    primary_span: Option<SourceSpan>,
) -> InsertOutcome {
    bag.push(diagnostic_from_error(error, primary_span))
}

/// Convert a ZCIR error, validate all source spans against a canonical
/// source map, and return the diagnostic.
///
/// A missing primary span is valid for failures that do not originate from
/// a particular source construct. Any provided primary, secondary, or fix-it
/// span is validated by the canonical diagnostic API.
pub fn validated_diagnostic_from_error(
    error: &ZcirError,
    primary_span: Option<SourceSpan>,
    source_map: &SourceMap,
) -> Result<Diagnostic, DiagnosticValidationError> {
    let diagnostic = diagnostic_from_error(error, primary_span);
    diagnostic.validate_against(source_map)?;
    Ok(diagnostic)
}

/// Map canonical ZUIR error phase context to the default diagnostic phase.
///
/// This mapping is deliberately centralized so ZCIR builders, verifiers,
/// analysis passes, and lowering adapters do not each invent their own
/// inconsistent phase mappings.
///
/// An explicit phase supplied by a caller takes precedence through
/// [`diagnostic_from_error_at_phase`].
#[must_use]
pub fn diagnostic_phase_for_error(
    phase: Option<ErrorPhase>,
) -> DiagnosticPhase {
    match phase {
        Some(ErrorPhase::Construction) => DiagnosticPhase::IrGeneration,
        Some(ErrorPhase::Verification) => DiagnosticPhase::IrVerification,
        Some(ErrorPhase::Analysis) => DiagnosticPhase::Semantic,
        Some(ErrorPhase::Transformation) => DiagnosticPhase::Optimization,
        Some(ErrorPhase::DialectRegistration) => DiagnosticPhase::Domain,
        Some(ErrorPhase::Compatibility) => DiagnosticPhase::Compatibility,
        Some(ErrorPhase::Migration) => DiagnosticPhase::Compatibility,
        Some(ErrorPhase::Serialization) => DiagnosticPhase::Tooling,
        Some(ErrorPhase::Deserialization) => DiagnosticPhase::Tooling,
        Some(ErrorPhase::Interchange) => DiagnosticPhase::Tooling,
        Some(ErrorPhase::Lowering) => DiagnosticPhase::Target,
        Some(ErrorPhase::Configuration) => DiagnosticPhase::Tooling,
        Some(ErrorPhase::Extension) => DiagnosticPhase::Domain,
        Some(ErrorPhase::RuntimeBoundary) => DiagnosticPhase::Runtime,

        // A missing phase, or a future canonical phase that this adapter
        // does not yet recognize, must have a deterministic fallback.
        _ => DiagnosticPhase::IrVerification,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::errors::ZuirErrorKind;

    #[test]
    fn conversion_preserves_canonical_error_code_and_message() {
        let error = ZcirError::new(
            ZuirErrorKind::VerificationFailed,
            "the classical representation failed verification",
        )
        .with_phase(ErrorPhase::Verification);

        let diagnostic = diagnostic_from_error(&error, None);

        assert_eq!(
            diagnostic.code().as_str(),
            error.code().as_str()
        );
        assert_eq!(
            diagnostic.message(),
            "the classical representation failed verification"
        );
        assert_eq!(
            diagnostic.phase().as_str(),
            "ir_verification"
        );
        assert_eq!(diagnostic.severity(), DiagnosticSeverity::Error);
    }

    #[test]
    fn explicit_phase_overrides_the_default_mapping() {
        let error = ZcirError::new(
            ZuirErrorKind::UnsupportedOperation,
            "the selected target cannot realize this operation",
        );

        let diagnostic = diagnostic_from_error_at_phase(
            &error,
            DiagnosticPhase::Target,
            None,
        );

        assert_eq!(diagnostic.phase().as_str(), "target");
    }

    #[test]
    fn canonical_error_phase_mapping_is_deterministic() {
        assert_eq!(
            diagnostic_phase_for_error(Some(ErrorPhase::Construction)).as_str(),
            "ir_generation"
        );
        assert_eq!(
            diagnostic_phase_for_error(Some(ErrorPhase::Verification)).as_str(),
            "ir_verification"
        );
        assert_eq!(
            diagnostic_phase_for_error(Some(ErrorPhase::Analysis)).as_str(),
            "semantic"
        );
        assert_eq!(
            diagnostic_phase_for_error(Some(ErrorPhase::Transformation)).as_str(),
            "optimization"
        );
        assert_eq!(
            diagnostic_phase_for_error(Some(ErrorPhase::Lowering)).as_str(),
            "target"
        );
        assert_eq!(
            diagnostic_phase_for_error(None).as_str(),
            "ir_verification"
        );
    }

    #[test]
    fn error_context_is_retained_as_structured_diagnostic_data() {
        let error = ZcirError::new(
            ZuirErrorKind::InvalidType,
            "operand and result types are incompatible",
        )
        .with_note("the operation's declared type contract was checked");

        let diagnostic = diagnostic_from_error(&error, None);

        assert!(diagnostic
            .notes()
            .iter()
            .any(|note| note == "the operation's declared type contract was checked"));
    }

    #[test]
    fn recording_uses_canonical_duplicate_detection() {
        let error = ZcirError::new(
            ZuirErrorKind::VerificationFailed,
            "verification failed",
        )
        .with_phase(ErrorPhase::Verification);

        let mut bag = DiagnosticBag::unbounded();

        assert_eq!(
            record_error(&mut bag, &error, None),
            InsertOutcome::Inserted
        );
        assert_eq!(
            record_error(&mut bag, &error, None),
            InsertOutcome::Duplicate
        );
        assert_eq!(bag.len(), 1);
    }

    #[test]
    fn configured_retention_budget_is_not_a_language_limit() {
        let error = ZcirError::new(
            ZuirErrorKind::VerificationFailed,
            "verification failed",
        );

        let mut bag = DiagnosticBag::with_limit(0);

        assert_eq!(
            record_error(&mut bag, &error, None),
            InsertOutcome::DroppedByBudget
        );
        assert!(bag.is_empty());
        assert_eq!(bag.dropped_count(), 1);
        assert!(bag.is_truncated());
    }
}
