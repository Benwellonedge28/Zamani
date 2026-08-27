//! Zamani Quantum Application Benchmarking
//!
//! This module is the authoritative module boundary for application-oriented
//! quantum-computing benchmarks.
//!
//! # Architectural responsibility
//!
//! `applications` owns module wiring and the public namespace for application
//! benchmark implementations. It does not own:
//!
//! - Quantum IR semantics;
//! - quantum circuit semantics;
//! - backend communication;
//! - hardware topology;
//! - calibration;
//! - statistical algorithms;
//! - generic benchmark orchestration;
//! - generic execution;
//! - generic reporting;
//! - error-correction implementation;
//! - compiler optimization;
//! - routing;
//! - scheduling.
//!
//! Those responsibilities remain in their owning subsystems.
//!
//! # Dependency direction
//!
//! ```text
//! Zamani language / stdlib
//!          │
//!          ▼
//! quantum::benchmarking
//!          │
//!          ▼
//! benchmarking::applications
//!          │
//!     ┌────┴───────────────────────────────┐
//!     │                                    │
//!     ▼                                    ▼
//! application benchmark              application generator
//! implementation                    / core benchmark contract
//!     │                                    │
//!     └────────────────┬───────────────────┘
//!                      ▼
//!                Quantum IR / algorithms
//!                      │
//!                      ▼
//!                execution backend
//! ```
//!
//! The dependency direction is intentionally one-way:
//!
//! ```text
//! applications → core/generators → IR/algorithms/execution
//! ```
//!
//! and never:
//!
//! ```text
//! IR → applications
//! applications → frontend
//! applications → concrete hardware vendor
//! ```
//!
//! # Production boundary
//!
//! Each application benchmark is independently implementable and testable.
//! This module only makes the implementations reachable through a stable
//! namespace.
//!
//! An application benchmark should normally implement the generic benchmark
//! contract defined by:
//!
//! ```text
//! quantum::benchmarking::core::benchmark
//! ```
//!
//! and use:
//!
//! ```text
//! quantum::benchmarking::core
//! quantum::benchmarking::generators
//! quantum::benchmarking::execution
//! quantum::benchmarking::statistics
//! quantum::benchmarking::metrics
//! ```
//!
//! where required.
//!
//! # Important separation
//!
//! Application benchmark files must not each invent their own incompatible
//! execution/result framework.
//!
//! They should produce the common `BenchmarkResult` contract through the
//! application benchmark implementation while retaining application-specific
//! metrics and success criteria.
//!
//! # Application benchmark families
//!
//! The current application namespace covers:
//!
//! - Deutsch-Jozsa;
//! - Bernstein-Vazirani;
//! - Hidden Shift;
//! - Quantum Fourier Transform;
//! - Grover Search;
//! - Quantum Phase Estimation;
//! - Quantum Amplitude Estimation;
//! - HHL linear-system solving;
//! - Hamiltonian simulation;
//! - Monte Carlo;
//! - QAOA;
//! - MaxCut;
//! - VQE;
//! - Shor;
//! - user-defined/custom application benchmarks.
//!
//! These correspond to the application-oriented benchmark direction used by
//! established quantum benchmarking efforts such as QED-C, while remaining
//! backend-neutral and native to Zamani.
//!
//! # Benchmark classification
//!
//! Application benchmarks may belong to one or more of these conceptual
//! classes:
//!
//! - `Application` — end-user algorithm/application performance;
//! - `Computation` — computational primitive or algorithmic workload;
//! - `Hybrid` — quantum/classical iterative workloads;
//! - `Sampling` — workloads whose principal output is a distribution/sample;
//! - `Scaling` — workloads evaluated across problem sizes;
//! - `Custom` — user-defined Zamani workloads.
//!
//! The individual benchmark implementation owns its precise classification.
//! This module does not duplicate that metadata.
//!
//! # Capability independence
//!
//! Application benchmarks must not assume that every execution target:
//!
//! - has physical qubits;
//! - exposes a state vector;
//! - supports arbitrary gates;
//! - supports mid-circuit measurement;
//! - supports dynamic circuits;
//! - exposes exact amplitudes;
//! - exposes calibration information.
//!
//! Capability negotiation belongs to the generic benchmarking/hardware
//! integration layer.
//!
//! A benchmark that requires a capability must declare that requirement
//! through the generic benchmark contract rather than failing later with an
//! unexplained backend error.
//!
//! # Reproducibility
//!
//! Application benchmarks must consume explicit benchmark seeds/configuration
//! supplied by the generic benchmarking layer.
//!
//! This module deliberately does not create a process-global RNG or benchmark
//! state.
//!
//! # Naming compatibility
//!
//! The repository currently contains the file:
//!
//! ```text
//! Bernstein_vazirani.rs
//! ```
//!
//! rather than the conventional lowercase Rust filename
//! `bernstein_vazirani.rs`.
//!
//! The module is therefore declared with an explicit `#[path]` attribute.
//! The public Rust module name remains the idiomatic:
//!
//! ```text
//! bernstein_vazirani
//! ```
//!
//! This avoids a filesystem rename being required merely to establish the
//! canonical Rust namespace.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97 / Rust 1.97.1, Rust 2021.
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! The parent benchmarking module should expose this directory with:
//!
//! ```text
//! pub mod applications;
//! ```
//!
//! The top-level quantum module should eventually expose the complete
//! benchmarking subsystem through:
//!
//! ```text
//! quantum::benchmarking::applications
//! ```
//!
//! The application modules themselves should not need to be edited merely
//! because another application benchmark is added.
//!
//! Adding a new application intentionally requires one new module declaration
//! here because this file is the authoritative namespace boundary.
//!
//! # Registry integration
//!
//! `registry/builtin.rs` is responsible for registering executable benchmark
//! implementations. This module does not register benchmarks globally.
//!
//! This separation is intentional:
//!
//! ```text
//! applications/mod.rs
//!     = namespace/module ownership
//!
//! registry/builtin.rs
//!     = executable benchmark discovery/registration
//! ```
//!
//! Therefore registry changes do not require application implementation
//! changes, and application implementation changes do not require changes to
//! unrelated benchmarks.
//!
//! # Generator integration
//!
//! Generic application circuit/workload generation belongs in:
//!
//! ```text
//! quantum::benchmarking::generators::application
//! ```
//!
//! Application-specific generation may remain inside an individual benchmark
//! when the workload is intrinsically algorithm-specific.
//!
//! The preferred direction is:
//!
//! ```text
//! application benchmark
//!         │
//!         ▼
//! application generator contract
//!         │
//!         ▼
//! Quantum IR
//! ```
//!
//! # Algorithm integration
//!
//! Existing Zamani quantum algorithms remain authoritative.
//!
//! For example:
//!
//! ```text
//! applications::vqe
//!       │
//!       ▼
//! quantum::algorithms::variational
//!       │
//!       ▼
//! quantum::ir
//! ```
//!
//! The benchmark must measure the algorithm; it must not silently replace the
//! algorithm implementation with a benchmark-specific duplicate.
//!
//! # Execution integration
//!
//! Application modules must not directly couple themselves to a specific
//! simulator or hardware vendor.
//!
//! Execution should flow through the generic benchmark execution contract:
//!
//! ```text
//! application benchmark
//!       │
//!       ▼
//! BenchmarkExperiment
//!       │
//!       ▼
//! BenchmarkExecutor
//!       │
//!       ├── simulator
//!       ├── emulator
//!       ├── hardware
//!       └── other supported quantum execution target
//! ```
//!
//! # Analysis integration
//!
//! Application benchmarks should expose application-specific measurements
//! while using generic statistical and metric infrastructure whenever the
//! mathematics is shared.
//!
//! Typical application metrics include:
//!
//! - success probability;
//! - approximation ratio;
//! - objective value;
//! - estimation error;
//! - energy error;
//! - solution quality;
//! - circuit depth;
//! - gate count;
//! - two-qubit gate count;
//! - shot count;
//! - iteration count;
//! - quantum execution time;
//! - classical execution time;
//! - end-to-end time-to-solution;
//! - resource usage.
//!
//! # QED-C alignment without dependency
//!
//! Zamani intentionally does not depend on the QED-C implementation.
//!
//! QED-C's application-oriented benchmark work demonstrates the value of
//! measuring quantum workloads across problem size and platforms, with
//! workload-specific performance measures rather than one universal scalar.
//!
//! Zamani follows the same general principle while keeping its implementation
//! native to the Zamani IR/runtime architecture.
//!
//! # Public namespace
//!
//! The stable application namespace is:
//!
//! ```text
//! quantum::benchmarking::applications::deutsch_jozsa
//! quantum::benchmarking::applications::bernstein_vazirani
//! quantum::benchmarking::applications::hidden_shift
//! quantum::benchmarking::applications::qft
//! quantum::benchmarking::applications::grover
//! quantum::benchmarking::applications::phase_estimation
//! quantum::benchmarking::applications::amplitude_estimation
//! quantum::benchmarking::applications::hhl
//! quantum::benchmarking::applications::hamiltonian
//! quantum::benchmarking::applications::monte_carlo
//! quantum::benchmarking::applications::qaoa
//! quantum::benchmarking::applications::maxcut
//! quantum::benchmarking::applications::vqe
//! quantum::benchmarking::applications::shor
//! quantum::benchmarking::applications::custom
//! ```
//!
//! This module intentionally does not re-export arbitrary implementation
//! types from those modules. The application module remains the ownership
//! boundary for its implementation API and prevents accidental API pollution
//! at the parent namespace.
//!
//! # API stability
//!
//! The module names above form the stable structural API.
//!
//! Individual benchmark types and functions may evolve according to their
//! protocol versions and the generic benchmark contract.
//!
//! The module boundary itself should remain stable once released.

/// Deutsch-Jozsa application benchmark.
///
/// Measures an oracle-based promise-problem workload, including circuit
/// resources, execution characteristics and classification success.
pub mod deutsch_jozsa;

/// Bernstein-Vazirani application benchmark.
///
/// The source repository currently uses the filename
/// `Bernstein_vazirani.rs`; the public Rust namespace intentionally remains
/// lowercase and idiomatic.
#[path = "Bernstein_vazirani.rs"]
pub mod bernstein_vazirani;

/// Hidden-Shift application benchmark.
///
/// Measures hidden-shift workload scaling and application-level correctness.
pub mod hidden_shift;

/// Quantum Fourier Transform application benchmark.
///
/// Measures QFT resource growth, approximation/error behaviour where
/// applicable, and execution performance.
pub mod qft;

/// Grover-search application benchmark.
///
/// Measures search success probability, oracle/resource cost, repetitions,
/// execution time and time-to-solution.
pub mod grover;

/// Quantum Phase Estimation application benchmark.
///
/// Measures estimation accuracy, precision, circuit resources and execution
/// performance.
pub mod phase_estimation;

/// Quantum Amplitude Estimation application benchmark.
///
/// Measures amplitude-estimation accuracy, target precision, query/shot cost,
/// circuit resources and execution performance.
pub mod amplitude_estimation;

/// HHL linear-system application benchmark.
///
/// Measures linear-system accuracy and the quantum/resource envelope of the
/// workload. Small instances may be classically verified; larger workloads
/// must report verification limitations explicitly.
pub mod hhl;

/// Hamiltonian-simulation application benchmark.
///
/// Measures observable/energy error, circuit resources, sampling cost and
/// execution performance.
pub mod hamiltonian;

/// Quantum Monte Carlo application benchmark.
///
/// Measures estimation error, sampling/query cost, circuit resources and
/// end-to-end execution characteristics.
pub mod monte_carlo;

/// QAOA application benchmark.
///
/// Measures hybrid quantum-classical optimization behaviour, including
/// objective quality, approximation quality, iterations and time-to-solution.
pub mod qaoa;

/// MaxCut application benchmark.
///
/// Provides a graph-optimization workload and application-level metrics such
/// as objective value and approximation ratio.
pub mod maxcut;

/// Variational Quantum Eigensolver application benchmark.
///
/// Measures convergence, energy error, iteration count, circuit evaluations,
/// quantum execution time, classical optimization time and end-to-end
/// time-to-solution.
pub mod vqe;

/// Shor application benchmark.
///
/// Supports small verifiable factoring workloads and resource-oriented
/// benchmarking for larger instances where full classical verification is
/// impractical.
pub mod shor;

/// Custom Zamani application benchmark.
///
/// Provides the application-level extension point for user-defined benchmark
/// workloads without requiring modification of the built-in benchmark
/// implementations.
pub mod custom;

// =============================================================================
// Stable application identifiers
// =============================================================================

/// Stable identifier for the Deutsch-Jozsa application benchmark.
pub const DEUTSCH_JOZSA_ID: &str = "deutsch_jozsa";

/// Stable identifier for the Bernstein-Vazirani application benchmark.
pub const BERNSTEIN_VAZIRANI_ID: &str = "bernstein_vazirani";

/// Stable identifier for the Hidden-Shift application benchmark.
pub const HIDDEN_SHIFT_ID: &str = "hidden_shift";

/// Stable identifier for the Quantum Fourier Transform benchmark.
pub const QFT_ID: &str = "qft";

/// Stable identifier for the Grover-search benchmark.
pub const GROVER_ID: &str = "grover";

/// Stable identifier for the Quantum Phase Estimation benchmark.
pub const PHASE_ESTIMATION_ID: &str = "phase_estimation";

/// Stable identifier for the Quantum Amplitude Estimation benchmark.
pub const AMPLITUDE_ESTIMATION_ID: &str = "amplitude_estimation";

/// Stable identifier for the HHL benchmark.
pub const HHL_ID: &str = "hhl";

/// Stable identifier for the Hamiltonian-simulation benchmark.
pub const HAMILTONIAN_ID: &str = "hamiltonian";

/// Stable identifier for the Monte Carlo benchmark.
pub const MONTE_CARLO_ID: &str = "monte_carlo";

/// Stable identifier for the QAOA benchmark.
pub const QAOA_ID: &str = "qaoa";

/// Stable identifier for the MaxCut benchmark.
pub const MAXCUT_ID: &str = "maxcut";

/// Stable identifier for the VQE benchmark.
pub const VQE_ID: &str = "vqe";

/// Stable identifier for the Shor benchmark.
pub const SHOR_ID: &str = "shor";

/// Stable identifier for the custom application benchmark.
pub const CUSTOM_ID: &str = "custom";

/// Stable ordered list of all built-in application benchmark identifiers.
///
/// The order is intentionally deterministic so that callers such as the
/// registry, documentation generator and Zamani-language tooling can produce
/// reproducible output.
///
/// This list contains identifiers only. Executable benchmark registration
/// remains the responsibility of `registry::builtin`.
pub const BUILTIN_APPLICATION_BENCHMARK_IDS: &[&str] = &[
    DEUTSCH_JOZSA_ID,
    BERNSTEIN_VAZIRANI_ID,
    HIDDEN_SHIFT_ID,
    QFT_ID,
    GROVER_ID,
    PHASE_ESTIMATION_ID,
    AMPLITUDE_ESTIMATION_ID,
    HHL_ID,
    HAMILTONIAN_ID,
    MONTE_CARLO_ID,
    QAOA_ID,
    MAXCUT_ID,
    VQE_ID,
    SHOR_ID,
    CUSTOM_ID,
];

/// Returns the stable identifier of every built-in application benchmark.
///
/// This is a function rather than exposing mutable/static registry state.
/// Consequently the application namespace remains process-global-state-free.
#[inline]
pub const fn builtin_application_benchmark_ids() -> &'static [&'static str] {
    BUILTIN_APPLICATION_BENCHMARK_IDS
}

/// Tests whether a stable application benchmark identifier belongs to this
/// application namespace.
///
/// This function only performs namespace membership checking. It does not
/// instantiate or execute a benchmark.
#[inline]
pub fn is_builtin_application_benchmark(id: &str) -> bool {
    BUILTIN_APPLICATION_BENCHMARK_IDS
        .iter()
        .any(|candidate| *candidate == id)
}

// =============================================================================
// Architectural tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_builtin_application_identifier_is_unique() {
        for (index, id) in BUILTIN_APPLICATION_BENCHMARK_IDS.iter().enumerate() {
            assert!(
                !BUILTIN_APPLICATION_BENCHMARK_IDS[..index]
                    .iter()
                    .any(|previous| previous == id),
                "duplicate application benchmark identifier: {id}"
            );
        }
    }

    #[test]
    fn every_builtin_identifier_is_non_empty() {
        for id in BUILTIN_APPLICATION_BENCHMARK_IDS {
            assert!(!id.is_empty());
            assert!(!id.trim().is_empty());
        }
    }

    #[test]
    fn identifiers_are_stable_machine_names() {
        for id in BUILTIN_APPLICATION_BENCHMARK_IDS {
            assert!(
                id.chars()
                    .all(|character| character.is_ascii_lowercase()
                        || character.is_ascii_digit()
                        || character == '_'),
                "invalid application benchmark identifier: {id}"
            );
        }
    }

    #[test]
    fn expected_application_benchmark_count_is_present() {
        assert_eq!(BUILTIN_APPLICATION_BENCHMARK_IDS.len(), 15);
    }

    #[test]
    fn known_application_identifiers_are_recognized() {
        assert!(is_builtin_application_benchmark(DEUTSCH_JOZSA_ID));
        assert!(is_builtin_application_benchmark(BERNSTEIN_VAZIRANI_ID));
        assert!(is_builtin_application_benchmark(HIDDEN_SHIFT_ID));
        assert!(is_builtin_application_benchmark(QFT_ID));
        assert!(is_builtin_application_benchmark(GROVER_ID));
        assert!(is_builtin_application_benchmark(PHASE_ESTIMATION_ID));
        assert!(is_builtin_application_benchmark(AMPLITUDE_ESTIMATION_ID));
        assert!(is_builtin_application_benchmark(HHL_ID));
        assert!(is_builtin_application_benchmark(HAMILTONIAN_ID));
        assert!(is_builtin_application_benchmark(MONTE_CARLO_ID));
        assert!(is_builtin_application_benchmark(QAOA_ID));
        assert!(is_builtin_application_benchmark(MAXCUT_ID));
        assert!(is_builtin_application_benchmark(VQE_ID));
        assert!(is_builtin_application_benchmark(SHOR_ID));
        assert!(is_builtin_application_benchmark(CUSTOM_ID));
    }

    #[test]
    fn unknown_application_identifier_is_rejected() {
        assert!(!is_builtin_application_benchmark("not_a_benchmark"));
        assert!(!is_builtin_application_benchmark(""));
        assert!(!is_builtin_application_benchmark("quantum_volume"));
        assert!(!is_builtin_application_benchmark("randomized_benchmarking"));
    }
}