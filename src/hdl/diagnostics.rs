
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDL Diagnostics
//!
//! Path: `src/hdl/diagnostics.rs`
//! Rust: 1.97+
//! Edition: 2021
//!
//! ## Purpose
//!
//! Provide the HDL-specific diagnostic vocabulary and construction helpers
//! while reusing the canonical Zamani semantic diagnostic representation.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/spec/diagnostics.md`
//! - `grammar/spec/source-spans.md`
//! - `grammar/hdl/hdl.g4` and its delegate grammars
//! - `src/frontend/semantic/diagnostics/diagnostic.rs`
//! - `src/frontend/ast/node/source/span.rs`
//!
//! The grammar and specifications define language meaning. The canonical
//! semantic diagnostic module owns the diagnostic record, source spans,
//! labels, fix-its, metadata, and common diagnostic classifications.
//!
//! ## Owns
//!
//! - Stable identifiers for HDL-specific diagnostic conditions.
//! - HDL-specific diagnostic construction helpers.
//! - The mapping from HDL reporting contexts to canonical diagnostic phases
//!   and categories.
//!
//! ## Does not own
//!
//! - A replacement diagnostic record or source-span representation.
//! - Source maps, AST construction, parsing, or semantic validation rules.
//! - Diagnostic collection budgets, ordering, deduplication, or rendering.
//! - Hardware allocation, synthesis, placement, routing, or target discovery.
//! - HDL intermediate representation implementation.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Integration
//!
//! HDL validators and analyses call these helpers and return the canonical
//! `Diagnostic` type. The frontend semantic pipeline may consume these
//! diagnostics directly or pass them through its existing integration
//! adapter. Source locations must use the canonical AST `Span`.
//!
//! ## Scalability and determinism
//!
//! This module imposes no fixed diagnostic-count, source-size, signal-count,
//! width, module-count, or hardware-capacity limits. It does not accumulate
//! diagnostics, so collection budgets remain the responsibility of the
//! owning compiler or analysis session.
//!
//! Stable codes are independent of human-readable messages. This module
//! introduces no global mutable state and uses no unsafe Rust.
//!
//! A finite compiler cannot process literally infinite input. Processing is
//! limited by available resources and any explicit operational budgets;
//! budget exhaustion must never be reported as successful complete analysis.

use crate::frontend::ast::node::source::span::Span;
use crate::frontend::semantic::diagnostics::{
    Diagnostic, DiagnosticCategory, DiagnosticCode, DiagnosticPhase,
    DiagnosticSeverity, RecoveryClassification,
};

/// Canonical diagnostic record used by the HDL subsystem.
///
/// This alias deliberately prevents HDL from introducing a second
/// incompatible diagnostic representation.
pub type HdlDiagnostic = Diagnostic;

/// Stable machine-readable identifiers for HDL diagnostics.
///
/// Codes are open-world strings rather than a closed enum. New diagnostic
/// conditions can be introduced without changing this module's type system.
/// Once published, a code must not be silently reused for a different
/// meaning.
///
/// These identifiers are HDL-domain identifiers; they do not replace the
/// repository's general semantic diagnostic registry.
pub mod codes {
    /// A hardware declaration or construct is semantically invalid.
    pub const INVALID_CONSTRUCT: &str = "ZMN-HDL-INVALID-CONSTRUCT";

    /// A referenced HDL declaration cannot be resolved.
    pub const UNRESOLVED_REFERENCE: &str = "ZMN-HDL-UNRESOLVED-REFERENCE";

    /// An HDL declaration conflicts with another declaration.
    pub const DUPLICATE_DECLARATION: &str = "ZMN-HDL-DUPLICATE-DECLARATION";

    /// A connection does not satisfy the port or interface contract.
    pub const INVALID_CONNECTION: &str = "ZMN-HDL-INVALID-CONNECTION";

    /// A connection or assignment has incompatible widths.
    pub const WIDTH_MISMATCH: &str = "ZMN-HDL-WIDTH-MISMATCH";

    /// An HDL type or value conversion is invalid.
    pub const INVALID_TYPE: &str = "ZMN-HDL-INVALID-TYPE";

    /// A value cannot be resolved from the available symbolic information.
    pub const UNRESOLVED_VALUE: &str = "ZMN-HDL-UNRESOLVED-VALUE";

    /// A symbolic expression contains an invalid dependency cycle.
    pub const SYMBOLIC_DEPENDENCY_CYCLE: &str =
        "ZMN-HDL-SYMBOLIC-DEPENDENCY-CYCLE";

    /// A module or instance hierarchy contains an invalid cycle.
    pub const HIERARCHY_CYCLE: &str = "ZMN-HDL-HIERARCHY-CYCLE";

    /// A signal has conflicting or illegal drivers.
    pub const DRIVER_CONFLICT: &str = "ZMN-HDL-DRIVER-CONFLICT";

    /// A combinational dependency cycle violates the applicable rules.
    pub const COMBINATIONAL_CYCLE: &str = "ZMN-HDL-COMBINATIONAL-CYCLE";

    /// A clock declaration or clock relationship is invalid.
    pub const INVALID_CLOCK: &str = "ZMN-HDL-INVALID-CLOCK";

    /// A reset declaration or reset relationship is invalid.
    pub const INVALID_RESET: &str = "ZMN-HDL-INVALID-RESET";

    /// A clock-domain crossing cannot be proven safe.
    pub const UNSAFE_CLOCK_DOMAIN_CROSSING: &str =
        "ZMN-HDL-UNSAFE-CLOCK-DOMAIN-CROSSING";

    /// A timing requirement or timing constraint is invalid.
    pub const INVALID_TIMING_CONSTRAINT: &str =
        "ZMN-HDL-INVALID-TIMING-CONSTRAINT";

    /// A memory declaration or memory behavior is invalid.
    pub const INVALID_MEMORY: &str = "ZMN-HDL-INVALID-MEMORY";

    /// A verification property or assertion is invalid.
    pub const INVALID_VERIFICATION_PROPERTY: &str =
        "ZMN-HDL-INVALID-VERIFICATION-PROPERTY";

    /// Verification could not establish the requested property.
    pub const VERIFICATION_INCONCLUSIVE: &str =
        "ZMN-HDL-VERIFICATION-INCONCLUSIVE";

    /// The requested target does not offer a required capability.
    pub const UNSUPPORTED_TARGET_CAPABILITY: &str =
        "ZMN-HDL-UNSUPPORTED-TARGET-CAPABILITY";

    /// The requested target cannot satisfy a design requirement.
    pub const TARGET_INFEASIBLE: &str = "ZMN-HDL-TARGET-INFEASIBLE";

    /// A target description is incomplete or invalid.
    pub const INVALID_TARGET_DESCRIPTION: &str =
        "ZMN-HDL-INVALID-TARGET-DESCRIPTION";

    /// A requested operation is unsupported by the selected provider.
    pub const UNSUPPORTED_REALIZATION: &str =
        "ZMN-HDL-UNSUPPORTED-REALIZATION";

    /// A configured operational budget was exhausted.
    pub const OPERATIONAL_BUDGET_EXHAUSTED: &str =
        "ZMN-HDL-OPERATIONAL-BUDGET-EXHAUSTED";

    /// An analysis ended without enough information to reach a conclusion.
    pub const ANALYSIS_INCOMPLETE: &str = "ZMN-HDL-ANALYSIS-INCOMPLETE";

    /// An HDL interoperability conversion cannot preserve required meaning.
    pub const INTEROPERABILITY_LOSS: &str =
        "ZMN-HDL-INTEROPERABILITY-LOSS";

    /// An external HDL construct is not supported by the selected importer.
    pub const UNSUPPORTED_IMPORTED_CONSTRUCT: &str =
        "ZMN-HDL-UNSUPPORTED-IMPORTED-CONSTRUCT";

    /// An internal integration contract was violated.
    pub const INTERNAL_INTEGRATION_ERROR: &str =
        "ZMN-HDL-INTERNAL-INTEGRATION-ERROR";
}

/// Construct an HDL semantic error using the canonical diagnostic model.
///
/// `span` is optional because some errors concern an entire design or an
/// analysis session rather than one source expression.
///
/// This helper reports a source/domain error. It must not be used to disguise
/// target infeasibility or operational-budget exhaustion as invalid source.
#[must_use]
pub fn error(
    code: impl Into<DiagnosticCode>,
    message: impl Into<String>,
    span: Option<Span>,
) -> HdlDiagnostic {
    Diagnostic::new(
        code,
        DiagnosticPhase::DomainAnalysis,
        DiagnosticCategory::Domain,
        DiagnosticSeverity::Error,
        message,
        span,
    )
    .with_recovery(RecoveryClassification::Undetermined)
}

/// Construct a recoverable HDL diagnostic.
///
/// Use this only when the producer has established that analysis can safely
/// continue after reporting the condition. This does not certify that the
/// affected construct is valid.
#[must_use]
pub fn recoverable_error(
    code: impl Into<DiagnosticCode>,
    message: impl Into<String>,
    span: Option<Span>,
) -> HdlDiagnostic {
    Diagnostic::new(
        code,
        DiagnosticPhase::DomainAnalysis,
        DiagnosticCategory::Domain,
        DiagnosticSeverity::Error,
        message,
        span,
    )
    .with_recovery(RecoveryClassification::Recoverable)
}

/// Construct an HDL warning.
///
/// A warning does not, by itself, establish portability, target feasibility,
/// verification success, or completeness of analysis.
#[must_use]
pub fn warning(
    code: impl Into<DiagnosticCode>,
    message: impl Into<String>,
    span: Option<Span>,
) -> HdlDiagnostic {
    Diagnostic::new(
        code,
        DiagnosticPhase::DomainAnalysis,
        DiagnosticCategory::Domain,
        DiagnosticSeverity::Warning,
        message,
        span,
    )
    .with_recovery(RecoveryClassification::Continue)
}

/// Construct an informational HDL diagnostic.
///
/// Information is advisory and must not be used as a substitute for a
/// structured compilation or analysis outcome.
#[must_use]
pub fn information(
    code: impl Into<DiagnosticCode>,
    message: impl Into<String>,
    span: Option<Span>,
) -> HdlDiagnostic {
    Diagnostic::new(
        code,
        DiagnosticPhase::DomainAnalysis,
        DiagnosticCategory::Domain,
        DiagnosticSeverity::Information,
        message,
        span,
    )
    .with_recovery(RecoveryClassification::Continue)
}

/// Report that a requested target lacks a required capability.
///
/// This is a target-planning diagnostic, not a source-syntax error. The
/// portable design can remain semantically valid even when a particular
/// realization is unavailable.
///
/// The `requirement` and `target` metadata are optional descriptive values;
/// they must not be treated as proof that target discovery or feasibility
/// analysis has completed.
#[must_use]
pub fn unsupported_target_capability(
    message: impl Into<String>,
    span: Option<Span>,
    requirement: Option<&str>,
    target: Option<&str>,
) -> HdlDiagnostic {
    let mut diagnostic = Diagnostic::new(
        codes::UNSUPPORTED_TARGET_CAPABILITY,
        DiagnosticPhase::TargetPlanning,
        DiagnosticCategory::Capability,
        DiagnosticSeverity::Error,
        message,
        span,
    )
    .with_recovery(RecoveryClassification::Continue);

    if let Some(requirement) = requirement {
        diagnostic = diagnostic.with_metadata("required_capability", requirement);
    }

    if let Some(target) = target {
        diagnostic = diagnostic.with_metadata("target", target);
    }

    diagnostic
}

/// Report that a particular target cannot satisfy a design requirement.
///
/// This report records realization infeasibility; it does not retroactively
/// invalidate the HDL source. The caller must establish the stated
/// infeasibility through the appropriate feasibility analysis.
#[must_use]
pub fn target_infeasible(
    message: impl Into<String>,
    span: Option<Span>,
    requirement: Option<&str>,
    target: Option<&str>,
) -> HdlDiagnostic {
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

/// Report exhaustion of a configured HDL analysis budget.
///
/// Budget exhaustion is an operational limitation, not a language-level
/// limit on the number of signals, ports, bits, clocks, modules, or instances.
/// The caller must mark the overall analysis incomplete or aborted rather
/// than treating this diagnostic as successful validation.
#[must_use]
pub fn operational_budget_exhausted(
    message: impl Into<String>,
    span: Option<Span>,
    budget: &str,
) -> HdlDiagnostic {
    Diagnostic::new(
        codes::OPERATIONAL_BUDGET_EXHAUSTED,
        DiagnosticPhase::DomainAnalysis,
        DiagnosticCategory::OperationalLimit,
        DiagnosticSeverity::Error,
        message,
        span,
    )
    .with_recovery(RecoveryClassification::Undetermined)
    .with_metadata("budget", budget)
}

/// Report that an HDL analysis could not reach a sound conclusion.
///
/// Use this when the result is unknown or incomplete, rather than emitting a
/// success result or asserting that a property is false without evidence.
#[must_use]
pub fn analysis_incomplete(
    message: impl Into<String>,
    span: Option<Span>,
    reason: Option<&str>,
) -> HdlDiagnostic {
    let mut diagnostic = Diagnostic::new(
        codes::ANALYSIS_INCOMPLETE,
        DiagnosticPhase::DomainAnalysis,
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

/// Report an internal violation of the HDL integration contract.
///
/// This diagnostic describes an implementation or integration failure, not
/// necessarily a defect in the user's HDL program. Infrastructure failures
/// should remain distinguishable from ordinary source errors.
#[must_use]
pub fn internal_integration_error(
    message: impl Into<String>,
    span: Option<Span>,
) -> HdlDiagnostic {
    Diagnostic::new(
        codes::INTERNAL_INTEGRATION_ERROR,
        DiagnosticPhase::DomainAnalysis,
        DiagnosticCategory::Internal,
        DiagnosticSeverity::Fatal,
        message,
        span,
    )
    .with_recovery(RecoveryClassification::Unrecoverable)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hdl_diagnostic_is_the_canonical_diagnostic_type() {
        let diagnostic: HdlDiagnostic = error(
            codes::INVALID_CONSTRUCT,
            "invalid HDL construct",
            None,
        );

        assert_eq!(diagnostic.code().as_str(), codes::INVALID_CONSTRUCT);
        assert_eq!(diagnostic.phase(), &DiagnosticPhase::DomainAnalysis);
        assert_eq!(diagnostic.category(), &DiagnosticCategory::Domain);
        assert_eq!(diagnostic.severity(), DiagnosticSeverity::Error);
        assert_eq!(
            diagnostic.recovery(),
            RecoveryClassification::Undetermined
        );
        assert!(diagnostic.primary_span().is_none());
    }

    #[test]
    fn stable_codes_are_machine_readable_and_nonempty() {
        let identifiers = [
            codes::INVALID_CONSTRUCT,
            codes::INVALID_CONNECTION,
            codes::WIDTH_MISMATCH,
            codes::INVALID_CLOCK,
            codes::INVALID_RESET,
            codes::TARGET_INFEASIBLE,
            codes::OPERATIONAL_BUDGET_EXHAUSTED,
        ];

        for identifier in identifiers {
            assert!(!identifier.is_empty());
            assert!(identifier.starts_with("ZMN-HDL-"));
        }
    }

    #[test]
    fn target_capability_failure_is_not_a_source_domain_error() {
        let diagnostic = unsupported_target_capability(
            "selected target lacks a required capability",
            None,
            Some("memory"),
            Some("example-target"),
        );

        assert_eq!(
            diagnostic.code().as_str(),
            codes::UNSUPPORTED_TARGET_CAPABILITY
        );
        assert_eq!(diagnostic.phase(), &DiagnosticPhase::TargetPlanning);
        assert_eq!(diagnostic.category(), &DiagnosticCategory::Capability);
        assert_eq!(
            diagnostic.metadata_value("required_capability"),
            Some(&"memory".into())
        );
        assert_eq!(
            diagnostic.metadata_value("target"),
            Some(&"example-target".into())
        );
    }

    #[test]
    fn target_infeasibility_preserves_requirement_metadata() {
        let diagnostic = target_infeasible(
            "the target cannot satisfy the required throughput",
            None,
            Some("throughput >= requested"),
            Some("example-target"),
        );

        assert_eq!(diagnostic.code().as_str(), codes::TARGET_INFEASIBLE);
        assert_eq!(diagnostic.phase(), &DiagnosticPhase::TargetPlanning);
        assert_eq!(
            diagnostic.category(),
            &DiagnosticCategory::Portability
        );
        assert_eq!(
            diagnostic.metadata_value("requirement"),
            Some(&"throughput >= requested".into())
        );
    }

    #[test]
    fn budget_exhaustion_is_not_reported_as_success() {
        let diagnostic = operational_budget_exhausted(
            "analysis work budget exhausted",
            None,
            "analysis_work",
        );

        assert!(diagnostic.is_error());
        assert_eq!(
            diagnostic.category(),
            &DiagnosticCategory::OperationalLimit
        );
        assert_eq!(
            diagnostic.recovery(),
            RecoveryClassification::Undetermined
        );
        assert_eq!(
            diagnostic.metadata_value("budget"),
            Some(&"analysis_work".into())
        );
    }

    #[test]
    fn incomplete_analysis_is_explicitly_inconclusive() {
        let diagnostic = analysis_incomplete(
            "the analysis could not establish the property",
            None,
            Some("unresolved symbolic dependency"),
        );

        assert_eq!(
            diagnostic.code().as_str(),
            codes::ANALYSIS_INCOMPLETE
        );
        assert_eq!(
            diagnostic.recovery(),
            RecoveryClassification::Undetermined
        );
        assert_eq!(
            diagnostic.metadata_value("reason"),
            Some(&"unresolved symbolic dependency".into())
        );
    }

    #[test]
    fn warning_and_information_are_nonfatal() {
        let warning = warning(
            codes::VERIFICATION_INCONCLUSIVE,
            "verification is incomplete",
            None,
        );
        let information = information(
            codes::INVALID_CONSTRUCT,
            "additional analysis is available",
            None,
        );

        assert_eq!(warning.severity(), DiagnosticSeverity::Warning);
        assert_eq!(
            warning.recovery(),
            RecoveryClassification::Continue
        );
        assert_eq!(
            information.severity(),
            DiagnosticSeverity::Information
        );
    }

    #[test]
    fn internal_integration_failure_is_distinct() {
        let diagnostic = internal_integration_error(
            "HDL adapter contract was violated",
            None,
        );

        assert_eq!(
            diagnostic.code().as_str(),
            codes::INTERNAL_INTEGRATION_ERROR
        );
        assert_eq!(diagnostic.category(), &DiagnosticCategory::Internal);
        assert_eq!(diagnostic.severity(), DiagnosticSeverity::Fatal);
        assert_eq!(
            diagnostic.recovery(),
            RecoveryClassification::Unrecoverable
        );
    }

    #[test]
    fn repeated_construction_is_deterministic() {
        let first = error(
            codes::WIDTH_MISMATCH,
            "connection width mismatch",
            None,
        );
        let second = error(
            codes::WIDTH_MISMATCH,
            "connection width mismatch",
            None,
        );

        assert_eq!(first, second);
        assert_eq!(
            first.compare_for_reporting(&second),
            std::cmp::Ordering::Equal
        );
    }
}
