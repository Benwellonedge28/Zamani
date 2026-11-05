
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Universal Compiler Pass Manager
//!
//! Path: src/compiler/passes/manager.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! STATUS
//!
//! Compiler-facing pass-management integration.
//!
//! PURPOSE
//!
//! Provide a stable compiler-level interface for configuring and executing
//! compiler transformation and analysis passes.
//!
//! ARCHITECTURAL AUTHORITY
//!
//! The canonical pass execution implementation belongs to:
//!
//!     crate::zuir::pass::manager
//!
//! The canonical pass registration implementation belongs to:
//!
//!     crate::zuir::pass::registry
//!
//! The canonical pass contracts belong to:
//!
//!     crate::zuir::pass::contracts
//!
//! This module is an integration facade.
//! It MUST NOT implement a competing pass manager.
//!
//! ---------------------------------------------------------------------------
//! OWNERSHIP
//! ---------------------------------------------------------------------------
//!
//! OWNS:
//!
//! - Compiler-facing pass-manager namespace.
//! - Compiler-facing operational type exports.
//! - Compatibility aliases.
//! - Documentation of compiler/pass-manager integration.
//!
//! DOES NOT OWN:
//!
//! - Pass execution algorithms.
//! - Pass registration.
//! - Pass identity.
//! - Pass descriptors.
//! - Pass versioning.
//! - Analysis computation.
//! - Analysis invalidation algorithms.
//! - IR verification algorithms.
//! - Semantic preservation proofs.
//! - Optimization algorithms.
//! - Hardware discovery.
//! - Device allocation.
//! - Physical scheduling.
//! - Runtime execution.
//!
//! ---------------------------------------------------------------------------
//! SCALABILITY
//! ---------------------------------------------------------------------------
//!
//! No universal pass-count limit is introduced.
//!
//! No fixed computational-domain enumeration is introduced.
//!
//! No hardware population limit is introduced.
//!
//! Resource consumption is governed by:
//!
//! - Available memory.
//! - Available processing capacity.
//! - Configured execution budgets.
//! - Actual implementation representation limits.
//!
//! The underlying manager owns execution-budget enforcement.
//!
//! ---------------------------------------------------------------------------
//! DETERMINISM
//! ---------------------------------------------------------------------------
//!
//! Pass execution order is determined by the underlying manager's
//! explicitly registered order.
//!
//! This facade does not reorder passes.
//!
//! It does not infer execution dependencies.
//!
//! It does not silently skip passes.
//!
//! ---------------------------------------------------------------------------
//! SAFETY
//! ---------------------------------------------------------------------------
//!
//! Unsafe Rust is forbidden.
//!
//! No global mutable registry is introduced.
//!
//! No hidden hardware assumptions are introduced.
//!
//! ---------------------------------------------------------------------------
//! INTEGRATION
//! ---------------------------------------------------------------------------
//!
//! Compiler pass registration:
//!
//!     crate::compiler::passes::registry
//!
//! Canonical pass registration:
//!
//!     crate::zuir::pass::registry
//!
//! Canonical pass contracts:
//!
//!     crate::zuir::pass::contracts
//!
//! Operational pass manager:
//!
//!     crate::zuir::pass::manager
//!
//! Compiler optimization:
//!
//!     crate::optimizer
//!
//! Quantum optimization:
//!
//!     crate::quantum::optimization
//!
//! Canonical universal IR:
//!
//!     crate::zuir
//!
//! Domain-specific IR implementations remain owned by their respective
//! repository subsystems.
//!
//! ---------------------------------------------------------------------------
//! IMPORTANT COMPATIBILITY RULE
//! ---------------------------------------------------------------------------
//!
//! The current ZUIR operational manager still defines legacy operational
//! contract types.
//!
//! Those types are explicitly named Operational* below.
//!
//! Canonical contract types are separately exposed under their canonical
//! names.
//!
//! This avoids falsely claiming that the outstanding migration between
//! manager.rs and contracts.rs has already been completed.
//!
//! A future canonical-manager migration must be performed in the ZUIR
//! authority itself, not by creating a competing compiler implementation.

/// Operational pass-management implementation.
///
/// This is the existing ZUIR implementation, not a second manager.
pub use crate::zuir::pass::manager::PassManager;

/// Operational pass trait.
///
/// This trait is currently defined by the existing ZUIR manager.
pub use crate::zuir::pass::manager::Pass as OperationalPass;

/// Operational pass execution context.
pub use crate::zuir::pass::manager::PassContext;

/// Operational IR verifier interface.
pub use crate::zuir::pass::manager::IrVerifier;

/// Verification policy.
pub use crate::zuir::pass::manager::VerificationPolicy;

/// Cancellation token.
pub use crate::zuir::pass::manager::CancellationToken;

/// Execution budget.
pub use crate::zuir::pass::manager::ExecutionBudget;

/// Analysis availability state.
pub use crate::zuir::pass::manager::AnalysisState;

/// Pass-manager configuration.
pub use crate::zuir::pass::manager::PassManagerConfig;

/// Pass execution record.
pub use crate::zuir::pass::manager::PassExecutionRecord;

/// Completed pipeline report.
pub use crate::zuir::pass::manager::PipelineReport;

// ---------------------------------------------------------------------------
// Legacy operational contract aliases
// ---------------------------------------------------------------------------
//
// These aliases preserve compatibility with the currently implemented
// ZUIR operational manager.
//
// They are not the canonical contract definitions.

/// Legacy operational pass identity.
pub use crate::zuir::pass::manager::PassId as OperationalPassId;

/// Legacy operational pass descriptor.
pub use crate::zuir::pass::manager::PassDescriptor as OperationalPassDescriptor;

/// Legacy operational pass version.
pub use crate::zuir::pass::manager::PassVersion as OperationalPassVersion;

/// Legacy operational pass error.
pub use crate::zuir::pass::manager::PassError as OperationalPassError;

/// Legacy operational pass error category.
pub use crate::zuir::pass::manager::PassErrorKind as OperationalPassErrorKind;

/// Legacy operational pass outcome.
pub use crate::zuir::pass::manager::PassOutcome as OperationalPassOutcome;

// ---------------------------------------------------------------------------
// Canonical pass contracts
// ---------------------------------------------------------------------------
//
// These are owned by contracts.rs.
// Do not replace them with compiler-specific implementations.

/// Canonical pass identity.
pub use crate::zuir::pass::contracts::PassId;

/// Canonical pass descriptor.
pub use crate::zuir::pass::contracts::PassDescriptor;

/// Canonical pass version.
pub use crate::zuir::pass::contracts::PassVersion;

/// Canonical structured pass error.
pub use crate::zuir::pass::contracts::PassError;

/// Canonical pass error category.
pub use crate::zuir::pass::contracts::PassErrorKind;

/// Canonical pass execution outcome.
pub use crate::zuir::pass::contracts::PassOutcome;

// ---------------------------------------------------------------------------
// Canonical registry
// ---------------------------------------------------------------------------

/// Canonical pass registry.
///
/// Registration and duplicate detection remain owned by ZUIR.
pub use crate::zuir::pass::registry::PassRegistry;

// ---------------------------------------------------------------------------
// Determinism
// ---------------------------------------------------------------------------

/// Canonical pipeline determinism assessment.
pub use crate::zuir::pass::determinism::DeterminismAssessment;

/// Canonical determinism policy.
pub use crate::zuir::pass::determinism::DeterminismPolicy;

/// Validate an individual pass's determinism declaration.
pub use crate::zuir::pass::determinism::validate_pass_determinism;

/// Assess the determinism declarations of an ordered pipeline.
pub use crate::zuir::pass::determinism::assess_pipeline_determinism;

// ---------------------------------------------------------------------------
// Analysis invalidation
// ---------------------------------------------------------------------------

/// Canonical analysis invalidation implementation.
pub use crate::zuir::pass::invalidation::compute_invalidated_analyses;

// ---------------------------------------------------------------------------
// Compiler-facing aliases
// ---------------------------------------------------------------------------

/// Compiler-facing pass manager.
///
/// The underlying IR representation is generic.
///
/// No compiler-specific IR or hardware enumeration is introduced.
pub type CompilerPassManager<I> = PassManager<I>;

/// Compiler-facing operational pass trait.
pub use OperationalPass as CompilerOperationalPass;

/// Compiler-facing execution context.
pub use PassContext as CompilerPassContext;

/// Compiler-facing verification interface.
pub use IrVerifier as CompilerIrVerifier;

/// Compiler-facing manager configuration.
pub use PassManagerConfig as CompilerPassManagerConfig;

/// Compiler-facing pipeline report.
pub use PipelineReport as CompilerPipelineReport;

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;

    #[derive(Debug)]
    struct IdentityVerifier;

    impl IrVerifier<u64> for IdentityVerifier {
        fn verify(&self, _ir: &u64) -> Result<(), OperationalPassError> {
            Ok(())
        }
    }

    #[derive(Debug)]
    struct IncrementPass {
        descriptor: OperationalPassDescriptor,
    }

    impl OperationalPass<u64> for IncrementPass {
        fn descriptor(&self) -> OperationalPassDescriptor {
            self.descriptor.clone()
        }

        fn run(
            &mut self,
            ir: &mut u64,
            _context: &mut PassContext,
        ) -> Result<OperationalPassOutcome, OperationalPassError> {
            *ir = ir.checked_add(1).ok_or_else(|| {
                OperationalPassError::new(
                    OperationalPassErrorKind::PassFailed,
                    "integer overflow in test pass",
                )
            })?;

            Ok(OperationalPassOutcome::changed())
        }
    }

    fn increment_pass() -> IncrementPass {
        IncrementPass {
            descriptor: OperationalPassDescriptor::new(
                OperationalPassId::new("compiler.test_increment")
                    .expect("test pass ID must be valid"),
                OperationalPassVersion::new(1, 0, 0),
                "Increment test IR",
            ),
        }
    }

    #[test]
    fn compiler_manager_is_the_canonical_zuir_manager() {
        let manager: CompilerPassManager<u64> =
            CompilerPassManager::new(PassManagerConfig::default());

        assert_eq!(manager.pass_count(), 0);
        assert!(manager.is_empty());
    }

    #[test]
    fn compiler_manager_executes_registered_passes() {
        let mut manager = CompilerPassManager::<u64>::new(
            PassManagerConfig::default(),
        )
        .with_verifier(Arc::new(IdentityVerifier));

        manager
            .add_pass(increment_pass())
            .expect("pass registration should succeed");

        let mut ir = 10_u64;

        let report = manager
            .run(&mut ir)
            .expect("pipeline execution should succeed");

        assert_eq!(ir, 11);
        assert_eq!(report.executed_passes(), 1);
        assert!(report.changed);
    }

    #[test]
    fn manager_preserves_existing_operational_api() {
        let manager = CompilerPassManager::<u64>::new(
            PassManagerConfig::default(),
        );

        assert_eq!(manager.pass_count(), 0);
        assert!(manager.is_empty());
    }

    #[test]
    fn canonical_contracts_are_available() {
        let id = PassId::new("classical.canonicalize")
            .expect("canonical pass ID must be valid");

        let descriptor = PassDescriptor::new(
            id,
            PassVersion::new(1, 0, 0),
            "Canonical contract integration test",
        );

        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn deterministic_policy_is_available() {
        let policy = DeterminismPolicy::default();

        let descriptor = PassDescriptor::new(
            PassId::new("quantum.test")
                .expect("pass ID must be valid"),
            PassVersion::new(1, 0, 0),
            "Quantum test pass",
        );

        assert!(
            validate_pass_determinism(&descriptor, policy).is_ok()
        );
    }

    #[test]
    fn cancellation_is_available_through_compiler_namespace() {
        let token = CancellationToken::new();

        assert!(!token.is_cancelled());

        token.cancel();

        assert!(token.is_cancelled());
    }
}
