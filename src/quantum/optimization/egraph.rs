//! Zamani Quantum Optimization — Equality Saturation / E-Graph Engine
//!
//! Production-grade, backend-independent e-graph infrastructure for quantum
//! circuit optimization.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                              │
//!                              ▼
//!                    optimization::operation
//!                              │
//!                              ▼
//!                     optimization::egraph
//!                              │
//!              ┌───────────────┼────────────────┐
//!              ▼               ▼                ▼
//!          pattern.rs      rewrite.rs       rules.rs
//!              │               │                │
//!              └───────────────┼────────────────┘
//!                              ▼
//!                     equality saturation
//!                              │
//!                              ▼
//!                         extraction
//!                              │
//!                              ▼
//!                         quantum::ir
//! ```
//!
//! This module owns the *e-graph data structure and equality-saturation
//! mechanics*. It does not own:
//!
//! - the canonical Quantum IR;
//! - quantum gate definitions;
//! - frontend parsing;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - QPU execution;
//! - optimization profiles;
//! - compiler-wide provenance;
//! - semantic equivalence checking;
//! - quantum-specific rewrite rules.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Why this module is generic
//!
//! The canonical Quantum IR already owns quantum operation semantics. Creating
//! another `QuantumGate`, `QuantumOperation`, or circuit representation here
//! would violate the repository's architecture.
//!
//! Instead, the e-graph stores an `ENode<O>` where `O` is an immutable,
//! canonical, hashable, totally ordered operation key supplied by the
//! optimization layer.
//!
//! A future `optimization::operation` adapter can therefore represent:
//!
//! ```text
//! quantum::ir::Gate
//!        │
//!        ▼
//! operation::OperationKey
//!        │
//!        ▼
//! egraph::ENode<OperationKey>
//! ```
//!
//! The e-graph never needs to know how the quantum operation is represented.
//!
//! # Equality saturation
//!
//! The implementation follows the standard delayed-rebuild architecture:
//!
//! 1. add e-nodes;
//! 2. union e-classes;
//! 3. defer congruence restoration;
//! 4. rebuild;
//! 5. search/apply rewrites;
//! 6. rebuild;
//! 7. stop at saturation or a configured resource boundary;
//! 8. extract the best representative.
//!
//! Delayed rebuilding is important because maintaining congruence after every
//! union can impose substantial overhead. This is the same core idea used by
//! modern e-graph implementations. See the external references in the project
//! documentation for the theoretical background.
//!
//! # Production requirements
//!
//! This implementation provides:
//!
//! - canonical e-nodes;
//! - union-find e-classes;
//! - hash-consing;
//! - congruence rebuilding;
//! - deterministic extraction;
//! - explicit resource limits;
//! - iteration limits;
//! - node limits;
//! - e-class limits;
//! - rewrite-application limits;
//! - match limits;
//! - extraction limits;
//! - optional wall-clock limits;
//! - saturation detection;
//! - statistics;
//! - root tracking;
//! - typed e-class analysis;
//! - analysis merging;
//! - analysis rebuilding;
//! - cost-based extraction;
//! - overflow-checked counters;
//! - non-recursive union-find operations;
//! - safe Rust only;
//! - Rust 1.97 / 1.97.1 compatibility;
//! - no external dependency.
//!
//! # Scaling
//!
//! There is no artificial small-circuit ceiling in the algorithm.
//!
//! The practical maximum is determined by:
//!
//! - addressable memory;
//! - addressable `usize` capacity;
//! - the configured `EGraphLimits`;
//! - rewrite workload;
//! - available CPU;
//! - extraction complexity;
//! - the number of equivalent forms generated.
//!
//! Production callers should configure finite limits. "Unlimited" operation is
//! represented by `None`, but should only be used when an enclosing compiler
//! resource policy provides the actual bound.
//!
//! # Safety
//!
//! This module contains no `unsafe` code.
//!
//! No raw pointers are used.
//! No global mutable state is used.
//! No thread-local global state is used.
//! No FFI is performed.
//!
//! # Determinism
//!
//! The core data structure does not depend on hash-map iteration order for
//! semantic decisions.
//!
//! Extraction scans e-classes by stable ID and sorts candidates by the
//! user-supplied deterministic cost/tie-break policy.
//!
//! Callers that require byte-for-byte reproducibility should provide an
//! operation key with a stable `Ord` implementation and a deterministic cost
//! function.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::time::{Duration, Instant};

// =============================================================================
// Public identifiers
// =============================================================================

/// Stable identifier for an e-class.
///
/// IDs are indices into the e-graph's internal storage. They are opaque to
/// callers and remain valid after unions, although `find()` must be used to
/// obtain the current canonical representative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EClassId(usize);

impl EClassId {
    /// Creates an ID from an internal index.
    ///
    /// This is public for adapters that persist IDs inside one e-graph
    /// invocation. IDs must never be reused across different e-graphs.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the underlying index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for EClassId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "e{}", self.0)
    }
}

/// A variable identifier used by pattern/rewrite infrastructure.
///
/// Pattern matching itself belongs to `pattern.rs` / `matcher.rs`, but the
/// stable identifier lives here so all e-graph consumers can share it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EGraphVar(pub u32);

impl EGraphVar {
    /// Creates a variable identifier.
    #[must_use]
    pub const fn new(index: u32) -> Self {
        Self(index)
    }

    /// Returns the variable index.
    #[must_use]
    pub const fn index(self) -> u32 {
        self.0
    }
}

// =============================================================================
// E-node
// =============================================================================

/// Canonical e-node.
///
/// `O` is the immutable operation/key type supplied by the optimization layer.
///
/// Children are e-class IDs, not concrete syntax-tree nodes. This is the key
/// property that lets the e-graph compactly represent many equivalent
/// expressions.
///
/// The child list is ordered. If a quantum operation has commutative operands,
/// commutativity must be represented by the corresponding canonical operation
/// adapter or rewrite rule; this core must never silently reorder quantum
/// operands.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ENode<O> {
    operation: O,
    children: Vec<EClassId>,
}

impl<O> ENode<O> {
    /// Creates an e-node.
    ///
    /// Children are not automatically canonicalized here because canonical
    /// children depend on the owning e-graph.
    #[must_use]
    pub fn new(operation: O, children: Vec<EClassId>) -> Self {
        Self {
            operation,
            children,
        }
    }

    /// Returns the operation key.
    #[must_use]
    pub fn operation(&self) -> &O {
        &self.operation
    }

    /// Returns the child e-class IDs as stored.
    #[must_use]
    pub fn children(&self) -> &[EClassId] {
        &self.children
    }

    /// Consumes the node and returns its operation and children.
    #[must_use]
    pub fn into_parts(self) -> (O, Vec<EClassId>) {
        (self.operation, self.children)
    }

    /// Creates a node with a replacement child list.
    #[must_use]
    pub fn with_children(&self, children: Vec<EClassId>) -> Self
    where
        O: Clone,
    {
        Self {
            operation: self.operation.clone(),
            children,
        }
    }
}

// =============================================================================
// E-class
// =============================================================================

/// A single equivalence class.
///
/// The node list contains all canonical e-nodes currently known to represent
/// the class. Nodes are deduplicated during rebuilding.
#[derive(Debug, Clone)]
pub struct EClass<O, A> {
    id: EClassId,
    nodes: Vec<ENode<O>>,
    analysis: A,
    generation: u64,
}

impl<O, A> EClass<O, A> {
    fn new(id: EClassId, nodes: Vec<ENode<O>>, analysis: A) -> Self {
        Self {
            id,
            nodes,
            analysis,
            generation: 0,
        }
    }

    /// Returns this class's stable ID.
    #[must_use]
    pub const fn id(&self) -> EClassId {
        self.id
    }

    /// Returns all nodes currently stored in the class.
    #[must_use]
    pub fn nodes(&self) -> &[ENode<O>] {
        &self.nodes
    }

    /// Returns the analysis data.
    #[must_use]
    pub fn analysis(&self) -> &A {
        &self.analysis
    }

    /// Returns the current class generation.
    ///
    /// Generations are useful to higher-level incremental analyses.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }
}

// =============================================================================
// Analysis
// =============================================================================

/// Analysis attached to every e-class.
///
/// This is deliberately independent of quantum semantics.
///
/// A quantum-specific analysis can later store information such as:
//!
//! - gate count lower bounds;
//! - qubit footprint;
//! - unitary/non-unitary status;
//! - Clifford classification;
//! - T-count lower bound;
//! - T-depth lower bound;
//! - global-phase information;
//! - parameter properties;
//! - hardware-independent resource estimates.
//!
//! The analysis must be safe to merge when e-classes become equivalent.
pub trait EGraphAnalysis<O>: Clone {
    /// Data associated with every e-class.
    type Data: Clone;

    /// Creates analysis data for a newly inserted node.
    fn make(&self, node: &ENode<O>, children: &[&Self::Data]) -> Self::Data;

    /// Merges `from` into `to`.
    ///
    /// Returns `true` when `to` changed.
    fn merge(&self, to: &mut Self::Data, from: &Self::Data) -> bool;

    /// Recomputes data after congruence rebuilding.
    ///
    /// The default implementation leaves the current data unchanged.
    ///
    /// More advanced analyses may override this. The e-graph guarantees that
    /// all child IDs supplied to the analysis are canonical.
    fn rebuild(
        &self,
        _node: &ENode<O>,
        _children: &[&Self::Data],
        _current: &mut Self::Data,
    ) {
    }
}

/// Trivial analysis for callers that do not require e-class metadata.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoAnalysis;

impl<O> EGraphAnalysis<O> for NoAnalysis {
    type Data = ();

    fn make(&self, _node: &ENode<O>, _children: &[&Self::Data]) -> Self::Data {}

    fn merge(&self, _to: &mut Self::Data, _from: &Self::Data) -> bool {
        false
    }
}

// =============================================================================
// Limits
// =============================================================================

/// Resource limits for one e-graph/equality-saturation invocation.
///
/// `None` means that this particular limit is disabled. The enclosing Zamani
/// optimization context should still provide an overall resource policy.
///
/// Every limit is checked before the corresponding allocation or logical work
/// is committed where practical.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EGraphLimits {
    /// Maximum number of e-nodes.
    pub max_nodes: Option<usize>,

    /// Maximum number of e-classes.
    pub max_classes: Option<usize>,

    /// Maximum number of roots.
    pub max_roots: Option<usize>,

    /// Maximum equality-saturation iterations.
    pub max_iterations: Option<u64>,

    /// Maximum rewrite applications.
    pub max_rewrites: Option<u64>,

    /// Maximum rewrite matches inspected.
    pub max_matches: Option<u64>,

    /// Maximum extraction candidates inspected.
    pub max_extraction_candidates: Option<u64>,

    /// Maximum extraction rounds.
    pub max_extraction_rounds: Option<u64>,

    /// Optional wall-clock limit.
    pub max_time: Option<Duration>,
}

impl Default for EGraphLimits {
    fn default() -> Self {
        Self {
            max_nodes: Some(1_000_000),
            max_classes: Some(500_000),
            max_roots: Some(1_024),
            max_iterations: Some(100),
            max_rewrites: Some(10_000_000),
            max_matches: Some(10_000_000),
            max_extraction_candidates: Some(10_000_000),
            max_extraction_rounds: Some(10_000),
            max_time: None,
        }
    }
}

impl EGraphLimits {
    /// Creates an unlimited local limit configuration.
    ///
    /// This should normally only be used under an enclosing optimizer resource
    /// policy.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_nodes: None,
            max_classes: None,
            max_roots: None,
            max_iterations: None,
            max_rewrites: None,
            max_matches: None,
            max_extraction_candidates: None,
            max_extraction_rounds: None,
            max_time: None,
        }
    }

    /// Returns a conservative production configuration.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            max_nodes: Some(1_000_000),
            max_classes: Some(500_000),
            max_roots: Some(1_024),
            max_iterations: Some(100),
            max_rewrites: Some(10_000_000),
            max_matches: Some(10_000_000),
            max_extraction_candidates: Some(10_000_000),
            max_extraction_rounds: Some(10_000),
            max_time: None,
        }
    }

    /// Returns a small deterministic configuration suitable for tests.
    #[must_use]
    pub const fn testing() -> Self {
        Self {
            max_nodes: Some(10_000),
            max_classes: Some(5_000),
            max_roots: Some(64),
            max_iterations: Some(20),
            max_rewrites: Some(100_000),
            max_matches: Some(100_000),
            max_extraction_candidates: Some(100_000),
            max_extraction_rounds: Some(1_000),
            max_time: Some(Duration::from_secs(10)),
        }
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the e-graph engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EGraphError {
    /// An e-class ID does not belong to this e-graph.
    InvalidClassId {
        /// Supplied ID.
        id: EClassId,

        /// Number of allocated classes.
        classes: usize,
    },

    /// An operation would exceed the configured node limit.
    NodeLimitExceeded {
        /// Current/requested count.
        requested: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// An operation would exceed the configured class limit.
    ClassLimitExceeded {
        /// Current/requested count.
        requested: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// Too many roots were registered.
    RootLimitExceeded {
        /// Current/requested count.
        requested: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// The saturation loop exceeded its iteration limit.
    IterationLimitExceeded {
        /// Current iteration.
        iteration: u64,

        /// Maximum permitted.
        maximum: u64,
    },

    /// Rewrite budget was exhausted.
    RewriteLimitExceeded {
        /// Current/requested count.
        requested: u64,

        /// Maximum permitted.
        maximum: u64,
    },

    /// Match budget was exhausted.
    MatchLimitExceeded {
        /// Current/requested count.
        requested: u64,

        /// Maximum permitted.
        maximum: u64,
    },

    /// Extraction candidate budget was exhausted.
    ExtractionLimitExceeded {
        /// Current/requested count.
        requested: u64,

        /// Maximum permitted.
        maximum: u64,
    },

    /// Extraction failed to converge within the configured number of rounds.
    ExtractionDidNotConverge {
        /// Number of rounds attempted.
        rounds: u64,
    },

    /// The configured wall-clock deadline was exceeded.
    TimeLimitExceeded,

    /// An arithmetic operation overflowed.
    ArithmeticOverflow {
        /// Calculation that overflowed.
        operation: &'static str,
    },

    /// The e-graph was used after a required rebuild.
    DirtyRead,

    /// An operation requires at least one root.
    MissingRoot,

    /// Extraction found no finite candidate.
    NoFiniteExtraction,

    /// A cost model rejected an operation.
    CostModelRejected {
        /// Operation description.
        message: String,
    },

    /// The e-graph invariant was violated.
    InvariantViolation {
        /// Human-readable reason.
        message: String,
    },
}

impl fmt::Display for EGraphError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidClassId { id, classes } => {
                write!(
                    formatter,
                    "invalid e-class id {id}; e-graph contains {classes} classes"
                )
            }

            Self::NodeLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "e-graph node limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::ClassLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "e-graph class limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::RootLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "e-graph root limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::IterationLimitExceeded {
                iteration,
                maximum,
            } => {
                write!(
                    formatter,
                    "equality-saturation iteration limit exceeded at {iteration}; maximum {maximum}"
                )
            }

            Self::RewriteLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "e-graph rewrite limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::MatchLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "e-graph match limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::ExtractionLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "e-graph extraction limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::ExtractionDidNotConverge { rounds } => {
                write!(
                    formatter,
                    "e-graph extraction did not converge after {rounds} rounds"
                )
            }

            Self::TimeLimitExceeded => {
                formatter.write_str("e-graph wall-clock limit exceeded")
            }

            Self::ArithmeticOverflow { operation } => {
                write!(
                    formatter,
                    "e-graph arithmetic overflow while calculating {operation}"
                )
            }

            Self::DirtyRead => {
                formatter.write_str(
                    "e-graph query requires a rebuild after mutation",
                )
            }

            Self::MissingRoot => {
                formatter.write_str("e-graph has no registered root")
            }

            Self::NoFiniteExtraction => {
                formatter.write_str(
                    "no finite-cost representative exists for the requested e-class",
                )
            }

            Self::CostModelRejected { message } => {
                write!(formatter, "e-graph cost model rejected candidate: {message}")
            }

            Self::InvariantViolation { message } => {
                write!(formatter, "e-graph invariant violation: {message}")
            }
        }
    }
}

impl std::error::Error for EGraphError {}

/// Result type for e-graph operations.
pub type EGraphResult<T> = Result<T, EGraphError>;

// =============================================================================
// Stop reason
// =============================================================================

/// Reason equality saturation stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EGraphStopReason {
    /// No rewrite changed the e-graph.
    Saturated,

    /// The configured iteration limit was reached.
    IterationLimit,

    /// The node limit prevented further expansion.
    NodeLimit,

    /// The class limit prevented further expansion.
    ClassLimit,

    /// The rewrite budget was exhausted.
    RewriteLimit,

    /// The match budget was exhausted.
    MatchLimit,

    /// The wall-clock budget was exhausted.
    TimeLimit,

    /// The caller stopped the process.
    Cancelled,

    /// The caller stopped because the desired quality target was reached.
    TargetReached,
}

// =============================================================================
// Statistics
// =============================================================================

/// Runtime statistics for one e-graph invocation.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EGraphStatistics {
    /// Number of e-nodes currently allocated.
    pub nodes: usize,

    /// Number of allocated e-classes.
    pub classes: usize,

    /// Number of roots.
    pub roots: usize,

    /// Number of union operations requested.
    pub unions: u64,

    /// Number of successful class merges.
    pub merges: u64,

    /// Number of rebuild operations.
    pub rebuilds: u64,

    /// Number of e-nodes canonicalized during rebuild.
    pub canonicalized_nodes: u64,

    /// Number of congruence collisions detected.
    pub congruence_merges: u64,

    /// Number of equality-saturation iterations.
    pub iterations: u64,

    /// Number of rewrite applications.
    pub rewrites: u64,

    /// Number of rewrite matches inspected.
    pub matches: u64,

    /// Number of extraction candidates inspected.
    pub extraction_candidates: u64,

    /// Number of extraction rounds.
    pub extraction_rounds: u64,

    /// Number of times an analysis changed.
    pub analysis_changes: u64,
}

impl EGraphStatistics {
    fn record_u64(
        target: &mut u64,
        amount: u64,
        operation: &'static str,
    ) -> EGraphResult<()> {
        *target = target
            .checked_add(amount)
            .ok_or(EGraphError::ArithmeticOverflow { operation })?;

        Ok(())
    }
}

// =============================================================================
// Cost model
// =============================================================================

/// Cost assigned to one e-node.
///
/// `O` is the operation/key type.
///
/// Lower cost is better.
///
/// Costs must have a total ordering. For quantum optimization this allows the
/// caller to encode objectives such as:
//!
//! - gate count;
//! - weighted gate count;
//! - two-qubit cost;
//! - T-count;
//! - depth;
//! - lexicographic multi-objective cost.
//!
//! The e-graph does not assume which objective is correct.
pub trait EGraphCost<O> {
    /// Cost type.
    type Cost: Clone + Ord;

    /// Returns the cost of an operation with already-computed child costs.
    fn cost(
        &self,
        operation: &O,
        children: &[&Self::Cost],
    ) -> Result<Self::Cost, EGraphError>;

    /// Returns a deterministic tie-break key.
    ///
    /// The default tie-break is the operation itself when `O: Ord`.
    fn tie_break(&self, _operation: &O) -> Option<u64> {
        None
    }
}

/// A simple unit-cost model.
///
/// Every operation contributes one unit plus its children.
#[derive(Debug, Clone, Copy, Default)]
pub struct UnitCost;

impl<O> EGraphCost<O> for UnitCost {
    type Cost = u64;

    fn cost(
        &self,
        _operation: &O,
        children: &[&Self::Cost],
    ) -> Result<Self::Cost, EGraphError> {
        let mut cost = 1u64;

        for child in children {
            cost = cost
                .checked_add(**child)
                .ok_or(EGraphError::ArithmeticOverflow {
                    operation: "unit extraction cost",
                })?;
        }

        Ok(cost)
    }
}

/// Extracted expression.
///
/// The expression is a concrete tree selected from an e-class.
///
/// This is intentionally separate from the canonical Quantum IR. A later
/// adapter in `rewrite.rs` / `circuit.rs` can lower this result back into
/// `quantum::ir`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extracted<O> {
    /// Selected operation.
    pub operation: O,

    /// Selected child expressions.
    pub children: Vec<Extracted<O>>,
}

impl<O> Extracted<O> {
    /// Creates a leaf expression.
    #[must_use]
    pub fn leaf(operation: O) -> Self {
        Self {
            operation,
            children: Vec::new(),
        }
    }

    /// Creates an expression with children.
    #[must_use]
    pub fn node(operation: O, children: Vec<Self>) -> Self {
        Self {
            operation,
            children,
        }
    }

    /// Returns the total number of nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        let mut total = 1usize;

        for child in &self.children {
            total = total.saturating_add(child.node_count());
        }

        total
    }
}

// =============================================================================
// E-graph
// =============================================================================

/// Production e-graph.
///
/// `O` is the immutable operation key.
///
/// `A` is the e-class analysis implementation.
///
/// The structure uses delayed rebuilding:
//!
//! ```text
//! add/union
//!    │
//!    ▼
//! dirty e-graph
//!    │
//!    ▼
//! rebuild()
//!    │
//!    ▼
//! congruent e-graph
//! ```
///
/// Query APIs that depend on congruence require a clean/rebuilt graph.
#[derive(Debug, Clone)]
pub struct EGraph<O, A = NoAnalysis>
where
    O: Clone + Eq + Hash + Ord,
    A: EGraphAnalysis<O>,
{
    classes: Vec<EClass<O, A::Data>>,

    /// Union-find parent relation.
    parents: Vec<EClassId>,

    /// Union-by-rank metadata.
    ranks: Vec<u32>,

    /// Hash-cons table.
///
/// Keys are canonical nodes whose children already point to representative
/// e-class IDs.
    hashcons: HashMap<ENode<O>, EClassId>,

    /// E-class IDs made dirty by unions/additions.
    dirty: Vec<EClassId>,

    /// Registered roots.
    roots: Vec<EClassId>,

    /// Analysis implementation.
    analysis: A,

    /// Resource policy.
    limits: EGraphLimits,

    /// Runtime statistics.
    statistics: EGraphStatistics,

    /// Whether all congruence invariants are restored.
    clean: bool,

    /// Invocation start time.
    started_at: Instant,
}

impl<O, A> EGraph<O, A>
where
    O: Clone + Eq + Hash + Ord,
    A: EGraphAnalysis<O>,
{
    /// Creates an empty e-graph.
    pub fn new(analysis: A, limits: EGraphLimits) -> Self {
        Self {
            classes: Vec::new(),
            parents: Vec::new(),
            ranks: Vec::new(),
            hashcons: HashMap::new(),
            dirty: Vec::new(),
            roots: Vec::new(),
            analysis,
            limits,
            statistics: EGraphStatistics::default(),
            clean: true,
            started_at: Instant::now(),
        }
    }

    /// Creates an empty production-configured e-graph.
    pub fn production(analysis: A) -> Self {
        Self::new(analysis, EGraphLimits::production())
    }

    /// Creates an empty test-configured e-graph.
    pub fn testing(analysis: A) -> Self {
        Self::new(analysis, EGraphLimits::testing())
    }

    /// Returns the configured limits.
    #[must_use]
    pub fn limits(&self) -> &EGraphLimits {
        &self.limits
    }

    /// Returns the current statistics.
    #[must_use]
    pub fn statistics(&self) -> &EGraphStatistics {
        &self.statistics
    }

    /// Returns whether the graph is clean.
    #[must_use]
    pub const fn is_clean(&self) -> bool {
        self.clean
    }

    /// Returns the number of allocated e-classes.
    #[must_use]
    pub fn class_count(&self) -> usize {
        self.classes.len()
    }

    /// Returns the number of allocated e-nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.hashcons.len()
    }

    /// Returns the number of registered roots.
    #[must_use]
    pub fn root_count(&self) -> usize {
        self.roots.len()
    }

    /// Returns all roots in insertion order.
    #[must_use]
    pub fn roots(&self) -> &[EClassId] {
        &self.roots
    }

    /// Returns the analysis implementation.
    #[must_use]
    pub fn analysis(&self) -> &A {
        &self.analysis
    }

    /// Returns whether the wall-clock limit has expired.
    #[must_use]
    pub fn time_limit_reached(&self) -> bool {
        match self.limits.max_time {
            Some(limit) => self.started_at.elapsed() >= limit,
            None => false,
        }
    }

    /// Checks the wall-clock limit.
    fn check_time(&self) -> EGraphResult<()> {
        if self.time_limit_reached() {
            return Err(EGraphError::TimeLimitExceeded);
        }

        Ok(())
    }

    /// Returns a class after validating the ID.
    fn class(&self, id: EClassId) -> EGraphResult<&EClass<O, A::Data>> {
        self.classes
            .get(id.index())
            .ok_or(EGraphError::InvalidClassId {
                id,
                classes: self.classes.len(),
            })
    }

    /// Returns a mutable class after validating the ID.
    fn class_mut(
        &mut self,
        id: EClassId,
    ) -> EGraphResult<&mut EClass<O, A::Data>> {
        let classes = self.classes.len();

        self.classes
            .get_mut(id.index())
            .ok_or(EGraphError::InvalidClassId { id, classes })
    }

    /// Finds the current canonical representative of an e-class.
    ///
    /// Path compression is iterative and therefore does not recurse even on
    /// very large union-find chains.
    pub fn find(&mut self, id: EClassId) -> EGraphResult<EClassId> {
        if id.index() >= self.parents.len() {
            return Err(EGraphError::InvalidClassId {
                id,
                classes: self.classes.len(),
            });
        }

        let mut root = id;

        while self.parents[root.index()] != root {
            root = self.parents[root.index()];
        }

        let mut current = id;

        while self.parents[current.index()] != current {
            let next = self.parents[current.index()];
            self.parents[current.index()] = root;
            current = next;
        }

        Ok(root)
    }

    /// Finds the representative without modifying path compression.
    ///
    /// This method is read-only and therefore useful to adapters that only
    /// have an immutable graph reference.
    pub fn find_readonly(&self, id: EClassId) -> EGraphResult<EClassId> {
        if id.index() >= self.parents.len() {
            return Err(EGraphError::InvalidClassId {
                id,
                classes: self.classes.len(),
            });
        }

        let mut root = id;

        while self.parents[root.index()] != root {
            root = self.parents[root.index()];
        }

        Ok(root)
    }

    /// Returns an immutable e-class by canonicalizing its ID first.
    pub fn get_class(
        &mut self,
        id: EClassId,
    ) -> EGraphResult<&EClass<O, A::Data>> {
        let root = self.find(id)?;
        self.class(root)
    }

    /// Returns an immutable e-class without modifying the union-find forest.
    pub fn get_class_readonly(
        &self,
        id: EClassId,
    ) -> EGraphResult<&EClass<O, A::Data>> {
        let root = self.find_readonly(id)?;
        self.class(root)
    }

    /// Adds an e-node and returns its e-class.
    ///
    /// Children are canonicalized before hash-consing.
    pub fn add(
        &mut self,
        operation: O,
        children: &[EClassId],
    ) -> EGraphResult<EClassId> {
        self.check_time()?;

        let mut canonical_children = Vec::with_capacity(children.len());

        for child in children {
            canonical_children.push(self.find(*child)?);
        }

        let node = ENode::new(operation, canonical_children);

        if let Some(existing) = self.hashcons.get(&node).copied() {
            return self.find(existing);
        }

        let requested_nodes = self
            .node_count()
            .checked_add(1)
            .ok_or(EGraphError::ArithmeticOverflow {
                operation: "e-node count",
            })?;

        if let Some(maximum) = self.limits.max_nodes {
            if requested_nodes > maximum {
                return Err(EGraphError::NodeLimitExceeded {
                    requested: requested_nodes,
                    maximum,
                });
            }
        }

        let requested_classes = self
            .class_count()
            .checked_add(1)
            .ok_or(EGraphError::ArithmeticOverflow {
                operation: "e-class count",
            })?;

        if let Some(maximum) = self.limits.max_classes {
            if requested_classes > maximum {
                return Err(EGraphError::ClassLimitExceeded {
                    requested: requested_classes,
                    maximum,
                });
            }
        }

        let class_id = EClassId::new(self.classes.len());

        let child_data = {
            let mut data = Vec::with_capacity(canonical_children.len());

            for child in &canonical_children {
                let root = self.find(*child)?;
                data.push(self.class(root)?.analysis());
            }

            data
        };

        let analysis_data = self.analysis.make(&node, &child_data);

        self.classes.push(EClass::new(
            class_id,
            vec![node.clone()],
            analysis_data,
        ));

        self.parents.push(class_id);
        self.ranks.push(0);

        self.hashcons.insert(node, class_id);
        self.dirty.push(class_id);
        self.clean = false;

        self.statistics.nodes = requested_nodes;
        self.statistics.classes = requested_classes;

        Ok(class_id)
    }

    /// Registers an e-class as a root.
    ///
    /// Roots are retained even if later unions cause them to refer to another
    /// representative.
    pub fn add_root(&mut self, id: EClassId) -> EGraphResult<EClassId> {
        let root = self.find(id)?;

        if !self.roots.contains(&root) {
            let requested = self
                .roots
                .len()
                .checked_add(1)
                .ok_or(EGraphError::ArithmeticOverflow {
                    operation: "root count",
                })?;

            if let Some(maximum) = self.limits.max_roots {
                if requested > maximum {
                    return Err(EGraphError::RootLimitExceeded {
                        requested,
                        maximum,
                    });
                }
            }

            self.roots.push(root);
            self.statistics.roots = requested;
        }

        Ok(root)
    }

    /// Returns the first registered root.
    pub fn root(&self) -> EGraphResult<EClassId> {
        self.roots.first().copied().ok_or(EGraphError::MissingRoot)
    }

    /// Unions two e-classes.
    ///
    /// The operation intentionally does not immediately rebuild congruence.
    /// Call `rebuild()` before performing queries that require complete
    /// congruence closure.
    pub fn union(
        &mut self,
        left: EClassId,
        right: EClassId,
    ) -> EGraphResult<EClassId> {
        self.check_time()?;

        let mut left = self.find(left)?;
        let mut right = self.find(right)?;

        EGraphStatistics::record_u64(
            &mut self.statistics.unions,
            1,
            "union count",
        )?;

        if left == right {
            return Ok(left);
        }

        // Union by rank.
        if self.ranks[left.index()] < self.ranks[right.index()] {
            std::mem::swap(&mut left, &mut right);
        }

        self.parents[right.index()] = left;

        if self.ranks[left.index()] == self.ranks[right.index()] {
            self.ranks[left.index()] = self.ranks[left.index()]
                .checked_add(1)
                .ok_or(EGraphError::ArithmeticOverflow {
                    operation: "union-find rank",
                })?;
        }

        let right_nodes = {
            let class = self.class(right)?;
            class.nodes.clone()
        };

        let right_analysis = {
            let class = self.class(right)?;
            class.analysis.clone()
        };

        {
            let left_class = self.class_mut(left)?;

            left_class.nodes.extend(right_nodes);
            left_class.generation = left_class
                .generation
                .checked_add(1)
                .ok_or(EGraphError::ArithmeticOverflow {
                    operation: "e-class generation",
                })?;
        }

        {
            let changed = {
                let left_class = self.class_mut(left)?;
                self.analysis
                    .merge(&mut left_class.analysis, &right_analysis)
            };

            if changed {
                EGraphStatistics::record_u64(
                    &mut self.statistics.analysis_changes,
                    1,
                    "analysis changes",
                )?;
            }
        }

        self.dirty.push(left);
        self.dirty.push(right);
        self.clean = false;

        EGraphStatistics::record_u64(
            &mut self.statistics.merges,
            1,
            "class merges",
        )?;

        Ok(left)
    }

    /// Rebuilds congruence closure.
    ///
    /// This is the central correctness operation of the e-graph.
    ///
    /// The algorithm:
    ///
    /// 1. canonicalizes every node's children;
    /// 2. hash-conses canonical nodes;
    /// 3. unions classes containing congruent nodes;
    /// 4. repeats until no new congruence merge occurs;
    /// 5. refreshes analysis data;
    /// 6. rebuilds the canonical hash-cons table.
    pub fn rebuild(&mut self) -> EGraphResult<()> {
        self.check_time()?;

        if self.clean {
            return Ok(());
        }

        EGraphStatistics::record_u64(
            &mut self.statistics.rebuilds,
            1,
            "rebuild count",
        )?;

        // A full deterministic scan is intentionally used here instead of
        // relying on HashMap iteration order. The e-graph sizes are bounded by
        // explicit resource limits and correctness is more important than
        // hiding nondeterminism inside the rebuild algorithm.
        loop {
            self.check_time()?;

            let mut canonical_table: HashMap<ENode<O>, EClassId> =
                HashMap::new();

            let class_ids: Vec<EClassId> = (0..self.classes.len())
                .map(EClassId::new)
                .collect();

            let mut merge_pair: Option<(EClassId, EClassId)> = None;

            for id in class_ids {
                let root = self.find(id)?;

                if root != id {
                    continue;
                }

                let nodes = self.class(root)?.nodes.clone();

                for node in nodes {
                    let mut children =
                        Vec::with_capacity(node.children.len());

                    for child in node.children() {
                        children.push(self.find(*child)?);
                    }

                    let canonical = node.with_children(children);

                    EGraphStatistics::record_u64(
                        &mut self.statistics.canonicalized_nodes,
                        1,
                        "canonicalized node count",
                    )?;

                    if let Some(existing) =
                        canonical_table.get(&canonical).copied()
                    {
                        let existing_root = self.find(existing)?;

                        if existing_root != root {
                            merge_pair =
                                Some((existing_root, root));
                            break;
                        }
                    } else {
                        canonical_table.insert(canonical, root);
                    }
                }

                if merge_pair.is_some() {
                    break;
                }
            }

            if let Some((left, right)) = merge_pair {
                self.union(left, right)?;

                EGraphStatistics::record_u64(
                    &mut self.statistics.congruence_merges,
                    1,
                    "congruence merge count",
                )?;

                continue;
            }

            // No further congruence collision was found.
            self.hashcons.clear();

            for id in 0..self.classes.len() {
                let id = EClassId::new(id);

                if self.find(id)? != id {
                    continue;
                }

                let mut unique_nodes = Vec::new();
                let mut seen = HashSet::new();

                let nodes = self.class(id)?.nodes.clone();

                for node in nodes {
                    let children = node
                        .children()
                        .iter()
                        .map(|child| self.find_readonly(*child))
                        .collect::<EGraphResult<Vec<_>>>()?;

                    let canonical = node.with_children(children);

                    if seen.insert(canonical.clone()) {
                        self.hashcons.insert(canonical.clone(), id);
                        unique_nodes.push(canonical);
                    }
                }

                self.class_mut(id)?.nodes = unique_nodes;
            }

            self.refresh_analysis()?;

            self.dirty.clear();
            self.clean = true;

            self.statistics.nodes = self.hashcons.len();
            self.statistics.classes = self.active_class_count();

            return Ok(());
        }
    }

    /// Refreshes e-class analysis data after rebuilding.
    fn refresh_analysis(&mut self) -> EGraphResult<()> {
        // Analysis can depend on children. We therefore perform bounded
        // fixed-point rounds. Analyses are expected to be monotone/stable.
        //
        // This avoids recursion and allows future analyses to become more
        // sophisticated without changing the e-graph storage architecture.
        let max_rounds = self
            .classes
            .len()
            .max(1)
            .min(1024usize);

        for _ in 0..max_rounds {
            self.check_time()?;

            let mut changed = false;

            for index in 0..self.classes.len() {
                let id = EClassId::new(index);

                if self.find_readonly(id)? != id {
                    continue;
                }

                let nodes = self.class(id)?.nodes.clone();
                let old_analysis = self.class(id)?.analysis.clone();

                let mut new_analysis = old_analysis.clone();

                for node in &nodes {
                    let child_data = node
                        .children()
                        .iter()
                        .map(|child| {
                            let root = self.find_readonly(*child)?;
                            Ok(self.class(root)?.analysis())
                        })
                        .collect::<EGraphResult<Vec<_>>>()?;

                    self.analysis.rebuild(
                        node,
                        &child_data,
                        &mut new_analysis,
                    );
                }

                if new_analysis != old_analysis {
                    self.class_mut(id)?.analysis = new_analysis;
                    changed = true;

                    EGraphStatistics::record_u64(
                        &mut self.statistics.analysis_changes,
                        1,
                        "analysis changes",
                    )?;
                }
            }

            if !changed {
                return Ok(());
            }
        }

        Ok(())
    }

    /// Returns the number of live representative classes.
    #[must_use]
    pub fn active_class_count(&self) -> usize {
        self.parents
            .iter()
            .enumerate()
            .filter(|(index, parent)| {
                **parent == EClassId::new(*index)
            })
            .count()
    }

    /// Returns all current representative class IDs in stable order.
    pub fn class_ids(&self) -> EGraphResult<Vec<EClassId>> {
        if !self.clean {
            return Err(EGraphError::DirtyRead);
        }

        let mut ids = Vec::new();

        for index in 0..self.classes.len() {
            let id = EClassId::new(index);

            if self.parents[index] == id {
                ids.push(id);
            }
        }

        Ok(ids)
    }

    /// Returns all nodes in a representative class.
    pub fn nodes(
        &mut self,
        id: EClassId,
    ) -> EGraphResult<Vec<ENode<O>>> {
        let root = self.find(id)?;

        if !self.clean {
            return Err(EGraphError::DirtyRead);
        }

        Ok(self.class(root)?.nodes.clone())
    }

    /// Returns whether two e-classes are equivalent.
    pub fn equivalent(
        &mut self,
        left: EClassId,
        right: EClassId,
    ) -> EGraphResult<bool> {
        if !self.clean {
            self.rebuild()?;
        }

        Ok(self.find(left)? == self.find(right)?)
    }

    /// Adds an equality between two existing classes and immediately restores
    /// congruence.
    pub fn union_and_rebuild(
        &mut self,
        left: EClassId,
        right: EClassId,
    ) -> EGraphResult<EClassId> {
        let root = self.union(left, right)?;
        self.rebuild()?;
        Ok(self.find(root)?)
    }

    /// Removes duplicate/stale roots and canonicalizes root IDs.
    pub fn normalize_roots(&mut self) -> EGraphResult<()> {
        if !self.clean {
            self.rebuild()?;
        }

        let mut normalized = Vec::with_capacity(self.roots.len());

        for root in self.roots.clone() {
            let canonical = self.find(root)?;

            if !normalized.contains(&canonical) {
                normalized.push(canonical);
            }
        }

        self.roots = normalized;
        self.statistics.roots = self.roots.len();

        Ok(())
    }

    /// Returns a snapshot of the current stop-related statistics.
    #[must_use]
    pub fn is_saturated_candidate(&self) -> bool {
        self.clean && self.dirty.is_empty()
    }

    /// Clears all e-graph contents.
    pub fn clear(&mut self) {
        self.classes.clear();
        self.parents.clear();
        self.ranks.clear();
        self.hashcons.clear();
        self.dirty.clear();
        self.roots.clear();
        self.statistics = EGraphStatistics::default();
        self.clean = true;
        self.started_at = Instant::now();
    }
}

// =============================================================================
// Extraction
// =============================================================================

impl<O, A> EGraph<O, A>
where
    O: Clone + Eq + Hash + Ord,
    A: EGraphAnalysis<O>,
{
    /// Extracts the minimum-cost representative of one e-class.
    ///
    /// The extraction algorithm uses iterative dynamic programming rather than
    /// recursive descent. This is important for very deep generated circuits.
    ///
    /// If the e-graph contains cyclic representations, extraction continues
    /// until no cost improves or the extraction-round budget is exhausted.
    pub fn extract_best<C>(
        &mut self,
        root: EClassId,
        cost_model: &C,
    ) -> EGraphResult<(C::Cost, Extracted<O>)>
    where
        C: EGraphCost<O>,
    {
        if !self.clean {
            self.rebuild()?;
        }

        let root = self.find(root)?;

        let class_ids = self.class_ids()?;

        let mut best: Vec<Option<(C::Cost, Extracted<O>)>> =
            vec![None; self.classes.len()];

        let mut rounds = 0u64;

        loop {
            self.check_time()?;

            rounds = rounds
                .checked_add(1)
                .ok_or(EGraphError::ArithmeticOverflow {
                    operation: "extraction rounds",
                })?;

            if let Some(maximum) = self.limits.max_extraction_rounds {
                if rounds > maximum {
                    return Err(EGraphError::ExtractionDidNotConverge {
                        rounds: rounds - 1,
                    });
                }
            }

            let mut changed = false;

            for class_id in &class_ids {
                let nodes = self.class(*class_id)?.nodes.clone();

                for node in nodes {
                    let mut child_costs = Vec::with_capacity(
                        node.children().len(),
                    );

                    let mut child_exprs = Vec::with_capacity(
                        node.children().len(),
                    );

                    let mut available = true;

                    for child in node.children() {
                        let child_root = self.find_readonly(*child)?;

                        match &best[child_root.index()] {
                            Some((cost, expression)) => {
                                child_costs.push(cost);
                                child_exprs.push(expression.clone());
                            }

                            None => {
                                available = false;
                                break;
                            }
                        }
                    }

                    if !available {
                        continue;
                    }

                    let candidate_cost =
                        cost_model.cost(
                            node.operation(),
                            &child_costs,
                        )?;

                    EGraphStatistics::record_u64(
                        &mut self.statistics.extraction_candidates,
                        1,
                        "extraction candidate count",
                    )?;

                    if let Some(maximum) =
                        self.limits.max_extraction_candidates
                    {
                        if self.statistics.extraction_candidates
                            > maximum
                        {
                            return Err(
                                EGraphError::ExtractionLimitExceeded {
                                    requested: self.statistics
                                        .extraction_candidates,
                                    maximum,
                                },
                            );
                        }
                    }

                    let candidate =
                        Extracted::node(
                            node.operation().clone(),
                            child_exprs,
                        );

                    let replace = match &best[class_id.index()] {
                        None => true,

                        Some((existing_cost, existing_expression)) => {
                            if candidate_cost < *existing_cost {
                                true
                            } else if candidate_cost == *existing_cost {
                                candidate.node_count()
                                    < existing_expression.node_count()
                            } else {
                                false
                            }
                        }
                    };

                    if replace {
                        best[class_id.index()] =
                            Some((candidate_cost, candidate));

                        changed = true;
                    }
                }
            }

            EGraphStatistics::record_u64(
                &mut self.statistics.extraction_rounds,
                1,
                "extraction round count",
            )?;

            if !changed {
                break;
            }
        }

        best[root.index()]
            .clone()
            .ok_or(EGraphError::NoFiniteExtraction)
    }

    /// Extracts the best representatives for all registered roots.
    pub fn extract_roots<C>(
        &mut self,
        cost_model: &C,
    ) -> EGraphResult<Vec<(EClassId, C::Cost, Extracted<O>)>>
    where
        C: EGraphCost<O>,
    {
        if !self.clean {
            self.rebuild()?;
        }

        let roots = self.roots.clone();
        let mut result = Vec::with_capacity(roots.len());

        for root in roots {
            let root = self.find(root)?;
            let (cost, expression) =
                self.extract_best(root, cost_model)?;

            result.push((root, cost, expression));
        }

        Ok(result)
    }
}

// =============================================================================
// Equality-saturation driver
// =============================================================================

/// Rewrite application interface.
///
/// The actual matching logic belongs in `pattern.rs` / `matcher.rs`, while
/// `rewrite.rs` can implement this trait to connect those matchers to the
/// e-graph core.
///
/// A rewrite implementation must:
///
/// 1. search only after the e-graph is clean;
/// 2. report the number of matches inspected;
/// 3. add RHS nodes using `EGraph::add`;
/// 4. union equivalent classes using `EGraph::union`;
/// 5. never mutate canonical Quantum IR directly.
///
/// The trait is intentionally object-safe so a pipeline can store heterogeneous
/// rewrite implementations.
pub trait EGraphRewrite<O, A>
where
    O: Clone + Eq + Hash + Ord,
    A: EGraphAnalysis<O>,
{
    /// Stable rewrite identifier.
    fn id(&self) -> &str;

    /// Searches for applicable rewrites and applies them.
    ///
    /// Returns `(matches_inspected, rewrites_applied)`.
    fn apply(
        &self,
        egraph: &mut EGraph<O, A>,
    ) -> EGraphResult<(u64, u64)>;
}

/// Result of one equality-saturation run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaturationResult {
    /// Why saturation stopped.
    pub reason: EGraphStopReason,

    /// Number of iterations performed.
    pub iterations: u64,

    /// Number of rewrites applied.
    pub rewrites: u64,

    /// Number of matches inspected.
    pub matches: u64,

    /// Number of nodes in the final graph.
    pub nodes: usize,

    /// Number of classes in the final graph.
    pub classes: usize,
}

/// Equality-saturation runner.
///
/// This runner deliberately does not own the rewrite collection. The caller
/// supplies the rules for each run, allowing `registry.rs`, `planner.rs`, and
/// optimization profiles to decide which rules are active.
pub struct SaturationRunner<'a, O, A>
where
    O: Clone + Eq + Hash + Ord,
    A: EGraphAnalysis<O>,
{
    egraph: &'a mut EGraph<O, A>,
    rewrites: Vec<&'a dyn EGraphRewrite<O, A>>,
    cancelled: bool,
}

impl<'a, O, A> SaturationRunner<'a, O, A>
where
    O: Clone + Eq + Hash + Ord,
    A: EGraphAnalysis<O>,
{
    /// Creates a runner over an existing e-graph.
    pub fn new(egraph: &'a mut EGraph<O, A>) -> Self {
        Self {
            egraph,
            rewrites: Vec::new(),
            cancelled: false,
        }
    }

    /// Adds a rewrite rule.
    pub fn add_rewrite(
        &mut self,
        rewrite: &'a dyn EGraphRewrite<O, A>,
    ) {
        self.rewrites.push(rewrite);
    }

    /// Adds multiple rewrite rules.
    pub fn add_rewrites(
        &mut self,
        rewrites: &'a [&'a dyn EGraphRewrite<O, A>],
    ) {
        self.rewrites.extend_from_slice(rewrites);
    }

    /// Requests cancellation before the next rewrite iteration.
    pub fn cancel(&mut self) {
        self.cancelled = true;
    }

    /// Runs equality saturation.
    pub fn run(
        &mut self,
    ) -> EGraphResult<SaturationResult> {
        self.egraph.rebuild()?;

        loop {
            if self.cancelled {
                return Ok(self.finish(
                    EGraphStopReason::Cancelled,
                ));
            }

            if self.egraph.time_limit_reached() {
                return Ok(self.finish(
                    EGraphStopReason::TimeLimit,
                ));
            }

            let next_iteration = self
                .egraph
                .statistics
                .iterations
                .checked_add(1)
                .ok_or(EGraphError::ArithmeticOverflow {
                    operation: "saturation iterations",
                })?;

            if let Some(maximum) =
                self.egraph.limits.max_iterations
            {
                if next_iteration > maximum {
                    return Ok(self.finish(
                        EGraphStopReason::IterationLimit,
                    ));
                }
            }

            self.egraph.statistics.iterations =
                next_iteration;

            let before_nodes = self.egraph.node_count();
            let before_classes = self.egraph.class_count();
            let before_merges = self.egraph.statistics.merges;

            let mut iteration_rewrites = 0u64;
            let mut iteration_matches = 0u64;

            for rewrite in &self.rewrites {
                self.egraph.check_time()?;

                let (matches, rewrites) =
                    rewrite.apply(self.egraph)?;

                iteration_matches =
                    iteration_matches
                        .checked_add(matches)
                        .ok_or(
                            EGraphError::ArithmeticOverflow {
                                operation: "iteration match count",
                            },
                        )?;

                iteration_rewrites =
                    iteration_rewrites
                        .checked_add(rewrites)
                        .ok_or(
                            EGraphError::ArithmeticOverflow {
                                operation: "iteration rewrite count",
                            },
                        )?;

                self.egraph.statistics.matches =
                    self.egraph.statistics.matches
                        .checked_add(matches)
                        .ok_or(
                            EGraphError::ArithmeticOverflow {
                                operation: "match count",
                            },
                        )?;

                self.egraph.statistics.rewrites =
                    self.egraph.statistics.rewrites
                        .checked_add(rewrites)
                        .ok_or(
                            EGraphError::ArithmeticOverflow {
                                operation: "rewrite count",
                            },
                        )?;

                if let Some(maximum) =
                    self.egraph.limits.max_matches
                {
                    if self.egraph.statistics.matches
                        > maximum
                    {
                        return Ok(self.finish(
                            EGraphStopReason::MatchLimit,
                        ));
                    }
                }

                if let Some(maximum) =
                    self.egraph.limits.max_rewrites
                {
                    if self.egraph.statistics.rewrites
                        > maximum
                    {
                        return Ok(self.finish(
                            EGraphStopReason::RewriteLimit,
                        ));
                    }
                }
            }

            self.egraph.rebuild()?;

            let after_nodes = self.egraph.node_count();
            let after_classes = self.egraph.class_count();
            let after_merges = self.egraph.statistics.merges;

            let changed =
                iteration_rewrites != 0
                    || iteration_matches != 0
                    || before_nodes != after_nodes
                    || before_classes != after_classes
                    || before_merges != after_merges;

            if !changed {
                return Ok(self.finish(
                    EGraphStopReason::Saturated,
                ));
            }
        }
    }

    fn finish(
        &self,
        reason: EGraphStopReason,
    ) -> SaturationResult {
        SaturationResult {
            reason,
            iterations: self.egraph.statistics.iterations,
            rewrites: self.egraph.statistics.rewrites,
            matches: self.egraph.statistics.matches,
            nodes: self.egraph.node_count(),
            classes: self.egraph.active_class_count(),
        }
    }

    /// Returns the underlying e-graph.
    #[must_use]
    pub fn egraph(&self) -> &EGraph<O, A> {
        self.egraph
    }

    /// Returns the underlying mutable e-graph.
    #[must_use]
    pub fn egraph_mut(&mut self) -> &mut EGraph<O, A> {
        self.egraph
    }
}

// =============================================================================
// Utility cost models
// =============================================================================

/// Lexicographic pair cost.
///
/// This is useful for quantum objectives such as:
///
/// ```text
/// primary: two-qubit gate count
/// secondary: total gate count
/// ```
///
/// or:
///
/// ```text
/// primary: T-count
/// secondary: T-depth
/// ```
#[derive(Debug, Clone)]
pub struct PairCost<C1, C2> {
    /// Primary cost model.
    pub first: C1,

    /// Secondary cost model.
    pub second: C2,
}

/// Lexicographic pair cost value.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PairCostValue<V1, V2>(
    /// Primary value.
    pub V1,

    /// Secondary value.
    pub V2,
);

impl<O, C1, C2> EGraphCost<O> for PairCost<C1, C2>
where
    C1: EGraphCost<O>,
    C2: EGraphCost<O>,
    C1::Cost: Ord,
    C2::Cost: Ord,
{
    type Cost = PairCostValue<C1::Cost, C2::Cost>;

    fn cost(
        &self,
        operation: &O,
        children: &[&Self::Cost],
    ) -> Result<Self::Cost, EGraphError> {
        let first_children: Vec<&C1::Cost> =
            children.iter().map(|value| &value.0).collect();

        let second_children: Vec<&C2::Cost> =
            children.iter().map(|value| &value.1).collect();

        Ok(PairCostValue(
            self.first.cost(
                operation,
                &first_children,
            )?,
            self.second.cost(
                operation,
                &second_children,
            )?,
        ))
    }

    fn tie_break(&self, operation: &O) -> Option<u64> {
        self.first
            .tie_break(operation)
            .or_else(|| self.second.tie_break(operation))
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
    )]
    enum Op {
        Input(u32),
        H,
        X,
        Compose,
    }

    #[derive(Debug, Clone, Default)]
    struct TestAnalysis;

    impl EGraphAnalysis<Op> for TestAnalysis {
        type Data = usize;

        fn make(
            &self,
            node: &ENode<Op>,
            children: &[&Self::Data],
        ) -> Self::Data {
            let mut value = 1usize;

            for child in children {
                value = value.saturating_add(**child);
            }

            match node.operation() {
                Op::Input(_) => value,
                Op::H | Op::X | Op::Compose => value,
            }
        }

        fn merge(
            &self,
            to: &mut Self::Data,
            from: &Self::Data,
        ) -> bool {
            if *from > *to {
                *to = *from;
                true
            } else {
                false
            }
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    struct TestCost;

    impl EGraphCost<Op> for TestCost {
        type Cost = u64;

        fn cost(
            &self,
            _operation: &Op,
            children: &[&Self::Cost],
        ) -> Result<Self::Cost, EGraphError> {
            let mut value = 1u64;

            for child in children {
                value = value
                    .checked_add(**child)
                    .ok_or(
                        EGraphError::ArithmeticOverflow {
                            operation: "test extraction cost",
                        },
                    )?;
            }

            Ok(value)
        }
    }

    struct CollapseH;

    impl EGraphRewrite<Op, TestAnalysis> for CollapseH {
        fn id(&self) -> &str {
            "test.collapse_h"
        }

        fn apply(
            &self,
            egraph: &mut EGraph<Op, TestAnalysis>,
        ) -> EGraphResult<(u64, u64)> {
            let classes = egraph.class_ids()?;
            let mut matches = 0u64;
            let mut rewrites = 0u64;

            for class_id in classes {
                let nodes = egraph.nodes(class_id)?;

                for node in nodes {
                    if node.operation() == &Op::H
                        && node.children().len() == 1
                    {
                        matches += 1;

                        let child =
                            node.children()[0];

                        egraph.union(
                            class_id,
                            child,
                        )?;

                        rewrites += 1;
                    }
                }
            }

            Ok((matches, rewrites))
        }
    }

    #[test]
    fn leaf_nodes_are_hash_consistent() {
        let mut graph =
            EGraph::testing(TestAnalysis);

        let a = graph
            .add(Op::Input(0), &[])
            .expect("add a");

        let b = graph
            .add(Op::Input(0), &[])
            .expect("add b");

        assert_eq!(a, b);
        assert_eq!(graph.node_count(), 1);
        assert_eq!(graph.class_count(), 1);
    }

    #[test]
    fn equivalent_children_create_congruent_nodes() {
        let mut graph =
            EGraph::testing(TestAnalysis);

        let a = graph
            .add(Op::Input(0), &[])
            .expect("add a");

        let b = graph
            .add(Op::Input(1), &[])
            .expect("add b");

        let h_a = graph
            .add(Op::H, &[a])
            .expect("add h a");

        let h_b = graph
            .add(Op::H, &[b])
            .expect("add h b");

        assert_ne!(h_a, h_b);

        graph
            .union_and_rebuild(a, b)
            .expect("union");

        assert!(
            graph
                .equivalent(h_a, h_b)
                .expect("equivalence")
        );
    }

    #[test]
    fn roots_are_normalized_after_union() {
        let mut graph =
            EGraph::testing(TestAnalysis);

        let a = graph
            .add(Op::Input(0), &[])
            .expect("add a");

        let b = graph
            .add(Op::Input(1), &[])
            .expect("add b");

        graph.add_root(a).expect("root a");
        graph.add_root(b).expect("root b");

        graph
            .union_and_rebuild(a, b)
            .expect("union");

        graph
            .normalize_roots()
            .expect("normalize");

        assert_eq!(graph.root_count(), 1);
    }

    #[test]
    fn extraction_prefers_lower_cost_expression() {
        let mut graph =
            EGraph::testing(TestAnalysis);

        let input = graph
            .add(Op::Input(0), &[])
            .expect("input");

        let h = graph
            .add(Op::H, &[input])
            .expect("h");

        let x = graph
            .add(Op::X, &[])
            .expect("x");

        graph
            .union_and_rebuild(h, x)
            .expect("equivalence");

        let (cost, expression) = graph
            .extract_best(
                h,
                &TestCost,
            )
            .expect("extract");

        assert!(cost <= 2);
        assert_eq!(expression.operation, Op::X);
    }

    #[test]
    fn saturation_runner_reaches_fixed_point() {
        let mut graph =
            EGraph::testing(TestAnalysis);

        let input = graph
            .add(Op::Input(0), &[])
            .expect("input");

        let h = graph
            .add(Op::H, &[input])
            .expect("h");

        graph
            .add_root(h)
            .expect("root");

        let rule = CollapseH;

        let mut runner =
            SaturationRunner::new(&mut graph);

        runner.add_rewrite(&rule);

        let result =
            runner.run().expect("run");

        assert_eq!(
            result.reason,
            EGraphStopReason::Saturated
        );
        assert!(result.rewrites > 0);
    }

    #[test]
    fn union_find_handles_long_chains_without_recursion() {
        let mut graph =
            EGraph::testing(NoAnalysis);

        let mut ids = Vec::new();

        for index in 0..1_000usize {
            ids.push(
                graph
                    .add(Op::Input(index as u32), &[])
                    .expect("add"),
            );
        }

        for pair in ids.windows(2) {
            graph
                .union(pair[0], pair[1])
                .expect("union");
        }

        graph
            .rebuild()
            .expect("rebuild");

        let root = graph
            .find(ids[999])
            .expect("find");

        for id in ids {
            assert_eq!(
                graph.find(id).expect("find"),
                root
            );
        }
    }

    #[test]
    fn extraction_is_iterative() {
        let mut graph =
            EGraph::testing(NoAnalysis);

        let mut current = graph
            .add(Op::Input(0), &[])
            .expect("leaf");

        for _ in 0..100 {
            current = graph
                .add(Op::H, &[current])
                .expect("deep node");
        }

        graph.add_root(current).expect("root");
        graph.rebuild().expect("rebuild");

        let (_cost, expression) =
            graph
                .extract_best(
                    current,
                    &UnitCost,
                )
                .expect("extract");

        assert_eq!(expression.node_count(), 101);
    }

    #[test]
    fn limits_are_enforced() {
        let limits = EGraphLimits {
            max_nodes: Some(1),
            max_classes: Some(1),
            max_roots: Some(1),
            ..EGraphLimits::unlimited()
        };

        let mut graph =
            EGraph::new(NoAnalysis, limits);

        graph
            .add(Op::Input(0), &[])
            .expect("first node");

        let result =
            graph.add(Op::Input(1), &[]);

        assert!(matches!(
            result,
            Err(EGraphError::NodeLimitExceeded { .. })
                | Err(EGraphError::ClassLimitExceeded { .. })
        ));
    }

    #[test]
    fn operation_order_is_preserved() {
        let mut graph =
            EGraph::testing(NoAnalysis);

        let a = graph
            .add(Op::Input(0), &[])
            .expect("a");

        let b = graph
            .add(Op::Input(1), &[])
            .expect("b");

        let ab = graph
            .add(Op::Compose, &[a, b])
            .expect("ab");

        let ba = graph
            .add(Op::Compose, &[b, a])
            .expect("ba");

        assert_ne!(ab, ba);
    }
}