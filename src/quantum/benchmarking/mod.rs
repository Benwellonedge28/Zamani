//! Zamani Quantum Benchmarking
//!
//! Production boundary for the Zamani quantum-computing benchmarking
//! subsystem.
//!
//! # Purpose
//!
//! This module is the authoritative namespace for quantum benchmarking in
//! Zamani. It owns module wiring, public API exposure, compatibility
//! boundaries, documentation, and architectural invariants.
//!
//! Benchmark implementations must not turn this module into a second quantum
//! compiler or hardware abstraction layer.
//!
//! The intended production architecture is:
//!
//! ```text
//!                         Zamani language
//!                               │
//!                               ▼
//!                    stdlib::quantum / frontend
//!                               │
//!                               ▼
//!                    quantum::benchmarking
//!                               │
//!          ┌────────────────────┼────────────────────┐
//!          │                    │                    │
//!          ▼                    ▼                    ▼
//!       workload             protocol             analysis
//!          │                    │                    │
//!          ▼                    ▼                    ▼
//!       generator ───────► execution ───────► statistics
//!                               │                    │
//!                 ┌─────────────┴────────────┐       │
//!                 ▼                          ▼       │
//!             simulator                   hardware   │
//!                 │                          │       │
//!                 └─────────────┬────────────┘       │
//!                               ▼                    │
//!                         observations ───────────────┘
//!                               │
//!                               ▼
//!                         BenchmarkResult
//!                               │
//!                 ┌─────────────┼─────────────┐
//!                 ▼             ▼             ▼
//!              reports       baselines      regression
//! ```
//!
//! # Architectural ownership
//!
//! Benchmarking owns:
//!
//! - benchmark experiment definitions;
//! - benchmark protocol orchestration;
//! - benchmark workload generation;
//! - benchmark execution contracts;
//! - raw benchmark observations;
//! - statistical analysis;
//! - benchmark metrics;
//! - benchmark result normalization;
//! - reproducibility metadata;
//! - benchmark comparison;
//! - regression detection;
//! - benchmark reporting.
//!
//! Benchmarking does NOT own:
//!
//! - the canonical Quantum IR;
//! - quantum gate semantics;
//! - source-language parsing;
//! - frontend lowering;
//! - optimization;
//! - routing;
//! - scheduling;
//! - hardware topology;
//! - hardware calibration;
//! - backend implementation;
//! - QEC implementation;
//! - simulator implementation.
//!
//! Those responsibilities remain in their existing owning subsystems.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::benchmarking
//!        │
//!        ├──────────────► quantum::ir
//!        │
//!        ├──────────────► quantum::algorithms
//!        │
//!        ├──────────────► quantum::error_correction
//!        │
//!        ├──────────────► quantum::hardware
//!        │
//!        └──────────────► runtime
//!
//! quantum::ir
//!        │
//!        └──────X──────► quantum::benchmarking
//! ```
//!
//! The last dependency must never be introduced.
//!
//! The Quantum IR remains the canonical semantic representation. Benchmarking
//! consumes IR; it does not redefine it.
//!
//! # Current implementation boundary
//!
//! At the current repository state, the implemented benchmarking component is:
//!
//! ```text
//! benchmarking/
//! └── volume_estimator.rs
//! ```
//!
//! `volume_estimator.rs` is intentionally a pure Quantum Volume mathematical
//! and statistical component. It does not generate circuits, execute circuits,
//! communicate with hardware, select a backend, perform transpilation, perform
//! routing, perform scheduling, or depend on the Quantum IR.
//!
//! That separation is retained deliberately.
//!
//! The permanent production architecture therefore distinguishes:
//!
//! ```text
//! generators/qv.rs
//!        │
//!        ▼
//! protocols/quantum_volume.rs
//!        │
//!        ▼
//! execution/
//!        │
//!        ▼
//! observations
//!        │
//!        ▼
//! volume_estimator.rs
//! ```
//!
//! In particular:
//!
//! - `generators/qv.rs` will own QV circuit construction;
//! - `protocols/quantum_volume.rs` will own the QV experimental protocol;
//! - `execution/` will own execution contracts and normalized observations;
//! - `volume_estimator.rs` will own QV statistical mathematics.
//!
//! This prevents Quantum Volume from becoming coupled to a particular
//! simulator or hardware provider.
//!
//! # Future module layout
//!
//! The benchmarking subsystem is intentionally designed to grow into the
//! following architecture:
//!
//! ```text
//! benchmarking/
//! │
//! ├── mod.rs
//! ├── core/
//! │   ├── mod.rs
//! │   ├── benchmark.rs
//! │   ├── config.rs
//! │   ├── experiment.rs
//! │   ├── workload.rs
//! │   ├── circuit.rs
//! │   ├── execution.rs
//! │   ├── observation.rs
//! │   ├── result.rs
//! │   ├── metric.rs
//! │   ├── dimension.rs
//! │   ├── provenance.rs
//! │   ├── reproducibility.rs
//! │   ├── limits.rs
//! │   └── errors.rs
//! │
//! ├── generators/
//! │   ├── mod.rs
//! │   ├── random.rs
//! │   ├── deterministic.rs
//! │   ├── random_circuits.rs
//! │   ├── mirror_circuits.rs
//! │   ├── clifford.rs
//! │   ├── pauli.rs
//! │   ├── qv.rs
//! │   └── application.rs
//! │
//! ├── execution/
//! │   ├── mod.rs
//! │   ├── executor.rs
//! │   ├── request.rs
//! │   ├── response.rs
//!   ├── batching.rs
//! │   ├── sampler.rs
//! │   ├── timing.rs
//! │   └── cancellation.rs
//! │
//! ├── statistics/
//! │   ├── mod.rs
//! │   ├── distributions.rs
//! │   ├── confidence.rs
//! │   ├── bootstrap.rs
//! │   ├── regression.rs
//! │   ├── hypothesis.rs
//! │   ├── outliers.rs
//! │   └── aggregation.rs
//! │
//! ├── metrics/
//! │   ├── mod.rs
//! │   ├── fidelity.rs
//! │   ├── probability.rs
//!   ├── gate_error.rs
//! │   ├── readout.rs
//! │   ├── runtime.rs
//! │   ├── throughput.rs
//! │   ├── resource.rs
//! │   ├── stability.rs
//! │   ├── leakage.rs
//! │   └── logical.rs
//! │
//! ├── protocols/
//! │   ├── mod.rs
//! │   ├── quantum_volume.rs
//! │   ├── randomized_benchmarking.rs
//! │   ├── interleaved_rb.rs
//! │   ├── simultaneous_rb.rs
//! │   ├── purity_rb.rs
//! │   ├── leakage_rb.rs
//! │   ├── cycle_benchmarking.rs
//! │   ├── layer_fidelity.rs
//! │   ├── xeb.rs
//! │   ├── random_circuit_sampling.rs
//! │   ├── mirror.rs
//! │   ├── spam.rs
//! │   ├── gate_fidelity.rs
//! │   ├── process_fidelity.rs
//! │   ├── coherence.rs
//! │   ├── crosstalk.rs
//! │   ├── drift.rs
//! │   └── tomography.rs
//! │
//! ├── volumetric/
//! │   ├── mod.rs
//! │   ├── volume.rs
//! │   ├── surface.rs
//! │   ├── frontier.rs
//! │   └── positioning.rs
//! │
//! ├── applications/
//! │   ├── mod.rs
//! │   ├── deutsch_jozsa.rs
//!   ├── bernstein_vazirani.rs
//! │   ├── hidden_shift.rs
//! │   ├── qft.rs
//!   ├── grover.rs
//! │   ├── phase_estimation.rs
//! │   ├── amplitude_estimation.rs
//! │   ├── vqe.rs
//! │   ├── qaoa.rs
//! │   ├── maxcut.rs
//! │   ├── hhl.rs
//! │   ├── monte_carlo.rs
//! │   ├── hamiltonian.rs
//! │   ├── shor.rs
//! │   └── custom.rs
//! │
//! ├── qec/
//! │   ├── mod.rs
//! │   ├── physical.rs
//! │   ├── logical.rs
//! │   ├── threshold.rs
//! │   ├── decoder.rs
//! │   ├── syndrome.rs
//! │   ├── surface_code.rs
//! │   └── resource_overhead.rs
//! │
//! ├── hardware/
//! │   ├── mod.rs
//! │   ├── capabilities.rs
//! │   ├── topology.rs
//! │   ├── calibration.rs
//! │   ├── timing.rs
//! │   └── metadata.rs
//! │
//! ├── analysis/
//! │   ├── mod.rs
//! │   ├── compare.rs
//! │   ├── baseline.rs
//! │   ├── regression.rs
//! │   ├── attribution.rs
//! │   ├── bottleneck.rs
//! │   └── diagnosis.rs
//! │
//! ├── reporting/
//! │   ├── mod.rs
//! │   ├── report.rs
//! │   ├── summary.rs
//! │   ├── table.rs
//! │   ├── json.rs
//! │   ├── csv.rs
//! │   └── markdown.rs
//! │
//! ├── registry/
//! │   ├── mod.rs
//!   ├── registry.rs
//! │   ├── builtin.rs
//! │   └── compatibility.rs
//! │
//! ├── validation/
//! │   ├── mod.rs
//! │   ├── input.rs
//!   ├── statistical.rs
//! │   ├── physical.rs
//! │   └── reproducibility.rs
//! │
//! └── tests/
//!     ├── mod.rs
//!     ├── quantum_volume_tests.rs
//!     ├── rb_tests.rs
//!     ├── xeb_tests.rs
//!     ├── cycle_tests.rs
//!     ├── application_tests.rs
//!     ├── qec_tests.rs
//!     ├── statistics_tests.rs
//!     ├── reproducibility_tests.rs
//!     ├── security_tests.rs
//!     └── regression_tests.rs
//! ```
//!
//! These files are intentionally NOT declared here until their implementations
//! exist. Rust must never be made to depend on nonexistent source files.
//!
//! # Benchmark domains
//!
//! Once the complete subsystem is implemented, the stable namespace is
//! intended to cover six benchmark domains:
//!
//! 1. Computation
//!    - quantum algorithms;
//!    - application workloads;
//!    - user-defined Zamani workloads.
//!
//! 2. Device
//!    - gate quality;
//!    - readout;
//!    - SPAM;
//!    - coherence;
//!    - leakage;
//!    - crosstalk;
//!    - calibration drift.
//!
//! 3. System
//!    - compilation;
//!    - transpilation;
//!    - routing;
//!    - scheduling;
//!    - queue latency;
//!    - execution latency;
//!    - throughput.
//!
//! 4. Scaling
//!    - Quantum Volume;
//!    - volumetric benchmarking;
//!    - random circuit sampling;
//!    - XEB;
//!    - width/depth performance surfaces.
//!
//! 5. Fault tolerance
//!    - physical error rates;
//!    - logical error rates;
//!    - threshold experiments;
//!    - decoder performance;
//!    - syndrome extraction;
//!    - logical lifetime;
//!    - resource overhead.
//!
//! 6. End-to-end
//!    - quality;
//!    - time;
//!    - resources;
//!    - throughput;
//!    - solution quality;
//!    - reproducibility.
//!
//! # Benchmark result invariant
//!
//! Every production benchmark eventually needs a common result envelope
//! containing, directly or through a stable nested representation:
//!
//! ```text
//! schema_version
//! benchmark_id
//! benchmark_version
//! experiment_id
//! workload
//! backend
//! compiler
//! calibration
//! metrics
//! observations
//! statistics
//! provenance
//! reproducibility
//! warnings
//! errors
//! result_hash
//! ```
//!
//! A protocol-specific result may contain additional information, but it must
//! never make the universal benchmark contract impossible to consume.
//!
//! # Reproducibility invariant
//!
//! Randomized benchmarks must make randomness explicit.
//!
//! At minimum, future randomized protocols must be able to identify:
//!
//! - benchmark seed;
//! - generator version;
//! - configuration fingerprint;
//! - circuit fingerprint;
//! - backend identity;
//! - compiler identity;
//! - optimization configuration;
//! - routing configuration;
//! - scheduling configuration.
//!
//! No benchmark may depend on an implicit process-global random generator.
//!
//! # Statistical invariant
//!
//! Benchmark results must not expose a bare floating-point number as the only
//! representation of a scientific measurement.
//!
//! A production metric should eventually be able to retain:
//!
//! ```text
//! value
//! unit
//! uncertainty
//! confidence
//! sample_count
//! method
//! provenance
//! validity
//! ```
//!
//! This is particularly important for Quantum Volume, randomized benchmarking,
//! XEB, cycle benchmarking, and logical-error-rate experiments.
//!
//! # Execution invariant
//!
//! Benchmark protocols must not directly assume that every backend is a
//! conventional gate-model QPU.
//!
//! The eventual execution model must be capable of representing:
//!
//! - simulators;
//! - emulators;
//! - gate-model QPUs;
//! - analog systems;
//! - annealers;
//! - samplers;
//! - hybrid systems;
//! - logical-qubit systems;
//! - future quantum technologies.
//!
//! Backend capabilities must therefore be negotiated before execution.
//!
//! # Resource-safety invariant
//!
//! Benchmark configurations are untrusted inputs when exposed through the
//! Zamani language, package tooling, CI, or external configuration files.
//!
//! Future benchmarking modules must enforce finite limits for:
//!
//! - qubits;
//! - circuit depth;
//! - operation count;
//! - shots;
//! - circuit count;
//! - experiment count;
//! - result size;
//! - bootstrap samples;
//! - statistical iterations;
//! - parallelism;
//! - execution time.
//!
//! No benchmark should permit an unbounded allocation merely because a caller
//! supplied a large integer.
//!
//! # Failure invariant
//!
//! Library code must return structured errors.
//!
//! Benchmark implementations must not use direct process termination or
//! diagnostic printing for recoverable benchmark failures.
//!
//! Partial execution must be representable where practical so that a hardware
//! failure after several successful circuits does not destroy already collected
//! observations.
//!
//! # Current public API
//!
//! The currently implemented public API is the Quantum Volume mathematical
//! estimator:
//!
//! ```text
//! quantum::benchmarking::volume_estimator
//! ```
//!
//! The complete estimator API is re-exported below so callers can use either:
//!
//! ```text
//! quantum::benchmarking::volume_estimator::QuantumVolumeConfig
//! ```
//!
//! or the shorter stable path:
//!
//! ```text
//! quantum::benchmarking::QuantumVolumeConfig
//! ```
//!
//! The latter is the preferred public path for new Zamani code.
//!
//! # Compatibility
//!
//! The quantum root currently exposes benchmarking through an inline module.
//! The parent `quantum/mod.rs` must eventually change from that inline module
//! to:
//!
//! ```rust
//! pub mod benchmarking;
//! ```
//!
//! and retain the historical flat compatibility path:
//!
//! ```rust
//! pub use benchmarking::volume_estimator;
//! ```
//!
//! That parent-level change is intentionally outside this file because this
//! file cannot modify its parent module declaration.
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
//! # Dependency policy
//!
//! This module intentionally does not introduce dependencies.
//!
//! Protocol implementations should reuse dependencies already present in
//! `Cargo.toml` where appropriate rather than creating a dependency-heavy
//! quantum benchmarking framework.
//!
//! The repository currently targets Rust 1.97.1 and already provides
//! `serde`, `serde_json`, `thiserror`, `anyhow`, `rand`, and other supporting
//! dependencies. New dependencies should only be introduced when a protocol
//! genuinely requires them.
//!
//! # Testing policy
//!
//! This module contains only boundary/integration tests.
//!
//! Mathematical tests belong beside `volume_estimator.rs`.
//!
//! Future protocol tests belong in the corresponding protocol test modules.
//!
//! Future cross-layer tests should verify:
//!
//! ```text
//! generator
//!     ↓
//! protocol
//!     ↓
//! execution
//!     ↓
//! observation
//!     ↓
//! analysis
//!     ↓
//! result
//! ```
//!
//! Hardware access must not be required for ordinary unit tests.
//!
//! # Production maturity rule
//!
//! This module deliberately does NOT pretend that the entire future benchmark
//! architecture exists merely by declaring empty modules.
//!
//! A benchmarking component becomes part of the public API only when its
//! implementation, tests, validation, reproducibility behavior, statistics,
//! error handling, and integration contract are complete.
//!
//! This keeps `cargo check`, `cargo test`, documentation generation, and
//! downstream users reliable throughout the incremental implementation.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

/// Quantum Volume mathematical/statistical estimator.
///
/// This module remains deliberately independent from circuit generation,
/// execution, hardware, and the Quantum IR.
pub mod volume_estimator;

// =============================================================================
// Stable public Quantum Volume API
// =============================================================================

pub use volume_estimator::{
    ConfidenceInterval,
    ConfidenceIntervalMethod,
    QuantumVolumeConfig,
    QuantumVolumeError,
    QuantumVolumeEstimator,
    QuantumVolumeResult,
    DEFAULT_CONFIDENCE_LEVEL,
    DEFAULT_HEAVY_OUTPUT_THRESHOLD,
    MAX_CONFIDENCE_LEVEL,
    MAX_PROBABILITY,
    MIN_CONFIDENCE_LEVEL,
    MIN_PROBABILITY,
    QUANTUM_VOLUME_BENCHMARK_ID,
    QUANTUM_VOLUME_RESULT_SCHEMA_VERSION,
    TWO_SIGMA_ONE_SIDED_CONFIDENCE_LEVEL,
};

// =============================================================================
// Controlled prelude
// =============================================================================

/// Stable benchmarking prelude.
///
/// New benchmarking-facing code should prefer this prelude over importing
/// implementation modules directly.
///
/// Only currently implemented, production-stable types are exported here.
/// Future benchmark families should be added only after their public contracts
/// are complete.
pub mod prelude {
    pub use super::{
        ConfidenceInterval,
        ConfidenceIntervalMethod,
        QuantumVolumeConfig,
        QuantumVolumeError,
        QuantumVolumeEstimator,
        QuantumVolumeResult,
        DEFAULT_CONFIDENCE_LEVEL,
        DEFAULT_HEAVY_OUTPUT_THRESHOLD,
        MAX_CONFIDENCE_LEVEL,
        MAX_PROBABILITY,
        MIN_CONFIDENCE_LEVEL,
        MIN_PROBABILITY,
        QUANTUM_VOLUME_BENCHMARK_ID,
        QUANTUM_VOLUME_RESULT_SCHEMA_VERSION,
        TWO_SIGMA_ONE_SIDED_CONFIDENCE_LEVEL,
    };
}

// =============================================================================
// Benchmark subsystem identity
// =============================================================================

/// Stable identifier for the Zamani quantum benchmarking subsystem.
pub const BENCHMARKING_SUBSYSTEM_ID: &str = "zamani.quantum.benchmarking";

/// Public benchmark subsystem API version.
///
/// This is intentionally independent of the Zamani compiler version and of
/// individual protocol/result schema versions.
pub const BENCHMARKING_API_VERSION: u32 = 1;

/// Current implementation status of the benchmarking subsystem.
///
/// This value describes the source-tree implementation boundary; it must not
/// be interpreted as a claim that every planned benchmark protocol is already
/// implemented.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BenchmarkingImplementationStatus {
    /// The stable benchmarking boundary exists and the currently implemented
    /// benchmark components are production-oriented.
    Incremental,

    /// All planned benchmark domains and protocols have been implemented,
    /// validated, tested, and integrated.
    Complete,
}

impl BenchmarkingImplementationStatus {
    /// Returns the stable machine-readable status identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Incremental => "incremental",
            Self::Complete => "complete",
        }
    }
}

/// Returns the current implementation status.
///
/// This function intentionally returns a compile-time constant and performs no
/// global initialization or mutation.
#[inline]
pub const fn implementation_status() -> BenchmarkingImplementationStatus {
    BenchmarkingImplementationStatus::Incremental
}

/// Returns the stable benchmarking subsystem identifier.
#[inline]
pub const fn subsystem_id() -> &'static str {
    BENCHMARKING_SUBSYSTEM_ID
}

/// Returns the stable benchmarking API version.
#[inline]
pub const fn api_version() -> u32 {
    BENCHMARKING_API_VERSION
}

// =============================================================================
// Architectural capability inventory
// =============================================================================

/// High-level benchmark domain.
///
/// This enum is an architectural classification rather than an indication
/// that every domain is already implemented in the current source tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BenchmarkDomain {
    /// Quantum algorithm/application computation.
    Computation,

    /// Device-level characterization.
    Device,

    /// Compiler/runtime/system performance.
    System,

    /// Width/depth/scaling benchmarks.
    Scaling,

    /// Quantum error correction and fault tolerance.
    FaultTolerance,

    /// End-to-end workload performance.
    EndToEnd,
}

impl BenchmarkDomain {
    /// Stable machine-readable domain identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Computation => "computation",
            Self::Device => "device",
            Self::System => "system",
            Self::Scaling => "scaling",
            Self::FaultTolerance => "fault_tolerance",
            Self::EndToEnd => "end_to_end",
        }
    }
}

/// Benchmark protocol family.
///
/// This is deliberately a lightweight classification layer. Protocol
/// implementations belong in their respective modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BenchmarkProtocol {
    /// Quantum Volume.
    QuantumVolume,

    /// Randomized Benchmarking.
    RandomizedBenchmarking,

    /// Interleaved Randomized Benchmarking.
    InterleavedRandomizedBenchmarking,

    /// Simultaneous Randomized Benchmarking.
    SimultaneousRandomizedBenchmarking,

    /// Purity Randomized Benchmarking.
    PurityRandomizedBenchmarking,

    /// Leakage Randomized Benchmarking.
    LeakageRandomizedBenchmarking,

    /// Cycle Benchmarking.
    CycleBenchmarking,

    /// Layer Fidelity.
    LayerFidelity,

    /// Cross-Entropy Benchmarking.
    Xeb,

    /// Random Circuit Sampling.
    RandomCircuitSampling,

    /// Mirror Circuits.
    MirrorCircuits,

    /// SPAM/readout characterization.
    Spam,

    /// Gate fidelity characterization.
    GateFidelity,

    /// Process fidelity characterization.
    ProcessFidelity,

    /// Coherence characterization.
    Coherence,

    /// Crosstalk characterization.
    Crosstalk,

    /// Hardware/calibration drift.
    Drift,

    /// Tomographic characterization.
    Tomography,

    /// Volumetric benchmarking.
    Volumetric,

    /// Application-specific benchmark.
    Application,

    /// Physical error-rate benchmark.
    PhysicalErrorRate,

    /// Logical error-rate benchmark.
    LogicalErrorRate,

    /// QEC threshold benchmark.
    QecThreshold,

    /// Decoder benchmark.
    Decoder,

    /// Custom Zamani benchmark.
    Custom,
}

impl BenchmarkProtocol {
    /// Stable machine-readable protocol identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QuantumVolume => "quantum_volume",
            Self::RandomizedBenchmarking => "randomized_benchmarking",
            Self::InterleavedRandomizedBenchmarking => "interleaved_rb",
            Self::SimultaneousRandomizedBenchmarking => "simultaneous_rb",
            Self::PurityRandomizedBenchmarking => "purity_rb",
            Self::LeakageRandomizedBenchmarking => "leakage_rb",
            Self::CycleBenchmarking => "cycle_benchmarking",
            Self::LayerFidelity => "layer_fidelity",
            Self::Xeb => "xeb",
            Self::RandomCircuitSampling => "random_circuit_sampling",
            Self::MirrorCircuits => "mirror",
            Self::Spam => "spam",
            Self::GateFidelity => "gate_fidelity",
            Self::ProcessFidelity => "process_fidelity",
            Self::Coherence => "coherence",
            Self::Crosstalk => "crosstalk",
            Self::Drift => "drift",
            Self::Tomography => "tomography",
            Self::Volumetric => "volumetric",
            Self::Application => "application",
            Self::PhysicalErrorRate => "physical_error_rate",
            Self::LogicalErrorRate => "logical_error_rate",
            Self::QecThreshold => "qec_threshold",
            Self::Decoder => "decoder",
            Self::Custom => "custom",
        }
    }

    /// Returns the high-level domain to which the protocol primarily belongs.
    pub const fn domain(self) -> BenchmarkDomain {
        match self {
            Self::QuantumVolume
            | Self::RandomCircuitSampling
            | Self::Xeb
            | Self::Volumetric => BenchmarkDomain::Scaling,

            Self::RandomizedBenchmarking
            | Self::InterleavedRandomizedBenchmarking
            | Self::SimultaneousRandomizedBenchmarking
            | Self::PurityRandomizedBenchmarking
            | Self::LeakageRandomizedBenchmarking
            | Self::CycleBenchmarking
            | Self::LayerFidelity
            | Self::Spam
            | Self::GateFidelity
            | Self::ProcessFidelity
            | Self::Coherence
            | Self::Crosstalk
            | Self::Drift
            | Self::Tomography => BenchmarkDomain::Device,

            Self::PhysicalErrorRate
            | Self::LogicalErrorRate
            | Self::QecThreshold
            | Self::Decoder => BenchmarkDomain::FaultTolerance,

            Self::Application | Self::Custom => BenchmarkDomain::Computation,

            Self::MirrorCircuits => BenchmarkDomain::System,
        }
    }
}

// =============================================================================
// Integration contract
// =============================================================================

/// Describes the dependency ownership expected by the benchmarking boundary.
///
/// This is intentionally a data-free marker. Its purpose is to make the
/// architecture explicit in generated documentation and prevent callers from
/// treating benchmarking as the owner of quantum semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BenchmarkingArchitecture;

/// Returns the architectural contract marker.
#[inline]
pub const fn architecture() -> BenchmarkingArchitecture {
    BenchmarkingArchitecture
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subsystem_identity_is_stable() {
        assert_eq!(
            subsystem_id(),
            "zamani.quantum.benchmarking"
        );

        assert_eq!(api_version(), 1);
    }

    #[test]
    fn current_implementation_status_is_explicit() {
        assert_eq!(
            implementation_status(),
            BenchmarkingImplementationStatus::Incremental
        );

        assert_eq!(
            implementation_status().as_str(),
            "incremental"
        );
    }

    #[test]
    fn all_domains_have_stable_identifiers() {
        let domains = [
            BenchmarkDomain::Computation,
            BenchmarkDomain::Device,
            BenchmarkDomain::System,
            BenchmarkDomain::Scaling,
            BenchmarkDomain::FaultTolerance,
            BenchmarkDomain::EndToEnd,
        ];

        for domain in domains {
            assert!(!domain.as_str().is_empty());
        }
    }

    #[test]
    fn all_protocols_have_stable_identifiers() {
        let protocols = [
            BenchmarkProtocol::QuantumVolume,
            BenchmarkProtocol::RandomizedBenchmarking,
            BenchmarkProtocol::InterleavedRandomizedBenchmarking,
            BenchmarkProtocol::SimultaneousRandomizedBenchmarking,
            BenchmarkProtocol::PurityRandomizedBenchmarking,
            BenchmarkProtocol::LeakageRandomizedBenchmarking,
            BenchmarkProtocol::CycleBenchmarking,
            BenchmarkProtocol::LayerFidelity,
            BenchmarkProtocol::Xeb,
            BenchmarkProtocol::RandomCircuitSampling,
            BenchmarkProtocol::MirrorCircuits,
            BenchmarkProtocol::Spam,
            BenchmarkProtocol::GateFidelity,
            BenchmarkProtocol::ProcessFidelity,
            BenchmarkProtocol::Coherence,
            BenchmarkProtocol::Crosstalk,
            BenchmarkProtocol::Drift,
            BenchmarkProtocol::Tomography,
            BenchmarkProtocol::Volumetric,
            BenchmarkProtocol::Application,
            BenchmarkProtocol::PhysicalErrorRate,
            BenchmarkProtocol::LogicalErrorRate,
            BenchmarkProtocol::QecThreshold,
            BenchmarkProtocol::Decoder,
            BenchmarkProtocol::Custom,
        ];

        for protocol in protocols {
            assert!(!protocol.as_str().is_empty());
        }
    }

    #[test]
    fn quantum_volume_belongs_to_scaling_domain() {
        assert_eq!(
            BenchmarkProtocol::QuantumVolume.domain(),
            BenchmarkDomain::Scaling
        );
    }

    #[test]
    fn qec_protocols_belong_to_fault_tolerance_domain() {
        assert_eq!(
            BenchmarkProtocol::PhysicalErrorRate.domain(),
            BenchmarkDomain::FaultTolerance
        );

        assert_eq!(
            BenchmarkProtocol::LogicalErrorRate.domain(),
            BenchmarkDomain::FaultTolerance
        );

        assert_eq!(
            BenchmarkProtocol::QecThreshold.domain(),
            BenchmarkDomain::FaultTolerance
        );

        assert_eq!(
            BenchmarkProtocol::Decoder.domain(),
            BenchmarkDomain::FaultTolerance
        );
    }

    #[test]
    fn quantum_volume_public_api_is_reachable() {
        let configuration =
            QuantumVolumeConfig::new(4, 4).expect("valid QV configuration");

        assert_eq!(configuration.num_qubits, 4);
        assert_eq!(configuration.gate_depth, 4);
        assert_eq!(
            configuration.exponent(),
            4
        );
    }

    #[test]
    fn architecture_marker_is_constructible() {
        let _ = architecture();
    }

    #[test]
    fn prelude_exposes_current_stable_api() {
        use super::prelude::{
            QuantumVolumeConfig,
            QUANTUM_VOLUME_BENCHMARK_ID,
        };

        let configuration =
            QuantumVolumeConfig::new(2, 2).expect("valid QV configuration");

        assert_eq!(configuration.exponent(), 2);
        assert_eq!(
            QUANTUM_VOLUME_BENCHMARK_ID,
            "quantum_volume"
        );
    }
}