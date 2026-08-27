//! Zamani Quantum Benchmarking — Analysis subsystem.
//!
//! This module is the authoritative wiring boundary for post-execution
//! benchmark analysis.
//!
//! It does NOT:
//!
//! - execute quantum workloads;
//! - generate quantum circuits;
//! - select quantum hardware;
//! - communicate with providers;
//! - compile circuits;
//! - transpile circuits;
//! - route circuits;
//! - schedule circuits;
//! - implement benchmark protocol mathematics;
//! - own benchmark execution state;
//! - mutate historical benchmark baselines;
//! - access clocks, filesystems, networks, or environment state;
//! - print diagnostics;
//! - maintain process-global state.
//!
//! Those responsibilities belong to the corresponding benchmarking,
//! quantum-runtime, hardware, compiler, and protocol layers.
//!
//! # Architectural position
//!
//! ```text
//! Benchmark protocol / execution
//!             │
//!             ▼
//!      core::result / core::metric
//!             │
//!             ▼
//!       analysis::compare
//!             │
//!             ▼
//!       analysis::baseline
//!             │
//!       ┌─────┴──────────────────┐
//!       ▼                        ▼
//! analysis::regression    analysis::attribution
//!       │                        │
//!       └──────────┬─────────────┘
//!                  ▼
//!          analysis::bottleneck
//!                  │
//!                  ▼
//!           analysis::diagnosis
//!                  │
//!          ┌───────┴────────┐
//!          ▼                ▼
//!      reporting           CI
//! ```
//!
//! The analysis subsystem is therefore downstream of measured benchmark data.
//!
//! ```text
//! quantum::benchmarking::core
//!             │
//!             ▼
//! quantum::benchmarking::analysis
//!             │
//!      ┌──────┼────────┐
//!      ▼      ▼        ▼
//! reporting   CI     registry
//! ```
//!
//! The reverse dependency is forbidden:
//!
//! ```text
//! analysis ─X→ core semantic ownership
//! analysis ─X→ Quantum IR
//! analysis ─X→ execution
//! analysis ─X→ hardware
//! ```
//!
//! # Analysis responsibilities
//!
//! ## `compare`
//!
//! Answers:
//!
//! > What changed between two compatible metric observations?
//!
//! It owns:
//!
//! - absolute difference;
//! - relative difference;
//! - ratio;
//! - metric-direction interpretation;
//! - uncertainty separation;
//! - confidence-interval relationship;
//! - metric-set matching.
//!
//! It must remain the authoritative numerical comparison implementation.
//!
//! ## `baseline`
//!
//! Answers:
//!
//! > Which historical/reference observation corresponds to this candidate?
//!
//! It owns:
//!
//! - immutable baseline snapshots;
//! - benchmark scope;
//! - metric scope;
//! - dimension matching;
//! - baseline/candidate matching.
//!
//! It must not reimplement numerical comparison semantics.
//!
//! ## `regression`
//!
//! Answers:
//!
//! > Does the observed change violate the configured regression policy?
//!
//! It owns:
//!
//! - degradation thresholds;
//! - CI-gate policy;
//! - missing-metric policy;
//! - unresolved-statistics policy;
//! - regression severity.
//!
//! It consumes comparison/baseline results instead of calculating metric
//! deltas itself.
//!
//! ## `attribution`
//!
//! Answers:
//!
//! > Which measured factors are associated with the observed change?
//!
//! It owns:
//!
//! - contribution analysis;
//! - evidence strength;
//! - association versus causation semantics;
//! - factor normalization;
//! - attribution findings.
//!
//! Attribution must never silently turn correlation into causation.
//!
//! ## `bottleneck`
//!
//! Answers:
//!
//! > Which measurable dimension currently limits benchmark performance?
//!
//! It owns bottleneck classification and ranking without owning the underlying
//! benchmark measurements.
//!
//! ## `diagnosis`
//!
//! Answers:
//!
//! > What structured engineering/scientific finding can be made from the
//! available benchmark evidence?
//!
//! Diagnosis consumes already-established findings. It does not execute
//! experiments or silently manufacture missing evidence.
//!
//! # Dependency rules
//!
//! The intended dependency graph inside analysis is:
//!
//! ```text
//! core::metric
//!      │
//!      ▼
//! analysis::compare
//!      │
//!      ▼
//! analysis::baseline
//!      │
//!      ▼
//! analysis::regression
//! ```
//!
//! Attribution, bottleneck analysis, and diagnosis are downstream analytical
//! consumers and may consume appropriate analysis/result data without becoming
//! dependencies of lower-level metric primitives.
//!
//! In particular:
//!
//! ```text
//! core::metric ──► analysis::compare
//! core::metric ──► analysis::baseline
//! analysis::compare ──► analysis::baseline
//! analysis::baseline ──► analysis::regression
//! ```
//!
//! but never:
//!
//! ```text
//! analysis ──► Quantum IR
//! Quantum IR ──► analysis
//! analysis ──► execution
//! execution ──► analysis
//! hardware ──► analysis
//! analysis ──► hardware implementation
//! ```
//!
//! # Why this file contains no business logic
//!
//! `mod.rs` is intentionally a module boundary, not a second analysis engine.
//!
//! Putting comparison, regression, diagnosis, or baseline logic here would
//! create a second ownership layer and would make the subsystem harder to
//! evolve independently.
//!
//! Each child module is therefore responsible for its complete implementation,
//! validation, errors, deterministic behavior, and public API.
//!
//! This file only:
//!
//! 1. declares the authoritative analysis modules;
//! 2. establishes their public namespace;
//! 3. provides a controlled facade;
//! 4. documents dependency boundaries;
//! 5. enforces module-level safety/lint policy.
//!
//! # Public namespace
//!
//! The stable paths are:
//!
//! ```text
//! quantum::benchmarking::analysis::compare
//! quantum::benchmarking::analysis::baseline
//! quantum::benchmarking::analysis::regression
//! quantum::benchmarking::analysis::attribution
//! quantum::benchmarking::analysis::bottleneck
//! quantum::benchmarking::analysis::diagnosis
//! ```
//!
//! Consumers should prefer these explicit namespaces over glob imports.
//!
//! # Integration with the rest of benchmarking
//!
//! The expected production flow is:
//!
//! ```text
//! protocol
//!     │
//!     ▼
//! execution
//!     │
//!     ▼
//! core::observation
//!     │
//!     ▼
//! protocol-specific analysis / metrics
//!     │
//!     ▼
//! core::result
//!     │
//!     ▼
//! analysis
//!     │
//! ├── compare
//! ├── baseline
//! ├── regression
//! ├── attribution
//! ├── bottleneck
//! └── diagnosis
//!     │
//!     ▼
//! reporting / CI / registry / Zamani stdlib
//! ```
//!
//! The analysis layer must therefore be capable of consuming completed
//! benchmark results without requiring the benchmark to be re-executed.
//!
//! This is essential for:
//!
//! - offline analysis;
//! - historical result analysis;
//! - CI regression checks;
//! - simulator-versus-hardware comparison;
//! - compiler-version comparison;
//! - calibration comparison;
//! - cross-backend comparison;
//! - scientific re-analysis;
//! - reproducibility audits.
//!
//! # Baseline and comparison ownership
//!
//! `compare.rs` is authoritative for metric comparison. The existing baseline
//! implementation is intentionally designed to call the comparison layer
//! rather than duplicate it.
//!
//! Therefore the dependency is:
//!
//! ```text
//! baseline
//!    │
//!    ▼
//! compare
//! ```
//!
//! and regression consumes the resulting baseline comparison:
//!
//! ```text
//! baseline
//!    │
//!    ▼
//! BaselineComparison
//!    │
//!    ▼
//! regression
//! ```
//!
//! No future reporting or CI module should implement its own independent
//! definition of metric regression.
//!
//! # Statistical ownership
//!
//! Analysis does not replace the statistical subsystem.
//!
//! Statistical operations such as:
//!
//! - confidence intervals;
//! - bootstrap;
//! - regression fitting;
//! - hypothesis testing;
//! - outlier detection;
//! - distribution analysis
//!
//! belong to `quantum::benchmarking::statistics`.
//!
//! Analysis consumes their established outputs and applies comparison,
//! historical, attribution, bottleneck, or engineering policy semantics.
//!
//! This prevents statistical mathematics from being duplicated across
//! comparison and regression implementations.
//!
//! # Protocol ownership
//!
//! Protocol implementations remain under:
//!
//! ```text
//! quantum::benchmarking::protocols
//! ```
//!
//! Examples include:
//!
//! - Quantum Volume;
//! - randomized benchmarking;
//! - interleaved randomized benchmarking;
//! - simultaneous randomized benchmarking;
//! - purity randomized benchmarking;
//! - leakage benchmarking;
//! - cycle benchmarking;
//! - layer fidelity;
//! - XEB;
//! - random circuit sampling;
//! - mirror circuits;
//! - SPAM;
//! - coherence;
//! - crosstalk;
//! - drift;
//! - tomography.
//!
//! Those protocols calculate or produce benchmark observations. Analysis
//! interprets already-produced observations and results.
//!
//! # Application benchmark ownership
//!
//! Application workloads remain under:
//!
//! ```text
//! quantum::benchmarking::applications
//! ```
//!
//! Examples:
//!
//! - VQE;
//! - QAOA;
//! - MaxCut;
//! - Grover;
//! - QFT;
//! - phase estimation;
//! - amplitude estimation;
//! - Hamiltonian simulation;
//! - HHL;
//! - Monte Carlo;
//! - Shor;
//! - user-defined Zamani benchmarks.
//!
//! Analysis must remain application-independent.
//!
//! # Hardware ownership
//!
//! Hardware capability, topology, calibration, timing, and provider-specific
//! execution remain owned by the hardware subsystem.
//!
//! Analysis may consume normalized hardware metadata already attached to a
//! benchmark result, but it must not access hardware directly.
//!
//! This preserves the important architectural direction:
//!
//! ```text
//! hardware → execution → observation/result → analysis
//! ```
//!
//! rather than:
//!
//! ```text
//! analysis → hardware
//! ```
//!
//! # Quantum IR ownership
//!
//! The canonical Quantum IR remains the authoritative representation of
//! quantum semantics.
//!
//! Analysis does not create a second circuit representation and does not
//! modify Quantum IR semantics.
//!
//! The intended direction is:
//!
//! ```text
//! Quantum IR
//!      │
//!      ▼
//! benchmark generation/execution
//!      │
//!      ▼
//! benchmark result
//!      │
//!      ▼
//! analysis
//! ```
//!
//! # Reproducibility
//!
//! This module has no process-global state and introduces no randomness.
//!
//! Loading this module must not:
//!
//! - access the clock;
//! - read environment variables;
//! - read files;
//! - access the network;
//! - initialize hardware;
//! - initialize a simulator;
//! - allocate benchmark datasets;
//! - create threads.
//!
//! Determinism is inherited from the child analysis modules and their explicit
//! inputs.
//!
//! # Security and resource safety
//!
//! The analysis module itself performs no unbounded work during module loading.
//!
//! Individual child modules are responsible for validating untrusted analysis
//! inputs.
//!
//! In particular, the existing analysis architecture is expected to reject or
//! bound:
//!
//! - non-finite metric values;
//! - invalid uncertainties;
//! - malformed thresholds;
//! - duplicate metric identities;
//! - duplicate baseline scopes;
//! - excessive metric collections;
//! - malformed attribution factors;
//! - invalid bottleneck data;
//! - invalid diagnosis inputs.
//!
//! No child module should assume that benchmark results came from a trusted
//! local producer. Serialized benchmark results may have crossed a process or
//! machine boundary.
//!
//! # Serialization boundary
//!
//! This module does not define a competing serialization format.
//!
//! Serialization ownership remains with the underlying data/result/reporting
//! layers.
//!
//! In particular, analysis results should be serializable without requiring
//! execution to be repeated.
//!
//! # Versioning
//!
//! Each child analysis module owns its own semantic schema/version identifier.
//! This module deliberately does not invent a second combined schema version,
//! because doing so would falsely imply that all analysis contracts evolve as
//! one indivisible artifact.
//!
//! A future universal benchmark-result schema may record the individual
//! analysis schema versions explicitly.
//!
//! # Integration contract for future files
//!
//! Once this file is installed, future benchmarking modules should integrate
//! through these existing boundaries without modifying this file merely because
//! a new consumer is added:
//!
//! ```text
//! core::result
//!      │
//!      ├── analysis::compare
//!      ├── analysis::baseline
//!      ├── analysis::attribution
//!      ├── analysis::bottleneck
//!      └── analysis::diagnosis
//!
//! analysis::baseline
//!      │
//!      ▼
//! analysis::regression
//! ```
//!
//! Reporting, CI, registry, and Zamani standard-library integrations should
//! consume the established public APIs of these modules.
//!
//! A new analysis family should be added as a new child module rather than by
//! putting its implementation into this file.
//!
//! # Parent-module integration
//!
//! The current `quantum/mod.rs` historically declares `benchmarking` inline and
//! exposes `volume_estimator` through an explicit path. For this directory
//! module to become reachable, the parent must expose this `analysis`
//! submodule under `quantum::benchmarking`.
//!
//! The required parent-side shape is:
//!
//! ```text
//! quantum::benchmarking
//!     ├── volume_estimator
//!     └── analysis
//!          ├── compare
//!          ├── baseline
//!          ├── regression
//!          ├── attribution
//!          ├── bottleneck
//!          └── diagnosis
//! ```
//!
//! The parent should not duplicate any child analysis declarations elsewhere.
//!
//! # Backwards compatibility
//!
//! This file does not remove or rename the existing flat
//! `quantum::volume_estimator` compatibility path.
//!
//! The legacy path remains a parent-module concern.
//!
//! The authoritative new path is:
//!
//! ```text
//! quantum::benchmarking::analysis::*
//! ```
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021 edition
//!
//! No nightly features are required.
//!
//! No unsafe code is permitted.
//!
//! No additional dependency is introduced by this module.
//!
//! =============================================================================
//! Module declarations
//! =============================================================================

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

/// Deterministic comparison of compatible benchmark metrics and metric sets.
///
/// This is the authoritative owner of numerical comparison semantics.
pub mod compare;

/// Immutable, scope-aware historical/reference benchmark baselines.
///
/// Baselines delegate metric comparison to [`compare`].
pub mod baseline;

/// Explicit policy-based historical regression analysis.
///
/// Regression consumes baseline/comparison results and applies policy.
pub mod regression;

/// Evidence-aware association/contribution analysis.
///
/// Attribution does not claim causality without explicit controlled evidence.
pub mod attribution;

/// Identification of limiting benchmark performance dimensions.
///
/// Bottleneck analysis consumes benchmark evidence but does not execute it.
pub mod bottleneck;

/// Structured interpretation and diagnosis of benchmark findings.
///
/// Diagnosis remains downstream of measurement and analysis.
pub mod diagnosis;

// =============================================================================
// Controlled public facade
// =============================================================================

/// Stable analysis facade.
///
/// The facade intentionally re-exports module namespaces rather than every
/// symbol. This prevents collisions between independently evolving analysis
/// APIs and keeps call sites explicit.
///
/// Example:
///
/// ```rust
/// use crate::quantum::benchmarking::analysis::prelude::compare;
///
/// // compare::compare_metrics(...)
/// ```
pub mod prelude {
    pub use super::attribution;
    pub use super::baseline;
    pub use super::bottleneck;
    pub use super::compare;
    pub use super::diagnosis;
    pub use super::regression;
}

// =============================================================================
// Architectural tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analysis_modules_are_reachable() {
        let _ = std::any::type_name::<compare::ComparisonPolicy>();
        let _ = std::any::type_name::<baseline::Baseline>();
        let _ = std::any::type_name::<regression::RegressionPolicy>();
        let _ = std::any::type_name::<attribution::MetricSnapshot>();
    }

    #[test]
    fn analysis_facade_is_reachable() {
        let _ = std::any::type_name::<prelude::compare::ComparisonPolicy>();
        let _ = std::any::type_name::<prelude::baseline::Baseline>();
        let _ = std::any::type_name::<prelude::regression::RegressionPolicy>();
    }
}