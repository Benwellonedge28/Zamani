
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Scheduling Policies
//!
//! Path: src/zur/scheduling/policy.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Defines declarative operational scheduling policies for ZUR-managed
//! runtime work.
//!
//! Scheduling policies describe how a runtime scheduler should prioritize
//! and organize eligible work. They do not execute tasks or determine
//! whether a program is semantically valid.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Scheduling algorithm preferences.
//! - Scheduling objectives.
//! - Objective ordering.
//! - Tie-breaking preferences.
//! - Fairness policies.
//! - Starvation policies.
//! - Preemption policies.
//! - Extensible scheduling policy values.
//! - Scheduling policy validation.
//!
//! EXISTING AUTHORITIES
//!
//! - zur/error.rs owns ZurError and ZurResult.
//! - zur/policy.rs owns RuntimePolicy.
//! - zur/config.rs owns RuntimeConfig and operational budgets.
//! - zur/ids.rs owns runtime identities.
//! - zur/concurrency/task.rs owns task lifecycle.
//! - zur/concurrency/executor.rs owns task execution.
//! - zur/resources/ owns runtime resource management.
//! - compiler/planning owns compilation planning.
//! - distributed/planning owns distributed scheduling planning.
//! - hybrid/planning owns hybrid planning.
//! - quantum/scheduling owns quantum-operation scheduling.
//! - frontend/semantic owns source-language scheduling semantics.
//!
//! THIS MODULE DOES NOT OWN
//!
//! - Task execution.
//! - Thread creation.
//! - Task identity.
//! - Task lifecycle.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Target placement.
//! - Distributed scheduling.
//! - Quantum scheduling.
//! - Hybrid planning.
//! - Compiler optimization.
//! - Source grammar.
//! - Semantic requirements.
//! - Capability definitions.
//! - Resource feasibility.
//! - Runtime-wide service activation.
//!
//! SCALABILITY
//!
//! No arbitrary limits are imposed on:
//!
//! - Scheduling policies.
//! - Scheduling objectives.
//! - Scheduling extensions.
//! - Tasks.
//! - Providers.
//! - Execution domains.
//! - Scheduling participants.
//! - Resource quantities.
//! - Namespace depth.
//!
//! Collections grow according to available memory.
//!
//! No finite machine guarantees infinite physical computation.
//!
//! SAFETY
//!
//! No unsafe Rust.
//! No raw pointers.
//! No global mutable state.
//! No implicit hardware discovery.
//! No hidden environment access.
//! No fabricated scheduling guarantees.
//!
//! DETERMINISM
//!
//! Ordered maps preserve deterministic extension representation.
//! Objective and tie-breaking ordering is explicitly preserved.
//!
//! INTEGRATION
//!
//! - scheduling/context.rs supplies invocation-specific scheduling context.
//! - scheduling/engine.rs consumes validated policy declarations.
//! - scheduling/algorithms/ implements actual scheduling algorithms.
//! - concurrency/executor.rs executes the resulting selected tasks.
//! - resources/ supplies resource availability and reservation interfaces.
//! - zur/policy.rs continues to own runtime-wide operational policies.
//! - grammar/execution/scheduling.g4 remains the source scheduling grammar.
//!
//! A policy declaration is not proof of feasibility or execution success.

use core::fmt;
use std::collections::BTreeMap;

use crate::zur::error::{ZurError, ZurResult};

use crate::zur::policy::{PolicyKey, PolicyValue};

// =============================================================================
// ERROR CONSTRUCTION
// =============================================================================

/// Constructs a scheduling-policy configuration error.
fn policy_error(message: impl Into<String>) -> ZurError {
    ZurError::configuration(message)
        .with_component("zur.scheduling.policy")
}

// =============================================================================
// SCHEDULING ALGORITHM
// =============================================================================

/// Identifies the scheduling algorithm requested by a policy.
///
/// Built-in algorithms describe stable operational scheduling contracts.
///
/// Custom algorithms use an extensible namespaced identifier.
///
/// Selecting an algorithm does not prove that its implementation is
/// installed or available.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SchedulingAlgorithm {
    /// First-in, first-out scheduling.
    ///
    /// Eligible work is considered in submission order.
    Fifo,

    /// Priority-oriented scheduling.
    ///
    /// Priority values must come from the canonical task or scheduling
    /// context rather than being invented by this policy.
    Priority,

    /// Deadline-oriented scheduling.
    ///
    /// Deadline information must be supplied by the scheduling context.
    EarliestDeadline,

    /// Fair-share scheduling.
    ///
    /// Requires an implementation capable of evaluating the applicable
    /// fairness information.
    FairShare,

    /// Work-conserving scheduling.
    ///
    /// The implementation attempts to keep eligible execution capacity
    /// productively occupied without violating constraints.
    WorkConserving,

    /// A custom, independently registered scheduling algorithm.
    ///
    /// The identifier does not establish implementation availability.
    Custom(PolicyKey),
}

impl SchedulingAlgorithm {
    /// Returns the stable algorithm identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Fifo => "zur.scheduling.fifo",
            Self::Priority => "zur.scheduling.priority",
            Self::EarliestDeadline => "zur.scheduling.earliest_deadline",
            Self::FairShare => "zur.scheduling.fair_share",
            Self::WorkConserving => "zur.scheduling.work_conserving",
            Self::Custom(identifier) => identifier.as_str(),
        }
    }

    /// Creates a custom scheduling algorithm reference.
    ///
    /// The identifier must be a valid namespaced policy key.
    pub fn custom(identifier: impl Into<String>) -> ZurResult<Self> {
        Ok(Self::Custom(PolicyKey::new(identifier)?))
    }

    /// Returns whether this algorithm is custom.
    #[must_use]
    pub const fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }
}

impl Default for SchedulingAlgorithm {
    fn default() -> Self {
        Self::Fifo
    }
}

impl fmt::Display for SchedulingAlgorithm {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// SCHEDULING OBJECTIVES
// =============================================================================

/// Direction of a scheduling optimization objective.
///
/// Objectives only guide optimization among otherwise valid schedules.
///
/// They must never override mandatory execution constraints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ObjectiveDirection {
    /// Prefer a smaller objective value.
    Minimize,

    /// Prefer a larger objective value.
    Maximize,
}

impl ObjectiveDirection {
    /// Returns the stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Minimize => "minimize",
            Self::Maximize => "maximize",
        }
    }
}

/// An individual scheduling objective.
///
/// The objective identifier is open-ended.
///
/// This permits future scheduling objectives without modifying
/// a universal closed enumeration.
///
/// Examples:
///
/// - `zur.scheduling.objective.latency`
/// - `zur.scheduling.objective.throughput`
/// - `zur.scheduling.objective.energy`
/// - `quantum.scheduling.objective.fidelity`
/// - `distributed.scheduling.objective.locality`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SchedulingObjective {
    identifier: PolicyKey,
    direction: ObjectiveDirection,
}

impl SchedulingObjective {
    /// Creates a scheduling objective.
    pub fn new(
        identifier: impl Into<String>,
        direction: ObjectiveDirection,
    ) -> ZurResult<Self> {
        Ok(Self {
            identifier: PolicyKey::new(identifier)?,
            direction,
        })
    }

    /// Returns the objective identifier.
    #[must_use]
    pub fn identifier(&self) -> &PolicyKey {
        &self.identifier
    }

    /// Returns the optimization direction.
    #[must_use]
    pub const fn direction(&self) -> ObjectiveDirection {
        self.direction
    }
}

// =============================================================================
// TIE-BREAKING
// =============================================================================

/// Defines how equally preferred eligible work is ordered.
///
/// The actual ordering implementation belongs to the scheduler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TieBreakRule {
    /// Preserve submission order.
    SubmissionOrder,

    /// Use canonical runtime task or operation identity.
    ///
    /// The scheduler must use the authoritative identity supplied
    /// by the task subsystem.
    StableIdentity,

    /// Prefer work that has been ready for longer.
    ///
    /// Readiness timestamps must be supplied by the scheduling context.
    ReadySince,

    /// Use the task's authoritative priority information.
    Priority,
}

impl TieBreakRule {
    /// Returns the stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SubmissionOrder => "submission_order",
            Self::StableIdentity => "stable_identity",
            Self::ReadySince => "ready_since",
            Self::Priority => "priority",
        }
    }
}

impl Default for TieBreakRule {
    fn default() -> Self {
        Self::SubmissionOrder
    }
}

// =============================================================================
// PREEMPTION
// =============================================================================

/// Defines the permitted preemption behavior.
///
/// This is an operational policy declaration, not an implementation
/// of task interruption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum PreemptionPolicy {
    /// Do not preempt running work automatically.
    Disabled,

    /// Permit cooperative preemption when the execution implementation
    /// supports it.
    #[default]
    Cooperative,

    /// Require a compatible preemption implementation.
    ///
    /// If preemption cannot be provided, the scheduler must report
    /// incompatibility instead of silently weakening this policy.
    Required,
}

impl PreemptionPolicy {
    /// Returns the stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Cooperative => "cooperative",
            Self::Required => "required",
        }
    }
}

// =============================================================================
// FAIRNESS
// =============================================================================

/// Defines the requested fairness behavior.
///
/// Fairness guarantees depend on the actual scheduler implementation,
/// workload information, resource availability and scheduling context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum FairnessPolicy {
    /// No additional fairness guarantee is requested.
    Disabled,

    /// Prefer fair allocation when supported.
    ///
    /// The implementation may report that stronger fairness is unavailable.
    #[default]
    BestEffort,

    /// Require the selected scheduler to provide its declared
    /// fairness guarantees.
    ///
    /// Unsupported fairness requirements must produce an explicit error.
    Required,
}

impl FairnessPolicy {
    /// Returns the stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::BestEffort => "best_effort",
            Self::Required => "required",
        }
    }
}

// =============================================================================
// STARVATION PREVENTION
// =============================================================================

/// Defines the requested starvation-prevention behavior.
///
/// Actual waiting-time measurements and aging calculations belong
/// to the scheduling implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum StarvationPolicy {
    /// No additional starvation-prevention guarantee.
    Disabled,

    /// Prefer aging eligible work to reduce starvation.
    #[default]
    AgeReadyWork,

    /// Require the selected scheduler to provide a documented
    /// bounded-wait guarantee.
    ///
    /// The implementation must reject this policy when it cannot
    /// establish the requested guarantee.
    RequireBoundedWait,
}

impl StarvationPolicy {
    /// Returns the stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::AgeReadyWork => "age_ready_work",
            Self::RequireBoundedWait => "require_bounded_wait",
        }
    }
}

// =============================================================================
// EXTENSION VALUES
// =============================================================================

/// Runtime scheduling policy extensions.
///
/// Extensions are opaque to the core scheduler unless an independently
/// registered implementation understands and validates their meaning.
///
/// Unknown extensions never grant execution permission or override
/// mandatory constraints.
pub type SchedulingExtensions = BTreeMap<PolicyKey, PolicyValue>;

// =============================================================================
// SCHEDULING POLICY
// =============================================================================

/// Immutable declarative policy for ZUR operational scheduling.
///
/// This policy describes scheduling intent and operational preferences.
///
/// It does not:
///
/// - Select physical hardware.
/// - Allocate resources.
/// - Execute tasks.
/// - Establish feasibility.
/// - Override semantic requirements.
///
/// Objectives are ordered. Earlier objectives have precedence over
/// later objectives when the scheduler supports multi-objective evaluation.
///
/// Correctness constraints always take precedence over objectives.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulingPolicy {
    algorithm: SchedulingAlgorithm,
    objectives: Vec<SchedulingObjective>,
    tie_breakers: Vec<TieBreakRule>,
    preemption: PreemptionPolicy,
    fairness: FairnessPolicy,
    starvation: StarvationPolicy,
    extensions: SchedulingExtensions,
}

impl Default for SchedulingPolicy {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedulingPolicy {
    /// Creates a conservative default scheduling policy.
    ///
    /// Defaults:
    ///
    /// - FIFO scheduling.
    /// - No additional optimization objectives.
    /// - Submission-order tie-breaking.
    /// - Cooperative preemption.
    /// - Best-effort fairness.
    /// - Aging of ready work.
    ///
    /// The defaults do not imply that all requested behavior is
    /// supported by every scheduler.
    #[must_use]
    pub fn new() -> Self {
        Self {
            algorithm: SchedulingAlgorithm::default(),
            objectives: Vec::new(),
            tie_breakers: vec![TieBreakRule::SubmissionOrder],
            preemption: PreemptionPolicy::default(),
            fairness: FairnessPolicy::default(),
            starvation: StarvationPolicy::default(),
            extensions: BTreeMap::new(),
        }
    }

    /// Returns the requested scheduling algorithm.
    #[must_use]
    pub fn algorithm(&self) -> &SchedulingAlgorithm {
        &self.algorithm
    }

    /// Returns the ordered optimization objectives.
    #[must_use]
    pub fn objectives(&self) -> &[SchedulingObjective] {
        &self.objectives
    }

    /// Returns the ordered tie-breaking rules.
    #[must_use]
    pub fn tie_breakers(&self) -> &[TieBreakRule] {
        &self.tie_breakers
    }

    /// Returns the preemption policy.
    #[must_use]
    pub const fn preemption(&self) -> PreemptionPolicy {
        self.preemption
    }

    /// Returns the fairness policy.
    #[must_use]
    pub const fn fairness(&self) -> FairnessPolicy {
        self.fairness
    }

    /// Returns the starvation policy.
    #[must_use]
    pub const fn starvation(&self) -> StarvationPolicy {
        self.starvation
    }

    /// Returns all opaque policy extensions.
    #[must_use]
    pub fn extensions(&self) -> &SchedulingExtensions {
        &self.extensions
    }

    /// Returns one extension value.
    #[must_use]
    pub fn extension(&self, key: &PolicyKey) -> Option<&PolicyValue> {
        self.extensions.get(key)
    }

    /// Changes the requested scheduling algorithm.
    #[must_use]
    pub fn with_algorithm(
        mut self,
        algorithm: SchedulingAlgorithm,
    ) -> Self {
        self.algorithm = algorithm;
        self
    }

    /// Adds an ordered scheduling objective.
    ///
    /// Duplicate objective identifiers are rejected.
    pub fn with_objective(
        mut self,
        objective: SchedulingObjective,
    ) -> ZurResult<Self> {
        if self.objectives.iter().any(|existing| {
            existing.identifier == objective.identifier
        }) {
            return Err(policy_error(
                format!(
                    "duplicate scheduling objective: {}",
                    objective.identifier
                ),
            ));
        }

        self.objectives.push(objective);

        Ok(self)
    }

    /// Replaces the complete objective ordering.
    ///
    /// Duplicate objective identifiers are rejected.
    pub fn with_objectives(
        mut self,
        objectives: Vec<SchedulingObjective>,
    ) -> ZurResult<Self> {
        let mut seen = std::collections::BTreeSet::new();

        for objective in &objectives {
            if !seen.insert(objective.identifier.clone()) {
                return Err(policy_error(
                    format!(
                        "duplicate scheduling objective: {}",
                        objective.identifier
                    ),
                ));
            }
        }

        self.objectives = objectives;

        Ok(self)
    }

    /// Replaces the tie-breaking order.
    ///
    /// Duplicate rules are rejected to prevent redundant or ambiguous
    /// ordering declarations.
    pub fn with_tie_breakers(
        mut self,
        tie_breakers: Vec<TieBreakRule>,
    ) -> ZurResult<Self> {
        let mut seen = std::collections::BTreeSet::new();

        for rule in &tie_breakers {
            if !seen.insert(*rule) {
                return Err(policy_error(
                    format!(
                        "duplicate scheduling tie-breaker: {}",
                        rule.as_str()
                    ),
                ));
            }
        }

        self.tie_breakers = tie_breakers;

        Ok(self)
    }

    /// Changes the preemption policy.
    #[must_use]
    pub fn with_preemption(
        mut self,
        preemption: PreemptionPolicy,
    ) -> Self {
        self.preemption = preemption;
        self
    }

    /// Changes the fairness policy.
    #[must_use]
    pub fn with_fairness(
        mut self,
        fairness: FairnessPolicy,
    ) -> Self {
        self.fairness = fairness;
        self
    }

    /// Changes the starvation policy.
    #[must_use]
    pub fn with_starvation(
        mut self,
        starvation: StarvationPolicy,
    ) -> Self {
        self.starvation = starvation;
        self
    }

    /// Adds a new extension without replacing an existing extension.
    ///
    /// Duplicate identifiers are rejected.
    pub fn with_extension(
        mut self,
        key: PolicyKey,
        value: PolicyValue,
    ) -> ZurResult<Self> {
        if self.extensions.contains_key(&key) {
            return Err(policy_error(
                format!(
                    "scheduling extension already exists: {key}"
                ),
            ));
        }

        self.extensions.insert(key, value);

        Ok(self)
    }

    /// Explicitly inserts or replaces an extension.
    ///
    /// Returns the previous value when one existed.
    pub fn replace_extension(
        &mut self,
        key: PolicyKey,
        value: PolicyValue,
    ) -> Option<PolicyValue> {
        self.extensions.insert(key, value)
    }

    /// Removes an extension.
    ///
    /// Returns the removed value when one existed.
    pub fn remove_extension(
        &mut self,
        key: &PolicyKey,
    ) -> Option<PolicyValue> {
        self.extensions.remove(key)
    }

    /// Validates the internal policy invariants.
    ///
    /// This validates policy structure only.
    ///
    /// It does not establish that the selected algorithm is installed,
    /// that resources are available, or that a workload is schedulable.
    pub fn validate(&self) -> ZurResult<()> {
        let mut objective_identifiers =
            std::collections::BTreeSet::new();

        for objective in &self.objectives {
            if !objective_identifiers.insert(
                objective.identifier.clone(),
            ) {
                return Err(policy_error(
                    format!(
                        "duplicate scheduling objective: {}",
                        objective.identifier
                    ),
                ));
            }
        }

        let mut tie_breakers =
            std::collections::BTreeSet::new();

        for rule in &self.tie_breakers {
            if !tie_breakers.insert(*rule) {
                return Err(policy_error(
                    format!(
                        "duplicate scheduling tie-breaker: {}",
                        rule.as_str()
                    ),
                ));
            }
        }

        if let SchedulingAlgorithm::Custom(identifier) = &self.algorithm {
            if identifier.as_str().trim().is_empty() {
                return Err(policy_error(
                    "custom scheduling algorithm identifier is empty",
                ));
            }
        }

        Ok(())
    }

    /// Returns the number of registered policy extensions.
    ///
    /// This is informational only and is not a resource limit.
    #[must_use]
    pub fn extension_count(&self) -> usize {
        self.extensions.len()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn objective(
        identifier: &str,
        direction: ObjectiveDirection,
    ) -> SchedulingObjective {
        SchedulingObjective::new(identifier, direction).unwrap()
    }

    fn key(identifier: &str) -> PolicyKey {
        PolicyKey::new(identifier).unwrap()
    }

    fn value(value: &str) -> PolicyValue {
        PolicyValue::new(value).unwrap()
    }

    #[test]
    fn default_policy_is_valid() {
        let policy = SchedulingPolicy::new();

        assert!(policy.validate().is_ok());

        assert_eq!(
            policy.algorithm(),
            &SchedulingAlgorithm::Fifo
        );

        assert_eq!(
            policy.preemption(),
            PreemptionPolicy::Cooperative
        );

        assert_eq!(
            policy.fairness(),
            FairnessPolicy::BestEffort
        );

        assert_eq!(
            policy.starvation(),
            StarvationPolicy::AgeReadyWork
        );
    }

    #[test]
    fn supports_builtin_algorithms() {
        let algorithms = [
            SchedulingAlgorithm::Fifo,
            SchedulingAlgorithm::Priority,
            SchedulingAlgorithm::EarliestDeadline,
            SchedulingAlgorithm::FairShare,
            SchedulingAlgorithm::WorkConserving,
        ];

        for algorithm in algorithms {
            assert!(!algorithm.as_str().is_empty());
        }
    }

    #[test]
    fn supports_custom_algorithms() {
        let algorithm = SchedulingAlgorithm::custom(
            "future.scheduling.algorithm"
        ).unwrap();

        assert!(algorithm.is_custom());

        assert_eq!(
            algorithm.as_str(),
            "future.scheduling.algorithm"
        );
    }

    #[test]
    fn rejects_invalid_custom_algorithm_identifiers() {
        assert!(
            SchedulingAlgorithm::custom("").is_err()
        );

        assert!(
            SchedulingAlgorithm::custom("algorithm").is_err()
        );

        assert!(
            SchedulingAlgorithm::custom("Future.Algorithm").is_err()
        );
    }

    #[test]
    fn supports_multiple_ordered_objectives() {
        let policy = SchedulingPolicy::new()
            .with_objective(objective(
                "zur.scheduling.objective.latency",
                ObjectiveDirection::Minimize,
            ))
            .unwrap()
            .with_objective(objective(
                "zur.scheduling.objective.throughput",
                ObjectiveDirection::Maximize,
            ))
            .unwrap();

        assert_eq!(policy.objectives().len(), 2);

        assert_eq!(
            policy.objectives()[0].direction(),
            ObjectiveDirection::Minimize
        );

        assert_eq!(
            policy.objectives()[1].direction(),
            ObjectiveDirection::Maximize
        );

        assert!(policy.validate().is_ok());
    }

    #[test]
    fn rejects_duplicate_objectives() {
        let first = objective(
            "zur.scheduling.objective.latency",
            ObjectiveDirection::Minimize,
        );

        let second = objective(
            "zur.scheduling.objective.latency",
            ObjectiveDirection::Maximize,
        );

        let policy = SchedulingPolicy::new()
            .with_objective(first)
            .unwrap();

        assert!(policy.with_objective(second).is_err());
    }

    #[test]
    fn supports_open_ended_objective_namespaces() {
        let objective = objective(
            "future.photonic.scheduling.objective.energy",
            ObjectiveDirection::Minimize,
        );

        assert_eq!(
            objective.identifier().as_str(),
            "future.photonic.scheduling.objective.energy"
        );
    }

    #[test]
    fn supports_tie_breaking_configuration() {
        let policy = SchedulingPolicy::new()
            .with_tie_breakers(vec![
                TieBreakRule::Priority,
                TieBreakRule::ReadySince,
                TieBreakRule::StableIdentity,
            ])
            .unwrap();

        assert_eq!(policy.tie_breakers().len(), 3);

        assert!(policy.validate().is_ok());
    }

    #[test]
    fn rejects_duplicate_tie_breakers() {
        assert!(
            SchedulingPolicy::new()
                .with_tie_breakers(vec![
                    TieBreakRule::Priority,
                    TieBreakRule::Priority,
                ])
                .is_err()
        );
    }

    #[test]
    fn permits_empty_tie_breaker_list() {
        let policy = SchedulingPolicy::new()
            .with_tie_breakers(Vec::new())
            .unwrap();

        assert!(policy.validate().is_ok());
    }

    #[test]
    fn supports_preemption_configuration() {
        for preemption in [
            PreemptionPolicy::Disabled,
            PreemptionPolicy::Cooperative,
            PreemptionPolicy::Required,
        ] {
            let policy = SchedulingPolicy::new()
                .with_preemption(preemption);

            assert_eq!(policy.preemption(), preemption);
            assert!(policy.validate().is_ok());
        }
    }

    #[test]
    fn supports_fairness_configuration() {
        for fairness in [
            FairnessPolicy::Disabled,
            FairnessPolicy::BestEffort,
            FairnessPolicy::Required,
        ] {
            let policy = SchedulingPolicy::new()
                .with_fairness(fairness);

            assert_eq!(policy.fairness(), fairness);
            assert!(policy.validate().is_ok());
        }
    }

    #[test]
    fn supports_starvation_configuration() {
        for starvation in [
            StarvationPolicy::Disabled,
            StarvationPolicy::AgeReadyWork,
            StarvationPolicy::RequireBoundedWait,
        ] {
            let policy = SchedulingPolicy::new()
                .with_starvation(starvation);

            assert_eq!(policy.starvation(), starvation);
            assert!(policy.validate().is_ok());
        }
    }

    #[test]
    fn supports_extensions() {
        let policy = SchedulingPolicy::new()
            .with_extension(
                key("future.scheduling.policy"),
                value("enabled"),
            )
            .unwrap();

        assert_eq!(policy.extension_count(), 1);

        assert_eq!(
            policy.extension(
                &key("future.scheduling.policy")
            ).unwrap().as_str(),
            "enabled"
        );
    }

    #[test]
    fn rejects_duplicate_extensions() {
        let policy = SchedulingPolicy::new()
            .with_extension(
                key("future.scheduling.policy"),
                value("enabled"),
            )
            .unwrap();

        assert!(
            policy.with_extension(
                key("future.scheduling.policy"),
                value("disabled"),
            ).is_err()
        );
    }

    #[test]
    fn supports_explicit_extension_replacement() {
        let mut policy = SchedulingPolicy::new();

        policy.replace_extension(
            key("future.scheduling.policy"),
            value("enabled"),
        );

        let previous = policy.replace_extension(
            key("future.scheduling.policy"),
            value("validated"),
        );

        assert_eq!(
            previous.unwrap().as_str(),
            "enabled"
        );

        assert_eq!(
            policy.extension(
                &key("future.scheduling.policy")
            ).unwrap().as_str(),
            "validated"
        );
    }

    #[test]
    fn supports_extension_removal() {
        let mut policy = SchedulingPolicy::new();

        policy.replace_extension(
            key("future.scheduling.policy"),
            value("enabled"),
        );

        assert!(
            policy.remove_extension(
                &key("future.scheduling.policy")
            ).is_some()
        );

        assert_eq!(policy.extension_count(), 0);
    }

    #[test]
    fn preserves_policy_equality() {
        let first = SchedulingPolicy::new()
            .with_algorithm(SchedulingAlgorithm::Priority)
            .with_fairness(FairnessPolicy::Required);

        let second = SchedulingPolicy::new()
            .with_algorithm(SchedulingAlgorithm::Priority)
            .with_fairness(FairnessPolicy::Required);

        assert_eq!(first, second);
    }

    #[test]
    fn policy_cloning_preserves_configuration() {
        let policy = SchedulingPolicy::new()
            .with_algorithm(SchedulingAlgorithm::FairShare)
            .with_objective(objective(
                "zur.scheduling.objective.throughput",
                ObjectiveDirection::Maximize,
            ))
            .unwrap()
            .with_fairness(FairnessPolicy::Required);

        assert_eq!(policy, policy.clone());
    }

    #[test]
    fn policy_has_no_domain_specific_algorithm_requirement() {
        let algorithm = SchedulingAlgorithm::custom(
            "future.neuromorphic.scheduler"
        ).unwrap();

        let policy = SchedulingPolicy::new()
            .with_algorithm(algorithm);

        assert!(policy.validate().is_ok());
    }

    #[test]
    fn policy_does_not_impose_machine_size_limits() {
        let policy = SchedulingPolicy::new()
            .with_algorithm(SchedulingAlgorithm::WorkConserving);

        assert!(policy.validate().is_ok());
    }
}
