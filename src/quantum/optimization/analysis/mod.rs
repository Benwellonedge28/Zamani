//! Zamani Quantum Optimization — Analysis Framework
//!
//! Production analysis namespace for logical quantum-circuit optimization.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                            │
//!                            ▼
//!              optimization::analysis
//!                            │
//!          ┌─────────────────┼──────────────────┐
//!          │                 │                  │
//!          ▼                 ▼                  ▼
//!       structural       semantic          resource
//!        analyses        analyses           analyses
//!          │                 │                  │
//!          └─────────────────┼──────────────────┘
//!                            ▼
//!                   optimization passes
//! ```
//!
//! This module is the authoritative namespace for analyses used by
//! `quantum::optimization`.
//!
//! The canonical quantum representation remains:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! This module MUST NOT introduce another:
//!
//! - quantum circuit representation;
//! - gate representation;
//! - qubit representation;
//! - operation representation;
//! - physical topology representation;
//! - scheduling representation;
//! - backend representation.
//!
//! Analyses observe the canonical Quantum IR and produce immutable information
//! that optimization infrastructure and passes can consume.
//!
//! # Current analyses
//!
//! The production analysis family currently consists of:
//!
//! - [`qubit_use`] — logical qubit operand usage and use intervals;
//! - [`dependency`] — operation dependency relationships;
//! - [`commutation`] — operation commutation relationships;
//! - [`liveness`] — logical qubit liveness;
//! - [`depth`] — logical circuit depth and layer information;
//! - [`width`] — logical width and peak resource usage;
//! - [`critical_path`] — critical-path information;
//! - [`gate_counts`] — operation/gate resource counts;
//! - [`parameter_usage`] — symbolic parameter usage;
//! - [`entanglement`] — conservative logical interaction/entanglement analysis.
//!
//! The modules are deliberately independent. A new analysis should normally be
//! added as a new child module rather than modifying unrelated analyses.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::ir
//!     │
//!     ├──────────────► analysis::qubit_use
//!     │                       │
//!     │                       ├──► analysis::liveness
//!     │                       └──► analysis consumers
//!     │
//!     ├──────────────► analysis::dependency
//!     │                       │
//!     │                       ├──► analysis::critical_path
//!     │                       └──► optimization passes
//!     │
//!     ├──────────────► analysis::commutation
//!     │
//!     ├──────────────► analysis::depth
//!     │
//!     ├──────────────► analysis::width
//!     │
//!     ├──────────────► analysis::gate_counts
//!     │
//!     ├──────────────► analysis::parameter_usage
//!     │
//!     └──────────────► analysis::entanglement
//!
//! optimization::context
//!     │
//!     └──────────────► cached analysis results
//!
//! optimization::pass
//!     │
//!     └──────────────► analysis requirements/invalidation
//!
//! optimization::pipeline
//!     │
//!     └──────────────► analysis lifecycle
//! ```
//!
//! Analysis modules should not depend upward on:
//!
//! - `pipeline`;
//! - `scheduler`;
//! - `planner`;
//! - optimization transformations;
//! - routing;
//! - scheduling;
//! - hardware;
//! - benchmarking;
//! - QPU execution.
//!
//! # Why this module is intentionally thin
//!
//! `analysis/mod.rs` is a namespace and contract boundary, not an analysis
//! implementation.
//!
//! Keeping it thin is important for the requested development model:
//!
//! > Finish one file completely, then integrate later files without having to
//! > reopen the finished file merely because another analysis was introduced.
//!
//! A concrete analysis should therefore:
//!
//! 1. own its result type;
//! 2. own its analysis-specific error type;
//! 3. consume canonical `quantum::ir` types;
//! 4. expose an immutable result;
//! 5. use checked arithmetic where overflow is possible;
//! 6. avoid global state;
//! 7. avoid unsafe code;
//! 8. remain deterministic unless explicitly documented otherwise;
//! 9. document its complexity;
//! 10. document its invalidation requirements.
//!
//! This module then only needs one additional `pub mod` declaration when a
//! genuinely new analysis family is introduced.
//!
//! # Analysis lifecycle
//!
//! Analyses are conceptually evaluated as follows:
//!
//! ```text
//! QuantumCircuit
//!       │
//!       ▼
//! validate canonical IR
//!       │
//!       ▼
//! analysis computation
//!       │
//!       ▼
//! immutable analysis result
//!       │
//!       ├──────────────► optimization pass
//!       │
//!       ├──────────────► cost model
//!       │
//!       ├──────────────► planner
//!       │
//!       └──────────────► verification
//!
//! circuit mutation
//!       │
//!       ▼
//! invalidate affected analyses
//! ```
//!
//! The analysis namespace itself does not own cache invalidation. Cache
//! lifecycle belongs to `optimization::context`.
//!
//! # Immutability
//!
//! Analysis results should be immutable after construction.
//!
//! This allows:
//!
//! - safe sharing between read-only optimization components;
//! - deterministic pass behavior;
//! - future parallel analysis;
//! - reproducible optimization;
//! - typed caching in `OptimizationContext`.
//!
//! No analysis in this namespace should require a global mutable cache.
//!
//! # Scaling model
//!
//! There is deliberately no fixed maximum circuit size in this module.
//!
//! A circuit may range from:
//!
//! ```text
//! 1 operation
//!
//!             →
//!
//! thousands
//!             →
//!
//! millions
//!             →
//!
//! billions
//!             →
//!
//! larger workloads subject to available resources
//! ```
//!
//! Practical limits are supplied by the canonical IR and optimization
//! resource-policy layers.
//!
//! Individual analyses must choose data structures appropriate to their own
//! workload. In particular, sparse logical-qubit workloads should avoid
//! allocating storage proportional to the declared qubit namespace when only a
//! small subset of qubits is actually used.
//!
//! The existing [`qubit_use`] and [`depth`] implementations follow this
//! principle by using sparse state for encountered qubits. Their documented
//! complexity is proportional to operations and actual operands rather than
//! blindly scaling with the declared logical namespace. 
//!
//! # Determinism
//!
//! Unless an analysis explicitly documents otherwise:
//!
//! - identical canonical input must produce identical output;
//! - public collections must have deterministic ordering;
//! - hash-map iteration order must never become public compiler behavior;
//! - analysis results must not depend on thread scheduling;
//! - no ambient random state may be consulted.
//!
//! This is particularly important because analysis results influence optimizer
//! decisions and therefore can influence generated circuits.
//!
//! # Resource safety
//!
//! Every analysis is expected to:
//!
//! - use checked arithmetic for counters and sizes;
//! - avoid unbounded recursion;
//! - avoid process-global mutable state;
//! - avoid implicit I/O;
//! - avoid backend calls;
//! - avoid QPU execution;
//! - avoid spawning threads internally;
//! - respect resource limits supplied by higher-level optimization
//!   infrastructure where applicable.
//!
//! An analysis may fail explicitly when its result cannot be represented safely.
//!
//! It must never silently wrap counters or indexes.
//!
//! # Safety
//!
//! This entire module forbids unsafe Rust.
//!
//! All child analysis modules are expected to do the same.
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! # Rust compatibility
//!
//! This namespace targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # Public API policy
//!
//! Child analysis modules are public because they are useful to optimization
//! infrastructure and advanced compiler integrations.
//!
//! However, callers should normally use the high-level optimizer APIs rather
//! than depending on implementation-specific analysis internals unless they
//! are themselves implementing compiler passes.
//!
//! Concrete result types remain owned by their corresponding analysis module.
//!
//! This avoids creating a giant centralized type-definition file that would
//! become a merge point for unrelated analysis work.
//!
//! # Analysis taxonomy
//!
//! [`AnalysisKind`] provides a stable, lightweight identifier for the analysis
//! families known by the optimizer.
//!
//! It is intentionally independent from concrete result types.
//!
//! This allows:
//!
//! - `pass.rs` to declare analysis requirements;
//! - `pipeline.rs` to reason about invalidation;
//! - `context.rs` to identify analysis generations;
//! - `planner.rs` to select analyses;
//! - future analyses to be introduced without coupling existing result types.
//!
//! The taxonomy is descriptive rather than a runtime registry. Concrete typed
//! storage remains the responsibility of `OptimizationContext`.
//!
//! # Analysis invalidation
//!
//! [`AnalysisInvalidation`] describes broad circuit mutations that can make an
//! analysis stale.
//!
//! This is deliberately conservative.
//!
//! A pass may invalidate more analyses than strictly necessary, but it must
//! never retain an analysis after changing information that analysis depends
//! upon.
//!
//! The optimizer's context remains responsible for the actual cache lifecycle.
//!
//! # Integration with `OptimizationContext`
//!
//! `optimization::context` already provides typed analysis storage and
//! invalidation-generation infrastructure. It should store concrete analysis
//! result values rather than requiring this module to know every concrete
//! result type.
//!
//! Conceptually:
//!
//! ```text
//! context
//!    │
//!    ├── analysis::<qubit_use::QubitUseAnalysis>()
//!    ├── analysis::<dependency::DependencyAnalysis>()
//!    ├── analysis::<depth::DepthAnalysis>()
//!    └── ...
//! ```
//!
//! The exact typed-storage API remains owned by `context.rs`.
//!
//! # Integration with `pass.rs`
//!
//! A pass may conceptually declare:
//!
//! ```text
//! requires:
//!     QubitUse
//!     Dependency
//!
//! invalidates:
//!     Depth
//!     GateCounts
//! ```
//!
//! This module provides the stable taxonomy; `pass.rs` owns the pass contract.
//!
//! # Integration with `pipeline.rs`
//!
//! The pipeline may use analysis requirements and invalidation metadata to:
//!
//! 1. avoid unnecessary recomputation;
//! 2. reuse immutable analysis results;
//! 3. invalidate stale results after circuit mutation;
//! 4. compute independent analyses in parallel in future implementations;
//! 5. maintain deterministic execution.
//!
//! The analysis namespace does not own pipeline scheduling.
//!
//! # Integration with `planner.rs`
//!
//! The planner may use analysis kinds to select optimization strategies.
//!
//! Examples:
//!
//! ```text
//! gate-heavy circuit
//!     → GateCounts
//!
//! deep circuit
//!     → Depth + CriticalPath
//!
//! sparse logical namespace
//!     → QubitUse + Width
//!
//! parameterized circuit
//!     → ParameterUsage
//!
//! highly connected circuit
//!     → Dependency + Entanglement
//! ```
//!
//! The planner owns the decision. Analysis only supplies facts.
//!
//! # Integration with cost models
//!
//! Cost models may consume analysis results such as:
//!
//! - gate counts;
//! - two-qubit counts;
//! - logical depth;
//! - width;
//! - T counts;
//! - parameter usage;
//! - critical path.
//!
//! Analysis must never depend on the cost model.
//!
//! This keeps the direction:
//!
//! ```text
//! analysis ──► facts
//! cost      ──► valuation of facts
//! ```
//!
//! rather than:
//!
//! ```text
//! analysis ──X──► cost
//! ```
//!
//! # Integration with routing
//!
//! Analysis may provide logical facts useful to routing, but routing does not
//! become an analysis dependency.
//!
//! In particular:
//!
//! - dependency analysis can inform routing heuristics;
//! - qubit-use analysis can inform logical-resource reasoning;
//! - depth can provide a logical lower bound;
//! - entanglement/interaction analysis can help identify important regions.
//!
//! Physical topology remains owned by `quantum::routing` and
//! `quantum::hardware`.
//!
//! # Integration with scheduling
//!
//! Logical depth and dependency information may be consumed by scheduling.
//!
//! This module does not perform hardware scheduling, pulse scheduling, timing,
//! calibration handling, or execution ordering.
//!
//! # Integration with benchmarking
//!
//! Benchmarking may consume analysis results to report optimization effects:
//!
//! ```text
//! original circuit
//!       │
//!       ├── analysis
//!       │
//!       ▼
//! optimization
//!       │
//!       ▼
//! optimized circuit
//!       │
//!       └── analysis
//!             │
//!             ▼
//!          benchmark
//! ```
//!
//! Benchmarking remains a consumer. Analysis must not depend on benchmarking.
//!
//! # Integration with verification
//!
//! Verification may use analysis to construct structural checks or compare
//! resource properties before and after optimization.
//!
//! Analysis is not semantic equivalence verification.
//!
//! In particular:
//!
//! ```text
//! same depth
//!     ≠
//! same quantum semantics
//!
//! same gate count
//!     ≠
//! same quantum semantics
//! ```
//!
//! Semantic equivalence remains owned by the verification/equivalence layers.
//!
//! # Adding a new analysis
//!
//! To add a new analysis:
//!
//! 1. create `src/quantum/optimization/analysis/<name>.rs`;
//! 2. make it consume canonical `quantum::ir`;
//! 3. make its result immutable;
//! 4. document complexity and memory behavior;
//! 5. use checked arithmetic;
//! 6. forbid unsafe code;
//! 7. document invalidation requirements;
//! 8. add `pub mod <name>;` here;
//! 9. add an [`AnalysisKind`] variant if the optimizer infrastructure needs to
//!    identify it generically;
//! 10. add tests in the analysis itself;
//! 11. integrate it into pass requirements only when an actual pass needs it.
//!
//! Existing analyses should not need to be modified merely because another
//! independent analysis was added.
//!
//! # Current module inventory
//!
//! ```text
//! analysis/
//! ├── mod.rs                  ← this file
//! ├── qubit_use.rs
//! ├── dependency.rs
//! ├── commutation.rs
//! ├── liveness.rs
//! ├── depth.rs
//! ├── width.rs
//! ├── critical_path.rs
//! ├── gate_counts.rs
//! ├── parameter_usage.rs
//! └── entanglement.rs
//! ```
//!
//! The repository confirms these concrete analysis files are already present.
//! 
//!
//! # Architectural smoke tests
//!
//! The tests at the bottom of this module intentionally test only namespace
//! contracts. Detailed mathematical tests belong inside each concrete analysis
//! module.
//!
//! This prevents `analysis/mod.rs` from becoming a second test implementation
//! of every analysis algorithm.

#![forbid(unsafe_code)]

// =============================================================================
// Concrete analysis modules
// =============================================================================

/// Logical qubit operand-use analysis.
///
/// Provides first/last use, use counts, measurement/reset use, and related
/// logical-qubit usage information.
pub mod qubit_use;

/// Logical operation dependency analysis.
///
/// Provides the dependency relationships required by optimization and
/// critical-path reasoning.
pub mod dependency;

/// Gate-operation commutation analysis.
///
/// Determines conservative commutation relationships without modifying the
/// circuit.
pub mod commutation;

/// Logical qubit liveness analysis.
///
/// Determines logical lifetime/use information needed by width and resource
/// optimization.
pub mod liveness;

/// Backend-independent logical circuit-depth analysis.
///
/// Computes logical ASAP depth and related layer metrics. It does not perform
/// hardware scheduling.
pub mod depth;

/// Logical circuit-width/resource analysis.
///
/// Computes logical width and related resource metrics.
pub mod width;

/// Critical-path analysis.
///
/// Identifies dependency/depth information relevant to critical-path
/// optimization.
pub mod critical_path;

/// Gate/operation resource-count analysis.
///
/// Counts logical operation resources independently from depth analysis.
pub mod gate_counts;

/// Symbolic parameter-usage analysis.
///
/// Tracks parameter references and their use across the canonical circuit.
pub mod parameter_usage;

/// Conservative interaction/entanglement analysis.
///
/// Provides logical interaction information useful for optimization planning.
pub mod entanglement;

// =============================================================================
// Stable analysis taxonomy
// =============================================================================

/// Stable identifier for a class of optimization analysis.
///
/// `AnalysisKind` deliberately identifies an analysis *family*, not a concrete
/// Rust result type. Concrete results remain owned by their analysis modules.
///
/// This separation allows `OptimizationContext` to use typed storage while
/// `pass.rs` and `pipeline.rs` can reason about requirements without importing
/// every concrete analysis result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum AnalysisKind {
    /// Logical qubit operand-use analysis.
    QubitUse,

    /// Logical operation dependency analysis.
    Dependency,

    /// Operation commutation analysis.
    Commutation,

    /// Logical qubit liveness analysis.
    Liveness,

    /// Logical circuit-depth analysis.
    Depth,

    /// Logical circuit-width/resource analysis.
    Width,

    /// Critical-path analysis.
    CriticalPath,

    /// Gate/operation resource-count analysis.
    GateCounts,

    /// Symbolic parameter-usage analysis.
    ParameterUsage,

    /// Logical interaction/entanglement analysis.
    Entanglement,
}

impl AnalysisKind {
    /// Returns the canonical stable identifier for this analysis.
    ///
    /// These identifiers are suitable for:
    ///
    /// - diagnostics;
    /// - provenance;
    /// - logging;
    /// - serialization;
    /// - metrics;
    /// - debugging;
    /// - pass metadata.
    ///
    /// They are intentionally lowercase and stable.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QubitUse => "qubit_use",
            Self::Dependency => "dependency",
            Self::Commutation => "commutation",
            Self::Liveness => "liveness",
            Self::Depth => "depth",
            Self::Width => "width",
            Self::CriticalPath => "critical_path",
            Self::GateCounts => "gate_counts",
            Self::ParameterUsage => "parameter_usage",
            Self::Entanglement => "entanglement",
        }
    }

    /// Returns all currently registered analysis kinds in deterministic order.
    ///
    /// This is intentionally a fixed slice rather than a dynamically allocated
    /// collection.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::QubitUse,
            Self::Dependency,
            Self::Commutation,
            Self::Liveness,
            Self::Depth,
            Self::Width,
            Self::CriticalPath,
            Self::GateCounts,
            Self::ParameterUsage,
            Self::Entanglement,
        ]
    }
}

impl AsRef<str> for AnalysisKind {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AnalysisKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Analysis invalidation taxonomy
// =============================================================================

/// Broad categories of circuit mutation that can invalidate cached analyses.
///
/// This is intentionally conservative and independent from concrete analysis
/// implementation details.
///
/// Higher-level optimization infrastructure may invalidate a superset of the
/// strictly necessary analyses. It must never retain stale analysis facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum AnalysisInvalidation {
    /// The operation sequence itself changed.
    ///
    /// Examples:
    ///
    /// - insert operation;
    /// - remove operation;
    /// - reorder operation;
    /// - replace operation with a different operation.
    OperationSequence,

    /// One or more logical-qubit operands changed.
    QubitOperands,

    /// Operation parameters changed.
    Parameters,

    /// Operation semantic classification changed.
    ///
    /// Examples include changing an operation from unitary to measurement or
    /// from one gate family to another.
    OperationSemantics,

    /// Classical-control or measurement dependencies changed.
    ClassicalDependencies,

    /// Region/control-flow structure changed.
    ControlFlow,

    /// Gate metadata changed in a way that can affect analysis semantics.
    Metadata,

    /// The canonical circuit's declared logical-qubit namespace changed.
    QubitNamespace,

    /// The canonical circuit was replaced wholesale.
    WholeCircuit,
}

impl AnalysisInvalidation {
    /// Returns whether this mutation conservatively invalidates the supplied
    /// analysis.
    ///
    /// The mapping is intentionally conservative. More precise dependency
    /// tracking belongs to the pass/pipeline infrastructure once a concrete
    /// pass declares its exact requirements.
    #[must_use]
    pub const fn invalidates(self, analysis: AnalysisKind) -> bool {
        match self {
            Self::WholeCircuit => true,

            Self::QubitNamespace => matches!(
                analysis,
                AnalysisKind::QubitUse
                    | AnalysisKind::Liveness
                    | AnalysisKind::Width
                    | AnalysisKind::Depth
                    | AnalysisKind::CriticalPath
                    | AnalysisKind::Dependency
                    | AnalysisKind::Commutation
                    | AnalysisKind::Entanglement
            ),

            Self::OperationSequence => true,

            Self::QubitOperands => matches!(
                analysis,
                AnalysisKind::QubitUse
                    | AnalysisKind::Dependency
                    | AnalysisKind::Commutation
                    | AnalysisKind::Liveness
                    | AnalysisKind::Depth
                    | AnalysisKind::Width
                    | AnalysisKind::CriticalPath
                    | AnalysisKind::GateCounts
                    | AnalysisKind::Entanglement
            ),

            Self::Parameters => matches!(
                analysis,
                AnalysisKind::ParameterUsage
                    | AnalysisKind::Commutation
                    | AnalysisKind::Depth
                    | AnalysisKind::CriticalPath
                    | AnalysisKind::GateCounts
            ),

            Self::OperationSemantics => true,

            Self::ClassicalDependencies => matches!(
                analysis,
                AnalysisKind::Dependency
                    | AnalysisKind::Commutation
                    | AnalysisKind::Liveness
                    | AnalysisKind::Depth
                    | AnalysisKind::CriticalPath
                    | AnalysisKind::Entanglement
            ),

            Self::ControlFlow => true,

            Self::Metadata => matches!(
                analysis,
                AnalysisKind::GateCounts
                    | AnalysisKind::Commutation
                    | AnalysisKind::ParameterUsage
            ),
        }
    }

    /// Returns all currently known analysis kinds that are conservatively
    /// invalidated by this mutation.
    ///
    /// The returned slice is statically allocated and deterministic.
    #[must_use]
    pub fn invalidated_analyses(self) -> Vec<AnalysisKind> {
        AnalysisKind::all()
            .iter()
            .copied()
            .filter(|analysis| self.invalidates(*analysis))
            .collect()
    }
}

// =============================================================================
// Analysis requirements
// =============================================================================

/// Immutable collection of analysis requirements for a pass.
///
/// This small value type is useful for pass metadata and pipeline planning.
///
/// It intentionally uses a fixed bitset instead of a heap-allocated collection.
/// Therefore adding/removing a requirement is O(1), and copying the set is
/// cheap.
///
/// The public representation is opaque so its storage can evolve without
/// breaking callers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AnalysisSet(u16);

impl AnalysisSet {
    /// Creates an empty analysis set.
    #[must_use]
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Creates a set containing one analysis kind.
    #[must_use]
    pub const fn singleton(kind: AnalysisKind) -> Self {
        Self(0).with(kind)
    }

    /// Adds an analysis requirement.
    #[must_use]
    pub const fn with(self, kind: AnalysisKind) -> Self {
        Self(self.0 | Self::bit(kind))
    }

    /// Removes an analysis requirement.
    #[must_use]
    pub const fn without(self, kind: AnalysisKind) -> Self {
        Self(self.0 & !Self::bit(kind))
    }

    /// Returns whether this set contains the specified analysis.
    #[must_use]
    pub const fn contains(self, kind: AnalysisKind) -> bool {
        (self.0 & Self::bit(kind)) != 0
    }

    /// Returns whether this set contains no analyses.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns the number of analyses in this set.
    #[must_use]
    pub const fn len(self) -> usize {
        self.0.count_ones() as usize
    }

    /// Returns the raw bit representation.
    ///
    /// This is intended for compact diagnostics/provenance and not for
    /// implementing analysis semantics outside this module.
    #[must_use]
    pub const fn bits(self) -> u16 {
        self.0
    }

    /// Returns an iterator over contained analysis kinds in deterministic
    /// [`AnalysisKind`] order.
    #[must_use]
    pub fn iter(self) -> AnalysisSetIter {
        AnalysisSetIter {
            set: self,
            index: 0,
        }
    }

    const fn bit(kind: AnalysisKind) -> u16 {
        match kind {
            AnalysisKind::QubitUse => 1 << 0,
            AnalysisKind::Dependency => 1 << 1,
            AnalysisKind::Commutation => 1 << 2,
            AnalysisKind::Liveness => 1 << 3,
            AnalysisKind::Depth => 1 << 4,
            AnalysisKind::Width => 1 << 5,
            AnalysisKind::CriticalPath => 1 << 6,
            AnalysisKind::GateCounts => 1 << 7,
            AnalysisKind::ParameterUsage => 1 << 8,
            AnalysisKind::Entanglement => 1 << 9,
        }
    }
}

impl std::ops::BitOr for AnalysisSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for AnalysisSet {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitAnd for AnalysisSet {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for AnalysisSet {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::Sub for AnalysisSet {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 & !rhs.0)
    }
}

impl std::ops::SubAssign for AnalysisSet {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 &= !rhs.0;
    }
}

/// Iterator over an [`AnalysisSet`].
#[derive(Debug, Clone)]
pub struct AnalysisSetIter {
    set: AnalysisSet,
    index: usize,
}

impl Iterator for AnalysisSetIter {
    type Item = AnalysisKind;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < AnalysisKind::all().len() {
            let kind = AnalysisKind::all()[self.index];
            self.index += 1;

            if self.set.contains(kind) {
                return Some(kind);
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self
            .set
            .bits()
            .count_ones()
            .saturating_sub(
                self.index
                    .min(AnalysisKind::all().len())
                    as u32,
            ) as usize;

        (0, Some(remaining))
    }
}

impl ExactSizeIterator for AnalysisSetIter {
    fn len(&self) -> usize {
        self.set
            .bits()
            .count_ones()
            .saturating_sub(
                self.index
                    .min(AnalysisKind::all().len())
                    as u32,
            ) as usize
    }
}

// =============================================================================
// Public convenience constants
// =============================================================================

/// Empty analysis requirement set.
pub const NO_ANALYSES: AnalysisSet = AnalysisSet::empty();

/// Analysis set containing the logical-qubit-use analysis.
pub const QUBIT_USE: AnalysisSet = AnalysisSet::singleton(AnalysisKind::QubitUse);

/// Analysis set containing dependency analysis.
pub const DEPENDENCY: AnalysisSet = AnalysisSet::singleton(AnalysisKind::Dependency);

/// Analysis set containing commutation analysis.
pub const COMMUTATION: AnalysisSet = AnalysisSet::singleton(AnalysisKind::Commutation);

/// Analysis set containing liveness analysis.
pub const LIVENESS: AnalysisSet = AnalysisSet::singleton(AnalysisKind::Liveness);

/// Analysis set containing depth analysis.
pub const DEPTH: AnalysisSet = AnalysisSet::singleton(AnalysisKind::Depth);

/// Analysis set containing width analysis.
pub const WIDTH: AnalysisSet = AnalysisSet::singleton(AnalysisKind::Width);

/// Analysis set containing critical-path analysis.
pub const CRITICAL_PATH: AnalysisSet =
    AnalysisSet::singleton(AnalysisKind::CriticalPath);

/// Analysis set containing gate-count analysis.
pub const GATE_COUNTS: AnalysisSet =
    AnalysisSet::singleton(AnalysisKind::GateCounts);

/// Analysis set containing parameter-usage analysis.
pub const PARAMETER_USAGE: AnalysisSet =
    AnalysisSet::singleton(AnalysisKind::ParameterUsage);

/// Analysis set containing entanglement analysis.
pub const ENTANGLEMENT: AnalysisSet =
    AnalysisSet::singleton(AnalysisKind::Entanglement);

// =============================================================================
// Common analysis bundles
// =============================================================================

/// Structural analysis bundle.
///
/// Useful for passes that need the basic logical structure of a circuit.
pub const STRUCTURAL: AnalysisSet = AnalysisSet(
    QUBIT_USE.bits()
        | DEPENDENCY.bits()
        | DEPTH.bits()
        | WIDTH.bits(),
);

/// Optimization/resource analysis bundle.
///
/// Useful for cost-oriented optimization planning.
pub const RESOURCE: AnalysisSet = AnalysisSet(
    GATE_COUNTS.bits()
        | DEPTH.bits()
        | WIDTH.bits()
        | CRITICAL_PATH.bits(),
);

/// Interaction analysis bundle.
///
/// Useful for transformations involving dependencies, commutation and
/// multi-qubit interactions.
pub const INTERACTION: AnalysisSet = AnalysisSet(
    QUBIT_USE.bits()
        | DEPENDENCY.bits()
        | COMMUTATION.bits()
        | ENTANGLEMENT.bits(),
);

/// Parameterized-circuit analysis bundle.
pub const PARAMETERIZED: AnalysisSet = AnalysisSet(
    PARAMETER_USAGE.bits()
        | GATE_COUNTS.bits()
        | DEPTH.bits(),
);

/// Broad analysis bundle for aggressive optimization planning.
///
/// This is intentionally explicit rather than an alias to an implementation
/// detail of the concrete analysis registry.
pub const ALL: AnalysisSet = AnalysisSet(
    QUBIT_USE.bits()
        | DEPENDENCY.bits()
        | COMMUTATION.bits()
        | LIVENESS.bits()
        | DEPTH.bits()
        | WIDTH.bits()
        | CRITICAL_PATH.bits()
        | GATE_COUNTS.bits()
        | PARAMETER_USAGE.bits()
        | ENTANGLEMENT.bits(),
);

// =============================================================================
// Compatibility aliases
// =============================================================================

/// Alias for [`AnalysisKind`].
///
/// Kept as a semantic convenience for callers that refer to analysis IDs
/// rather than analysis kinds.
pub type AnalysisId = AnalysisKind;

/// Alias for [`AnalysisSet`].
///
/// Useful when a caller conceptually treats a set as a requirement set.
pub type AnalysisRequirements = AnalysisSet;

// =============================================================================
// Namespace smoke tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_current_analysis_modules_are_registered() {
        assert_eq!(AnalysisKind::all().len(), 10);
    }

    #[test]
    fn analysis_identifiers_are_stable() {
        let identifiers: Vec<&'static str> =
            AnalysisKind::all().iter().map(|kind| kind.as_str()).collect();

        assert_eq!(
            identifiers,
            vec![
                "qubit_use",
                "dependency",
                "commutation",
                "liveness",
                "depth",
                "width",
                "critical_path",
                "gate_counts",
                "parameter_usage",
                "entanglement",
            ]
        );
    }

    #[test]
    fn analysis_set_is_deterministic() {
        let set = QUBIT_USE | DEPENDENCY | DEPTH;

        assert_eq!(set.len(), 3);

        let values: Vec<AnalysisKind> = set.iter().collect();

        assert_eq!(
            values,
            vec![
                AnalysisKind::QubitUse,
                AnalysisKind::Dependency,
                AnalysisKind::Depth,
            ]
        );
    }

    #[test]
    fn analysis_set_operations_are_correct() {
        let all = QUBIT_USE | DEPENDENCY | DEPTH;

        assert!(all.contains(AnalysisKind::QubitUse));
        assert!(all.contains(AnalysisKind::Dependency));
        assert!(all.contains(AnalysisKind::Depth));
        assert!(!all.contains(AnalysisKind::Width));

        let reduced = all.without(AnalysisKind::Dependency);

        assert!(reduced.contains(AnalysisKind::QubitUse));
        assert!(!reduced.contains(AnalysisKind::Dependency));
        assert!(reduced.contains(AnalysisKind::Depth));
    }

    #[test]
    fn common_bundles_are_non_empty() {
        assert!(!STRUCTURAL.is_empty());
        assert!(!RESOURCE.is_empty());
        assert!(!INTERACTION.is_empty());
        assert!(!PARAMETERIZED.is_empty());
        assert!(!ALL.is_empty());
    }

    #[test]
    fn all_contains_every_registered_analysis() {
        for kind in AnalysisKind::all() {
            assert!(ALL.contains(*kind));
        }
    }

    #[test]
    fn invalidation_is_conservative() {
        assert!(
            AnalysisInvalidation::WholeCircuit
                .invalidated_analyses()
                .len()
                == AnalysisKind::all().len()
        );

        assert!(
            AnalysisInvalidation::OperationSequence
                .invalidated_analyses()
                .len()
                == AnalysisKind::all().len()
        );

        assert!(
            AnalysisInvalidation::QubitOperands
                .invalidates(AnalysisKind::QubitUse)
        );

        assert!(
            AnalysisInvalidation::QubitOperands
                .invalidates(AnalysisKind::Dependency)
        );

        assert!(
            AnalysisInvalidation::Parameters
                .invalidates(AnalysisKind::ParameterUsage)
        );
    }

    #[test]
    fn analysis_kind_order_is_stable() {
        let mut kinds = AnalysisKind::all().to_vec();
        kinds.sort();

        assert_eq!(kinds.len(), AnalysisKind::all().len());
    }

    #[test]
    fn analysis_kind_display_is_stable() {
        assert_eq!(AnalysisKind::QubitUse.to_string(), "qubit_use");
        assert_eq!(AnalysisKind::CriticalPath.to_string(), "critical_path");
        assert_eq!(AnalysisKind::ParameterUsage.to_string(), "parameter_usage");
    }

    #[test]
    fn empty_analysis_set_is_empty() {
        assert!(NO_ANALYSES.is_empty());
        assert_eq!(NO_ANALYSES.len(), 0);
        assert_eq!(NO_ANALYSES.iter().next(), None);
    }
}