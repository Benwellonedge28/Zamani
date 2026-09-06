//! Zamani Quantum Resilience — Rerouting Adaptation
//!
//! Path:
//!     src/quantum/resilience/adaptation/rerouting.rs
//!
//! Purpose:
//!     Provide the production resilience adapter for physical rerouting.
//!
//! ============================================================================
//! Architectural responsibility
//! ============================================================================
//!
//! This module translates the canonical resilience action:
//!
//!     ActionKind::Reroute
//!
//! into a request to the authoritative quantum routing subsystem.
//!
//! This module DOES:
//!
//! - implement `AdaptationAdapter`;
//! - accept only the canonical `Reroute` action;
//! - enforce transactional adaptation semantics;
//! - enforce deterministic-mode requirements;
//! - protect against stale execution generations;
//! - protect against stale semantic revisions;
//! - delegate actual route computation to an injected routing service;
//! - preserve provider independence;
//! - expose adapter capabilities;
//! - provide an opaque candidate for prepare/commit;
//! - provide adapter-local verification;
//! - avoid partial commits;
//! - avoid hidden global state;
//! - avoid machine-size assumptions;
//! - support arbitrarily large finite routing problems subject only to the
//!   resources and limits supplied by the routing subsystem;
//! - provide deterministic adapter metadata.
//!
//! This module DOES NOT:
//!
//! - implement Dijkstra;
//! - implement A*;
//! - implement SABRE;
//! - implement noise-aware routing;
//! - implement dynamic routing;
//! - implement topology discovery;
//! - implement logical-to-physical remapping;
//! - insert SWAP operations;
//! - decompose gates;
//! - schedule operations;
//! - compile circuits;
//! - optimize circuits;
//! - communicate directly with providers;
//! - own hardware state;
//! - own calibration data;
//! - own routing topology;
//! - authorize recovery;
//! - decide whether rerouting is safe;
//! - verify final quantum semantics;
//! - contain retry loops;
//! - contain fixed qubit counts;
//! - contain fixed topology sizes;
//! - use `unsafe`.
//!
//! ============================================================================
//! Architectural boundary
//! ============================================================================
//!
//! ```text
//!                    Resilience Planner
//!                           |
//!                           v
//!                    RecoveryAction
//!                           |
//!                           v
//!                  AdaptationRequest
//!                           |
//!                           v
//!                 ReroutingAdapter
//!                           |
//!                           v
//!                 ReroutingService
//!                           |
//!                           v
//!                  quantum::routing
//!                           |
//!               +-----------+-----------+
//!               |           |           |
//!               v           v           v
//!             topology   mapping     routing algorithm
//!               |           |           |
//!               +-----------+-----------+
//!                           |
//!                           v
//!                     routing candidate
//!                           |
//!                           v
//!                 semantic verification
//! ```
//!
//! The important boundary is:
//!
//!     resilience decides WHEN rerouting is required;
//!     routing decides HOW the physical route is computed.
//!
//! ============================================================================
//! Write once, scale everywhere
//! ============================================================================
//!
//! This module introduces no architectural upper bound on:
//!
//! - logical qubits;
//! - physical qubits;
//! - operations;
//! - circuit depth;
//! - topology vertices;
//! - topology edges;
//! - routing epochs;
//! - execution environments;
//! - distributed resources.
//!
//! No machine-specific mapping such as:
//!
//!     logical 0 -> physical 0
//!
//! is created here.
//!
//! No fixed retry count is created here.
//!
//! No fixed topology size is created here.
//!
//! No provider name is encoded into the implementation.
//!
//! Every actual limit is supplied by the authoritative routing, hardware,
//! execution, policy, or resource subsystem.
//!
//! "Infinity" therefore means:
//!
//!     no artificial resilience-level machine-size ceiling.
//!
//! Every physical execution remains finite because real memory, time,
//! topology, provider capacity and execution budgets are finite.
//!
//! ============================================================================
//! Canonical contracts
//! ============================================================================
//!
//! Adaptation contract:
//!
//!     crate::quantum::resilience::adaptation::adapter
//!
//! Canonical action:
//!
//!     crate::quantum::resilience::planning::action
//!
//! Canonical resilience errors:
//!
//!     crate::quantum::resilience::errors
//!
//! Canonical routing subsystem:
//!
//!     crate::quantum::routing
//!
//! Canonical quantum identities, where required by routing implementations:
//!
//!     crate::quantum::ir::qubit
//!
//! This file deliberately does not define a second `QubitId`,
//! `PhysicalQubitId`, topology type, route type, or routing algorithm.
//!
//! ============================================================================
//! Transactional semantics
//! ============================================================================
//!
//! Rerouting follows:
//!
//!     Preflight
//!         |
//!         v
//!     Prepare
//!         |
//!         v
//!     candidate
//!         |
//!         v
//!     Commit
//!         |
//!         v
//!     Verify
//!
//! Preparation MUST NOT claim that the route has been committed.
//!
//! A candidate is bound to:
//!
//! - adapter identity;
//! - action kind;
//! - execution generation;
//! - semantic revision.
//!
//! Therefore a candidate prepared against stale execution state cannot be
//! committed against a newer execution state.
//!
//! ============================================================================
//! Determinism
//! ============================================================================
//!
//! When deterministic behavior is requested:
//!
//! - the adapter does not select a random routing algorithm;
//! - the adapter does not access global mutable state;
//! - the adapter does not manufacture random identifiers;
//! - the injected routing service is responsible for deterministic routing;
//! - the routing configuration/state supplied to that service must therefore
//!   be deterministic.
//!
//! The adapter itself is deterministic for equal explicit inputs.
//!
//! ============================================================================
//! Safety
//! ============================================================================
//!
//! This adapter is NOT an authorization boundary.
//!
//! It must be called only after:
//!
//!     policy validation
//!     capability validation
//!     feasibility validation
//!     security authorization
//!     semantic compatibility validation
//!
//! The final result must still pass the repository's verification subsystem.
//!
//! A successful routing candidate is NOT by itself proof that:
//!
//! - the circuit semantics are preserved;
//! - native gates are executable;
//! - timing constraints are satisfied;
//! - calibration constraints are satisfied;
//! - QEC requirements are preserved;
//! - the execution result is correct.
//!
//! Those concerns remain owned by their authoritative subsystems.
//!
//! ============================================================================
//! Rust compatibility
//! ============================================================================
//!
//! Required:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! No external crate is required by this file.
//!
//! ============================================================================
//! Integration contract
//! ============================================================================
//!
//! `planning/action.rs`:
//!
//!     produces `ActionKind::Reroute`.
//!
//! `planning/planner.rs`:
//!
//!     determines when rerouting should be planned.
//!
//! `planning/feasibility.rs`:
//!
//!     determines whether rerouting is feasible before execution.
//!
//! `adaptation/adapter.rs`:
//!
//!     supplies `AdaptationAdapter`, `AdaptationRequest`,
//!     `AdaptationCandidate`, and `AdaptationResult`.
//!
//! `rerouting.rs`:
//!
//!     implements the adapter contract in this file.
//!
//! `quantum::routing`:
//!
//!     implements the actual routing algorithms.
//!
//! `quantum::hardware`:
//!
//!     supplies authoritative hardware/topology/capability information.
//!
//! `quantum::ir::qubit`:
//!
//!     remains the canonical qubit-identity boundary where concrete routing
//!     implementations need qubit identities.
//!
//! `verification/*`:
//!
//!     verifies semantic correctness after the route is committed.
//!
//! `telemetry/*`:
//!
//!     records adaptation intent/outcome externally.
//!
//! `history/*`:
//!
//!     records routing adaptation outcomes externally.
//!
//! `registry/*`:
//!
//!     registers this adapter.
//!
//! This file does not need to be modified when a new routing algorithm is
//! added. The new algorithm should be exposed through the injected
//! `ReroutingService` implementation or through the existing routing router.
//!
//! ============================================================================
//! Important design decision
//! ============================================================================
//!
//! Do NOT directly depend on one concrete routing algorithm here.
//!
//! For example, this file must not permanently call:
//!
//!     SabreRouter::route(...)
//!
//! because resilience would then become coupled to one implementation.
//!
//! Instead:
//!
//!     ReroutingAdapter<R>
//!
//! receives an implementation of:
//!
//!     ReroutingService
//!
//! The existing `quantum::routing::router` can implement that service.
//!
//! Dynamic routing, noise-aware routing, topology-aware routing and future
//! routing algorithms can then be selected by the routing subsystem without
//! changing this resilience adapter.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

use std::fmt;
use std::sync::Arc;

use crate::quantum::resilience::adaptation::adapter::{
    AdaptationAdapter,
    AdaptationCandidate,
    AdaptationCapabilities,
    AdaptationOperation,
    AdaptationPhase,
    AdaptationRequest,
    AdaptationResult,
    AdaptationStatus,
    AdapterId,
    AdapterVersion,
};

use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};

use crate::quantum::resilience::planning::action::ActionKind;

// ============================================================================
// Stable adapter identity
// ============================================================================

/// Stable semantic identifier for the resilience rerouting adapter.
pub const REROUTING_ADAPTER_ID: &str =
    "zamani.quantum.resilience.rerouting";

/// Semantic version of this adapter implementation.
pub const REROUTING_ADAPTER_VERSION: AdapterVersion =
    AdapterVersion::new(1, 0, 0);

/// Stable schema identifier for this adapter.
pub const REROUTING_ADAPTER_SCHEMA_ID: &str =
    "zamani.quantum.resilience.adaptation.rerouting";

/// Schema version for this adapter contract.
pub const REROUTING_ADAPTER_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Supported action set
// ============================================================================

/// Canonical actions supported by this adapter.
///
/// This is intentionally a single-action adapter.
///
/// Keeping the action set narrow prevents accidental execution of another
/// adaptation operation through the rerouting implementation.
static SUPPORTED_ACTIONS: [ActionKind; 1] = [ActionKind::Reroute];

// ============================================================================
// Routing candidate identity
// ============================================================================

/// Opaque identity returned by the authoritative routing service.
///
/// The resilience layer deliberately does not know whether the underlying
/// router represents a route using:
//!
//! - a route graph;
//! - a sequence of physical edges;
//! - a routing plan;
//! - a dynamic-routing epoch;
//! - a compiled transformation;
//! - a provider-neutral routing artifact.
//!
//! The routing subsystem owns that representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RoutingCandidateId(String);

impl RoutingCandidateId {
    /// Creates a routing candidate identity.
    ///
    /// Empty identities are rejected because an empty identity cannot safely
    /// participate in transactional commit or audit/provenance.
    pub fn new(value: impl Into<String>) -> ResilienceResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidIdentifier,
            ));
        }

        Ok(Self(value))
    }

    /// Returns the opaque routing candidate identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identity and returns the underlying string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for RoutingCandidateId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// Routing service contract
// ============================================================================

/// Authoritative routing service used by the resilience rerouting adapter.
///
/// This is deliberately an integration boundary rather than another routing
/// algorithm.
///
/// A concrete implementation should normally live in or above
/// `crate::quantum::routing` and delegate to the repository's existing router.
///
/// # Required behavior
///
/// Implementations MUST:
///
/// - treat the supplied request as immutable;
/// - validate routing state against the current target;
/// - reject stale routing state;
/// - never return a partial successful route;
/// - preserve the logical computation;
/// - respect the routing subsystem's topology/capability rules;
/// - honor the deterministic requirement;
/// - honor the execution generation;
/// - honor the semantic revision;
/// - avoid hidden global mutable routing state.
///
/// Implementations MUST NOT:
///
/// - silently change logical semantics;
/// - silently change the selected execution environment;
/// - invent physical resources;
/// - assume a fixed number of qubits;
/// - assume a fixed topology;
/// - silently ignore unavailable edges;
/// - silently reverse directed native connectivity.
///
/// The actual routing implementation remains responsible for interpreting
/// the routing request against the canonical `quantum::routing` contracts.
pub trait ReroutingService: Send + Sync + fmt::Debug {
    /// Preflight the rerouting request without changing routing state.
    ///
    /// This should verify all routing-specific preconditions that can be
    /// checked without creating a candidate.
    fn preflight(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<()>;

    /// Prepare a rerouting candidate without committing it.
    ///
    /// The returned candidate identity MUST identify an immutable routing
    /// artifact that can later be passed to `commit`.
    fn prepare(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<RoutingCandidateId>;

    /// Commit a previously prepared routing candidate.
    ///
    /// Implementations MUST reject the candidate if the execution generation,
    /// semantic revision, target topology, or other routing-relevant state has
    /// changed in a way that invalidates the candidate.
    ///
    /// `true` means the candidate was committed.
    ///
    /// `false` means the candidate was not committed.
    fn commit(
        &self,
        request: &AdaptationRequest,
        candidate: &RoutingCandidateId,
    ) -> ResilienceResult<bool>;

    /// Perform adapter-local verification of a committed routing candidate.
    ///
    /// This is NOT the final semantic verification performed by
    /// `quantum::resilience::verification`.
    ///
    /// The service should verify routing-internal invariants such as:
    ///
    /// - route references existing resources;
    /// - route edges are valid;
    /// - route is compatible with the target snapshot;
    /// - route candidate belongs to the expected execution generation;
    /// - route candidate belongs to the expected semantic revision.
    fn verify(
        &self,
        request: &AdaptationRequest,
        candidate: &RoutingCandidateId,
    ) -> ResilienceResult<bool>;

    /// Returns whether the routing service guarantees deterministic behavior
    /// for equal explicit inputs.
    fn deterministic(&self) -> bool {
        true
    }
}

// ============================================================================
// Adapter
// ============================================================================

/// Production resilience adapter for physical rerouting.
///
/// `R` is deliberately injected.
///
/// This prevents the resilience layer from becoming coupled to a particular
/// routing implementation.
#[derive(Clone)]
pub struct ReroutingAdapter<R>
where
    R: ReroutingService,
{
    id: AdapterId,
    router: Arc<R>,
}

impl<R> fmt::Debug for ReroutingAdapter<R>
where
    R: ReroutingService,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ReroutingAdapter")
            .field("id", &self.id)
            .field("router", &self.router)
            .finish()
    }
}

impl<R> ReroutingAdapter<R>
where
    R: ReroutingService,
{
    /// Creates a rerouting adapter around an authoritative routing service.
    ///
    /// The routing service is shared through `Arc` so a single immutable
    /// routing implementation can safely be used by multiple resilience
    /// execution paths.
    pub fn new(router: Arc<R>) -> ResilienceResult<Self> {
        let id = AdapterId::new(REROUTING_ADAPTER_ID)?;

        Ok(Self { id, router })
    }

    /// Creates a rerouting adapter from an owned routing service.
    pub fn from_router(router: R) -> ResilienceResult<Self> {
        Self::new(Arc::new(router))
    }

    /// Returns the injected routing service.
    #[must_use]
    pub fn router(&self) -> &R {
        self.router.as_ref()
    }

    /// Returns the shared routing service handle.
    #[must_use]
    pub fn router_handle(&self) -> &Arc<R> {
        &self.router
    }

    /// Returns whether the underlying routing service is deterministic.
    #[must_use]
    pub fn routing_is_deterministic(&self) -> bool {
        self.router.deterministic()
    }

    /// Validates that a request is specifically a rerouting request.
    fn validate_reroute_request(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<()> {
        request.validate()?;

        if request.action_kind() != ActionKind::Reroute {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        if request.deterministic_required()
            && !self.router.deterministic()
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::CompatibilityFailure,
            ));
        }

        Ok(())
    }

    /// Validates an opaque adaptation candidate before forwarding it to the
    /// routing service.
    fn validate_candidate(
        &self,
        request: &AdaptationRequest,
        candidate: &AdaptationCandidate,
    ) -> ResilienceResult<RoutingCandidateId> {
        if candidate.adapter() != &self.id {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        if candidate.action() != ActionKind::Reroute {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        if candidate.generation() != request.generation() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::PlanStale,
            ));
        }

        if candidate.semantic_revision()
            != request.semantic_revision()
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::SemanticAdaptationViolation,
            ));
        }

        RoutingCandidateId::new(candidate.identity().to_owned())
    }

    /// Creates the resilience-level candidate from a routing-service
    /// candidate.
    fn make_candidate(
        &self,
        request: &AdaptationRequest,
        routing_candidate: RoutingCandidateId,
    ) -> ResilienceResult<AdaptationCandidate> {
        AdaptationCandidate::new(
            self.id.clone(),
            ActionKind::Reroute,
            request.generation().clone(),
            request.semantic_revision().clone(),
            routing_candidate.into_string(),
        )
    }

    /// Performs preparation through the routing service.
    fn prepare_reroute(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<AdaptationResult> {
        self.validate_reroute_request(request)?;

        let candidate_id = self.router.prepare(request)?;

        let candidate =
            self.make_candidate(request, candidate_id)?;

        Ok(
            AdaptationResult::new(
                self.id.clone(),
                ActionKind::Reroute,
                AdaptationStatus::Prepared,
                request.generation().clone(),
                request.semantic_revision().clone(),
            )
            .with_candidate(candidate),
        )
    }

    /// Commits a previously prepared routing candidate.
    fn commit_reroute(
        &self,
        request: &AdaptationRequest,
        candidate: &AdaptationCandidate,
    ) -> ResilienceResult<AdaptationResult> {
        self.validate_reroute_request(request)?;

        let routing_candidate =
            self.validate_candidate(request, candidate)?;

        let committed =
            self.router.commit(request, &routing_candidate)?;

        if !committed {
            return Ok(
                AdaptationResult::new(
                    self.id.clone(),
                    ActionKind::Reroute,
                    AdaptationStatus::Rejected,
                    request.generation().clone(),
                    request.semantic_revision().clone(),
                )
                .with_candidate(candidate.clone()),
            );
        }

        Ok(
            AdaptationResult::new(
                self.id.clone(),
                ActionKind::Reroute,
                AdaptationStatus::CommittedPendingVerification,
                request.generation().clone(),
                request.semantic_revision().clone(),
            )
            .with_candidate(candidate.clone()),
        )
    }

    /// Performs adapter-local verification.
    fn verify_reroute(
        &self,
        request: &AdaptationRequest,
        candidate: &AdaptationCandidate,
    ) -> ResilienceResult<AdaptationResult> {
        self.validate_reroute_request(request)?;

        let routing_candidate =
            self.validate_candidate(request, candidate)?;

        let verified =
            self.router.verify(request, &routing_candidate)?;

        if !verified {
            return Ok(
                AdaptationResult::new(
                    self.id.clone(),
                    ActionKind::Reroute,
                    AdaptationStatus::Rejected,
                    request.generation().clone(),
                    request.semantic_revision().clone(),
                )
                .with_candidate(candidate.clone()),
            );
        }

        Ok(
            AdaptationResult::new(
                self.id.clone(),
                ActionKind::Reroute,
                AdaptationStatus::Committed,
                request.generation().clone(),
                request.semantic_revision().clone(),
            )
            .with_candidate(candidate.clone()),
        )
    }
}

// ============================================================================
// AdaptationAdapter implementation
// ============================================================================

impl<R> AdaptationAdapter for ReroutingAdapter<R>
where
    R: ReroutingService,
{
    fn id(&self) -> &AdapterId {
        &self.id
    }

    fn version(&self) -> AdapterVersion {
        REROUTING_ADAPTER_VERSION
    }

    fn capabilities(&self) -> AdaptationCapabilities {
        AdaptationCapabilities::new(
            true,  // prepare
            true,  // commit
            true,  // preflight
            self.router.deterministic(),
            true,  // scoped
            true,  // partial preservation is delegated to routing
            false, // rollback is owned by recovery, not rerouting
        )
    }

    fn supported_actions(&self) -> &[ActionKind] {
        &SUPPORTED_ACTIONS
    }

    fn preflight(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<()> {
        self.validate_reroute_request(request)?;

        self.router.preflight(request)
    }

    fn execute(
        &self,
        operation: &AdaptationOperation,
        request: &AdaptationRequest,
    ) -> ResilienceResult<AdaptationResult> {
        match operation {
            AdaptationOperation::Preflight => {
                self.preflight(request)?;

                Ok(
                    AdaptationResult::new(
                        self.id.clone(),
                        ActionKind::Reroute,
                        AdaptationStatus::Prepared,
                        request.generation().clone(),
                        request.semantic_revision().clone(),
                    ),
                )
            }

            AdaptationOperation::Prepare => {
                self.prepare_reroute(request)
            }

            AdaptationOperation::Commit { candidate } => {
                self.commit_reroute(request, candidate)
            }

            AdaptationOperation::Verify { candidate } => {
                self.verify_reroute(request, candidate)
            }
        }
    }
}

// ============================================================================
// Shared adapter handle
// ============================================================================

/// Thread-safe shared rerouting adapter handle.
///
/// This is useful when the resilience registry stores the adapter as a
/// type-erased `AdaptationAdapterHandle`.
pub type ReroutingAdapterHandle<R> =
    Arc<ReroutingAdapter<R>>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::resilience::adaptation::adapter::{
        AdaptationCandidate,
        AdaptationOperation,
        AdaptationRequest,
        ExecutionGeneration,
        SemanticRevision,
    };

    use crate::quantum::resilience::planning::action::{
        ActionPayload,
        ActionScope,
        RecoveryAction,
    };

    /// Deterministic in-memory routing service used only for contract tests.
    ///
    /// It deliberately does not model a particular machine size or topology.
    #[derive(Debug, Default)]
    struct TestRoutingService;

    impl ReroutingService for TestRoutingService {
        fn preflight(
            &self,
            request: &AdaptationRequest,
        ) -> ResilienceResult<()> {
            if request.action_kind() != ActionKind::Reroute {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::InvalidArgument,
                ));
            }

            Ok(())
        }

        fn prepare(
            &self,
            _request: &AdaptationRequest,
        ) -> ResilienceResult<RoutingCandidateId> {
            RoutingCandidateId::new("test-routing-candidate")
        }

        fn commit(
            &self,
            _request: &AdaptationRequest,
            candidate: &RoutingCandidateId,
        ) -> ResilienceResult<bool> {
            Ok(candidate.as_str() == "test-routing-candidate")
        }

        fn verify(
            &self,
            _request: &AdaptationRequest,
            candidate: &RoutingCandidateId,
        ) -> ResilienceResult<bool> {
            Ok(candidate.as_str() == "test-routing-candidate")
        }

        fn deterministic(&self) -> bool {
            true
        }
    }

    fn reroute_request() -> AdaptationRequest {
        let action = RecoveryAction::new(
            ActionKind::Reroute,
            ActionPayload::Reroute {
                scope: ActionScope::Execution,
            },
        );

        AdaptationRequest::new(
            action,
            ExecutionGeneration::new("generation-1")
                .expect("test generation must be valid"),
            SemanticRevision::new("semantic-1")
                .expect("test semantic revision must be valid"),
        )
    }

    #[test]
    fn adapter_supports_only_reroute() {
        let adapter =
            ReroutingAdapter::from_router(TestRoutingService)
                .expect("adapter construction must succeed");

        assert_eq!(
            adapter.supported_actions(),
            &[ActionKind::Reroute]
        );
    }

    #[test]
    fn preflight_delegates_to_routing_service() {
        let adapter =
            ReroutingAdapter::from_router(TestRoutingService)
                .expect("adapter construction must succeed");

        let request = reroute_request();

        adapter
            .preflight(&request)
            .expect("preflight must succeed");
    }

    #[test]
    fn prepare_is_not_commit() {
        let adapter =
            ReroutingAdapter::from_router(TestRoutingService)
                .expect("adapter construction must succeed");

        let request = reroute_request();

        let result = adapter
            .prepare(&request)
            .expect("prepare must succeed");

        assert_eq!(
            result.status(),
            AdaptationStatus::Prepared
        );

        assert!(result.candidate().is_some());
        assert!(!result.status().changed_state());
    }

    #[test]
    fn commit_requires_matching_generation() {
        let adapter =
            ReroutingAdapter::from_router(TestRoutingService)
                .expect("adapter construction must succeed");

        let request = reroute_request();

        let candidate = adapter
            .prepare(&request)
            .expect("prepare must succeed")
            .candidate()
            .expect("candidate must exist")
            .clone();

        let stale_request = AdaptationRequest::new(
            RecoveryAction::new(
                ActionKind::Reroute,
                ActionPayload::Reroute {
                    scope: ActionScope::Execution,
                },
            ),
            ExecutionGeneration::new("generation-2")
                .expect("test generation must be valid"),
            SemanticRevision::new("semantic-1")
                .expect("test semantic revision must be valid"),
        );

        let result = adapter.commit(
            &stale_request,
            candidate,
        );

        assert_eq!(
            result.expect_err(
                "stale candidate must be rejected"
            ),
            ResilienceError::new(
                ResilienceErrorCode::PlanStale,
            )
        );
    }

    #[test]
    fn commit_requires_matching_semantic_revision() {
        let adapter =
            ReroutingAdapter::from_router(TestRoutingService)
                .expect("adapter construction must succeed");

        let request = reroute_request();

        let candidate = adapter
            .prepare(&request)
            .expect("prepare must succeed")
            .candidate()
            .expect("candidate must exist")
            .clone();

        let changed_request = AdaptationRequest::new(
            RecoveryAction::new(
                ActionKind::Reroute,
                ActionPayload::Reroute {
                    scope: ActionScope::Execution,
                },
            ),
            ExecutionGeneration::new("generation-1")
                .expect("test generation must be valid"),
            SemanticRevision::new("semantic-2")
                .expect("test semantic revision must be valid"),
        );

        let result = adapter.commit(
            &changed_request,
            candidate,
        );

        assert_eq!(
            result.expect_err(
                "semantic revision mismatch must be rejected"
            ),
            ResilienceError::new(
                ResilienceErrorCode::SemanticAdaptationViolation,
            )
        );
    }

    #[test]
    fn commit_returns_pending_verification() {
        let adapter =
            ReroutingAdapter::from_router(TestRoutingService)
                .expect("adapter construction must succeed");

        let request = reroute_request();

        let candidate = adapter
            .prepare(&request)
            .expect("prepare must succeed")
            .candidate()
            .expect("candidate must exist")
            .clone();

        let result = adapter
            .commit(&request, candidate)
            .expect("commit must succeed");

        assert_eq!(
            result.status(),
            AdaptationStatus::CommittedPendingVerification
        );

        assert!(result.verification_required());
        assert!(result.realization_changed());
    }

    #[test]
    fn verify_completes_successfully() {
        let adapter =
            ReroutingAdapter::from_router(TestRoutingService)
                .expect("adapter construction must succeed");

        let request = reroute_request();

        let candidate = adapter
            .prepare(&request)
            .expect("prepare must succeed")
            .candidate()
            .expect("candidate must exist")
            .clone();

        adapter
            .commit(&request, candidate.clone())
            .expect("commit must succeed");

        let result = adapter
            .verify(&request, candidate)
            .expect("verification must succeed");

        assert_eq!(
            result.status(),
            AdaptationStatus::Committed
        );
    }

    #[test]
    fn foreign_candidate_is_rejected() {
        let adapter =
            ReroutingAdapter::from_router(TestRoutingService)
                .expect("adapter construction must succeed");

        let request = reroute_request();

        let foreign_adapter =
            AdapterId::new("another.adapter")
                .expect("test adapter id must be valid");

        let candidate = AdaptationCandidate::new(
            foreign_adapter,
            ActionKind::Reroute,
            request.generation().clone(),
            request.semantic_revision().clone(),
            "foreign-candidate",
        )
        .expect("candidate must be constructible");

        let result = adapter.commit(
            &request,
            candidate,
        );

        assert_eq!(
            result.expect_err(
                "foreign candidate must be rejected"
            ),
            ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            )
        );
    }

    #[test]
    fn deterministic_requirement_is_enforced() {
        #[derive(Debug)]
        struct NonDeterministicRouter;

        impl ReroutingService for NonDeterministicRouter {
            fn preflight(
                &self,
                _request: &AdaptationRequest,
            ) -> ResilienceResult<()> {
                Ok(())
            }

            fn prepare(
                &self,
                _request: &AdaptationRequest,
            ) -> ResilienceResult<RoutingCandidateId> {
                RoutingCandidateId::new("candidate")
            }

            fn commit(
                &self,
                _request: &AdaptationRequest,
                _candidate: &RoutingCandidateId,
            ) -> ResilienceResult<bool> {
                Ok(true)
            }

            fn verify(
                &self,
                _request: &AdaptationRequest,
                _candidate: &RoutingCandidateId,
            ) -> ResilienceResult<bool> {
                Ok(true)
            }

            fn deterministic(&self) -> bool {
                false
            }
        }

        let adapter =
            ReroutingAdapter::from_router(NonDeterministicRouter)
                .expect("adapter construction must succeed");

        let request = reroute_request();

        assert_eq!(
            adapter
                .preflight(&request)
                .expect_err(
                    "deterministic request must reject nondeterministic router"
                ),
            ResilienceError::new(
                ResilienceErrorCode::CompatibilityFailure,
            )
        );
    }
}