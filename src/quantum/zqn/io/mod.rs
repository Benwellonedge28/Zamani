#![forbid(unsafe_code)]

//! # Zamani Quantum Noise — IO
//!
//! This module is the authoritative composition boundary for ZQN's
//! persistence, interchange, canonicalization, schema-versioning, and
//! compatibility infrastructure.
//!
//! # Mission
//!
//! `quantum::zqn::io` defines how ZQN semantic objects cross persistence or
//! interchange boundaries without making Rust's in-memory representation,
//! compiler version, backend implementation, or machine size part of the
//! public data contract.
//!
//! The IO subsystem exists to provide:
//!
//! - stable versioned schemas;
//! - serialization;
//! - deserialization;
//! - canonical representation;
//! - deterministic identity/hashing support;
//! - schema compatibility;
//! - explicit migrations;
//! - resource-bounded untrusted-input handling;
//! - reproducible interchange;
//! - forward evolution of ZQN without silently changing semantics.
//!
//! # Critical architectural rule
//!
//! ```text
//! ZQN semantics
//!       │
//!       ▼
//! typed ZQN objects
//!       │
//!       ▼
//!      IO
//!       │
//!       ├── schema
//!       ├── serialization
//!       ├── deserialization
//!       ├── canonicalization
//!       └── compatibility
//! ```
//!
//! IO is downstream of ZQN semantics.
//!
//! IO MUST NOT become the owner of:
//!
//! - quantum semantics;
//! - probability mathematics;
//! - quantum channels;
//! - faults;
//! - noise models;
//! - calibration;
//! - characterization;
//! - simulation;
//! - propagation;
//! - target capabilities;
//! - routing;
//! - scheduling;
//! - QEC;
//! - hardware providers;
//! - runtime execution.
//!
//! Those responsibilities belong to their respective ZQN or quantum
//! subsystems.
//!
//! # Write-once, scale-everywhere contract
//!
//! Serialization must never encode an artificial machine-size limit.
//!
//! In particular, this module MUST NOT contain:
//!
//! ```text
//! MAX_QUBITS
//! MAX_QUBIT_COUNT
//! MAX_CIRCUIT_SIZE
//! MAX_OPERATIONS
//! MAX_CHANNEL_DIMENSION
//! MAX_NOISE_RESOURCES
//! ```
//!
//! Machine/resource limits are supplied by callers through explicit resource
//! policies where appropriate.
//!
//! The serialized representation describes the computation/noise semantics;
//! it does not encode a particular machine size as an architectural ceiling.
//!
//! Therefore the same semantic ZQN object can be serialized for:
//!
//! - a tiny simulator;
//! - a workstation;
//! - a distributed simulator;
//! - a QPU;
//! - a large-scale quantum system;
//! - a future quantum technology;
//! - a distributed quantum network;
//! - any other compatible target.
//!
//! The only practical limits are the resources and policies available to the
//! serializer, deserializer, runtime, or target.
//!
//! # Canonical quantum identifiers
//!
//! ZQN IO does NOT define a second qubit identifier.
//!
//! Whenever a serialized ZQN object contains quantum-resource identity, the
//! semantic owner remains:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! IO merely serializes/deserializes those canonical values through the
//! contracts established by their owning subsystem.
//!
//! This module must never introduce:
//!
//! ```text
//! zqn::io::QubitId
//! zqn::io::PhysicalQubitId
//! ```
//!
//! or any equivalent competing identity.
//!
//! # Serialization architecture
//!
//! ```text
//! ┌────────────────────────────────────────────┐
//! │              ZQN semantic object           │
//! └──────────────────────┬─────────────────────┘
//!                        │
//!                        ▼
//!                 schema.rs
//!             stable document shape
//!                        │
//!                        ▼
//!             serialization.rs
//!             bytes / interchange
//!                        │
//!                        ▼
//!                canonical.rs
//!          deterministic representation
//! ```
//!
//! Deserialization follows the reverse direction:
//!
//! ```text
//! bytes
//!   │
//!   ▼
//! deserialization.rs
//!   │
//!   ├── framing validation
//!   ├── resource validation
//!   ├── schema-version extraction
//!   ├── structural validation
//!   │
//!   ▼
//! compatibility.rs
//!   │
//!   ├── exact compatibility
//!   ├── explicitly registered compatibility
//!   └── explicitly registered migration
//!   │
//!   ▼
//! schema.rs
//!   │
//!   ▼
//! typed ZQN object
//! ```
//!
//! # No silent compatibility
//!
//! IO must never silently assume that two schema versions are compatible merely
//! because:
//!
//! - their major versions match;
//! - their minor versions differ;
//! - their Rust structures happen to look similar;
//! - serde happens to deserialize both;
//! - unknown fields happen to be ignored.
//!
//! Compatibility must be explicit.
//!
//! A reader may accept an older schema only when:
//!
//! 1. the schema is exactly the current schema;
//! 2. direct compatibility has explicitly been declared; or
//! 3. an explicit deterministic migration path exists.
//!
//! Unsupported schemas must fail explicitly.
//!
//! # Migration ownership
//!
//! `compatibility.rs` owns schema migration policy and migration execution.
//!
//! `deserialization.rs` owns byte/document decoding.
//!
//! `schema.rs` owns the actual stable document/envelope definitions.
//!
//! `serialization.rs` owns conversion from typed objects to the selected
//! interchange representation.
//!
//! `canonical.rs` owns canonical ordering and canonical representation.
//!
//! These responsibilities must not be duplicated.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//!                 core/version
//!                       │
//!                       ▼
//!                    io/schema
//!                       │
//!          ┌────────────┼────────────┐
//!          ▼            ▼            ▼
//! serialization  deserialization  canonical
//!          │            │            │
//!          │            ▼            │
//!          │      compatibility      │
//!          │            │            │
//!          └────────────┼────────────┘
//!                       ▼
//!                  ZQN semantic API
//! ```
//!
//! More precisely, the IO composition root only declares the children.
//! Individual children should depend on the narrowest semantic/core modules
//! necessary for their own contracts.
//!
//! # Forbidden dependency directions
//!
//! IO MUST NOT depend on:
//!
//! ```text
//! frontend ASTs
//! routing implementations
//! scheduling implementations
//! QEC decoder implementations
//! benchmarking implementations
//! UI
//! CLI
//! vendor SDKs
//! cloud APIs
//! network transports
//! runtime executors
//! ```
//!
//! A hardware adapter may choose to use ZQN IO, but ZQN IO must not know about
//! the hardware provider.
//!
//! # Format neutrality
//!
//! The IO composition boundary intentionally does not choose one universal
//! physical storage format.
//!
//! A stable semantic schema and a serialization mechanism are different
//! concerns.
//!
//! Future implementations may provide:
//!
//! - JSON;
//! - binary formats;
//! - canonical byte representations;
//! - streaming formats;
//! - database/document representations;
//! - future standardized interchange formats.
//!
//! Adding another representation must not require changing the semantic ZQN
//! objects.
//!
//! `serialization.rs` and `deserialization.rs` own format-specific mechanics
//! behind their public contracts.
//!
//! # Canonical representation
//!
//! Canonicalization exists for cases where byte-for-byte reproducibility is
//! required, including:
//!
//! - content identity;
//! - hashing;
//! - cache keys;
//! - provenance;
//! - distributed execution;
//! - scientific reproducibility;
//! - regression tests;
//! - deterministic snapshots.
//!
//! Canonicalization MUST NOT be confused with ordinary serialization.
//!
//! Ordinary serialization may preserve a valid representation without being
//! byte-for-byte canonical.
//!
//! Canonical serialization must have a deterministic result for the same
//! semantic input and canonicalization policy.
//!
//! # Determinism
//!
//! IO must be deterministic wherever the selected API promises canonical or
//! deterministic output.
//!
//! It must not depend on:
//!
//! - hash-map iteration order;
//! - thread scheduling;
//! - process identity;
//! - memory addresses;
//! - pointer values;
//! - filesystem ordering;
//! - locale;
//! - wall-clock time;
//! - global mutable state.
//!
//! Timestamps and other intentionally variable metadata must only affect
//! serialized output when they are explicitly part of the semantic/document
//! contract.
//!
//! # Resource safety
//!
//! Deserialization processes potentially untrusted input.
//!
//! Therefore the IO subsystem must support caller-controlled resource policies
//! for potentially expensive operations, including where applicable:
//!
//! - input byte count;
//! - output byte count;
//! - document depth;
//! - collection size;
//! - string size;
//! - number of migration steps;
//! - number of decoded semantic objects;
//! - numerical/tensor dimensions;
//! - allocation-sensitive structures.
//!
//! Resource limits are safety policies, not semantic limits.
//!
//! An implementation MUST reject an input when the caller's configured limit
//! is exceeded.
//!
//! It must never silently truncate the input or semantic object to make it fit.
//!
//! # Error behavior
//!
//! IO operations must distinguish at least:
//!
//! - malformed input;
//! - unsupported schema;
//! - incompatible schema;
//! - failed migration;
//! - resource exhaustion;
//! - invalid semantic data;
//! - canonicalization failure;
//! - serialization failure;
//! - deserialization failure.
//!
//! Errors must preserve useful context without leaking secrets or raw
//! potentially enormous input buffers.
//!
//! No IO function may silently return a partially decoded semantic object.
//!
//! # Atomicity
//!
//! Deserialization must be atomic from the caller's perspective:
//!
//! ```text
//! invalid input ──► error
//!
//! valid input ────► complete validated object
//! ```
//!
//! It must not return an object whose schema migration or validation is only
//! partially complete.
//!
//! Similarly, serialization must not report success after producing only a
//! partial output when the selected API promises complete output.
//!
//! # Version ownership
//!
//! There are two distinct concepts that must not be conflated:
//!
//! 1. ZQN software/package version.
//! 2. Serialized document/schema version.
//!
//! The ZQN software version belongs to:
//!
//! ```text
//! crate::quantum::zqn::core::version
//! ```
//!
//! The serialized document's schema compatibility is owned by:
//!
//! ```text
//! crate::quantum::zqn::io::schema
//! crate::quantum::zqn::io::compatibility
//! ```
//!
//! This prevents package release numbering from being incorrectly used as a
//! schema migration mechanism.
//!
//! # Provenance
//!
//! IO must preserve provenance fields defined by the owning ZQN semantic
//! objects/schema.
//!
//! It must never invent scientific provenance.
//!
//! In particular, deserializing a model does not make it experimentally
//! validated.
//!
//! The IO layer only preserves the declared provenance.
//!
//! # Security boundary
//!
//! Serialized ZQN documents are data, not executable code.
//!
//! IO must never:
//!
//! - execute serialized code;
//! - execute migration-provided arbitrary external commands;
//! - load vendor plugins implicitly;
//! - make network requests;
//! - access credentials;
//! - discover hardware;
//! - modify global process state.
//!
//! Migrations must be deterministic code registered by the trusted ZQN
//! compatibility layer.
//!
//! # Thread safety
//!
//! The IO composition boundary owns no mutable shared state.
//!
//! Child modules should prefer immutable, `Send`/`Sync`-compatible values where
//! semantically possible.
//!
//! A migration registry, serializer configuration, schema descriptor, or
//! canonicalization policy must not require a global singleton.
//!
//! Caller-owned instances should be passed explicitly.
//!
//! # Streaming
//!
//! IO APIs should support streaming where the underlying format permits it.
//!
//! The composition root itself does not impose a requirement that every
//! serialization operation materialize the complete quantum/noise model in
//! memory.
//!
//! In particular, future large-scale ZQN documents may contain:
//!
//! - many resources;
//! - large fault streams;
//! - large calibration datasets;
//! - large characterization observations;
//! - distributed execution metadata.
//!
//! APIs must therefore avoid making `Vec<T>` the only representation for
//! arbitrarily large collections when a streaming/iterator representation is
//! semantically appropriate.
//!
//! # Compatibility with future quantum technologies
//!
//! This module must remain neutral with respect to:
//!
//! - qubit systems;
//! - qudit systems;
//! - continuous-variable systems;
//! - bosonic systems;
//! - analog systems;
//! - annealing systems;
//! - measurement-based systems;
//! - photonic systems;
//! - distributed quantum systems;
//! - fault-tolerant logical systems;
//! - future quantum computational models.
//!
//! IO serializes the semantic model exposed by ZQN. It must not assume that a
//! quantum operation necessarily has:
//!
//! - one qubit;
//! - two qubits;
//! - a fixed gate arity;
//! - a fixed topology;
//! - a fixed state-vector dimension;
//! - a fixed number of classical outputs.
//!
//! # Integration with canonical IR
//!
//! The IO subsystem may serialize ZQN structures that refer to canonical IR
//! resources.
//!
//! It must not replace canonical IR identities with local IDs.
//!
//! When `QubitId` or `PhysicalQubitId` occurs, its serialization contract is
//! determined by the canonical type owner:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! This module therefore intentionally contains no qubit-specific code.
//!
//! # Integration with calibration
//!
//! Calibration snapshots may be serialized through ZQN IO.
//!
//! IO preserves:
//!
//! - calibration identity;
//! - schema version;
//! - validity information;
//! - uncertainty;
//! - provenance;
//! - resource association.
//!
//! IO does not determine whether a calibration is physically valid. That is the
//! responsibility of `zqn::calibration::validation`.
//!
//! # Integration with characterization
//!
//! Characterization observations and derived models may be serialized.
//!
//! IO preserves the distinction between:
//!
//! ```text
//! raw observation
//! estimated parameter
//! uncertainty
//! characterized model
//! validated model
//! ```
//!
//! It must never upgrade an estimate to a validated physical fact.
//!
//! # Integration with simulation
//!
//! Simulation configuration/results may use ZQN IO for persistence and
//! reproducibility.
//!
//! The IO layer does not execute simulations and does not own random-number
//! generation.
//!
//! Reproducibility metadata is serialized only according to the simulation and
//! schema contracts.
//!
//! # Integration with propagation
//!
//! Error budgets, uncertainty estimates, fidelity estimates, bounds and
//! sensitivity information may be serialized.
//!
//! IO does not calculate those values.
//!
//! # Integration with target capabilities
//!
//! Target requirements/capabilities may be serialized when they form part of a
//! persistent ZQN document.
//!
//! IO does not perform target negotiation.
//!
//! Compatibility of a semantic ZQN object with a physical target remains the
//! responsibility of `zqn::target` and the hardware integration layer.
//!
//! # Integration with QEC
//!
//! ZQN IO may persist physical-noise/fault models consumed by QEC.
//!
//! It must not encode QEC decoder behavior or create a second QEC schema unless
//! explicitly owned by the QEC subsystem.
//!
//! # Integration with routing and scheduling
//!
//! IO may persist noise information consumed by routing/scheduling.
//!
//! It must not encode routing or scheduling semantics into the ZQN schema merely
//! because those consumers use ZQN information.
//!
//! # Integration with benchmarking
//!
//! Benchmarking may serialize ZQN characterization data, observations,
//! calibration snapshots, and noise models.
//!
//! ZQN IO must not become dependent on benchmark implementations.
//!
//! # Public API policy
//!
//! This module should expose the child namespaces rather than wildcard-reexport
//! every implementation detail.
//!
//! Consumers should normally import from the specific child module:
//!
//! ```text
//! quantum::zqn::io::schema
//! quantum::zqn::io::serialization
//! quantum::zqn::io::deserialization
//! quantum::zqn::io::canonical
//! quantum::zqn::io::compatibility
//! ```
//!
//! Stable high-level convenience re-exports, if required, should be introduced
//! by the corresponding child module and then selectively re-exported from the
//! ZQN root/prelude.
//!
//! This composition boundary should not become a namespace containing every
//! type in the IO implementation.
//!
//! # File-completion contract
//!
//! This file is considered complete when:
//!
//! 1. every declared IO child exists;
//! 2. every child owns exactly one responsibility;
//! 3. no serialization implementation exists here;
//! 4. no deserialization implementation exists here;
//! 5. no schema definition is duplicated here;
//! 6. no migration implementation exists here;
//! 7. no canonicalization algorithm exists here;
//! 8. no vendor dependency exists here;
//! 9. no runtime dependency exists here;
//! 10. no global mutable state exists here;
//! 11. no unsafe Rust exists here;
//! 12. no machine-size ceiling exists here;
//! 13. canonical `quantum::ir::qubit` identity remains authoritative;
//! 14. schema compatibility is explicit;
//! 15. child APIs can evolve independently behind this boundary;
//! 16. adding a new serialization format does not require changing semantic ZQN
//!     objects;
//! 17. changing a hardware backend does not require changing this module;
//! 18. changing simulator implementation does not require changing this module;
//! 19. changing routing/scheduling/QEC implementations does not require changing
//!     this module;
//! 20. the module remains compatible with Rust 1.97 and Rust 1.97.1.
//!
//! # Testing contract
//!
//! This file intentionally contains no domain-level unit tests.
//!
//! Compilation of this module verifies that all declared child boundaries
//! exist.
//!
//! The children own their tests:
//!
//! ```text
//! schema.rs
//!     schema invariants
//!
//! serialization.rs
//!     serialization tests
//!
//! deserialization.rs
//!     hostile-input/resource/error tests
//!
//! canonical.rs
//!     canonical ordering/hash tests
//!
//! compatibility.rs
//!     migration/compatibility tests
//! ```
//!
//! Cross-module IO tests belong under:
//!
//! ```text
//! src/quantum/zqn/tests/integration/
//! src/quantum/zqn/tests/compatibility/
//! src/quantum/zqn/tests/determinism/
//! ```
//!
//! # Rust version
//!
//! This module deliberately uses only Rust constructs available on:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition.
//!
//! No `unsafe` code or nightly feature is required.
//!
//! # Module declarations
//!
//! The order below reflects dependency/reading order, not an implementation
//! dependency imposed by Rust.
//!
//! `schema` defines the stable document boundary.
//! `serialization` and `deserialization` implement conversion.
//! `canonical` defines deterministic canonical representation.
//! `compatibility` defines explicit schema compatibility and migration.
//!
//! These modules should remain independently testable.

// -----------------------------------------------------------------------------
// Stable schema boundary
// -----------------------------------------------------------------------------

/// Stable, versioned ZQN document/envelope definitions.
///
/// Owns the external schema shape, field semantics, schema identifiers and
/// schema-level invariants.
///
/// Does not own byte encoding or migrations.
pub mod schema;

// -----------------------------------------------------------------------------
// Serialization
// -----------------------------------------------------------------------------

/// Conversion of validated ZQN semantic documents/objects into an external
/// representation.
///
/// Owns output configuration, framing where applicable, output limits and
/// serialization errors.
///
/// Does not own schema migration.
pub mod serialization;

// -----------------------------------------------------------------------------
// Deserialization
// -----------------------------------------------------------------------------

/// Conversion of external representations into validated ZQN documents/objects.
///
/// Owns input validation, framing validation, input resource limits and
/// deserialization errors.
///
/// It delegates schema compatibility/migration to `compatibility`.
pub mod deserialization;

// -----------------------------------------------------------------------------
// Canonical representation
// -----------------------------------------------------------------------------

/// Deterministic canonical representation and canonical identity support.
///
/// Owns ordering/canonicalization rules.
///
/// Does not own ordinary serialization or schema migration.
pub mod canonical;

// -----------------------------------------------------------------------------
// Schema compatibility
// -----------------------------------------------------------------------------

/// Explicit schema compatibility declarations and deterministic migrations.
///
/// Owns compatibility decisions and migration paths.
///
/// Does not own byte parsing or semantic model definitions.
pub mod compatibility;