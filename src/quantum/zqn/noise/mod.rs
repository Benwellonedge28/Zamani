//! # Zamani Quantum Noise (ZQN) — Noise Subsystem
//!
//! This module is the composition boundary for the ZQN noise subsystem.
//!
//! ZQN provides backend-independent semantics for representing, validating,
//! composing, applying, characterizing, and propagating quantum noise.
//!
//! ## Architectural position
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
//!      │ canonical program semantics
//!      │ canonical QubitId / PhysicalQubitId / OperationId
//!      ▼
//! quantum::zqn
//!      │
//!      ▼
//! quantum::zqn::noise
//!      │
//!      ├── model
//!      ├── specification
//!      ├── application
//!      ├── composition
//!      ├── correlation
//!      ├── spatial
//!      ├── temporal
//!      ├── drift
//!      ├── crosstalk
//!      └── non_markovian
//!      │
//!      ├───────────────┬───────────────┬───────────────┐
//!      ▼               ▼               ▼               ▼
//!   channel          fault         calibration    characterization
//!      │               │               │               │
//!      └───────────────┴───────────────┴───────────────┘
//!                              │
//!                              ▼
//!                         execution layers
//!                              │
//!                 ┌────────────┼────────────┐
//!                 ▼            ▼            ▼
//!             simulation      QEC       hardware
//!                 │            │            │
//!                 └────────────┼────────────┘
//!                              ▼
//!                 routing / scheduling / runtime
//!                              │
//!                              ▼
//!                         benchmarking
//! ```
//!
//! ## Core responsibility
//!
//! This module owns the *composition boundary* of ZQN noise.
//!
//! It owns:
//!
//! - child-module organization;
//! - module visibility;
//! - subsystem-level architectural contracts;
//! - dependency-direction documentation;
//! - the boundary between noise semantics and downstream execution systems;
//! - the boundary between canonical Quantum IR and physical noise semantics.
//!
//! It does **not** own the mathematical implementation of individual noise
//! mechanisms.
//!
//! ## Non-responsibilities
//!
//! This module does not own:
//!
//! - the canonical Quantum IR;
//! - quantum resource identities;
//! - gate definitions;
//! - quantum state representations;
//! - channel mathematics;
//! - probability mathematics;
//! - fault mathematics;
//! - calibration acquisition;
//! - characterization protocols;
//! - simulation engines;
//! - routing;
//! - scheduling;
//! - QEC decoding;
//! - hardware APIs;
//! - hardware credentials;
//! - networking;
//! - persistence;
//! - serialization wire formats;
//! - random-number generation;
//! - global mutable state.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! ## Canonical Quantum IR identity
//!
//! Whenever ZQN refers to quantum resources already represented by the Quantum
//! IR, it must use the canonical identities from:
//!
//! ```text
//! crate::quantum::ir::qubit
//! crate::quantum::ir::identity
//! ```
//!
//! In particular, ZQN must not introduce a competing quantum-resource identity
//! type when the canonical IR type is semantically sufficient.
//!
//! The intended identities include:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! crate::quantum::ir::identity::OperationId
//! ```
//!
//! This prevents identity fragmentation between:
//!
//! ```text
//! frontend
//!   ↓
//! IR
//!   ↓
//! ZQN
//!   ↓
//! routing
//!   ↓
//! scheduling
//!   ↓
//! QEC
//!   ↓
//! hardware
//! ```
//!
//! ## Write once, scale everywhere
//!
//! ZQN has no semantic upper bound on:
//!
//! - number of quantum resources;
//! - number of operations;
//! - circuit depth;
//! - number of noise locations;
//! - correlation cardinality;
//! - execution duration;
//! - number of shots;
//! - number of distributed resources;
//! - number of execution nodes.
//!
//! This module intentionally contains no constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_NOISE_LOCATIONS
//! MAX_CORRELATED_QUBITS
//! MAX_FAULTS
//! ```
//!
//! Such limits are not semantic properties of quantum noise.
//!
//! Concrete implementations may still require resource limits to protect
//! against exhaustion of:
//!
//! - memory;
//! - CPU;
//! - accelerator memory;
//! - storage;
//! - network resources;
//! - execution time;
//! - target capacity;
//! - configured runtime budgets.
//!
//! Those limits must be explicit execution/resource policy rather than hidden
//! semantic limits.
//!
//! "Infinity" therefore means:
//!
//! ```text
//! no artificial finite architectural ceiling
//! ```
//!
//! It does not mean that an implementation can exceed the resources available
//! to the process or target.
//!
//! ## Scalability requirements
//!
//! Implementations below this module should prefer representations appropriate
//! to the requested computation and available resources, including:
//!
//! - lazy evaluation;
//! - iterators;
//! - streaming;
//! - bounded batches;
//! - sparse representations;
//! - tensorized representations;
//! - symbolic representations;
//! - stochastic sampling;
//! - target-native representations;
//! - distributed execution;
//! - caller-provided resource policies.
//!
//! An implementation must not silently substitute an inaccurate approximation
//! merely because a requested representation is expensive.
//!
//! Any approximation must be explicit and accompanied by its applicable
//! accuracy/error contract.
//!
//! ## Quantum-technology independence
//!
//! ZQN must not assume that every quantum system is:
//!
//! ```text
//! qubit + gate + Pauli error
//! ```
//!
//! The subsystem must remain extensible to noise affecting:
//!
//! - qubits;
//! - qudits;
//! - modes;
//! - bosonic systems;
//! - continuous-variable systems;
//! - analog systems;
//! - Hamiltonian systems;
//! - annealing systems;
//! - fermionic systems;
//! - measurement-based systems;
//! - logical resources;
//! - physical resources;
//! - transport resources;
//! - distributed quantum links;
//! - future quantum modalities.
//!
//! The concrete mathematical representation is selected by the appropriate
//! channel/noise subsystem and target capabilities.
//!
//! ## Noise is not synonymous with a fault
//!
//! ZQN distinguishes:
//!
//! ```text
//! noise model
//!     │
//!     ├── channel
//!     ├── stochastic process
//!     ├── coherent deviation
//!     ├── correlated process
//!     ├── drift
//!     ├── leakage
//!     ├── loss
//!     └── discrete fault realization
//! ```
//!
//! A fault is therefore one possible realization or abstraction of noise; it
//! is not the universal definition of noise.
//!
//! ## Module responsibilities
//!
//! ### `model`
//!
//! Defines the backend-independent noise-model contract.
//!
//! A noise model describes physical uncertainty/noise semantics without
//! owning simulator execution or hardware access.
//!
//! ### `specification`
//!
//! Defines declarative noise specifications.
//!
//! This is the appropriate boundary for compiler, configuration, and language
//! layers to express noise intent without coupling source syntax to a concrete
//! physical implementation.
//!
//! ### `application`
//!
//! Defines how a noise model is associated with an execution scope, operation,
//! resource, time interval, measurement, reset, or other supported location.
//!
//! It maintains the distinction between:
//!
//! ```text
//! model
//! application
//! channel
//! fault
//! ```
//!
//! ### `composition`
//!
//! Defines representation-independent composition of noise semantics.
//!
//! Composition must not depend on a fixed machine size or fixed hardware gate
//! set.
//!
//! ### `correlation`
//!
//! Defines general correlation semantics.
//!
//! Correlation cardinality is data and execution context, not a hard-coded
//! architectural constant.
//!
//! ### `spatial`
//!
//! Defines spatially dependent noise.
//!
//! Topology comes from the applicable quantum-resource context rather than
//! from hard-coded assumptions about a particular hardware architecture.
//!
//! ### `temporal`
//!
//! Defines time-dependent noise semantics.
//!
//! It does not own scheduling or the execution clock.
//!
//! ### `drift`
//!
//! Defines deterministic semantic representations of changing noise
//! parameters/models.
//!
//! Drift must not implicitly read wall-clock time when semantic reproducibility
//! is required. Execution context must provide the relevant temporal state.
//!
//! ### `crosstalk`
//!
//! Defines provider-independent crosstalk semantics.
//!
//! It describes unwanted interactions between resources but does not perform
//! routing or scheduling.
//!
//! ### `non_markovian`
//!
//! Defines noise with memory or history dependence.
//!
//! This prevents the rest of the subsystem from assuming that all noise is
//! memoryless.
//!
//! ## Dependency direction
//!
//! The intended conceptual dependency direction is:
//!
//! ```text
//! ZQN core
//!    │
//!    ▼
//! probability / channel / fault
//!    │
//!    ▼
//! noise
//!    │
//!    ├── model
//!    ├── specification
//!    ├── application
//!    ├── composition
//!    ├── correlation
//!    ├── spatial
//!    ├── temporal
//!    ├── drift
//!    ├── crosstalk
//!    └── non_markovian
//! ```
//!
//! This file is the composition boundary. It must not create reverse
//! dependencies from ZQN noise into concrete implementations of:
//!
//! - frontend;
//! - routing;
//! - scheduling;
//! - QEC;
//! - hardware;
//! - runtime;
//! - benchmarking.
//!
//! ## Quantum IR integration
//!
//! The intended relationship is:
//!
//! ```text
//! quantum::ir
//!      │
//!      │ OperationId / QubitId / PhysicalQubitId
//!      ▼
//! ZQN noise application
//!      │
//!      ▼
//! NoiseModel
//!      │
//!      ▼
//! NoiseApplication
//! ```
//!
//! Quantum IR remains authoritative for program meaning and canonical quantum
//! resource identity.
//!
//! ZQN augments that semantic program with physical uncertainty/noise
//! information.
//!
//! ZQN must not turn the canonical Quantum IR into a hardware-specific noise
//! representation.
//!
//! ## Channel integration
//!
//! Mathematical channel representations remain owned by the ZQN channel
//! subsystem.
//!
//! Examples include:
//!
//! - Kraus operators;
//! - Choi matrices;
//! - superoperators;
//! - Pauli-transfer representations;
//! - stochastic maps;
//! - Lindblad generators;
//! - other supported representations.
//!
//! `noise` describes *when/where/why* noise semantics apply.
//!
//! `channel` describes the mathematical transformation itself.
//!
//! ## Fault integration
//!
//! Fault definitions remain owned by the ZQN fault subsystem.
//!
//! QEC and simulation may consume fault realizations derived from a noise
//! model, but the noise module must not duplicate a second universal fault
//! representation.
//!
//! ## Calibration integration
//!
//! Calibration is an external producer of physical parameter information.
//!
//! The intended direction is:
//!
//! ```text
//! calibration
//!     │
//!     ▼
//! noise model
//! ```
//!
//! This module must not contain:
//!
//! - vendor calibration formats;
//! - QPU credentials;
//! - calibration acquisition;
//! - mutable global calibration state;
//! - hardware-specific calibration APIs.
//!
//! ## Characterization integration
//!
//! Characterization determines or estimates physical noise parameters from
//! observations.
//!
//! The intended direction is:
//!
//! ```text
//! observations
//!     │
//!     ▼
//! characterization
//!     │
//!     ▼
//! noise specification/model
//! ```
//!
//! Noise does not own the experimental protocols that produce those
//! observations.
//!
//! ## Routing integration
//!
//! Routing may consume ZQN information such as:
//!
//! - estimated operation error;
//! - fidelity impact;
//! - crosstalk risk;
//! - correlation risk;
//! - transport error;
//! - resource-dependent noise.
//!
//! Routing remains responsible for deciding where logical resources are placed.
//!
//! The dependency direction remains:
//!
//! ```text
//! routing → ZQN noise information
//! ```
//!
//! rather than:
//!
//! ```text
//! ZQN → concrete routing implementation
//! ```
//!
//! ## Scheduling integration
//!
//! Scheduling may consume temporal ZQN information including:
//!
//! - idle noise;
//! - duration-dependent error;
//! - crosstalk;
//! - drift;
//! - calibration validity;
//! - time-dependent channels.
//!
//! ZQN describes the noise; scheduling decides the execution order and timing.
//!
//! ## QEC integration
//!
//! The intended long-term architecture is:
//!
//! ```text
//! ZQN
//!  │
//!  ├── noise models
//!  ├── channels
//!  ├── faults
//!  └── correlations
//!       │
//!       ▼
//! QEC integration adapter
//!       │
//!       ├── syndrome generation
//!       ├── decoding
//!       ├── correction
//!       └── logical-error analysis
//! ```
//!
//! QEC owns fault-tolerance algorithms and decoding. It must not maintain a
//! competing universal noise-model abstraction.
//!
//! ## Simulation integration
//!
//! Simulation consumes ZQN semantics.
//!
//! The direction is:
//!
//! ```text
//! ZQN semantics
//!      │
//!      ▼
//! simulation engine
//! ```
//!
//! This module must remain independent of any particular:
//!
//! - state-vector simulator;
//! - density-matrix simulator;
//! - stabilizer simulator;
//! - tensor-network simulator;
//! - trajectory engine;
//! - Monte Carlo implementation;
//! - analog simulator;
//! - hardware emulator.
//!
//! ## Hardware integration
//!
//! Hardware adapters provide abstract information such as:
//!
//! - target capabilities;
//! - calibration state;
//! - observed noise;
//! - resource characteristics.
//!
//! The direction is:
//!
//! ```text
//! hardware adapter
//!      │
//!      ├── capabilities
//!      ├── calibration
//!      └── observations
//!              │
//!              ▼
//!             ZQN
//! ```
//!
//! ZQN must never contain vendor-specific execution logic.
//!
//! There must be no architectural requirement for files such as:
//!
//! ```text
//! noise/ibm.rs
//! noise/ionq.rs
//! noise/rigetti.rs
//! noise/quantinuum.rs
//! ```
//!
//! Vendor integration belongs in hardware/provider layers.
//!
//! ## Benchmarking integration
//!
//! Benchmarking may consume:
//!
//! - noise models;
//! - noise observations;
//! - characterization results;
//! - calibration snapshots;
//! - error estimates;
//! - uncertainty information.
//!
//! ZQN must remain independent of concrete benchmark implementations.
//!
//! ## Runtime integration
//!
//! Runtime supplies execution context, including where applicable:
//!
//! - resource policies;
//! - cancellation;
//! - deterministic execution policy;
//! - target capabilities;
//! - calibration selection;
//! - timing context.
//!
//! Noise semantics are returned to the runtime for realization/execution.
//!
//! ZQN must not create hidden runtime state.
//!
//! ## Determinism
//!
//! This composition module contains:
//!
//! - no random-number generator;
//! - no global mutable state;
//! - no hidden execution clock;
//! - no process-global noise state.
//!
//! Child modules must follow the ZQN deterministic-execution contract.
//!
//! When deterministic execution is requested, stochastic realization must
//! derive from explicit caller-controlled execution context.
//!
//! Parallelization must not inherently alter deterministic semantics.
//!
//! ## Numerical safety
//!
//! This module does not choose a numerical representation.
//!
//! Child modules must reject invalid numerical states rather than silently
//! repairing them.
//!
//! In particular, implementations must not silently transform:
//!
//! ```text
//! NaN       → 0
//! infinity  → finite value
//! negative probability → absolute value
//! invalid parameter → arbitrary valid parameter
//! ```
//!
//! Any approximation must be explicit.
//!
//! ## Security
//!
//! A ZQN noise specification is data, not executable authority.
//!
//! Loading a model must not grant:
//!
//! - filesystem capabilities;
//! - network capabilities;
//! - process-execution capabilities;
//! - QPU credentials;
//! - arbitrary runtime privileges.
//!
//! Runtime capability enforcement remains outside this module.
//!
//! Untrusted models and serialized data must be processed under explicit
//! resource and validation policies.
//!
//! ## Serialization
//!
//! Serialization wire formats are owned by the ZQN I/O subsystem.
//!
//! This module must not rely on Rust's in-memory representation as an external
//! compatibility contract.
//!
//! External serialized noise models must be versioned and validated.
//!
//! ## API stability
//!
//! This module intentionally exposes child modules through explicit module
//! paths.
//!
//! It deliberately avoids broad wildcard re-exports such as:
//!
//! ```text
//! pub use model::*;
//! pub use application::*;
//! ```
//!
//! Broad re-exports create ambiguous ownership, increase coupling, and make
//! independent evolution of the child modules harder.
//!
//! Consumers should import a type from the module that owns its contract.
//!
//! For example:
//!
//! ```text
//! crate::quantum::zqn::noise::model::NoiseModel
//! crate::quantum::zqn::noise::application::NoiseApplication
//! ```
//!
//! ## Independent-file completion contract
//!
//! This file is intentionally independent from implementation details in its
//! child modules.
//!
//! A child module may evolve its internal types without requiring this file to
//! be edited, provided its module-level responsibility and public module path
//! remain intact.
//!
//! New noise functionality should normally be introduced by adding or
//! extending the appropriate child module rather than modifying this file.
//!
//! ## Rust requirements
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly-only features;
//! - no `unsafe` code.
//!
//! ============================================================================
//! Module declarations
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

/// Applies noise semantics to quantum operations, resources, execution
/// scopes, measurements, resets, timing intervals, and other supported
/// locations.
pub mod application;

/// Composes noise semantics while preserving their declared mathematical and
/// physical meaning.
pub mod composition;

/// Represents arbitrary spatial and resource correlations between noise
/// processes.
pub mod correlation;

/// Represents unwanted interactions between otherwise distinct quantum
/// resources or operations.
pub mod crosstalk;

/// Represents deterministic evolution of noise parameters or models over an
/// explicit temporal context.
pub mod drift;

/// Defines the canonical backend-independent noise-model abstraction.
pub mod model;

/// Represents noise processes with memory or history dependence.
pub mod non_markovian;

/// Defines declarative, backend-independent descriptions of noise.
pub mod specification;

/// Represents spatially dependent noise using externally supplied resource
/// topology/context.
pub mod spatial;

/// Represents explicitly time-dependent noise behavior.
pub mod temporal;

// ============================================================================
// Public API policy
// ============================================================================
//
// No wildcard re-exports are intentionally provided here.
//
// The owning module remains the API namespace:
//
//     quantum::zqn::noise::model
//     quantum::zqn::noise::application
//     quantum::zqn::noise::composition
//     quantum::zqn::noise::correlation
//     quantum::zqn::noise::crosstalk
//     quantum::zqn::noise::drift
//     quantum::zqn::noise::non_markovian
//     quantum::zqn::noise::specification
//     quantum::zqn::noise::spatial
//     quantum::zqn::noise::temporal
//
// This keeps this composition boundary independent from the implementation
// details and public-type evolution of child modules.
//
// ============================================================================
// Module-level tests
// ============================================================================
//
// No tests are placed here that construct child-module implementations or
// assume object-safety, concrete constructors, fixed resource counts, specific
// hardware, numerical representations, or particular implementation details.
//
// Child modules own their unit/property/differential/scaling tests.
//
// The absence of tests in this composition boundary is intentional: importing
// and exposing the module tree is itself compile-time validated by Rust.
// ============================================================================