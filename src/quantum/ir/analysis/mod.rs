//! Zamani Quantum IR — Analysis Module
//!
//! Path:
//!     src/quantum/ir/analysis/mod.rs
//!
//! # Purpose
//!
//! This module is the stable public boundary for all read-only analysis of
//! Zamani's canonical Quantum IR.
//!
//! Analysis answers:
//!
//! > What properties can be derived from the semantic IR without changing
//! > what the program means?
//!
//! Analysis is intentionally downstream of the canonical IR and upstream of
//! consumers such as optimization, routing, scheduling, resource planning,
//! benchmarking, diagnostics, visualization and compilation planning.
//!
//! # Architectural boundary
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! frontend
//!      │
//!      ▼
//! ┌───────────────────────┐
//! │ canonical Quantum IR  │
//! └───────────┬───────────┘
//!             │
//!             ▼
//! ┌───────────────────────┐
//! │ quantum::ir::analysis │
//! │       READ ONLY       │
//! └───────────┬───────────┘
//!             │
//!       ┌─────┼──────┬──────────┬───────────┐
//!       ▼     ▼      ▼          ▼           ▼
//!    optimize route schedule resource   benchmark
//! ```
//!
//! Analysis MUST NOT:
//!
//! - mutate canonical IR;
//! - optimize;
//! - rewrite;
//! - route;
//! - allocate physical qubits;
//! - select hardware;
//! - select a backend;
//! - select native instructions;
//! - perform calibration;
//! - synthesize hardware pulses;
//! - execute a QPU;
//! - simulate quantum state;
//! - decode QEC syndromes;
//! - introduce hardware-specific dependencies.
//!
//! Those responsibilities belong to other subsystems.
//!
//! # Universal quantum-computing principle
//!
//! The canonical Zamani IR is intentionally broader than a gate-oriented
//! circuit. Analysis must therefore be layered so that circuit analysis does
//! not become the architectural definition of quantum computation.
//!
//! The eventual analysis surface must be able to cover:
//!
//! - static circuits;
//! - dynamic circuits;
//! - classical computation;
//! - measurement;
//! - classical feedback;
//! - symbolic parameters;
//! - pulse programs;
//! - waveform programs;
//! - timing;
//! - analog/Hamiltonian programs;
//! - annealing;
//! - QUBO;
//! - measurement-based computation;
//! - continuous-variable computation;
//! - fermionic/bosonic models;
//! - logical/fault-tolerant computation;
//! - distributed quantum computation;
//! - future dialects and extensions.
//!
//! # Scalability contract
//!
//! "Scale from atom to everywhere" means that this module introduces no
//! artificial quantum-machine size.
//!
//! It MUST NOT contain architectural constants such as:
//!
//! ```text
//! MAX_QUBITS = 64
//! MAX_QUBITS = 128
//! MAX_REGISTER_SIZE = 4096
//! ```
//!
//! A program containing:
//!
//! ```text
//! 1
//! 2
//! 64
//! 1_000
//! 1_000_000
//! N
//! ```
//!
//! logical qubits is valid whenever the canonical IR, active resource policy,
//! host resources and eventual target can represent it.
//!
//! "Infinity" therefore means:
//!
//! > no artificial finite machine-size ceiling is introduced by analysis.
//!
//! A concrete in-memory program remains finite and is constrained by the
//! available address space, memory, execution time and explicit resource
//! policy.
//!
//! # Sparse-analysis principle
//!
//! Analysis modules MUST NOT allocate storage proportional to a declared
//! namespace when the analysis only needs resources actually referenced by the
//! program.
//!
//! Prefer:
//!
//! ```text
//! resource identity -> derived information
//! ```
//!
//! rather than:
//!
//! ```text
//! Vec<DerivedInformation> sized to declared resource count
//! ```
//!
//! This is especially important for sparse programs with enormous logical
//! namespaces.
//!
//! # Determinism
//!
//! Public analysis results MUST be deterministic.
//!
//! The analysis layer therefore prefers:
//!
//! - source/program order where order has semantic meaning;
//! - `BTreeMap` / `BTreeSet` where sorted deterministic lookup is required;
//! - explicitly ordered vectors for ordered results;
//! - stable IDs from `quantum::ir::identity`;
//! - checked arithmetic;
//! - no global mutable state;
//! - no dependency on randomized hash iteration;
//! - no host-specific ordering.
//!
//! Derived analysis must never make canonical IR hashing nondeterministic.
//!
//! # Canonical identity
//!
//! All quantum analysis MUST use the canonical identity types owned by:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! In particular:
//!
//! ```rust
//! use crate::quantum::ir::qubit::QubitId;
//! ```
//!
//! New analysis code MUST NOT introduce another `QubitId`.
//!
//! Logical and physical identities must remain distinct.
//!
//! Analysis of logical semantics must not silently reinterpret a logical
//! `QubitId` as a physical hardware location.
//!
//! # Resource-policy boundary
//!
//! `QuantumIrLimits` is a per-analysis/per-compilation resource and security
//! policy.
//!
//! It is NOT:
//!
//! - the maximum number of qubits Zamani supports;
//! - the maximum size of a quantum computer;
//! - a hardware capability description;
//! - a topology description.
//!
//! Conceptually:
//!
//! ```text
//! QuantumIrLimits
//!     = how much work this invocation permits
//!
//! hardware capability
//!     = what the selected target can provide
//!
//! IR semantics
//!     = what the program means
//! ```
//!
//! Analysis must preserve this separation.
//!
//! # Error policy
//!
//! Analysis is fallible.
//!
//! It MUST NOT silently:
//!
//! - skip malformed operations;
//! - ignore invalid qubits;
//! - ignore invalid classical references;
//! - truncate counters;
//! - saturate semantic counts without reporting it;
//! - ignore unknown semantic objects;
//! - convert overflow into plausible-looking results.
//!
//! Expensive analyses should reject work that exceeds the active
//! `QuantumIrLimits` before performing avoidable allocations.
//!
//! # Ownership
//!
//! This module owns the public organization and integration boundary of
//! analysis.
//!
//! Individual child modules own their specific derived information.
//!
//! ```text
//! analysis/
//! ├── mod.rs
//! │   └── public analysis boundary — THIS FILE
//! │
//! ├── analysis.rs
//! │   └── circuit-level statistics and basic analysis
//! │
//! ├── dependencies.rs
//! │   └── semantic dependency graph
//! │
//! ├── liveness.rs
//! │   └── value/resource liveness
//! │
//! ├── resource_usage.rs
//! │   └── abstract resource consumption
//! │
//! ├── properties.rs
//! │   └── derived semantic properties
//! │
//! ├── statistics.rs
//! │   └── reusable aggregate/statistical analysis
//! │
//! ├── operation.rs
//! │   └── universal operation-level analysis
//! │
//! └── program.rs
//!     └── whole-program analysis
//! ```
//!
//! Only modules that are actually implemented and part of the current
//! repository contract should be enabled below.
//!
//! # Dependency direction
//!
//! ```text
//! quantum::ir::core / identity / types / value
//!                 │
//!                 ▼
//!        quantum::ir::operation
//!                 │
//!                 ▼
//!       quantum::ir::program
//!                 │
//!                 ▼
//!      quantum::ir::analysis
//!                 │
//!       ┌─────────┼─────────┐
//!       ▼         ▼         ▼
//! optimization routing scheduling
//! ```
//!
//! Analysis MUST NOT import those downstream systems.
//!
//! # Derived-data rule
//!
//! Analysis results are derived data.
//!
//! They are not automatically part of the canonical semantic IR.
//!
//! Therefore:
//!
//! ```text
//! canonical IR hash
//!       ≠
//! analysis result hash
//! ```
//!
//! unless a future IR specification explicitly promotes an analysis result
//! into a semantic IR object.
//!
//! Serialization of analysis artifacts must likewise remain separate from
//! canonical IR serialization unless explicitly specified otherwise.
//!
//! # Incremental-analysis compatibility
//!
//! Child modules should be designed so future incremental compilation can
//! invalidate only affected analysis.
//!
//! This means analysis APIs should prefer stable identities and explicit input
//! contracts over hidden global state.
//!
//! A future incremental engine may use:
//!
//! ```text
//! OperationId
//! QubitId
//! ClassicalBitId
//! RegionId
//! BlockId
//! ```
//!
//! as invalidation keys.
//!
//! # Thread-safety
//!
//! The module owns no mutable global state.
//!
//! Analysis functions should normally accept immutable references and return
//! owned immutable results.
//!
//! This makes independent analyses suitable for future parallel execution
//! without changing semantic behavior.
//!
//! Parallel execution is an implementation concern and MUST NOT change result
//! ordering or meaning.
//!
//! # Rust compatibility
//!
//! Required:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe.
//!
//! The no-unsafe requirement is compiler-enforced.
//!
//! # No-unsafe contract
//!
//! This module deliberately forbids unsafe code even if a future dependency
//! or implementation would otherwise make it tempting.
//!
//! Analysis must remain memory-safe and portable.
//!
//! # Public API policy
//!
//! This module uses two levels of API:
//!
//! 1. Child-module APIs remain available through their canonical module path.
//! 2. Only deliberately stable, high-value analysis APIs are re-exported here.
//!
//! This prevents `quantum::ir::analysis::*` from becoming an uncontrolled
//! namespace containing every internal implementation detail.
//!
//! Downstream code should prefer:
//!
//! ```text
//! quantum::ir::analysis::dependencies
//! quantum::ir::analysis::liveness
//! quantum::ir::analysis::resource_usage
//! ```
//!
//! for specialized analysis.
//!
//! # Compatibility policy
//!
//! The old flat file:
//!
//! ```text
//! src/quantum/ir/analysis.rs
//! ```
//!
//! must be migrated to:
//!
//! ```text
//! src/quantum/ir/analysis/analysis.rs
//! ```
//!
//! before this directory module is enabled.
//!
//! Rust does not permit both:
//!
//! ```text
//! analysis.rs
//! analysis/mod.rs
//! ```
//!
//! to define the same `analysis` module.
//!
//! The migration is therefore:
//!
//! ```text
//! BEFORE
//!
//! quantum/ir/
//! └── analysis.rs
//!
//! AFTER
//!
//! quantum/ir/
//! └── analysis/
//!     ├── mod.rs
//!     ├── analysis.rs
//!     └── dependencies.rs
//! ```
//!
//! No semantic rewrite is required merely because of the file move.
//!
//! # Integration contract
//!
//! Parent module:
//!
//! ```text
//! quantum::ir::mod.rs
//! ```
//!
//! declares:
//!
//! ```rust
//! pub mod analysis;
//! ```
//!
//! Consumers then use:
//!
//! ```rust
//! use crate::quantum::ir::analysis;
//! ```
//!
//! or a specific API:
//!
//! ```rust
//! use crate::quantum::ir::analysis::analyze;
//! ```
//!
//! Circuit-specific analysis consumes:
//!
//! ```text
//! quantum::ir::circuit::QuantumCircuit
//! quantum::ir::gate::Gate
//! quantum::ir::qubit::QubitId
//! quantum::ir::limits::QuantumIrLimits
//! ```
//!
//! Universal analysis consumes the broader:
//!
//! ```text
//! quantum::ir::operation
//! quantum::ir::program
//! quantum::ir::region
//! ```
//!
//! Hardware-specific analysis must NOT be added here. It belongs downstream.
//!
//! # Stability rule
//!
//! Adding a new analysis implementation should normally require only:
//!
//! 1. adding the child file;
//! 2. adding its module declaration here;
//! 3. optionally adding a deliberate re-export.
//!
//! Existing analysis modules must not be edited merely because another
//! analysis module was added.
//!
//! This is the required "finish one file and do not reopen it later" property.
//!
//! # Testing
//!
//! Cross-module tests should live under the IR test architecture and should
//! verify:
//!
//! - deterministic results;
//! - sparse behavior;
//! - no fixed qubit limits;
//! - checked arithmetic;
//! - invalid-input rejection;
//! - stable ordering;
//! - canonical `QubitId` usage;
//! - no mutation of input IR;
//! - compatibility with `QuantumIrLimits`;
//! - independence from hardware/backend modules.
//!
//! Module-specific unit tests belong in the child implementation files.
//!
//! # Module declarations
//!
//! The declarations below intentionally form the analysis-layer boundary.
//!
//! `analysis.rs` is the migrated home of the existing circuit analysis.
//! `dependencies.rs` is the existing sparse dependency analysis.
//!
//! Additional modules should be enabled only when their implementation is
//! present. This avoids creating a permanently broken compile boundary merely
//! because the architecture anticipates future analysis categories.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Core analysis implementations
// =============================================================================

/// Circuit-level deterministic statistics and structural analysis.
///
/// This is the migrated implementation of the former:
///
/// `quantum::ir::analysis.rs`
pub mod analysis;

/// Sparse semantic dependency analysis.
///
/// This module derives operation dependencies without introducing hardware
/// topology or scheduling constraints.
pub mod dependencies;

// =============================================================================
// Future universal analysis modules
// =============================================================================
//
// These are intentionally documented here before activation.
//
// Once each corresponding implementation exists and has its own frozen
// integration contract, add exactly one declaration here:
//
// pub mod liveness;
// pub mod operation;
// pub mod program;
// pub mod properties;
// pub mod resource_usage;
// pub mod statistics;
//
// Do NOT add speculative declarations for files that do not yet exist.
// That keeps this module compilable at every migration stage.

// =============================================================================
// Stable high-value re-exports
// =============================================================================
//
// Re-export only the circuit-analysis APIs that already form part of the
// stable public IR analysis contract.
//
// Specialized consumers should use the child module path directly.

pub use analysis::{
    analyze,
    analyze_with_limits,
    basic_statistics,
    basic_statistics_with_limits,
    qubit_usage,
    qubit_usage_with_limits,
    classical_bit_usage,
    classical_bit_usage_with_limits,
    arity_histogram,
    gate_histogram,
    logical_depth,
    CircuitStatistics,
    BasicCircuitStatistics,
    ArityCount,
    GateKindCount,
    QubitUsage,
    ClassicalBitUsage,
};

// Dependency analysis remains available through:
//
//     quantum::ir::analysis::dependencies
//
// rather than being flattened into this namespace.
//
// This prevents future additions from creating accidental API collisions.

// =============================================================================
// Integration assertions
// =============================================================================
//
// These are intentionally compile-time API expectations expressed through
// documentation rather than executable initialization.
//
// Canonical qubit identity:
//
//     quantum::ir::qubit::QubitId
//
// Resource policy:
//
//     quantum::ir::limits::QuantumIrLimits
//
// Circuit:
//
//     quantum::ir::circuit::QuantumCircuit
//
// Gate:
//
//     quantum::ir::gate::Gate
//
// Universal operation:
//
//     quantum::ir::operation::Operation
//
// Analysis must remain a consumer of these contracts and must not redefine
// them.

// =============================================================================
// Architectural prohibition
// =============================================================================
//
// Do not add any of the following dependencies here:
//
//     quantum::hardware
//     quantum::backend
//     quantum::routing
//     quantum::scheduling
//     quantum::optimization
//     quantum::simulator
//     quantum::qec
//     quantum::frontend
//
// Those systems may consume analysis results, but analysis must remain
// independent of their implementation details.