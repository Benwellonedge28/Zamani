//! Zamani Quantum Memory Subsystem
//!
//! Production module boundary for quantum and hybrid quantum-classical memory
//! in Zamani.
//!
//! # Scope
//!
//! `quantum::memory` is the representation-neutral memory/resource substrate
//! used by Zamani's quantum execution stack.
//!
//! It provides module boundaries for:
//!
//! - logical quantum-memory resources;
//! - classical companion memory;
//! - quantum-state representations;
//! - memory allocation and budgeting;
//! - resource admission and limits;
//! - numerical and complex-value policy;
//! - memory layouts and safe indexing;
//! - state views, slicing and permutations;
//! - measurement, collapse and reset;
//! - snapshots, checkpoints and serialization;
//! - host/device/distributed coherence;
//! - CPU, SIMD and GPU memory abstractions;
//! - distributed quantum-state resources;
//! - provider-neutral QPU resource contracts;
//! - backend-native state handles;
//! - state migration and compaction;
//! - diagnostics, telemetry and cache infrastructure.
//!
//! # Architectural ownership
//!
//! This module is deliberately NOT the owner of:
//!
//! - Zamani source syntax;
//! - OpenQASM parsing;
//! - canonical quantum IR semantics;
//! - gate definitions;
//! - circuit optimization;
//! - routing algorithms;
//! - scheduling algorithms;
//! - QEC algorithms;
//! - QEC decoding;
//! - vendor SDKs;
//! - provider authentication;
//! - benchmark protocols;
//! - benchmark statistics;
//! - application-level quantum algorithms.
//!
//! Those responsibilities belong to their respective quantum subsystems.
//!
//! The intended high-level flow is:
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! quantum::frontend
//!      |
//!      v
//! quantum::ir
//!      |
//!      +-------------------------------+
//!      | canonical quantum semantics   |
//!      +-------------------------------+
//!          |       |       |       |
//!          v       v       v       v
//!     algorithms routing scheduling QEC
//!          |       |       |       |
//!          +-------+-------+-------+
//!                  |
//!                  v
//!             execution layer
//!                  |
//!          +-------+--------+
//!          |                |
//!          v                v
//! quantum::memory    quantum::hardware
//!          |                |
//!          +-------+--------+
//!                  |
//!                  v
//!               runtime
//! ```
//!
//! `quantum::benchmarking` consumes execution and memory telemetry. The memory
//! subsystem must never depend on benchmarking implementations.
//!
//! # Representation neutrality
//!
//! Memory is not synonymous with a dense CPU state vector.
//!
//! The current subsystem provides separate module boundaries for:
//!
//! - state-vector simulation;
//! - density-matrix simulation;
//! - stabilizer/tableau simulation;
//! - sparse-state simulation;
//! - tensor-network simulation;
//! - backend-native state;
//! - QPU resource state;
//! - CPU memory;
//! - SIMD facilities;
//! - GPU/device memory;
//! - distributed memory.
//!
//! This permits Zamani to support both classical simulation and real quantum
//! hardware without forcing hardware execution into a simulator-only model.
//!
//! In particular, a real QPU generally does not expose a byte-addressable
//! quantum state vector to the host. `qpu` and `backend_state` therefore model
//! provider-neutral resource and opaque-state contracts rather than pretending
//! that every device is conventional RAM.
//!
//! # Hardware neutrality
//!
//! Hardware-specific implementations remain outside this module.
//!
//! The memory boundary is intentionally compatible with:
//!
//! - superconducting QPUs;
//! - trapped-ion QPUs;
//! - neutral-atom systems;
//! - photonic systems;
//! - spin-based systems;
//! - semiconductor qubits;
//! - annealing/adiabatic systems;
//! - qudit and oscillator systems;
//! - hybrid quantum-classical systems;
//! - local simulators;
//! - remote simulators;
//! - cloud QPUs;
//! - on-premises QPUs;
//! - custom/future quantum architectures.
//!
//! No provider SDK, credential type, vendor-specific topology type, or vendor
//! execution protocol belongs in this module boundary.
//!
//! Provider-specific adapters are owned by `quantum::hardware` and connect to
//! the provider-neutral contracts exposed here.
//!
//! # Resource safety
//!
//! Quantum state storage can grow exponentially:
//!
//! ```text
//! dense state vector:
//!     amplitudes = 2^n
//!
//! dense density matrix:
//!     complex elements = 4^n
//! ```
//!
//! Consequently, potentially large operations must conceptually follow:
//!
//! ```text
//! estimate
//!    |
//!    v
//! validate limits
//!    |
//!    v
//! reserve budget
//!    |
//!    v
//! allocate
//!    |
//!    v
//! initialize
//!    |
//!    v
//! commit
//! ```
//!
//! The individual stages are owned by:
//!
//! - `limits`;
//! - `budget`;
//! - `reservation`;
//! - `allocator`;
//! - `pool`.
//!
//! Representation implementations must not bypass these contracts for managed
//! allocations.
//!
//! # Safety policy
//!
//! This entire subsystem is safe Rust.
//!
//! `unsafe` is denied at the module boundary. This is intentional: memory
//! management is security- and correctness-critical, so an implementation
//! cannot silently introduce unsafe code into this subsystem.
//!
//! Raw pointers must not form part of the public API.
//!
//! Accelerator and hardware integrations must instead expose safe abstractions
//! such as handles, buffers, allocations, streams, events and opaque provider
//! resources.
//!
//! # Numerical correctness
//!
//! `numeric` and `complex` provide the numerical boundary used by quantum-state
//! implementations.
//!
//! Numerical policy must not be duplicated in individual representations.
//! In particular, tolerance values, finite-value validation and checked
//! numerical operations belong at the shared numerical boundary.
//!
//! # Identity ownership
//!
//! Program-level quantum identity remains owned by `quantum::ir`.
//!
//! Memory must not create a second semantic definition of a circuit, operation,
//! gate or program-level qubit.
//!
//! Memory-specific allocation/resource identities belong in `types`.
//!
//! Where another subsystem already provides the canonical semantic identity,
//! memory integrations must adapt to that identity rather than inventing a
//! competing one.
//!
//! # Layout and routing
//!
//! `quantum::routing` owns the routing algorithm and logical-to-physical
//! placement policy.
//!
//! `layout` and `permutation` provide the storage-side representation needed
//! to execute that placement.
//!
//! The intended boundary is:
//!
//! ```text
//! routing
//!    |
//!    v
//! logical -> physical mapping
//!    |
//!    v
//! memory::permutation
//!    |
//!    v
//! memory::layout
//!    |
//!    v
//! state representation
//! ```
//!
//! Memory therefore does not implement SABRE, search, routing heuristics,
//! topology optimization or placement algorithms.
//!
//! # Scheduling
//!
//! `quantum::scheduling` owns execution ordering and timing.
//!
//! Memory exposes resource information needed by scheduling, including memory
//! availability, allocation requirements, coherence state and synchronization
//! dependencies.
//!
//! Memory does not own scheduling policy.
//!
//! # QEC
//!
//! `quantum::error_correction` owns QEC algorithms and decoders.
//!
//! Memory provides the substrate those algorithms may consume:
//!
//! - physical-qubit resources;
//! - logical-qubit resources;
//! - stabilizer state;
//! - syndrome/classical memory;
//! - measurement results;
//! - Pauli-frame-related state;
//! - checkpoints.
//!
//! This separation permits QEC implementations to use an efficient stabilizer
//! representation rather than forcing an exponentially large dense state.
//!
//! # Runtime
//!
//! The runtime may compose operations exposed by this subsystem in a lifecycle
//! such as:
//!
//! ```text
//! allocate
//!    -> initialize
//!    -> execute
//!    -> measure
//!    -> collapse/reset when required
//!    -> snapshot/checkpoint when requested
//!    -> synchronize/migrate when required
//!    -> release
//! ```
//!
//! Runtime policy remains outside the memory subsystem.
//!
//! # Persistence
//!
//! `snapshot`, `checkpoint` and `serialization` form the persistence boundary.
//!
//! Persisted state must retain enough information to prevent accidental
//! interpretation under incompatible:
//!
//! - representation;
//! - precision;
//! - layout;
//! - qubit ordering;
//! - serialization version;
//! - storage format;
//! - execution/resource assumptions.
//!
//! Restoration must validate the persisted metadata before committing restored
//! state to live memory.
//!
//! # Coherence and concurrency
//!
//! `coherence` and `synchronization` define consistency across:
//!
//! - host memory;
//! - pinned host memory;
//! - accelerator memory;
//! - unified memory;
//! - distributed partitions;
//! - remote resources;
//! - provider-managed resources.
//!
//! `cache` may optimize access but must never become an independent source of
//! quantum-state truth.
//!
//! # Telemetry
//!
//! `diagnostics` and `telemetry` expose memory observations to higher layers.
//!
//! Typical measurements include:
//!
//! - allocated bytes;
//! - reserved bytes;
//! - peak bytes;
//! - allocation counts;
//! - allocation failures;
//! - representation;
//! - state size;
//! - migration counts;
//! - migration bytes;
//! - synchronization activity;
//! - cache activity;
//! - device memory;
//! - distributed memory;
//! - checkpoint size.
//!
//! The dependency direction remains:
//!
//! ```text
//! memory --telemetry--> benchmarking
//! ```
//!
//! and never:
//!
//! ```text
//! memory --> benchmarking implementation
//! ```
//!
//! # Stable module boundary
//!
//! The module declarations below intentionally correspond to the files that
//! currently exist under `src/quantum/memory/`.
//!
//! Do not add declarations for speculative future files here. A new subsystem
//! file must first exist with its own completed API contract and tests before it
//! becomes part of this composition boundary.
//!
//! # Module dependency layers
//!
//! The declarations are grouped by architectural responsibility:
//!
//! ```text
//!  1. foundational contracts
//!       types / errors / numeric / complex / representation
//!
//!  2. indexing and resource admission
//!       limits / layout / indexing
//!
//!  3. allocation
//!       address / allocator / budget / reservation / pool
//!
//!  4. logical memory
//!       qubit / register / classical / lifetime
//!
//!  5. state
//!       state / state_vector / density_matrix / stabilizer / sparse
//!       tensor_network / backend_state
//!
//!  6. transformations
//!       view / permutation / slice / tensor / copy_on_write
//!
//!  7. quantum state operations
//!       measurement / collapse / reset
//!
//!  8. persistence
//!       serialization / snapshot / checkpoint
//!
//!  9. coherence
//!       coherence / synchronization / cache
//!
//! 10. execution resources
//!       cpu / simd / gpu / distributed / qpu
//!
//! 11. lifecycle and observability
//!       migration / compaction / diagnostics / telemetry
//! ```
//!
//! Rust does not require declarations to appear in dependency order, but this
//! ordering makes the intended architecture explicit for maintainers.
//!
//! # Child-module contract
//!
//! Every child module in this namespace must obey the following rules:
//!
//! 1. Remain safe Rust.
//! 2. Do not introduce `unsafe`.
//! 3. Do not expose raw pointers.
//! 4. Use canonical quantum identities from `quantum::ir` where applicable.
//! 5. Use `memory::types` for memory-resource quantities and identities.
//! 6. Use `memory::errors` for memory-domain failures.
//! 7. Validate untrusted or potentially unbounded resource requests.
//! 8. Use checked arithmetic for allocation-size calculations.
//! 9. Respect `limits`, `budget` and `reservation` policies.
//! 10. Never silently perform an unbounded exponential allocation.
//! 11. Never silently truncate quantum-state data.
//! 12. Never silently change state representation when semantic guarantees
//!     could change.
//! 13. Preserve logical qubit identity across storage transformations.
//! 14. Preserve declared layout and qubit-order semantics.
//! 15. Keep provider-specific implementations outside provider-neutral APIs.
//! 16. Do not perform provider I/O from the generic memory core.
//! 17. Do not store credentials or authentication material in memory state.
//! 18. Do not introduce global mutable quantum state.
//! 19. Do not print diagnostics directly to stdout or stderr.
//! 20. Use explicit RNG ownership/injection for stochastic operations.
//! 21. Keep deterministic operations deterministic for identical inputs and
//!     configuration.
//! 22. Make persistence versions explicit.
//! 23. Validate persisted data before committing it to live state.
//! 24. Preserve transactional semantics for allocation and migration.
//! 25. Keep benchmarking as a consumer of telemetry, never as a dependency.
//!
//! # Testing boundary
//!
//! The `tests` module is compiled only for tests and is responsible for
//! subsystem-level contract verification.
//!
//! Representation-specific behavior should remain tested in its corresponding
//! test module or implementation tests.
//!
//! The production module boundary must not depend on test-only infrastructure.
//!
//! # Rust compatibility
//!
//! This subsystem targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly-only feature is required by this module boundary.
//!
//! # Public API policy
//!
//! The module namespaces are public because the runtime, simulator, hardware
//! adapters, QEC subsystem, compiler integration and tooling may need
//! specialized capabilities.
//!
//! However, callers should prefer stable public types/contracts exposed by the
//! individual modules instead of reaching into implementation-private details.
//!
//! This file intentionally does not perform broad `pub use *` re-exports.
//! Wildcard re-exports make future additions prone to name collisions and make
//! the root namespace unstable. Consumers should use explicit paths such as:
//!
//! ```text
//! quantum::memory::state::QuantumState
//! quantum::memory::state_vector::StateVector
//! quantum::memory::limits::MemoryLimits
//! quantum::memory::qpu::...
//! ```
//!
//! This keeps ownership and API provenance unambiguous.
//!
//! # Future extension
//!
//! New state representations, accelerator providers and hardware resource
//! models should implement existing provider-neutral contracts wherever
//! possible.
//!
//! Adding a new provider must not require adding a vendor-specific enum to the
//! generic memory layer merely to make that provider compile.
//!
//! The preferred extension model is:
//!
//! ```text
//! new provider
//!      |
//!      v
//! quantum::hardware adapter
//!      |
//!      v
//! memory provider-neutral contract
//!      |
//!      +--> allocator/resource handle
//!      +--> state/backend handle
//!      +--> synchronization/coherence
//!      +--> classical results
//! ```
//!
//! This permits Zamani to add future QPUs and quantum architectures without
//! redesigning the memory core.
//!
//! # Security and failure model
//!
//! Memory exhaustion, malformed serialized state, invalid dimensions,
//! unsupported representations, unavailable hardware resources and failed
//! synchronization are expected runtime failures, not reasons for undefined
//! behavior.
//!
//! They must be represented through the subsystem's error contracts.
//!
//! A production caller must therefore be able to distinguish at least:
//!
//! ```text
//! invalid request
//! resource unavailable
//! resource limit exceeded
//! budget exceeded
//! unsupported capability
//! invalid state
//! persistence failure
//! synchronization failure
//! backend/provider failure
//! ```
//!
//! No memory implementation may convert such conditions into silent corruption
//! or unchecked allocation behavior.
//!
//! # Final composition boundary
//!
//! This file is deliberately a composition root, not a second implementation
//! of quantum memory.
//!
//! It defines:
//!
//! - what belongs to `quantum::memory`;
//! - which modules are part of the stable subsystem;
//! - the safety policy;
//! - integration boundaries;
//! - dependency direction;
//! - provider-neutrality requirements;
//! - test isolation.
//!
//! The actual algorithms and data structures remain in their dedicated modules.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// -----------------------------------------------------------------------------
// Foundational contracts
// -----------------------------------------------------------------------------

pub mod types;
pub mod errors;
pub mod numeric;
pub mod complex;
pub mod representation;

// -----------------------------------------------------------------------------
// Resource sizing, layout and safe indexing
// -----------------------------------------------------------------------------

pub mod limits;
pub mod layout;
pub mod indexing;

// -----------------------------------------------------------------------------
// Allocation and resource management
// -----------------------------------------------------------------------------

pub mod address;
pub mod allocator;
pub mod budget;
pub mod reservation;
pub mod pool;

// -----------------------------------------------------------------------------
// Logical quantum/classical memory
// -----------------------------------------------------------------------------

pub mod qubit;
pub mod register;
pub mod classical;
pub mod lifetime;

// -----------------------------------------------------------------------------
// Quantum-state contract and representations
// -----------------------------------------------------------------------------

pub mod state;
pub mod state_vector;
pub mod density_matrix;
pub mod stabilizer;
pub mod sparse;
pub mod tensor_network;
pub mod backend_state;

// -----------------------------------------------------------------------------
// Views and state transformations
// -----------------------------------------------------------------------------

pub mod view;
pub mod permutation;
pub mod slice;
pub mod tensor;
pub mod copy_on_write;

// -----------------------------------------------------------------------------
// Measurement and state lifecycle operations
// -----------------------------------------------------------------------------

pub mod measurement;
pub mod collapse;
pub mod reset;

// -----------------------------------------------------------------------------
// Persistence
// -----------------------------------------------------------------------------

pub mod serialization;
pub mod snapshot;
pub mod checkpoint;

// -----------------------------------------------------------------------------
// Coherence, synchronization and caching
// -----------------------------------------------------------------------------

pub mod coherence;
pub mod synchronization;
pub mod cache;

// -----------------------------------------------------------------------------
// Execution-resource and accelerator boundaries
// -----------------------------------------------------------------------------

pub mod cpu;
pub mod simd;
pub mod gpu;
pub mod distributed;
pub mod qpu;

// -----------------------------------------------------------------------------
// Lifecycle, optimization and observability
// -----------------------------------------------------------------------------

pub mod migration;
pub mod compaction;
pub mod diagnostics;
pub mod telemetry;

// -----------------------------------------------------------------------------
// Integration tests
// -----------------------------------------------------------------------------
//
// Test-only code is deliberately excluded from normal library builds.
// `tests/mod.rs` owns the subsystem-level test coordinator and must remain
// independent from production implementation details.

#[cfg(test)]
mod tests;