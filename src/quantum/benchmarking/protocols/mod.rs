//! Zamani Quantum Benchmarking — Protocol Boundary
//!
//! Authoritative module wiring for protocol-level quantum benchmarking.
//!
//! This file owns only protocol-module composition and stable protocol IDs. It
//! does not own protocol mathematics, circuit generation, execution, generic
//! statistics, metrics, reporting, persistence, hardware communication, or
//! compiler transformations.
//!
//! # Architecture
//!
//! ```text
//! Zamani benchmark specification
//!             │
//!             ▼
//!      benchmarking::core
//!             │
//!      ┌──────┼──────┐
//!      ▼      ▼      ▼
//! generators execution protocols (this module)
//!                    │
//!          ┌─────────┼─────────┐
//!          ▼         ▼         ▼
//!      statistics  metrics    result
//! ```
//!
//! Protocols define the scientific/experimental meaning of a benchmark. They
//! consume stable generator and execution contracts and produce the universal
//! benchmark result model.
//!
//! # Ownership boundary
//!
//! This module owns:
//!
//! - protocol module wiring;
//! - stable protocol identifiers;
//! - deterministic protocol discovery;
//! - protocol-module catalog validation.
//!
//! This module does NOT own:
//!
//! - Quantum IR;
//! - frontend parsing;
//! - compiler optimization;
//! - routing;
//! - scheduling;
//! - hardware communication;
//! - simulator implementation;
//! - generic probability/fidelity calculations;
//! - generic confidence intervals;
//! - generic regression fitting;
//! - result serialization;
//! - persistence;
//! - runtime lifecycle;
//! - application benchmark implementations;
//! - QEC implementations.
//!
//! Those responsibilities remain in their owning subsystems.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! protocol → generators → quantum::ir
//! protocol → core::execution → runtime/hardware/simulator
//! protocol → statistics / metrics / validation
//! protocol → core::result
//! ```
//!
//! The protocol layer must never become a second hardware abstraction, second
//! circuit representation, or second statistical framework.
//!
//! # Protocol portfolio
//!
//! This module exposes the protocol implementations currently present in the
//! repository:
//!
//! - Quantum Volume;
//! - randomized benchmarking;
//! - interleaved randomized benchmarking;
//! - simultaneous randomized benchmarking;
//! - purity randomized benchmarking;
//! - leakage randomized benchmarking;
//! - cycle benchmarking;
//! - layer fidelity;
//! - cross-entropy benchmarking (XEB);
//! - random circuit sampling;
//! - mirror circuits;
//! - SPAM characterization;
//! - gate fidelity;
//! - process fidelity;
//! - coherence;
//! - crosstalk;
//! - drift/stability;
//! - tomography.
//!
//! Application benchmarks and QEC benchmarks intentionally remain sibling
//! modules under `benchmarking::applications` and `benchmarking::qec`.
//!
//! # Protocol versus generator
//!
//! A protocol consumes a generator. It must not duplicate the generator's
//! algorithm.
//!
//! ```text
//! protocols::quantum_volume
//!          │
//!          ▼
//! generators::qv
//!          │
//!          ▼
//! quantum::ir
//! ```
//!
//! ```text
//! protocols::randomized_benchmarking
//!          │
//!          ▼
//! generators::clifford
//! ```
//!
//! ```text
//! protocols::cycle_benchmarking
//!          │
//!          ▼
//! generators::pauli
//! ```
//!
//! ```text
//! protocols::xeb
//!          │
//!          ▼
//! generators::random_circuits
//! ```
//!
//! ```text
//! protocols::mirror
//!          │
//!          ▼
//! generators::mirror_circuits
//! ```
//!
//! # Protocol versus execution
//!
//! Protocols define what must be executed. The execution subsystem defines how
//! execution is performed.
//!
//! ```text
//! protocol experiment
//!        │
//!        ▼
//! core::execution request
//!        │
//!        ▼
//! runtime / simulator / hardware
//!        │
//!        ▼
//! core::observation
//! ```
//!
//! A protocol must not directly open network connections, invoke provider SDKs,
//! mutate hardware state, or depend on a concrete simulator.
//!
//! # Protocol versus statistics
//!
//! Protocols own the scientific meaning of an experiment. Shared statistical
//! modules own reusable mathematical machinery.
//!
//! For example:
//!
//! ```text
//! randomized_benchmarking
//!          │
//!          ├── RB experiment semantics
//!          ├── survival definition
//!          └── RB assumptions
//!
//! benchmarking::statistics
//!          │
//!          ├── regression
//!          ├── confidence intervals
//!          └── aggregation
//! ```
//!
//! This prevents each protocol from inventing incompatible uncertainty and
//! statistical conventions.
//!
//! # Quantum Volume
//!
//! `quantum_volume.rs` owns the QV experimental protocol.
//!
//! ```text
//! generators::qv
//!       │
//!       ▼
//! protocols::quantum_volume
//!       │
//!       ▼
//! volume_estimator
//!       │
//!       ▼
//! core::result
//! ```
//!
//! The existing `volume_estimator.rs` remains the reusable mathematical/QV
//! estimator. It must not become a circuit generator or backend executor.
//!
//! # Randomized benchmarking family
//!
//! The RB family is intentionally separated:
//!
//! - `randomized_benchmarking` — reference RB;
//! - `interleaved_rb` — target-gate interleaved RB;
//! - `simultaneous_rb` — simultaneous/reference comparison;
//! - `purity_rb` — purity/unitarity-oriented RB;
//! - `leakage_rb` — leakage characterization.
//!
//! Shared Clifford construction belongs to `generators::clifford`.
//! Shared regression/confidence machinery belongs to `statistics`.
//!
//! Individual protocols retain their own assumptions, validation rules, and
//! scientific interpretation.
//!
//! # Cycle and layer protocols
//!
//! `cycle_benchmarking` and `layer_fidelity` operate on parallel cycles/layers.
//! Their implementations must preserve parallelism metadata because replacing
//! a parallel layer with an arbitrary sequential representation can change the
//! meaning of the benchmark.
//!
//! # XEB and random circuit sampling
//!
//! `random_circuit_sampling` owns the sampling experiment boundary.
//!
//! `xeb` owns cross-entropy/XEB analysis.
//!
//! XEB implementations must distinguish:
//!
//! - exact ideal probabilities;
//! - approximate ideal probabilities;
//! - partial ideal information;
//! - unavailable ideal information.
//!
//! An approximation must never silently be represented as an exact result.
//!
//! # Mirror circuits
//!
//! `mirror` owns the mirror-circuit protocol.
//!
//! Construction of forward/inverse workloads belongs to
//! `generators::mirror_circuits`.
//!
//! # Device characterization
//!
//! The following are capability-dependent characterization protocols:
//!
//! - `spam`;
//! - `gate_fidelity`;
//! - `process_fidelity`;
//! - `coherence`;
//! - `crosstalk`;
//! - `drift`.
//!
//! Their presence in this module does not imply that every backend supports
//! them. Capability negotiation must occur before execution.
//!
//! # Tomography
//!
//! `tomography` is a capability-dependent diagnostic protocol rather than a
//! universally scalable benchmark. Its implementation must enforce shared
//! benchmark resource limits before allocating or executing large experiments.
//!
//! # Backend neutrality
//!
//! Protocols must not assume that every target has:
//!
//! - a conventional qubit count;
//! - a fixed gate set;
//! - computational-basis measurements;
//! - state-vector access;
//! - density-matrix access;
//! - mid-circuit measurement;
//! - calibration data;
//! - a conventional circuit-execution model.
//!
//! Capability requirements must be explicit. Unsupported capabilities must
//! result in structured errors rather than silent workload changes.
//!
//! The surrounding architecture can therefore support:
//!
//! - superconducting systems;
//! - trapped-ion systems;
//! - neutral-atom systems;
//! - photonic systems;
//! - spin/semiconductor systems;
//! - topological systems;
//! - analog quantum systems;
//! - quantum annealers;
//! - logical/fault-tolerant systems;
//! - CPU simulators;
//! - GPU simulators;
//! - state-vector simulators;
//! - density-matrix simulators;
//! - stabilizer simulators;
//! - tensor-network simulators;
//! - emulators;
//! - hybrid quantum-classical systems.
//!
//! # Reproducibility
//!
//! Randomized protocols must use explicit deterministic randomness and retain
//! enough provenance for replay and independent analysis.
//!
//! At minimum, protocol execution should preserve:
//!
//! - protocol identifier;
//! - protocol version;
//! - generator identifier/version;
//! - seed/domain information;
//! - configuration identity;
//! - workload/circuit identity;
//! - backend identity where applicable;
//! - compiler configuration where applicable.
//!
//! This module itself contains no randomness and no mutable global state.
//!
//! # Resource safety
//!
//! Resource-intensive protocol implementations must validate limits before
//! generating or requesting workloads.
//!
//! This is especially important for:
//!
//! - Quantum Volume;
//! - XEB;
//! - random circuit sampling;
//! - long RB sequences;
//! - tomography;
//! - large cycle/layer experiments.
//!
//! This module itself performs no dynamic allocation and contains no
//! unbounded execution loop.
//!
//! # Error handling
//!
//! Module wiring must not introduce a second error hierarchy.
//!
//! Concrete protocols should use the canonical benchmarking error contracts
//! established by `benchmarking::core::errors` or a domain error that converts
//! explicitly into the canonical hierarchy.
//!
//! Protocols must not:
//!
//! - use `println!`/`eprintln!` for library diagnostics;
//! - terminate the process;
//! - hide execution failures;
//! - turn failed execution into a fabricated successful result.
//!
//! # Registry integration
//!
//! This module deliberately does not depend on `benchmarking::registry`.
//!
//! The intended direction is:
//!
//! ```text
//! protocols/*
//!      │
//!      ▼
//! registry/builtin.rs
//!      │
//!      ▼
//! registry/registry.rs
//! ```
//!
//! Merely having a source file under this directory does not mean that the
//! protocol is executable through the universal benchmark registry.
//!
//! A protocol should be registered as executable only when it satisfies the
//! stable `core::benchmark::Benchmark` contract and an explicit adapter/factory
//! exists.
//!
//! # Application and QEC integration
//!
//! Application and QEC benchmarks remain siblings:
//!
//! ```text
//! benchmarking/
//! ├── protocols/
//! ├── applications/
//! └── qec/
//! ```
//!
//! They share the same universal benchmark lifecycle and result model but do
//! not belong inside this module.
//!
//! # Zamani-language integration
//!
//! Future Zamani syntax should lower to stable benchmark/protocol identifiers
//! and universal benchmark configuration.
//!
//! ```text
//! benchmark quantum_volume { ... }
//!              │
//!              ▼
//!       stable protocol ID
//!              │
//!              ▼
//!       benchmark registry
//!              │
//!              ▼
//! protocols::quantum_volume
//! ```
//!
//! The language layer must not need to know protocol implementation internals.
//!
//! # Public API policy
//!
//! Child protocol modules are public because they are explicit integration
//! boundaries.
//!
//! This module intentionally does not flatten all protocol types into the
//! parent namespace. Prefer:
//!
//! ```text
//! quantum::benchmarking::protocols::quantum_volume::...
//! ```
//!
//! rather than broad wildcard re-exports.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only.
//!
//! No nightly features are required.

#![allow(clippy::module_inception)]

// =============================================================================
// Stable module identity
// =============================================================================

/// Stable identifier for this module boundary.
pub const PROTOCOLS_MODULE_ID: &str = "zamani.quantum.benchmarking.protocols";

/// Version of the parent protocol catalog contract.
///
/// This version describes the module/catalog API and is independent of the
/// scientific version of an individual protocol.
pub const PROTOCOLS_MODULE_VERSION: &str = "1.0.0";

// =============================================================================
// Stable protocol identifiers
// =============================================================================

/// Quantum Volume.
pub const ID_QUANTUM_VOLUME: &str = "quantum_volume";

/// Standard randomized benchmarking.
pub const ID_RANDOMIZED_BENCHMARKING: &str = "randomized_benchmarking";

/// Interleaved randomized benchmarking.
pub const ID_INTERLEAVED_RB: &str = "interleaved_rb";

/// Simultaneous randomized benchmarking.
pub const ID_SIMULTANEOUS_RB: &str = "simultaneous_rb";

/// Purity randomized benchmarking.
pub const ID_PURITY_RB: &str = "purity_rb";

/// Leakage randomized benchmarking.
pub const ID_LEAKAGE_RB: &str = "leakage_rb";

/// Cycle benchmarking.
pub const ID_CYCLE_BENCHMARKING: &str = "cycle_benchmarking";

/// Layer fidelity.
pub const ID_LAYER_FIDELITY: &str = "layer_fidelity";

/// Cross-entropy benchmarking.
pub const ID_XEB: &str = "xeb";

/// Random circuit sampling.
pub const ID_RANDOM_CIRCUIT_SAMPLING: &str = "random_circuit_sampling";

/// Mirror circuits.
pub const ID_MIRROR: &str = "mirror";

/// SPAM characterization.
pub const ID_SPAM: &str = "spam";

/// Gate fidelity characterization.
pub const ID_GATE_FIDELITY: &str = "gate_fidelity";

/// Process fidelity characterization.
pub const ID_PROCESS_FIDELITY: &str = "process_fidelity";

/// Coherence characterization.
pub const ID_COHERENCE: &str = "coherence";

/// Crosstalk characterization.
pub const ID_CROSSTALK: &str = "crosstalk";

/// Drift/stability characterization.
pub const ID_DRIFT: &str = "drift";

/// State/process tomography.
pub const ID_TOMOGRAPHY: &str = "tomography";

/// Number of protocol modules wired by this boundary.
pub const PROTOCOL_COUNT: usize = 18;

/// Complete deterministic catalog of protocol identifiers.
///
/// The ordering is stable and is suitable for discovery, documentation,
/// validation, registry integration, CI checks, and language-level discovery.
/// It does not imply execution priority or scientific ranking.
pub const PROTOCOL_IDS: &[&str] = &[
    ID_QUANTUM_VOLUME,
    ID_RANDOMIZED_BENCHMARKING,
    ID_INTERLEAVED_RB,
    ID_SIMULTANEOUS_RB,
    ID_PURITY_RB,
    ID_LEAKAGE_RB,
    ID_CYCLE_BENCHMARKING,
    ID_LAYER_FIDELITY,
    ID_XEB,
    ID_RANDOM_CIRCUIT_SAMPLING,
    ID_MIRROR,
    ID_SPAM,
    ID_GATE_FIDELITY,
    ID_PROCESS_FIDELITY,
    ID_COHERENCE,
    ID_CROSSTALK,
    ID_DRIFT,
    ID_TOMOGRAPHY,
];

/// Returns `true` if `id` belongs to the protocol catalog owned by this
/// module.
///
/// This operation is allocation-free, deterministic, and side-effect-free.
#[inline]
pub fn is_known_protocol_id(id: &str) -> bool {
    PROTOCOL_IDS.iter().any(|candidate| *candidate == id)
}

// =============================================================================
// Concrete protocol module wiring
// =============================================================================

/// Quantum Volume experimental protocol.
///
/// Generation belongs to `generators::qv`; reusable QV mathematical
/// estimation belongs to `volume_estimator`.
pub mod quantum_volume;

/// Standard randomized benchmarking protocol.
pub mod randomized_benchmarking;

/// Interleaved randomized benchmarking protocol.
pub mod interleaved_rb;

/// Simultaneous randomized benchmarking protocol.
pub mod simultaneous_rb;

/// Purity randomized benchmarking protocol.
pub mod purity_rb;

/// Leakage randomized benchmarking protocol.
pub mod leakage_rb;

/// Cycle benchmarking protocol.
pub mod cycle_benchmarking;

/// Layer-fidelity protocol.
pub mod layer_fidelity;

/// Cross-entropy benchmarking protocol.
pub mod xeb;

/// Random-circuit-sampling protocol.
pub mod random_circuit_sampling;

/// Mirror-circuit protocol.
pub mod mirror;

/// SPAM characterization protocol.
pub mod spam;

/// Gate-fidelity characterization protocol.
pub mod gate_fidelity;

/// Process-fidelity characterization protocol.
pub mod process_fidelity;

/// Coherence characterization protocol.
pub mod coherence;

/// Crosstalk characterization protocol.
pub mod crosstalk;

/// Drift/stability characterization protocol.
pub mod drift;

/// State/process tomography protocol.
pub mod tomography;

// =============================================================================
// Controlled prelude
// =============================================================================

/// Controlled protocol prelude.
///
/// Only modules and stable protocol identifiers are exported. Concrete
/// implementation types remain under their owning protocol module.
pub mod prelude {
    pub use super::{
        coherence,
        crosstalk,
        cycle_benchmarking,
        drift,
        gate_fidelity,
        interleaved_rb,
        layer_fidelity,
        leakage_rb,
        mirror,
        process_fidelity,
        purity_rb,
        quantum_volume,
        randomized_benchmarking,
        random_circuit_sampling,
        simultaneous_rb,
        spam,
        tomography,
        xeb,
    };

    pub use super::{
        ID_COHERENCE,
        ID_CROSSTALK,
        ID_CYCLE_BENCHMARKING,
        ID_DRIFT,
        ID_GATE_FIDELITY,
        ID_INTERLEAVED_RB,
        ID_LAYER_FIDELITY,
        ID_LEAKAGE_RB,
        ID_MIRROR,
        ID_PROCESS_FIDELITY,
        ID_PURITY_RB,
        ID_QUANTUM_VOLUME,
        ID_RANDOMIZED_BENCHMARKING,
        ID_RANDOM_CIRCUIT_SAMPLING,
        ID_SIMULTANEOUS_RB,
        ID_SPAM,
        ID_TOMOGRAPHY,
        ID_XEB,
    };
}

// =============================================================================
// Architectural boundary tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_count_matches_protocol_count() {
        assert_eq!(PROTOCOL_COUNT, 18);
        assert_eq!(PROTOCOL_IDS.len(), PROTOCOL_COUNT);
    }

    #[test]
    fn catalog_identifiers_are_non_empty() {
        for id in PROTOCOL_IDS {
            assert!(!id.is_empty());
        }
    }

    #[test]
    fn catalog_identifiers_are_known() {
        for id in PROTOCOL_IDS {
            assert!(is_known_protocol_id(id));
        }
    }

    #[test]
    fn catalog_identifiers_are_unique() {
        for (index, id) in PROTOCOL_IDS.iter().enumerate() {
            assert!(
                !PROTOCOL_IDS[..index].contains(id),
                "duplicate protocol identifier: {id}"
            );
        }
    }

    #[test]
    fn unknown_protocol_identifiers_are_rejected() {
        assert!(!is_known_protocol_id(""));
        assert!(!is_known_protocol_id("unknown_protocol"));
        assert!(!is_known_protocol_id("__invalid_zamani_protocol__"));
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            PROTOCOLS_MODULE_ID,
            "zamani.quantum.benchmarking.protocols"
        );
        assert_eq!(PROTOCOLS_MODULE_VERSION, "1.0.0");
    }
}