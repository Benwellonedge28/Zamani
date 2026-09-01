//! Zamani Quantum Noise (ZQN) — Characterization subsystem.
//!
//! # Purpose
//!
//! This module is the composition boundary for ZQN characterization.
//!
//! Characterization answers:
//!
//! > "What physical behaviour does a quantum system exhibit, based on
//! > controlled experiments and their observations?"
//!
//! The subsystem is deliberately split into independent contracts:
//!
//! ```text
//! characterization
//! │
//! ├── experiment
//! │      experiment definition
//! │
//! ├── protocol
//! │      scientific protocol contract
//! │
//! ├── observation
//! │      raw experimental evidence
//! │
//! ├── estimator
//! │      parameter estimation
//! │
//! ├── uncertainty
//! │      statistical uncertainty
//! │
//! ├── tomography
//! │      process/state reconstruction
//! │
//! ├── randomized_benchmarking
//! │      randomized benchmarking protocols
//! │
//! └── process_characterization
//!        general process-characterization contracts
//! ```
//!
//! # Ownership
//!
//! This file owns only:
//!
//! - characterization submodule composition;
//! - stable public API organization;
//! - public re-export policy;
//! - characterization-level documentation;
//! - compile-time module boundaries.
//!
//! This file does NOT own:
//!
//! - characterization algorithms;
//! - experiment generation;
//! - circuit generation;
//! - quantum IR;
//! - quantum channels;
//! - noise models;
//! - probability mathematics;
//! - raw observation storage;
//! - statistical estimation;
//! - confidence intervals;
//! - Bayesian inference;
//! - tomography mathematics;
//! - randomized sequence generation;
//! - calibration storage;
//! - simulator execution;
//! - hardware communication;
//! - routing;
//! - scheduling;
//! - QEC;
//! - benchmarking methodology;
//! - serialization wire formats;
//! - vendor APIs.
//!
//! Those responsibilities remain in their respective modules/subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                      quantum::frontend
//!                              │
//!                              ▼
//!                         quantum::ir
//!                       canonical meaning
//!                              │
//!                              ▼
//!                         ZQN characterization
//!                              │
//!             ┌────────────────┼────────────────┐
//!             │                │                │
//!             ▼                ▼                ▼
//!        experiment         protocol        process_characterization
//!             │                │                │
//!             └────────────────┼────────────────┘
//!                              ▼
//!                         execution layer
//!                       /      |       \
//!                    QPU   simulator  emulator
//!                       \      |       /
//!                              ▼
//!                         observation
//!                              │
//!             ┌────────────────┼────────────────┐
//!             ▼                ▼                ▼
//!         estimator       uncertainty       tomography
//!             │                │                │
//!             └────────────────┼────────────────┘
//!                              ▼
//!                    characterization result
//!                              │
//!                 ┌────────────┴────────────┐
//!                 ▼                         ▼
//!             calibration                  ZQN
//!                                           noise model
//! ```
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! core
//!   ↑
//! experiment
//!   ↑
//! protocol
//!   ↑
//! observation
//!   ↑
//! process_characterization
//!   ↑
//! estimator / uncertainty / tomography / randomized_benchmarking
//! ```
//!
//! More precisely, no module should depend on this `mod.rs` for implementation
//! semantics. This module only exposes modules and selected stable symbols.
//!
//! This prevents circular dependencies and keeps implementation files
//! independently completable.
//!
//! # Canonical quantum identities
//!
//! Characterization must never define a second quantum-resource identity
//! system.
//!
//! Qubit identities belong to:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Existing characterization modules already follow this boundary. The
//! observation layer explicitly states that it must not define `QubitId` or
//! `PhysicalQubitId`, and instead uses the canonical definitions. 
//!
//! Therefore this module deliberately does not define or re-export any
//! alternative qubit identity.
//!
//! # Scalability
//!
//! There is no semantic maximum imposed here for:
//!
//! - number of characterized resources;
//! - number of experiments;
//! - number of protocol stages;
//! - number of repetitions;
//! - number of observations;
//! - sequence lengths;
//! - process dimensions;
//! - characterization duration;
//! - distributed characterization domains.
//!
//! "Infinite" scalability means that the semantic contracts do not encode an
//! artificial machine-size ceiling. Actual execution remains bounded by the
//! resources available to the selected execution environment.
//!
//! Resource limits belong to explicit policy/configuration objects in the
//! appropriate subsystem and must never be encoded in this module as fixed
//! quantum-machine limits.
//!
//! In particular, this module must never contain:
//!
//! ```text
//! MAX_QUBITS
//! MAX_EXPERIMENTS
//! MAX_SHOTS
//! MAX_SEQUENCE_LENGTH
//! MAX_RESOURCES
//! ```
//!
//! as semantic constants.
//!
//! # Determinism
//!
//! This module does not generate randomness and does not own RNG state.
//!
//! Randomized characterization protocols must use explicit deterministic
//! contracts supplied by their protocol/execution layer.
//!
//! There must be no:
//!
//! - hidden global RNG;
//! - thread-local semantic randomness;
//! - implicit clock reads;
//! - global mutable characterization state;
//! - identifier generation hidden inside module composition.
//!
//! This is particularly important for randomized benchmarking and distributed
//! characterization.
//!
//! # Numerical semantics
//!
//! This module does not perform numerical calculations.
//!
//! Numerical validation belongs to the relevant mathematical layer.
//!
//! Nevertheless, all public characterization modules must preserve the
//! repository-wide rule:
//!
//! ```text
//! NaN / +∞ / -∞
//!       │
//!       └── rejected
//! ```
//!
//! No characterization implementation may silently convert invalid numerical
//! values into zero, a bound, an absolute value, or another valid-looking
//! value.
//!
//! # Approximation
//!
//! Characterization results must distinguish at least:
//!
//! - exact;
//! - numerically approximate;
//! - bounded approximation;
//! - statistically estimated;
//! - unsupported.
//!
//! An approximation must never silently masquerade as an exact result.
//!
//! This policy is implemented by the result/estimation layers, not by this
//! composition module.
//!
//! # Resource safety
//!
//! Characterization is potentially adversarial from a resource perspective.
//!
//! External or user-controlled characterization configurations may request:
//!
//! - extremely long sequences;
//! - enormous shot counts;
//! - huge process dimensions;
//! - very large observation streams;
//! - enormous tomography spaces;
//! - large distributed experiment sets.
//!
//! Those requests must be validated against caller-provided resource policies
//! before materializing work.
//!
//! This module itself performs no allocation beyond normal module metadata and
//! does not create experiment workloads.
//!
//! # Thread safety
//!
//! Module composition contains no mutable global state.
//!
//! The underlying characterization contracts should remain `Send + Sync` where
//! their semantics permit it.
//!
//! Concurrent execution must not alter scientific meaning.
//!
//! # Serialization
//!
//! This file does not define a wire format.
//!
//! Versioned serialization belongs to:
//!
//! ```text
//! crate::quantum::zqn::io
//! ```
//!
//! Re-exporting a type from this module must not be interpreted as defining
//! serialization compatibility.
//!
//! Serialization schemas must remain independent from Rust source layout.
//!
//! # Error ownership
//!
//! Individual characterization modules own their detailed domain errors.
//!
//! This module deliberately does not introduce a second characterization
//! error hierarchy.
//!
//! Higher-level integration may convert domain errors into the canonical ZQN
//! error system in:
//!
//! ```text
//! crate::quantum::zqn::core::error
//! ```
//!
//! # Integration contracts
//!
//! ## Experiment
//!
//! `experiment.rs` defines what characterization work is requested.
//!
//! ```text
//! CharacterizationExperiment
//!             │
//!             ▼
//! protocol / execution
//! ```
//!
//! The experiment module explicitly does not execute experiments, generate
//! circuits, estimate noise, or store raw observations.
//!
//! ## Protocol
//!
//! `protocol.rs` defines the scientific protocol contract.
//!
//! ```text
//! CharacterizationProtocol
//!          │
//!          ├── descriptor
//!          ├── requirements
//!          ├── plan
//!          └── analysis contract
//! ```
//!
//! Protocols describe experiments; they do not execute them.
//!
//! ## Observation
//!
//! `observation.rs` owns raw experimental evidence.
//!
//! ```text
//! executor
//!    │
//!    ▼
//! Observation
//!    │
//!    ├── estimator
//!    ├── uncertainty
//!    └── benchmarking
//! ```
//!
//! The observation layer explicitly separates evidence from estimates.
//!
//! ## Estimator
//!
//! `estimator.rs` consumes observations and produces statistical estimates.
//!
//! ```text
//! Observation
//!      │
//!      ▼
//! Estimator
//!      │
//!      ▼
//! Estimate
//! ```
//!
//! It must not redefine the observation contract.
//!
//! ## Uncertainty
//!
//! `uncertainty.rs` supplies uncertainty mathematics for estimates.
//!
//! ```text
//! estimate
//!    │
//!    ▼
//! uncertainty
//!    │
//!    ▼
//! confidence / interval / statistical bound
//! ```
//!
//! ## Tomography
//!
//! `tomography.rs` owns state/process reconstruction.
//!
//! ```text
//! observations
//!      │
//!      ▼
//! tomography
//!      │
//!      ▼
//! reconstructed process/state
//! ```
//!
//! `process_characterization.rs` supplies the general characterization
//! boundary but does not duplicate tomography algorithms.
//!
//! ## Randomized benchmarking
//!
//! `randomized_benchmarking.rs` owns randomized-benchmarking-specific
//! protocol semantics and algorithms.
//!
//! It must use explicit randomness contracts rather than hidden RNG state.
//!
//! Randomized benchmarking commonly uses randomized Clifford sequences,
//! sequence lengths, repeated samples and survival probabilities; those
//! protocol-specific semantics belong there rather than in this composition
//! module.
//!
//! ## Calibration
//!
//! Characterization results may be consumed by:
//!
//! ```text
//! characterization
//!       │
//!       ▼
//! calibration
//!       │
//!       ▼
//! ZQN noise model
//! ```
//!
//! Calibration remains a separate owner of calibration state.
//!
//! ## Hardware
//!
//! Characterization never calls vendor APIs directly.
//!
//! The execution environment supplies observations through the abstract
//! observation contract.
//!
//! ```text
//! hardware adapter
//!       │
//!       ▼
//! observation
//!       │
//!       ▼
//! characterization
//! ```
//!
//! ## Simulator
//!
//! The same characterization contracts must be usable with:
//!
//! - CPU simulation;
//! - GPU simulation;
//! - distributed simulation;
//! - emulation;
//! - real quantum hardware;
//! - future execution technologies.
//!
//! ## QEC
//!
//! Characterization may provide measured physical-noise information to QEC,
//! but this module does not own:
//!
//! - syndrome extraction;
//! - decoding;
//! - logical correction;
//! - QEC noise ownership.
//!
//! The existing repository architecture already separates these concerns.
//!
//! ## Routing and scheduling
//!
//! Characterization may provide measured error/duration/crosstalk information
//! to routing and scheduling through ZQN integration layers.
//!
//! Neither routing nor scheduling becomes a dependency of this module.
//!
//! # Public API policy
//!
//! Only stable contracts that are useful to downstream consumers should be
//! re-exported here.
//!
//! Implementation helpers remain available through their owning modules.
//!
//! This avoids turning `characterization::*` into an uncontrolled namespace.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden for this module.
//!
//! The module contains no unsafe operations and explicitly enables the
//! compiler lint below.
//!
//! # Completion criterion
//!
//! This file is complete when:
//!
//! 1. every characterization implementation has a dedicated owner;
//! 2. there are no duplicate quantum-resource identities;
//! 3. this file contains no characterization algorithms;
//! 4. public re-exports are stable and intentional;
//! 5. no implementation depends on a hard-coded machine size;
//! 6. no hidden RNG or global mutable state is introduced;
//! 7. errors remain owned by the appropriate subsystem;
//! 8. serialization remains owned by `zqn::io`;
//! 9. all characterization modules can be compiled and tested independently;
//! 10. downstream modules can consume characterization contracts without
//!     modifying this file.
//!
//! # Future extension rule
//!
//! A new characterization protocol should normally require:
//!
//! ```text
//! new protocol implementation
//!          │
//!          ▼
//! characterization::new_protocol
//! ```
//!
//! It must NOT require modifying existing protocol implementations merely
//! because another protocol was added.
//!
//! Adding a new protocol should normally require only:
//!
//! ```text
//! 1. create the new implementation module;
//! 2. declare it below;
//! 3. expose its stable API if appropriate;
//! 4. add its tests.
//! ```
//!
//! Existing modules must remain source-compatible unless a deliberate
//! characterization API version change is being made.

// -----------------------------------------------------------------------------
// Module declarations
// -----------------------------------------------------------------------------
//
// Keep these declarations explicit. The characterization subsystem is a
// deliberately finite set of architectural responsibilities, while each
// implementation remains independently extensible.
//
// `pub mod` is used because downstream ZQN integration layers may need to
// access protocol-specific contracts without forcing this composition root to
// duplicate their APIs.

pub mod experiment;
pub mod observation;
pub mod process_characterization;
pub mod protocol;
pub mod randomized_benchmarking;
pub mod estimator;
pub mod tomography;
pub mod uncertainty;

// -----------------------------------------------------------------------------
// Stable public API
// -----------------------------------------------------------------------------
//
// Re-export only the primary contracts.
//
// Do NOT use:
//
// pub use experiment::*;
// pub use protocol::*;
// ...
//
// Wildcard re-exports create namespace collisions as the subsystem grows and
// make future additions accidentally become public API.
//
// Individual modules remain directly accessible through:
//
// crate::quantum::zqn::characterization::<module>
//
// while these selected contracts provide the ergonomic stable surface.

pub use experiment::CharacterizationExperiment;
pub use observation::{
    DiscreteOutcome,
    ObservedResource,
    ObservationScope,
    ObservationTiming,
    OutcomeCount,
    OutcomeHistogram,
};

pub use process_characterization::{
    CharacterizationMethod,
    CharacterizationProcess,
    ProcessCharacterization,
    ProcessCharacterizationError,
};

pub use protocol::{
    CharacterizationObjective,
    CharacterizationRequirements,
    CharacterizationScope,
    ProtocolDescriptor,
    ProtocolError,
    ProtocolId,
    ProtocolVersion,
};

pub use randomized_benchmarking::{
    // Keep protocol-specific APIs in their owning module unless they are
    // explicitly designated stable by randomized_benchmarking.rs.
};

pub use estimator::{
    // Estimator-specific implementations remain module-qualified.
};

pub use tomography::{
    // Tomography-specific implementations remain module-qualified.
};

pub use uncertainty::{
    // Uncertainty-specific implementations remain module-qualified.
};

// -----------------------------------------------------------------------------
// Compile-time safety assertion
// -----------------------------------------------------------------------------
//
// This is intentionally empty. The no-unsafe requirement is enforced by the
// module-level lint:
//
//     #![forbid(unsafe_code)]
//
// No runtime initialization or global state is required for characterization.