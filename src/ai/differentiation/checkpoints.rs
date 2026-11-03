#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

//! # Zamani AI — Differentiation Checkpoint Policies
//!
//! Path: `src/ai/differentiation/checkpoints.rs`
//! Rust baseline: 1.97+
//! Edition: 2021
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines target-independent contracts for deciding which forward
//! computation values may be retained for differentiation and which
//! computations may be recomputed during derivative propagation.
//!
//! Checkpointing is a trade-off between retaining intermediate values and
//! recomputing them later. The best trade-off depends on the computation,
//! its mathematical semantics, available resources, and the execution
//! environment.
//!
//! This module describes policies and plans. It does not execute them.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Checkpointing policy descriptions.
//! - Checkpoint candidate descriptions.
//! - Recompute candidate descriptions.
//! - Abstract checkpoint plan representation.
//! - Policy validation.
//! - Plan structural validation.
//! - Deterministic plan ordering.
//! - Checkpointing decision contracts.
//!
//! This module does not own:
//!
//! - Canonical model graph construction.
//! - Differentiation graph construction.
//! - Mathematical differentiation rules.
//! - Forward- or reverse-mode propagation.
//! - Tensor or value storage.
//! - Serialization of canonical Zamani values.
//! - Durable checkpoint persistence.
//! - Device allocation.
//! - Memory management.
//! - Hardware selection.
//! - Scheduling.
//! - Distributed recovery.
//! - Runtime execution.
//!
//! ## Integration
//!
//! `graph.rs`:
//!     Supplies differentiation dependency relationships. An adapter may
//!     map canonical graph identities to the generic identifiers used here.
//!
//! `forward.rs` and `reverse.rs`:
//!     Consume the checkpoint/recomputation plan through execution adapters.
//!     They remain responsible for differentiation propagation.
//!
//! `validation.rs`:
//!     May use policy and plan validation as part of differentiation legality.
//!
//! `distributed/resilience/checkpoint.rs`:
//!     Owns durable checkpoint capture, integrity, persistence, and restore.
//!     This module does not duplicate those responsibilities.
//!
//! `classical::collections::tensor`:
//!     Remains the canonical classical tensor storage abstraction.
//!
//! `zuir`:
//!     Remains the canonical computational representation.
//!
//! ## POCO-REAF
//!
//! This module imposes no fixed maximum on:
//!
//! - Differentiation values.
//! - Checkpoints.
//! - Recomputation steps.
//! - Graph nodes.
//! - Model parameters.
//! - Tensor rank or dimensions.
//! - Devices.
//! - Workers.
//! - Memory capacity.
//!
//! A configured budget is an operational constraint, not a language-level
//! semantic ceiling. The policy must report when it cannot be satisfied.
//!
//! Literal infinite computation is not promised. Actual feasibility depends
//! on available resources and representational limits.
//!
//! ## Important semantic rule
//!
//! Recomputing a value is valid only when the responsible semantic/runtime
//! layer establishes that recomputation preserves the program's required
//! behavior. This is especially important for effects, randomness, mutable
//! state, measurements, and externally observable operations.
//!
//! This module records the caller's eligibility declaration; it does not
//! independently prove purity or deterministic behavior.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

// ============================================================================
// IDENTIFIERS
// ============================================================================

/// Identifies a logical value that may be retained or recomputed.
///
/// The identifier is supplied by the canonical model, differentiation,
/// or execution-trace adapter. This module does not allocate identities.
///
/// `Ord` is required to make plan ordering and diagnostics deterministic.
pub trait CheckpointValueId:
    Clone + Eq + Ord + fmt::Debug + fmt::Display
{
}

impl<T> CheckpointValueId for T where
    T: Clone + Eq + Ord + fmt::Debug + fmt::Display
{
}

/// Identifies a logical operation that may be recomputed.
pub trait CheckpointOperationId:
    Clone + Eq + Ord + fmt::Debug + fmt::Display
{
}

impl<T> CheckpointOperationId for T where
    T: Clone + Eq + Ord + fmt::Debug + fmt::Display
{
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by checkpoint policy or plan validation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CheckpointError {
    /// A policy contains an invalid configuration.
    InvalidPolicy {
        /// Description of the invalid configuration.
        reason: String,
    },

    /// A checkpoint entry refers to a value that is not in the plan.
    UnknownValue {
        /// The value identifier.
        value: String,
    },

    /// A recomputation entry refers to an operation that is not in the plan.
    UnknownOperation {
        /// The operation identifier.
        operation: String,
    },

    /// A value was assigned conflicting plan actions.
    ConflictingAction {
        /// The value identifier.
        value: String,
    },

    /// A checkpoint was requested for a value that cannot be retained.
    CheckpointNotEligible {
        /// The value identifier.
        value: String,
        /// Why the value is not eligible.
        reason: String,
    },

    /// A recomputation was requested for an operation that cannot be replayed.
    RecomputeNotEligible {
        /// The operation identifier.
        operation: String,
        /// Why the operation is not eligible.
        reason: String,
    },

    /// The plan violates an internal structural invariant.
    InvalidPlan {
        /// Description of the structural problem.
        reason: String,
    },

    /// A configured budget cannot be met by the proposed plan.
    BudgetExceeded {
        /// Resource whose budget was exceeded.
        resource: String,
        /// Required amount, if known.
        required: Option<u128>,
        /// Configured budget.
        budget: u128,
    },
}

impl fmt::Display for CheckpointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPolicy { reason } => {
                write!(f, "invalid checkpoint policy: {reason}")
            }
            Self::UnknownValue { value } => {
                write!(f, "unknown checkpoint value: {value}")
            }
            Self::UnknownOperation { operation } => {
                write!(f, "unknown checkpoint operation: {operation}")
            }
            Self::ConflictingAction { value } => {
                write!(f, "conflicting checkpoint actions for value: {value}")
            }
            Self::CheckpointNotEligible { value, reason } => {
                write!(f, "value {value} is not checkpoint-eligible: {reason}")
            }
            Self::RecomputeNotEligible { operation, reason } => {
                write!(
                    f,
                    "operation {operation} is not recompute-eligible: {reason}"
                )
            }
            Self::InvalidPlan { reason } => {
                write!(f, "invalid checkpoint plan: {reason}")
            }
            Self::BudgetExceeded {
                resource,
                required,
                budget,
            } => match required {
                Some(required) => write!(
                    f,
                    "checkpoint budget exceeded for {resource}: \
                     required {required}, budget {budget}"
                ),
                None => write!(
                    f,
                    "checkpoint budget exceeded for {resource}: \
                     budget {budget}, required amount unknown"
                ),
            },
        }
    }
}

impl Error for CheckpointError {}

// ============================================================================
// POLICY
// ============================================================================

/// Describes how checkpoint retention and recomputation should be chosen.
///
/// These policies express intent. They do not perform planning or execution.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CheckpointPolicy {
    /// Retain every eligible intermediate value.
    ///
    /// This minimizes recomputation but may require substantial memory.
    RetainAll,

    /// Retain no intermediate values unless required by the execution
    /// contract; recompute eligible operations when needed.
    ///
    /// This can reduce retained state while increasing computation.
    RecomputeWhenPossible,

    /// Retain checkpoints subject to explicit caller-provided budgets.
    ///
    /// Budgets are operational constraints, not universal model limits.
    Budgeted(CheckpointBudget),

    /// Use a caller-supplied policy identifier.
    ///
    /// The identifier is resolved by an external policy registry/provider.
    /// This module does not assume a closed list of strategies.
    Extension {
        /// Stable policy identifier.
        policy_id: String,

        /// Optional policy version.
        version: Option<String>,
    },
}

impl CheckpointPolicy {
    /// Validates the policy's local configuration.
    ///
    /// This does not establish whether a particular graph can satisfy it.
    pub fn validate(&self) -> Result<(), CheckpointError> {
        match self {
            Self::RetainAll | Self::RecomputeWhenPossible => Ok(()),

            Self::Budgeted(budget) => budget.validate(),

            Self::Extension { policy_id, version } => {
                if policy_id.trim().is_empty() {
                    return Err(CheckpointError::InvalidPolicy {
                        reason: "extension policy_id must not be empty".to_owned(),
                    });
                }

                if version
                    .as_ref()
                    .is_some_and(|value| value.trim().is_empty())
                {
                    return Err(CheckpointError::InvalidPolicy {
                        reason: "extension version must not be empty when supplied"
                            .to_owned(),
                    });
                }

                Ok(())
            }
        }
    }
}

/// Explicit operational limits supplied by the caller.
///
/// `None` means that this policy does not declare a budget for that
/// resource. It does not mean that the resource is infinite.
///
/// No field is a language-level maximum.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CheckpointBudget {
    /// Maximum retained checkpoint bytes, when known.
    pub retained_bytes: Option<u128>,

    /// Maximum number of retained checkpoint entries.
    pub checkpoint_count: Option<u128>,

    /// Maximum recomputation work units, when the provider defines them.
    pub recomputation_work: Option<u128>,
}

impl CheckpointBudget {
    /// Validates that the budget is structurally well-formed.
    ///
    /// Zero is valid and means that the corresponding resource is not
    /// available to this policy.
    pub fn validate(&self) -> Result<(), CheckpointError> {
        // Optional unsigned budgets are structurally valid at every value.
        // Feasibility is checked against a concrete plan/provider.
        Ok(())
    }
}

// ============================================================================
// ELIGIBILITY DESCRIPTORS
// ============================================================================

/// Describes whether a value may be retained as a checkpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointEligibility {
    /// Whether retaining the value is permitted by its owner.
    pub eligible: bool,

    /// Estimated or known retained size in bytes.
    ///
    /// `None` means that the size is unknown, symbolic, or not applicable.
    pub retained_bytes: Option<u128>,

    /// Optional explanation for an ineligible value.
    pub reason: Option<String>,
}

impl CheckpointEligibility {
    /// Creates an eligible value descriptor.
    #[must_use]
    pub const fn eligible(retained_bytes: Option<u128>) -> Self {
        Self {
            eligible: true,
            retained_bytes,
            reason: None,
        }
    }

    /// Creates an ineligible value descriptor.
    #[must_use]
    pub fn ineligible(reason: impl Into<String>) -> Self {
        Self {
            eligible: false,
            retained_bytes: None,
            reason: Some(reason.into()),
        }
    }

    /// Validates the eligibility descriptor.
    pub fn validate(&self) -> Result<(), CheckpointError> {
        if self.eligible && self.reason.is_some() {
            return Err(CheckpointError::InvalidPlan {
                reason: "eligible checkpoint value must not carry an ineligibility reason"
                    .to_owned(),
            });
        }

        if !self.eligible
            && self
                .reason
                .as_ref()
                .is_none_or(|reason| reason.trim().is_empty())
        {
            return Err(CheckpointError::InvalidPlan {
                reason: "ineligible checkpoint value must provide a reason".to_owned(),
            });
        }

        Ok(())
    }
}

/// Describes whether an operation may be recomputed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecomputeEligibility {
    /// Whether recomputation is permitted by the authoritative owner.
    pub eligible: bool,

    /// Estimated or known recomputation work units.
    pub work_units: Option<u128>,

    /// Optional explanation for an ineligible operation.
    pub reason: Option<String>,
}

impl RecomputeEligibility {
    /// Creates a recomputation-eligible operation descriptor.
    #[must_use]
    pub const fn eligible(work_units: Option<u128>) -> Self {
        Self {
            eligible: true,
            work_units,
            reason: None,
        }
    }

    /// Creates a recomputation-ineligible operation descriptor.
    #[must_use]
    pub fn ineligible(reason: impl Into<String>) -> Self {
        Self {
            eligible: false,
            work_units: None,
            reason: Some(reason.into()),
        }
    }

    /// Validates the eligibility descriptor.
    pub fn validate(&self) -> Result<(), CheckpointError> {
        if self.eligible && self.reason.is_some() {
            return Err(CheckpointError::InvalidPlan {
                reason: "eligible recomputation operation must not carry an \
                         ineligibility reason"
                    .to_owned(),
            });
        }

        if !self.eligible
            && self
                .reason
                .as_ref()
                .is_none_or(|reason| reason.trim().is_empty())
        {
            return Err(CheckpointError::InvalidPlan {
                reason: "ineligible recomputation operation must provide a reason"
                    .to_owned(),
            });
        }

        Ok(())
    }
}

// ============================================================================
// PLAN INPUT
// ============================================================================

/// Describes a value available to a checkpoint planner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointValue<ValueId> {
    /// Canonical logical value identity.
    pub id: ValueId,

    /// Checkpoint eligibility and size information.
    pub eligibility: CheckpointEligibility,
}

/// Describes an operation available to a checkpoint planner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointOperation<OperationId> {
    /// Canonical logical operation identity.
    pub id: OperationId,

    /// Recompute eligibility and work information.
    pub eligibility: RecomputeEligibility,
}

// ============================================================================
// PLAN ACTIONS
// ============================================================================

/// The abstract action assigned to a logical value.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CheckpointAction {
    /// Retain the value for later derivative propagation.
    Retain,

    /// Do not retain the value; permit its producer to be recomputed
    /// when the execution adapter requires it.
    Recompute,

    /// The value is an external input or otherwise supplied by an
    /// authoritative owner; this plan does not retain or recompute it.
    External,

    /// The plan leaves the value decision to a downstream provider.
    Deferred {
        /// Stable provider or policy identifier.
        provider: String,
    },
}

/// An abstract checkpoint decision for one logical value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointDecision<ValueId> {
    /// Value identity.
    pub value: ValueId,

    /// Selected action.
    pub action: CheckpointAction,
}

/// An abstract recomputation decision for one logical operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecomputationDecision<OperationId> {
    /// Operation identity.
    pub operation: OperationId,

    /// Whether the operation is selected for recomputation.
    pub recompute: bool,

    /// Optional explanation.
    pub reason: Option<String>,
}

// ============================================================================
// PLAN
// ============================================================================

/// A target-independent checkpoint/recomputation plan.
///
/// This is a declarative plan, not a persisted checkpoint payload or
/// executable schedule.
///
/// Values and operations are keyed by their canonical identities. BTree
/// collections provide deterministic ordering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointPlan<ValueId, OperationId> {
    policy: CheckpointPolicy,

    values: BTreeMap<ValueId, CheckpointValue<ValueId>>,

    operations: BTreeMap<OperationId, CheckpointOperation<OperationId>>,

    value_decisions: BTreeMap<ValueId, CheckpointAction>,

    operation_decisions: BTreeMap<OperationId, RecomputationDecision<OperationId>>,
}

impl<ValueId, OperationId> CheckpointPlan<ValueId, OperationId>
where
    ValueId: CheckpointValueId,
    OperationId: CheckpointOperationId,
{
    /// Creates an empty plan using the supplied policy.
    ///
    /// # Errors
    ///
    /// Returns an error if the policy is malformed.
    pub fn new(policy: CheckpointPolicy) -> Result<Self, CheckpointError> {
        policy.validate()?;

        Ok(Self {
            policy,
            values: BTreeMap::new(),
            operations: BTreeMap::new(),
            value_decisions: BTreeMap::new(),
            operation_decisions: BTreeMap::new(),
        })
    }

    /// Returns the policy used by this plan.
    #[must_use]
    pub const fn policy(&self) -> &CheckpointPolicy {
        &self.policy
    }

    /// Registers a value descriptor.
    ///
    /// Duplicate value identities are rejected. No existing plan state is
    /// changed when registration fails.
    pub fn add_value(
        &mut self,
        value: CheckpointValue<ValueId>,
    ) -> Result<(), CheckpointError> {
        value.eligibility.validate()?;

        if self.values.contains_key(&value.id) {
            return Err(CheckpointError::ConflictingAction {
                value: value.id.to_string(),
            });
        }

        self.values.insert(value.id.clone(), value);
        Ok(())
    }

    /// Registers an operation descriptor.
    ///
    /// Duplicate operation identities are rejected.
    pub fn add_operation(
        &mut self,
        operation: CheckpointOperation<OperationId>,
    ) -> Result<(), CheckpointError> {
        operation.eligibility.validate()?;

        if self.operations.contains_key(&operation.id) {
            return Err(CheckpointError::InvalidPlan {
                reason: format!(
                    "operation {} was registered more than once",
                    operation.id
                ),
            });
        }

        self.operations.insert(operation.id.clone(), operation);
        Ok(())
    }

    /// Assigns a checkpoint action to a registered value.
    ///
    /// A value may receive only one action.
    pub fn set_value_action(
        &mut self,
        value: ValueId,
        action: CheckpointAction,
    ) -> Result<(), CheckpointError> {
        let descriptor = self
            .values
            .get(&value)
            .ok_or_else(|| CheckpointError::UnknownValue {
                value: value.to_string(),
            })?;

        match &action {
            CheckpointAction::Retain if !descriptor.eligibility.eligible => {
                return Err(CheckpointError::CheckpointNotEligible {
                    value: value.to_string(),
                    reason: descriptor
                        .eligibility
                        .reason
                        .clone()
                        .unwrap_or_else(|| "eligibility was denied".to_owned()),
                });
            }
            CheckpointAction::Deferred { provider } if provider.trim().is_empty() => {
                return Err(CheckpointError::InvalidPlan {
                    reason: "deferred checkpoint action requires a provider identifier"
                        .to_owned(),
                });
            }
            _ => {}
        }

        if self.value_decisions.contains_key(&value) {
            return Err(CheckpointError::ConflictingAction {
                value: value.to_string(),
            });
        }

        self.value_decisions.insert(value, action);
        Ok(())
    }

    /// Assigns a recomputation decision to a registered operation.
    ///
    /// Recomputing an operation requires an affirmative eligibility
    /// declaration from its authoritative owner.
    pub fn set_operation_recompute(
        &mut self,
        operation: OperationId,
        recompute: bool,
        reason: Option<String>,
    ) -> Result<(), CheckpointError> {
        let descriptor = self
            .operations
            .get(&operation)
            .ok_or_else(|| CheckpointError::UnknownOperation {
                operation: operation.to_string(),
            })?;

        if recompute && !descriptor.eligibility.eligible {
            return Err(CheckpointError::RecomputeNotEligible {
                operation: operation.to_string(),
                reason: descriptor
                    .eligibility
                    .reason
                    .clone()
                    .unwrap_or_else(|| "eligibility was denied".to_owned()),
            });
        }

        if reason
            .as_ref()
            .is_some_and(|reason| reason.trim().is_empty())
        {
            return Err(CheckpointError::InvalidPlan {
                reason: "recomputation decision reason must not be empty".to_owned(),
            });
        }

        if self.operation_decisions.contains_key(&operation) {
            return Err(CheckpointError::InvalidPlan {
                reason: format!(
                    "operation {} already has a recomputation decision",
                    operation
                ),
            });
        }

        self.operation_decisions.insert(
            operation.clone(),
            RecomputationDecision {
                operation,
                recompute,
                reason,
            },
        );

        Ok(())
    }

    /// Returns a value descriptor, if registered.
    #[must_use]
    pub fn value(&self, id: &ValueId) -> Option<&CheckpointValue<ValueId>> {
        self.values.get(id)
    }

    /// Returns an operation descriptor, if registered.
    #[must_use]
    pub fn operation(
        &self,
        id: &OperationId,
    ) -> Option<&CheckpointOperation<OperationId>> {
        self.operations.get(id)
    }

    /// Returns the action assigned to a value, if any.
    #[must_use]
    pub fn value_action(&self, id: &ValueId) -> Option<&CheckpointAction> {
        self.value_decisions.get(id)
    }

    /// Returns the recomputation decision for an operation, if any.
    #[must_use]
    pub fn operation_decision(
        &self,
        id: &OperationId,
    ) -> Option<&RecomputationDecision<OperationId>> {
        self.operation_decisions.get(id)
    }

    /// Returns all registered values in deterministic identity order.
    pub fn values(
        &self,
    ) -> impl Iterator<Item = &CheckpointValue<ValueId>> {
        self.values.values()
    }

    /// Returns all registered operations in deterministic identity order.
    pub fn operations(
        &self,
    ) -> impl Iterator<Item = &CheckpointOperation<OperationId>> {
        self.operations.values()
    }

    /// Returns all value decisions in deterministic identity order.
    pub fn value_decisions(
        &self,
    ) -> impl Iterator<Item = (&ValueId, &CheckpointAction)> {
        self.value_decisions.iter()
    }

    /// Returns all operation decisions in deterministic identity order.
    pub fn operation_decisions(
        &self,
    ) -> impl Iterator<Item = (&OperationId, &RecomputationDecision<OperationId>)> {
        self.operation_decisions.iter()
    }

    /// Validates that every decision references a registered descriptor
    /// and that all local plan invariants hold.
    pub fn validate(&self) -> Result<(), CheckpointError> {
        self.policy.validate()?;

        for (value_id, action) in &self.value_decisions {
            let descriptor = self
                .values
                .get(value_id)
                .ok_or_else(|| CheckpointError::UnknownValue {
                    value: value_id.to_string(),
                })?;

            if matches!(action, CheckpointAction::Retain)
                && !descriptor.eligibility.eligible
            {
                return Err(CheckpointError::CheckpointNotEligible {
                    value: value_id.to_string(),
                    reason: descriptor
                        .eligibility
                        .reason
                        .clone()
                        .unwrap_or_else(|| "eligibility was denied".to_owned()),
                });
            }

            if let CheckpointAction::Deferred { provider } = action {
                if provider.trim().is_empty() {
                    return Err(CheckpointError::InvalidPlan {
                        reason: "deferred action has an empty provider".to_owned(),
                    });
                }
            }
        }

        for (operation_id, decision) in &self.operation_decisions {
            let descriptor = self
                .operations
                .get(operation_id)
                .ok_or_else(|| CheckpointError::UnknownOperation {
                    operation: operation_id.to_string(),
                })?;

            if decision.recompute && !descriptor.eligibility.eligible {
                return Err(CheckpointError::RecomputeNotEligible {
                    operation: operation_id.to_string(),
                    reason: descriptor
                        .eligibility
                        .reason
                        .clone()
                        .unwrap_or_else(|| "eligibility was denied".to_owned()),
                });
            }

            if &decision.operation != operation_id {
                return Err(CheckpointError::InvalidPlan {
                    reason: "operation decision key does not match its operation id"
                        .to_owned(),
                });
            }
        }

        Ok(())
    }

    /// Computes the total known retained size of selected checkpoints.
    ///
    /// Returns `None` if any retained value has an unknown size or if the
    /// sum cannot be represented by `u128`.
    #[must_use]
    pub fn known_retained_bytes(&self) -> Option<u128> {
        self.value_decisions
            .iter()
            .filter_map(|(value_id, action)| {
                if matches!(action, CheckpointAction::Retain) {
                    self.values
                        .get(value_id)
                        .and_then(|value| value.eligibility.retained_bytes)
                } else {
                    None
                }
            })
            .try_fold(0_u128, |total, size| total.checked_add(size))
    }

    /// Counts explicitly retained values.
    #[must_use]
    pub fn retained_value_count(&self) -> usize {
        self.value_decisions
            .values()
            .filter(|action| matches!(action, CheckpointAction::Retain))
            .count()
    }

    /// Counts explicitly selected recomputation operations.
    #[must_use]
    pub fn recomputation_count(&self) -> usize {
        self.operation_decisions
            .values()
            .filter(|decision| decision.recompute)
            .count()
    }
}

// ============================================================================
// PROVIDER CONTRACT
// ============================================================================

/// A provider that resolves an abstract checkpoint policy into a plan.
///
/// This trait is intentionally independent of the graph, tensor storage,
/// execution engine, and durable checkpoint implementation.
///
/// A provider must validate its output before returning it.
pub trait CheckpointPlanner<ValueId, OperationId>
where
    ValueId: CheckpointValueId,
    OperationId: CheckpointOperationId,
{
    /// Resolves the policy for the supplied value and operation descriptors.
    ///
    /// Implementations should be deterministic for identical inputs and
    /// policy versions unless their contract explicitly declares otherwise.
    fn plan(
        &self,
        policy: &CheckpointPolicy,
        values: &[CheckpointValue<ValueId>],
        operations: &[CheckpointOperation<OperationId>],
    ) -> Result<CheckpointPlan<ValueId, OperationId>, CheckpointError>;
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct ValueId(&'static str);

    impl fmt::Display for ValueId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(self.0)
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct OperationId(&'static str);

    impl fmt::Display for OperationId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(self.0)
        }
    }

    fn value(id: &'static str, bytes: Option<u128>) -> CheckpointValue<ValueId> {
        CheckpointValue {
            id: ValueId(id),
            eligibility: CheckpointEligibility::eligible(bytes),
        }
    }

    fn operation(id: &'static str, work: Option<u128>) -> CheckpointOperation<OperationId> {
        CheckpointOperation {
            id: OperationId(id),
            eligibility: RecomputeEligibility::eligible(work),
        }
    }

    #[test]
    fn accepts_empty_plan() {
        let plan =
            CheckpointPlan::<ValueId, OperationId>::new(CheckpointPolicy::RetainAll)
                .expect("policy is valid");

        assert!(plan.validate().is_ok());
        assert_eq!(plan.retained_value_count(), 0);
        assert_eq!(plan.recomputation_count(), 0);
    }

    #[test]
    fn rejects_empty_extension_policy_id() {
        let result = CheckpointPolicy::Extension {
            policy_id: " ".to_owned(),
            version: None,
        }
        .validate();

        assert!(matches!(
            result,
            Err(CheckpointError::InvalidPolicy { .. })
        ));
    }

    #[test]
    fn registers_and_retains_eligible_value() {
        let mut plan =
            CheckpointPlan::<ValueId, OperationId>::new(CheckpointPolicy::RetainAll)
                .expect("policy is valid");

        let id = ValueId("v");
        plan.add_value(value("v", Some(128)))
            .expect("value registration succeeds");
        plan.set_value_action(id, CheckpointAction::Retain)
            .expect("eligible value can be retained");

        assert!(plan.validate().is_ok());
        assert_eq!(plan.known_retained_bytes(), Some(128));
        assert_eq!(plan.retained_value_count(), 1);
    }

    #[test]
    fn rejects_retaining_ineligible_value() {
        let mut plan =
            CheckpointPlan::<ValueId, OperationId>::new(CheckpointPolicy::RetainAll)
                .expect("policy is valid");

        plan.add_value(CheckpointValue {
            id: ValueId("v"),
            eligibility: CheckpointEligibility::ineligible("externally mutable"),
        })
        .expect("descriptor is valid");

        let result =
            plan.set_value_action(ValueId("v"), CheckpointAction::Retain);

        assert!(matches!(
            result,
            Err(CheckpointError::CheckpointNotEligible { .. })
        ));
    }

    #[test]
    fn rejects_recomputing_ineligible_operation() {
        let mut plan =
            CheckpointPlan::<ValueId, OperationId>::new(
                CheckpointPolicy::RecomputeWhenPossible,
            )
            .expect("policy is valid");

        plan.add_operation(CheckpointOperation {
            id: OperationId("op"),
            eligibility: RecomputeEligibility::ineligible(
                "operation has a non-replayable effect",
            ),
        })
        .expect("descriptor is valid");

        let result =
            plan.set_operation_recompute(OperationId("op"), true, None);

        assert!(matches!(
            result,
            Err(CheckpointError::RecomputeNotEligible { .. })
        ));
    }

    #[test]
    fn rejects_duplicate_value_action() {
        let mut plan =
            CheckpointPlan::<ValueId, OperationId>::new(CheckpointPolicy::RetainAll)
                .expect("policy is valid");

        plan.add_value(value("v", Some(4)))
            .expect("registration succeeds");

        plan.set_value_action(ValueId("v"), CheckpointAction::Retain)
            .expect("first action succeeds");

        let result =
            plan.set_value_action(ValueId("v"), CheckpointAction::External);

        assert!(matches!(
            result,
            Err(CheckpointError::ConflictingAction { .. })
        ));
    }

    #[test]
    fn unknown_value_is_rejected() {
        let mut plan =
            CheckpointPlan::<ValueId, OperationId>::new(CheckpointPolicy::RetainAll)
                .expect("policy is valid");

        let result =
            plan.set_value_action(ValueId("missing"), CheckpointAction::External);

        assert!(matches!(
            result,
            Err(CheckpointError::UnknownValue { .. })
        ));
    }

    #[test]
    fn retained_size_is_unknown_when_any_retained_size_is_unknown() {
        let mut plan =
            CheckpointPlan::<ValueId, OperationId>::new(CheckpointPolicy::RetainAll)
                .expect("policy is valid");

        plan.add_value(value("known", Some(10)))
            .expect("registration succeeds");
        plan.add_value(value("unknown", None))
            .expect("registration succeeds");

        plan.set_value_action(ValueId("known"), CheckpointAction::Retain)
            .expect("action succeeds");
        plan.set_value_action(ValueId("unknown"), CheckpointAction::Retain)
            .expect("action succeeds");

        assert_eq!(plan.known_retained_bytes(), None);
    }

    #[test]
    fn detects_retained_size_overflow_without_panicking() {
        let mut plan =
            CheckpointPlan::<ValueId, OperationId>::new(CheckpointPolicy::RetainAll)
                .expect("policy is valid");

        plan.add_value(value("a", Some(u128::MAX)))
            .expect("registration succeeds");
        plan.add_value(value("b", Some(1)))
            .expect("registration succeeds");

        plan.set_value_action(ValueId("a"), CheckpointAction::Retain)
            .expect("action succeeds");
        plan.set_value_action(ValueId("b"), CheckpointAction::Retain)
            .expect("action succeeds");

        assert_eq!(plan.known_retained_bytes(), None);
    }

    #[test]
    fn returns_values_in_deterministic_identity_order() {
        let mut plan =
            CheckpointPlan::<ValueId, OperationId>::new(CheckpointPolicy::RetainAll)
                .expect("policy is valid");

        plan.add_value(value("z", Some(1)))
            .expect("registration succeeds");
        plan.add_value(value("a", Some(1)))
            .expect("registration succeeds");

        let ids: Vec<_> = plan.values().map(|entry| entry.id.0).collect();

        assert_eq!(ids, vec!["a", "z"]);
    }

    #[test]
    fn budget_policy_accepts_zero_as_an_explicit_budget() {
        let policy = CheckpointPolicy::Budgeted(CheckpointBudget {
            retained_bytes: Some(0),
            checkpoint_count: Some(0),
            recomputation_work: Some(0),
        });

        assert!(policy.validate().is_ok());
    }

    #[test]
    fn validates_operation_decision_consistency() {
        let mut plan =
            CheckpointPlan::<ValueId, OperationId>::new(
                CheckpointPolicy::RecomputeWhenPossible,
            )
            .expect("policy is valid");

        plan.add_operation(operation("op", Some(3)))
            .expect("registration succeeds");

        plan.set_operation_recompute(
            OperationId("op"),
            true,
            Some("replay eligible operation".to_owned()),
        )
        .expect("decision succeeds");

        assert!(plan.validate().is_ok());
        assert_eq!(plan.recomputation_count(), 1);
    }
}