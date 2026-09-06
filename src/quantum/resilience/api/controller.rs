//! Zamani Quantum Resilience — Production Controller
//!
//! Path:
//!     src/quantum/resilience/api/controller.rs
//!
//! Purpose:
//!     Stable public orchestration boundary for quantum execution resilience.
//!
//! Architectural role:
//!     `ResilienceController` coordinates the resilience lifecycle without
//!     implementing detection, diagnosis, policy, planning, adaptation,
//!     recovery, mitigation, or verification algorithms itself.
//!
//! Lifecycle:
//!
//!     OBSERVE
//!        ↓
//!     DETECT
//!        ↓
//!     DIAGNOSE
//!        ↓
//!     POLICY
//!        ↓
//!     PLAN
//!        ↓
//!     ADAPT
//!        ↓
//!     RECOVER / CONTINUE
//!        ↓
//!     VERIFY
//!        ↓
//!     ACCEPT / REPEAT / ESCALATE / REJECT
//!
//! Design requirements:
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021 edition
//! - stable Rust only
//! - no `unsafe`
//! - no provider-specific implementation
//! - no hard-coded machine-size limit
//! - no hard-coded qubit count
//! - no hard-coded retry count
//! - no hard-coded fidelity threshold
//! - deterministic orchestration when the supplied dependencies are
//!   deterministic
//! - canonical quantum identities remain owned by `quantum::ir::qubit`
//! - resilience remains an orchestration layer
//! - all external behavior is supplied through explicit contracts
//! - no hidden global mutable state
//! - no environment-variable configuration
//! - no implicit I/O
//! - no background threads created by this controller
//!
//! # Canonical quantum identity
//!
//! This file deliberately does not define `QubitId` or `PhysicalQubitId`.
//!
//! The canonical definitions remain:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! If a future controller extension needs to identify logical or physical
//! qubits, it MUST use those canonical types.
//!
//! # Dependency direction
//!
//! ```text
//! quantum::ir
//!      │
//!      ├───────────────┐
//!      │               │
//!      ▼               ▼
//! resilience API   quantum execution subsystems
//!      │               │
//!      └───────┬───────┘
//!              ▼
//!       ResilienceController
//!              │
//!       ┌──────┼────────┬────────┬─────────┐
//!       ▼      ▼        ▼        ▼         ▼
//!     detect diagnose policy   plan      verify
//!                       │
//!                       ▼
//!                    adapt/recover
//! ```
//!
//! The controller owns orchestration only.
//!
//! # Write once, scale everywhere
//!
//! This file contains no architectural machine-size constants.
//!
//! It MUST NOT assume:
//!
//! ```text
//! 1 qubit
//! 127 qubits
//! 1000 qubits
//! 1_000_000 qubits
//! ```
//!
//! Nor may it assume a finite maximum number of:
//!
//! - resources;
//! - incidents;
//! - recovery cycles;
//! - backends;
//! - devices;
//! - logical qubits;
//! - physical qubits;
//! - operations.
//!
//! Actual limits are supplied by request policy, target capabilities and
//! available execution resources.
//!
//! "Infinite" scalability therefore means that this controller imposes no
//! artificial finite quantum-machine ceiling.
//!
//! # Safety invariant
//!
//! A controller MUST NOT accept a result merely because an execution
//! eventually completed.
//!
//! Acceptance is determined by the verification contract.
//!
//! ```text
//! execution completed
//!        ≠
//! result correct
//! ```
//!
//! # No hidden retry loop
//!
//! The controller does not implement:
//!
//! ```text
//! for _ in 0..3
//! ```
//!
//! or any equivalent fixed recovery loop.
//!
//! Recovery repetition is controlled by the explicit resilience policy and
//! planner state supplied by the caller.
//!
//! This is necessary for both scalability and correctness.
//!
//! # Determinism
//!
//! The controller itself is deterministic with respect to its inputs and
//! supplied dependency behavior.
//!
//! It does not:
//!
//! - read the clock;
//! - generate randomness;
//! - read environment variables;
//! - access global mutable state;
//! - perform implicit I/O;
//! - inspect memory addresses.
//!
//! If randomized resilience is required, randomness MUST be supplied through
//! the explicit request/context contract and therefore becomes part of the
//! reproducibility boundary.
//!
//! # Security
//!
//! The controller never handles provider credentials directly.
//!
//! Authorization, authentication and trusted observation validation belong to
//! the corresponding security/integration contracts.
//!
//! The controller MUST NOT:
//!
//! - bypass policy;
//! - bypass verification;
//! - silently downgrade semantic guarantees;
//! - manufacture trusted telemetry;
//! - expose secrets through errors;
//! - authorize a recovery action merely because it improves availability.
//!
//! # Integration contract
//!
//! The controller is intentionally written against traits defined in the
//! sibling API/domain modules rather than concrete implementation types.
//!
//! Expected stable public API modules:
//!
//! ```text
//! crate::quantum::resilience::api::request
//! crate::quantum::resilience::api::response
//! crate::quantum::resilience::api::context
//! ```
//!
//! Expected orchestration contracts:
//!
//! ```text
//! detection
//! diagnosis
//! policy
//! planning
//! adaptation
//! recovery
//! mitigation
//! verification
//! ```
//!
//! Those contracts are injected through `ResilienceContext`.
//!
//! Consequently this controller does not need to be edited when a detector,
//! recovery strategy, backend adapter, QEC implementation or verification
//! implementation changes.
//!
//! # Important integration rule
//!
//! `ResilienceController` should be the only high-level entry point required
//! by runtime/execution code.
//!
//! Runtime code should not directly call individual resilience internals when
//! it intends to execute the complete resilience lifecycle.
//!
//! # Failure handling
//!
//! Every dependency failure is propagated as `ResilienceError`.
//!
//! The controller does not silently convert a failure into success.
//!
//! A dependency may explicitly return a decision requiring escalation or
//! rejection; that decision remains observable in the response.
//!
//! # Ownership
//!
//! The controller borrows the supplied request/context for the duration of
//! one orchestration operation.
//!
//! It does not retain borrowed execution state after the operation returns.
//!
//! This avoids hidden lifetime coupling and makes the controller suitable for
//! synchronous runtime integration, deterministic replay, testing and future
//! distributed orchestration.
//!
//! # Extension rule
//!
//! New resilience mechanisms should normally be implemented behind the
//! appropriate context contract rather than added as another provider-specific
//! branch in this file.
//!
//! For example, do NOT add:
//!
//! ```text
//! if backend == IBM { ... }
//! if backend == AWS { ... }
//! if backend == Google { ... }
//! ```
//!
//! Instead, backend capabilities and behavior must be exposed through the
//! hardware/execution contracts consumed by the resilience context.
//!
//! # Rust safety
//!
//! This module explicitly forbids unsafe Rust.
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;

use crate::quantum::resilience::errors::error::ResilienceError;

// =============================================================================
// Public schema
// =============================================================================

/// Stable schema identifier for the controller API.
pub const RESILIENCE_CONTROLLER_SCHEMA_ID: &str =
    "zamani.quantum.resilience.api.controller";

/// Semantic version of the controller contract.
///
/// This version changes only when the externally observable controller
/// contract changes according to the resilience compatibility policy.
pub const RESILIENCE_CONTROLLER_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Lifecycle phase
// =============================================================================

/// A single phase of one resilience orchestration cycle.
///
/// The controller never assumes that every cycle must execute every phase.
/// A supplied orchestration contract may legitimately determine that a phase
/// is unnecessary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResiliencePhase {
    /// Initial observation/collection boundary.
    Observe,

    /// Fault/anomaly detection.
    Detect,

    /// Fault/incident diagnosis.
    Diagnose,

    /// Policy evaluation.
    Policy,

    /// Recovery/adaptation planning.
    Plan,

    /// Physical/logical execution adaptation.
    Adapt,

    /// Recovery or continued execution.
    Recover,

    /// Optional error-mitigation stage.
    Mitigate,

    /// Semantic/result verification.
    Verify,

    /// Final lifecycle decision.
    Decide,
}

impl ResiliencePhase {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Observe => "observe",
            Self::Detect => "detect",
            Self::Diagnose => "diagnose",
            Self::Policy => "policy",
            Self::Plan => "plan",
            Self::Adapt => "adapt",
            Self::Recover => "recover",
            Self::Mitigate => "mitigate",
            Self::Verify => "verify",
            Self::Decide => "decide",
        }
    }
}

impl fmt::Display for ResiliencePhase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Controller lifecycle decision
// =============================================================================

/// Final decision produced by a resilience cycle.
///
/// This is deliberately an outcome rather than an implementation instruction.
/// The runtime can use the decision to determine what happens next without the
/// controller knowing how the runtime executes that action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResilienceDecision {
    /// Execution/result satisfies the applicable acceptance requirements.
    Accept,

    /// Execution may continue, but the result is explicitly degraded.
    DegradedAccept,

    /// The current execution should be repeated according to the supplied
    /// policy/planning state.
    Repeat,

    /// A higher-level operator or policy authority must decide what happens
    /// next.
    Escalate,

    /// The result/execution must not be accepted.
    Reject,
}

impl ResilienceDecision {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Accept => "accept",
            Self::DegradedAccept => "degraded_accept",
            Self::Repeat => "repeat",
            Self::Escalate => "escalate",
            Self::Reject => "reject",
        }
    }

    /// Returns whether the decision constitutes an accepted result.
    pub const fn is_accepted(self) -> bool {
        matches!(self, Self::Accept | Self::DegradedAccept)
    }

    /// Returns whether the decision requires another resilience cycle.
    pub const fn requires_repeat(self) -> bool {
        matches!(self, Self::Repeat)
    }

    /// Returns whether external escalation is required.
    pub const fn requires_escalation(self) -> bool {
        matches!(self, Self::Escalate)
    }
}

impl fmt::Display for ResilienceDecision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Cycle identifier
// =============================================================================

/// Opaque identifier for one resilience orchestration cycle.
///
/// The controller does not generate identifiers because identifier generation
/// is an observability/provenance concern and must not introduce hidden
/// nondeterminism.
///
/// The runtime/request/context supplies the value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResilienceCycleId(u64);

impl ResilienceCycleId {
    /// Creates a cycle identifier from an externally supplied stable value.
    ///
    /// No machine-size or execution-count assumption is encoded here.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying stable identifier.
    pub const fn get(self) -> u64 {
        self.0
    }
}

// =============================================================================
// Controller statistics
// =============================================================================

/// Immutable summary of the work performed by one controller invocation.
///
/// This is intentionally a compact summary. Detailed telemetry belongs to
/// `quantum::resilience::telemetry`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ResilienceCycleSummary {
    /// Number of lifecycle phases successfully entered.
    phases_completed: u64,

    /// Number of adaptation operations requested by the orchestration
    /// contract.
    adaptations_requested: u64,

    /// Number of recovery operations requested by the orchestration contract.
    recoveries_requested: u64,

    /// Number of mitigation operations requested by the orchestration
    /// contract.
    mitigations_requested: u64,

    /// Number of verification operations performed.
    verifications_performed: u64,
}

impl ResilienceCycleSummary {
    /// Creates an empty summary.
    pub const fn new() -> Self {
        Self {
            phases_completed: 0,
            adaptations_requested: 0,
            recoveries_requested: 0,
            mitigations_requested: 0,
            verifications_performed: 0,
        }
    }

    /// Returns the number of completed lifecycle phases.
    pub const fn phases_completed(self) -> u64 {
        self.phases_completed
    }

    /// Returns the number of requested adaptations.
    pub const fn adaptations_requested(self) -> u64 {
        self.adaptations_requested
    }

    /// Returns the number of requested recoveries.
    pub const fn recoveries_requested(self) -> u64 {
        self.recoveries_requested
    }

    /// Returns the number of requested mitigations.
    pub const fn mitigations_requested(self) -> u64 {
        self.mitigations_requested
    }

    /// Returns the number of verification operations.
    pub const fn verifications_performed(self) -> u64 {
        self.verifications_performed
    }

    const fn with_phase_completed(self) -> Self {
        Self {
            phases_completed: self.phases_completed.saturating_add(1),
            ..self
        }
    }

    const fn with_adaptation(self) -> Self {
        Self {
            adaptations_requested: self.adaptations_requested.saturating_add(1),
            ..self
        }
    }

    const fn with_recovery(self) -> Self {
        Self {
            recoveries_requested: self.recoveries_requested.saturating_add(1),
            ..self
        }
    }

    const fn with_mitigation(self) -> Self {
        Self {
            mitigations_requested: self.mitigations_requested.saturating_add(1),
            ..self
        }
    }

    const fn with_verification(self) -> Self {
        Self {
            verifications_performed: self.verifications_performed.saturating_add(1),
            ..self
        }
    }
}

// =============================================================================
// Controller result
// =============================================================================

/// Stable controller result.
///
/// The concrete execution/result/provenance values are intentionally supplied
/// by `ResilienceResponse` rather than duplicated here.
#[derive(Debug)]
pub struct ResilienceControllerResult<R> {
    /// Final resilience decision.
    decision: ResilienceDecision,

    /// Cycle identifier supplied by the caller.
    cycle_id: ResilienceCycleId,

    /// Controller summary.
    summary: ResilienceCycleSummary,

    /// Response produced by the orchestration context.
    response: R,
}

impl<R> ResilienceControllerResult<R> {
    /// Creates a controller result.
    pub fn new(
        cycle_id: ResilienceCycleId,
        decision: ResilienceDecision,
        summary: ResilienceCycleSummary,
        response: R,
    ) -> Self {
        Self {
            decision,
            cycle_id,
            summary,
            response,
        }
    }

    /// Returns the final decision.
    pub const fn decision(&self) -> ResilienceDecision {
        self.decision
    }

    /// Returns the cycle identifier.
    pub const fn cycle_id(&self) -> ResilienceCycleId {
        self.cycle_id
    }

    /// Returns the cycle summary.
    pub const fn summary(&self) -> ResilienceCycleSummary {
        self.summary
    }

    /// Borrows the underlying response.
    pub fn response(&self) -> &R {
        &self.response
    }

    /// Consumes the controller result and returns the response.
    pub fn into_response(self) -> R {
        self.response
    }

    /// Returns the response and final decision.
    pub fn into_parts(self) -> (ResilienceDecision, R) {
        (self.decision, self.response)
    }
}

// =============================================================================
// Orchestration contract
// =============================================================================

/// The execution-resilience orchestration contract consumed by the controller.
///
/// This trait is intentionally defined at the API boundary rather than making
/// the controller depend on concrete detector/planner/recovery implementations.
///
/// A production `ResilienceContext` should implement this trait by delegating
/// to the repository's existing:
///
/// - telemetry/observation layer;
/// - detection layer;
/// - diagnosis layer;
/// - policy layer;
/// - planning layer;
/// - adaptation layer;
/// - recovery layer;
/// - mitigation layer;
/// - verification layer.
///
/// The controller therefore remains stable while those implementations evolve.
///
/// # Why this trait is intentionally broad
///
/// The controller is the lifecycle coordinator. The context is the dependency
/// boundary.
///
/// Putting concrete implementations directly into this file would make the
/// public API depend on implementation details and would force controller
/// changes whenever a detector, planner or recovery strategy changes.
///
/// # Determinism
///
/// Implementations should be deterministic when deterministic execution is
/// requested by the supplied `ResilienceRequest`.
///
/// # Error contract
///
/// Implementations MUST return `ResilienceError` rather than silently treating
/// failure as an empty observation or successful recovery.
pub trait ResilienceOrchestration {
    /// Input request type.
    type Request;

    /// Final response type.
    type Response;

    /// Observation produced by the observation stage.
    type Observation;

    /// Detection result.
    type Detection;

    /// Diagnosis result.
    type Diagnosis;

    /// Policy evaluation result.
    type PolicyDecision;

    /// Recovery/adaptation plan.
    type Plan;

    /// Adaptation result.
    type Adaptation;

    /// Recovery result.
    type Recovery;

    /// Mitigation result.
    type Mitigation;

    /// Verification result.
    type Verification;

    /// Returns the cycle identifier associated with the request.
    fn cycle_id(&self, request: &Self::Request) -> Result<ResilienceCycleId, ResilienceError>;

    /// Observe the current execution/environment.
    fn observe(
        &self,
        request: &Self::Request,
    ) -> Result<Self::Observation, ResilienceError>;

    /// Detect faults/anomalies from the observation.
    fn detect(
        &self,
        request: &Self::Request,
        observation: &Self::Observation,
    ) -> Result<Self::Detection, ResilienceError>;

    /// Diagnose detected conditions.
    fn diagnose(
        &self,
        request: &Self::Request,
        observation: &Self::Observation,
        detection: &Self::Detection,
    ) -> Result<Self::Diagnosis, ResilienceError>;

    /// Apply resilience policy to the diagnosis.
    fn evaluate_policy(
        &self,
        request: &Self::Request,
        observation: &Self::Observation,
        detection: &Self::Detection,
        diagnosis: &Self::Diagnosis,
    ) -> Result<Self::PolicyDecision, ResilienceError>;

    /// Produce a feasible recovery/adaptation plan.
    fn plan(
        &self,
        request: &Self::Request,
        observation: &Self::Observation,
        detection: &Self::Detection,
        diagnosis: &Self::Diagnosis,
        policy: &Self::PolicyDecision,
    ) -> Result<Self::Plan, ResilienceError>;

    /// Determine whether the plan requires physical/logical adaptation.
    ///
    /// The default implementation assumes no adaptation is required.
    fn requires_adaptation(&self, _plan: &Self::Plan) -> bool {
        false
    }

    /// Apply an adaptation plan.
    ///
    /// Implementations should delegate to existing routing, scheduling,
    /// compilation, optimization, QEC and backend-selection contracts.
    fn adapt(
        &self,
        request: &Self::Request,
        observation: &Self::Observation,
        diagnosis: &Self::Diagnosis,
        plan: &Self::Plan,
    ) -> Result<Self::Adaptation, ResilienceError>;

    /// Determine whether recovery is required.
    ///
    /// The default implementation assumes that a plan which does not require
    /// adaptation can continue without a separate recovery operation.
    fn requires_recovery(&self, _plan: &Self::Plan) -> bool {
        false
    }

    /// Execute recovery.
    ///
    /// Recovery implementations may perform retry, restart, resume,
    /// rollback, migration or mathematically valid compensation according to
    /// policy and the plan.
    fn recover(
        &self,
        request: &Self::Request,
        observation: &Self::Observation,
        diagnosis: &Self::Diagnosis,
        plan: &Self::Plan,
        adaptation: Option<&Self::Adaptation>,
    ) -> Result<Self::Recovery, ResilienceError>;

    /// Determine whether mitigation is required.
    ///
    /// The default implementation assumes no mitigation is required.
    fn requires_mitigation(&self, _plan: &Self::Plan) -> bool {
        false
    }

    /// Execute the selected mitigation strategy.
    ///
    /// This must delegate to the mitigation subsystem rather than implementing
    /// mitigation algorithms inside the controller.
    fn mitigate(
        &self,
        request: &Self::Request,
        observation: &Self::Observation,
        diagnosis: &Self::Diagnosis,
        plan: &Self::Plan,
        adaptation: Option<&Self::Adaptation>,
        recovery: Option<&Self::Recovery>,
    ) -> Result<Self::Mitigation, ResilienceError>;

    /// Verify the resulting execution.
    ///
    /// Verification is mandatory at the controller boundary. The controller
    /// does not accept a result without this call.
    fn verify(
        &self,
        request: &Self::Request,
        observation: &Self::Observation,
        detection: &Self::Detection,
        diagnosis: &Self::Diagnosis,
        policy: &Self::PolicyDecision,
        plan: &Self::Plan,
        adaptation: Option<&Self::Adaptation>,
        recovery: Option<&Self::Recovery>,
        mitigation: Option<&Self::Mitigation>,
    ) -> Result<Self::Verification, ResilienceError>;

    /// Convert the verified lifecycle state into the final public response.
    fn build_response(
        &self,
        request: &Self::Request,
        observation: &Self::Observation,
        detection: &Self::Detection,
        diagnosis: &Self::Diagnosis,
        policy: &Self::PolicyDecision,
        plan: &Self::Plan,
        adaptation: Option<&Self::Adaptation>,
        recovery: Option<&Self::Recovery>,
        mitigation: Option<&Self::Mitigation>,
        verification: &Self::Verification,
    ) -> Result<Self::Response, ResilienceError>;

    /// Convert verification into the final controller decision.
    ///
    /// This method belongs to the supplied verification/acceptance contract.
    /// The controller deliberately does not invent acceptance thresholds.
    fn decision(
        &self,
        request: &Self::Request,
        verification: &Self::Verification,
    ) -> Result<ResilienceDecision, ResilienceError>;
}

// =============================================================================
// Controller
// =============================================================================

/// Production quantum-resilience controller.
///
/// The controller contains no quantum-provider implementation and no
/// hardware-specific assumptions.
///
/// It is intentionally stateless between calls.
///
/// # Example architecture
///
/// ```text
/// Runtime
///   │
///   ▼
/// ResilienceController
///   │
///   ▼
/// ResilienceOrchestration
///   ├── observe
///   ├── detect
///   ├── diagnose
///   ├── policy
///   ├── plan
///   ├── adapt
///   ├── recover
///   ├── mitigate
///   ├── verify
///   └── response
/// ```
///
/// # Why stateless
///
/// Stateful recovery history belongs to the repository's `state`/`history`
/// subsystems. Persisting it inside the controller would create hidden state,
/// make deterministic replay harder and couple the controller to storage.
///
/// The controller therefore receives all required state through its request
/// and orchestration context.
#[derive(Debug, Default, Clone, Copy)]
pub struct ResilienceController;

impl ResilienceController {
    /// Creates a new stateless controller.
    pub const fn new() -> Self {
        Self
    }

    /// Executes one complete resilience lifecycle.
    ///
    /// This is the principal public orchestration operation.
    ///
    /// The lifecycle is:
    ///
    /// ```text
    /// observe
    ///   ↓
    /// detect
    ///   ↓
    /// diagnose
    ///   ↓
    /// policy
    ///   ↓
    /// plan
    ///   ↓
    /// adapt? ─────────┐
    ///   ↓              │
    /// recover?         │
    ///   ↓              │
    /// mitigate?        │
    ///   ↓              │
    /// verify           │
    ///   ↓              │
    /// decision         │
    ///   ↓              │
    /// response         │
    /// ```
    ///
    /// The controller performs at most one orchestration cycle per call.
    ///
    /// It does NOT implement an implicit unbounded or fixed retry loop.
    /// Repetition is explicitly controlled by the caller's policy/planning
    /// layer.
    ///
    /// # Safety properties
    ///
    /// - No result is accepted without verification.
    /// - No adaptation is performed unless the plan requests it.
    /// - No recovery is performed unless the plan requests it.
    /// - No mitigation is performed unless the plan requests it.
    /// - No policy is bypassed.
    /// - No hard-coded retry count exists.
    /// - No hard-coded quantum-resource count exists.
    /// - All lower-level failures are propagated.
    pub fn execute<O>(
        &self,
        orchestration: &O,
        request: &O::Request,
    ) -> Result<ResilienceControllerResult<O::Response>, ResilienceError>
    where
        O: ResilienceOrchestration,
    {
        let cycle_id = orchestration.cycle_id(request)?;

        let mut summary = ResilienceCycleSummary::new();

        // ---------------------------------------------------------------------
        // OBSERVE
        // ---------------------------------------------------------------------

        let observation = orchestration.observe(request)?;
        summary = summary.with_phase_completed();

        // ---------------------------------------------------------------------
        // DETECT
        // ---------------------------------------------------------------------

        let detection = orchestration.detect(request, &observation)?;
        summary = summary.with_phase_completed();

        // ---------------------------------------------------------------------
        // DIAGNOSE
        // ---------------------------------------------------------------------

        let diagnosis =
            orchestration.diagnose(request, &observation, &detection)?;
        summary = summary.with_phase_completed();

        // ---------------------------------------------------------------------
        // POLICY
        // ---------------------------------------------------------------------

        let policy = orchestration.evaluate_policy(
            request,
            &observation,
            &detection,
            &diagnosis,
        )?;
        summary = summary.with_phase_completed();

        // ---------------------------------------------------------------------
        // PLAN
        // ---------------------------------------------------------------------

        let plan = orchestration.plan(
            request,
            &observation,
            &detection,
            &diagnosis,
            &policy,
        )?;
        summary = summary.with_phase_completed();

        // ---------------------------------------------------------------------
        // ADAPT
        // ---------------------------------------------------------------------

        let adaptation = if orchestration.requires_adaptation(&plan) {
            let result = orchestration.adapt(
                request,
                &observation,
                &diagnosis,
                &plan,
            )?;

            summary = summary.with_phase_completed();
            summary = summary.with_adaptation();

            Some(result)
        } else {
            None
        };

        // ---------------------------------------------------------------------
        // RECOVER
        // ---------------------------------------------------------------------

        let recovery = if orchestration.requires_recovery(&plan) {
            let result = orchestration.recover(
                request,
                &observation,
                &diagnosis,
                &plan,
                adaptation.as_ref(),
            )?;

            summary = summary.with_phase_completed();
            summary = summary.with_recovery();

            Some(result)
        } else {
            None
        };

        // ---------------------------------------------------------------------
        // MITIGATE
        // ---------------------------------------------------------------------

        let mitigation = if orchestration.requires_mitigation(&plan) {
            let result = orchestration.mitigate(
                request,
                &observation,
                &diagnosis,
                &plan,
                adaptation.as_ref(),
                recovery.as_ref(),
            )?;

            summary = summary.with_phase_completed();
            summary = summary.with_mitigation();

            Some(result)
        } else {
            None
        };

        // ---------------------------------------------------------------------
        // VERIFY
        // ---------------------------------------------------------------------

        //
        // Verification is mandatory even when no adaptation, recovery or
        // mitigation was necessary.
        //
        let verification = orchestration.verify(
            request,
            &observation,
            &detection,
            &diagnosis,
            &policy,
            &plan,
            adaptation.as_ref(),
            recovery.as_ref(),
            mitigation.as_ref(),
        )?;

        summary = summary.with_phase_completed();
        summary = summary.with_verification();

        // ---------------------------------------------------------------------
        // DECIDE
        // ---------------------------------------------------------------------

        let decision = orchestration.decision(request, &verification)?;
        summary = summary.with_phase_completed();

        // ---------------------------------------------------------------------
        // RESPONSE
        // ---------------------------------------------------------------------

        let response = orchestration.build_response(
            request,
            &observation,
            &detection,
            &diagnosis,
            &policy,
            &plan,
            adaptation.as_ref(),
            recovery.as_ref(),
            mitigation.as_ref(),
            &verification,
        )?;

        Ok(ResilienceControllerResult::new(
            cycle_id,
            decision,
            summary,
            response,
        ))
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default)]
    struct MockOrchestration {
        cycle: ResilienceCycleId,
        adaptation: bool,
        recovery: bool,
        mitigation: bool,
    }

    impl ResilienceOrchestration for MockOrchestration {
        type Request = ();
        type Response = &'static str;
        type Observation = ();
        type Detection = ();
        type Diagnosis = ();
        type PolicyDecision = ();
        type Plan = ();
        type Adaptation = ();
        type Recovery = ();
        type Mitigation = ();
        type Verification = ();

        fn cycle_id(
            &self,
            _request: &Self::Request,
        ) -> Result<ResilienceCycleId, ResilienceError> {
            Ok(self.cycle)
        }

        fn observe(
            &self,
            _request: &Self::Request,
        ) -> Result<Self::Observation, ResilienceError> {
            Ok(())
        }

        fn detect(
            &self,
            _request: &Self::Request,
            _observation: &Self::Observation,
        ) -> Result<Self::Detection, ResilienceError> {
            Ok(())
        }

        fn diagnose(
            &self,
            _request: &Self::Request,
            _observation: &Self::Observation,
            _detection: &Self::Detection,
        ) -> Result<Self::Diagnosis, ResilienceError> {
            Ok(())
        }

        fn evaluate_policy(
            &self,
            _request: &Self::Request,
            _observation: &Self::Observation,
            _detection: &Self::Detection,
            _diagnosis: &Self::Diagnosis,
        ) -> Result<Self::PolicyDecision, ResilienceError> {
            Ok(())
        }

        fn plan(
            &self,
            _request: &Self::Request,
            _observation: &Self::Observation,
            _detection: &Self::Detection,
            _diagnosis: &Self::Diagnosis,
            _policy: &Self::PolicyDecision,
        ) -> Result<Self::Plan, ResilienceError> {
            Ok(())
        }

        fn requires_adaptation(&self, _plan: &Self::Plan) -> bool {
            self.adaptation
        }

        fn adapt(
            &self,
            _request: &Self::Request,
            _observation: &Self::Observation,
            _diagnosis: &Self::Diagnosis,
            _plan: &Self::Plan,
        ) -> Result<Self::Adaptation, ResilienceError> {
            Ok(())
        }

        fn requires_recovery(&self, _plan: &Self::Plan) -> bool {
            self.recovery
        }

        fn recover(
            &self,
            _request: &Self::Request,
            _observation: &Self::Observation,
            _diagnosis: &Self::Diagnosis,
            _plan: &Self::Plan,
            _adaptation: Option<&Self::Adaptation>,
        ) -> Result<Self::Recovery, ResilienceError> {
            Ok(())
        }

        fn requires_mitigation(&self, _plan: &Self::Plan) -> bool {
            self.mitigation
        }

        fn mitigate(
            &self,
            _request: &Self::Request,
            _observation: &Self::Observation,
            _diagnosis: &Self::Diagnosis,
            _plan: &Self::Plan,
            _adaptation: Option<&Self::Adaptation>,
            _recovery: Option<&Self::Recovery>,
        ) -> Result<Self::Mitigation, ResilienceError> {
            Ok(())
        }

        fn verify(
            &self,
            _request: &Self::Request,
            _observation: &Self::Observation,
            _detection: &Self::Detection,
            _diagnosis: &Self::Diagnosis,
            _policy: &Self::PolicyDecision,
            _plan: &Self::Plan,
            _adaptation: Option<&Self::Adaptation>,
            _recovery: Option<&Self::Recovery>,
            _mitigation: Option<&Self::Mitigation>,
        ) -> Result<Self::Verification, ResilienceError> {
            Ok(())
        }

        fn build_response(
            &self,
            _request: &Self::Request,
            _observation: &Self::Observation,
            _detection: &Self::Detection,
            _diagnosis: &Self::Diagnosis,
            _policy: &Self::PolicyDecision,
            _plan: &Self::Plan,
            _adaptation: Option<&Self::Adaptation>,
            _recovery: Option<&Self::Recovery>,
            _mitigation: Option<&Self::Mitigation>,
            _verification: &Self::Verification,
        ) -> Result<Self::Response, ResilienceError> {
            Ok("verified")
        }

        fn decision(
            &self,
            _request: &Self::Request,
            _verification: &Self::Verification,
        ) -> Result<ResilienceDecision, ResilienceError> {
            Ok(ResilienceDecision::Accept)
        }
    }

    #[test]
    fn controller_is_constructible_without_runtime_state() {
        let controller = ResilienceController::new();
        let _ = controller;
    }

    #[test]
    fn cycle_id_is_stable() {
        let id = ResilienceCycleId::new(42);
        assert_eq!(id.get(), 42);
    }

    #[test]
    fn decision_acceptance_semantics_are_stable() {
        assert!(ResilienceDecision::Accept.is_accepted());
        assert!(ResilienceDecision::DegradedAccept.is_accepted());
        assert!(!ResilienceDecision::Repeat.is_accepted());
        assert!(!ResilienceDecision::Escalate.is_accepted());
        assert!(!ResilienceDecision::Reject.is_accepted());
    }

    #[test]
    fn phase_names_are_stable() {
        assert_eq!(ResiliencePhase::Observe.as_str(), "observe");
        assert_eq!(ResiliencePhase::Detect.as_str(), "detect");
        assert_eq!(ResiliencePhase::Diagnose.as_str(), "diagnose");
        assert_eq!(ResiliencePhase::Policy.as_str(), "policy");
        assert_eq!(ResiliencePhase::Plan.as_str(), "plan");
        assert_eq!(ResiliencePhase::Adapt.as_str(), "adapt");
        assert_eq!(ResiliencePhase::Recover.as_str(), "recover");
        assert_eq!(ResiliencePhase::Mitigate.as_str(), "mitigate");
        assert_eq!(ResiliencePhase::Verify.as_str(), "verify");
        assert_eq!(ResiliencePhase::Decide.as_str(), "decide");
    }

    #[test]
    fn controller_executes_without_optional_stages() {
        let orchestration = MockOrchestration {
            cycle: ResilienceCycleId::new(1),
            adaptation: false,
            recovery: false,
            mitigation: false,
        };

        let controller = ResilienceController::new();

        let result = controller
            .execute(&orchestration, &())
            .expect("controller execution should succeed");

        assert_eq!(result.cycle_id().get(), 1);
        assert_eq!(result.decision(), ResilienceDecision::Accept);
        assert_eq!(result.response(), &"verified");
        assert_eq!(result.summary().adaptations_requested(), 0);
        assert_eq!(result.summary().recoveries_requested(), 0);
        assert_eq!(result.summary().mitigations_requested(), 0);
        assert_eq!(result.summary().verifications_performed(), 1);
    }

    #[test]
    fn controller_executes_all_optional_stages() {
        let orchestration = MockOrchestration {
            cycle: ResilienceCycleId::new(2),
            adaptation: true,
            recovery: true,
            mitigation: true,
        };

        let controller = ResilienceController::new();

        let result = controller
            .execute(&orchestration, &())
            .expect("controller execution should succeed");

        assert_eq!(result.cycle_id().get(), 2);
        assert_eq!(result.decision(), ResilienceDecision::Accept);
        assert_eq!(result.summary().adaptations_requested(), 1);
        assert_eq!(result.summary().recoveries_requested(), 1);
        assert_eq!(result.summary().mitigations_requested(), 1);
        assert_eq!(result.summary().verifications_performed(), 1);
    }

    #[test]
    fn summary_is_zero_initialized() {
        let summary = ResilienceCycleSummary::new();

        assert_eq!(summary.phases_completed(), 0);
        assert_eq!(summary.adaptations_requested(), 0);
        assert_eq!(summary.recoveries_requested(), 0);
        assert_eq!(summary.mitigations_requested(), 0);
        assert_eq!(summary.verifications_performed(), 0);
    }
}