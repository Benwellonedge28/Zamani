//! Zamani Quantum Optimization — Loop Structure
//!
//! Production-grade loop modeling for the logical quantum optimization layer.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source / frontend / canonical Quantum IR
//!                         │
//!                         ▼
//!              optimization::circuit
//!                         │
//!                         ▼
//!              optimization::structure
//!                         │
//!             ┌───────────┴───────────┐
//!             │                       │
//!          Block                  LoopStructure
//!             │                       │
//!             └───────────┬───────────┘
//!                         ▼
//!                    optimization
//!                         │
//!                         ▼
//!                      routing
//!                         │
//!                         ▼
//!                    scheduling
//!                         │
//!                         ▼
//!                     hardware
//! ```
//!
//! `LoopStructure` is an optimizer-owned structural description.
//!
//! It is NOT:
//!
//! - a second quantum IR;
//! - a replacement for the canonical Quantum IR;
//! - a classical-language AST;
//! - an execution engine;
//! - a runtime loop scheduler;
//! - a hardware loop controller;
//! - a loop unroller;
//! - a QPU API.
//!
//! The authoritative quantum representation remains the canonical Quantum IR.
//!
//! # Purpose
//!
//! Quantum programs may contain loops whose bodies contain quantum operations.
//! Optimizing such programs requires considerably more information than simply
//! knowing the range of operations belonging to the body.
//!
//! In particular, an optimizer must distinguish:
//!
//! - statically known iteration counts;
//! - bounded but runtime-dependent iteration counts;
//! - dynamically terminating loops;
//! - loops with no known finite upper bound;
//! - zero-iteration loops;
//! - exactly-one-iteration loops;
//! - nested loops;
//! - loops whose condition depends on measurements;
//! - loops whose condition depends only on classical state;
//! - loops whose body is invariant with respect to the loop condition;
//! - loops whose body may alter the condition;
//! - loops that may not safely be unrolled;
//! - loops that may be optimized by fixed-point reasoning without unrolling.
//!
//! This module provides the structural contract required by later optimizer
//! components.
//!
//! # Core design rule
//!
//! A loop descriptor does NOT imply that its body can be duplicated, moved,
//! removed, or transformed.
//!
//! Every transformation still has to prove semantic preservation.
//!
//! For example:
//!
//! ```text
//! while condition {
//!     H(q);
//! }
//! ```
//!
//! cannot automatically be treated as:
//!
//! ```text
//! H(q);
//! ```
//!
//! because the runtime may execute the body zero, one, or many times.
//!
//! Likewise, a loop whose condition depends on a measurement cannot be treated
//! as a statically known repetition merely because its body has a fixed number
//! of operations.
//!
//! # Integration contract
//!
//! This module intentionally depends only on `structure::block` and the Rust
//! standard library.
//!
//! ```text
//! optimization::structure::block
//!                 │
//!                 ▼
//! optimization::structure::loop
//! ```
//!
//! It does NOT depend on:
//!
//! - `OptimizationPipeline`;
//! - `OptimizationPass`;
//! - `OptimizationContext`;
//! - `CircuitEditor`;
//! - routing;
//! - scheduling;
//! - hardware;
//! - benchmarking;
//! - execution;
//! - a particular optimization algorithm;
//! - frontend AST types.
//!
//! This dependency direction is intentional so that this file can be completed
//! independently and does not need to be rewritten when later optimizer files
//! are implemented.
//!
//! # Consumers
//!
//! Future modules may consume this file:
//!
//! - `structure/region.rs` can place loops into region hierarchies;
//! - `structure/conditional.rs` can model conditions surrounding loop execution;
//! - `structure/control_flow.rs` can aggregate loops and branches;
//! - `analysis/dependency.rs` can analyze loop-carried dependencies;
//! - `analysis/liveness.rs` can analyze qubit liveness across iterations;
//! - `analysis/depth.rs` can calculate static lower/upper depth bounds;
//! - `passes/optimize_depth.rs` can use loop summaries;
//! - `passes/optimize_width.rs` can reason about loop-local resources;
//! - `passes/normalize.rs` can normalize loop metadata;
//! - `planner.rs` can choose whether loop transformations are worthwhile;
//! - `verification/*` can verify loop transformations.
//!
//! None of those modules are required for this file to compile conceptually.
//!
//! # Scaling
//!
//! A loop descriptor stores metadata and references to optimizer-local block
//! identifiers. It does not clone the loop body or quantum operations.
//!
//! Therefore the representation is O(1) with respect to the number of
//! operations in the body.
//!
//! This module does not artificially cap iteration counts at a small integer.
//! Large iteration counts use checked arithmetic and explicit symbolic/runtime
//! classifications.
//!
//! "Infinite" or otherwise unbounded execution is represented as an explicit
//! semantic category rather than by overflowing an integer.
//!
//! Actual resource limits belong to `OptimizationLimits` and to the individual
//! optimization passes.
//!
//! # Determinism
//!
//! This module is deterministic:
//!
//! - no randomness;
//! - no hash iteration;
//! - no global mutable state;
//! - no runtime clocks;
//! - no backend calls;
//! - no unsafe code.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1.
//!
//! No nightly features are required.
//!
//! No external dependencies are required.
//!
//! # Safety
//!
//! This file contains no `unsafe` code.

use std::fmt;

use super::block::{
    BlockDescriptor,
    BlockId,
    BlockKind,
};

// ============================================================================
// Loop identifiers
// ============================================================================

/// Invocation-local identifier for an optimizer loop.
///
/// This identifier is intentionally separate from `BlockId`.
///
/// A loop may own multiple structural blocks, such as:
///
/// - preheader;
/// - condition;
/// - body;
/// - latch;
/// - exit.
///
/// Therefore a loop is not equivalent to a single block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LoopId(usize);

impl LoopId {
    /// Creates an invocation-local loop identifier.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the invocation-local index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for LoopId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "loop{}", self.0)
    }
}

// ============================================================================
// Loop kind
// ============================================================================

/// Structural kind of a loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoopKind {
    /// A conventional `while`-style loop whose condition is evaluated before
    /// each possible body execution.
    While,

    /// A conventional `do ... while`-style loop whose body executes before
    /// the first condition evaluation.
    DoWhile,

    /// A counted loop whose iteration domain is structurally known.
    For,

    /// A loop whose exact source-level kind is intentionally abstracted away.
    Generic,

    /// A loop generated or introduced by a compiler transformation.
    Generated,
}

impl Default for LoopKind {
    fn default() -> Self {
        Self::Generic
    }
}

impl LoopKind {
    /// Returns true if the body is guaranteed to execute at least once.
    #[must_use]
    pub const fn guarantees_one_iteration(self) -> bool {
        matches!(self, Self::DoWhile)
    }

    /// Returns true if the condition is evaluated before the body.
    #[must_use]
    pub const fn is_pre_test(self) -> bool {
        matches!(self, Self::While | Self::For)
    }

    /// Returns true if the loop is naturally represented as a counted loop.
    #[must_use]
    pub const fn is_counted(self) -> bool {
        matches!(self, Self::For)
    }
}

// ============================================================================
// Iteration bounds
// ============================================================================

/// Static/runtime classification of the number of loop iterations.
///
/// This is deliberately richer than `Option<usize>`.
///
/// `None` alone cannot distinguish:
///
/// - unknown;
/// - runtime bounded;
/// - unbounded;
/// - impossible;
/// - symbolic.
///
/// Those distinctions matter for safe quantum optimization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IterationBound {
    /// The loop executes exactly `count` times.
    Exact(usize),

    /// The loop may execute zero or more times, with a known finite maximum.
    ///
    /// The minimum is represented separately by `IterationSummary`.
    Bounded {
        /// Inclusive minimum number of iterations.
        min: usize,

        /// Inclusive maximum number of iterations.
        max: usize,
    },

    /// The loop has a finite runtime-dependent count, but no useful static
    /// upper bound is available.
    DynamicFinite,

    /// The loop may execute without a statically known finite upper bound.
    Unbounded,

    /// The compiler cannot determine the iteration cardinality.
    Unknown,
}

impl Default for IterationBound {
    fn default() -> Self {
        Self::Unknown
    }
}

impl IterationBound {
    /// Creates an exact iteration bound.
    #[must_use]
    pub const fn exact(count: usize) -> Self {
        Self::Exact(count)
    }

    /// Creates a bounded iteration range.
    ///
    /// Returns `None` if `min > max`.
    #[must_use]
    pub const fn bounded(min: usize, max: usize) -> Option<Self> {
        if min > max {
            None
        } else {
            Some(Self::Bounded { min, max })
        }
    }

    /// Returns true when the exact number of iterations is known.
    #[must_use]
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact(_))
    }

    /// Returns true when a finite maximum is statically known.
    #[must_use]
    pub const fn has_finite_maximum(self) -> bool {
        matches!(
            self,
            Self::Exact(_) | Self::Bounded { .. }
        )
    }

    /// Returns true when the loop can execute zero times.
    #[must_use]
    pub const fn may_execute_zero_times(self) -> bool {
        match self {
            Self::Exact(count) => count == 0,
            Self::Bounded { min, .. } => min == 0,
            Self::DynamicFinite
            | Self::Unbounded
            | Self::Unknown => true,
        }
    }

    /// Returns true when the loop is guaranteed to execute at least once.
    #[must_use]
    pub const fn guaranteed_nonzero(self) -> bool {
        match self {
            Self::Exact(count) => count > 0,
            Self::Bounded { min, .. } => min > 0,
            Self::DynamicFinite
            | Self::Unbounded
            | Self::Unknown => false,
        }
    }

    /// Returns an exact count if one is available.
    #[must_use]
    pub const fn exact_count(self) -> Option<usize> {
        match self {
            Self::Exact(count) => Some(count),
            Self::Bounded { .. }
            | Self::DynamicFinite
            | Self::Unbounded
            | Self::Unknown => None,
        }
    }

    /// Returns the known maximum, if one exists.
    #[must_use]
    pub const fn maximum(self) -> Option<usize> {
        match self {
            Self::Exact(count) => Some(count),
            Self::Bounded { max, .. } => Some(max),
            Self::DynamicFinite
            | Self::Unbounded
            | Self::Unknown => None,
        }
    }

    /// Returns the known minimum, if one exists.
    #[must_use]
    pub const fn minimum(self) -> Option<usize> {
        match self {
            Self::Exact(count) => Some(count),
            Self::Bounded { min, .. } => Some(min),
            Self::DynamicFinite
            | Self::Unbounded
            | Self::Unknown => None,
        }
    }

    /// Returns true when static full unrolling is semantically cardinality-safe
    /// based solely on this bound.
    ///
    /// This does NOT mean that unrolling is actually safe. Dependencies,
    /// measurements, classical state, resource limits, and cost models still
    /// need to be checked.
    #[must_use]
    pub const fn permits_exact_unroll(self) -> bool {
        matches!(self, Self::Exact(_))
    }
}

// ============================================================================
// Loop condition classification
// ============================================================================

/// Classification of the value that controls loop continuation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoopConditionKind {
    /// Condition is known statically to be true.
    AlwaysTrue,

    /// Condition is known statically to be false.
    AlwaysFalse,

    /// Condition depends only on compile-time/static information.
    Static,

    /// Condition depends on classical runtime state.
    ClassicalRuntime,

    /// Condition depends on a quantum measurement/result.
    MeasurementDependent,

    /// Condition depends on quantum/classical state whose exact dependency
    /// cannot be represented here.
    QuantumDependent,

    /// Condition is intentionally opaque to this optimizer layer.
    Opaque,

    /// Condition could not be classified.
    Unknown,
}

impl Default for LoopConditionKind {
    fn default() -> Self {
        Self::Unknown
    }
}

impl LoopConditionKind {
    /// Returns true if the condition may depend on runtime state.
    #[must_use]
    pub const fn is_runtime_dependent(self) -> bool {
        matches!(
            self,
            Self::ClassicalRuntime
                | Self::MeasurementDependent
                | Self::QuantumDependent
                | Self::Opaque
                | Self::Unknown
        )
    }

    /// Returns true when the condition is known to be measurement-dependent.
    #[must_use]
    pub const fn is_measurement_dependent(self) -> bool {
        matches!(self, Self::MeasurementDependent)
    }

    /// Returns true when the loop is statically known never to execute.
    #[must_use]
    pub const fn is_always_false(self) -> bool {
        matches!(self, Self::AlwaysFalse)
    }

    /// Returns true when the loop is statically known to continue forever.
    #[must_use]
    pub const fn is_always_true(self) -> bool {
        matches!(self, Self::AlwaysTrue)
    }
}

// ============================================================================
// Loop-carried state
// ============================================================================

/// Classification of state carried from one iteration into the next.
///
/// This is intentionally conservative.
///
/// A loop optimizer must not assume that a loop body is independent across
/// iterations merely because the body contains unitary operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoopCarriedState {
    /// No loop-carried state has been identified.
    None,

    /// Only classical state is carried.
    Classical,

    /// Quantum state is carried between iterations.
    Quantum,

    /// Measurement results affect subsequent iterations.
    Measurement,

    /// Both classical and quantum state are carried.
    ClassicalAndQuantum,

    /// The optimizer cannot determine the carried state.
    Unknown,
}

impl Default for LoopCarriedState {
    fn default() -> Self {
        Self::Unknown
    }
}

impl LoopCarriedState {
    /// Returns true when quantum state is carried across iterations.
    #[must_use]
    pub const fn carries_quantum_state(self) -> bool {
        matches!(
            self,
            Self::Quantum
                | Self::ClassicalAndQuantum
        )
    }

    /// Returns true when measurement state participates in loop-carried
    /// semantics.
    #[must_use]
    pub const fn carries_measurement_dependency(self) -> bool {
        matches!(
            self,
            Self::Measurement
                | Self::ClassicalAndQuantum
        )
    }

    /// Returns true when the optimizer must conservatively assume unknown
    /// state dependencies.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

// ============================================================================
// Loop transformation capabilities
// ============================================================================

/// Conservative capabilities inferred for a loop.
///
/// These flags are intentionally separate from the loop kind and iteration
/// count. A loop may have a static count but still be unsafe to unroll.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoopCapabilities {
    /// The loop body can be analyzed as a closed structural unit.
    body_analyzable: bool,

    /// The loop can be legally considered for exact unrolling, subject to
    /// pass-specific cost/resource checks.
    exact_unroll_candidate: bool,

    /// The loop can be considered for partial unrolling.
    partial_unroll_candidate: bool,

    /// The loop body may be optimized once and reused for all iterations.
    body_local_optimization: bool,

    /// Loop-invariant operations may potentially be identified.
    invariant_analysis: bool,

    /// The loop may be simplified using its known iteration cardinality.
    cardinality_simplification: bool,

    /// The loop may be represented by a compact summary instead of expanded.
    summary_optimization: bool,

    /// The loop may safely participate in fixed-point optimization without
    /// requiring physical unrolling.
    fixed_point_optimization: bool,
}

impl Default for LoopCapabilities {
    fn default() -> Self {
        Self {
            body_analyzable: true,
            exact_unroll_candidate: false,
            partial_unroll_candidate: false,
            body_local_optimization: true,
            invariant_analysis: true,
            cardinality_simplification: true,
            summary_optimization: true,
            fixed_point_optimization: true,
        }
    }
}

impl LoopCapabilities {
    /// Returns conservative capabilities derived from loop metadata.
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            body_analyzable: true,
            exact_unroll_candidate: false,
            partial_unroll_candidate: false,
            body_local_optimization: true,
            invariant_analysis: true,
            cardinality_simplification: true,
            summary_optimization: true,
            fixed_point_optimization: true,
        }
    }

    /// Returns whether body-local optimization is permitted.
    #[must_use]
    pub const fn body_local_optimization(self) -> bool {
        self.body_local_optimization
    }

    /// Returns whether exact unrolling is a candidate.
    #[must_use]
    pub const fn exact_unroll_candidate(self) -> bool {
        self.exact_unroll_candidate
    }

    /// Returns whether partial unrolling is a candidate.
    #[must_use]
    pub const fn partial_unroll_candidate(self) -> bool {
        self.partial_unroll_candidate
    }

    /// Returns whether invariant analysis is enabled.
    #[must_use]
    pub const fn invariant_analysis(self) -> bool {
        self.invariant_analysis
    }

    /// Returns whether cardinality simplification is enabled.
    #[must_use]
    pub const fn cardinality_simplification(self) -> bool {
        self.cardinality_simplification
    }

    /// Returns whether summary-based optimization is permitted.
    #[must_use]
    pub const fn summary_optimization(self) -> bool {
        self.summary_optimization
    }

    /// Returns whether fixed-point optimization is permitted.
    #[must_use]
    pub const fn fixed_point_optimization(self) -> bool {
        self.fixed_point_optimization
    }

    /// Returns whether the body is structurally analyzable.
    #[must_use]
    pub const fn body_analyzable(self) -> bool {
        self.body_analyzable
    }
}

// ============================================================================
// Loop summary
// ============================================================================

/// Compact summary of loop execution behavior.
///
/// This is useful for large programs because downstream analyses do not need
/// to materialize every possible runtime iteration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoopSummary {
    /// Minimum possible number of iterations.
    min_iterations: Option<usize>,

    /// Maximum possible number of iterations.
    max_iterations: Option<usize>,

    /// Whether the loop is statically guaranteed to terminate.
    statically_terminating: bool,

    /// Whether zero iterations are possible.
    may_execute_zero: bool,

    /// Whether at least one iteration is guaranteed.
    guarantees_one: bool,

    /// Whether runtime-dependent termination exists.
    runtime_termination: bool,
}

impl LoopSummary {
    /// Constructs a summary from loop metadata.
    #[must_use]
    pub const fn from_bound(
        bound: IterationBound,
        kind: LoopKind,
        condition: LoopConditionKind,
    ) -> Self {
        let minimum = bound.minimum();
        let maximum = bound.maximum();

        let guarantees_one =
            kind.guarantees_one_iteration()
                && !condition.is_always_false();

        let may_execute_zero =
            if guarantees_one {
                false
            } else {
                bound.may_execute_zero_times()
            };

        let statically_terminating =
            match bound {
                IterationBound::Exact(_)
                | IterationBound::Bounded { .. } => true,

                IterationBound::DynamicFinite
                | IterationBound::Unbounded
                | IterationBound::Unknown => false,
            };

        Self {
            min_iterations: minimum,
            max_iterations: maximum,
            statically_terminating,
            may_execute_zero,
            guarantees_one,
            runtime_termination: condition.is_runtime_dependent(),
        }
    }

    /// Returns the minimum iteration count if known.
    #[must_use]
    pub const fn min_iterations(self) -> Option<usize> {
        self.min_iterations
    }

    /// Returns the maximum iteration count if known.
    #[must_use]
    pub const fn max_iterations(self) -> Option<usize> {
        self.max_iterations
    }

    /// Returns true when static termination is proven.
    #[must_use]
    pub const fn statically_terminating(self) -> bool {
        self.statically_terminating
    }

    /// Returns true when zero iterations may occur.
    #[must_use]
    pub const fn may_execute_zero(self) -> bool {
        self.may_execute_zero
    }

    /// Returns true when at least one iteration is guaranteed.
    #[must_use]
    pub const fn guarantees_one(self) -> bool {
        self.guarantees_one
    }

    /// Returns true when runtime state participates in termination.
    #[must_use]
    pub const fn runtime_termination(self) -> bool {
        self.runtime_termination
    }
}

// ============================================================================
// Loop structure
// ============================================================================

/// Immutable optimizer-owned description of a quantum-program loop.
///
/// The structure references existing optimizer blocks instead of copying
/// operations.
///
/// A loop can therefore represent:
///
/// ```text
/// preheader
///    │
///    ▼
/// condition ──────── false ──────► exit
///    │
///   true
///    │
///    ▼
///  body
///    │
///    ▼
///  latch
///    │
///    └────────────────────────────► condition
/// ```
///
/// Not every source construct has all five components. Optional block IDs
/// allow this abstraction to represent different frontend/IR lowering
/// strategies without requiring this module to know the source language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopStructure {
    id: LoopId,

    kind: LoopKind,

    /// Optional block executed immediately before the first condition test.
    preheader: Option<BlockId>,

    /// Optional block containing/evaluating the loop condition.
    condition: Option<BlockId>,

    /// Main loop body.
    body: BlockId,

    /// Optional block executed at the end of an iteration before control
    /// returns to the condition.
    latch: Option<BlockId>,

    /// Optional loop exit block.
    exit: Option<BlockId>,

    /// Optional enclosing loop.
    parent: Option<LoopId>,

    /// Loop depth, where root loops have depth zero.
    depth: usize,

    /// Iteration cardinality classification.
    bound: IterationBound,

    /// Condition classification.
    condition_kind: LoopConditionKind,

    /// Loop-carried state classification.
    carried_state: LoopCarriedState,

    /// Whether the body itself has been validated as a suitable optimizer
    /// block.
    body_validated: bool,

    /// Whether the loop's structural edges are known to be valid.
    structure_validated: bool,

    /// Optimizer capabilities.
    capabilities: LoopCapabilities,

    /// Compact execution summary.
    summary: LoopSummary,
}

impl LoopStructure {
    /// Creates a loop around an existing optimizer block.
    ///
    /// The constructor is conservative:
    ///
    /// - the body is validated structurally;
    /// - the iteration count is unknown;
    /// - the condition is unknown;
    /// - carried state is unknown;
    /// - no unrolling capability is granted automatically.
    ///
    /// This makes the constructor safe for use by future control-flow
    /// discovery code.
    pub fn new(
        id: LoopId,
        body: BlockDescriptor,
    ) -> Result<Self, LoopError> {
        validate_body_block(&body)?;

        let bound = IterationBound::Unknown;
        let condition_kind = LoopConditionKind::Unknown;

        Ok(Self {
            id,
            kind: LoopKind::Generic,
            preheader: None,
            condition: None,
            body: body.id(),
            latch: None,
            exit: None,
            parent: None,
            depth: 0,
            bound,
            condition_kind,
            carried_state: LoopCarriedState::Unknown,
            body_validated: true,
            structure_validated: true,
            capabilities: LoopCapabilities::conservative(),
            summary: LoopSummary::from_bound(
                bound,
                LoopKind::Generic,
                condition_kind,
            ),
        })
    }

    /// Returns the loop identifier.
    #[must_use]
    pub const fn id(&self) -> LoopId {
        self.id
    }

    /// Returns the loop kind.
    #[must_use]
    pub const fn kind(&self) -> LoopKind {
        self.kind
    }

    /// Returns the body block identifier.
    #[must_use]
    pub const fn body(&self) -> BlockId {
        self.body
    }

    /// Returns the optional preheader block.
    #[must_use]
    pub const fn preheader(&self) -> Option<BlockId> {
        self.preheader
    }

    /// Returns the optional condition block.
    #[must_use]
    pub const fn condition(&self) -> Option<BlockId> {
        self.condition
    }

    /// Returns the optional latch block.
    #[must_use]
    pub const fn latch(&self) -> Option<BlockId> {
        self.latch
    }

    /// Returns the optional exit block.
    #[must_use]
    pub const fn exit(&self) -> Option<BlockId> {
        self.exit
    }

    /// Returns the optional enclosing loop.
    #[must_use]
    pub const fn parent(&self) -> Option<LoopId> {
        self.parent
    }

    /// Returns the nesting depth.
    #[must_use]
    pub const fn depth(&self) -> usize {
        self.depth
    }

    /// Returns the iteration bound.
    #[must_use]
    pub const fn bound(&self) -> IterationBound {
        self.bound
    }

    /// Returns the condition classification.
    #[must_use]
    pub const fn condition_kind(&self) -> LoopConditionKind {
        self.condition_kind
    }

    /// Returns the loop-carried state classification.
    #[must_use]
    pub const fn carried_state(&self) -> LoopCarriedState {
        self.carried_state
    }

    /// Returns the optimization capabilities.
    #[must_use]
    pub const fn capabilities(&self) -> LoopCapabilities {
        self.capabilities
    }

    /// Returns the compact execution summary.
    #[must_use]
    pub const fn summary(&self) -> LoopSummary {
        self.summary
    }

    /// Returns true when the body is structurally validated.
    #[must_use]
    pub const fn body_validated(&self) -> bool {
        self.body_validated
    }

    /// Returns true when the complete loop structure has been validated.
    #[must_use]
    pub const fn structure_validated(&self) -> bool {
        self.structure_validated
    }

    /// Returns true when this loop is nested inside another loop.
    #[must_use]
    pub const fn is_nested(&self) -> bool {
        self.parent.is_some()
    }

    /// Returns true when this is a root loop.
    #[must_use]
    pub const fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    /// Returns true when the body may execute zero times.
    #[must_use]
    pub const fn may_execute_zero_times(&self) -> bool {
        self.summary.may_execute_zero()
    }

    /// Returns true when at least one iteration is guaranteed.
    #[must_use]
    pub const fn guarantees_one_iteration(&self) -> bool {
        self.summary.guarantees_one()
    }

    /// Returns true when exact iteration cardinality is known.
    #[must_use]
    pub const fn has_exact_iteration_count(&self) -> bool {
        self.bound.is_exact()
    }

    /// Returns the exact iteration count if known.
    #[must_use]
    pub const fn exact_iteration_count(&self) -> Option<usize> {
        self.bound.exact_count()
    }

    /// Returns true when the loop has a statically known finite maximum.
    #[must_use]
    pub const fn has_finite_maximum(&self) -> bool {
        self.bound.has_finite_maximum()
    }

    /// Returns the statically known maximum iteration count.
    #[must_use]
    pub const fn maximum_iterations(&self) -> Option<usize> {
        self.bound.maximum()
    }

    /// Returns the statically known minimum iteration count.
    #[must_use]
    pub const fn minimum_iterations(&self) -> Option<usize> {
        self.bound.minimum()
    }

    /// Returns true when the condition depends on runtime state.
    #[must_use]
    pub const fn is_runtime_dependent(&self) -> bool {
        self.condition_kind.is_runtime_dependent()
    }

    /// Returns true when measurement results influence continuation.
    #[must_use]
    pub const fn is_measurement_dependent(&self) -> bool {
        self.condition_kind.is_measurement_dependent()
            || self.carried_state.carries_measurement_dependency()
    }

    /// Returns true when quantum state crosses iteration boundaries.
    #[must_use]
    pub const fn carries_quantum_state(&self) -> bool {
        self.carried_state.carries_quantum_state()
    }

    /// Returns true when static full unrolling is a structural candidate.
    ///
    /// This method intentionally remains conservative. Resource limits,
    /// cost-model decisions, semantic dependencies, and verification must
    /// still be performed by the transformation pass.
    #[must_use]
    pub const fn is_exact_unroll_candidate(&self) -> bool {
        self.capabilities.exact_unroll_candidate()
    }

    /// Returns true when body-local optimization can be applied without
    /// requiring loop unrolling.
    #[must_use]
    pub const fn permits_body_local_optimization(&self) -> bool {
        self.capabilities.body_local_optimization()
    }

    /// Returns true when summary/fixed-point optimization is available.
    #[must_use]
    pub const fn permits_summary_optimization(&self) -> bool {
        self.capabilities.summary_optimization()
            && self.capabilities.fixed_point_optimization()
    }

    /// Returns a compact semantic classification useful to planners.
    #[must_use]
    pub const fn optimization_class(&self) -> LoopOptimizationClass {
        if self.condition_kind.is_always_false() {
            return LoopOptimizationClass::ZeroIteration;
        }

        if let Some(count) = self.bound.exact_count() {
            if count == 0 {
                return LoopOptimizationClass::ZeroIteration;
            }

            if count == 1 {
                return LoopOptimizationClass::SingleIteration;
            }

            if self.is_measurement_dependent()
                || self.carries_quantum_state()
            {
                return LoopOptimizationClass::StaticCountWithState;
            }

            return LoopOptimizationClass::ExactCount;
        }

        match self.bound {
            IterationBound::Bounded { .. } => {
                if self.is_measurement_dependent()
                    || self.carries_quantum_state()
                {
                    LoopOptimizationClass::BoundedRuntime
                } else {
                    LoopOptimizationClass::Bounded
                }
            }

            IterationBound::DynamicFinite => {
                LoopOptimizationClass::DynamicFinite
            }

            IterationBound::Unbounded => {
                LoopOptimizationClass::PotentiallyUnbounded
            }

            IterationBound::Unknown => {
                LoopOptimizationClass::Unknown
            }

            IterationBound::Exact(_) => {
                // Already handled above.
                LoopOptimizationClass::ExactCount
            }
        }
    }
}

// ============================================================================
// Loop optimization classification
// ============================================================================

/// High-level optimization classification for a loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoopOptimizationClass {
    /// Loop provably executes zero times.
    ZeroIteration,

    /// Loop provably executes exactly once.
    SingleIteration,

    /// Loop has a statically known exact count greater than one.
    ExactCount,

    /// Exact count is known but quantum/classical state crosses iterations.
    StaticCountWithState,

    /// Loop has a finite statically known range but runtime cardinality.
    Bounded,

    /// Bounded loop with runtime state dependencies.
    BoundedRuntime,

    /// Finite termination is expected but no static maximum is available.
    DynamicFinite,

    /// No finite upper bound is available.
    PotentiallyUnbounded,

    /// Insufficient structural information.
    Unknown,
}

impl LoopOptimizationClass {
    /// Returns true when full loop removal is structurally possible.
    ///
    /// The caller must still verify that the condition is actually always
    /// false and that removing associated control-flow metadata is legal.
    #[must_use]
    pub const fn is_zero_iteration(self) -> bool {
        matches!(self, Self::ZeroIteration)
    }

    /// Returns true when replacing the loop by one body execution is
    /// structurally plausible.
    #[must_use]
    pub const fn is_single_iteration(self) -> bool {
        matches!(self, Self::SingleIteration)
    }

    /// Returns true when the loop can potentially be unrolled using a static
    /// cardinality.
    #[must_use]
    pub const fn has_static_cardinality(self) -> bool {
        matches!(
            self,
            Self::SingleIteration
                | Self::ExactCount
                | Self::StaticCountWithState
        )
    }

    /// Returns true when unbounded expansion should be avoided.
    #[must_use]
    pub const fn requires_summary_strategy(self) -> bool {
        matches!(
            self,
            Self::DynamicFinite
                | Self::PotentiallyUnbounded
                | Self::Unknown
        )
    }
}

// ============================================================================
// Builder
// ============================================================================

/// Builder for `LoopStructure`.
///
/// The builder exists so future modules can construct a complete loop
/// description without mutating a finalized loop object.
///
/// This prevents partially initialized loop structures from escaping.
#[derive(Debug, Clone)]
pub struct LoopBuilder {
    id: LoopId,
    kind: LoopKind,
    preheader: Option<BlockId>,
    condition: Option<BlockId>,
    body: Option<BlockId>,
    latch: Option<BlockId>,
    exit: Option<BlockId>,
    parent: Option<LoopId>,
    depth: usize,
    bound: IterationBound,
    condition_kind: LoopConditionKind,
    carried_state: LoopCarriedState,
    body_validated: bool,
    structure_validated: bool,
}

impl LoopBuilder {
    /// Creates a builder with conservative defaults.
    #[must_use]
    pub const fn new(id: LoopId) -> Self {
        Self {
            id,
            kind: LoopKind::Generic,
            preheader: None,
            condition: None,
            body: None,
            latch: None,
            exit: None,
            parent: None,
            depth: 0,
            bound: IterationBound::Unknown,
            condition_kind: LoopConditionKind::Unknown,
            carried_state: LoopCarriedState::Unknown,
            body_validated: false,
            structure_validated: false,
        }
    }

    /// Sets the loop kind.
    #[must_use]
    pub const fn with_kind(mut self, kind: LoopKind) -> Self {
        self.kind = kind;
        self
    }

    /// Sets the loop body.
    #[must_use]
    pub const fn with_body(mut self, body: BlockId) -> Self {
        self.body = Some(body);
        self
    }

    /// Sets the condition block.
    #[must_use]
    pub const fn with_condition(mut self, condition: BlockId) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Sets the preheader block.
    #[must_use]
    pub const fn with_preheader(mut self, preheader: BlockId) -> Self {
        self.preheader = Some(preheader);
        self
    }

    /// Sets the latch block.
    #[must_use]
    pub const fn with_latch(mut self, latch: BlockId) -> Self {
        self.latch = Some(latch);
        self
    }

    /// Sets the exit block.
    #[must_use]
    pub const fn with_exit(mut self, exit: BlockId) -> Self {
        self.exit = Some(exit);
        self
    }

    /// Sets the enclosing loop.
    #[must_use]
    pub const fn with_parent(
        mut self,
        parent: LoopId,
        depth: usize,
    ) -> Self {
        self.parent = Some(parent);
        self.depth = depth;
        self
    }

    /// Sets the loop iteration bound.
    #[must_use]
    pub const fn with_bound(
        mut self,
        bound: IterationBound,
    ) -> Self {
        self.bound = bound;
        self
    }

    /// Sets the loop condition classification.
    #[must_use]
    pub const fn with_condition_kind(
        mut self,
        condition_kind: LoopConditionKind,
    ) -> Self {
        self.condition_kind = condition_kind;
        self
    }

    /// Sets the loop-carried state classification.
    #[must_use]
    pub const fn with_carried_state(
        mut self,
        carried_state: LoopCarriedState,
    ) -> Self {
        self.carried_state = carried_state;
        self
    }

    /// Marks the body as validated.
    #[must_use]
    pub const fn body_validated(mut self, validated: bool) -> Self {
        self.body_validated = validated;
        self
    }

    /// Marks the structure as validated.
    #[must_use]
    pub const fn structure_validated(
        mut self,
        validated: bool,
    ) -> Self {
        self.structure_validated = validated;
        self
    }

    /// Finalizes the loop after structural validation.
    pub fn build(self) -> Result<LoopStructure, LoopError> {
        let body = self.body.ok_or(LoopError::MissingBody {
            loop_id: self.id,
        })?;

        if !self.body_validated {
            return Err(LoopError::BodyNotValidated {
                loop_id: self.id,
                body,
            });
        }

        validate_loop_edges(
            self.id,
            self.preheader,
            self.condition,
            body,
            self.latch,
            self.exit,
        )?;

        validate_iteration_bound(self.bound)?;

        let summary = LoopSummary::from_bound(
            self.bound,
            self.kind,
            self.condition_kind,
        );

        let capabilities = derive_capabilities(
            self.kind,
            self.bound,
            self.condition_kind,
            self.carried_state,
            self.body_validated,
            self.structure_validated,
        );

        Ok(LoopStructure {
            id: self.id,
            kind: self.kind,
            preheader: self.preheader,
            condition: self.condition,
            body,
            latch: self.latch,
            exit: self.exit,
            parent: self.parent,
            depth: self.depth,
            bound: self.bound,
            condition_kind: self.condition_kind,
            carried_state: self.carried_state,
            body_validated: self.body_validated,
            structure_validated: self.structure_validated,
            capabilities,
            summary,
        })
    }
}

// ============================================================================
// Loop collection
// ============================================================================

/// Invocation-local collection of loop structures.
///
/// The collection owns metadata only. It does not own quantum operations.
///
/// This allows:
///
/// - deterministic iteration;
/// - nested-loop lookup;
/// - parent/child relationships;
/// - duplicate-ID detection;
/// - bounded metadata memory.
///
/// The collection intentionally uses a `Vec` instead of a hash map so ordering
/// remains deterministic and no hashing policy becomes part of the semantic
/// contract.
#[derive(Debug, Clone, Default)]
pub struct LoopForest {
    loops: Vec<LoopStructure>,
}

impl LoopForest {
    /// Creates an empty loop forest.
    #[must_use]
    pub const fn new() -> Self {
        Self { loops: Vec::new() }
    }

    /// Returns the number of loops.
    #[must_use]
    pub fn len(&self) -> usize {
        self.loops.len()
    }

    /// Returns true if there are no loops.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.loops.is_empty()
    }

    /// Adds a loop.
    ///
    /// Loop IDs must be unique within this invocation.
    pub fn insert(
        &mut self,
        loop_structure: LoopStructure,
    ) -> Result<(), LoopError> {
        if self
            .loops
            .iter()
            .any(|existing| existing.id() == loop_structure.id())
        {
            return Err(LoopError::DuplicateLoopId {
                loop_id: loop_structure.id(),
            });
        }

        if let Some(parent) = loop_structure.parent() {
            if parent == loop_structure.id() {
                return Err(LoopError::SelfParent {
                    loop_id: loop_structure.id(),
                });
            }

            let parent_exists =
                self.loops.iter().any(|existing| {
                    existing.id() == parent
                });

            if !parent_exists {
                return Err(LoopError::MissingParent {
                    loop_id: loop_structure.id(),
                    parent,
                });
            }
        }

        self.loops.push(loop_structure);
        Ok(())
    }

    /// Returns a loop by ID.
    #[must_use]
    pub fn get(&self, id: LoopId) -> Option<&LoopStructure> {
        self.loops.iter().find(|item| item.id() == id)
    }

    /// Returns an iterator over loops in deterministic insertion order.
    pub fn iter(&self) -> impl Iterator<Item = &LoopStructure> {
        self.loops.iter()
    }

    /// Returns an iterator over root loops.
    pub fn roots(&self) -> impl Iterator<Item = &LoopStructure> {
        self.loops.iter().filter(|item| item.is_root())
    }

    /// Returns child loops of a parent.
    pub fn children(
        &self,
        parent: LoopId,
    ) -> impl Iterator<Item = &LoopStructure> {
        self.loops
            .iter()
            .filter(move |item| item.parent() == Some(parent))
    }

    /// Returns loops whose body is the specified block.
    pub fn loops_for_body(
        &self,
        body: BlockId,
    ) -> impl Iterator<Item = &LoopStructure> {
        self.loops
            .iter()
            .filter(move |item| item.body() == body)
    }

    /// Returns true if any loop has the supplied ID.
    #[must_use]
    pub fn contains(&self, id: LoopId) -> bool {
        self.loops.iter().any(|item| item.id() == id)
    }

    /// Returns the immutable slice of loops.
    #[must_use]
    pub fn as_slice(&self) -> &[LoopStructure] {
        &self.loops
    }
}

impl<'a> IntoIterator for &'a LoopForest {
    type Item = &'a LoopStructure;
    type IntoIter = std::slice::Iter<'a, LoopStructure>;

    fn into_iter(self) -> Self::IntoIter {
        self.loops.iter()
    }
}

// ============================================================================
// Analysis helpers
// ============================================================================

/// Determines whether a loop may be safely optimized by analyzing its body
/// once rather than expanding every iteration.
///
/// This is intentionally structural and conservative.
///
/// It does not inspect individual gates.
#[must_use]
pub fn permits_body_only_optimization(
    loop_structure: &LoopStructure,
) -> bool {
    loop_structure.body_validated()
        && loop_structure.structure_validated()
        && loop_structure
            .capabilities()
            .body_local_optimization()
}

/// Determines whether a loop should be treated as a summary rather than
/// materialized into repeated body copies.
///
/// This is the preferred strategy for very large or unbounded loops.
#[must_use]
pub fn should_prefer_summary(
    loop_structure: &LoopStructure,
) -> bool {
    loop_structure
        .optimization_class()
        .requires_summary_strategy()
}

/// Determines whether a loop's iteration count can be multiplied by a
/// per-iteration metric without overflowing `usize`.
///
/// This helper does not perform the multiplication because the caller may have
/// a different integer representation for metrics.
#[must_use]
pub fn can_multiply_iterations(
    loop_structure: &LoopStructure,
    per_iteration: usize,
) -> bool {
    match loop_structure.bound().maximum() {
        Some(maximum) => maximum.checked_mul(per_iteration).is_some(),

        None => {
            // A missing maximum means that expansion is not statically
            // bounded. It is therefore never safe to claim that a concrete
            // expansion count fits based only on this metadata.
            false
        }
    }
}

/// Checked multiplication of a finite iteration count.
///
/// This helper exists so downstream passes do not accidentally use wrapping
/// arithmetic when estimating expansion costs.
pub fn checked_iteration_product(
    iterations: usize,
    per_iteration: usize,
) -> Result<usize, LoopError> {
    iterations
        .checked_mul(per_iteration)
        .ok_or(LoopError::ArithmeticOverflow {
            operation: "iteration_count * per_iteration_cost",
        })
}

/// Checked addition of loop-level costs.
///
/// This is useful for summary analyses that aggregate preheader/body/latch/exit
/// operation counts without risking integer wraparound.
pub fn checked_add_cost(
    left: usize,
    right: usize,
) -> Result<usize, LoopError> {
    left.checked_add(right).ok_or(LoopError::ArithmeticOverflow {
        operation: "loop cost accumulation",
    })
}

// ============================================================================
// Validation
// ============================================================================

fn validate_body_block(
    body: &BlockDescriptor,
) -> Result<(), LoopError> {
    if body.kind() == BlockKind::Unknown {
        return Err(LoopError::InvalidBodyKind {
            body: body.id(),
        });
    }

    if body.len() == 0
        && body.kind() != BlockKind::ControlFlowBody
        && body.kind() != BlockKind::LoopBody
    {
        // Empty loops are legal in many languages and IRs. We therefore do
        // not reject them. The branch exists intentionally as documentation
        // of that decision.
    }

    Ok(())
}

fn validate_loop_edges(
    loop_id: LoopId,
    preheader: Option<BlockId>,
    condition: Option<BlockId>,
    body: BlockId,
    latch: Option<BlockId>,
    exit: Option<BlockId>,
) -> Result<(), LoopError> {
    let mut ids = [None; 5];

    ids[0] = preheader;
    ids[1] = condition;
    ids[2] = Some(body);
    ids[3] = latch;
    ids[4] = exit;

    for first_index in 0..ids.len() {
        let Some(first) = ids[first_index] else {
            continue;
        };

        for second_index in (first_index + 1)..ids.len() {
            let Some(second) = ids[second_index] else {
                continue;
            };

            if first == second {
                return Err(LoopError::DuplicateBlockRole {
                    loop_id,
                    block: first,
                });
            }
        }
    }

    Ok(())
}

fn validate_iteration_bound(
    bound: IterationBound,
) -> Result<(), LoopError> {
    match bound {
        IterationBound::Bounded { min, max } if min > max => {
            Err(LoopError::InvalidIterationRange { min, max })
        }

        IterationBound::Exact(_) => Ok(()),

        IterationBound::Bounded { .. }
        | IterationBound::DynamicFinite
        | IterationBound::Unbounded
        | IterationBound::Unknown => Ok(()),
    }
}

fn derive_capabilities(
    kind: LoopKind,
    bound: IterationBound,
    condition: LoopConditionKind,
    carried_state: LoopCarriedState,
    body_validated: bool,
    structure_validated: bool,
) -> LoopCapabilities {
    if !body_validated || !structure_validated {
        return LoopCapabilities {
            body_analyzable: false,
            exact_unroll_candidate: false,
            partial_unroll_candidate: false,
            body_local_optimization: false,
            invariant_analysis: false,
            cardinality_simplification: false,
            summary_optimization: true,
            fixed_point_optimization: false,
        };
    }

    let state_blocks_unrolling = matches!(
        carried_state,
        LoopCarriedState::Quantum
            | LoopCarriedState::Measurement
            | LoopCarriedState::ClassicalAndQuantum
            | LoopCarriedState::Unknown
    );

    let runtime_condition_blocks_exact_unroll =
        condition.is_runtime_dependent();

    let exact_count = bound.is_exact();

    let exact_unroll_candidate =
        exact_count
            && !state_blocks_unrolling
            && !runtime_condition_blocks_exact_unroll;

    let partial_unroll_candidate =
        bound.has_finite_maximum()
            && !matches!(
                carried_state,
                LoopCarriedState::Unknown
            )
            && !condition.is_always_false();

    let body_local_optimization =
        !condition.is_always_false();

    let invariant_analysis =
        !matches!(
            carried_state,
            LoopCarriedState::Unknown
        );

    let cardinality_simplification =
        bound.has_finite_maximum()
            || condition.is_always_false();

    let summary_optimization =
        matches!(
            bound,
            IterationBound::DynamicFinite
                | IterationBound::Unbounded
                | IterationBound::Unknown
        ) || state_blocks_unrolling;

    let fixed_point_optimization =
        body_local_optimization
            && matches!(
                kind,
                LoopKind::While
                    | LoopKind::DoWhile
                    | LoopKind::For
                    | LoopKind::Generic
                    | LoopKind::Generated
            );

    LoopCapabilities {
        body_analyzable: true,
        exact_unroll_candidate,
        partial_unroll_candidate,
        body_local_optimization,
        invariant_analysis,
        cardinality_simplification,
        summary_optimization,
        fixed_point_optimization,
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by loop-structure construction and analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoopError {
    /// A loop does not have a body.
    MissingBody {
        /// Invalid loop identifier.
        loop_id: LoopId,
    },

    /// The body was not validated before finalization.
    BodyNotValidated {
        /// Loop identifier.
        loop_id: LoopId,

        /// Body block.
        body: BlockId,
    },

    /// The body has a semantic kind that cannot be used as a loop body.
    InvalidBodyKind {
        /// Body block.
        body: BlockId,
    },

    /// Two structural roles refer to the same block.
    DuplicateBlockRole {
        /// Loop identifier.
        loop_id: LoopId,

        /// Duplicated block.
        block: BlockId,
    },

    /// Iteration bounds are invalid.
    InvalidIterationRange {
        /// Minimum.
        min: usize,

        /// Maximum.
        max: usize,
    },

    /// A loop identifier has already been inserted.
    DuplicateLoopId {
        /// Duplicate ID.
        loop_id: LoopId,
    },

    /// A loop cannot be its own parent.
    SelfParent {
        /// Loop identifier.
        loop_id: LoopId,
    },

    /// A referenced parent loop does not exist.
    MissingParent {
        /// Child loop.
        loop_id: LoopId,

        /// Missing parent.
        parent: LoopId,
    },

    /// Checked arithmetic overflow occurred.
    ArithmeticOverflow {
        /// Operation that overflowed.
        operation: &'static str,
    },
}

impl fmt::Display for LoopError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingBody { loop_id } => {
                write!(formatter, "{loop_id} has no body")
            }

            Self::BodyNotValidated { loop_id, body } => {
                write!(
                    formatter,
                    "{loop_id} body {body} was not validated"
                )
            }

            Self::InvalidBodyKind { body } => {
                write!(
                    formatter,
                    "block {body} has an unknown semantic kind and \
                     cannot be used as a validated loop body"
                )
            }

            Self::DuplicateBlockRole {
                loop_id,
                block,
            } => {
                write!(
                    formatter,
                    "{loop_id} uses block {block} for multiple structural roles"
                )
            }

            Self::InvalidIterationRange { min, max } => {
                write!(
                    formatter,
                    "invalid loop iteration range: minimum {min} \
                     exceeds maximum {max}"
                )
            }

            Self::DuplicateLoopId { loop_id } => {
                write!(
                    formatter,
                    "duplicate loop identifier {loop_id}"
                )
            }

            Self::SelfParent { loop_id } => {
                write!(
                    formatter,
                    "{loop_id} cannot be its own parent"
                )
            }

            Self::MissingParent { loop_id, parent } => {
                write!(
                    formatter,
                    "{loop_id} references missing parent {parent}"
                )
            }

            Self::ArithmeticOverflow { operation } => {
                write!(
                    formatter,
                    "arithmetic overflow during {operation}"
                )
            }
        }
    }
}

impl std::error::Error for LoopError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn block(id: usize, kind: BlockKind) -> BlockDescriptor {
        BlockDescriptor::new(
            BlockId::new(id),
            id..id.saturating_add(1),
            kind,
        )
        .expect("test block must be valid")
    }

    #[test]
    fn exact_iteration_bound_is_exact() {
        let bound = IterationBound::exact(42);

        assert!(bound.is_exact());
        assert_eq!(bound.exact_count(), Some(42));
        assert_eq!(bound.minimum(), Some(42));
        assert_eq!(bound.maximum(), Some(42));
        assert!(!bound.may_execute_zero_times());
    }

    #[test]
    fn zero_iteration_bound_is_detected() {
        let bound = IterationBound::exact(0);

        assert!(bound.is_exact());
        assert!(bound.may_execute_zero_times());
        assert_eq!(bound.exact_count(), Some(0));
    }

    #[test]
    fn bounded_range_is_validated() {
        let bound = IterationBound::bounded(2, 10)
            .expect("2..10 is valid");

        assert_eq!(
            bound.minimum(),
            Some(2)
        );
        assert_eq!(
            bound.maximum(),
            Some(10)
        );
        assert!(!bound.may_execute_zero_times());
    }

    #[test]
    fn invalid_bounded_range_is_rejected() {
        assert!(
            IterationBound::bounded(10, 2).is_none()
        );
    }

    #[test]
    fn do_while_guarantees_one_iteration() {
        let summary = LoopSummary::from_bound(
            IterationBound::Unknown,
            LoopKind::DoWhile,
            LoopConditionKind::ClassicalRuntime,
        );

        assert!(summary.guarantees_one());
        assert!(!summary.may_execute_zero());
        assert!(summary.runtime_termination());
    }

    #[test]
    fn always_false_condition_is_zero_iteration() {
        let summary = LoopSummary::from_bound(
            IterationBound::Unknown,
            LoopKind::While,
            LoopConditionKind::AlwaysFalse,
        );

        assert!(summary.may_execute_zero());
        assert!(!summary.guarantees_one());
    }

    #[test]
    fn exact_single_iteration_is_classified() {
        let body = block(0, BlockKind::LoopBody);

        let loop_structure = LoopBuilder::new(LoopId::new(0))
            .with_kind(LoopKind::For)
            .with_body(body.id())
            .with_bound(IterationBound::Exact(1))
            .with_condition_kind(LoopConditionKind::Static)
            .with_carried_state(LoopCarriedState::None)
            .body_validated(true)
            .structure_validated(true)
            .build()
            .expect("loop should build");

        assert_eq!(
            loop_structure.optimization_class(),
            LoopOptimizationClass::SingleIteration
        );
        assert_eq!(
            loop_structure.exact_iteration_count(),
            Some(1)
        );
    }

    #[test]
    fn static_exact_loop_can_be_unroll_candidate() {
        let body = block(0, BlockKind::LoopBody);

        let loop_structure = LoopBuilder::new(LoopId::new(0))
            .with_kind(LoopKind::For)
            .with_body(body.id())
            .with_bound(IterationBound::Exact(8))
            .with_condition_kind(LoopConditionKind::Static)
            .with_carried_state(LoopCarriedState::None)
            .body_validated(true)
            .structure_validated(true)
            .build()
            .expect("loop should build");

        assert!(
            loop_structure.is_exact_unroll_candidate()
        );
    }

    #[test]
    fn measurement_dependent_loop_is_not_exact_unroll_candidate() {
        let body = block(0, BlockKind::LoopBody);

        let loop_structure = LoopBuilder::new(LoopId::new(0))
            .with_kind(LoopKind::For)
            .with_body(body.id())
            .with_bound(IterationBound::Exact(8))
            .with_condition_kind(
                LoopConditionKind::MeasurementDependent,
            )
            .with_carried_state(LoopCarriedState::Measurement)
            .body_validated(true)
            .structure_validated(true)
            .build()
            .expect("loop should build");

        assert!(
            !loop_structure.is_exact_unroll_candidate()
        );
        assert!(
            loop_structure.is_measurement_dependent()
        );
    }

    #[test]
    fn quantum_carried_state_blocks_exact_unrolling() {
        let body = block(0, BlockKind::LoopBody);

        let loop_structure = LoopBuilder::new(LoopId::new(0))
            .with_kind(LoopKind::For)
            .with_body(body.id())
            .with_bound(IterationBound::Exact(4))
            .with_condition_kind(LoopConditionKind::Static)
            .with_carried_state(LoopCarriedState::Quantum)
            .body_validated(true)
            .structure_validated(true)
            .build()
            .expect("loop should build");

        assert!(
            !loop_structure.is_exact_unroll_candidate()
        );
        assert!(loop_structure.carries_quantum_state());
    }

    #[test]
    fn unknown_loop_prefers_summary_strategy() {
        let body = block(0, BlockKind::LoopBody);

        let loop_structure = LoopBuilder::new(LoopId::new(0))
            .with_kind(LoopKind::While)
            .with_body(body.id())
            .with_bound(IterationBound::Unknown)
            .with_condition_kind(LoopConditionKind::Unknown)
            .with_carried_state(LoopCarriedState::Unknown)
            .body_validated(true)
            .structure_validated(true)
            .build()
            .expect("loop should build");

        assert!(should_prefer_summary(&loop_structure));
        assert!(
            permits_body_only_optimization(&loop_structure)
        );
    }

    #[test]
    fn large_iteration_product_is_checked() {
        assert_eq!(
            checked_iteration_product(10, 20)
                .expect("10 * 20 must fit"),
            200
        );
    }

    #[test]
    fn iteration_product_overflow_is_rejected() {
        let result =
            checked_iteration_product(usize::MAX, 2);

        assert!(
            matches!(
                result,
                Err(LoopError::ArithmeticOverflow { .. })
            )
        );
    }

    #[test]
    fn loop_forest_rejects_duplicate_ids() {
        let body = block(0, BlockKind::LoopBody);

        let first = LoopBuilder::new(LoopId::new(0))
            .with_body(body.id())
            .body_validated(true)
            .structure_validated(true)
            .build()
            .expect("loop should build");

        let second = LoopBuilder::new(LoopId::new(0))
            .with_body(body.id())
            .body_validated(true)
            .structure_validated(true)
            .build()
            .expect("loop should build");

        let mut forest = LoopForest::new();

        forest
            .insert(first)
            .expect("first insertion must succeed");

        assert!(
            matches!(
                forest.insert(second),
                Err(LoopError::DuplicateLoopId { .. })
            )
        );
    }

    #[test]
    fn loop_forest_supports_parent_child_relationships() {
        let root_body = block(0, BlockKind::LoopBody);
        let child_body = block(1, BlockKind::LoopBody);

        let root = LoopBuilder::new(LoopId::new(0))
            .with_body(root_body.id())
            .body_validated(true)
            .structure_validated(true)
            .build()
            .expect("root loop should build");

        let child = LoopBuilder::new(LoopId::new(1))
            .with_body(child_body.id())
            .with_parent(LoopId::new(0), 1)
            .body_validated(true)
            .structure_validated(true)
            .build()
            .expect("child loop should build");

        let mut forest = LoopForest::new();

        forest.insert(root).expect("root insertion");
        forest.insert(child).expect("child insertion");

        assert_eq!(forest.len(), 2);
        assert_eq!(
            forest.children(LoopId::new(0)).count(),
            1
        );
        assert_eq!(
            forest.roots().count(),
            1
        );
    }

    #[test]
    fn body_lookup_is_deterministic() {
        let body = block(7, BlockKind::LoopBody);

        let first = LoopBuilder::new(LoopId::new(0))
            .with_body(body.id())
            .body_validated(true)
            .structure_validated(true)
            .build()
            .expect("first loop");

        let second = LoopBuilder::new(LoopId::new(1))
            .with_body(body.id())
            .body_validated(true)
            .structure_validated(true)
            .build()
            .expect("second loop");

        let mut forest = LoopForest::new();

        forest.insert(first).expect("first insertion");
        forest.insert(second).expect("second insertion");

        let ids: Vec<LoopId> =
            forest
                .loops_for_body(body.id())
                .map(LoopStructure::id)
                .collect();

        assert_eq!(
            ids,
            vec![LoopId::new(0), LoopId::new(1)]
        );
    }

    #[test]
    fn checked_cost_addition_detects_overflow() {
        assert_eq!(
            checked_add_cost(4, 5)
                .expect("4 + 5 must fit"),
            9
        );

        assert!(
            matches!(
                checked_add_cost(usize::MAX, 1),
                Err(LoopError::ArithmeticOverflow { .. })
            )
        );
    }
}