//! Zamani Quantum Computing Subsystem.
//!
//! This module is the authoritative composition boundary for quantum computing
//! functionality in Zamani.
//!
//! # Purpose
//!
//! `crate::quantum` defines the public namespace and dependency boundaries for
//! the quantum subsystem. It does not implement quantum semantics, algorithms,
//! hardware execution, simulation, routing, scheduling, error correction,
//! benchmarking, or noise modelling.
//!
//! Domain semantics belong to the appropriate child subsystem.
//!
//! The most important semantic boundary is:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! The canonical quantum IR describes what a quantum computation means.
//! Downstream subsystems determine how that computation is transformed,
//! mapped, scheduled, executed, characterized, and measured.
//!
//! # Architectural principle
//!
//! Zamani follows:
//!
//! ```text
//!                       Zamani source
//!                            │
//!                            ▼
//!                    quantum::frontend
//!                            │
//!                            ▼
//!                     quantum::ir
//!                            │
//!          ┌─────────────────┼──────────────────┐
//!          │                 │                  │
//!          ▼                 ▼                  ▼
//!      algorithms       optimization        analysis
//!          │                 │                  │
//!          └─────────────────┼──────────────────┘
//!                            │
//!                ┌───────────┼───────────┐
//!                │           │           │
//!                ▼           ▼           ▼
//!             routing    scheduling      ZQN
//!                │           │           │
//!                └───────────┼───────────┘
//!                            ▼
//!                  error-correction layer
//!                            │
//!                            ▼
//!                    quantum::hardware
//!                            │
//!                            ▼
//!                         runtime
//!                            │
//!             ┌──────────────┼──────────────┐
//!             ▼              ▼              ▼
//!          simulator        QPU          emulator
//!             │              │              │
//!             └──────────────┼──────────────┘
//!                            ▼
//!                       observations
//!                            │
//!                            ▼
//!                    quantum::benchmarking
//! ```
//!
//! The exact implementation dependency graph is owned by the individual
//! subsystem contracts. This root only establishes the namespace boundary.
//!
//! # Write once, scale everywhere
//!
//! A Zamani quantum program must not encode the physical size of the machine
//! on which it will execute.
//!
//! This module therefore imposes no architectural maximum on:
//!
//! - logical qubits;
//! - physical qubits;
//! - classical bits;
//! - registers;
//! - operations;
//! - circuit depth;
//! - gate count;
//! - gate arity;
//! - topology size;
//! - number of devices;
//! - number of execution targets;
//! - quantum technology;
//! - vendor;
//! - execution architecture.
//!
//! The word "infinity" means:
//!
//! > the language and quantum namespace do not impose an artificial finite
//! > machine-size ceiling.
//!
//! Every concrete compilation or execution remains bounded by the resources
//! actually available to that invocation.
//!
//! Resource constraints therefore belong to explicit policies and capabilities,
//! not to this namespace.
//!
//! Examples include:
//!
//! ```text
//! compiler resource policy
//! runtime resource policy
//! memory budget
//! execution budget
//! target capabilities
//! device capacity
//! backend constraints
//! security limits
//! ```
//!
//! Such limits must never become semantic constants in this composition root.
//!
//! # Canonical qubit identity
//!
//! The sole authoritative qubit identity implementation is:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! New quantum code must use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This root deliberately defines neither type.
//!
//! No quantum child subsystem should introduce a competing `QubitId` merely
//! because it needs an identifier.
//!
//! If a subsystem needs a different identity semantic, it must use a distinct,
//! explicitly named identity owned by that subsystem rather than pretending to
//! be the canonical qubit identity.
//!
//! # Ownership
//!
//! ## `algorithms`
//!
//! Owns backend-independent quantum algorithm construction and orchestration.
//!
//! It does not own:
//!
//! - canonical IR semantics;
//! - hardware;
//! - routing;
//! - scheduling;
//! - vendor APIs;
//! - execution;
//! - QPU credentials.
//!
//! ## `benchmarking`
//!
//! Owns benchmark protocols, workload generation, execution contracts,
//! statistical analysis, metrics, Quantum Volume, randomized benchmarking,
//! cross-entropy benchmarking, volumetric/application/QEC benchmarking,
//! reporting, and regression analysis.
//!
//! Benchmarking consumes other quantum subsystems. It is not the semantic
//! foundation of the quantum namespace.
//!
//! ## `error_correction`
//!
//! Owns quantum error-correction algorithms, codes, syndrome processing,
//! decoding, logical fault tolerance, and related fault-tolerant mechanisms.
//!
//! Universal noise semantics should ultimately be supplied by `zqn` rather
//! than duplicated inside QEC.
//!
//! ## `frontend`
//!
//! Owns parsing and lowering from Zamani quantum syntax and supported external
//! quantum formats into the canonical `quantum::ir` representation.
//!
//! The IR must never depend on frontend ASTs.
//!
//! ## `hardware`
//!
//! Owns provider-neutral hardware abstraction, target capabilities,
//! instruction capabilities, topology, calibration, device lifecycle,
//! execution adapters, and provider-specific integration.
//!
//! Vendor-specific implementation belongs here, not in the canonical IR or
//! ZQN namespace.
//!
//! ## `ir`
//!
//! Owns canonical, hardware-independent quantum semantics.
//!
//! It is the stable semantic boundary between frontend and downstream quantum
//! compilation/execution systems.
//!
//! The IR must remain independent of:
//!
//! - vendor SDKs;
//! - backend credentials;
//! - hardware connections;
//! - network clients;
//! - execution engines;
//! - routing implementations;
//! - scheduler implementations;
//! - QEC implementations;
//! - benchmark implementations.
//!
//! ## `memory`
//!
//! Owns quantum/hybrid memory and state-resource management.
//!
//! It must remain representation-aware but execution-backend independent and
//! must not redefine canonical IR identities.
//!
//! ## `optimization`
//!
//! Owns logical and representation-preserving quantum transformations.
//!
//! It must not become the owner of physical hardware topology or execution.
//!
//! ## `routing`
//!
//! Owns logical-to-physical placement and connectivity-aware transformation.
//!
//! It consumes canonical IR and target/resource information.
//!
//! It must use the canonical qubit identities from `quantum::ir::qubit`.
//!
//! ## `scheduling`
//!
//! Owns ordering and timing of executable operations.
//!
//! Hardware timing capabilities are supplied through the hardware boundary.
//!
//! ## `zqn`
//!
//! When exposed, owns the canonical Zamani Quantum Noise semantics:
//!
//! - channels;
//! - faults;
//! - stochastic distributions;
//! - correlated noise;
//! - temporal noise;
//! - spatial noise;
//! - crosstalk;
//! - leakage;
//! - loss;
//! - erasure;
//! - calibration uncertainty;
//! - drift;
//! - characterization;
//! - uncertainty;
//! - reproducible stochastic execution.
//!
//! ZQN is not a competing IR.
//!
//! Its relationship to the canonical IR is:
//!
//! ```text
//! quantum::ir
//!     │
//!     │ computation semantics
//!     ▼
//!     ZQN
//!     │
//!     │ physical uncertainty / noise semantics
//!     ▼
//! routing / scheduling / QEC / simulation / hardware
//! ```
//!
//! # Why unfinished modules are not declared
//!
//! A directory existing in the source tree is not sufficient reason for this
//! root to expose it as a Rust module.
//!
//! A public declaration such as:
//!
//! ```text
//! pub mod zqml;
//! ```
//!
//! requires a valid child module boundary and compilable implementation.
//!
//! The same rule applies to:
//!
//! ```text
//! self_healing
//! zqml
//! zqn
//! ```
//!
//! In particular, the current repository has historically contained an
//! incorrectly named ZQN module file with a trailing space in the filename.
//! That filesystem issue must be corrected before `zqn` can be declared here.
//!
//! This root therefore refuses to make an incomplete subsystem a dependency of
//! the complete quantum namespace.
//!
//! # Dependency direction
//!
//! The composition root does not enforce every internal dependency through
//! Rust's type system, but the architectural direction is:
//!
//! ```text
//! frontend
//!     │
//!     ▼
//!    IR
//!     │
//!     ├──────────────► algorithms
//!     │
//!     ├──────────────► optimization
//!     │
//!     ├──────────────► routing
//!     │
//!     ├──────────────► scheduling
//!     │
//!     └──────────────► analysis
//!
//! IR + target information
//!     │
//!     ├──────────────► routing
//!     ├──────────────► scheduling
//!     └──────────────► lowering
//!
//! IR + ZQN
//!     │
//!     ├──────────────► simulation
//!     ├──────────────► routing
//!     ├──────────────► scheduling
//!     └──────────────► QEC
//!
//! compiled representation
//!     │
//!     ▼
//! hardware
//!     │
//!     ▼
//! runtime / backend
//!     │
//!     ▼
//! observations
//!     │
//!     ▼
//! benchmarking / characterization
//! ```
//!
//! The following reverse dependencies are prohibited architecturally:
//!
//! ```text
//! IR ─X─► frontend
//! IR ─X─► hardware
//! IR ─X─► routing implementation
//! IR ─X─► scheduling implementation
//! IR ─X─► benchmarking implementation
//! IR ─X─► vendor SDK
//!
//! ZQN ─X─► vendor SDK
//! ZQN ─X─► credentials
//! ZQN ─X─► UI
//! ZQN ─X─► CLI
//!
//! frontend ─X─► hardware execution
//! algorithms ─X─► vendor API
//! ```
//!
//! # Composition-root rule
//!
//! This file is intentionally a composition root rather than a second domain
//! implementation.
//!
//! It should contain:
//!
//! - module declarations;
//! - module-level documentation;
//! - narrowly justified compatibility aliases;
//! - no quantum algorithms;
//! - no state representation;
//! - no hardware discovery;
//! - no device initialization;
//! - no backend connection;
//! - no random number generation;
//! - no global mutable state;
//! - no serialization implementation;
//! - no numerical algorithms;
//! - no vendor logic.
//!
//! A new independent quantum subsystem should normally require only a new
//! completed child module and one declaration here.
//!
//! It should not require unrelated existing quantum modules to be edited.
//!
//! # Public API policy
//!
//! The preferred API is namespace-oriented:
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
//! crate::quantum::routing
//! crate::quantum::scheduling
//! crate::quantum::zqn
//! ```
//!
//! The root deliberately avoids wildcard re-exports.
//!
//! In particular, this root must not do:
//!
//! ```text
//! pub use algorithms::*;
//! pub use ir::*;
//! pub use hardware::*;
//! ```
//!
//! Such exports make ownership ambiguous, cause accidental API collisions,
//! increase coupling, and force this file to change whenever an unrelated
//! child module adds a public symbol.
//!
//! The canonical IR itself already owns its own carefully selected
//! compatibility exports and qubit identity boundary.
//!
//! # Compatibility policy
//!
//! Historical flat APIs should be preserved by the module that owns the
//! underlying implementation whenever possible.
//!
//! This composition root should not become a compatibility dumping ground.
//!
//! If an old API must be preserved at this level, it must be:
//!
//! 1. an explicit alias or re-export;
//! 2. backed by exactly one canonical implementation;
//! 3. documented as compatibility API;
//! 4. free of duplicate types;
//! 5. independent of implementation details;
//! 6. removable under an explicitly documented compatibility policy.
//!
//! # No hard-coded machine assumptions
//!
//! This module intentionally contains none of the following:
//!
//! ```text
//! MAX_QUBITS
//! MAX_PHYSICAL_QUBITS
//! MAX_CLASSICAL_BITS
//! MAX_OPERATIONS
//! MAX_DEPTH
//! MAX_GATE_ARITY
//! DEFAULT_QUBIT_COUNT
//! DEFAULT_TOPOLOGY
//! IBM_QUBIT_COUNT
//! IONQ_QUBIT_COUNT
//! GPU_COUNT
//! CPU_COUNT
//! ```
//!
//! These values, where they are meaningful, belong to explicit resource or
//! target policies.
//!
//! # Resource and security boundaries
//!
//! "No hard-coded size limit" does not mean "unbounded allocation is safe."
//!
//! The quantum architecture must distinguish:
//!
//! ```text
//! semantic capacity
//!       ≠
//! resource policy
//!       ≠
//! physical target capacity
//! ```
//!
//! A compiler invocation may impose a memory budget.
//! A simulator may impose a state-representation budget.
//! A QPU may expose a finite number of physical resources.
//! A scheduler may impose an execution deadline.
//!
//! None of those limits belongs in this composition root.
//!
//! # Safety
//!
//! The entire quantum namespace is intended to be safe Rust.
//!
//! This composition boundary explicitly forbids unsafe Rust.
//!
//! Child modules should enforce the same invariant independently.
//!
//! No raw pointers, unsafe blocks, unsafe functions, backend FFI, mutable
//! globals, or unsafe execution primitives belong here.
//!
//! # Global state
//!
//! This module owns no global mutable state.
//!
//! It performs no:
//!
//! - initialization of hardware;
//! - initialization of simulators;
//! - network access;
//! - filesystem access;
//! - random sampling;
//! - thread creation;
//! - memory-pool initialization;
//! - backend discovery;
//! - credential loading.
//!
//! Resource ownership belongs to the subsystem that created the resource.
//!
//! # Determinism
//!
//! The composition root itself performs no stochastic work.
//!
//! Determinism contracts belong to the subsystems that require them.
//!
//! In particular, ZQN and stochastic simulation must use explicit execution
//! contexts and caller-controlled reproducibility information rather than
//! hidden global RNG state.
//!
//! # Thread safety
//!
//! This module owns no runtime state and therefore introduces no global
//! thread-safety requirement.
//!
//! Child APIs should document whether their objects are:
//!
//! - `Send`;
//! - `Sync`;
//! - immutable/shareable;
//! - internally synchronized;
//! - intentionally single-threaded.
//!
//! The composition root must not impose unnecessary synchronization on those
//! implementations.
//!
//! # Serialization
//!
//! This module owns no serialized representation.
//!
//! Serialization contracts belong to the subsystem that owns the serialized
//! semantic object.
//!
//! For example:
//!
//! ```text
//! quantum::ir      → canonical IR serialization
//! quantum::zqn     → ZQN serialization
//! quantum::hardware→ target/device serialization
//! ```
//!
//! There must not be a second quantum-wide serialization format invented here.
//!
//! # Versioning
//!
//! This module does not invent a separate quantum version number.
//!
//! Individual subsystem schemas and APIs own their versioning.
//!
//! Workspace/compiler compatibility is governed by the repository's Rust and
//! package configuration.
//!
//! Target language/runtime compatibility for this file:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Integration with `quantum::ir`
//!
//! The canonical IR is intentionally declared independently of all downstream
//! quantum subsystems.
//!
//! This means a future subsystem can consume:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! without changing the IR merely because the subsystem exists.
//!
//! The canonical qubit identity remains:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! # Integration with ZQN
//!
//! Once ZQN is complete, the only composition-root change required should be:
//!
//! ```rust
//! /// Zamani Quantum Noise: backend-independent noise,
//! /// uncertainty, fault, channel and characterization semantics.
//! pub mod zqn;
//! ```
//!
//! The root must not import individual ZQN implementation types merely to make
//! them globally visible.
//!
//! ZQN consumers should import the specific APIs from:
//!
//! ```text
//! crate::quantum::zqn
//! ```
//!
//! or its narrower child namespaces.
//!
//! # Integration with routing
//!
//! Routing should consume canonical IR identities and target information.
//!
//! If routing needs noise-aware costs, it should consume an explicit ZQN
//! contract rather than defining another independent noise model.
//!
//! The composition root itself does not implement that adapter.
//!
//! # Integration with scheduling
//!
//! Scheduling consumes canonical semantic operations and target timing/resource
//! information.
//!
//! Noise-dependent scheduling may consume ZQN through an explicit integration
//! contract.
//!
//! The root itself remains unaware of scheduling policy.
//!
//! # Integration with QEC
//!
//! QEC remains the owner of fault-tolerant algorithms.
//!
//! Universal physical noise semantics should be provided by ZQN.
//!
//! The eventual relationship is:
//!
//! ```text
//! ZQN noise/channel/fault model
//!              │
//!              ▼
//!       QEC integration layer
//!              │
//!       ┌──────┴──────┐
//!       ▼             ▼
//!   syndrome       decoding
//!       │             │
//!       └──────┬──────┘
//!              ▼
//!       logical analysis
//! ```
//!
//! This avoids duplicate physical-noise definitions.
//!
//! # Integration with hardware
//!
//! Hardware owns provider-specific behavior.
//!
//! It should expose provider-neutral capabilities and execution contracts to
//! the rest of the quantum system.
//!
//! The root never selects a vendor or device.
//!
//! # Integration with benchmarking
//!
//! Benchmarking consumes canonical IR, algorithms, execution results,
//! hardware information, QEC information, memory information, calibration,
//! characterization, and ZQN information as appropriate.
//!
//! Benchmarking must not become a dependency of the canonical IR merely because
//! this root exposes the benchmarking namespace.
//!
//! # Integration with memory
//!
//! Memory remains a state/resource substrate.
//!
//! ZQN channels may eventually be applied through an explicit memory/channel
//! integration boundary, but neither subsystem should absorb the other's
//! ownership.
//!
//! # Integration with frontend
//!
//! Frontends produce canonical IR.
//!
//! The frontend must not cause this composition root to depend on a frontend
//! AST or parser implementation.
//!
//! # Testing contract
//!
//! This file intentionally has no domain-level tests.
//!
//! The Rust compiler verifies that every declared module exists and can be
//! compiled. Each child subsystem owns its own unit, property, integration,
//! determinism, scaling, and compatibility tests.
//!
//! Composition-level integration tests should live under the quantum test
//! infrastructure rather than turning this namespace file into a test registry.
//!
//! The important root invariants are:
//!
//! 1. every declared module has a valid child module;
//! 2. this file contains no unsafe code;
//! 3. this file contains no global mutable state;
//! 4. this file contains no machine-size constants;
//! 5. this file does not define a competing `QubitId`;
//! 6. the canonical IR remains independent;
//! 7. unfinished modules are not exposed;
//! 8. wildcard re-exports are avoided;
//! 9. vendor APIs are not imported;
//! 10. adding an independent child subsystem does not require unrelated
//!     subsystem modifications.
//!
//! # Completion criterion
//!
//! This file is complete when the quantum namespace can be compiled while
//! preserving all of the following:
//!
//! ```text
//! safe Rust
//!       +
//! canonical IR ownership
//!       +
//! canonical qubit identity
//!       +
//! explicit subsystem boundaries
//!       +
//! no artificial machine-size limit
//!       +
//! no vendor coupling
//!       +
//! no global state
//!       +
//! no speculative modules
//!       +
//! stable public namespaces
//! ```
//!
//! =============================================================================
//! Compiler-enforced safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Production quantum subsystem boundaries
// =============================================================================
//
// IMPORTANT:
//
// Each declaration below points to the authoritative `mod.rs` owned by that
// subsystem. This root does not reproduce child module trees.
//
// If a subsystem is incomplete, it must not be declared until its own module
// boundary is production-ready.
//

/// Backend-independent quantum algorithm construction.
///
/// Owns logical algorithm generation and orchestration.
pub mod algorithms;

/// Quantum benchmarking, measurement, statistics and benchmark orchestration.
///
/// Benchmarking is a consumer of the quantum stack and not the canonical
/// semantic foundation.
pub mod benchmarking;

/// Quantum error correction and fault-tolerant computation.
///
/// Physical noise semantics should be consumed from ZQN once the ZQN boundary
/// is complete.
pub mod error_correction;

/// Quantum source-language and external-format frontends.
///
/// Frontends lower into `quantum::ir`.
pub mod frontend;

/// Provider-neutral quantum hardware abstraction and execution boundary.
///
/// Vendor-specific implementations remain inside the hardware subsystem.
pub mod hardware;

/// Canonical, hardware-independent Zamani Quantum IR.
///
/// This is the semantic `WHAT` boundary for quantum computation.
pub mod ir;

/// Quantum and hybrid memory/state-resource subsystem.
pub mod memory;

/// Backend-independent logical quantum optimization.
pub mod optimization;

/// Logical-to-physical routing and connectivity-aware transformation.
pub mod routing;

/// Quantum operation scheduling and timing representation.
pub mod scheduling;

// =============================================================================
// Intentionally deferred subsystem boundaries
// =============================================================================
//
// These directories may exist in the repository before their APIs are ready.
// A directory alone is not a valid reason to expose a public Rust module.
//
// `zqn` is especially important: the ZQN subsystem is being constructed as an
// independent production subsystem and must be exposed only after its child
// module has a valid `mod.rs` and its historical malformed filename has been
// corrected.
//
// DO NOT uncomment any of these merely to make the namespace look complete.
//
// pub mod self_healing;
// pub mod zqml;
// pub mod zqn;

// =============================================================================
// Stable subsystem prelude
// =============================================================================
//
// The prelude exposes subsystem boundaries, not every type contained within
// them.
//
// This deliberately avoids wildcard exports.
//
// Users requiring a concrete API should import it from the owning subsystem:
//
//     crate::quantum::ir::qubit::QubitId
//     crate::quantum::hardware::...
//     crate::quantum::zqn::...
//
// once the corresponding subsystem is available.

/// Stable namespace-oriented quantum prelude.
///
/// This prelude is deliberately composed of modules rather than a collection
/// of domain types. That keeps ownership explicit and prevents accidental API
/// collisions when independent quantum subsystems evolve.
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
// Lifecycle compatibility boundary
// =============================================================================
//
// The quantum composition root does not own resources.
//
// These functions therefore provide no initialization semantics. They exist
// only if older callers require a namespace-level lifecycle hook.
//
// New code should construct and own resources through the subsystem that
// actually owns them.
//
// They deliberately perform no:
//
// - allocation;
// - I/O;
// - device discovery;
// - backend connection;
// - simulator initialization;
// - thread creation;
// - global-state mutation;
// - random generation.
//

/// Compatibility lifecycle hook for callers that historically initialized the
/// quantum namespace.
///
/// This function is intentionally side-effect free. Concrete initialization
/// belongs to the owning subsystem.
#[inline]
pub const fn init_quantum() {}

/// Compatibility lifecycle hook for callers that historically shut down the
/// quantum namespace.
///
/// This function is intentionally side-effect free. Concrete resource cleanup
/// belongs to the owning subsystem.
#[inline]
pub const fn shutdown_quantum() {}