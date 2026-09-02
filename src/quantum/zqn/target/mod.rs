//! Zamani Quantum Noise (ZQN) — Target Subsystem
//!
//! # Purpose
//!
//! This module is the composition boundary for target-independent and
//! target-facing ZQN functionality.
//!
//! The target subsystem answers four separate questions:
//!
//! 1. `requirements.rs`:
//!    What does a computation require from a target?
//!
//! 2. `capabilities.rs`:
//!    What can a target provide?
//!
//! 3. `compatibility.rs`:
//!    Can the target satisfy the computation's requirements under an explicit
//!    compatibility/approximation policy?
//!
//! 4. `lowering.rs`:
//!    How is an accepted target-independent ZQN description transformed into
//!    a target-supported realization?
//!
//! `validation.rs` provides the final structural and policy validation
//! boundary before target-dependent execution.
//!
//! The target subsystem therefore implements the following architectural
//! boundary:
//!
//! ```text
//!                    Zamani source
//!                         │
//!                         ▼
//!                   quantum::ir
//!                         │
//!                         ▼
//!                ZQN semantic model
//!                         │
//!                         ▼
//!              target::requirements
//!                         │
//!                         │ "I require X"
//!                         ▼
//!              target::capabilities
//!                         │
//!                         │ "I provide Y"
//!                         ▼
//!             target::compatibility
//!                         │
//!              ┌──────────┴──────────┐
//!              │                     │
//!              ▼                     ▼
//!          compatible            incompatible
//!              │
//!              ▼
//!         target::lowering
//!              │
//!              ▼
//!        target::validation
//!              │
//!              ▼
//!        runtime / hardware
//! ```
//!
//! # Architectural ownership
//!
//! This module owns ONLY:
//!
//! - the target subsystem namespace;
//! - public target-module composition;
//! - stable re-export boundaries;
//! - target-module documentation;
//! - dependency direction between target components.
//!
//! It does NOT own:
//!
//! - quantum IR;
//! - qubit identity;
//! - physical qubit identity;
//! - target discovery;
//! - hardware inventory;
//! - vendor APIs;
//! - provider credentials;
//! - network access;
//! - hardware topology;
//! - calibration values;
//! - quantum channels;
//! - noise models;
//! - faults;
//! - simulation;
//! - routing;
//! - scheduling;
//! - QEC;
//! - benchmarking;
//! - runtime execution;
//! - resource allocation.
//!
//! Those responsibilities remain in their owning subsystems.
//!
//! # Canonical quantum identity
//!
//! This module MUST NOT define another `QubitId` or `PhysicalQubitId`.
//!
//! Canonical identities remain:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Target-specific capability and requirement scopes must ultimately use the
//! canonical quantum identity types established by `quantum::ir::qubit`.
//!
//! No target module may interpret a numeric quantum identifier as a machine
//! size or capacity.
//!
//! # Write once, scale everywhere
//!
//! The target namespace imposes NO semantic upper bound on:
//!
//! - logical qubits;
//! - physical qubits;
//! - quantum resources;
//! - operation count;
//! - operation arity;
//! - circuit depth;
//! - target capability count;
//! - topology size;
//! - distributed nodes;
//! - execution resources;
//! - quantum technology;
//! - vendor;
//! - backend;
//! - machine size.
//!
//! There must be no target-level constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_PHYSICAL_QUBITS
//! MAX_TARGETS
//! MAX_CAPABILITIES
//! MAX_RESOURCES
//! MAX_ARITY
//! ```
//!
//! within this composition boundary.
//!
//! "Infinity" means that ZQN does not encode an artificial finite machine-size
//! ceiling. It does NOT mean that physical hardware, memory, execution time,
//! address space, network bandwidth, or runtime resources are infinite.
//!
//! Actual limits belong to:
//!
//! - target resource descriptions;
//! - runtime policies;
//! - memory/resource policies;
//! - execution policies;
//! - security policies;
//! - hardware capacity;
//! - simulator/emulator capacity.
//!
//! # Requirement/capability separation
//!
//! Requirements and capabilities are intentionally different abstractions.
//!
//! A requirement means:
//!
//! ```text
//! "The computation needs X."
//! ```
//!
//! A capability means:
//!
//! ```text
//! "The target provides X."
//! ```
//!
//! Compatibility is the only layer that should determine whether those two
//! descriptions can be reconciled.
//!
//! This module MUST NOT perform compatibility evaluation itself.
//!
//! # Exactness and approximation
//!
//! The target subsystem preserves the distinction between:
//!
//! - native support;
//! - exact emulation;
//! - explicit approximation;
//! - unsupported functionality.
//!
//! Approximation MUST NEVER silently become exact support.
//!
//! In particular:
//!
//! ```text
//! exact requirement + approximate target = incompatible
//!
//! exact requirement + exact emulation = potentially compatible
//!
//! native requirement + emulated target = incompatible
//!
//! approximate requirement + permitted approximation = potentially compatible
//! ```
//!
//! The actual policy belongs to the target compatibility layer.
//!
//! # Technology neutrality
//!
//! Nothing in this module assumes that quantum computation is limited to:
//!
//! - qubits;
//! - two-level systems;
//! - gate-model circuits;
//! - Pauli channels;
//! - superconducting devices;
//! - trapped ions;
//! - neutral atoms;
//! - photonics;
//! - a particular vendor;
//! - a particular instruction set.
//!
//! The target abstraction must remain capable of representing future
//! technologies without requiring this module to be redesigned.
//!
//! Examples of target modalities that may eventually consume this boundary
//! include:
//!
//! - gate-model quantum computers;
//! - analog quantum systems;
//! - annealing systems;
//! - measurement-based quantum computers;
//! - bosonic systems;
//! - continuous-variable systems;
//! - fermionic systems;
//! - photonic systems;
//! - distributed quantum systems;
//! - logical/fault-tolerant systems;
//! - hybrid quantum systems;
//! - future quantum technologies.
//!
//! # No vendor coupling
//!
//! This namespace MUST NOT contain vendor-specific implementation modules such
//! as:
//!
//! ```text
//! ibm.rs
//! ionq.rs
//! rigetti.rs
//! quantinuum.rs
//! aws.rs
//! azure.rs
//! ```
//!
//! Provider-specific behavior belongs to hardware/provider adapter layers.
//!
//! Such adapters produce target capability descriptions and consume target
//! lowering/validation contracts; they are not embedded in ZQN target
//! semantics.
//!
//! # Determinism
//!
//! Target descriptions are value-oriented semantic data.
//!
//! This composition module performs no:
//!
//! - random generation;
//! - clock reads;
//! - environment inspection;
//! - network access;
//! - provider discovery;
//! - process-global state access;
//! - credential lookup;
//! - implicit configuration loading.
//!
//! Deterministic behavior is therefore delegated to the underlying target
//! modules and their explicit inputs.
//!
//! # Resource safety
//!
//! No target module may allocate a representation proportional to physical
//! machine size merely to inspect target capabilities.
//!
//! Capability declarations should remain declarative and resource-aware.
//!
//! Potentially expensive operations must remain under explicit caller/runtime
//! resource policy.
//!
//! A target capability profile MUST NOT itself imply permission to allocate,
//! connect to, authenticate with, or control hardware.
//!
//! # Security boundary
//!
//! Target descriptions are data, not authority.
//!
//! A capability declaration does not grant:
//!
//! - QPU access;
//! - credentials;
//! - filesystem permissions;
//! - network permissions;
//! - process execution;
//! - hardware-control permissions.
//!
//! Authentication and authorization belong to the runtime/hardware security
//! layers.
//!
//! Untrusted target descriptions must be validated before they are used to
//! allocate resources or execute operations.
//!
//! # Dependency direction
//!
//! The target subsystem follows this dependency direction:
//!
//! ```text
//! core
//!  │
//!  ├───────────────┐
//!  ▼               ▼
//! requirements   capabilities
//!  │               │
//!  └───────┬───────┘
//!          ▼
//!    compatibility
//!          │
//!          ▼
//!       lowering
//!          │
//!          ▼
//!      validation
//!          │
//!          ▼
//!   runtime / hardware
//! ```
//!
//! More precisely:
//!
//! ```text
//! target::requirements
//!        │
//!        ▼
//! target::compatibility ◄──── target::capabilities
//!        │
//!        ▼
//! target::lowering
//!        │
//!        ▼
//! target::validation
//! ```
//!
//! `requirements.rs` describes what is needed.
//!
//! `capabilities.rs` describes what is available.
//!
//! `compatibility.rs` determines whether the two are compatible.
//!
//! `lowering.rs` determines how an accepted abstract representation can be
//! realized.
//!
//! `validation.rs` ensures the resulting target-facing representation is
//! structurally and semantically valid before execution.
//!
//! The composition module itself does not call any of these modules.
//!
//! # Integration with quantum::ir
//!
//! The canonical quantum IR remains:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! The target subsystem consumes semantic information from the IR through
//! explicit integration contracts.
//!
//! It MUST NOT introduce a competing IR.
//!
//! The intended flow is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! semantic analysis
//!      │
//!      ▼
//! ZQN requirements
//!      │
//!      ▼
//! target requirements
//! ```
//!
//! The target subsystem therefore remains downstream from the canonical IR.
//!
//! # Integration with ZQN noise
//!
//! ZQN noise/channel models may describe requirements such as:
//!
//! - channel representation;
//! - correlated noise;
//! - temporal noise;
//! - spatial noise;
//! - crosstalk;
//! - leakage;
//! - erasure;
//! - loss;
//! - readout noise;
//! - calibration dependence;
//! - uncertainty;
//! - non-Markovian behavior;
//! - deterministic sampling.
//!
//! Those semantic models remain owned by their respective ZQN modules.
//!
//! The target subsystem only answers whether a target can represent or realize
//! the required semantics and, where permitted, how they can be lowered.
//!
//! # Integration with calibration
//!
//! Calibration data remains owned by:
//!
//! ```text
//! crate::quantum::zqn::calibration
//! ```
//!
//! Target capability declarations MUST NOT be treated as calibration data.
//!
//! A target may advertise support for calibration-aware execution without
//! embedding a particular calibration snapshot inside its static capability
//! profile.
//!
//! Runtime validity of calibration belongs to the calibration/runtime layers.
//!
//! # Integration with routing
//!
//! Routing may consume target capabilities and compatibility results to answer:
//!
//! ```text
//! "Can this logical computation be placed on this target?"
//! ```
//!
//! ZQN target capabilities may contribute information about:
//!
//! - supported operations;
//! - resource scopes;
//! - supported noise semantics;
//! - crosstalk modeling;
//! - correlated-noise modeling;
//! - error characterization;
//! - fidelity-related constraints.
//!
//! Routing itself remains outside this module.
//!
//! # Integration with scheduling
//!
//! Scheduling may consume target capability and validation information for:
//!
//! - supported timing semantics;
//! - time-dependent noise;
//! - idle-noise requirements;
//! - pulse requirements;
//! - dynamic execution;
//! - calibration validity.
//!
//! Scheduling remains outside this module.
//!
//! # Integration with QEC
//!
//! QEC may consume target capability information to determine whether a target
//! can support the physical or logical noise semantics required by a fault
//! tolerant workload.
//!
//! QEC remains the owner of:
//!
//! - syndrome processing;
//! - decoding;
//! - correction;
//! - logical fault-tolerance algorithms.
//!
//! ZQN target support does not replace QEC.
//!
//! # Integration with hardware
//!
//! Hardware/provider adapters are responsible for transforming real target
//! information into provider-neutral capability and resource descriptions.
//!
//! Conceptually:
//!
//! ```text
//! hardware/provider adapter
//!             │
//!             ▼
//! TargetCapabilities
//!             │
//!             ▼
//! target compatibility
//!             │
//!             ▼
//! target lowering
//!             │
//!             ▼
//! hardware execution adapter
//! ```
//!
//! This keeps vendor APIs outside ZQN.
//!
//! # Integration with simulation
//!
//! A simulator may expose a `TargetCapabilities` profile describing which ZQN
//! semantics it can faithfully represent.
//!
//! For example, one simulator might support:
//!
//! - exact Kraus channels;
//! - Monte Carlo sampling;
//! - correlated noise;
//!
//! while another may support only:
//!
//! - Pauli stochastic noise.
//!
//! The simulator declares the difference through capabilities rather than
//! requiring ZQN to contain simulator-specific conditionals.
//!
//! # Integration with benchmarking
//!
//! Benchmarking consumes target capabilities and ZQN characterization results.
//!
//! It may use them to determine:
//!
//! - which experiments are executable;
//! - which noise models can be characterized;
//! - which approximations are valid;
//! - which measurements are available.
//!
//! Benchmark methodology remains owned by the benchmarking subsystem.
//!
//! # Integration with runtime
//!
//! Runtime is responsible for:
//!
//! - resource allocation;
//! - authentication;
//! - authorization;
//! - execution context;
//! - cancellation;
//! - deadlines;
//! - actual hardware communication;
//! - runtime resource limits.
//!
//! ZQN target declarations do not grant any runtime authority.
//!
//! # Serialization boundary
//!
//! This module does not define a wire format.
//!
//! Canonical serialization remains owned by:
//!
//! ```text
//! crate::quantum::zqn::io
//! ```
//!
//! Target module APIs therefore remain semantic Rust interfaces rather than
//! being coupled to a particular serialization representation.
//!
//! # Versioning
//!
//! Target semantics must remain compatible with the ZQN versioning policy.
//!
//! Version/schema compatibility belongs to the ZQN core and I/O layers.
//!
//! Adding a new target capability must not require modifying this module.
//!
//! New target technologies should normally add new capability identifiers or
//! adapter-side abstractions rather than creating another target composition
//! root.
//!
//! # Public API policy
//!
//! The stable target namespace consists of:
//!
//! ```text
//! target::requirements
//! target::capabilities
//! target::compatibility
//! target::lowering
//! target::validation
//! ```
//!
//! This module intentionally re-exports only those module namespaces.
//!
//! Individual implementation details should remain owned by their respective
//! files.
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
//! - no external dependency requirements;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes accidental unsafe additions a compile-time
//! error.
//!
//! # File completion contract
//!
//! This file is considered complete when:
//!
//! 1. all target subsystem modules are declared exactly once;
//! 2. no target implementation is duplicated here;
//! 3. no quantum identity type is redefined here;
//! 4. no vendor-specific implementation is imported here;
//! 5. no runtime side effect occurs here;
//! 6. no artificial scalability limit is introduced here;
//! 7. the target dependency direction remains acyclic;
//! 8. downstream modules can import the target namespace without modifying
//!    this file merely because their implementations evolve;
//! 9. adding a new capability does not require changing this file;
//! 10. adding a new provider does not require changing this file;
//! 11. adding a new lowering strategy does not require changing this file;
//! 12. the canonical quantum identity remains owned by
//!     `quantum::ir::qubit`.
//!
//! =============================================================================
//! Module declarations
//! =============================================================================

#![forbid(unsafe_code)]

/// Target-independent requirements for a ZQN computation.
///
/// This module answers what a computation requires from an eventual target.
/// It does not discover or select targets.
pub mod requirements;

/// Target capability declarations and capability-profile operations.
///
/// This module answers what a target declares that it can provide. It does not
/// perform hardware discovery or execute anything.
pub mod capabilities;

/// Requirement/capability compatibility evaluation.
///
/// This module determines whether target requirements can be satisfied by a
/// capability profile under an explicit policy.
pub mod compatibility;

/// Target-independent to target-supported lowering contracts.
///
/// This module defines how an already-accepted abstract ZQN representation may
/// be lowered toward a target realization. It does not communicate with
/// hardware.
pub mod lowering;

/// Final target-facing structural and semantic validation.
///
/// This module validates target-facing descriptions before runtime/hardware
/// execution. It does not execute or authorize hardware access.
pub mod validation;