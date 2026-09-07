//! Zamani Quantum Computing Subsystem.
//!
//! Path:
//!     `src/quantum/mod.rs`
//!
//! Status:
//!     Production composition boundary.
//!
//! Rust:
//!     Rust 1.97 / Rust 1.97.1
//!     Edition 2021
//!
//! Safety:
//!     This module and the quantum namespace are designed for safe Rust.
//!     `unsafe` is not permitted by the quantum architecture.
//!
//! # Purpose
//!
//! `crate::quantum` is the authoritative namespace boundary for Zamani's
//! quantum-computing architecture.
//!
//! This module composes quantum subsystems. It does not implement quantum
//! algorithms, IR semantics, routing, scheduling, hardware execution,
//! simulation, error correction, noise modelling, or resilience algorithms.
//!
//! Each responsibility belongs to exactly one child subsystem.
//!
//! The most important semantic boundary is:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! The canonical quantum IR defines what a quantum computation means.
//! Other subsystems determine how that computation is optimized, mapped,
//! scheduled, executed, characterized, protected, and recovered.
//!
//! # Architectural objective
//!
//! Zamani quantum programs follow the principle:
//!
//! > Write the quantum computation once and allow the implementation to scale
//! > across the resources and capabilities actually available to the execution.
//!
//! The quantum namespace therefore imposes no artificial finite machine-size
//! limit.
//!
//! It must not encode assumptions about:
//!
//! - a particular number of qubits;
//! - a particular number of physical qubits;
//! - a particular number of logical qubits;
//! - a particular circuit depth;
//! - a particular gate count;
//! - a particular gate arity;
//! - a particular topology size;
//! - a particular number of devices;
//! - a particular number of QPUs;
//! - a particular vendor;
//! - a particular quantum technology;
//! - a particular execution architecture.
//!
//! "Scale to infinity" means that the architecture does not impose an
//! artificial finite semantic ceiling. Every concrete compilation, simulation,
//! or execution remains bounded by the resources, capabilities, policies,
//! security controls, operating system, and physical machine available to that
//! invocation.
//!
//! Resource limits therefore belong to explicit resource/capability contracts,
//! not to this composition root.
//!
//! # Canonical architecture
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
//!       ┌──────────────────────┼──────────────────────┐
//!       │                      │                      │
//!       ▼                      ▼                      ▼
//!  algorithms            optimization                ZQN
//!       │                      │                      │
//!       └──────────────────────┼──────────────────────┘
//!                              │
//!                    ┌─────────┼─────────┐
//!                    │         │         │
//!                    ▼         ▼         ▼
//!                 routing  scheduling   QEC
//!                    │         │         │
//!                    └─────────┼─────────┘
//!                              ▼
//!                    quantum::hardware
//!                              │
//!                              ▼
//!                         execution
//!                              │
//!                  ┌───────────┴───────────┐
//!                  │                       │
//!                  ▼                       ▼
//!              observations              results
//!                  │                       │
//!                  └───────────┬───────────┘
//!                              ▼
//!                    quantum::resilience
//!                              │
//!          ┌───────────────────┼───────────────────┐
//!          │                   │                   │
//!          ▼                   ▼                   ▼
//!       detection           diagnosis         verification
//!          │                   │                   │
//!          └───────────────────┼───────────────────┘
//!                              ▼
//!                           policy
//!                              │
//!                              ▼
//!                           planning
//!                              │
//!                              ▼
//!                          adaptation
//!                              │
//!              ┌───────────────┼───────────────┐
//!              │               │               │
//!              ▼               ▼               ▼
//!           reroute       reschedule       recompile
//!              │               │               │
//!              └───────────────┼───────────────┘
//!                              ▼
//!                           recovery
//!                              │
//!                              ▼
//!                          mitigation
//!                              │
//!                              ▼
//!                         verification
//!                              │
//!                    ┌─────────┴─────────┐
//!                    ▼                   ▼
//!                 ACCEPT          REPEAT/ESCALATE
//! ```
//!
//! The feedback loop is a first-class architectural property.
//!
//! # Public quantum subsystems
//!
//! Each declaration below represents an ownership boundary. The child module
//! owns its own implementation, API, invariants, tests, compatibility rules,
//! and integration contracts.
//!
//! This root intentionally does not re-export individual types from those
//! modules.
//!
//! ## Algorithms
//!
//! `quantum::algorithms` owns backend-independent quantum algorithm
//! construction and orchestration.
//!
//! It must not own:
//!
//! - canonical IR semantics;
//! - physical hardware;
//! - provider credentials;
//! - routing;
//! - scheduling;
//! - execution.
//!
//! ```text
//! program/algorithm
//!     └──► quantum::ir
//! ```
//!
//! ## Benchmarking
//!
//! `quantum::benchmarking` owns benchmark protocols, workload generation,
//! execution contracts, statistical analysis, characterization, metrics,
//! regression detection, and reporting.
//!
//! Benchmarking observes the other quantum subsystems; it is not the semantic
//! foundation of the quantum namespace.
//!
//! ## Error correction
//!
//! `quantum::error_correction` owns quantum error-correction codes,
//! encoding, syndrome processing, decoding, correction, logical fault
//! tolerance, and related mechanisms.
//!
//! Resilience decides when QEC should be adapted or invoked, but does not
//! duplicate QEC implementations.
//!
//! ## Frontend
//!
//! `quantum::frontend` owns parsing and lowering from Zamani quantum syntax
//! and supported external formats into canonical `quantum::ir`.
//!
//! The canonical IR must not depend on frontend ASTs.
//!
//! ```text
//! frontend AST
//!     │
//!     ▼
//! canonical quantum IR
//! ```
//!
//! ## Hardware
//!
//! `quantum::hardware` owns the provider-neutral hardware abstraction layer,
//! capabilities, instruction sets, topology, calibration, device state,
//! execution contracts, provider adapters, and hardware discovery.
//!
//! Provider-specific details remain below the hardware abstraction boundary.
//!
//! Resilience consumes hardware capabilities and execution observations; it
//! does not redefine the hardware abstraction.
//!
//! ## IR
//!
//! `quantum::ir` is the canonical semantic representation of a quantum
//! computation.
//!
//! It is the stable boundary between frontend and downstream quantum
//! compilation/execution infrastructure.
//!
//! The IR must remain independent of:
//!
//! - vendor SDKs;
//! - credentials;
//! - network clients;
//! - hardware connections;
//! - backend implementations;
//! - routing implementations;
//! - scheduling implementations;
//! - QEC implementations;
//! - benchmarking implementations;
//! - resilience implementations.
//!
//! ## Memory
//!
//! `quantum::memory` owns quantum/hybrid memory and state-resource management.
//!
//! It must not create a competing canonical qubit identity or quantum IR.
//!
//! ## Optimization
//!
//! `quantum::optimization` owns backend-independent transformations of the
//! canonical quantum IR.
//!
//! Optimization may consume target capability/cost information, but it does
//! not own hardware topology or execution.
//!
//! ## Resilience
//!
//! `quantum::resilience` owns the provider-neutral resilience decision and
//! orchestration layer.
//!
//! It coordinates:
//!
//! - detection;
//! - diagnosis;
//! - policy;
//! - planning;
//! - adaptation;
//! - recovery;
//! - mitigation;
//! - verification;
//! - checkpoint coordination;
//! - provenance;
//! - telemetry;
//! - resilience history;
//! - optional predictive learning;
//! - distributed resilience coordination.
//!
//! It does not replace the canonical IR, ZQN, hardware, routing, scheduling,
//! optimization, QEC, or execution subsystems.
//!
//! Its integration point is deliberately one module:
//!
//! ```rust
//! pub mod resilience;
//! ```
//!
//! The resilience subsystem itself owns its internal module graph.
//!
//! ## Routing
//!
//! `quantum::routing` owns logical-to-physical mapping and connectivity-aware
//! transformations.
//!
//! Routing must use canonical qubit identities from:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! Resilience may request rerouting, but does not implement a routing
//! algorithm.
//!
//! ## Scheduling
//!
//! `quantum::scheduling` owns operation ordering, dependencies, timing,
//! resource conflicts, and schedule construction.
//!
//! Resilience may request rescheduling after resource, routing, timing,
//! calibration, or capability changes.
//!
//! ## ZQN
//!
//! `quantum::zqn` owns canonical Zamani Quantum Noise semantics, including
//! noise, faults, uncertainty, stochastic effects, calibration-dependent
//! imperfections, characterization, and related physical uncertainty.
//!
//! ZQN is not a competing quantum IR.
//!
//! ```text
//! quantum::ir
//!     │
//!     │ computation semantics
//!     ▼
//! quantum::zqn
//!     │
//!     │ physical uncertainty/noise/fault semantics
//!     ▼
//! routing / scheduling / QEC / simulation / hardware / resilience
//! ```
//!
//! Resilience must consume the canonical ZQN fault/noise contracts rather than
//! creating a second quantum fault ontology.
//!
//! # Canonical qubit identity
//!
//! The authoritative qubit identity implementation is:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! Quantum subsystems that require canonical qubit identity must use the types
//! owned by that module, including where available:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This root deliberately defines neither type.
//!
//! It must never introduce another root-level `QubitId`.
//!
//! In particular, the following pattern is prohibited:
//!
//! ```text
//! quantum::QubitId
//! quantum::resilience::QubitId
//! quantum::routing::QubitId
//! quantum::hardware::QubitId
//! ```
//!
//! when those names attempt to represent the same canonical quantum identity.
//!
//! A subsystem may define a distinct identifier only when its semantics are
//! genuinely different and the type is explicitly named according to its
//! domain.
//!
//! # Dependency direction
//!
//! The composition boundary follows these architectural rules:
//!
//! ```text
//! frontend
//!     │
//!     ▼
//!    IR
//!     │
//!     ├──────────────► algorithms
//!     ├──────────────► optimization
//!     ├──────────────► routing
//!     ├──────────────► scheduling
//!     └──────────────► analysis/validation
//!
//! IR + target capabilities
//!     │
//!     ├──────────────► routing
//!     ├──────────────► scheduling
//!     └──────────────► lowering
//!
//! IR + ZQN
//!     │
//!     ├──────────────► simulation
//!     ├──────────────► QEC
//!     ├──────────────► characterization
//!     └──────────────► resilience
//!
//! compiled representation
//!     │
//!     ▼
//! hardware
//!     │
//!     ▼
//! execution
//!     │
//!     ▼
//! observations/results
//!     │
//!     ├──────────────► benchmarking
//!     └──────────────► resilience
//! ```
//!
//! The following reverse dependencies are prohibited architecturally:
//!
//! ```text
//! IR ──X──► frontend implementation
//! IR ──X──► hardware implementation
//! IR ──X──► routing implementation
//! IR ──X──► scheduling implementation
//! IR ──X──► benchmarking implementation
//! IR ──X──► resilience implementation
//! IR ──X──► vendor SDK
//!
//! ZQN ──X──► vendor SDK
//! ZQN ──X──► credentials
//! ZQN ──X──► UI
//! ZQN ──X──► CLI
//!
//! frontend ──X──► hardware execution
//! algorithms ──X──► vendor API
//! ```
//!
//! # Write-once/scalable architecture
//!
//! This composition root intentionally contains no values such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_PHYSICAL_QUBITS
//! MAX_LOGICAL_QUBITS
//! MAX_OPERATIONS
//! MAX_DEPTH
//! MAX_GATE_ARITY
//! MAX_DEVICES
//! MAX_BACKENDS
//! MAX_RETRIES
//! DEFAULT_QUBIT_COUNT
//! DEFAULT_TOPOLOGY
//! ```
//!
//! No provider or hardware technology is selected here.
//!
//! A concrete target supplies capabilities through the hardware abstraction.
//! A concrete execution supplies resource limits through explicit policies.
//! A concrete compiler invocation supplies compilation resources.
//! A simulator supplies its representation/resource limits.
//! A resilience policy supplies recovery budgets.
//!
//! Therefore a larger machine does not require a different `quantum/mod.rs`.
//!
//! # Why `zqml` and `zqnp` are not exposed here
//!
//! The repository currently contains `quantum/zqml` and `quantum/zqnp`
//! directories, but their current `mod.rs` files are placeholders rather than
//! complete Rust module boundaries.
//!
//! Exposing either module here would turn an unfinished subsystem into a
//! mandatory dependency of the entire quantum namespace.
//!
//! They must be added only after their own module trees contain valid Rust
//! composition roots and their integration contracts are complete.
//!
//! This is intentional production behavior:
//!
//! > A directory existing in the source tree is not sufficient justification
//! > for exposing it as a compiled Rust module.
//!
//! # Why resilience is exposed here
//!
//! `quantum/resilience/mod.rs` is a complete composition boundary for the
//! resilience architecture and explicitly defines the integration contract
//! with this root.
//!
//! Resilience therefore belongs directly under:
//!
//! ```text
//! crate::quantum::resilience
//! ```
//!
//! Individual resilience children must remain below that boundary:
//!
//! ```text
//! quantum::resilience::api
//! quantum::resilience::model
//! quantum::resilience::detection
//! quantum::resilience::diagnosis
//! quantum::resilience::policy
//! quantum::resilience::planning
//! quantum::resilience::adaptation
//! quantum::resilience::recovery
//! quantum::resilience::mitigation
//! quantum::resilience::verification
//! quantum::resilience::state
//! quantum::resilience::checkpoint
//! quantum::resilience::telemetry
//! quantum::resilience::history
//! quantum::resilience::learning
//! quantum::resilience::coordination
//! quantum::resilience::serialization
//! quantum::resilience::errors
//! quantum::resilience::limits
//! quantum::resilience::registry
//! ```
//!
//! They must not be promoted to:
//!
//! ```text
//! quantum::planning
//! quantum::recovery
//! quantum::verification
//! quantum::mitigation
//! ```
//!
//! because doing so would destroy the ownership boundary of the resilience
//! subsystem.
//!
//! # No global state
//!
//! This composition root performs no initialization.
//!
//! It does not:
//!
//! - connect to hardware;
//! - discover devices;
//! - load credentials;
//! - initialize a simulator;
//! - allocate quantum state;
//! - create threads;
//! - access the network;
//! - access the filesystem;
//! - initialize an RNG;
//! - maintain mutable global state.
//!
//! All resources belong to the subsystem that owns them.
//!
//! # Determinism
//!
//! The composition root performs no stochastic computation.
//!
//! Determinism requirements belong to the subsystem that introduces the
//! relevant computation.
//!
//! For example:
//!
//! ```text
//! ZQN/randomized simulation
//!     → explicit execution context/seed
//!
//! routing
//!     → deterministic mode when requested
//!
//! optimization
//!     → deterministic pass ordering when requested
//!
//! resilience
//!     → deterministic planning when requested
//! ```
//!
//! The root itself adds no hidden source of nondeterminism.
//!
//! # Serialization
//!
//! This module owns no serialization format.
//!
//! Serialization belongs to the subsystem that owns the semantic object:
//!
//! ```text
//! quantum::ir
//!     → canonical IR serialization
//!
//! quantum::zqn
//!     → ZQN serialization
//!
//! quantum::hardware
//!     → hardware/capability serialization
//!
//! quantum::resilience
//!     → resilience serialization
//! ```
//!
//! The quantum root must not invent a second universal representation.
//!
//! # Versioning
//!
//! This module does not invent a second quantum-wide semantic version.
//!
//! Each child subsystem owns its schema/API versioning.
//!
//! Rust/compiler compatibility is governed by the workspace/package
//! configuration.
//!
//! Target compatibility must be expressed through explicit capability and
//! compatibility contracts.
//!
//! # Safety policy
//!
//! No `unsafe` code belongs in this composition root.
//!
//! More broadly, the quantum architecture is intended to operate without
//! `unsafe` Rust.
//!
//! Backend FFI, if ever required by a concrete provider adapter, must be
//! isolated behind the appropriate hardware boundary and must not leak unsafe
//! implementation details into the canonical quantum namespace.
//!
//! The root itself contains no FFI, pointers, unsafe blocks, or unsafe
//! functions.
//!
//! # Public API policy
//!
//! The preferred public form is namespace-oriented:
//!
//! ```text
//! crate::quantum::algorithms
//! crate::quantum::benchmarking
//! crate::quantum::error_correction
//! crate::quantum::frontend
//! crate::quantum::hardware
//! crate::quantum::ir
//! crate::quantum::memory
//! crate::quantum::optimization
//! crate::quantum::resilience
//! crate::quantum::routing
//! crate::quantum::scheduling
//! crate::quantum::zqn
//! ```
//!
//! This root deliberately avoids:
//!
//! ```text
//! pub use algorithms::*;
//! pub use ir::*;
//! pub use hardware::*;
//! pub use resilience::*;
//! ```
//!
//! Wildcard exports make the API fragile, create accidental symbol collisions,
//! and force this root to change whenever unrelated child modules introduce
//! public names.
//!
//! # Integration contract
//!
//! Adding a new top-level quantum subsystem should normally require:
//!
//! 1. a complete child `mod.rs`;
//! 2. complete child implementation;
//! 3. child tests;
//! 4. child documentation;
//! 5. dependency-direction review;
//! 6. then one `pub mod <name>;` declaration here.
//!
//! The new subsystem should not require changes to unrelated quantum modules.
//!
//! # Module declarations
//!
//! The declarations below are intentionally explicit and stable.
//!
//! A declaration here means the child subsystem is part of the public quantum
//! architecture.
//!
//! Internal child files remain owned by their respective child `mod.rs` files.
//!
//! Backend-independent algorithm construction.
pub mod algorithms;

//! Quantum benchmarking and characterization.
pub mod benchmarking;

//! Quantum error correction and fault-tolerant mechanisms.
pub mod error_correction;

//! Quantum language/external-format frontend and canonical-IR lowering.
pub mod frontend;

//! Provider-neutral quantum hardware abstraction and execution contracts.
pub mod hardware;

//! Canonical, hardware-independent quantum intermediate representation.
pub mod ir;

//! Quantum/hybrid memory and state-resource management.
pub mod memory;

//! Backend-independent quantum optimization.
pub mod optimization;

//! Provider-neutral quantum resilience, adaptation, recovery, mitigation,
//! verification, and observability.
pub mod resilience;

//! Logical-to-physical quantum routing.
pub mod routing;

//! Quantum operation scheduling and timing.
pub mod scheduling;

//! Canonical Zamani Quantum Noise semantics.
pub mod zqn;

// -----------------------------------------------------------------------------
// Stable namespace prelude
// -----------------------------------------------------------------------------
//
// The prelude exports namespaces, not individual symbols.
//
// This preserves the ownership boundaries of every subsystem while providing
// callers with one convenient import surface when they explicitly opt into the
// quantum prelude.
//
// No wildcard child-module exports are used.

/// Stable namespace-only prelude for the quantum subsystem.
///
/// This prelude intentionally exports modules rather than flattening their
/// symbols. It therefore remains resilient to additions inside individual
/// quantum subsystems.
///
/// Example:
///
/// ```rust
/// use crate::quantum::prelude::{ir, resilience};
/// ```
///
/// Canonical qubit identity remains available through:
///
/// ```rust
/// use crate::quantum::ir::qubit::QubitId;
/// ```
pub mod prelude {
    pub use super::algorithms;
    pub use super::benchmarking;
    pub use super::error_correction;
    pub use super::frontend;
    pub use super::hardware;
    pub use super::ir;
    pub use super::memory;
    pub use super::optimization;
    pub use super::resilience;
    pub use super::routing;
    pub use super::scheduling;
    pub use super::zqn;
}

// -----------------------------------------------------------------------------
// Architectural invariants
// -----------------------------------------------------------------------------
//
// These constants identify the namespace only. They are not machine limits,
// resource limits, capacity limits, or semantic version numbers.
//
// Keeping these identifiers compile-time constants makes them usable by
// documentation, diagnostics, and integration tests without introducing
// mutable global state.

/// Stable namespace identifier.
pub const MODULE_ID: &str = "zamani.quantum";

/// Stable composition-boundary identifier.
///
/// This identifies the Rust namespace rather than a hardware target.
pub const COMPOSITION_BOUNDARY_ID: &str = "zamani.quantum.composition";

/// Architectural policy statement for machine-size scalability.
///
/// This is descriptive metadata only. It is deliberately not a numerical
/// limit.
pub const SCALABILITY_POLICY: &str = "resource-bounded-by-available-capabilities";

/// Safety policy for the quantum composition root.
pub const SAFETY_POLICY: &str = "safe-rust-no-unsafe";

/// Canonical qubit identity module path.
pub const CANONICAL_QUBIT_IDENTITY_PATH: &str = "crate::quantum::ir::qubit";

// -----------------------------------------------------------------------------
// Compile-time documentation invariants
// -----------------------------------------------------------------------------
//
// These functions deliberately contain no runtime work. They provide stable
// integration points for tests and documentation without global mutable state.

/// Returns the stable quantum namespace identifier.
#[must_use]
pub const fn module_id() -> &'static str {
    MODULE_ID
}

/// Returns the stable composition-boundary identifier.
#[must_use]
pub const fn composition_boundary_id() -> &'static str {
    COMPOSITION_BOUNDARY_ID
}

/// Returns the canonical qubit identity module path.
#[must_use]
pub const fn canonical_qubit_identity_path() -> &'static str {
    CANONICAL_QUBIT_IDENTITY_PATH
}

/// Returns the architecture's machine-size scalability policy.
///
/// The returned value is descriptive. It does not claim that physical quantum
/// hardware is infinite; it states that this namespace imposes no artificial
/// finite machine-size ceiling.
#[must_use]
pub const fn scalability_policy() -> &'static str {
    SCALABILITY_POLICY
}

/// Returns the quantum composition root's safety policy.
#[must_use]
pub const fn safety_policy() -> &'static str {
    SAFETY_POLICY
}