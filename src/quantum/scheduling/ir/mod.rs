//! Zamani Quantum Scheduling — Internal Scheduling IR
//!
//! # Module contract
//!
//! This module is the stable composition boundary for the scheduler's internal
//! intermediate representation.
//!
//! The scheduling IR is NOT the canonical quantum semantic IR.
//!
//! The ownership boundary is:
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::frontend
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      │ canonical quantum semantics
//!      ▼
//! quantum::optimization
//!      │
//!      ▼
//! quantum::routing
//!      │
//!      │ logical → target-compatible representation
//!      ▼
//! quantum::scheduling::adapters::ir
//!      │
//!      │ normalized scheduling view
//!      ▼
//! quantum::scheduling::ir
//!      │
//!      ├── operation
//!      ├── dependency
//!      ├── graph
//!      └── critical_path
//!      │
//!      ▼
//! quantum::scheduling::planners
//! ```
//!
//! # Responsibilities
//!
//! This module owns the public composition boundary for:
//!
//! - scheduler operation representations;
//! - scheduler dependency representations;
//! - dependency graphs;
//! - critical-path analysis;
//! - scheduler-IR errors and result types exposed by those child modules.
//!
//! It does NOT own:
//!
//! - canonical quantum semantics;
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - canonical `OperationId`;
//! - canonical `ResourceId`;
//! - hardware topology;
//! - hardware discovery;
//! - calibration;
//! - routing;
//! - scheduling policies;
//! - scheduling algorithms;
//! - resource calendars;
//! - timing models;
//! - QEC algorithms;
//! - runtime execution;
//! - serialization formats.
//!
//! Those responsibilities remain in their canonical subsystems.
//!
//! # Canonical identity rule
//!
//! Scheduling IR MUST NOT introduce another logical or physical qubit identity.
//!
//! When qubit identity is required, downstream scheduling IR uses:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The canonical IR remains authoritative for quantum identity.
//!
//! This is particularly important because Zamani's repository explicitly
//! establishes `quantum::ir::qubit` as the canonical logical/physical qubit
//! identity boundary.
//!
//! # Operation identity rule
//!
//! Scheduler-specific references may wrap or use scheduler-level views of
//! canonical operations, but this module MUST NOT redefine the semantic
//! operation model.
//!
//! In particular, this module must not introduce another:
//!
//! - `QuantumOperation`;
//! - `QuantumCircuit`;
//! - `Gate`;
//! - `QubitId`;
//! - `PhysicalQubitId`.
//!
//! The scheduler operates on normalized scheduling information derived from the
//! canonical IR.
//!
//! # Dependency direction
//!
//! ```text
//!                  quantum::ir
//!                      │
//!                      ▼
//!             scheduling adapter
//!                      │
//!                      ▼
//!       ┌──────────────────────────┐
//!       │ scheduling::ir::operation│
//!       └─────────────┬────────────┘
//!                     │
//!             ┌───────┴────────┐
//!             ▼                ▼
//!       dependency          timing/resources
//!             │                │
//!             ▼                │
//!          graph ◄─────────────┘
//!             │
//!             ▼
//!       critical_path
//!             │
//!             ▼
//!          planners
//! ```
//!
//! The dependency graph is intentionally independent of timing, resources,
//! policies and planners. This permits the same graph to be consumed by:
//!
//! - ASAP scheduling;
//! - ALAP scheduling;
//! - list scheduling;
//! - critical-path scheduling;
//! - resource-constrained scheduling;
//! - distributed scheduling;
//! - verification;
//! - diagnostics;
//! - future scheduling algorithms.
//!
//! # Scalability contract
//!
//! This module contains no machine-size assumptions.
//!
//! There are no constants defining:
//!
//! - maximum qubits;
//! - maximum operations;
//! - maximum dependencies;
//! - maximum graph depth;
//! - maximum graph width;
//! - maximum resource count;
//! - maximum scheduling horizon;
//! - maximum operation arity;
//! - maximum QEC distance;
//! - maximum number of scheduling rounds.
//!
//! A target is therefore free to provide whatever resources are available.
//!
//! "Infinity" in Zamani's architecture means:
//!
//! > no artificial finite machine-size ceiling is encoded in this module.
//!
//! A real compilation remains bounded by:
//!
//! - available memory;
//! - host address space;
//! - compilation time;
//! - explicit user/compiler limits;
//! - target resources;
//! - target capabilities;
//! - operating-system constraints.
//!
//! Those constraints must be represented explicitly by the surrounding
//! scheduler context and limits subsystem rather than hidden in this module.
//!
//! # Determinism
//!
//! The child graph implementation uses deterministic collections for canonical
//! graph ordering. This module preserves that deterministic contract by
//! providing stable module paths and exports.
//!
//! No global mutable state is introduced here.
//!
//! # Concurrency
//!
//! This module contains no locks, global state, thread-local scheduler state,
//! or interior mutability.
//!
//! Child data structures remain explicitly owned by their callers.
//!
//! This permits higher-level schedulers to use immutable scheduling snapshots,
//! `Arc`, parallel analysis, partitioned scheduling, and distributed scheduling
//! without making the IR module itself responsible for synchronization.
//!
//! # Safety
//!
//! Rust 1.97 / Rust 1.97.1.
//!
//! Rust 2021 edition.
//!
//! Stable Rust only.
//!
//! No nightly features.
//!
//! No `unsafe`.
//!
//! The compiler-enforced `forbid(unsafe_code)` below makes the no-unsafe
//! requirement explicit.
//!
//! # Integration contract
//!
//! The intended integration sequence is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! scheduling::adapters::ir
//!      │
//!      ├── validates canonical operation information
//!      ├── preserves canonical QubitId
//!      ├── preserves canonical operation identity
//!      └── constructs SchedulingOperation
//!      │
//!      ▼
//! scheduling::ir::operation
//!      │
//!      ▼
//! scheduling::ir::dependency
//!      │
//!      ▼
//! scheduling::ir::graph
//!      │
//!      ├──────────────► scheduling::ir::critical_path
//!      │
//!      ├──────────────► scheduling::planners
//!      ├──────────────► scheduling::verification
//!      └──────────────► scheduling::diagnostics
//! ```
//!
//! The hardware adapter supplies target-specific timing and resource
//! information separately.
//!
//! ```text
//! quantum::hardware
//!       │
//!       ▼
//! scheduling::adapters::hardware
//!       │
//!       ▼
//! scheduling::timing
//! scheduling::resources
//!       │
//!       ▼
//! scheduling::planners
//! ```
//!
//! Routing remains separate:
//!
//! ```text
//! quantum::routing
//!       │
//!       ▼
//! scheduling::adapters::routing
//!       │
//!       ▼
//! scheduling::ir
//! ```
//!
//! QEC remains separate:
//!
//! ```text
//! quantum::error_correction
//!       │
//!       ▼
//! scheduling::adapters::qec
//!       │
//!       ▼
//! scheduling::ir
//! ```
//!
//! # Stable public surface
//!
//! Child modules are declared here so the remainder of the scheduler can
//! depend on stable paths:
//!
//! ```text
//! quantum::scheduling::ir::operation
//! quantum::scheduling::ir::dependency
//! quantum::scheduling::ir::graph
//! quantum::scheduling::ir::critical_path
//! ```
//!
//! The module also re-exports the principal scheduler-IR types at this
//! boundary. This provides ergonomic imports without moving ownership away
//! from the child modules.
//!
//! # File-name compatibility
//!
//! The repository currently contains the operation implementation under a
//! filename with a trailing space:
//!
//! ```text
//! operation.rs 
//! ```
//!
//! A normal Rust module declaration expects:
//!
//! ```text
//! operation.rs
//! ```
//!
//! Therefore this module temporarily uses an explicit `#[path]` declaration
//! for compatibility with the repository's current tree.
//!
//! The production repository should rename the file to:
//!
//! ```text
//! src/quantum/scheduling/ir/operation.rs
//! ```
//!
//! After that repository hygiene correction, the declaration should be changed
//! from:
//!
//! ```rust
//! #[path = "operation.rs "]
//! pub mod operation;
//! ```
//!
//! to:
//!
//! ```rust
//! pub mod operation;
//! ```
//!
//! No semantic changes to this module should be required by that rename.
//!
//! # Why this module is intentionally small
//!
//! `mod.rs` is a composition root, not an implementation dumping ground.
//!
//! It must not contain:
//!
//! - graph algorithms;
//! - scheduling algorithms;
//! - dependency inference;
//! - timing calculations;
//! - resource allocation;
//! - hardware queries;
//! - routing;
//! - QEC logic;
//! - optimization logic;
//! - runtime logic.
//!
//! Keeping those responsibilities in their dedicated files is what allows a
//! single IR contract to scale from a tiny device to a very large or
//! distributed quantum system.
//!
//! # Future extension rule
//!
//! When a new scheduling-IR concern is introduced:
//!
//! 1. create a dedicated child module;
//! 2. give that module a complete ownership contract;
//! 3. make it depend only on lower-level scheduler contracts;
//! 4. avoid creating duplicate canonical IR identities;
//! 5. add its module declaration here;
//! 6. expose only the stable public API required by callers;
//! 7. add unit/property/integration tests in the corresponding test layer.
//!
//! Do not place implementation logic in this file merely to avoid creating a
//! new module.
//!
//! # Module inventory
//!
//! Current scheduler IR:
//!
//! ```text
//! ir/
//! ├── mod.rs
//! ├── operation.rs
//! ├── dependency.rs
//! ├── graph.rs
//! └── critical_path.rs
//! ```
//!
//! Planned scheduler extensions remain separate concerns:
//!
//! ```text
//! resources/*
//! timing/*
//! constraints/*
//! policies/*
//! planners/*
//! verification/*
//! transformations/*
//! ```
//!
//! Those modules should consume this IR rather than duplicate it.
//!
//! # Production invariants
//!
//! The scheduling IR boundary must preserve these invariants:
//!
//! 1. Canonical qubit identity remains canonical.
//! 2. Canonical operation identity remains canonical.
//! 3. Scheduler-specific metadata cannot silently redefine quantum semantics.
//! 4. Dependency endpoints refer to valid scheduling operations.
//! 5. Dependency graphs remain acyclic when used for static scheduling.
//! 6. Dynamic execution dependencies remain representable without pretending
//!    that runtime conditions are static.
//! 7. No scheduler IR type contains a fixed hardware size.
//! 8. No scheduler IR type contains a vendor-specific assumption.
//! 9. No scheduler IR type requires a particular quantum technology.
//! 10. No scheduler IR type performs hardware I/O.
//! 11. No scheduler IR type owns scheduling policy.
//! 12. No scheduler IR type owns execution.
//! 13. All graph traversals remain iterative where graph depth could be large.
//! 14. Failed graph mutations do not leave partially inconsistent graph state.
//! 15. Deterministic ordering remains available for reproducible compilation.
//!
//! # Versioning
//!
//! This module is part of the internal scheduler architecture but exposes
//! stable paths used by sibling scheduler modules.
//!
//! Adding a new child module is non-breaking.
//!
//! Renaming/removing an existing child module or changing a re-exported public
//! type is a compatibility-sensitive change and must be handled through the
//! repository's normal versioning/deprecation policy.
//!
//! # Testing contract
//!
//! This module itself contains composition tests rather than algorithm tests.
//!
//! Algorithm correctness belongs to the respective child modules.
//!
//! Integration testing must verify that:
//!
//! ```text
//! canonical IR
//!      ↓
//! scheduling adapter
//!      ↓
//! scheduling IR
//!      ↓
//! dependency graph
//!      ↓
//! critical path
//!      ↓
//! planner
//! ```
//!
//! preserves operation and qubit identity.
//!
//! Scalability testing must verify increasing graph sizes without relying on
//! any scheduler-IR constant representing a maximum machine size.
//!
//! # Public API
//!
//! The primary public imports supplied by this module are intentionally
//! limited to the stable scheduler-IR vocabulary.
//!
//! More specialized implementation details remain available from their
//! respective child modules when explicitly required.
//!
//! # No-unsafe policy
//!
//! This module deliberately contains:
//!
//! ```rust
//! #![forbid(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! ```
//!
//! This policy is inherited conceptually by every scheduler component and is
//! enforced independently in the child implementation files as well.
//!

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Child modules
// =============================================================================

// IMPORTANT:
// The current repository contains `operation.rs ` (with a trailing space).
// Keep this explicit path until the repository file is renamed to the normal
// `operation.rs` filename.
//
// Production tree target:
//     src/quantum/scheduling/ir/operation.rs
#[path = "operation.rs "]
pub mod operation;

pub mod dependency;
pub mod graph;
pub mod critical_path;

// =============================================================================
// Stable scheduler-IR exports
// =============================================================================
//
// Re-export only scheduler-IR concepts here.
//
// Canonical quantum IR types remain owned by quantum::ir and are intentionally
// not shadowed or recreated here.

// Operation model.
pub use operation::{
    ClassicalDependencyId,
    OperationClass,
    OperationMetadata,
    OperationProvenance,
    OperandRole,
    QubitOperand,
    SchedulingOperation,
};

// Dependency model.
pub use dependency::{
    Dependency,
    DependencyAnalysis,
    DependencyAnalyzer,
    DependencyError,
    DependencyKind,
    DependencyRef,
};

// Graph model.
pub use graph::{
    DependencyGraph,
    DependencyGraphError,
    DependencyGraphResult,
};

// Critical-path model.
//
// The exact public analysis/result vocabulary is intentionally re-exported
// only when supplied by the implementation. The child module remains the
// owner of the concrete analysis structures.
//
// `OperationCriticalPathInfo` is part of the current implementation's public
// analysis vocabulary.
pub use critical_path::OperationCriticalPathInfo;

// =============================================================================
// Canonical identity guidance
// =============================================================================
//
// DO NOT add aliases such as:
//
// pub type QubitId = ...;
// pub type PhysicalQubitId = ...;
//
// here.
//
// New code requiring logical/physical qubit identity must import:
//
// use crate::quantum::ir::qubit::QubitId;
// use crate::quantum::ir::qubit::PhysicalQubitId;
//
// The canonical repository contract explicitly places these identities under
// quantum::ir::qubit.
//
// Likewise, do not create another OperationId or ResourceId here. Use the
// canonical scheduler/types or canonical IR identity types according to the
// ownership contract of the consuming module.

// =============================================================================
// Internal architecture notes
// =============================================================================
//
// This module deliberately does NOT expose blanket:
//
//     pub use dependency::*;
//     pub use graph::*;
//     pub use critical_path::*;
//
// exports.
//
// Explicit exports make the scheduler's public surface reviewable and prevent
// accidental API expansion when implementation-only symbols are added to a
// child module.
//
// If a new child type becomes part of the stable scheduler-IR API, add it here
// deliberately and document its ownership contract.
//
// =============================================================================
// Integration checkpoints
// =============================================================================
//
// CHECKPOINT 1 — Canonical IR
//
// `scheduling::ir` must consume canonical IR information through adapters.
// It must not parse Zamani source or reconstruct semantic quantum operations.
//
// CHECKPOINT 2 — Qubit identity
//
// All logical/physical qubit identity must remain canonical under:
//
//     crate::quantum::ir::qubit
//
// CHECKPOINT 3 — Routing
//
// Routing supplies target-compatible placement information. Scheduling does
// not decide placement.
//
// CHECKPOINT 4 — Timing
//
// Timing modules attach target-specific timing semantics after this IR has
// established operation/dependency structure.
//
// CHECKPOINT 5 — Resources
//
// Resource modules attach control/readout/communication/compute/etc. resource
// requirements without modifying the semantic operation model.
//
// CHECKPOINT 6 — Planning
//
// Planners consume this IR. They do not mutate canonical quantum semantics.
//
// CHECKPOINT 7 — Verification
//
// Verification checks that the resulting schedule preserves dependency and
// semantic invariants.
//
// CHECKPOINT 8 — Runtime
//
// Runtime consumes the verified schedule. This module performs no execution.
//
// =============================================================================
// Scalability boundary
// =============================================================================
//
// The scheduler IR must remain valid for:
//
//     one operation
//     one qubit
//     one device
//
// through:
//
//     large circuits
//     large QEC patches
//     multi-chip systems
//     multi-QPU systems
//     distributed quantum networks
//     heterogeneous quantum/classical systems
//
// without changing the semantic source program.
//
// Scaling is achieved by changing the target/resource/timing/routing context,
// not by changing this IR.
//
// =============================================================================
// End of module contract
// =============================================================================