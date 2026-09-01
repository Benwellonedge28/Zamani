//! Zamani Quantum Noise (ZQN) — Probability subsystem.
//!
//! This module is the authoritative composition boundary for probability
//! semantics used by ZQN. It exposes the probability primitives without
//! coupling the mathematical probability layer to quantum hardware, execution,
//! routing, scheduling, QEC, or a particular quantum technology.
//!
//! # Ownership
//!
//! The probability subsystem owns:
//!
//! - scalar probabilities in the closed interval `[0, 1]`;
//! - deterministic probability bounds;
//! - finite categorical distributions;
//! - finite discrete distributions and their operations;
//! - continuous scalar distributions;
//! - descriptive probability statistics;
//! - validation contracts for these mathematical objects;
//! - public composition/re-export boundaries for the subsystem.
//!
//! It does NOT own:
//!
//! - quantum states or state-vector/density-matrix storage;
//! - canonical quantum IR;
//! - `QubitId` or `PhysicalQubitId` definitions;
//! - quantum channels;
//! - faults;
//! - noise models;
//! - calibration;
//! - characterization protocols;
//! - simulation engines;
//! - hardware providers;
//! - routing or scheduling;
//! - benchmark orchestration;
//! - external serialization schemas;
//! - RNG ownership or global randomness.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Canonical quantum identity boundary
//!
//! Probability objects are intentionally resource-agnostic. This module must
//! not define a second qubit identity type.
//!
//! When a probability is associated with a quantum resource, the owning
//! higher-level type must use the canonical identities from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! In particular, do not add `QubitId`, `PhysicalQubitId`, or a ZQN-specific
//! replacement to this module merely to make probability APIs convenient.
//!
//! The existing probability implementation correctly keeps scalar and
//! distribution mathematics independent of quantum-resource identity. That
//! separation is preserved here.
//!
//! # Write once, scale everywhere
//!
//! The probability subsystem imposes no semantic limit on:
//!
//! - qubit count;
//! - resource count;
//! - operation count;
//! - circuit depth;
//! - distribution cardinality;
//! - number of observations;
//! - number of machines;
//! - quantum technology.
//!
//! Collection implementations necessarily use Rust collection indices and
//! therefore are ultimately constrained by available addressable resources.
//! That is an implementation/resource constraint, not a ZQN semantic machine
//! size limit.
//!
//! No `MAX_QUBITS`, `MAX_OUTCOMES`, `MAX_CATEGORIES`, `MAX_DISTRIBUTIONS`, or
//! similar architectural ceiling belongs in this module.
//!
//! "Infinity" means that no artificial finite quantum-machine-size ceiling is
//! encoded in the API. An individual execution remains bounded by available
//! memory, address space, CPU/GPU resources, runtime policy, and target
//! capabilities.
//!
//! # Module boundaries
//!
//! ```text
//! probability
//! ├── probability.rs    scalar [0, 1] primitive
//! ├── bounds.rs         deterministic [lower, upper] bounds
//! ├── categorical.rs    ordered finite categorical distribution
//! ├── distribution.rs   generic finite discrete distribution
//! ├── continuous.rs     continuous scalar distributions
//! └── statistics.rs     streaming/descriptive statistics
//! ```
//!
//! The dependency direction is deliberately one-way:
//!
//! ```text
//! probability.rs
//!      │
//!      ├──────────────► bounds.rs
//!      │
//!      ├──────────────► categorical.rs
//!      │
//!      ├──────────────► distribution.rs
//!      │                    │
//!      │                    ▼
//!      │               statistics.rs
//!      │
//!      └──────────────► continuous.rs
//! ```
//!
//! The module declaration itself contains no mathematical implementation. The
//! child files own their contracts and can therefore be completed and tested
//! independently before this composition boundary is integrated.
//!
//! # Stable public API
//!
//! The child modules remain available through explicit paths such as:
//!
//! ```text
//! crate::quantum::zqn::probability::probability::Probability
//! crate::quantum::zqn::probability::bounds::ProbabilityBounds
//! crate::quantum::zqn::probability::categorical::Categorical
//! crate::quantum::zqn::probability::distribution::Distribution
//! crate::quantum::zqn::probability::continuous::Normal
//! crate::quantum::zqn::probability::statistics::WeightedStatistics
//! ```
//!
//! Stable high-frequency mathematical types are also re-exported directly from
//! this module. The re-exports are aliases to the authoritative child
//! implementations; they do not create duplicate types or implementations.
//!
//! # Integration with later ZQN modules
//!
//! The intended dependency direction is:
//!
//! ```text
//! zqn::probability
//!       │
//!       ├──► channel
//!       ├──► fault
//!       ├──► noise
//!       ├──► calibration
//!       ├──► characterization
//!       ├──► simulation
//!       └──► propagation
//! ```
//!
//! Those downstream modules consume probability semantics. This module must
//! not import them merely to provide convenience APIs.
//!
//! `core::limits` may be used by higher-level consumers to establish explicit
//! resource admission policies before constructing large collections. Such
//! policies must not be encoded as semantic limits here.
//!
//! `core::error` may later provide a unified ZQN error boundary. The
//! foundational probability files intentionally retain domain-specific errors
//! so each mathematical file can be developed independently. This composition
//! module therefore does not invent a competing aggregate error type.
//!
//! # Sampling boundary
//!
//! Probability mathematics and random sampling are separate concerns.
//!
//! The probability subsystem must never create a hidden global RNG or seed a
//! generator from time, process identity, thread identity, or memory address.
//!
//! Where a child distribution exposes sampling, the RNG is caller-owned. The
//! runtime/simulation layer is responsible for deterministic RNG-stream
//! derivation, for example from:
//!
//! ```text
//! master seed
//!     + program identity
//!     + operation identity
//!     + resource identity
//!     + shot index
//! ```
//!
//! This permits sequential and parallel execution to share the same explicit
//! reproducibility contract.
//!
//! # Numerical integrity
//!
//! The probability subsystem must reject invalid numerical states rather than
//! silently repair them.
//!
//! In particular:
//!
//! - `NaN` is never a valid probability;
//! - positive/negative infinity is never a probability value;
//! - negative probability is rejected;
//! - probability bounds do not silently swap endpoints;
//! - distributions do not silently turn malformed input into a different
//!   distribution unless that transformation is explicitly part of the named
//!   operation's contract;
//! - statistical calculations must not silently hide materially invalid
//!   numerical results.
//!
//! A PDF is a density, not a probability, so continuous `pdf()` APIs are not
//! constrained to `[0, 1]`. CDF/survival values are probability values and use
//! the canonical `Probability` type where the child contract specifies it.
//!
//! # Determinism
//!
//! Composition through this module is deterministic. No initialization code,
//! global state, device discovery, I/O, or random generation occurs here.
//!
//! The public re-export set is fixed by source code and does not depend on hash
//! iteration order, filesystem traversal, thread scheduling, or runtime state.
//!
//! # Resource safety
//!
//! This module:
//!
//! - performs no unsafe operations;
//! - owns no global mutable state;
//! - performs no I/O;
//! - performs no network access;
//! - performs no dynamic code loading;
//! - performs no device initialization;
//! - imposes no machine-size ceiling;
//! - does not allocate merely by being imported.
//!
//! Collection/resource admission remains the responsibility of constructors
//! and higher-level runtime policies. A module declaration must never guess
//! available memory or CPU capacity.
//!
//! # Serialization
//!
//! This module does not define an external wire format. The versioned ZQN
//! serialization boundary belongs under `crate::quantum::zqn::io`.
//!
//! Re-exporting a type does not make its Rust layout a serialization contract.
//!
//! # Rust compatibility
//!
//! This file targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust only;
//! - no nightly features;
//! - no `unsafe`.
//!
//! The workspace currently declares Rust 1.97.x as its intended toolchain. This
//! module intentionally uses only ordinary Rust module/re-export syntax and
//! therefore does not require a newer language feature.
//!
//! # Integration with `quantum::ir::qubit`
//!
//! No direct import is required by this module because none of the mathematical
//! probability primitives has inherent quantum-resource identity.
//!
//! This is intentional and must remain so. Higher-level objects that attach a
//! probability distribution to a resource must use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This preserves the repository-wide canonical identity boundary established
//! by `quantum::ir`.
//!
//! # Integration with the existing child files
//!
//! The probability directory contains the foundational probability files. This
//! module wires them together without changing their mathematical ownership:
//!
//! - `probability.rs` provides `Probability` and its local error type;
//! - `bounds.rs` consumes `Probability` for validated intervals;
//! - `categorical.rs` provides ordered finite categorical distributions;
//! - `distribution.rs` provides generic finite discrete distributions;
//! - `continuous.rs` provides continuous distribution semantics;
//! - `statistics.rs` consumes distribution/probability semantics for streaming
//!   descriptive statistics.
//!
//! No child implementation is duplicated here.
//!
//! # Definition of done
//!
//! This composition boundary is complete when:
//!
//! 1. every declared child module exists;
//! 2. every child owns its own implementation and local invariants;
//! 3. no implementation is duplicated by this file;
//! 4. public re-exports refer only to existing authoritative types;
//! 5. no quantum identity is redefined;
//! 6. no hardware or vendor assumption is introduced;
//! 7. no semantic machine-size maximum is introduced;
//! 8. no RNG or global mutable state is introduced;
//! 9. no unsafe code is permitted;
//! 10. downstream ZQN modules can depend on this boundary without requiring
//!     this file to be rewritten when unrelated probability internals change;
//! 11. explicit child module paths remain stable for callers that require them;
//! 12. the module compiles on the repository's declared Rust 1.97.x toolchain.
//!
//! # Testing
//!
//! Mathematical tests belong to the child modules. This composition boundary
//! has only integration-level tests verifying that the public re-export surface
//! resolves to the intended modules.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Authoritative child modules
// =============================================================================

/// Scalar probability primitive: finite values in the closed interval `[0, 1]`.
pub mod probability;

/// Deterministic closed probability intervals and bounds.
pub mod bounds;

/// Ordered finite categorical probability distributions.
pub mod categorical;

/// Generic finite discrete probability distributions.
pub mod distribution;

/// Continuous scalar probability distributions.
pub mod continuous;

/// Streaming and descriptive probability statistics.
pub mod statistics;

// =============================================================================
// Stable high-frequency re-exports
// =============================================================================

pub use bounds::ProbabilityBounds;

pub use categorical::{
    Categorical,
    CategoricalEntry,
    CategoricalError,
    DEFAULT_NORMALIZATION_TOLERANCE,
};

pub use continuous::{
    ContinuousDistribution,
    ContinuousDistributionError,
    ContinuousResult,
    Exponential,
    LogNormal,
    Normal,
    Support,
    Uniform,
};

pub use distribution::{
    Distribution,
    DistributionCount,
    DistributionError,
    ProbabilityWeight,
};

pub use probability::{
    Probability,
    ProbabilityError,
    MAX_PROBABILITY,
    MIN_PROBABILITY,
};

pub use statistics::{
    DistributionStatistics,
    ObservationCount,
    StatisticsError,
    WeightedStatistics,
};