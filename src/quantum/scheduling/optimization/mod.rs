//! Zamani Quantum Scheduling — Optimization
//!
//! Path:
//!     src/quantum/scheduling/optimization/mod.rs
//!
//! # Purpose
//!
//! This module is the stable public namespace and composition boundary for
//! optimization of already-constructed quantum schedules.
//!
//! Scheduling optimization answers:
//!
//! > "Among schedules that satisfy all hard semantic, dependency, resource,
//! > timing, target, and execution constraints, which valid schedule is
//! > preferable according to an explicitly supplied objective?"
//!
//! This module does not itself construct schedules.
//!
//! It exposes independent optimization components that operate on canonical
//! scheduling representations and can be consumed by:
//!
//! - scheduling planners;
//! - scheduling algorithms;
//! - scheduling policies;
//! - verification;
//! - diagnostics;
//! - benchmarking;
//! - hardware/ZQN adapters;
//! - future optimization implementations.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                     quantum::frontend
//!                              │
//!                              ▼
//!                       quantum::ir
//!                              │
//!                              ▼
//!                        optimization
//!                              │
//!                              ▼
//!                           routing
//!                              │
//!                              ▼
//!                         scheduling
//!                              │
//!                              ▼
//!                    candidate Schedule
//!                              │
//!                              ▼
//!                       verification
//!                              │
//!                              ▼
//!             scheduling::optimization  ◄── this namespace
//!                              │
//!            ┌─────────────────┼─────────────────┐
//!            │                 │                 │
//!            ▼                 ▼                 ▼
//!        makespan            depth           idle_time
//!            │                 │                 │
//!            ├─────────────────┼─────────────────┤
//!            │                 │                 │
//!            ▼                 ▼                 ▼
//!        fidelity            energy*       multi_objective
//!                              │
//!                              ▼
//!                       candidate comparison
//!                              │
//!                              ▼
//!                     best valid schedule
//! ```
//!
//! `energy*` is intentionally not declared until a corresponding implementation
//! exists in this directory. Rust module declarations are compilation
//! dependencies; exposing a nonexistent module would make the repository fail
//! to compile.
//!
//! # Core ownership boundary
//!
//! This namespace owns:
//!
//! - schedule objective semantics;
//! - schedule-quality metrics;
//! - candidate comparison;
//! - objective aggregation;
//! - objective-specific analysis;
//! - optimization-specific diagnostics;
//! - optimization contracts.
//!
//! It does NOT own:
//!
//! - canonical quantum semantics;
//! - quantum operation definitions;
//! - qubit identity;
//! - physical qubit identity;
//! - routing;
//! - hardware discovery;
//! - hardware execution;
//! - QEC algorithms;
//! - noise semantics;
//! - frontend parsing;
//! - runtime execution;
//! - vendor APIs;
//! - credentials;
//! - schedule construction.
//!
//! Those responsibilities remain in their canonical subsystems.
//!
//! # Canonical IR boundary
//!
//! The canonical quantum IR is the sole owner of quantum semantic identity.
//!
//! In particular, this namespace must never introduce another:
//!
//! ```text
//! QubitId
//! PhysicalQubitId
//! OperationId
//! ```
//!
//! When qubit identity is required by an optimization implementation, it must
//! use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! When operation identity is required, the implementation must use the
//! canonical operation identity owned by the IR, currently exposed through:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! ```
//!
//! This module itself does not import or redefine those identities because a
//! namespace boundary should not create unnecessary coupling.
//!
//! # Canonical scheduling representation
//!
//! Optimization modules consume the canonical scheduling representation
//! produced by the scheduling subsystem.
//!
//! They must not create competing definitions of:
//!
//! ```text
//! Schedule
//! ScheduledOperation
//! ScheduleResource
//! TimePoint
//! Duration
//! TimeInterval
//! ```
//!
//! The existing optimization implementations follow this ownership rule.
//! For example, idle-time optimization explicitly consumes canonical scheduling
//! types rather than defining replacements.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once at the semantic level.
//!
//! Optimization must therefore be independent of:
//!
//! - logical qubit count;
//! - physical qubit count;
//! - operation count;
//! - circuit depth;
//! - gate count;
//! - gate arity;
//! - resource count;
//! - channel count;
//! - topology dimensions;
//! - QPU count;
//! - hardware vendor;
//! - quantum technology;
//! - simulator size;
//! - emulator size.
//!
//! This module deliberately contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_PHYSICAL_QUBITS
//! MAX_OPERATIONS
//! MAX_GATES
//! MAX_DEPTH
//! MAX_RESOURCES
//! MAX_CHANNELS
//! MAX_QPUS
//! ```
//!
//! "Infinity" means that this optimization namespace imposes no artificial
//! finite machine-size ceiling.
//!
//! A concrete optimization invocation is naturally bounded by:
//!
//! - the supplied candidate schedules;
//! - available memory;
//! - explicit compiler/resource policies;
//! - target capabilities;
//! - execution budgets;
//! - the complexity of the selected objective.
//!
//! Those limits must never become semantic constants here.
//!
//! # Optimization is subordinate to validity
//!
//! The fundamental ordering is:
//!
//! ```text
//! semantic correctness
//!        │
//!        ▼
//! dependency correctness
//!        │
//!        ▼
//! resource correctness
//!        │
//!        ▼
//! timing correctness
//!        │
//!        ▼
//! target compatibility
//!        │
//!        ▼
//! verification
//!        │
//!        ▼
//! objective optimization
//! ```
//!
//! Optimization MUST NOT trade correctness for objective value.
//!
//! Therefore:
//!
//! ```text
//! invalid schedule with score 1
//!
//!        is NEVER preferable to
//!
//! valid schedule with score 100
//! ```
//!
//! Hard constraints belong to scheduling and verification. Objectives rank
//! feasible candidates.
//!
//! # Objective separation
//!
//! Each objective implementation owns its own semantics.
//!
//! Current objective modules:
//!
//! ```text
//! makespan.rs
//! depth.rs
//! idle_time.rs
//! fidelity.rs
//! multi_objective.rs
//! ```
//!
//! Their responsibilities are intentionally separated:
//!
//! `makespan`
//!     Minimizes elapsed schedule horizon.
//!
//! `depth`
//!     Minimizes dependency depth.
//!
//! `idle_time`
//!     Measures and optimizes resource idle intervals and retiming candidates.
//!
//! `fidelity`
//!     Provides a provider-neutral fidelity objective and fidelity model
//!     boundary.
//!
//! `multi_objective`
//!     Combines independently evaluated objective values through explicit
//!     scalarization, lexicographic ordering, and Pareto reasoning.
//!
//! Future objective modules may be added without changing the semantics of the
//! existing objectives.
//!
//! # Why no wildcard re-exports
//!
//! This module deliberately does NOT use:
//!
//! ```text
//! pub use makespan::*;
//! pub use depth::*;
//! pub use idle_time::*;
//! pub use fidelity::*;
//! pub use multi_objective::*;
//! ```
//!
//! Wildcard re-exports would make the public API unstable because adding an
//! unrelated public symbol to one objective module could silently change this
//! namespace.
//!
//! They can also introduce name collisions and make ownership unclear.
//!
//! Consumers should therefore use explicit paths:
//!
//! ```text
//! crate::quantum::scheduling::optimization::makespan
//! crate::quantum::scheduling::optimization::depth
//! crate::quantum::scheduling::optimization::idle_time
//! crate::quantum::scheduling::optimization::fidelity
//! crate::quantum::scheduling::optimization::multi_objective
//! ```
//!
//! # Stable namespace
//!
//! The namespace itself is intentionally stable.
//!
//! Individual objective implementations may evolve internally while preserving
//! their documented contracts.
//!
//! The composition root should not need modification merely because an
//! objective implementation adds an internal helper, private type, algorithm,
//! or diagnostic facility.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! scheduling
//!      │
//!      ▼
//! scheduling::result / canonical schedule
//!      │
//!      ▼
//! scheduling::optimization
//!      │
//!      ├── makespan
//!      ├── depth
//!      ├── idle_time
//!      ├── fidelity
//!      └── multi_objective
//! ```
//!
//! Optimization modules may be consumed by:
//!
//! ```text
//! scheduling::policies
//! scheduling::planners
//! scheduling::algorithms
//! scheduling::diagnostics
//! scheduling::benchmarking
//! ```
//!
//! They must not depend on those consumers merely to calculate their objective.
//!
//! In particular, this namespace must not introduce a dependency cycle such
//! as:
//!
//! ```text
//! optimization → planner → optimization
//! ```
//!
//! Objective implementations should depend only on the canonical inputs they
//! actually require.
//!
//! # Routing boundary
//!
//! Routing answers:
//!
//! > "WHERE should an operation execute?"
//!
//! Scheduling answers:
//!
//! > "WHEN should an operation execute?"
//!
//! Optimization answers:
//!
//! > "WHICH VALID SCHEDULE IS PREFERABLE?"
//!
//! Therefore optimization must not perform logical-to-physical routing.
//!
//! It may evaluate metrics that depend on routing results, but the actual
//! mapping remains owned by `quantum::routing`.
//!
//! # Hardware boundary
//!
//! Hardware-specific information must enter optimization through explicit
//! target/context/model interfaces.
//!
//! Optimization must not:
//!
//! - discover hardware;
//! - connect to a QPU;
//! - load credentials;
//! - query vendor APIs;
//! - mutate calibration state;
//! - perform device I/O.
//!
//! Hardware-specific models may provide information such as:
//!
//! - operation durations;
//! - fidelity estimates;
//! - resource costs;
//! - availability;
//! - energy estimates;
//! - timing constraints.
//!
//! Those models remain owned by the hardware/adaptation boundary.
//!
//! # ZQN boundary
//!
//! When the ZQN subsystem provides physical uncertainty information,
//! optimization may consume it through an explicit model/adapter.
//!
//! Optimization must not redefine:
//!
//! - noise channels;
//! - faults;
//! - stochastic distributions;
//! - drift;
//! - crosstalk;
//! - leakage;
//! - calibration uncertainty.
//!
//! A fidelity objective can consume a ZQN-derived model without making ZQN a
//! dependency of every optimization implementation.
//!
//! # QEC boundary
//!
//! Scheduling optimization may evaluate schedules produced for fault-tolerant
//! programs, but it must not become the owner of QEC semantics.
//!
//! For example:
//!
//! ```text
//! QEC
//!   │
//!   ▼
//! fault-tolerant operations
//!   │
//!   ▼
//! routing
//!   │
//!   ▼
//! scheduling
//!   │
//!   ▼
//! valid candidate schedules
//!   │
//!   ▼
//! optimization
//! ```
//!
//! Optimization can compare those candidates by makespan, depth, fidelity,
//! idle time, or a multi-objective policy.
//!
//! # Dynamic-circuit boundary
//!
//! Objective evaluation must remain compatible with schedules containing:
//!
//! - measurement;
//! - reset;
//! - classical dependencies;
//! - conditional operations;
//! - runtime feedback;
//! - dynamic timing;
//! - communication events.
//!
//! An objective must not assume that every quantum program is a static,
//! unconditional gate DAG.
//!
//! # Distributed boundary
//!
//! The optimization namespace must remain valid for:
//!
//! ```text
//! one qubit
//!      │
//!      ▼
//! one QPU
//!      │
//!      ▼
//! multi-chip system
//!      │
//!      ▼
//! distributed QPU
//!      │
//!      ▼
//! quantum network
//!      │
//!      ▼
//! future distributed quantum infrastructure
//! ```
//!
//! No objective may encode a fixed number of devices, links, channels, or
//! qubits.
//!
//! # Sparse scalability
//!
//! Optimization implementations should process only data represented by the
//! supplied schedule or candidate set.
//!
//! They must not allocate structures proportional to the entire declared
//! machine size when the schedule touches only a sparse subset of resources.
//!
//! For example, an optimization implementation must prefer:
//!
//! ```text
//! resources referenced by schedule
//! ```
//!
//! over:
//!
//! ```text
//! every resource physically present on the target
//! ```
//!
//! unless the selected objective explicitly requires the latter and the caller
//! has supplied that information.
//!
//! # Determinism
//!
//! The composition root performs no computation and therefore introduces no
//! randomness.
//!
//! Individual optimization modules must document their determinism contract.
//!
//! Where deterministic mode is supported:
//!
//! ```text
//! same canonical schedule
//! + same target/model snapshot
//! + same configuration
//! + same seed when stochasticity is explicitly required
//! = same optimization result
//! ```
//!
//! Semantic ordering must never depend on unspecified `HashMap` iteration.
//!
//! Deterministic ordering should use canonical ordered identities or explicit
//! caller-supplied ordering.
//!
//! # Numeric safety
//!
//! Scheduling time and durations must remain exact according to the canonical
//! scheduling timing model.
//!
//! Objective modules that require floating-point estimates must validate:
//!
//! - NaN;
//! - positive infinity;
//! - negative infinity;
//! - invalid ranges;
//! - invalid normalization;
//! - non-finite arithmetic results.
//!
//! The multi-objective implementation already establishes a finite floating
//! point boundary for objective values.
//!
//! # Overflow safety
//!
//! Optimization must never use wrapping arithmetic for semantic scheduling
//! values.
//!
//! Any operation involving:
//!
//! - time;
//! - duration;
//! - makespan;
//! - counters with semantic meaning;
//! - weighted objective calculations;
//! - resource costs;
//!
//! must either use checked arithmetic or a representation whose semantics
//! guarantee that overflow cannot silently corrupt the result.
//!
//! # Empty schedules
//!
//! Objective implementations must define their empty-schedule behavior.
//!
//! In particular:
//!
//! - makespan can legitimately be zero;
//! - dependency depth can legitimately be zero;
//! - idle-time analysis must not invent idle time without an explicit finite
//!   window;
//! - fidelity objectives must define whether an empty computation represents
//!   an identity/no-op estimate or requires explicit model semantics;
//! - multi-objective optimization must distinguish an empty candidate set from
//!   a valid empty schedule.
//!
//! These semantics belong to the individual objective modules, not this
//! namespace boundary.
//!
//! # Candidate validity
//!
//! Optimization should never mutate a candidate merely to make it appear
//! better.
//!
//! Candidate generation belongs to the appropriate planner/transformation.
//!
//! Candidate verification belongs to scheduling verification.
//!
//! Objective modules evaluate already-defined candidate semantics.
//!
//! The general contract is:
//!
//! ```text
//! candidate
//!     │
//!     ▼
//! verification
//!     │
//!     ├── invalid ──► reject
//!     │
//!     ▼
//! feasible candidate
//!     │
//!     ▼
//! objective evaluation
//!     │
//!     ▼
//! comparison
//! ```
//!
//! # No hidden defaults
//!
//! This namespace does not establish machine-specific defaults for:
//!
//! - qubit count;
//! - topology;
//! - duration;
//! - fidelity;
//! - energy;
//! - resource capacity;
//! - channel count;
//! - timing resolution;
//! - objective weight.
//!
//! Objective defaults, where mathematically meaningful, belong inside the
//! corresponding objective contract and must not represent a physical-machine
//! assumption.
//!
//! In particular, multi-objective weights must be explicitly supplied when
//! their relative importance is semantically significant.
//!
//! # Public module contract
//!
//! The currently implemented stable child modules are:
//!
//! ```text
//! makespan
//! depth
//! idle_time
//! fidelity
//! multi_objective
//! ```
//!
//! Their module declarations are intentionally kept together here so the
//! namespace has one authoritative Rust composition boundary.
//!
//! # Adding a new optimization module
//!
//! A future optimization module must satisfy all of the following before it is
//! declared here:
//!
//! 1. The file exists and compiles on Rust 1.97/1.97.1.
//! 2. It contains `#![forbid(unsafe_code)]`.
//! 3. It does not redefine canonical quantum identities.
//! 4. It does not redefine canonical scheduling types.
//! 5. It has no machine-size constants.
//! 6. It documents its ownership boundary.
//! 7. It documents its integration contract.
//! 8. It validates external inputs.
//! 9. It uses checked arithmetic where semantic arithmetic can overflow.
//! 10. It defines deterministic behavior where ordering is observable.
//! 11. It does not acquire hardware resources.
//! 12. It does not perform runtime execution.
//! 13. It does not introduce a dependency cycle.
//! 14. It has unit tests appropriate to its contract.
//! 15. It has integration tests where it crosses subsystem boundaries.
//!
//! Only after those requirements are satisfied should a new `pub mod`
//! declaration be added here.
//!
//! This deliberate requirement prevents an unfinished module from becoming a
//! dependency of the complete scheduling subsystem.
//!
//! # Compatibility
//!
//! The current repository contains a legacy stabilizer scheduler under:
//!
//! ```text
//! crate::quantum::scheduling::stabilizer_scheduler
//! ```
//!
//! That component should not be imported into this optimization namespace.
//!
//! Stabilizer/QEC scheduling belongs to the QEC-specific scheduling boundary,
//! while generic objective evaluation belongs here.
//!
//! The legacy implementation currently emits placeholder IR instructions and
//! comments rather than constructing a complete canonical schedule. It
//! therefore must not become an implicit dependency of objective evaluation.
//!
//! # Serialization
//!
//! This module does not own serialization.
//!
//! Objective-specific serialized data belongs to the owning objective module or
//! to the scheduling serialization boundary.
//!
//! This prevents the optimization namespace from becoming a second serialization
//! framework.
//!
//! # Diagnostics
//!
//! Objective-specific diagnostics belong to their owning modules.
//!
//! Cross-objective explanations belong to:
//!
//! ```text
//! crate::quantum::scheduling::diagnostics
//! ```
//!
//! This namespace should not depend on diagnostics merely to expose an
//! objective.
//!
//! # Benchmarking
//!
//! Benchmarking may consume optimization results to measure:
//!
//! - makespan improvement;
//! - depth improvement;
//! - idle-time reduction;
//! - fidelity improvement;
//! - energy/resource cost;
//! - Pareto-frontier characteristics;
//! - optimization runtime;
//! - memory consumption.
//!
//! Benchmarking remains an external consumer.
//!
//! # Performance contract
//!
//! The composition root itself performs no runtime work.
//!
//! Child optimization modules should:
//!
//! - avoid unnecessary schedule copies;
//! - prefer immutable borrowing where possible;
//! - avoid allocations proportional to unused target capacity;
//! - use streaming/iterator-based processing where practical;
//! - use iterative algorithms for potentially deep dependency structures;
//! - preserve deterministic ordering where requested;
//! - avoid global caches unless explicitly owned and synchronized by a separate
//!   subsystem contract.
//!
//! No optimization module may sacrifice correctness merely to achieve a
//! particular asymptotic bound.
//!
//! # Thread safety
//!
//! This namespace contains no mutable global state.
//!
//! Child objective implementations should remain safe to use concurrently when
//! their input contracts permit it.
//!
//! Any stateful optimizer must own its state explicitly rather than storing
//! scheduler state in globals.
//!
//! # Security and resource exhaustion
//!
//! Absence of artificial machine-size limits does not mean unbounded resource
//! consumption is acceptable.
//!
//! Resource limits must be explicit at the compiler, scheduler, or execution
//! policy boundary.
//!
//! This namespace must not silently introduce:
//!
//! - fixed operation limits;
//! - fixed candidate limits;
//! - fixed frontier limits;
//! - fixed memory limits;
//! - fixed qubit limits.
//!
//! If a caller requires such limits for security or resource control, they must
//! be passed explicitly through the relevant configuration.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! The safety rule is compiler-enforced below.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================
//
// This composition root must remain safe Rust. Every child optimization
// implementation should enforce the same rule independently.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Stable optimization modules
// =============================================================================

/// Makespan objective and schedule-horizon optimization.
///
/// This module owns makespan semantics and comparison but does not construct
/// schedules or discover hardware.
pub mod makespan;

/// Dependency-depth objective.
///
/// This module measures dependency depth independently of physical timing and
/// hardware topology.
pub mod depth;

/// Resource idle-time measurement and retiming-candidate generation.
///
/// This module does not commit retiming changes to the canonical schedule.
pub mod idle_time;

/// Provider-neutral fidelity objective and fidelity-model contract.
///
/// Hardware/ZQN-specific fidelity models are supplied through explicit
/// integration boundaries rather than embedded here.
pub mod fidelity;

/// Multi-objective scalarization, lexicographic comparison, and Pareto
/// optimization.
///
/// This module combines independently defined objective values and does not
/// construct schedules.
pub mod multi_objective;

// =============================================================================
// Public API policy
// =============================================================================
//
// Deliberately no wildcard re-exports:
//
//     pub use makespan::*;
//     pub use depth::*;
//     pub use idle_time::*;
//     pub use fidelity::*;
//     pub use multi_objective::*;
//
// Consumers should use explicit module-qualified paths. This keeps the public
// namespace stable when an individual objective module gains new symbols.
//
// Examples:
//
//     crate::quantum::scheduling::optimization::makespan::Makespan
//     crate::quantum::scheduling::optimization::depth::ScheduleDepth
//     crate::quantum::scheduling::optimization::idle_time::IdleTimeMetrics
//     crate::quantum::scheduling::optimization::fidelity::Fidelity
//     crate::quantum::scheduling::optimization::multi_objective::ObjectiveVector

// =============================================================================
// Compile-time namespace tests
// =============================================================================

#[cfg(test)]
mod tests {
    /// Verifies that the optimization composition root exposes the intended
    /// independent modules without introducing implementation state.
    #[test]
    fn optimization_namespace_is_composed_from_independent_modules() {
        let _ = super::makespan::Makespan::ZERO;
        let _ = super::depth::ScheduleDepth::ZERO;
        let _ = super::idle_time::IdleTimeResourceWeight::ONE;
        let _ = super::fidelity::Fidelity::PERFECT;
        let _ = super::multi_objective::ObjectiveDirection::Minimize;
    }
}