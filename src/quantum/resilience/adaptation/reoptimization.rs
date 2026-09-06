//! Zamani Quantum Resilience — Reoptimization Adaptation
//!
//! Path:
//!     src/quantum/resilience/adaptation/reoptimization.rs
//!
//! Purpose:
//!     Provide the production resilience adapter for target-aware quantum
//!     optimization after execution conditions, hardware capabilities,
//!     topology, calibration, QEC requirements, resource availability, or
//!     execution constraints change.
//!
//! ============================================================================
//! Architectural responsibility
//! ============================================================================
//!
//! This module translates the canonical resilience action:
//!
//!     ActionKind::Reoptimize
//!
//! into a request to the authoritative quantum optimization subsystem.
//!
//! Resilience answers:
//!
//!     "WHEN should optimization be reconsidered?"
//!
//! Optimization answers:
//!
//!     "HOW should the canonical quantum program be transformed?"
//!
//! This module therefore MUST NOT implement optimization algorithms.
//!
//! It provides:
//!
//! - the `AdaptationAdapter` implementation;
//! - the canonical `Reoptimize` action boundary;
//! - transactional prepare/commit/verify semantics;
//! - stale-generation protection;
//! - stale-semantic-revision protection;
//! - deterministic-mode enforcement;
//! - capability metadata;
//! - an opaque optimization-candidate identity;
//! - an injected optimization-service boundary;
//! - adapter-local validation;
//! - adapter-local verification delegation;
//! - provider independence;
//! - arbitrary execution scope;
//! - support for local, regional and global optimization;
//! - support for target-aware and fault-tolerant optimization;
//! - no fixed machine-size assumptions;
//! - no fixed qubit limits;
//! - no fixed retry counts;
//! - no provider-specific logic;
//! - no unsafe Rust.
//!
//! ============================================================================
//! What this module does NOT own
//! ============================================================================
//!
//! This module MUST NOT:
//!
//! - implement gate cancellation;
//! - implement peephole optimization;
//! - implement algebraic rewriting;
//! - implement e-graphs;
//! - implement synthesis;
//! - implement Clifford+T optimization;
//! - implement T-count reduction;
//! - implement T-depth reduction;
//! - implement magic-state optimization;
//! - implement parameter optimization;
//! - implement stochastic optimization;
//! - implement target selection;
//! - implement optimization-pass scheduling;
//! - implement routing;
//! - implement hardware discovery;
//! - implement scheduling;
//! - implement QEC;
//! - communicate directly with a quantum provider;
//! - authorize resilience actions;
//! - decide policy;
//! - perform final semantic verification;
//! - contain retry loops;
//! - contain machine-size constants;
//! - contain provider-specific branches;
//! - use `unsafe`.
//!
//! All actual optimization remains owned by:
//!
//!     crate::quantum::optimization
//!
//! Hardware capability information remains owned by:
//!
//!     crate::quantum::hardware
//!
//! Canonical quantum semantics remain owned by:
//!
//!     crate::quantum::ir
//!
//! Canonical fault semantics remain owned by the repository's ZQN/fault
//! subsystem.
//!
//! ============================================================================
//! Write once, scale everywhere
//! ============================================================================
//!
//! Reoptimization MUST NOT assume:
//!
//! - a fixed number of qubits;
//! - a fixed number of operations;
//! - a fixed circuit depth;
//! - a fixed topology;
//! - a fixed native gate set;
//! - a fixed backend;
//! - a fixed QEC code;
//! - a fixed optimization pass count;
//! - a fixed optimization iteration count.
//!
//! The same logical Zamani program may therefore be reoptimized for:
//!
//! - one physical qubit;
//! - a small QPU;
//! - a large QPU;
//! - a fault-tolerant logical machine;
//! - a simulator;
//! - an emulator;
//! - a heterogeneous quantum system;
//! - a distributed quantum execution environment.
//!
//! "Infinity" means that this module imposes no artificial finite machine-size
//! ceiling. Actual execution remains bounded by the resources, capabilities,
//! limits and policies supplied by the authoritative subsystems.
//!
//! ============================================================================
//! Incremental optimization
//! ============================================================================
//!
//! The resilience scalability contract requires reoptimization to support:
//!
//! - affected-region optimization;
//! - global optimization;
//! - fault-tolerant optimization;
//! - target-aware optimization.
//!
//! This module represents the requested scope through the canonical
//! `AdaptationRequest` and `ActionKind::Reoptimize` action.
//!
//! It does NOT define a second scope model.
//!
//! The optimization service interprets the canonical action scope and decides
//! how that scope maps to optimization regions.
//!
//! Examples:
//!
//!     local fault
//!         -> optimize affected region
//!
//!     target capability change
//!         -> optimize affected implementation
//!
//!     major hardware migration
//!         -> global target-aware optimization
//!
//!     changed logical/QEC requirements
//!         -> fault-tolerant reoptimization
//!
//! The choice is owned by the optimization subsystem and resilience policy.
//!
//! ============================================================================
//! Canonical contracts
//! ============================================================================
//!
//! Adaptation:
//!
//!     crate::quantum::resilience::adaptation::adapter
//!
//! Actions:
//!
//!     crate::quantum::resilience::planning::action
//!
//! Errors:
//!
//!     crate::quantum::resilience::errors
//!
//! Optimization:
//!
//!     crate::quantum::optimization
//!
//! Canonical quantum IR:
//!
//!     crate::quantum::ir
//!
//! Canonical qubit identity, when an optimization implementation actually
//! requires a qubit identity:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! This adapter does not require a qubit identifier itself because the
//! optimization scope is already represented by the canonical resilience
//! action/context. Concrete optimization implementations must use
//! `quantum::ir::qubit`, never a resilience-local QubitId.
//!
//! ============================================================================
//! Transactional semantics
//! ============================================================================
//!
//! Reoptimization follows:
//!
//!     Preflight
//!         |
//!         v
//!     Prepare
//!         |
//!         v
//!     Optimization candidate
//!         |
//!         v
//!     Commit
//!         |
//!         v
//!     Verify
//!
//! Preparation MUST NOT mutate the committed execution artifact.
//!
//! Commit MUST NOT accept a candidate prepared against stale:
//!
//! - execution generation;
//! - semantic revision;
//! - target state;
//! - optimization-relevant capability state;
//! - optimization context.
//!
//! The injected optimization service owns validation of target-specific state.
//!
//! ============================================================================
//! Determinism
//! ============================================================================
//!
//! When deterministic behavior is requested:
//!
//! - the adapter does not choose random optimization passes;
//! - the adapter does not create random candidate identities;
//! - the adapter does not access global mutable state;
//! - the injected optimization service must guarantee deterministic behavior;
//! - stochastic optimization may only be used when explicitly permitted by
//!   the supplied optimization configuration and resilience policy.
//!
//! Equal explicit request state must produce equivalent adapter behavior.
//!
//! ============================================================================
//! Safety
//! ============================================================================
//!
//! This adapter is NOT an authorization boundary.
//!
//! Before invocation, the surrounding resilience system must establish:
//!
//! - policy validity;
//! - capability validity;
//! - feasibility;
//! - security authorization;
//! - semantic compatibility;
//! - resource availability;
//! - execution preconditions.
//!
//! Reoptimization MUST never be accepted merely because it produced a cheaper
//! circuit. The resulting transformation still requires final semantic
//! verification by the resilience verification subsystem.
//!
//! ============================================================================
//! Integration
//! ============================================================================
//!
//! `planning/action.rs`
//!     |
//!     | ActionKind::Reoptimize
//!     v
//! `adaptation/adapter.rs`
//!     |
//!     v
//! `ReoptimizationAdapter`
//!     |
//!     v
//! `ReoptimizationService`
//!     |
//!     v
//! `quantum::optimization`
//!     |
//!     +--> canonical quantum::ir
//!     +--> optimization analyses
//!     +--> optimization targets
//!     +--> optimization planner
//!     +--> optimization pipeline
//!     +--> fault-tolerant optimization
//!     +--> optimization verification
//!
//! Hardware information reaches optimization through the repository's target
//! and capability contracts. This file does not directly access hardware.
//!
//! Routing remains owned by `quantum::routing`.
//!
//! Scheduling remains owned by `quantum::scheduling`.
//!
//! QEC remains owned by the QEC subsystem.
//!
//! ============================================================================
//! Rust compatibility
//! ============================================================================
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no unsafe code
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

/// Stable semantic identifier for the resilience reoptimization adapter.
pub const REOPTIMIZATION_ADAPTER_ID: &str =
    "zamani.quantum.resilience.reoptimization";

/// Semantic implementation version of the adapter.
pub const REOPTIMIZATION_ADAPTER_VERSION: AdapterVersion =
    AdapterVersion::new(1, 0, 0);

/// Stable schema identifier for this adapter.
pub const REOPTIMIZATION_ADAPTER_SCHEMA_ID: &str =
    "zamani.quantum.resilience.adaptation.reoptimization";

/// Schema version for this adapter contract.
pub const REOPTIMIZATION_ADAPTER_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Supported action set
// ============================================================================

/// Actions accepted by this adapter.
///
/// The adapter intentionally handles exactly one canonical action.
static SUPPORTED_ACTIONS: [ActionKind; 1] = [ActionKind::Reoptimize];

// ============================================================================
// Optimization candidate identity
// ============================================================================

/// Opaque identity of a prepared optimization candidate.
///
/// The resilience layer does not interpret the representation.
///
/// The optimization subsystem may internally represent a candidate as:
///
/// - an optimized IR artifact;
/// - an optimization pipeline result;
/// - a transformation graph;
/// - an immutable compiler artifact;
/// - a target-specific compilation artifact;
/// - a fault-tolerant logical artifact.
///
/// None of those implementation details belong in resilience.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OptimizationCandidateId(String);

impl OptimizationCandidateId {
    /// Creates a validated optimization-candidate identity.
    pub fn new(value: impl Into<String>) -> ResilienceResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidIdentifier,
            ));
        }

        Ok(Self(value))
    }

    /// Returns the opaque candidate identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the candidate identity.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for OptimizationCandidateId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// Optimization-service capabilities
// ============================================================================

/// Capabilities of an injected optimization service.
///
/// These are capabilities of the service boundary, not hardware capabilities.
///
/// Hardware capabilities remain owned by `quantum::hardware` and are consumed
/// by the concrete optimization implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReoptimizationCapabilities {
    /// Service supports non-mutating preflight.
    preflight: bool,

    /// Service can construct an uncommitted candidate.
    prepare: bool,

    /// Service can atomically commit a prepared candidate.
    commit: bool,

    /// Service guarantees deterministic behavior for equal explicit inputs.
    deterministic: bool,

    /// Service understands scoped optimization.
    scoped: bool,

    /// Service can preserve valid unaffected regions when requested and safe.
    partial: bool,

    /// Service can reverse an optimization transformation itself.
    ///
    /// Normal rollback belongs to the recovery subsystem, so the default is
    /// false.
    reversible: bool,
}

impl ReoptimizationCapabilities {
    /// Creates service capabilities.
    #[must_use]
    pub const fn new(
        preflight: bool,
        prepare: bool,
        commit: bool,
        deterministic: bool,
        scoped: bool,
        partial: bool,
        reversible: bool,
    ) -> Self {
        Self {
            preflight,
            prepare,
            commit,
            deterministic,
            scoped,
            partial,
            reversible,
        }
    }

    /// Whether preflight is supported.
    #[must_use]
    pub const fn supports_preflight(self) -> bool {
        self.preflight
    }

    /// Whether preparation is supported.
    #[must_use]
    pub const fn supports_prepare(self) -> bool {
        self.prepare
    }

    /// Whether commit is supported.
    #[must_use]
    pub const fn supports_commit(self) -> bool {
        self.commit
    }

    /// Whether deterministic execution is guaranteed.
    #[must_use]
    pub const fn deterministic(self) -> bool {
        self.deterministic
    }

    /// Whether scoped optimization is supported.
    #[must_use]
    pub const fn scoped(self) -> bool {
        self.scoped
    }

    /// Whether partial preservation is supported.
    #[must_use]
    pub const fn partial(self) -> bool {
        self.partial
    }

    /// Whether the service itself can reverse transformations.
    #[must_use]
    pub const fn reversible(self) -> bool {
        self.reversible
    }
}

impl Default for ReoptimizationCapabilities {
    fn default() -> Self {
        Self::new(
            true,
            true,
            true,
            true,
            true,
            true,
            false,
        )
    }
}

// ============================================================================
// Optimization service contract
// ============================================================================

/// Authoritative optimization boundary used by resilience.
///
/// A concrete implementation should live at the integration boundary to
/// `crate::quantum::optimization` and delegate to its canonical optimizer,
/// planner and pipeline.
///
/// This trait intentionally does not expose optimization-specific concrete
/// types. That prevents resilience from becoming coupled to one optimizer
/// implementation.
///
/// # Required implementation behavior
///
/// Implementations MUST:
///
/// - consume the immutable `AdaptationRequest`;
/// - interpret the canonical `ActionKind::Reoptimize`;
/// - honor the action scope;
/// - preserve canonical quantum semantics;
/// - use canonical `quantum::ir` representations;
/// - validate current optimization target/capability state;
/// - reject stale optimization state;
/// - reject unavailable resources;
/// - honor deterministic requirements;
/// - respect configured optimization/resource limits;
/// - avoid partial committed transformations;
/// - provide an immutable candidate identity;
/// - verify candidate validity before reporting success.
///
/// Implementations MUST NOT:
///
/// - silently change the logical computation;
/// - invent physical resources;
/// - assume a fixed number of qubits;
/// - assume a fixed circuit size;
/// - assume a fixed optimization pass list;
/// - assume a fixed iteration count;
/// - select a provider by name inside this resilience boundary.
///
/// # Canonical optimization ownership
///
/// The concrete service should delegate to the existing optimization
/// subsystem, which already owns:
///
/// - optimization passes;
/// - optimization pipeline;
/// - target-aware optimization;
/// - analyses;
/// - synthesis;
/// - fault-tolerant optimization;
/// - verification;
/// - optimization resource limits.
///
/// This adapter therefore remains stable when new optimization algorithms are
/// introduced.
pub trait ReoptimizationService: Send + Sync + fmt::Debug {
    /// Returns service capabilities.
    fn capabilities(&self) -> ReoptimizationCapabilities;

    /// Validates whether reoptimization can begin without mutating state.
    fn preflight(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<()>;

    /// Builds an immutable optimization candidate without committing it.
    fn prepare(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<OptimizationCandidateId>;

    /// Commits a previously prepared candidate.
    ///
    /// Returns `true` only when the candidate was actually committed.
    ///
    /// The implementation MUST reject candidates whose target, semantic
    /// revision, execution generation, capability snapshot, or other
    /// optimization-relevant state has become stale.
    fn commit(
        &self,
        request: &AdaptationRequest,
        candidate: &OptimizationCandidateId,
    ) -> ResilienceResult<bool>;

    /// Performs adapter-local optimization verification.
    ///
    /// This does not replace final resilience semantic verification.
    fn verify(
        &self,
        request: &AdaptationRequest,
        candidate: &OptimizationCandidateId,
    ) -> ResilienceResult<bool>;
}

// ============================================================================
// Adapter
// ============================================================================

/// Production resilience adapter for quantum reoptimization.
///
/// `O` is injected so resilience does not become coupled to one concrete
/// optimization implementation.
#[derive(Clone)]
pub struct ReoptimizationAdapter<O>
where
    O: ReoptimizationService,
{
    id: AdapterId,
    optimizer: Arc<O>,
}

impl<O> fmt::Debug for ReoptimizationAdapter<O>
where
    O: ReoptimizationService,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ReoptimizationAdapter")
            .field("id", &self.id)
            .field("optimizer", &self.optimizer)
            .finish()
    }
}

impl<O> ReoptimizationAdapter<O>
where
    O: ReoptimizationService,
{
    /// Creates a reoptimization adapter around an authoritative optimizer.
    pub fn new(optimizer: Arc<O>) -> ResilienceResult<Self> {
        let id = AdapterId::new(REOPTIMIZATION_ADAPTER_ID)?;

        Ok(Self { id, optimizer })
    }

    /// Creates an adapter from an owned optimization service.
    pub fn from_optimizer(optimizer: O) -> ResilienceResult<Self> {
        Self::new(Arc::new(optimizer))
    }

    /// Returns the injected optimization service.
    #[must_use]
    pub fn optimizer(&self) -> &O {
        self.optimizer.as_ref()
    }

    /// Returns the shared optimization service handle.
    #[must_use]
    pub fn optimizer_handle(&self) -> &Arc<O> {
        &self.optimizer
    }

    /// Returns the optimization service capabilities.
    #[must_use]
    pub fn optimization_capabilities(&self) -> ReoptimizationCapabilities {
        self.optimizer.capabilities()
    }

    /// Returns whether the underlying optimization service is deterministic.
    #[must_use]
    pub fn optimization_is_deterministic(&self) -> bool {
        self.optimizer.capabilities().deterministic()
    }

    /// Validates that a request is specifically a reoptimization request.
    fn validate_reoptimization_request(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<()> {
        request.validate()?;

        if request.action_kind() != ActionKind::Reoptimize {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        let capabilities = self.optimizer.capabilities();

        if request.deterministic_required()
            && !capabilities.deterministic()
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::CompatibilityFailure,
            ));
        }

        if !capabilities.supports_preflight()
            || !capabilities.supports_prepare()
            || !capabilities.supports_commit()
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::ReoptimizationFailed,
            ));
        }

        Ok(())
    }

    /// Validates an opaque resilience candidate and converts its identity into
    /// the optimization-service candidate type.
    fn validate_candidate(
        &self,
        request: &AdaptationRequest,
        candidate: &AdaptationCandidate,
    ) -> ResilienceResult<OptimizationCandidateId> {
        if candidate.adapter() != &self.id {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        if candidate.action() != ActionKind::Reoptimize {
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

        OptimizationCandidateId::new(candidate.identity().to_owned())
    }

    /// Converts an optimization-service candidate into the canonical
    /// resilience candidate.
    fn make_candidate(
        &self,
        request: &AdaptationRequest,
        optimization_candidate: OptimizationCandidateId,
    ) -> ResilienceResult<AdaptationCandidate> {
        AdaptationCandidate::new(
            self.id.clone(),
            ActionKind::Reoptimize,
            request.generation().clone(),
            request.semantic_revision().clone(),
            optimization_candidate.into_string(),
        )
    }

    /// Prepares an optimization candidate.
    fn prepare_reoptimization(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<AdaptationResult> {
        self.validate_reoptimization_request(request)?;

        self.optimizer.preflight(request)?;

        let candidate_id = self.optimizer.prepare(request)?;

        let candidate = self.make_candidate(request, candidate_id)?;

        Ok(
            AdaptationResult::new(
                self.id.clone(),
                ActionKind::Reoptimize,
                AdaptationStatus::Prepared,
                request.generation().clone(),
                request.semantic_revision().clone(),
            )
            .with_candidate(candidate),
        )
    }

    /// Commits a previously prepared optimization candidate.
    fn commit_reoptimization(
        &self,
        request: &AdaptationRequest,
        candidate: &AdaptationCandidate,
    ) -> ResilienceResult<AdaptationResult> {
        self.validate_reoptimization_request(request)?;

        let optimization_candidate =
            self.validate_candidate(request, candidate)?;

        let committed =
            self.optimizer.commit(request, &optimization_candidate)?;

        if !committed {
            return Ok(
                AdaptationResult::new(
                    self.id.clone(),
                    ActionKind::Reoptimize,
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
                ActionKind::Reoptimize,
                AdaptationStatus::CommittedPendingVerification,
                request.generation().clone(),
                request.semantic_revision().clone(),
            )
            .with_candidate(candidate.clone()),
        )
    }

    /// Performs optimization-service-local verification.
    fn verify_reoptimization(
        &self,
        request: &AdaptationRequest,
        candidate: &AdaptationCandidate,
    ) -> ResilienceResult<AdaptationResult> {
        self.validate_reoptimization_request(request)?;

        let optimization_candidate =
            self.validate_candidate(request, candidate)?;

        let verified =
            self.optimizer.verify(request, &optimization_candidate)?;

        if !verified {
            return Ok(
                AdaptationResult::new(
                    self.id.clone(),
                    ActionKind::Reoptimize,
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
                ActionKind::Reoptimize,
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

impl<O> AdaptationAdapter for ReoptimizationAdapter<O>
where
    O: ReoptimizationService,
{
    fn id(&self) -> &AdapterId {
        &self.id
    }

    fn version(&self) -> AdapterVersion {
        REOPTIMIZATION_ADAPTER_VERSION
    }

    fn capabilities(&self) -> AdaptationCapabilities {
        let capabilities = self.optimizer.capabilities();

        AdaptationCapabilities::new(
            capabilities.supports_prepare(),
            capabilities.supports_commit(),
            capabilities.supports_preflight(),
            capabilities.deterministic(),
            capabilities.scoped(),
            capabilities.partial(),
            // Rollback remains owned by recovery/rollback.rs.
            false,
        )
    }

    fn supported_actions(&self) -> &[ActionKind] {
        &SUPPORTED_ACTIONS
    }

    fn preflight(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<()> {
        self.validate_reoptimization_request(request)?;

        self.optimizer.preflight(request)
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
                        ActionKind::Reoptimize,
                        AdaptationStatus::Prepared,
                        request.generation().clone(),
                        request.semantic_revision().clone(),
                    ),
                )
            }

            AdaptationOperation::Prepare => {
                self.prepare_reoptimization(request)
            }

            AdaptationOperation::Commit { candidate } => {
                self.commit_reoptimization(request, candidate)
            }

            AdaptationOperation::Verify { candidate } => {
                self.verify_reoptimization(request, candidate)
            }
        }
    }
}

// ============================================================================
// Shared adapter handle
// ============================================================================

/// Thread-safe shared reoptimization adapter handle.
pub type ReoptimizationAdapterHandle<O> =
    Arc<ReoptimizationAdapter<O>>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::resilience::adaptation::adapter::{
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

    /// Minimal deterministic optimization service used exclusively to test
    /// the resilience adapter contract.
    ///
    /// It deliberately contains no machine-size assumptions.
    #[derive(Debug, Default)]
    struct TestOptimizationService;

    impl ReoptimizationService for TestOptimizationService {
        fn capabilities(&self) -> ReoptimizationCapabilities {
            ReoptimizationCapabilities::new(
                true,
                true,
                true,
                true,
                true,
                true,
                false,
            )
        }

        fn preflight(
            &self,
            request: &AdaptationRequest,
        ) -> ResilienceResult<()> {
            if request.action_kind() != ActionKind::Reoptimize {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::InvalidArgument,
                ));
            }

            Ok(())
        }

        fn prepare(
            &self,
            _request: &AdaptationRequest,
        ) -> ResilienceResult<OptimizationCandidateId> {
            OptimizationCandidateId::new(
                "test-optimization-candidate",
            )
        }

        fn commit(
            &self,
            _request: &AdaptationRequest,
            candidate: &OptimizationCandidateId,
        ) -> ResilienceResult<bool> {
            Ok(
                candidate.as_str()
                    == "test-optimization-candidate",
            )
        }

        fn verify(
            &self,
            _request: &AdaptationRequest,
            candidate: &OptimizationCandidateId,
        ) -> ResilienceResult<bool> {
            Ok(
                candidate.as_str()
                    == "test-optimization-candidate",
            )
        }
    }

    fn reoptimization_request() -> AdaptationRequest {
        let action = RecoveryAction::new(
            ActionKind::Reoptimize,
            ActionPayload::Reoptimize {
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
    fn adapter_supports_only_reoptimization() {
        let adapter =
            ReoptimizationAdapter::from_optimizer(
                TestOptimizationService,
            )
            .expect("adapter construction must succeed");

        assert_eq!(
            adapter.supported_actions(),
            &[ActionKind::Reoptimize]
        );
    }

    #[test]
    fn preflight_delegates_to_optimization_service() {
        let adapter =
            ReoptimizationAdapter::from_optimizer(
                TestOptimizationService,
            )
            .expect("adapter construction must succeed");

        let request = reoptimization_request();

        adapter
            .preflight(&request)
            .expect("preflight must succeed");
    }

    #[test]
    fn prepare_does_not_claim_commit() {
        let adapter =
            ReoptimizationAdapter::from_optimizer(
                TestOptimizationService,
            )
            .expect("adapter construction must succeed");

        let request = reoptimization_request();

        let result = adapter
            .execute(
                &AdaptationOperation::Prepare,
                &request,
            )
            .expect("prepare must succeed");

        assert_eq!(
            result.status(),
            AdaptationStatus::Prepared
        );

        assert!(result.candidate().is_some());
    }

    #[test]
    fn commit_requires_matching_candidate() {
        let adapter =
            ReoptimizationAdapter::from_optimizer(
                TestOptimizationService,
            )
            .expect("adapter construction must succeed");

        let request = reoptimization_request();

        let prepared = adapter
            .execute(
                &AdaptationOperation::Prepare,
                &request,
            )
            .expect("prepare must succeed");

        let candidate = prepared
            .candidate()
            .cloned()
            .expect("prepare must return a candidate");

        let committed = adapter
            .execute(
                &AdaptationOperation::Commit {
                    candidate,
                },
                &request,
            )
            .expect("commit must succeed");

        assert_eq!(
            committed.status(),
            AdaptationStatus::CommittedPendingVerification
        );
    }

    #[test]
    fn verification_completes_transaction() {
        let adapter =
            ReoptimizationAdapter::from_optimizer(
                TestOptimizationService,
            )
            .expect("adapter construction must succeed");

        let request = reoptimization_request();

        let prepared = adapter
            .execute(
                &AdaptationOperation::Prepare,
                &request,
            )
            .expect("prepare must succeed");

        let candidate = prepared
            .candidate()
            .cloned()
            .expect("prepare must return a candidate");

        let committed = adapter
            .execute(
                &AdaptationOperation::Commit {
                    candidate: candidate.clone(),
                },
                &request,
            )
            .expect("commit must succeed");

        assert_eq!(
            committed.status(),
            AdaptationStatus::CommittedPendingVerification
        );

        let verified = adapter
            .execute(
                &AdaptationOperation::Verify {
                    candidate,
                },
                &request,
            )
            .expect("verification must succeed");

        assert_eq!(
            verified.status(),
            AdaptationStatus::Committed
        );
    }

    #[test]
    fn stale_generation_is_rejected() {
        let adapter =
            ReoptimizationAdapter::from_optimizer(
                TestOptimizationService,
            )
            .expect("adapter construction must succeed");

        let original = reoptimization_request();

        let prepared = adapter
            .execute(
                &AdaptationOperation::Prepare,
                &original,
            )
            .expect("prepare must succeed");

        let candidate = prepared
            .candidate()
            .cloned()
            .expect("prepare must return candidate");

        let stale_action = RecoveryAction::new(
            ActionKind::Reoptimize,
            ActionPayload::Reoptimize {
                scope: ActionScope::Execution,
            },
        );

        let stale_request = AdaptationRequest::new(
            stale_action,
            ExecutionGeneration::new("generation-2")
                .expect("test generation must be valid"),
            SemanticRevision::new("semantic-1")
                .expect("test semantic revision must be valid"),
        );

        let result = adapter.execute(
            &AdaptationOperation::Commit { candidate },
            &stale_request,
        );

        assert!(result.is_err());

        assert_eq!(
            result
                .expect_err("stale generation must fail")
                .code(),
            ResilienceErrorCode::PlanStale
        );
    }

    #[test]
    fn stale_semantic_revision_is_rejected() {
        let adapter =
            ReoptimizationAdapter::from_optimizer(
                TestOptimizationService,
            )
            .expect("adapter construction must succeed");

        let original = reoptimization_request();

        let prepared = adapter
            .execute(
                &AdaptationOperation::Prepare,
                &original,
            )
            .expect("prepare must succeed");

        let candidate = prepared
            .candidate()
            .cloned()
            .expect("prepare must return candidate");

        let stale_action = RecoveryAction::new(
            ActionKind::Reoptimize,
            ActionPayload::Reoptimize {
                scope: ActionScope::Execution,
            },
        );

        let stale_request = AdaptationRequest::new(
            stale_action,
            ExecutionGeneration::new("generation-1")
                .expect("test generation must be valid"),
            SemanticRevision::new("semantic-2")
                .expect("test semantic revision must be valid"),
        );

        let result = adapter.execute(
            &AdaptationOperation::Commit { candidate },
            &stale_request,
        );

        assert!(result.is_err());

        assert_eq!(
            result
                .expect_err("stale semantic revision must fail")
                .code(),
            ResilienceErrorCode::SemanticAdaptationViolation
        );
    }
}