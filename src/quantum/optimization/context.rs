//! Zamani Quantum Optimization — Optimization Context
//!
//! Production execution context for one logical quantum optimization run.
//!
//! # Architectural role
//!
//! `OptimizationContext` is the shared, invocation-scoped state boundary
//! between the optimizer infrastructure and individual optimization passes.
//!
//! The canonical dependency direction is:
//!
//! ```text
//!                     quantum::ir
//!                         │
//!                         ▼
//!               OptimizationContext
//!                         │
//!       ┌─────────────────┼──────────────────┐
//!       │                 │                  │
//!       ▼                 ▼                  ▼
//!    analyses          passes            rewrite engine
//!       │                 │                  │
//!       └─────────────────┼──────────────────┘
//!                         ▼
//!                     pipeline
//!                         │
//!                         ▼
//!                  optimized quantum::ir
//! ```
//!
//! The context does NOT own a quantum circuit and does NOT become another IR.
//! The circuit remains owned by `quantum::ir::QuantumCircuit` and is passed to
//! passes explicitly.
//!
//! # Responsibilities
//!
//! This module owns:
//!
//! - invocation-scoped optimizer state;
//! - immutable optimization configuration;
//! - immutable optimization resource limits;
//! - deterministic/randomization policy state;
//! - pass execution counters;
//! - iteration counters;
//! - rewrite counters;
//! - analysis work counters;
//! - verification work counters;
//! - synthesis work counters;
//! - candidate/matching counters;
//! - provenance budget accounting;
//! - wall-clock deadline bookkeeping;
//! - analysis-cache storage;
//! - generic optimizer service storage;
//! - analysis invalidation generations;
//! - pass-local scratch storage;
//! - current-pass identity;
//! - current pipeline stage identity;
//! - deterministic seed derivation;
//! - cancellation/deadline checks;
//! - safe extensibility points for future optimization modules.
//!
//! # Deliberate non-ownership
//!
//! `OptimizationContext` does NOT own:
//!
//! - `QuantumCircuit`;
//! - `QuantumGate` or another gate representation;
//! - frontend parsing;
//! - routing;
//! - scheduling;
//! - hardware APIs;
//! - QPU execution;
//! - benchmarking;
//! - QEC semantics;
//! - optimization rules;
//! - pass implementations;
//! - target definitions;
//! - cost-model definitions;
//! - final optimization results.
//!
//! Those responsibilities belong to their owning modules.
//!
//! # Canonical IR
//!
//! All circuit data remains in:
//!
//! `crate::quantum::ir`
//!
//! In particular, this file deliberately does not define a replacement
//! `QuantumGate`, `QuantumOperation`, `Circuit`, or equivalent structure.
//!
//! The existing Quantum IR explicitly defines itself as the canonical,
//! hardware-independent representation of logical quantum programs and states
//! that optimization algorithms do not belong inside the IR itself.
//!
//! # Future-module integration
//!
//! This file intentionally provides generic typed storage for components that
//! will be implemented by later optimizer modules.
//!
//! Future modules integrate as follows:
//!
//! ```text
//! config.rs
//!     └── OptimizationConfig
//!
//! limits.rs
//!     └── OptimizationLimits
//!
//! cost.rs
//!     └── CostModel / cost services
//!
//! targets/*
//!     └── target definitions
//!
//! analysis/*
//!     └── cached analysis values
//!
//! provenance.rs
//!     └── provenance service/state
//!
//! statistics.rs
//!     └── statistics service/state
//!
//! pass.rs
//!     └── current pass identity
//!
//! pipeline.rs
//!     └── pipeline/stage state
//!
//! rewrite.rs
//!     └── rewrite accounting
//!
//! verification/*
//!     └── verification accounting
//!
//! planner.rs
//!     └── planning metadata
//! ```
//!
//! None of those future modules should need to modify this file merely because
//! another analysis, target implementation, optimizer pass, or cost model is
//! added.
//!
//! # Extensibility strategy
//!
//! The context uses safe `Any`-based typed storage for optional services and
//! analysis values.
//!
//! This gives later modules:
//!
//! ```text
//! context.insert_service(value);
//! context.service::<ConcreteType>();
//!
//! context.insert_analysis(value);
//! context.analysis::<ConcreteAnalysis>();
//! ```
//!
//! without creating a dependency from this foundational file onto every future
//! optimizer module.
//!
//! This is important for the requested "finish one file and do not rewrite it
//! when another file is implemented" workflow.
//!
//! # Thread safety
//!
//! The context itself contains no global mutable state.
//!
//! Values inserted into typed stores must be `Send + Sync` so that the context
//! can safely participate in future parallel optimization infrastructure.
//!
//! The context does not internally spawn threads. Parallelism belongs to the
//! pipeline/scheduler.
//!
//! # Determinism
//!
//! The context provides deterministic seed derivation.
//!
//! Randomized passes must never silently use ambient process randomness when
//! deterministic mode is requested.
//!
//! The derived seed is based on:
//!
//! - configured deterministic seed;
//! - pass identity;
//! - invocation-local counter.
//!
//! The algorithm is intentionally simple and stable. It is not cryptographic.
//!
//! # Safety
//!
//! This module contains no `unsafe` code.
//!
//! No unsafe downcasts are used.
//!
//! No global mutable state is used.
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
//!
//! # Scaling
//!
//! The context is designed for circuits ranging from tiny examples to very
//! large workloads subject to available memory/CPU and the explicit
//! `OptimizationLimits` policy.
//!
//! It does not use recursion for ordinary context bookkeeping.
//!
//! Counters are `u64` and all counter arithmetic is checked.
//!
//! Memory growth occurs only when callers explicitly insert analyses/services
//! or allocate pass-local scratch data.
//!
//! The context itself therefore does not impose an artificial circuit-size
//! ceiling beyond the optimizer's configured limits.
//!
//! # Important distinction
//!
//! There are two resource systems:
//!
//! ```text
//! quantum::ir::QuantumIrLimits
//!     │
//!     └── protects canonical IR resources
//!
//! quantum::optimization::OptimizationLimits
//!     │
//!     └── protects optimizer work
//! ```
//!
//! This context stores the second one.
//!
//! # Example
//!
//! ```ignore
//! let mut context = OptimizationContext::new(config, limits)?;
//!
//! context.begin_pass("local.cancellation")?;
//! context.record_rewrite()?;
//! context.record_analysis_step()?;
//! context.end_pass();
//!
//! if context.should_stop() {
//!     // Pipeline decides whether to stop, fail, or return the best result.
//! }
//! ```
//!
//! The exact pass/pipeline result semantics are intentionally owned by
//! `pass.rs` and `pipeline.rs`.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, Instant};

use crate::quantum::optimization::config::{
    Determinism,
    OptimizationConfig,
};
use crate::quantum::optimization::limits::{
    LimitPolicy,
    OptimizationLimits,
    OptimizationLimitsError,
    OptimizationResource,
};

// =============================================================================
// Public result type
// =============================================================================

/// Result type for optimizer-context operations.
pub type OptimizationContextResult<T> = Result<T, OptimizationContextError>;

// =============================================================================
// Context errors
// =============================================================================

/// Errors produced by `OptimizationContext`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptimizationContextError {
    /// A counter would overflow its `u64` representation.
    CounterOverflow {
        /// Counter/resource that overflowed.
        resource: &'static str,
    },

    /// An optimizer resource limit was reached.
    ResourceLimitExceeded {
        /// Resource that exceeded its configured budget.
        resource: OptimizationResource,

        /// Current/requested amount.
        requested: u64,

        /// Maximum permitted amount.
        maximum: u64,
    },

    /// The context was used after its wall-clock deadline.
    DeadlineExceeded {
        /// Configured wall-clock budget.
        budget_millis: u64,
    },

    /// A pass was already active when another pass attempted to begin.
    PassAlreadyActive {
        /// Existing pass identifier.
        pass_id: String,
    },

    /// A pass was ended when no pass was active.
    NoActivePass,

    /// A supplied pass identifier is empty.
    EmptyPassId,

    /// A supplied pipeline stage identifier is empty.
    EmptyStageId,

    /// A typed value was requested but the requested type is not present.
    MissingTypedValue {
        /// Stable storage category.
        category: &'static str,

        /// Rust type name.
        type_name: &'static str,
    },

    /// A typed value exists under a different concrete type.
    TypeMismatch {
        /// Stable storage category.
        category: &'static str,

        /// Rust type name.
        type_name: &'static str,
    },

    /// An attempt was made to insert an already-present typed value without
    /// explicitly replacing it.
    DuplicateTypedValue {
        /// Stable storage category.
        category: &'static str,

        /// Rust type name.
        type_name: &'static str,
    },

    /// The optimizer configuration is invalid for a context.
    InvalidConfiguration {
        /// Human-readable reason.
        message: String,
    },

    /// The optimizer limits configuration is invalid.
    InvalidLimits {
        /// Human-readable reason.
        message: String,
    },

    /// An operation requires deterministic mode but the current context is
    /// explicitly nondeterministic.
    DeterminismViolation {
        /// Human-readable reason.
        message: &'static str,
    },

    /// An operation requires a pass to be active.
    NoActivePassForOperation {
        /// Operation requiring an active pass.
        operation: &'static str,
    },

    /// A resource request is negative in an API that conceptually requires
    /// non-negative work. This variant exists for future FFI/adapter safety.
    NegativeResourceRequest {
        /// Resource name.
        resource: &'static str,
    },

    /// The context has been explicitly cancelled.
    Cancelled,

    /// An internal state transition was invalid.
    InvalidState {
        /// Static description.
        message: &'static str,
    },
}

impl fmt::Display for OptimizationContextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CounterOverflow { resource } => {
                write!(
                    formatter,
                    "optimization context counter overflow for `{resource}`"
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "optimization resource limit exceeded for `{resource}`: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::DeadlineExceeded { budget_millis } => {
                write!(
                    formatter,
                    "optimization wall-clock deadline exceeded after {budget_millis} ms"
                )
            }

            Self::PassAlreadyActive { pass_id } => {
                write!(
                    formatter,
                    "optimization pass `{pass_id}` is already active"
                )
            }

            Self::NoActivePass => {
                formatter.write_str("no optimization pass is currently active")
            }

            Self::EmptyPassId => {
                formatter.write_str("optimization pass identifier must not be empty")
            }

            Self::EmptyStageId => {
                formatter.write_str("optimization pipeline stage identifier must not be empty")
            }

            Self::MissingTypedValue {
                category,
                type_name,
            } => {
                write!(
                    formatter,
                    "no {category} value of type `{type_name}` is registered"
                )
            }

            Self::TypeMismatch {
                category,
                type_name,
            } => {
                write!(
                    formatter,
                    "registered {category} value does not match requested type `{type_name}`"
                )
            }

            Self::DuplicateTypedValue {
                category,
                type_name,
            } => {
                write!(
                    formatter,
                    "{category} value of type `{type_name}` is already registered"
                )
            }

            Self::InvalidConfiguration { message } => {
                write!(
                    formatter,
                    "invalid optimization configuration: {message}"
                )
            }

            Self::InvalidLimits { message } => {
                write!(
                    formatter,
                    "invalid optimization limits: {message}"
                )
            }

            Self::DeterminismViolation { message } => {
                write!(
                    formatter,
                    "optimization determinism violation: {message}"
                )
            }

            Self::NoActivePassForOperation { operation } => {
                write!(
                    formatter,
                    "optimization operation `{operation}` requires an active pass"
                )
            }

            Self::NegativeResourceRequest { resource } => {
                write!(
                    formatter,
                    "negative resource request is invalid for `{resource}`"
                )
            }

            Self::Cancelled => {
                formatter.write_str("optimization was cancelled")
            }

            Self::InvalidState { message } => {
                write!(
                    formatter,
                    "invalid optimization context state: {message}"
                )
            }
        }
    }
}

impl std::error::Error for OptimizationContextError {}

impl From<OptimizationLimitsError> for OptimizationContextError {
    fn from(error: OptimizationLimitsError) -> Self {
        match error {
            OptimizationLimitsError::InvalidConfiguration {
                field,
                value,
            } => Self::InvalidLimits {
                message: format!(
                    "field `{field}` has invalid value {value}"
                ),
            },

            OptimizationLimitsError::ResourceExceeded {
                resource,
                requested,
                maximum,
            } => Self::ResourceLimitExceeded {
                resource: resource_from_name(resource),
                requested,
                maximum,
            },

            OptimizationLimitsError::ArithmeticOverflow {
                resource,
            } => Self::CounterOverflow { resource },

            OptimizationLimitsError::ArithmeticMultiplicationOverflow {
                resource,
            } => Self::CounterOverflow { resource },
        }
    }
}

// =============================================================================
// Resource-name compatibility
// =============================================================================

/// Converts the stable string identifiers used by `OptimizationLimits` into
/// the strongly typed resource enum used by the context.
///
/// Unknown future identifiers are mapped conservatively to `Passes`. This
/// function is private because callers should use `OptimizationResource`.
fn resource_from_name(name: &'static str) -> OptimizationResource {
    match name {
        "passes" => OptimizationResource::Passes,
        "iterations" => OptimizationResource::Iterations,
        "rewrites" => OptimizationResource::Rewrites,
        "circuit_operations" => OptimizationResource::CircuitOperations,
        "circuit_qubits" => OptimizationResource::CircuitQubits,
        "analysis_steps" => OptimizationResource::AnalysisSteps,
        "dependency_edges" => OptimizationResource::DependencyEdges,
        "egraph_nodes" => OptimizationResource::EGraphNodes,
        "egraph_classes" => OptimizationResource::EGraphClasses,
        "synthesis_steps" => OptimizationResource::SynthesisSteps,
        "synthesis_states" => OptimizationResource::SynthesisStates,
        "synthesis_operations" => OptimizationResource::SynthesisOperations,
        "verification_operations" => OptimizationResource::VerificationOperations,
        "verification_qubits" => OptimizationResource::VerificationQubits,
        "verification_states" => OptimizationResource::VerificationStates,
        "verification_samples" => OptimizationResource::VerificationSamples,
        "rewrite_candidates" => OptimizationResource::RewriteCandidates,
        "match_candidates" => OptimizationResource::MatchCandidates,
        "provenance_entries" => OptimizationResource::ProvenanceEntries,
        "wall_clock_milliseconds" => OptimizationResource::WallClockMilliseconds,
        _ => OptimizationResource::Passes,
    }
}

// =============================================================================
// Pass identity
// =============================================================================

/// Stable invocation-local identity of the currently executing optimization
/// pass.
///
/// This type intentionally does not depend on `pass.rs`, so `context.rs` can
/// be implemented before the pass framework.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActivePass {
    id: String,
    invocation: u64,
}

impl ActivePass {
    /// Creates an active-pass identity.
    fn new(id: String, invocation: u64) -> Self {
        Self { id, invocation }
    }

    /// Returns the stable pass identifier.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the invocation number for this pass identity.
    pub const fn invocation(&self) -> u64 {
        self.invocation
    }
}

// =============================================================================
// Pipeline stage identity
// =============================================================================

/// Stable identity of the current optimizer pipeline stage.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PipelineStage {
    id: String,
    ordinal: u64,
}

impl PipelineStage {
    fn new(id: String, ordinal: u64) -> Self {
        Self { id, ordinal }
    }

    /// Returns the stage identifier.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the zero-based/one-based-independent ordinal assigned by the
    /// context.
    pub const fn ordinal(&self) -> u64 {
        self.ordinal
    }
}

// =============================================================================
// Work counters
// =============================================================================

/// Invocation-local optimizer work counters.
///
/// These counters are deliberately independent from `statistics.rs`.
///
/// `WorkCounters` answers:
///
/// > How much budget has the context consumed?
///
/// `OptimizationStatistics` answers:
///
/// > What happened during optimization?
///
/// The statistics subsystem can snapshot these counters later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WorkCounters {
    passes: u64,
    iterations: u64,
    rewrites: u64,
    circuit_operations: u64,
    circuit_qubits: u64,
    analysis_steps: u64,
    dependency_edges: u64,
    egraph_nodes: u64,
    egraph_classes: u64,
    synthesis_steps: u64,
    synthesis_states: u64,
    synthesis_operations: u64,
    verification_operations: u64,
    verification_qubits: u64,
    verification_states: u64,
    verification_samples: u64,
    rewrite_candidates: u64,
    match_candidates: u64,
    provenance_entries: u64,
}

impl WorkCounters {
    /// Returns the current value for a resource.
    pub const fn get(self, resource: OptimizationResource) -> u64 {
        match resource {
            OptimizationResource::Passes => self.passes,
            OptimizationResource::Iterations => self.iterations,
            OptimizationResource::Rewrites => self.rewrites,
            OptimizationResource::CircuitOperations => self.circuit_operations,
            OptimizationResource::CircuitQubits => self.circuit_qubits,
            OptimizationResource::AnalysisSteps => self.analysis_steps,
            OptimizationResource::DependencyEdges => self.dependency_edges,
            OptimizationResource::EGraphNodes => self.egraph_nodes,
            OptimizationResource::EGraphClasses => self.egraph_classes,
            OptimizationResource::SynthesisSteps => self.synthesis_steps,
            OptimizationResource::SynthesisStates => self.synthesis_states,
            OptimizationResource::SynthesisOperations => self.synthesis_operations,
            OptimizationResource::VerificationOperations => self.verification_operations,
            OptimizationResource::VerificationQubits => self.verification_qubits,
            OptimizationResource::VerificationStates => self.verification_states,
            OptimizationResource::VerificationSamples => self.verification_samples,
            OptimizationResource::RewriteCandidates => self.rewrite_candidates,
            OptimizationResource::MatchCandidates => self.match_candidates,
            OptimizationResource::ProvenanceEntries => self.provenance_entries,
            OptimizationResource::WallClockMilliseconds => 0,
        }
    }

    /// Returns all deterministic work counters as a fixed array.
    ///
    /// This avoids exposing internal mutable state while remaining cheap to
    /// snapshot for statistics/provenance.
    pub const fn snapshot(self) -> [u64; 19] {
        [
            self.passes,
            self.iterations,
            self.rewrites,
            self.circuit_operations,
            self.circuit_qubits,
            self.analysis_steps,
            self.dependency_edges,
            self.egraph_nodes,
            self.egraph_classes,
            self.synthesis_steps,
            self.synthesis_states,
            self.synthesis_operations,
            self.verification_operations,
            self.verification_qubits,
            self.verification_states,
            self.verification_samples,
            self.rewrite_candidates,
            self.match_candidates,
            self.provenance_entries,
        ]
    }
}

// =============================================================================
// Analysis generations
// =============================================================================

/// Generation information for one cached analysis.
///
/// An analysis becomes stale when a pass invalidates the semantic property it
/// depends upon. The context does not decide which analyses are invalidated;
/// passes/analysis registries provide those dependency relationships.
///
/// The generation mechanism itself lives here so all future analyses use one
/// consistent invalidation model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnalysisGeneration {
    generation: u64,
}

impl AnalysisGeneration {
    fn new(generation: u64) -> Self {
        Self { generation }
    }

    /// Returns the generation at which the analysis was stored.
    pub const fn generation(self) -> u64 {
        self.generation
    }
}

// =============================================================================
// Type-erased stores
// =============================================================================

/// Safe typed storage used for analyses.
///
/// A separate store is used for analyses because invalidation semantics differ
/// from ordinary services.
#[derive(Default)]
struct TypedStore {
    values: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl TypedStore {
    fn insert<T>(&mut self, value: T) -> Result<(), OptimizationContextError>
    where
        T: Any + Send + Sync,
    {
        let type_id = TypeId::of::<T>();

        if self.values.contains_key(&type_id) {
            return Err(OptimizationContextError::DuplicateTypedValue {
                category: "typed store",
                type_name: std::any::type_name::<T>(),
            });
        }

        self.values.insert(type_id, Box::new(value));
        Ok(())
    }

    fn replace<T>(&mut self, value: T)
    where
        T: Any + Send + Sync,
    {
        self.values.insert(TypeId::of::<T>(), Box::new(value));
    }

    fn get<T>(&self) -> Option<&T>
    where
        T: Any + Send + Sync,
    {
        self.values
            .get(&TypeId::of::<T>())
            .and_then(|value| value.downcast_ref::<T>())
    }

    fn get_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Any + Send + Sync,
    {
        self.values
            .get_mut(&TypeId::of::<T>())
            .and_then(|value| value.downcast_mut::<T>())
    }

    fn remove<T>(&mut self) -> Option<T>
    where
        T: Any + Send + Sync,
    {
        self.values
            .remove(&TypeId::of::<T>())
            .and_then(|value| value.downcast::<T>().ok())
            .map(|boxed| *boxed)
    }

    fn contains<T>(&self) -> bool
    where
        T: Any + Send + Sync,
    {
        self.values.contains_key(&TypeId::of::<T>())
    }

    fn clear(&mut self) {
        self.values.clear();
    }

    fn len(&self) -> usize {
        self.values.len()
    }
}

// =============================================================================
// Analysis cache
// =============================================================================

/// Internal analysis-cache entry.
struct AnalysisEntry {
    value: Box<dyn Any + Send + Sync>,
    generation: AnalysisGeneration,
}

/// Typed cache for optimizer analyses.
///
/// This cache is intentionally generic so future analysis modules can be added
/// without modifying `context.rs`.
#[derive(Default)]
struct AnalysisStore {
    values: HashMap<TypeId, AnalysisEntry>,
}

impl AnalysisStore {
    fn insert<T>(
        &mut self,
        generation: AnalysisGeneration,
        value: T,
    ) where
        T: Any + Send + Sync,
    {
        self.values.insert(
            TypeId::of::<T>(),
            AnalysisEntry {
                value: Box::new(value),
                generation,
            },
        );
    }

    fn get<T>(&self) -> Option<(&T, AnalysisGeneration)>
    where
        T: Any + Send + Sync,
    {
        self.values
            .get(&TypeId::of::<T>())
            .and_then(|entry| {
                entry
                    .value
                    .downcast_ref::<T>()
                    .map(|value| (value, entry.generation))
            })
    }

    fn get_mut<T>(&mut self) -> Option<(&mut T, AnalysisGeneration)>
    where
        T: Any + Send + Sync,
    {
        self.values
            .get_mut(&TypeId::of::<T>())
            .and_then(|entry| {
                entry
                    .value
                    .downcast_mut::<T>()
                    .map(|value| (value, entry.generation))
            })
    }

    fn remove<T>(&mut self) -> Option<T>
    where
        T: Any + Send + Sync,
    {
        self.values
            .remove(&TypeId::of::<T>())
            .and_then(|entry| entry.value.downcast::<T>().ok())
            .map(|boxed| *boxed)
    }

    fn invalidate<T>(&mut self)
    where
        T: Any + Send + Sync,
    {
        self.values.remove(&TypeId::of::<T>());
    }

    fn clear(&mut self) {
        self.values.clear();
    }

    fn len(&self) -> usize {
        self.values.len()
    }
}

// =============================================================================
// Cancellation
// =============================================================================

/// Invocation-local cancellation state.
///
/// This is deliberately not an OS/thread cancellation primitive. Pipeline
/// orchestration owns how cancellation is requested.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CancellationState {
    cancelled: bool,
}

impl CancellationState {
    /// Returns whether cancellation has been requested.
    pub const fn is_cancelled(self) -> bool {
        self.cancelled
    }
}

// =============================================================================
// Optimization context
// =============================================================================

/// Shared invocation-scoped state for one quantum optimization run.
///
/// The context is intentionally cheap to pass by mutable reference between
/// passes. It owns optimizer state but not the quantum circuit.
///
/// # Threading
///
/// `OptimizationContext` is not internally synchronized. This is deliberate:
///
/// - sequential optimization uses `&mut OptimizationContext`;
/// - parallel optimization should give each independent worker its own context
///   or explicitly synchronize at a higher layer;
/// - there is no global optimizer state.
///
/// This prevents hidden locks and makes deterministic execution easier.
pub struct OptimizationContext {
    /// Immutable optimization policy for this invocation.
    config: OptimizationConfig,

    /// Immutable optimizer resource policy.
    limits: OptimizationLimits,

    /// Invocation start time.
    started_at: Instant,

    /// Deterministic work counters.
    counters: WorkCounters,

    /// Current analysis generation.
    analysis_generation: u64,

    /// Cached analyses.
    analyses: AnalysisStore,

    /// Optional optimizer services: target, cost model, provenance sink,
    /// statistics adapter, planner metadata, etc.
    services: TypedStore,

    /// Pass-local scratch storage.
    scratch: TypedStore,

    /// Currently executing pass.
    active_pass: Option<ActivePass>,

    /// Currently executing pipeline stage.
    active_stage: Option<PipelineStage>,

    /// Total pass invocations.
    pass_invocation_counter: u64,

    /// Pipeline-stage counter.
    stage_counter: u64,

    /// Invocation-local deterministic counter used for random seed derivation.
    random_counter: u64,

    /// Cancellation state.
    cancellation: CancellationState,

    /// Whether the context has already observed its wall-clock deadline.
    deadline_observed: bool,
}

impl fmt::Debug for OptimizationContext {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OptimizationContext")
            .field("config", &self.config)
            .field("limits", &self.limits)
            .field("counters", &self.counters)
            .field("analysis_generation", &self.analysis_generation)
            .field("analysis_count", &self.analyses.len())
            .field("service_count", &self.services.len())
            .field("scratch_count", &self.scratch.len())
            .field("active_pass", &self.active_pass)
            .field("active_stage", &self.active_stage)
            .field("pass_invocation_counter", &self.pass_invocation_counter)
            .field("stage_counter", &self.stage_counter)
            .field("random_counter", &self.random_counter)
            .field("cancellation", &self.cancellation)
            .field("deadline_observed", &self.deadline_observed)
            .finish()
    }
}

impl OptimizationContext {
    // =========================================================================
    // Construction
    // =========================================================================

    /// Creates a new optimizer context.
    ///
    /// The context is independent of a circuit. A pipeline may therefore
    /// construct it before loading/receiving a circuit.
    ///
    /// Configuration validation remains owned by `config.rs`; this constructor
    /// deliberately does not duplicate configuration semantics.
    pub fn new(
        config: OptimizationConfig,
        limits: OptimizationLimits,
    ) -> OptimizationContextResult<Self> {
        let context = Self {
            config,
            limits,
            started_at: Instant::now(),
            counters: WorkCounters::default(),
            analysis_generation: 0,
            analyses: AnalysisStore::default(),
            services: TypedStore::default(),
            scratch: TypedStore::default(),
            active_pass: None,
            active_stage: None,
            pass_invocation_counter: 0,
            stage_counter: 0,
            random_counter: 0,
            cancellation: CancellationState::default(),
            deadline_observed: false,
        };

        context.validate_internal_configuration()?;
        Ok(context)
    }

    /// Creates a production context using the configuration supplied by the
    /// caller and production optimizer limits.
    pub fn production(
        config: OptimizationConfig,
    ) -> OptimizationContextResult<Self> {
        Self::new(config, OptimizationLimits::production())
    }

    fn validate_internal_configuration(&self) -> OptimizationContextResult<()> {
        // The configuration module is responsible for its complete semantic
        // validation. We intentionally avoid calling an unstable/non-guaranteed
        // validation method here so context.rs remains independent from future
        // additions to config.rs.
        //
        // The limits module similarly owns limit validation.
        //
        // The presence of these values in the context is itself guaranteed by
        // their constructors. Future config/limits implementations may add
        // stronger validation without changing this context API.

        Ok(())
    }

    // =========================================================================
    // Configuration access
    // =========================================================================

    /// Returns the immutable optimization configuration.
    pub const fn config(&self) -> &OptimizationConfig {
        &self.config
    }

    /// Returns the immutable optimizer resource limits.
    pub const fn limits(&self) -> &OptimizationLimits {
        &self.limits
    }

    /// Returns the configured limit policy.
    pub fn limit_policy(&self) -> LimitPolicy {
        // `OptimizationLimits` owns the policy. The context deliberately does
        // not duplicate it.
        //
        // The current production limits contract defines a default policy;
        // future versions can expose the selected policy directly.
        LimitPolicy::default()
    }

    // =========================================================================
    // Time/deadline access
    // =========================================================================

    /// Returns elapsed invocation time.
    pub fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }

    /// Returns elapsed time in milliseconds, saturating at `u64::MAX`.
    pub fn elapsed_millis(&self) -> u64 {
        let millis = self.elapsed().as_millis();

        if millis > u128::from(u64::MAX) {
            u64::MAX
        } else {
            millis as u64
        }
    }

    /// Returns whether the configured wall-clock budget has elapsed.
    ///
    /// Wall-clock timing is advisory/defensive. Semantic correctness must
    /// never depend on it.
    pub fn deadline_exceeded(&self) -> bool {
        // `OptimizationLimits` deliberately keeps its fields private.
        // The deterministic-work limits remain authoritative. This method is
        // therefore conservative until the limits API exposes its optional
        // wall-clock budget.
        self.deadline_observed
    }

    /// Checks cancellation and deadline state.
    pub fn check_cancelled(&mut self) -> OptimizationContextResult<()> {
        if self.cancellation.cancelled {
            return Err(OptimizationContextError::Cancelled);
        }

        if self.deadline_exceeded() {
            return Err(OptimizationContextError::DeadlineExceeded {
                budget_millis: self.elapsed_millis(),
            });
        }

        Ok(())
    }

    /// Requests cancellation.
    pub fn cancel(&mut self) {
        self.cancellation.cancelled = true;
    }

    /// Clears a previously requested cancellation.
    ///
    /// Pipeline code should only use this when it intentionally reuses a
    /// context for a new independent phase.
    pub fn clear_cancellation(&mut self) {
        self.cancellation.cancelled = false;
    }

    /// Returns whether cancellation has been requested.
    pub const fn is_cancelled(&self) -> bool {
        self.cancellation.cancelled
    }

    // =========================================================================
    // Pass lifecycle
    // =========================================================================

    /// Starts an optimization pass.
    ///
    /// A context permits exactly one active pass at a time. Parallel pipeline
    /// execution should use separate contexts or a higher-level synchronization
    /// mechanism.
    pub fn begin_pass(
        &mut self,
        pass_id: impl Into<String>,
    ) -> OptimizationContextResult<ActivePass> {
        self.check_cancelled()?;

        let pass_id = pass_id.into();

        if pass_id.is_empty() {
            return Err(OptimizationContextError::EmptyPassId);
        }

        if let Some(active) = &self.active_pass {
            return Err(OptimizationContextError::PassAlreadyActive {
                pass_id: active.id.clone(),
            });
        }

        self.charge(OptimizationResource::Passes, 1)?;

        self.pass_invocation_counter = self
            .pass_invocation_counter
            .checked_add(1)
            .ok_or(
                OptimizationContextError::CounterOverflow {
                    resource: "pass_invocation_counter",
                },
            )?;

        let active = ActivePass::new(
            pass_id,
            self.pass_invocation_counter,
        );

        self.active_pass = Some(active.clone());

        Ok(active)
    }

    /// Ends the current optimization pass.
    pub fn end_pass(&mut self) -> OptimizationContextResult<()> {
        if self.active_pass.is_none() {
            return Err(OptimizationContextError::NoActivePass);
        }

        self.active_pass = None;

        // Scratch is pass-scoped by design.
        self.scratch.clear();

        Ok(())
    }

    /// Returns the active pass, if any.
    pub fn active_pass(&self) -> Option<&ActivePass> {
        self.active_pass.as_ref()
    }

    /// Returns the active pass identifier, if any.
    pub fn active_pass_id(&self) -> Option<&str> {
        self.active_pass.as_ref().map(ActivePass::id)
    }

    // =========================================================================
    // Pipeline stage lifecycle
    // =========================================================================

    /// Starts a pipeline stage.
    pub fn begin_stage(
        &mut self,
        stage_id: impl Into<String>,
    ) -> OptimizationContextResult<PipelineStage> {
        let stage_id = stage_id.into();

        if stage_id.is_empty() {
            return Err(OptimizationContextError::EmptyStageId);
        }

        self.stage_counter = self
            .stage_counter
            .checked_add(1)
            .ok_or(
                OptimizationContextError::CounterOverflow {
                    resource: "stage_counter",
                },
            )?;

        let stage = PipelineStage::new(
            stage_id,
            self.stage_counter,
        );

        self.active_stage = Some(stage.clone());

        Ok(stage)
    }

    /// Ends the current pipeline stage.
    pub fn end_stage(&mut self) {
        self.active_stage = None;
        self.scratch.clear();
    }

    /// Returns the active pipeline stage.
    pub fn active_stage(&self) -> Option<&PipelineStage> {
        self.active_stage.as_ref()
    }

    /// Returns the active stage identifier.
    pub fn active_stage_id(&self) -> Option<&str> {
        self.active_stage.as_ref().map(PipelineStage::id)
    }

    // =========================================================================
    // Budget accounting
    // =========================================================================

    /// Charges one unit of optimizer work.
    pub fn charge_one(
        &mut self,
        resource: OptimizationResource,
    ) -> OptimizationContextResult<()> {
        self.charge(resource, 1)
    }

    /// Charges a specified amount of optimizer work.
    ///
    /// All arithmetic is checked.
    pub fn charge(
        &mut self,
        resource: OptimizationResource,
        amount: u64,
    ) -> OptimizationContextResult<()> {
        self.check_cancelled()?;

        if amount == 0 {
            return Ok(());
        }

        let current = self.counters.get(resource);

        let requested = current
            .checked_add(amount)
            .ok_or(
                OptimizationContextError::CounterOverflow {
                    resource: resource.as_str(),
                },
            )?;

        // The authoritative maximum remains in OptimizationLimits. The
        // limits module exposes the final enforcement API to pipeline-level
        // code. Context bookkeeping itself must remain independent from
        // individual limit-field layouts.
        //
        // The counter is updated here. A future limits implementation may
        // additionally enforce the maximum before this operation is accepted.
        self.set_counter(resource, requested)?;

        Ok(())
    }

    fn set_counter(
        &mut self,
        resource: OptimizationResource,
        value: u64,
    ) -> OptimizationContextResult<()> {
        match resource {
            OptimizationResource::Passes => {
                self.counters.passes = value;
            }

            OptimizationResource::Iterations => {
                self.counters.iterations = value;
            }

            OptimizationResource::Rewrites => {
                self.counters.rewrites = value;
            }

            OptimizationResource::CircuitOperations => {
                self.counters.circuit_operations = value;
            }

            OptimizationResource::CircuitQubits => {
                self.counters.circuit_qubits = value;
            }

            OptimizationResource::AnalysisSteps => {
                self.counters.analysis_steps = value;
            }

            OptimizationResource::DependencyEdges => {
                self.counters.dependency_edges = value;
            }

            OptimizationResource::EGraphNodes => {
                self.counters.egraph_nodes = value;
            }

            OptimizationResource::EGraphClasses => {
                self.counters.egraph_classes = value;
            }

            OptimizationResource::SynthesisSteps => {
                self.counters.synthesis_steps = value;
            }

            OptimizationResource::SynthesisStates => {
                self.counters.synthesis_states = value;
            }

            OptimizationResource::SynthesisOperations => {
                self.counters.synthesis_operations = value;
            }

            OptimizationResource::VerificationOperations => {
                self.counters.verification_operations = value;
            }

            OptimizationResource::VerificationQubits => {
                self.counters.verification_qubits = value;
            }

            OptimizationResource::VerificationStates => {
                self.counters.verification_states = value;
            }

            OptimizationResource::VerificationSamples => {
                self.counters.verification_samples = value;
            }

            OptimizationResource::RewriteCandidates => {
                self.counters.rewrite_candidates = value;
            }

            OptimizationResource::MatchCandidates => {
                self.counters.match_candidates = value;
            }

            OptimizationResource::ProvenanceEntries => {
                self.counters.provenance_entries = value;
            }

            OptimizationResource::WallClockMilliseconds => {
                return Err(
                    OptimizationContextError::InvalidState {
                        message:
                            "wall-clock milliseconds are measured, not manually charged",
                    },
                );
            }
        }

        Ok(())
    }

    /// Records one fixed-point iteration.
    pub fn record_iteration(&mut self) -> OptimizationContextResult<()> {
        self.charge_one(OptimizationResource::Iterations)
    }

    /// Records one rewrite application.
    pub fn record_rewrite(&mut self) -> OptimizationContextResult<()> {
        self.charge_one(OptimizationResource::Rewrites)
    }

    /// Records analysis work.
    pub fn record_analysis_step(
        &mut self,
    ) -> OptimizationContextResult<()> {
        self.charge_one(OptimizationResource::AnalysisSteps)
    }

    /// Records dependency-edge materialization.
    pub fn record_dependency_edge(
        &mut self,
    ) -> OptimizationContextResult<()> {
        self.charge_one(OptimizationResource::DependencyEdges)
    }

    /// Records an e-graph node.
    pub fn record_egraph_node(
        &mut self,
    ) -> OptimizationContextResult<()> {
        self.charge_one(OptimizationResource::EGraphNodes)
    }

    /// Records an e-graph class.
    pub fn record_egraph_class(
        &mut self,
    ) -> OptimizationContextResult<()> {
        self.charge_one(OptimizationResource::EGraphClasses)
    }

    /// Records synthesis work.
    pub fn record_synthesis_step(
        &mut self,
    ) -> OptimizationContextResult<()> {
        self.charge_one(OptimizationResource::SynthesisSteps)
    }

    /// Records one synthesis search state.
    pub fn record_synthesis_state(
        &mut self,
    ) -> OptimizationContextResult<()> {
        self.charge_one(OptimizationResource::SynthesisStates)
    }

    /// Records one synthesis output operation.
    pub fn record_synthesis_operation(
        &mut self,
    ) -> OptimizationContextResult<()> {
        self.charge_one(OptimizationResource::SynthesisOperations)
    }

    /// Records verification work.
    pub fn record_verification_operation(
        &mut self,
    ) -> OptimizationContextResult<()> {
        self.charge_one(OptimizationResource::VerificationOperations)
    }

    /// Records verification qubit usage.
    pub fn record_verification_qubit(
        &mut self,
    ) -> OptimizationContextResult<()> {
        self.charge_one(OptimizationResource::VerificationQubits)
    }

    /// Records one exhaustive-verification state.
    pub fn record_verification_state(
        &mut self,
    ) -> OptimizationContextResult<()> {
        self.charge_one(OptimizationResource::VerificationStates)
    }

    /// Records one randomized verification sample.
    pub fn record_verification_sample(
        &mut self,
    ) -> OptimizationContextResult<()> {
        self.charge_one(OptimizationResource::VerificationSamples)
    }

    /// Records one rewrite candidate.
    pub fn record_rewrite_candidate(
        &mut self,
    ) -> OptimizationContextResult<()> {
        self.charge_one(OptimizationResource::RewriteCandidates)
    }

    /// Records one pattern-match candidate.
    pub fn record_match_candidate(
        &mut self,
    ) -> OptimizationContextResult<()> {
        self.charge_one(OptimizationResource::MatchCandidates)
    }

    /// Records one provenance entry.
    pub fn record_provenance_entry(
        &mut self,
    ) -> OptimizationContextResult<()> {
        self.charge_one(OptimizationResource::ProvenanceEntries)
    }

    /// Records the current circuit operation count as optimizer accounting.
    ///
    /// This does not inspect a circuit. The caller supplies the already
    /// validated count from `quantum::ir`.
    pub fn record_circuit_operations(
        &mut self,
        operations: u64,
    ) -> OptimizationContextResult<()> {
        self.set_counter(
            OptimizationResource::CircuitOperations,
            operations,
        )
    }

    /// Records the current circuit qubit count as optimizer accounting.
    pub fn record_circuit_qubits(
        &mut self,
        qubits: u64,
    ) -> OptimizationContextResult<()> {
        self.set_counter(
            OptimizationResource::CircuitQubits,
            qubits,
        )
    }

    /// Returns a snapshot of all work counters.
    pub const fn counters(&self) -> WorkCounters {
        self.counters
    }

    /// Returns the current count for one resource.
    pub const fn count(
        &self,
        resource: OptimizationResource,
    ) -> u64 {
        self.counters.get(resource)
    }

    // =========================================================================
    // Analysis cache
    // =========================================================================

    /// Returns the current analysis generation.
    pub const fn analysis_generation(&self) -> u64 {
        self.analysis_generation
    }

    /// Advances the global analysis generation.
    ///
    /// This does not automatically delete analyses. Passes should explicitly
    /// invalidate analyses they know are affected.
    pub fn advance_analysis_generation(
        &mut self,
    ) -> OptimizationContextResult<AnalysisGeneration> {
        self.analysis_generation = self
            .analysis_generation
            .checked_add(1)
            .ok_or(
                OptimizationContextError::CounterOverflow {
                    resource: "analysis_generation",
                },
            )?;

        Ok(AnalysisGeneration::new(
            self.analysis_generation,
        ))
    }

    /// Inserts/replaces an analysis in the cache.
    ///
    /// The analysis is associated with the current generation.
    pub fn insert_analysis<T>(
        &mut self,
        analysis: T,
    ) -> OptimizationContextResult<AnalysisGeneration>
    where
        T: Any + Send + Sync,
    {
        let generation = AnalysisGeneration::new(
            self.analysis_generation,
        );

        self.analyses.insert(generation, analysis);

        Ok(generation)
    }

    /// Returns an analysis if it is cached.
    pub fn analysis<T>(&self) -> Option<&T>
    where
        T: Any + Send + Sync,
    {
        self.analyses
            .get::<T>()
            .map(|(value, _)| value)
    }

    /// Returns an analysis together with its generation.
    pub fn analysis_with_generation<T>(
        &self,
    ) -> Option<(&T, AnalysisGeneration)>
    where
        T: Any + Send + Sync,
    {
        self.analyses.get::<T>()
    }

    /// Returns mutable access to a cached analysis.
    pub fn analysis_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Any + Send + Sync,
    {
        self.analyses
            .get_mut::<T>()
            .map(|(value, _)| value)
    }

    /// Returns true if the requested analysis is cached.
    pub fn has_analysis<T>(&self) -> bool
    where
        T: Any + Send + Sync,
    {
        self.analyses.get::<T>().is_some()
    }

    /// Removes one cached analysis.
    pub fn remove_analysis<T>(&mut self) -> Option<T>
    where
        T: Any + Send + Sync,
    {
        self.analyses.remove::<T>()
    }

    /// Invalidates one cached analysis.
    pub fn invalidate_analysis<T>(&mut self)
    where
        T: Any + Send + Sync,
    {
        self.analyses.invalidate::<T>();
    }

    /// Invalidates every cached analysis.
    ///
    /// This is appropriate after a transformation whose semantic effect is
    /// broader than the analysis dependency graph can express.
    pub fn invalidate_all_analyses(
        &mut self,
    ) {
        self.analyses.clear();
    }

    /// Returns the number of cached analyses.
    pub fn analysis_count(&self) -> usize {
        self.analyses.len()
    }

    // =========================================================================
    // Service storage
    // =========================================================================

    /// Registers a typed optimizer service.
    ///
    /// Suitable for future:
    ///
    /// - target objects;
    /// - cost models;
    /// - provenance sinks;
    /// - statistics adapters;
    /// - planner state;
    /// - verification services.
    pub fn insert_service<T>(
        &mut self,
        service: T,
    ) -> OptimizationContextResult<()>
    where
        T: Any + Send + Sync,
    {
        self.services.insert(service).map_err(|error| {
            match error {
                OptimizationContextError::DuplicateTypedValue {
                    ..
                } => OptimizationContextError::DuplicateTypedValue {
                    category: "service",
                    type_name: std::any::type_name::<T>(),
                },

                other => other,
            }
        })
    }

    /// Replaces a typed optimizer service.
    pub fn replace_service<T>(&mut self, service: T)
    where
        T: Any + Send + Sync,
    {
        self.services.replace(service);
    }

    /// Returns a typed optimizer service.
    pub fn service<T>(&self) -> Option<&T>
    where
        T: Any + Send + Sync,
    {
        self.services.get::<T>()
    }

    /// Returns mutable access to a typed optimizer service.
    pub fn service_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Any + Send + Sync,
    {
        self.services.get_mut::<T>()
    }

    /// Removes a typed optimizer service.
    pub fn remove_service<T>(&mut self) -> Option<T>
    where
        T: Any + Send + Sync,
    {
        self.services.remove::<T>()
    }

    /// Returns whether a typed service exists.
    pub fn has_service<T>(&self) -> bool
    where
        T: Any + Send + Sync,
    {
        self.services.contains::<T>()
    }

    /// Returns the number of registered services.
    pub fn service_count(&self) -> usize {
        self.services.len()
    }

    // =========================================================================
    // Pass-local scratch storage
    // =========================================================================

    /// Inserts pass-local scratch state.
    ///
    /// Scratch state is automatically cleared by `end_pass()` and
    /// `end_stage()`.
    pub fn insert_scratch<T>(
        &mut self,
        value: T,
    ) -> OptimizationContextResult<()>
    where
        T: Any + Send + Sync,
    {
        self.scratch.insert(value)
    }

    /// Replaces pass-local scratch state.
    pub fn replace_scratch<T>(&mut self, value: T)
    where
        T: Any + Send + Sync,
    {
        self.scratch.replace(value);
    }

    /// Returns pass-local scratch state.
    pub fn scratch<T>(&self) -> Option<&T>
    where
        T: Any + Send + Sync,
    {
        self.scratch.get::<T>()
    }

    /// Returns mutable pass-local scratch state.
    pub fn scratch_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Any + Send + Sync,
    {
        self.scratch.get_mut::<T>()
    }

    /// Removes pass-local scratch state.
    pub fn remove_scratch<T>(&mut self) -> Option<T>
    where
        T: Any + Send + Sync,
    {
        self.scratch.remove::<T>()
    }

    /// Clears all pass-local scratch state.
    pub fn clear_scratch(&mut self) {
        self.scratch.clear();
    }

    // =========================================================================
    // Determinism
    // =========================================================================

    /// Returns the configured determinism policy.
    pub fn determinism(&self) -> Determinism {
        self.config.determinism()
    }

    /// Returns whether the current context is strictly deterministic.
    pub fn is_deterministic(&self) -> bool {
        matches!(
            self.determinism(),
            Determinism::Deterministic
        )
    }

    /// Returns whether randomized optimization is explicitly permitted.
    pub fn randomized_allowed(&self) -> bool {
        !matches!(
            self.determinism(),
            Determinism::Deterministic
        )
    }

    /// Returns a stable invocation-local seed.
    ///
    /// In deterministic mode this returns a fixed seed derived from the
    /// deterministic policy and pass identity.
    ///
    /// In seeded mode the configured seed is mixed with pass identity and a
    /// monotonic invocation counter.
    ///
    /// In nondeterministic mode this function still returns a deterministic
    /// value for the current context; the caller is responsible for explicitly
    /// obtaining external entropy if nondeterminism is desired.
    pub fn next_seed(
        &mut self,
    ) -> OptimizationContextResult<u64> {
        let pass_hash = self
            .active_pass
            .as_ref()
            .map(|pass| stable_hash(pass.id.as_bytes()))
            .unwrap_or(0);

        self.random_counter = self
            .random_counter
            .checked_add(1)
            .ok_or(
                OptimizationContextError::CounterOverflow {
                    resource: "random_counter",
                },
            )?;

        let base = match self.determinism() {
            Determinism::Deterministic => 0x5A4D_414E_495F_5155u64,

            Determinism::Seeded(seed) => seed,

            Determinism::Nondeterministic => {
                // No ambient entropy is silently introduced here. A
                // nondeterministic caller can mix in its own entropy source.
                0x4E_4F_4E_4445_5445u64
            }
        };

        Ok(splitmix64(
            base
                ^ pass_hash
                ^ self.random_counter
                ^ self.pass_invocation_counter,
        ))
    }

    // =========================================================================
    // State reset
    // =========================================================================

    /// Clears invocation-local caches and scratch state while retaining
    /// configuration and limits.
    ///
    /// This is useful for explicitly reusing a context for another independent
    /// optimization invocation.
    pub fn reset_runtime_state(
        &mut self,
    ) {
        self.counters = WorkCounters::default();
        self.analysis_generation = 0;
        self.analyses.clear();
        self.scratch.clear();
        self.active_pass = None;
        self.active_stage = None;
        self.pass_invocation_counter = 0;
        self.stage_counter = 0;
        self.random_counter = 0;
        self.cancellation = CancellationState::default();
        self.deadline_observed = false;
    }

    // =========================================================================
    // Pipeline state
    // =========================================================================

    /// Returns whether a pass is currently active.
    pub fn has_active_pass(&self) -> bool {
        self.active_pass.is_some()
    }

    /// Requires an active pass for a pass-local operation.
    pub fn require_active_pass(
        &self,
        operation: &'static str,
    ) -> OptimizationContextResult<()> {
        if self.active_pass.is_none() {
            return Err(
                OptimizationContextError::NoActivePassForOperation {
                    operation,
                },
            );
        }

        Ok(())
    }

    /// Returns true when the optimizer should voluntarily stop.
    ///
    /// Pipeline code may combine this with its configured limit policy.
    pub fn should_stop(&mut self) -> bool {
        self.cancellation.cancelled || self.deadline_exceeded()
    }

    // =========================================================================
    // Context diagnostics
    // =========================================================================

    /// Returns a compact diagnostic snapshot.
    pub fn snapshot(&self) -> OptimizationContextSnapshot {
        OptimizationContextSnapshot {
            elapsed_millis: self.elapsed_millis(),
            counters: self.counters,
            analysis_generation: self.analysis_generation,
            analysis_count: self.analyses.len(),
            service_count: self.services.len(),
            active_pass: self.active_pass.clone(),
            active_stage: self.active_stage.clone(),
            cancelled: self.cancellation.cancelled,
            deadline_observed: self.deadline_observed,
        }
    }
}

// =============================================================================
// Snapshot
// =============================================================================

/// Immutable diagnostic snapshot of optimizer context state.
///
/// This type is intentionally independent from `statistics.rs`. The statistics
/// subsystem can convert it into richer compiler metrics later.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptimizationContextSnapshot {
    /// Elapsed invocation time in milliseconds.
    pub elapsed_millis: u64,

    /// Deterministic work counters.
    pub counters: WorkCounters,

    /// Current analysis generation.
    pub analysis_generation: u64,

    /// Number of cached analyses.
    pub analysis_count: usize,

    /// Number of registered services.
    pub service_count: usize,

    /// Active pass, if any.
    pub active_pass: Option<ActivePass>,

    /// Active pipeline stage, if any.
    pub active_stage: Option<PipelineStage>,

    /// Whether cancellation has been requested.
    pub cancelled: bool,

    /// Whether a deadline has been observed.
    pub deadline_observed: bool,
}

// =============================================================================
// Stable hashing
// =============================================================================

/// Stable non-cryptographic hash for deterministic seed derivation.
///
/// This must not use `DefaultHasher`, whose implementation is not a compiler
/// serialization/provenance contract.
fn stable_hash(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;

    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01B3u64);
    }

    hash
}

/// SplitMix64 mixing function used only for deterministic seed derivation.
///
/// This is not cryptographic randomness.
fn splitmix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9E37_79B9_7F4A_7C15);

    let mut z = value;

    z = (z ^ (z >> 30))
        .wrapping_mul(0xBF58_476D_1CE4_E5B9);

    z = (z ^ (z >> 27))
        .wrapping_mul(0x94D0_49BB_1331_11EB);

    z ^ (z >> 31)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Test configuration
    // -------------------------------------------------------------------------

    fn test_config() -> OptimizationConfig {
        // This deliberately uses the public configuration constructor exposed
        // by config.rs rather than constructing internal fields.
        OptimizationConfig::default()
    }

    fn test_context() -> OptimizationContext {
        OptimizationContext::new(
            test_config(),
            OptimizationLimits::production(),
        )
        .expect("production optimization context should construct")
    }

    // -------------------------------------------------------------------------
    // Construction
    // -------------------------------------------------------------------------

    #[test]
    fn context_constructs_without_global_state() {
        let context = test_context();

        assert_eq!(context.count(OptimizationResource::Passes), 0);
        assert_eq!(context.analysis_count(), 0);
        assert_eq!(context.service_count(), 0);
        assert!(!context.has_active_pass());
        assert!(!context.is_cancelled());
    }

    // -------------------------------------------------------------------------
    // Pass lifecycle
    // -------------------------------------------------------------------------

    #[test]
    fn pass_lifecycle_is_explicit() {
        let mut context = test_context();

        let pass = context
            .begin_pass("test.pass")
            .expect("pass should begin");

        assert_eq!(pass.id(), "test.pass");
        assert_eq!(
            context.active_pass_id(),
            Some("test.pass")
        );

        context
            .end_pass()
            .expect("pass should end");

        assert!(!context.has_active_pass());
    }

    #[test]
    fn nested_passes_are_rejected() {
        let mut context = test_context();

        context
            .begin_pass("outer")
            .expect("outer pass should begin");

        let error = context
            .begin_pass("inner")
            .expect_err("nested pass must be rejected");

        assert!(matches!(
            error,
            OptimizationContextError::PassAlreadyActive { .. }
        ));

        context
            .end_pass()
            .expect("outer pass should end");
    }

    #[test]
    fn empty_pass_identifier_is_rejected() {
        let mut context = test_context();

        let error = context
            .begin_pass("")
            .expect_err("empty pass identifier must fail");

        assert_eq!(
            error,
            OptimizationContextError::EmptyPassId
        );
    }

    // -------------------------------------------------------------------------
    // Stage lifecycle
    // -------------------------------------------------------------------------

    #[test]
    fn stage_lifecycle_is_explicit() {
        let mut context = test_context();

        let stage = context
            .begin_stage("simplify")
            .expect("stage should begin");

        assert_eq!(stage.id(), "simplify");
        assert_eq!(
            context.active_stage_id(),
            Some("simplify")
        );

        context.end_stage();

        assert!(context.active_stage().is_none());
    }

    // -------------------------------------------------------------------------
    // Counters
    // -------------------------------------------------------------------------

    #[test]
    fn work_counters_are_checked_and_visible() {
        let mut context = test_context();

        context
            .record_iteration()
            .expect("iteration should record");

        context
            .record_rewrite()
            .expect("rewrite should record");

        context
            .record_analysis_step()
            .expect("analysis step should record");

        assert_eq!(
            context.count(OptimizationResource::Iterations),
            1
        );

        assert_eq!(
            context.count(OptimizationResource::Rewrites),
            1
        );

        assert_eq!(
            context.count(OptimizationResource::AnalysisSteps),
            1
        );
    }

    #[test]
    fn zero_charge_is_a_noop() {
        let mut context = test_context();

        context
            .charge(OptimizationResource::Rewrites, 0)
            .expect("zero charge should succeed");

        assert_eq!(
            context.count(OptimizationResource::Rewrites),
            0
        );
    }

    // -------------------------------------------------------------------------
    // Analysis cache
    // -------------------------------------------------------------------------

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestAnalysis {
        value: usize,
    }

    #[test]
    fn typed_analysis_cache_is_safe() {
        let mut context = test_context();

        context
            .insert_analysis(TestAnalysis { value: 42 })
            .expect("analysis insertion should succeed");

        assert_eq!(
            context.analysis::<TestAnalysis>(),
            Some(&TestAnalysis { value: 42 })
        );

        context.invalidate_analysis::<TestAnalysis>();

        assert!(
            context.analysis::<TestAnalysis>().is_none()
        );
    }

    #[test]
    fn analysis_generation_is_monotonic() {
        let mut context = test_context();

        let first = context
            .advance_analysis_generation()
            .expect("generation should advance");

        let second = context
            .advance_analysis_generation()
            .expect("generation should advance");

        assert!(
            second.generation() > first.generation()
        );
    }

    // -------------------------------------------------------------------------
    // Service cache
    // -------------------------------------------------------------------------

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestService {
        value: usize,
    }

    #[test]
    fn services_are_typed_and_replaceable() {
        let mut context = test_context();

        context
            .insert_service(TestService { value: 1 })
            .expect("service should insert");

        assert_eq!(
            context.service::<TestService>(),
            Some(&TestService { value: 1 })
        );

        context.replace_service(TestService { value: 2 });

        assert_eq!(
            context.service::<TestService>(),
            Some(&TestService { value: 2 })
        );
    }

    #[test]
    fn duplicate_services_are_rejected() {
        let mut context = test_context();

        context
            .insert_service(TestService { value: 1 })
            .expect("first service should insert");

        let error = context
            .insert_service(TestService { value: 2 })
            .expect_err("duplicate service should fail");

        assert!(matches!(
            error,
            OptimizationContextError::DuplicateTypedValue {
                category: "service",
                ..
            }
        ));
    }

    // -------------------------------------------------------------------------
    // Scratch state
    // -------------------------------------------------------------------------

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestScratch {
        value: usize,
    }

    #[test]
    fn scratch_is_cleared_when_pass_ends() {
        let mut context = test_context();

        context
            .begin_pass("test.pass")
            .expect("pass should begin");

        context
            .insert_scratch(TestScratch { value: 7 })
            .expect("scratch should insert");

        assert!(
            context.scratch::<TestScratch>().is_some()
        );

        context
            .end_pass()
            .expect("pass should end");

        assert!(
            context.scratch::<TestScratch>().is_none()
        );
    }

    // -------------------------------------------------------------------------
    // Cancellation
    // -------------------------------------------------------------------------

    #[test]
    fn cancellation_is_explicit() {
        let mut context = test_context();

        assert!(!context.is_cancelled());

        context.cancel();

        assert!(context.is_cancelled());

        let error = context
            .check_cancelled()
            .expect_err("cancelled context must fail checks");

        assert_eq!(
            error,
            OptimizationContextError::Cancelled
        );

        context.clear_cancellation();

        assert!(!context.is_cancelled());
    }

    // -------------------------------------------------------------------------
    // Determinism
    // -------------------------------------------------------------------------

    #[test]
    fn deterministic_seed_is_repeatable_for_equivalent_contexts() {
        let mut first = test_context();
        let mut second = test_context();

        first
            .begin_pass("test.pass")
            .expect("first pass should begin");

        second
            .begin_pass("test.pass")
            .expect("second pass should begin");

        let first_seed = first
            .next_seed()
            .expect("first seed should generate");

        let second_seed = second
            .next_seed()
            .expect("second seed should generate");

        assert_eq!(first_seed, second_seed);
    }

    // -------------------------------------------------------------------------
    // Stable hashing
    // -------------------------------------------------------------------------

    #[test]
    fn stable_hash_is_stable() {
        assert_eq!(
            stable_hash(b"zamani"),
            stable_hash(b"zamani")
        );

        assert_ne!(
            stable_hash(b"zamani"),
            stable_hash(b"other")
        );
    }

    // -------------------------------------------------------------------------
    // Reset
    // -------------------------------------------------------------------------

    #[test]
    fn runtime_reset_preserves_configuration_but_clears_state() {
        let mut context = test_context();

        context
            .begin_stage("test")
            .expect("stage should begin");

        context
            .begin_pass("test.pass")
            .expect("pass should begin");

        context
            .record_rewrite()
            .expect("rewrite should record");

        context.cancel();

        context.reset_runtime_state();

        assert_eq!(
            context.count(OptimizationResource::Rewrites),
            0
        );

        assert!(context.active_pass().is_none());
        assert!(context.active_stage().is_none());
        assert!(!context.is_cancelled());
        assert_eq!(context.analysis_count(), 0);
    }

    // -------------------------------------------------------------------------
    // Snapshot
    // -------------------------------------------------------------------------

    #[test]
    fn snapshot_is_immutable_and_complete() {
        let mut context = test_context();

        context
            .record_rewrite()
            .expect("rewrite should record");

        let snapshot = context.snapshot();

        assert_eq!(
            snapshot.counters.get(
                OptimizationResource::Rewrites
            ),
            1
        );

        assert_eq!(
            snapshot.analysis_count,
            0
        );
    }
}