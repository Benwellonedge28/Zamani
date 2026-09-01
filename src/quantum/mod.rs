//! Zamani Quantum Computing Subsystem
//!
//! Authoritative composition boundary for all quantum-computing functionality
//! in Zamani.
//!
//! # Architectural mission
//!
//! `quantum::mod` is a composition root.
//!
//! It defines:
//!
//! - which quantum subsystems exist;
//! - their public namespace boundaries;
//! - high-level dependency direction;
//! - intentionally retained compatibility paths;
//! - the safety boundary for the quantum namespace.
//!
//! It does NOT define quantum semantics.
//!
//! Semantic ownership belongs to the appropriate child subsystem, especially:
//!
//! ```text
//! quantum::ir
//! ```
//!
//! The canonical quantum IR is the stable semantic boundary between source
//! frontends and downstream quantum compilation/execution systems.
//!
//! # Fundamental architecture
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                    ┌──────────────────┐
//!                    │ quantum::frontend│
//!                    └────────┬─────────┘
//!                             │
//!                             ▼
//!                    ┌──────────────────┐
//!                    │   quantum::ir    │
//!                    │                  │
//!                    │ canonical WHAT   │
//!                    └────────┬─────────┘
//!                             │
//!            ┌────────────────┼─────────────────┐
//!            │                │                 │
//!            ▼                ▼                 ▼
//!       algorithms       optimization        analysis
//!            │                │                 │
//!            │                ▼                 │
//!            │             routing              │
//!            │                │                 │
//!            │                ▼                 │
//!            │            scheduling             │
//!            │                │                 │
//!            └────────────────┼─────────────────┘
//!                             │
//!                             ▼
//!                    error correction
//!                             │
//!                             ▼
//!                       quantum::hardware
//!                             │
//!                    ┌────────┼────────┐
//!                    │        │        │
//!                    ▼        ▼        ▼
//!                 simulator adapters  QPU
//!                    │        │        │
//!                    └────────┼────────┘
//!                             ▼
//!                         execution
//! ```
//!
//! Benchmarking consumes the other subsystems; it is not a dependency of the
//! canonical IR.
//!
//! # Canonical semantic boundary
//!
//! The authoritative semantic representation is:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! In particular, quantum identifiers must use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! No second `QubitId` may be created in this root module.
//!
//! The IR itself explicitly establishes `quantum::ir::qubit` as the canonical
//! qubit identity boundary. All downstream quantum subsystems must consume
//! that type rather than manufacturing competing identities.
//!
//! # Write once, scale everywhere
//!
//! The quantum namespace deliberately contains no architectural maximum for:
//!
//! - logical qubits;
//! - physical qubits;
//! - classical bits;
//! - registers;
//! - operations;
//! - circuit depth;
//! - gate count;
//! - gate arity;
//! - topology;
//! - machine size;
//! - vendor;
//! - quantum technology.
//!
//! A Zamani program may therefore describe any finite computation that can be
//! represented and processed by the available compiler, runtime, memory and
//! target resources.
//!
//! "Infinity" means:
//!
//! > no artificial finite machine-size ceiling is encoded in the quantum
//! > language architecture.
//!
//! It does NOT mean that an individual machine has infinite memory, address
//! space, processing capacity or physical qubits.
//!
//! Concrete resource limits belong to the subsystem that owns them:
//!
//! ```text
//! frontend input limits
//! IR limits
//! optimization limits
//! memory limits
//! hardware capacity
//! execution limits
//! backend limits
//! ```
//!
//! Those limits are policies/capabilities, not semantic limits of Zamani.
//!
//! # Separation of concerns
//!
//! ```text
//! IR          = WHAT the computation means
//! Algorithm   = HOW a logical workload is constructed
//! Optimization= BETTER equivalent representation
//! Routing     = WHERE logical resources are placed
//! Scheduling  = WHEN operations execute
//! Hardware    = WHAT a target can provide
//! Backend     = HOW a target is driven
//! Runtime     = HOW execution is orchestrated
//! Memory      = HOW state/resources are represented and managed
//! QEC         = HOW fault tolerance is represented and implemented
//! Benchmarking= HOW systems are measured
//! ```
//!
//! No child module should move these responsibilities into this composition
//! root.
//!
//! # Dependency direction
//!
//! The preferred dependency direction is:
//!
//! ```text
//! frontend
//!    │
//!    ▼
//! ir
//!    │
//!    ├──────────────► algorithms
//!    │
//!    ├──────────────► optimization
//!    │
//!    ├──────────────► routing
//!    │
//!    ├──────────────► scheduling
//!    │
//!    └──────────────► error_correction
//!                         │
//!                         ▼
//!                     hardware
//!                         │
//!                         ▼
//!                       runtime
//! ```
//!
//! Memory participates as an execution/resource substrate:
//!
//! ```text
//! ir
//!  │
//!  ├──► algorithms / optimization / routing / scheduling / QEC
//!  │
//!  ▼
//! memory + hardware
//!  │
//!  ▼
//! runtime
//! ```
//!
//! Benchmarking consumes execution, hardware, memory, algorithms, QEC and IR
//! information:
//!
//! ```text
//! ir ───────────────┐
//! algorithms ───────┤
//! optimization ─────┤
//! QEC ──────────────┤
//! memory ───────────┤
//! hardware ─────────┤
//!                    ▼
//!              benchmarking
//! ```
//!
//! The reverse dependency is forbidden:
//!
//! ```text
//! ir ─X─► benchmarking
//! hardware ─X─► benchmarking implementation
//! memory ─X─► benchmarking implementation
//! ```
//!
//! # Ownership
//!
//! ## `ir`
//!
//! Owns canonical, hardware-independent quantum semantics.
//!
//! It contains the universal semantic vocabulary required to represent:
//!
//! - gate circuits;
//! - dynamic circuits;
//! - classical control;
//! - measurement;
//! - reset;
//! - initialization;
//! - symbolic parameters;
//! - timing;
//! - pulse semantics;
//! - analog/Hamiltonian computation;
//! - annealing/QUBO models;
//! - logical/fault-tolerant computation;
//! - distributed quantum computation;
//! - extensible future quantum models.
//!
//! The IR must remain independent of all execution backends.
//!
//! ## `frontend`
//!
//! Owns source and external quantum-format parsing and lowering.
//!
//! Frontends lower INTO `quantum::ir`.
//!
//! The IR never depends on frontend ASTs.
//!
//! ## `algorithms`
//!
//! Owns backend-independent quantum algorithm construction.
//!
//! It consumes canonical IR concepts but does not redefine them.
//!
//! ## `optimization`
//!
//! Owns logical optimization and transformation passes.
//!
//! It consumes canonical IR.
//!
//! It does not own physical topology, execution or benchmarking.
//!
//! ## `routing`
//!
//! Owns logical-to-physical placement and connectivity-aware routing.
//!
//! It consumes canonical IR and hardware/resource descriptions.
//!
//! It does not redefine `QubitId`.
//!
//! ## `scheduling`
//!
//! Owns operation ordering and scheduling policy.
//!
//! Hardware timing constraints are supplied by the hardware subsystem.
//!
//! ## `error_correction`
//!
//! Owns quantum error-correction algorithms, encodings, decoding and
//! fault-tolerant mechanisms.
//!
//! Logical/physical identity remains interoperable with the canonical IR.
//!
//! ## `hardware`
//!
//! Owns the provider-neutral Hardware Abstraction Layer and provider/device
//! integration.
//!
//! It contains the physical concepts deliberately excluded from canonical IR:
//!
//! - capabilities;
//! - topology;
//! - calibration;
//! - instruction sets;
//! - execution;
//! - providers;
//! - devices;
//! - backend lifecycle;
//! - adapters.
//!
//! Provider-specific details remain inside hardware adapters.
//!
//! ## `memory`
//!
//! Owns quantum/hybrid memory and state-resource management.
//!
//! It must remain representation-neutral and must not redefine the canonical
//! quantum IR.
//!
//! ## `benchmarking`
//!
//! Owns benchmark orchestration, workload generation, execution contracts,
//! statistics, metrics, Quantum Volume, randomized benchmarking, XEB,
//! volumetric benchmarking, application benchmarking, QEC benchmarking,
//! reporting and regression analysis.
//!
//! Benchmarking is a consumer of quantum subsystems, never their semantic
//! foundation.
//!
//! ## `self_healing`
//!
//! This namespace is intentionally NOT declared here until its module boundary
//! is valid Rust and has a production contract.
//!
//! The current repository contains a placeholder `self_healing/mod.rs` rather
//! than a completed Rust module. A composition root must never expose an
//! uncompilable placeholder simply because a directory exists.
//!
//! ## `zqml`
//!
//! This namespace is intentionally not declared until its current module
//! contents are a valid production Rust API.
//!
//! The quantum root must never expose incomplete source merely because the
//! directory exists.
//!
//! ## `zqn`
//!
//! The current repository contains a malformed filename with a trailing space
//! (`mod.rs `). It is intentionally not declared here.
//!
//! The filename must first be corrected and the module given a completed API
//! contract before it becomes part of the stable quantum namespace.
//!
//! # Module declaration policy
//!
//! Every directory declared below MUST contain its own authoritative `mod.rs`.
//!
//! This file must NOT recreate those module trees inline.
//!
//! Correct:
//!
//! ```text
//! pub mod hardware;
//! ```
//!
//! Incorrect:
//!
//! ```text
//! pub mod hardware {
//!     #[path = "..."]
//!     pub mod backend;
//! }
//! ```
//!
//! The latter duplicates the responsibility of `hardware/mod.rs` and couples
//! this composition root to internal file layout.
//!
//! # Safety
//!
//! The entire quantum namespace is safe Rust.
//!
//! This composition boundary forbids unsafe Rust explicitly.
//!
//! Child modules are expected to enforce the same rule locally.
//!
//! No unsafe execution primitive, raw pointer, FFI handle or backend memory
//! access belongs in this file.
//!
//! # Global state
//!
//! This module owns no global mutable quantum state.
//!
//! It performs no:
//!
//! - device initialization;
//! - network I/O;
//! - backend connection;
//! - simulator initialization;
//! - random generation;
//! - memory allocation;
//! - benchmark execution;
//! - source parsing;
//! - hardware discovery.
//!
//! Concrete objects must be created and owned by their respective subsystems.
//!
//! # Compatibility
//!
//! Existing public flat paths are retained only where they already form part of
//! the repository API and where the authoritative implementation is available
//! through a child module.
//!
//! New code should prefer the explicit subsystem paths:
//!
//! ```text
//! quantum::algorithms
//! quantum::benchmarking
//! quantum::error_correction
//! quantum::frontend
//! quantum::hardware
//! quantum::ir
//! quantum::memory
//! quantum::optimization
//! quantum::routing
//! quantum::scheduling
//! ```
//!
//! Compatibility re-exports never create duplicate implementations.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features.
//!
//! No language feature newer than the declared MSRV is required by this
//! composition boundary.
//!
//! # API stability
//!
//! This file intentionally contains very little executable behavior.
//!
//! Adding a new quantum subsystem should normally require only:
//!
//! 1. a completed child module;
//! 2. a public `mod.rs` contract for that child;
//! 3. one `pub mod` declaration here;
//! 4. integration tests in the child subsystem.
//!
//! Existing unrelated quantum files must not need modification merely because
//! a new independent subsystem was introduced.
//!
//! # File-completion invariant
//!
//! This file is complete when:
//!
//! 1. every declared module physically exists;
//! 2. every declared module owns its own implementation boundary;
//! 3. no child module is duplicated inline;
//! 4. no vendor SDK is imported;
//! 5. no backend is instantiated;
//! 6. no global mutable state exists;
//! 7. no unsafe Rust exists;
//! 8. no quantum-size limit exists;
//! 9. canonical `quantum::ir::qubit` identity remains authoritative;
//! 10. compatibility exports do not create duplicate implementations;
//! 11. incomplete placeholder directories are not exposed as production APIs;
//! 12. the module remains valid on Rust 1.97/1.97.1;
//! 13. adding an independent quantum subsystem does not require reworking
//!     unrelated subsystem implementations.
//!
//! # Integration contract
//!
//! Downstream code should depend on the narrowest appropriate namespace.
//!
//! Examples:
//!
//! ```text
//! use crate::quantum::ir::qubit::QubitId;
//! use crate::quantum::ir::program;
//! use crate::quantum::hardware;
//! use crate::quantum::optimization;
//! ```
//!
//! Do not import a qubit identity from a compatibility path if the canonical
//! `quantum::ir::qubit::QubitId` path is available.
//!
//! The canonical data flow is:
//!
//! ```text
//! source
//!   │
//!   ▼
//! frontend
//!   │
//!   ▼
//! quantum::ir
//!   │
//!   ├── validate
//!   ├── analyze
//!   ├── optimize
//!   ├── map
//!   ├── route
//!   ├── schedule
//!   ├── lower
//!   │
//!   ▼
//! hardware target
//!   │
//!   ▼
//! backend/runtime
//! ```
//!
//! The same canonical semantic program may therefore be lowered to different
//! compatible targets without changing the source-level program merely because
//! the target has a different number of qubits, topology, technology or native
//! instruction set.
//!
//! # No hard-coded hardware assumptions
//!
//! This composition root deliberately contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_CLASSICAL_BITS
//! MAX_OPERATIONS
//! MAX_DEPTH
//! IBM_QUBITS
//! IONQ_QUBITS
//! GPU_COUNT
//! CPU_COUNT
//! DEFAULT_TOPOLOGY
//! ```
//!
//! Such information belongs to target/resource descriptions.
//!
//! # Testing
//!
//! Composition-root testing should remain lightweight.
//!
//! Domain behavior belongs to child modules and their dedicated tests.
//!
//! The quantum root should primarily be verified by the normal Rust module
//! compilation process and workspace integration tests.
//!
//! In particular, this file deliberately avoids tests that name speculative
//! types from unrelated subsystems. Such tests make this composition root
//! unnecessarily fragile and violate the independence requirement.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]

// =============================================================================
// Canonical quantum subsystems
// =============================================================================

/// Backend-independent quantum algorithm construction.
pub mod algorithms;

/// Production quantum benchmarking framework.
///
/// Benchmarking is a consumer/orchestration subsystem and must not become a
/// dependency of the canonical Quantum IR.
pub mod benchmarking;

/// Quantum error correction and fault-tolerant computation.
pub mod error_correction;

/// Quantum source-language and external-format frontends.
///
/// Frontends lower into the canonical `quantum::ir` representation.
pub mod frontend;

/// Provider-neutral quantum hardware abstraction layer.
pub mod hardware;

/// Canonical, hardware-independent Zamani Quantum IR.
pub mod ir;

/// Quantum and hybrid memory/resource substrate.
pub mod memory;

/// Backend-independent logical quantum optimization.
pub mod optimization;

/// Hardware-connectivity-aware routing and transpilation.
pub mod routing;

/// Quantum scheduling subsystem.
pub mod scheduling;

// =============================================================================
// Compatibility exports
// =============================================================================
//
// These exports preserve established flat public paths without defining a
// second implementation.
//
// New code should prefer the explicit subsystem paths documented above.
// =============================================================================

/// Historical compatibility path for the Quantum Volume estimator.
///
/// Preferred path:
///
/// `quantum::benchmarking::volume_estimator`
pub use benchmarking::volume_estimator;

/// Historical compatibility path for T-gate reduction.
///
/// Preferred path:
///
/// `quantum::optimization::t_gate_reduction`
pub use optimization::t_gate_reduction;

/// Historical compatibility path for quantum transpilation.
///
/// Preferred path:
///
/// `quantum::routing::transpiler`
pub use routing::transpiler;

/// Historical compatibility path for stabilizer scheduling.
///
/// Preferred path:
///
/// `quantum::scheduling::stabilizer_scheduler`
pub use scheduling::stabilizer_scheduler;

/// Historical compatibility path for variational algorithms.
///
/// Preferred path:
///
/// `quantum::algorithms::variational`
pub use algorithms::variational;

// =============================================================================
// Stable subsystem prelude
// =============================================================================

/// Small, stable import surface for applications that need the quantum
/// subsystem boundaries rather than individual implementation types.
///
/// Specialized APIs must continue to be imported from their owning modules.
///
/// This prelude intentionally does not flatten the Quantum IR and therefore
/// does not expose duplicate or ambiguous semantic types.
pub mod prelude {
    pub use super::algorithms;
    pub use super::benchmarking;
    pub use super::error_correction;
    pub use super::frontend;
    pub use super::hardware;
    pub use super::ir;
    pub use super::memory;
    pub use super::optimization;
    pub use super::routing;
    pub use super::scheduling;
}

// =============================================================================
// Lifecycle compatibility hooks
// =============================================================================
//
// The quantum root owns no global runtime state. These functions therefore
// remain intentionally empty compatibility boundaries.
//
// Concrete initialization belongs to concrete subsystem constructors and
// runtime ownership contexts.
// =============================================================================

/// Initializes the quantum subsystem namespace.
///
/// This is intentionally a side-effect-free compatibility hook. It does not
/// initialize a backend, allocate memory, contact a QPU, start a simulator or
/// mutate global state.
#[inline]
pub fn init_quantum() {}

/// Shuts down the quantum subsystem namespace.
///
/// This is intentionally a side-effect-free compatibility hook. Concrete
/// resources are released by their owners.
#[inline]
pub fn shutdown_quantum() {}