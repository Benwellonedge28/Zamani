//! Zamani Quantum Resilience — Backend Selection Adaptation
//!
//! Path:
//!     src/quantum/resilience/adaptation/backend_selection.rs
//!
//! Purpose:
//!     Provides the resilience-to-execution-target selection boundary used
//!     when the current quantum execution environment is unavailable,
//!     degraded, incompatible, or otherwise no longer satisfies the
//!     computation's declared requirements.
//!
//! ============================================================================
//! Architectural responsibility
//! ============================================================================
//!
//! Backend selection answers:
//!
//!     "Which already-discovered execution environment is valid for this
//!      computation under the current policy and capability state?"
//!
//! It does NOT implement provider discovery, hardware capability modelling,
//! routing, scheduling, compilation, optimization, QEC, execution, or policy.
//!
//! The dependency direction is:
//!
//! ```text
//! canonical Zamani program / execution state
//!                 |
//!                 v
//!          resilience planner
//!                 |
//!             Migrate action
//!                 |
//!                 v
//!       BackendSelectionAdapter
//!                 |
//!                 v
//!       BackendSelectionService
//!                 |
//!        +--------+---------+
//!        |                  |
//!        v                  v
//! hardware registry     compatibility
//!        |                  |
//!        +--------+---------+
//!                 |
//!                 v
//!          selected target
//!                 |
//!        +--------+---------+
//!        |                  |
//!        v                  v
//!     recompile          remap/reroute
//!        |                  |
//!        +--------+---------+
//!                 v
//!             reschedule
//!                 |
//!                 v
//!             execution
//! ```
//!
//! The concrete service is the integration point for the authoritative
//! `crate::quantum::hardware` registry/compatibility contracts. Keeping that
//! integration behind a trait prevents resilience from becoming coupled to a
//! provider, registry implementation, or hardware technology.
//!
//! ============================================================================
//! Canonical action ownership
//! ============================================================================
//!
//! The canonical action for changing execution environment is:
//!
//!     crate::quantum::resilience::planning::action::ActionKind::Migrate
//!
//! with:
//!
//!     ActionPayload::Migrate { scope, target }
//!
//! Backend selection is the target-validation/selection mechanism behind that
//! action. Migration/execution of state remains owned by the recovery/runtime
//! layers.
//!
//! This module MUST NOT introduce another migration or backend-selection action
//! enum.
//!
//! ============================================================================
//! Canonical hardware ownership
//! ============================================================================
//!
//! Hardware identity, capabilities, topology, calibration, status and device
//! registration remain owned by:
//!
//!     crate::quantum::hardware
//!
//! In particular, the authoritative registry is:
//!
//!     crate::quantum::hardware::device_registry
//!
//! and workload/backend compatibility is owned by:
//!
//!     crate::quantum::hardware::compatibility
//!
//! This file intentionally does not duplicate those structures.
//!
//! A concrete `BackendSelectionService` implementation should consume those
//! APIs and return an opaque candidate identity. It may also use
//! `crate::quantum::ir::qubit::QubitId` when translating canonical qubit
//! requirements, but this adapter itself does not need to materialize a qubit
//! list merely to select a target.
//!
//! ============================================================================
//! Write once, scale everywhere
//! ============================================================================
//!
//! This module introduces no artificial finite limit on:
//!
//! - qubits;
//! - logical qubits;
//! - physical qubits;
//! - operations;
//! - circuit depth;
//! - devices;
//! - providers;
//! - execution environments;
//! - topology size;
//! - distributed resources.
//!
//! Selection is capability-driven. Actual limits are supplied by the hardware
//! registry, compatibility subsystem, execution policy and available runtime
//! resources.
//!
//! "Infinity" means that this adapter imposes no finite machine-size ceiling;
//! every concrete selection is constrained only by authoritative resources and
//! policy.
//!
//! ============================================================================
//! Selection versus migration
//! ============================================================================
//!
//! These are deliberately separate concerns:
//!
//!     backend selection
//!         -> identify a compatible target
//!
//!     migration
//!         -> move/continue execution on that target
//!
//! The selected target must be compatible before migration is authorized.
//! Migration may additionally require:
//!
//! - checkpoint availability;
//! - a valid resume boundary;
//! - state-transfer support;
//! - compatible QEC state;
//! - compatible classical execution state;
//! - compatible compiled/routed/scheduled artifacts.
//!
//! Those conditions belong to recovery, execution, compiler, QEC and hardware
//! contracts respectively.
//!
//! ============================================================================
//! Selection versus optimization/routing/scheduling
//! ============================================================================
//!
//! Selecting a backend does not mean that the existing executable artifact is
//! valid on that backend.
//!
//! A target change can require a composed adaptation such as:
//!
//!     Migrate
//!       -> Recompile
//!       -> Remap/Reroute
//!       -> Reschedule
//!       -> Verify
//!
//! This adapter only establishes the target-selection boundary. It must not
//! silently perform the other adaptations.
//!
//! ============================================================================
//! Security invariant
//! ============================================================================
//!
//! A backend must never be selected merely because it is available.
//!
//! The concrete service must establish, through the authoritative hardware and
//! resilience contracts, all applicable requirements including:
//!
//! - capability compatibility;
//! - workload compatibility;
//! - resource availability;
//! - health/status suitability;
//! - calibration requirements;
//! - QEC compatibility;
//! - execution-model compatibility;
//! - policy constraints;
//! - security authorization;
//! - provenance requirements.
//!
//! This adapter is not an authorization boundary. It does not bypass policy or
//! security checks supplied by the surrounding resilience lifecycle.
//!
//! ============================================================================
//! Determinism
//! ============================================================================
//!
//! The adapter contains no global mutable state, wall-clock reads, randomness,
//! provider-specific ordering, or implicit target preference.
//!
//! When deterministic mode is required, the injected service must guarantee
//! deterministic candidate selection for identical explicit request state and
//! registry/capability snapshots.
//!
//! If a policy intentionally permits nondeterministic selection, the service is
//! responsible for recording the selected target and the decision provenance.
//!
//! ============================================================================
//! Transactional semantics
//! ============================================================================
//!
//! Selection follows the generic adaptation lifecycle:
//!
//!     Preflight
//!         |
//!         v
//!     Prepare
//!         |
//!         v
//!     selected-target candidate
//!         |
//!         v
//!     Commit
//!         |
//!         v
//!     Verify
//!
//! Preparation must not mutate the authoritative execution environment.
//!
//! Commit must reject a candidate if the selected target has become stale,
//! unavailable, unhealthy, incompatible, or unauthorized since preparation.
//!
//! Verification confirms that the selection actually became authoritative at
//! the service boundary. It does not replace semantic verification of the
//! quantum computation.
//!
//! ============================================================================
//! Candidate identity
//! ============================================================================
//!
//! `BackendCandidateId` is deliberately opaque to resilience.
//!
//! It may represent a backend ID, a device ID, an immutable registry snapshot
//! identity, or a stronger selection transaction identity. The service owns
//! its meaning.
//!
//! The target itself is already carried by the canonical `Migrate` action, so
//! the adapter does not need a second target field in `AdaptationCandidate`.
//!
//! ============================================================================
//! Canonical qubit identity
//! ============================================================================
//!
//! This file does not define a `QubitId`.
//!
//! If a concrete hardware/compatibility integration needs qubit identities it
//! MUST use:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! and never a resilience-local replacement.
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
//! Integration contract
//! ============================================================================
//!
//! `planning/action.rs`
//!     |
//!     | ActionKind::Migrate
//!     | ActionPayload::Migrate { scope, target }
//!     v
//! `adaptation/adapter.rs`
//!     |
//!     v
//! `BackendSelectionAdapter`
//!     |
//!     v
//! `BackendSelectionService`
//!     |
//!     +--> hardware::device_registry
//!     +--> hardware::compatibility
//!     +--> hardware::health/status
//!     +--> hardware::capabilities
//!     +--> execution/recovery compatibility
//!     |
//!     v
//! selected target candidate
//!     |
//!     +--> recompilation when required
//!     +--> remapping/rerouting when required
//!     +--> rescheduling when required
//!     +--> migration/recovery
//!     +--> verification
//!
//! No provider SDK type is permitted to cross this boundary.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]
#![deny(clippy::all)]

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

use crate::quantum::resilience::planning::action::{
    ActionKind,
    ActionPayload,
    ResourceId,
};

// ============================================================================
// Stable schema identity
// ============================================================================

/// Stable identifier for the backend-selection adaptation boundary.
pub const BACKEND_SELECTION_SCHEMA_ID: &str =
    "zamani.quantum.resilience.adaptation.backend_selection";

/// Semantic version of the backend-selection contract.
pub const BACKEND_SELECTION_SCHEMA_VERSION: u16 = 1;

/// Stable adapter identity.
pub const BACKEND_SELECTION_ADAPTER_ID: &str =
    "zamani.quantum.resilience.adaptation.backend_selection";

/// Semantic implementation version of the adapter.
pub const BACKEND_SELECTION_ADAPTER_VERSION: AdapterVersion =
    AdapterVersion::new(1, 0, 0);

// ============================================================================
// Supported action
// ============================================================================

/// Canonical action handled by backend selection.
static SUPPORTED_ACTIONS: [ActionKind; 1] = [ActionKind::Migrate];

// ============================================================================
// Candidate identity
// ============================================================================

/// Opaque identity for a prepared backend-selection candidate.
///
/// The identity must be generated by the authoritative selection service. The
/// resilience adapter does not manufacture target identities or infer them
/// from provider-specific information.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BackendCandidateId(String);

impl BackendCandidateId {
    /// Creates a validated opaque candidate identity.
    pub fn new(value: impl Into<String>) -> ResilienceResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidIdentifier,
            ));
        }

        Ok(Self(value))
    }

    /// Returns the opaque identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identity.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for BackendCandidateId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// Service capabilities
// ============================================================================

/// Capabilities of the injected backend-selection service.
///
/// These describe the service boundary, not the hardware. Hardware
/// capabilities remain authoritative in `quantum::hardware`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BackendSelectionCapabilities {
    preflight: bool,
    prepare: bool,
    commit: bool,
    deterministic: bool,
    scoped: bool,
    partial: bool,
}

impl BackendSelectionCapabilities {
    /// Creates service capabilities.
    #[must_use]
    pub const fn new(
        preflight: bool,
        prepare: bool,
        commit: bool,
        deterministic: bool,
        scoped: bool,
        partial: bool,
    ) -> Self {
        Self {
            preflight,
            prepare,
            commit,
            deterministic,
            scoped,
            partial,
        }
    }

    /// Whether non-mutating preflight is supported.
    #[must_use]
    pub const fn supports_preflight(self) -> bool {
        self.preflight
    }

    /// Whether candidate preparation is supported.
    #[must_use]
    pub const fn supports_prepare(self) -> bool {
        self.prepare
    }

    /// Whether candidate commit is supported.
    #[must_use]
    pub const fn supports_commit(self) -> bool {
        self.commit
    }

    /// Whether deterministic selection is guaranteed.
    #[must_use]
    pub const fn deterministic(self) -> bool {
        self.deterministic
    }

    /// Whether scoped target selection is supported.
    #[must_use]
    pub const fn scoped(self) -> bool {
        self.scoped
    }

    /// Whether the service can preserve unaffected execution scope.
    #[must_use]
    pub const fn partial(self) -> bool {
        self.partial
    }
}

impl Default for BackendSelectionCapabilities {
    fn default() -> Self {
        Self::new(true, true, true, true, true, true)
    }
}

// ============================================================================
// Target reference
// ============================================================================

/// Immutable provider-neutral target reference extracted from the canonical
/// migration action.
///
/// The target remains an opaque `ResourceId`; the hardware registry owns its
/// meaning. This prevents backend selection from depending on a provider's
/// naming scheme.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BackendTarget {
    resource_id: ResourceId,
}

impl BackendTarget {
    /// Creates a target reference from a canonical resource ID.
    #[must_use]
    pub fn new(resource_id: ResourceId) -> Self {
        Self { resource_id }
    }

    /// Returns the canonical target resource ID.
    #[must_use]
    pub fn resource_id(&self) -> &ResourceId {
        &self.resource_id
    }

    /// Returns the target identifier as an opaque string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.resource_id.as_str()
    }
}

impl fmt::Display for BackendTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.resource_id.fmt(formatter)
    }
}

// ============================================================================
// Service contract
// ============================================================================

/// Authoritative backend-selection service used by resilience.
///
/// A concrete implementation should integrate with:
///
/// - `crate::quantum::hardware::device_registry`;
/// - `crate::quantum::hardware::compatibility`;
/// - hardware capability/status/calibration contracts;
/// - the execution/recovery compatibility boundary.
///
/// It must never expose provider SDK types through this trait.
///
/// The service owns actual target discovery/selection semantics. The adapter
/// only enforces the generic resilience contract around it.
pub trait BackendSelectionService: Send + Sync + fmt::Debug {
    /// Returns service capabilities.
    fn capabilities(&self) -> BackendSelectionCapabilities;

    /// Performs non-mutating validation of the migration/selection request.
    fn preflight(&self, request: &AdaptationRequest) -> ResilienceResult<()>;

    /// Prepares a target-selection candidate without making it authoritative.
    fn prepare(
        &self,
        request: &AdaptationRequest,
        target: &BackendTarget,
    ) -> ResilienceResult<BackendCandidateId>;

    /// Commits a previously prepared selection candidate.
    ///
    /// Returns `true` only if the target selection was actually committed.
    /// The service must revalidate the target against the current authoritative
    /// registry/capability state before committing.
    fn commit(
        &self,
        request: &AdaptationRequest,
        target: &BackendTarget,
        candidate: &BackendCandidateId,
    ) -> ResilienceResult<bool>;

    /// Verifies the service-local target-selection result.
    ///
    /// This is not a replacement for final semantic verification.
    fn verify(
        &self,
        request: &AdaptationRequest,
        target: &BackendTarget,
        candidate: &BackendCandidateId,
    ) -> ResilienceResult<bool>;
}

// ============================================================================
// Adapter
// ============================================================================

/// Production resilience adapter for backend/target selection.
///
/// `S` is injected so the resilience layer remains independent of the
/// hardware registry implementation and provider ecosystem.
#[derive(Clone)]
pub struct BackendSelectionAdapter<S>
where
    S: BackendSelectionService,
{
    id: AdapterId,
    selector: Arc<S>,
}

impl<S> fmt::Debug for BackendSelectionAdapter<S>
where
    S: BackendSelectionService,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BackendSelectionAdapter")
            .field("id", &self.id)
            .field("selector", &self.selector)
            .finish()
    }
}

impl<S> BackendSelectionAdapter<S>
where
    S: BackendSelectionService,
{
    /// Creates an adapter around an authoritative selection service.
    pub fn new(selector: Arc<S>) -> ResilienceResult<Self> {
        let id = AdapterId::new(BACKEND_SELECTION_ADAPTER_ID)?;
        Ok(Self { id, selector })
    }

    /// Creates an adapter from an owned selection service.
    pub fn from_selector(selector: S) -> ResilienceResult<Self> {
        Self::new(Arc::new(selector))
    }

    /// Returns the injected selection service.
    #[must_use]
    pub fn selector(&self) -> &S {
        self.selector.as_ref()
    }

    /// Returns the shared selection-service handle.
    #[must_use]
    pub fn selector_handle(&self) -> &Arc<S> {
        &self.selector
    }

    /// Returns selection-service capabilities.
    #[must_use]
    pub fn selection_capabilities(&self) -> BackendSelectionCapabilities {
        self.selector.capabilities()
    }

    /// Extracts and validates the canonical migration target.
    fn target_from_request(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<BackendTarget> {
        match request.action_payload() {
            ActionPayload::Migrate { target, .. } => {
                Ok(BackendTarget::new(target.clone()))
            }
            _ => Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            )),
        }
    }

    /// Validates the generic request and backend-selection service contract.
    fn validate_request(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<()> {
        request.validate()?;

        if request.action_kind() != ActionKind::Migrate {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        let capabilities = self.selector.capabilities();

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
                ResilienceErrorCode::BackendSelectionFailed,
            ));
        }

        let target = self.target_from_request(request)?;

        if target.as_str().is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidIdentifier,
            ));
        }

        Ok(())
    }

    /// Validates an existing resilience candidate and extracts its opaque
    /// backend-selection identity.
    fn validate_candidate(
        &self,
        request: &AdaptationRequest,
        candidate: &AdaptationCandidate,
    ) -> ResilienceResult<BackendCandidateId> {
        if candidate.adapter() != &self.id {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        if candidate.action() != ActionKind::Migrate {
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

        BackendCandidateId::new(candidate.identity().to_owned())
    }

    /// Converts a service candidate into the canonical resilience candidate.
    fn make_candidate(
        &self,
        request: &AdaptationRequest,
        candidate_id: BackendCandidateId,
    ) -> ResilienceResult<AdaptationCandidate> {
        AdaptationCandidate::new(
            self.id.clone(),
            ActionKind::Migrate,
            request.generation().clone(),
            request.semantic_revision().clone(),
            candidate_id.into_string(),
        )
    }

    /// Prepares a backend-selection candidate.
    fn prepare_selection(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<AdaptationResult> {
        self.validate_request(request)?;

        let target = self.target_from_request(request)?;
        self.selector.preflight(request)?;

        let candidate_id = self.selector.prepare(request, &target)?;
        let candidate = self.make_candidate(request, candidate_id)?;

        Ok(
            AdaptationResult::new(
                self.id.clone(),
                ActionKind::Migrate,
                AdaptationStatus::Prepared,
                request.generation().clone(),
                request.semantic_revision().clone(),
            )
            .with_candidate(candidate),
        )
    }

    /// Commits a prepared backend-selection candidate.
    fn commit_selection(
        &self,
        request: &AdaptationRequest,
        candidate: &AdaptationCandidate,
    ) -> ResilienceResult<AdaptationResult> {
        self.validate_request(request)?;

        let target = self.target_from_request(request)?;
        let candidate_id = self.validate_candidate(request, candidate)?;

        let committed = self.selector.commit(
            request,
            &target,
            &candidate_id,
        )?;

        if !committed {
            return Ok(
                AdaptationResult::new(
                    self.id.clone(),
                    ActionKind::Migrate,
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
                ActionKind::Migrate,
                AdaptationStatus::CommittedPendingVerification,
                request.generation().clone(),
                request.semantic_revision().clone(),
            )
            .with_candidate(candidate.clone()),
        )
    }

    /// Verifies a committed backend-selection candidate.
    fn verify_selection(
        &self,
        request: &AdaptationRequest,
        candidate: &AdaptationCandidate,
    ) -> ResilienceResult<AdaptationResult> {
        self.validate_request(request)?;

        let target = self.target_from_request(request)?;
        let candidate_id = self.validate_candidate(request, candidate)?;

        let verified = self.selector.verify(
            request,
            &target,
            &candidate_id,
        )?;

        if !verified {
            return Ok(
                AdaptationResult::new(
                    self.id.clone(),
                    ActionKind::Migrate,
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
                ActionKind::Migrate,
                AdaptationStatus::Committed,
                request.generation().clone(),
                request.semantic_revision().clone(),
            )
            .with_candidate(candidate.clone()),
        )
    }
}

// ============================================================================
// Generic adaptation implementation
// ============================================================================

impl<S> AdaptationAdapter for BackendSelectionAdapter<S>
where
    S: BackendSelectionService,
{
    fn id(&self) -> &AdapterId {
        &self.id
    }

    fn version(&self) -> AdapterVersion {
        BACKEND_SELECTION_ADAPTER_VERSION
    }

    fn capabilities(&self) -> AdaptationCapabilities {
        let capabilities = self.selector.capabilities();

        AdaptationCapabilities::new(
            capabilities.supports_prepare(),
            capabilities.supports_commit(),
            capabilities.supports_preflight(),
            capabilities.deterministic(),
            capabilities.scoped(),
            capabilities.partial(),
            // Rollback is owned by resilience recovery/rollback.rs.
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
        self.validate_request(request)?;
        self.selector.preflight(request)
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
                        ActionKind::Migrate,
                        AdaptationStatus::Prepared,
                        request.generation().clone(),
                        request.semantic_revision().clone(),
                    ),
                )
            }

            AdaptationOperation::Prepare => {
                self.prepare_selection(request)
            }

            AdaptationOperation::Commit { candidate } => {
                self.commit_selection(request, candidate)
            }

            AdaptationOperation::Verify { candidate } => {
                self.verify_selection(request, candidate)
            }
        }
    }
}

// ============================================================================
// Shared handle
// ============================================================================

/// Thread-safe shared backend-selection adapter handle.
pub type BackendSelectionAdapterHandle<S> =
    Arc<BackendSelectionAdapter<S>>;

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
        ResourceId,
    };

    #[derive(Debug, Default)]
    struct TestBackendSelectionService;

    impl BackendSelectionService for TestBackendSelectionService {
        fn capabilities(&self) -> BackendSelectionCapabilities {
            BackendSelectionCapabilities::new(
                true,
                true,
                true,
                true,
                true,
                true,
            )
        }

        fn preflight(
            &self,
            request: &AdaptationRequest,
        ) -> ResilienceResult<()> {
            if request.action_kind() != ActionKind::Migrate {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::InvalidArgument,
                ));
            }

            Ok(())
        }

        fn prepare(
            &self,
            _request: &AdaptationRequest,
            target: &BackendTarget,
        ) -> ResilienceResult<BackendCandidateId> {
            BackendCandidateId::new(format!(
                "candidate:{}",
                target.as_str()
            ))
        }

        fn commit(
            &self,
            _request: &AdaptationRequest,
            _target: &BackendTarget,
            candidate: &BackendCandidateId,
        ) -> ResilienceResult<bool> {
            Ok(candidate.as_str().starts_with("candidate:"))
        }

        fn verify(
            &self,
            _request: &AdaptationRequest,
            _target: &BackendTarget,
            candidate: &BackendCandidateId,
        ) -> ResilienceResult<bool> {
            Ok(candidate.as_str().starts_with("candidate:"))
        }
    }

    fn migration_request() -> AdaptationRequest {
        let target = ResourceId::new("target-environment")
            .expect("test target must be valid");

        let action = RecoveryAction::new(
            ActionKind::Migrate,
            ActionPayload::Migrate {
                scope: ActionScope::Execution,
                target,
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
    fn adapter_supports_only_migration() {
        let adapter = BackendSelectionAdapter::from_selector(
            TestBackendSelectionService,
        )
        .expect("adapter construction must succeed");

        assert_eq!(
            adapter.supported_actions(),
            &[ActionKind::Migrate]
        );
    }

    #[test]
    fn prepare_commit_verify_are_transactional() {
        let adapter = BackendSelectionAdapter::from_selector(
            TestBackendSelectionService,
        )
        .expect("adapter construction must succeed");

        let request = migration_request();

        let prepared = adapter
            .execute(&AdaptationOperation::Prepare, &request)
            .expect("prepare must succeed");

        let candidate = prepared
            .candidate()
            .expect("prepare must return candidate");

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
                    candidate: candidate.clone(),
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
        let adapter = BackendSelectionAdapter::from_selector(
            TestBackendSelectionService,
        )
        .expect("adapter construction must succeed");

        let request = migration_request();

        let prepared = adapter
            .execute(&AdaptationOperation::Prepare, &request)
            .expect("prepare must succeed");

        let candidate = prepared
            .candidate()
            .expect("candidate must exist")
            .clone();

        let stale_request = {
            let target = ResourceId::new("target-environment")
                .expect("test target must be valid");

            let action = RecoveryAction::new(
                ActionKind::Migrate,
                ActionPayload::Migrate {
                    scope: ActionScope::Execution,
                    target,
                },
            );

            AdaptationRequest::new(
                action,
                ExecutionGeneration::new("generation-2")
                    .expect("test generation must be valid"),
                SemanticRevision::new("semantic-1")
                    .expect("test semantic revision must be valid"),
            )
        };

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
    fn wrong_action_is_rejected() {
        let adapter = BackendSelectionAdapter::from_selector(
            TestBackendSelectionService,
        )
        .expect("adapter construction must succeed");

        let action = RecoveryAction::new(
            ActionKind::Reoptimize,
            ActionPayload::Reoptimize {
                scope: ActionScope::Execution,
            },
        );

        let request = AdaptationRequest::new(
            action,
            ExecutionGeneration::new("generation-1")
                .expect("test generation must be valid"),
            SemanticRevision::new("semantic-1")
                .expect("test semantic revision must be valid"),
        );

        let result = adapter.preflight(&request);

        assert!(result.is_err());
        assert_eq!(
            result
                .expect_err("wrong action must fail")
                .code(),
            ResilienceErrorCode::InvalidArgument
        );
    }

    #[test]
    fn deterministic_selection_is_reported() {
        let adapter = BackendSelectionAdapter::from_selector(
            TestBackendSelectionService,
        )
        .expect("adapter construction must succeed");

        assert!(
            adapter
                .selection_capabilities()
                .deterministic()
        );

        assert!(
            adapter
                .capabilities()
                .deterministic()
        );
    }
}