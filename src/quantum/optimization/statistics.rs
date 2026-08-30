//! Zamani Quantum Optimization — Production Statistics
//!
//! This module defines the canonical statistics model for the quantum
//! optimization subsystem.
//!
//! # Architectural role
//!
//! `statistics.rs` is deliberately independent from optimization passes,
//! pipelines, analyses, targets, cost models, verification, and the canonical
//! Quantum IR implementation.
//!
//! Its responsibility is to provide a stable, lossless, composable accounting
//! model for optimization activity.
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! optimization passes
//!      │
//!      ▼
//! statistics
//!      │
//!      ├──────────────► result
//!      ├──────────────► provenance
//!      ├──────────────► reporting
//!      └──────────────► benchmarking
//! ```
//!
//! `statistics.rs` must NOT depend on any of those downstream consumers.
//!
//! # Design goals
//!
//! This implementation is designed for:
//!
//! - tiny circuits;
//! - large circuits;
//! - very large compilation workloads;
//! - repeated optimization;
//! - fixed-point pipelines;
//! - parallel compilation;
//! - deterministic compilation;
//! - incremental optimization;
//! - fault-tolerant optimization;
//! - synthesis;
//! - e-graph/equality-saturation optimization;
//! - verification;
//! - resource accounting;
//! - future optimization techniques not yet present in Zamani.
//!
//! # Important invariant
//!
//! Statistics are observational data. They must never influence the semantic
//! result of an optimization pass.
//!
//! In particular:
//!
//! - recording statistics must not mutate the circuit;
//! - statistics must not require the canonical Quantum IR;
//! - statistics must not perform backend I/O;
//! - statistics must not allocate global state;
//! - statistics must not use `unsafe`;
//! - statistics must not panic because a counter reached its representable
//!   maximum;
//! - statistics must be mergeable without losing information.
//!
//! # Overflow policy
//!
//! Optimization workloads can be arbitrarily large relative to machine
//! resources. Statistics therefore use `u128` for monotonically increasing
//! quantities.
//!
//! Individual checked arithmetic operations are provided for callers that
//! require strict accounting. The default accumulation path saturates rather
//! than panics. This means statistics collection can never turn a successful
//! optimization into a process crash solely because an accounting counter
//! overflowed.
//!
//! # Rust compatibility
//!
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no `unsafe`
//!
//! # Serialization
//!
//! The public data structures derive `Serialize` and `Deserialize` because
//! optimization statistics are expected to be consumed by:
//!
//! - optimization results;
//! - diagnostics;
//! - provenance;
//! - benchmarking;
//! - JSON reports;
//! - reproducibility tooling;
//! - CI regression detection.
//!
//! Serialization remains representation-only. No file I/O is performed here.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::fmt;

// =============================================================================
// Primitive identifiers
// =============================================================================

/// Stable identifier for an optimization pass.
///
/// Pass identifiers are intentionally represented as owned strings rather
/// than depending on a future `pass.rs` type. This keeps statistics independent
/// from the pass framework and allows external/plugin passes to report
/// statistics without changing this module.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PassId(String);

impl PassId {
    /// Creates a pass identifier.
    ///
    /// Empty identifiers are rejected.
    pub fn new(value: impl Into<String>) -> Result<Self, StatisticsError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(StatisticsError::EmptyIdentifier {
                kind: IdentifierKind::Pass,
            });
        }

        Ok(Self(value))
    }

    /// Creates a pass identifier without validation.
    ///
    /// This is intended for compile-time/static identifiers controlled by
    /// Zamani's own implementation.
    ///
    /// The caller must guarantee that the value is non-empty.
    pub fn from_static(value: &'static str) -> Self {
        debug_assert!(!value.trim().is_empty());
        Self(value.to_owned())
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for PassId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for PassId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Stable identifier for a rewrite rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RuleId(String);

impl RuleId {
    /// Creates a validated rule identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, StatisticsError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(StatisticsError::EmptyIdentifier {
                kind: IdentifierKind::Rule,
            });
        }

        Ok(Self(value))
    }

    /// Creates a rule identifier from a trusted static value.
    pub fn from_static(value: &'static str) -> Self {
        debug_assert!(!value.trim().is_empty());
        Self(value.to_owned())
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for RuleId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RuleId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Identifier for a statistics collection session.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StatisticsId(String);

impl StatisticsId {
    /// Creates a statistics identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, StatisticsError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(StatisticsError::EmptyIdentifier {
                kind: IdentifierKind::Statistics,
            });
        }

        Ok(Self(value))
    }

    /// Creates an identifier from a trusted static value.
    pub fn from_static(value: &'static str) -> Self {
        debug_assert!(!value.trim().is_empty());
        Self(value.to_owned())
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for StatisticsId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for StatisticsId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Category of identifier that failed validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentifierKind {
    /// Optimization pass.
    Pass,

    /// Rewrite rule.
    Rule,

    /// Statistics collection.
    Statistics,
}

impl fmt::Display for IdentifierKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pass => formatter.write_str("pass"),
            Self::Rule => formatter.write_str("rule"),
            Self::Statistics => formatter.write_str("statistics"),
        }
    }
}

/// Errors produced by statistics construction and strict arithmetic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatisticsError {
    /// An identifier was empty.
    EmptyIdentifier {
        /// Identifier category.
        kind: IdentifierKind,
    },

    /// A checked arithmetic operation overflowed.
    ArithmeticOverflow {
        /// Human-readable description of the operation.
        operation: &'static str,
    },

    /// An invalid value was supplied.
    InvalidValue {
        /// Field that was invalid.
        field: &'static str,

        /// Human-readable explanation.
        reason: &'static str,
    },
}

impl fmt::Display for StatisticsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { kind } => {
                write!(formatter, "{kind} identifier must not be empty")
            }

            Self::ArithmeticOverflow { operation } => {
                write!(formatter, "statistics arithmetic overflow: {operation}")
            }

            Self::InvalidValue { field, reason } => {
                write!(formatter, "invalid statistics value for {field}: {reason}")
            }
        }
    }
}

impl std::error::Error for StatisticsError {}

/// Result type used by strict statistics operations.
pub type StatisticsResult<T> = Result<T, StatisticsError>;

// =============================================================================
// Saturating counter
// =============================================================================

/// A monotonic counter used by optimization statistics.
///
/// Counters saturate at `u128::MAX` when using the normal accumulation APIs.
/// Saturation is explicitly observable through [`SaturatingCounter::is_saturated`].
///
/// This avoids a statistics-related panic on extremely large workloads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SaturatingCounter {
    value: u128,
    saturated: bool,
}

impl Default for SaturatingCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl SaturatingCounter {
    /// Creates an empty counter.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            value: 0,
            saturated: false,
        }
    }

    /// Creates a counter from an initial value.
    #[must_use]
    pub const fn from_value(value: u128) -> Self {
        Self {
            value,
            saturated: false,
        }
    }

    /// Returns the current value.
    #[must_use]
    pub const fn value(&self) -> u128 {
        self.value
    }

    /// Returns true when the counter has saturated.
    #[must_use]
    pub const fn is_saturated(&self) -> bool {
        self.saturated
    }

    /// Adds a value using saturating semantics.
    pub fn add(&mut self, amount: u128) {
        match self.value.checked_add(amount) {
            Some(value) => {
                self.value = value;
            }
            None => {
                self.value = u128::MAX;
                self.saturated = true;
            }
        }
    }

    /// Increments the counter.
    pub fn increment(&mut self) {
        self.add(1);
    }

    /// Adds another counter using saturating semantics.
    pub fn merge(&mut self, other: Self) {
        self.add(other.value);

        if other.saturated {
            self.value = u128::MAX;
            self.saturated = true;
        }
    }

    /// Performs strict checked addition.
    pub fn checked_add(
        &self,
        amount: u128,
    ) -> StatisticsResult<Self> {
        let value = self.value.checked_add(amount).ok_or(
            StatisticsError::ArithmeticOverflow {
                operation: "counter addition",
            },
        )?;

        Ok(Self {
            value,
            saturated: self.saturated,
        })
    }
}

// =============================================================================
// Optimization phase
// =============================================================================

/// Broad category of optimization work.
///
/// This classification is intentionally broader than individual pass names so
/// future passes can report statistics without requiring a change to this
/// module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptimizationPhase {
    /// Input validation.
    Validation,

    /// Canonicalization/normalization.
    Normalization,

    /// Local transformations.
    Local,

    /// Algebraic transformations.
    Algebraic,

    /// Parameter optimization.
    Parameter,

    /// Clifford optimization.
    Clifford,

    /// Phase-polynomial optimization.
    PhasePolynomial,

    /// Fault-tolerant optimization.
    FaultTolerant,

    /// Circuit synthesis.
    Synthesis,

    /// Structural/control-flow optimization.
    Structural,

    /// Target-aware logical optimization.
    TargetAware,

    /// Search/equality saturation.
    Search,

    /// Verification.
    Verification,

    /// Statistics-only analysis.
    Analysis,

    /// Other optimization work.
    Other,
}

// =============================================================================
// Pass outcome
// =============================================================================

/// Result classification for an optimization pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PassOutcome {
    /// Pass completed and changed the circuit.
    Changed,

    /// Pass completed but did not change the circuit.
    Unchanged,

    /// Pass was intentionally skipped.
    Skipped,

    /// Pass stopped because a configured resource limit was reached.
    LimitReached,

    /// Pass could not complete because verification failed.
    VerificationFailed,

    /// Pass completed only partially.
    PartiallyCompleted,

    /// Pass failed before producing a valid optimization result.
    Failed,
}

impl PassOutcome {
    /// Returns true when the pass changed the circuit.
    #[must_use]
    pub const fn changed(self) -> bool {
        matches!(
            self,
            Self::Changed | Self::PartiallyCompleted
        )
    }

    /// Returns true when the pass completed without an error state.
    #[must_use]
    pub const fn completed(self) -> bool {
        matches!(
            self,
            Self::Changed
                | Self::Unchanged
                | Self::Skipped
                | Self::LimitReached
                | Self::PartiallyCompleted
        )
    }
}

// =============================================================================
// Operation accounting
// =============================================================================

/// Operation-count accounting.
///
/// These fields are deliberately generic and do not depend on the canonical
/// Quantum IR's `GateKind`. The optimizer can map its IR into these counters
/// without coupling this module to the IR implementation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationStatistics {
    /// Total operations.
    pub operations: SaturatingCounter,

    /// One-qubit operations.
    pub single_qubit_operations: SaturatingCounter,

    /// Two-qubit operations.
    pub two_qubit_operations: SaturatingCounter,

    /// Operations involving three or more qubits.
    pub multi_qubit_operations: SaturatingCounter,

    /// Unitary operations.
    pub unitary_operations: SaturatingCounter,

    /// Non-unitary operations.
    pub non_unitary_operations: SaturatingCounter,

    /// Parameterized operations.
    pub parameterized_operations: SaturatingCounter,

    /// Measurement operations.
    pub measurement_operations: SaturatingCounter,

    /// Reset operations.
    pub reset_operations: SaturatingCounter,

    /// Barrier operations.
    pub barrier_operations: SaturatingCounter,

    /// Identity operations.
    pub identity_operations: SaturatingCounter,
}

impl OperationStatistics {
    /// Records an operation.
    pub fn record_operation(
        &mut self,
        arity: usize,
        unitary: bool,
        parameterized: bool,
    ) {
        self.operations.increment();

        match arity {
            0 | 1 => self.single_qubit_operations.increment(),
            2 => self.two_qubit_operations.increment(),
            _ => self.multi_qubit_operations.increment(),
        }

        if unitary {
            self.unitary_operations.increment();
        } else {
            self.non_unitary_operations.increment();
        }

        if parameterized {
            self.parameterized_operations.increment();
        }
    }

    /// Records a measurement.
    pub fn record_measurement(&mut self) {
        self.measurement_operations.increment();
        self.non_unitary_operations.increment();
        self.operations.increment();
    }

    /// Records a reset.
    pub fn record_reset(&mut self) {
        self.reset_operations.increment();
        self.non_unitary_operations.increment();
        self.operations.increment();
    }

    /// Records a barrier.
    pub fn record_barrier(&mut self) {
        self.barrier_operations.increment();
        self.non_unitary_operations.increment();
        self.operations.increment();
    }

    /// Records an identity.
    pub fn record_identity(&mut self) {
        self.identity_operations.increment();
        self.operations.increment();
    }

    /// Merges another operation-statistics value.
    pub fn merge(&mut self, other: &Self) {
        self.operations.merge(other.operations);
        self.single_qubit_operations
            .merge(other.single_qubit_operations);
        self.two_qubit_operations
            .merge(other.two_qubit_operations);
        self.multi_qubit_operations
            .merge(other.multi_qubit_operations);
        self.unitary_operations
            .merge(other.unitary_operations);
        self.non_unitary_operations
            .merge(other.non_unitary_operations);
        self.parameterized_operations
            .merge(other.parameterized_operations);
        self.measurement_operations
            .merge(other.measurement_operations);
        self.reset_operations.merge(other.reset_operations);
        self.barrier_operations.merge(other.barrier_operations);
        self.identity_operations
            .merge(other.identity_operations);
    }
}

// =============================================================================
// Transformation accounting
// =============================================================================

/// Accounting for circuit transformations.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransformationStatistics {
    /// Operations removed.
    pub operations_removed: SaturatingCounter,

    /// Operations inserted.
    pub operations_added: SaturatingCounter,

    /// Operations replaced.
    pub operations_replaced: SaturatingCounter,

    /// Operations moved.
    pub operations_moved: SaturatingCounter,

    /// Operations fused.
    pub operations_fused: SaturatingCounter,

    /// Operations decomposed.
    pub operations_decomposed: SaturatingCounter,

    /// Identity operations removed.
    pub identities_removed: SaturatingCounter,

    /// Inverse pairs cancelled.
    pub inverse_pairs_cancelled: SaturatingCounter,

    /// Rotation combinations performed.
    pub rotations_combined: SaturatingCounter,

    /// Template rewrites applied.
    pub template_rewrites: SaturatingCounter,

    /// Commutation transformations performed.
    pub commutations: SaturatingCounter,

    /// Gate decompositions performed.
    pub decompositions: SaturatingCounter,

    /// Synthesis operations performed.
    pub synthesis_operations: SaturatingCounter,
}

impl TransformationStatistics {
    /// Records a removed operation.
    pub fn record_removed(&mut self, count: u128) {
        self.operations_removed.add(count);
    }

    /// Records an inserted operation.
    pub fn record_added(&mut self, count: u128) {
        self.operations_added.add(count);
    }

    /// Records a replacement.
    pub fn record_replacement(&mut self, count: u128) {
        self.operations_replaced.add(count);
    }

    /// Records a moved operation.
    pub fn record_moved(&mut self, count: u128) {
        self.operations_moved.add(count);
    }

    /// Records gate fusion.
    pub fn record_fusion(&mut self, count: u128) {
        self.operations_fused.add(count);
    }

    /// Records decomposition.
    pub fn record_decomposition(&mut self, count: u128) {
        self.operations_decomposed.add(count);
        self.decompositions.add(count);
    }

    /// Records identity removal.
    pub fn record_identity_removal(&mut self, count: u128) {
        self.identities_removed.add(count);
        self.operations_removed.add(count);
    }

    /// Records inverse cancellation.
    pub fn record_inverse_cancellation(&mut self, pairs: u128) {
        self.inverse_pairs_cancelled.add(pairs);

        let removed = pairs.saturating_mul(2);
        self.operations_removed.add(removed);
    }

    /// Records rotation combination.
    pub fn record_rotation_combination(&mut self) {
        self.rotations_combined.increment();
        self.operations_replaced.increment();
    }

    /// Records a template rewrite.
    pub fn record_template_rewrite(&mut self) {
        self.template_rewrites.increment();
    }

    /// Records a commutation transformation.
    pub fn record_commutation(&mut self) {
        self.commutations.increment();
    }

    /// Records synthesis.
    pub fn record_synthesis(&mut self) {
        self.synthesis_operations.increment();
    }

    /// Merges another transformation statistics value.
    pub fn merge(&mut self, other: &Self) {
        self.operations_removed.merge(other.operations_removed);
        self.operations_added.merge(other.operations_added);
        self.operations_replaced
            .merge(other.operations_replaced);
        self.operations_moved.merge(other.operations_moved);
        self.operations_fused.merge(other.operations_fused);
        self.operations_decomposed
            .merge(other.operations_decomposed);
        self.identities_removed
            .merge(other.identities_removed);
        self.inverse_pairs_cancelled
            .merge(other.inverse_pairs_cancelled);
        self.rotations_combined
            .merge(other.rotations_combined);
        self.template_rewrites
            .merge(other.template_rewrites);
        self.commutations.merge(other.commutations);
        self.decompositions.merge(other.decompositions);
        self.synthesis_operations
            .merge(other.synthesis_operations);
    }
}

// =============================================================================
// Resource accounting
// =============================================================================

/// Resource-oriented optimization statistics.
///
/// These counters are intentionally independent of the target hardware
/// abstraction. They describe logical resources and optimizer work.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceStatistics {
    /// Logical qubits observed.
    pub logical_qubits: SaturatingCounter,

    /// Ancillas observed.
    pub ancillas: SaturatingCounter,

    /// Peak live qubits.
    pub peak_live_qubits: SaturatingCounter,

    /// Logical circuit depth before optimization.
    pub depth_before: SaturatingCounter,

    /// Logical circuit depth after optimization.
    pub depth_after: SaturatingCounter,

    /// Two-qubit depth before optimization.
    pub two_qubit_depth_before: SaturatingCounter,

    /// Two-qubit depth after optimization.
    pub two_qubit_depth_after: SaturatingCounter,

    /// T gates before optimization.
    pub t_count_before: SaturatingCounter,

    /// T gates after optimization.
    pub t_count_after: SaturatingCounter,

    /// T depth before optimization.
    pub t_depth_before: SaturatingCounter,

    /// T depth after optimization.
    pub t_depth_after: SaturatingCounter,

    /// Measurement operations before optimization.
    pub measurements_before: SaturatingCounter,

    /// Measurement operations after optimization.
    pub measurements_after: SaturatingCounter,

    /// Reset operations before optimization.
    pub resets_before: SaturatingCounter,

    /// Reset operations after optimization.
    pub resets_after: SaturatingCounter,

    /// Two-qubit operations before optimization.
    pub two_qubit_operations_before: SaturatingCounter,

    /// Two-qubit operations after optimization.
    pub two_qubit_operations_after: SaturatingCounter,
}

impl ResourceStatistics {
    /// Merges another resource statistics value.
    pub fn merge(&mut self, other: &Self) {
        self.logical_qubits.merge(other.logical_qubits);
        self.ancillas.merge(other.ancillas);
        self.peak_live_qubits
            .merge(other.peak_live_qubits);
        self.depth_before.merge(other.depth_before);
        self.depth_after.merge(other.depth_after);
        self.two_qubit_depth_before
            .merge(other.two_qubit_depth_before);
        self.two_qubit_depth_after
            .merge(other.two_qubit_depth_after);
        self.t_count_before.merge(other.t_count_before);
        self.t_count_after.merge(other.t_count_after);
        self.t_depth_before.merge(other.t_depth_before);
        self.t_depth_after.merge(other.t_depth_after);
        self.measurements_before
            .merge(other.measurements_before);
        self.measurements_after
            .merge(other.measurements_after);
        self.resets_before.merge(other.resets_before);
        self.resets_after.merge(other.resets_after);
        self.two_qubit_operations_before
            .merge(other.two_qubit_operations_before);
        self.two_qubit_operations_after
            .merge(other.two_qubit_operations_after);
    }
}

// =============================================================================
// Rewrite accounting
// =============================================================================

/// Statistics for rewrite-rule activity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RewriteStatistics {
    /// Rule identifier.
    pub rule_id: RuleId,

    /// Number of times the rule matched.
    pub matches: SaturatingCounter,

    /// Number of times the rule was successfully applied.
    pub applications: SaturatingCounter,

    /// Number of times the rule was rejected after matching.
    pub rejected_matches: SaturatingCounter,

    /// Number of operations removed by the rule.
    pub operations_removed: SaturatingCounter,

    /// Number of operations added by the rule.
    pub operations_added: SaturatingCounter,
}

impl RewriteStatistics {
    /// Creates statistics for a rule.
    pub fn new(rule_id: RuleId) -> Self {
        Self {
            rule_id,
            matches: SaturatingCounter::default(),
            applications: SaturatingCounter::default(),
            rejected_matches: SaturatingCounter::default(),
            operations_removed: SaturatingCounter::default(),
            operations_added: SaturatingCounter::default(),
        }
    }

    /// Records a match.
    pub fn record_match(&mut self) {
        self.matches.increment();
    }

    /// Records a rejected match.
    pub fn record_rejected_match(&mut self) {
        self.rejected_matches.increment();
    }

    /// Records an application.
    pub fn record_application(
        &mut self,
        removed: u128,
        added: u128,
    ) {
        self.applications.increment();
        self.operations_removed.add(removed);
        self.operations_added.add(added);
    }
}

// =============================================================================
// Verification statistics
// =============================================================================

/// Verification method used for optimization-result validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerificationMethod {
    /// No verification.
    None,

    /// Structural IR validation.
    Structural,

    /// Exact unitary equivalence.
    ExactUnitary,

    /// Equivalence up to global phase.
    UpToGlobalPhase,

    /// Statevector comparison.
    Statevector,

    /// Measurement-distribution comparison.
    MeasurementDistribution,

    /// Observable-equivalence checking.
    Observable,

    /// Exhaustive small-circuit verification.
    Exhaustive,

    /// Randomized differential verification.
    Randomized,

    /// Approximate/probabilistic verification.
    Probabilistic,

    /// External certificate verification.
    Certificate,
}

/// Result of semantic verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerificationOutcome {
    /// Verification succeeded.
    Passed,

    /// Verification failed.
    Failed,

    /// Verification was skipped.
    Skipped,

    /// Verification could not complete within configured limits.
    LimitReached,

    /// Verification was inconclusive.
    Inconclusive,
}

/// Statistics describing verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationStatistics {
    /// Number of verification checks.
    pub checks: SaturatingCounter,

    /// Number of successful checks.
    pub passed: SaturatingCounter,

    /// Number of failed checks.
    pub failed: SaturatingCounter,

    /// Number of skipped checks.
    pub skipped: SaturatingCounter,

    /// Number of checks stopped by limits.
    pub limit_reached: SaturatingCounter,

    /// Number of inconclusive checks.
    pub inconclusive: SaturatingCounter,

    /// Total verification work units.
    pub work_units: SaturatingCounter,

    /// Verification method of the most recent check.
    pub last_method: VerificationMethod,

    /// Outcome of the most recent check.
    pub last_outcome: VerificationOutcome,
}

impl Default for VerificationStatistics {
    fn default() -> Self {
        Self {
            checks: SaturatingCounter::default(),
            passed: SaturatingCounter::default(),
            failed: SaturatingCounter::default(),
            skipped: SaturatingCounter::default(),
            limit_reached: SaturatingCounter::default(),
            inconclusive: SaturatingCounter::default(),
            work_units: SaturatingCounter::default(),
            last_method: VerificationMethod::None,
            last_outcome: VerificationOutcome::Skipped,
        }
    }
}

impl VerificationStatistics {
    /// Records a verification event.
    pub fn record(
        &mut self,
        method: VerificationMethod,
        outcome: VerificationOutcome,
        work_units: u128,
    ) {
        self.checks.increment();
        self.work_units.add(work_units);
        self.last_method = method;
        self.last_outcome = outcome;

        match outcome {
            VerificationOutcome::Passed => {
                self.passed.increment();
            }
            VerificationOutcome::Failed => {
                self.failed.increment();
            }
            VerificationOutcome::Skipped => {
                self.skipped.increment();
            }
            VerificationOutcome::LimitReached => {
                self.limit_reached.increment();
            }
            VerificationOutcome::Inconclusive => {
                self.inconclusive.increment();
            }
        }
    }

    /// Merges verification statistics.
    ///
    /// The last method/outcome from `other` become the last method/outcome when
    /// `other` contains at least one check.
    pub fn merge(&mut self, other: &Self) {
        self.checks.merge(other.checks);
        self.passed.merge(other.passed);
        self.failed.merge(other.failed);
        self.skipped.merge(other.skipped);
        self.limit_reached.merge(other.limit_reached);
        self.inconclusive.merge(other.inconclusive);
        self.work_units.merge(other.work_units);

        if other.checks.value() > 0 {
            self.last_method = other.last_method;
            self.last_outcome = other.last_outcome;
        }
    }
}

// =============================================================================
// Pass statistics
// =============================================================================

/// Complete statistics for one optimization pass invocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PassStatistics {
    /// Pass identifier.
    pub pass_id: PassId,

    /// Optimization phase.
    pub phase: OptimizationPhase,

    /// Outcome.
    pub outcome: PassOutcome,

    /// Number of iterations performed.
    pub iterations: SaturatingCounter,

    /// Number of rewrite attempts.
    pub rewrite_attempts: SaturatingCounter,

    /// Number of successful rewrites.
    pub rewrites: SaturatingCounter,

    /// Number of fixed-point rounds.
    pub fixed_point_rounds: SaturatingCounter,

    /// Number of analysis requests made by the pass.
    pub analysis_requests: SaturatingCounter,

    /// Number of operations observed at pass entry.
    pub operations_before: SaturatingCounter,

    /// Number of operations observed at pass exit.
    pub operations_after: SaturatingCounter,

    /// Transformation accounting.
    pub transformations: TransformationStatistics,

    /// Resource accounting.
    pub resources: ResourceStatistics,

    /// Verification accounting.
    pub verification: VerificationStatistics,

    /// Elapsed wall-clock time in nanoseconds.
    ///
    /// This is an observation, not a semantic property.
    pub elapsed_nanos: SaturatingCounter,

    /// Rewrite-rule statistics.
    ///
    /// Kept as a vector to preserve deterministic ordering.
    pub rewrite_rules: Vec<RewriteStatistics>,
}

impl PassStatistics {
    /// Creates empty statistics for a pass.
    pub fn new(
        pass_id: PassId,
        phase: OptimizationPhase,
    ) -> Self {
        Self {
            pass_id,
            phase,
            outcome: PassOutcome::Unchanged,
            iterations: SaturatingCounter::default(),
            rewrite_attempts: SaturatingCounter::default(),
            rewrites: SaturatingCounter::default(),
            fixed_point_rounds: SaturatingCounter::default(),
            analysis_requests: SaturatingCounter::default(),
            operations_before: SaturatingCounter::default(),
            operations_after: SaturatingCounter::default(),
            transformations: TransformationStatistics::default(),
            resources: ResourceStatistics::default(),
            verification: VerificationStatistics::default(),
            elapsed_nanos: SaturatingCounter::default(),
            rewrite_rules: Vec::new(),
        }
    }

    /// Records one rewrite attempt.
    pub fn record_rewrite_attempt(&mut self) {
        self.rewrite_attempts.increment();
    }

    /// Records one successful rewrite.
    pub fn record_rewrite(&mut self) {
        self.rewrites.increment();
    }

    /// Records an analysis request.
    pub fn record_analysis_request(&mut self) {
        self.analysis_requests.increment();
    }

    /// Records an iteration.
    pub fn record_iteration(&mut self) {
        self.iterations.increment();
    }

    /// Records a fixed-point round.
    pub fn record_fixed_point_round(&mut self) {
        self.fixed_point_rounds.increment();
    }

    /// Records elapsed time.
    pub fn add_elapsed_nanos(&mut self, nanos: u128) {
        self.elapsed_nanos.add(nanos);
    }

    /// Adds or updates rule statistics while preserving deterministic order.
    pub fn record_rule_application(
        &mut self,
        rule_id: RuleId,
        removed: u128,
        added: u128,
    ) {
        if let Some(rule) = self
            .rewrite_rules
            .iter_mut()
            .find(|rule| rule.rule_id == rule_id)
        {
            rule.record_match();
            rule.record_application(removed, added);
            return;
        }

        let mut rule = RewriteStatistics::new(rule_id);
        rule.record_match();
        rule.record_application(removed, added);
        self.rewrite_rules.push(rule);
    }

    /// Returns true when the pass changed the operation count.
    #[must_use]
    pub fn operation_count_changed(&self) -> bool {
        self.operations_before.value()
            != self.operations_after.value()
    }

    /// Returns the signed operation-count delta when it fits in `i128`.
    ///
    /// A positive value means the pass increased the operation count.
    /// A negative value means the pass reduced it.
    #[must_use]
    pub fn operation_delta(&self) -> Option<i128> {
        let before = self.operations_before.value();
        let after = self.operations_after.value();

        if after >= before {
            i128::try_from(after - before).ok()
        } else {
            i128::try_from(before - after).ok().map(|value| -value)
        }
    }
}

// =============================================================================
// Aggregate statistics
// =============================================================================

/// Complete aggregate statistics for an optimization invocation.
///
/// This is the primary statistics object that `context.rs`, `pipeline.rs`,
/// `result.rs`, benchmarking, provenance, and reporting should consume.
///
/// It contains no reference to the canonical Quantum IR and therefore does
/// not create a dependency cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptimizationStatistics {
    /// Optional stable identifier for this statistics collection.
    pub id: Option<StatisticsId>,

    /// Number of circuits processed.
    pub circuits_processed: SaturatingCounter,

    /// Total optimization passes invoked.
    pub passes_run: SaturatingCounter,

    /// Number of passes that changed the circuit.
    pub passes_changed: SaturatingCounter,

    /// Number of passes that did not change the circuit.
    pub passes_unchanged: SaturatingCounter,

    /// Number of skipped passes.
    pub passes_skipped: SaturatingCounter,

    /// Number of passes stopped by resource limits.
    pub passes_limit_reached: SaturatingCounter,

    /// Number of failed passes.
    pub passes_failed: SaturatingCounter,

    /// Number of passes that partially completed.
    pub passes_partially_completed: SaturatingCounter,

    /// Aggregate operation statistics.
    pub operations: OperationStatistics,

    /// Aggregate transformation statistics.
    pub transformations: TransformationStatistics,

    /// Aggregate resource statistics.
    pub resources: ResourceStatistics,

    /// Aggregate verification statistics.
    pub verification: VerificationStatistics,

    /// Total optimization iterations.
    pub iterations: SaturatingCounter,

    /// Total rewrite attempts.
    pub rewrite_attempts: SaturatingCounter,

    /// Total successful rewrites.
    pub rewrites: SaturatingCounter,

    /// Total analysis requests.
    pub analysis_requests: SaturatingCounter,

    /// Total elapsed optimization time in nanoseconds.
    pub elapsed_nanos: SaturatingCounter,

    /// Number of statistics counters that saturated.
    ///
    /// This is an accounting-quality indicator. Saturation never invalidates
    /// the circuit or optimization result.
    pub saturated_counter_events: SaturatingCounter,

    /// Statistics for individual pass invocations.
    ///
    /// Ordering is the order in which passes were recorded.
    pub passes: Vec<PassStatistics>,
}

impl Default for OptimizationStatistics {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizationStatistics {
    /// Creates an empty aggregate.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            id: None,
            circuits_processed: SaturatingCounter::new(),
            passes_run: SaturatingCounter::new(),
            passes_changed: SaturatingCounter::new(),
            passes_unchanged: SaturatingCounter::new(),
            passes_skipped: SaturatingCounter::new(),
            passes_limit_reached: SaturatingCounter::new(),
            passes_failed: SaturatingCounter::new(),
            passes_partially_completed: SaturatingCounter::new(),
            operations: OperationStatistics {
                operations: SaturatingCounter::new(),
                single_qubit_operations: SaturatingCounter::new(),
                two_qubit_operations: SaturatingCounter::new(),
                multi_qubit_operations: SaturatingCounter::new(),
                unitary_operations: SaturatingCounter::new(),
                non_unitary_operations: SaturatingCounter::new(),
                parameterized_operations: SaturatingCounter::new(),
                measurement_operations: SaturatingCounter::new(),
                reset_operations: SaturatingCounter::new(),
                barrier_operations: SaturatingCounter::new(),
                identity_operations: SaturatingCounter::new(),
            },
            transformations: TransformationStatistics {
                operations_removed: SaturatingCounter::new(),
                operations_added: SaturatingCounter::new(),
                operations_replaced: SaturatingCounter::new(),
                operations_moved: SaturatingCounter::new(),
                operations_fused: SaturatingCounter::new(),
                operations_decomposed: SaturatingCounter::new(),
                identities_removed: SaturatingCounter::new(),
                inverse_pairs_cancelled: SaturatingCounter::new(),
                rotations_combined: SaturatingCounter::new(),
                template_rewrites: SaturatingCounter::new(),
                commutations: SaturatingCounter::new(),
                decompositions: SaturatingCounter::new(),
                synthesis_operations: SaturatingCounter::new(),
            },
            resources: ResourceStatistics {
                logical_qubits: SaturatingCounter::new(),
                ancillas: SaturatingCounter::new(),
                peak_live_qubits: SaturatingCounter::new(),
                depth_before: SaturatingCounter::new(),
                depth_after: SaturatingCounter::new(),
                two_qubit_depth_before: SaturatingCounter::new(),
                two_qubit_depth_after: SaturatingCounter::new(),
                t_count_before: SaturatingCounter::new(),
                t_count_after: SaturatingCounter::new(),
                t_depth_before: SaturatingCounter::new(),
                t_depth_after: SaturatingCounter::new(),
                measurements_before: SaturatingCounter::new(),
                measurements_after: SaturatingCounter::new(),
                resets_before: SaturatingCounter::new(),
                resets_after: SaturatingCounter::new(),
                two_qubit_operations_before: SaturatingCounter::new(),
                two_qubit_operations_after: SaturatingCounter::new(),
            },
            verification: VerificationStatistics {
                checks: SaturatingCounter::new(),
                passed: SaturatingCounter::new(),
                failed: SaturatingCounter::new(),
                skipped: SaturatingCounter::new(),
                limit_reached: SaturatingCounter::new(),
                inconclusive: SaturatingCounter::new(),
                work_units: SaturatingCounter::new(),
                last_method: VerificationMethod::None,
                last_outcome: VerificationOutcome::Skipped,
            },
            iterations: SaturatingCounter::new(),
            rewrite_attempts: SaturatingCounter::new(),
            rewrites: SaturatingCounter::new(),
            analysis_requests: SaturatingCounter::new(),
            elapsed_nanos: SaturatingCounter::new(),
            saturated_counter_events: SaturatingCounter::new(),
            passes: Vec::new(),
        }
    }

    /// Creates aggregate statistics with an explicit identifier.
    pub fn with_id(id: StatisticsId) -> Self {
        let mut statistics = Self::new();
        statistics.id = Some(id);
        statistics
    }

    /// Records that one circuit was processed.
    pub fn record_circuit(&mut self) {
        self.circuits_processed.increment();
    }

    /// Records one completed pass.
    pub fn record_pass(&mut self, pass: PassStatistics) {
        self.passes_run.increment();

        match pass.outcome {
            PassOutcome::Changed => {
                self.passes_changed.increment();
            }
            PassOutcome::Unchanged => {
                self.passes_unchanged.increment();
            }
            PassOutcome::Skipped => {
                self.passes_skipped.increment();
            }
            PassOutcome::LimitReached => {
                self.passes_limit_reached.increment();
            }
            PassOutcome::VerificationFailed
            | PassOutcome::Failed => {
                self.passes_failed.increment();
            }
            PassOutcome::PartiallyCompleted => {
                self.passes_partially_completed.increment();
                self.passes_changed.increment();
            }
        }

        self.iterations.merge(pass.iterations);
        self.rewrite_attempts.merge(pass.rewrite_attempts);
        self.rewrites.merge(pass.rewrites);
        self.analysis_requests
            .merge(pass.analysis_requests);
        self.elapsed_nanos.merge(pass.elapsed_nanos);

        self.transformations.merge(&pass.transformations);
        self.resources.merge(&pass.resources);
        self.verification.merge(&pass.verification);

        self.passes.push(pass);
    }

    /// Records total elapsed optimization time.
    pub fn add_elapsed_nanos(&mut self, nanos: u128) {
        self.elapsed_nanos.add(nanos);
    }

    /// Merges another complete statistics collection.
    ///
    /// This is the fundamental operation needed for:
    ///
    /// - parallel compilation;
    /// - incremental compilation;
    /// - nested pipelines;
    /// - distributed compilation;
    /// - optimization workers;
    /// - batch processing.
    pub fn merge(&mut self, other: &Self) {
        self.circuits_processed
            .merge(other.circuits_processed);
        self.passes_run.merge(other.passes_run);
        self.passes_changed.merge(other.passes_changed);
        self.passes_unchanged
            .merge(other.passes_unchanged);
        self.passes_skipped.merge(other.passes_skipped);
        self.passes_limit_reached
            .merge(other.passes_limit_reached);
        self.passes_failed.merge(other.passes_failed);
        self.passes_partially_completed
            .merge(other.passes_partially_completed);

        self.operations.merge(&other.operations);
        self.transformations.merge(&other.transformations);
        self.resources.merge(&other.resources);
        self.verification.merge(&other.verification);

        self.iterations.merge(other.iterations);
        self.rewrite_attempts
            .merge(other.rewrite_attempts);
        self.rewrites.merge(other.rewrites);
        self.analysis_requests
            .merge(other.analysis_requests);
        self.elapsed_nanos.merge(other.elapsed_nanos);

        self.passes
            .extend(other.passes.iter().cloned());

        self.recalculate_saturation_indicator();
    }

    /// Returns the total elapsed time in nanoseconds.
    #[must_use]
    pub const fn elapsed_nanos(&self) -> u128 {
        self.elapsed_nanos.value()
    }

    /// Returns elapsed time in seconds as an f64.
    ///
    /// This is intended only for presentation/reporting. It must not be used
    /// for semantic decisions.
    #[must_use]
    pub fn elapsed_seconds(&self) -> f64 {
        self.elapsed_nanos.value() as f64 / 1_000_000_000.0
    }

    /// Returns the number of operations removed.
    #[must_use]
    pub const fn operations_removed(&self) -> u128 {
        self.transformations.operations_removed.value()
    }

    /// Returns the number of operations added.
    #[must_use]
    pub const fn operations_added(&self) -> u128 {
        self.transformations.operations_added.value()
    }

    /// Returns the number of operations replaced.
    #[must_use]
    pub const fn operations_replaced(&self) -> u128 {
        self.transformations.operations_replaced.value()
    }

    /// Returns the number of successful rewrites.
    #[must_use]
    pub const fn rewrites(&self) -> u128 {
        self.rewrites.value()
    }

    /// Returns the number of passes executed.
    #[must_use]
    pub const fn passes_run(&self) -> u128 {
        self.passes_run.value()
    }

    /// Returns true if at least one pass changed the circuit.
    #[must_use]
    pub const fn changed(&self) -> bool {
        self.passes_changed.value() > 0
    }

    /// Returns true if any pass reached a configured limit.
    #[must_use]
    pub const fn limit_reached(&self) -> bool {
        self.passes_limit_reached.value() > 0
    }

    /// Returns true if any pass failed.
    #[must_use]
    pub const fn failed(&self) -> bool {
        self.passes_failed.value() > 0
    }

    /// Returns true if semantic verification failed.
    #[must_use]
    pub const fn verification_failed(&self) -> bool {
        self.verification.failed.value() > 0
    }

    /// Returns the number of circuits processed.
    #[must_use]
    pub const fn circuits_processed(&self) -> u128 {
        self.circuits_processed.value()
    }

    /// Returns the number of saturated counters detected.
    #[must_use]
    pub const fn saturated_counter_events(&self) -> u128 {
        self.saturated_counter_events.value()
    }

    /// Returns true when any accounting counter has saturated.
    #[must_use]
    pub const fn accounting_saturated(&self) -> bool {
        self.saturated_counter_events.value() > 0
    }

    /// Recomputes the saturation indicator.
    ///
    /// This is intentionally conservative: if an aggregate counter is
    /// saturated, the indicator is incremented. The indicator itself is
    /// saturating.
    pub fn recalculate_saturation_indicator(&mut self) {
        let mut saturated = 0u128;

        let counters = [
            self.circuits_processed,
            self.passes_run,
            self.passes_changed,
            self.passes_unchanged,
            self.passes_skipped,
            self.passes_limit_reached,
            self.passes_failed,
            self.passes_partially_completed,
            self.iterations,
            self.rewrite_attempts,
            self.rewrites,
            self.analysis_requests,
            self.elapsed_nanos,
        ];

        for counter in counters {
            if counter.is_saturated() {
                saturated = saturated.saturating_add(1);
            }
        }

        if saturated > 0 {
            self.saturated_counter_events
                .add(saturated);
        }
    }
}

// =============================================================================
// Snapshot
// =============================================================================

/// Compact immutable snapshot of aggregate optimization statistics.
///
/// This is useful when callers need to capture statistics at a point in time
/// without exposing the mutable aggregation object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatisticsSnapshot {
    /// Circuits processed.
    pub circuits_processed: u128,

    /// Passes run.
    pub passes_run: u128,

    /// Passes that changed the circuit.
    pub passes_changed: u128,

    /// Operations removed.
    pub operations_removed: u128,

    /// Operations added.
    pub operations_added: u128,

    /// Operations replaced.
    pub operations_replaced: u128,

    /// Rewrites.
    pub rewrites: u128,

    /// Logical depth before optimization.
    pub depth_before: u128,

    /// Logical depth after optimization.
    pub depth_after: u128,

    /// Two-qubit gates before optimization.
    pub two_qubit_operations_before: u128,

    /// Two-qubit gates after optimization.
    pub two_qubit_operations_after: u128,

    /// T-count before optimization.
    pub t_count_before: u128,

    /// T-count after optimization.
    pub t_count_after: u128,

    /// T-depth before optimization.
    pub t_depth_before: u128,

    /// T-depth after optimization.
    pub t_depth_after: u128,

    /// Verification checks.
    pub verification_checks: u128,

    /// Verification failures.
    pub verification_failures: u128,

    /// Elapsed nanoseconds.
    pub elapsed_nanos: u128,

    /// Whether accounting saturated.
    pub accounting_saturated: bool,
}

impl OptimizationStatistics {
    /// Produces a compact immutable snapshot.
    #[must_use]
    pub fn snapshot(&self) -> StatisticsSnapshot {
        StatisticsSnapshot {
            circuits_processed: self.circuits_processed(),
            passes_run: self.passes_run(),
            passes_changed: self.passes_changed.value(),
            operations_removed: self.operations_removed(),
            operations_added: self.operations_added(),
            operations_replaced: self.operations_replaced(),
            rewrites: self.rewrites(),
            depth_before: self.resources.depth_before.value(),
            depth_after: self.resources.depth_after.value(),
            two_qubit_operations_before: self
                .resources
                .two_qubit_operations_before
                .value(),
            two_qubit_operations_after: self
                .resources
                .two_qubit_operations_after
                .value(),
            t_count_before: self.resources.t_count_before.value(),
            t_count_after: self.resources.t_count_after.value(),
            t_depth_before: self.resources.t_depth_before.value(),
            t_depth_after: self.resources.t_depth_after.value(),
            verification_checks: self.verification.checks.value(),
            verification_failures: self.verification.failed.value(),
            elapsed_nanos: self.elapsed_nanos(),
            accounting_saturated: self.accounting_saturated(),
        }
    }
}

// =============================================================================
// Optimization delta
// =============================================================================

/// Signed presentation of an optimization change.
///
/// The underlying counters remain unsigned and lossless; this type is only
/// used when reporting a before/after difference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptimizationDelta {
    /// Before value.
    pub before: u128,

    /// After value.
    pub after: u128,

    /// Absolute amount removed when `after < before`.
    pub reduced_by: u128,

    /// Absolute amount added when `after > before`.
    pub increased_by: u128,
}

impl OptimizationDelta {
    /// Creates a delta.
    #[must_use]
    pub const fn new(before: u128, after: u128) -> Self {
        if after >= before {
            Self {
                before,
                after,
                reduced_by: 0,
                increased_by: after - before,
            }
        } else {
            Self {
                before,
                after,
                reduced_by: before - after,
                increased_by: 0,
            }
        }
    }

    /// Returns true if the value decreased.
    #[must_use]
    pub const fn decreased(&self) -> bool {
        self.reduced_by > 0
    }

    /// Returns true if the value increased.
    #[must_use]
    pub const fn increased(&self) -> bool {
        self.increased_by > 0
    }

    /// Returns true if the value remained unchanged.
    #[must_use]
    pub const fn unchanged(&self) -> bool {
        self.before == self.after
    }
}

impl StatisticsSnapshot {
    /// Returns the operation-count delta.
    #[must_use]
    pub const fn operation_delta(
        &self,
        operations_before: u128,
        operations_after: u128,
    ) -> OptimizationDelta {
        OptimizationDelta::new(
            operations_before,
            operations_after,
        )
    }

    /// Returns the depth delta.
    #[must_use]
    pub const fn depth_delta(&self) -> OptimizationDelta {
        OptimizationDelta::new(
            self.depth_before,
            self.depth_after,
        )
    }

    /// Returns the two-qubit-operation delta.
    #[must_use]
    pub const fn two_qubit_delta(&self) -> OptimizationDelta {
        OptimizationDelta::new(
            self.two_qubit_operations_before,
            self.two_qubit_operations_after,
        )
    }

    /// Returns the T-count delta.
    #[must_use]
    pub const fn t_count_delta(&self) -> OptimizationDelta {
        OptimizationDelta::new(
            self.t_count_before,
            self.t_count_after,
        )
    }

    /// Returns the T-depth delta.
    #[must_use]
    pub const fn t_depth_delta(&self) -> OptimizationDelta {
        OptimizationDelta::new(
            self.t_depth_before,
            self.t_depth_after,
        )
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for pass statistics.
///
/// This allows future optimization passes to construct statistics without
/// depending on the aggregate object directly.
#[derive(Debug)]
pub struct PassStatisticsBuilder {
    statistics: PassStatistics,
}

impl PassStatisticsBuilder {
    /// Creates a builder.
    pub fn new(
        pass_id: PassId,
        phase: OptimizationPhase,
    ) -> Self {
        Self {
            statistics: PassStatistics::new(pass_id, phase),
        }
    }

    /// Sets the pass outcome.
    #[must_use]
    pub fn outcome(mut self, outcome: PassOutcome) -> Self {
        self.statistics.outcome = outcome;
        self
    }

    /// Sets the operation count before optimization.
    #[must_use]
    pub fn operations_before(mut self, count: u128) -> Self {
        self.statistics.operations_before =
            SaturatingCounter::from_value(count);
        self
    }

    /// Sets the operation count after optimization.
    #[must_use]
    pub fn operations_after(mut self, count: u128) -> Self {
        self.statistics.operations_after =
            SaturatingCounter::from_value(count);
        self
    }

    /// Records an iteration.
    #[must_use]
    pub fn iteration(mut self) -> Self {
        self.statistics.record_iteration();
        self
    }

    /// Records a rewrite attempt.
    #[must_use]
    pub fn rewrite_attempt(mut self) -> Self {
        self.statistics.record_rewrite_attempt();
        self
    }

    /// Records a successful rewrite.
    #[must_use]
    pub fn rewrite(mut self) -> Self {
        self.statistics.record_rewrite();
        self
    }

    /// Records elapsed nanoseconds.
    #[must_use]
    pub fn elapsed_nanos(mut self, nanos: u128) -> Self {
        self.statistics.add_elapsed_nanos(nanos);
        self
    }

    /// Finishes construction.
    #[must_use]
    pub fn finish(self) -> PassStatistics {
        self.statistics
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_addition_is_saturating() {
        let mut counter =
            SaturatingCounter::from_value(u128::MAX);

        counter.add(1);

        assert_eq!(counter.value(), u128::MAX);
        assert!(counter.is_saturated());
    }

    #[test]
    fn counter_merge_is_saturating() {
        let mut first =
            SaturatingCounter::from_value(u128::MAX - 1);

        let second =
            SaturatingCounter::from_value(10);

        first.merge(second);

        assert_eq!(first.value(), u128::MAX);
        assert!(first.is_saturated());
    }

    #[test]
    fn strict_counter_addition_reports_overflow() {
        let counter =
            SaturatingCounter::from_value(u128::MAX);

        let result = counter.checked_add(1);

        assert!(matches!(
            result,
            Err(StatisticsError::ArithmeticOverflow { .. })
        ));
    }

    #[test]
    fn pass_identifier_rejects_empty_values() {
        let result = PassId::new("   ");

        assert!(matches!(
            result,
            Err(StatisticsError::EmptyIdentifier {
                kind: IdentifierKind::Pass
            })
        ));
    }

    #[test]
    fn operation_statistics_record_arity() {
        let mut statistics =
            OperationStatistics::default();

        statistics.record_operation(1, true, false);
        statistics.record_operation(2, true, true);
        statistics.record_operation(3, true, false);

        assert_eq!(
            statistics.operations.value(),
            3
        );

        assert_eq!(
            statistics.single_qubit_operations.value(),
            1
        );

        assert_eq!(
            statistics.two_qubit_operations.value(),
            1
        );

        assert_eq!(
            statistics.multi_qubit_operations.value(),
            1
        );

        assert_eq!(
            statistics.parameterized_operations.value(),
            1
        );
    }

    #[test]
    fn transformation_statistics_track_inverse_cancellation() {
        let mut statistics =
            TransformationStatistics::default();

        statistics.record_inverse_cancellation(3);

        assert_eq!(
            statistics.inverse_pairs_cancelled.value(),
            3
        );

        assert_eq!(
            statistics.operations_removed.value(),
            6
        );
    }

    #[test]
    fn verification_statistics_track_failures() {
        let mut statistics =
            VerificationStatistics::default();

        statistics.record(
            VerificationMethod::ExactUnitary,
            VerificationOutcome::Passed,
            100,
        );

        statistics.record(
            VerificationMethod::ExactUnitary,
            VerificationOutcome::Failed,
            200,
        );

        assert_eq!(
            statistics.checks.value(),
            2
        );

        assert_eq!(
            statistics.passed.value(),
            1
        );

        assert_eq!(
            statistics.failed.value(),
            1
        );

        assert_eq!(
            statistics.work_units.value(),
            300
        );

        assert_eq!(
            statistics.last_outcome,
            VerificationOutcome::Failed
        );
    }

    #[test]
    fn pass_statistics_record_rule_application() {
        let pass_id =
            PassId::from_static("local.cancellation");

        let rule_id =
            RuleId::from_static("inverse_pair");

        let mut statistics =
            PassStatistics::new(
                pass_id,
                OptimizationPhase::Local,
            );

        statistics.record_rewrite_attempt();
        statistics.record_rule_application(
            rule_id,
            2,
            0,
        );

        assert_eq!(
            statistics.rewrite_attempts.value(),
            1
        );

        assert_eq!(
            statistics.rewrite_rules.len(),
            1
        );

        assert_eq!(
            statistics
                .rewrite_rules[0]
                .applications
                .value(),
            1
        );

        assert_eq!(
            statistics
                .rewrite_rules[0]
                .operations_removed
                .value(),
            2
        );
    }

    #[test]
    fn aggregate_records_changed_pass() {
        let mut statistics =
            OptimizationStatistics::new();

        statistics.record_circuit();

        let pass = PassStatisticsBuilder::new(
            PassId::from_static(
                "local.cancellation",
            ),
            OptimizationPhase::Local,
        )
        .outcome(PassOutcome::Changed)
        .operations_before(100)
        .operations_after(80)
        .iteration()
        .rewrite()
        .finish();

        statistics.record_pass(pass);

        assert_eq!(
            statistics.circuits_processed(),
            1
        );

        assert_eq!(
            statistics.passes_run(),
            1
        );

        assert!(statistics.changed());

        assert_eq!(
            statistics.passes_changed.value(),
            1
        );

        assert_eq!(
            statistics.rewrites(),
            1
        );
    }

    #[test]
    fn aggregate_merge_is_associative_for_basic_counters() {
        let mut a =
            OptimizationStatistics::new();

        let mut b =
            OptimizationStatistics::new();

        let mut c =
            OptimizationStatistics::new();

        a.record_circuit();
        b.record_circuit();
        c.record_circuit();

        a.rewrites.add(10);
        b.rewrites.add(20);
        c.rewrites.add(30);

        let mut left = a.clone();
        left.merge(&b);
        left.merge(&c);

        let mut right = a;
        let mut bc = b;
        bc.merge(&c);
        right.merge(&bc);

        assert_eq!(
            left.rewrites.value(),
            right.rewrites.value()
        );

        assert_eq!(
            left.circuits_processed(),
            right.circuits_processed()
        );
    }

    #[test]
    fn snapshot_is_stable() {
        let mut statistics =
            OptimizationStatistics::new();

        statistics.resources.depth_before =
            SaturatingCounter::from_value(100);

        statistics.resources.depth_after =
            SaturatingCounter::from_value(60);

        statistics.resources.t_count_before =
            SaturatingCounter::from_value(40);

        statistics.resources.t_count_after =
            SaturatingCounter::from_value(12);

        let snapshot = statistics.snapshot();

        assert_eq!(
            snapshot.depth_before,
            100
        );

        assert_eq!(
            snapshot.depth_after,
            60
        );

        assert_eq!(
            snapshot.t_count_before,
            40
        );

        assert_eq!(
            snapshot.t_count_after,
            12
        );

        assert_eq!(
            snapshot.depth_delta().reduced_by,
            40
        );

        assert_eq!(
            snapshot.t_count_delta().reduced_by,
            28
        );
    }

    #[test]
    fn delta_handles_increase() {
        let delta =
            OptimizationDelta::new(10, 15);

        assert!(!delta.decreased());
        assert!(delta.increased());
        assert!(!delta.unchanged());
        assert_eq!(delta.increased_by, 5);
    }

    #[test]
    fn delta_handles_decrease() {
        let delta =
            OptimizationDelta::new(15, 10);

        assert!(delta.decreased());
        assert!(!delta.increased());
        assert!(!delta.unchanged());
        assert_eq!(delta.reduced_by, 5);
    }

    #[test]
    fn delta_handles_equality() {
        let delta =
            OptimizationDelta::new(10, 10);

        assert!(!delta.decreased());
        assert!(!delta.increased());
        assert!(delta.unchanged());
    }

    #[test]
    fn statistics_are_serializable() {
        let statistics =
            OptimizationStatistics::new();

        let encoded =
            serde_json::to_string(&statistics)
                .expect("statistics should serialize");

        let decoded: OptimizationStatistics =
            serde_json::from_str(&encoded)
                .expect("statistics should deserialize");

        assert_eq!(statistics, decoded);
    }

    #[test]
    fn no_statistics_operation_requires_global_state() {
        let mut first =
            OptimizationStatistics::new();

        let mut second =
            OptimizationStatistics::new();

        first.record_circuit();
        second.record_circuit();

        first.merge(&second);

        assert_eq!(
            first.circuits_processed(),
            2
        );
    }
}