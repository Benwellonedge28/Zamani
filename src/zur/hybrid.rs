
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Hybrid Execution Integration
//!
//! Path: `src/zur/hybrid.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provide the ZUR execution integration boundary for validated hybrid
//! computations represented by the canonical `crate::hybrid` subsystem.
//!
//! ZUR coordinates execution. The hybrid subsystem remains authoritative
//! for hybrid computation representation, domain composition, dependency
//! relationships, cross-domain data contracts, synchronization,
//! validation and abstract realization planning.
//!
//! ## ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - Hybrid runtime execution request validation.
//! - Integration between canonical hybrid plans and ZUR execution.
//! - An explicit hybrid execution-authority interface.
//! - Execution delegation.
//! - Execution-observation validation.
//! - Hybrid execution receipt construction.
//! - Graph identity preservation.
//! - Runtime cancellation checks at the execution boundary.
//!
//! This module does NOT own:
//!
//! - Source syntax or grammar.
//! - AST construction.
//! - Semantic analysis or type checking.
//! - Classical, quantum, AI, tensor or HDL semantics.
//! - Classical IR, quantum IR, HDLIR or ZUIR.
//! - Hybrid graph construction.
//! - Hybrid dependency definitions.
//! - Hybrid data-exchange contracts.
//! - Hybrid synchronization semantics.
//! - Hybrid resource requirements or capabilities.
//! - Hybrid feasibility assessment.
//! - Compiler planning or optimization.
//! - Physical hardware discovery.
//! - Hardware selection.
//! - Resource allocation or reservation.
//! - Domain-specific scheduling.
//! - Quantum routing or error correction.
//! - Device drivers.
//! - Provider registration or discovery.
//! - Distributed execution planning.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! ## EXECUTION ARCHITECTURE
//!
//! Canonical hybrid representation:
//!
//!     crate::hybrid::graph
//!             |
//!             v
//!     crate::hybrid::planning
//!             |
//!             v
//!     Validated HybridExecutionRequest
//!             |
//!             v
//!     ZUR HybridExecutionCoordinator
//!             |
//!             v
//!     HybridExecutionAuthority
//!             |
//!             v
//!     Existing ZUR execution/provider infrastructure
//!             |
//!             v
//!     Actual execution observations
//!             |
//!             v
//!     HybridExecutionReceipt
//!
//! The authority implementation is responsible for connecting the
//! validated request to the appropriate compiler artifacts, execution
//! engine, provider contracts and operational services.
//!
//! ## IMPORTANT DISTINCTIONS
//!
//! - A valid graph is not proof of target feasibility.
//! - A valid plan is not proof of resource availability.
//! - Provider registration is not proof of provider availability.
//! - Capability evidence is not resource reservation.
//! - Submission is not execution completion.
//! - Cancellation request is not confirmed cancellation.
//! - Partial completion is not full completion.
//!
//! ## SCALABILITY
//!
//! No fixed maximum is imposed on:
//!
//! - Computational domains.
//! - Graph nodes.
//! - Graph edges.
//! - Operations.
//! - Hybrid execution sessions.
//! - Providers.
//! - Devices.
//! - Quantum resources.
//! - Accelerators.
//! - Distributed participants.
//! - Cross-domain execution relationships.
//!
//! The underlying canonical structures and execution authorities
//! remain subject to actual memory, address-space, processing-time,
//! provider and resource availability.
//!
//! Explicit operational budgets belong to their existing owners.
//!
//! Literal infinite physical computation is not promised.
//!
//! ## PORTABILITY
//!
//! Hybrid execution must preserve the validated computation's meaning
//! across compatible execution environments.
//!
//! The execution authority must not silently remove dependencies,
//! weaken requirements, substitute incompatible operations or report
//! success when execution has not actually completed.
//!
//! ## SAFETY
//!
//! This module uses safe Rust exclusively.
//!
//! It introduces:
//!
//! - No unsafe code.
//! - No raw-pointer ownership.
//! - No global mutable state.
//! - No hard-coded hardware inventories.
//! - No fabricated execution results.
//! - No implicit physical resource allocation.
//! - No automatic provider selection.
//! - No duplicate hybrid representation.

use core::fmt;
use core::future::Future;
use core::pin::Pin;

use std::sync::Arc;

use crate::hybrid::graph::{
    HybridEdgeSet,
    HybridGraph,
};

use crate::hybrid::ids::HybridGraphId;

use crate::hybrid::planning::HybridPlan;

use crate::stdlib::context::OperationContext;

use crate::zur::contracts::execution::{
    ExecutionObservation,
    ExecutionStatus,
};

use crate::zur::error::{
    ZurError,
    ZurResult,
};

// ============================================================================
// ASYNCHRONOUS EXECUTION CONTRACT
// ============================================================================

/// Future returned by a hybrid execution authority.
///
/// The future is tied to the lifetime of the execution authority and
/// the canonical hybrid execution request.
///
/// `Send` allows compatible multithreaded execution environments to
/// drive the operation.
///
/// Implementations that require a different execution model must
/// provide an appropriate compatible adapter.
pub type HybridExecutionFuture<'a> =
    Pin<
        Box<
            dyn Future<Output = ZurResult<ExecutionObservation>>
                + Send
                + 'a
        >
    >;

// ============================================================================
// HYBRID EXECUTION REQUEST
// ============================================================================

/// A validated request to execute a canonical hybrid computation.
///
/// The request borrows the canonical graph, edge set and realization
/// plan. It does not copy or reconstruct any of those representations.
///
/// The operation context supplies cancellation and operation metadata.
///
/// Construction validates the plan against the exact graph and edge
/// set supplied by the caller.
///
/// This establishes structural consistency. It does not establish
/// physical resource availability, provider availability, authorization
/// or target feasibility.
///
/// All referenced representations remain immutable through this API.
pub struct HybridExecutionRequest<'a> {
    graph: &'a HybridGraph,

    edges: &'a HybridEdgeSet,

    plan: &'a HybridPlan,

    context: &'a OperationContext,
}

impl<'a> HybridExecutionRequest<'a> {
    /// Constructs a hybrid execution request.
    ///
    /// Validation includes:
    ///
    /// 1. Operation-context cancellation checking.
    /// 2. Canonical hybrid graph validation.
    /// 3. Canonical edge-set validation.
    /// 4. Hybrid plan validation.
    /// 5. Graph/edge/plan identity consistency.
    ///
    /// The canonical plan's `validate_against` operation performs
    /// the structural checks against the supplied graph and edges.
    ///
    /// # Errors
    ///
    /// Returns a structured ZUR error when cancellation has already
    /// been requested or the hybrid representation is inconsistent.
    pub fn new(
        graph: &'a HybridGraph,
        edges: &'a HybridEdgeSet,
        plan: &'a HybridPlan,
        context: &'a OperationContext,
    ) -> ZurResult<Self> {
        if context.is_cancelled() {
            return Err(
                ZurError::cancelled(
                    "hybrid execution request was cancelled before validation",
                )
            );
        }

        plan.validate_against(graph, edges)
            .map_err(|error| {
                ZurError::invalid_data(
                    format!(
                        "hybrid graph, edge set and realization plan \
                         are inconsistent: {error}"
                    ),
                )
            })?;

        Ok(Self {
            graph,
            edges,
            plan,
            context,
        })
    }

    /// Returns the canonical hybrid computation graph.
    #[must_use]
    pub fn graph(&self) -> &'a HybridGraph {
        self.graph
    }

    /// Returns the canonical dependency edge set.
    #[must_use]
    pub fn edges(&self) -> &'a HybridEdgeSet {
        self.edges
    }

    /// Returns the validated hybrid realization plan.
    #[must_use]
    pub fn plan(&self) -> &'a HybridPlan {
        self.plan
    }

    /// Returns the operation context.
    ///
    /// The execution authority must propagate cancellation to the
    /// actual operations it manages.
    #[must_use]
    pub fn context(&self) -> &'a OperationContext {
        self.context
    }

    /// Returns the graph identity associated with this request.
    #[must_use]
    pub fn graph_id(&self) -> HybridGraphId {
        self.graph.id()
    }

    /// Returns the number of canonical graph nodes.
    ///
    /// This describes the supplied graph. It is not an execution
    /// capacity or hardware limit.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Returns the number of canonical dependency edges.
    ///
    /// This describes the supplied edge set. It is not a maximum
    /// supported dependency count.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Revalidates the immutable request at an execution boundary.
    ///
    /// This method is useful when a request crosses another subsystem
    /// boundary before being passed to an execution authority.
    ///
    /// It does not perform hardware discovery or target selection.
    pub fn validate(&self) -> ZurResult<()> {
        if self.context.is_cancelled() {
            return Err(
                ZurError::cancelled(
                    "hybrid execution was cancelled before authority invocation",
                )
            );
        }

        self.plan
            .validate_against(self.graph, self.edges)
            .map_err(|error| {
                ZurError::invalid_data(
                    format!(
                        "hybrid execution request failed revalidation: {error}"
                    ),
                )
            })
    }
}

impl fmt::Debug for HybridExecutionRequest<'_> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("HybridExecutionRequest")
            .field("graph_id", &self.graph.id())
            .field("node_count", &self.graph.node_count())
            .field("edge_count", &self.edges.len())
            .field("plan_node_count", &self.plan.node_count())
            .field("plan_dependency_count", &self.plan.dependency_count())
            .field("cancelled", &self.context.is_cancelled())
            .finish()
    }
}

// ============================================================================
// HYBRID EXECUTION AUTHORITY
// ============================================================================

/// Execution authority for a validated hybrid computation.
///
/// This interface is the boundary between ZUR's hybrid coordination
/// and the existing execution/provider infrastructure.
///
/// An implementation must delegate actual computation to the appropriate
/// existing execution engine and domain-provider implementations.
///
/// It must not replace:
///
/// - The compiler.
/// - The backend registry.
/// - The canonical hybrid planner.
/// - The domain-provider registry.
/// - The distributed execution subsystem.
/// - Quantum execution infrastructure.
/// - Hardware abstraction facilities.
///
/// # Implementation requirements
///
/// Implementations must:
///
/// - Preserve the supplied graph and plan semantics.
/// - Preserve all required dependency relationships.
/// - Respect canonical synchronization contracts.
/// - Preserve cross-domain data ownership and lifetime requirements.
/// - Use existing provider contracts.
/// - Validate actual artifact and execution compatibility.
/// - Respect security and authorization policies.
/// - Acquire resources only through their authoritative resource systems.
/// - Propagate cancellation to underlying operations.
/// - Report actual execution observations.
/// - Preserve partial completion and failure information.
/// - Never fabricate successful execution.
///
/// Implementations must not claim that an operation completed merely
/// because it was accepted or submitted.
///
/// A returned observation must represent an actual observation from
/// the authoritative execution infrastructure.
pub trait HybridExecutionAuthority: Send + Sync {
    /// Executes the supplied validated hybrid computation.
    ///
    /// The implementation may coordinate multiple existing execution
    /// engines and domain providers.
    ///
    /// It must return an observation for the complete hybrid execution
    /// request, not merely an indication that submission succeeded.
    ///
    /// The observation must represent an actual execution state.
    ///
    /// Nonterminal observations are rejected by the coordinator.
    ///
    /// Partial completion remains distinguishable from full completion.
    fn execute<'a>(
        &'a self,
        request: HybridExecutionRequest<'a>,
    ) -> HybridExecutionFuture<'a>;
}

// ============================================================================
// HYBRID EXECUTION RECEIPT
// ============================================================================

/// Receipt for an observed hybrid execution.
///
/// The receipt retains the identity of the canonical hybrid graph and
/// the authoritative ZUR execution observation.
///
/// It does not invent result values or copy provider-owned result data.
///
/// The result reference, if present, remains owned by the execution
/// authority identified through the canonical ZUR execution contracts.
#[derive(Debug, Clone)]
pub struct HybridExecutionReceipt {
    graph_id: HybridGraphId,

    observation: ExecutionObservation,
}

impl HybridExecutionReceipt {
    /// Constructs a receipt from an authoritative terminal observation.
    ///
    /// A nonterminal observation cannot be accepted as a completed
    /// hybrid execution receipt.
    ///
    /// Partial completion is retained as a valid, distinguishable
    /// execution outcome.
    ///
    /// # Errors
    ///
    /// Returns an error if the observation is not terminal.
    fn new(
        graph_id: HybridGraphId,
        observation: ExecutionObservation,
    ) -> ZurResult<Self> {
        if !observation.state().is_terminal() {
            return Err(
                ZurError::invalid_data(
                    format!(
                        "hybrid execution authority returned a nonterminal \
                         observation: {:?}",
                        observation.state().status()
                    ),
                )
            );
        }

        Ok(Self {
            graph_id,
            observation,
        })
    }

    /// Returns the identity of the executed hybrid graph.
    #[must_use]
    pub const fn graph_id(&self) -> HybridGraphId {
        self.graph_id
    }

    /// Returns the authoritative ZUR execution observation.
    #[must_use]
    pub const fn observation(&self) -> &ExecutionObservation {
        &self.observation
    }

    /// Returns the observed execution status.
    #[must_use]
    pub const fn status(&self) -> ExecutionStatus {
        self.observation.state().status()
    }

    /// Returns whether execution completed successfully in full.
    ///
    /// Partial completion, cancellation, timeout and failure all
    /// return `false`.
    #[must_use]
    pub const fn is_fully_completed(&self) -> bool {
        self.status().is_successful()
    }

    /// Returns whether the observation represents partial completion.
    ///
    /// Partial completion is not silently promoted to full success.
    #[must_use]
    pub const fn is_partially_completed(&self) -> bool {
        matches!(
            self.status(),
            ExecutionStatus::PartiallyCompleted
        )
    }

    /// Returns the authoritative result reference, if available.
    ///
    /// Result ownership remains with the execution authority.
    #[must_use]
    pub fn result(
        &self,
    ) -> Option<
        &crate::zur::contracts::execution::ExecutionReference
    > {
        self.observation.result()
    }

    /// Returns the authoritative failure reference, if available.
    #[must_use]
    pub fn failure(
        &self,
    ) -> Option<
        &crate::zur::contracts::execution::ExecutionReference
    > {
        self.observation.failure()
    }
}

// ============================================================================
// HYBRID EXECUTION COORDINATOR
// ============================================================================

/// Coordinates hybrid execution through an explicitly supplied authority.
///
/// The coordinator owns no global state and performs no hardware discovery.
///
/// It does not select providers or allocate physical resources.
///
/// The execution authority is injected explicitly and may itself delegate
/// to the existing ZUR execution engine and domain-provider infrastructure.
///
/// Cloned coordinators share the same authority implementation through
/// `Arc`.
#[derive(Clone)]
pub struct HybridExecutionCoordinator {
    authority: Arc<dyn HybridExecutionAuthority>,
}

impl HybridExecutionCoordinator {
    /// Creates a coordinator with an explicit execution authority.
    ///
    /// No providers are discovered and no services are initialized.
    #[must_use]
    pub fn new(
        authority: Arc<dyn HybridExecutionAuthority>,
    ) -> Self {
        Self {
            authority,
        }
    }

    /// Executes a validated hybrid computation.
    ///
    /// Execution proceeds as follows:
    ///
    /// 1. Revalidate the canonical hybrid request.
    /// 2. Check cooperative cancellation.
    /// 3. Delegate execution to the injected authority.
    /// 4. Await an authoritative execution observation.
    /// 5. Reject nonterminal observations.
    /// 6. Construct a graph-associated execution receipt.
    ///
    /// This method does not convert partial completion into success.
    ///
    /// It also does not interpret a successful return as proof that
    /// every physical resource is permanently available.
    ///
    /// # Errors
    ///
    /// Returns a structured error when request validation fails,
    /// execution is cancelled, the authority reports a failure, or
    /// the returned observation is not terminal.
    pub async fn execute(
        &self,
        request: HybridExecutionRequest<'_>,
    ) -> ZurResult<HybridExecutionReceipt> {
        request.validate()?;

        if request.context().is_cancelled() {
            return Err(
                ZurError::cancelled(
                    "hybrid execution was cancelled before delegation",
                )
            );
        }

        let graph_id = request.graph_id();

        let observation = self
            .authority
            .execute(request)
            .await?;

        HybridExecutionReceipt::new(
            graph_id,
            observation,
        )
    }

    /// Returns whether the coordinator has an execution authority.
    ///
    /// A constructed coordinator always has an explicitly supplied
    /// authority. This method does not establish that the authority
    /// currently has available providers or resources.
    #[must_use]
    pub const fn has_authority(&self) -> bool {
        true
    }
}

impl fmt::Debug for HybridExecutionCoordinator {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("HybridExecutionCoordinator")
            .field("authority_injected", &true)
            .finish()
    }
}

// ============================================================================
// INTEGRATION CONTRACT
// ============================================================================

/// Integration requirements for the concrete ZUR hybrid authority.
///
/// A production authority implementation must connect this module to
/// the following existing repository authorities:
///
/// - `crate::hybrid::graph`: canonical graph and dependency relationships.
/// - `crate::hybrid::planning`: validated hybrid realization plans.
/// - `crate::hybrid::data`: cross-domain data and ownership contracts.
/// - `crate::hybrid::sync`: synchronization and completion contracts.
/// - `crate::hybrid::resources`: requirements, capabilities and feasibility.
/// - `crate::zur::contracts::execution`: execution lifecycle and observations.
/// - `crate::zur::contracts::provider`: provider execution operations.
/// - `crate::zur::domains`: domain-provider integration.
/// - `crate::zur::resources`: runtime resource coordination.
/// - `crate::zur::security`: authorization and isolation.
/// - `crate::distributed`: distributed execution.
/// - `crate::quantum`: quantum computation and hardware.
/// - `crate::compiler`: compilation and executable artifacts.
///
/// The authority must not implement those subsystems a second time.
///
/// This marker documents the integration contract without imposing
/// a closed list of computational domains.
pub struct HybridIntegrationContract;

impl HybridIntegrationContract {
    /// Returns the canonical hybrid subsystem module path.
    #[must_use]
    pub const fn hybrid_authority() -> &'static str {
        "crate::hybrid"
    }

    /// Returns the canonical ZUR execution-contract module path.
    #[must_use]
    pub const fn execution_authority() -> &'static str {
        "crate::zur::contracts::execution"
    }

    /// Returns the canonical ZUR provider-contract module path.
    #[must_use]
    pub const fn provider_authority() -> &'static str {
        "crate::zur::contracts::provider"
    }
}
