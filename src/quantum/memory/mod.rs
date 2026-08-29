//! Zamani Quantum Memory Subsystem
//!
//! Production module boundary for quantum and hybrid quantum-classical memory
//! in Zamani.
//!
//! # Purpose
//!
//! `quantum::memory` provides the representation-independent memory substrate
//! used by Zamani's quantum execution stack.
//!
//! It is responsible for:
//!
//! - logical quantum-memory ownership;
//! - classical companion memory;
//! - quantum-state storage;
//! - state-representation abstraction;
//! - memory allocation and resource budgeting;
//! - memory limits and admission control;
//! - memory layouts and indexing;
//! - state views and transformations;
//! - measurement, collapse and reset support;
//! - snapshots and checkpoints;
//! - serialization;
//! - host/device coherence;
//! - synchronization;
//! - CPU/SIMD/GPU memory facilities;
//! - distributed quantum-state memory;
//! - QPU/quantum-hardware memory/resource contracts;
//! - state migration;
//! - compaction;
//! - diagnostics;
//! - telemetry;
//! - cache infrastructure.
//!
//! It does **not** own:
//!
//! - Zamani source-language syntax;
//! - OpenQASM parsing;
//! - canonical quantum IR semantics;
//! - circuit optimization;
//! - routing algorithms;
//! - scheduling algorithms;
//! - QEC algorithms;
//! - vendor-specific hardware implementations;
//! - benchmark protocols;
//! - benchmark statistics;
//! - application-level quantum algorithms.
//!
//! Those responsibilities remain in their owning quantum subsystems.
//!
//! # Architectural position
//!
//! The intended dependency direction is:
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
//!      +--------------------------------------------------+
//!      |             canonical quantum semantics          |
//!      +--------------------------------------------------+
//!                         |
//!          +--------------+---------------+
//!          |              |               |
//!          v              v               v
//!      algorithms    optimization      error_correction
//!          |              |               |
//!          +--------------+---------------+
//!                         |
//!                         v
//!                routing / scheduling
//!                         |
//!              +----------+----------+
//!              |                     |
//!              v                     v
//!         quantum::memory       quantum::hardware
//!              |                     |
//!              +----------+----------+
//!                         |
//!                         v
//!                     runtime
//!
//! quantum::benchmarking consumes the above layers and must not become a
//! dependency of the memory subsystem.
//! ```
//!
//! # Canonical ownership rules
//!
//! ## Quantum IR
//!
//! `quantum::ir` remains authoritative for program-level quantum identity and
//! semantics.
//!
//! In particular, memory must not create competing semantic definitions of:
//!
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - `ClassicalBitId`;
//! - circuit identities;
//! - operation identities;
//! - gate semantics;
//! - measurement semantics at the IR level.
//!
//! Memory-specific resource identities belong to `types.rs`.
//!
//! ## Hardware
//!
//! `quantum::hardware` owns:
//!
//! - backend implementations;
//! - hardware capabilities;
//! - calibration;
//! - physical topology;
//! - provider-specific execution contracts.
//!
//! `memory::qpu` provides the provider-neutral memory/resource contract needed
//! to connect those hardware implementations to the memory subsystem.
//!
//! ## Routing
//!
//! `quantum::routing` owns logical-to-physical placement and routing.
//!
//! `memory::layout` and `memory::permutation` provide the storage-side
//! representation of the mapping without taking ownership of the routing
//! algorithm.
//!
//! ## Scheduling
//!
//! `quantum::scheduling` owns execution ordering and timing constraints.
//!
//! Memory exposes allocation, availability, coherence and synchronization
//! information needed by schedulers without implementing scheduling itself.
//!
//! ## Error correction
//!
//! `quantum::error_correction` owns QEC algorithms and mechanisms.
//!
//! Memory provides the substrate required by QEC for:
//!
//! - physical/logical qubit resources;
//! - stabilizer state storage;
//! - syndrome/classical memory;
//! - Pauli-frame-related state;
//! - measurement results;
//! - checkpoints.
//!
//! ## Benchmarking
//!
//! `quantum::benchmarking` consumes memory telemetry and execution data.
//!
//! Memory must never depend on benchmark implementations, benchmark protocols,
//! Quantum Volume, randomized benchmarking, XEB, or benchmark reporting.
//!
//! # Representation neutrality
//!
//! Memory is intentionally not synonymous with a dense state vector.
//!
//! The subsystem supports independent representations for workloads including:
//!
//! ```text
//! StateVector
//! DensityMatrix
//! Stabilizer
//! SparseState
//! TensorNetwork
//! BackendNative / QPU resource state
//! ```
//!
//! The individual representation implementations are owned by their respective
//! modules.
//!
//! This allows the same Zamani program to execute against:
//!
//! - CPU state-vector simulation;
//! - density-matrix simulation;
//! - stabilizer simulation;
//! - sparse simulation;
//! - tensor-network simulation;
//! - GPU simulation;
//! - distributed simulation;
//! - real QPUs;
//! - quantum annealers;
//! - photonic hardware;
//! - trapped-ion hardware;
//! - neutral-atom hardware;
//! - superconducting hardware;
//! - spin/qubit hardware;
//! - other provider-native quantum execution systems.
//!
//! The memory layer does not assume that every quantum device exposes
//! byte-addressable quantum RAM. `qpu.rs` explicitly models quantum hardware
//! resources instead of forcing every device into a classical-memory model.
//!
//! # Resource safety
//!
//! Quantum memory can scale exponentially.
//!
//! A dense state vector for `n` qubits requires `2^n` amplitudes, while a dense
//! density matrix requires `4^n` complex matrix elements.
//!
//! Consequently, production memory operations must follow:
//!
//! ```text
//! estimate
//!     |
//!     v
//! validate limits
//!     |
//!     v
//! reserve budget
//!     |
//!     v
//! allocate
//!     |
//!     v
//! initialize
//!     |
//!     v
//! commit
//! ```
//!
//! The following modules own the corresponding stages:
//!
//! - `limits` — resource admissibility;
//! - `budget` — resource accounting;
//! - `reservation` — transactional reservation;
//! - `allocator` — allocation abstraction;
//! - `pool` — reusable allocation management.
//!
//! No representation module should bypass those contracts for managed
//! allocations.
//!
//! # Safety policy
//!
//! This subsystem is explicitly safe Rust.
//!
//! The entire memory module forbids `unsafe` code so that a later contributor
//! cannot accidentally introduce an unsafe implementation into one of the
//! memory providers without the module boundary rejecting it.
//!
//! Raw pointers must not appear in public memory APIs.
//!
//! Hardware and accelerator implementations must expose safe abstractions such
//! as handles, buffers, allocations, streams and events.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly-only language feature is required by this module boundary.
//!
//! # Module organization
//!
//! The declarations are grouped according to dependency responsibility rather
//! than alphabetical order. Rust's module resolution does not require the
//! implementation files to be declared in dependency order, but this ordering
//! documents the intended architecture.
//!
//! ```text
//! foundational contracts
//!         |
//!         v
//! allocation/resource management
//!         |
//!         v
//! logical memory
//!         |
//!         v
//! state representations
//!         |
//!         v
//! state views/transforms
//!         |
//!         v
//! measurement/reset
//!         |
//!         v
//! persistence
//!         |
//!         v
//! coherence/synchronization/cache
//!         |
//!         v
//! CPU/SIMD/GPU/distributed/QPU
//!         |
//!         v
//! migration/compaction/observability
//! ```
//!
//! # Public API stability
//!
//! The module names below are the stable subsystem boundaries.
//!
//! Callers should prefer:
//!
//! ```text
//! quantum::memory::<subsystem>
//! ```
//!
//! rather than depending on implementation details inside another memory
//! module.
//!
//! Individual modules are public because the quantum runtime, simulator,
//! hardware adapters, QEC subsystem, compiler integration and tooling may need
//! specialized capabilities. Each module remains responsible for its own API
//! invariants.
//!
//! # Integration contract
//!
//! Every child module must obey these subsystem-wide rules:
//!
//! 1. Use canonical quantum identities from `quantum::ir` where program-level
//!    quantum identity is required.
//! 2. Use `memory::types` for memory-resource quantities and identities.
//! 3. Use `memory::errors` for memory-domain failures.
//! 4. Validate potentially unbounded resource requests through `limits`.
//! 5. Respect `budget` and `reservation` contracts before large allocations.
//! 6. Never silently convert a resource-intensive operation into an
//!    unbounded allocation.
//! 7. Never silently change state representation when doing so can alter
//!    numerical or semantic guarantees.
//! 8. Preserve logical qubit identity across physical-memory transformations.
//! 9. Preserve declared layout and endianness semantics.
//! 10. Never expose provider-specific types through provider-neutral APIs.
//! 11. Never depend on benchmarking for core memory functionality.
//! 12. Never perform backend I/O from the memory core.
//! 13. Never store credentials or authentication material in memory state.
//! 14. Never use global mutable quantum state.
//! 15. Never use `unsafe`.
//! 16. Never print diagnostic information directly to stdout/stderr from the
//!     memory core.
//! 17. Use explicit RNG ownership/injection where stochastic measurement is
//!     required.
//! 18. Make deterministic operations deterministic for identical inputs and
//!     configuration.
//! 19. Make serialization/versioning explicit at persistence boundaries.
//! 20. Preserve transactional semantics for allocation, migration and
//!     checkpoint restoration.
//!
//! # Integration with `quantum::ir`
//!
//! The canonical flow is:
//!
//! ```text
//! quantum::ir
//!     |
//!     | QubitId / PhysicalQubitId / ClassicalBitId
//!     v
//! quantum::memory
//!     |
//!     +--> logical ownership
//!     +--> storage layout
//!     +--> state representation
//!     +--> measurement storage
//!     +--> execution resources
//! ```
//!
//! Memory does not replace the IR.
//!
//! # Integration with routing
//!
//! Routing owns the algorithm that computes a logical-to-physical mapping.
//! Memory owns the representation required to execute that mapping efficiently:
//!
//! ```text
//! routing
//!     |
//!     v
//! logical -> physical mapping
//!     |
//!     v
//! memory::permutation
//!     |
//!     v
//! memory::layout
//!     |
//!     v
//! state representation
//! ```
//!
//! This permits routing algorithms to change without forcing changes to state
//! storage.
//!
//! # Integration with hardware and QPUs
//!
//! Hardware-specific implementations must remain outside the generic memory
//! namespace.
//!
//! The intended flow is:
//!
//! ```text
//! quantum::hardware
//!        |
//!        | capabilities/topology/calibration/backend
//!        v
//! quantum::memory::qpu
//!        |
//!        | provider-neutral resource contract
//!        v
//! quantum::memory
//!        |
//!        +--> allocation
//!        +--> buffers
//!        +--> classical results
//!        +--> synchronization
//!        +--> snapshots/checkpoints
//! ```
//!
//! A provider may be:
//!
//! - superconducting;
//! - trapped-ion;
//! - neutral-atom;
//! - photonic;
//! - spin-based;
//! - annealing/adiabatic;
//! - qudit-based;
//! - oscillator-based;
//! - hybrid quantum-classical;
//! - simulator-backed;
//! - remote/cloud;
//! - local/on-premises.
//!
//! The memory subsystem must not require a provider SDK to compile.
//!
//! # Integration with simulators
//!
//! Simulator implementations consume the state abstractions:
//!
//! ```text
//! Quantum IR
//!     |
//!     v
//! execution
//!     |
//!     v
//! QuantumState
//!     |
//!     +--> StateVector
//!     +--> DensityMatrix
//!     +--> Stabilizer
//!     +--> SparseState
//!     +--> TensorNetwork
//!     +--> BackendNative
//! ```
//!
//! The memory module does not choose a simulator algorithm merely by virtue of
//! being imported.
//!
//! # Integration with QEC
//!
//! QEC may use different memory representations depending on the code and
//! execution mode:
//!
//! ```text
//! QEC
//!  |
//!  +--> stabilizer state
//!  +--> syndrome/classical memory
//!  +--> physical qubit resources
//!  +--> logical state metadata
//!  +--> measurement results
//!  +--> checkpoint state
//! ```
//!
//! Memory therefore cannot assume that every workload is a dense state vector.
//!
//! # Integration with runtime
//!
//! The runtime may compose memory operations such as:
//!
//! ```text
//! allocate
//!     -> initialize
//!     -> apply execution operation
//!     -> measure
//!     -> collapse/reset where required
//!     -> checkpoint/snapshot where requested
//!     -> release
//! ```
//!
//! Runtime policy remains outside this module.
//!
//! # Integration with benchmarking and telemetry
//!
//! Memory exposes measurement points through `diagnostics` and `telemetry`.
//!
//! Benchmarking may consume values such as:
//!
//! - allocated bytes;
//! - reserved bytes;
//! - peak bytes;
//! - allocation count;
//! - allocation failures;
//! - state representation;
//! - state size;
//! - migration count;
//! - migration bytes;
//! - cache activity;
//! - synchronization activity;
//! - GPU/device memory;
//! - distributed-memory usage;
//! - checkpoint size.
//!
//! The dependency remains one-way:
//!
//! ```text
//! memory --telemetry--> benchmarking
//! ```
//!
//! never:
//!
//! ```text
//! memory --> benchmarking implementation
//! ```
//!
//! # Persistence boundary
//!
//! `snapshot`, `checkpoint` and `serialization` form the persistence boundary.
//!
//! A persisted state must retain enough metadata to prevent accidental
//! interpretation under an incompatible:
//!
//! - representation;
//! - scalar precision;
//! - layout;
//! - qubit ordering;
//! - schema version;
//! - memory format.
//!
//! Restoration must be validated before state ownership is committed.
//!
//! # Concurrency boundary
//!
//! `coherence` and `synchronization` define consistency across:
//!
//! - host memory;
//! - accelerator/device memory;
//! - distributed partitions;
//! - backend-native resources.
//!
//! `cache` may optimize repeated access but must never become a source of
//! semantic state divergence.
//!
//! # Accelerator boundary
//!
//! `cpu` provides CPU-side memory facilities.
//!
//! `simd` provides safe vectorized execution abstractions.
//!
//! `gpu` provides provider-neutral device-memory abstractions.
//!
//! `distributed` provides provider-neutral distributed-memory abstractions.
//!
//! None of these modules should force Zamani to one vendor or instruction-set
//! implementation.
//!
//! # Why `qpu.rs` is part of memory
//!
//! A QPU does not generally expose a conventional addressable quantum-memory
//! array. The appropriate abstraction is therefore a resource contract rather
//! than a fake RAM abstraction.
//!
//! `qpu.rs` is consequently part of this namespace alongside
//! `backend_state.rs`:
//!
//! - `backend_state` represents opaque externally owned execution state;
//! - `qpu` represents provider-neutral physical quantum resource/memory
//!   contracts.
//!
//! Provider-specific adapters remain owned by `quantum::hardware`.
//!
//! # Deliberate omissions
//!
//! The repository currently does not contain `numeric.rs` or
//! `representation.rs` under this directory. They are therefore intentionally
//! not declared here.
//!
//! Numerical policy and representation concepts must use the existing module
//! contracts until those concerns receive dedicated, independently completed
//! files.
//!
//! Adding a `pub mod numeric;` or `pub mod representation;` declaration here
//! before those files exist would make the crate fail to compile.
//!
//! # Test boundary
//!
//! The production test coordinator lives under:
//!
//! ```text
//! quantum::memory::tests
//! ```
//!
//! It is compiled only for tests and is intentionally not part of the runtime
//! public API.
//!
//! The test coordinator validates cross-module invariants without becoming a
//! production dependency.
//!
//! # Definition of done for this module
//!
//! This file is complete when:
//!
//! - every existing production memory module is declared exactly once;
//! - no nonexistent memory module is declared;
//! - the test coordinator is connected only under `cfg(test)`;
//! - no implementation logic is duplicated here;
//! - no provider-specific hardware dependency is introduced;
//! - no benchmark dependency is introduced;
//! - no unsafe code is permitted;
//! - the dependency boundaries are documented;
//! - the module remains compatible with Rust 1.97/1.97.1.
//!
//! Implementation correctness belongs to the individual child modules and
//! their tests; this file is the authoritative composition boundary.

// -----------------------------------------------------------------------------
// Safety policy
// -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// -----------------------------------------------------------------------------
// Foundational memory contracts
// -----------------------------------------------------------------------------

/// Strongly typed memory-domain quantities and resource identities.
pub mod types;

/// Unified memory-domain error taxonomy.
pub mod errors;

/// Memory layout, qubit ordering and storage mapping.
pub mod layout;

/// Checked index and basis-state calculations.
pub mod indexing;

/// Complex/scalar representation primitives used by state storage.
pub mod complex;

/// Resource limits and memory-requirement validation.
pub mod limits;

// -----------------------------------------------------------------------------
// Memory addressing and allocation
// -----------------------------------------------------------------------------

/// Provider-neutral memory-address abstractions.
pub mod address;

/// General memory-allocation contract.
pub mod allocator;

/// Memory budget and hierarchical resource accounting.
pub mod budget;

/// Transactional memory reservation.
pub mod reservation;

/// Reusable allocation pools.
// Keep this after allocator/budget/reservation at the namespace level so the
/// architectural dependency is immediately visible.
pub mod pool;

// -----------------------------------------------------------------------------
// Logical quantum and classical memory
// -----------------------------------------------------------------------------

/// Logical qubit resource ownership.
pub mod qubit;

/// Quantum-register ownership and register operations.
pub mod register;

/// Classical companion memory.
pub mod classical;

/// Quantum-resource lifecycle and ownership state.
pub mod lifetime;

// -----------------------------------------------------------------------------
// Quantum-state representations
// -----------------------------------------------------------------------------

/// Representation-independent quantum-state contract.
pub mod state;

/// Dense pure-state/state-vector representation.
pub mod state_vector;

/// Mixed-state/density-matrix representation.
pub mod density_matrix;

/// Stabilizer/tableau-oriented state representation.
pub mod stabilizer;

/// Sparse quantum-state representation.
pub mod sparse;

/// Tensor-network state representation.
pub mod tensor_network;

/// Opaque state/resource handles for externally owned execution backends.
pub mod backend_state;

/// Provider-neutral QPU and physical quantum-resource memory contract.
pub mod qpu;

// -----------------------------------------------------------------------------
// Views and state transformations
// -----------------------------------------------------------------------------

/// Borrowed/non-owning memory and state views.
pub mod view;

/// Logical/physical qubit permutation support.
pub mod permutation;

/// Safe register/state slicing and projection boundaries.
pub mod slice;

/// Generic tensor storage and tensor operations.
pub mod tensor;

/// Copy-on-write state/storage support.
pub mod copy_on_write;

// -----------------------------------------------------------------------------
// Measurement and state transitions
// -----------------------------------------------------------------------------

/// Measurement result and measurement-memory infrastructure.
pub mod measurement;

/// Quantum measurement-collapse machinery.
pub mod collapse;

/// Quantum reset semantics and reset-memory support.
pub mod reset;

// -----------------------------------------------------------------------------
// Persistence
// -----------------------------------------------------------------------------

/// Immutable quantum-memory snapshots.
pub mod snapshot;

/// Restartable execution checkpoints.
pub mod checkpoint;

/// Versioned serialization/deserialization boundary.
pub mod serialization;

// -----------------------------------------------------------------------------
// Coherence and synchronization
// -----------------------------------------------------------------------------

/// Host/device/distributed state-coherence model.
pub mod coherence;

/// Explicit memory synchronization and fencing.
pub mod synchronization;

/// Bounded memory/state cache infrastructure.
pub mod cache;

// -----------------------------------------------------------------------------
// Execution-memory providers
// -----------------------------------------------------------------------------

/// CPU-side memory provider and host-memory facilities.
pub mod cpu;

/// Safe SIMD/vectorized memory execution layer.
pub mod simd;

/// Provider-neutral GPU/device-memory abstraction.
pub mod gpu;

/// Provider-neutral distributed quantum-memory abstraction.
pub mod distributed;

// -----------------------------------------------------------------------------
// Memory lifecycle and observability
// -----------------------------------------------------------------------------

/// Transactional movement between memory locations/representations.
pub mod migration;

/// Safe compaction of managed memory resources.
pub mod compaction;

/// Diagnostics and human/machine-readable memory inspection.
pub mod diagnostics;

/// Memory telemetry and metrics emission.
pub mod telemetry;

// -----------------------------------------------------------------------------
// Test composition
// -----------------------------------------------------------------------------

/// Cross-module production test coordinator.
///
/// This is deliberately not exported as part of the runtime API.
#[cfg(test)]
mod tests;