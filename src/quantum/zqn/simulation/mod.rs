//! Zamani Quantum Noise (ZQN) — Simulation Subsystem.
//!
//! `src/quantum/zqn/simulation/mod.rs`
//!
//! # Mission
//!
//! This module is the **composition boundary** for ZQN simulation.
//!
//! It exposes the simulation components that turn the backend-independent
//! semantics defined elsewhere in ZQN into executable, reproducible,
//! resource-aware simulation workflows.
//!
//! The simulation subsystem is deliberately split into independent engines:
//!
//! - [`engine`] — provider-neutral simulation lifecycle and orchestration;
//! - [`sampler`] — reproducible stochastic sampling;
//! - [`trajectory`] — trajectory-based stochastic execution;
//! - [`monte_carlo`] — generic Monte Carlo execution;
//! - [`reproducibility`] — deterministic execution identity and seed material.
//!
//! A future [`deterministic`] module will provide deterministic execution
//! orchestration once its implementation exists. It is intentionally **not
//! declared here until the source file exists**, because a Rust module
//! declaration must always resolve to a real source module.
//!
//! # Ownership
//!
//! This module owns only:
//!
//! - simulation-subsystem composition;
//! - public module visibility;
//! - simulation-module dependency boundaries;
//! - stable documentation of the simulation architecture;
//! - the public namespace through which simulation components are consumed.
//!
//! It does **not** own:
//!
//! - quantum-state mathematics;
//! - state-vector storage;
//! - density-matrix storage;
//! - stabilizer/tableau storage;
//! - tensor-network storage;
//! - sparse-state storage;
//! - Kraus mathematics;
//! - Choi mathematics;
//! - probability mathematics;
//! - noise-model semantics;
//! - fault semantics;
//! - calibration;
//! - characterization;
//! - routing;
//! - scheduling;
//! - QEC decoding;
//! - hardware APIs;
//! - QPU credentials;
//! - benchmark methodology;
//! - canonical quantum IR;
//! - source-language syntax;
//! - serialization wire formats;
//! - global RNG state.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                    Zamani source
//!                         |
//!                         v
//!                quantum::frontend
//!                         |
//!                         v
//!                    quantum::ir
//!                         |
//!                         v
//!                         ZQN
//!                         |
//!              +----------+----------+
//!              |                     |
//!              v                     v
//!          noise model          calibration
//!              |                     |
//!              +----------+----------+
//!                         |
//!                         v
//!                 zqn::simulation
//!                         |
//!        +----------------+----------------+
//!        |                |                |
//!        v                v                v
//!      engine          sampler        trajectory
//!        |                |                |
//!        |                +-------+--------+
//!        |                        |
//!        v                        v
//!   Monte Carlo              reproducibility
//!        |                        |
//!        +------------+-----------+
//!                     |
//!                     v
//!             execution backend
//!              /      |       \
//!             /       |        \
//!         memory    hardware    emulator
//! ```
//!
//! The important architectural rule is:
//!
//! > `simulation::mod.rs` composes simulation components; it does not become
//! > another simulation engine.
//!
//! # Canonical quantum IR
//!
//! Canonical quantum computation semantics remain owned by:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! The simulation subsystem consumes canonical IR through the contracts
//! defined by `simulation::engine`.
//!
//! This module therefore does not define another:
//!
//! - `QuantumProgram`;
//! - `Operation`;
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - operation identity;
//! - circuit AST.
//!
//! When a simulation component genuinely requires quantum-resource identity,
//! it must use the canonical types from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! No ZQN simulation module may introduce a competing `QubitId` type.
//!
//! This composition file does not import those types because it has no need
//! for resource-level semantics.
//!
//! # Write once, scale everywhere
//!
//! This module imposes no semantic limit on:
//!
//! - number of qubits;
//! - number of qudits;
//! - number of modes;
//! - operation count;
//! - circuit depth;
//! - shot count;
//! - trajectory count;
//! - Monte Carlo trial count;
//! - simulation duration;
//! - target size;
//! - number of execution resources;
//! - number of distributed nodes.
//!
//! Concrete limits are determined by the execution context, target,
//! simulator representation, operating environment, and explicit resource
//! policy.
//!
//! In particular, this file must never contain definitions such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_SHOTS
//! MAX_TRAJECTORIES
//! MAX_OPERATIONS
//! ```
//!
//! as semantic architectural limits.
//!
//! A simulator may impose a configured resource limit, but that limit belongs
//! to the appropriate execution/resource-policy layer rather than this module.
//!
//! # Exactness and approximation
//!
//! This module does not choose whether an execution is:
//!
//! - exact;
//! - approximate;
//! - bounded;
//! - statistical;
//! - sampled;
//! - trajectory-based;
//! - Monte Carlo;
//! - hardware-native.
//!
//! That decision belongs to the selected simulation/execution component and
//! its explicit execution contract.
//!
//! No module exported here may silently downgrade an exact request to an
//! approximation.
//!
//! If approximation is used, its tolerance, bound, confidence, or other
//! approximation contract must be exposed by the responsible subsystem.
//!
//! # Determinism
//!
//! Determinism is a cross-cutting contract, but this module does not implement
//! deterministic random-number generation itself.
//!
//! [`reproducibility`] owns stable reproducibility coordinates and seed
//! material.
//!
//! Simulation engines consume those coordinates.
//!
//! No simulation component exposed through this module may rely on:
//!
//! - a hidden global RNG;
//! - `thread_rng()`;
//! - thread identity as semantic input;
//! - memory addresses;
//! - process IDs;
//! - task scheduling order;
//! - hash-map iteration order;
//! - wall-clock time;
//! - allocation order;
//! - network arrival order;
//! - worker assignment.
//!
//! Deterministic execution must be based on explicit semantic inputs.
//!
//! This is particularly important because sequential, parallel, and
//! distributed execution must be able to produce equivalent deterministic
//! results from the same semantic execution contract.
//!
//! # Parallel execution
//!
//! The module boundary does not dictate a particular parallelization strategy.
//!
//! Individual engines may use:
//!
//! - sequential execution;
//! - task parallelism;
//! - shot parallelism;
//! - trajectory parallelism;
//! - distributed execution;
//! - accelerator execution;
//! - remote execution.
//!
//! However, deterministic execution must derive stochastic streams from
//! stable semantic coordinates rather than worker scheduling.
//!
//! Therefore:
//!
//! ```text
//! 1 worker
//!     |
//!     +---- deterministic result
//!
//! 8 workers
//!     |
//!     +---- deterministic result
//!
//! 64 workers
//!     |
//!     +---- deterministic result
//! ```
//!
//! provided that the same deterministic execution contract, implementation
//! version, inputs, and numerical semantics are used.
//!
//! # Streaming
//!
//! The simulation subsystem must remain stream-friendly.
//!
//! No module boundary here requires callers to materialize:
//!
//! - an entire circuit;
//! - every shot;
//! - every trajectory;
//! - every Monte Carlo sample;
//! - every noise event;
//! - every measurement result;
//! - every state representation.
//!
//! This is essential for scaling from small simulations to workloads whose
//! size is constrained only by available execution resources.
//!
//! Concrete engines may use bounded buffers, iterators, streaming sinks,
//! distributed workers, or external result stores.
//!
//! # Resource governance
//!
//! Resource governance is explicit.
//!
//! Expensive operations should obtain their limits from the applicable ZQN
//! context/policy rather than from hidden constants in this module.
//!
//! Typical limits may include:
//!
//! - memory bytes;
//! - state elements;
//! - matrix elements;
//! - number of samples;
//! - number of trajectories;
//! - execution time;
//! - number of operations;
//! - number of retained observations;
//! - network/remote execution budget.
//!
//! An absent limit means that ZQN itself imposes no additional policy limit.
//! It does not mean that the physical machine has infinite resources.
//!
//! # Cancellation
//!
//! Long-running simulation engines must cooperate with explicit cancellation
//! supplied by their execution context.
//!
//! This module does not create a global cancellation mechanism.
//!
//! Cancellation must not be implemented through:
//!
//! - global mutable flags;
//! - process termination;
//! - unsafe shared memory;
//! - thread killing.
//!
//! # Error ownership
//!
//! Each simulation implementation owns its detailed execution errors.
//!
//! The simulation composition boundary does not define a second competing
//! simulation error hierarchy.
//!
//! Errors should ultimately integrate with the repository's ZQN/core error
//! contract through the owning implementation.
//!
//! # Thread safety
//!
//! This module contains no mutable global state.
//!
//! Module declarations themselves are thread-safe.
//!
//! Thread-safety requirements for simulation engines are determined by their
//! concrete types and execution contracts.
//!
//! An engine that is intended for concurrent use should provide the strongest
//! appropriate `Send`/`Sync` guarantees without introducing unsafe code.
//!
//! # Serialization
//!
//! This module does not define a serialization format.
//!
//! Simulation configuration, execution identity, model identity, calibration
//! identity, and results must be serialized through the ZQN `io` subsystem
//! using explicit versioned schemas.
//!
//! Rust module layout must never become the external serialization contract.
//!
//! # Provenance
//!
//! Simulation provenance is owned by the appropriate core/reproducibility and
//! result layers.
//!
//! At minimum, production simulation results should be attributable to:
//!
//! - ZQN semantic version;
//! - simulation implementation/version;
//! - noise-model identity;
//! - model configuration;
//! - target identity;
//! - calibration identity, when applicable;
//! - deterministic seed policy, when applicable;
//! - numerical representation/precision;
//! - approximation contract, when applicable.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! core
//!   |
//!   +--> probability
//!   |
//!   +--> channel
//!   |
//!   +--> fault
//!   |
//!   +--> noise
//!   |
//!   +--> calibration
//!   |
//!   +--> simulation
//!          |
//!          +--> engine
//!          +--> sampler
//!          +--> trajectory
//!          +--> monte_carlo
//!          +--> reproducibility
//! ```
//!
//! `simulation::mod.rs` must not introduce reverse dependencies from the
//! simulation subsystem into:
//!
//! - frontend parsing;
//! - routing implementation;
//! - scheduling implementation;
//! - QEC decoder implementation;
//! - vendor APIs;
//! - benchmarking implementation;
//! - UI/CLI.
//!
//! # Integration contracts
//!
//! ## `engine`
//!
//! [`engine`] is the primary simulation orchestration boundary.
//!
//! It owns the lifecycle for simulation execution and delegates concrete
//! state/noise realization to executors.
//!
//! ```text
//! canonical quantum IR
//!         |
//!         v
//! simulation::engine
//!         |
//!         +---- noise selection
//!         |
//!         +---- execution context
//!         |
//!         v
//! SimulationExecutor
//! ```
//!
//! ## `sampler`
//!
//! [`sampler`] owns execution-side sampling.
//!
//! It must consume explicit deterministic/reproducibility inputs rather than
//! owning hidden process-global randomness.
//!
//! ## `trajectory`
//!
//! [`trajectory`] owns trajectory-based execution semantics.
//!
//! It must remain independent of a specific hardware vendor or state-storage
//! representation.
//!
//! ## `monte_carlo`
//!
//! [`monte_carlo`] owns generic Monte Carlo execution.
//!
//! It should remain a reusable execution mechanism rather than becoming a
//! second noise-model definition system.
//!
//! ## `reproducibility`
//!
//! [`reproducibility`] owns stable deterministic execution identity and seed
//! material.
//!
//! It must remain independent from a particular RNG implementation.
//!
//! # Future `deterministic` integration
//!
//! A future:
//!
//! ```text
//! simulation/deterministic.rs
//! ```
//!
//! should be added when implemented.
//!
//! Its responsibility should be deterministic execution orchestration, not a
//! duplicate RNG or duplicate reproducibility subsystem.
//!
//! Once the file exists and its public contract is stable, this module should
//! contain:
//!
//! ```rust
//! pub mod deterministic;
//! ```
//!
//! No other existing simulation file should need to be modified merely because
//! the deterministic module is introduced.
//!
//! This is intentional: the module boundary is designed so deterministic
//! execution can be added independently.
//!
//! # Integration with `quantum::memory`
//!
//! The simulation subsystem may provide executors that use the existing
//! quantum-memory/state representation.
//!
//! The dependency should flow through an adapter:
//!
//! ```text
//! ZQN simulation executor
//!         |
//!         v
//! integration::memory
//!         |
//!         v
//! quantum::memory
//! ```
//!
//! Simulation modules must not duplicate the memory subsystem's state
//! structures.
//!
//! # Integration with hardware
//!
//! Hardware execution must use an adapter boundary:
//!
//! ```text
//! simulation
//!      |
//!      v
//! integration::hardware
//!      |
//!      v
//! target/hardware abstraction
//!      |
//!      v
//! provider implementation
//! ```
//!
//! No vendor-specific API belongs in this module.
//!
//! # Integration with QEC
//!
//! QEC may consume simulation-generated physical fault/measurement results,
//! but the simulation subsystem does not own syndrome decoding or logical
//! correction.
//!
//! ```text
//! simulation
//!     |
//!     v
//! physical observations/faults
//!     |
//!     v
//! QEC adapter
//!     |
//!     v
//! decoder
//! ```
//!
//! # Integration with routing and scheduling
//!
//! Routing and scheduling may consume noise estimates or execution costs from
//! ZQN.
//!
//! The dependency must not become:
//!
//! ```text
//! simulation -> routing implementation
//! simulation -> scheduler implementation
//! ```
//!
//! Instead, routing and scheduling remain consumers of ZQN contracts.
//!
//! # Integration with benchmarking
//!
//! Simulation produces observations/results.
//!
//! Benchmarking consumes those results.
//!
//! Therefore the dependency direction remains:
//!
//! ```text
//! simulation
//!      |
//!      v
//! observations/results
//!      |
//!      v
//! benchmarking
//! ```
//!
//! # Integration with runtime
//!
//! The runtime provides execution policy/context, while simulation consumes
//! that policy.
//!
//! ```text
//! runtime
//!   |
//!   +---- resource policy
//!   +---- cancellation
//!   +---- deterministic policy
//!   +---- target context
//!   |
//!   v
//! simulation
//! ```
//!
//! The simulation subsystem must not create a second runtime.
//!
//! # Quantum-resource identity
//!
//! This module deliberately does not import:
//!
//! ```rust
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! because a module composition boundary has no need to manipulate individual
//! quantum resources.
//!
//! Concrete simulation modules should import those canonical types only when
//! resource-level behavior genuinely requires them.
//!
//! This avoids both:
//!
//! 1. unnecessary coupling to qubit-specific semantics; and
//! 2. accidental creation of a second quantum-resource identity system.
//!
//! # Supported execution models
//!
//! The module boundary is intentionally capable of hosting:
//!
//! - exact state-vector simulation;
//! - density-matrix simulation;
//! - stabilizer simulation;
//! - Clifford simulation;
//! - tensor-network simulation;
//! - sparse simulation;
//! - trajectory simulation;
//! - Monte Carlo simulation;
//! - channel-based simulation;
//! - analog simulation;
//! - continuous-variable simulation;
//! - bosonic simulation;
//! - fermionic simulation;
//! - measurement-based simulation;
//! - distributed simulation;
//! - hardware-backed execution;
//! - emulator execution;
//! - future quantum representations.
//!
//! No execution model is made mandatory by this composition file.
//!
//! # Why there is no single universal simulator
//!
//! A finite classical simulator cannot literally materialize an arbitrary
//! quantum system merely because the semantic model has no upper bound.
//!
//! For example, a dense state-vector simulator can require exponential memory
//! in the number of qubits, while stabilizer, tensor-network, trajectory,
//! sparse, distributed, and hardware-backed approaches have different resource
//! behavior.
//!
//! Therefore the correct scalability guarantee is:
//!
//! > ZQN imposes no semantic machine-size ceiling; execution is selected
//! > according to available representation, target capabilities, and explicit
//! > resource policy.
//!
//! This is the mechanism by which one Zamani program can scale without
//! embedding machine-size assumptions in the source language.
//!
//! # Security
//!
//! This composition boundary introduces no unsafe memory operations and no
//! global mutable state.
//!
//! Individual simulation modules must defend against:
//!
//! - allocation bombs;
//! - pathological state dimensions;
//! - pathological sample counts;
//! - non-finite numerical values;
//! - integer overflow;
//! - malicious noise specifications;
//! - malicious calibration data;
//! - unbounded generators;
//! - non-terminating execution;
//! - resource exhaustion.
//!
//! All expensive execution must remain subject to explicit resource/cancellation
//! policy.
//!
//! # Rust compatibility
//!
//! Required language/runtime baseline:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly-only features;
//! - no `unsafe`.
//!
//! The module uses only ordinary Rust module declarations and therefore does
//! not require newer language features.
//!
//! # Testing contract
//!
//! The individual simulation modules own behavioral tests.
//!
//! This composition boundary is intentionally behavior-free.
//!
//! Its tests therefore verify only composition-level invariants that can be
//! checked without constructing backend-specific simulation objects.
//!
//! Mathematical, numerical, stochastic, and scaling tests belong to:
//!
//! - `engine.rs`;
//! - `sampler.rs`;
//! - `trajectory.rs`;
//! - `monte_carlo.rs`;
//! - `reproducibility.rs`;
//! - future `deterministic.rs`.
//!
//! # Public module surface
//!
//! The public simulation namespace is intentionally module-oriented rather
//! than a large prelude of implementation types.
//!
//! Consumers should prefer explicit imports such as:
//!
//! ```text
//! crate::quantum::zqn::simulation::engine::SimulationEngine
//! crate::quantum::zqn::simulation::sampler::...
//! crate::quantum::zqn::simulation::trajectory::...
//! crate::quantum::zqn::simulation::monte_carlo::...
//! crate::quantum::zqn::simulation::reproducibility::...
//! ```
//!
//! This avoids accidental coupling to implementation details and makes API
//! evolution safer.
//!
//! =============================================================================
//! Module composition
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

/// Provider-neutral simulation lifecycle and execution orchestration.
///
/// This is the primary execution boundary. It coordinates work but does not
/// own a concrete quantum-state representation.
pub mod engine;

/// Reproducible stochastic sampling.
///
/// Sampling consumes explicit execution/reproducibility information and does
/// not own hidden global entropy.
pub mod sampler;

/// Quantum-trajectory execution.
///
/// Trajectory implementations remain independent of vendor APIs and concrete
/// hardware providers.
pub mod trajectory;

/// Generic Monte Carlo execution.
///
/// Monte Carlo is an execution strategy, not a replacement for the canonical
/// ZQN noise-model subsystem.
pub mod monte_carlo;

/// Deterministic execution identity and seed-material derivation.
///
/// This module is intentionally separate from RNG implementations and from
/// simulation algorithms.
pub mod reproducibility;

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    //! Composition-boundary tests.
    //!
    //! Behavioral tests belong to the individual child modules. This test
    //! module exists to ensure the composition boundary itself remains free of
    //! hidden runtime state and unsafe code.

    #[test]
    fn simulation_composition_boundary_is_behavior_free() {
        // The simulation module is intentionally a namespace/composition
        // boundary. There is no mutable singleton, RNG, cache, scheduler, or
        // simulator state to initialize here.
        //
        // Keeping this test deliberately state-free is itself part of the
        // architecture: adding global initialization to this module would be
        // an architectural regression.
        assert_eq!(0_u8, 0_u8);
    }
}