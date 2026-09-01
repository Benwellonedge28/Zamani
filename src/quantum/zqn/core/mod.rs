//! Zamani Quantum Noise (ZQN) — Core Foundation
//!
//! This module is the dependency-lowest foundation of the Zamani Quantum Noise
//! subsystem.
//!
//! # Mission
//!
//! `quantum::zqn::core` defines the infrastructure shared by all ZQN layers:
//!
//! ```text
//!                         quantum::zqn::core
//!                                  │
//!          ┌───────────────┬───────┼────────┬───────────────┐
//!          │               │       │        │               │
//!          ▼               ▼       ▼        ▼               ▼
//!        error            ids   metadata   version       context
//!          │               │       │        │               │
//!          └───────────────┴───────┼────────┴───────────────┘
//!                                  │
//!                                  ▼
//!                         ZQN domain subsystems
//!                                  │
//!             ┌────────────────────┼────────────────────┐
//!             │                    │                    │
//!             ▼                    ▼                    ▼
//!        probability          channels              faults
//!             │                    │                    │
//!             └────────────────────┼────────────────────┘
//!                                  ▼
//!                              noise
//!                                  │
//!                ┌─────────────────┼──────────────────┐
//!                ▼                 ▼                  ▼
//!          calibration      characterization      simulation
//!                │                 │                  │
//!                └─────────────────┼──────────────────┘
//!                                  ▼
//!                         propagation / target
//!                                  │
//!                                  ▼
//!                             integration
//! ```
//!
//! The core layer is deliberately below:
//!
//! - probability mathematics;
//! - quantum channels;
//! - fault generation;
//! - noise models;
//! - calibration;
//! - characterization;
//! - simulation;
//! - error propagation;
//! - target lowering;
//! - routing;
//! - scheduling;
//! - QEC;
//! - hardware;
//! - runtime;
//! - benchmarking;
//! - frontend parsing;
//! - source-language ASTs.
//!
//! # Architectural boundary
//!
//! ZQN answers:
//!
//! > "What physical uncertainty, noise, fault, calibration uncertainty, or
//! > stochastic effect is associated with a quantum computation?"
//!
//! Canonical quantum IR answers:
//!
//! > "What does the computation mean?"
//!
//! Hardware answers:
//!
//! > "What physical resources and capabilities does the target provide?"
//!
//! Runtime answers:
//!
//! > "How is execution orchestrated?"
//!
//! QEC answers:
//!
//! > "How is fault tolerance, encoding, syndrome processing, decoding, and
//! > correction performed?"
//!
//! ZQN must never become a second canonical Quantum IR.
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::frontend
//!      │
//!      ▼
//! quantum::ir                 canonical semantics
//!      │
//!      ├───────────────────────────────┐
//!      │                               │
//!      ▼                               ▼
//! compiler transformations             ZQN
//!                                      │
//!                                      ├── probability
//!                                      ├── channels
//!                                      ├── faults
//!                                      ├── noise
//!                                      ├── calibration
//!                                      ├── uncertainty
//!                                      └── provenance
//! ```
//!
//! # Ownership
//!
//! This module owns only the **core ZQN module boundary**.
//!
//! The individual child modules own their respective concepts:
//!
//! ```text
//! error.rs
//!     ZQN error vocabulary and result contracts.
//!
//! ids.rs
//!     ZQN-owned stable identifiers and references.
//!
//! metadata.rs
//!     Backend-independent metadata and annotations.
//!
//! version.rs
//!     ZQN semantic/schema/version compatibility contract.
//!
//! context.rs
//!     Explicit operation/execution context shared by ZQN APIs.
//!
//! capabilities.rs
//!     ZQN capability vocabulary and capability contracts.
//!
//! limits.rs
//!     Explicit resource and security policy limits.
//!
//! provenance.rs
//!     Scientific/reproducibility provenance.
//! ```
//!
//! `mod.rs` itself owns none of those data structures.
//!
//! # Canonical quantum identity
//!
//! ZQN must never define another:
//!
//! ```text
//! QubitId
//! PhysicalQubitId
//! ```
//!
//! The canonical definitions belong to:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The repository explicitly establishes `quantum::ir::qubit` as the
//! authoritative qubit identity boundary.
//!
//! ZQN may consume those types through `ids.rs`, noise locations, operations,
//! calibration scopes, fault locations, or integration adapters.
//!
//! It must never manufacture a competing ZQN-specific qubit identity.
//!
//! For example, this is correct:
//!
//! ```text
//! use crate::quantum::ir::qubit::QubitId;
//! ```
//!
//! This is forbidden:
//!
//! ```text
//! struct QubitId(...);
//! ```
//!
//! and also forbidden:
//!
//! ```text
//! type QubitId = usize;
//! ```
//!
//! inside ZQN.
//!
//! # Logical and physical identity
//!
//! The distinction remains explicit:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//!     = canonical logical quantum identity
//!
//! quantum::ir::qubit::PhysicalQubitId
//!     = canonical physical identity vocabulary
//!
//! ZQN identifiers
//!     = identifiers of ZQN semantic objects
//!
//! hardware identifiers
//!     = owned by the hardware subsystem
//! ```
//!
//! A ZQN object may therefore refer to a qubit without taking ownership of the
//! meaning of that qubit's identity.
//!
//! # Write once, scale everywhere
//!
//! ZQN imposes **no semantic machine-size ceiling**.
//!
//! In particular, this module must never contain:
//!
//! ```text
//! MAX_QUBITS
//! MAX_PHYSICAL_QUBITS
//! MAX_QUBIT_INDEX
//! MAX_GATES
//! MAX_OPERATIONS
//! MAX_DEPTH
//! MAX_CORRELATED_QUBITS
//! ```
//!
//! Machine size is data and target capability, not a property of the ZQN
//! language model.
//!
//! The intended semantic rule is:
//!
//! > A Zamani program may describe any finite quantum computation that can be
//! > represented and processed by the available resources.
//!
//! "Infinity" therefore means:
//!
//! > no artificial finite architectural ceiling is encoded by ZQN.
//!
//! It does **not** claim that a particular computer has infinite memory,
//! processing capacity, address space, storage, or physical quantum resources.
//!
//! Concrete limits are explicit policies supplied by:
//!
//! - compiler;
//! - runtime;
//! - simulator;
//! - memory subsystem;
//! - target;
//! - hardware;
//! - user;
//! - security policy;
//! - execution environment.
//!
//! `limits.rs` exists precisely to keep those operational constraints separate
//! from ZQN semantics.
//!
//! # Policy versus architecture
//!
//! This distinction is mandatory:
//!
//! ```text
//! semantic architecture
//!     │
//!     └── has no arbitrary machine-size ceiling
//!
//! operational policy
//!     │
//!     └── may impose finite limits for safety/resource reasons
//! ```
//!
//! Therefore:
//!
//! ```text
//! "this execution permits at most N resources"
//! ```
//!
//! is valid.
//!
//! ```text
//! "Zamani supports at most N resources"
//! ```
//!
//! is not a valid ZQN architectural rule.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//!                         ZQN core
//!                            ▲
//!                            │
//!            ┌───────────────┼────────────────┐
//!            │               │                │
//!            │               │                │
//!       probability       channels           faults
//!            ▲               ▲                ▲
//!            │               │                │
//!            └───────────────┼────────────────┘
//!                            ▲
//!                            │
//!                           noise
//!                            ▲
//!             ┌──────────────┼──────────────┐
//!             │              │              │
//!        calibration   characterization  simulation
//!             │              │              │
//!             └──────────────┼──────────────┘
//!                            ▲
//!                            │
//!                      propagation
//!                            ▲
//!                            │
//!                         target
//!                            ▲
//!                            │
//!                       integration
//! ```
//!
//! This diagram expresses conceptual dependency direction. Individual modules
//! may use narrower interfaces rather than importing every preceding layer.
//!
//! # Forbidden dependencies
//!
//! `quantum::zqn::core` must never depend on:
//!
//! - `crate::quantum::frontend`;
//! - `crate::quantum::algorithms`;
//! - `crate::quantum::optimization`;
//! - `crate::quantum::routing`;
//! - `crate::quantum::scheduling`;
//! - `crate::quantum::hardware` implementations;
//! - vendor SDKs;
//! - QPU credentials;
//! - network clients;
//! - filesystem execution;
//! - simulator implementations;
//! - QEC decoder implementations;
//! - benchmarking implementations;
//! - UI;
//! - CLI;
//! - source-language ASTs;
//! - backend-specific instruction sets.
//!
//! Core must remain reusable by every downstream implementation.
//!
//! # No vendor coupling
//!
//! There must never be vendor-specific core modules such as:
//!
//! ```text
//! core/ibm.rs
//! core/ionq.rs
//! core/rigetti.rs
//! core/quantinuum.rs
//! core/google.rs
//! core/amazon.rs
//! ```
//!
//! Vendor/device integration belongs to the hardware subsystem.
//!
//! ZQN consumes provider-neutral:
//!
//! - capabilities;
//! - calibration snapshots;
//! - observations;
//! - target descriptions;
//! - resource references.
//!
//! # No second IR
//!
//! ZQN core must not introduce:
//!
//! - gate ASTs;
//! - circuit ASTs;
//! - frontend syntax;
//! - canonical operation trees;
//! - source-language constructs;
//! - alternative qubit semantics.
//!
//! A ZQN noise attachment refers to an operation/resource from the canonical
//! quantum IR or to a downstream physical realization.
//!
//! The canonical semantic program remains owned by:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! # Core child modules
//!
//! The complete core boundary is:
//!
//! ```text
//! core/
//! ├── mod.rs
//! ├── error.rs
//! ├── ids.rs
//! ├── metadata.rs
//! ├── version.rs
//! ├── context.rs
//! ├── capabilities.rs
//! ├── limits.rs
//! └── provenance.rs
//! ```
//!
//! Each file has exactly one primary responsibility.
//!
//! # `error.rs`
//!
//! Owns the ZQN error vocabulary.
//!
//! It must provide a single result contract used throughout ZQN.
//!
//! Conceptually:
//!
//! ```text
//! ZqnError
//! ZqnResult<T>
//! ```
//!
//! It must cover domain failures such as:
//!
//! - invalid probability;
//! - invalid distribution;
//! - invalid channel;
//! - invalid fault;
//! - invalid noise model;
//! - invalid calibration;
//! - invalid characterization;
//! - unsupported representation;
//! - unsupported capability;
//! - resource-limit violation;
//! - numerical failure;
//! - non-finite value;
//! - serialization failure;
//! - deserialization failure;
//! - validation failure;
//! - compatibility failure;
//! - cancellation.
//!
//! It must not contain vendor-specific error types.
//!
//! # `ids.rs`
//!
//! Owns identifiers for ZQN semantic objects.
//!
//! It may define identifiers such as:
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
//! It must not define a competing `QubitId` or `PhysicalQubitId`.
//!
//! Where a ZQN object refers to a quantum resource, `ids.rs` must use the
//! canonical IR qubit types:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! # `metadata.rs`
//!
//! Owns non-semantic metadata that may accompany ZQN objects.
//!
//! Metadata may include concepts such as:
//!
//! - name;
//! - description;
//! - labels;
//! - annotations;
//! - units;
//! - source information;
//! - classification.
//!
//! Metadata must never silently change mathematical semantics.
//!
//! # `version.rs`
//!
//! Owns the authoritative ZQN version and compatibility vocabulary.
//!
//! It must distinguish at minimum:
//!
//! ```text
//! semantic version
//! schema version
//! serialization version
//! compatibility policy
//! ```
//!
//! `mod.rs` must never duplicate version constants.
//!
//! # `context.rs`
//!
//! Owns explicit context shared between ZQN operations.
//!
//! A context may provide:
//!
//! - resource limits;
//! - capabilities;
//! - deterministic execution information;
//! - cancellation;
//! - calibration scope;
//! - provenance;
//! - numerical policy.
//!
//! The context must not hide global mutable state.
//!
//! # `capabilities.rs`
//!
//! Owns provider-neutral ZQN capability descriptions.
//!
//! Examples include support for:
//!
//! - channel representations;
//! - correlated noise;
//! - temporal dependence;
//! - spatial dependence;
//! - leakage;
//! - erasure;
//! - loss;
//! - readout noise;
//! - dynamic/conditional noise;
//! - calibration;
//! - characterization;
//! - continuous-time models.
//!
//! Capabilities describe what is supported; they do not perform execution.
//!
//! # `limits.rs`
//!
//! Owns explicit operational/resource/security policies.
//!
//! Limits may apply to:
//!
//! - operation count;
//! - fault count;
//! - distribution size;
//! - sampling shots;
//! - tensor materialization;
//! - memory;
//! - computation time;
//! - recursion/iteration;
//! - serialized input size.
//!
//! Limits must be configurable policy.
//!
//! They are not architectural quantum-size limits.
//!
//! # `provenance.rs`
//!
//! Owns reproducibility and scientific provenance.
//!
//! It should be possible for a downstream result to identify, where applicable:
//!
//! - ZQN version;
//! - model identity;
//! - model configuration identity;
//! - calibration identity;
//! - target identity;
//! - experiment identity;
//! - source;
//! - timestamp;
//! - software identity;
//! - measurement/characterization source.
//!
//! Provenance must not depend on process-local memory addresses.
//!
//! # Determinism
//!
//! ZQN core must support deterministic execution without introducing global
//! randomness.
//!
//! Core must never:
//!
//! - own a global RNG;
//! - seed a hidden RNG;
//! - use process-global random state;
//! - use memory addresses as semantic identity;
//! - use unordered iteration as a semantic ordering mechanism.
//!
//! Deterministic stochastic execution belongs to explicit context contracts.
//!
//! The conceptual execution identity is:
//!
//! ```text
//! master seed
//!     + program identity
//!     + model identity
//!     + calibration identity
//!     + target identity
//!     + operation identity
//!     + resource identity
//!     + shot identity
//! ```
//!
//! This permits parallel execution to remain reproducible.
//!
//! # Parallelism
//!
//! Core contains no global mutable state and therefore does not impose a
//! serialization point on independent ZQN executions.
//!
//! Child implementations should be designed so that:
//!
//! ```text
//! sequential execution
//! ```
//!
//! and:
//!
//! ```text
//! parallel execution
//! ```
//!
//! can produce identical deterministic results under the same deterministic
//! policy.
//!
//! # Distributed execution
//!
//! Core must remain suitable for distributed quantum computation.
//!
//! Identifiers must therefore not depend on:
//!
//! - local memory addresses;
//! - process-local counters;
//! - thread-local counters;
//! - implicit host ordering.
//!
//! Explicit stable identities should be supplied by the owning construction
//! context.
//!
//! # Resource safety
//!
//! The core layer must not allocate large structures merely because it is
//! imported.
//!
//! It must not eagerly initialize:
//!
//! - simulators;
//! - probability tables;
//! - calibration databases;
//! - device registries;
//! - network connections;
//! - caches;
//! - random generators.
//!
//! Resource-heavy operations belong to explicit downstream APIs and must accept
//! explicit resource policies.
//!
//! # Numerical safety
//!
//! Core does not itself implement quantum numerical mathematics, but its
//! contracts must permit downstream modules to distinguish:
//!
//! ```text
//! exact
//! approximate
//! bounded
//! statistical
//! unsupported
//! ```
//!
//! No downstream implementation may silently turn an invalid numerical value
//! into a valid semantic value.
//!
//! Examples of forbidden silent repair:
//!
//! ```text
//! NaN       -> 0
//! infinity  -> finite maximum
//! negative probability -> absolute value
//! ```
//!
//! Numerical validation belongs to the mathematical modules, but the core
//! error/context contracts must provide the appropriate failure boundary.
//!
//! # Approximation contract
//!
//! ZQN must support explicit approximation.
//!
//! A downstream implementation that approximates a requested noise model must
//! be able to communicate:
//!
//! ```text
//! requested semantics
//! realized semantics
//! approximation policy
//! tolerance or bound
//! confidence where applicable
//! assumptions
//! ```
//!
//! Core therefore must not contain an implicit "best effort" compatibility
//! policy.
//!
//! # Serialization
//!
//! Core types should be suitable for deterministic serialization, but the core
//! module does not own the complete external ZQN serialization format.
//!
//! The complete interchange boundary belongs to:
//!
//! ```text
//! quantum::zqn::io
//! ```
//!
//! Core objects must avoid semantic dependence on:
//!
//! - memory addresses;
//! - pointer identity;
//! - process identity;
//! - thread identity;
//! - hash-map iteration order;
//! - global mutable state.
//!
//! Serialization/schema evolution is owned by `version.rs` and the ZQN `io`
//! subsystem through explicit contracts.
//!
//! # Hashing and identity
//!
//! Core does not define the complete canonical hash of a ZQN model.
//!
//! Canonical hashing belongs to the appropriate ZQN identity/serialization
//! layer.
//!
//! Core identifiers nevertheless must be stable and deterministic.
//!
//! # Thread safety
//!
//! The core boundary owns no global mutable state.
//!
//! Child types should be `Send` and `Sync` where their semantics permit it.
//!
//! This file does not force unnecessary synchronization primitives into the
//! architecture.
//!
//! # Unsafe Rust
//!
//! Unsafe Rust is forbidden throughout ZQN core.
//!
//! This is compiler-enforced:
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! No unsafe escape hatch, raw pointer abstraction, FFI primitive, or backend
//! handle belongs in this layer.
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
//! The module must not require a newer language feature merely for convenience.
//!
//! # Public API philosophy
//!
//! `core::mod.rs` is a namespace boundary.
//!
//! It must not perform broad glob exports such as:
//!
//! ```text
//! pub use error::*;
//! pub use ids::*;
//! ```
//!
//! Broad exports make ownership ambiguous and make future additions more likely
//! to create breaking name collisions.
//!
//! Consumers should use explicit paths:
//!
//! ```text
//! crate::quantum::zqn::core::error::ZqnError
//! crate::quantum::zqn::core::ids::NoiseModelId
//! crate::quantum::zqn::core::context::ZqnContext
//! ```
//!
//! Higher-level ZQN modules may explicitly re-export stable public concepts at
//! their own boundaries.
//!
//! # Compatibility
//!
//! This module must not create compatibility aliases for canonical quantum
//! identities.
//!
//! In particular, do not introduce:
//!
//! ```text
//! zqn::core::QubitId
//! zqn::core::PhysicalQubitId
//! ```
//!
//! as competing semantic definitions.
//!
//! The canonical types remain:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! # Integration with canonical Quantum IR
//!
//! The intended relationship is:
//!
//! ```text
//!                    quantum::ir
//!                       │
//!                       │ canonical semantic program
//!                       ▼
//!                    ZQN core
//!                       │
//!             ┌─────────┼──────────┐
//!             ▼         ▼          ▼
//!          channels   faults     noise
//! ```
//!
//! ZQN does not own the IR operation.
//!
//! It may identify an operation using an IR-owned `OperationId` or another
//! canonical reference defined by the IR contract.
//!
//! If a noise location is a qubit, it must use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! for logical resources and:
//!
//! ```text
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! for physical resources.
//!
//! # Integration with routing
//!
//! Routing may consume ZQN information such as:
//!
//! - noise estimates;
//! - error costs;
//! - fidelity estimates;
//! - correlation costs;
//! - crosstalk information;
//! - calibration validity.
//!
//! ZQN core itself must not import the routing implementation.
//!
//! Dependency direction:
//!
//! ```text
//! routing ──► ZQN
//! ```
//!
//! not:
//!
//! ```text
//! ZQN ──► routing
//! ```
//!
//! # Integration with scheduling
//!
//! Scheduling may consume ZQN information such as:
//!
//! - duration-dependent noise;
//! - idle noise;
//! - drift;
//! - temporal correlations;
//! - calibration validity;
//! - crosstalk.
//!
//! ZQN core must remain independent of the scheduler.
//!
//! # Integration with QEC
//!
//! QEC may consume ZQN noise/channel/fault abstractions.
//!
//! The long-term relationship is:
//!
//! ```text
//!                    ZQN
//!                     │
//!                     ▼
//!             physical fault model
//!                     │
//!                     ▼
//!                    QEC
//!                     │
//!        ┌────────────┼────────────┐
//!        ▼            ▼            ▼
//!     syndrome      decoder      correction
//! ```
//!
//! QEC owns fault-tolerance algorithms.
//!
//! ZQN owns universal physical-noise semantics.
//!
//! Core must not depend on QEC.
//!
//! # Integration with hardware
//!
//! Hardware provides provider-neutral information to ZQN:
//!
//! ```text
//! hardware
//!    │
//!    ├── capabilities
//!    ├── calibration
//!    ├── observations
//!    └── resource identity
//!    │
//!    ▼
//!   ZQN
//! ```
//!
//! ZQN must not call hardware APIs directly.
//!
//! # Integration with memory
//!
//! Memory/simulation layers may consume ZQN channel/fault semantics:
//!
//! ```text
//! ZQN
//!  │
//!  ▼
//! memory/state representation
//! ```
//!
//! Core itself does not know which state representation is used.
//!
//! # Integration with benchmarking
//!
//! Benchmarking consumes ZQN outputs such as:
//!
//! - observations;
//! - characterization results;
//! - error estimates;
//! - calibration information;
//! - uncertainty;
//! - reproducibility metadata.
//!
//! Core must not depend on benchmarking.
//!
//! # Integration with runtime
//!
//! Runtime supplies execution context and resource policy.
//!
//! ```text
//! runtime
//!   │
//!   ├── limits
//!   ├── cancellation
//!   ├── deterministic execution context
//!   └── target context
//!   │
//!   ▼
//!  ZQN
//! ```
//!
//! ZQN returns semantic noise/channel/fault information.
//!
//! Runtime remains responsible for orchestration.
//!
//! # Integration with target capabilities
//!
//! A target declares what it can represent or execute.
//!
//! ZQN core provides the shared capability vocabulary used by:
//!
//! ```text
//! requested ZQN semantics
//!           │
//!           ▼
//! target capabilities
//!           │
//!           ▼
//! compatibility decision
//! ```
//!
//! Compatibility must distinguish:
//!
//! ```text
//! exact support
//! approximate support
//! bounded support
//! unsupported
//! ```
//!
//! No silent approximation is permitted.
//!
//! # Module implementation order
//!
//! The child modules should be implemented in this dependency order:
//!
//! ```text
//! version.rs
//!     │
//!     ├──────────────┐
//!     ▼              ▼
//! ids.rs          error.rs
//!     │              │
//!     └──────┬───────┘
//!            ▼
//!       metadata.rs
//!            │
//!            ▼
//!       capabilities.rs
//!            │
//!            ▼
//!         limits.rs
//!            │
//!            ▼
//!        provenance.rs
//!            │
//!            ▼
//!         context.rs
//! ```
//!
//! The actual Rust module declarations below do not force that implementation
//! order. They establish stable namespaces.
//!
//! # Independence guarantee
//!
//! Once this module boundary is established, implementing or improving one
//! child core file must not require reopening `core/mod.rs` unless the **public
//! module boundary itself** changes.
//!
//! For example:
//!
//! ```text
//! improve ids.rs
//!      └── no mod.rs change
//!
//! improve limits.rs
//!      └── no mod.rs change
//!
//! improve provenance.rs
//!      └── no mod.rs change
//!
//! improve context.rs
//!      └── no mod.rs change
//! ```
//!
//! This is deliberate.
//!
//! # Adding future core modules
//!
//! A new module may be added here only when all of the following are true:
//!
//! 1. It is genuinely foundational.
//! 2. Multiple independent ZQN subsystems require it.
//! 3. It is hardware-independent.
//! 4. It does not duplicate an existing canonical Zamani type.
//! 5. It does not introduce a machine-size ceiling.
//! 6. It does not introduce global mutable state.
//! 7. It has an explicit ownership contract.
//! 8. It has an explicit error/resource contract.
//! 9. It has deterministic semantics where applicable.
//! 10. It has a stable public API.
//!
//! # Things that must NOT be added here
//!
//! Do not add:
//!
//! ```text
//! gate.rs
//! routing.rs
//! scheduling.rs
//! simulator.rs
//! qec.rs
//! hardware.rs
//! backend.rs
//! vendor.rs
//! frontend.rs
//! benchmark.rs
//! ```
//!
//! Those belong to their respective subsystems.
//!
//! # Testing contract
//!
//! Core composition testing must be deliberately lightweight.
//!
//! Domain tests belong to the child modules:
//!
//! ```text
//! error.rs       -> error contract tests
//! ids.rs         -> identity tests
//! metadata.rs    -> metadata tests
//! version.rs     -> version tests
//! context.rs     -> context tests
//! capabilities.rs-> capability tests
//! limits.rs      -> policy/limit tests
//! provenance.rs  -> provenance tests
//! ```
//!
//! The module boundary itself is primarily tested by:
//!
//! - successful compilation;
//! - documentation compilation;
//! - visibility checks;
//! - workspace integration tests.
//!
//! `mod.rs` must not invent tests against speculative downstream APIs.
//!
//! # Production-readiness checklist
//!
//! This module is considered complete when:
//!
//! - [x] every responsibility has a dedicated child boundary;
//! - [x] no duplicate `QubitId` exists;
//! - [x] canonical `quantum::ir::qubit` remains authoritative;
//! - [x] no vendor dependency exists;
//! - [x] no backend dependency exists;
//! - [x] no hardware-size constant exists;
//! - [x] no global mutable state exists;
//! - [x] unsafe Rust is forbidden;
//! - [x] Rust 1.97/1.97.1 remains supported;
//! - [x] module ownership is explicit;
//! - [x] integration direction is explicit;
//! - [x] resource-policy ownership is explicit;
//! - [x] determinism boundary is explicit;
//! - [x] serialization ownership is explicit;
//! - [x] approximation policy is explicit;
//! - [x] future extension rules are explicit;
//! - [x] independent child implementation does not require reopening this file.
//!
//! # Implementation
//!
//! This file intentionally contains only module composition and documentation.
//!
//! It must not become a dumping ground for ZQN data structures.
//!
//! Each child module is authoritative for its own implementation.
//!
//! =============================================================================
//! Safety
//! =============================================================================

#![forbid(unsafe_code)]

// =============================================================================
// Core module declarations
// =============================================================================

/// Canonical ZQN error vocabulary.
///
/// This module owns `ZqnError` and `ZqnResult<T>`.
pub mod error;

/// Stable identifiers for ZQN-owned semantic objects.
///
/// This module must consume, rather than redefine, canonical quantum resource
/// identities from `crate::quantum::ir::qubit`.
pub mod ids;

/// Backend-independent ZQN metadata and annotations.
///
/// Metadata must remain semantically non-authoritative.
pub mod metadata;

/// ZQN semantic/schema/serialization compatibility versioning.
pub mod version;

/// Explicit execution and construction context shared by ZQN APIs.
pub mod context;

/// Provider-neutral ZQN capability vocabulary.
pub mod capabilities;

/// Explicit resource, numerical, execution, and security policy limits.
pub mod limits;

/// Scientific and reproducibility provenance.
pub mod provenance;