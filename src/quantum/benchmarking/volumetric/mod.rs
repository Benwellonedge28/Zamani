//! Zamani Quantum Benchmarking — Volumetric Benchmarking
//!
//! # Purpose
//!
//! This module is the public module boundary for Zamani's volumetric quantum
//! benchmarking subsystem.
//!
//! Volumetric benchmarking measures quantum-computing performance across
//! multiple workload dimensions rather than reducing a system to a single
//! scalar number.
//!
//! The canonical Zamani volumetric space is currently:
//!
//! ```text
//!                         circuit depth
//!                              ↑
//!                              │
//!                              │       ●
//!                              │    ●  ●
//!                              │ ●  ●  ●
//!                              │● ●  ●
//!                              └──────────────────→ circuit width
//! ```
//!
//! Each measured point represents:
//!
//! ```text
//! (width, depth) -> quality
//! ```
//!
//! The subsystem is deliberately designed so that additional dimensions can
//! be introduced by the broader benchmarking framework without changing the
//! semantics of the existing two-dimensional volumetric API.
//!
//! # Architectural responsibility
//!
//! This module owns:
//!
//! - volumetric module wiring;
//! - public visibility of volumetric components;
//! - stable volumetric namespace boundaries;
//! - volumetric-family documentation;
//! - public re-exports of stable volumetric primitives;
//! - architectural smoke tests;
//! - dependency-direction guarantees.
//!
//! This module does NOT own:
//!
//! - quantum circuit generation;
//! - quantum circuit execution;
//! - backend selection;
//! - hardware communication;
//! - simulator execution;
//! - compilation;
//! - optimization;
//! - routing;
//! - scheduling;
//! - Quantum IR semantics;
//! - statistical fitting;
//! - Quantum Volume protocol semantics;
//! - XEB protocol semantics;
//! - randomized benchmarking;
//! - application algorithms;
//! - QEC algorithms;
//! - report generation;
//! - process-global state.
//!
//! Those responsibilities remain in their owning subsystems.
//!
//! # Dependency direction
//!
//! The intended architecture is:
//!
//! ```text
//!                     Zamani Quantum Program
//!                              │
//!                              ▼
//!                         Quantum IR
//!                              │
//!                              ▼
//!                    Benchmark Workload
//!                              │
//!             ┌────────────────┴────────────────┐
//!             ▼                                 ▼
//!       Circuit Generator                 Application Generator
//!             │                                 │
//!             └────────────────┬────────────────┘
//!                              ▼
//!                         Compilation
//!                              │
//!                              ▼
//!                    Benchmark Execution
//!                              │
//!                              ▼
//!                    Raw Observations
//!                              │
//!                              ▼
//!                     Metrics / Statistics
//!                              │
//!                              ▼
//!                    volumetric::volume
//!                              │
//!                 ┌────────────┼────────────┐
//!                 ▼            ▼            ▼
//!              surface      frontier    positioning
//!                 │            │            │
//!                 └────────────┼────────────┘
//!                              ▼
//!                         Analysis
//!                              │
//!                              ▼
//!                         Reporting
//! ```
//!
//! The important dependency invariant is:
//!
//! ```text
//! volumetric
//!     ──may depend on──> core/data-model abstractions
//!     ──may be consumed by──> protocols/applications/QEC/analysis/reporting
//!
//! volumetric
//!     ──must NOT own──> IR
//!     ──must NOT own──> execution
//!     ──must NOT own──> hardware
//!     ──must NOT own──> compilation
//! ```
//!
//! # Two-dimensional canonical model
//!
//! The first production volumetric model uses:
//!
//! ```text
//! X = circuit width
//! Y = circuit depth
//! Z = measured quality
//! ```
//!
//! `volume.rs` owns the fundamental point/data-model representation.
//!
//! `surface.rs` owns the collection and deterministic representation of
//! measured points.
//!
//! `frontier.rs` owns extraction of supported/performance boundaries from a
//! measured surface.
//!
//! `positioning.rs` owns comparison/positioning of systems or benchmark
//! results in volumetric space.
//!
//! Keeping these responsibilities separate is intentional.
//!
//! # Why this module does not implement the mathematics
//!
//! `mod.rs` is a namespace boundary, not a computational implementation.
//!
//! Putting mathematical algorithms here would create an unstable aggregation
//! layer and make future protocol integration unnecessarily difficult.
//!
//! The implementation files are therefore intentionally independent:
//!
//! ```text
//! volume.rs
//!     │
//!     ├── data model
//!     ├── coordinate validation
//!     ├── quality semantics
//!     └── deterministic point handling
//!
//! surface.rs
//!     │
//!     └── measured performance surface
//!
//! frontier.rs
//!     │
//!     └── supported/performance frontier
//!
//! positioning.rs
//!     │
//!     └── cross-result/system positioning
//! ```
//!
//! # Integration with Quantum Volume
//!
//! Quantum Volume is a protocol, not the volumetric subsystem itself.
//!
//! The intended relationship is:
//!
//! ```text
//! generators::qv
//!       │
//!       ▼
//! protocols::quantum_volume
//!       │
//!       ▼
//! execution
//!       │
//!       ▼
//! volume_estimator
//!       │
//!       ▼
//! volumetric::volume
//!       │
//!       ▼
//! volumetric::surface
//!       │
//!       ▼
//! volumetric::frontier
//! ```
//!
//! `volume_estimator.rs` remains responsible for Quantum Volume's heavy-output
//! statistical calculation. The volumetric subsystem consumes the resulting
//! measured quality rather than reimplementing the QV mathematics.
//!
//! The existing QV estimator explicitly separates mathematical evaluation from
//! circuit generation and execution; this boundary is preserved here.
//!
//! # Integration with XEB
//!
//! XEB may produce a quality value for each circuit width/depth point.
//!
//! The intended relationship is:
//!
//! ```text
//! protocols::xeb
//!       │
//!       ▼
//! XEB quality estimate
//!       │
//!       ▼
//! volumetric::volume
//!       │
//!       ▼
//! volumetric::surface
//! ```
//!
//! The volumetric subsystem does not assume that the quality value came from
//! XEB. It can represent fidelity, success probability, error rate, runtime,
//! energy, or another explicitly defined metric.
//!
//! # Integration with application benchmarks
//!
//! Application benchmarks may produce points such as:
//!
//! ```text
//! (problem_size, circuit_depth) -> success_probability
//! ```
//!
//! or:
//!
//! ```text
//! (qubits, depth) -> execution_time
//! ```
//!
//! The application subsystem is responsible for mapping its workload
//! dimensions to the canonical volumetric width/depth representation.
//!
//! The volumetric subsystem must never know whether the workload originated
//! from:
//!
//! - QFT;
//! - Grover;
//! - QAOA;
//! - VQE;
//! - MaxCut;
//! - Hamiltonian simulation;
//! - phase estimation;
//! - amplitude estimation;
//! - chemistry;
//! - cryptography;
//! - machine learning;
//! - a user-defined Zamani benchmark.
//!
//! # Integration with QEC
//!
//! QEC benchmarks can use volumetric representation for measurements such as:
//!
//! ```text
//! (physical_qubits, logical_depth) -> logical_fidelity
//! ```
//!
//! or:
//!
//! ```text
//! (code_distance, logical_cycles) -> logical_error_rate
//! ```
//!
//! The QEC subsystem owns the physical/logical semantics.
//!
//! This module only provides the multidimensional benchmark representation.
//!
//! # Quality semantics
//!
//! A volumetric point must never assume that larger numerical values are
//! automatically better.
//!
//! Examples:
//!
//! ```text
//! fidelity             -> higher is better
//! success_probability  -> higher is better
//! error_rate           -> lower is better
//! runtime              -> lower is better
//! energy               -> lower is better
//! throughput           -> higher is better
//! ```
//!
//! `volume.rs` therefore owns explicit quality-direction and quality-domain
//! semantics. `frontier.rs` and `positioning.rs` must consume those semantics
//! rather than inventing their own comparisons.
//!
//! # Missing measurements
//!
//! A missing `(width, depth)` point is not equivalent to a failed benchmark.
//!
//! The subsystem must distinguish:
//!
//! ```text
//! measured failure
//! measured success
//! missing measurement
//! invalid measurement
//! unavailable backend capability
//! execution failure
//! cancelled execution
//! ```
//!
//! `surface.rs` owns the representation of measured points.
//!
//! Higher-level execution/validation layers remain responsible for explaining
//! why a point is absent.
//!
//! # No interpolation by default
//!
//! Volumetric benchmarking must not silently interpolate missing points.
//!
//! If interpolation is ever introduced, it must be an explicitly requested
//! analysis operation outside the canonical measured surface.
//!
//! This preserves scientific reproducibility and prevents estimated values
//! from being mistaken for experimental observations.
//!
//! # Determinism
//!
//! This module guarantees deterministic module-level behavior.
//!
//! It owns no:
//!
//! - global RNG;
//! - global cache;
//! - global backend;
//! - global configuration;
//! - global mutable registry;
//! - process-wide benchmark state.
//!
//! Deterministic benchmark generation belongs to
//! `benchmarking::generators`.
//!
//! Reproducibility metadata belongs to `benchmarking::core`.
//!
//! # Resource safety
//!
//! The volumetric implementation is intended to be usable with both:
//!
//! - trusted internal benchmark results;
//! - externally supplied/serialized benchmark data.
//!
//! Consequently, the underlying volumetric implementation is expected to
//! enforce:
//!
//! - finite-number validation;
//! - coordinate validation;
//! - duplicate detection;
//! - bounded point counts;
//! - overflow-safe coordinate calculations;
//! - explicit quality semantics;
//! - deterministic ordering.
//!
//! The module boundary must not bypass those checks by exposing alternate
//! unchecked constructors.
//!
//! # Public API policy
//!
//! The preferred public API is:
//!
//! ```text
//! quantum::benchmarking::volumetric::volume
//! quantum::benchmarking::volumetric::surface
//! quantum::benchmarking::volumetric::frontier
//! quantum::benchmarking::volumetric::positioning
//! ```
//!
//! Stable commonly used primitives are also re-exported from this module.
//!
//! The implementation modules remain public because advanced benchmarking,
//! protocol, testing, and analysis code may need their complete APIs.
//!
//! # Integration with `benchmarking::core`
//!
//! The eventual universal benchmark result should be able to carry volumetric
//! information without making the core result model depend on this concrete
//! module.
//!
//! The preferred direction is:
//!
//! ```text
//! core::result
//!      │
//!      └── contains/owns generic benchmark metrics
//!                    │
//!                    ▼
//!             volumetric::surface
//! ```
//!
//! not:
//!
//! ```text
//! core::result
//!      ▲
//!      │
//! volumetric::mod
//! ```
//!
//! The volumetric subsystem therefore remains a specialized analysis/data
//! layer, while `core` remains the universal benchmark contract.
//!
//! # Integration with reporting
//!
//! Reporting modules should consume the typed volumetric structures rather than
//! parsing implementation-specific strings.
//!
//! The intended flow is:
//!
//! ```text
//! volumetric result
//!       │
//!       ├── JSON
//!       ├── CSV
//!       ├── Markdown
//!       └── human summary
//! ```
//!
//! No reporting code should be required to modify this module's semantics.
//!
//! # Integration with regression analysis
//!
//! Regression analysis should be able to compare two volumetric surfaces:
//!
//! ```text
//! baseline surface
//!       │
//!       ├── point-by-point comparison
//!       ├── frontier comparison
//!       ├── supported-volume comparison
//!       └── quality degradation analysis
//!
//! candidate surface
//! ```
//!
//! The volumetric module supplies deterministic data structures; the analysis
//! subsystem owns the meaning of a software/hardware regression.
//!
//! # Integration with the Zamani language
//!
//! A future Zamani source-level benchmark may conceptually express:
//!
//! ```text
//! benchmark my_algorithm {
//!     dimensions {
//!         width 2..32
//!         depth 1..128
//!     }
//!
//!     metric success_probability
//!
//!     threshold 0.95
//! }
//! ```
//!
//! The exact Zamani syntax belongs to the language/frontend layer.
//!
//! This Rust module must remain independent of that syntax.
//!
//! The language frontend should lower the source declaration into the stable
//! benchmark/core representation and eventually produce volumetric points.
//!
//! Therefore adding or changing Zamani benchmark syntax must not require
//! changing this `mod.rs`.
//!
//! # Backend independence
//!
//! Volumetric benchmarking is backend-neutral.
//!
//! It may consume measurements produced by:
//!
//! - CPU simulators;
//! - GPU simulators;
//! - state-vector simulators;
//! - stabilizer simulators;
//! - tensor-network simulators;
//! - density-matrix simulators;
//! - superconducting hardware;
//! - trapped-ion hardware;
//! - neutral-atom hardware;
//! - photonic hardware;
//! - spin/semiconductor hardware;
//! - topological systems;
//! - annealers;
//! - analog quantum systems;
//! - logical-qubit/fault-tolerant systems.
//!
//! Backend capability negotiation belongs to `quantum::hardware` and the
//! benchmark execution layer.
//!
//! # Stable namespace
//!
//! The public namespace established by this file is:
//!
//! ```text
//! quantum::benchmarking::volumetric
//! ├── volume
//! ├── surface
//! ├── frontier
//! └── positioning
//! ```
//!
//! New volumetric analysis modules may be added in future versions, but they
//! must not redefine the semantics of the four canonical components.
//!
//! # Compatibility policy
//!
//! Existing users of:
//!
//! ```text
//! volumetric::volume
//! volumetric::surface
//! volumetric::frontier
//! volumetric::positioning
//! ```
//!
//! must continue to work across compatible releases.
//!
//! New functionality should be additive where possible.
//!
//! Breaking changes require an explicit schema/API version change rather than
//! silently changing the meaning of an existing metric.
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
//! # File integration contract
//!
//! This file is deliberately written so that the sibling implementation files
//! can be completed independently.
//!
//! The sibling contracts are:
//!
//! ## `volume.rs`
//!
//! Owns:
//!
//! - `VolumePoint`;
//! - quality semantics;
//! - coordinate validation;
//! - point-level validation;
//! - deterministic point representation.
//!
//! It must not import `surface`, `frontier`, or `positioning`.
//!
//! ## `surface.rs`
//!
//! Owns:
//!
//! - collection of measured points;
//! - duplicate detection;
//! - deterministic point lookup;
//! - missing-point analysis;
//! - surface-level validation.
//!
//! It may depend on `volume`.
//!
//! It must not depend on `frontier` or `positioning`.
//!
//! ## `frontier.rs`
//!
//! Owns:
//!
//! - supported boundary extraction;
//! - passing frontier;
//! - Pareto/frontier analysis;
//! - maximum supported coordinates.
//!
//! It may depend on `volume` and `surface`.
//!
//! It must not modify the meaning of either.
//!
//! ## `positioning.rs`
//!
//! Owns:
//!
//! - comparison of volumetric benchmark results;
//! - relative positioning;
//! - point/surface comparison;
//! - comparison metadata.
//!
//! It may depend on `volume`, `surface`, and `frontier`.
//!
//! It must not introduce a second quality model.
//!
//! # Future extensibility
//!
//! The architecture deliberately leaves room for future volumetric dimensions,
//! including:
//!
//! - width;
//! - depth;
//! - logical width;
//! - physical width;
//! - time;
//! - fidelity;
//! - error rate;
//! - throughput;
//! - energy;
//! - memory;
//! - cost;
//! - logical cycle count;
//! - code distance.
//!
//! The current canonical surface remains two-dimensional so that existing
//! benchmarks remain deterministic and easy to compare.
//!
//! Future higher-dimensional benchmark models should be introduced explicitly
//! rather than changing the meaning of `(width, depth)`.
//!
//! # Scientific integrity
//!
//! This module follows the core principle that a benchmark result must preserve
//! the distinction between:
//!
//! ```text
//! observation
//! estimate
//! threshold decision
//! extrapolation
//! comparison
//! ```
//!
//! A volumetric surface represents observations/validated benchmark results.
//! Statistical estimation belongs to the statistics layer.
//!
//! Threshold selection belongs to the protocol/configuration layer.
//!
//! Cross-system interpretation belongs to analysis.
//!
//! Reports belong to reporting.
//!
//! This separation prevents a visualization or convenience API from silently
//! becoming a scientific source of truth.
//!
//! # External benchmark alignment
//!
//! The design is intentionally compatible with application-oriented benchmark
//! methodologies in which workload size, quality, runtime, and quantum resources
//! are evaluated together rather than relying on a single scalar benchmark.
//!
//! It also supports the style of benchmark data organization used by hardware
//! benchmark suites where Quantum Volume, random-circuit sampling, mirror
//! benchmarking, algorithmic workloads, and error-correction experiments are
//! maintained as separate benchmark families.
//!
//! The volumetric subsystem is therefore a reusable representation layer rather
//! than an implementation of any vendor-specific benchmark methodology.

// ============================================================================
// Canonical volumetric implementation modules
// ============================================================================

/// Fundamental volumetric point, quality semantics, limits, validation, and
/// deterministic point-level data model.
///
/// This is the lowest-level module in the volumetric subsystem.
pub mod volume;

/// Measured volumetric performance surface.
///
/// This module aggregates validated `(width, depth)` measurements without
/// executing or generating quantum workloads.
pub mod surface;

/// Supported/performance frontier extraction.
///
/// This module derives boundaries from an already validated surface.
pub mod frontier;

/// Cross-surface and cross-system volumetric positioning/comparison.
///
/// This module performs comparison and positioning but does not alter the
/// underlying measurement semantics.
pub mod positioning;

// ============================================================================
// Stable re-exports
// ============================================================================

/// Stable volumetric benchmark-family identifier.
///
/// Re-exported from `volume` so callers do not need to know which implementation
/// file owns the constant.
pub use volume::VOLUMETRIC_BENCHMARK_ID;

/// Stable volumetric data-model schema version.
///
/// This is the schema version of the underlying volumetric point/data model,
/// not the version of the entire Zamani benchmark framework.
pub use volume::VOLUMETRIC_VOLUME_SCHEMA_VERSION;

// ============================================================================
// Stable public prelude
// ============================================================================

/// Small stable prelude for applications and benchmark protocols that need the
/// common volumetric primitives.
///
/// The complete implementation remains available through the four public
/// modules.
///
/// This prelude intentionally contains only foundational types whose ownership
/// is stable. Specialized frontier and positioning APIs remain under their
/// respective modules to avoid namespace collisions as the subsystem evolves.
pub mod prelude {
    pub use super::volume::{
        QualityDirection,
        QualityDomain,
        QualitySpec,
    };
}

// ============================================================================
// Architectural smoke tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_modules_are_reachable() {
        let _ = std::any::TypeId::of::<
            volume::QualityDirection,
        >();

        let _ = std::any::TypeId::of::<
            volume::QualityDomain,
        >();

        let _ = std::any::TypeId::of::<
            volume::QualitySpec,
        >();
    }

    #[test]
    fn canonical_benchmark_identity_is_stable() {
        assert_eq!(
            VOLUMETRIC_BENCHMARK_ID,
            "volumetric"
        );
    }

    #[test]
    fn canonical_schema_version_is_nonzero() {
        assert!(
            VOLUMETRIC_VOLUME_SCHEMA_VERSION > 0
        );
    }

    #[test]
    fn quality_direction_semantics_are_explicit() {
        assert!(
            QualityDirection::HigherIsBetter
                .satisfies(0.95, 0.90)
        );

        assert!(
            QualityDirection::LowerIsBetter
                .satisfies(0.05, 0.10)
        );

        assert!(
            QualityDirection::HigherIsBetter
                .is_better(0.95, 0.90)
        );

        assert!(
            QualityDirection::LowerIsBetter
                .is_better(0.05, 0.10)
        );
    }

    #[test]
    fn quality_domains_are_explicit() {
        assert_eq!(
            QualityDomain::UnitInterval.as_str(),
            "unit_interval"
        );

        assert_eq!(
            QualityDomain::Finite.as_str(),
            "finite"
        );
    }

    #[test]
    fn standard_quality_specifications_are_constructible() {
        let fidelity =
            QualitySpec::fidelity(0.90)
                .expect("valid fidelity specification");

        assert_eq!(
            fidelity.metric_id,
            "fidelity"
        );

        assert_eq!(
            fidelity.direction,
            QualityDirection::HigherIsBetter
        );

        let error_rate =
            QualitySpec::error_rate(0.10)
                .expect("valid error-rate specification");

        assert_eq!(
            error_rate.metric_id,
            "error_rate"
        );

        assert_eq!(
            error_rate.direction,
            QualityDirection::LowerIsBetter
        );
    }
}