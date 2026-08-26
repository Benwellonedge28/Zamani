//! Zamani Quantum Benchmarking — Core API
//!
//! Canonical public boundary for the quantum benchmarking subsystem.
//!
//! # Purpose
//!
//! `core` defines the stable, backend-independent contracts shared by every
//! Zamani quantum benchmark:
//!
//! - benchmark definitions;
//! - benchmark configuration;
//! - workloads;
//! - circuit metadata;
//! - experiment identity;
//! - execution contracts;
//! - raw observations;
//! - normalized results;
//! - dimensions;
//! - metrics;
//! - provenance;
//! - reproducibility;
//! - resource limits;
//! - benchmarking errors.
//!
//! This module contains **API wiring only**. Benchmark mathematics,
//! statistical analysis, circuit generation, backend execution, reporting,
//! and protocol-specific behavior belong to their respective sibling
//! subsystems.
//!
//! # Architectural boundary
//!
//! The intended dependency direction is:
//!
//! ```text
//!                         Zamani language
//!                              │
//!                              ▼
//!                    stdlib::quantum / frontend
//!                              │
//!                              ▼
//!                  quantum::benchmarking
//!                              │
//!                              ▼
//!                  quantum::benchmarking::core
//!                              │
//!        ┌─────────────────────┼─────────────────────┐
//!        │                     │                     │
//!        ▼                     ▼                     ▼
//!     generators          execution             protocols
//!        │                     │                     │
//!        │                     ▼                     │
//!        │                  hardware                │
//!        │                     │                     │
//!        └──────────────┬──────┴─────────────────────┘
//!                       ▼
//!                  statistics
//!                       │
//!                       ▼
//!                    metrics
//!                       │
//!                       ▼
//!                     result
//!                       │
//!                       ▼
//!               analysis / reporting
//! ```
//!
//! The benchmarking core may consume the canonical Quantum IR:
//!
//! ```text
//! quantum::benchmarking::core::circuit
//!                         │
//!                         ▼
//!                  quantum::ir
//! ```
//!
//! but the dependency must never be reversed:
//!
//! ```text
//! quantum::ir ─X─> quantum::benchmarking
//! ```
//!
//! The Quantum IR remains the authoritative representation of quantum
//! programs. Benchmarking merely describes, executes, observes and analyzes
//! experiments involving those programs.
//!
//! # Ownership boundaries
//!
//! `core` owns:
//!
//! - benchmark identity;
//! - experiment identity;
//! - benchmark configuration;
//! - workload classification;
//! - circuit benchmark metadata;
//! - execution contracts;
//! - normalized observations;
//! - normalized benchmark results;
//! - metric representation;
//! - dimensional quantities;
//! - provenance;
//! - reproducibility contracts;
//! - resource/safety limits;
//! - benchmark-wide error vocabulary.
//!
//! `core` does **not** own:
//!
//! - OpenQASM parsing;
//! - quantum-language parsing;
//! - Quantum IR implementation;
//! - circuit optimization;
//! - routing;
//! - scheduling;
//! - hardware communication;
//! - simulator implementation;
//! - benchmark-specific mathematics;
//! - statistical fitting;
//! - random-circuit generation;
//! - QEC implementation;
//! - report formatting.
//!
//! Those responsibilities remain in their owning subsystems.
//!
//! # Stable dependency layers
//!
//! The core files are intentionally arranged into four conceptual layers.
//!
//! ## Layer 0 — independent primitives
//!
//! ```text
//! errors
//! limits
//! dimension
//! metric
//! provenance
//! reproducibility
//! config
//! ```
//!
//! These files define vocabulary and constraints and should not depend on
//! protocol implementations.
//!
//! ## Layer 1 — experiment description
//!
//! ```text
//! workload
//! circuit
//! observation
//! ```
//!
//! These describe what is being benchmarked and what can be observed.
//!
//! ## Layer 2 — execution and results
//!
//! ```text
//! execution
//! experiment
//! result
//! ```
//!
//! These connect benchmark definitions to execution without coupling the
//! benchmark framework to any particular backend.
//!
//! ## Layer 3 — benchmark lifecycle
//!
//! ```text
//! benchmark
//! ```
//!
//! This is the highest-level protocol contract and consumes the lower-level
//! core abstractions.
//!
//! # Production invariants
//!
//! All production benchmark implementations must preserve the following
//! invariants:
//!
//! 1. No benchmark-specific error hierarchy may replace `core::errors`.
//! 2. No benchmark-specific result envelope may replace `core::result`.
//! 3. No protocol may return an unexplained naked floating-point metric.
//! 4. Measurements must retain their sample-count/provenance context.
//! 5. Randomized experiments must have explicit reproducibility metadata.
//! 6. Resource-consuming operations must respect `core::limits`.
//! 7. Backend capabilities must be validated before execution.
//! 8. Raw observations must remain distinguishable from derived metrics.
//! 9. Statistical uncertainty must never be silently discarded.
//! 10. A failed experiment must be representable without fabricating a
//!     successful result.
//! 11. Partial execution must be distinguishable from complete execution.
//! 12. Benchmark configuration must be serializable through its owning API.
//! 13. Results must be independently analyzable after execution.
//! 14. Benchmarking must not mutate the canonical Quantum IR.
//! 15. Benchmarking must not assume that every quantum backend is a
//!     gate-model, qubit-counting device.
//!
//! # Backend neutrality
//!
//! The core API must support benchmark targets including:
//!
//! - gate-model quantum computers;
//! - superconducting systems;
//! - trapped-ion systems;
//! - neutral-atom systems;
//! - photonic systems;
//! - spin/semiconductor systems;
//! - topological systems;
//! - quantum annealers;
//! - analog quantum systems;
//! - logical-qubit/fault-tolerant systems;
//! - CPU simulators;
//! - GPU simulators;
//! - state-vector simulators;
//! - density-matrix simulators;
//! - stabilizer simulators;
//! - tensor-network simulators;
//! - emulators;
//! - hybrid quantum-classical systems.
//!
//! Consequently, core types must describe capabilities and observations
//! abstractly rather than assuming that every execution produces ordinary
//! qubit bitstrings.
//!
//! # Error handling
//!
//! This module does not introduce a second error framework. All core
//! implementations must use the canonical errors defined by `errors.rs` or
//! expose domain-specific errors that can be converted into the canonical
//! benchmarking error hierarchy.
//!
//! Library code must not print diagnostics directly. In particular, core
//! modules must not use `println!`, `eprintln!`, logging side effects, or
//! process termination as an error mechanism.
//!
//! # Resource safety
//!
//! Benchmark configuration is untrusted input from the perspective of a
//! library boundary. Limits must therefore be validated before allocating
//! benchmark workloads, generating circuits, allocating observation buffers,
//! or requesting large shot counts.
//!
//! This is particularly important for:
//!
//! - random circuit benchmarks;
//! - Quantum Volume;
//! - XEB;
//! - tomography;
//! - bootstrap statistics;
//! - volumetric experiments;
//! - QEC threshold sweeps;
//! - user-defined Zamani benchmarks.
//!
//! # Reproducibility
//!
//! A benchmark's reproducibility identity must be based on the complete
//! experiment definition rather than merely a random seed.
//!
//! At minimum, the identity should account for:
//!
//! - benchmark identity/version;
//! - configuration;
//! - workload;
//! - generator identity/version;
//! - seed;
//! - relevant compiler configuration;
//! - relevant backend identity;
//! - protocol version.
//!
//! Hardware-specific provenance such as calibration identity and timestamp
//! belongs in the provenance contract without being incorrectly treated as a
//! deterministic circuit-generation input.
//!
//! # Raw observations versus metrics
//!
//! The architecture deliberately separates:
//!
//! ```text
//! raw observation
//!       │
//!       ▼
//! statistical analysis
//!       │
//!       ▼
//! derived metric
//! ```
//!
//! For example, a Quantum Volume experiment may retain:
//!
//! ```text
//! samples
//! heavy-output counts
//! circuit identity
//! backend identity
//! timing
//! calibration
//! ```
//!
//! while the protocol later derives:
//!
//! ```text
//! heavy-output probability
//! confidence interval
//! pass/fail
//! quantum volume
//! ```
//!
//! This separation makes re-analysis possible without re-running hardware.
//!
//! # Quantum Volume integration
//!
//! The existing `volume_estimator.rs` remains a mathematical/statistical
//! component. It must not become responsible for:
//!
//! - circuit generation;
//! - backend execution;
//! - hardware discovery;
//! - reporting;
//! - experiment orchestration.
//!
//! The eventual integration is:
//!
//! ```text
//! core::config
//!       │
//!       ▼
//! protocols::quantum_volume
//!       │
//!       ├── generators::qv
//!       │
//!       ├── execution::executor
//!       │
//!       ▼
//! volume_estimator
//!       │
//!       ▼
//! core::metric / core::result
//! ```
//!
//! This preserves the useful boundary already established by the existing
//! estimator.
//!
//! # Integration with Quantum IR
//!
//! `core::circuit` may wrap or reference the canonical
//! `quantum::ir::QuantumCircuit` and its associated identity/resource
//! information.
//!
//! Benchmarking must never define a competing circuit representation that
//! becomes more authoritative than Quantum IR.
//!
//! # Integration with hardware
//!
//! `core::execution` defines what a benchmark executor must accept and return.
//! The actual backend implementation remains outside `core`.
//!
//! Conceptually:
//!
//! ```text
//! BenchmarkExperiment
//!        │
//!        ▼
//! BenchmarkExecutor
//!        │
//!        ▼
//! quantum::hardware / runtime / simulator
//!        │
//!        ▼
//! BenchmarkObservation
//! ```
//!
//! # Integration with the Zamani language
//!
//! Future Zamani syntax such as:
//!
//! ```text
//! benchmark quantum_volume { ... }
//! benchmark randomized_benchmarking { ... }
//! benchmark my_algorithm { ... }
//! ```
//!
//! must lower into these stable core contracts rather than directly into
//! protocol implementation details.
//!
//! Therefore changing the Zamani syntax should not require redesigning the
//! core benchmark result model.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust only.
//!
//! No nightly features are required or permitted by this module.
//!
//! # Public API policy
//!
//! The individual implementation modules are public because protocol and
//! integration code needs explicit ownership boundaries. However, downstream
//! code should normally consume the re-exported types from this module or the
//! controlled `prelude` rather than reaching into implementation details.
//!
//! This file intentionally contains no protocol implementation logic.


// =============================================================================
// Core Layer 0 — independent primitives
// =============================================================================

/// Canonical benchmarking error vocabulary.
///
/// This module is the single error boundary for the benchmarking subsystem.
pub mod errors;

/// Resource and execution safety limits.
///
/// Prevents pathological benchmark configurations from causing uncontrolled
/// allocation, execution, or statistical workloads.
pub mod limits;

/// Universal benchmark dimensions.
///
/// Defines quantities such as qubits, depth, gates, time, fidelity,
/// throughput, and resource dimensions.
pub mod dimension;

/// Universal benchmark metric representation.
///
/// Metrics carry values together with units, uncertainty and quality
/// information instead of exposing unexplained primitive numbers.
pub mod metric;

/// Scientific and execution provenance.
///
/// Records the information necessary to identify how an experiment was
/// produced and under which backend/compiler/calibration environment.
pub mod provenance;

/// Reproducibility and experiment fingerprinting.
///
/// Defines deterministic identities for benchmark configurations, generated
/// workloads and results.
pub mod reproducibility;

/// Universal benchmark configuration.
///
/// Contains benchmark-independent execution, statistical, resource and
/// validation settings.
pub mod config;


// =============================================================================
// Core Layer 1 — experiment description
// =============================================================================

/// Backend-independent benchmark workload description.
///
/// Supports circuit, application, QEC, analog and other workload classes
/// without assuming that every quantum system is gate based.
pub mod workload;

/// Benchmark circuit metadata and Quantum IR integration.
///
/// This module is the benchmarking view of a circuit; the canonical quantum
/// semantics remain owned by `quantum::ir`.
pub mod circuit;

/// Raw and normalized benchmark observations.
///
/// Observations represent what execution produced before protocol-specific
/// analysis derives higher-level metrics.
pub mod observation;


// =============================================================================
// Core Layer 2 — execution and experiment lifecycle
// =============================================================================

/// Backend-independent execution contract.
///
/// Defines the interface between benchmark experiments and concrete
/// simulators, runtimes and quantum hardware.
pub mod execution;

/// Complete benchmark experiment description.
///
/// Combines benchmark configuration, workload, execution requirements and
/// experiment identity without embedding protocol-specific mathematics.
pub mod experiment;

/// Canonical benchmark result envelope.
///
/// Provides the common result representation consumed by reporting, analysis,
/// regression and the Zamani language integration.
pub mod result;


// =============================================================================
// Core Layer 3 — benchmark lifecycle
// =============================================================================

/// Stable benchmark protocol contract.
///
/// This is the highest-level abstraction implemented by concrete benchmark
/// protocols such as Quantum Volume, RB, XEB, application benchmarks and QEC
/// benchmarks.
pub mod benchmark;


// =============================================================================
// Controlled public prelude
// =============================================================================

/// Stable collection of the primary benchmarking-core modules.
///
/// Prefer explicit imports from `core` when API ownership matters. Use this
/// prelude for application/compiler integration code that needs the complete
/// core vocabulary.
///
/// The prelude exports modules rather than flattening every individual type.
/// This prevents future additions to one module from silently creating name
/// collisions in downstream code and keeps ownership visible.
///
/// Example:
///
/// ```ignore
/// use crate::quantum::benchmarking::core::prelude::*;
///
/// let configuration = config::BenchmarkConfig::default();
/// ```
pub mod prelude {
    pub use super::{
        benchmark,
        circuit,
        config,
        dimension,
        errors,
        execution,
        experiment,
        limits,
        metric,
        observation,
        provenance,
        reproducibility,
        result,
        workload,
    };
}


// =============================================================================
// Explicit stable module-level exports
// =============================================================================
//
// These aliases intentionally expose the modules themselves rather than
// copying implementation types into this file.
//
// This keeps `core/mod.rs` stable when individual core files evolve.
//
// Downstream code can therefore use:
//
//     benchmarking::core::config::...
//     benchmarking::core::metric::...
//     benchmarking::core::result::...
//
// without requiring `mod.rs` to be rewritten every time a new type is added
// to an individual core module.

pub use benchmark as benchmark_api;
pub use circuit as circuit_api;
pub use config as config_api;
pub use dimension as dimension_api;
pub use errors as errors_api;
pub use execution as execution_api;
pub use experiment as experiment_api;
pub use limits as limits_api;
pub use metric as metric_api;
pub use observation as observation_api;
pub use provenance as provenance_api;
pub use reproducibility as reproducibility_api;
pub use result as result_api;
pub use workload as workload_api;


// =============================================================================
// Architectural compile-time boundary tests
// =============================================================================

#[cfg(test)]
mod tests {
    //! Tests for the `core` module boundary itself.
    //!
    //! Protocol mathematics and individual type behavior belong in their
    //! respective implementation modules. These tests exist only to verify
    //! that the public module structure remains intact.

    use super::*;

    #[test]
    fn all_core_boundaries_are_reachable() {
        // The bindings intentionally reference the modules rather than
        // concrete implementation types. This keeps this test independent
        // of future API evolution inside each file.
        let _ = errors_api;
        let _ = limits_api;
        let _ = dimension_api;
        let _ = metric_api;
        let _ = provenance_api;
        let _ = reproducibility_api;
        let _ = config_api;
        let _ = workload_api;
        let _ = circuit_api;
        let _ = observation_api;
        let _ = execution_api;
        let _ = experiment_api;
        let _ = result_api;
        let _ = benchmark_api;
    }

    #[test]
    fn prelude_exposes_the_complete_core_surface() {
        use super::prelude::*;

        let _ = benchmark;
        let _ = circuit;
        let _ = config;
        let _ = dimension;
        let _ = errors;
        let _ = execution;
        let _ = experiment;
        let _ = limits;
        let _ = metric;
        let _ = observation;
        let _ = provenance;
        let _ = reproducibility;
        let _ = result;
        let _ = workload;
    }
}