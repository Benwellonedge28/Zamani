
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Toolchain — Operational Budget Integration.
//!
//! Path:
//!     src/toolchain/configuration/budget.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum supported version: Rust 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Provides the toolchain-facing operational budget API by integrating
//! the canonical compiler budget implementation.
//!
//! The toolchain coordinates project, workspace, dependency, package,
//! build, task, testing and deployment operations.
//!
//! These operations must be able to share explicit operational accounting
//! without creating independent or competing budget implementations.
//!
//! This module therefore reuses:
//!
//!     crate::compiler::configuration::budget
//!
//! It introduces no second budget ledger, resource model, accounting
//! algorithm or physical resource-management implementation.
//!
//! ============================================================================
//! ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! This module owns:
//!
//!     - Toolchain-facing budget API exports.
//!     - Toolchain budget naming aliases.
//!     - Toolchain budget API version.
//!     - Integration documentation and compatibility guarantees.
//!
//! The canonical compiler budget module owns:
//!
//!     - BudgetDimension.
//!     - BudgetLimit.
//!     - CompilationBudget.
//!     - BudgetRequest.
//!     - BudgetTracker.
//!     - BudgetUsage.
//!     - BudgetSnapshot.
//!     - BudgetError.
//!     - BudgetResult.
//!     - Atomic accounting.
//!     - Transactional batch accounting.
//!     - Overflow detection.
//!     - Concurrent accounting synchronization.
//!     - Deterministic usage snapshots.
//!
//! ============================================================================
//! NON-OWNERSHIP
//! ============================================================================
//!
//! This module does NOT own:
//!
//!     - Source-language budget syntax.
//!     - Semantic resource requirements.
//!     - Resource capabilities.
//!     - Resource preferences.
//!     - Resource constraints.
//!     - Resource hints.
//!     - Feasibility evaluation.
//!     - Physical hardware discovery.
//!     - Hardware capacity models.
//!     - Physical resource allocation.
//!     - Device selection.
//!     - Runtime scheduling.
//!     - Distributed scheduling.
//!     - Quantum routing.
//!     - Quantum error correction.
//!     - Backend implementation.
//!     - Runtime execution.
//!     - Package dependency resolution.
//!     - Filesystem or network access.
//!
//! These responsibilities remain with their existing authoritative
//! repository subsystems.
//!
//! ============================================================================
//! POCO-REAF AND SCALABILITY
//! ============================================================================
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! No universal hardware-capacity ceiling is introduced here.
//!
//! In particular, this module defines no maximum for:
//!
//!     - CPUs, cores, workers or threads;
//!     - GPUs, FPGAs, QPUs or accelerators;
//!     - physical or logical qubits;
//!     - memory or storage capacity;
//!     - tensor dimensions or tensor rank;
//!     - registers or register widths;
//!     - distributed participants;
//!     - nodes, devices or network links;
//!     - compilation units or operations;
//!     - targets, providers or computational domains.
//!
//! An unlimited operational budget means that no explicit limit is
//! configured for the corresponding accounting dimension.
//!
//! It does NOT mean that physical resources are infinite.
//!
//! Actual execution remains subject to:
//!
//!     - available implementation resources;
//!     - operating-system limits;
//!     - provider capabilities;
//!     - semantic requirements;
//!     - explicit policies;
//!     - physical resource availability;
//!     - operational budgets.
//!
//! The canonical accounting API uses u128 counters.
//!
//! Accounting overflow is an explicit error. It is never silently
//! wrapped, truncated or saturated.
//!
//! These counters must not be used as a replacement for arbitrary-precision
//! semantic resource quantities or physical capacity representations.
//!
//! ============================================================================
//! RESOURCE SEMANTICS
//! ============================================================================
//!
//! The following concepts MUST remain separate:
//!
//! Requirement:
//!     What a program or operation needs.
//!
//! Capability:
//!     What a provider or environment can supply.
//!
//! Preference:
//!     Which compatible realization is preferred.
//!
//! Constraint:
//!     A condition that must be satisfied.
//!
//! Hint:
//!     Advisory information for planning or selection.
//!
//! Feasibility:
//!     Whether a proposed realization satisfies the applicable conditions.
//!
//! Budget:
//!     An operational allowance or accounting boundary.
//!
//! This module does not convert one category into another.
//!
//! In particular, an exhausted toolchain budget does not establish
//! that a program is semantically invalid or that a hardware target
//! is inherently incapable of executing it.
//!
//! ============================================================================
//! CONCURRENCY
//! ============================================================================
//!
//! All toolchain consumers must share the same BudgetTracker for one
//! logical operation whenever their work contributes to the same budget.
//!
//! Cloning a BudgetTracker shares its existing ledger.
//!
//! Constructing another tracker from the same CompilationBudget creates
//! another independent ledger and MUST NOT be used as a substitute for
//! sharing the original tracker.
//!
//! Multi-dimensional consumption must use consume_batch when all requested
//! dimensions must succeed or fail together.
//!
//! A failed accounting operation must not be reported as successful.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! The canonical implementation uses ordered dimension and usage maps.
//!
//! Budget snapshots therefore have deterministic ordering.
//!
//! Observed consumption remains dependent on the actual work performed.
//! Deterministic ordering does not imply deterministic execution time.
//!
//! ============================================================================
//! ERROR CONTRACT
//! ============================================================================
//!
//! Budget failures remain the original typed BudgetError.
//!
//! They are not converted into strings or generic success/failure flags.
//!
//! Toolchain callers must preserve:
//!
//!     - budget exhaustion;
//!     - accounting overflow;
//!     - invalid dimensions;
//!     - poisoned accounting state.
//!
//! Retry, cancellation, fallback and recovery decisions belong to the
//! operation coordinator and its applicable policies.
//!
//! ============================================================================
//! INTEGRATION CONTRACT
//! ============================================================================
//!
//! Toolchain configuration:
//!     configuration/config.rs
//!
//! The aggregate configuration may carry budget configuration through
//! its existing typed configuration-section mechanism.
//!
//! Toolchain build:
//!     build/executor.rs
//!
//! Creates or receives the tracker for the logical build operation.
//!
//! Toolchain tasks:
//!     tasks/executor.rs
//!
//! Shares the same tracker with tasks contributing to the build.
//!
//! Compiler:
//!     compiler/context/compilation_context.rs
//!
//! Receives the shared compiler BudgetTracker.
//!
//! Compiler pipeline:
//!     compiler/pipeline/execution.rs
//!
//! Charges measurable stage work.
//!
//! Parallel compilation:
//!     compiler/parallel/
//!
//! Shares the same tracker across participating workers.
//!
//! Incremental compilation:
//!     compiler/incremental/
//!
//! Includes the effective budget configuration in cache identity
//! whenever that configuration can change compilation results.
//!
//! Runtime:
//!     zur/
//!
//! Uses runtime-owned resource allocation and accounting contracts.
//!
//! Physical hardware:
//!     zhal/
//!
//! Uses hardware-provider-owned capacity and allocation contracts.
//!
//! Source-language semantics:
//!     frontend/semantic/
//!
//! Remains authoritative for resource meaning.
//!
//! ============================================================================
//! IMPORTANT INTEGRATION RULE
//! ============================================================================
//!
//! The toolchain must not create a fresh compiler tracker for every task.
//!
//! The correct ownership model is:
//!
//!     One logical toolchain operation
//!                  |
//!                  v
//!         One shared BudgetTracker
//!                  |
//!          +-------+-------+
//!          |       |       |
//!          v       v       v
//!        Task A  Task B  Task C
//!          |       |       |
//!          +-------+-------+
//!                  |
//!                  v
//!          Compiler context
//!                  |
//!          +-------+-------+
//!          |       |       |
//!          v       v       v
//!       Frontend Pipeline Backend
//!                  |
//!                  v
//!          Shared accounting
//!
//! This ensures that consumption is accounted against one logical
//! operational budget rather than several independent copies.
//!
//! ============================================================================
//! FREEZE CRITERIA
//! ============================================================================
//!
//! This file is ready to freeze when:
//!
//!     - The canonical compiler budget implementation is available.
//!     - The toolchain configuration module exports this module.
//!     - Public aliases compile.
//!     - Shared tracker identity is preserved.
//!     - Batch accounting remains transactional.
//!     - Overflow remains an explicit error.
//!     - All downstream consumers preserve accounting errors.
//!     - No duplicate budget implementation is introduced.

/// Canonical operational budget result.
pub use crate::compiler::configuration::budget::BudgetResult;

/// Canonical operational budget dimension.
pub use crate::compiler::configuration::budget::BudgetDimension;

/// Canonical operational budget error.
pub use crate::compiler::configuration::budget::BudgetError;

/// Canonical optional operational budget limit.
pub use crate::compiler::configuration::budget::BudgetLimit;

/// Canonical immutable operational budget configuration.
pub use crate::compiler::configuration::budget::CompilationBudget;

/// Canonical multi-dimensional consumption request.
pub use crate::compiler::configuration::budget::BudgetRequest;

/// Canonical shared, thread-safe budget tracker.
pub use crate::compiler::configuration::budget::BudgetTracker;

/// Canonical usage information for one dimension.
pub use crate::compiler::configuration::budget::BudgetUsage;

/// Canonical deterministic accounting snapshot.
pub use crate::compiler::configuration::budget::BudgetSnapshot;

// ============================================================================
// TOOLCHAIN-FACING ALIASES
// ============================================================================

/// Toolchain-facing operational budget configuration.
///
/// This is an alias, not a second budget implementation.
pub type ToolchainBudget = CompilationBudget;

/// Toolchain-facing shared accounting tracker.
///
/// All consumers of one logical operation must share the same instance.
pub type ToolchainBudgetTracker = BudgetTracker;

/// Toolchain-facing accounting request.
pub type ToolchainBudgetRequest = BudgetRequest;

/// Toolchain-facing budget error.
pub type ToolchainBudgetError = BudgetError;

/// Toolchain-facing budget result.
pub type ToolchainBudgetResult<T> = BudgetResult<T>;

/// Toolchain-facing accounting snapshot.
pub type ToolchainBudgetSnapshot = BudgetSnapshot;

/// Toolchain-facing usage information.
pub type ToolchainBudgetUsage = BudgetUsage;

// ============================================================================
// API VERSION
// ============================================================================

/// Public version of the toolchain budget integration contract.
///
/// Independent of:
///
/// - Zamani language version.
/// - Grammar version.
/// - Compiler release version.
/// - Compiler budget implementation version.
/// - Toolchain release version.
/// - Backend ABI version.
/// - Runtime version.
/// - Hardware-provider versions.
///
/// Increment when this module's public integration contract changes
/// incompatibly.
pub const TOOLCHAIN_BUDGET_API_VERSION: u16 = 1;

/// Returns the toolchain budget integration API version.
#[must_use]
pub const fn api_version() -> u16 {
    TOOLCHAIN_BUDGET_API_VERSION
}

// ============================================================================
// CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_version_is_nonzero() {
        assert_ne!(TOOLCHAIN_BUDGET_API_VERSION, 0);
        assert_eq!(api_version(), TOOLCHAIN_BUDGET_API_VERSION);
    }

    #[test]
    fn toolchain_budget_is_the_canonical_compiler_budget() {
        let budget = ToolchainBudget::unlimited();

        assert!(budget.is_unlimited());
    }

    #[test]
    fn custom_dimensions_are_open_world() {
        let dimension = BudgetDimension::new(
            "future_domain.operation_work",
            "work_units",
        )
        .expect("valid extension dimension");

        assert_eq!(dimension.name(), "future_domain.operation_work");
        assert_eq!(dimension.unit(), "work_units");
    }

    #[test]
    fn units_are_part_of_dimension_identity() {
        let bytes = BudgetDimension::new("memory", "bytes")
            .expect("valid dimension");

        let operations = BudgetDimension::new("memory", "operations")
            .expect("valid dimension");

        assert_ne!(bytes, operations);
    }

    #[test]
    fn cloned_trackers_share_one_accounting_ledger() {
        let dimension = BudgetDimension::new(
            "toolchain.build_work",
            "work_units",
        )
        .expect("valid dimension");

        let budget = ToolchainBudget::new().with_limit(
            dimension.clone(),
            BudgetLimit::bounded(10),
        );

        let tracker = budget.tracker();
        let shared = tracker.shared();

        tracker
            .consume(dimension.clone(), 4)
            .expect("first consumption succeeds");

        shared
            .consume(dimension.clone(), 3)
            .expect("shared consumption succeeds");

        assert_eq!(
            tracker.consumed(&dimension).expect("accounting succeeds"),
            7
        );

        assert_eq!(
            shared.consumed(&dimension).expect("accounting succeeds"),
            7
        );
    }

    #[test]
    fn budget_requests_use_the_canonical_accounting_contract() {
        let dimension = BudgetDimension::new(
            "toolchain.package_work",
            "work_units",
        )
        .expect("valid dimension");

        let budget = ToolchainBudget::new().with_limit(
            dimension.clone(),
            BudgetLimit::bounded(8),
        );

        let tracker = budget.tracker();

        let request = ToolchainBudgetRequest::new(
            dimension.clone(),
            5,
        );

        tracker
            .consume_batch(&[request])
            .expect("transaction succeeds");

        assert_eq!(
            tracker.consumed(&dimension).expect("accounting succeeds"),
            5
        );
    }

    #[test]
    fn failed_batch_does_not_partially_consume_dimensions() {
        let first = BudgetDimension::new(
            "toolchain.first",
            "work_units",
        )
        .expect("valid dimension");

        let second = BudgetDimension::new(
            "toolchain.second",
            "work_units",
        )
        .expect("valid dimension");

        let budget = ToolchainBudget::new()
            .with_limit(
                first.clone(),
                BudgetLimit::bounded(10),
            )
            .with_limit(
                second.clone(),
                BudgetLimit::bounded(0),
            );

        let tracker = budget.tracker();

        let requests = [
            ToolchainBudgetRequest::new(first.clone(), 5),
            ToolchainBudgetRequest::new(second.clone(), 1),
        ];

        assert!(tracker.consume_batch(&requests).is_err());

        assert_eq!(
            tracker.consumed(&first).expect("accounting succeeds"),
            0
        );

        assert_eq!(
            tracker.consumed(&second).expect("accounting succeeds"),
            0
        );
    }

    #[test]
    fn unlimited_budget_does_not_mean_infinite_resources() {
        let budget = ToolchainBudget::unlimited();

        let dimension = BudgetDimension::new(
            "future_domain.capacity_accounting",
            "units",
        )
        .expect("valid dimension");

        assert!(
            budget.limit(&dimension).is_unlimited()
        );
    }

    #[test]
    fn accounting_overflow_is_reported() {
        let dimension = BudgetDimension::new(
            "toolchain.accounting",
            "units",
        )
        .expect("valid dimension");

        let tracker = ToolchainBudget::unlimited().tracker();

        tracker
            .consume(dimension.clone(), u128::MAX)
            .expect("maximum representable accounting value");

        assert!(matches!(
            tracker.consume(dimension, 1),
            Err(BudgetError::AccountingOverflow { .. })
        ));
    }
}
