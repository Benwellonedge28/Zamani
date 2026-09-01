//! Zamani Quantum Noise (ZQN) — Fault subsystem.
//!
//! # Purpose
//!
//! This module is the public composition boundary for the canonical ZQN fault
//! subsystem.
//!
//! ZQN faults represent realized physical, logical, environmental, transport,
//! measurement, preparation, reset, timing, leakage, loss, erasure,
//! correlated, crosstalk, and other explicitly represented deviations from
//! intended quantum computation.
//!
//! The central distinction is:
//!
//! ```text
//! quantum::ir
//!     │
//!     │ intended computation / semantic operation
//!     ▼
//! ZQN
//!     │
//!     │ realized physical/abstract deviation
//!     ▼
//! Fault
//! ```
//!
//! A `Fault` is therefore not another quantum IR and is not a replacement for
//! a quantum channel or noise model.
//!
//! # Architectural responsibility
//!
//! The `fault` subsystem owns the representation and analysis of realized
//! faults.
//!
//! It provides:
//!
//! - canonical realized fault representation;
//! - fault locations;
//! - fault classifications;
//! - fault effects;
//! - correlated faults;
//! - leakage faults;
//! - erasure faults;
//! - loss faults;
//! - materialized fault batches;
//! - deterministic inspection and validation;
//! - specialized fault views;
//! - canonical integration points for QEC, simulation, routing,
//!   scheduling, benchmarking, characterization, and runtime layers.
//!
//! # Non-ownership
//!
//! This module does NOT own:
//!
//! - canonical quantum program semantics;
//! - canonical quantum IR;
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - `QubitRef`;
//! - quantum channels;
//! - probability distributions;
//! - stochastic sampling;
//! - random-number generation;
//! - noise-model generation;
//! - calibration;
//! - characterization protocols;
//! - routing;
//! - scheduling;
//! - syndrome extraction;
//! - decoding;
//! - logical correction;
//! - hardware APIs;
//! - QPU credentials;
//! - backend execution;
//! - benchmark methodology;
//! - serialization formats;
//! - global resource limits;
//! - global registries;
//! - global mutable state.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Canonical quantum-resource identity
//!
//! Quantum-resource identity is owned by:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! In particular:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! crate::quantum::ir::qubit::QubitRef
//! ```
//!
//! The fault subsystem MUST NOT define:
//!
//! ```text
//! ZqnQubitId
//! FaultQubitId
//! FaultPhysicalQubitId
//! NoiseQubitId
//! ```
//!
//! or any equivalent competing identity abstraction.
//!
//! This preserves the repository-wide identity boundary established by the
//! Quantum IR.
//!
//! # Dependency direction
//!
//! The intended dependency graph is:
//!
//! ```text
//! quantum::ir::qubit
//!        │
//!        │ canonical resource identity
//!        ▼
//! ┌──────────────────────────┐
//! │     zqn::fault::fault    │
//! │ canonical Fault value    │
//! └────────────┬─────────────┘
//!              │
//!      ┌───────┼────────┬────────────┐
//!      │       │        │            │
//!      ▼       ▼        ▼            ▼
//! location classification correlated batch
//!      │       │        │            │
//!      └───────┼────────┼────────────┘
//!              │
//!        ┌─────┴─────┐
//!        ▼           ▼
//!     leakage     erasure
//!        │           │
//!        └─────┬─────┘
//!              ▼
//!             loss
//! ```
//!
//! The specialized modules depend on the canonical fault model; the canonical
//! fault model must not depend on specialized wrappers.
//!
//! # Dependency rule
//!
//! `mod.rs` contains only composition and public API organization.
//!
//! It MUST NOT become a place where semantic implementation is added merely
//! because the implementation is convenient here.
//!
//! New semantic behavior belongs in the module that owns that responsibility.
//!
//! # Write once, scale everywhere
//!
//! This module deliberately imposes no machine-size ceiling.
//!
//! It contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_FAULTS
//! MAX_CORRELATED_QUBITS
//! MAX_OPERATIONS
//! MAX_BATCH_SIZE
//! ```
//!
//! A fault may refer to any number of resources representable by the selected
//! data structures and permitted by the caller's explicit resource policy.
//!
//! The absence of a semantic limit does NOT imply infinite physical resources.
//!
//! Actual execution is constrained by:
//!
//! - available memory;
//! - storage;
//! - CPU/GPU capacity;
//! - distributed resources;
//! - target capabilities;
//! - runtime policy;
//! - configured ZQN limits;
//! - cancellation/deadline policy;
//! - hardware availability.
//!
//! The architectural guarantee is that the fault semantics themselves do not
//! encode an artificial finite machine size.
//!
//! # Fault versus noise
//!
//! The distinction is fundamental:
//!
//! ```text
//! NoiseModel
//!     = law/process capable of producing deviations
//!
//! QuantumChannel
//!     = physical transformation/channel
//!
//! Fault
//!     = realized deviation/event
//! ```
//!
//! Therefore this module must never become the owner of stochastic generation.
//!
//! A noise model may produce:
//!
//! ```text
//! zero faults
//! one fault
//! many faults
//! ```
//!
//! depending on its semantics and execution context.
//!
//! # Fault versus QEC
//!
//! ZQN owns the representation of the fault.
//!
//! QEC owns what happens after the fault is interpreted in a code-specific
//! fault-tolerance context:
//!
//! ```text
//! ZQN Fault
//!     │
//!     ▼
//! QEC adapter
//!     │
//!     ├── syndrome extraction
//!     ├── decoder
//!     ├── correction
//!     └── logical-fault analysis
//! ```
//!
//! The QEC subsystem must not need to duplicate ZQN's fundamental fault,
//! location, identity, or correlation representations.
//!
//! # Fault versus routing
//!
//! Routing may inspect fault information to calculate costs such as:
//!
//! - correlated-error risk;
//! - crosstalk;
//! - physical-resource sensitivity;
//! - transport faults;
//! - location-specific error characteristics.
//!
//! Routing remains responsible for deciding where logical resources are placed.
//!
//! The fault subsystem does not perform routing.
//!
//! # Fault versus scheduling
//!
//! Scheduling may inspect timing-related faults and fault locations to estimate
//! the consequences of:
//!
//! - idle time;
//! - duration;
//! - transport;
//! - synchronization;
//! - temporal correlations;
//! - calibration validity.
//!
//! Scheduling remains responsible for temporal placement.
//!
//! The fault subsystem does not schedule operations.
//!
//! # Fault versus simulation
//!
//! Simulation consumes faults.
//!
//! It may interpret them as:
//!
//! - state transformations;
//! - channel applications;
//! - stochastic trajectories;
//! - measurement changes;
//! - leakage/loss state transitions;
//! - logical or physical events.
//!
//! The fault subsystem itself does not own quantum-state evolution.
//!
//! # Fault versus hardware
//!
//! Hardware adapters may translate observed hardware behavior into ZQN faults.
//!
//! The direction is:
//!
//! ```text
//! hardware adapter
//!        │
//!        ▼
//! abstract observation/fault
//!        │
//!        ▼
//! ZQN fault
//! ```
//!
//! ZQN must never contain vendor-specific QPU APIs or credentials.
//!
//! # Fault versus benchmarking
//!
//! Benchmarking may consume realized fault batches and classification reports
//! to calculate:
//!
//! - fault rates;
//! - distributions;
//! - correlations;
//! - leakage rates;
//! - loss/erasure rates;
//! - application-level error metrics;
//! - characterization results.
//!
//! Benchmark methodology remains outside this subsystem.
//!
//! # Determinism
//!
//! The fault subsystem contains no hidden randomness.
//!
//! It MUST NOT:
//!
//! - call a global RNG;
//! - call a thread-local RNG;
//! - derive identity from memory addresses;
//! - derive semantics from hash-map iteration order;
//! - read system time implicitly;
//! - maintain global mutable state.
//!
//! Stochastic fault generation belongs to the ZQN noise/simulation layers.
//!
//! Once a fault is materialized, it is an immutable semantic value.
//!
//! # Parallel determinism
//!
//! The fault representation itself must remain safe to consume from sequential
//! or parallel execution.
//!
//! Deterministic ordering must be explicitly requested where canonical output
//! is required.
//!
//! Consumers must not depend on:
//!
//! - thread scheduling;
//! - hash-map iteration order;
//! - allocator behavior;
//! - process identity.
//!
//! # Resource safety
//!
//! Semantic fault definitions do not own global resource limits.
//!
//! A caller processing untrusted or extremely large input must apply the
//! appropriate ZQN resource policy before materializing arbitrarily large
//! structures.
//!
//! For example:
//!
//! ```text
//! untrusted stream
//!       │
//!       ▼
//! ZqnLimits / execution policy
//!       │
//!       ▼
//! FaultBatch
//! ```
//!
//! This keeps semantic correctness separate from resource governance.
//!
//! # No artificial batch ceiling
//!
//! `FaultBatch` may be finite and materialized, but its semantic API must not
//! declare a universal maximum number of faults.
//!
//! Streaming callers should use a stream/iterator or bounded admission policy
//! instead of requiring the entire execution history to be resident in memory.
//!
//! # Numerical safety
//!
//! Faults themselves must not introduce a second probability abstraction.
//!
//! Probabilities and distributions belong to:
//!
//! ```text
//! crate::quantum::zqn::probability
//! ```
//!
//! If a specialized fault representation contains a probability-like value,
//! it must follow the canonical probability contract of the ZQN subsystem
//! rather than silently introducing another floating-point convention.
//!
//! # Serialization
//!
//! This module does not define the external serialization format.
//!
//! Serialization belongs to:
//!
//! ```text
//! crate::quantum::zqn::io
//! ```
//!
//! A serialized fault must preserve enough semantic information to reconstruct
//! the same fault domain without collapsing:
//!
//! - logical and physical identities;
//! - fault identity;
//! - location domain;
//! - classification;
//! - effect;
//! - timing;
//! - correlation;
//! - specialized fault semantics.
//!
//! Versioning and schema migration are IO-layer responsibilities.
//!
//! # Compatibility
//!
//! The public module structure is intended to remain stable while internal
//! implementations evolve.
//!
//! New fault kinds should normally be added by extending the owning semantic
//! module rather than changing unrelated consumers.
//!
//! Consumers should prefer the canonical APIs exported by the fault subsystem
//! instead of reaching into implementation details.
//!
//! # Security
//!
//! Fault descriptions are data, not authorization credentials.
//!
//! A `Fault`, `FaultId`, `PhysicalQubitId`, or related value MUST NOT grant:
//!
//! - hardware access;
//! - QPU execution permission;
//! - calibration access;
//! - private experiment access;
//! - filesystem access;
//! - network access.
//!
//! Authorization belongs to the surrounding security/capability subsystem.
//!
//! Untrusted fault streams must be processed under explicit resource limits
//! and cancellation policies to prevent resource-exhaustion attacks.
//!
//! # Unsafe-code policy
//!
//! This module and every child module in the fault subsystem must remain free
//! of unsafe Rust.
//!
//! The crate-level policy is reinforced here so accidental unsafe additions
//! fail compilation immediately.
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
//! # Public module inventory
//!
//! ```text
//! fault/
//! ├── mod.rs
//! ├── fault.rs
//! ├── location.rs
//! ├── classification.rs
//! ├── correlated.rs
//! ├── leakage.rs
//! ├── erasure.rs
//! ├── loss.rs
//! └── batch.rs
//! ```
//!
//! Each child module owns one coherent responsibility.
//!
//! # Module responsibilities
//!
//! ## `fault`
//!
//! Canonical realized fault representation.
//!
//! ```text
//! Fault
//! FaultClassification
//! FaultEffect
//! FaultTiming
//! ```
//!
//! Exact public types are owned by `fault.rs`; this module does not recreate
//! them.
//!
//! ## `location`
//!
//! Canonical fault location representation.
//!
//! It is responsible for describing affected logical/physical resources,
//! composite locations, and non-resource-specific locations.
//!
//! ## `classification`
//!
//! Derived and analytical classification APIs.
//!
//! This module must not redefine the canonical `FaultClassification` enum.
//!
//! It extends the canonical semantics with deterministic predicates,
//! reports, and aggregation utilities.
//!
//! ## `correlated`
//!
//! Specialized representation of correlated faults.
//!
//! Correlation size is data-driven and must not be represented by fixed
//! two-resource, three-resource, or N-resource types.
//!
//! ## `leakage`
//!
//! Specialized leakage-fault semantics.
//!
//! Leakage is represented independently of ordinary Pauli fault semantics.
//!
//! ## `erasure`
//!
//! Specialized erasure-fault semantics.
//!
//! Erasure is not silently collapsed into ordinary loss or a generic bit flip.
//!
//! ## `loss`
//!
//! Specialized loss-fault semantics.
//!
//! Loss remains distinct from erasure and leakage because downstream physical
//! and QEC semantics may treat those conditions differently.
//!
//! ## `batch`
//!
//! Materialized ordered collections of realized faults.
//!
//! It owns container behavior, not the semantics of an individual fault.
//!
//! # Integration contract
//!
//! Downstream ZQN modules should consume this subsystem through:
//!
//! ```text
//! crate::quantum::zqn::fault
//! ```
//!
//! and its child modules.
//!
//! The intended dependency direction is:
//!
//! ```text
//! zqn::fault
//!      ▲
//!      │
//! ┌────┼───────────────────────────────────────────┐
//! │    │                                           │
//! │    │                                           │
//! noise  simulation                         characterization
//! │    │                                           │
//! │    ├───────────────────────────────────────────┤
//! │                    │                           │
//! ▼                    ▼                           ▼
//! QEC                routing                    benchmarking
//! │                    │                           │
//! └────────────────────┼───────────────────────────┘
//!                      ▼
//!                    runtime
//! ```
//!
//! `fault` is a semantic provider, not an orchestrator.
//!
//! # Public API stability
//!
//! The child modules are public because they form the ZQN subsystem's
//! integration boundaries.
//!
//! However, consumers should use the highest-level stable type appropriate to
//! their responsibility:
//!
//! ```text
//! Fault
//! FaultBatch
//! FaultLocation
//! FaultClassification
//! CorrelatedFault
//! Leakage
//! Erasure
//! Loss
//! ```
//!
//! rather than depending on internal helper implementation details.
//!
//! # Extension rule
//!
//! When adding a future fault specialization:
//!
//! 1. determine whether it is genuinely a new semantic category;
//! 2. keep the canonical `Fault` representation authoritative;
//! 3. reuse canonical IR qubit identities;
//! 4. avoid introducing a machine-size constant;
//! 5. avoid introducing a new probability representation;
//! 6. avoid introducing an RNG;
//! 7. avoid introducing hardware/vendor dependencies;
//! 8. provide deterministic inspection;
//! 9. define validation invariants;
//! 10. add tests in the owning module;
//! 11. expose it from this composition boundary only after its contract is
//!     stable.
//!
//! # Testing contract
//!
//! The fault subsystem should be tested at several levels.
//!
//! ```text
//! fault.rs
//!     │
//!     ├── unit tests
//!     └── invariant tests
//!
//! location.rs
//!     │
//!     ├── identity-domain tests
//!     └── composite-location tests
//!
//! classification.rs
//!     │
//!     ├── predicate tests
//!     └── deterministic aggregation tests
//!
//! correlated.rs
//!     │
//!     ├── arbitrary-cardinality tests
//!     └── correlation invariant tests
//!
//! leakage.rs / erasure.rs / loss.rs
//!     │
//!     └── specialization/validation tests
//!
//! batch.rs
//!     │
//!     ├── ordered-storage tests
//!     ├── resource-policy tests
//!     └── deterministic iteration tests
//! ```
//!
//! Integration tests should additionally prove that:
//!
//! ```text
//! canonical IR qubit identity
//!         ↓
//! ZQN fault location
//!         ↓
//! Fault
//!         ↓
//! FaultBatch
//!         ↓
//! QEC / simulation / routing / benchmarking
//! ```
//!
//! preserves identity domains and semantic information.
//!
//! # Compile-time safety
//!
//! `unsafe_code` is forbidden for this module.
//!
//! `unsafe_op_in_unsafe_fn` is denied as an additional guard.
//!
//! No child module is permitted to weaken this policy.
//!
//! =============================================================================
//! Module declarations
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

/// Canonical realized fault representation.
///
/// This is the semantic root of the fault subsystem. Specialized modules
/// consume this representation rather than defining competing fault models.
pub mod fault;

/// Canonical locations affected by a fault.
///
/// Uses `crate::quantum::ir::qubit::{QubitId, PhysicalQubitId, QubitRef}` where
/// quantum-resource identity is required.
pub mod location;

/// Deterministic classification and analysis of canonical faults.
///
/// This module extends, but does not redefine, the canonical classification
/// vocabulary owned by `fault.rs`.
pub mod classification;

/// Arbitrary-cardinality correlated fault semantics.
///
/// Correlation is data-driven; there is no fixed two-resource or N-resource
/// architectural ceiling.
pub mod correlated;

/// Specialized leakage-fault semantics.
///
/// Leakage remains distinct from ordinary Pauli/error semantics.
pub mod leakage;

/// Specialized erasure-fault semantics.
///
/// Erasure remains distinct from loss and leakage.
pub mod erasure;

/// Specialized loss-fault semantics.
///
/// Loss represents resource/information disappearance and is not silently
/// collapsed into erasure or leakage.
pub mod loss;

/// Ordered materialized collections of realized faults.
///
/// This module owns collection/resource-admission behavior, not individual
/// fault semantics.
pub mod batch;

// =============================================================================
// Stable public convenience exports
// =============================================================================
//
// Only canonical semantic roots are re-exported here.
//
// We deliberately do NOT re-export every helper type from every child module.
// That would make this composition boundary unstable and would encourage
// consumers to depend on implementation details.
//
// Child-module APIs remain available through:
//
//     crate::quantum::zqn::fault::<module>::<type>
//
// while the canonical `Fault` type is available directly as:
//
//     crate::quantum::zqn::fault::Fault
//
// =============================================================================

pub use self::fault::Fault;

// =============================================================================
// Internal compile-time contract tests
// =============================================================================

#[cfg(test)]
mod tests {
    //! Composition-boundary tests.
    //!
    //! These tests intentionally remain small. The semantic implementation
    //! tests belong to the individual child modules.
    //!
    //! The purpose here is to ensure that:
    //!
    //! - all required child modules are actually part of the fault subsystem;
    //! - the canonical `Fault` is exposed at the subsystem boundary;
    //! - the module contains no competing resource identity abstraction;
    //! - the composition boundary itself remains dependency-light.

    #![allow(clippy::missing_const_for_fn)]

    use super::Fault;

    #[test]
    fn canonical_fault_is_exposed() {
        fn assert_fault_type<T>() {}

        assert_fault_type::<Fault>();
    }

    #[test]
    fn all_fault_submodules_are_wired() {
        // Referencing the modules ensures that the declarations above remain
        // part of the compiled module graph.
        let _ = core::any::type_name::<super::fault::Fault>();
        let _ = core::any::type_name::<super::location::FaultLocation>();
        let _ = core::any::type_name::<super::classification::FaultEffectKind>();
        let _ = core::any::type_name::<super::correlated::CorrelatedFault>();
        let _ = core::any::type_name::<super::leakage::Leakage>();
        let _ = core::any::type_name::<super::erasure::Erasure>();
        let _ = core::any::type_name::<super::loss::Loss>();
        let _ = core::any::type_name::<super::batch::FaultBatch>();
    }
}