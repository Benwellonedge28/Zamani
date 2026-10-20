
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR Effect Analysis
//!
//! Path: `src/hdl/hdlir/analysis/effects.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//! Status: Independent analysis foundation; freeze after integration tests.
//!
//! ## Purpose
//!
//! Summarize declared hardware-operation effects without mutating the IR.
//! This module provides the stable analysis contract that a future adapter
//! can feed from verified HDLIR operations and registered operation schemas.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/schema/operation.rs`
//! - `src/hdl/hdlir/schema/registry.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! Source syntax and source-level semantic truth remain frontend-owned.
//! ZUIR remains the universal computational IR. HDLIR owns hardware-domain
//! effect analysis. Physical allocation, placement, routing, synthesis,
//! timing closure, and device execution remain downstream.
//!
//! ## Owns
//!
//! - Effect observations and their certainty.
//! - Deterministic read/write/state-transition summaries.
//! - Explicit unknown-effect and incomplete-analysis reporting.
//! - Caller-configurable operational work budgets.
//!
//! ## Does not own
//!
//! - Operation instances, the IR graph, schema registration, or verification.
//! - Source parsing, name resolution, or source-level semantic validation.
//! - Physical resources, device handles, target discovery, or allocation.
//! - A second operation-effect vocabulary.
//! - Proof of behavioral equivalence or timing closure.
//!
//! ## Integration contract
//!
//! `OperationEffect` is reused from `schema::operation`. The current
//! repository does not yet expose a complete HDLIR operation graph, so this
//! module accepts observations through a narrow input contract. A later
//! adapter must translate registered operation contracts and verified IR
//! instances into these observations without guessing missing effects.
//!
//! The adapter must preserve logical resource identity, operation identity,
//! symbolic references, extension-defined effects, and uncertainty.
//! It must not convert an unknown effect into an empty effect set.
//!
//! ## Scalability and determinism
//!
//! There are no hardware-specific maximum counts or widths. An optional
//! work budget limits observations processed by one invocation; it is an
//! operational budget, not a language-level capacity limit.
//!
//! Resource names are opaque logical keys. They are not physical addresses,
//! pins, cells, registers, or device identifiers. Symbolic identities remain
//! symbolic and are not evaluated here.
//!
//! Summaries use ordered sets and checked counters. The same ordered input
//! and configuration produce the same result. Reordering independent input
//! observations does not change set-valued effect summaries, although the
//! observation count remains significant.
//!
//! ## Completion criteria
//!
//! - Unknown effects are never reported as known-empty effects.
//! - Budget exhaustion produces an incomplete summary.
//! - Arithmetic overflow is reported as a structured HDLIR error.
//! - Analysis does not mutate its inputs.
//! - No unsafe code, hardware maxima, or physical-allocation assumptions.
//! - The module is registered only after its sibling dependencies exist.

use std::collections::BTreeSet;

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::schema::operation::OperationEffect;

/// An opaque logical identity for a resource affected by an operation.
///
/// The key may name a logical signal, abstract memory, clock/reset entity,
/// communication endpoint, or an extension-defined resource. It must not
/// be interpreted as a physical resource identifier.
///
/// Keys are compared byte-for-byte. Canonicalization and namespace policy
/// belong to the producer of observations.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ResourceKey(String);

impl ResourceKey {
    /// Construct a resource key.
    ///
    /// Empty keys are rejected because they cannot reliably identify a
    /// logical resource. No assumptions are made about the key's syntax.
    pub fn new(value: impl Into<String>) -> HdlirResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(HdlirError::invalid_input(
                "effect.resource_key",
                "resource identity must not be empty",
            ));
        }

        Ok(Self(value))
    }

    /// Return the opaque resource identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The interpretation supplied for one operation effect.
///
/// `Declared` means the producer has obtained this effect from a registered
/// operation contract or another authoritative, versioned semantic source.
/// It does not mean that the operation has been physically implemented.
///
/// `Unknown` means the producer cannot establish the operation's complete
/// effects. Such an observation makes the analysis incomplete.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EffectKnowledge {
    /// A declared effect from an authoritative operation contract.
    Declared(OperationEffect),

    /// The operation's effect semantics could not be established.
    Unknown {
        /// Stable or producer-defined operation identity.
        operation: String,

        /// Explanation of why the effect is unknown.
        reason: String,
    },
}

/// One effect observation supplied by an IR/schema adapter.
///
/// A missing resource key means the effect is operation-wide or its
/// resource identity is not available. It must not be interpreted as proof
/// that no resource was affected.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectObservation {
    operation: String,
    resource: Option<ResourceKey>,
    knowledge: EffectKnowledge,
}

impl EffectObservation {
    /// Create an observation with a known, declared effect.
    ///
    /// `operation` should identify the IR operation instance or another
    /// stable identity within the analyzed design.
    pub fn declared(
        operation: impl Into<String>,
        resource: Option<ResourceKey>,
        effect: OperationEffect,
    ) -> HdlirResult<Self> {
        let operation = operation.into();

        if operation.is_empty() {
            return Err(HdlirError::invalid_input(
                "effect.operation",
                "operation identity must not be empty",
            ));
        }

        Ok(Self {
            operation,
            resource,
            knowledge: EffectKnowledge::Declared(effect),
        })
    }

    /// Create an observation whose effect semantics are unknown.
    pub fn unknown(
        operation: impl Into<String>,
        resource: Option<ResourceKey>,
        reason: impl Into<String>,
    ) -> HdlirResult<Self> {
        let operation = operation.into();
        let reason = reason.into();

        if operation.is_empty() {
            return Err(HdlirError::invalid_input(
                "effect.operation",
                "operation identity must not be empty",
            ));
        }

        if reason.trim().is_empty() {
            return Err(HdlirError::invalid_input(
                "effect.reason",
                "unknown-effect reason must not be empty",
            ));
        }

        Ok(Self {
            operation: operation.clone(),
            resource,
            knowledge: EffectKnowledge::Unknown { operation, reason },
        })
    }

    /// Return the operation identity.
    #[must_use]
    pub fn operation(&self) -> &str {
        &self.operation
    }

    /// Return the optional logical resource identity.
    #[must_use]
    pub fn resource(&self) -> Option<&ResourceKey> {
        self.resource.as_ref()
    }

    /// Return the effect knowledge supplied by the producer.
    #[must_use]
    pub fn knowledge(&self) -> &EffectKnowledge {
        &self.knowledge
    }
}

/// The completeness state of an effect-analysis result.
///
/// A complete result means every observation supplied to the analysis was
/// processed and no unknown effects were encountered. It does not prove
/// that the producer supplied every effect of every operation: that is the
/// responsibility of the adapter and its verified schema/IR contracts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EffectAnalysisStatus {
    /// All supplied observations were processed and had known effects.
    Complete,

    /// At least one supplied observation had unknown effect semantics.
    UnknownEffects,

    /// Processing stopped because the configured observation budget ended.
    BudgetExhausted,

    /// Both unknown effects and budget exhaustion occurred.
    UnknownEffectsAndBudgetExhausted,
}

impl EffectAnalysisStatus {
    /// Return whether this analysis completed with known effects.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// A resource-specific effect.
///
/// The set-valued representation allows multiple operations to affect the
/// same logical resource without imposing an arbitrary maximum number of
/// readers, writers, or state transitions.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ResourceEffect {
    resource: ResourceKey,
    effect: OperationEffect,
}

impl ResourceEffect {
    /// Return the affected logical resource.
    #[must_use]
    pub fn resource(&self) -> &ResourceKey {
        &self.resource
    }

    /// Return the declared effect category.
    #[must_use]
    pub const fn effect(&self) -> OperationEffect {
        self.effect
    }
}

/// A deterministic summary of effects observed in a design or subgraph.
///
/// This type is descriptive. It does not schedule operations, mutate the
/// design, establish physical implementation, or prove equivalence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectSummary {
    status: EffectAnalysisStatus,
    observations_processed: u64,
    reads: BTreeSet<ResourceKey>,
    writes: BTreeSet<ResourceKey>,
    state_transitions: BTreeSet<ResourceKey>,
    other_effects: BTreeSet<ResourceEffect>,
    operation_wide_effects: BTreeSet<OperationEffect>,
    unknown_operations: BTreeSet<String>,
}

impl EffectSummary {
    fn new() -> Self {
        Self {
            status: EffectAnalysisStatus::Complete,
            observations_processed: 0,
            reads: BTreeSet::new(),
            writes: BTreeSet::new(),
            state_transitions: BTreeSet::new(),
            other_effects: BTreeSet::new(),
            operation_wide_effects: BTreeSet::new(),
            unknown_operations: BTreeSet::new(),
        }
    }

    /// Return the completeness status.
    #[must_use]
    pub fn status(&self) -> &EffectAnalysisStatus {
        &self.status
    }

    /// Return whether all processed observations were known and all supplied
    /// observations were processed.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.status.is_complete()
    }

    /// Return the number of observations successfully processed.
    #[must_use]
    pub const fn observations_processed(&self) -> u64 {
        self.observations_processed
    }

    /// Return the logical resources read by at least one declared effect.
    #[must_use]
    pub fn reads(&self) -> &BTreeSet<ResourceKey> {
        &self.reads
    }

    /// Return the logical resources written by at least one declared effect.
    #[must_use]
    pub fn writes(&self) -> &BTreeSet<ResourceKey> {
        &self.writes
    }

    /// Return the logical resources with declared state-transition effects.
    #[must_use]
    pub fn state_transitions(&self) -> &BTreeSet<ResourceKey> {
        &self.state_transitions
    }

    /// Return resource-specific effects not covered by the dedicated read,
    /// write, or state-transition sets.
    #[must_use]
    pub fn other_effects(&self) -> &BTreeSet<ResourceEffect> {
        &self.other_effects
    }

    /// Return effects whose resource identity was not supplied.
    ///
    /// These effects remain visible and must not be treated as resource-free.
    #[must_use]
    pub fn operation_wide_effects(&self) -> &BTreeSet<OperationEffect> {
        &self.operation_wide_effects
    }

    /// Return operation identities for which effects could not be established.
    #[must_use]
    pub fn unknown_operations(&self) -> &BTreeSet<String> {
        &self.unknown_operations
    }

    fn record(
        &mut self,
        observation: EffectObservation,
    ) -> HdlirResult<()> {
        self.observations_processed = self
            .observations_processed
            .checked_add(1)
            .ok_or_else(|| HdlirError::ArithmeticOverflow {
                operation: "counting effect observations",
                context: Some("EffectSummary.observations_processed".to_owned()),
            })?;

        match observation.knowledge {
            EffectKnowledge::Unknown { operation, .. } => {
                self.unknown_operations.insert(operation);
                self.refresh_status(false);
            }
            EffectKnowledge::Declared(effect) => {
                match observation.resource {
                    Some(resource) => match effect {
                        OperationEffect::Read | OperationEffect::MemoryAccess => {
                            self.reads.insert(resource.clone());
                            self.other_effects.insert(ResourceEffect {
                                resource,
                                effect,
                            });
                        }
                        OperationEffect::Write => {
                            self.writes.insert(resource.clone());
                            self.other_effects.insert(ResourceEffect {
                                resource,
                                effect,
                            });
                        }
                        OperationEffect::StateTransition => {
                            self.state_transitions.insert(resource.clone());
                            self.other_effects.insert(ResourceEffect {
                                resource,
                                effect,
                            });
                        }
                        _ => {
                            self.other_effects.insert(ResourceEffect {
                                resource,
                                effect,
                            });
                        }
                    },
                    None => {
                        self.operation_wide_effects.insert(effect);
                    }
                }
            }
        }

        Ok(())
    }

    fn refresh_status(&mut self, budget_exhausted: bool) {
        let has_unknown = !self.unknown_operations.is_empty();

        self.status = match (has_unknown, budget_exhausted) {
            (false, false) => EffectAnalysisStatus::Complete,
            (true, false) => EffectAnalysisStatus::UnknownEffects,
            (false, true) => EffectAnalysisStatus::BudgetExhausted,
            (true, true) => EffectAnalysisStatus::UnknownEffectsAndBudgetExhausted,
        };
    }
}

/// Operational controls for one effect-analysis invocation.
///
/// `max_observations = None` means no caller-imposed observation-count budget.
/// The process can still be constrained by available memory, address space,
/// execution time, and other operating-system or application limits.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EffectAnalysisConfig {
    max_observations: Option<u64>,
}

impl EffectAnalysisConfig {
    /// Create a configuration without an explicit observation-count budget.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_observations: None,
        }
    }

    /// Create a configuration with a caller-specified observation budget.
    ///
    /// A budget of zero processes no observations. This is an operational
    /// limit and does not mean that a design with observations is invalid.
    #[must_use]
    pub const fn with_max_observations(max_observations: u64) -> Self {
        Self {
            max_observations: Some(max_observations),
        }
    }

    /// Return the configured observation budget, if any.
    #[must_use]
    pub const fn max_observations(self) -> Option<u64> {
        self.max_observations
    }
}

/// Analyze an iterator of effect observations.
///
/// The iterator is consumed only as needed. If a budget is configured, the
/// analyzer probes for one additional item to distinguish an exactly-sized
/// input from an input that was truncated by the budget. That item is not
/// processed or included in the summary.
///
/// Unknown effects are retained as unknown operations and make the result
/// incomplete. A complete summary only certifies the observations supplied
/// by the producer; it does not certify that the producer enumerated all
/// effects in the underlying IR.
///
/// # Errors
///
/// Returns a structured HDLIR error if the processed-observation counter
/// overflows. Budget exhaustion and unknown effects are represented in the
/// returned summary rather than converted into success or a generic error.
pub fn analyze_effects<I>(
    observations: I,
    config: EffectAnalysisConfig,
) -> HdlirResult<EffectSummary>
where
    I: IntoIterator<Item = EffectObservation>,
{
    let mut summary = EffectSummary::new();
    let mut iterator = observations.into_iter();

    loop {
        if let Some(limit) = config.max_observations {
            if summary.observations_processed >= limit {
                if iterator.next().is_some() {
                    summary.refresh_status(true);
                }
                break;
            }
        }

        match iterator.next() {
            Some(observation) => summary.record(observation)?,
            None => break,
        }
    }

    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resource(name: &str) -> ResourceKey {
        ResourceKey::new(name).expect("resource key must be valid")
    }

    fn declared(
        operation: &str,
        resource: Option<ResourceKey>,
        effect: OperationEffect,
    ) -> EffectObservation {
        EffectObservation::declared(operation, resource, effect)
            .expect("declared observation must be valid")
    }

    #[test]
    fn summarizes_reads_writes_and_state_transitions() {
        let observations = vec![
            declared(
                "op.read",
                Some(resource("signal.data")),
                OperationEffect::Read,
            ),
            declared(
                "op.write",
                Some(resource("signal.result")),
                OperationEffect::Write,
            ),
            declared(
                "op.register",
                Some(resource("register.state")),
                OperationEffect::StateTransition,
            ),
        ];

        let summary = analyze_effects(
            observations,
            EffectAnalysisConfig::unlimited(),
        )
        .expect("analysis must succeed");

        assert!(summary.is_complete());
        assert_eq!(summary.observations_processed(), 3);
        assert!(summary.reads().contains(&resource("signal.data")));
        assert!(summary.writes().contains(&resource("signal.result")));
        assert!(
            summary
                .state_transitions()
                .contains(&resource("register.state"))
        );
    }

    #[test]
    fn unknown_effects_never_look_like_known_empty_effects() {
        let unknown = EffectObservation::unknown(
            "extension.operation",
            None,
            "No authoritative effect contract is registered.",
        )
        .expect("unknown observation must be valid");

        let summary = analyze_effects(
            [unknown],
            EffectAnalysisConfig::unlimited(),
        )
        .expect("analysis should return an incomplete summary");

        assert!(!summary.is_complete());
        assert_eq!(
            summary.status(),
            &EffectAnalysisStatus::UnknownEffects
        );
        assert!(
            summary
                .unknown_operations()
                .contains("extension.operation")
        );
    }

    #[test]
    fn budget_exhaustion_is_explicit() {
        let observations = [
            declared("op.a", None, OperationEffect::Read),
            declared("op.b", None, OperationEffect::Write),
        ];

        let summary = analyze_effects(
            observations,
            EffectAnalysisConfig::with_max_observations(1),
        )
        .expect("budget exhaustion is a summary status");

        assert_eq!(summary.observations_processed(), 1);
        assert_eq!(
            summary.status(),
            &EffectAnalysisStatus::BudgetExhausted
        );
        assert!(!summary.is_complete());
    }

    #[test]
    fn exact_budget_does_not_report_false_exhaustion() {
        let observations = [
            declared("op.a", None, OperationEffect::Read),
            declared("op.b", None, OperationEffect::Write),
        ];

        let summary = analyze_effects(
            observations,
            EffectAnalysisConfig::with_max_observations(2),
        )
        .expect("exactly fitting input must complete");

        assert_eq!(summary.observations_processed(), 2);
        assert!(summary.is_complete());
    }

    #[test]
    fn zero_budget_processes_no_observations() {
        let observations = [
            declared("op.a", None, OperationEffect::Read),
        ];

        let summary = analyze_effects(
            observations,
            EffectAnalysisConfig::with_max_observations(0),
        )
        .expect("budget exhaustion must be reported in the summary");

        assert_eq!(summary.observations_processed(), 0);
        assert_eq!(
            summary.status(),
            &EffectAnalysisStatus::BudgetExhausted
        );
    }

    #[test]
    fn operation_wide_effects_are_not_discarded() {
        let summary = analyze_effects(
            [declared("op.clock", None, OperationEffect::ClockOrReset)],
            EffectAnalysisConfig::unlimited(),
        )
        .expect("analysis must succeed");

        assert!(
            summary
                .operation_wide_effects()
                .contains(&OperationEffect::ClockOrReset)
        );
        assert!(summary.is_complete());
    }

    #[test]
    fn memory_access_is_retained_as_a_read_and_as_its_specific_effect() {
        let key = resource("memory.main");

        let summary = analyze_effects(
            [declared(
                "op.memory_read",
                Some(key.clone()),
                OperationEffect::MemoryAccess,
            )],
            EffectAnalysisConfig::unlimited(),
        )
        .expect("analysis must succeed");

        assert!(summary.reads().contains(&key));
        assert!(summary.other_effects().iter().any(|effect| {
            effect.resource() == &key
                && effect.effect() == OperationEffect::MemoryAccess
        }));
    }

    #[test]
    fn empty_resource_keys_are_rejected() {
        assert!(ResourceKey::new("").is_err());
    }

    #[test]
    fn empty_operation_identity_is_rejected() {
        assert!(
            EffectObservation::declared(
                "",
                None,
                OperationEffect::Read,
            )
            .is_err()
        );
    }

    #[test]
    fn empty_unknown_effect_reason_is_rejected() {
        assert!(
            EffectObservation::unknown("op.unknown", None, "  ").is_err()
        );
    }

    #[test]
    fn summary_sets_are_deterministic_and_deduplicated() {
        let first = analyze_effects(
            [
                declared(
                    "op.read.1",
                    Some(resource("signal.a")),
                    OperationEffect::Read,
                ),
                declared(
                    "op.read.2",
                    Some(resource("signal.a")),
                    OperationEffect::Read,
                ),
            ],
            EffectAnalysisConfig::unlimited(),
        )
        .expect("analysis must succeed");

        let second = analyze_effects(
            [
                declared(
                    "op.read.2",
                    Some(resource("signal.a")),
                    OperationEffect::Read,
                ),
                declared(
                    "op.read.1",
                    Some(resource("signal.a")),
                    OperationEffect::Read,
                ),
            ],
            EffectAnalysisConfig::unlimited(),
        )
        .expect("analysis must succeed");

        assert_eq!(first.reads(), second.reads());
        assert_eq!(first.other_effects(), second.other_effects());
        assert_eq!(first.observations_processed(), 2);
    }
}
