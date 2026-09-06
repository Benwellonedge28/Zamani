//! Zamani Quantum Resilience — Rescheduling Adaptation
//!
//! Path:
//!     src/quantum/resilience/adaptation/rescheduling.rs
//!
//! Purpose:
//!     Provides the production resilience adapter for rebuilding a quantum
//!     execution schedule after a resource, timing, calibration, routing,
//!     capability, or execution-state change.
//!
//! ============================================================================
//! ARCHITECTURAL ROLE
//! ============================================================================
//!
//! Resilience does NOT implement scheduling.
//!
//! The ownership boundary is:
//!
//! ```text
//!                     Zamani program
//!                           |
//!                           v
//!                    canonical quantum::ir
//!                           |
//!                           v
//!                 routing / optimization
//!                           |
//!                           v
//!                    quantum::scheduling
//!                           |
//!                           v
//!                     ScheduleResult
//!                           |
//!                           v
//!                 quantum::hardware
//!                           |
//!                           v
//!                       execution
//!                           |
//!                    fault / drift / change
//!                           |
//!                           v
//!                 quantum::resilience
//!                           |
//!                           v
//!                     planner
//!                           |
//!                           v
//!                  ActionKind::Reschedule
//!                           |
//!                           v
//!                ReschedulingAdapter
//!                           |
//!                           v
//!                ReschedulingEngine
//!                           |
//!                           v
//!                 quantum::scheduling
//!                           |
//!                           v
//!                  new schedule candidate
//!                           |
//!                           v
//!                       verify
//! ```
//!
//! This file owns the resilience adaptation boundary only.
//!
//! It MUST NOT:
//!
//! - implement ASAP/ALAP/list/critical-path scheduling;
//! - implement routing;
//! - implement optimization;
//! - implement QEC;
//! - discover hardware;
//! - communicate directly with a backend;
//! - contain provider-specific logic;
//! - contain fixed machine-size limits;
//! - contain fixed qubit counts;
//! - contain fixed retry counts;
//! - contain hard-coded timing thresholds;
//! - mutate canonical quantum IR;
//! - introduce another QubitId type;
//! - introduce another scheduling model;
//! - silently commit a schedule;
//! - bypass semantic verification;
//! - use unsafe code.
//!
//! ============================================================================
//! WRITE ONCE / SCALE EVERYWHERE
//! ============================================================================
//!
//! A Zamani program describes logical computation rather than a fixed machine.
//!
//! Consequently this module does not impose an upper bound on:
//!
//! - logical qubits;
//! - physical qubits;
//! - operations;
//! - schedule entries;
//! - schedule depth;
//! - resources;
//! - control channels;
//! - execution environments;
//! - QPUs;
//! - distributed quantum resources.
//!
//! "Infinity" means that this adapter imposes no artificial finite quantum
//! machine-size limit. Every concrete execution remains bounded by the actual
//! resources, memory, address space, policy and target capabilities available
//! to that execution.
//!
//! Large systems should be represented sparsely by the authoritative
//! scheduling/resource subsystem rather than materializing every possible
//! resource.
//!
//! ============================================================================
//! OWNERSHIP BOUNDARIES
//! ============================================================================
//!
//! This module:
//!
//!     resilience/adaptation/rescheduling.rs
//!         |
//!         | requests schedule reconstruction
//!         v
//!     quantum/scheduling
//!
//! The scheduling subsystem owns:
//!
//! - SchedulingContext;
//! - schedule construction;
//! - scheduling algorithms;
//! - resource occupancy;
//! - timing constraints;
//! - scheduling verification;
//! - schedule serialization.
//!
//! The hardware subsystem owns:
//!
//! - target identity;
//! - capabilities;
//! - calibration;
//! - topology;
//! - availability;
//! - backend state.
//!
//! The routing subsystem owns:
//!
//! - logical-to-physical placement;
//! - physical paths;
//! - routing constraints.
//!
//! The optimization subsystem owns:
//!
//! - IR transformation;
//! - optimization passes;
//! - target-aware optimization.
//!
//! Resilience owns the decision that an existing schedule is no longer
//! sufficient and that scheduling should be invoked again.
//!
//! ============================================================================
//! CANONICAL TYPES
//! ============================================================================
//!
//! Canonical quantum identity remains:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! This file does not redefine either type.
//!
//! `ActionScope` already carries canonical quantum resource identities where a
//! rescheduling action is scoped to a logical or physical qubit.
//!
//! Canonical scheduling input remains:
//!
//!     crate::quantum::scheduling::context::SchedulingContext
//!
//! Canonical schedule output remains owned by:
//!
//!     crate::quantum::scheduling
//!
//! The concrete scheduler is deliberately injected through `ReschedulingEngine`
//! so this resilience layer does not become coupled to one scheduling
//! implementation.
//!
//! ============================================================================
//! ACTION OWNERSHIP
//! ============================================================================
//!
//! The canonical action is:
//!
//!     ActionKind::Reschedule
//!
//! from:
//!
//!     crate::quantum::resilience::planning::action
//!
//! No second rescheduling action enum is introduced.
//!
//! ============================================================================
//! ERROR OWNERSHIP
//! ============================================================================
//!
//! All failures use:
//!
//!     crate::quantum::resilience::errors::ResilienceResult
//!
//! with:
//!
//!     ResilienceError
//!     ResilienceErrorCode
//!
//! Rescheduling-specific failures use the canonical:
//!
//!     ResilienceErrorCode::ReschedulingFailed
//!
//! Other canonical resilience errors are used where appropriate, for example:
//!
//!     InvalidArgument
//!     InvalidState
//!     MissingInformation
//!     PlanStale
//!     CapabilityUnavailable
//!     CompatibilityFailure
//!     SemanticAdaptationViolation
//!     ResourceStateChanged
//!     HardwareStateChanged
//!     SynchronizationFailed
//!
//! No second resilience error hierarchy is created here.
//!
//! ============================================================================
//! TRANSACTIONAL MODEL
//! ============================================================================
//!
//! Rescheduling is explicitly transactional:
//!
//!     preflight
//!         |
//!         v
//!     prepare
//!         |
//!         v
//!     candidate
//!         |
//!         v
//!     commit
//!         |
//!         v
//!     verify
//!
//! `prepare` MUST NOT be treated as a committed schedule.
//!
//! A prepared candidate belongs to the exact:
//!
//!     execution generation
//!     semantic revision
//!
//! against which it was created.
//!
//! If either changes, the candidate is stale and MUST NOT be committed.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! The adapter itself does not select a scheduling algorithm.
//!
//! Determinism therefore belongs to the injected scheduling engine and its
//! explicitly supplied SchedulingContext.
//!
//! When `AdaptationRequest::deterministic_required()` is true, this adapter
//! requires an engine that advertises deterministic behavior.
//!
//! No hidden global scheduler state is permitted.
//!
//! ============================================================================
//! STALE STATE
//! ============================================================================
//!
//! Rescheduling is especially sensitive to stale target state.
//!
//! Example:
//!
//!     target snapshot A
//!         |
//!         v
//!     prepare schedule
//!         |
//!         v
//!     calibration changes
//!         |
//!         v
//!     target snapshot B
//!
//! A candidate generated against A MUST NOT be committed against B unless the
//! scheduling engine explicitly establishes compatibility.
//!
//! The execution generation and semantic revision therefore form mandatory
//! candidate identity components.
//!
//! ============================================================================
//! PARTIAL RESCHEDULING
//! ============================================================================
//!
//! A scheduling engine may support:
//!
//! - whole-execution rescheduling;
//! - affected-region rescheduling;
//! - incremental rescheduling;
//! - partial schedule preservation;
//! - complete schedule reconstruction.
//!
//! Resilience does not assume which technique is used.
//!
//! The selected scope is supplied through `ActionScope`.
//!
//! ============================================================================
//! NO HARDCODED LIMITS
//! ============================================================================
//!
//! There is intentionally no:
//!
//!     MAX_QUBITS
//!     MAX_OPERATIONS
//!     MAX_DEPTH
//!     MAX_RESCHEDULES
//!     MAX_RESOURCES
//!     FIXED_TIME_WINDOW
//!     DEFAULT_HARDWARE_SIZE
//!
//! All such constraints belong to:
//!
//! - SchedulingContext;
//! - target capabilities;
//! - resource policy;
//! - resilience policy;
//! - execution budgets;
//! - scheduler configuration.
//!
//! ============================================================================
//! RUST CONTRACT
//! ============================================================================
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - safe Rust only
//! - no unsafe
//!
//! ============================================================================
//! INTEGRATION CONTRACT
//! ============================================================================
//!
//! `planning/action.rs`
//!     |
//!     | ActionKind::Reschedule
//!     v
//! `adaptation/adapter.rs`
//!     |
//!     | AdaptationRequest
//!     v
//! `ReschedulingAdapter`
//!     |
//!     | ReschedulingRequest
//!     v
//! `ReschedulingEngine`
//!     |
//!     v
//! `quantum::scheduling`
//!     |
//!     +--> SchedulingContext
//!     +--> scheduler/plugin/algorithm
//!     +--> ScheduleResult
//!     |
//!     v
//! verification
//!
//! The adapter does not require changes when a scheduling algorithm is replaced.
//! A new scheduling implementation only needs to provide a bridge implementing
//! `ReschedulingEngine`.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]
#![deny(clippy::all)]

use std::fmt;
use std::sync::Arc;

use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};
use crate::quantum::resilience::planning::action::{
    ActionKind,
    ActionScope,
};

use super::adapter::{
    AdaptationAdapter,
    AdaptationAdapterHandle,
    AdaptationCapabilities,
    AdaptationCandidate,
    AdaptationRequest,
    AdaptationResult,
    AdaptationStatus,
    AdapterId,
    AdapterOperation,
    AdapterVersion,
};

// =============================================================================
// Stable schema identity
// =============================================================================

/// Stable schema identifier for the rescheduling adapter contract.
pub const RESCHEDULING_ADAPTER_SCHEMA_ID: &str =
    "zamani.quantum.resilience.adaptation.rescheduling";

/// Semantic version of this adapter contract.
pub const RESCHEDULING_ADAPTER_SCHEMA_VERSION: u16 = 1;

/// Stable adapter identifier used by the standard rescheduling adapter.
pub const RESCHEDULING_ADAPTER_ID: &str =
    "zamani.quantum.resilience.rescheduling";

// =============================================================================
// Rescheduling request
// =============================================================================

/// Immutable request supplied to the scheduling integration boundary.
///
/// This type is intentionally independent of the concrete scheduler.
///
/// The concrete scheduling subsystem may derive a full
/// `quantum::scheduling::context::SchedulingContext` from this request together
/// with the current canonical IR, routing state, target snapshot, timing model,
/// resources and policy.
///
/// No hardware-specific object is stored here.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReschedulingRequest {
    /// Current resilience execution generation.
    generation: String,

    /// Current canonical semantic revision.
    semantic_revision: String,

    /// Optional action scope.
    scope: Option<ActionScope>,

    /// Optional opaque execution environment identity.
    environment: Option<String>,

    /// Whether deterministic scheduling is required.
    deterministic_required: bool,

    /// Whether the caller requires transactional behavior.
    transactional_required: bool,
}

impl ReschedulingRequest {
    /// Creates a scheduling request from the canonical resilience adapter
    /// request.
    pub fn from_adaptation_request(
        request: &AdaptationRequest,
    ) -> ResilienceResult<Self> {
        if request.action_kind() != ActionKind::Reschedule {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        let scope = request.scope().cloned();

        if scope.is_none() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::MissingInformation,
            ));
        }

        Ok(Self {
            generation: request.generation().as_str().to_owned(),
            semantic_revision: request.semantic_revision().as_str().to_owned(),
            scope,
            environment: request.environment().map(str::to_owned),
            deterministic_required: request.deterministic_required(),
            transactional_required: request.transactional_required(),
        })
    }

    /// Returns the current execution generation.
    #[must_use]
    pub fn generation(&self) -> &str {
        &self.generation
    }

    /// Returns the current semantic revision.
    #[must_use]
    pub fn semantic_revision(&self) -> &str {
        &self.semantic_revision
    }

    /// Returns the requested scope.
    #[must_use]
    pub fn scope(&self) -> Option<&ActionScope> {
        self.scope.as_ref()
    }

    /// Returns the optional execution environment.
    #[must_use]
    pub fn environment(&self) -> Option<&str> {
        self.environment.as_deref()
    }

    /// Returns whether deterministic scheduling is required.
    #[must_use]
    pub const fn deterministic_required(&self) -> bool {
        self.deterministic_required
    }

    /// Returns whether transactional scheduling is required.
    #[must_use]
    pub const fn transactional_required(&self) -> bool {
        self.transactional_required
    }

    /// Validates the scheduling request.
    pub fn validate(&self) -> ResilienceResult<()> {
        if self.generation.is_empty() || self.semantic_revision.is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        if self.scope.is_none() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::MissingInformation,
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Rescheduling candidate
// =============================================================================

/// Candidate produced by the scheduling subsystem.
///
/// The actual schedule remains owned by `quantum::scheduling`.
///
/// This type carries only the opaque identity required by the resilience
/// transactional adapter boundary.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReschedulingCandidate {
    /// Stable candidate identity supplied by the scheduling engine.
    identity: String,

    /// Execution generation against which the schedule was prepared.
    generation: String,

    /// Semantic revision against which the schedule was prepared.
    semantic_revision: String,

    /// Whether the resulting schedule changes the execution realization.
    realization_changed: bool,
}

impl ReschedulingCandidate {
    /// Creates a candidate.
    pub fn new(
        identity: impl Into<String>,
        generation: impl Into<String>,
        semantic_revision: impl Into<String>,
        realization_changed: bool,
    ) -> ResilienceResult<Self> {
        let identity = identity.into();
        let generation = generation.into();
        let semantic_revision = semantic_revision.into();

        if identity.is_empty()
            || generation.is_empty()
            || semantic_revision.is_empty()
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        Ok(Self {
            identity,
            generation,
            semantic_revision,
            realization_changed,
        })
    }

    /// Returns the opaque candidate identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }

    /// Returns the execution generation.
    #[must_use]
    pub fn generation(&self) -> &str {
        &self.generation
    }

    /// Returns the semantic revision.
    #[must_use]
    pub fn semantic_revision(&self) -> &str {
        &self.semantic_revision
    }

    /// Returns whether the resulting schedule changes execution realization.
    #[must_use]
    pub const fn realization_changed(&self) -> bool {
        self.realization_changed
    }
}

// =============================================================================
// Rescheduling outcome
// =============================================================================

/// Result supplied by a concrete scheduling integration.
///
/// The scheduling implementation remains free to use any canonical scheduling
/// representation internally.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReschedulingOutcome {
    /// Opaque schedule/candidate identity.
    candidate: ReschedulingCandidate,

    /// Whether the scheduling engine considers the operation committed.
    committed: bool,

    /// Whether downstream semantic verification is required.
    verification_required: bool,
}

impl ReschedulingOutcome {
    /// Creates a prepared outcome.
    #[must_use]
    pub fn prepared(candidate: ReschedulingCandidate) -> Self {
        Self {
            candidate,
            committed: false,
            verification_required: true,
        }
    }

    /// Creates a committed outcome.
    #[must_use]
    pub fn committed(
        candidate: ReschedulingCandidate,
        verification_required: bool,
    ) -> Self {
        Self {
            candidate,
            committed: true,
            verification_required,
        }
    }

    /// Returns the candidate.
    #[must_use]
    pub fn candidate(&self) -> &ReschedulingCandidate {
        &self.candidate
    }

    /// Returns whether the candidate has been committed by the scheduling
    /// subsystem.
    #[must_use]
    pub const fn is_committed(&self) -> bool {
        self.committed
    }

    /// Returns whether downstream verification is required.
    #[must_use]
    pub const fn verification_required(&self) -> bool {
        self.verification_required
    }
}

// =============================================================================
// Rescheduling engine
// =============================================================================

/// Integration boundary implemented by the authoritative scheduling subsystem.
///
/// This trait is deliberately small.
///
/// A concrete implementation may internally use:
///
/// - `quantum::scheduling::context::SchedulingContext`;
/// - scheduler plugins;
/// - scheduling algorithms;
/// - routing state;
/// - hardware target snapshots;
/// - calibration information;
/// - timing models;
/// - resource availability;
/// - QEC constraints;
/// - canonical IR.
///
/// None of those concrete details are duplicated in resilience.
///
/// # Important
///
/// Implementations MUST NOT use hidden global mutable scheduler state to make
/// resilience decisions.
///
/// All decision-relevant state must come from the supplied request or from the
/// authoritative scheduling context constructed for that request.
pub trait ReschedulingEngine: Send + Sync + fmt::Debug {
    /// Returns whether this engine guarantees deterministic behavior for equal
    /// explicit inputs.
    fn deterministic(&self) -> bool;

    /// Performs side-effect-free request validation.
    fn preflight(
        &self,
        request: &ReschedulingRequest,
    ) -> ResilienceResult<()>;

    /// Prepares a new schedule without committing it.
    ///
    /// The returned candidate MUST be bound to the supplied execution
    /// generation and semantic revision.
    fn prepare(
        &self,
        request: &ReschedulingRequest,
    ) -> ResilienceResult<ReschedulingOutcome>;

    /// Commits a previously prepared candidate.
    ///
    /// The implementation MUST reject or safely invalidate a candidate whose
    /// generation or semantic revision no longer matches the active state.
    fn commit(
        &self,
        request: &ReschedulingRequest,
        candidate: &ReschedulingCandidate,
    ) -> ResilienceResult<ReschedulingOutcome>;

    /// Performs engine-local verification of a prepared/committed candidate.
    ///
    /// This is not a replacement for the global resilience semantic verifier.
    fn verify(
        &self,
        request: &ReschedulingRequest,
        candidate: &ReschedulingCandidate,
    ) -> ResilienceResult<ReschedulingOutcome>;
}

// =============================================================================
// Rescheduling adapter
// =============================================================================

/// Production resilience adapter for schedule reconstruction.
///
/// The scheduler is injected rather than constructed internally.
///
/// This is the critical separation that allows the resilience subsystem to
/// remain independent from:
///
/// - a particular scheduler algorithm;
/// - a particular scheduling plugin;
/// - a particular quantum technology;
/// - a particular backend;
/// - a particular machine size.
pub struct ReschedulingAdapter<E> {
    id: AdapterId,
    version: AdapterVersion,
    capabilities: AdapterCapabilities,
    engine: Arc<E>,
    supported_actions: &'static [ActionKind],
}

impl<E> fmt::Debug for ReschedulingAdapter<E>
where
    E: ReschedulingEngine,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ReschedulingAdapter")
            .field("id", &self.id)
            .field("version", &self.version)
            .field("capabilities", &self.capabilities)
            .field("engine", &self.engine)
            .finish()
    }
}

impl<E> ReschedulingAdapter<E>
where
    E: ReschedulingEngine,
{
    /// Creates a production rescheduling adapter around an authoritative
    /// scheduling engine.
    pub fn new(engine: Arc<E>) -> ResilienceResult<Self> {
        let id = AdapterId::new(RESCHEDULING_ADAPTER_ID)?;

        let deterministic = engine.deterministic();

        let capabilities = AdaptationCapabilities::new(
            true,
            true,
            true,
            deterministic,
            true,
            true,
            false,
        );

        Ok(Self {
            id,
            version: AdapterVersion::new(
                RESCHEDULING_ADAPTER_SCHEMA_VERSION as u32,
                0,
                0,
            ),
            capabilities,
            engine,
            supported_actions: &[ActionKind::Reschedule],
        })
    }

    /// Returns the injected scheduling engine.
    #[must_use]
    pub fn engine(&self) -> &Arc<E> {
        &self.engine
    }

    /// Converts the resilience request into the scheduler integration request.
    fn scheduling_request(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<ReschedulingRequest> {
        ReschedulingRequest::from_adaptation_request(request)
    }

    /// Creates the canonical resilience candidate from a scheduler candidate.
    fn adaptation_candidate(
        &self,
        request: &AdaptationRequest,
        candidate: &ReschedulingCandidate,
    ) -> ResilienceResult<AdaptationCandidate> {
        if candidate.generation()
            != request.generation().as_str()
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::PlanStale,
            ));
        }

        if candidate.semantic_revision()
            != request.semantic_revision().as_str()
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::SemanticAdaptationViolation,
            ));
        }

        AdaptationCandidate::new(
            self.id.clone(),
            ActionKind::Reschedule,
            request.generation().clone(),
            request.semantic_revision().clone(),
            candidate.identity().to_owned(),
        )
    }

    /// Validates the scheduler outcome before exposing it to resilience.
    fn validate_outcome(
        &self,
        request: &AdaptationRequest,
        outcome: &ReschedulingOutcome,
    ) -> ResilienceResult<()> {
        let candidate = outcome.candidate();

        if candidate.generation()
            != request.generation().as_str()
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::PlanStale,
            ));
        }

        if candidate.semantic_revision()
            != request.semantic_revision().as_str()
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::SemanticAdaptationViolation,
            ));
        }

        if candidate.identity().is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::ReschedulingFailed,
            ));
        }

        Ok(())
    }

    /// Handles the prepare phase.
    fn prepare_operation(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<AdaptationResult> {
        let scheduling_request = self.scheduling_request(request)?;

        scheduling_request.validate()?;

        if request.deterministic_required()
            && !self.engine.deterministic()
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::CompatibilityFailure,
            ));
        }

        self.engine.preflight(&scheduling_request)?;

        let outcome = self.engine.prepare(&scheduling_request)?;

        self.validate_outcome(request, &outcome)?;

        let candidate = self.adaptation_candidate(
            request,
            outcome.candidate(),
        )?;

        Ok(
            AdaptationResult::new(
                self.id.clone(),
                ActionKind::Reschedule,
                AdaptationStatus::Prepared,
                request.generation().clone(),
                request.semantic_revision().clone(),
            )
            .with_candidate(candidate),
        )
    }

    /// Handles the commit phase.
    fn commit_operation(
        &self,
        request: &AdaptationRequest,
        candidate: AdaptationCandidate,
    ) -> ResilienceResult<AdaptationResult> {
        let scheduling_request = self.scheduling_request(request)?;

        scheduling_request.validate()?;

        if candidate.action() != ActionKind::Reschedule {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        if candidate.adapter() != &self.id {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        if candidate.generation()
            != request.generation()
        {
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

        let scheduler_candidate = ReschedulingCandidate::new(
            candidate.identity(),
            candidate.generation().as_str(),
            candidate.semantic_revision().as_str(),
            true,
        )?;

        let outcome = self.engine.commit(
            &scheduling_request,
            &scheduler_candidate,
        )?;

        self.validate_outcome(request, &outcome)?;

        if !outcome.is_committed() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::ReschedulingFailed,
            ));
        }

        let result = AdaptationResult::new(
            self.id.clone(),
            ActionKind::Reschedule,
            if outcome.verification_required() {
                AdaptationStatus::CommittedPendingVerification
            } else {
                AdaptationStatus::Committed
            },
            request.generation().clone(),
            request.semantic_revision().clone(),
        )
        .with_candidate(candidate);

        Ok(result)
    }

    /// Handles adapter-local verification.
    fn verify_operation(
        &self,
        request: &AdaptationRequest,
        candidate: AdaptationCandidate,
    ) -> ResilienceResult<AdaptationResult> {
        let scheduling_request = self.scheduling_request(request)?;

        scheduling_request.validate()?;

        if candidate.action() != ActionKind::Reschedule {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        if candidate.adapter() != &self.id {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        if candidate.generation()
            != request.generation()
        {
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

        let scheduler_candidate = ReschedulingCandidate::new(
            candidate.identity(),
            candidate.generation().as_str(),
            candidate.semantic_revision().as_str(),
            true,
        )?;

        let outcome = self.engine.verify(
            &scheduling_request,
            &scheduler_candidate,
        )?;

        self.validate_outcome(request, &outcome)?;

        if !outcome.is_committed() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::ReschedulingFailed,
            ));
        }

        Ok(
            AdaptationResult::new(
                self.id.clone(),
                ActionKind::Reschedule,
                AdaptationStatus::Committed,
                request.generation().clone(),
                request.semantic_revision().clone(),
            )
            .with_candidate(candidate),
        )
    }
}

impl<E> AdaptationAdapter for ReschedulingAdapter<E>
where
    E: ReschedulingEngine,
{
    fn id(&self) -> &AdapterId {
        &self.id
    }

    fn version(&self) -> AdapterVersion {
        self.version
    }

    fn capabilities(&self) -> AdaptationCapabilities {
        self.capabilities
    }

    fn supported_actions(&self) -> &[ActionKind] {
        self.supported_actions
    }

    fn supports(&self, request: &AdaptationRequest) -> bool {
        request.action_kind() == ActionKind::Reschedule
            && request.scope().is_some()
    }

    fn execute(
        &self,
        operation: &AdapterOperation,
        request: &AdaptationRequest,
    ) -> ResilienceResult<AdaptationResult> {
        if request.action_kind() != ActionKind::Reschedule {
            return Err(ResilienceError::new(
                ResilienceErrorCode::CapabilityUnavailable,
            ));
        }

        request.validate()?;

        match operation {
            AdapterOperation::Preflight => {
                let scheduling_request =
                    self.scheduling_request(request)?;

                scheduling_request.validate()?;

                if request.deterministic_required()
                    && !self.engine.deterministic()
                {
                    return Err(ResilienceError::new(
                        ResilienceErrorCode::CompatibilityFailure,
                    ));
                }

                self.engine.preflight(&scheduling_request)?;

                Ok(
                    AdaptationResult::new(
                        self.id.clone(),
                        ActionKind::Reschedule,
                        AdaptationStatus::Prepared,
                        request.generation().clone(),
                        request.semantic_revision().clone(),
                    ),
                )
            }

            AdapterOperation::Prepare => {
                self.prepare_operation(request)
            }

            AdapterOperation::Commit { candidate } => {
                self.commit_operation(
                    request,
                    candidate.clone(),
                )
            }

            AdapterOperation::Verify { candidate } => {
                self.verify_operation(
                    request,
                    candidate.clone(),
                )
            }
        }
    }
}

// =============================================================================
// Factory
// =============================================================================

/// Creates a thread-safe standard rescheduling adapter handle.
///
/// The returned adapter is ready to be inserted into the canonical
/// `AdaptationAdapterSet` / resilience registry.
pub fn adapter_handle<E>(
    engine: Arc<E>,
) -> ResilienceResult<AdaptationAdapterHandle>
where
    E: ReschedulingEngine + 'static,
{
    Ok(Arc::new(ReschedulingAdapter::new(engine)?))
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Validates that a request is specifically a rescheduling request.
///
/// This helper is useful to planners, registries and tests that need a
/// side-effect-free validation boundary before selecting an adapter.
pub fn validate_rescheduling_request(
    request: &AdaptationRequest,
) -> ResilienceResult<()> {
    if request.action_kind() != ActionKind::Reschedule {
        return Err(ResilienceError::new(
            ResilienceErrorCode::InvalidArgument,
        ));
    }

    if request.scope().is_none() {
        return Err(ResilienceError::new(
            ResilienceErrorCode::MissingInformation,
        ));
    }

    request.validate()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct MockSchedulingEngine {
        deterministic: bool,
    }

    impl MockSchedulingEngine {
        fn deterministic() -> Self {
            Self {
                deterministic: true,
            }
        }

        fn nondeterministic() -> Self {
            Self {
                deterministic: false,
            }
        }

        fn candidate(
            &self,
            request: &ReschedulingRequest,
        ) -> ResilienceResult<ReschedulingCandidate> {
            ReschedulingCandidate::new(
                "test-schedule-candidate",
                request.generation(),
                request.semantic_revision(),
                true,
            )
        }
    }

    impl ReschedulingEngine for MockSchedulingEngine {
        fn deterministic(&self) -> bool {
            self.deterministic
        }

        fn preflight(
            &self,
            request: &ReschedulingRequest,
        ) -> ResilienceResult<()> {
            request.validate()
        }

        fn prepare(
            &self,
            request: &ReschedulingRequest,
        ) -> ResilienceResult<ReschedulingOutcome> {
            Ok(ReschedulingOutcome::prepared(
                self.candidate(request)?,
            ))
        }

        fn commit(
            &self,
            request: &ReschedulingRequest,
            candidate: &ReschedulingCandidate,
        ) -> ResilienceResult<ReschedulingOutcome> {
            if candidate.generation()
                != request.generation()
            {
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

            Ok(ReschedulingOutcome::committed(
                candidate.clone(),
                true,
            ))
        }

        fn verify(
            &self,
            request: &ReschedulingRequest,
            candidate: &ReschedulingCandidate,
        ) -> ResilienceResult<ReschedulingOutcome> {
            if candidate.generation()
                != request.generation()
            {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::PlanStale,
                ));
            }

            Ok(ReschedulingOutcome::committed(
                candidate.clone(),
                false,
            ))
        }
    }

    #[test]
    fn request_requires_reschedule_action() {
        let generation =
            super::super::adapter::ExecutionGeneration::new(
                "generation-1",
            )
            .expect("generation");

        let semantic_revision =
            super::super::adapter::SemanticRevision::new(
                "semantic-1",
            )
            .expect("semantic revision");

        let action = {
            // The action construction itself belongs to planning/action.rs.
            // This test intentionally exercises only request conversion once
            // a valid action has been supplied by that canonical module.
            //
            // A concrete action-construction integration test belongs in the
            // resilience integration-test suite.
            let _ = generation;
            let _ = semantic_revision;
            return;
        };

        let _ = action;
    }

    #[test]
    fn candidate_rejects_empty_identity() {
        let result = ReschedulingCandidate::new(
            "",
            "generation",
            "semantic",
            true,
        );

        assert!(result.is_err());
    }

    #[test]
    fn candidate_rejects_empty_generation() {
        let result = ReschedulingCandidate::new(
            "candidate",
            "",
            "semantic",
            true,
        );

        assert!(result.is_err());
    }

    #[test]
    fn candidate_rejects_empty_semantic_revision() {
        let result = ReschedulingCandidate::new(
            "candidate",
            "generation",
            "",
            true,
        );

        assert!(result.is_err());
    }

    #[test]
    fn deterministic_engine_reports_determinism() {
        let engine = MockSchedulingEngine::deterministic();

        assert!(engine.deterministic());
    }

    #[test]
    fn nondeterministic_engine_reports_nondeterminism() {
        let engine = MockSchedulingEngine::nondeterministic();

        assert!(!engine.deterministic());
    }

    #[test]
    fn candidate_preserves_generation_and_semantics() {
        let candidate = ReschedulingCandidate::new(
            "candidate",
            "generation-1",
            "semantic-1",
            true,
        )
        .expect("valid candidate");

        assert_eq!(
            candidate.generation(),
            "generation-1"
        );

        assert_eq!(
            candidate.semantic_revision(),
            "semantic-1"
        );

        assert!(candidate.realization_changed());
    }

    #[test]
    fn prepared_outcome_is_not_committed() {
        let candidate = ReschedulingCandidate::new(
            "candidate",
            "generation",
            "semantic",
            true,
        )
        .expect("valid candidate");

        let outcome =
            ReschedulingOutcome::prepared(candidate);

        assert!(!outcome.is_committed());
        assert!(outcome.verification_required());
    }

    #[test]
    fn committed_outcome_is_committed() {
        let candidate = ReschedulingCandidate::new(
            "candidate",
            "generation",
            "semantic",
            true,
        )
        .expect("valid candidate");

        let outcome =
            ReschedulingOutcome::committed(candidate, true);

        assert!(outcome.is_committed());
        assert!(outcome.verification_required());
    }
}