
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Distributed Execution Adapter
//!
//! Path: src/zur/distributed/adapter.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Provides the ZUR integration boundary for the canonical Zamani
//! distributed execution subsystem.
//!
//! This adapter connects:
//!
//!     ZUR execution requests
//!             |
//!             v
//!     Opaque execution-plan references
//!             |
//!             v
//!     Authoritative distributed-plan resolution
//!             |
//!             v
//!     Canonical distributed execution plans
//!             |
//!             v
//!     DistributedExecutor
//!             |
//!             v
//!     Actual execution providers
//!
//! The adapter does not implement another distributed execution engine.
//!
//! ## ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - ZUR-to-distributed execution-plan resolution.
//! - Distributed executor integration.
//! - Execution-request adaptation.
//! - Distributed execution result propagation.
//! - Preservation of partial execution failures.
//! - The adapter's dependency-injection boundary.
//!
//! This module DOES NOT own:
//!
//! - Distributed semantic analysis.
//! - AST representations.
//! - ZUIR or domain-specific IR.
//! - Distributed task definitions.
//! - Distributed execution-plan construction.
//! - Feasibility analysis.
//! - Resource allocation.
//! - Resource reservation.
//! - Workload partitioning.
//! - Placement.
//! - Scheduling.
//! - Routing.
//! - Node discovery.
//! - Membership management.
//! - Network transport.
//! - Authentication.
//! - Authorization.
//! - Checkpoint storage.
//! - Recovery algorithms.
//! - Provider implementation.
//! - Backend selection.
//! - Hardware discovery.
//! - Quantum execution.
//! - Hybrid execution.
//!
//! These remain the responsibilities of their authoritative subsystems.
//!
//! ## POCO-REAF
//!
//! The adapter supports:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! It introduces no fixed maximum for:
//!
//! - Distributed plans.
//! - Logical tasks.
//! - Participants.
//! - Providers.
//! - Computational domains.
//! - Devices.
//! - Workload sizes.
//! - Execution results.
//!
//! Actual limits are determined by available resources, the underlying
//! distributed executor, provider capabilities and explicit policies.
//!
//! Literal infinite physical computation is not promised.
//!
//! ## SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe blocks.
//! No raw-pointer ownership.
//! No mutable global state.
//! No implicit provider selection in this adapter.
//! No fabricated execution success.
//! No automatic retry.
//! No silent plan substitution.
//!
//! ## INTEGRATION AUTHORITIES
//!
//! ZUR:
//! - crate::zur::error
//! - crate::zur::contracts::execution
//!
//! Distributed:
//! - crate::distributed::execution::executor
//! - crate::distributed::planning
//! - crate::distributed::error
//!
//! The adapter consumes these authorities without redefining them.

use std::fmt;
use std::sync::Arc;

use crate::distributed::execution::executor::{
    DistributedExecutor,
    ExecutionControl,
    ExecutionFailure,
    ExecutionReport,
    ExecutablePlan,
};

use crate::distributed::error::DistributedResult;

use crate::zur::contracts::execution::{
    ExecutionPlanReference,
    ExecutionRequest,
};

use crate::zur::error::{
    ZurError,
    ZurResult,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Canonical component identity of this adapter.
pub const COMPONENT: &str = "zur.distributed.adapter";

/// Adapter contract identity.
pub const ADAPTER_CONTRACT_ID: &str =
    "zamani.zur.distributed.adapter";

/// Adapter contract major version.
pub const ADAPTER_CONTRACT_MAJOR: u16 = 1;

/// Adapter contract minor version.
pub const ADAPTER_CONTRACT_MINOR: u16 = 0;

/// Adapter contract patch version.
pub const ADAPTER_CONTRACT_PATCH: u16 = 0;

// ============================================================================
// DISTRIBUTED PLAN RESOLUTION
// ============================================================================

/// Resolves an opaque ZUR execution-plan reference into its
/// authoritative distributed execution-plan representation.
///
/// # Ownership
///
/// The resolver implementation must integrate with the existing
/// distributed planning and plan-storage authorities.
///
/// It must not construct a competing execution plan.
///
/// # Validation
///
/// A resolver must establish that:
///
/// - The referenced plan exists.
/// - The reference identifies the intended plan authority.
/// - The plan belongs to the appropriate execution context.
/// - The plan has not been substituted with another plan.
/// - The plan's integrity and provenance are established by
///   the authoritative plan owner.
/// - The returned plan implements the canonical distributed
///   ExecutablePlan contract.
///
/// The distributed executor independently invokes
/// `validate_for_execution()` before executing the plan.
///
/// # Resource ownership
///
/// The returned Arc represents shared ownership of the resolved
/// plan representation.
///
/// It does not reserve execution resources or establish provider
/// availability.
///
/// # Extensibility
///
/// Implementations may use:
///
/// - In-memory plan stores.
/// - Durable plan repositories.
/// - Distributed plan services.
/// - Compiler artifact authorities.
/// - Validated plan caches.
/// - Future plan-storage implementations.
///
/// These are implementation choices, not a closed inventory.
///
/// # Thread safety
///
/// Implementations must support concurrent resolution when shared
/// between multiple adapter operations.
///
/// # Errors
///
/// Returns the original ZUR error if resolution or validation fails.
pub trait DistributedPlanResolver<P>: Send + Sync
where
    P: ExecutablePlan + 'static,
{
    /// Resolves the supplied opaque plan reference.
    ///
    /// The returned plan must be the exact authoritative plan
    /// identified by the supplied reference.
    fn resolve(
        &self,
        reference: &ExecutionPlanReference,
    ) -> ZurResult<Arc<P>>;
}

// ============================================================================
// DISTRIBUTED EXECUTION ADAPTER
// ============================================================================

/// ZUR adapter over the canonical distributed execution engine.
///
/// The adapter is generic over the existing distributed plan,
/// work-item representation, provider output and plan resolver.
///
/// It does not define new distributed work-item or plan types.
///
/// # Type parameters
///
/// `P`:
///     Existing authoritative distributed execution-plan type.
///
/// `W`:
///     Existing distributed execution work-item representation.
///
/// `O`:
///     Provider-produced execution output.
///
/// `R`:
///     Authoritative ZUR plan-reference resolver.
///
/// # Ownership
///
/// The adapter owns references to:
///
/// - The canonical distributed executor.
/// - The authoritative plan resolver.
///
/// It does not own the distributed providers, their physical
/// resources or the plan-storage implementation.
///
/// # Concurrency
///
/// The adapter contains no mutable process-global state.
///
/// Shared executor and resolver implementations must satisfy
/// their respective concurrency contracts.
pub struct DistributedExecutionAdapter<P, W, O, R>
where
    P: ExecutablePlan<Work = W> + 'static,
    W: Send + Sync + 'static,
    R: DistributedPlanResolver<P> + 'static,
{
    executor: DistributedExecutor<W, O>,
    resolver: Arc<R>,
}

impl<P, W, O, R> fmt::Debug
    for DistributedExecutionAdapter<P, W, O, R>
where
    P: ExecutablePlan<Work = W> + 'static,
    W: Send + Sync + 'static,
    R: DistributedPlanResolver<P> + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DistributedExecutionAdapter")
            .field("component", &COMPONENT)
            .field("contract_id", &ADAPTER_CONTRACT_ID)
            .finish_non_exhaustive()
    }
}

// ============================================================================
// CONSTRUCTION
// ============================================================================

impl<P, W, O, R> DistributedExecutionAdapter<P, W, O, R>
where
    P: ExecutablePlan<Work = W> + 'static,
    W: Send + Sync + 'static,
    R: DistributedPlanResolver<P> + 'static,
{
    /// Constructs a distributed execution adapter.
    ///
    /// All dependencies are supplied explicitly.
    ///
    /// No providers are discovered or initialized automatically.
    ///
    /// No hardware is selected during construction.
    ///
    /// # Arguments
    ///
    /// `executor`:
    ///     The existing canonical distributed execution executor.
    ///
    /// `resolver`:
    ///     The authoritative ZUR-to-distributed-plan resolver.
    ///
    /// # Errors
    ///
    /// Construction does not perform execution-plan resolution,
    /// provider discovery or resource reservation.
    ///
    /// Invalid dependencies must be rejected by their owning
    /// constructors before being supplied here.
    #[must_use]
    pub fn new(
        executor: DistributedExecutor<W, O>,
        resolver: Arc<R>,
    ) -> Self {
        Self {
            executor,
            resolver,
        }
    }

    /// Returns the adapter contract identity.
    #[must_use]
    pub const fn contract_id(&self) -> &'static str {
        ADAPTER_CONTRACT_ID
    }

    /// Returns the adapter contract version.
    #[must_use]
    pub const fn contract_version(&self) -> (u16, u16, u16) {
        (
            ADAPTER_CONTRACT_MAJOR,
            ADAPTER_CONTRACT_MINOR,
            ADAPTER_CONTRACT_PATCH,
        )
    }

    /// Resolves a canonical distributed plan from a ZUR plan reference.
    ///
    /// This operation does not execute the plan.
    ///
    /// The resolver must establish the identity, authority and
    /// provenance of the referenced plan.
    ///
    /// The returned plan is subsequently validated by the canonical
    /// distributed executor before execution.
    ///
    /// # Errors
    ///
    /// Returns the original resolution error.
    pub fn resolve_plan(
        &self,
        reference: &ExecutionPlanReference,
    ) -> ZurResult<Arc<P>> {
        self.resolver
            .resolve(reference)
            .map_err(|error| {
                error.with_component(COMPONENT)
                    .with_operation("resolve_distributed_plan")
            })
    }

    /// Executes a previously resolved authoritative distributed plan.
    ///
    /// This method delegates all actual execution to the canonical
    /// distributed executor.
    ///
    /// It does not:
    ///
    /// - Modify the plan.
    /// - Reorder work items.
    /// - Select physical nodes.
    /// - Recalculate placement.
    /// - Recalculate scheduling.
    /// - Select providers independently.
    /// - Retry failed operations.
    ///
    /// Provider assessment and execution are delegated to
    /// `DistributedExecutor`.
    ///
    /// # Results
    ///
    /// The outer result represents adapter-level failure.
    ///
    /// The inner result represents actual distributed execution:
    ///
    /// - `Ok(ExecutionReport)` means every planned work item
    ///   completed successfully.
    /// - `Err(ExecutionFailure)` preserves the original execution
    ///   failure and all successfully completed preceding tasks.
    ///
    /// Partial execution must never be promoted to full success.
    ///
    /// # Errors
    ///
    /// Returns the original distributed execution failure,
    /// preserving its partial report.
    pub fn execute_resolved(
        &self,
        plan: &P,
        control: &ExecutionControl,
    ) -> Result<
        ExecutionReport<O>,
        ExecutionFailure<O>,
    > {
        self.executor.execute(plan, control)
    }

    /// Resolves and executes the distributed plan referenced by a
    /// canonical ZUR execution request.
    ///
    /// This is the principal integration entry point for ZUR's
    /// distributed execution layer.
    ///
    /// # Execution sequence
    ///
    /// 1. Receive the already-constructed ZUR execution request.
    /// 2. Obtain its opaque canonical plan reference.
    /// 3. Resolve that exact reference through the injected
    ///    authoritative resolver.
    /// 4. Delegate structural and execution validation to the
    ///    canonical distributed executor.
    /// 5. Delegate provider assessment and execution.
    /// 6. Return the actual execution report or original failure.
    ///
    /// # Important
    ///
    /// The caller must construct `ExecutionControl` from the
    /// execution's authoritative cancellation and deadline context.
    ///
    /// Cancellation must be observed throughout execution by the
    /// distributed execution providers.
    ///
    /// The supplied request's operation context must also be
    /// observed by the surrounding ZUR execution coordinator.
    ///
    /// The adapter does not manufacture cancellation guarantees
    /// that the underlying providers do not implement.
    ///
    /// Artifact integrity, ABI compatibility and authorization
    /// remain the responsibilities of their canonical ZUR and
    /// distributed authorities.
    ///
    /// # Errors
    ///
    /// The outer error reports plan-reference resolution failure.
    ///
    /// The inner error reports actual distributed execution failure
    /// and preserves completed partial results.
    pub fn execute_request(
        &self,
        request: &ExecutionRequest<'_>,
        control: &ExecutionControl,
    ) -> ZurResult<
        Result<ExecutionReport<O>, ExecutionFailure<O>>,
    > {
        let plan = self.resolve_plan(request.plan())?;

        let execution = self.execute_resolved(
            plan.as_ref(),
            control,
        );

        Ok(execution)
    }

    /// Executes a plan using an explicit opaque ZUR plan reference.
    ///
    /// This method is useful when a surrounding ZUR execution
    /// coordinator manages the artifact and execution lifecycle
    /// independently.
    ///
    /// # Errors
    ///
    /// Returns a plan-resolution error or the original distributed
    /// execution failure.
    pub fn execute_reference(
        &self,
        reference: &ExecutionPlanReference,
        control: &ExecutionControl,
    ) -> ZurResult<
        Result<ExecutionReport<O>, ExecutionFailure<O>>,
    > {
        let plan = self.resolve_plan(reference)?;

        Ok(self.execute_resolved(
            plan.as_ref(),
            control,
        ))
    }
}

// ============================================================================
// DISTRIBUTED RESULT ADAPTATION
// ============================================================================

/// Canonical result type for distributed adapter resolution operations.
pub type DistributedAdapterResult<T> = ZurResult<T>;

/// Converts a distributed-subsystem error into a ZUR operational error.
///
/// This helper is intended for integration boundaries that require
/// a ZUR-level diagnostic.
///
/// The original distributed error is retained as the causal source.
///
/// Prefer returning `ExecutionFailure` directly whenever partial
/// execution results must remain available.
pub fn adapt_distributed_error(
    error: crate::distributed::error::DistributedError,
) -> ZurError {
    let kind = match error.kind() {
        crate::distributed::error::DistributedErrorKind::InvalidInput
        | crate::distributed::error::DistributedErrorKind::InvalidConfiguration
        | crate::distributed::error::DistributedErrorKind::SemanticInvalid => {
            crate::zur::error::ErrorKind::Validation
        }

        crate::distributed::error::DistributedErrorKind::CapabilityUnsatisfied => {
            crate::zur::error::ErrorKind::CapabilityUnavailable
        }

        crate::distributed::error::DistributedErrorKind::ResourceUnsatisfied => {
            crate::zur::error::ErrorKind::ResourceUnavailable
        }

        crate::distributed::error::DistributedErrorKind::Unrepresentable => {
            crate::zur::error::ErrorKind::Infeasible
        }

        crate::distributed::error::DistributedErrorKind::TransportFailed => {
            crate::zur::error::ErrorKind::Communication
        }

        crate::distributed::error::DistributedErrorKind::AuthenticationFailed
        | crate::distributed::error::DistributedErrorKind::AuthorizationDenied => {
            crate::zur::error::ErrorKind::Security
        }

        crate::distributed::error::DistributedErrorKind::DeadlineExceeded => {
            crate::zur::error::ErrorKind::TimedOut
        }

        crate::distributed::error::DistributedErrorKind::Cancelled => {
            crate::zur::error::ErrorKind::Cancelled
        }

        crate::distributed::error::DistributedErrorKind::OperationOutcomeUnknown => {
            crate::zur::error::ErrorKind::Distributed
        }

        _ => crate::zur::error::ErrorKind::Distributed,
    };

    let message = error.to_string();

    let source = error;

    ZurError::with_code(
        kind,
        "zur.distributed.execution.failed",
        message,
    )
    .expect("the adapter error code is a reviewed constant")
    .with_source(source)
    .with_component(COMPONENT)
    .with_operation("distributed_execution")
}

// ============================================================================
// INTEGRATION INVARIANTS
// ============================================================================

/// Documents the execution invariants required by this adapter.
///
/// These invariants are intentionally represented as documentation
/// rather than duplicated runtime state.
pub mod invariants {
    /// A plan reference must resolve to its exact authoritative plan.
    pub const EXACT_PLAN_RESOLUTION: &str =
        "The adapter must never silently substitute another plan.";

    /// Distributed execution must validate the resolved plan.
    pub const VALIDATED_PLAN_REQUIRED: &str =
        "Only canonical executable plans may reach the executor.";

    /// Execution results must represent actual provider outcomes.
    pub const ACTUAL_EXECUTION_REQUIRED: &str =
        "Submission or provider selection is not execution completion.";

    /// Partial execution must retain its completed task results.
    pub const PARTIAL_RESULTS_REQUIRED: &str =
        "Execution failures preserve their partial reports.";

    /// Distributed planning remains independently authoritative.
    pub const PLANNING_AUTHORITY_PRESERVED: &str =
        "The adapter must not repeat placement, routing or scheduling.";

    /// Physical communication remains independently authoritative.
    pub const TRANSPORT_AUTHORITY_PRESERVED: &str =
        "All physical communication remains owned by distributed transport.";

    /// Retry decisions remain independently authoritative.
    pub const RESILIENCE_AUTHORITY_PRESERVED: &str =
        "The adapter must not retry operations independently.";

    /// No computational-domain inventory is hard-coded.
    pub const OPEN_DOMAIN_EXTENSIBILITY: &str =
        "Future computational domains integrate through existing contracts.";

    /// Safe Rust is mandatory.
    pub const SAFE_RUST_ONLY: &str =
        "Unsafe Rust is prohibited.";
}

// ============================================================================
// COMPLETION CONTRACT
// ============================================================================
//
// This adapter is complete when:
//
// 1. The authoritative distributed plan resolver is implemented.
// 2. The resolver validates opaque plan references.
// 3. The canonical DistributedExecutor is injected.
// 4. Existing distributed providers are injected through its registry.
// 5. Actual distributed execution is performed by the canonical executor.
// 6. Partial results are preserved.
// 7. Distributed errors retain their original classification.
// 8. Cancellation and deadlines are propagated by the caller.
// 9. Artifact and ABI validation remain with ZUR execution.
// 10. Authentication and authorization remain with their authorities.
// 11. No distributed planner or transport implementation is duplicated.
// 12. No arbitrary resource ceilings are introduced.
// 13. No global mutable state is introduced.
// 14. No unsafe Rust is introduced.
// 15. All dependent repository modules compile and their tests pass.
