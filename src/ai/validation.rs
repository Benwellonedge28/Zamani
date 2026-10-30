
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI Domain Validation API
//!
//! File: src/ai/validation.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! This module exposes the public validation boundary of the Zamani AI
//! computational domain.
//!
//! It delegates AI-specific semantic validation to the canonical frontend
//! semantic implementation:
//!
//!     crate::frontend::semantic::domains::ai
//!
//! It does not introduce another semantic analyzer.
//!
//! ## AUTHORITY
//!
//! The following components retain their respective ownership:
//!
//! - grammar/ai/: AI source grammar.
//! - frontend/ast/node/: canonical AST.
//! - frontend/semantic/: canonical semantic analysis.
//! - frontend/semantic/domains/ai.rs: AI semantic validation.
//! - frontend/semantic/types/: canonical type system.
//! - frontend/semantic/effects/: effect semantics.
//! - frontend/semantic/capabilities/: capability semantics.
//! - frontend/semantic/resources/: resource semantics.
//! - frontend/semantic/diagnostics/: canonical diagnostics.
//! - zuir/dialects/ai/: AI computational IR operations.
//!
//! ## OWNS
//!
//! - Public AI validation API.
//! - Public access to AI semantic validation contracts.
//! - Delegation to the authoritative AI semantic validator.
//! - Validation API documentation and integration tests.
//!
//! ## DOES NOT OWN
//!
//! - Parsing or AST construction.
//! - Name resolution.
//! - Type inference.
//! - A replacement semantic type system.
//! - Tensor storage.
//! - Model execution.
//! - Training or inference execution.
//! - Hardware discovery.
//! - Resource allocation.
//! - Device placement.
//! - Scheduling or routing.
//! - ZUIR construction or lowering.
//! - Runtime execution.
//!
//! ## INPUT
//!
//! An AiProgram assembled by an adapter from canonical frontend AST nodes
//! and established shared semantic facts.
//!
//! ## OUTPUT
//!
//! An AiValidationReport containing AI-specific validation issues and
//! an explicit completeness status.
//!
//! An incomplete analysis must never be considered valid.
//!
//! ## POCO-REAF
//!
//! This module imposes no universal limits on:
//!
//! - Model size.
//! - Parameter count.
//! - Tensor rank.
//! - Tensor dimensions.
//! - Operation count.
//! - AI model count.
//! - Dataset size.
//! - Training workload.
//! - Agent count.
//! - Accelerator count.
//! - CPU or GPU count.
//! - Distributed worker count.
//! - Available memory.
//!
//! Operational budgets are controlled by the owning semantic session,
//! configuration and downstream execution infrastructure.
//!
//! Exhausting a budget must not produce a successful validation result.
//!
//! No finite implementation can process literally infinite input.
//! The objective is to preserve scalable semantics subject to available
//! resources and explicitly configured operational policies.
//!
//! ## DETERMINISM
//!
//! Validation behavior and diagnostic ordering are inherited from the
//! canonical AI semantic validator.
//!
//! The semantic context must provide deterministic answers for equivalent
//! inputs and configuration.
//!
//! ## INTEGRATION
//!
//! The frontend AI semantic module must be declared by:
//!
//!     frontend/semantic/domains/mod.rs
//!
//! This module is declared by:
//!
//!     src/ai/mod.rs
//!
//! The crate root already exposes:
//!
//!     pub mod ai;
//!
//! No additional crate-root declaration is required.
//!
//! ## FREEZE CRITERIA
//!
//! - The canonical frontend AI validator is registered.
//! - The public re-exports resolve.
//! - Validation delegates to the canonical implementation.
//! - Incomplete analysis cannot become successful validation.
//! - No parallel semantic representation is introduced.
//! - API and integration tests pass.
//! - Workspace formatting, compilation, tests and lint checks pass.

/// Canonical AI semantic validation contracts.
///
/// These types are defined and owned by the frontend semantic subsystem.
/// Re-exporting them preserves a single authoritative implementation.
pub use crate::frontend::semantic::domains::ai::{
    AiId,
    AiInputError,
    AiIssue,
    AiOperation,
    AiOperationKind,
    AiOrigin,
    AiProgram,
    AiSemanticContext,
    AiSeverity,
    AiValidationReport,
    AiValue,
    AiValueFlow,
    AiValueKind,
    CheckStatus,
    ContextCheck,
    DimensionExpr,
    QualifiedName,
    TensorShape,
};

/// Validate an AI-domain program using the canonical semantic validator.
///
/// This is the public entry point for AI-specific semantic validation.
///
/// The supplied semantic context must delegate shared language rules to
/// their authoritative implementations, including:
///
/// - Operation registration.
/// - Type checking.
/// - Symbolic dimension and constraint checking.
/// - Value-flow compatibility.
/// - Effect checking.
/// - Capability checking.
/// - Resource requirement checking.
/// - Contract and policy obligations.
///
/// This function does not establish physical hardware feasibility or
/// execute AI operations.
///
/// # Arguments
///
/// * `program` - AI-domain records adapted from the canonical AST.
/// * `context` - Shared semantic services implementing `AiSemanticContext`.
///
/// # Returns
///
/// An `AiValidationReport`.
///
/// `report.is_valid()` is true only when analysis completed and no
/// error-level issue was reported.
///
/// # Scalability
///
/// This function imposes no artificial model-size, tensor-rank,
/// operation-count or hardware-capacity ceilings.
///
/// Actual work is bounded by the supplied input and the operational
/// policies of the owning semantic infrastructure.
///
/// # Examples
///
/// ```ignore
/// use zamani_compiler::ai::validation::{
///     validate_program,
///     AiProgram,
///     AiSemanticContext,
/// };
///
/// let program = AiProgram::default();
///
/// // `context` is supplied by the canonical semantic integration.
/// let report = validate_program(&program, &context);
///
/// if !report.is_valid() {
///     // Handle diagnostics or incomplete analysis.
/// }
/// ```
#[must_use = "validation results must be inspected"]
pub fn validate_program<C>(
    program: &AiProgram,
    context: &C,
) -> AiValidationReport
where
    C: AiSemanticContext,
{
    program.validate(context)
}

/// Report whether a completed AI validation result is valid.
///
/// This helper deliberately delegates to the canonical report instead
/// of duplicating validation-state interpretation.
///
/// An incomplete report is never considered valid.
#[must_use]
pub fn is_valid(report: &AiValidationReport) -> bool {
    report.is_valid()
}

/// Report whether the AI validation result contains errors.
///
/// This does not imply that analysis completed. Callers must inspect
/// completeness separately.
#[must_use]
pub fn has_errors(report: &AiValidationReport) -> bool {
    report.has_errors()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A semantic context for testing public API integration.
    ///
    /// It accepts all semantic checks. Production integrations must
    /// supply the actual shared semantic services.
    struct AcceptAllContext;

    impl AiSemanticContext for AcceptAllContext {
        fn validate_operation(
            &self,
            _operation: &AiOperation,
        ) -> ContextCheck {
            ContextCheck::valid()
        }

        fn validate_value_type(
            &self,
            _value: &AiValue,
        ) -> ContextCheck {
            ContextCheck::valid()
        }

        fn validate_dimension(
            &self,
            _dimension: &DimensionExpr,
        ) -> ContextCheck {
            ContextCheck::valid()
        }

        fn validate_flow(
            &self,
            _flow: &AiValueFlow,
            _value: &AiValue,
            _producer: &AiOperation,
            _consumer: &AiOperation,
        ) -> ContextCheck {
            ContextCheck::valid()
        }

        fn validate_obligations(
            &self,
            _operation: &AiOperation,
        ) -> ContextCheck {
            ContextCheck::valid()
        }
    }

    #[test]
    fn public_api_validates_empty_program() {
        let program = AiProgram::default();
        let context = AcceptAllContext;

        let report = validate_program(&program, &context);

        assert!(report.is_valid());
        assert!(!report.has_errors());
        assert!(report.complete);
    }

    #[test]
    fn public_validity_helper_delegates_to_report() {
        let program = AiProgram::default();
        let context = AcceptAllContext;

        let report = validate_program(&program, &context);

        assert_eq!(is_valid(&report), report.is_valid());
    }

    #[test]
    fn public_error_helper_delegates_to_report() {
        let program = AiProgram::default();
        let context = AcceptAllContext;

        let report = validate_program(&program, &context);

        assert_eq!(has_errors(&report), report.has_errors());
    }

    #[test]
    fn public_api_preserves_symbolic_dimensions() {
        let shape = TensorShape::new(vec![
            DimensionExpr::Constant(128),
            DimensionExpr::Symbol(
                AiId::new("batch")
                    .expect("valid symbolic dimension identity"),
            ),
        ]);

        assert_eq!(shape.rank(), 2);
        assert!(!shape.is_scalar());
    }

    #[test]
    fn incomplete_reports_are_not_valid() {
        let report = AiValidationReport {
            issues: Vec::new(),
            complete: false,
        };

        assert!(!is_valid(&report));
    }

    #[test]
    fn validation_api_does_not_require_hardware_information() {
        let program = AiProgram::default();
        let context = AcceptAllContext;

        let report = validate_program(&program, &context);

        assert!(report.is_valid());
    }
}
