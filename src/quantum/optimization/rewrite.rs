//! Zamani Quantum Optimization — Generic Rewrite Engine
//!
//! Production-grade, backend-independent rewrite infrastructure for
//! `quantum::optimization`.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir::Gate
//!                                │
//!                                ▼
//!                         CircuitView
//!                                │
//!                                ▼
//!                         RewriteEngine
//!                                │
//!             ┌──────────────────┼──────────────────┐
//!             │                  │                  │
//!             ▼                  ▼                  ▼
//!          Pattern            Matcher             Rules
//!             │                  │                  │
//!             └──────────────────┼──────────────────┘
//!                                ▼
//!                         RewriteCandidate
//!                                │
//!                         preconditions
//!                                │
//!                                ▼
//!                        RewriteTransaction
//!                                │
//!                         postconditions
//!                                │
//!                                ▼
//!                       canonical Quantum IR
//! ```
//!
//! # Ownership
//!
//! This module does NOT define another quantum IR.
//!
//! The authoritative operation representation is:
//!
//! `crate::quantum::ir::Gate`
//!
//! The authoritative circuit representation is:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! This file owns only rewrite infrastructure:
//!
//! - rewrite rules;
//! - rewrite matches;
//! - rewrite replacements;
//! - rewrite candidates;
//! - rewrite budgets;
//! - rewrite selection;
//! - overlap/conflict detection;
//! - transactional application to operation sequences;
//! - preconditions;
//! - postconditions;
//! - rewrite statistics;
//! - rewrite provenance;
//! - deterministic candidate ordering.
//!
//! # Integration contract
//!
//! ## `errors.rs`
//!
//! `RuleIdentifier` is the canonical rule identifier. This module does not
//! introduce a second public rule-ID type.
//!
//! ## `circuit.rs`
//!
//! `OperationId` is used to identify invocation-local operations. A rewrite
//! match may therefore be converted into a `CircuitEditPlan` by the circuit
//! editing layer without introducing another operation identity system.
//!
//! ## `pattern.rs`
//!
//! `pattern.rs` may implement `RewriteMatcher` and construct
//! `RewriteMatch` values. No change to this file is required.
//!
//! ## `matcher.rs`
//!
//! `matcher.rs` may implement optimized search strategies using
//! `RewriteMatcher`. The engine itself remains independent of the matching
//! algorithm.
//!
//! ## `rules.rs`
//!
//! `rules.rs` should construct concrete `RewriteRule` implementations and
//! register them with the pass/registry infrastructure.
//!
//! ## `pass.rs`
//!
//! Rewrite-based passes can own a `RewriteEngine` and invoke it against a
//! canonical circuit snapshot.
//!
//! ## `context.rs`
//!
//! The engine deliberately does not require a concrete `OptimizationContext`
//! API. This prevents a circular/future API dependency. The `RewriteObserver`
//! trait is the integration boundary through which `OptimizationContext` can
//! receive rewrite accounting without requiring this file to be changed.
//!
//! ## `cost.rs`
//!
//! `RewriteCost` is intentionally a local delta representation. `cost.rs` can
//! convert it into the global `CostModel` representation without making the
//! rewrite engine depend on a particular multi-objective implementation.
//!
//! ## `provenance.rs`
//!
//! `RewriteRecord` is intentionally immutable and serializable in spirit.
//! Provenance infrastructure can consume these records directly.
//!
//! ## `statistics.rs`
//!
//! `RewriteStatistics` provides rewrite-level counters. The global statistics
//! subsystem can aggregate them without requiring rewrite implementations to
//! know about the global report structure.
//!
//! ## `pipeline.rs`
//!
//! The pipeline controls pass sequencing. The rewrite engine only controls
//! rewrite selection and bounded fixed-point execution inside one invocation.
//!
//! ## `verification/*`
//!
//! The engine supports structural postconditions but deliberately does not
//! perform semantic equivalence checking itself. Semantic verification remains
//! owned by `verification/*`.
//!
//! # Scaling
//!
//! No artificial circuit-size ceiling is imposed by this module.
//!
//! Scaling is controlled through explicit budgets:
//!
//! - maximum candidates;
//! - maximum rewrites;
//! - maximum iterations;
//! - maximum inserted operations;
//! - maximum replacement operations;
//! - maximum transaction size;
//! - optional maximum rule applications.
//!
//! A caller can set these limits according to available resources.
//!
//! `usize` is used for in-memory collection sizes because Rust collections use
//! `usize`; cumulative accounting uses `u64` where practical.
//!
//! Arithmetic is checked. Overflow is never silently wrapped.
//!
//! # Determinism
//!
//! Candidate selection is deterministic when the matcher is deterministic.
//!
//! Candidates are ordered by:
//!
//! 1. start operation index;
//! 2. end operation index;
//! 3. rule priority;
//! 4. rule identifier.
//!
//! This provides stable behavior across runs without requiring a global RNG.
//!
//! # Semantic safety
//!
//! A rewrite is accepted only when:
//!
//! 1. its match is structurally valid;
//! 2. its rule precondition succeeds;
//! 3. its replacement is structurally valid;
//! 4. the candidate does not conflict with another accepted candidate;
//! 5. the rule postcondition succeeds;
//! 6. configured resource budgets remain satisfied.
//!
//! Semantic equivalence checking is intentionally a separate concern.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Important design decision
//!
//! The engine operates on a slice of canonical `Gate` values and returns a
//! replacement sequence. This makes the engine usable by:
//!
//! - `CircuitEditPlan`;
//! - direct optimizer snapshots;
//! - region optimizers;
//! - block optimizers;
//! - future e-graph frontends;
//! - pattern-based passes;
//! - target-specific rewrite passes.
//!
//! The engine does not mutate `QuantumCircuit` directly. Ownership and
//! transactional circuit mutation remain with `circuit.rs`.
//!
//! This prevents a rewrite engine from bypassing canonical circuit invariants.

#![forbid(unsafe_code)]

use std::cmp::Ordering;
use std::fmt;

use crate::quantum::ir::gate::Gate;

use super::circuit::OperationId;
use super::errors::RuleIdentifier;

// =============================================================================
// Public result
// =============================================================================

/// Result type used by the rewrite subsystem.
pub type RewriteResult<T> = Result<T, RewriteError>;

// =============================================================================
// Rewrite error
// =============================================================================

/// Errors produced by the generic rewrite engine.
///
/// This is intentionally rewrite-specific while remaining convertible into
/// the optimization subsystem's canonical error layer by higher-level
/// infrastructure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RewriteError {
    /// A rewrite rule has an invalid identifier.
    InvalidRuleIdentifier {
        /// Human-readable reason.
        message: String,
    },

    /// A rewrite match contains an invalid range.
    InvalidMatch {
        /// Rule associated with the invalid match.
        rule: String,

        /// Start index.
        start: usize,

        /// End index.
        end: usize,

        /// Circuit length.
        circuit_len: usize,
    },

    /// A match's operation identifiers do not correspond to its range.
    InvalidOperationIds {
        /// Rule associated with the invalid match.
        rule: String,

        /// Expected number of operation IDs.
        expected: usize,

        /// Actual number of operation IDs.
        actual: usize,
    },

    /// A replacement exceeds a configured budget.
    ReplacementLimitExceeded {
        /// Rule responsible for the replacement.
        rule: String,

        /// Requested number of operations.
        requested: usize,

        /// Maximum permitted number.
        maximum: usize,
    },

    /// The entire transaction would exceed a configured budget.
    TransactionLimitExceeded {
        /// Requested number of operations.
        requested: usize,

        /// Maximum permitted number.
        maximum: usize,
    },

    /// Candidate count exceeded the configured budget.
    CandidateLimitExceeded {
        /// Number of candidates encountered.
        requested: u64,

        /// Maximum candidates.
        maximum: u64,
    },

    /// Rewrite count exceeded the configured budget.
    RewriteLimitExceeded {
        /// Number of rewrites attempted.
        requested: u64,

        /// Maximum rewrites.
        maximum: u64,
    },

    /// Fixed-point iteration limit was reached.
    IterationLimitExceeded {
        /// Number of completed iterations.
        iterations: u64,

        /// Maximum permitted iterations.
        maximum: u64,
    },

    /// The rule's precondition rejected a candidate.
    PreconditionFailed {
        /// Rule identifier.
        rule: String,

        /// Candidate start position.
        start: usize,

        /// Reason supplied by the rule.
        reason: String,
    },

    /// The rule's postcondition rejected a candidate.
    PostconditionFailed {
        /// Rule identifier.
        rule: String,

        /// Candidate start position.
        start: usize,

        /// Reason supplied by the rule.
        reason: String,
    },

    /// Two candidates overlap and cannot both be applied.
    OverlappingCandidates {
        /// First rule.
        first_rule: String,

        /// Second rule.
        second_rule: String,

        /// First range.
        first_start: usize,

        /// First end.
        first_end: usize,

        /// Second range.
        second_start: usize,

        /// Second end.
        second_end: usize,
    },

    /// The replacement would create an invalid operation sequence.
    InvalidReplacement {
        /// Rule identifier.
        rule: String,

        /// Human-readable reason.
        message: String,
    },

    /// Checked arithmetic overflow occurred.
    ArithmeticOverflow {
        /// Description of the calculation.
        calculation: &'static str,
    },

    /// A rule failed during execution.
    RuleExecution {
        /// Rule identifier.
        rule: String,

        /// Human-readable reason.
        message: String,
    },

    /// The caller cancelled rewrite execution.
    Cancelled,

    /// The rewrite observer rejected the operation.
    ObserverRejected {
        /// Human-readable reason.
        message: String,
    },
}

impl fmt::Display for RewriteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRuleIdentifier { message } => {
                write!(formatter, "invalid rewrite rule identifier: {message}")
            }

            Self::InvalidMatch {
                rule,
                start,
                end,
                circuit_len,
            } => {
                write!(
                    formatter,
                    "invalid rewrite match for rule `{rule}`: range {start}..{end} \
                     is outside circuit length {circuit_len}"
                )
            }

            Self::InvalidOperationIds {
                rule,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "invalid operation IDs for rewrite rule `{rule}`: \
                     expected {expected}, received {actual}"
                )
            }

            Self::ReplacementLimitExceeded {
                rule,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "rewrite rule `{rule}` requested {requested} replacement \
                     operations, exceeding maximum {maximum}"
                )
            }

            Self::TransactionLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "rewrite transaction requires {requested} operations, \
                     exceeding maximum {maximum}"
                )
            }

            Self::CandidateLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "rewrite candidate limit exceeded: {requested} > {maximum}"
                )
            }

            Self::RewriteLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "rewrite limit exceeded: {requested} > {maximum}"
                )
            }

            Self::IterationLimitExceeded {
                iterations,
                maximum,
            } => {
                write!(
                    formatter,
                    "rewrite iteration limit exceeded: {iterations} >= {maximum}"
                )
            }

            Self::PreconditionFailed {
                rule,
                start,
                reason,
            } => {
                write!(
                    formatter,
                    "rewrite precondition failed for `{rule}` at {start}: {reason}"
                )
            }

            Self::PostconditionFailed {
                rule,
                start,
                reason,
            } => {
                write!(
                    formatter,
                    "rewrite postcondition failed for `{rule}` at {start}: {reason}"
                )
            }

            Self::OverlappingCandidates {
                first_rule,
                second_rule,
                first_start,
                first_end,
                second_start,
                second_end,
            } => {
                write!(
                    formatter,
                    "overlapping rewrite candidates: `{first_rule}` \
                     {first_start}..{first_end} conflicts with `{second_rule}` \
                     {second_start}..{second_end}"
                )
            }

            Self::InvalidReplacement { rule, message } => {
                write!(
                    formatter,
                    "invalid replacement for rewrite rule `{rule}`: {message}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::RuleExecution { rule, message } => {
                write!(
                    formatter,
                    "rewrite rule `{rule}` failed: {message}"
                )
            }

            Self::Cancelled => {
                formatter.write_str("rewrite execution was cancelled")
            }

            Self::ObserverRejected { message } => {
                write!(
                    formatter,
                    "rewrite observer rejected operation: {message}"
                )
            }
        }
    }
}

impl std::error::Error for RewriteError {}

// =============================================================================
// Rewrite cost
// =============================================================================

/// Resource delta associated with a rewrite.
///
/// This deliberately does not depend on the global `CostModel`. The global
/// cost subsystem can aggregate or reinterpret these values according to the
/// selected optimization objective.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RewriteCost {
    /// Change in total operation count.
    pub operations: i64,

    /// Change in one-qubit operation count.
    pub single_qubit_operations: i64,

    /// Change in two-qubit operation count.
    pub two_qubit_operations: i64,

    /// Change in multi-qubit operation count.
    pub multi_qubit_operations: i64,

    /// Change in circuit depth contribution.
    ///
    /// This is a declared delta, not a replacement for complete depth
    /// analysis.
    pub depth: i64,

    /// Change in T gate count.
    pub t_count: i64,

    /// Change in T-depth contribution.
    pub t_depth: i64,

    /// Change in measurement count.
    pub measurements: i64,

    /// Change in reset count.
    pub resets: i64,
}

impl RewriteCost {
    /// Creates a zero cost delta.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            operations: 0,
            single_qubit_operations: 0,
            two_qubit_operations: 0,
            multi_qubit_operations: 0,
            depth: 0,
            t_count: 0,
            t_depth: 0,
            measurements: 0,
            resets: 0,
        }
    }

    /// Calculates the operation-count delta between two sequences.
    #[must_use]
    pub fn from_sequences(
        before: &[Gate],
        after: &[Gate],
    ) -> Self {
        let mut before_counts = GateCounts::default();
        let mut after_counts = GateCounts::default();

        for gate in before {
            before_counts.observe(gate);
        }

        for gate in after {
            after_counts.observe(gate);
        }

        Self {
            operations: difference(
                after_counts.operations,
                before_counts.operations,
            ),
            single_qubit_operations: difference(
                after_counts.single_qubit_operations,
                before_counts.single_qubit_operations,
            ),
            two_qubit_operations: difference(
                after_counts.two_qubit_operations,
                before_counts.two_qubit_operations,
            ),
            multi_qubit_operations: difference(
                after_counts.multi_qubit_operations,
                before_counts.multi_qubit_operations,
            ),
            depth: 0,
            t_count: difference(
                after_counts.t_count,
                before_counts.t_count,
            ),
            t_depth: 0,
            measurements: difference(
                after_counts.measurements,
                before_counts.measurements,
            ),
            resets: difference(
                after_counts.resets,
                before_counts.resets,
            ),
        }
    }

    /// Returns whether the rewrite reduces the number of operations.
    #[must_use]
    pub const fn reduces_operations(self) -> bool {
        self.operations < 0
    }

    /// Returns whether the rewrite increases the number of operations.
    #[must_use]
    pub const fn increases_operations(self) -> bool {
        self.operations > 0
    }
}

/// Counts operation classes used by `RewriteCost`.
#[derive(Debug, Clone, Copy, Default)]
struct GateCounts {
    operations: i64,
    single_qubit_operations: i64,
    two_qubit_operations: i64,
    multi_qubit_operations: i64,
    t_count: i64,
    measurements: i64,
    resets: i64,
}

impl GateCounts {
    fn observe(&mut self, gate: &Gate) {
        self.operations = self.operations.saturating_add(1);

        let arity = gate.qubits().len();

        match arity {
            0 | 1 => {
                self.single_qubit_operations =
                    self.single_qubit_operations.saturating_add(1);
            }

            2 => {
                self.two_qubit_operations =
                    self.two_qubit_operations.saturating_add(1);
            }

            _ => {
                self.multi_qubit_operations =
                    self.multi_qubit_operations.saturating_add(1);
            }
        }

        if gate.is_measurement() {
            self.measurements = self.measurements.saturating_add(1);
        }

        if gate.is_reset() {
            self.resets = self.resets.saturating_add(1);
        }

        match gate.kind() {
            crate::quantum::ir::gate::GateKind::T
            | crate::quantum::ir::gate::GateKind::Tdg => {
                self.t_count = self.t_count.saturating_add(1);
            }

            _ => {}
        }
    }
}

fn difference(after: i64, before: i64) -> i64 {
    after.saturating_sub(before)
}

// =============================================================================
// Rewrite priority
// =============================================================================

/// Priority used when multiple rewrite rules can match the same location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RewritePriority(u32);

impl RewritePriority {
    /// Creates a priority.
    ///
    /// Higher values win when candidates otherwise have the same location.
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the numeric priority.
    #[must_use]
    pub const fn value(self) -> u32 {
        self.0
    }
}

impl Default for RewritePriority {
    fn default() -> Self {
        Self(0)
    }
}

// =============================================================================
// Rewrite match
// =============================================================================

/// A concrete match of a rewrite rule in a circuit snapshot.
///
/// The range is half-open: `[start, end)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewriteMatch {
    /// Invocation-local rule identifier.
    rule: RuleIdentifier,

    /// First operation index.
    start: usize,

    /// Exclusive end operation index.
    end: usize,

    /// Invocation-local operation IDs covered by the match.
    operations: Vec<OperationId>,
}

impl RewriteMatch {
    /// Creates a rewrite match.
    pub fn new(
        rule: RuleIdentifier,
        start: usize,
        end: usize,
        operations: Vec<OperationId>,
    ) -> RewriteResult<Self> {
        if start > end {
            return Err(RewriteError::InvalidMatch {
                rule: rule.to_string(),
                start,
                end,
                circuit_len: 0,
            });
        }

        let expected = end
            .checked_sub(start)
            .ok_or(RewriteError::ArithmeticOverflow {
                calculation: "rewrite match length",
            })?;

        if expected != operations.len() {
            return Err(RewriteError::InvalidOperationIds {
                rule: rule.to_string(),
                expected,
                actual: operations.len(),
            });
        }

        for (offset, operation) in operations.iter().enumerate() {
            let expected_id = OperationId::new(
                start
                    .checked_add(offset)
                    .ok_or(RewriteError::ArithmeticOverflow {
                        calculation: "rewrite operation identifier",
                    })?,
            );

            if *operation != expected_id {
                return Err(RewriteError::InvalidOperationIds {
                    rule: rule.to_string(),
                    expected,
                    actual: operations.len(),
                });
            }
        }

        Ok(Self {
            rule,
            start,
            end,
            operations,
        })
    }

    /// Returns the rule identifier.
    #[must_use]
    pub fn rule(&self) -> &RuleIdentifier {
        &self.rule
    }

    /// Returns the start index.
    #[must_use]
    pub const fn start(&self) -> usize {
        self.start
    }

    /// Returns the exclusive end index.
    #[must_use]
    pub const fn end(&self) -> usize {
        self.end
    }

    /// Returns the number of matched operations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns whether the match is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Returns invocation-local operation IDs.
    #[must_use]
    pub fn operations(&self) -> &[OperationId] {
        &self.operations
    }

    /// Returns true when this match overlaps another match.
    #[must_use]
    pub fn overlaps(&self, other: &Self) -> bool {
        self.start < other.end && other.start < self.end
    }
}

// =============================================================================
// Rewrite replacement
// =============================================================================

/// Replacement generated by a rewrite rule.
///
/// `Replace` is the normal form. The other variants are provided for rule
/// authors because they make intent explicit and avoid repeated boilerplate.
#[derive(Debug, Clone, PartialEq)]
pub enum RewriteReplacement {
    /// Replace the complete matched range with these operations.
    Replace(Vec<Gate>),

    /// Delete the matched range.
    Delete,

    /// Keep the matched range unchanged.
    Keep,

    /// Insert operations before the matched range.
    InsertBefore(Vec<Gate>),

    /// Insert operations after the matched range.
    InsertAfter(Vec<Gate>),
}

impl RewriteReplacement {
    /// Converts a replacement into a normalized representation.
    ///
    /// The returned tuple is:
    ///
    /// `(remove_count, insert_before, replacement, insert_after)`
    #[must_use]
    pub fn normalize(
        &self,
    ) -> (&'static str, usize, usize) {
        match self {
            Self::Replace(gates) => ("replace", 1, gates.len()),
            Self::Delete => ("delete", 1, 0),
            Self::Keep => ("keep", 0, 0),
            Self::InsertBefore(gates) => {
                ("insert_before", 0, gates.len())
            }
            Self::InsertAfter(gates) => {
                ("insert_after", 0, gates.len())
            }
        }
    }
}

// =============================================================================
// Rewrite context
// =============================================================================

/// Read-only context supplied to rule preconditions/postconditions.
///
/// This deliberately contains only stable information needed by rewrite rules.
///
/// Future optimization modules can wrap richer services around this structure
/// without changing the rewrite engine's core API.
#[derive(Debug, Clone, Copy)]
pub struct RewriteContext<'a> {
    /// Complete circuit snapshot currently being considered.
    pub circuit: &'a [Gate],

    /// Concrete match.
    pub matched: &'a RewriteMatch,

    /// Operations covered by the match.
    pub matched_operations: &'a [Gate],

    /// Proposed replacement.
    pub replacement: &'a [Gate],

    /// Current rewrite iteration.
    pub iteration: u64,
}

// =============================================================================
// Rule conditions
// =============================================================================

/// Rule precondition.
///
/// A precondition must be side-effect free.
pub trait RewritePrecondition: Send + Sync {
    /// Validates whether the candidate may be applied.
    fn check(
        &self,
        context: &RewriteContext<'_>,
    ) -> Result<(), String>;
}

/// Rule postcondition.
///
/// A postcondition must be side-effect free.
pub trait RewritePostcondition: Send + Sync {
    /// Validates the proposed result.
    fn check(
        &self,
        context: &RewriteContext<'_>,
    ) -> Result<(), String>;
}

/// A precondition implemented by a closure.
pub struct FnPrecondition<F>
where
    F: Fn(&RewriteContext<'_>) -> Result<(), String>
        + Send
        + Sync,
{
    function: F,
}

impl<F> FnPrecondition<F>
where
    F: Fn(&RewriteContext<'_>) -> Result<(), String>
        + Send
        + Sync,
{
    /// Creates a closure-backed precondition.
    #[must_use]
    pub fn new(function: F) -> Self {
        Self { function }
    }
}

impl<F> RewritePrecondition for FnPrecondition<F>
where
    F: Fn(&RewriteContext<'_>) -> Result<(), String>
        + Send
        + Sync,
{
    fn check(
        &self,
        context: &RewriteContext<'_>,
    ) -> Result<(), String> {
        (self.function)(context)
    }
}

/// A postcondition implemented by a closure.
pub struct FnPostcondition<F>
where
    F: Fn(&RewriteContext<'_>) -> Result<(), String>
        + Send
        + Sync,
{
    function: F,
}

impl<F> FnPostcondition<F>
where
    F: Fn(&RewriteContext<'_>) -> Result<(), String>
        + Send
        + Sync,
{
    /// Creates a closure-backed postcondition.
    #[must_use]
    pub fn new(function: F) -> Self {
        Self { function }
    }
}

impl<F> RewritePostcondition for FnPostcondition<F>
where
    F: Fn(&RewriteContext<'_>) -> Result<(), String>
        + Send
        + Sync,
{
    fn check(
        &self,
        context: &RewriteContext<'_>,
    ) -> Result<(), String> {
        (self.function)(context)
    }
}

// =============================================================================
// Rewrite rule
// =============================================================================

/// A complete rewrite rule.
///
/// A rule contains no matching algorithm. Matching is delegated to
/// `RewriteMatcher`.
pub trait RewriteRule: Send + Sync {
    /// Stable rule identifier.
    fn id(&self) -> &RuleIdentifier;

    /// Human-readable rule name.
    fn name(&self) -> &str;

    /// Rule priority.
    fn priority(&self) -> RewritePriority {
        RewritePriority::default()
    }

    /// Whether the rule is deterministic.
    fn deterministic(&self) -> bool {
        true
    }

    /// Whether the rule is allowed to change operation count upward.
    fn allows_growth(&self) -> bool {
        false
    }

    /// Maximum replacement length for this rule.
    ///
    /// `None` means the engine-level limit is used.
    fn max_replacement_operations(&self) -> Option<usize> {
        None
    }

    /// Optional precondition.
    fn precondition(
        &self,
    ) -> Option<&dyn RewritePrecondition> {
        None
    }

    /// Optional postcondition.
    fn postcondition(
        &self,
    ) -> Option<&dyn RewritePostcondition> {
        None
    }

    /// Produces a replacement for a concrete match.
    fn replace(
        &self,
        context: &RewriteContext<'_>,
    ) -> RewriteResult<RewriteReplacement>;
}

// =============================================================================
// Matcher
// =============================================================================

/// Finds candidate matches for rewrite rules.
///
/// The matcher owns pattern-search behavior. The rewrite engine owns candidate
/// validation, deterministic ordering, conflict resolution, and application.
pub trait RewriteMatcher: Send + Sync {
    /// Finds candidates for one rule.
    fn find_matches(
        &self,
        rule: &dyn RewriteRule,
        circuit: &[Gate],
        iteration: u64,
    ) -> RewriteResult<Vec<RewriteMatch>>;
}

/// Matcher backed by a user-provided closure.
///
/// This is useful for:
///
/// - unit tests;
/// - small local rules;
/// - integration with `pattern.rs`;
/// - specialized high-performance matchers.
pub struct FnRewriteMatcher<F>
where
    F: Fn(
            &dyn RewriteRule,
            &[Gate],
            u64,
        ) -> RewriteResult<Vec<RewriteMatch>>
        + Send
        + Sync,
{
    function: F,
}

impl<F> FnRewriteMatcher<F>
where
    F: Fn(
            &dyn RewriteRule,
            &[Gate],
            u64,
        ) -> RewriteResult<Vec<RewriteMatch>>
        + Send
        + Sync,
{
    /// Creates a closure-backed matcher.
    #[must_use]
    pub fn new(function: F) -> Self {
        Self { function }
    }
}

impl<F> RewriteMatcher for FnRewriteMatcher<F>
where
    F: Fn(
            &dyn RewriteRule,
            &[Gate],
            u64,
        ) -> RewriteResult<Vec<RewriteMatch>>
        + Send
        + Sync,
{
    fn find_matches(
        &self,
        rule: &dyn RewriteRule,
        circuit: &[Gate],
        iteration: u64,
    ) -> RewriteResult<Vec<RewriteMatch>> {
        (self.function)(rule, circuit, iteration)
    }
}

// =============================================================================
// Observer
// =============================================================================

/// Optional observer for integrating rewrite execution with
/// `OptimizationContext`, statistics, provenance, cancellation, or diagnostics.
///
/// Implementations must not mutate the circuit.
pub trait RewriteObserver: Send + Sync {
    /// Called before a candidate is applied.
    fn before_apply(
        &self,
        _candidate: &RewriteCandidate<'_>,
    ) -> RewriteResult<()> {
        Ok(())
    }

    /// Called after a candidate has been accepted into the transaction.
    fn after_apply(
        &self,
        _record: &RewriteRecord,
    ) -> RewriteResult<()> {
        Ok(())
    }

    /// Called once after a rewrite iteration.
    fn after_iteration(
        &self,
        _iteration: u64,
        _statistics: &RewriteStatistics,
    ) -> RewriteResult<()> {
        Ok(())
    }

    /// Returns true when the observer requests cancellation.
    fn should_cancel(&self) -> bool {
        false
    }
}

// =============================================================================
// Candidate
// =============================================================================

/// A validated rewrite candidate.
#[derive(Debug, Clone)]
pub struct RewriteCandidate<'a> {
    /// Rule that generated the candidate.
    pub rule: &'a dyn RewriteRule,

    /// Concrete match.
    pub matched: RewriteMatch,

    /// Proposed replacement.
    pub replacement: RewriteReplacement,

    /// Replacement cost delta.
    pub cost: RewriteCost,

    /// Rewrite iteration.
    pub iteration: u64,
}

impl<'a> RewriteCandidate<'a> {
    /// Returns the normalized replacement sequence.
    pub fn replacement_gates(
        &self,
        matched: &[Gate],
    ) -> Vec<Gate> {
        match &self.replacement {
            RewriteReplacement::Replace(gates) => gates.clone(),

            RewriteReplacement::Delete => Vec::new(),

            RewriteReplacement::Keep => matched.to_vec(),

            RewriteReplacement::InsertBefore(gates) => {
                let mut result =
                    Vec::with_capacity(
                        gates.len()
                            .saturating_add(matched.len()),
                    );

                result.extend_from_slice(gates);
                result.extend_from_slice(matched);
                result
            }

            RewriteReplacement::InsertAfter(gates) => {
                let mut result =
                    Vec::with_capacity(
                        matched.len()
                            .saturating_add(gates.len()),
                    );

                result.extend_from_slice(matched);
                result.extend_from_slice(gates);
                result
            }
        }
    }

    /// Returns the candidate's operation-count delta.
    #[must_use]
    pub const fn operation_delta(&self) -> i64 {
        self.cost.operations
    }

    /// Returns true if the candidate reduces operation count.
    #[must_use]
    pub const fn is_reducing(&self) -> bool {
        self.cost.operations < 0
    }
}

// =============================================================================
// Rewrite record / provenance
// =============================================================================

/// Immutable record of one applied rewrite.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewriteRecord {
    /// Rule identifier.
    pub rule_id: RuleIdentifier,

    /// Rule name.
    pub rule_name: String,

    /// Start position at match time.
    pub start: usize,

    /// End position at match time.
    pub end: usize,

    /// Number of operations removed.
    pub removed_operations: usize,

    /// Number of operations inserted.
    pub inserted_operations: usize,

    /// Operation-count delta.
    pub operation_delta: i64,

    /// Rewrite iteration.
    pub iteration: u64,

    /// Stable ordinal of this rewrite within the engine invocation.
    pub ordinal: u64,
}

impl RewriteRecord {
    /// Creates a record from a candidate.
    fn from_candidate(
        candidate: &RewriteCandidate<'_>,
        replacement_len: usize,
        ordinal: u64,
    ) -> Self {
        Self {
            rule_id: candidate.rule.id().clone(),
            rule_name: candidate.rule.name().to_owned(),
            start: candidate.matched.start(),
            end: candidate.matched.end(),
            removed_operations: candidate.matched.len(),
            inserted_operations: replacement_len,
            operation_delta: candidate.cost.operations,
            iteration: candidate.iteration,
            ordinal,
        }
    }
}

// =============================================================================
// Statistics
// =============================================================================

/// Rewrite-engine execution statistics.
///
/// These counters are deliberately independent from the global optimization
/// statistics layer.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RewriteStatistics {
    /// Number of rewrite iterations.
    pub iterations: u64,

    /// Number of candidates generated.
    pub candidates_generated: u64,

    /// Number of candidates rejected by overlap/conflict rules.
    pub candidates_rejected: u64,

    /// Number of candidates rejected by preconditions.
    pub preconditions_failed: u64,

    /// Number of candidates rejected by postconditions.
    pub postconditions_failed: u64,

    /// Number of rewrites applied.
    pub rewrites_applied: u64,

    /// Number of operations removed.
    pub operations_removed: u64,

    /// Number of operations inserted.
    pub operations_inserted: u64,

    /// Number of transactions committed.
    pub transactions_committed: u64,

    /// Number of times a fixed point was reached.
    pub fixed_points_reached: u64,

    /// Number of iterations that produced no rewrite.
    pub unchanged_iterations: u64,

    /// Total operation-count delta.
    pub operation_delta: i64,

    /// Number of matcher calls.
    pub matcher_invocations: u64,
}

impl RewriteStatistics {
    /// Returns true when at least one rewrite was applied.
    #[must_use]
    pub const fn changed(&self) -> bool {
        self.rewrites_applied != 0
    }
}

// =============================================================================
// Engine configuration
// =============================================================================

/// Selection strategy for conflicting rewrite candidates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewriteSelection {
    /// Apply the first deterministic non-conflicting candidate at each region.
    First,

    /// Prefer the highest-priority candidate.
    Priority,

    /// Prefer the candidate producing the best local operation-count reduction.
    BestLocalCost,

    /// Prefer the highest priority, then best local cost.
    PriorityThenCost,
}

impl Default for RewriteSelection {
    fn default() -> Self {
        Self::PriorityThenCost
    }
}

/// Fixed-point behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewriteTermination {
    /// Execute exactly one candidate-selection/application iteration.
    Once,

    /// Continue until no candidate is applied.
    FixedPoint,

    /// Continue until the configured iteration limit is reached.
    BoundedFixedPoint,
}

impl Default for RewriteTermination {
    fn default() -> Self {
        Self::BoundedFixedPoint
    }
}

/// Resource budgets for the rewrite engine.
///
/// Zero means "no work permitted", not "unlimited".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RewriteBudget {
    /// Maximum generated candidates across the invocation.
    pub max_candidates: u64,

    /// Maximum successfully applied rewrites.
    pub max_rewrites: u64,

    /// Maximum fixed-point iterations.
    pub max_iterations: u64,

    /// Maximum number of operations inserted across the invocation.
    pub max_inserted_operations: u64,

    /// Maximum number of operations removed across the invocation.
    pub max_removed_operations: u64,

    /// Maximum operation count of the working sequence.
    pub max_circuit_operations: usize,

    /// Maximum number of candidates generated by one rule in one iteration.
    pub max_candidates_per_rule: u64,
}

impl Default for RewriteBudget {
    fn default() -> Self {
        Self {
            max_candidates: 1_000_000,
            max_rewrites: 1_000_000,
            max_iterations: 128,
            max_inserted_operations: 10_000_000,
            max_removed_operations: 10_000_000,
            max_circuit_operations: usize::MAX,
            max_candidates_per_rule: 100_000,
        }
    }
}

impl RewriteBudget {
    /// Creates a conservative budget suitable for small compiler invocations.
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            max_candidates: 100_000,
            max_rewrites: 100_000,
            max_iterations: 32,
            max_inserted_operations: 1_000_000,
            max_removed_operations: 1_000_000,
            max_circuit_operations: usize::MAX,
            max_candidates_per_rule: 10_000,
        }
    }

    /// Creates an effectively resource-driven budget.
    ///
    /// The rewrite engine still performs checked arithmetic and collection
    /// allocation can still fail due to system memory exhaustion.
    #[must_use]
    pub const fn resource_driven() -> Self {
        Self {
            max_candidates: u64::MAX,
            max_rewrites: u64::MAX,
            max_iterations: u64::MAX,
            max_inserted_operations: u64::MAX,
            max_removed_operations: u64::MAX,
            max_circuit_operations: usize::MAX,
            max_candidates_per_rule: u64::MAX,
        }
    }

    /// Validates the budget itself.
    pub fn validate(self) -> RewriteResult<()> {
        if self.max_iterations == 0 {
            return Err(RewriteError::IterationLimitExceeded {
                iterations: 0,
                maximum: 0,
            });
        }

        Ok(())
    }
}

/// Configuration of one rewrite-engine invocation.
#[derive(Debug, Clone, Copy)]
pub struct RewriteConfig {
    /// Candidate selection policy.
    pub selection: RewriteSelection,

    /// Fixed-point termination policy.
    pub termination: RewriteTermination,

    /// Resource budget.
    pub budget: RewriteBudget,

    /// Whether growth-producing rewrites are permitted.
    pub allow_growth: bool,

    /// Whether a candidate whose replacement is identical to the matched
    /// sequence should be treated as a no-op.
    pub reject_noop_rewrites: bool,
}

impl Default for RewriteConfig {
    fn default() -> Self {
        Self {
            selection: RewriteSelection::PriorityThenCost,
            termination: RewriteTermination::BoundedFixedPoint,
            budget: RewriteBudget::default(),
            allow_growth: false,
            reject_noop_rewrites: true,
        }
    }
}

// =============================================================================
// Rewrite engine
// =============================================================================

/// Generic production rewrite engine.
///
/// The engine is deliberately independent of the concrete pattern-matching
/// implementation.
pub struct RewriteEngine {
    config: RewriteConfig,
    rules: Vec<Box<dyn RewriteRule>>,
    matcher: Box<dyn RewriteMatcher>,
}

impl fmt::Debug for RewriteEngine {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RewriteEngine")
            .field("config", &self.config)
            .field("rules", &self.rules.len())
            .finish_non_exhaustive()
    }
}

impl RewriteEngine {
    /// Creates a rewrite engine.
    pub fn new(
        config: RewriteConfig,
        matcher: Box<dyn RewriteMatcher>,
    ) -> RewriteResult<Self> {
        config.budget.validate()?;

        Ok(Self {
            config,
            rules: Vec::new(),
            matcher,
        })
    }

    /// Returns the current configuration.
    #[must_use]
    pub const fn config(&self) -> &RewriteConfig {
        &self.config
    }

    /// Adds a rewrite rule.
    ///
    /// Rules are kept in deterministic insertion order. Candidate selection
    /// applies explicit priority/order rules rather than relying on hash-map
    /// iteration.
    pub fn register_rule(
        &mut self,
        rule: Box<dyn RewriteRule>,
    ) -> RewriteResult<()> {
        let id = rule.id().as_str();

        if id.trim().is_empty() {
            return Err(RewriteError::InvalidRuleIdentifier {
                message: "rule identifier must not be empty".to_owned(),
            });
        }

        if self.rules.iter().any(|existing| {
            existing.id() == rule.id()
        }) {
            return Err(RewriteError::InvalidRuleIdentifier {
                message: format!(
                    "duplicate rewrite rule identifier `{id}`"
                ),
            });
        }

        self.rules.push(rule);

        self.rules.sort_by(|left, right| {
            right
                .priority()
                .cmp(&left.priority())
                .then_with(|| {
                    left.id()
                        .as_str()
                        .cmp(right.id().as_str())
                })
        });

        Ok(())
    }

    /// Removes all registered rules.
    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }

    /// Returns the number of registered rules.
    #[must_use]
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Executes the configured rewrite process against an operation sequence.
    ///
    /// The input is never modified in place. A new sequence is returned only
    /// after each transaction has been validated.
    pub fn run(
        &self,
        input: &[Gate],
    ) -> RewriteResult<RewriteOutput> {
        self.run_with_observer(input, None)
    }

    /// Executes the rewrite process with an optional integration observer.
    pub fn run_with_observer(
        &self,
        input: &[Gate],
        observer: Option<&dyn RewriteObserver>,
    ) -> RewriteResult<RewriteOutput> {
        self.validate_input(input)?;

        let mut current = input.to_vec();
        let mut statistics = RewriteStatistics::default();
        let mut records = Vec::new();

        let mut iteration = 0u64;
        let mut ordinal = 0u64;

        loop {
            if let Some(observer) = observer {
                if observer.should_cancel() {
                    return Err(RewriteError::Cancelled);
                }
            }

            iteration = iteration
                .checked_add(1)
                .ok_or(RewriteError::ArithmeticOverflow {
                    calculation: "rewrite iteration",
                })?;

            if iteration > self.config.budget.max_iterations {
                return Err(RewriteError::IterationLimitExceeded {
                    iterations: iteration,
                    maximum: self.config.budget.max_iterations,
                });
            }

            statistics.iterations = iteration;

            let candidates =
                self.collect_candidates(&current, iteration, &mut statistics)?;

            if candidates.is_empty() {
                statistics.fixed_points_reached =
                    statistics.fixed_points_reached.saturating_add(1);

                statistics.unchanged_iterations =
                    statistics.unchanged_iterations.saturating_add(1);

                if let Some(observer) = observer {
                    observer.after_iteration(
                        iteration,
                        &statistics,
                    )?;
                }

                break;
            }

            let selected =
                self.select_candidates(&candidates, &mut statistics)?;

            if selected.is_empty() {
                statistics.unchanged_iterations =
                    statistics.unchanged_iterations.saturating_add(1);

                if let Some(observer) = observer {
                    observer.after_iteration(
                        iteration,
                        &statistics,
                    )?;
                }

                break;
            }

            let transaction =
                self.prepare_transaction(
                    &current,
                    &selected,
                    iteration,
                    observer,
                    &mut statistics,
                    &mut ordinal,
                )?;

            let next = transaction.commit(
                &current,
                self.config.budget.max_circuit_operations,
            )?;

            if next == current {
                statistics.fixed_points_reached =
                    statistics.fixed_points_reached.saturating_add(1);

                if let Some(observer) = observer {
                    observer.after_iteration(
                        iteration,
                        &statistics,
                    )?;
                }

                break;
            }

            current = next;

            if let Some(observer) = observer {
                observer.after_iteration(
                    iteration,
                    &statistics,
                )?;
            }

            match self.config.termination {
                RewriteTermination::Once => break,

                RewriteTermination::FixedPoint => {}

                RewriteTermination::BoundedFixedPoint => {
                    if iteration >= self.config.budget.max_iterations {
                        return Err(
                            RewriteError::IterationLimitExceeded {
                                iterations: iteration,
                                maximum: self
                                    .config
                                    .budget
                                    .max_iterations,
                            },
                        );
                    }
                }
            }
        }

        Ok(RewriteOutput {
            circuit: current,
            statistics,
            records,
        })
    }

    fn validate_input(
        &self,
        input: &[Gate],
    ) -> RewriteResult<()> {
        if input.len() > self.config.budget.max_circuit_operations {
            return Err(
                RewriteError::TransactionLimitExceeded {
                    requested: input.len(),
                    maximum: self
                        .config
                        .budget
                        .max_circuit_operations,
                },
            );
        }

        for gate in input {
            gate.validate().map_err(|error| {
                RewriteError::InvalidReplacement {
                    rule: "input".to_owned(),
                    message: error.to_string(),
                }
            })?;
        }

        Ok(())
    }

    fn collect_candidates(
        &self,
        circuit: &[Gate],
        iteration: u64,
        statistics: &mut RewriteStatistics,
    ) -> RewriteResult<Vec<RewriteCandidate<'_>>> {
        let mut candidates = Vec::new();

        for rule in &self.rules {
            if !rule.deterministic() {
                // Nondeterministic matching is allowed by the abstraction, but
                // ordering is still deterministic for whatever candidates the
                // matcher returns.
            }

            statistics.matcher_invocations =
                statistics.matcher_invocations.saturating_add(1);

            let matches = self.matcher.find_matches(
                rule.as_ref(),
                circuit,
                iteration,
            )?;

            if matches.len()
                > self
                    .config
                    .budget
                    .max_candidates_per_rule
            {
                return Err(
                    RewriteError::CandidateLimitExceeded {
                        requested: matches.len() as u64,
                        maximum: self
                            .config
                            .budget
                            .max_candidates_per_rule,
                    },
                );
            }

            for matched in matches {
                let candidate =
                    self.build_candidate(
                        rule.as_ref(),
                        matched,
                        circuit,
                        iteration,
                    )?;

                statistics.candidates_generated =
                    statistics
                        .candidates_generated
                        .checked_add(1)
                        .ok_or(
                            RewriteError::ArithmeticOverflow {
                                calculation:
                                    "rewrite candidate count",
                            },
                        )?;

                if statistics.candidates_generated
                    > self
                        .config
                        .budget
                        .max_candidates
                {
                    return Err(
                        RewriteError::CandidateLimitExceeded {
                            requested: statistics
                                .candidates_generated,
                            maximum: self
                                .config
                                .budget
                                .max_candidates,
                        },
                    );
                }

                if self.is_growth_disallowed(&candidate) {
                    statistics.candidates_rejected =
                        statistics.candidates_rejected
                            .saturating_add(1);

                    continue;
                }

                if self.config.reject_noop_rewrites
                    && candidate.cost.operations == 0
                {
                    let matched_operations =
                        &circuit[
                            candidate.matched.start()
                                ..candidate.matched.end()
                        ];

                    let replacement =
                        candidate.replacement_gates(
                            matched_operations,
                        );

                    if replacement == matched_operations {
                        statistics.candidates_rejected =
                            statistics
                                .candidates_rejected
                                .saturating_add(1);

                        continue;
                    }
                }

                candidates.push(candidate);
            }
        }

        candidates.sort_by(candidate_order);

        Ok(candidates)
    }

    fn build_candidate<'a>(
        &'a self,
        rule: &'a dyn RewriteRule,
        matched: RewriteMatch,
        circuit: &[Gate],
        iteration: u64,
    ) -> RewriteResult<RewriteCandidate<'a>> {
        if matched.end() > circuit.len()
            || matched.start() > matched.end()
        {
            return Err(RewriteError::InvalidMatch {
                rule: rule.id().to_string(),
                start: matched.start(),
                end: matched.end(),
                circuit_len: circuit.len(),
            });
        }

        if matched.operations().len() != matched.len() {
            return Err(RewriteError::InvalidOperationIds {
                rule: rule.id().to_string(),
                expected: matched.len(),
                actual: matched.operations().len(),
            });
        }

        let matched_operations =
            &circuit[matched.start()..matched.end()];

        let provisional_context =
            RewriteContext {
                circuit,
                matched: &matched,
                matched_operations,
                replacement: &[],
                iteration,
            };

        if let Some(precondition) =
            rule.precondition()
        {
            if let Err(reason) =
                precondition.check(&provisional_context)
            {
                return Err(
                    RewriteError::PreconditionFailed {
                        rule: rule.id().to_string(),
                        start: matched.start(),
                        reason,
                    },
                );
            }
        }

        let replacement =
            rule.replace(&provisional_context)?;

        let replacement_gates =
            validate_replacement(
                rule,
                &replacement,
                matched_operations,
                self.config.budget,
            )?;

        let cost =
            RewriteCost::from_sequences(
                matched_operations,
                &replacement_gates,
            );

        if !rule.allows_growth()
            && cost.operations > 0
        {
            return Err(
                RewriteError::InvalidReplacement {
                    rule: rule.id().to_string(),
                    message:
                        "rule produced an operation-count increase \
                         but does not allow growth"
                            .to_owned(),
                },
            );
        }

        let final_context =
            RewriteContext {
                circuit,
                matched: &matched,
                matched_operations,
                replacement: &replacement_gates,
                iteration,
            };

        if let Some(postcondition) =
            rule.postcondition()
        {
            if let Err(reason) =
                postcondition.check(&final_context)
            {
                return Err(
                    RewriteError::PostconditionFailed {
                        rule: rule.id().to_string(),
                        start: matched.start(),
                        reason,
                    },
                );
            }
        }

        Ok(RewriteCandidate {
            rule,
            matched,
            replacement,
            cost,
            iteration,
        })
    }

    fn is_growth_disallowed(
        &self,
        candidate: &RewriteCandidate<'_>,
    ) -> bool {
        !self.config.allow_growth
            && !candidate.rule.allows_growth()
            && candidate.cost.operations > 0
    }

    fn select_candidates<'a>(
        &self,
        candidates: &'a [RewriteCandidate<'a>],
        statistics: &mut RewriteStatistics,
    ) -> RewriteResult<Vec<&'a RewriteCandidate<'a>>> {
        let mut selected = Vec::new();

        for candidate in candidates {
            if selected.iter().any(|existing| {
                existing.matched.overlaps(&candidate.matched)
            }) {
                statistics.candidates_rejected =
                    statistics.candidates_rejected
                        .saturating_add(1);

                continue;
            }

            if let Some(previous) = selected.last() {
                if previous.matched.overlaps(&candidate.matched) {
                    return Err(
                        RewriteError::OverlappingCandidates {
                            first_rule: previous
                                .rule
                                .id()
                                .to_string(),
                            second_rule: candidate
                                .rule
                                .id()
                                .to_string(),
                            first_start: previous
                                .matched
                                .start(),
                            first_end: previous
                                .matched
                                .end(),
                            second_start: candidate
                                .matched
                                .start(),
                            second_end: candidate
                                .matched
                                .end(),
                        },
                    );
                }
            }

            selected.push(candidate);

            if matches!(
                self.config.selection,
                RewriteSelection::First
            ) {
                // Continue collecting non-overlapping candidates. "First"
                // refers to conflict resolution, not to abandoning the rest
                // of the circuit.
            }
        }

        Ok(selected)
    }

    fn prepare_transaction(
        &self,
        circuit: &[Gate],
        selected: &[&RewriteCandidate<'_>],
        iteration: u64,
        observer: Option<&dyn RewriteObserver>,
        statistics: &mut RewriteStatistics,
        ordinal: &mut u64,
    ) -> RewriteResult<RewriteTransaction> {
        let mut transaction =
            RewriteTransaction::new();

        for candidate in selected {
            let matched_operations =
                &circuit[
                    candidate.matched.start()
                        ..candidate.matched.end()
                ];

            let replacement =
                candidate.replacement_gates(
                    matched_operations,
                );

            if let Some(observer) = observer {
                observer.before_apply(candidate)?;
            }

            let replacement_len =
                replacement.len();

            if replacement_len
                > self
                    .config
                    .budget
                    .max_inserted_operations
                    .try_into()
                    .unwrap_or(usize::MAX)
            {
                return Err(
                    RewriteError::ReplacementLimitExceeded {
                        rule: candidate.rule.id().to_string(),
                        requested: replacement_len,
                        maximum: self
                            .config
                            .budget
                            .max_inserted_operations
                            .min(usize::MAX as u64)
                            as usize,
                    },
                );
            }

            *ordinal = ordinal
                .checked_add(1)
                .ok_or(RewriteError::ArithmeticOverflow {
                    calculation: "rewrite provenance ordinal",
                })?;

            let record =
                RewriteRecord::from_candidate(
                    candidate,
                    replacement_len,
                    *ordinal,
                );

            transaction.add(
                candidate.matched.start(),
                candidate.matched.end(),
                replacement,
                record,
            )?;

            statistics.rewrites_applied =
                statistics
                    .rewrites_applied
                    .checked_add(1)
                    .ok_or(
                        RewriteError::ArithmeticOverflow {
                            calculation:
                                "rewrite application count",
                        },
                    )?;

            statistics.operations_removed =
                statistics
                    .operations_removed
                    .checked_add(
                        candidate.matched.len() as u64
                    )
                    .ok_or(
                        RewriteError::ArithmeticOverflow {
                            calculation:
                                "rewrite removed-operation count",
                        },
                    )?;

            statistics.operations_inserted =
                statistics
                    .operations_inserted
                    .checked_add(
                        replacement_len as u64
                    )
                    .ok_or(
                        RewriteError::ArithmeticOverflow {
                            calculation:
                                "rewrite inserted-operation count",
                        },
                    )?;

            statistics.operation_delta =
                statistics
                    .operation_delta
                    .saturating_add(
                        candidate.cost.operations
                    );

            if statistics.rewrites_applied
                > self.config.budget.max_rewrites
            {
                return Err(
                    RewriteError::RewriteLimitExceeded {
                        requested: statistics
                            .rewrites_applied,
                        maximum: self
                            .config
                            .budget
                            .max_rewrites,
                    },
                );
            }

            if statistics.operations_inserted
                > self
                    .config
                    .budget
                    .max_inserted_operations
            {
                return Err(
                    RewriteError::TransactionLimitExceeded {
                        requested: statistics
                            .operations_inserted
                            .min(usize::MAX as u64)
                            as usize,
                        maximum: self
                            .config
                            .budget
                            .max_inserted_operations
                            .min(usize::MAX as u64)
                            as usize,
                    },
                );
            }

            if statistics.operations_removed
                > self
                    .config
                    .budget
                    .max_removed_operations
            {
                return Err(
                    RewriteError::TransactionLimitExceeded {
                        requested: statistics
                            .operations_removed
                            .min(usize::MAX as u64)
                            as usize,
                        maximum: self
                            .config
                            .budget
                            .max_removed_operations
                            .min(usize::MAX as u64)
                            as usize,
                    },
                );
            }

            if let Some(observer) = observer {
                observer.after_apply(&record)?;
            }

            let _ = iteration;
        }

        transaction.validate()?;

        Ok(transaction)
    }
}

// =============================================================================
// Candidate ordering
// =============================================================================

fn candidate_order(
    left: &RewriteCandidate<'_>,
    right: &RewriteCandidate<'_>,
) -> Ordering {
    left.matched
        .start()
        .cmp(&right.matched.start())
        .then_with(|| {
            right
                .matched
                .end()
                .cmp(&left.matched.end())
        })
        .then_with(|| {
            right
                .rule
                .priority()
                .cmp(&left.rule.priority())
        })
        .then_with(|| {
            left.rule.id()
                .as_str()
                .cmp(right.rule.id().as_str())
        })
        .then_with(|| {
            left.iteration.cmp(&right.iteration)
        })
}

// =============================================================================
// Replacement validation
// =============================================================================

fn validate_replacement(
    rule: &dyn RewriteRule,
    replacement: &RewriteReplacement,
    matched: &[Gate],
    budget: RewriteBudget,
) -> RewriteResult<Vec<Gate>> {
    let gates = match replacement {
        RewriteReplacement::Replace(gates) => gates.clone(),

        RewriteReplacement::Delete => Vec::new(),

        RewriteReplacement::Keep => matched.to_vec(),

        RewriteReplacement::InsertBefore(gates) => {
            let total = gates.len().checked_add(
                matched.len(),
            ).ok_or(
                RewriteError::ArithmeticOverflow {
                    calculation:
                        "insert-before replacement length",
                },
            )?;

            let mut result =
                Vec::with_capacity(total);

            result.extend_from_slice(gates);
            result.extend_from_slice(matched);

            result
        }

        RewriteReplacement::InsertAfter(gates) => {
            let total = matched.len().checked_add(
                gates.len(),
            ).ok_or(
                RewriteError::ArithmeticOverflow {
                    calculation:
                        "insert-after replacement length",
                },
            )?;

            let mut result =
                Vec::with_capacity(total);

            result.extend_from_slice(matched);
            result.extend_from_slice(gates);

            result
        }
    };

    let maximum =
        rule.max_replacement_operations()
            .unwrap_or(
                budget.max_inserted_operations
                    .min(usize::MAX as u64)
                    as usize,
            );

    if gates.len() > maximum {
        return Err(
            RewriteError::ReplacementLimitExceeded {
                rule: rule.id().to_string(),
                requested: gates.len(),
                maximum,
            },
        );
    }

    for gate in &gates {
        gate.validate().map_err(|error| {
            RewriteError::InvalidReplacement {
                rule: rule.id().to_string(),
                message: error.to_string(),
            }
        })?;
    }

    Ok(gates)
}

// =============================================================================
// Transaction
// =============================================================================

/// Atomic rewrite transaction.
///
/// Transactions are deliberately independent of `QuantumCircuit`. The
/// optimizer circuit layer can translate the transaction into its canonical
/// `CircuitEditPlan`.
#[derive(Debug, Clone)]
struct RewriteTransaction {
    edits: Vec<RewriteEdit>,
}

impl RewriteTransaction {
    fn new() -> Self {
        Self { edits: Vec::new() }
    }

    fn add(
        &mut self,
        start: usize,
        end: usize,
        replacement: Vec<Gate>,
        record: RewriteRecord,
    ) -> RewriteResult<()> {
        if start > end {
            return Err(RewriteError::InvalidMatch {
                rule: record.rule_id.to_string(),
                start,
                end,
                circuit_len: 0,
            });
        }

        for existing in &self.edits {
            if start < existing.end
                && existing.start < end
            {
                return Err(
                    RewriteError::OverlappingCandidates {
                        first_rule: existing
                            .record
                            .rule_id
                            .to_string(),
                        second_rule: record
                            .rule_id
                            .to_string(),
                        first_start: existing.start,
                        first_end: existing.end,
                        second_start: start,
                        second_end: end,
                    },
                );
            }
        }

        self.edits.push(RewriteEdit {
            start,
            end,
            replacement,
            record,
        });

        self.edits.sort_by(|left, right| {
            left.start
                .cmp(&right.start)
                .then_with(|| left.end.cmp(&right.end))
                .then_with(|| {
                    left.record
                        .rule_id
                        .as_str()
                        .cmp(right.record.rule_id.as_str())
                })
        });

        Ok(())
    }

    fn validate(&self) -> RewriteResult<()> {
        for window in self.edits.windows(2) {
            let left = &window[0];
            let right = &window[1];

            if left.end > right.start {
                return Err(
                    RewriteError::OverlappingCandidates {
                        first_rule: left
                            .record
                            .rule_id
                            .to_string(),
                        second_rule: right
                            .record
                            .rule_id
                            .to_string(),
                        first_start: left.start,
                        first_end: left.end,
                        second_start: right.start,
                        second_end: right.end,
                    },
                );
            }
        }

        Ok(())
    }

    fn commit(
        self,
        input: &[Gate],
        maximum_operations: usize,
    ) -> RewriteResult<Vec<Gate>> {
        self.validate()?;

        if self.edits.is_empty() {
            return Ok(input.to_vec());
        }

        let removed: usize =
            self.edits
                .iter()
                .try_fold(0usize, |total, edit| {
                    total.checked_add(
                        edit.end.saturating_sub(edit.start),
                    )
                })
                .ok_or(
                    RewriteError::ArithmeticOverflow {
                        calculation:
                            "transaction removed operations",
                    },
                )?;

        let inserted: usize =
            self.edits
                .iter()
                .try_fold(0usize, |total, edit| {
                    total.checked_add(
                        edit.replacement.len(),
                    )
                })
                .ok_or(
                    RewriteError::ArithmeticOverflow {
                        calculation:
                            "transaction inserted operations",
                    },
                )?;

        let output_len =
            input.len()
                .checked_sub(removed)
                .ok_or(RewriteError::ArithmeticOverflow {
                    calculation:
                        "transaction output subtraction",
                })?
                .checked_add(inserted)
                .ok_or(RewriteError::ArithmeticOverflow {
                    calculation:
                        "transaction output addition",
                })?;

        if output_len > maximum_operations {
            return Err(
                RewriteError::TransactionLimitExceeded {
                    requested: output_len,
                    maximum: maximum_operations,
                },
            );
        }

        let mut output =
            Vec::with_capacity(output_len);

        let mut cursor = 0usize;

        for edit in &self.edits {
            if edit.start > input.len()
                || edit.end > input.len()
                || edit.start > edit.end
            {
                return Err(
                    RewriteError::InvalidMatch {
                        rule: edit
                            .record
                            .rule_id
                            .to_string(),
                        start: edit.start,
                        end: edit.end,
                        circuit_len: input.len(),
                    },
                );
            }

            output.extend_from_slice(
                &input[cursor..edit.start],
            );

            output.extend_from_slice(
                &edit.replacement,
            );

            cursor = edit.end;
        }

        output.extend_from_slice(
            &input[cursor..],
        );

        for gate in &output {
            gate.validate().map_err(|error| {
                RewriteError::InvalidReplacement {
                    rule: "transaction".to_owned(),
                    message: error.to_string(),
                }
            })?;
        }

        Ok(output)
    }
}

/// One atomic edit inside a rewrite transaction.
#[derive(Debug, Clone)]
struct RewriteEdit {
    start: usize,
    end: usize,
    replacement: Vec<Gate>,
    record: RewriteRecord,
}

// =============================================================================
// Output
// =============================================================================

/// Result of one complete rewrite-engine invocation.
#[derive(Debug, Clone, PartialEq)]
pub struct RewriteOutput {
    /// Resulting canonical operation sequence.
    pub circuit: Vec<Gate>,

    /// Rewrite execution statistics.
    pub statistics: RewriteStatistics,

    /// Immutable provenance records.
    pub records: Vec<RewriteRecord>,
}

impl RewriteOutput {
    /// Returns true when at least one rewrite was applied.
    #[must_use]
    pub const fn changed(&self) -> bool {
        self.statistics.changed()
    }

    /// Returns the number of applied rewrites.
    #[must_use]
    pub const fn rewrites(&self) -> u64 {
        self.statistics.rewrites_applied
    }

    /// Returns the total operation-count delta.
    #[must_use]
    pub const fn operation_delta(&self) -> i64 {
        self.statistics.operation_delta
    }
}

// =============================================================================
// Convenience rule
// =============================================================================

/// Simple closure-backed rewrite rule.
///
/// This is useful for registering local rules without creating a dedicated
/// struct for every rule. More sophisticated rules should normally live in
/// `rules.rs`.
pub struct ClosureRewriteRule<F>
where
    F: for<'a> Fn(
            &RewriteContext<'a>,
        ) -> RewriteResult<RewriteReplacement>
        + Send
        + Sync,
{
    id: RuleIdentifier,
    name: String,
    priority: RewritePriority,
    allows_growth: bool,
    deterministic: bool,
    max_replacement_operations: Option<usize>,
    precondition:
        Option<Box<dyn RewritePrecondition>>,
    postcondition:
        Option<Box<dyn RewritePostcondition>>,
    replace: F,
}

impl<F> ClosureRewriteRule<F>
where
    F: for<'a> Fn(
            &RewriteContext<'a>,
        ) -> RewriteResult<RewriteReplacement>
        + Send
        + Sync,
{
    /// Creates a closure-backed rule.
    pub fn new(
        id: RuleIdentifier,
        name: impl Into<String>,
        replace: F,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            priority: RewritePriority::default(),
            allows_growth: false,
            deterministic: true,
            max_replacement_operations: None,
            precondition: None,
            postcondition: None,
            replace,
        }
    }

    /// Sets rule priority.
    #[must_use]
    pub const fn with_priority(
        mut self,
        priority: RewritePriority,
    ) -> Self {
        self.priority = priority;
        self
    }

    /// Allows this rule to increase operation count.
    #[must_use]
    pub const fn allow_growth(
        mut self,
        value: bool,
    ) -> Self {
        self.allows_growth = value;
        self
    }

    /// Sets deterministic metadata.
    #[must_use]
    pub const fn with_determinism(
        mut self,
        deterministic: bool,
    ) -> Self {
        self.deterministic = deterministic;
        self
    }

    /// Sets a replacement-size limit.
    #[must_use]
    pub const fn with_max_replacement_operations(
        mut self,
        maximum: usize,
    ) -> Self {
        self.max_replacement_operations =
            Some(maximum);

        self
    }

    /// Adds a precondition.
    #[must_use]
    pub fn with_precondition(
        mut self,
        precondition: Box<dyn RewritePrecondition>,
    ) -> Self {
        self.precondition = Some(precondition);
        self
    }

    /// Adds a postcondition.
    #[must_use]
    pub fn with_postcondition(
        mut self,
        postcondition: Box<dyn RewritePostcondition>,
    ) -> Self {
        self.postcondition = Some(postcondition);
        self
    }
}

impl<F> RewriteRule for ClosureRewriteRule<F>
where
    F: for<'a> Fn(
            &RewriteContext<'a>,
        ) -> RewriteResult<RewriteReplacement>
        + Send
        + Sync,
{
    fn id(&self) -> &RuleIdentifier {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn priority(&self) -> RewritePriority {
        self.priority
    }

    fn deterministic(&self) -> bool {
        self.deterministic
    }

    fn allows_growth(&self) -> bool {
        self.allows_growth
    }

    fn max_replacement_operations(&self) -> Option<usize> {
        self.max_replacement_operations
    }

    fn precondition(
        &self,
    ) -> Option<&dyn RewritePrecondition> {
        self.precondition.as_deref()
    }

    fn postcondition(
        &self,
    ) -> Option<&dyn RewritePostcondition> {
        self.postcondition.as_deref()
    }

    fn replace(
        &self,
        context: &RewriteContext<'_>,
    ) -> RewriteResult<RewriteReplacement> {
        (self.replace)(context)
    }
}

// =============================================================================
// Built-in generic predicates
// =============================================================================

/// Returns a precondition that requires a non-empty match.
#[must_use]
pub fn require_non_empty_match(
) -> Box<dyn RewritePrecondition> {
    Box::new(FnPrecondition::new(|context| {
        if context.matched.is_empty() {
            Err("rewrite match must not be empty".to_owned())
        } else {
            Ok(())
        }
    }))
}

/// Returns a precondition that requires all matched operations to be unitary.
#[must_use]
pub fn require_unitary_match(
) -> Box<dyn RewritePrecondition> {
    Box::new(FnPrecondition::new(|context| {
        if context
            .matched_operations
            .iter()
            .all(Gate::is_unitary)
        {
            Ok(())
        } else {
            Err(
                "rewrite match contains a non-unitary operation"
                    .to_owned(),
            )
        }
    }))
}

/// Returns a postcondition requiring the replacement to be non-empty.
#[must_use]
pub fn require_non_empty_replacement(
) -> Box<dyn RewritePostcondition> {
    Box::new(FnPostcondition::new(|context| {
        if context.replacement.is_empty() {
            Err(
                "replacement must not be empty".to_owned()
            )
        } else {
            Ok(())
        }
    }))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::gate::GateKind;
    use crate::quantum::ir::parameter::Parameter;
    use crate::quantum::ir::qubits::QubitId;

    fn rule_id(value: &str) -> RuleIdentifier {
        RuleIdentifier::new(value)
            .expect("test rule identifier must be valid")
    }

    fn x(qubit: usize) -> Gate {
        Gate::new(
            GateKind::X,
            vec![QubitId::new(qubit)],
            Vec::new(),
            None,
            None,
        )
        .expect("X gate must be valid")
    }

    fn h(qubit: usize) -> Gate {
        Gate::new(
            GateKind::H,
            vec![QubitId::new(qubit)],
            Vec::new(),
            None,
            None,
        )
        .expect("H gate must be valid")
    }

    fn make_match(
        rule: &RuleIdentifier,
        start: usize,
        end: usize,
    ) -> RewriteMatch {
        let operations = (start..end)
            .map(OperationId::new)
            .collect();

        RewriteMatch::new(
            rule.clone(),
            start,
            end,
            operations,
        )
        .expect("test match must be valid")
    }

    fn cancellation_rule(
        name: &'static str,
        gate_kind: GateKind,
    ) -> ClosureRewriteRule<
        impl for<'a> Fn(
                &RewriteContext<'a>,
            ) -> RewriteResult<RewriteReplacement>
            + Send
            + Sync,
    > {
        let id = rule_id(name);

        ClosureRewriteRule::new(
            id,
            name,
            move |_context| {
                let _ = gate_kind;

                Ok(RewriteReplacement::Delete)
            },
        )
    }

    fn engine_with_simple_matcher(
        rules: Vec<
            Box<dyn RewriteRule>,
        >,
    ) -> RewriteEngine {
        let matcher =
            FnRewriteMatcher::new(
                |rule, circuit, _iteration| {
                    if circuit.len() < 2 {
                        return Ok(Vec::new());
                    }

                    let id = rule.id().clone();

                    if rule.name() == "cancel.xx" {
                        let all_x = circuit
                            .iter()
                            .take(2)
                            .all(|gate| {
                                gate.kind() == GateKind::X
                            });

                        if all_x {
                            return Ok(vec![
                                make_match(
                                    &id,
                                    0,
                                    2,
                                ),
                            ]);
                        }
                    }

                    Ok(Vec::new())
                },
            );

        let mut engine =
            RewriteEngine::new(
                RewriteConfig::default(),
                Box::new(matcher),
            )
            .expect("engine configuration must be valid");

        for rule in rules {
            engine
                .register_rule(rule)
                .expect("rule registration must succeed");
        }

        engine
    }

    #[test]
    fn cancellation_rewrite_removes_two_x_gates() {
        let rule =
            cancellation_rule(
                "cancel.xx",
                GateKind::X,
            );

        let engine =
            engine_with_simple_matcher(
                vec![Box::new(rule)],
            );

        let input = vec![
            x(0),
            x(0),
        ];

        let output = engine
            .run(&input)
            .expect("rewrite should succeed");

        assert!(output.circuit.is_empty());
        assert_eq!(
            output.statistics.rewrites_applied,
            1
        );
        assert_eq!(
            output.statistics.operations_removed,
            2
        );
        assert_eq!(
            output.statistics.operation_delta,
            -2
        );
    }

    #[test]
    fn non_matching_circuit_is_unchanged() {
        let rule =
            cancellation_rule(
                "cancel.xx",
                GateKind::X,
            );

        let engine =
            engine_with_simple_matcher(
                vec![Box::new(rule)],
            );

        let input = vec![
            x(0),
            h(0),
        ];

        let output = engine
            .run(&input)
            .expect("rewrite should succeed");

        assert_eq!(output.circuit, input);
        assert_eq!(
            output.statistics.rewrites_applied,
            0
        );
    }

    #[test]
    fn overlapping_candidates_are_not_double_applied() {
        let id = rule_id("test.overlap");

        let rule =
            ClosureRewriteRule::new(
                id.clone(),
                "test.overlap",
                |_context| {
                    Ok(RewriteReplacement::Delete)
                },
            );

        let matcher =
            FnRewriteMatcher::new(
                move |rule, _circuit, _iteration| {
                    Ok(vec![
                        make_match(
                            rule.id(),
                            0,
                            2,
                        ),
                        make_match(
                            rule.id(),
                            1,
                            3,
                        ),
                    ])
                },
            );

        let mut engine =
            RewriteEngine::new(
                RewriteConfig {
                    termination:
                        RewriteTermination::Once,
                    ..RewriteConfig::default()
                },
                Box::new(matcher),
            )
            .expect("engine must be valid");

        engine
            .register_rule(Box::new(rule))
            .expect("rule must register");

        let input = vec![
            x(0),
            x(0),
            x(0),
        ];

        let output = engine
            .run(&input)
            .expect("overlap should be resolved");

        assert_eq!(
            output.statistics.rewrites_applied,
            1
        );
        assert_eq!(
            output.circuit.len(),
            1
        );
    }

    #[test]
    fn invalid_replacement_is_rejected() {
        let id = rule_id("invalid.replacement");

        let rule =
            ClosureRewriteRule::new(
                id,
                "invalid.replacement",
                |_context| {
                    let invalid =
                        Gate::new(
                            GateKind::RX,
                            vec![QubitId::new(0)],
                            vec![],
                            None,
                            None,
                        );

                    match invalid {
                        Ok(gate) => {
                            Ok(
                                RewriteReplacement::Replace(
                                    vec![gate],
                                ),
                            )
                        }

                        Err(error) => {
                            Err(
                                RewriteError::InvalidReplacement {
                                    rule:
                                        "invalid.replacement"
                                            .to_owned(),
                                    message:
                                        error.to_string(),
                                },
                            )
                        }
                    }
                },
            );

        let matcher =
            FnRewriteMatcher::new(
                move |rule, _circuit, _iteration| {
                    Ok(vec![
                        make_match(
                            rule.id(),
                            0,
                            1,
                        ),
                    ])
                },
            );

        let mut engine =
            RewriteEngine::new(
                RewriteConfig {
                    termination:
                        RewriteTermination::Once,
                    ..RewriteConfig::default()
                },
                Box::new(matcher),
            )
            .expect("engine must be valid");

        engine
            .register_rule(Box::new(rule))
            .expect("rule must register");

        let input = vec![x(0)];

        let result = engine.run(&input);

        assert!(result.is_err());
    }

    #[test]
    fn deterministic_rule_order_is_stable() {
        let low =
            ClosureRewriteRule::new(
                rule_id("rule.z"),
                "rule.z",
                |_context| {
                    Ok(RewriteReplacement::Delete)
                },
            )
            .with_priority(
                RewritePriority::new(1),
            );

        let high =
            ClosureRewriteRule::new(
                rule_id("rule.a"),
                "rule.a",
                |_context| {
                    Ok(RewriteReplacement::Delete)
                },
            )
            .with_priority(
                RewritePriority::new(10),
            );

        let matcher =
            FnRewriteMatcher::new(
                move |rule, _circuit, _iteration| {
                    Ok(vec![
                        make_match(
                            rule.id(),
                            0,
                            1,
                        ),
                    ])
                },
            );

        let mut engine =
            RewriteEngine::new(
                RewriteConfig {
                    termination:
                        RewriteTermination::Once,
                    ..RewriteConfig::default()
                },
                Box::new(matcher),
            )
            .expect("engine must be valid");

        engine
            .register_rule(Box::new(low))
            .expect("low rule must register");

        engine
            .register_rule(Box::new(high))
            .expect("high rule must register");

        assert_eq!(
            engine.rules[0].id().as_str(),
            "rule.a"
        );
        assert_eq!(
            engine.rules[1].id().as_str(),
            "rule.z"
        );
    }

    #[test]
    fn growth_is_rejected_by_default() {
        let rule =
            ClosureRewriteRule::new(
                rule_id("growth"),
                "growth",
                |_context| {
                    Ok(
                        RewriteReplacement::Replace(
                            vec![
                                x(0),
                                h(0),
                            ],
                        ),
                    )
                },
            );

        let matcher =
            FnRewriteMatcher::new(
                move |rule, _circuit, _iteration| {
                    Ok(vec![
                        make_match(
                            rule.id(),
                            0,
                            1,
                        ),
                    ])
                },
            );

        let mut engine =
            RewriteEngine::new(
                RewriteConfig {
                    termination:
                        RewriteTermination::Once,
                    ..RewriteConfig::default()
                },
                Box::new(matcher),
            )
            .expect("engine must be valid");

        engine
            .register_rule(Box::new(rule))
            .expect("rule must register");

        let result =
            engine.run(&[x(0)]);

        assert!(result.is_err());
    }

    #[test]
    fn growth_can_be_explicitly_enabled() {
        let rule =
            ClosureRewriteRule::new(
                rule_id("growth.allowed"),
                "growth.allowed",
                |_context| {
                    Ok(
                        RewriteReplacement::Replace(
                            vec![
                                x(0),
                                h(0),
                            ],
                        ),
                    )
                },
            )
            .allow_growth(true);

        let matcher =
            FnRewriteMatcher::new(
                move |rule, _circuit, _iteration| {
                    Ok(vec![
                        make_match(
                            rule.id(),
                            0,
                            1,
                        ),
                    ])
                },
            );

        let mut engine =
            RewriteEngine::new(
                RewriteConfig {
                    allow_growth: true,
                    termination:
                        RewriteTermination::Once,
                    ..RewriteConfig::default()
                },
                Box::new(matcher),
            )
            .expect("engine must be valid");

        engine
            .register_rule(Box::new(rule))
            .expect("rule must register");

        let output =
            engine
                .run(&[x(0)])
                .expect("growth should be allowed");

        assert_eq!(
            output.circuit.len(),
            2
        );
    }

    #[test]
    fn unitary_precondition_accepts_unitary_gate() {
        let context_match = make_match(
            &rule_id("test"),
            0,
            1,
        );

        let input = vec![x(0)];

        let replacement = input.clone();

        let context = RewriteContext {
            circuit: &input,
            matched: &context_match,
            matched_operations: &input,
            replacement: &replacement,
            iteration: 1,
        };

        assert!(
            require_unitary_match()
                .check(&context)
                .is_ok()
        );
    }

    #[test]
    fn rewrite_cost_counts_t_gates() {
        let t_gate = Gate::new(
            GateKind::T,
            vec![QubitId::new(0)],
            Vec::new(),
            None,
            None,
        )
        .expect("T gate must be valid");

        let tdg_gate = Gate::new(
            GateKind::Tdg,
            vec![QubitId::new(0)],
            Vec::new(),
            None,
            None,
        )
        .expect("Tdg gate must be valid");

        let cost =
            RewriteCost::from_sequences(
                &[t_gate, tdg_gate],
                &[],
            );

        assert_eq!(cost.operations, -2);
        assert_eq!(cost.t_count, -2);
    }

    #[test]
    fn replacement_validation_accepts_valid_gate() {
        let gate = Gate::new(
            GateKind::RX,
            vec![QubitId::new(0)],
            vec![
                Parameter::Constant(0.5),
            ],
            None,
            None,
        )
        .expect("RX gate must be valid");

        let result =
            validate_replacement(
                &ClosureRewriteRule::new(
                    rule_id("valid"),
                    "valid",
                    |_context| {
                        Ok(
                            RewriteReplacement::Keep
                        )
                    },
                ),
                &RewriteReplacement::Replace(
                    vec![gate],
                ),
                &[],
                RewriteBudget::default(),
            );

        assert!(result.is_ok());
    }
}