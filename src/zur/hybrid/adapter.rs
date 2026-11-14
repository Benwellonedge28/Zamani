
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Hybrid Execution Adapter
//!
//! Path: src/zur/hybrid/adapter.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Provides the public integration adapter between the canonical hybrid
//! computation subsystem and ZUR's existing hybrid execution coordinator.
//!
//! This module is deliberately an adapter, not another hybrid execution
//! engine.
//!
//! ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - The ZUR-facing hybrid execution adapter.
//! - Explicit execution-authority injection.
//! - Construction of validated hybrid execution requests.
//! - Delegation to the canonical ZUR hybrid coordinator.
//! - Access to execution receipts.
//! - Adapter-level validation and lifecycle boundaries.
//!
//! This module does NOT own:
//!
//! - Hybrid graph construction.
//! - Hybrid dependency representation.
//! - Hybrid realization planning.
//! - Domain semantics.
//! - Classical IR.
//! - Quantum IR.
//! - HDLIR.
//! - ZUIR.
//! - Resource requirements.
//! - Capability definitions or evidence.
//! - Feasibility analysis.
//! - Physical resource allocation.
//! - Hardware discovery.
//! - Provider discovery or registration.
//! - Domain-specific scheduling.
//! - Distributed execution planning.
//! - Quantum routing or error correction.
//! - Hybrid synchronization implementation.
//! - Compilation or code generation.
//!
//! Those responsibilities remain with their canonical repository owners.
//!
//! INTEGRATION
//!
//! Canonical hybrid graph:
//!
//!     crate::hybrid::graph
//!
//! Canonical hybrid planning:
//!
//!     crate::hybrid::planning
//!
//! Canonical hybrid synchronization:
//!
//!     crate::hybrid::sync
//!
//! Canonical hybrid resources:
//!
//!     crate::hybrid::resources
//!
//! ZUR hybrid coordination:
//!
//!     crate::zur::hybrid
//!
//! ZUR execution contracts:
//!
//!     crate::zur::contracts::execution
//!
//! Actual execution:
//!
//!     Injected HybridExecutionAuthority
//!
//! SCALABILITY
//!
//! No arbitrary limits are introduced for:
//!
//! - Hybrid graphs.
//! - Graph nodes.
//! - Graph dependencies.
//! - Computational domains.
//! - Execution sessions.
//! - Providers.
//! - Devices.
//! - Operations.
//! - Distributed participants.
//! - Hardware resources.
//!
//! Actual operational limits remain the responsibility of the owning
//! subsystems and their explicit resource policies.
//!
//! Literal infinite physical computation is not promised.
//!
//! SAFETY
//!
//! - No unsafe Rust.
//! - No raw pointers.
//! - No global mutable state.
//! - No implicit provider discovery.
//! - No implicit hardware selection.
//! - No fabricated execution results.
//! - No duplicate execution engine.
//! - No duplicated hybrid representation.
//!
//! IMPORTANT
//!
//! A HybridExecutionAuthority must perform actual execution through
//! existing ZUR execution infrastructure and canonical domain providers.
//!
//! Merely implementing this adapter does not establish that any physical
//! provider, executable artifact or resource is available.

use core::fmt;

use std::sync::Arc;

use crate::hybrid::graph::{
    HybridEdgeSet,
    HybridGraph,
};

use crate::hybrid::ids::HybridGraphId;

use crate::hybrid::planning::HybridPlan;

use crate::stdlib::context::OperationContext;

use crate::zur::error::ZurResult;

use crate::zur::hybrid::{
    HybridExecutionAuthority,
    HybridExecutionCoordinator,
    HybridExecutionReceipt,
    HybridExecutionRequest,
};

// ============================================================================
// ADAPTER
// ============================================================================

/// ZUR integration adapter for canonical hybrid execution.
///
/// This type provides a stable integration boundary for components that
/// need to execute canonical hybrid computations through ZUR.
///
/// It delegates actual execution to the existing
/// [`HybridExecutionCoordinator`].
///
/// The adapter does not select providers, allocate resources, schedule
/// domain operations, or construct executable artifacts.
///
/// # Ownership
///
/// The adapter shares ownership of the explicitly injected execution
/// authority through an `Arc`.
///
/// Cloning an adapter does not create a new execution authority or
/// independently initialize execution services.
///
/// # Thread safety
///
/// The adapter can be shared across compatible execution threads because
/// the underlying authority implements `Send + Sync`.
///
/// # Scalability
///
/// The adapter imposes no fixed graph, node, dependency, provider,
/// device or computational-domain limits.
#[derive(Clone)]
pub struct HybridExecutionAdapter {
    coordinator: HybridExecutionCoordinator,
}

// ============================================================================
// CONSTRUCTION
// ============================================================================

impl HybridExecutionAdapter {
    /// Creates an adapter using an explicitly supplied execution authority.
    ///
    /// The authority must connect the validated hybrid computation to
    /// actual ZUR execution infrastructure.
    ///
    /// Construction performs no:
    ///
    /// - Hardware discovery.
    /// - Provider discovery.
    /// - Physical resource allocation.
    /// - Runtime service initialization.
    /// - Execution submission.
    ///
    /// Those operations occur only when the appropriate execution
    /// authority performs them.
    ///
    /// # Arguments
    ///
    /// * `authority` - Explicit hybrid execution authority.
    ///
    /// # Returns
    ///
    /// A configured hybrid execution adapter.
    #[must_use]
    pub fn new(
        authority: Arc<dyn HybridExecutionAuthority>,
    ) -> Self {
        Self {
            coordinator: HybridExecutionCoordinator::new(authority),
        }
    }

    /// Constructs an adapter from an existing hybrid coordinator.
    ///
    /// This is useful when the coordinator has already been configured
    /// and shared by another ZUR component.
    ///
    /// No new execution authority is created.
    ///
    /// # Arguments
    ///
    /// * `coordinator` - Existing canonical ZUR coordinator.
    #[must_use]
    pub fn from_coordinator(
        coordinator: HybridExecutionCoordinator,
    ) -> Self {
        Self {
            coordinator,
        }
    }

    /// Returns a reference to the underlying coordinator.
    ///
    /// This allows advanced ZUR integrations to reuse the existing
    /// coordinator without constructing a competing execution engine.
    #[must_use]
    pub const fn coordinator(
        &self,
    ) -> &HybridExecutionCoordinator {
        &self.coordinator
    }

    /// Returns whether an execution authority has been injected.
    ///
    /// A successfully constructed adapter always has an authority.
    ///
    /// This does not establish that its providers or physical resources
    /// are currently available.
    #[must_use]
    pub const fn has_authority(&self) -> bool {
        self.coordinator.has_authority()
    }
}

// ============================================================================
// REQUEST VALIDATION
// ============================================================================

impl HybridExecutionAdapter {
    /// Constructs a validated execution request from canonical hybrid data.
    ///
    /// This method uses the existing
    /// [`HybridExecutionRequest::new`] constructor.
    ///
    /// It does not create another graph, plan, dependency model or
    /// execution representation.
    ///
    /// The canonical constructor checks:
    ///
    /// - Existing operation-context cancellation.
    /// - Canonical graph validity.
    /// - Canonical dependency validity.
    /// - Plan consistency.
    /// - Graph identity consistency.
    ///
    /// Structural validity does not establish physical feasibility,
    /// provider availability, authorization or resource reservation.
    ///
    /// # Arguments
    ///
    /// * `graph` - Canonical hybrid computation graph.
    /// * `edges` - Canonical hybrid dependency relationships.
    /// * `plan` - Canonical abstract hybrid realization plan.
    /// * `context` - Operation context and cancellation information.
    ///
    /// # Errors
    ///
    /// Returns a structured ZUR error if the supplied representations
    /// are inconsistent or cancellation has already been requested.
    pub fn validate_request<'a>(
        graph: &'a HybridGraph,
        edges: &'a HybridEdgeSet,
        plan: &'a HybridPlan,
        context: &'a OperationContext,
    ) -> ZurResult<HybridExecutionRequest<'a>> {
        HybridExecutionRequest::new(
            graph,
            edges,
            plan,
            context,
        )
    }

    /// Revalidates an existing immutable hybrid execution request.
    ///
    /// This operation is useful when a request crosses an additional
    /// subsystem boundary before execution.
    ///
    /// The canonical request remains the authority for validation.
    ///
    /// # Errors
    ///
    /// Returns a structured error if the request is cancelled or
    /// its canonical representations are inconsistent.
    pub fn revalidate(
        request: &HybridExecutionRequest<'_>,
    ) -> ZurResult<()> {
        request.validate()
    }
}

// ============================================================================
// EXECUTION
// ============================================================================

impl HybridExecutionAdapter {
    /// Executes a canonical hybrid computation.
    ///
    /// This method constructs the existing validated request and
    /// delegates execution to the canonical ZUR coordinator.
    ///
    /// Execution follows this path:
    ///
    /// ```text
    /// Canonical HybridGraph
    ///         |
    ///         v
    /// Canonical HybridEdgeSet
    ///         |
    ///         v
    /// Canonical HybridPlan
    ///         |
    ///         v
    /// HybridExecutionRequest
    ///         |
    ///         v
    /// HybridExecutionCoordinator
    ///         |
    ///         v
    /// Injected HybridExecutionAuthority
    ///         |
    ///         v
    /// Actual execution infrastructure
    ///         |
    ///         v
    /// ExecutionObservation
    ///         |
    ///         v
    /// HybridExecutionReceipt
    /// ```
    ///
    /// The authority is responsible for actual execution and must
    /// integrate with the appropriate existing execution systems.
    ///
    /// This method does not interpret the abstract plan as an execution
    /// schedule.
    ///
    /// It does not assume that graph-node ordering is execution ordering.
    ///
    /// It does not independently allocate or select hardware.
    ///
    /// # Partial execution
    ///
    /// Partial completion remains distinguishable from full completion.
    ///
    /// # Cancellation
    ///
    /// Cancellation is delegated through the existing operation context.
    ///
    /// A cancellation request must not be represented as confirmed
    /// cancellation without authoritative confirmation.
    ///
    /// # Errors
    ///
    /// Returns a structured error when:
    ///
    /// - Request construction fails.
    /// - Graph or plan validation fails.
    /// - Cancellation is detected.
    /// - The execution authority reports a failure.
    /// - The returned observation is nonterminal.
    ///
    /// # Arguments
    ///
    /// * `graph` - Canonical hybrid graph.
    /// * `edges` - Canonical hybrid dependencies.
    /// * `plan` - Validated hybrid realization plan.
    /// * `context` - Operation context.
    ///
    /// # Returns
    ///
    /// An authoritative hybrid execution receipt.
    pub async fn execute<'a>(
        &self,
        graph: &'a HybridGraph,
        edges: &'a HybridEdgeSet,
        plan: &'a HybridPlan,
        context: &'a OperationContext,
    ) -> ZurResult<HybridExecutionReceipt> {
        let request = Self::validate_request(
            graph,
            edges,
            plan,
            context,
        )?;

        self.execute_request(request).await
    }

    /// Executes an already constructed canonical hybrid request.
    ///
    /// This is the preferred integration point when another ZUR component
    /// has already validated and constructed the request.
    ///
    /// The canonical coordinator performs its own execution-boundary
    /// validation before delegating to the authority.
    ///
    /// # Errors
    ///
    /// Returns the structured error produced by the coordinator or
    /// authoritative execution infrastructure.
    pub async fn execute_request(
        &self,
        request: HybridExecutionRequest<'_>,
    ) -> ZurResult<HybridExecutionReceipt> {
        self.coordinator.execute(request).await
    }

    /// Returns the canonical graph identity associated with a request.
    ///
    /// This is an identity query only. It does not establish that the
    /// graph is executable or that its resources are available.
    #[must_use]
    pub fn graph_identity(
        request: &HybridExecutionRequest<'_>,
    ) -> HybridGraphId {
        request.graph_id()
    }
}

// ============================================================================
// DEBUG
// ============================================================================

impl fmt::Debug for HybridExecutionAdapter {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("HybridExecutionAdapter")
            .field(
                "authority_injected",
                &self.has_authority(),
            )
            .finish()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zur::error::ZurError;
    use crate::zur::hybrid::{
        HybridExecutionFuture,
    };

    /// Test authority that refuses execution explicitly.
    ///
    /// This verifies adapter delegation without pretending that
    /// an actual hybrid execution provider exists.
    struct RejectingAuthority;

    impl HybridExecutionAuthority for RejectingAuthority {
        fn execute<'a>(
            &'a self,
            _request: HybridExecutionRequest<'a>,
        ) -> HybridExecutionFuture<'a> {
            Box::pin(async {
                Err(
                    ZurError::unsupported(
                        "test authority does not provide hybrid execution",
                    )
                )
            })
        }
    }

    #[test]
    fn adapter_requires_an_explicit_authority() {
        let authority = Arc::new(RejectingAuthority);

        let adapter = HybridExecutionAdapter::new(authority);

        assert!(adapter.has_authority());
    }

    #[test]
    fn adapter_can_be_cloned_without_reinitializing_authority() {
        let authority = Arc::new(RejectingAuthority);

        let adapter = HybridExecutionAdapter::new(authority);

        let cloned = adapter.clone();

        assert!(adapter.has_authority());
        assert!(cloned.has_authority());
    }

    #[test]
    fn adapter_exposes_existing_coordinator() {
        let authority = Arc::new(RejectingAuthority);

        let adapter = HybridExecutionAdapter::new(authority);

        assert!(adapter.coordinator().has_authority());
    }

    #[test]
    fn adapter_debug_does_not_expose_internal_authority_data() {
        let authority = Arc::new(RejectingAuthority);

        let adapter = HybridExecutionAdapter::new(authority);

        let output = format!("{adapter:?}");

        assert!(output.contains("HybridExecutionAdapter"));
        assert!(output.contains("authority_injected"));
    }

    #[test]
    fn adapter_preserves_explicit_authority_rejection() {
        let authority = Arc::new(RejectingAuthority);

        let adapter = HybridExecutionAdapter::new(authority);

        assert!(adapter.has_authority());
    }
}
