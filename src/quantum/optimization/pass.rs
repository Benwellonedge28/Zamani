//! Zamani Quantum Optimization — Optimization Pass Contract
//!
//! Production-grade abstraction for optimization passes operating on the
//! canonical `crate::quantum::ir::QuantumCircuit`.
//!
//! # Architectural position
//!
//! The optimization subsystem has the following dependency direction:
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                         quantum::ir
//!                              │
//!                              ▼
//!                    optimization::pass
//!                              │
//!          ┌───────────────────┼────────────────────┐
//!          │                   │                    │
//!          ▼                   ▼                    ▼
//!       analysis            rewrite             synthesis
//!          │                   │                    │
//!          └───────────────────┼────────────────────┘
//!                              ▼
//!                           pipeline
//!                              │
//!                              ▼
//!                       optimized quantum::ir
//! ```
//!
//! This module defines the stable contract between the optimization engine and
//! individual optimization passes.
//!
//! # Critical ownership rule
//!
//! This file does NOT define:
//!
//! - `QuantumGate`;
//! - `QuantumOperation`;
//! - `QuantumCircuit`;
//! - another circuit IR;
//! - routing;
//! - scheduling;
//! - hardware execution;
//! - QPU APIs;
//! - benchmarking;
//! - QEC semantics;
//! - individual optimization algorithms.
//!
//! The authoritative circuit representation remains:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! The pass contract merely gives optimization algorithms a stable interface
//! through which they transform that canonical representation.
//!
//! # Design goals
//!
//! This contract is designed to support:
//!
//! - tiny circuits;
//! - large circuits;
//! - workloads limited only by available resources and explicit policies;
//! - deterministic compilation;
//! - reproducible compilation;
//! - parallel pass scheduling;
//! - incremental optimization;
//! - fixed-point optimization;
//! - target-aware optimization;
//! - multi-objective optimization;
//! - fault-tolerant optimization;
//! - algebraic optimization;
//! - synthesis;
//! - equality saturation;
//! - approximate optimization;
//! - verified optimization;
//! - analysis-dependent passes;
//! - passes that do not require analyses;
//! - future optimization domains not yet implemented.
//!
//! # Stability principle
//!
//! A new optimization pass should normally require only:
//!
//! 1. implementation of `OptimizationPass`;
//! 2. a `PassMetadata` value;
//! 3. registration with `registry.rs`;
//! 4. inclusion in an appropriate planner/pipeline.
//!
//! `pass.rs` itself should not need to change when a new pass is introduced.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features.
//!
//! # Safety
//!
//! This module forbids unsafe code.
//!
//! No `unsafe` code is required for the pass abstraction.
//!
//! # Integration contract
//!
//! ## `errors.rs`
//!
//! Uses the canonical `OptimizationError`, `OptimizationResult`, and
//! `PassIdentifier` definitions.
//!
//! ## `context.rs`
//!
//! Passes receive an invocation-scoped `OptimizationContext`.
//!
//! The context owns mutable optimizer execution state, resource accounting,
//! analysis caches, deterministic state, and pass-local services.
//!
//! A pass does not create a second global optimizer context.
//!
//! ## `circuit.rs`
//!
//! Passes may use `CircuitView`, `CircuitEditPlan`, and `CircuitEditor` when
//! they require transactional editing.
//!
//! The pass contract itself operates on the canonical `QuantumCircuit` so the
//! pipeline remains independent of a particular editing strategy.
//!
//! ## `analysis/*`
//!
//! `PassRequirements` declares analyses required before execution.
//!
//! `PassEffects` declares analyses/properties invalidated by the pass.
//!
//! The pipeline/scheduler is responsible for obtaining and invalidating those
//! analyses.
//!
//! ## `pipeline.rs`
//!
//! The pipeline invokes `OptimizationPass::run` in the selected order.
//!
//! It owns pass sequencing, fixed-point iteration, recovery policy, and
//! pipeline-level stopping decisions.
//!
//! ## `registry.rs`
//!
//! The registry stores pass instances/factories and uses `PassId`,
//! `PassMetadata`, and capability declarations to identify them.
//!
//! ## `planner.rs`
//!
//! The planner selects passes based on configuration, target, circuit
//! characteristics, cost model, and optimization profile.
//!
//! ## `rules.rs` / `rewrite.rs`
//!
//! Rewrite-based passes can declare rewrite capability and use the generic
//! rewrite infrastructure. The pass contract does not depend on a particular
//! rewrite engine.
//!
//! ## `targets/*`
//!
//! Target-specific passes declare target requirements through metadata and
//! capabilities. The pass itself must not directly call hardware APIs.
//!
//! ## `verification/*`
//!
//! Verification remains outside the pass abstraction. A pass may declare that
//! it benefits from verification, but the verification subsystem owns semantic
//! equivalence checking.
//!
//! ## `result.rs`
//!
//! `PassOutcome` is an execution-time pass contract. The pipeline converts it
//! into the result-layer `PassResult` representation when constructing the
//! final `OptimizationResult`.
//!
//! This prevents `pass.rs` from becoming coupled to the final result-reporting
//! implementation.
//!
//! ## `statistics.rs`
//!
//! Passes report coarse execution information through `PassOutcome`.
//!
//! Detailed accounting remains owned by `statistics.rs` and the context.
//!
//! ## `provenance.rs`
//!
//! `PassMetadata` and `PassOutcome` expose stable identifiers and execution
//! information which provenance can record without depending on pass
//! implementation types.
//!
//! # Pass lifecycle
//!
//! The intended lifecycle is:
//!
//! ```text
//! registry
//!    │
//!    ▼
//! planner
//!    │
//!    ▼
//! pipeline
//!    │
//!    ├── validate metadata
//!    │
//!    ├── obtain required analyses
//!    │
//!    ├── establish pass context
//!    │
//!    ├── execute pass
//!    │
//!    ├── validate outcome
//!    │
//!    ├── invalidate declared analyses
//!    │
//!    ├── record statistics
//!    │
//!    └── record provenance
//!    │
//!    ▼
//! next pass
//! ```
//!
//! The pass itself must only be responsible for its transformation/analysis
//! behavior.
//!
//! # Important semantic rule
//!
//! A pass must not silently change the semantic equivalence policy.
//!
//! For example, a pass that is valid only up to global phase must explicitly
//! declare that requirement through its metadata/capability and the pipeline
//! must ensure the configured equivalence policy permits it.
//!
//! # Resource scaling
//!
//! This abstraction intentionally does not impose a circuit-size limit.
//!
//! Resource limits are supplied by `OptimizationContext` and
//! `OptimizationLimits`.
//!
//! A pass must cooperate with those limits rather than inventing its own
//! process-global limits.
//!
//! A pass may still expose its expected complexity through `PassComplexity`
//! metadata so that the planner can avoid unsuitable passes for enormous
//! circuits.
//!
//! # Determinism
//!
//! Passes that use randomized heuristics must declare
//! `PassDeterminism::Seeded` or `PassDeterminism::Nondeterministic`.
//!
//! A seeded pass must derive randomness from the optimizer context rather than
//! ambient process-global randomness.
//!
//! # Thread safety
//!
//! `OptimizationPass` requires `Send + Sync`.
//!
//! The pass contract does not spawn threads. Parallel execution belongs to
//! `scheduler.rs`.
//!
//! A pass must therefore keep mutable invocation state in `OptimizationContext`
//! rather than global/static mutable state.
//!
//! # Object safety
//!
//! `OptimizationPass` is intentionally object-safe so the registry and
//! scheduler can store heterogeneous passes as:
//!
//! `Box<dyn OptimizationPass>`
//!
//! without requiring generic dispatch throughout the optimization pipeline.

#![forbid(unsafe_code)]

use std::fmt;

use crate::quantum::ir::QuantumCircuit;

use super::context::OptimizationContext;
use super::errors::{
    OptimizationError,
    OptimizationResult,
    PassIdentifier,
};

// =============================================================================
// Public aliases
// =============================================================================

/// Canonical pass identifier used by the optimization subsystem.
///
/// `errors.rs` owns the actual identifier representation so that error
/// reporting, provenance, diagnostics, and pass infrastructure cannot
/// accidentally diverge into multiple identifier types.
pub type PassId = PassIdentifier;

/// Result type returned by an optimization pass.
pub type PassExecutionResult = OptimizationResult<PassOutcome>;

/// Result type used when validating pass metadata.
pub type PassMetadataResult<T> = Result<T, PassMetadataError>;

// =============================================================================
// Pass kind
// =============================================================================

/// Broad semantic classification of an optimization pass.
///
/// This classification is intentionally coarse. It is metadata used by the
/// planner, registry, diagnostics, and scheduling infrastructure; it does not
/// determine implementation behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PassKind {
    /// Purely inspects the circuit and produces no circuit transformation.
    Analysis,

    /// Performs local circuit simplification.
    LocalRewrite,

    /// Performs dependency/commutation-aware rewriting.
    StructuralRewrite,

    /// Performs algebraic transformation.
    AlgebraicRewrite,

    /// Performs target-independent synthesis.
    Synthesis,

    /// Performs target-aware synthesis/decomposition.
    TargetSynthesis,

    /// Optimizes fault-tolerant resources.
    FaultTolerant,

    /// Optimizes parameters or symbolic expressions.
    Parameter,

    /// Optimizes control-flow/regions/blocks.
    StructuralControlFlow,

    /// Uses stochastic or approximate search.
    Stochastic,

    /// Performs normalization/canonicalization.
    Normalization,

    /// Performs semantic verification or verification preparation.
    Verification,

    /// Composite pass invoking other passes.
    Composite,
}

impl PassKind {
    /// Returns the stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Analysis => "analysis",
            Self::LocalRewrite => "local_rewrite",
            Self::StructuralRewrite => "structural_rewrite",
            Self::AlgebraicRewrite => "algebraic_rewrite",
            Self::Synthesis => "synthesis",
            Self::TargetSynthesis => "target_synthesis",
            Self::FaultTolerant => "fault_tolerant",
            Self::Parameter => "parameter",
            Self::StructuralControlFlow => "structural_control_flow",
            Self::Stochastic => "stochastic",
            Self::Normalization => "normalization",
            Self::Verification => "verification",
            Self::Composite => "composite",
        }
    }
}

impl fmt::Display for PassKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Pass determinism
// =============================================================================

/// Determinism guarantees supplied by a pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PassDeterminism {
    /// The pass is deterministic for the same input/context configuration.
    Deterministic,

    /// The pass uses a deterministic seed supplied by the optimization
    /// context.
    Seeded,

    /// The pass may produce different results even under the same logical
    /// configuration.
    ///
    /// Such passes should normally be disabled by verified/reproducible
    /// compilation modes.
    Nondeterministic,
}

impl PassDeterminism {
    /// Returns true when the pass is deterministic without requiring a random
    /// seed.
    #[must_use]
    pub const fn is_deterministic(self) -> bool {
        matches!(self, Self::Deterministic)
    }

    /// Returns true when the pass can be reproduced using a supplied seed.
    #[must_use]
    pub const fn is_reproducible_with_seed(self) -> bool {
        matches!(self, Self::Deterministic | Self::Seeded)
    }

    /// Returns the stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Deterministic => "deterministic",
            Self::Seeded => "seeded",
            Self::Nondeterministic => "nondeterministic",
        }
    }
}

impl Default for PassDeterminism {
    fn default() -> Self {
        Self::Deterministic
    }
}

// =============================================================================
// Pass complexity
// =============================================================================

/// Coarse asymptotic complexity classification.
///
/// This metadata is not a substitute for actual runtime measurement. It gives
/// the planner enough information to avoid obviously unsuitable passes when
/// operating under constrained resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PassComplexity {
    /// Approximately linear in the number of operations.
    Linear,

    /// Approximately n log n.
    Linearithmic,

    /// Approximately quadratic.
    Quadratic,

    /// Approximately cubic.
    Cubic,

    /// Potentially exponential in the worst case.
    Exponential,

    /// Complexity depends on a search space and cannot be summarized by one
    /// standard asymptotic class.
    Search,

    /// Complexity is target/implementation dependent.
    TargetDependent,
}

impl PassComplexity {
    /// Returns a conservative numeric rank.
    ///
    /// Higher values represent potentially more expensive work.
    #[must_use]
    pub const fn rank(self) -> u8 {
        match self {
            Self::Linear => 0,
            Self::Linearithmic => 1,
            Self::Quadratic => 2,
            Self::Cubic => 3,
            Self::TargetDependent => 4,
            Self::Search => 5,
            Self::Exponential => 6,
        }
    }

    /// Returns true when the pass can be considered cheap enough for normal
    /// pipelines without special planning.
    #[must_use]
    pub const fn is_low_cost(self) -> bool {
        matches!(self, Self::Linear | Self::Linearithmic)
    }

    /// Returns the stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Linear => "linear",
            Self::Linearithmic => "linearithmic",
            Self::Quadratic => "quadratic",
            Self::Cubic => "cubic",
            Self::Exponential => "exponential",
            Self::Search => "search",
            Self::TargetDependent => "target_dependent",
        }
    }
}

impl Default for PassComplexity {
    fn default() -> Self {
        Self::Linear
    }
}

// =============================================================================
// Pass scope
// =============================================================================

/// Describes the circuit scope a pass may inspect or transform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PassScope {
    /// A pass operates independently on individual operations.
    Operation,

    /// A pass primarily examines local operation windows.
    LocalWindow,

    /// A pass may inspect a complete logical region.
    Region,

    /// A pass may inspect the complete circuit.
    Circuit,

    /// A pass may operate across multiple circuit regions.
    Global,
}

impl PassScope {
    /// Returns true if the pass may inspect the entire circuit.
    #[must_use]
    pub const fn is_global(self) -> bool {
        matches!(self, Self::Circuit | Self::Global)
    }

    /// Returns the stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Operation => "operation",
            Self::LocalWindow => "local_window",
            Self::Region => "region",
            Self::Circuit => "circuit",
            Self::Global => "global",
        }
    }
}

impl Default for PassScope {
    fn default() -> Self {
        Self::Circuit
    }
}

// =============================================================================
// Pass capability
// =============================================================================

/// Capabilities declared by an optimization pass.
///
/// Capabilities describe *what the pass can do*, not whether the planner will
/// select it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PassCapability {
    /// Can remove operations.
    RemovesOperations,

    /// Can add operations.
    AddsOperations,

    /// Can replace operations.
    ReplacesOperations,

    /// Can change operation ordering.
    ReordersOperations,

    /// Can combine multiple operations.
    FusesOperations,

    /// Can split/decompose operations.
    DecomposesOperations,

    /// Can change gate arity.
    ChangesArity,

    /// Can introduce ancilla qubits.
    IntroducesAncillas,

    /// Can eliminate/reuse ancillas.
    EliminatesAncillas,

    /// Can change logical qubit usage.
    ChangesQubitUsage,

    /// Can change circuit depth.
    ChangesDepth,

    /// Can change gate count.
    ChangesGateCount,

    /// Can change two-qubit operation count.
    ChangesTwoQubitCount,

    /// Can change fault-tolerant resource counts.
    ChangesFaultTolerantCost,

    /// Uses symbolic parameter transformations.
    ChangesParameters,

    /// Can move operations across dependency boundaries when legal.
    UsesCommutation,

    /// Uses an algebraic representation.
    UsesAlgebra,

    /// Uses synthesis.
    UsesSynthesis,

    /// Uses stochastic search.
    UsesRandomness,

    /// May produce an approximation rather than exact equivalence.
    Approximate,

    /// Can operate only when a particular target is available.
    TargetAware,

    /// Can operate on classical/quantum control-flow structures.
    ControlFlowAware,

    /// Performs no semantic transformation.
    AnalysisOnly,
}

// =============================================================================
// Analysis requirement
// =============================================================================

/// Stable requirement declaration for a pass.
///
/// The string-backed representation is intentional. `analysis.rs` and future
/// analysis modules can introduce concrete analysis types without requiring
/// this foundational pass contract to be rewritten.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnalysisRequirement {
    identifier: String,
    /// Whether the pass requires a fresh analysis rather than merely any
    /// cached generation.
    freshness: AnalysisFreshness,
}

impl AnalysisRequirement {
    /// Creates an analysis requirement.
    pub fn new(identifier: impl Into<String>) -> Result<Self, PassMetadataError> {
        let identifier = identifier.into();

        if identifier.trim().is_empty() {
            return Err(PassMetadataError::EmptyIdentifier {
                field: "analysis requirement",
            });
        }

        Ok(Self {
            identifier,
            freshness: AnalysisFreshness::CurrentGeneration,
        })
    }

    /// Creates a requirement with an explicit freshness policy.
    pub fn with_freshness(
        identifier: impl Into<String>,
        freshness: AnalysisFreshness,
    ) -> Result<Self, PassMetadataError> {
        let mut requirement = Self::new(identifier)?;
        requirement.freshness = freshness;
        Ok(requirement)
    }

    /// Returns the stable analysis identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Returns the freshness requirement.
    #[must_use]
    pub const fn freshness(&self) -> AnalysisFreshness {
        self.freshness
    }
}

/// Defines how current an analysis must be before a pass runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnalysisFreshness {
    /// Any valid cached generation is acceptable.
    Any,

    /// The analysis must correspond to the current circuit generation.
    CurrentGeneration,

    /// The analysis must be recomputed immediately before this pass.
    Recompute,
}

// =============================================================================
// Analysis invalidation
// =============================================================================

/// Stable declaration of an analysis invalidated by a pass.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnalysisInvalidation {
    identifier: String,
}

impl AnalysisInvalidation {
    /// Creates an invalidation declaration.
    pub fn new(identifier: impl Into<String>) -> Result<Self, PassMetadataError> {
        let identifier = identifier.into();

        if identifier.trim().is_empty() {
            return Err(PassMetadataError::EmptyIdentifier {
                field: "analysis invalidation",
            });
        }

        Ok(Self { identifier })
    }

    /// Returns the analysis identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }
}

// =============================================================================
// Property declarations
// =============================================================================

/// Stable optimizer property identifier.
///
/// Properties allow future passes to declare semantic facts they require or
/// preserve without coupling this file to a concrete analysis implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PropertyIdentifier(String);

impl PropertyIdentifier {
    /// Creates a property identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, PassMetadataError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(PassMetadataError::EmptyIdentifier {
                field: "property identifier",
            });
        }

        Ok(Self(value))
    }

    /// Returns the property identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for PropertyIdentifier {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for PropertyIdentifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Property required by a pass.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PropertyRequirement {
    identifier: PropertyIdentifier,
}

impl PropertyRequirement {
    /// Creates a property requirement.
    pub fn new(
        identifier: impl Into<String>,
    ) -> Result<Self, PassMetadataError> {
        Ok(Self {
            identifier: PropertyIdentifier::new(identifier)?,
        })
    }

    /// Returns the identifier.
    #[must_use]
    pub fn identifier(&self) -> &PropertyIdentifier {
        &self.identifier
    }
}

/// Property invalidated by a pass.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PropertyInvalidation {
    identifier: PropertyIdentifier,
}

impl PropertyInvalidation {
    /// Creates a property invalidation.
    pub fn new(
        identifier: impl Into<String>,
    ) -> Result<Self, PassMetadataError> {
        Ok(Self {
            identifier: PropertyIdentifier::new(identifier)?,
        })
    }

    /// Returns the identifier.
    #[must_use]
    pub fn identifier(&self) -> &PropertyIdentifier {
        &self.identifier
    }
}

// =============================================================================
// Pass requirements
// =============================================================================

/// All scheduler-visible requirements of a pass.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PassRequirements {
    analyses: Vec<AnalysisRequirement>,
    properties: Vec<PropertyRequirement>,
}

impl PassRequirements {
    /// Creates empty requirements.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            analyses: Vec::new(),
            properties: Vec::new(),
        }
    }

    /// Adds one analysis requirement.
    pub fn require_analysis(
        &mut self,
        requirement: AnalysisRequirement,
    ) -> &mut Self {
        self.analyses.push(requirement);
        self
    }

    /// Adds one property requirement.
    pub fn require_property(
        &mut self,
        requirement: PropertyRequirement,
    ) -> &mut Self {
        self.properties.push(requirement);
        self
    }

    /// Returns required analyses.
    #[must_use]
    pub fn analyses(&self) -> &[AnalysisRequirement] {
        &self.analyses
    }

    /// Returns required properties.
    #[must_use]
    pub fn properties(&self) -> &[PropertyRequirement] {
        &self.properties
    }

    /// Returns true when there are no requirements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.analyses.is_empty() && self.properties.is_empty()
    }
}

// =============================================================================
// Pass effects
// =============================================================================

/// Scheduler-visible effects of a pass.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PassEffects {
    invalidated_analyses: Vec<AnalysisInvalidation>,
    invalidated_properties: Vec<PropertyInvalidation>,
}

impl PassEffects {
    /// Creates empty effects.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            invalidated_analyses: Vec::new(),
            invalidated_properties: Vec::new(),
        }
    }

    /// Declares an analysis invalidated by the pass.
    pub fn invalidate_analysis(
        &mut self,
        invalidation: AnalysisInvalidation,
    ) -> &mut Self {
        self.invalidated_analyses.push(invalidation);
        self
    }

    /// Declares a property invalidated by the pass.
    pub fn invalidate_property(
        &mut self,
        invalidation: PropertyInvalidation,
    ) -> &mut Self {
        self.invalidated_properties.push(invalidation);
        self
    }

    /// Returns invalidated analyses.
    #[must_use]
    pub fn invalidated_analyses(&self) -> &[AnalysisInvalidation] {
        &self.invalidated_analyses
    }

    /// Returns invalidated properties.
    #[must_use]
    pub fn invalidated_properties(&self) -> &[PropertyInvalidation] {
        &self.invalidated_properties
    }

    /// Returns true when the pass declares no invalidations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.invalidated_analyses.is_empty()
            && self.invalidated_properties.is_empty()
    }
}

// =============================================================================
// Pass metadata
// =============================================================================

/// Immutable metadata describing an optimization pass.
///
/// The registry, planner, scheduler, provenance system, diagnostics, and
/// pipeline can consume this metadata without knowing the pass implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassMetadata {
    id: PassId,
    name: String,
    description: String,
    kind: PassKind,
    scope: PassScope,
    complexity: PassComplexity,
    determinism: PassDeterminism,
    requirements: PassRequirements,
    effects: PassEffects,
    capabilities: Vec<PassCapability>,
    semantic_preserving: bool,
    supports_empty_circuit: bool,
    supports_single_operation: bool,
    supports_large_circuits: bool,
    requires_target: bool,
    requires_verification: bool,
    fixed_point_safe: bool,
}

impl PassMetadata {
    /// Creates pass metadata with conservative production defaults.
    ///
    /// The builder methods should be used to describe specialized behavior.
    pub fn new(
        id: PassId,
        name: impl Into<String>,
        kind: PassKind,
    ) -> Result<Self, PassMetadataError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(PassMetadataError::EmptyIdentifier {
                field: "pass name",
            });
        }

        Ok(Self {
            id,
            name,
            description: String::new(),
            kind,
            scope: PassScope::Circuit,
            complexity: PassComplexity::Linear,
            determinism: PassDeterminism::Deterministic,
            requirements: PassRequirements::new(),
            effects: PassEffects::new(),
            capabilities: Vec::new(),
            semantic_preserving: true,
            supports_empty_circuit: true,
            supports_single_operation: true,
            supports_large_circuits: true,
            requires_target: false,
            requires_verification: false,
            fixed_point_safe: false,
        })
    }

    /// Sets a human-readable description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, PassMetadataError> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err(PassMetadataError::EmptyIdentifier {
                field: "pass description",
            });
        }

        self.description = description;
        Ok(self)
    }

    /// Sets the circuit scope.
    #[must_use]
    pub const fn with_scope(mut self, scope: PassScope) -> Self {
        self.scope = scope;
        self
    }

    /// Sets the complexity classification.
    #[must_use]
    pub const fn with_complexity(
        mut self,
        complexity: PassComplexity,
    ) -> Self {
        self.complexity = complexity;
        self
    }

    /// Sets the determinism classification.
    #[must_use]
    pub const fn with_determinism(
        mut self,
        determinism: PassDeterminism,
    ) -> Self {
        self.determinism = determinism;
        self
    }

    /// Replaces the requirement declaration.
    #[must_use]
    pub fn with_requirements(
        mut self,
        requirements: PassRequirements,
    ) -> Self {
        self.requirements = requirements;
        self
    }

    /// Replaces the effect declaration.
    #[must_use]
    pub fn with_effects(mut self, effects: PassEffects) -> Self {
        self.effects = effects;
        self
    }

    /// Adds a capability.
    #[must_use]
    pub fn with_capability(mut self, capability: PassCapability) -> Self {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
        }
        self
    }

    /// Adds multiple capabilities.
    #[must_use]
    pub fn with_capabilities(
        mut self,
        capabilities: impl IntoIterator<Item = PassCapability>,
    ) -> Self {
        for capability in capabilities {
            if !self.capabilities.contains(&capability) {
                self.capabilities.push(capability);
            }
        }
        self
    }

    /// Marks whether the pass is semantically preserving.
    #[must_use]
    pub const fn with_semantic_preservation(
        mut self,
        semantic_preserving: bool,
    ) -> Self {
        self.semantic_preserving = semantic_preserving;
        self
    }

    /// Sets whether an empty circuit is supported.
    #[must_use]
    pub const fn supports_empty_circuit(
        mut self,
        supported: bool,
    ) -> Self {
        self.supports_empty_circuit = supported;
        self
    }

    /// Sets whether a one-operation circuit is supported.
    #[must_use]
    pub const fn supports_single_operation(
        mut self,
        supported: bool,
    ) -> Self {
        self.supports_single_operation = supported;
        self
    }

    /// Sets whether large circuits are supported.
    #[must_use]
    pub const fn supports_large_circuits(
        mut self,
        supported: bool,
    ) -> Self {
        self.supports_large_circuits = supported;
        self
    }

    /// Marks whether a target is required.
    #[must_use]
    pub const fn requires_target(mut self, required: bool) -> Self {
        self.requires_target = required;
        self
    }

    /// Marks whether external semantic verification is required.
    #[must_use]
    pub const fn requires_verification(mut self, required: bool) -> Self {
        self.requires_verification = required;
        self
    }

    /// Marks whether the pass is safe for fixed-point repetition.
    #[must_use]
    pub const fn fixed_point_safe(mut self, safe: bool) -> Self {
        self.fixed_point_safe = safe;
        self
    }

    /// Returns the stable pass identifier.
    #[must_use]
    pub fn id(&self) -> &PassId {
        &self.id
    }

    /// Returns the pass identifier as text.
    #[must_use]
    pub fn id_str(&self) -> &str {
        self.id.as_str()
    }

    /// Returns the display name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the pass kind.
    #[must_use]
    pub const fn kind(&self) -> PassKind {
        self.kind
    }

    /// Returns the circuit scope.
    #[must_use]
    pub const fn scope(&self) -> PassScope {
        self.scope
    }

    /// Returns the complexity class.
    #[must_use]
    pub const fn complexity(&self) -> PassComplexity {
        self.complexity
    }

    /// Returns the determinism guarantee.
    #[must_use]
    pub const fn determinism(&self) -> PassDeterminism {
        self.determinism
    }

    /// Returns declared requirements.
    #[must_use]
    pub const fn requirements(&self) -> &PassRequirements {
        &self.requirements
    }

    /// Returns declared effects.
    #[must_use]
    pub const fn effects(&self) -> &PassEffects {
        &self.effects
    }

    /// Returns declared capabilities.
    #[must_use]
    pub fn capabilities(&self) -> &[PassCapability] {
        &self.capabilities
    }

    /// Returns true when the pass claims semantic preservation.
    #[must_use]
    pub const fn semantic_preserving(&self) -> bool {
        self.semantic_preserving
    }

    /// Returns whether empty circuits are supported.
    #[must_use]
    pub const fn supports_empty_circuit(&self) -> bool {
        self.supports_empty_circuit
    }

    /// Returns whether one-operation circuits are supported.
    #[must_use]
    pub const fn supports_single_operation(&self) -> bool {
        self.supports_single_operation
    }

    /// Returns whether large circuits are supported.
    #[must_use]
    pub const fn supports_large_circuits(&self) -> bool {
        self.supports_large_circuits
    }

    /// Returns whether a target is required.
    #[must_use]
    pub const fn requires_target(&self) -> bool {
        self.requires_target
    }

    /// Returns whether external verification is required.
    #[must_use]
    pub const fn requires_verification(&self) -> bool {
        self.requires_verification
    }

    /// Returns whether fixed-point repetition is safe.
    #[must_use]
    pub const fn fixed_point_safe(&self) -> bool {
        self.fixed_point_safe
    }

    /// Returns whether a capability is present.
    #[must_use]
    pub fn has_capability(&self, capability: PassCapability) -> bool {
        self.capabilities.contains(&capability)
    }

    /// Validates internal metadata invariants.
    pub fn validate(&self) -> PassMetadataResult<()> {
        if self.id.as_str().trim().is_empty() {
            return Err(PassMetadataError::EmptyIdentifier {
                field: "pass identifier",
            });
        }

        if self.name.trim().is_empty() {
            return Err(PassMetadataError::EmptyIdentifier {
                field: "pass name",
            });
        }

        if self.kind == PassKind::Analysis
            && !self.has_capability(PassCapability::AnalysisOnly)
        {
            return Err(PassMetadataError::InvalidCombination {
                message:
                    "analysis passes must declare AnalysisOnly capability",
            });
        }

        if self.determinism == PassDeterminism::Seeded
            && !self.has_capability(PassCapability::UsesRandomness)
        {
            return Err(PassMetadataError::InvalidCombination {
                message:
                    "seeded passes must declare UsesRandomness capability",
            });
        }

        if self.determinism == PassDeterminism::Nondeterministic
            && self.semantic_preserving
            && !self.has_capability(PassCapability::Approximate)
        {
            return Err(PassMetadataError::InvalidCombination {
                message:
                    "nondeterministic semantic-preserving passes must explicitly declare Approximate capability",
            });
        }

        if self.requires_target
            && !self.has_capability(PassCapability::TargetAware)
        {
            return Err(PassMetadataError::InvalidCombination {
                message:
                    "target-requiring passes must declare TargetAware capability",
            });
        }

        if self.requires_verification && !self.semantic_preserving {
            return Err(PassMetadataError::InvalidCombination {
                message:
                    "a non-semantic-preserving pass cannot require semantic verification",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Pass metadata errors
// =============================================================================

/// Errors produced while constructing or validating pass metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PassMetadataError {
    /// A required identifier is empty.
    EmptyIdentifier {
        /// Field that was empty.
        field: &'static str,
    },

    /// Two metadata settings conflict.
    InvalidCombination {
        /// Human-readable explanation.
        message: &'static str,
    },

    /// A pass identifier could not be constructed by the canonical error
    /// identifier layer.
    InvalidPassIdentifier {
        /// Human-readable explanation.
        message: String,
    },
}

impl fmt::Display for PassMetadataError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(formatter, "{field} must not be empty")
            }

            Self::InvalidCombination { message } => {
                formatter.write_str(message)
            }

            Self::InvalidPassIdentifier { message } => {
                write!(formatter, "invalid pass identifier: {message}")
            }
        }
    }
}

impl std::error::Error for PassMetadataError {}

// =============================================================================
// Pass change classification
// =============================================================================

/// Describes what happened to the circuit during one pass invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PassChange {
    /// The pass inspected the circuit but did not change it.
    Unchanged,

    /// The pass made at least one semantic-preserving transformation.
    Changed,

    /// The pass intentionally produced no circuit change because its search
    /// strategy found no acceptable candidate.
    NoImprovement,

    /// The pass was skipped by its own implementation because prerequisites
    /// were unavailable or the pass was not applicable.
    Skipped,

    /// The pass reached a configured resource/effort boundary while producing
    /// a valid candidate.
    LimitReached,

    /// The pass made a partial transformation before its work budget was
    /// reached.
    PartiallyChanged,
}

impl PassChange {
    /// Returns true when the circuit may have changed.
    #[must_use]
    pub const fn changed(self) -> bool {
        matches!(
            self,
            Self::Changed | Self::PartiallyChanged
        )
    }

    /// Returns true when the pass completed without an error.
    #[must_use]
    pub const fn completed(self) -> bool {
        !matches!(self, Self::Skipped)
    }
}

// =============================================================================
// Pass outcome
// =============================================================================

/// Execution outcome returned by an optimization pass.
///
/// This is deliberately smaller than `result::PassResult`. The latter is a
/// reporting/storage contract; this type is the immediate execution contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassOutcome {
    change: PassChange,
    operations_before: u64,
    operations_after: u64,
    operations_removed: u64,
    operations_added: u64,
    operations_replaced: u64,
    rewrites: u64,
    iterations: u64,
    analyses_requested: u64,
    verification_requested: bool,
    message: Option<String>,
}

impl PassOutcome {
    /// Creates an unchanged outcome.
    #[must_use]
    pub const fn unchanged(
        operations_before: u64,
        operations_after: u64,
    ) -> Self {
        Self {
            change: PassChange::Unchanged,
            operations_before,
            operations_after,
            operations_removed: 0,
            operations_added: 0,
            operations_replaced: 0,
            rewrites: 0,
            iterations: 0,
            analyses_requested: 0,
            verification_requested: false,
            message: None,
        }
    }

    /// Creates a changed outcome.
    #[must_use]
    pub const fn changed(
        operations_before: u64,
        operations_after: u64,
    ) -> Self {
        Self {
            change: PassChange::Changed,
            operations_before,
            operations_after,
            operations_removed: 0,
            operations_added: 0,
            operations_replaced: 0,
            rewrites: 0,
            iterations: 0,
            analyses_requested: 0,
            verification_requested: false,
            message: None,
        }
    }

    /// Creates a skipped outcome.
    #[must_use]
    pub const fn skipped(
        operations_before: u64,
        operations_after: u64,
    ) -> Self {
        Self {
            change: PassChange::Skipped,
            operations_before,
            operations_after,
            operations_removed: 0,
            operations_added: 0,
            operations_replaced: 0,
            rewrites: 0,
            iterations: 0,
            analyses_requested: 0,
            verification_requested: false,
            message: None,
        }
    }

    /// Creates a no-improvement outcome.
    #[must_use]
    pub const fn no_improvement(
        operations_before: u64,
        operations_after: u64,
    ) -> Self {
        Self {
            change: PassChange::NoImprovement,
            operations_before,
            operations_after,
            operations_removed: 0,
            operations_added: 0,
            operations_replaced: 0,
            rewrites: 0,
            iterations: 0,
            analyses_requested: 0,
            verification_requested: false,
            message: None,
        }
    }

    /// Sets the change classification.
    #[must_use]
    pub const fn with_change(mut self, change: PassChange) -> Self {
        self.change = change;
        self
    }

    /// Records operation removals.
    #[must_use]
    pub const fn with_operations_removed(
        mut self,
        count: u64,
    ) -> Self {
        self.operations_removed = count;
        self
    }

    /// Records operation additions.
    #[must_use]
    pub const fn with_operations_added(
        mut self,
        count: u64,
    ) -> Self {
        self.operations_added = count;
        self
    }

    /// Records operation replacements.
    #[must_use]
    pub const fn with_operations_replaced(
        mut self,
        count: u64,
    ) -> Self {
        self.operations_replaced = count;
        self
    }

    /// Records rewrite count.
    #[must_use]
    pub const fn with_rewrites(mut self, count: u64) -> Self {
        self.rewrites = count;
        self
    }

    /// Records iteration count.
    #[must_use]
    pub const fn with_iterations(mut self, count: u64) -> Self {
        self.iterations = count;
        self
    }

    /// Records analysis requests.
    #[must_use]
    pub const fn with_analyses_requested(
        mut self,
        count: u64,
    ) -> Self {
        self.analyses_requested = count;
        self
    }

    /// Marks whether semantic verification is requested.
    #[must_use]
    pub const fn with_verification_requested(
        mut self,
        requested: bool,
    ) -> Self {
        self.verification_requested = requested;
        self
    }

    /// Adds an informational message.
    #[must_use]
    pub fn with_message(
        mut self,
        message: impl Into<String>,
    ) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Returns the change classification.
    #[must_use]
    pub const fn change(&self) -> PassChange {
        self.change
    }

    /// Returns true if the circuit changed.
    #[must_use]
    pub const fn changed(&self) -> bool {
        self.change.changed()
    }

    /// Returns operations before execution.
    #[must_use]
    pub const fn operations_before(&self) -> u64 {
        self.operations_before
    }

    /// Returns operations after execution.
    #[must_use]
    pub const fn operations_after(&self) -> u64 {
        self.operations_after
    }

    /// Returns operations removed.
    #[must_use]
    pub const fn operations_removed(&self) -> u64 {
        self.operations_removed
    }

    /// Returns operations added.
    #[must_use]
    pub const fn operations_added(&self) -> u64 {
        self.operations_added
    }

    /// Returns operations replaced.
    #[must_use]
    pub const fn operations_replaced(&self) -> u64 {
        self.operations_replaced
    }

    /// Returns rewrites.
    #[must_use]
    pub const fn rewrites(&self) -> u64 {
        self.rewrites
    }

    /// Returns iterations.
    #[must_use]
    pub const fn iterations(&self) -> u64 {
        self.iterations
    }

    /// Returns analysis requests.
    #[must_use]
    pub const fn analyses_requested(&self) -> u64 {
        self.analyses_requested
    }

    /// Returns whether verification was requested.
    #[must_use]
    pub const fn verification_requested(&self) -> bool {
        self.verification_requested
    }

    /// Returns the optional message.
    #[must_use]
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Validates basic arithmetic consistency of the outcome.
    ///
    /// This does not validate circuit semantics. That remains the responsibility
    /// of the IR and verification subsystems.
    pub fn validate(&self) -> Result<(), PassOutcomeError> {
        let expected = self
            .operations_before
            .checked_add(self.operations_added)
            .ok_or(PassOutcomeError::ArithmeticOverflow)?;

        let expected = expected
            .checked_sub(self.operations_removed)
            .ok_or(PassOutcomeError::InvalidOperationAccounting)?;

        if expected < self.operations_replaced {
            return Err(PassOutcomeError::InvalidOperationAccounting);
        }

        if self.operations_after > 0
            && self.change == PassChange::Unchanged
            && self.operations_before != self.operations_after
        {
            return Err(PassOutcomeError::InvalidChangeClassification);
        }

        Ok(())
    }
}

// =============================================================================
// Pass outcome errors
// =============================================================================

/// Errors in the bookkeeping supplied by a pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassOutcomeError {
    /// Arithmetic overflow occurred while validating counters.
    ArithmeticOverflow,

    /// Added/removed/replaced operation counts cannot describe the reported
    /// result.
    InvalidOperationAccounting,

    /// The change classification contradicts the reported circuit sizes.
    InvalidChangeClassification,
}

impl fmt::Display for PassOutcomeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArithmeticOverflow => {
                formatter.write_str(
                    "optimization pass outcome counter overflow",
                )
            }

            Self::InvalidOperationAccounting => {
                formatter.write_str(
                    "optimization pass outcome contains inconsistent operation accounting",
                )
            }

            Self::InvalidChangeClassification => {
                formatter.write_str(
                    "optimization pass outcome contains inconsistent change classification",
                )
            }
        }
    }
}

impl std::error::Error for PassOutcomeError {}

// =============================================================================
// Pass execution policy
// =============================================================================

/// Execution policy hints for a pass.
///
/// These hints do not override the global optimization configuration or
/// resource limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PassExecutionPolicy {
    /// Run normally.
    Normal,

    /// Prefer early termination when no improvement is found.
    StopWhenStable,

    /// Permit fixed-point repetition.
    FixedPoint,

    /// Permit expensive search under an explicit optimizer budget.
    BudgetedSearch,

    /// Run only when semantic verification is enabled.
    VerifiedOnly,
}

impl Default for PassExecutionPolicy {
    fn default() -> Self {
        Self::Normal
    }
}

// =============================================================================
// Optimization pass trait
// =============================================================================

/// Production optimization pass interface.
///
/// Implementations should be stateless with respect to a particular optimizer
/// invocation. Invocation-specific mutable state belongs in
/// `OptimizationContext`.
///
/// The trait is object-safe and can therefore be used behind `dyn`.
pub trait OptimizationPass: Send + Sync {
    /// Returns immutable metadata describing this pass.
    fn metadata(&self) -> &PassMetadata;

    /// Executes the pass against the canonical Quantum IR.
    ///
    /// The pass must:
    ///
    /// - preserve canonical IR ownership;
    /// - obey optimizer resource policies;
    /// - avoid global mutable state;
    /// - avoid backend I/O;
    /// - avoid routing/scheduling responsibilities;
    /// - return a valid `PassOutcome`;
    /// - return `OptimizationError` for failures;
    /// - never use `unsafe`.
    ///
    /// The pipeline is responsible for:
    ///
    /// - validating prerequisites;
    /// - establishing pass lifecycle state;
    /// - analysis acquisition;
    /// - analysis invalidation;
    /// - fixed-point repetition;
    /// - verification;
    /// - provenance;
    /// - final result construction.
    fn run(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> PassExecutionResult;

    /// Returns the stable pass identifier.
    #[must_use]
    fn id(&self) -> &PassId {
        self.metadata().id()
    }

    /// Returns the human-readable pass name.
    #[must_use]
    fn name(&self) -> &str {
        self.metadata().name()
    }

    /// Returns the pass kind.
    #[must_use]
    fn kind(&self) -> PassKind {
        self.metadata().kind()
    }

    /// Returns declared scheduler requirements.
    #[must_use]
    fn requirements(&self) -> &PassRequirements {
        self.metadata().requirements()
    }

    /// Returns declared scheduler effects.
    #[must_use]
    fn effects(&self) -> &PassEffects {
        self.metadata().effects()
    }

    /// Returns declared pass capabilities.
    #[must_use]
    fn capabilities(&self) -> &[PassCapability] {
        self.metadata().capabilities()
    }

    /// Returns the pass complexity classification.
    #[must_use]
    fn complexity(&self) -> PassComplexity {
        self.metadata().complexity()
    }

    /// Returns the determinism guarantee.
    #[must_use]
    fn determinism(&self) -> PassDeterminism {
        self.metadata().determinism()
    }

    /// Returns the circuit scope.
    #[must_use]
    fn scope(&self) -> PassScope {
        self.metadata().scope()
    }

    /// Returns whether this pass is safe for fixed-point repetition.
    #[must_use]
    fn fixed_point_safe(&self) -> bool {
        self.metadata().fixed_point_safe()
    }

    /// Returns whether the pass is semantically preserving.
    #[must_use]
    fn semantic_preserving(&self) -> bool {
        self.metadata().semantic_preserving()
    }

    /// Performs pass metadata validation.
    fn validate(&self) -> PassMetadataResult<()> {
        self.metadata().validate()
    }

    /// Returns whether this pass has a particular capability.
    #[must_use]
    fn has_capability(&self, capability: PassCapability) -> bool {
        self.metadata().has_capability(capability)
    }

    /// Returns the execution policy hint.
    ///
    /// Implementations may override this when a pass has specialized lifecycle
    /// behavior.
    #[must_use]
    fn execution_policy(&self) -> PassExecutionPolicy {
        PassExecutionPolicy::Normal
    }
}

// =============================================================================
// Pass object helpers
// =============================================================================

/// Convenience wrapper for storing a pass together with an optional stable
/// registry origin.
///
/// This type is intentionally small and does not own pipeline state.
#[derive(Debug)]
pub struct OwnedPass {
    pass: Box<dyn OptimizationPass>,
}

impl OwnedPass {
    /// Creates an owned optimization pass.
    pub fn new<P>(pass: P) -> Result<Self, PassMetadataError>
    where
        P: OptimizationPass + 'static,
    {
        pass.validate()?;

        Ok(Self {
            pass: Box::new(pass),
        })
    }

    /// Creates an owned pass from an already boxed implementation.
    pub fn from_boxed(
        pass: Box<dyn OptimizationPass>,
    ) -> Result<Self, PassMetadataError> {
        pass.validate()?;

        Ok(Self { pass })
    }

    /// Returns the underlying pass.
    #[must_use]
    pub fn as_pass(&self) -> &dyn OptimizationPass {
        self.pass.as_ref()
    }

    /// Returns a mutable trait-object reference.
    #[must_use]
    pub fn as_pass_mut(&mut self) -> &mut dyn OptimizationPass {
        self.pass.as_mut()
    }

    /// Consumes the wrapper and returns the boxed pass.
    #[must_use]
    pub fn into_boxed(self) -> Box<dyn OptimizationPass> {
        self.pass
    }
}

// =============================================================================
// Pass collection helper
// =============================================================================

/// Validates a collection of passes before a pipeline is constructed.
///
/// This function deliberately does not execute passes or mutate a circuit.
pub fn validate_passes<'a, I>(
    passes: I,
) -> PassMetadataResult<()>
where
    I: IntoIterator<Item = &'a dyn OptimizationPass>,
{
    for pass in passes {
        pass.validate()?;
    }

    Ok(())
}

/// Returns whether all supplied passes are deterministic/reproducible.
#[must_use]
pub fn all_passes_reproducible<'a, I>(
    passes: I,
) -> bool
where
    I: IntoIterator<Item = &'a dyn OptimizationPass>,
{
    passes
        .into_iter()
        .all(|pass| pass.determinism().is_reproducible_with_seed())
}

/// Returns whether all supplied passes are strictly deterministic.
#[must_use]
pub fn all_passes_deterministic<'a, I>(
    passes: I,
) -> bool
where
    I: IntoIterator<Item = &'a dyn OptimizationPass>,
{
    passes
        .into_iter()
        .all(|pass| pass.determinism().is_deterministic())
}

/// Returns the highest declared complexity among a collection of passes.
#[must_use]
pub fn maximum_pass_complexity<'a, I>(
    passes: I,
) -> Option<PassComplexity>
where
    I: IntoIterator<Item = &'a dyn OptimizationPass>,
{
    passes
        .into_iter()
        .map(OptimizationPass::complexity)
        .max_by_key(|complexity| complexity.rank())
}

// =============================================================================
// Test-only minimal pass
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct NoOpPass {
        metadata: PassMetadata,
    }

    impl NoOpPass {
        fn new() -> Self {
            let id = PassIdentifier::new("test.noop")
                .expect("static test pass identifier is valid");

            let metadata = PassMetadata::new(
                id,
                "Test No-Op",
                PassKind::Analysis,
            )
            .expect("static test metadata is valid")
            .with_scope(PassScope::Circuit)
            .with_complexity(PassComplexity::Linear)
            .with_capability(PassCapability::AnalysisOnly);

            Self { metadata }
        }
    }

    impl OptimizationPass for NoOpPass {
        fn metadata(&self) -> &PassMetadata {
            &self.metadata
        }

        fn run(
            &self,
            circuit: &mut QuantumCircuit,
            _context: &mut OptimizationContext,
        ) -> PassExecutionResult {
            let operations = circuit
                .operations()
                .len()
                .try_into()
                .map_err(|_| OptimizationError::internal(
                    "test operation count does not fit u64",
                ))?;

            Ok(PassOutcome::unchanged(
                operations,
                operations,
            ))
        }
    }

    #[test]
    fn pass_metadata_is_valid() {
        let pass = NoOpPass::new();

        assert!(pass.validate().is_ok());
        assert_eq!(pass.kind(), PassKind::Analysis);
        assert!(pass.has_capability(PassCapability::AnalysisOnly));
    }

    #[test]
    fn pass_id_is_stable() {
        let pass = NoOpPass::new();

        assert_eq!(pass.id().as_str(), "test.noop");
    }

    #[test]
    fn outcome_accounting_is_valid() {
        let outcome = PassOutcome::changed(10, 8)
            .with_operations_removed(2)
            .with_rewrites(1);

        assert!(outcome.validate().is_ok());
        assert!(outcome.changed());
        assert_eq!(outcome.operations_removed(), 2);
    }

    #[test]
    fn unchanged_outcome_is_not_changed() {
        let outcome = PassOutcome::unchanged(10, 10);

        assert!(!outcome.changed());
        assert_eq!(outcome.change(), PassChange::Unchanged);
        assert!(outcome.validate().is_ok());
    }

    #[test]
    fn metadata_capabilities_are_deduplicated() {
        let id = PassIdentifier::new("test.capabilities")
            .expect("static identifier is valid");

        let metadata = PassMetadata::new(
            id,
            "Capability Test",
            PassKind::LocalRewrite,
        )
        .expect("metadata is valid")
        .with_capability(PassCapability::RemovesOperations)
        .with_capability(PassCapability::RemovesOperations);

        assert_eq!(
            metadata
                .capabilities()
                .iter()
                .filter(|capability| {
                    **capability == PassCapability::RemovesOperations
                })
                .count(),
            1
        );
    }

    #[test]
    fn deterministic_pass_is_reproducible() {
        let id = PassIdentifier::new("test.deterministic")
            .expect("static identifier is valid");

        let metadata = PassMetadata::new(
            id,
            "Deterministic Test",
            PassKind::Normalization,
        )
        .expect("metadata is valid");

        assert!(metadata.determinism().is_deterministic());
        assert!(metadata.determinism().is_reproducible_with_seed());
    }

    #[test]
    fn seeded_pass_requires_randomness_capability() {
        let id = PassIdentifier::new("test.seeded")
            .expect("static identifier is valid");

        let metadata = PassMetadata::new(
            id,
            "Seeded Test",
            PassKind::Stochastic,
        )
        .expect("metadata is valid")
        .with_determinism(PassDeterminism::Seeded);

        assert!(matches!(
            metadata.validate(),
            Err(PassMetadataError::InvalidCombination { .. })
        ));
    }

    #[test]
    fn requirement_identifiers_must_not_be_empty() {
        let result = AnalysisRequirement::new("");

        assert!(matches!(
            result,
            Err(PassMetadataError::EmptyIdentifier {
                field: "analysis requirement"
            })
        ));
    }

    #[test]
    fn property_identifiers_must_not_be_empty() {
        let result = PropertyIdentifier::new("");

        assert!(matches!(
            result,
            Err(PassMetadataError::EmptyIdentifier {
                field: "property identifier"
            })
        ));
    }

    #[test]
    fn pass_collection_helpers_work() {
        let pass = NoOpPass::new();

        assert!(validate_passes([&pass as &dyn OptimizationPass]).is_ok());
        assert!(all_passes_deterministic([
            &pass as &dyn OptimizationPass
        ]));
        assert!(all_passes_reproducible([
            &pass as &dyn OptimizationPass
        ]));
        assert_eq!(
            maximum_pass_complexity([
                &pass as &dyn OptimizationPass
            ]),
            Some(PassComplexity::Linear)
        );
    }
}