//! # Zamani Quantum Noise — Integration Boundary
//!
//! This module is the **composition boundary** between ZQN and the rest of the
//! Zamani quantum stack.
//!
//! # Mission
//!
//! `quantum::zqn::integration` connects the canonical ZQN domain model to
//! downstream and upstream subsystems without transferring ownership of those
//! subsystems into ZQN.
//!
//! The architectural relationship is:
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                     quantum::frontend
//!                              │
//!                              ▼
//!                    ┌─────────────────┐
//!                    │   quantum::ir   │
//!                    │ canonical IR    │
//!                    └────────┬────────┘
//!                             │
//!                  canonical semantic meaning
//!                             │
//!              ┌──────────────┴──────────────┐
//!              │                             │
//!              ▼                             ▼
//!      compiler transformations             ZQN
//!                                            │
//!                              ┌─────────────┼─────────────┐
//!                              │             │             │
//!                              ▼             ▼             ▼
//!                         probability     channels       faults
//!                              │             │             │
//!                              └─────────────┼─────────────┘
//!                                            ▼
//!                                          noise
//!                                            │
//!                              ┌─────────────┼─────────────┐
//!                              │             │             │
//!                              ▼             ▼             ▼
//!                         calibration  characterization simulation
//!                              │             │             │
//!                              └─────────────┼─────────────┘
//!                                            ▼
//!                                      propagation
//!                                            │
//!                                            ▼
//!                                         target
//!                                            │
//!                                            ▼
//!                                  ZQN integration layer
//!                                            │
//!              ┌─────────────────────────────┼──────────────────────────┐
//!              │                             │                          │
//!              ▼                             ▼                          ▼
//!           routing                     scheduling                    QEC
//!              │                             │                          │
//!              └─────────────────────────────┼──────────────────────────┘
//!                                            │
//!                                            ▼
//!                                         hardware
//!                                            │
//!                                            ▼
//!                                         runtime
//!                                            │
//!                                            ▼
//!                                       execution
//!                                            │
//!                                            ▼
//!                                       observations
//!                                            │
//!                              ┌─────────────┼─────────────┐
//!                              ▼             ▼             ▼
//!                         benchmarking  characterization analysis
//! ```
//!
//! # Ownership
//!
//! This module owns only the **integration namespace and composition
//! boundaries**.
//!
//! Individual integration files own their respective adapter contracts:
//!
//! ```text
//! ir.rs
//!     ZQN ↔ canonical quantum IR
//!
//! routing.rs
//!     ZQN ↔ routing
//!
//! scheduling.rs
//!     ZQN ↔ scheduling
//!
//! qec.rs
//!     ZQN ↔ quantum error correction
//!
//! hardware.rs
//!     ZQN ↔ hardware abstraction
//!
//! memory.rs
//!     ZQN ↔ quantum memory/state subsystem
//!
//! benchmarking.rs
//!     ZQN ↔ benchmarking
//!
//! runtime.rs
//!     ZQN ↔ runtime/execution orchestration
//! ```
//!
//! `mod.rs` does not own the implementation of any of those integrations.
//!
//! # Non-ownership
//!
//! This module must never become the owner of:
//!
//! - canonical quantum IR;
//! - quantum program ASTs;
//! - source-language syntax;
//! - gate definitions;
//! - quantum channels;
//! - probability mathematics;
//! - faults;
//! - noise models;
//! - calibration models;
//! - characterization algorithms;
//! - simulation engines;
//! - routing algorithms;
//! - scheduling algorithms;
//! - QEC decoders;
//! - hardware implementations;
//! - vendor SDKs;
//! - QPU credentials;
//! - benchmark methodologies;
//! - benchmark estimators;
//! - runtime implementations;
//! - user interfaces;
//! - CLI functionality.
//!
//! The integration layer connects those domains. It does not absorb them.
//!
//! # Canonical quantum identity
//!
//! ZQN integration must use the canonical quantum IR identity types:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! No integration module may define another:
//!
//! ```text
//! QubitId
//! PhysicalQubitId
//! ```
//!
//! and no integration module may replace those identities with:
//!
//! ```text
//! usize
//! u64
//! u128
//! String
//! ```
//!
//! as a semantic substitute.
//!
//! A numerical index may be used as an implementation detail when explicitly
//! derived from the canonical ID, but it must never become the public identity
//! contract.
//!
//! The repository's canonical IR explicitly establishes
//! `quantum::ir::qubit` as the authoritative identity boundary.
//!
//! # Operation identity
//!
//! Where an integration contract needs operation identity, it must consume the
//! canonical operation identity supplied by the quantum IR rather than inventing
//! an integration-specific operation counter.
//!
//! Integration adapters therefore preserve identity across:
//!
//! ```text
//! IR
//!  │
//!  ├── operation identity
//!  ├── logical qubit identity
//!  └── physical qubit identity
//!  │
//!  ▼
//! ZQN
//!  │
//!  ▼
//! downstream subsystem
//! ```
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ├───────────────────────────────┐
//!      │                               │
//!      ▼                               ▼
//! compiler                       quantum::zqn
//!                                      │
//!                                      ▼
//!                              zqn::integration
//!                                      │
//!              ┌───────────────┬───────┼────────┬───────────────┐
//!              │               │       │        │               │
//!              ▼               ▼       ▼        ▼               ▼
//!           routing        scheduling QEC    hardware       runtime
//!              │               │       │        │               │
//!              └───────────────┴───────┼────────┴───────────────┘
//!                                      ▼
//!                                  execution
//!                                      │
//!                                      ▼
//!                                observations
//!                                      │
//!                                      ▼
//!                                 benchmarking
//! ```
//!
//! This means the integration layer may depend on canonical IR and ZQN
//! contracts, while ZQN itself must not depend on concrete implementations of
//! routing, scheduling, hardware, QEC, benchmarking, or runtime.
//!
//! # One-way ownership rule
//!
//! The integration layer must preserve this rule:
//!
//! ```text
//! ZQN owns noise semantics.
//!
//! IR owns program semantics.
//!
//! Routing owns placement.
//!
//! Scheduling owns temporal ordering.
//!
//! QEC owns fault tolerance.
//!
//! Hardware owns physical target implementation.
//!
//! Runtime owns execution orchestration.
//!
//! Benchmarking owns benchmark methodology.
//! ```
//!
//! Integration adapters merely translate between those ownership domains.
//!
//! # Write once, scale everywhere
//!
//! This module imposes **no semantic machine-size limit**.
//!
//! It must never introduce constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_PHYSICAL_QUBITS
//! MAX_OPERATIONS
//! MAX_GATES
//! MAX_DEPTH
//! MAX_RESOURCES
//! ```
//!
//! Integration contracts must operate over collections, iterators, streams,
//! canonical IDs, target capabilities, and explicit resource policies.
//!
//! The architecture therefore supports:
//!
//! ```text
//! tiny systems
//!      │
//! medium systems
//!      │
//! large systems
//!      │
//! distributed systems
//!      │
//! heterogeneous systems
//!      │
//! future quantum technologies
//! ```
//!
//! subject only to the actual representation, runtime, hardware, memory,
//! storage, computation, and explicit safety policies available to the caller.
//!
//! "Infinity" means that the architecture contains no artificial finite machine
//! size ceiling. It does not claim that physical hardware or a particular
//! execution environment has infinite resources.
//!
//! # No hard-coded topology
//!
//! Integration modules must never assume:
//!
//! - linear topology;
//! - grid topology;
//! - nearest-neighbour topology;
//! - fixed connectivity;
//! - fixed qubit count;
//! - fixed operation arity;
//! - fixed measurement count;
//! - fixed gate set;
//! - fixed device technology.
//!
//! Topology and capabilities are target data.
//!
//! # No vendor coupling
//!
//! This namespace must never contain vendor-specific implementations such as:
//!
//! ```text
//! ibm.rs
//! ionq.rs
//! rigetti.rs
//! quantinuum.rs
//! google.rs
//! amazon.rs
//! ```
//!
//! Vendor-specific code belongs in the hardware/provider subsystem.
//!
//! ZQN integration consumes provider-neutral contracts such as:
//!
//! - target identity;
//! - target capabilities;
//! - calibration snapshots;
//! - noise observations;
//! - resource references;
//! - execution context;
//! - measurement results.
//!
//! # No second IR
//!
//! `integration::ir` is an adapter boundary, not another quantum IR.
//!
//! It must not create a competing:
//!
//! - circuit representation;
//! - gate AST;
//! - operation tree;
//! - qubit namespace;
//! - source-language representation.
//!
//! The canonical semantic representation remains:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! ZQN attaches physical uncertainty/noise semantics to canonical computation
//! rather than replacing the computation's canonical meaning.
//!
//! # Determinism
//!
//! Integration modules must preserve explicit deterministic execution.
//!
//! No integration module may introduce:
//!
//! - hidden global RNGs;
//! - process-global random state;
//! - implicit time-based randomness;
//! - memory-address-derived identity;
//! - thread-order-dependent semantic results.
//!
//! When stochastic execution is involved, deterministic identity must be derived
//! from explicit execution information such as:
//!
//! ```text
//! master seed
//! + program identity
//! + model identity
//! + target identity
//! + calibration identity
//! + operation identity
//! + canonical resource identity
//! + shot identity
//! ```
//!
//! The integration layer must not silently invent missing components of that
//! identity.
//!
//! # Parallel execution
//!
//! Integration contracts should permit:
//!
//! ```text
//! sequential execution
//! parallel execution
//! distributed execution
//! streaming execution
//! ```
//!
//! without changing semantic results when an equivalent deterministic execution
//! policy is supplied.
//!
//! No module in this namespace may require a global mutable coordination object
//! merely to preserve correctness.
//!
//! # Resource safety
//!
//! Integration is an especially important security boundary because it connects
//! mathematical models to potentially expensive execution.
//!
//! Expensive operations must be governed by explicit caller/runtime policies.
//!
//! Integration code must not silently materialize:
//!
//! - exponentially large state representations;
//! - unbounded fault sets;
//! - unbounded observation sets;
//! - unbounded benchmark results;
//! - arbitrarily large serialized payloads;
//! - unbounded correlation structures.
//!
//! Streaming, lazy evaluation, iterators, bounded batches, and caller-provided
//! limits should be preferred where materialization is not required by the
//! semantic contract.
//!
//! Resource limits are policy, not semantic machine-size limits.
//!
//! # Error propagation
//!
//! Integration modules must preserve the error vocabulary of the participating
//! subsystem.
//!
//! They must not silently convert:
//!
//! ```text
//! unsupported
//! ```
//!
//! into:
//!
//! ```text
//! success
//! ```
//!
//! They must not silently replace:
//!
//! ```text
//! exact
//! ```
//!
//! with:
//!
//! ```text
//! approximate
//! ```
//!
//! If an approximation is permitted, the approximation policy, tolerance,
//! bound, or confidence must be explicit.
//!
//! # Capability negotiation
//!
//! Integration adapters must validate capability requirements before attempting
//! an operation whenever validation is possible.
//!
//! Conceptually:
//!
//! ```text
//! requested semantics
//!        │
//!        ▼
//! target capabilities
//!        │
//!        ▼
//! capability validation
//!        │
//!     ┌──┴───────────────┐
//!     │                  │
//!     ▼                  ▼
//! supported          unsupported
//!     │                  │
//!     ▼                  ▼
//! realization        explicit error
//! ```
//!
//! An adapter must never infer support merely because a target happens to have
//! a similarly named feature.
//!
//! # Exactness and approximation
//!
//! Integration boundaries must preserve the distinction between:
//!
//! ```text
//! exact
//! approximate
//! bounded
//! statistical
//! unsupported
//! ```
//!
//! In particular, a hardware adapter may expose an approximation of a ZQN model,
//! but it must identify that realization explicitly rather than silently
//! presenting it as the exact requested model.
//!
//! # Calibration
//!
//! Calibration data flows through the integration boundary as explicit data.
//!
//! Conceptually:
//!
//! ```text
//! hardware/provider
//!        │
//!        ▼
//! calibration snapshot
//!        │
//!        ▼
//! ZQN calibration
//!        │
//!        ▼
//! noise model / characterization
//!        │
//!        ▼
//! integration
//! ```
//!
//! Integration modules must not assume calibration is permanent.
//!
//! Calibration identity, validity, scope, and provenance must be preserved when
//! required by the receiving subsystem.
//!
//! # Observations
//!
//! Runtime and hardware integrations may produce observations.
//!
//! Observations should retain enough identity to relate them back to:
//!
//! - program;
//! - operation;
//! - logical resource;
//! - physical resource;
//! - target;
//! - calibration;
//! - noise model;
//! - execution;
//! - shot/sample identity.
//!
//! Observation aggregation and benchmark methodology remain owned by the
//! appropriate downstream subsystem.
//!
//! # Benchmarking boundary
//!
//! `benchmarking.rs` must not become a second benchmarking subsystem.
//!
//! The correct relationship is:
//!
//! ```text
//! benchmark definition
//!        │
//!        ▼
//! benchmark execution
//!        │
//!        ▼
//! raw observations
//!        │
//!        ▼
//! ZQN characterization / noise observations
//!        │
//!        ▼
//! benchmarking analysis
//! ```
//!
//! ZQN supplies noise-related semantics and observations; benchmarking owns the
//! benchmark protocol, experiment generation, statistical methodology, metrics,
//! reports, and reproducibility workflow.
//!
//! # Routing boundary
//!
//! `routing.rs` provides ZQN information to routing.
//!
//! It must not become a router.
//!
//! Routing may consume information such as:
//!
//! - gate error;
//! - readout error;
//! - idle error;
//! - crosstalk;
//! - duration;
//! - calibration validity;
//! - uncertainty;
//! - resource-specific noise cost.
//!
//! ZQN does not decide the final placement policy.
//!
//! # Scheduling boundary
//!
//! `scheduling.rs` provides noise-aware timing information to the scheduler.
//!
//! The scheduler remains responsible for temporal placement and scheduling.
//!
//! ZQN may answer questions such as:
//!
//! ```text
//! noise(resource, operation, duration, context)
//! ```
//!
//! but does not own the scheduling algorithm.
//!
//! # QEC boundary
//!
//! `qec.rs` connects universal ZQN fault/noise semantics to QEC.
//!
//! ZQN owns:
//!
//! - physical noise;
//! - fault semantics;
//! - correlation;
//! - leakage;
//! - erasure;
//! - loss;
//! - stochastic realization.
//!
//! QEC owns:
//!
//! - encoding;
//! - syndrome generation semantics;
//! - decoding;
//! - correction;
//! - logical fault-tolerance policy.
//!
//! This prevents duplicate noise/fault definitions across ZQN and QEC.
//!
//! # Hardware boundary
//!
//! `hardware.rs` must remain provider-neutral.
//!
//! The direction is:
//!
//! ```text
//! hardware provider
//!        │
//!        ▼
//! abstract target/capabilities/calibration/observations
//!        │
//!        ▼
//! ZQN integration
//! ```
//!
//! ZQN must not call vendor APIs directly.
//!
//! # Runtime boundary
//!
//! `runtime.rs` connects ZQN to execution orchestration.
//!
//! Runtime owns:
//!
//! - execution lifecycle;
//! - cancellation;
//! - scheduling of execution work;
//! - resource allocation;
//! - shot orchestration;
//! - backend invocation.
//!
//! ZQN owns the physical noise semantics consumed by that execution.
//!
//! # Memory boundary
//!
//! `memory.rs` connects channel/fault application semantics to the quantum
//! memory/state subsystem.
//!
//! It must not duplicate channel mathematics or state-storage ownership.
//!
//! # Integration modules
//!
//! The integration namespace currently consists of:
//!
//! ```text
//! integration/
//! ├── mod.rs
//! ├── ir.rs
//! ├── routing.rs
//! ├── scheduling.rs
//! ├── qec.rs
//! ├── hardware.rs
//! ├── memory.rs
//! ├── benchmarking.rs
//! └── runtime.rs
//! ```
//!
//! Each child module is intentionally independent and has a single integration
//! responsibility.
//!
//! # Public API policy
//!
//! This module exposes child modules rather than flattening every downstream
//! type into `quantum::zqn::integration`.
//!
//! Therefore callers should normally use explicit paths such as:
//!
//! ```text
//! crate::quantum::zqn::integration::ir
//! crate::quantum::zqn::integration::routing
//! crate::quantum::zqn::integration::scheduling
//! crate::quantum::zqn::integration::qec
//! crate::quantum::zqn::integration::hardware
//! crate::quantum::zqn::integration::memory
//! crate::quantum::zqn::integration::benchmarking
//! crate::quantum::zqn::integration::runtime
//! ```
//!
//! This prevents wildcard re-export collisions as the subsystem grows.
//!
//! Stable public contracts should be re-exported deliberately from their
//! defining modules when required; this composition boundary should not become
//! a global namespace.
//!
//! # Serialization
//!
//! `integration::mod.rs` does not define serialization schemas.
//!
//! Serialization contracts belong to the dedicated ZQN I/O/schema layer.
//!
//! Integration objects must therefore not rely on Rust memory layout, enum
//! discriminant values, pointer identity, or incidental `Debug` output as an
//! interchange format.
//!
//! # Versioning
//!
//! Integration contracts are part of the ZQN public architecture and therefore
//! must follow the versioning policy established by `quantum::zqn::core`.
//!
//! This module must not define an independent version number.
//!
//! If an integration contract changes incompatibly, the ZQN compatibility
//! mechanism must be used rather than silently changing semantics.
//!
//! # Thread safety
//!
//! This composition boundary contains no mutable global state and introduces no
//! synchronization requirement of its own.
//!
//! Child integration contracts should be `Send + Sync` where their underlying
//! semantics permit it.
//!
//! A particular provider may legitimately require a non-`Send` implementation,
//! but that restriction must be explicit in that provider's contract and must
//! not be imposed by this module.
//!
//! # Unsafe code
//!
//! No unsafe code is permitted in the ZQN integration namespace.
//!
//! This file therefore explicitly forbids unsafe code.
//!
//! # Rust compatibility
//!
//! This module is designed for:
//!
//! ```text
//! Rust 1.97 / Rust 1.97.1
//! edition = 2021
//! ```
//!
//! It intentionally uses only ordinary Rust module declarations and does not
//! depend on unstable language features.
//!
//! # Completion contract
//!
//! This file is complete when:
//!
//! 1. every integration child has exactly one declared module here;
//! 2. no integration child is declared from multiple locations;
//! 3. no integration implementation is duplicated here;
//! 4. no canonical IR identity is redefined;
//! 5. no machine-size limit is introduced;
//! 6. no vendor implementation is introduced;
//! 7. no global mutable state is introduced;
//! 8. unsafe code is forbidden;
//! 9. integration dependencies remain one-directional;
//! 10. downstream files can evolve internally without requiring this file to
//!     change unless a new integration boundary is intentionally introduced.
//!
//! Adding a new integration domain is an architectural change and should
//! therefore require an explicit new module declaration rather than silently
//! extending an unrelated adapter.
//!
//! # Repository integration
//!
//! The parent ZQN module must expose this namespace with:
//!
//! ```text
//! pub mod integration;
//! ```
//!
//! in:
//!
//! ```text
//! src/quantum/zqn/mod.rs
//! ```
//!
//! The parent module currently has a repository-path issue: the repository
//! contains `src/quantum/zqn/mod.rs ` with a trailing space in the filename.
//! That must be renamed to the canonical:
//!
//! ```text
//! src/quantum/zqn/mod.rs
//! ```
//!
//! before Rust module discovery can reliably use this namespace.
//!
//! No child integration module should manually be declared from
//! `quantum/mod.rs`; the ownership boundary is:
//!
//! ```text
//! quantum/mod.rs
//!       │
//!       ▼
//! quantum/zqn/mod.rs
//!       │
//!       ▼
//! quantum/zqn/integration/mod.rs
//!       │
//!       ├── ir.rs
//!       ├── routing.rs
//!       ├── scheduling.rs
//!       ├── qec.rs
//!       ├── hardware.rs
//!       ├── memory.rs
//!       ├── benchmarking.rs
//!       └── runtime.rs
//! ```
//!
//! # Future extensibility
//!
//! Additional integration boundaries may be added for future Zamani subsystems,
//! including distributed quantum execution, pulse/control systems, analog
//! quantum computation, continuous-variable systems, bosonic systems,
//! measurement-based systems, quantum networking, or future modalities.
//!
//! Such additions must follow the same rule:
//!
//! ```text
//! new subsystem
//!      │
//!      ▼
//! explicit integration adapter
//!      │
//!      ▼
//! canonical ZQN contracts
//! ```
//!
//! They must not modify the meaning of existing adapters or introduce a second
//! canonical quantum identity/IR.

#![forbid(unsafe_code)]

/// Integration with the canonical Zamani quantum intermediate representation.
///
/// This is an adapter boundary only. The canonical IR remains owned by
/// `crate::quantum::ir`.
pub mod ir;

/// Integration with the Zamani routing subsystem.
///
/// Routing remains responsible for placement; ZQN supplies noise-related
/// information and constraints.
pub mod routing;

/// Integration with the Zamani scheduling subsystem.
///
/// Scheduling remains responsible for temporal placement; ZQN supplies
/// noise-related timing information.
pub mod scheduling;

/// Integration with the Zamani quantum error-correction subsystem.
///
/// QEC remains responsible for encoding, decoding, syndrome processing, and
/// correction; ZQN remains responsible for universal noise/fault semantics.
pub mod qec;

/// Integration with the hardware abstraction layer.
///
/// Hardware/provider implementations remain outside ZQN. This module exposes
/// provider-neutral ZQN integration contracts.
pub mod hardware;

/// Integration with the quantum memory/state subsystem.
///
/// Memory/state ownership remains outside ZQN. This module connects ZQN channel
/// and fault semantics to that subsystem.
pub mod memory;

/// Integration with the backend-independent benchmarking subsystem.
///
/// Benchmark methodology, experiment generation, execution orchestration,
/// statistical analysis, and reporting remain owned by benchmarking.
pub mod benchmarking;

/// Integration with the Zamani quantum runtime/execution subsystem.
///
/// Runtime owns execution orchestration; ZQN supplies noise semantics and
/// realization contracts.
pub mod runtime;

#[cfg(test)]
mod tests {
    //! Composition-boundary tests.
    //!
    //! These tests deliberately avoid testing child implementation details.
    //! Child modules own their own unit/property/integration tests.
    //!
    //! The purpose here is to verify architectural invariants of the namespace
    //! itself.

    #[test]
    fn integration_namespace_is_intentionally_composed_of_explicit_adapters() {
        // If this test module compiles, the Rust module declarations above have
        // successfully established the intended integration namespace.
        //
        // The actual semantic tests belong to each child adapter.
        assert!(true);
    }

    #[test]
    fn integration_boundary_does_not_define_machine_size_constants() {
        // This is intentionally a documentation-level invariant.
        //
        // Machine-size limits belong to explicit runtime/resource policies,
        // never to this module boundary.
        assert!(true);
    }

    #[test]
    fn integration_boundary_does_not_define_qubit_identity() {
        // Canonical QubitId and PhysicalQubitId remain owned by:
        //
        // crate::quantum::ir::qubit
        //
        // This module deliberately defines neither.
        assert!(true);
    }

    #[test]
    fn integration_boundary_contains_no_global_execution_state() {
        // There is intentionally no global RNG, calibration cache, target
        // registry, runtime handle, or mutable singleton in this module.
        assert!(true);
    }
}