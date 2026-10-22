
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Hybrid Semantic Integration
//!
//! Path: `src/hybrid/integration/semantic.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Connect the hybrid subsystem to the canonical frontend semantic validator.
//!
//! The authoritative hybrid semantic rules live in:
//!
//! `crate::frontend::semantic::domains::hybrid`
//!
//! The canonical semantic API contracts live in:
//!
//! `crate::frontend::semantic::api`
//!
//! This adapter delegates validation to the existing validator and translates
//! its structured issues into diagnostics supplied by the canonical semantic
//! diagnostic implementation.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The integration boundary between hybrid validation and semantic analysis.
//! - Mapping hybrid issues into caller-provided canonical diagnostics.
//! - Reporting hybrid validation completion and validity without conflating
//!   either property with target feasibility.
//!
//! This module does not own:
//!
//! - Parsing, AST construction, or AST-to-hybrid-model conversion.
//! - Name resolution, general type checking, or type inference.
//! - Hybrid semantic rules or a second hybrid validator.
//! - The canonical semantic model or diagnostic implementation.
//! - Classical IR, `quantum::ir`, HDL IR, ZUIR, or ZCIR.
//! - Target discovery, resource allocation, scheduling, routing, lowering,
//!   backend selection, or runtime execution.
//!
//! ## Integration contract
//!
//! 1. The frontend adapter constructs a `HybridProgram` from established
//!    semantic facts; syntax classifications alone are insufficient.
//! 2. The semantic pipeline supplies a `HybridSemanticContext` backed by the
//!    existing semantic services.
//! 3. This module calls `validate_hybrid` exactly once per invocation.
//! 4. Each resulting `HybridIssue` is passed to the canonical diagnostic
//!    adapter without losing its rule, category, severity, entity, or source
//!    node information.
//! 5. Incomplete validation never becomes successful validation.
//! 6. Semantic validity does not imply that a physical target can execute
//!    the program.
//!
//! ## Scalability
//!
//! This module introduces no fixed limit on domains, nodes, values, edges,
//! or diagnostics. Collections grow with the results produced by the
//! authoritative validator and the available process resources.
//!
//! It does not promise literal infinite processing. Operational work and
//! memory are necessarily bounded by available resources and the policies
//! enforced by the caller and semantic session.
//!
//! ## Authority
//!
//! The integration follows the repository's hybrid grammar and semantic
//! contracts, including `grammar/hybrid/hybrid.g4`, `grammar/hybrid/README.md`,
//! `grammar/spec/hybrid.md`, and `grammar/specification/semantic-model.md`.
//! Those authorities define meaning; this adapter does not redefine it.

use crate::frontend::semantic::api::{SemanticDiagnostic, SemanticStatus};
use crate::frontend::semantic::domains::hybrid::{
    validate_hybrid, HybridIssue, HybridProgram, HybridSemanticContext,
    HybridValidation,
};

/// Adapter that converts an authoritative hybrid issue into the canonical
/// semantic diagnostic type.
///
/// Implementations belong at the integration point for the repository's
/// existing diagnostic subsystem. They must not infer meaning by parsing
/// `HybridIssue::message()`.
///
/// The mapping must preserve the issue's rule identifier, category, severity,
/// associated hybrid entity, and optional canonical AST source node. If the
/// canonical diagnostic representation cannot directly store one of those
/// fields, the implementation must preserve it through its established
/// structured metadata mechanism rather than discarding it.
///
/// An incomplete hybrid issue must not be converted into a diagnostic that
/// implies that the corresponding semantic check succeeded.
pub trait HybridSemanticDiagnosticAdapter {
    /// The canonical diagnostic type used by frontend semantic analysis.
    type Diagnostic: SemanticDiagnostic;

    /// Convert one authoritative hybrid issue into a canonical diagnostic.
    fn adapt_issue(&self, issue: &HybridIssue) -> Self::Diagnostic;
}

/// Outcome of integrating one hybrid semantic validation pass.
///
/// This is an integration report, not a replacement semantic model. The
/// contained [`HybridValidation`] remains the authoritative result of hybrid
/// validation, while `diagnostics` contains its mapped canonical diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HybridSemanticIntegrationReport<D> {
    status: SemanticStatus,
    validation: HybridValidation,
    diagnostics: Vec<D>,
}

impl<D> HybridSemanticIntegrationReport<D> {
    /// Return the overall semantic-analysis status for the hybrid pass.
    ///
    /// `Incomplete` means this pass did not establish all required facts.
    /// `Complete` means the hybrid validator completed; it does not imply
    /// semantic validity.
    #[must_use]
    pub const fn status(&self) -> SemanticStatus {
        self.status
    }

    /// Return the authoritative hybrid validation report.
    #[must_use]
    pub const fn validation(&self) -> &HybridValidation {
        &self.validation
    }

    /// Return the canonical diagnostics mapped from hybrid issues.
    ///
    /// Diagnostic ordering follows the authoritative validator's issue order.
    #[must_use]
    pub fn diagnostics(&self) -> &[D] {
        &self.diagnostics
    }

    /// Consume this report and return its mapped diagnostics.
    #[must_use]
    pub fn into_diagnostics(self) -> Vec<D> {
        self.diagnostics
    }

    /// Whether hybrid validation completed and found no semantic errors.
    ///
    /// This method deliberately does not test target feasibility. That
    /// assessment belongs to the downstream planning and target-realization
    /// layers.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.status == SemanticStatus::Complete
            && self.validation.is_complete()
            && self.validation.is_valid()
    }

    /// Whether all required checks in this hybrid validation pass completed.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.status.is_complete() && self.validation.is_complete()
    }
}

/// Validate a hybrid semantic input and map its issues into canonical
/// semantic diagnostics.
///
/// This function is the integration entry point for the semantic pipeline.
/// It delegates all hybrid semantic decisions to
/// [`validate_hybrid`]; it does not duplicate the validator's rules.
///
/// # Arguments
///
/// - `program`: A hybrid input constructed from established frontend semantic
///   facts. It must not be fabricated from syntax classifications alone.
/// - `context`: Shared semantic services for operation validation, type
///   compatibility, domain transitions, and synchronization.
/// - `diagnostic_adapter`: Adapter to the repository's canonical diagnostic
///   representation.
///
/// # Result
///
/// Returns a report containing the authoritative hybrid validation result,
/// its overall completion status, and mapped canonical diagnostics.
///
/// An incomplete result remains incomplete even if the validator emitted no
/// error-severity issues. A complete but invalid result remains invalid.
/// Neither outcome is a target-feasibility assessment.
pub fn validate_hybrid_for_semantic<C, A>(
    program: &HybridProgram,
    context: &C,
    diagnostic_adapter: &A,
) -> HybridSemanticIntegrationReport<A::Diagnostic>
where
    C: HybridSemanticContext + ?Sized,
    A: HybridSemanticDiagnosticAdapter + ?Sized,
{
    let validation = validate_hybrid(program, context);

    let status = if validation.is_complete() {
        SemanticStatus::Complete
    } else {
        SemanticStatus::Incomplete
    };

    // Preserve the authoritative issue order. Do not sort by rendered
    // messages or use diagnostic strings as semantic identities.
    let diagnostics = validation
        .issues()
        .iter()
        .map(|issue| diagnostic_adapter.adapt_issue(issue))
        .collect();

    HybridSemanticIntegrationReport {
        status,
        validation,
        diagnostics,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::semantic::domains::hybrid::{
        HybridContextFailure, HybridDomainId, HybridEntityId,
        HybridNode, HybridNodeKind, HybridOperationId,
    };
    use std::collections::BTreeSet;

    /// Minimal test diagnostic; production integration uses the canonical
    /// diagnostic implementation owned by the frontend semantic subsystem.
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestDiagnostic {
        code: String,
        error: bool,
    }

    impl SemanticDiagnostic for TestDiagnostic {
        fn is_error(&self) -> bool {
            self.error
        }
    }

    struct TestDiagnosticAdapter;

    impl HybridSemanticDiagnosticAdapter for TestDiagnosticAdapter {
        type Diagnostic = TestDiagnostic;

        fn adapt_issue(&self, issue: &HybridIssue) -> Self::Diagnostic {
            TestDiagnostic {
                code: issue.rule().to_owned(),
                error: matches!(
                    issue.severity(),
                    crate::frontend::semantic::domains::hybrid::HybridIssueSeverity::Error
                ),
            }
        }
    }

    #[derive(Default)]
    struct TestContext {
        invalid_operations: BTreeSet<String>,
        incomplete_operations: BTreeSet<String>,
    }

    impl HybridSemanticContext for TestContext {
        fn validate_operation(
            &self,
            node: &HybridNode,
        ) -> Result<(), HybridContextFailure> {
            let operation = node.operation().qualified_name();

            if self.incomplete_operations.contains(&operation) {
                return Err(
                    HybridContextFailure::incomplete(
                        "TEST_INCOMPLETE",
                        "operation validation is incomplete",
                    )
                    .expect("test diagnostic input is valid"),
                );
            }

            if self.invalid_operations.contains(&operation) {
                return Err(
                    HybridContextFailure::invalid(
                        "TEST_INVALID_OPERATION",
                        "operation is invalid",
                    )
                    .expect("test diagnostic input is valid"),
                );
            }

            Ok(())
        }

        fn check_type_compatibility(
            &self,
            _value_type: &str,
            _expected_type: &str,
            _source_node: Option<
                &crate::frontend::ast::node::node_id::NodeId,
            >,
            _consumer_node: Option<
                &crate::frontend::ast::node::node_id::NodeId,
            >,
        ) -> Result<(), HybridContextFailure> {
            Ok(())
        }

        fn validate_domain_transition(
            &self,
            _producer: &HybridNode,
            _consumer: &HybridNode,
            _value: &crate::frontend::semantic::domains::hybrid::HybridValue,
        ) -> Result<(), HybridContextFailure> {
            Ok(())
        }

        fn validate_synchronization(
            &self,
            _group: &HybridEntityId,
            _members: &[&HybridNode],
        ) -> Result<(), HybridContextFailure> {
            Ok(())
        }
    }

    fn empty_program() -> HybridProgram {
        HybridProgram::new()
    }

    fn one_operation_program() -> HybridProgram {
        let domain = HybridDomainId::new("zamani", "classical")
            .expect("domain identity is valid");
        let operation = HybridOperationId::new(domain, "compute")
            .expect("operation identity is valid");
        let node_id = HybridEntityId::new("node-1")
            .expect("node identity is valid");

        let mut program = HybridProgram::new();
        program.add_node(HybridNode::new(
            node_id,
            None,
            operation,
            HybridNodeKind::Compute,
            Vec::new(),
            Vec::new(),
            None,
        ));
        program
    }

    #[test]
    fn complete_valid_hybrid_input_stays_valid() {
        let report = validate_hybrid_for_semantic(
            &empty_program(),
            &TestContext::default(),
            &TestDiagnosticAdapter,
        );

        assert_eq!(report.status(), SemanticStatus::Complete);
        assert!(report.is_complete());
        assert!(report.is_valid());
        assert!(report.diagnostics().is_empty());
    }

    #[test]
    fn semantic_errors_are_mapped_and_not_certified_valid() {
        let program = one_operation_program();
        let context = TestContext {
            invalid_operations: BTreeSet::from([
                "zamani.classical::compute".to_owned(),
            ]),
            ..TestContext::default()
        };

        let report = validate_hybrid_for_semantic(
            &program,
            &context,
            &TestDiagnosticAdapter,
        );

        assert_eq!(report.status(), SemanticStatus::Complete);
        assert!(!report.is_valid());
        assert!(
            report.diagnostics().iter().any(SemanticDiagnostic::is_error)
        );
    }

    #[test]
    fn incomplete_semantic_work_is_not_certified_valid() {
        let program = one_operation_program();
        let context = TestContext {
            incomplete_operations: BTreeSet::from([
                "zamani.classical::compute".to_owned(),
            ]),
            ..TestContext::default()
        };

        let report = validate_hybrid_for_semantic(
            &program,
            &context,
            &TestDiagnosticAdapter,
        );

        assert_eq!(report.status(), SemanticStatus::Incomplete);
        assert!(!report.is_complete());
        assert!(!report.is_valid());
        assert!(report.diagnostics().iter().any(|diagnostic| {
            !diagnostic.is_error()
        }));
    }
}
