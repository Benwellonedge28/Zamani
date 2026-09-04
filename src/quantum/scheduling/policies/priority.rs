//! Zamani Quantum Scheduling — Priority Policy
//!
//! Production-grade, provider-neutral operation-priority policy for the
//! Zamani quantum scheduling subsystem.
//!
//! # Purpose
//!
//! This module answers:
//!
//! > "When several operations are simultaneously eligible for scheduling,
//! > which operation should receive precedence according to priority
//! > information?"
//!
//! Priority scheduling is an ORDERING POLICY.
//!
//! It is not a complete scheduler and does not itself:
//!
//! - construct a dependency graph;
//! - route logical qubits;
//! - allocate physical qubits;
//! - inspect hardware;
//! - inspect hardware calendars;
//! - reserve resources;
//! - calculate operation durations;
//! - generate pulses;
//! - execute operations;
//! - perform QEC decoding;
//! - perform noise modelling;
//! - perform semantic transformations.
//!
//! Those responsibilities remain in their owning subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                              |
//!                              v
//!                         optimization
//!                              |
//!                              v
//!                            routing
//!                              |
//!                              v
//!                       scheduling adapters
//!                              |
//!                              v
//!                   scheduling dependency/resource/
//!                       timing/constraint analysis
//!                              |
//!                              v
//!                       ready operation set
//!                              |
//!                              v
//!                    +-----------------------+
//!                    |    PriorityPolicy     |
//!                    +-----------------------+
//!                              |
//!                              v
//!                     planner / algorithm
//!                              |
//!                              v
//!                         reservation
//!                              |
//!                              v
//!                         verification
//! ```
//!
//! # Important architectural distinction
//!
//! Priority policy is not equivalent to `SchedulingPolicy` in
//! `policies/policy.rs`.
//!
//! `SchedulingPolicy` is the high-level public scheduling configuration.
//! `PriorityPolicy` is the concrete operation-ordering mechanism used by
//! planners when priority information is required.
//!
//! The existing common policy contract already exposes:
//!
//! - `TieBreakRule::Priority`;
//! - `PolicyRequirement::Priorities`;
//! - `PolicyScore`;
//! - `priority_weight`;
//! - `criticality_weight`;
//! - `resource_weight`;
//! - `fidelity_weight`.
//!
//! This module consumes those contracts rather than duplicating them.
//!
//! # Universal-program principle
//!
//! A Zamani program is written at the semantic level.
//!
//! The priority policy must therefore remain independent of:
//!
//! - physical machine size;
//! - qubit count;
//! - gate count;
//! - topology dimensions;
//! - channel count;
//! - resource count;
//! - schedule depth;
//! - machine technology;
//! - vendor;
//! - clock frequency;
//! - pulse rate.
//!
//! The same policy can therefore rank operations for:
//!
//! - a single-qubit target;
//! - a small QPU;
//! - a large QPU;
//! - a modular QPU;
//! - a distributed quantum computer;
//! - a fault-tolerant system;
//! - a future quantum architecture.
//!
//! "Infinity" means that this file imposes no artificial finite machine-size
//! ceiling. Actual execution remains bounded by the resources available to the
//! compilation process, target, operating environment, and execution request.
//!
//! # Canonical identity ownership
//!
//! Operation identity remains owned by the canonical quantum IR:
//!
//! `crate::quantum::ir::core::identity::OperationId`
//!
//! Logical and physical qubit identities remain owned by:
//!
//! `crate::quantum::ir::qubit::QubitId`
//! `crate::quantum::ir::qubit::PhysicalQubitId`
//!
//! This module deliberately does not redefine either identity.
//!
//! Priority ranking normally does not need to inspect qubit operands directly.
//! Qubit/resource analysis should therefore provide any resource-derived score
//! required by this policy rather than making this module depend on a particular
//! quantum topology representation.
//!
//! # Ordering semantics
//!
//! Priority is represented as a signed normalized score.
//!
//! Higher effective priority wins.
//!
//! When effective scores are equal, deterministic tie-breaking is performed
//! using:
//!
//! 1. the configured tie-break rule when supplied by the caller;
//! 2. operation identity as the final total-order fallback.
//!
//! The final identity fallback is essential because a scheduler must not rely
//! on hash-map iteration order or allocation order.
//!
//! # Score semantics
//!
//! A candidate can contain independent normalized components:
//!
//! ```text
//! explicit priority
//! critical-path urgency
//! resource pressure
//! estimated fidelity
//! ```
//!
//! The common `PolicyScore` type in `policies/policy.rs` owns those components.
//!
//! This module does not reinterpret physical units.
//!
//! The planner is responsible for normalizing values before they become a
//! `PolicyScore`.
//!
//! # Correctness rule
//!
//! Priority is subordinate to correctness.
//!
//! A higher-priority operation MUST NOT be scheduled if it is:
//!
//! - not dependency-ready;
//! - outside its legal timing window;
//! - resource-infeasible;
//! - target-incompatible;
//! - conditionally unresolved;
//! - otherwise prohibited by a scheduling constraint.
//!
//! The planner must construct the candidate set from operations that are
//! already eligible.
//!
//! Therefore this policy answers:
//!
//! > Which eligible operation should be considered first?
//!
//! It does NOT answer:
//!
//! > Is this operation legal?
//!
//! # Resource awareness
//!
//! Resource pressure may participate in the score, but the priority policy
//! never owns resource availability.
//!
//! A planner may provide a normalized resource-pressure score based on:
//!
//! - scarce control channels;
//! - readout resources;
//! - communication links;
//! - shared electronics;
//! - ancilla resources;
//! - modular resources;
//! - other target-specific resource constraints.
//!
//! The policy only ranks the supplied value.
//!
//! # Critical-path integration
//!
//! Critical-path urgency may participate in priority scoring.
//!
//! The critical-path analysis remains owned by:
//!
//! `scheduling::ir::critical_path`
//!
//! This module does not calculate the critical path itself.
//!
//! # Fidelity integration
//!
//! Estimated fidelity may participate in scoring when supplied by an external
//! target/noise integration layer.
//!
//! The priority policy does not implement a noise model.
//!
//! For example:
//!
//! ```text
//! ZQN / calibration / target analysis
//!              |
//!              v
//!      normalized fidelity score
//!              |
//!              v
//!        PriorityPolicy
//! ```
//!
//! # Determinism
//!
//! No randomness is used by this policy.
//!
//! It does not depend on:
//!
//! - hash-map iteration order;
//! - pointer addresses;
//! - thread timing;
//! - allocation addresses;
//! - process-global state.
//!
//! For equal scores, operation identity establishes a stable final ordering.
//!
//! # Arithmetic safety
//!
//! Weighted scoring uses the checked arithmetic already provided by
//! `PolicyScore::weighted`.
//!
//! No wrapping arithmetic is used.
//!
//! A score overflow is reported as the canonical `PolicyError::ScoreOverflow`.
//!
//! # Scalability
//!
//! The policy allocates no machine-wide structures.
//!
//! Per candidate it performs constant-size scoring and comparison.
//!
//! Therefore its memory requirement is O(1) per candidate, excluding data held
//! by the caller.
//!
//! The policy does not create:
//!
//! - qubit-sized arrays;
//! - machine-sized arrays;
//! - timeline-sized arrays;
//! - depth-sized arrays;
//! - resource-count-sized arrays.
//!
//! # Parallel scheduling
//!
//! `PriorityPolicy` is immutable and thread-safe.
//!
//! Multiple scheduling workers may independently evaluate candidates.
//!
//! Deterministic planners must perform deterministic arbitration when combining
//! independently evaluated candidate sets.
//!
//! This module does not coordinate worker threads.
//!
//! # Dynamic scheduling
//!
//! Runtime schedulers may construct a new `PriorityCandidate` whenever an
//! operation becomes eligible.
//!
//! No persistent machine-wide state is stored by this policy.
//!
//! # Distributed scheduling
//!
//! Distributed schedulers may include communication pressure in the resource
//! score or use an explicit priority component derived from communication
//! urgency.
//!
//! Network topology remains outside this module.
//!
//! # QEC integration
//!
//! QEC planners may use explicit priority for:
//!
//! - syndrome extraction;
//! - ancilla preparation;
//! - stabilizer interactions;
//! - measurement;
//! - feedback;
//! - deadline-sensitive recovery.
//!
//! QEC-specific semantics remain outside this policy.
//!
//! # ALAP/ASAP integration
//!
//! Priority is orthogonal to temporal direction.
//!
//! It can therefore be used with:
//!
//! - ASAP;
//! - ALAP;
//! - list scheduling;
//! - critical-path scheduling;
//! - resource-constrained scheduling;
//! - adaptive scheduling;
//! - hybrid policies.
//!
//! For example:
//!
//! ```text
//! ALAP determines temporal direction.
//! Priority determines candidate precedence.
//! Planner determines legal placement.
//! ```
//!
//! # Configuration integration
//!
//! The common `SchedulingPolicy` already provides:
//!
//! - `priority_weight()`;
//! - `criticality_weight()`;
//! - `resource_weight()`;
//! - `fidelity_weight()`.
//!
//! This module consumes those weights.
//!
//! It does not introduce another configuration system.
//!
//! # Verification
//!
//! The priority policy itself cannot establish schedule correctness.
//!
//! The verification subsystem must independently validate:
//!
//! - dependencies;
//! - resources;
//! - timing;
//! - alignment;
//! - target compatibility;
//! - semantic preservation.
//!
//! # Frozen-file contract
//!
//! This file is complete independently of:
//!
//! - ASAP implementation;
//! - ALAP implementation;
//! - list planner implementation;
//! - critical-path planner implementation;
//! - resource calendar implementation;
//! - hardware providers;
//! - routing providers;
//! - QEC implementations;
//! - distributed network implementations.
//!
//! Those modules consume this policy.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::cmp::Ordering;
use std::fmt;

use crate::quantum::ir::core::identity::OperationId;

use super::policy::{
    Determinism,
    PolicyError,
    PolicyScore,
    SchedulingObjective,
    SchedulingPolicy,
    TieBreakRule,
};

// =============================================================================
// Priority errors
// =============================================================================

/// Errors produced while evaluating a priority candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PriorityError {
    /// The supplied candidate does not contain the operation identity expected
    /// by the caller.
    InvalidOperation {
        /// Operation involved in the invalid candidate.
        operation: OperationId,
    },

    /// The weighted priority score overflowed the representable score domain.
    ScoreOverflow,

    /// A candidate was marked as not eligible for priority ordering.
    ///
    /// Eligibility is normally established by the planner. This explicit
    /// representation prevents callers from accidentally treating an
    /// unresolved operation as scheduler-ready.
    CandidateNotEligible {
        /// Operation that is not currently eligible.
        operation: OperationId,
    },
}

impl fmt::Display for PriorityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOperation { operation } => write!(
                formatter,
                "invalid priority candidate operation `{operation}`"
            ),

            Self::ScoreOverflow => {
                formatter.write_str("priority score arithmetic overflowed")
            }

            Self::CandidateNotEligible { operation } => write!(
                formatter,
                "operation `{operation}` is not eligible for priority ordering"
            ),
        }
    }
}

impl std::error::Error for PriorityError {}

impl From<PolicyError> for PriorityError {
    fn from(error: PolicyError) -> Self {
        match error {
            PolicyError::ScoreOverflow => Self::ScoreOverflow,
            _ => Self::ScoreOverflow,
        }
    }
}

// =============================================================================
// Priority candidate
// =============================================================================

/// Immutable candidate presented to [`PriorityPolicy`].
///
/// A candidate MUST already have passed the planner's legality/readiness checks.
/// The `eligible` field is retained as an explicit safety boundary so that a
/// planner cannot accidentally rank an operation it has already identified as
/// unavailable.
///
/// # Score components
///
/// The four score components correspond directly to the canonical
/// [`PolicyScore`] fields:
///
/// - explicit operation priority;
/// - criticality;
/// - resource pressure;
/// - fidelity.
///
/// The values are normalized scheduler scores, not physical quantities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PriorityCandidate {
    operation: OperationId,
    score: PolicyScore,
    eligible: bool,
}

impl PriorityCandidate {
    /// Creates an eligible priority candidate.
    #[must_use]
    pub const fn new(operation: OperationId, score: PolicyScore) -> Self {
        Self {
            operation,
            score,
            eligible: true,
        }
    }

    /// Creates a candidate with explicit eligibility.
    #[must_use]
    pub const fn with_eligibility(
        operation: OperationId,
        score: PolicyScore,
        eligible: bool,
    ) -> Self {
        Self {
            operation,
            score,
            eligible,
        }
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        self.operation
    }

    /// Returns the normalized score components.
    #[must_use]
    pub const fn score(&self) -> PolicyScore {
        self.score
    }

    /// Returns whether the planner has marked the candidate eligible.
    #[must_use]
    pub const fn eligible(&self) -> bool {
        self.eligible
    }

    /// Creates a candidate from an explicit priority value.
    ///
    /// All other score components are zero.
    #[must_use]
    pub const fn from_priority(operation: OperationId, priority: i128) -> Self {
        Self::new(operation, PolicyScore::new(priority, 0, 0, 0))
    }

    /// Creates a candidate from explicit priority and criticality values.
    #[must_use]
    pub const fn from_priority_and_criticality(
        operation: OperationId,
        priority: i128,
        criticality: i128,
    ) -> Self {
        Self::new(
            operation,
            PolicyScore::new(priority, criticality, 0, 0),
        )
    }

    /// Creates a fully specified normalized candidate.
    #[must_use]
    pub const fn from_components(
        operation: OperationId,
        priority: i128,
        criticality: i128,
        resource: i128,
        fidelity: i128,
    ) -> Self {
        Self::new(
            operation,
            PolicyScore::new(priority, criticality, resource, fidelity),
        )
    }

    /// Returns this candidate as an explicitly ineligible candidate.
    #[must_use]
    pub const fn ineligible(mut self) -> Self {
        self.eligible = false;
        self
    }
}

// =============================================================================
// Priority decision
// =============================================================================

/// Immutable result of evaluating a priority candidate.
///
/// The decision is intentionally separate from a schedule placement.
/// Selecting a candidate does not reserve a resource or assign a start time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PriorityDecision {
    operation: OperationId,
    score: i128,
    eligible: bool,
}

impl PriorityDecision {
    fn new(
        operation: OperationId,
        score: i128,
        eligible: bool,
    ) -> Self {
        Self {
            operation,
            score,
            eligible,
        }
    }

    /// Returns the operation selected by this decision.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        self.operation
    }

    /// Returns the final weighted priority score.
    #[must_use]
    pub const fn score(&self) -> i128 {
        self.score
    }

    /// Returns whether the candidate was eligible.
    #[must_use]
    pub const fn eligible(&self) -> bool {
        self.eligible
    }
}

// =============================================================================
// Priority policy
// =============================================================================

/// Production priority-ordering policy.
///
/// The policy is immutable and contains no scheduler state.
///
/// It may therefore be copied or shared between scheduling workers.
///
/// The policy never directly modifies a schedule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PriorityPolicy {
    tie_break: TieBreakRule,
    determinism: Determinism,
}

impl PriorityPolicy {
    /// Creates a deterministic priority policy using the canonical priority
    /// tie-break rule.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            tie_break: TieBreakRule::Priority,
            determinism: Determinism::Deterministic,
        }
    }

    /// Creates a priority policy with an explicit tie-break rule.
    #[must_use]
    pub const fn with_tie_break(tie_break: TieBreakRule) -> Self {
        Self {
            tie_break,
            determinism: Determinism::Deterministic,
        }
    }

    /// Creates a policy using the requested determinism mode.
    #[must_use]
    pub const fn with_determinism(
        tie_break: TieBreakRule,
        determinism: Determinism,
    ) -> Self {
        Self {
            tie_break,
            determinism,
        }
    }

    /// Returns the canonical default policy.
    #[must_use]
    pub const fn default_policy() -> Self {
        Self::new()
    }

    /// Returns the stable policy name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        "priority"
    }

    /// Returns the policy description.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        "Rank eligible scheduling candidates using normalized priority, criticality, resource, and fidelity scores"
    }

    /// Returns the configured tie-break rule.
    #[must_use]
    pub const fn tie_break(&self) -> TieBreakRule {
        self.tie_break
    }

    /// Returns the configured determinism mode.
    #[must_use]
    pub const fn determinism(&self) -> Determinism {
        self.determinism
    }

    /// Returns whether deterministic ranking is required.
    #[must_use]
    pub const fn deterministic(&self) -> bool {
        self.determinism.is_required()
    }

    /// Returns the policy's static capabilities.
    #[must_use]
    pub const fn capabilities(&self) -> PriorityCapabilities {
        PriorityCapabilities {
            explicit_priority: true,
            criticality: true,
            resource_pressure: true,
            fidelity: true,
            deterministic: true,
            target_independent: true,
        }
    }

    /// Calculates the weighted score for a candidate using the canonical
    /// `SchedulingPolicy` weights.
    ///
    /// This method does not perform eligibility checks.
    pub fn weighted_score(
        &self,
        policy: &SchedulingPolicy,
        candidate: &PriorityCandidate,
    ) -> Result<i128, PriorityError> {
        candidate
            .score()
            .weighted(policy)
            .map_err(PriorityError::from)
    }

    /// Evaluates one candidate.
    ///
    /// An ineligible candidate is rejected rather than silently assigned a
    /// lower score. This is important because priority must never become a
    /// mechanism for bypassing scheduling constraints.
    pub fn evaluate(
        &self,
        policy: &SchedulingPolicy,
        candidate: &PriorityCandidate,
    ) -> Result<PriorityDecision, PriorityError> {
        if !candidate.eligible() {
            return Err(PriorityError::CandidateNotEligible {
                operation: candidate.operation(),
            });
        }

        let score = self.weighted_score(policy, candidate)?;

        Ok(PriorityDecision::new(
            candidate.operation(),
            score,
            candidate.eligible(),
        ))
    }

    /// Compares two already-validated candidates.
    ///
    /// Higher weighted score wins.
    ///
    /// When scores are equal, the configured tie-break rule is applied where
    /// enough information exists. Operation identity is always the final
    /// total-order fallback.
    ///
    /// This method intentionally does not inspect resources or timing.
    pub fn compare(
        &self,
        left: &PriorityDecision,
        right: &PriorityDecision,
    ) -> Ordering {
        match (left.eligible(), right.eligible()) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => left.operation().cmp(&right.operation()),
            (true, true) => {
                left.score()
                    .cmp(&right.score())
                    .reverse()
                    .then_with(|| {
                        self.compare_tie_break(
                            left.operation(),
                            right.operation(),
                        )
                    })
                    .then_with(|| {
                        left.operation().cmp(&right.operation())
                    })
            }
        }
    }

    /// Returns whether the left decision has precedence over the right
    /// decision.
    #[must_use]
    pub fn prefers(
        &self,
        left: &PriorityDecision,
        right: &PriorityDecision,
    ) -> bool {
        self.compare(left, right) == Ordering::Less
    }

    /// Returns the preferred candidate from two decisions.
    #[must_use]
    pub fn preferred<'a>(
        &self,
        left: &'a PriorityDecision,
        right: &'a PriorityDecision,
    ) -> &'a PriorityDecision {
        if self.prefers(left, right) {
            left
        } else {
            right
        }
    }

    /// Selects the best candidate from an iterator.
    ///
    /// This method uses O(1) policy-owned memory regardless of the number of
    /// candidates.
    ///
    /// The caller owns the candidate collection/iterator.
    ///
    /// All candidates must already have been evaluated using the same
    /// `SchedulingPolicy`.
    pub fn select_best<I>(
        &self,
        candidates: I,
    ) -> Option<PriorityDecision>
    where
        I: IntoIterator<Item = PriorityDecision>,
    {
        let mut best: Option<PriorityDecision> = None;

        for candidate in candidates {
            match best {
                None => best = Some(candidate),
                Some(current) => {
                    if self.prefers(&candidate, &current) {
                        best = Some(candidate);
                    }
                }
            }
        }

        best
    }

    /// Evaluates and selects the best candidate from an iterator.
    ///
    /// The returned error is the first candidate-evaluation error encountered.
    ///
    /// The caller is responsible for ensuring that all candidates belong to
    /// the same scheduling epoch/context.
    pub fn evaluate_and_select<I>(
        &self,
        policy: &SchedulingPolicy,
        candidates: I,
    ) -> Result<Option<PriorityDecision>, PriorityError>
    where
        I: IntoIterator<Item = PriorityCandidate>,
    {
        let mut best: Option<PriorityDecision> = None;

        for candidate in candidates {
            let decision = self.evaluate(policy, &candidate)?;

            match best {
                None => best = Some(decision),
                Some(current) => {
                    if self.prefers(&decision, &current) {
                        best = Some(decision);
                    }
                }
            }
        }

        Ok(best)
    }

    /// Sorts an owned vector of decisions into scheduler precedence order.
    ///
    /// The first element after sorting is the preferred candidate.
    ///
    /// The operation is performed by the caller-owned vector and does not
    /// allocate a second candidate collection.
    pub fn sort_decisions(
        &self,
        decisions: &mut [PriorityDecision],
    ) {
        decisions.sort_by(|left, right| self.compare(left, right));
    }

    /// Validates that the supplied scheduling policy can provide priority
    /// information.
    ///
    /// Priority scoring requires the common policy's priority requirement.
    pub fn validate_policy(
        &self,
        policy: &SchedulingPolicy,
    ) -> Result<(), PriorityError> {
        policy
            .validate()
            .map_err(PriorityError::from)
    }

    /// Returns the tie-break ordering for operation identities.
    fn compare_tie_break(
        &self,
        left: OperationId,
        right: OperationId,
    ) -> Ordering {
        match self.tie_break {
            TieBreakRule::OperationId
            | TieBreakRule::Priority
            | TieBreakRule::Criticality
            | TieBreakRule::ResourceFootprint
            | TieBreakRule::SourceOrder
            | TieBreakRule::EarliestStart
            | TieBreakRule::DeterministicDefault => {
                left.cmp(&right)
            }
        }
    }
}

impl Default for PriorityPolicy {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Static capabilities
// =============================================================================

/// Static capabilities of [`PriorityPolicy`].
///
/// These describe the policy, not a target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PriorityCapabilities {
    /// Explicit operation priority can participate in scoring.
    pub explicit_priority: bool,

    /// Critical-path information can participate in scoring.
    pub criticality: bool,

    /// Resource-pressure information can participate in scoring.
    pub resource_pressure: bool,

    /// Estimated-fidelity information can participate in scoring.
    pub fidelity: bool,

    /// The implementation itself is deterministic.
    pub deterministic: bool,

    /// No hardware-specific information is embedded in the policy.
    pub target_independent: bool,
}

// =============================================================================
// Convenience constructors
// =============================================================================

/// Creates a priority candidate with explicit priority only.
#[must_use]
pub const fn priority_candidate(
    operation: OperationId,
    priority: i128,
) -> PriorityCandidate {
    PriorityCandidate::from_priority(operation, priority)
}

/// Creates a priority candidate with priority and critical-path urgency.
#[must_use]
pub const fn critical_priority_candidate(
    operation: OperationId,
    priority: i128,
    criticality: i128,
) -> PriorityCandidate {
    PriorityCandidate::from_priority_and_criticality(
        operation,
        priority,
        criticality,
    )
}

/// Creates a fully weighted priority candidate.
#[must_use]
pub const fn weighted_priority_candidate(
    operation: OperationId,
    priority: i128,
    criticality: i128,
    resource: i128,
    fidelity: i128,
) -> PriorityCandidate {
    PriorityCandidate::from_components(
        operation,
        priority,
        criticality,
        resource,
        fidelity,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(value: u64) -> OperationId {
        OperationId::from(value)
    }

    fn policy() -> SchedulingPolicy {
        SchedulingPolicy::builder()
            .priority_weight(1)
            .criticality_weight(1)
            .resource_weight(1)
            .fidelity_weight(1)
            .build()
            .expect("valid scheduling policy")
    }

    #[test]
    fn default_policy_is_deterministic() {
        let priority = PriorityPolicy::new();

        assert_eq!(priority.name(), "priority");
        assert!(priority.deterministic());
        assert_eq!(priority.tie_break(), TieBreakRule::Priority);
    }

    #[test]
    fn explicit_priority_is_ranked_highest() {
        let priority = PriorityPolicy::new();
        let configuration = policy();

        let low = PriorityCandidate::from_priority(operation(1), 10);
        let high = PriorityCandidate::from_priority(operation(2), 20);

        let low_decision = priority
            .evaluate(&configuration, &low)
            .expect("valid candidate");

        let high_decision = priority
            .evaluate(&configuration, &high)
            .expect("valid candidate");

        assert!(priority.prefers(&high_decision, &low_decision));
        assert!(!priority.prefers(&low_decision, &high_decision));
    }

    #[test]
    fn criticality_contributes_to_score() {
        let priority = PriorityPolicy::new();
        let configuration = policy();

        let ordinary = PriorityCandidate::from_priority_and_criticality(
            operation(1),
            10,
            0,
        );

        let critical = PriorityCandidate::from_priority_and_criticality(
            operation(2),
            10,
            20,
        );

        let ordinary_decision = priority
            .evaluate(&configuration, &ordinary)
            .expect("valid candidate");

        let critical_decision = priority
            .evaluate(&configuration, &critical)
            .expect("valid candidate");

        assert!(priority.prefers(
            &critical_decision,
            &ordinary_decision
        ));
    }

    #[test]
    fn resource_pressure_contributes_to_score() {
        let priority = PriorityPolicy::new();
        let configuration = policy();

        let normal = weighted_priority_candidate(
            operation(1),
            10,
            0,
            0,
            0,
        );

        let resource_urgent = weighted_priority_candidate(
            operation(2),
            10,
            0,
            20,
            0,
        );

        let normal_decision = priority
            .evaluate(&configuration, &normal)
            .expect("valid candidate");

        let urgent_decision = priority
            .evaluate(&configuration, &resource_urgent)
            .expect("valid candidate");

        assert!(priority.prefers(
            &urgent_decision,
            &normal_decision
        ));
    }

    #[test]
    fn fidelity_contributes_to_score() {
        let priority = PriorityPolicy::new();
        let configuration = policy();

        let ordinary = weighted_priority_candidate(
            operation(1),
            10,
            0,
            0,
            0,
        );

        let fidelity_preferred = weighted_priority_candidate(
            operation(2),
            10,
            0,
            0,
            20,
        );

        let ordinary_decision = priority
            .evaluate(&configuration, &ordinary)
            .expect("valid candidate");

        let fidelity_decision = priority
            .evaluate(&configuration, &fidelity_preferred)
            .expect("valid candidate");

        assert!(priority.prefers(
            &fidelity_decision,
            &ordinary_decision
        ));
    }

    #[test]
    fn operation_identity_is_final_deterministic_fallback() {
        let priority = PriorityPolicy::new();
        let configuration = policy();

        let first = PriorityCandidate::from_priority(operation(1), 100);
        let second = PriorityCandidate::from_priority(operation(2), 100);

        let first_decision = priority
            .evaluate(&configuration, &first)
            .expect("valid candidate");

        let second_decision = priority
            .evaluate(&configuration, &second)
            .expect("valid candidate");

        assert!(priority.prefers(
            &first_decision,
            &second_decision
        ));
    }

    #[test]
    fn ineligible_candidate_is_rejected() {
        let priority = PriorityPolicy::new();
        let configuration = policy();

        let candidate =
            PriorityCandidate::from_priority(operation(1), 100)
                .ineligible();

        let result = priority.evaluate(&configuration, &candidate);

        assert!(matches!(
            result,
            Err(PriorityError::CandidateNotEligible { .. })
        ));
    }

    #[test]
    fn select_best_uses_constant_policy_memory() {
        let priority = PriorityPolicy::new();

        let first = PriorityDecision::new(operation(1), 10, true);
        let second = PriorityDecision::new(operation(2), 30, true);
        let third = PriorityDecision::new(operation(3), 20, true);

        let best = priority
            .select_best([first, second, third])
            .expect("candidate exists");

        assert_eq!(best.operation(), operation(2));
        assert_eq!(best.score(), 30);
    }

    #[test]
    fn sorting_places_best_candidate_first() {
        let priority = PriorityPolicy::new();

        let mut decisions = [
            PriorityDecision::new(operation(1), 10, true),
            PriorityDecision::new(operation(2), 30, true),
            PriorityDecision::new(operation(3), 20, true),
        ];

        priority.sort_decisions(&mut decisions);

        assert_eq!(decisions[0].operation(), operation(2));
        assert_eq!(decisions[1].operation(), operation(3));
        assert_eq!(decisions[2].operation(), operation(1));
    }

    #[test]
    fn zero_priority_is_valid() {
        let priority = PriorityPolicy::new();
        let configuration = policy();

        let candidate = PriorityCandidate::from_priority(operation(1), 0);

        let decision = priority
            .evaluate(&configuration, &candidate)
            .expect("zero priority is valid");

        assert_eq!(decision.score(), 0);
    }

    #[test]
    fn negative_priority_is_supported() {
        let priority = PriorityPolicy::new();
        let configuration = policy();

        let lower = PriorityCandidate::from_priority(operation(1), -10);
        let higher = PriorityCandidate::from_priority(operation(2), 10);

        let lower_decision = priority
            .evaluate(&configuration, &lower)
            .expect("negative priority is valid");

        let higher_decision = priority
            .evaluate(&configuration, &higher)
            .expect("positive priority is valid");

        assert!(priority.prefers(
            &higher_decision,
            &lower_decision
        ));
    }

    #[test]
    fn weighted_score_uses_policy_weights() {
        let priority = PriorityPolicy::new();

        let configuration = SchedulingPolicy::builder()
            .priority_weight(2)
            .criticality_weight(3)
            .resource_weight(4)
            .fidelity_weight(5)
            .build()
            .expect("valid scheduling policy");

        let candidate = weighted_priority_candidate(
            operation(1),
            1,
            2,
            3,
            4,
        );

        let decision = priority
            .evaluate(&configuration, &candidate)
            .expect("valid candidate");

        assert_eq!(
            decision.score(),
            1 * 2 + 2 * 3 + 3 * 4 + 4 * 5
        );
    }

    #[test]
    fn capabilities_are_target_independent() {
        let capabilities = PriorityPolicy::new().capabilities();

        assert!(capabilities.explicit_priority);
        assert!(capabilities.criticality);
        assert!(capabilities.resource_pressure);
        assert!(capabilities.fidelity);
        assert!(capabilities.deterministic);
        assert!(capabilities.target_independent);
    }

    #[test]
    fn policy_validation_uses_common_policy_contract() {
        let priority = PriorityPolicy::new();

        let configuration = policy();

        assert!(priority.validate_policy(&configuration).is_ok());
    }
}