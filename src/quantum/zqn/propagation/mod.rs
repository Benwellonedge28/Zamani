//! Zamani Quantum Noise (ZQN) — Propagation
//!
//! Path:
//!     src/quantum/zqn/propagation/mod.rs
//!
//! # Purpose
//!
//! This module is the public composition boundary for ZQN propagation
//! analysis.
//!
//! Propagation answers questions such as:
//!
//! - How does uncertainty propagate through a computation?
//! - How does an error contribution accumulate?
//! - How sensitive is an observable to a physical parameter?
//! - How does fidelity or another distance/error measure change?
//! - What bounds can be established on propagated error?
//!
//! The individual propagation algorithms belong in their dedicated modules.
//! This file owns only the propagation namespace, module graph, public
//! re-exports, and architectural contracts.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              |
//!                              v
//!                         quantum::ir
//!                              |
//!               +--------------+--------------+
//!               |              |              |
//!               v              v              v
//!           algorithms     optimization    execution
//!               |              |
//!               +-------+------+
//!                       |
//!                       v
//!                     ZQN
//!                       |
//!          +------------+-------------+
//!          |            |             |
//!          v            v             v
//!      noise model   calibration   characterization
//!          |            |             |
//!          +------------+-------------+
//!                       |
//!                       v
//!                 propagation
//!                       |
//!       +---------------+----------------+
//!       |               |                |
//!       v               v                v
//!   uncertainty     sensitivity      error budget
//!       |               |                |
//!       +---------------+----------------+
//!                       |
//!                       v
//!                fidelity / analysis
//!                       |
//!            +----------+----------+
//!            |          |          |
//!            v          v          v
//!         routing   scheduling     QEC
//!            |          |          |
//!            +----------+----------+
//!                       |
//!                       v
//!                    runtime
//! ```
//!
//! # Ownership
//!
//! This module owns:
//!
//! - the propagation namespace;
//! - propagation submodule declarations;
//! - stable propagation-level public exports;
//! - propagation module documentation;
//! - propagation dependency-direction policy;
//! - propagation-level architectural invariants.
//!
//! Individual propagation files own their respective mathematical and
//! computational responsibilities.
//!
//! # Does NOT own
//!
//! This module does NOT own:
//!
//! - canonical Quantum IR;
//! - quantum source syntax;
//! - frontend parsing;
//! - quantum state representations;
//! - quantum channels;
//! - physical noise semantics;
//! - calibration state;
//! - characterization experiments;
//! - routing algorithms;
//! - scheduling algorithms;
//! - QEC decoding;
//! - hardware APIs;
//! - QPU transport;
//! - simulator execution;
//! - benchmark methodology;
//! - serialization wire formats;
//! - cryptographic hashing;
//! - vendor-specific behavior;
//! - global resource limits;
//! - random-number generation;
//! - global mutable state.
//!
//! # Current propagation modules
//!
//! The propagation subsystem is intentionally split by mathematical
//! responsibility:
//!
//! ```text
//! propagation/
//! ├── mod.rs
//! ├── uncertainty.rs
//! ├── error_budget.rs
//! ├── fidelity.rs
//! ├── sensitivity.rs
//! ├── bounds.rs
//! └── accumulation.rs
//! ```
//!
//! The module declarations in this file must correspond to files that
//! actually exist in the source tree. A future propagation module must be
//! added here only when that source file has been created and its public
//! contract has been established.
//!
//! This prevents the namespace file from becoming a compile-time dependency
//! on unfinished modules.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! zqn::core
//!      |
//!      +---- probability
//!      |
//!      +---- channel
//!      |
//!      +---- fault
//!      |
//!      +---- noise
//!      |
//!      +---- operations
//!      |
//!      +---- calibration
//!      |
//!      +---- characterization
//!      |
//!      +---- propagation
//! ```
//!
//! Propagation may consume contracts from the preceding ZQN layers, but
//! propagation must not introduce reverse dependencies into those layers.
//!
//! In particular:
//!
//! ```text
//! propagation -> noise
//! propagation -> calibration
//! propagation -> characterization
//!
//! allowed
//! ```
//!
//! whereas:
//!
//! ```text
//! noise -> propagation
//! calibration -> propagation
//! characterization -> propagation
//!
//! must not be required merely to define their fundamental semantics.
//!
//! Higher-level integration modules may connect these systems when required.
//!
//! # Separation of propagation responsibilities
//!
//! The propagation modules have intentionally different responsibilities.
//!
//! ## `uncertainty`
//!
//! Owns deterministic propagation of already-quantified uncertainty.
//!
//! It may perform:
//!
//! - first-order propagation;
//! - covariance-aware propagation;
//! - interval propagation;
//! - standard-uncertainty propagation;
//! - uncertainty validation.
//!
//! It does not infer statistical uncertainty from raw observations.
//!
//! ## `sensitivity`
//!
//! Owns sensitivity analysis.
//!
//! It may determine:
//!
//! - derivatives;
//! - gradients;
//! - Jacobians;
//! - normalized sensitivities;
//! - parameter influence;
//! - directional sensitivity;
//! - sensitivity rankings.
//!
//! It does not own uncertainty estimation itself.
//!
//! ## `error_budget`
//!
//! Owns allocation and aggregation of error budgets.
//!
//! It answers questions such as:
//!
//! ```text
//! Where is the available error budget being consumed?
//! ```
//!
//! It must not become a general-purpose uncertainty engine.
//!
//! ## `fidelity`
//!
//! Owns fidelity and distance/error metrics.
//!
//! Examples include:
//!
//! - state fidelity;
//! - process fidelity;
//! - entanglement fidelity;
//! - appropriate distance measures;
//! - explicitly qualified approximations or bounds.
//!
//! It must not silently convert every physical error into one universal
//! fidelity metric.
//!
//! ## `bounds`
//!
//! Owns conservative or mathematically justified bounds where the selected
//! representation permits them.
//!
//! Bounds must remain distinguishable from estimates.
//!
//! ## `accumulation`
//!
//! Owns analysis of how errors or uncertainties accumulate across multiple
//! operations, resources, layers, time intervals, or other explicitly
//! supplied dimensions.
//!
//! It must not assume that errors always add linearly.
//!
//! # Fundamental semantic separation
//!
//! Propagation is analysis, not physical noise definition.
//!
//! For example:
//!
//! ```text
//! T1 = 100 µs
//! uncertainty(T1) = 3 µs
//! ```
//!
//! may originate from calibration.
//!
//! ZQN propagation may determine how uncertainty in `T1` affects an
//! observable.
//!
//! However:
//!
//! ```text
//! "What is T1?"
//! ```
//!
//! remains a calibration/noise-domain question, not a propagation question.
//!
//! Likewise, propagation does not decide whether a physical process is:
//!
//! - amplitude damping;
//! - dephasing;
//! - leakage;
//! - loss;
//! - crosstalk;
//! - a correlated process;
//! - a non-Markovian process.
//!
//! Those semantics belong to the noise/channel layers.
//!
//! # Approximation policy
//!
//! Propagation operations must make approximation explicit.
//!
//! The subsystem must distinguish at least:
//!
//! ```text
//! Exact
//! Approximate
//! Bounded
//! Statistical
//! Unsupported
//! ```
//!
//! A first-order approximation must never be presented as an exact result.
//!
//! A bound must never be presented as an estimate.
//!
//! A statistical estimate must never be presented as a deterministic
//! physical bound.
//!
//! If an operation cannot preserve the requested semantic guarantee, the
//! operation must return an explicit incompatibility or approximation
//! result rather than silently changing methods.
//!
//! # Scalability
//!
//! There is no semantic upper bound in this module on:
//!
//! - number of parameters;
//! - number of outputs;
//! - number of operations;
//! - number of resources;
//! - number of qubits;
//! - number of modes;
//! - number of machines;
//! - circuit depth;
//! - execution duration;
//! - correlation dimensions.
//!
//! Dimensions must be determined from caller-supplied data.
//!
//! This module must never contain architecture-level constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_PARAMETERS
//! MAX_OPERATIONS
//! MAX_MACHINES
//! ```
//!
//! A concrete implementation may impose an explicit resource policy for a
//! particular operation, but that policy belongs to the operation or its
//! execution context rather than becoming a semantic limit of the ZQN
//! propagation model.
//!
//! # Resource governance
//!
//! Propagation can require substantial computational and memory resources.
//!
//! Examples include:
//!
//! ```text
//! Jacobian:          O(outputs × parameters)
//! dense covariance:  O(parameters²)
//! dense Hessian:     O(parameters²)
//! matrix operations: potentially superlinear
//! ```
//!
//! Therefore implementations must distinguish:
//!
//! ```text
//! semantic size
//! ```
//!
//! from:
//!
//! ```text
//! requested computational resources
//! ```
//!
//! Large workloads must be allowed when resources permit them.
//!
//! When resource governance is required, it must be:
//!
//! - explicit;
//! - caller-visible;
//! - deterministic;
//! - configurable;
//! - inspectable;
//! - reported through the established ZQN error contract.
//!
//! An implementation must never silently reduce precision, discard
//! parameters, truncate a covariance matrix, or switch algorithms merely to
//! fit an implicit hard-coded limit.
//!
//! # Streaming and matrix-free scalability
//!
//! Propagation modules should support matrix-free and streaming forms where
//! mathematically appropriate.
//!
//! For example, a large covariance model does not necessarily need to be
//! materialized as a dense `N × N` matrix.
//!
//! A large Jacobian does not necessarily need to be stored in memory when a
//! caller only needs one column at a time.
//!
//! Therefore the propagation namespace must remain compatible with:
//!
//! - iterators;
//! - streaming results;
//! - lazy evaluation;
//! - matrix-free operators;
//! - sparse representations;
//! - block representations;
//! - externally managed storage;
//! - caller-selected numerical backends.
//!
//! The module namespace itself must not force a particular storage strategy.
//!
//! # Quantum-resource identity
//!
//! Propagation mathematics is fundamentally resource-agnostic.
//!
//! This namespace therefore does not define another quantum-resource ID.
//!
//! When a propagation result needs to identify a quantum resource, the
//! integration layer must use the canonical identifiers owned by:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! In particular, ZQN must not introduce a competing:
//!
//! ```text
//! zqn::QubitId
//! ```
//!
//! when the canonical Quantum IR already supplies the required identity.
//!
//! `QubitId` and `PhysicalQubitId` should therefore be imported only in a
//! concrete propagation type when resource attribution is actually part of
//! that type's semantic contract.
//!
//! This namespace file intentionally remains independent of those IDs.
//!
//! # Integration with canonical Quantum IR
//!
//! The canonical Quantum IR remains the semantic representation of the
//! computation.
//!
//! Propagation does not replace the IR and must not become a second circuit
//! representation.
//!
//! The intended relationship is:
//!
//! ```text
//! Quantum IR
//!     |
//!     +------------------+
//!     |                  |
//!     v                  v
//! execution semantics   ZQN analysis
//!                            |
//!                            v
//!                    propagation result
//! ```
//!
//! A propagation result may refer to an IR operation or resource through
//! stable integration identifiers, but the propagation subsystem does not
//! own the IR itself.
//!
//! # Integration with uncertainty
//!
//! `uncertainty.rs` is the primary consumer/provider boundary for
//! deterministic uncertainty propagation.
//!
//! Other propagation modules should use its established uncertainty
//! representations rather than defining competing uncertainty types.
//!
//! In particular:
//!
//! ```text
//! calibration / characterization / noise
//!                 |
//!                 v
//!          quantified uncertainty
//!                 |
//!                 v
//!        propagation::uncertainty
//!                 |
//!          +------+------ +
//!          |             |
//!          v             v
//!     sensitivity    error_budget
//! ```
//!
//! The existing uncertainty module explicitly reserves sensitivity for
//! sensitivity analysis and error-budget management for budget aggregation.
//!
//! # Integration with sensitivity
//!
//! `sensitivity.rs` should provide parameter influence information that can
//! be consumed by uncertainty propagation.
//!
//! Conceptually:
//!
//! ```text
//! parameters
//!     |
//!     v
//! sensitivity
//!     |
//!     +---- gradient / Jacobian
//!                 |
//!                 v
//!          uncertainty
//!                 |
//!                 v
//!       propagated uncertainty
//! ```
//!
//! This does not mean the sensitivity module owns covariance propagation.
//!
//! The mathematical ownership remains separated.
//!
//! # Integration with error budgets
//!
//! Error-budget analysis may consume:
//!
//! - propagated uncertainty;
//! - sensitivity contributions;
//! - bounds;
//! - fidelity-derived error measures.
//!
//! It remains responsible for budget allocation and aggregation.
//!
//! A sensitivity score must not automatically become an error-budget
//! consumption value without an explicit semantic conversion.
//!
//! # Integration with fidelity
//!
//! Fidelity calculations may consume results produced by other propagation
//! modules.
//!
//! The reverse should be dependency-driven rather than namespace-driven:
//! propagation modules must not import `fidelity` merely because they are
//! located beside it.
//!
//! # Integration with accumulation
//!
//! Accumulation can consume:
//!
//! - local error contributions;
//! - uncertainty contributions;
//! - sensitivity-derived contributions;
//! - operation durations;
//! - resource relationships.
//!
//! It must preserve the distinction between:
//!
//! ```text
//! local quantity
//! propagated quantity
//! bound
//! estimate
//! statistical quantity
//! ```
//!
//! # Integration with bounds
//!
//! Bounds must remain mathematically justified.
//!
//! An upper bound is not automatically an expectation value.
//!
//! A lower bound is not automatically an estimate.
//!
//! If no valid bound can be established, the implementation must report that
//! fact rather than manufacture one through clamping or heuristic fallback.
//!
//! # Integration with calibration
//!
//! Calibration remains the owner of calibration state.
//!
//! Propagation may consume calibration values and their declared
//! uncertainties, covariance, validity intervals, or provenance.
//!
//! Propagation must not silently mutate calibration state.
//!
//! # Integration with characterization
//!
//! Characterization produces observations and estimates.
//!
//! Propagation may consume the resulting quantified values.
//!
//! Raw observations remain owned by characterization.
//!
//! The dependency is therefore:
//!
//! ```text
//! characterization
//!        |
//!        v
//! quantified result
//!        |
//!        v
//! propagation
//! ```
//!
//! not:
//!
//! ```text
//! propagation
//!        |
//!        v
//! raw experiment processing
//! ```
//!
//! # Integration with noise
//!
//! The noise subsystem owns physical noise semantics.
//!
//! Propagation may answer how changes in noise parameters affect a derived
//! observable.
//!
//! For example:
//!
//! ```text
//! noise parameter
//!       |
//!       v
//! sensitivity
//!       |
//!       v
//! observable uncertainty
//! ```
//!
//! Propagation must not redefine the physical meaning of the parameter.
//!
//! # Integration with routing
//!
//! Routing may consume propagation results as one component of a routing
//! objective.
//!
//! Possible inputs include:
//!
//! - estimated error;
//! - uncertainty;
//! - sensitivity;
//! - fidelity loss;
//! - conservative bounds;
//! - calibration confidence.
//!
//! Propagation must not own routing policy.
//!
//! # Integration with scheduling
//!
//! Scheduling may use propagation results to compare candidate schedules,
//! especially when duration-dependent noise and uncertainty are involved.
//!
//! Propagation must not own scheduling decisions.
//!
//! # Integration with QEC
//!
//! QEC may consume propagated physical quantities when evaluating logical
//! sensitivity or logical error budgets.
//!
//! Propagation does not own:
//!
//! - syndrome extraction;
//! - syndrome decoding;
//! - correction;
//! - decoder selection;
//! - logical-code implementation.
//!
//! # Integration with benchmarking
//!
//! Benchmarking owns benchmark methodology and experiment design.
//!
//! Propagation may consume or provide derived quantities needed by benchmark
//! analysis.
//!
//! It must not duplicate benchmark definitions already owned by the
//! benchmarking subsystem.
//!
//! # Integration with simulation
//!
//! Simulation executes quantum dynamics and produces observations.
//!
//! Propagation analyzes supplied mathematical quantities.
//!
//! Therefore:
//!
//! ```text
//! simulator
//!     |
//!     v
//! observation / derived quantity
//!     |
//!     v
//! propagation
//! ```
//!
//! is valid, while propagation must not become a simulator implementation.
//!
//! # Integration with hardware
//!
//! Propagation must remain hardware-independent.
//!
//! It may consume abstract:
//!
//! - target capabilities;
//! - calibration values;
//! - measured noise parameters;
//! - resource identities;
//! - timing information;
//! - uncertainty information.
//!
//! It must never directly depend on a vendor API.
//!
//! Forbidden dependencies include concepts equivalent to:
//!
//! ```text
//! propagation -> vendor SDK
//! propagation -> QPU credentials
//! propagation -> vendor transport
//! ```
//!
//! Hardware adapters belong outside ZQN.
//!
//! # Determinism
//!
//! The propagation namespace itself contains no random execution.
//!
//! Propagation implementations must not introduce:
//!
//! - hidden global RNGs;
//! - thread-local semantic randomness;
//! - wall-clock-dependent results;
//! - process-identity-dependent results;
//! - unordered iteration as a semantic ordering mechanism.
//!
//! Given equivalent inputs and equivalent numerical execution policy, the
//! same propagation operation must produce equivalent results.
//!
//! If a future stochastic propagation algorithm is introduced, its
//! reproducibility contract must be explicit and must use the established
//! ZQN determinism infrastructure rather than creating a private RNG policy.
//!
//! # Numerical safety
//!
//! Propagation implementations must reject invalid numerical inputs where
//! those values have no valid semantic meaning.
//!
//! In particular, implementations must not silently convert:
//!
//! ```text
//! NaN      -> 0
//! infinity -> finite maximum
//! negative uncertainty -> absolute value
//! invalid probability -> clamped probability
//! ```
//!
//! Numerical tolerances must be explicit.
//!
//! Approximation must be explicit.
//!
//! Numerical failure must be distinguishable from a valid physical result.
//!
//! # Error contract
//!
//! Propagation modules must use the established ZQN error infrastructure
//! rather than creating competing top-level ZQN error systems.
//!
//! The repository already provides ZQN error handling for concepts including
//! invalid uncertainty and invalid error-budget state.
//!
//! A propagation implementation should therefore convert domain-specific
//! failures into the canonical ZQN error contract at its public integration
//! boundary.
//!
//! Individual modules may use private/internal helper errors when necessary,
//! but those errors must not create a second incompatible public error
//! hierarchy.
//!
//! # Serialization
//!
//! This namespace does not define serialization formats.
//!
//! Propagation data that becomes externally persistent must use the versioned
//! ZQN IO layer.
//!
//! No propagation module should make Rust struct layout, enum discriminants,
//! or memory layout an accidental wire-format contract.
//!
//! Canonical serialization remains owned by:
//!
//! ```text
//! crate::quantum::zqn::io
//! ```
//!
//! # Versioning
//!
//! The propagation namespace itself must not maintain independent semantic
//! versions for every re-export.
//!
//! Version compatibility is governed by the ZQN version/schema system.
//!
//! A propagation type requiring a schema change must update the appropriate
//! centralized version/compatibility contract rather than inventing an
//! unrelated version mechanism inside this file.
//!
//! # Thread safety
//!
//! The namespace introduces no global mutable state.
//!
//! Individual propagation types should prefer `Send + Sync` where their
//! underlying data and algorithms permit it.
//!
//! Thread safety must never be achieved by introducing global mutable caches
//! that alter semantic results.
//!
//! # Caching
//!
//! Propagation results may be cached by higher-level systems, but cache
//! identity must include all semantic inputs relevant to the calculation.
//!
//! A cache must not use only:
//!
//! ```text
//! operation name
//! model name
//! parameter name
//! ```
//!
//! when other inputs can change the result.
//!
//! Appropriate cache identity may include:
//!
//! - model identity;
//! - parameter values;
//! - uncertainty values;
//! - covariance identity;
//! - calibration identity;
//! - target identity;
//! - numerical policy;
//! - approximation policy;
//! - schema version.
//!
//! # Security
//!
//! Propagation can operate on attacker-controlled or externally supplied
//! numerical data.
//!
//! Implementations must defend against:
//!
//! - allocation bombs;
//! - enormous dimensions;
//! - integer overflow in dimension calculations;
//! - NaN/Infinity injection;
//! - pathological covariance matrices;
//! - pathological numerical ranges;
//! - excessive iteration;
//! - nonterminating user-supplied evaluators;
//! - uncontrolled intermediate allocation.
//!
//! Security/resource failures must be explicit rather than silently producing
//! a degraded scientific result.
//!
//! No `unsafe` implementation is permitted in this subsystem.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly-only features;
//! - no `unsafe`.
//!
//! # Module declaration policy
//!
//! This file is intentionally conservative about declarations.
//!
//! A declaration such as:
//!
//! ```rust
//! pub mod future_module;
//! ```
//!
//! must not be added until `future_module.rs` exists and its contract is
//! ready.
//!
//! This ensures that the propagation namespace remains compilable throughout
//! incremental development.
//!
//! Conversely, once a propagation source file is established, its module
//! declaration belongs here rather than being exposed through ad-hoc imports
//! elsewhere in the repository.
//!
//! # Public API policy
//!
//! This namespace should expose stable module boundaries.
//!
//! Implementation details should remain inside their owning modules.
//!
//! Consumers should prefer:
//!
//! ```text
//! crate::quantum::zqn::propagation::<module>::<type>
//! ```
//!
//! during development.
//!
//! Stable convenience re-exports may be introduced only after the underlying
//! public contracts are stable.
//!
//! Avoid exporting every internal helper from this namespace.
//!
//! # File-completion contract
//!
//! This file is complete when:
//!
//! 1. every declared module exists;
//! 2. no nonexistent module is declared;
//! 3. propagation responsibilities remain separated;
//! 4. no propagation algorithm is implemented here;
//! 5. no duplicate ZQN error hierarchy is introduced;
//! 6. no duplicate quantum-resource identity is introduced;
//! 7. no vendor dependency is introduced;
//! 8. no machine-size limit is introduced;
//! 9. no global mutable state exists;
//! 10. no hidden RNG exists;
//! 11. serialization remains owned by ZQN IO;
//! 12. propagation remains downstream of canonical Quantum IR;
//! 13. uncertainty remains distinct from sensitivity;
//! 14. sensitivity remains distinct from error-budget management;
//! 15. fidelity remains a separate metric domain;
//! 16. accumulation remains a separate analysis domain;
//! 17. resource governance remains explicit;
//! 18. numerical failures remain explicit;
//! 19. approximation remains explicit;
//! 20. Rust 1.97/1.97.1 compatibility is preserved;
//! 21. `unsafe` is forbidden.
//!
//! # Maintenance rule
//!
//! When another propagation file changes its internal implementation, this
//! file must not require modification unless one of the following changes:
//!
//! - a module is added;
//! - a module is removed;
//! - a public module boundary changes;
//! - a stable public re-export changes;
//! - the propagation architecture itself changes.
//!
//! This keeps `mod.rs` independent from implementation details and satisfies
//! the project's requirement that completed files not need repeated editing
//! merely because another module evolves.

// -----------------------------------------------------------------------------
// Safety
// -----------------------------------------------------------------------------
//
// ZQN propagation must never rely on unsafe code.
//
// This applies to this namespace as well as its compilation unit.
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// -----------------------------------------------------------------------------
// Propagation modules
// -----------------------------------------------------------------------------
//
// These declarations correspond only to propagation files whose contracts
// have been established.
//
// Add a declaration when the corresponding source file is actually present.
//
// Current established propagation modules:
//
// - uncertainty.rs
// - error_budget.rs
// - sensitivity.rs
//
// Additional modules such as fidelity.rs, bounds.rs, and accumulation.rs
// should be declared here when their corresponding source files exist.
//
// Keeping declarations synchronized with the filesystem prevents the module
// root from creating artificial compile-time dependencies on unfinished work.

pub mod error_budget;
pub mod sensitivity;
pub mod uncertainty;

// -----------------------------------------------------------------------------
// Public namespace documentation
// -----------------------------------------------------------------------------

/// Semantic namespace for deterministic ZQN propagation analysis.
///
/// This marker type intentionally contains no state. It exists only as a
/// stable documentation and discovery point for tools that inspect the ZQN
/// module hierarchy.
///
/// Mathematical operations belong to the individual propagation modules.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PropagationNamespace;