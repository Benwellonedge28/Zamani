//! Zamani Quantum Noise (ZQN)
//!
//! Production composition boundary for the Zamani Quantum Noise subsystem.
//!
//! # Purpose
//!
//! `quantum::zqn` is the canonical, backend-independent subsystem for
//! representing and coordinating quantum noise, uncertainty, faults,
//! calibration-dependent imperfections, characterization results, and
//! noise-aware execution semantics.
//!
//! ZQN answers:
//!
//! > What physical uncertainty, noise, fault, calibration uncertainty,
//! > stochastic effect, environmental effect, or explicitly declared
//! > approximation affects this quantum computation?
//!
//! ZQN does **not** answer:
//!
//! > What does the quantum program mean?
//!
//! That responsibility belongs to:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! The canonical architecture is:
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                    quantum::frontend
//!                              │
//!                              ▼
//!                     ┌────────────────┐
//!                     │  quantum::ir   │
//!                     │ canonical WHAT │
//!                     └───────┬────────┘
//!                             │
//!               ┌─────────────┼─────────────┐
//!               │             │             │
//!               ▼             ▼             ▼
//!          algorithms    optimization    analysis
//!                             │
//!                             ▼
//!                     ┌──────────────┐
//!                     │     ZQN      │
//!                     │ physical     │
//!                     │ uncertainty  │
//!                     └──────┬───────┘
//!                            │
//!              ┌─────────────┼─────────────┐
//!              │             │             │
//!              ▼             ▼             ▼
//!           routing      scheduling        QEC
//!              │             │             │
//!              └─────────────┼─────────────┘
//!                            ▼
//!                     target / hardware
//!                            │
//!                 ┌──────────┼──────────┐
//!                 ▼          ▼          ▼
//!             simulator      QPU      emulator
//!                 │          │          │
//!                 └──────────┼──────────┘
//!                            ▼
//!                         runtime
//!                            │
//!                            ▼
//!                       observations
//!                            │
//!             ┌──────────────┼──────────────┐
//!             ▼              ▼              ▼
//!       characterization  benchmarking   propagation
//!             │
//!             ▼
//!         calibration
//!             │
//!             └──────────────► ZQN
//! ```
//!
//! # Architectural ownership
//!
//! The ownership boundaries are intentionally strict.
//!
//! ```text
//! quantum::ir
//!     WHAT the computation means.
//!
//! quantum::zqn
//!     WHAT physical uncertainty/noise affects it.
//!
//! quantum::routing
//!     WHERE logical resources are placed.
//!
//! quantum::scheduling
//!     WHEN operations execute.
//!
//! quantum::hardware
//!     WHAT a target provides.
//!
//! runtime
//!     HOW execution is orchestrated.
//!
//! error_correction
//!     HOW fault tolerance, encoding, decoding and correction are performed.
//!
//! benchmarking
//!     HOW the resulting system is measured.
//! ```
//!
//! ZQN must never become a second canonical quantum IR.
//!
//! # Canonical identity boundary
//!
//! ZQN does **not** define `QubitId` or `PhysicalQubitId`.
//!
//! The authoritative quantum-resource identity types are:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Any ZQN subsystem that needs to refer to a quantum resource must use those
//! canonical types where they are semantically appropriate.
//!
//! ZQN-owned identifiers are reserved for ZQN semantic objects, for example:
//!
//! ```text
//! NoiseModelId
//! ChannelId
//! FaultId
//! CalibrationId
//! CharacterizationId
//! NoiseSnapshotId
//! ExperimentId
//! ObservationId
//! ```
//!
//! ZQN must never create:
//!
//! ```text
//! struct QubitId(...);
//! type QubitId = ...;
//! struct PhysicalQubitId(...);
//! ```
//!
//! This prevents identity fragmentation across:
//!
//! ```text
//! frontend
//!     ↓
//! quantum::ir
//!     ↓
//! ZQN
//!     ↓
//! routing
//!     ↓
//! scheduling
//!     ↓
//! QEC
//!     ↓
//! hardware
//! ```
//!
//! # Write once, scale everywhere
//!
//! ZQN has no semantic upper bound on:
//!
//! - logical quantum resources;
//! - physical quantum resources;
//! - operation count;
//! - noise-location count;
//! - circuit depth;
//! - correlation cardinality;
//! - execution duration;
//! - shot count;
//! - distributed resources;
//! - execution nodes;
//! - quantum technology;
//! - target size.
//!
//! No ZQN source file in this composition boundary contains:
//!
//! ```text
//! MAX_QUBITS
//! MAX_PHYSICAL_QUBITS
//! MAX_QUBIT_INDEX
//! MAX_OPERATIONS
//! MAX_DEPTH
//! MAX_CORRELATED_QUBITS
//! MAX_FAULTS
//! ```
//!
//! The architectural meaning of "infinity" is:
//!
//! > No artificial finite quantum-machine-size ceiling is encoded in ZQN.
//!
//! It does **not** mean that a concrete process, simulator, computer, QPU,
//! network, filesystem, or memory subsystem has infinite resources.
//!
//! Concrete resource limits are runtime, target, security, compiler, memory,
//! simulator, or execution policies.
//!
//! This distinction is fundamental:
//!
//! ```text
//! ZQN semantics
//!     │
//!     └── no artificial machine-size ceiling
//!
//! resource policy
//!     │
//!     └── may impose finite limits for safety/capacity
//!
//! target capability
//!     │
//!     └── describes what the selected target can actually provide
//! ```
//!
//! A configured execution limit therefore means:
//!
//! ```text
//! "this invocation permits N resources"
//! ```
//!
//! and must never be interpreted as:
//!
//! ```text
//! "Zamani supports only N resources"
//! ```
//!
//! # Quantum-technology independence
//!
//! ZQN must not assume that quantum computing means only:
//!
//! ```text
//! qubit + gate + Pauli error
//! ```
//!
//! The architecture must remain capable of representing noise affecting:
//!
//! - gate-model systems;
//! - dynamic circuits;
//! - measurement-based computation;
//! - analog computation;
//! - Hamiltonian computation;
//! - annealing;
//! - QUBO;
//! - bosonic systems;
//! - continuous-variable systems;
//! - fermionic systems;
//! - photonic systems;
//! - distributed quantum computation;
//! - logical/fault-tolerant computation;
//! - transport-based systems;
//! - future quantum modalities.
//!
//! Representation choice belongs to the appropriate ZQN subsystem and target
//! capability contract rather than this composition root.
//!
//! # Explicit approximation contract
//!
//! ZQN must never silently replace an expensive or unsupported physical model
//! with an approximation.
//!
//! Any approximation must be explicit and carry an applicable contract such as:
//!
//! ```text
//! Exact
//! Approximate { tolerance }
//! Bounded { error_bound }
//! Statistical { confidence }
//! Unsupported
//! ```
//!
//! The realized representation must therefore be distinguishable from the
//! requested semantic model.
//!
//! # Determinism
//!
//! This composition root owns no random-number generator.
//!
//! ZQN stochastic execution must be driven by explicit caller/runtime context.
//!
//! A reproducible stochastic execution may derive its identity from:
//!
//! ```text
//! master seed
//! + program identity
//! + noise-model identity
//! + calibration identity
//! + target identity
//! + operation identity
//! + resource identity
//! + shot identity
//! ```
//!
//! No ZQN composition module may depend on:
//!
//! - process-global RNG state;
//! - current wall-clock time as an implicit seed;
//! - thread identity as semantic randomness;
//! - memory addresses as semantic identity;
//! - unordered iteration as semantic ordering.
//!
//! This permits sequential and parallel executions to share the same explicit
//! reproducibility contract.
//!
//! # Resource safety
//!
//! ZQN must be capable of processing extremely large computations without
//! embedding artificial architectural limits.
//!
//! Implementations should therefore support, where appropriate:
//!
//! - lazy evaluation;
//! - iterators;
//! - streaming;
//! - bounded batches;
//! - sparse representations;
//! - tensorized representations;
//! - symbolic representations;
//! - sampled representations;
//! - target-native representations;
//! - distributed processing;
//! - explicit resource policies;
//! - cancellation;
//! - checked arithmetic.
//!
//! Expensive operations must not assume unlimited memory or CPU capacity.
//!
//! Resource admission belongs to explicit policy objects, especially
//! `core::limits`, runtime policy, simulator policy, and target capability.
//!
//! # Numerical integrity
//!
//! ZQN must reject invalid numerical states rather than silently repairing them.
//!
//! In particular:
//!
//! - `NaN` must never become a valid probability;
//! - infinities must never silently become finite values;
//! - invalid probabilities must not be clamped without an explicit contract;
//! - invalid bounds must not silently swap endpoints;
//! - invalid channels must not silently become different channels;
//! - invalid calibration data must not silently become valid calibration;
//! - numerical overflow/underflow must be handled explicitly by the owning
//!   numerical subsystem.
//!
//! A numerical approximation is valid only when its approximation contract is
//! explicit.
//!
//! # Serialization
//!
//! ZQN's external persistence/interchange contract belongs to:
//!
//! ```text
//! crate::quantum::zqn::io
//! ```
//!
//! This composition root does not define a wire format.
//!
//! Rust's internal memory representation must never accidentally become the
//! public persistence ABI.
//!
//! # Provenance
//!
//! ZQN results must be capable of carrying scientific/reproducibility
//! provenance through the dedicated provenance and characterization layers.
//!
//! Relevant provenance may include:
//!
//! - ZQN version;
//! - model identity;
//! - model configuration identity;
//! - calibration identity;
//! - target identity;
//! - experiment identity;
//! - observation identity;
//! - source classification;
//! - timestamp where scientifically relevant;
//! - software identity;
//! - numerical policy;
//! - approximation/error contract.
//!
//! Provenance must not depend on process-local addresses or unstable collection
//! ordering.
//!
//! # Security
//!
//! ZQN can eventually consume externally supplied noise models, calibration
//! records, characterization results and serialized data.
//!
//! The architecture therefore treats the following as untrusted-input risks:
//!
//! - allocation bombs;
//! - enormous distributions;
//! - enormous correlated-fault sets;
//! - pathological correlation graphs;
//! - malformed serialized models;
//! - non-finite numerical values;
//! - integer overflow;
//! - pathological iteration;
//! - nonterminating generators;
//! - malicious calibration data;
//! - resource exhaustion.
//!
//! The composition root does not implement those checks itself. They belong to
//! the owning constructors, validators, deserializers and explicit resource
//! policies.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! core
//!   │
//!   ├──────────────► probability
//!   │
//!   ├──────────────► channel
//!   │
//!   ├──────────────► fault
//!   │
//!   └──────────────► noise
//!                         │
//!                         ├──► operations
//!                         ├──► calibration
//!                         └──► characterization
//!                                      │
//!                                      ▼
//!                                  simulation
//!                                      │
//!                                      ▼
//!                                 propagation
//!                                      │
//!                                      ▼
//!                                   target
//!                                      │
//!                                      ▼
//!                                 integration
//!                                      │
//!                                      ▼
//!                                      io
//! ```
//!
//! This is a conceptual dependency graph. Individual implementations should
//! depend only on the narrow interfaces they actually require.
//!
//! The composition root itself performs no domain work.
//!
//! # Forbidden dependencies
//!
//! `quantum::zqn` must not become directly coupled to:
//!
//! - frontend ASTs;
//! - source-language syntax;
//! - vendor SDKs;
//! - QPU credentials;
//! - provider network clients;
//! - hardware connection management;
//! - simulator implementations;
//! - routing algorithms;
//! - scheduling algorithms;
//! - QEC decoder implementations;
//! - benchmark implementations;
//! - UI;
//! - CLI;
//! - global mutable quantum state.
//!
//! Provider-specific implementation belongs to the hardware subsystem.
//!
//! ZQN consumes provider-neutral target capabilities, calibration information,
//! observations and resource references.
//!
//! # Integration with canonical Quantum IR
//!
//! The canonical semantic program remains:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! ZQN is a downstream physical/noise semantics layer:
//!
//! ```text
//! QuantumProgram / Quantum IR
//!             │
//!             ▼
//!       ZQN attachment
//!             │
//!             ▼
//!       Noise realization
//! ```
//!
//! ZQN must not replace the canonical IR with a competing circuit or operation
//! representation.
//!
//! The canonical qubit identity remains:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! # Integration with routing
//!
//! Routing may consume ZQN-derived physical costs such as:
//!
//! - gate error;
//! - readout error;
//! - idle error;
//! - crosstalk;
//! - duration-dependent error;
//! - correlation penalties;
//! - calibration uncertainty;
//! - target-specific fidelity estimates.
//!
//! ZQN does not perform routing.
//!
//! The intended relationship is:
//!
//! ```text
//! canonical IR
//!      │
//!      ├──────────────► routing
//!      │                  │
//!      │                  ▼
//!      │              placement
//!      │                  ▲
//!      │                  │
//!      └────────────── ZQN cost information
//! ```
//!
//! # Integration with scheduling
//!
//! Scheduling may query ZQN for noise effects associated with:
//!
//! - operation duration;
//! - idle duration;
//! - resource occupancy;
//! - crosstalk;
//! - time-dependent noise;
//! - calibration validity.
//!
//! ZQN does not schedule operations.
//!
//! # Integration with QEC
//!
//! ZQN is the long-term authoritative source for universal physical-noise
//! semantics.
//!
//! QEC owns:
//!
//! - encodings;
//! - syndrome generation algorithms;
//! - decoding;
//! - correction;
//! - logical fault-tolerance mechanisms;
//! - logical error analysis.
//!
//! The intended migration boundary is:
//!
//! ```text
//!                 ZQN
//!                  │
//!                  ▼
//!          physical noise model
//!                  │
//!                  ▼
//!          QEC physical-fault adapter
//!                  │
//!          ┌───────┼────────┐
//!          ▼       ▼        ▼
//!       syndrome decoder correction
//! ```
//!
//! Existing QEC noise functionality must not be duplicated indefinitely.
//!
//! # Integration with hardware
//!
//! Hardware providers supply provider-neutral information such as:
//!
//! - target capabilities;
//! - physical resource references;
//! - calibration snapshots;
//! - observations;
//! - timing information;
//! - supported representations.
//!
//! ZQN must not call vendor APIs directly.
//!
//! The relationship is:
//!
//! ```text
//! hardware adapter
//!       │
//!       ├──► target capabilities
//!       ├──► calibration
//!       └──► observations
//!                 │
//!                 ▼
//!                ZQN
//! ```
//!
//! # Integration with memory and simulation
//!
//! ZQN defines physical noise semantics.
//!
//! Memory/state implementations consume those semantics when a channel,
//! stochastic realization, fault or other supported physical process must be
//! applied to a quantum state representation.
//!
//! ZQN does not own state-vector, density-matrix, tensor-network, stabilizer,
//! trajectory, or other state-storage implementations.
//!
//! # Integration with benchmarking
//!
//! Benchmarking consumes ZQN-derived observations, characterization results,
//! calibration state and error estimates.
//!
//! ZQN does not own benchmark orchestration.
//!
//! The intended flow is:
//!
//! ```text
//! benchmark
//!     │
//!     ▼
//! execution
//!     │
//!     ▼
//! observations
//!     │
//!     ▼
//! ZQN characterization
//!     │
//!     ▼
//! noise/calibration model
//!     │
//!     └──────────────► later execution/benchmarking
//! ```
//!
//! # Integration with runtime
//!
//! Runtime supplies explicit execution context, including where applicable:
//!
//! - deterministic seed policy;
//! - cancellation;
//! - resource limits;
//! - target capabilities;
//! - calibration scope;
//! - numerical policy;
//! - execution identity.
//!
//! ZQN returns semantic noise/channel/fault/observation information.
//!
//! Runtime remains responsible for execution orchestration.
//!
//! # Module structure
//!
//! The production ZQN subsystem is divided into independently maintainable
//! boundaries:
//!
//! ```text
//! zqn/
//! ├── mod.rs
//! │
//! ├── core/
//! │   ├── mod.rs
//! │   ├── error.rs
//! │   ├── ids.rs
//!   │   ├── metadata.rs
//!   │   ├── version.rs
//!   │   ├── context.rs
//!   │   ├── capabilities.rs
//!   │   ├── limits.rs
//!   │   └── provenance.rs
//! │
//! ├── probability/
//! ├── channel/
//! ├── fault/
//! ├── noise/
//! ├── operations/
//! ├── calibration/
//! ├── characterization/
//! ├── simulation/
//! ├── propagation/
//! ├── target/
//! ├── integration/
//! └── io/
//! ```
//!
//! Each child composition module owns its internal files.
//!
//! This root must not recreate those trees inline with `#[path]` attributes.
//!
//! # Public API policy
//!
//! Child module paths remain public because they are independent integration
//! boundaries.
//!
//! This root deliberately avoids a giant glob re-export such as:
//!
//! ```text
//! pub use core::*;
//! pub use channel::*;
//! pub use noise::*;
//! ```
//!
//! Glob exports would make ownership ambiguous, increase accidental name
//! collisions, and cause unrelated future additions to change this public API.
//!
//! Consumers should therefore prefer:
//!
//! ```text
//! crate::quantum::zqn::core
//! crate::quantum::zqn::probability
//! crate::quantum::zqn::channel
//! crate::quantum::zqn::fault
//! crate::quantum::zqn::noise
//! crate::quantum::zqn::operations
//! crate::quantum::zqn::calibration
//! crate::quantum::zqn::characterization
//! crate::quantum::zqn::simulation
//! crate::quantum::zqn::propagation
//! crate::quantum::zqn::target
//! crate::quantum::zqn::integration
//! crate::quantum::zqn::io
//! ```
//!
//! Specialized public types should be imported from their authoritative
//! subsystem rather than copied into this root.
//!
//! # Why this file contains no ZQN domain implementation
//!
//! `mod.rs` is a composition root.
//!
//! If a probability invariant changes, only the probability subsystem should
//! need modification.
//!
//! If a channel representation changes, only the channel subsystem should need
//! modification.
//!
//! If a calibration format changes, only calibration and its explicitly
//! versioned consumers should need modification.
//!
//! If a new quantum technology is introduced, existing mathematical contracts
//! should remain usable without changing this root.
//!
//! If a new noise model is introduced, it should normally be added beneath
//! `noise` without modifying unrelated subsystems.
//!
//! This is what allows independently completed files to remain independently
//! maintainable.
//!
//! # Adding a future ZQN subsystem
//!
//! A future subsystem should normally require:
//!
//! 1. a new directory;
//! 2. its own `mod.rs`;
//! 3. its own implementation and tests;
//! 4. an explicit dependency contract;
//! 5. one `pub mod` declaration here.
//!
//! Existing unrelated ZQN implementation files should not have to be edited
//! merely because the new subsystem exists.
//!
//! # No vendor-specific modules
//!
//! Do not add provider modules here such as:
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
//! Provider-specific code belongs to the hardware layer.
//!
//! # No global state
//!
//! This composition root:
//!
//! - owns no global mutable state;
//! - performs no I/O;
//! - performs no network access;
//! - initializes no devices;
//! - initializes no simulators;
//! - creates no RNG;
//! - performs no benchmark execution;
//! - performs no source parsing;
//! - performs no memory-state allocation merely by being imported.
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
//! The safety requirement is compiler-enforced below.
//!
//! # Testing
//!
//! Domain-specific tests belong to the child subsystems.
//!
//! This composition root should be validated primarily through normal Rust
//! module compilation and workspace integration tests.
//!
//! The root must not name speculative types merely to manufacture tests here.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! 1. the filename is exactly `src/quantum/zqn/mod.rs`;
//! 2. all declared child composition modules exist;
//! 3. no child implementation is duplicated here;
//! 4. no competing quantum-resource identity is introduced;
//! 5. `quantum::ir::qubit` remains authoritative for quantum identities;
//! 6. no semantic machine-size limit is introduced;
//! 7. no vendor SDK is imported;
//! 8. no hardware connection is opened;
//! 9. no global mutable state is introduced;
//! 10. no global RNG is introduced;
//! 11. no unsafe Rust is permitted;
//! 12. the module is valid on Rust 1.97/1.97.1;
//! 13. external serialization remains owned by `io`;
//! 14. mathematical semantics remain owned by the relevant child subsystem;
//! 15. downstream modules can depend on stable subsystem paths;
//! 16. adding an independent ZQN subsystem does not require restructuring
//!     unrelated implementations;
//! 17. integration responsibilities remain explicit and one-directional.
//!
//! =============================================================================
//! Compiler-enforced safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Authoritative ZQN subsystem boundaries
// =============================================================================

/// Dependency-lowest ZQN infrastructure.
///
/// Owns errors, identifiers, metadata, versioning, context, capabilities,
/// explicit resource limits, and provenance.
pub mod core;

/// Backend-independent probability mathematics.
///
/// Owns validated probability values, distributions, bounds, continuous
/// distributions and statistics.
pub mod probability;

/// Quantum-channel semantics and channel representations.
///
/// Owns representation-independent channel abstractions and concrete channel
/// representations.
pub mod channel;

/// Realized physical and logical fault semantics.
///
/// Owns fault classification, locations, correlations, leakage, loss, erasure,
/// and fault batching.
pub mod fault;

/// Declarative and executable quantum-noise semantics.
///
/// Owns noise models, specifications, application, composition, correlations,
/// temporal/spatial behavior, crosstalk, drift, conditional noise and
/// non-Markovian behavior.
pub mod noise;

/// Noise associated with quantum operations.
///
/// Owns operation-level noise semantics for gates, preparation, reset,
/// measurement, idle time, pulses and transport.
pub mod operations;

/// Calibration state and calibration-derived uncertainty.
///
/// Owns calibration snapshots, parameters, device/gate/readout calibration,
/// drift, interpolation and validation.
pub mod calibration;

/// Experimental characterization of physical noise.
///
/// Owns characterization experiments, protocols, observations, estimators,
/// uncertainty and tomography/benchmarking-derived characterization.
pub mod characterization;

/// Noise-aware simulation and stochastic execution.
///
/// Owns simulation engines, sampling, trajectories, Monte Carlo execution,
/// deterministic execution and reproducibility machinery.
pub mod simulation;

/// Error/fidelity/uncertainty propagation.
///
/// Owns error budgets, fidelity analysis, bounds, sensitivity and accumulation
/// analysis.
pub mod propagation;

/// Target-independent requirements and target capability negotiation.
///
/// Owns compatibility checks and target-specific ZQN lowering contracts, but
/// does not own vendor APIs or QPU connections.
pub mod target;

/// Provider-neutral integration contracts.
///
/// Connects ZQN to canonical IR, routing, scheduling, QEC, hardware, memory,
/// benchmarking and runtime without moving ownership of those systems into
/// ZQN.
pub mod integration;

/// Versioned persistence and interchange boundary.
///
/// Owns ZQN schema, serialization, deserialization, canonical representation
/// and compatibility/migration contracts.
pub mod io;

// =============================================================================
// Public API policy
// =============================================================================
//
// Deliberately no glob re-exports:
//
//     pub use core::*;
//     pub use channel::*;
//     pub use noise::*;
//
// Such exports make ownership ambiguous and make unrelated future additions
// capable of changing this module's public namespace.
//
// Consumers should use explicit subsystem paths:
//
//     crate::quantum::zqn::core
//     crate::quantum::zqn::probability
//     crate::quantum::zqn::channel
//     crate::quantum::zqn::fault
//     crate::quantum::zqn::noise
//     crate::quantum::zqn::operations
//     crate::quantum::zqn::calibration
//     crate::quantum::zqn::characterization
//     crate::quantum::zqn::simulation
//     crate::quantum::zqn::propagation
//     crate::quantum::zqn::target
//     crate::quantum::zqn::integration
//     crate::quantum::zqn::io
//
// Canonical quantum-resource identities remain available through:
//
//     crate::quantum::ir::qubit::QubitId
//     crate::quantum::ir::qubit::PhysicalQubitId
//
// ZQN must not re-export competing identity definitions.
//
// =============================================================================
// Integration invariants
// =============================================================================
//
// The following invariants are intentionally expressed by the composition
// structure rather than implemented as runtime behavior:
//
// 1. quantum::ir remains the canonical semantic program representation.
// 2. quantum::ir::qubit remains the canonical quantum-resource identity.
// 3. ZQN owns physical-noise semantics.
// 4. Hardware owns provider/device implementation.
// 5. Runtime owns execution orchestration.
// 6. QEC owns fault-tolerance algorithms.
// 7. Routing owns placement.
// 8. Scheduling owns temporal ordering.
// 9. Benchmarking owns benchmark orchestration.
// 10. IO owns external ZQN persistence/interchange.
// 11. Resource limits are policy, not architectural machine-size limits.
// 12. Stochastic execution uses explicit deterministic context.
// 13. Approximation is explicit rather than silent.
// 14. No global mutable state exists in this composition root.
// 15. No unsafe Rust is permitted.
// 16. No vendor-specific implementation exists in this namespace root.
//
// =============================================================================
// End of composition root
// =============================================================================