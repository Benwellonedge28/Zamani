
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR diagnostics
//!
//! Path: `src/hdl/hdlir/diagnostics.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//! Status: HDLIR diagnostic foundation; freeze after integration tests pass.
//!
//! ## Purpose
//!
//! Provide stable, machine-readable diagnostics for HDLIR construction,
//! verification, schema compatibility, transformations, serialization,
//! lowering, and explicitly incomplete analysis.
//!
//! ## Authority
//!
//! This module follows:
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/spec/diagnostics.md`
//! - `grammar/spec/source-spans.md`
//! - `src/frontend/semantic/diagnostics/diagnostic.rs`
//! - `src/frontend/ast/node/source/span.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`, when established
//!
//! The canonical diagnostic implementation owns the diagnostic record,
//! common classifications, source labels, fix-its, metadata, and reporting
//! order. The canonical frontend owns `Span` and source identity.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - stable HDLIR-specific diagnostic code constants;
//! - HDLIR-specific diagnostic construction helpers;
//! - the mapping from HDLIR failures to canonical diagnostic phases,
//!   categories, severities, and recovery classifications.
//!
//! It does not own:
//!
//! - a second diagnostic record or source-span type;
//! - source text, source maps, parsing, or semantic validation;
//! - diagnostic accumulation, deduplication, sorting, rendering, or budgets;
//! - HDLIR error propagation policy or the final compilation status;
//! - hardware allocation, placement, routing, synthesis, or timing closure.
//!
//! ## Integration
//!
//! Consumers use [`HdlirDiagnostic`] as the canonical diagnostic record.
//! Builders, verifiers, transformations, schema management, serialization,
//! and lowering may use the helpers below. The owning operation or analysis
//! session remains responsible for collecting diagnostics and reporting
//! whether its work completed.
//!
//! HDLIR is downstream of the frontend semantic model and ZUIR. This module
//! may depend on canonical frontend diagnostics and spans, but neither the
//! frontend nor ZUIR should depend on HDLIR diagnostics.
//!
//! ## Scalability and determinism
//!
//! This module imposes no diagnostic-count or hardware-capacity ceiling.
//! It does not accumulate diagnostics and therefore does not set a collection
//! budget. Callers must enforce their configured operational budgets and
//! report incomplete work explicitly when those budgets are exhausted.
//!
//! Codes are independent of human-readable messages. Metadata uses the
//! canonical ordered representation. This module introduces no global state,
//! pointer-derived identity, or iteration-dependent diagnostic ordering.
//!
//! No finite process can handle literally infinite input. Actual work is
//! limited by available resources, representable values, and explicit
//! operational budgets; such limits must not become language-level hardware
//! restrictions.

use crate::frontend::ast::node::source::span::Span;
use crate::frontend::semantic::diagnostics::{
    Diagnostic, DiagnosticCategory, DiagnosticCode, DiagnosticPhase,
    DiagnosticSeverity, RecoveryClassification,
};

/// Canonical diagnostic record used by HDLIR.
///
/// This alias intentionally prevents HDLIR from introducing a competing
/// diagnostic representation. The record supports structured locations,
/// labels, notes, help, fix-its, recovery information, and ordered metadata.
pub type HdlirDiagnostic = Diagnostic;

/// Stable machine-readable identifiers for HDLIR diagnostic conditions.
///
/// Codes are open-world strings, not a closed enum. Extensions may introduce
/// additional namespaced codes without changing this module's public types.
/// Once published, a code must not be silently reused for another meaning.
pub mod codes {
    /// An HDLIR object violates a required structural invariant.
    pub const INVALID_STRUCTURE: &str = "ZMN-HDLIR-INVALID-STRUCTURE";

    /// An HDLIR operation has invalid or inconsistent operands or results.
    pub const INVALID_OPERATION: &str = "ZMN-HDLIR-INVALID-OPERATION";

    /// An HDLIR value, operand, or result references an unknown definition.
    pub const UNRESOLVED_VALUE: &str = "ZMN-HDLIR-UNRESOLVED-VALUE";

    /// An HDLIR identity is duplicated or used in an invalid namespace.
    pub const INVALID_IDENTITY: &str = "ZMN-HDLIR-INVALID-IDENTITY";

    /// An HDLIR type or type relationship is invalid.
    pub const INVALID_TYPE: &str = "ZMN-HDLIR-INVALID-TYPE";

    /// A concrete width or size cannot be represented or evaluated safely.
    pub const WIDTH_EVALUATION_FAILED: &str =
        "ZMN-HDLIR-WIDTH-EVALUATION-FAILED";

    /// A symbolic expression cannot be evaluated with the available facts.
    pub const UNRESOLVED_SYMBOLIC_VALUE: &str =
        "ZMN-HDLIR-UNRESOLVED-SYMBOLIC-VALUE";

    /// A connection references an invalid endpoint or declaration.
    pub const INVALID_CONNECTIVITY: &str = "ZMN-HDLIR-INVALID-CONNECTIVITY";

    /// A connection violates the applicable type or width contract.
    pub const CONNECTIVITY_TYPE_MISMATCH: &str =
        "ZMN-HDLIR-CONNECTIVITY-TYPE-MISMATCH";

    /// The design contains an illegal driver relationship.
    pub const DRIVER_CONFLICT: &str = "ZMN-HDLIR-DRIVER-CONFLICT";

    /// A process or control-flow structure violates its dialect contract.
    pub const INVALID_PROCESS: &str = "ZMN-HDLIR-INVALID-PROCESS";

    /// A clock, reset, or clock-domain relationship is invalid.
    pub const INVALID_CLOCKING: &str = "ZMN-HDLIR-INVALID-CLOCKING";

    /// A memory declaration or memory-access contract is invalid.
    pub const INVALID_MEMORY: &str = "ZMN-HDLIR-INVALID-MEMORY";

    /// An operation refers to an unregistered dialect or operation schema.
    pub const UNKNOWN_OPERATION: &str = "ZMN-HDLIR-UNKNOWN-OPERATION";

    /// A required operation, type, or interface definition is unavailable.
    pub const MISSING_SCHEMA_DEFINITION: &str =
        "ZMN-HDLIR-MISSING-SCHEMA-DEFINITION";

    /// Schema definitions conflict or cannot be registered consistently.
    pub const SCHEMA_CONFLICT: &str = "ZMN-HDLIR-SCHEMA-CONFLICT";

    /// The artifact or IR schema version is incompatible with this consumer.
    pub const INCOMPATIBLE_SCHEMA_VERSION: &str =
        "ZMN-HDLIR-INCOMPATIBLE-SCHEMA-VERSION";

    /// An artifact is malformed or violates its encoding contract.
    pub const MALFORMED_ARTIFACT: &str = "ZMN-HDLIR-MALFORMED-ARTIFACT";

    /// An artifact cannot be decoded without violating its declared contract.
    pub const DECODING_FAILED: &str = "ZMN-HDLIR-DECODING-FAILED";

    /// An artifact could not be encoded under the selected format contract.
    pub const ENCODING_FAILED: &str = "ZMN-HDLIR-ENCODING-FAILED";

    /// An artifact migration cannot preserve the required semantics.
    pub const MIGRATION_FAILED: &str = "ZMN-HDLIR-MIGRATION-FAILED";

    /// A requested transformation violated its declared contract.
    pub const TRANSFORMATION_FAILED: &str =
        "ZMN-HDLIR-TRANSFORMATION-FAILED";

    /// A transformation's required semantic-preservation conditions were
    /// not established.
    pub const PRESERVATION_NOT_ESTABLISHED: &str =
        "ZMN-HDLIR-PRESERVATION-NOT-ESTABLISHED";

    /// A requested lowering has no supported semantics-preserving mapping.
    pub const LOWERING_UNSUPPORTED: &str = "ZMN-HDLIR-LOWERING-UNSUPPORTED";

    /// A lowering failed to preserve the required semantic contract.
    pub const LOWERING_SEMANTIC_LOSS: &str =
        "ZMN-HDLIR-LOWERING-SEMANTIC-LOSS";

    /// A requested hardware target lacks a required capability.
    pub const UNSUPPORTED_TARGET_CAPABILITY: &str =
        "ZMN-HDLIR-UNSUPPORTED-TARGET-CAPABILITY";

    /// A target cannot satisfy a design requirement.
    pub const TARGET_INFEASIBLE: &str = "ZMN-HDLIR-TARGET-INFEASIBLE";

    /// A configured operational budget was exhausted.
    pub const OPERATIONAL_BUDGET_EXHAUSTED: &str =
        "ZMN-HDLIR-OPERATIONAL-BUDGET-EXHAUSTED";

    /// Analysis ended without enough evidence to reach a sound conclusion.
    pub const ANALYSIS_INCOMPLETE: &str = "ZMN-HDLIR-ANALYSIS-INCOMPLETE";

    /// A required invariant or integration contract could not be checked.
    pub const VERIFICATION_INCOMPLETE: &str =
        "ZMN-HDLIR-VERIFICATION-INCOMPLETE";

    /// An internal HDLIR integration contract was violated.
    pub const INTERNAL_INTEGRATION_ERROR: &str =
        "ZMN-HDLIR-INTERNAL-INTEGRATION-ERROR";
}

/// Construct an HDLIR verification error using the canonical diagnostic type.
///
/// `span` is optional because some failures concern a whole design, an
/// artifact, a schema, or an analysis session rather than one source
/// construct.
///
/// This helper is for an established error condition. It does not establish
/// that verification completed, nor does it replace the caller's result
/// status.
///
/// # Examples
///
/// ```
/// use zamani_compiler::hdl::hdlir::diagnostics::{self, codes};
///
/// let diagnostic = diagnostics::error(
///     codes::INVALID_STRUCTURE,
///     "A required IR definition is missing.",
///     None,
/// );
///
/// assert!(diagnostic.is_error());
/// assert_eq!(diagnostic.code().as_str(), codes::INVALID_STRUCTURE);
/// ```
#[must_use]
pub fn error(
    code: impl Into<DiagnosticCode>,
    message: impl Into<String>,
    span: Option<Span>,
) -> HdlirDiagnostic {
    Diagnostic::new(
        code,
        DiagnosticPhase::IrVerification,
        DiagnosticCategory::Domain,
        DiagnosticSeverity::Error,
        message,
        span,
    )
    .with_recovery(RecoveryClassification::Undetermined)
}

/// Construct a warning about HDLIR without asserting that the design is
/// invalid or that a requested analysis completed.
///
/// Use only when the reported condition is genuinely non-fatal for the
/// current operation. Warnings must not hide failed required checks.
#[must_use]
pub fn warning(
    code: impl Into<DiagnosticCode>,
    message: impl Into<String>,
    span: Option<Span>,
) -> HdlirDiagnostic {
    Diagnostic::new(
        code,
        DiagnosticPhase::IrVerification,
        DiagnosticCategory::Domain,
        DiagnosticSeverity::Warning,
        message,
        span,
    )
    .with_recovery(RecoveryClassification::Continue)
}

/// Construct an informational HDLIR diagnostic.
///
/// Informational diagnostics are advisory. They do not certify correctness,
/// completeness, portability, or target feasibility.
#[must_use]
pub fn information(
    code: impl Into<DiagnosticCode>,
    message: impl Into<String>,
    span: Option<Span>,
) -> HdlirDiagnostic {
    Diagnostic::new(
        code,
        DiagnosticPhase::IrVerification,
        DiagnosticCategory::Domain,
        DiagnosticSeverity::Information,
        message,
        span,
    )
    .with_recovery(RecoveryClassification::Continue)
}

/// Report an unsupported or unknown operation.
///
/// This helper records the operation and optional dialect as structured
/// metadata. An unknown operation must not be silently treated as a no-op or
/// as an operation whose semantics are understood.
#[must_use]
pub fn unknown_operation(
    message: impl Into<String>,
    span: Option<Span>,
    dialect: Option<&str>,
    operation: &str,
) -> HdlirDiagnostic {
    let mut diagnostic = Diagnostic::new(
        codes::UNKNOWN_OPERATION,
        DiagnosticPhase::IrVerification,
        DiagnosticCategory::Compatibility,
        DiagnosticSeverity::Error,
        message,
        span,
    )
    .with_recovery(RecoveryClassification::Undetermined)
    .with_metadata("operation", operation);

    if let Some(dialect) = dialect {
        diagnostic = diagnostic.with_metadata("dialect", dialect);
    }

    diagnostic
}

/// Report an incompatible HDLIR schema version.
///
/// This is an artifact or schema compatibility problem, not automatically a
/// source-language semantic error. The caller must not reinterpret the
/// artifact using different semantics merely to continue.
#[must_use]
pub fn incompatible_schema(
    message: impl Into<String>,
    span: Option<Span>,
    artifact_version: &str,
    supported_version: &str,
) -> HdlirDiagnostic {
    Diagnostic::new(
        codes::INCOMPATIBLE_SCHEMA_VERSION,
        DiagnosticPhase::IrVerification,
        DiagnosticCategory::Compatibility,
        DiagnosticSeverity::Error,
        message,
        span,
    )
    .with_recovery(RecoveryClassification::Undetermined)
    .with_metadata("artifact_schema_version", artifact_version)
    .with_metadata("supported_schema_version", supported_version)
}

/// Report exhaustion of a configured operational budget.
///
/// `budget` names the operational policy that was exhausted, not a universal
/// maximum on hardware size. The caller must also mark its enclosing
/// operation incomplete or aborted. This diagnostic alone is not a complete
/// analysis result.
#[must_use]
pub fn budget_exhausted(
    message: impl Into<String>,
    span: Option<Span>,
    budget: &str,
) -> HdlirDiagnostic {
    Diagnostic::new(
        codes::OPERATIONAL_BUDGET_EXHAUSTED,
        DiagnosticPhase::IrVerification,
        DiagnosticCategory::OperationalLimit,
        DiagnosticSeverity::Error,
        message,
        span,
    )
    .with_recovery(RecoveryClassification::Undetermined)
    .with_metadata("budget", budget)
}

/// Report that a requested analysis did not reach a sound conclusion.
///
/// Use this for unknown or incomplete outcomes. Do not use it to assert that
/// a property is false, or to turn an incomplete verification into success.
#[must_use]
pub fn analysis_incomplete(
    message: impl Into<String>,
    span: Option<Span>,
    reason: Option<&str>,
) -> HdlirDiagnostic {
    let mut diagnostic = Diagnostic::new(
        codes::ANALYSIS_INCOMPLETE,
        DiagnosticPhase::IrVerification,
        DiagnosticCategory::OperationalLimit,
        DiagnosticSeverity::Warning,
        message,
        span,
    )
    .with_recovery(RecoveryClassification::Undetermined);

    if let Some(reason) = reason {
        diagnostic = diagnostic.with_metadata("reason", reason);
    }

    diagnostic
}

/// Report that required verification could not be completed.
///
/// This is distinct from a failed invariant: an uncompleted check provides
/// no evidence that the invariant holds. The enclosing verification result
/// must preserve this incomplete status.
#[must_use]
pub fn verification_incomplete(
    message: impl Into<String>,
    span: Option<Span>,
    check: Option<&str>,
) -> HdlirDiagnostic {
    let mut diagnostic = Diagnostic::new(
        codes::VERIFICATION_INCOMPLETE,
        DiagnosticPhase::IrVerification,
        DiagnosticCategory::OperationalLimit,
        DiagnosticSeverity::Error,
        message,
        span,
    )
    .with_recovery(RecoveryClassification::Undetermined);

    if let Some(check) = check {
        diagnostic = diagnostic.with_metadata("check", check);
    }

    diagnostic
}

/// Report a transformation failure.
///
/// The caller should include the pass identity and relevant context in
/// metadata where available. This diagnostic does not claim that the
/// original or transformed design is valid.
#[must_use]
pub fn transformation_failed(
    message: impl Into<String>,
    span: Option<Span>,
    pass: Option<&str>,
) -> HdlirDiagnostic {
    let mut diagnostic = Diagnostic::new(
        codes::TRANSFORMATION_FAILED,
        DiagnosticPhase::Extension("hdlir.transformation".to_owned()),
        DiagnosticCategory::Domain,
        DiagnosticSeverity::Error,
        message,
        span,
    )
    .with_recovery(RecoveryClassification::Undetermined);

    if let Some(pass) = pass {
        diagnostic = diagnostic.with_metadata("pass", pass);
    }

    diagnostic
}

/// Report a lowering failure where a semantics-preserving mapping is
/// unavailable or could not be completed.
///
/// Lowering must fail explicitly rather than silently dropping an operation,
/// weakening a requirement, or changing the design's meaning.
#[must_use]
pub fn lowering_failed(
    message: impl Into<String>,
    span: Option<Span>,
    source_representation: Option<&str>,
    target_representation: Option<&str>,
) -> HdlirDiagnostic {
    let mut diagnostic = Diagnostic::new(
        codes::LOWERING_UNSUPPORTED,
        DiagnosticPhase::Lowering,
        DiagnosticCategory::Compatibility,
        DiagnosticSeverity::Error,
        message,
        span,
    )
    .with_recovery(RecoveryClassification::Undetermined);

    if let Some(source) = source_representation {
        diagnostic = diagnostic.with_metadata("source_representation", source);
    }

    if let Some(target) = target_representation {
        diagnostic = diagnostic.with_metadata("target_representation", target);
    }

    diagnostic
}

/// Report that a requested target cannot satisfy a design requirement.
///
/// This is a target-planning diagnostic, not proof that the source HDL is
/// invalid. The caller must have evidence from the appropriate feasibility
/// analysis before reporting the target as infeasible.
#[must_use]
pub fn target_infeasible(
    message: impl Into<String>,
    span: Option<Span>,
    requirement: Option<&str>,
    target: Option<&str>,
) -> HdlirDiagnostic {
    let mut diagnostic = Diagnostic::new(
        codes::TARGET_INFEASIBLE,
        DiagnosticPhase::TargetPlanning,
        DiagnosticCategory::Portability,
        DiagnosticSeverity::Error,
        message,
        span,
    )
    .with_recovery(RecoveryClassification::Continue);

    if let Some(requirement) = requirement {
        diagnostic = diagnostic.with_metadata("requirement", requirement);
    }

    if let Some(target) = target {
        diagnostic = diagnostic.with_metadata("target", target);
    }

    diagnostic
}

/// Report a failure of an internal HDLIR integration contract.
///
/// Use this for implementation or subsystem integration failures rather
/// than misattributing them to the user's source program.
#[must_use]
pub fn internal_integration_error(
    message: impl Into<String>,
    span: Option<Span>,
) -> HdlirDiagnostic {
    Diagnostic::new(
        codes::INTERNAL_INTEGRATION_ERROR,
        DiagnosticPhase::Extension("hdlir.integration".to_owned()),
        DiagnosticCategory::Internal,
        DiagnosticSeverity::Error,
        message,
        span,
    )
    .with_recovery(RecoveryClassification::Undetermined)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_uses_the_canonical_diagnostic_contract() {
        let diagnostic = error(
            codes::INVALID_STRUCTURE,
            "Missing definition.",
            None,
        );

        assert_eq!(diagnostic.code().as_str(), codes::INVALID_STRUCTURE);
        assert_eq!(diagnostic.phase(), &DiagnosticPhase::IrVerification);
        assert_eq!(diagnostic.category(), &DiagnosticCategory::Domain);
        assert_eq!(diagnostic.severity(), DiagnosticSeverity::Error);
        assert_eq!(
            diagnostic.recovery(),
            RecoveryClassification::Undetermined
        );
        assert!(diagnostic.is_error());
        assert_eq!(diagnostic.primary_span(), None);
    }

    #[test]
    fn unknown_operation_keeps_machine_readable_context() {
        let diagnostic = unknown_operation(
            "Operation is not registered.",
            None,
            Some("vendor.example"),
            "vendor.example.custom_op",
        );

        assert_eq!(diagnostic.code().as_str(), codes::UNKNOWN_OPERATION);
        assert_eq!(
            diagnostic.metadata_value("dialect"),
            Some(&crate::frontend::semantic::diagnostics::DiagnosticValue::Text(
                "vendor.example".to_owned()
            ))
        );
        assert_eq!(
            diagnostic.metadata_value("operation"),
            Some(&crate::frontend::semantic::diagnostics::DiagnosticValue::Text(
                "vendor.example.custom_op".to_owned()
            ))
        );
    }

    #[test]
    fn budget_exhaustion_is_not_reported_as_success() {
        let diagnostic = budget_exhausted(
            "Verification work budget exhausted.",
            None,
            "verification_work",
        );

        assert!(diagnostic.is_error());
        assert_eq!(
            diagnostic.recovery(),
            RecoveryClassification::Undetermined
        );
        assert_eq!(
            diagnostic.metadata_value("budget"),
            Some(&crate::frontend::semantic::diagnostics::DiagnosticValue::Text(
                "verification_work".to_owned()
            ))
        );
    }

    #[test]
    fn incomplete_analysis_remains_distinct_from_a_failed_invariant() {
        let incomplete = analysis_incomplete(
            "The analysis could not conclude.",
            None,
            Some("work budget exhausted"),
        );
        let invalid = error(
            codes::INVALID_STRUCTURE,
            "A required invariant is violated.",
            None,
        );

        assert_eq!(incomplete.code().as_str(), codes::ANALYSIS_INCOMPLETE);
        assert_eq!(incomplete.severity(), DiagnosticSeverity::Warning);
        assert_eq!(incomplete.recovery(), RecoveryClassification::Undetermined);
        assert_ne!(incomplete.code(), invalid.code());
    }

    #[test]
    fn target_infeasibility_is_not_a_source_semantic_error() {
        let diagnostic = target_infeasible(
            "The selected target cannot meet the memory requirement.",
            None,
            Some("memory >= requested"),
            Some("target.example"),
        );

        assert_eq!(diagnostic.phase(), &DiagnosticPhase::TargetPlanning);
        assert_eq!(
            diagnostic.category(),
            &DiagnosticCategory::Portability
        );
        assert_eq!(diagnostic.code().as_str(), codes::TARGET_INFEASIBLE);
        assert_eq!(
            diagnostic.metadata_value("target"),
            Some(&crate::frontend::semantic::diagnostics::DiagnosticValue::Text(
                "target.example".to_owned()
            ))
        );
    }

    #[test]
    fn published_codes_are_non_empty_and_namespaced() {
        let codes = [
            codes::INVALID_STRUCTURE,
            codes::INVALID_OPERATION,
            codes::INVALID_TYPE,
            codes::UNKNOWN_OPERATION,
            codes::INCOMPATIBLE_SCHEMA_VERSION,
            codes::OPERATIONAL_BUDGET_EXHAUSTED,
            codes::ANALYSIS_INCOMPLETE,
            codes::TARGET_INFEASIBLE,
        ];

        for code in codes {
            assert!(!code.trim().is_empty());
            assert!(code.starts_with("ZMN-HDLIR-"));
        }
    }
}
