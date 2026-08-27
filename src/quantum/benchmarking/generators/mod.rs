//! Zamani Quantum Benchmarking — Circuit Generator Subsystem
//!
//! This module is the public module boundary for all benchmark workload and
//! circuit generators.
//!
//! # Architectural role
//!
//! `benchmarking::generators` owns deterministic construction of benchmark
//! workloads and logical quantum circuits. It does NOT execute circuits,
//! select hardware, perform routing, perform scheduling, perform statistical
//! analysis, calculate benchmark scores, or communicate with a backend.
//!
//! The authoritative dependency direction is:
//!
//! ```text
//!                         benchmark specification
//!                                  │
//!                                  ▼
//!                    benchmarking::generators
//!                                  │
//!              ┌───────────────────┼───────────────────┐
//!              │                   │                   │
//!              ▼                   ▼                   ▼
//!       logical circuits     application       benchmark fixtures
//!              │             workloads              │
//!              │                   │                │
//!              └───────────────────┼────────────────┘
//!                                  ▼
//!                         canonical Quantum IR
//!                                  │
//!              ┌───────────────────┼────────────────────┐
//!              ▼                   ▼                    ▼
//!           routing             scheduling           execution
//!              │                   │                    │
//!              └───────────────────┼────────────────────┘
//!                                  ▼
//!                              backend
//! ```
//!
//! # Responsibilities
//!
//! This subsystem owns:
//!
//! - deterministic benchmark randomness;
//! - deterministic workload construction;
//! - random logical circuit generation;
//! - Clifford sequence generation;
//! - Pauli generation;
//! - Quantum Volume circuit generation;
//! - mirror-circuit generation;
//! - application benchmark workload generation;
//! - reusable circuit-generation primitives;
//! - generator identity/version information;
//! - generator-level resource validation;
//! - generator reproducibility boundaries.
//!
//! It does NOT own:
//!
//! - benchmark configuration as a whole;
//! - benchmark execution;
//! - backend selection;
//! - hardware capability negotiation;
//! - topology-aware routing;
//! - scheduling;
//! - calibration;
//! - statistical inference;
//! - confidence intervals;
//! - fidelity calculation;
//! - Quantum Volume scoring;
//! - XEB scoring;
//! - randomized-benchmarking fitting;
//! - reporting;
//! - persistence;
//! - runtime lifecycle;
//! - frontend parsing.
//!
//! Those responsibilities belong to the corresponding benchmarking,
//! compiler, runtime, or hardware subsystems.
//!
//! # Canonical Quantum IR boundary
//!
//! All logical circuit generators must ultimately target the canonical
//! `crate::quantum::ir` representation.
//!
//! The dependency direction is:
//!
//! ```text
//! benchmarking::generators
//!          │
//!          ▼
//! quantum::ir
//! ```
//!
//! and NEVER:
//!
//! ```text
//! quantum::ir
//!       │
//!       ▼
//! benchmarking::generators
//! ```
//!
//! The Quantum IR already defines the canonical hardware-independent logical
//! representation. Generators must not introduce a second circuit
//! representation.
//!
//! # Hardware independence
//!
//! Generators operate on logical workloads.
//!
//! They must not inspect:
//!
//! - physical qubit topology;
//! - calibration data;
//! - pulse schedules;
//! - backend-specific gate availability;
//! - hardware identifiers;
//! - provider SDKs;
//! - queue state;
//! - device temperature;
//! - physical qubit placement;
//! - backend routing decisions.
//!
//! A generator may describe logical constraints required by a benchmark, but
//! physical realization is owned by downstream routing, scheduling, and
//! hardware layers.
//!
//! # Reproducibility
//!
//! Randomized generators must receive their randomness explicitly through the
//! canonical generator random subsystem.
//!
//! In particular:
//!
//! - no global RNG;
//! - no `thread_rng()` inside benchmark generators;
//! - no system-clock-derived circuit structure;
//! - no environment-derived circuit structure;
//! - no process-global mutable generator state;
//! - no hidden random seeds;
//! - no unordered iteration whose ordering affects generated circuits.
//!
//! The canonical random subsystem is `random`.
//!
//! Its versioned algorithm identity is part of benchmark provenance. A
//! generator that changes circuit semantics must also change its own generator
//! identity/version.
//!
//! # Parallel reproducibility
//!
//! Generators should support independent experiment/circuit streams.
//!
//! Conceptually:
//!
//! ```text
//! root benchmark seed
//!          │
//!          ├── benchmark domain
//!          │       ├── circuit 0
//!          │       ├── circuit 1
//!          │       ├── circuit 2
//!          │       └── ...
//!          │
//!          └── independent benchmark domains
//! ```
//!
//! This allows circuits to be generated in parallel without making the
//! generated circuit depend on execution order.
//!
//! # Generator categories
//!
//! The subsystem is deliberately divided into specialized modules:
//!
//! ```text
//! random.rs
//!     canonical deterministic random infrastructure
//!
//! deterministic.rs
//!     deterministic/non-random benchmark fixtures
//!
//! clifford.rs
//!     Clifford-group generation and inverse construction
//!
//! pauli.rs
//!     Pauli generation and Pauli-frame/twirling primitives
//!
//! random_circuits.rs
//!     generic randomized logical circuits
//!
//! qv.rs
//!     Quantum Volume-specific logical circuits
//!
//! mirror_circuits.rs
//!     mirror-circuit workloads
//!
//! application.rs
//!     application-level benchmark workloads
//! ```
//!
//! # Separation of generator and protocol
//!
//! A generator constructs the workload.
//!
//! A protocol determines what is done with that workload.
//!
//! For example:
//!
//! ```text
//! generators::qv
//!       │
//!       │ creates QV circuits
//!       ▼
//! protocols::quantum_volume
//!       │
//!       │ executes and evaluates
//!       ▼
//! statistics / metrics / result
//! ```
//!
//! Therefore `qv.rs` must not be turned into the QV scoring engine.
//!
//! Likewise:
//!
//! ```text
//! generators::random_circuits
//!       │
//!       ▼
//! protocols::xeb
//! ```
//!
//! and:
//!
//! ```text
//! generators::clifford
//!       │
//!       ▼
//! protocols::randomized_benchmarking
//! ```
//!
//! # Separation of logical and physical generation
//!
//! These generators construct logical circuits.
//!
//! A backend may subsequently require:
//!
//! ```text
//! logical circuit
//!       │
//!       ▼
//! optimization
//!       │
//!       ▼
//! routing
//!       │
//!       ▼
//! scheduling
//!       │
//!       ▼
//! physical execution
//! ```
//!
//! No generator in this module is permitted to bypass that architecture by
//! directly constructing physical backend instructions.
//!
//! # Resource safety
//!
//! Generator implementations must validate caller-controlled dimensions before
//! performing potentially large allocations.
//!
//! Resource checks belong primarily in the concrete generator because the
//! required resource model differs between:
//!
//! - random circuits;
//! - Clifford sequences;
//! - QV;
//! - application workloads;
//! - mirror circuits.
//!
//! The shared benchmarking limits subsystem remains authoritative for global
//! benchmark resource limits.
//!
//! Generator implementations must use checked arithmetic for:
//!
//! - qubit counts;
//! - depth;
//! - gate counts;
//! - operation counts;
//! - circuit counts;
//! - parameter counts;
//! - sequence lengths;
//! - memory-size estimates.
//!
//! A malformed benchmark request must fail deterministically rather than
//! causing an uncontrolled allocation.
//!
//! # Generator identity
//!
//! Every generator that produces externally meaningful benchmark workloads
//! should expose a stable generator identity/version.
//!
//! A semantically incompatible generation change must never silently reuse the
//! previous generator identity.
//!
//! This is particularly important for:
//!
//! - Quantum Volume;
//! - randomized benchmarking;
//! - XEB;
//! - mirror circuits;
//! - regression fixtures;
//! - published benchmark results.
//!
//! # Public module policy
//!
//! The child modules are public because they represent stable generator
//! boundaries used by benchmark protocols.
//!
//! This module intentionally does not flatten every child type into the
//! `generators` namespace.
//!
//! Prefer:
//!
//! ```text
//! quantum::benchmarking::generators::qv::QuantumVolumeGenerator
//! ```
//!
//! over making every implementation type a top-level re-export.
//!
//! This keeps names unambiguous and prevents accidental API coupling between
//! unrelated generator families.
//!
//! # Compatibility policy
//!
//! Existing generator modules are authoritative implementations and are
//! exposed without renaming:
//!
//! - `application`
//! - `clifford`
//! - `deterministic`
//! - `mirror_circuits`
//! - `pauli`
//! - `qv`
//! - `random`
//! - `random_circuits`
//!
//! In particular, the existing `random.rs` and `random_circuits.rs` split is
//! intentional:
//!
//! ```text
//! random.rs
//!     │
//!     └── deterministic randomness
//!
//! random_circuits.rs
//!     │
//!     └── actual random logical circuit construction
//! ```
//!
//! Do not merge those responsibilities into this module.
//!
//! # Integration with benchmarking core
//!
//! Generator implementations may consume:
//!
//! ```text
//! benchmarking::core::circuit
//! benchmarking::core::limits
//! ```
//!
//! where those modules exist in the established benchmarking architecture.
//!
//! The generator layer may therefore produce a canonical benchmark circuit
//! wrapper when the concrete generator supports it.
//!
//! This module itself deliberately does not depend on concrete core result,
//! execution, statistics, or reporting types.
//!
//! # Integration with protocols
//!
//! Protocols should consume generator modules through their public APIs.
//!
//! Examples:
//!
//! ```text
//! protocols::quantum_volume
//!     -> generators::qv
//!
//! protocols::randomized_benchmarking
//!     -> generators::clifford
//!
//! protocols::cycle_benchmarking
//!     -> generators::pauli
//!
//! protocols::xeb
//!     -> generators::random_circuits
//!
//! protocols::mirror
//!     -> generators::mirror_circuits
//!
//! applications/*
//!     -> generators::application
//! ```
//!
//! A protocol must not duplicate a generator's algorithm merely because it
//! needs a slightly different experiment configuration. Generator parameters
//! should be extended in the generator module when the capability is truly
//! generation-related.
//!
//! # Integration with execution
//!
//! This module has no dependency on execution.
//!
//! The correct direction is:
//!
//! ```text
//! generator
//!     │
//!     ▼
//! benchmark circuit/workload
//!     │
//!     ▼
//! execution request
//! ```
//!
//! Never:
//!
//! ```text
//! generator
//!     │
//!     ▼
//! backend executor
//! ```
//!
//! This allows generated workloads to be:
//!
//! - unit tested;
//! - serialized;
//! - inspected;
//! - hashed;
//! - compared;
//! - executed locally;
//! - executed by a simulator;
//! - executed on hardware;
//! - replayed later;
//! - used as deterministic CI fixtures.
//!
//! # Integration with provenance
//!
//! Provenance belongs to the higher-level benchmark result/experiment layer.
//!
//! Generators nevertheless provide the metadata required by that layer:
//!
//! - generator identifier;
//! - generator revision/version;
//! - random algorithm identifier;
//! - benchmark seed/domain information;
//! - workload/circuit identity;
//! - generator-specific ensemble information where applicable.
//!
//! A higher-level benchmark result must record these values rather than
//! reconstructing them after execution.
//!
//! # Integration with canonical Quantum IR
//!
//! The generator modules may use:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! including the canonical:
//!
//! - `QuantumCircuit`;
//! - `Gate`;
//! - `GateKind`;
//! - `Parameter`;
//! - `QubitId`;
//! - circuit identities;
//! - canonical validation.
//!
//! Generators must use the canonical IR constructors and validation rather
//! than creating a parallel benchmark-only gate model.
//!
//! # Integration with optimization, routing and scheduling
//!
//! Generator output is logical.
//!
//! Therefore:
//!
//! ```text
//! generators
//!      ↓
//! Quantum IR
//!      ↓
//! optimization
//!      ↓
//! routing
//!      ↓
//! scheduling
//!      ↓
//! execution
//! ```
//!
//! Benchmark protocols must retain enough provenance to distinguish benchmark
//! generation from downstream transformation. For example, two results using
//! identical generated circuits but different routing strategies must remain
//! distinguishable.
//!
//! # Integration with hardware
//!
//! Hardware capability negotiation occurs after generation.
//!
//! A generator should not silently modify a requested benchmark merely because
//! a particular backend cannot execute the generated workload.
//!
//! The normal flow is:
//!
//! ```text
//! generate logical workload
//!          │
//!          ▼
//! inspect backend capabilities
//!          │
//!      ┌───┴────┐
//!      │        │
//!   supported  unsupported
//!      │        │
//!      ▼        ▼
//!   execute   structured error
//! ```
//!
//! This prevents benchmark results from becoming incomparable because a
//! backend-specific generator silently changed the workload.
//!
//! # Application benchmark integration
//!
//! `application.rs` is the common generation boundary for application
//! benchmarks.
//!
//! Individual application benchmarks should normally live under:
//!
//! ```text
//! benchmarking::applications
//! ```
//!
//! while reusable construction primitives belong here.
//!
//! This prevents application-specific benchmark scoring from leaking into the
//! generator subsystem.
//!
//! # Quantum Volume integration
//!
//! `qv.rs` is the sole generator boundary for Quantum Volume circuits.
//!
//! The intended architecture is:
//!
//! ```text
//! generators::qv
//!      │
//!      ▼
//! protocols::quantum_volume
//!      │
//!      ▼
//! volume_estimator
//! ```
//!
//! The existing `volume_estimator.rs` intentionally remains a pure
//! mathematical/statistical component and explicitly does not generate or
//! execute circuits. 
//!
//! The QV generator must therefore never be moved into
//! `volume_estimator.rs`.
//!
//! # Randomized Benchmarking integration
//!
//! `clifford.rs` owns Clifford generation.
//!
//! The RB protocol owns:
//!
//! - sequence-length selection;
//! - execution;
//! - survival-probability collection;
//! - decay fitting;
//! - error estimation;
//! - confidence analysis.
//!
//! The generator owns only the sequence/circuit construction required by the
//! protocol.
//!
//! # Cycle Benchmarking integration
//!
//! `pauli.rs` supplies reusable Pauli primitives.
//!
//! Cycle benchmarking remains responsible for the experimental protocol,
//! cycle definition, execution, analysis, and uncertainty.
//!
//! This avoids coupling Pauli generation to one protocol family.
//!
//! # XEB integration
//!
//! `random_circuits.rs` supplies generic randomized logical circuits.
//!
//! XEB analysis remains responsible for combining:
//!
//! - generated circuit;
//! - ideal probability information;
//! - measured samples;
//! - XEB estimator;
//! - statistical uncertainty.
//!
//! The generator must never claim that an ideal distribution is available.
//! That is an execution/verification capability question.
//!
//! # Mirror-circuit integration
//!
//! `mirror_circuits.rs` owns construction of forward/inverse mirror
//! workloads. Execution and quality analysis belong to the corresponding
//! protocol.
//!
//! # Deterministic fixtures
//!
//! `deterministic.rs` provides deterministic workloads for:
//!
//! - unit tests;
//! - regression tests;
//! - cross-version compatibility fixtures;
//! - protocol validation;
//! - debugging;
//! - reproducibility verification.
//!
//! Deterministic fixtures must not accidentally depend on the randomized
//! generator implementation.
//!
//! # API stability
//!
//! Adding a new generator should normally require only:
//!
//! 1. adding the new `pub mod` declaration here;
//! 2. implementing its documented generator contract;
//! 3. registering it with the appropriate benchmark registry;
//! 4. integrating it with the relevant protocol/application layer.
//!
//! Existing generator APIs should not be changed merely because another
//! generator is added.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! The module intentionally uses only standard Rust module declarations and
//! documentation, so it adds no dependencies.
//!
//! # Security properties
//!
//! This module introduces no unsafe code and no global state.
//!
//! Child generators are responsible for:
//!
//! - validating limits;
//! - checked arithmetic;
//! - bounded allocations;
//! - deterministic failure;
//! - rejecting malformed parameters;
//! - validating generated Quantum IR;
//! - avoiding backend/network/filesystem access.
//!
//! Benchmark randomness is not cryptographic randomness. The canonical random
//! subsystem documents this distinction and must not be used for secrets,
//! authentication, cryptographic keys, or security nonces. The benchmark RNG
//! exists for reproducible scientific experiments. 
//!
//! # Testing policy
//!
//! Cross-generator integration tests should eventually live under the
//! benchmarking test subsystem rather than becoming hidden implementation
//! coupling here.
//!
//! This module should only test contracts that are specifically properties of
//! the module boundary itself.
//!
//! Recommended integration assertions:
//!
//! - every required generator module is reachable;
//! - module declarations remain stable;
//! - no generator requires execution merely to construct a workload;
//! - deterministic generators remain deterministic;
//! - randomized generators expose reproducibility metadata;
//! - generator identities remain non-empty.
//!
//! Concrete generation tests belong in their respective files.
//!
//! # Module inventory
//!
//! The complete current generator inventory is intentionally explicit here.
//! Do not use dynamic discovery or filesystem inspection.
//!
//! ```text
//! application
//! clifford
//! deterministic
//! mirror_circuits
//! pauli
//! qv
//! random
//! random_circuits
//! ```
//!
//! This explicit inventory is important for reproducible builds and for
//! ensuring that the compiler, documentation generator, and downstream
//! registry all see the same generator surface.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Generator modules
// =============================================================================
//
// Keep these declarations explicit and stable.
//
// The order is intentionally grouped by dependency role:
//
// 1. randomness;
// 2. deterministic generation;
// 3. reusable algebraic primitives;
// 4. generic circuit generation;
// 5. specialized circuit families;
// 6. application workloads.
//
// Rust module compilation does not rely on this order, but the organization
// communicates the intended architecture to maintainers.

// -----------------------------------------------------------------------------
// Randomness foundation
// -----------------------------------------------------------------------------

/// Canonical deterministic benchmark random-number infrastructure.
///
/// This module has no dependency on quantum IR, protocols, execution,
/// statistics, or hardware.
pub mod random;

// -----------------------------------------------------------------------------
// Deterministic generation
// -----------------------------------------------------------------------------

/// Deterministic circuit/workload generation for fixtures and reproducibility.
///
/// This module must remain independent from randomized benchmark generation.
pub mod deterministic;

// -----------------------------------------------------------------------------
// Reusable algebraic generator primitives
// -----------------------------------------------------------------------------

/// Clifford-group and Clifford-sequence generation.
///
/// Used primarily by randomized-benchmarking protocols and reusable by other
/// Clifford-based experiments.
pub mod clifford;

/// Pauli generation and Pauli-related circuit primitives.
///
/// Used by cycle benchmarking, randomized compiling, twirling experiments,
/// and other protocols requiring Pauli structure.
pub mod pauli;

// -----------------------------------------------------------------------------
// Generic circuit generation
// -----------------------------------------------------------------------------

/// Generic reproducible randomized logical-circuit generation.
///
/// This is the common random-circuit construction layer for XEB, random
/// circuit sampling, volumetric experiments, simulator stress workloads, and
/// other protocols requiring randomized circuits.
pub mod random_circuits;

// -----------------------------------------------------------------------------
// Specialized circuit families
// -----------------------------------------------------------------------------

/// Quantum Volume circuit generation.
///
/// This module constructs QV workloads only; QV execution and statistical
/// scoring remain outside this module.
pub mod qv;

/// Mirror-circuit generation.
///
/// Construction belongs here; execution and mirror-circuit fidelity analysis
/// belong to the protocol/analysis layers.
pub mod mirror_circuits;

// -----------------------------------------------------------------------------
// Application workloads
// -----------------------------------------------------------------------------

/// Application-level benchmark workload generation.
///
/// This provides reusable generation contracts and primitives for application
/// benchmarks such as Grover, QFT, VQE, QAOA, and user-defined workloads.
pub mod application;

// =============================================================================
// Controlled public prelude
// =============================================================================

/// Stable generator prelude.
///
/// This prelude intentionally exposes module boundaries rather than flattening
/// every generator implementation type into one namespace.
///
/// Use it when a consumer needs access to multiple generator families:
///
/// ```text
/// use crate::quantum::benchmarking::generators::prelude::*;
/// ```
///
/// For long-lived public APIs, explicit module paths remain preferable because
/// they make the generator family visible at the call site.
pub mod prelude {
    pub use super::application;
    pub use super::clifford;
    pub use super::deterministic;
    pub use super::mirror_circuits;
    pub use super::pauli;
    pub use super::qv;
    pub use super::random;
    pub use super::random_circuits;
}

// =============================================================================
// Generator subsystem metadata
// =============================================================================

/// Stable identifier for the generator subsystem itself.
///
/// This is NOT the identifier of an individual generator algorithm.
///
/// Individual generators must expose their own algorithm/generator identifiers
/// because changing QV generation, for example, must not invalidate unrelated
/// deterministic application generators.
pub const GENERATORS_SUBSYSTEM_ID: &str =
    "zamani.quantum.benchmarking.generators";

/// Public API version of the generator module boundary.
///
/// This changes only when the public module-level contract changes.
///
/// Individual generator algorithm revisions remain owned by their respective
/// child modules.
pub const GENERATORS_API_VERSION: u32 = 1;

/// Stable list of generator module identifiers.
///
/// This is deliberately static rather than dynamically generated so that the
/// module inventory is deterministic and available without runtime state.
pub const GENERATOR_MODULE_IDS: &[&str] = &[
    "application",
    "clifford",
    "deterministic",
    "mirror_circuits",
    "pauli",
    "qv",
    "random",
    "random_circuits",
];

/// Returns the stable generator subsystem identifier.
#[must_use]
pub const fn subsystem_id() -> &'static str {
    GENERATORS_SUBSYSTEM_ID
}

/// Returns the public API version of the generator subsystem.
#[must_use]
pub const fn api_version() -> u32 {
    GENERATORS_API_VERSION
}

/// Returns the stable list of generator module identifiers.
///
/// The returned slice is immutable and contains no runtime-generated state.
#[must_use]
pub const fn module_ids() -> &'static [&'static str] {
    GENERATOR_MODULE_IDS
}

// =============================================================================
// Integration tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subsystem_identity_is_stable() {
        assert_eq!(
            subsystem_id(),
            "zamani.quantum.benchmarking.generators"
        );
    }

    #[test]
    fn api_version_is_supported() {
        assert_eq!(api_version(), 1);
    }

    #[test]
    fn all_current_generator_modules_are_declared() {
        assert_eq!(
            module_ids(),
            &[
                "application",
                "clifford",
                "deterministic",
                "mirror_circuits",
                "pauli",
                "qv",
                "random",
                "random_circuits",
            ]
        );
    }

    #[test]
    fn generator_module_inventory_contains_no_empty_identifiers() {
        assert!(
            module_ids()
                .iter()
                .all(|identifier| !identifier.is_empty())
        );
    }

    #[test]
    fn generator_module_inventory_contains_unique_identifiers() {
        for (index, identifier) in module_ids().iter().enumerate() {
            assert!(
                !module_ids()[index + 1..]
                    .iter()
                    .any(|other| other == identifier),
                "duplicate generator module identifier: {identifier}"
            );
        }
    }

    #[test]
    fn public_generator_boundaries_are_reachable() {
        // These references intentionally validate module wiring only. They do
        // not instantiate generator implementations or require a backend.
        let _ = application::module_path!();
        let _ = clifford::module_path!();
        let _ = deterministic::module_path!();
        let _ = mirror_circuits::module_path!();
        let _ = pauli::module_path!();
        let _ = qv::module_path!();
        let _ = random::module_path!();
        let _ = random_circuits::module_path!();
    }

    #[test]
    fn prelude_exposes_all_generator_boundaries() {
        use super::prelude::*;

        let _ = application::module_path!();
        let _ = clifford::module_path!();
        let _ = deterministic::module_path!();
        let _ = mirror_circuits::module_path!();
        let _ = pauli::module_path!();
        let _ = qv::module_path!();
        let _ = random::module_path!();
        let _ = random_circuits::module_path!();
    }
}