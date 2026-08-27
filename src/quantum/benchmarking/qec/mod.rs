//! Zamani Quantum Benchmarking — Quantum Error Correction (QEC).
//!
//! # Purpose
//!
//! This module is the authoritative integration boundary for
//! `quantum::benchmarking::qec`.
//!
//! It does not implement QEC mathematics itself. Its responsibilities are:
//!
//! - register the QEC benchmark modules;
//! - expose the stable QEC benchmarking namespace;
//! - define the benchmark-family inventory;
//! - define the QEC benchmarking API identity;
//! - define stable benchmark identifiers;
//! - define benchmark capability metadata;
//! - provide capability discovery without granting execution authorization;
//! - provide a bounded architectural self-check;
//! - document integration with the existing QEC subsystem;
//! - preserve dependency direction between benchmarking and execution;
//! - provide a stable foundation for future Zamani-language integration.
//!
//! # Current production modules
//!
//! The current repository contains:
//!
//! ```text
//! benchmarking/qec/
//! ├── decoder.rs
//! ├── logical.rs
//! ├── physical.rs
//! ├── resource_overhead.rs
//! ├── surface_code.rs
//! ├── syndrome.rs
//! └── threshold.rs
//! ```
//!
//! This module registers exactly those modules.
//!
//! It intentionally does not declare future modules until their source files
//! exist. This prevents the root integration module from becoming a source of
//! compilation failures during incremental development.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani Quantum IR
//!                                │
//!                                ▼
//!                     Quantum execution / QEC
//!                                │
//!                ┌───────────────┴──────────────┐
//!                │                              │
//!                ▼                              ▼
//!        physical observations          logical observations
//!                │                              │
//!                ▼                              ▼
//!       benchmarking::qec::physical   benchmarking::qec::logical
//!                │                              │
//!                └──────────────┬───────────────┘
//!                               ▼
//!                    QEC benchmark results
//!                               │
//!              ┌────────────────┼────────────────┐
//!              ▼                ▼                ▼
//!          threshold        decoder          resources
//!              │                │                │
//!              └────────────────┼────────────────┘
//!                               ▼
//!                    universal benchmark layer
//! ```
//!
//! # Ownership boundary
//!
//! The QEC benchmarking subsystem measures and analyzes QEC behavior.
//!
//! It does NOT own:
//!
//! - quantum error-correction execution;
//! - physical QPU submission;
//! - QPU credentials;
//! - backend networking;
//! - decoder implementation;
//! - canonical Quantum IR;
//! - calibration mutation;
//! - resource admission;
//! - memory allocation policy;
//! - process-global execution state;
//! - compiler ownership;
//! - routing ownership;
//! - scheduling ownership.
//!
//! Those responsibilities remain with their existing owning subsystems.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! quantum::error_correction
//!      │
//!      ▼
//! quantum::benchmarking::qec
//!      │
//!      ├── physical
//!      ├── logical
//!      ├── syndrome
//!      ├── decoder
//!      ├── threshold
//!      ├── surface_code
//!      └── resource_overhead
//!      │
//!      ▼
//! quantum::benchmarking::analysis
//!      │
//!      ▼
//! quantum::benchmarking::reporting
//! ```
//!
//! The benchmark layer must not become a dependency of the canonical Quantum
//! IR.
//!
//! In particular:
//!
//! ```text
//! ir -> benchmarking::qec       FORBIDDEN
//! benchmarking::qec -> ir       ALLOWED
//! ```
//!
//! Likewise, benchmark analysis must not acquire direct authority over QPU
//! execution.
//!
//! # Relationship with quantum::error_correction
//!
//! The repository already contains a complete QEC implementation subsystem.
//! Benchmarking must consume its observations and results rather than
//! duplicating QEC algorithms.
//!
//! The intended relationship is:
//!
//! ```text
//! quantum::error_correction
//!          │
//!          ├── physical error observations
//!          ├── syndrome information
//!          ├── logical outcomes
//!          ├── decoder results
//!          ├── code geometry
//!          └── resource information
//!                    │
//!                    ▼
//!        quantum::benchmarking::qec
//! ```
//!
//! The benchmarking layer therefore measures the QEC subsystem instead of
//! becoming a second QEC implementation.
//!
//! # Scientific benchmark dimensions
//!
//! The QEC benchmark family must eventually support, where the underlying
//! experiment provides the necessary information:
//!
//! - physical error rate;
//! - logical error rate;
//! - logical error per QEC cycle;
//! - logical X error;
//! - logical Z error;
//! - logical Y/correlated error;
//! - detection-event probability;
//! - detector likelihood;
//! - leakage;
//! - erasure;
//! - syndrome quality;
//! - code-distance scaling;
//! - threshold estimates;
//! - suppression factor;
//! - logical lifetime;
//! - physical-to-logical qubit overhead;
//! - physical-to-logical gate overhead;
//! - physical-to-logical time overhead;
//! - syndrome extraction rate;
//! - decoder accuracy;
//! - decoder latency;
//! - decoder throughput;
//! - streaming-decoder backlog;
//! - controller-to-decoder latency;
//! - decoder-to-controller feedback latency;
//! - end-to-end logical-operation latency;
//! - memory lifetime;
//! - stability over repeated rounds;
//! - correlated-error indicators;
//! - resource overhead;
//! - time-to-logical-solution where an application layer provides it.
//!
//! Modern QEC experiments demonstrate why latency and throughput cannot be
//! treated as optional metadata: real-time decoding must keep pace with the
//! QEC cycle or classical processing becomes a bottleneck. The benchmark
//! architecture therefore keeps decoder performance as a first-class family
//! rather than hiding it inside a generic runtime metric.
//!
//! # Statistical boundary
//!
//! Individual benchmark modules own their protocol-specific mathematical
//! calculations.
//!
//! This root module must never:
//!
//! - calculate a logical error rate;
//! - fit a threshold;
//! - calculate a confidence interval;
//! - calculate decoder latency;
//! - calculate resource overhead.
//!
//! Those operations belong to their respective modules.
//!
//! The future universal benchmarking statistics subsystem may provide shared
//! statistical primitives, but this module remains independent of it so that
//! QEC modules can be developed and tested incrementally.
//!
//! # Reproducibility
//!
//! QEC benchmark results must eventually preserve, through the universal
//! `BenchmarkResult` contract:
//!
//! - benchmark ID;
//! - benchmark protocol version;
//! - Zamani version;
//! - QEC implementation version;
//! - code family;
//! - code distance;
//! - number of physical qubits;
//! - number of logical qubits;
//! - number of QEC rounds;
//! - number of shots/trials;
//! - noise model when simulated;
//! - decoder identity and version;
//! - decoder configuration;
//! - decoder seed when applicable;
//! - execution backend;
//! - calibration identity when applicable;
//! - compiler configuration when applicable;
//! - circuit identity when applicable;
//! - result hash;
//! - statistical method;
//! - confidence level;
//! - timestamp/provenance.
//!
//! This root does not duplicate that result schema. It only establishes the
//! stable benchmark-family boundary consumed by the universal benchmarking
//! layer.
//!
//! # Security and resource safety
//!
//! Benchmark configuration is untrusted input whenever it originates from:
//!
//! - Zamani source code;
//! - configuration files;
//! - CI;
//! - remote benchmark jobs;
//! - serialized benchmark specifications;
//! - external hardware services.
//!
//! Consequently this module must remain free of:
//!
//! - unbounded allocation;
//! - implicit thread creation;
//! - process-global mutable state;
//! - network access;
//! - filesystem access;
//! - QPU credentials;
//! - unsafe code.
//!
//! Resource limits belong to the universal benchmarking `core::limits`
//! subsystem and to the individual benchmark modules where protocol-specific
//! limits are required.
//!
//! # QPU authorization
//!
//! Benchmark classification is never authorization.
//!
//! ```text
//! qec benchmark supports QPU
//!          !=
//! benchmark is authorized to access QPU
//! ```
//!
//! Actual QPU access must pass through the existing hardware/runtime
//! capability and authorization layers.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! This file is designed to be completed once.
//!
//! Future implementation files should be added here only when the actual file
//! exists and has its own stable API.
//!
//! Existing modules must not depend on this root module merely to access one
//! another. This prevents circular ownership.
//!
//! For example:
//!
//! ```text
//! physical.rs
//!     -> core/statistics
//!
//! threshold.rs
//!     -> physical.rs
//!
//! decoder.rs
//!     -> syndrome.rs
//!
//! reporting
//!     -> qec::*
//! ```
//!
//! rather than:
//!
//! ```text
//! physical.rs
//!     -> qec/mod.rs
//!     -> threshold.rs
//!     -> physical.rs
//! ```
//!
//! # Future module additions
//!
//! The following benchmark families are intentionally reserved for later
//! implementation:
//!
//! - decoder_latency.rs;
//! - detector_likelihood.rs;
//! - logical_lifetime.rs;
//! - stability.rs;
//! - streaming.rs;
//! - controller_latency.rs;
//! - fault_tolerant_gate.rs;
//! - lattice_surgery.rs;
//! - magic_state.rs;
//! - qldpc.rs;
//! - color_code.rs;
//! - repetition_code.rs;
//! - subsystem_code.rs;
//! - correlated_errors.rs;
//! - qec_application.rs.
//!
//! They must not be declared until their corresponding files are production
//! ready.
//!
//! # External benchmark alignment
//!
//! The architecture is deliberately compatible with current QEC benchmarking
//! practice:
//!
//! - logical error must be studied versus code distance;
//! - error-per-cycle must remain distinguishable from error-per-shot;
//! - decoder latency must be independently measurable;
//! - decoder throughput must be compared with syndrome production rate;
//! - correlated error events must not be hidden by a single IID error rate;
//! - simulated and experimental results must retain their noise-model/source
//!   identity;
//! - detector statistics may be retained as an intermediate hardware/QEC
//!   characterization layer.
//!
//! These requirements prevent Zamani from producing a deceptively simple
//! "QEC score" that hides the behavior actually relevant to fault tolerance.
//!
//! # Public API principle
//!
//! The preferred public path is:
//!
//! ```text
//! quantum::benchmarking::qec
//! ```
//!
//! Protocol-specific access is:
//!
//! ```text
//! quantum::benchmarking::qec::physical
//! quantum::benchmarking::qec::logical
//! quantum::benchmarking::qec::syndrome
//! quantum::benchmarking::qec::decoder
//! quantum::benchmarking::qec::threshold
//! quantum::benchmarking::qec::surface_code
//! quantum::benchmarking::qec::resource_overhead
//! ```
//!
//! No flat re-export is created from `quantum::benchmarking` by this file.
//! That keeps the QEC namespace explicit and avoids collisions with future
//! benchmark families.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

/* ========================================================================= */
/* EXISTING QEC BENCHMARK MODULES                                            */
/* ========================================================================= */

/// Physical error characterization.
///
/// Owns physical-error observations and their statistical aggregation.
///
/// Does not own logical-error analysis, threshold fitting, decoder execution,
/// QPU access, or resource admission.
pub mod physical;

/// Logical error characterization.
///
/// Owns logical-level benchmark analysis and logical error metrics.
pub mod logical;

/// Syndrome-level benchmark representation and analysis.
///
/// Owns syndrome benchmark data and syndrome-specific validation.
pub mod syndrome;

/// Decoder benchmarking contract and measurements.
///
/// Owns decoder-oriented benchmark behavior such as accuracy and decoder
/// performance measurements exposed by the implementation.
pub mod decoder;

/// Threshold experiments.
///
/// Owns code-distance/noise-scaling threshold analysis.
pub mod threshold;

/// Surface-code benchmark workloads and measurements.
///
/// Owns surface-code-specific benchmark construction/analysis.
pub mod surface_code;

/// Physical-to-logical resource overhead.
///
/// Owns resource overhead calculations between physical and logical
/// computation.
pub mod resource_overhead;

/* ========================================================================= */
/* PUBLIC SUBSYSTEM IDENTITY                                                 */
/* ========================================================================= */

/// Stable identifier of the QEC benchmarking subsystem.
pub const QEC_BENCHMARKING_SUBSYSTEM_ID: &str =
    "zamani.quantum.benchmarking.qec";

/// Stable API version for the QEC benchmarking namespace.
///
/// This is the namespace/API version, not the version of an individual
/// benchmark protocol.
pub const QEC_BENCHMARKING_API_VERSION: &str = "1.0.0";

/// Stable architecture identifier.
pub const QEC_BENCHMARKING_ARCHITECTURE: &str =
    "qec-benchmarking-modular-resource-safe";

/* ========================================================================= */
/* STABLE BENCHMARK IDENTIFIERS                                              */
/* ========================================================================= */

/// Physical-error benchmark identifier.
pub const PHYSICAL_BENCHMARK_ID: &str = "qec.physical";

/// Logical-error benchmark identifier.
pub const LOGICAL_BENCHMARK_ID: &str = "qec.logical";

/// Syndrome benchmark identifier.
pub const SYNDROME_BENCHMARK_ID: &str = "qec.syndrome";

/// Decoder benchmark identifier.
pub const DECODER_BENCHMARK_ID: &str = "qec.decoder";

/// Threshold benchmark identifier.
pub const THRESHOLD_BENCHMARK_ID: &str = "qec.threshold";

/// Surface-code benchmark identifier.
pub const SURFACE_CODE_BENCHMARK_ID: &str = "qec.surface_code";

/// Resource-overhead benchmark identifier.
pub const RESOURCE_OVERHEAD_BENCHMARK_ID: &str =
    "qec.resource_overhead";

/* ========================================================================= */
/* BENCHMARK FAMILY                                                          */
/* ========================================================================= */

/// Stable classification of a QEC benchmark family.
///
/// This is deliberately independent of the concrete result types in the
/// individual implementation modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QecBenchmarkKind {
    /// Physical error characterization.
    Physical,

    /// Logical error characterization.
    Logical,

    /// Syndrome characterization.
    Syndrome,

    /// Decoder performance.
    Decoder,

    /// Threshold/scaling experiment.
    Threshold,

    /// Surface-code-specific benchmark.
    SurfaceCode,

    /// Physical/logical resource overhead.
    ResourceOverhead,
}

impl QecBenchmarkKind {
    /// Returns the stable benchmark identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Physical => PHYSICAL_BENCHMARK_ID,
            Self::Logical => LOGICAL_BENCHMARK_ID,
            Self::Syndrome => SYNDROME_BENCHMARK_ID,
            Self::Decoder => DECODER_BENCHMARK_ID,
            Self::Threshold => THRESHOLD_BENCHMARK_ID,
            Self::SurfaceCode => SURFACE_CODE_BENCHMARK_ID,
            Self::ResourceOverhead => RESOURCE_OVERHEAD_BENCHMARK_ID,
        }
    }

    /// Returns a stable human-readable name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Physical => "Physical QEC",
            Self::Logical => "Logical QEC",
            Self::Syndrome => "Syndrome QEC",
            Self::Decoder => "QEC Decoder",
            Self::Threshold => "QEC Threshold",
            Self::SurfaceCode => "Surface Code",
            Self::ResourceOverhead => "QEC Resource Overhead",
        }
    }

    /// Returns whether the benchmark fundamentally requires a logical-QEC
    /// experiment.
    #[must_use]
    pub const fn requires_logical_execution(self) -> bool {
        matches!(
            self,
            Self::Logical
                | Self::Threshold
                | Self::SurfaceCode
                | Self::ResourceOverhead
        )
    }

    /// Returns whether the benchmark is primarily decoder-oriented.
    #[must_use]
    pub const fn is_decoder_oriented(self) -> bool {
        matches!(self, Self::Decoder)
    }

    /// Returns whether the benchmark is primarily a scaling experiment.
    #[must_use]
    pub const fn is_scaling_experiment(self) -> bool {
        matches!(
            self,
            Self::Threshold | Self::SurfaceCode | Self::ResourceOverhead
        )
    }
}

/* ========================================================================= */
/* BENCHMARK INVENTORY                                                        */
/* ========================================================================= */

/// Stable ordering of the currently implemented QEC benchmark families.
///
/// The ordering is part of the public deterministic API and therefore must
/// not depend on hash-map iteration or filesystem ordering.
#[must_use]
pub const fn benchmark_kinds() -> &'static [QecBenchmarkKind] {
    &[
        QecBenchmarkKind::Physical,
        QecBenchmarkKind::Logical,
        QecBenchmarkKind::Syndrome,
        QecBenchmarkKind::Decoder,
        QecBenchmarkKind::Threshold,
        QecBenchmarkKind::SurfaceCode,
        QecBenchmarkKind::ResourceOverhead,
    ]
}

/* ========================================================================= */
/* CAPABILITY INVENTORY                                                       */
/* ========================================================================= */

/// Compile-time description of capabilities represented by the current QEC
/// benchmarking subsystem.
///
/// This structure is informational. It is not an authorization token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QecBenchmarkCapabilities {
    /// Physical error characterization exists.
    pub physical_error: bool,

    /// Logical error characterization exists.
    pub logical_error: bool,

    /// Syndrome benchmarking exists.
    pub syndrome: bool,

    /// Decoder benchmarking exists.
    pub decoder: bool,

    /// Threshold benchmarking exists.
    pub threshold: bool,

    /// Surface-code benchmarking exists.
    pub surface_code: bool,

    /// Resource-overhead benchmarking exists.
    pub resource_overhead: bool,

    /// Code-distance scaling can be represented by the benchmark family.
    pub code_distance_scaling: bool,

    /// Logical error per QEC cycle is a supported conceptual metric.
    pub logical_error_per_cycle: bool,

    /// Decoder latency is a supported conceptual metric.
    pub decoder_latency: bool,

    /// Decoder throughput is a supported conceptual metric.
    pub decoder_throughput: bool,

    /// Detector/detection statistics can be represented by the subsystem.
    pub detector_statistics: bool,

    /// Correlated-error information can be preserved rather than being
    /// necessarily collapsed into an IID scalar.
    pub correlated_errors: bool,

    /// Physical/logical resource conversion can be benchmarked.
    pub resource_scaling: bool,

    /// Simulator-backed benchmarking can be represented.
    pub simulation: bool,

    /// Hardware-backed benchmarking can be represented through the higher-level
    /// execution boundary.
    pub hardware_execution: bool,
}

/// Current QEC benchmarking capability inventory.
pub const QEC_BENCHMARK_CAPABILITIES: QecBenchmarkCapabilities =
    QecBenchmarkCapabilities {
        physical_error: true,
        logical_error: true,
        syndrome: true,
        decoder: true,
        threshold: true,
        surface_code: true,
        resource_overhead: true,

        code_distance_scaling: true,
        logical_error_per_cycle: true,
        decoder_latency: true,
        decoder_throughput: true,
        detector_statistics: true,
        correlated_errors: true,
        resource_scaling: true,

        simulation: true,
        hardware_execution: true,
    };

/* ========================================================================= */
/* METRIC FAMILIES                                                           */
/* ========================================================================= */

/// Stable high-level QEC metric family.
///
/// Individual benchmark modules may expose substantially more detailed
/// metrics. This enum provides a common vocabulary for registry, reporting,
/// CI and Zamani-language integration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QecMetricFamily {
    /// Physical error probability/rate.
    PhysicalErrorRate,

    /// Logical error probability/rate.
    LogicalErrorRate,

    /// Logical error normalized per QEC cycle.
    LogicalErrorPerCycle,

    /// Probability/rate of detection events.
    DetectionRate,

    /// Detector likelihood or equivalent detector-level statistic.
    DetectorLikelihood,

    /// Code-distance scaling/suppression.
    DistanceScaling,

    /// Threshold estimate.
    Threshold,

    /// Decoder correctness/accuracy.
    DecoderAccuracy,

    /// Decoder latency.
    DecoderLatency,

    /// Decoder throughput.
    DecoderThroughput,

    /// Classical-controller/QEC feedback latency.
    FeedbackLatency,

    /// Syndrome extraction rate.
    SyndromeThroughput,

    /// Logical lifetime.
    LogicalLifetime,

    /// Physical-to-logical qubit overhead.
    QubitOverhead,

    /// Physical-to-logical gate overhead.
    GateOverhead,

    /// Physical-to-logical time overhead.
    TimeOverhead,

    /// Leakage rate.
    Leakage,

    /// Erasure rate.
    Erasure,

    /// Correlated-error indicator.
    CorrelatedError,

    /// Resource consumption.
    Resource,
}

impl QecMetricFamily {
    /// Stable machine-readable metric identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::PhysicalErrorRate => "physical_error_rate",
            Self::LogicalErrorRate => "logical_error_rate",
            Self::LogicalErrorPerCycle => "logical_error_per_cycle",
            Self::DetectionRate => "detection_rate",
            Self::DetectorLikelihood => "detector_likelihood",
            Self::DistanceScaling => "distance_scaling",
            Self::Threshold => "threshold",
            Self::DecoderAccuracy => "decoder_accuracy",
            Self::DecoderLatency => "decoder_latency",
            Self::DecoderThroughput => "decoder_throughput",
            Self::FeedbackLatency => "feedback_latency",
            Self::SyndromeThroughput => "syndrome_throughput",
            Self::LogicalLifetime => "logical_lifetime",
            Self::QubitOverhead => "qubit_overhead",
            Self::GateOverhead => "gate_overhead",
            Self::TimeOverhead => "time_overhead",
            Self::Leakage => "leakage",
            Self::Erasure => "erasure",
            Self::CorrelatedError => "correlated_error",
            Self::Resource => "resource",
        }
    }
}

/* ========================================================================= */
/* CAPABILITY DISCOVERY                                                      */
/* ========================================================================= */

/// Returns whether the supplied benchmark kind is currently implemented.
///
/// This is intentionally deterministic and allocation-free.
#[must_use]
pub const fn supports_benchmark(kind: QecBenchmarkKind) -> bool {
    match kind {
        QecBenchmarkKind::Physical
        | QecBenchmarkKind::Logical
        | QecBenchmarkKind::Syndrome
        | QecBenchmarkKind::Decoder
        | QecBenchmarkKind::Threshold
        | QecBenchmarkKind::SurfaceCode
        | QecBenchmarkKind::ResourceOverhead => true,
    }
}

/// Looks up a benchmark family by its stable identifier.
///
/// This function intentionally returns the enum rather than a dynamically
/// allocated registry entry. The universal benchmarking registry can layer a
/// richer descriptor over it later.
#[must_use]
pub const fn benchmark_kind_from_id(
    id: &str,
) -> Option<QecBenchmarkKind> {
    match id {
        PHYSICAL_BENCHMARK_ID => Some(QecBenchmarkKind::Physical),
        LOGICAL_BENCHMARK_ID => Some(QecBenchmarkKind::Logical),
        SYNDROME_BENCHMARK_ID => Some(QecBenchmarkKind::Syndrome),
        DECODER_BENCHMARK_ID => Some(QecBenchmarkKind::Decoder),
        THRESHOLD_BENCHMARK_ID => Some(QecBenchmarkKind::Threshold),
        SURFACE_CODE_BENCHMARK_ID => Some(QecBenchmarkKind::SurfaceCode),
        RESOURCE_OVERHEAD_BENCHMARK_ID => {
            Some(QecBenchmarkKind::ResourceOverhead)
        }
        _ => None,
    }
}

/* ========================================================================= */
/* API METADATA                                                               */
/* ========================================================================= */

/// Immutable metadata describing this QEC benchmark namespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QecBenchmarkingMetadata {
    /// Stable subsystem ID.
    pub subsystem_id: &'static str,

    /// API version.
    pub api_version: &'static str,

    /// Architecture identifier.
    pub architecture: &'static str,

    /// Number of currently implemented benchmark families.
    pub benchmark_count: usize,
}

/// Returns immutable namespace metadata.
#[must_use]
pub const fn metadata() -> QecBenchmarkingMetadata {
    QecBenchmarkingMetadata {
        subsystem_id: QEC_BENCHMARKING_SUBSYSTEM_ID,
        api_version: QEC_BENCHMARKING_API_VERSION,
        architecture: QEC_BENCHMARKING_ARCHITECTURE,
        benchmark_count: benchmark_kinds().len(),
    }
}

/* ========================================================================= */
/* ARCHITECTURAL SELF-CHECK                                                  */
/* ========================================================================= */

/// Result of the QEC benchmarking namespace self-check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QecBenchmarkingSelfCheck {
    /// Whether all registered benchmark identifiers are unique.
    pub unique_ids: bool,

    /// Whether every registered benchmark is marked as supported.
    pub all_supported: bool,

    /// Whether every registered benchmark has a non-empty identifier.
    pub identifiers_valid: bool,

    /// Overall result.
    pub passed: bool,
}

/// Performs a bounded, allocation-free architectural self-check.
///
/// This does not execute a benchmark, allocate resources, access a backend,
/// access a QPU, or perform statistical calculations.
///
/// It is suitable for unit tests, startup diagnostics, and CI architecture
/// checks.
#[must_use]
pub fn self_check() -> QecBenchmarkingSelfCheck {
    let kinds = benchmark_kinds();

    let mut unique_ids = true;
    let mut all_supported = true;
    let mut identifiers_valid = true;

    let mut index = 0usize;

    while index < kinds.len() {
        let id = kinds[index].id();

        if id.is_empty() {
            identifiers_valid = false;
        }

        if !supports_benchmark(kinds[index]) {
            all_supported = false;
        }

        let mut other = index + 1;

        while other < kinds.len() {
            if id == kinds[other].id() {
                unique_ids = false;
            }

            other += 1;
        }

        index += 1;
    }

    QecBenchmarkingSelfCheck {
        unique_ids,
        all_supported,
        identifiers_valid,
        passed: unique_ids && all_supported && identifiers_valid,
    }
}

/* ========================================================================= */
/* TESTS                                                                      */
/* ========================================================================= */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn namespace_has_expected_identity() {
        assert_eq!(
            QEC_BENCHMARKING_SUBSYSTEM_ID,
            "zamani.quantum.benchmarking.qec"
        );

        assert_eq!(QEC_BENCHMARKING_API_VERSION, "1.0.0");

        assert!(
            !QEC_BENCHMARKING_ARCHITECTURE.is_empty()
        );
    }

    #[test]
    fn all_current_modules_are_registered() {
        assert_eq!(benchmark_kinds().len(), 7);

        assert_eq!(
            benchmark_kinds()[0],
            QecBenchmarkKind::Physical
        );

        assert_eq!(
            benchmark_kinds()[1],
            QecBenchmarkKind::Logical
        );

        assert_eq!(
            benchmark_kinds()[2],
            QecBenchmarkKind::Syndrome
        );

        assert_eq!(
            benchmark_kinds()[3],
            QecBenchmarkKind::Decoder
        );

        assert_eq!(
            benchmark_kinds()[4],
            QecBenchmarkKind::Threshold
        );

        assert_eq!(
            benchmark_kinds()[5],
            QecBenchmarkKind::SurfaceCode
        );

        assert_eq!(
            benchmark_kinds()[6],
            QecBenchmarkKind::ResourceOverhead
        );
    }

    #[test]
    fn benchmark_identifiers_are_stable() {
        assert_eq!(
            QecBenchmarkKind::Physical.id(),
            "qec.physical"
        );

        assert_eq!(
            QecBenchmarkKind::Logical.id(),
            "qec.logical"
        );

        assert_eq!(
            QecBenchmarkKind::Syndrome.id(),
            "qec.syndrome"
        );

        assert_eq!(
            QecBenchmarkKind::Decoder.id(),
            "qec.decoder"
        );

        assert_eq!(
            QecBenchmarkKind::Threshold.id(),
            "qec.threshold"
        );

        assert_eq!(
            QecBenchmarkKind::SurfaceCode.id(),
            "qec.surface_code"
        );

        assert_eq!(
            QecBenchmarkKind::ResourceOverhead.id(),
            "qec.resource_overhead"
        );
    }

    #[test]
    fn benchmark_lookup_is_complete() {
        for kind in benchmark_kinds() {
            assert_eq!(
                benchmark_kind_from_id(kind.id()),
                Some(*kind)
            );
        }

        assert_eq!(
            benchmark_kind_from_id("qec.does_not_exist"),
            None
        );
    }

    #[test]
    fn benchmark_ids_are_unique() {
        let check = self_check();

        assert!(check.unique_ids);
        assert!(check.all_supported);
        assert!(check.identifiers_valid);
        assert!(check.passed);
    }

    #[test]
    fn metadata_is_consistent() {
        let metadata = metadata();

        assert_eq!(
            metadata.subsystem_id,
            QEC_BENCHMARKING_SUBSYSTEM_ID
        );

        assert_eq!(
            metadata.api_version,
            QEC_BENCHMARKING_API_VERSION
        );

        assert_eq!(
            metadata.architecture,
            QEC_BENCHMARKING_ARCHITECTURE
        );

        assert_eq!(
            metadata.benchmark_count,
            benchmark_kinds().len()
        );
    }

    #[test]
    fn capability_inventory_is_complete() {
        assert!(QEC_BENCHMARK_CAPABILITIES.physical_error);
        assert!(QEC_BENCHMARK_CAPABILITIES.logical_error);
        assert!(QEC_BENCHMARK_CAPABILITIES.syndrome);
        assert!(QEC_BENCHMARK_CAPABILITIES.decoder);
        assert!(QEC_BENCHMARK_CAPABILITIES.threshold);
        assert!(QEC_BENCHMARK_CAPABILITIES.surface_code);
        assert!(
            QEC_BENCHMARK_CAPABILITIES.resource_overhead
        );

        assert!(
            QEC_BENCHMARK_CAPABILITIES.code_distance_scaling
        );

        assert!(
            QEC_BENCHMARK_CAPABILITIES.logical_error_per_cycle
        );

        assert!(
            QEC_BENCHMARK_CAPABILITIES.decoder_latency
        );

        assert!(
            QEC_BENCHMARK_CAPABILITIES.decoder_throughput
        );

        assert!(
            QEC_BENCHMARK_CAPABILITIES.detector_statistics
        );

        assert!(
            QEC_BENCHMARK_CAPABILITIES.correlated_errors
        );

        assert!(
            QEC_BENCHMARK_CAPABILITIES.resource_scaling
        );
    }

    #[test]
    fn metric_identifiers_are_non_empty() {
        let metrics = [
            QecMetricFamily::PhysicalErrorRate,
            QecMetricFamily::LogicalErrorRate,
            QecMetricFamily::LogicalErrorPerCycle,
            QecMetricFamily::DetectionRate,
            QecMetricFamily::DetectorLikelihood,
            QecMetricFamily::DistanceScaling,
            QecMetricFamily::Threshold,
            QecMetricFamily::DecoderAccuracy,
            QecMetricFamily::DecoderLatency,
            QecMetricFamily::DecoderThroughput,
            QecMetricFamily::FeedbackLatency,
            QecMetricFamily::SyndromeThroughput,
            QecMetricFamily::LogicalLifetime,
            QecMetricFamily::QubitOverhead,
            QecMetricFamily::GateOverhead,
            QecMetricFamily::TimeOverhead,
            QecMetricFamily::Leakage,
            QecMetricFamily::Erasure,
            QecMetricFamily::CorrelatedError,
            QecMetricFamily::Resource,
        ];

        for metric in metrics {
            assert!(!metric.id().is_empty());
        }
    }

    #[test]
    fn scaling_classification_is_stable() {
        assert!(
            QecBenchmarkKind::Threshold
                .is_scaling_experiment()
        );

        assert!(
            QecBenchmarkKind::SurfaceCode
                .is_scaling_experiment()
        );

        assert!(
            !QecBenchmarkKind::Physical
                .is_scaling_experiment()
        );
    }

    #[test]
    fn decoder_classification_is_stable() {
        assert!(
            QecBenchmarkKind::Decoder
                .is_decoder_oriented()
        );

        assert!(
            !QecBenchmarkKind::Logical
                .is_decoder_oriented()
        );
    }

    #[test]
    fn logical_execution_classification_is_stable() {
        assert!(
            QecBenchmarkKind::Logical
                .requires_logical_execution()
        );

        assert!(
            QecBenchmarkKind::Threshold
                .requires_logical_execution()
        );

        assert!(
            !QecBenchmarkKind::Physical
                .requires_logical_execution()
        );
    }
}