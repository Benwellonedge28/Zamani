//! Zamani Quantum Noise (ZQN) — Quantum Channel subsystem.
//!
//! This module is the authoritative composition boundary for all
//! representation-independent and concrete quantum-channel functionality
//! provided by ZQN.
//!
//! # Mission
//!
//! The channel subsystem represents quantum processes that may be used to
//! model, analyze, compose, characterize, simulate, propagate, or lower
//! quantum noise.
//!
//! A quantum channel is treated as a semantic process rather than as a
//! particular matrix representation.
//!
//! The subsystem therefore supports multiple mathematical representations,
//! including but not limited to:
//!
//! - Kraus/operator-sum representations;
//! - Choi representations;
//! - process matrices;
//! - Pauli-transfer representations;
//! - stochastic maps;
//! - Lindblad/GKSL generators;
//! - superoperator/Liouville representations;
//! - parameterized/generalized channels;
//! - symbolic or future representations.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                    quantum::frontend
//!                              │
//!                              ▼
//!                         quantum::ir
//!                              │
//!                     canonical semantics
//!                              │
//!                              ▼
//!                    ┌───────────────────┐
//!                    │       ZQN         │
//!                    │                   │
//!                    │ quantum channel   │
//!                    └─────────┬─────────┘
//!                              │
//!             ┌────────────────┼────────────────┐
//!             │                │                │
//!             ▼                ▼                ▼
//!          channel        representation      noise
//!             │                │                │
//!       ┌─────┼──────┐         │                │
//!       ▼     ▼      ▼         ▼                ▼
//!     Kraus  Choi  Lindblad   conversion   noise application
//!       │     │      │         │                │
//!       └─────┴──────┴─────────┴────────────────┘
//!                              │
//!             ┌────────────────┼─────────────────┐
//!             │                │                 │
//!             ▼                ▼                 ▼
//!          simulation      propagation          QEC
//!             │                │                 │
//!             └────────────────┼─────────────────┘
//!                              ▼
//!                    routing / scheduling
//!                              │
//!                              ▼
//!                         hardware/runtime
//! ```
//!
//! The canonical quantum IR remains the semantic owner of the computation.
//! ZQN owns the physical-process/noise-channel representation.
//!
//! # Ownership
//!
//! This composition boundary owns:
//!
//! - the public module hierarchy of the channel subsystem;
//! - the public namespace for channel abstractions;
//! - the public namespace for channel representations;
//! - the public namespace for concrete channel implementations;
//! - documentation of dependency direction;
//! - the stable composition boundary between channel implementations and
//!   downstream ZQN consumers.
//!
//! The child modules own their respective implementations and invariants.
//!
//! # Non-ownership
//!
//! This file does NOT own:
//!
//! - quantum states;
//! - density matrices;
//! - state-vector storage;
//! - canonical quantum IR;
//! - quantum program parsing;
//! - quantum-resource identity;
//! - hardware APIs;
//! - vendor SDKs;
//! - QPU connections;
//! - routing;
//! - scheduling;
//! - QEC decoding;
//! - benchmark orchestration;
//! - calibration storage;
//! - RNG state;
//! - simulation execution;
//! - numerical linear-algebra engines;
//! - serialization wire formats.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Canonical quantum-resource identity
//!
//! This module MUST NOT define another `QubitId` or `PhysicalQubitId`.
//!
//! The repository-wide canonical identities remain:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The channel abstraction already consumes the canonical `QubitId` where a
//! channel is explicitly associated with a qubit resource. Concrete channel
//! implementations likewise use the canonical IR identity. This boundary
//! therefore deliberately does not import or wrap those identities.
//!
//! This preserves the repository rule that no downstream subsystem creates a
//! competing quantum-resource identity.
//!
//! # Write once, scale everywhere
//!
//! The channel subsystem deliberately contains no architectural maximum for:
//!
//! - qubit count;
//! - physical-qubit count;
//! - channel arity;
//! - subsystem count;
//! - Hilbert-space dimension;
//! - Kraus-operator count;
//! - matrix dimension;
//! - tensor dimension;
//! - operation count;
//! - circuit depth;
//! - machine size;
//! - target size;
//! - number of quantum technologies.
//!
//! The absence of an architectural maximum does NOT claim that an individual
//! execution has infinite resources.
//!
//! A concrete realization is necessarily constrained by:
//!
//! - available memory;
//! - address space;
//! - CPU/GPU resources;
//! - distributed resources;
//! - numerical precision;
//! - execution policy;
//! - target capabilities;
//! - caller-selected resource limits.
//!
//! Those are resource/admission constraints, not semantic limitations of the
//! Zamani channel model.
//!
//! In particular, this file MUST NOT introduce constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_CHANNEL_ARITY
//! MAX_MATRIX_SIZE
//! MAX_KRAUS_OPERATORS
//! MAX_CHANNEL_DIMENSION
//! MAX_TENSOR_SIZE
//! ```
//!
//! # Representation independence
//!
//! `channel::QuantumChannel` is the semantic abstraction.
//!
//! Concrete representations implement or support that abstraction without
//! making one numerical representation the universal representation.
//!
//! The intended relationship is:
//!
//! ```text
//! QuantumChannel
//!      │
//!      ▼
//! ChannelRepresentation
//!      │
//! ┌────┼────────┬───────────┬─────────────┐
//! ▼    ▼        ▼           ▼             ▼
//! Kraus Choi ProcessMatrix PauliTransfer Lindblad
//! │    │        │           │             │
//! └────┴────────┴───────────┴─────────────┘
//!                    │
//!                    ▼
//!             execution/analysis
//! ```
//!
//! A caller may select a representation according to:
//!
//! - mathematical requirements;
//! - target capabilities;
//! - numerical requirements;
//! - memory availability;
//! - performance;
//! - exactness requirements;
//! - approximation policy.
//!
//! No representation is implicitly treated as universally optimal.
//!
//! # Exactness and approximation
//!
//! Concrete representations and conversion layers must explicitly distinguish:
//!
//! ```text
//! Exact
//! Approximate
//! Bounded
//! Statistical
//! Unknown
//! ```
//!
//! This composition boundary does not silently convert an approximate channel
//! into an exact one.
//!
//! Conversion between representations belongs to the appropriate child or
//! integration layer and must expose the applicable error/tolerance contract.
//!
//! # Physical validity
//!
//! A channel intended to represent a physical quantum process normally needs
//! to satisfy the relevant physical conditions, including complete positivity
//! and trace preservation where the channel semantics require CPTP behavior.
//!
//! The composition boundary does not perform those mathematical checks itself.
//!
//! Concrete representation modules and the representation-independent channel
//! abstraction own the applicable validation contracts.
//!
//! Invalid, incomplete, symbolic, approximate or otherwise non-physical
//! intermediate objects must never be silently presented as validated physical
//! channels.
//!
//! # Determinism
//!
//! This composition boundary is deterministic.
//!
//! It performs:
//!
//! - no random generation;
//! - no time-based initialization;
//! - no process-dependent initialization;
//! - no thread-dependent initialization;
//! - no device discovery;
//! - no network access;
//! - no global mutable state.
//!
//! A channel description itself is semantic data and therefore contains no
//! hidden RNG.
//!
//! Stochastic realization belongs to the ZQN simulation/runtime boundary,
//! where the caller supplies an explicit reproducibility context.
//!
//! # Parallel reproducibility
//!
//! Concrete stochastic consumers must derive random streams from explicit
//! execution identity rather than relying on a process-global RNG.
//!
//! The intended model is conceptually:
//!
//! ```text
//! master seed
//!     │
//!     ├── program identity
//!     ├── channel identity
//!     ├── operation identity
//!     ├── resource identity
//!     └── shot index
//!             │
//!             ▼
//!       deterministic stream
//! ```
//!
//! Therefore changing execution parallelism must not inherently change the
//! semantic stochastic stream when deterministic execution has been requested.
//!
//! This file does not implement that RNG derivation; it merely preserves the
//! boundary that channel semantics do not own randomness.
//!
//! # Resource safety
//!
//! Importing this module performs no channel allocation.
//!
//! This file:
//!
//! - performs no matrix allocation;
//! - performs no tensor allocation;
//! - performs no state-vector allocation;
//! - performs no I/O;
//! - performs no network access;
//! - performs no device initialization;
//! - performs no dynamic code loading;
//! - owns no global mutable state;
//! - imposes no machine-size ceiling.
//!
//! Concrete implementations are responsible for using checked arithmetic and
//! caller-controlled resource policies before materializing potentially large
//! numerical objects.
//!
//! # Security
//!
//! Channel specifications may eventually originate from:
//!
//! - Zamani source programs;
//! - serialized ZQN models;
//! - calibration data;
//! - characterization results;
//! - external tools;
//! - hardware providers;
//! - user-generated configuration.
//!
//! No channel module may interpret channel metadata as executable code.
//!
//! Untrusted channel descriptions must be validated before expensive numerical
//! realization.
//!
//! Potential denial-of-service conditions include:
//!
//! - enormous dimensions;
//! - enormous operator counts;
//! - pathological tensor shapes;
//! - excessive composition depth;
//! - malicious symbolic expressions;
//! - numerical overflow;
//! - NaN/Infinity injection;
//! - allocation amplification.
//!
//! Resource admission belongs to explicit ZQN/runtime limits rather than
//! hard-coded semantic constants in this composition boundary.
//!
//! # Numerical safety
//!
//! This module performs no numerical calculations.
//!
//! Child implementations must:
//!
//! - reject non-finite semantic parameters where finiteness is required;
//! - use checked structural arithmetic;
//! - avoid silent overflow;
//! - avoid silently converting invalid values into valid-looking values;
//! - expose approximation explicitly;
//! - preserve declared tolerances and error bounds.
//!
//! In particular, invalid values must not be silently repaired by operations
//! such as:
//!
//! ```text
//! NaN → 0
//! negative probability → absolute value
//! Infinity → maximum finite value
//! invalid dimension → 1
//! ```
//!
//! # Thread safety
//!
//! This module owns no mutable state and therefore has no synchronization
//! requirements.
//!
//! Concrete channel types are expected to be `Send + Sync` when their contained
//! representations permit it. The representation-independent `QuantumChannel`
//! contract already establishes the appropriate concurrency boundary.
//!
//! # Integration with canonical IR
//!
//! The dependency direction is:
//!
//! ```text
//! crate::quantum::ir
//!          │
//!          ▼
//!   semantic operation
//!          │
//!          ▼
//! zqn::channel::channel
//!          │
//!          ▼
//! concrete channel representation
//! ```
//!
//! ZQN must not replace or duplicate the canonical IR.
//!
//! The IR answers:
//!
//! > What computation does the program mean?
//!
//! The channel subsystem answers:
//!
//! > What quantum process/noise channel acts on the relevant resources?
//!
//! This separation permits the same Zamani program to be realized against
//! different target machines without rewriting the program merely because
//! target size, topology or technology changes.
//!
//! # Integration with `quantum::ir::qubit`
//!
//! Concrete channel implementations that need a qubit identity already use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! This module does not redefine that type and does not provide a convenience
//! replacement.
//!
//! Physical-resource identity remains owned by the IR/hardware boundary as
//! established by the repository architecture.
//!
//! # Integration with noise
//!
//! The channel subsystem provides mathematical process semantics to the wider
//! ZQN noise layer:
//!
//! ```text
//! noise model
//!      │
//!      ▼
//! channel specification
//!      │
//!      ▼
//! channel realization
//!      │
//!      ├──► simulation
//!      ├──► propagation
//!      ├──► QEC
//!      ├──► routing
//!      ├──► scheduling
//!      └──► hardware/runtime
//! ```
//!
//! The channel subsystem must not become a second noise-model registry.
//!
//! `zqn::noise` owns noise-model semantics and application policy; this module
//! owns channel mathematics and representation boundaries.
//!
//! # Integration with simulation
//!
//! Simulation consumes channels.
//!
//! ```text
//! channel
//!    │
//!    ▼
//! simulation engine
//!    │
//!    ├── deterministic realization
//!    ├── stochastic realization
//!    ├── trajectory realization
//!    └── Monte Carlo realization
//! ```
//!
//! Simulation must not redefine the channel semantics merely to execute them.
//!
//! Channel definitions must remain reusable by multiple simulation strategies.
//!
//! # Integration with propagation
//!
//! Propagation consumes channel semantics to determine effects such as:
//!
//! - fidelity loss;
//! - uncertainty;
//! - error accumulation;
//! - sensitivity;
//! - error budgets;
//! - approximation consequences.
//!
//! Propagation must not create an incompatible channel abstraction.
//!
//! # Integration with QEC
//!
//! QEC consumes channel/fault information to model physical error processes and
//! derive logical consequences.
//!
//! The intended direction is:
//!
//! ```text
//! ZQN channel
//!      │
//!      ▼
//! QEC adapter
//!      │
//!      ▼
//! physical fault / syndrome model
//! ```
//!
//! QEC owns:
//!
//! - codes;
//! - encodings;
//! - syndrome extraction;
//! - decoding;
//! - correction;
//! - logical-fault analysis.
//!
//! ZQN channel semantics remain outside those responsibilities.
//!
//! # Integration with routing
//!
//! Routing may consume channel-derived costs such as:
//!
//! - gate error;
//! - channel fidelity;
//! - crosstalk impact;
//! - correlated-error cost;
//! - operation duration consequences.
//!
//! Routing owns placement and connectivity decisions.
//!
//! It must not redefine the underlying channel mathematics.
//!
//! # Integration with scheduling
//!
//! Scheduling may query channel/noise models for duration-dependent effects,
//! including idle-channel consequences and time-dependent decoherence.
//!
//! Scheduling owns temporal ordering.
//!
//! Channel semantics remain owned here and by their concrete channel modules.
//!
//! # Integration with hardware
//!
//! Hardware providers must not be embedded in this namespace.
//!
//! The intended relationship is:
//!
//! ```text
//! hardware capabilities
//!          │
//!          ▼
//! channel compatibility
//!          │
//!          ▼
//! channel realization/lowering
//!          │
//!          ▼
//! hardware/runtime
//! ```
//!
//! This module must never contain vendor-specific modules such as:
//!
//! ```text
//! ibm.rs
//! ionq.rs
//! rigetti.rs
//! quantinuum.rs
//! aws.rs
//! ```
//!
//! Vendor/provider behavior belongs in the hardware subsystem.
//!
//! # Integration with calibration
//!
//! Calibration supplies measured parameters to channel/noise models.
//!
//! The intended direction is:
//!
//! ```text
//! calibration snapshot
//!          │
//!          ▼
//! channel parameters
//!          │
//!          ▼
//! channel realization
//! ```
//!
//! This module does not own calibration snapshots or calibration lifecycle.
//!
//! # Integration with characterization
//!
//! Characterization experiments estimate channel parameters or channel
//! properties from observations.
//!
//! The intended direction is:
//!
//! ```text
//! experiment
//!    │
//!    ▼
//! observations
//!    │
//!    ▼
//! characterization
//!    │
//!    ▼
//! channel/noise model
//! ```
//!
//! Characterization methodology remains outside this composition boundary.
//!
//! # Integration with benchmarking
//!
//! Benchmarking consumes channel behavior and characterization results.
//!
//! The channel subsystem must not depend on benchmark orchestration.
//!
//! ```text
//! channel ────────────► benchmarking
//! characterization ───► benchmarking
//! execution ──────────► benchmarking
//! ```
//!
//! The reverse dependency is forbidden.
//!
//! # Integration with serialization
//!
//! This module does not define a wire format.
//!
//! The versioned ZQN serialization boundary belongs under:
//!
//! ```text
//! crate::quantum::zqn::io
//! ```
//!
//! Rust memory layout and Rust enum discriminants are not external schema
//! contracts.
//!
//! Any serialization implementation must use an explicit versioned schema and
//! compatibility policy.
//!
//! # Stable module paths
//!
//! The authoritative child modules are exposed through explicit paths:
//!
//! ```text
//! crate::quantum::zqn::channel::channel
//! crate::quantum::zqn::channel::representation
//! crate::quantum::zqn::channel::kraus
//! crate::quantum::zqn::channel::choi
//! crate::quantum::zqn::channel::process_matrix
//! crate::quantum::zqn::channel::pauli
//! crate::quantum::zqn::channel::stochastic
//! crate::quantum::zqn::channel::lindblad
//! crate::quantum::zqn::channel::thermal
//! crate::quantum::zqn::channel::amplitude
//! crate::quantum::zqn::channel::phase
//! crate::quantum::zqn::channel::depolarizing
//! crate::quantum::zqn::channel::generalized
//! crate::quantum::zqn::channel::composition
//! ```
//!
//! These paths correspond to the independently owned implementations.
//!
//! This composition boundary deliberately does not duplicate their types.
//!
//! # Why concrete types are not re-exported here
//!
//! The existing repository code already uses explicit child-module paths such
//! as:
//!
//! ```text
//! crate::quantum::zqn::channel::channel::ChannelError
//! crate::quantum::zqn::channel::channel::ChannelDescriptor
//! crate::quantum::zqn::channel::kraus::KrausChannel
//! ```
//!
//! Keeping those paths authoritative avoids creating a second public ownership
//! layer and minimizes coupling between this composition root and the internal
//! APIs of individual channel implementations.
//!
//! Consumers that want a stable high-level API should use the dedicated ZQN
//! prelude once that public API has been finalized.
//!
//! # Module dependency graph
//!
//! The intended dependency structure is:
//!
//! ```text
//!                         channel/mod.rs
//!                              │
//!             ┌────────────────┼────────────────┐
//!             │                │                │
//!             ▼                ▼                ▼
//!         channel       representation     composition
//!             │                │                │
//!             ├──────┬─────────┼───────┬────────┤
//!             │      │         │       │        │
//!             ▼      ▼         ▼       ▼        ▼
//!           kraus  choi   process_matrix pauli stochastic
//!             │      │         │       │        │
//!             └──────┴─────────┼───────┴────────┘
//!                              │
//!                 ┌────────────┼─────────────┐
//!                 ▼            ▼             ▼
//!             lindblad      thermal       generalized
//!                 │            │             │
//!                 └────────────┼─────────────┘
//!                              │
//!                    specialized channels
//!                              │
//!                 ┌────────────┴────────────┐
//!                 ▼                         ▼
//!             amplitude                    phase
//!                 │                         │
//!                 └────────────┬────────────┘
//!                              ▼
//!                         depolarizing
//! ```
//!
//! The exact implementation dependency direction remains owned by each child
//! module. This file only declares the module boundaries.
//!
//! # Addition of future representations
//!
//! New channel representations must be added as explicit modules.
//!
//! For example:
//!
//! ```text
//! channel/new_representation.rs
//! ```
//!
//! followed by one declaration:
//!
//! ```text
//! pub mod new_representation;
//! ```
//!
//! The existing representation-independent abstraction must not be silently
//! repurposed to mean the new representation.
//!
//! Adding a representation is therefore an explicit API change with its own:
//!
//! - mathematical semantics;
//! - validation contract;
//! - conversion contract;
//! - exactness contract;
//! - resource contract;
//! - serialization contract;
//! - tests.
//!
//! # No hard-coded hardware assumptions
//!
//! This module intentionally contains no:
//!
//! ```text
//! MAX_QUBITS
//! DEFAULT_QUBIT_COUNT
//! DEFAULT_GATE_SET
//! DEFAULT_TOPOLOGY
//! IBM_*
//! IONQ_*
//! RIGETTI_*
//! QUANTINUUM_*
//! ```
//!
//! Channel dimensions, supports, resources and target capabilities must always
//! be supplied by the applicable semantic or target layer.
//!
//! # No hidden allocation
//!
//! Merely importing `zqn::channel` must never construct:
//!
//! - a matrix;
//! - a tensor;
//! - a state vector;
//! - a density matrix;
//! - a Kraus operator list;
//! - a simulator;
//! - a hardware connection.
//!
//! Concrete constructors own allocation decisions and must respect caller
//! resource policies.
//!
//! # No unsafe Rust
//!
//! The entire ZQN channel subsystem is required to remain safe Rust.
//!
//! This composition boundary explicitly forbids unsafe code.
//!
//! No FFI, raw pointer manipulation, backend memory access or unchecked unsafe
//! abstraction is permitted here.
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
//! - no unsafe.
//!
//! The implementation intentionally uses only stable module declarations.
//!
//! # Testing strategy
//!
//! Mathematical correctness belongs to the individual child modules.
//!
//! This composition boundary is intentionally lightweight because adding tests
//! that duplicate child implementation details would make unrelated future
//! changes unnecessarily fragile.
//!
//! The required testing layers are:
//!
//! ```text
//! channel child tests
//!       │
//!       ├── mathematical invariants
//!       ├── physical validity
//!       ├── numerical stability
//!       ├── conversion correctness
//!       ├── deterministic behavior
//!       └── resource-limit behavior
//!
//! composition tests
//!       │
//!       ├── module visibility
//!       ├── module path stability
//!       └── feature integration
//! ```
//!
//! The normal Rust compiler is the primary verification mechanism for the
//! declarations in this file.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! 1. every declared child module exists;
//! 2. every child module owns its own implementation;
//! 3. no implementation is duplicated here;
//! 4. canonical `quantum::ir::qubit::QubitId` remains authoritative;
//! 5. no second `QubitId` or `PhysicalQubitId` exists here;
//! 6. no vendor API exists here;
//! 7. no hardware initialization occurs here;
//! 8. no RNG exists here;
//! 9. no global mutable state exists here;
//! 10. no machine-size maximum exists here;
//! 11. no channel-arity maximum exists here;
//! 12. no matrix-size maximum exists here;
//! 13. no numerical representation is privileged as universally canonical;
//! 14. exactness and approximation remain explicit in the child contracts;
//! 15. serialization remains outside this composition boundary;
//! 16. routing, scheduling, QEC and benchmarking remain downstream consumers;
//! 17. simulation remains responsible for stochastic execution;
//! 18. calibration remains responsible for calibration lifecycle;
//! 19. characterization remains responsible for measurement/estimation;
//! 20. the module compiles on Rust 1.97/1.97.1;
//! 21. the module uses no unsafe code;
//! 22. adding a larger quantum machine does not require modifying this file;
//! 23. adding a new channel representation requires only an explicit new module
//!     declaration plus that representation's independently defined contract;
//! 24. existing child implementations can continue using their authoritative
//!     nested module paths without requiring this file to be rewritten.
//!
//! # Public API policy
//!
//! The public API is intentionally exposed through child namespaces rather than
//! broad wildcard re-exports.
//!
//! This avoids accidental API commitments and keeps ownership explicit:
//!
//! ```text
//! channel::QuantumChannel
//! channel::ChannelError
//! channel::ChannelDescriptor
//!
//! representation::ChannelRepresentationKind
//! representation::ChannelDimensions
//!
//! kraus::KrausChannel
//! choi::Choi*
//! process_matrix::ProcessMatrix
//! stochastic::StochasticChannel
//! lindblad::LindbladGenerator
//! thermal::ThermalChannel
//! amplitude::AmplitudeDamping
//! phase::PhaseDampingChannel
//! depolarizing::DepolarizingChannel
//! generalized::...
//! composition::...
//! ```
//!
//! The child modules remain the authoritative owners of those types.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Representation-independent channel abstraction
// =============================================================================

/// Representation-independent quantum-channel semantics.
///
/// This module owns the fundamental channel trait, channel descriptors,
// identity, support metadata, capability metadata and channel-level errors.
pub mod channel;

// =============================================================================
// Representation contracts
// =============================================================================

/// Mathematical representation metadata and representation capability
/// contracts.
///
/// This module deliberately does not materialize matrices or tensors.
pub mod representation;

// =============================================================================
// Concrete mathematical representations
// =============================================================================

/// Kraus/operator-sum channel representation.
pub mod kraus;

/// Choi-Jamiołkowski channel representation.
pub mod choi;

/// General process-matrix representation.
pub mod process_matrix;

/// Pauli-transfer / Pauli-Liouville channel representation.
pub mod pauli;

/// Classical/stochastic channel representation.
pub mod stochastic;

/// Lindblad/GKSL generator representation.
pub mod lindblad;

// =============================================================================
// Generic and specialized channel semantics
// =============================================================================

/// Representation-independent thermalization channel semantics.
pub mod thermal;

/// Amplitude-damping channel semantics.
pub mod amplitude;

/// Phase-damping/dephasing channel semantics.
pub mod phase;

/// Depolarizing-channel semantics.
pub mod depolarizing;

/// Generalized channel-family semantics.
pub mod generalized;

// =============================================================================
// Channel composition
// =============================================================================

/// Representation-independent channel composition and tensor-product
/// composition semantics.
pub mod composition;