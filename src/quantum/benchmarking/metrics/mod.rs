//! Zamani Quantum Benchmarking — Metrics
//!
//! Canonical public module boundary for all derived quantum-benchmarking
//! metrics.
//!
//! # Purpose
//!
//! This module organizes the metric implementations used by
//! `quantum::benchmarking`.
//!
//! It deliberately contains:
//!
//! - module declarations;
//! - stable metric-family documentation;
//! - controlled public exports;
//! - architectural invariants;
//! - module-level compatibility tests.
//!
//! It deliberately does NOT contain:
//!
//! - benchmark protocol implementations;
//! - circuit generation;
//! - circuit execution;
//! - backend communication;
//! - simulator implementation;
//! - Quantum IR implementation;
//! - statistical experiment orchestration;
//! - benchmark-result orchestration;
//! - hardware calibration ownership;
//! - global mutable state;
//! - logging side effects;
//! - protocol-specific execution policy.
//!
//! # Architecture
//!
//! The intended dependency direction is:
//!
//! ```text
//!                         Zamani program
//!                              │
//!                              ▼
//!                    benchmarking API
//!                              │
//!                              ▼
//!                  benchmark protocol
//!                              │
//!                              ▼
//!                    raw observations
//!                              │
//!                              ▼
//!                    statistical analysis
//!                              │
//!                              ▼
//!                 benchmarking::metrics
//!                              │
//!             ┌────────────────┼────────────────┐
//!             │                │                │
//!             ▼                ▼                ▼
//!          fidelity        gate_error       probability
//!             │                │                │
//!             ├──────────┬─────┴──────┬─────────┤
//!             ▼          ▼            ▼         ▼
//!          readout    runtime      resource   throughput
//!             │          │            │         │
//!             └──────────┴─────┬──────┴─────────┘
//!                              ▼
//!                         stability
//!                              │
//!                              ▼
//!                           leakage
//!                              │
//!                              ▼
//!                           logical
//! ```
//!
//! Metric implementations consume validated observations or mathematically
//! well-defined inputs and return explicit metric values/results.
//!
//! They must not reach upward into protocols or execution.
//!
//! # Canonical core integration
//!
//! The universal metric representation lives in:
//!
//! ```text
//! quantum::benchmarking::core::metric
//! ```
//!
//! That module defines the canonical metric vocabulary, units, finite-value
//! guarantees, semantic directions, and related metric metadata.
//!
//! The metric-family modules in this directory implement calculations and
//! domain-specific metric operations. They must not introduce a competing
//! universal metric model.
//!
//! The dependency relationship is therefore:
//!
//! ```text
//! core::metric
//!       ▲
//!       │
//!       ├── metrics::probability
//!       ├── metrics::fidelity
//!       ├── metrics::gate_error
//!       ├── metrics::readout
//!       ├── metrics::runtime
//!       ├── metrics::throughput
//!       ├── metrics::resource
//!       ├── metrics::stability
//!       ├── metrics::leakage
//!       └── metrics::logical
//! ```
//!
//! The exact implementation modules may additionally use standard-library
//! types or their own domain-specific input/result types, but they must not
//! create a second universal benchmark result architecture.
//!
//! # Metric families
//!
//! ## Probability
//!
//! `probability` provides validated probability and distribution operations.
//!
//! It is used by metrics and protocols that transform raw counts into
//! probabilities or validate empirical distributions.
//!
//! ## Fidelity
//!
//! `fidelity` provides state, process, gate, and classical-distribution
//! fidelity calculations where mathematically appropriate.
//!
//! Fidelity conventions must remain explicit. In particular, callers must
//! never silently mix squared and unsquared fidelity conventions.
//!
//! ## Gate error
//!
//! `gate_error` provides gate-, Clifford-, cycle-, and infidelity-related
//! quantities.
//!
//! Protocols such as randomized benchmarking and cycle benchmarking may use
//! these calculations.
//!
//! ## Readout
//!
//! `readout` provides measurement assignment and readout-quality metrics,
//! including confusion/assignment information.
//!
//! ## Runtime
//!
//! `runtime` measures execution lifecycle timing without collapsing distinct
//! stages into a misleading single runtime value.
//!
//! Relevant stages can include:
//!
//! ```text
//! compilation
//! transpilation
//! routing
//! scheduling
//! queue
//! submission
//! execution
//! readout
//! analysis
//! end-to-end wall time
//! ```
//!
//! ## Throughput
//!
//! `throughput` provides rate-based measurements such as:
//!
//! - shots/second;
//! - circuits/second;
//! - gates/second;
//! - two-qubit gates/second;
//! - layers/second;
//! - related execution throughput.
//!
//! Throughput must remain distinguishable from quality/fidelity.
//!
//! ## Resource
//!
//! `resource` provides quantum and classical resource accounting, including:
//!
//! - qubits;
//! - logical qubits;
//! - physical qubits;
//! - gate count;
//! - two-qubit gate count;
//! - depth;
//! - T gates;
//! - measurements;
//! - classical operations;
//! - memory;
//! - energy where supported;
//! - space-time resources.
//!
//! ## Stability
//!
//! `stability` provides temporal and statistical stability measurements,
//! including drift-oriented quantities.
//!
//! ## Leakage
//!
//! `leakage` provides leakage-related measurements for systems in which the
//! computational subspace is physically meaningful.
//!
//! ## Logical
//!
//! `logical` provides logical/fault-tolerant metrics such as logical error
//! rate, logical fidelity, threshold-related quantities, and physical-to-
//! logical overhead.
//!
//! # Separation of observation and metric
//!
//! The production architecture requires the following distinction:
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
//! For example:
//!
//! ```text
//! measured counts
//!       │
//!       ▼
//! probability metric
//!       │
//!       ▼
//! fidelity / success / error metric
//! ```
//!
//! A metric module must not silently discard the raw-data context required to
//! interpret the result.
//!
//! # Universal metric representation
//!
//! `core::metric` is authoritative for universal metric semantics.
//!
//! Metric implementations should ultimately be capable of being represented
//! using the core metric contract, including where applicable:
//!
//! - metric kind;
//! - value;
//! - unit;
//! - uncertainty;
//! - confidence;
//! - sample count;
//! - directionality;
//! - validity;
//! - provenance.
//!
//! A bare `f64` is acceptable only as an internal intermediate numerical value.
//! Public production APIs should return a domain result or a validated core
//! metric representation when the corresponding implementation contract
//! supports it.
//!
//! # Scientific correctness
//!
//! Metric implementations must:
//!
//! 1. reject NaN and infinity;
//! 2. validate dimensions;
//! 3. validate probability ranges;
//! 4. validate normalization where required;
//! 5. distinguish exact from estimated quantities;
//! 6. preserve uncertainty where available;
//! 7. preserve sample-count information where applicable;
//! 8. expose assumptions required by the metric;
//! 9. never silently change a scientific convention;
//! 10. never silently repair invalid scientific input;
//! 11. avoid unchecked integer arithmetic for derived resource quantities;
//! 12. avoid unbounded allocations;
//! 13. avoid global mutable state;
//! 14. avoid printing diagnostics;
//! 15. return structured errors;
//! 16. remain deterministic for deterministic inputs;
//! 17. preserve backend neutrality.
//!
//! # Metric semantics versus benchmark semantics
//!
//! This directory owns metric calculations, not benchmark decisions.
//!
//! For example, the Quantum Volume protocol may define a pass condition based
//! on a heavy-output probability confidence bound. The probability and
//! confidence calculations may be implemented elsewhere, while this directory
//! provides reusable metric calculations.
//!
//! Similarly:
//!
//! ```text
//! RB protocol
//!     │
//!     ▼
//! decay analysis
//!     │
//!     ▼
//! gate-error metric
//! ```
//!
//! The metric module must not decide whether a complete benchmark experiment
//! passes or fails unless that decision is intrinsically part of the metric
//! definition itself.
//!
//! # Backend neutrality
//!
//! These modules must remain usable across supported quantum technologies,
//! including where applicable:
//!
//! - superconducting systems;
//! - trapped ions;
//! - neutral atoms;
//! - photonic systems;
//! - spin/semiconductor systems;
//! - topological systems;
//! - analog systems;
//! - quantum annealers;
//! - gate-model systems;
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
//! Not every metric is valid for every backend. Capability validation belongs
//! to the appropriate benchmark/backend boundary. Metric functions themselves
//! must validate the mathematical input they receive.
//!
//! # Capability-sensitive metrics
//!
//! Some metrics have physical meaning only for particular execution models.
//!
//! Examples:
//!
//! - T1/T2 require a physically meaningful coherence experiment;
//! - leakage requires a computational-subspace definition;
//! - logical error rate requires a logical-QEC context;
//! - energy requires an energy measurement source;
//! - state/process fidelity requires appropriate state/process information;
//! - gate metrics require an appropriate gate representation.
//!
//! A metric implementation must not fabricate a value when its required
//! information is unavailable.
//!
//! The correct behavior is a structured error or explicit unavailable result,
//! according to the metric implementation's public contract.
//!
//! # Exact versus estimated metrics
//!
//! The metric layer must preserve the distinction between:
//!
//! ```text
//! exact
//! estimated
//! inferred
//! simulated
//! experimentally measured
//! model-derived
//! ```
//!
//! A mathematically exact simulator quantity must not automatically be
//! represented as an experimentally measured hardware quantity.
//!
//! That distinction belongs in the result/provenance layer when the universal
//! result is assembled.
//!
//! # Uncertainty
//!
//! Uncertainty is not optional scientific decoration.
//!
//! When an input is statistically estimated, downstream metrics must preserve
//! or propagate uncertainty where the underlying mathematical method supports
//! it.
//!
//! The metric layer must not turn:
//!
//! ```text
//! 0.91 ± 0.03
//! ```
//!
//! into an unexplained:
//!
//! ```text
//! 0.91
//! ```
//!
//! merely for convenience.
//!
//! # Unit policy
//!
//! Every public metric that represents a dimensional quantity must have an
//! unambiguous unit.
//!
//! In particular:
//!
//! ```text
//! ns
//! µs
//! ms
//! s
//!
//! bytes
//! KiB
//! MiB
//! GiB
//!
//! Hz
//! kHz
//! MHz
//! GHz
//! ```
//!
//! must not be mixed without explicit conversion.
//!
//! Percentages and probabilities must also remain distinguishable:
//!
//! ```text
//! probability = 0.95
//! percentage  = 95.0
//! ```
//!
//! # Resource safety
//!
//! Metric calculations may receive data originating from:
//!
//! - hardware;
//! - simulators;
//! - files;
//! - network services;
//! - external providers;
//! - user-defined Zamani benchmarks.
//!
//! Therefore implementations must treat inputs as untrusted at the library
//! boundary.
//!
//! They must guard against:
//!
//! - integer overflow;
//! - allocation amplification;
//! - pathological matrix dimensions;
//! - malformed distributions;
//! - invalid timing values;
//! - invalid resource counts;
//! - NaN/infinity;
//! - inconsistent lengths.
//!
//! Resource limits themselves belong to `core::limits`; metric modules must
//! respect limits supplied by their callers.
//!
//! # Dependency direction
//!
//! The following dependency direction is prohibited:
//!
//! ```text
//! metrics ─X─> protocols
//! metrics ─X─> execution
//! metrics ─X─> hardware
//! metrics ─X─> runtime implementation
//! metrics ─X─> frontend
//! metrics ─X─> Zamani parser
//! metrics ─X─> stdlib::quantum
//! ```
//!
//! The intended relationship is instead:
//!
//! ```text
//! protocols
//!     │
//!     ▼
//! statistics
//!     │
//!     ▼
//! metrics
//!     │
//!     ▼
//! core::metric
//! ```
//!
//! More specifically, the metric implementations may use the core metric
//! vocabulary but must not make the core module depend on this directory.
//!
//! # Integration with `core::metric`
//!
//! The canonical universal metric model is:
//!
//! ```text
//! quantum::benchmarking::core::metric
//! ```
//!
//! This directory is an implementation layer below protocol/result assembly.
//!
//! The expected integration is:
//!
//! ```text
//! metrics::<family>
//!       │
//!       ▼
//! core::metric
//!       │
//!       ▼
//! core::result
//!       │
//!       ▼
//! reporting / analysis / Zamani API
//! ```
//!
//! If a family module has a specialized result type, the conversion into the
//! universal metric model should happen in that module or at the designated
//! result integration boundary. `metrics/mod.rs` intentionally does not own
//! those conversions.
//!
//! # Integration with statistics
//!
//! Statistical primitives live in:
//!
//! ```text
//! quantum::benchmarking::statistics
//! ```
//!
//! Statistics may consume metric primitives or produce inputs for metric
//! calculations, depending on the protocol.
//!
//! This directory must not duplicate:
//!
//! - bootstrap;
//! - confidence intervals;
//! - regression;
//! - hypothesis testing;
//! - outlier policy;
//! - aggregation policy.
//!
//! Those responsibilities belong to `statistics`.
//!
//! # Integration with protocols
//!
//! Protocol modules such as:
//!
//! ```text
//! protocols::quantum_volume
//! protocols::randomized_benchmarking
//! protocols::interleaved_rb
//! protocols::cycle_benchmarking
//! protocols::xeb
//! protocols::coherence
//! protocols::crosstalk
//! protocols::drift
//! ```
//!
//! consume metric functionality after the required experiment data has been
//! generated and analyzed.
//!
//! The protocol owns:
//!
//! - what experiment is performed;
//! - what data is collected;
//! - what statistical model is used;
//! - what benchmark-specific success criterion applies.
//!
//! The metric family owns:
//!
//! - the mathematical metric calculation;
//! - mathematical validation;
//! - metric-specific numerical safeguards.
//!
//! # Integration with application benchmarks
//!
//! Application benchmarks may consume metrics such as:
//!
//! - success probability;
//! - solution quality;
//! - approximation ratio;
//! - estimation error;
//! - energy error;
//! - observable error;
//! - runtime;
//! - throughput;
//! - resource counts;
//! - time-to-solution.
//!
//! Application modules must not require changes to this `mod.rs` merely because
//! they begin consuming an existing metric family.
//!
//! # Integration with QEC
//!
//! QEC benchmarks may consume:
//!
//! ```text
//! logical
//! leakage
//! resource
//! runtime
//! throughput
//! stability
//! fidelity
//! gate_error
//! ```
//!
//! QEC-specific semantics remain in `benchmarking::qec`; this directory
//! remains responsible only for reusable metric calculations.
//!
//! # Integration with hardware
//!
//! Hardware metadata, topology and calibration are owned by the hardware
//! subsystem.
//!
//! Metrics may consume normalized hardware observations but must not own or
//! mutate hardware state.
//!
//! In particular, metric calculation must never:
//!
//! - change calibration;
//! - submit a circuit;
//! - reserve hardware;
//! - modify topology;
//! - modify backend configuration.
//!
//! # Integration with the Zamani language
//!
//! Future Zamani syntax may allow declarations conceptually equivalent to:
//!
//! ```text
//! metric success_probability
//! metric execution_time
//! metric two_qubit_gate_count
//! metric logical_error_rate
//! ```
//!
//! Such language constructs must eventually lower into the stable
//! `quantum::benchmarking` API.
//!
//! The language layer must not depend on the internal implementation of an
//! individual metric module.
//!
//! Therefore adding a new metric implementation should normally require only:
//!
//! 1. adding the new metric module;
//! 2. registering it in the appropriate metric registry, if one exists;
//! 3. exposing its API through its own module.
//!
//! Existing protocols and language integration should not need to be modified
//! merely because another independent metric family has been added.
//!
//! # Public API policy
//!
//! The individual metric modules are deliberately public:
//!
//! ```text
//! benchmarking::metrics::probability
//! benchmarking::metrics::fidelity
//! benchmarking::metrics::gate_error
//! benchmarking::metrics::readout
//! benchmarking::metrics::runtime
//! benchmarking::metrics::throughput
//! benchmarking::metrics::resource
//! benchmarking::metrics::stability
//! benchmarking::metrics::leakage
//! benchmarking::metrics::logical
//! ```
//!
//! We intentionally do NOT wildcard-re-export every item from every module
//! here.
//!
//! This prevents:
//!
//! - name collisions;
//! - accidental public-API expansion;
//! - ambiguous error types;
//! - accidental shadowing of canonical `core::metric` types;
//! - downstream dependence on implementation details.
//!
//! Consumers should explicitly select the owning module.
//!
//! Example:
//!
//! ```ignore
//! use crate::quantum::benchmarking::metrics::fidelity;
//! use crate::quantum::benchmarking::metrics::probability;
//!
//! let probability = probability::validate_probability(0.95)?;
//! # let _ = probability;
//! ```
//!
//! The exact function names remain owned by the corresponding implementation
//! module.
//!
//! # Stable family identifiers
//!
//! The family identifiers below are intentionally independent of individual
//! function names. They can be used by a future registry without requiring
//! changes to this module when additional functions are added to a family.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only.
//!
//! No nightly features are required.
//!
//! # Testing policy
//!
//! This module tests only the module boundary and stable family identifiers.
//! Numerical correctness belongs to the individual metric files.
//!
//! Integration tests belong under the benchmarking test subsystem.
//!
//! # Production invariant
//!
//! This file must remain deliberately boring.
//!
//! If substantial mathematical logic appears here, it is almost certainly in
//! the wrong layer.


// =============================================================================
// Metric family modules
// =============================================================================

/// Probability validation and probability-distribution metrics.
///
/// Owns probability-domain mathematics and validation.
pub mod probability;

/// Quantum-state, quantum-process, gate, and classical-distribution fidelity
/// calculations.
///
/// Owns fidelity mathematics and fidelity-convention documentation.
pub mod fidelity;

/// Gate-error, gate-infidelity, Clifford-error, and cycle-error calculations.
///
/// Owns gate-error-domain mathematics.
pub mod gate_error;

/// Readout, assignment, measurement, and SPAM-related metrics.
///
/// Owns readout-domain mathematics.
pub mod readout;

/// Runtime and latency metrics.
///
/// Owns timing-domain calculations and timing validation.
pub mod runtime;

/// Throughput and rate metrics.
///
/// Owns rate calculations and throughput validation.
pub mod throughput;

/// Quantum/classical resource metrics.
///
/// Owns resource-count calculations and resource validation.
pub mod resource;

/// Temporal stability, variation, and drift metrics.
///
/// Owns stability-domain calculations.
pub mod stability;

/// Leakage metrics.
///
/// Owns computational-subspace leakage calculations.
pub mod leakage;

/// Logical and fault-tolerant metrics.
///
/// Owns logical-error, logical-fidelity, threshold, and overhead calculations.
pub mod logical;


// =============================================================================
// Stable metric-family identifiers
// =============================================================================

/// Stable identifier for the probability metric family.
pub const PROBABILITY_FAMILY: &str = "probability";

/// Stable identifier for the fidelity metric family.
pub const FIDELITY_FAMILY: &str = "fidelity";

/// Stable identifier for the gate-error metric family.
pub const GATE_ERROR_FAMILY: &str = "gate_error";

/// Stable identifier for the readout metric family.
pub const READOUT_FAMILY: &str = "readout";

/// Stable identifier for the runtime metric family.
pub const RUNTIME_FAMILY: &str = "runtime";

/// Stable identifier for the throughput metric family.
pub const THROUGHPUT_FAMILY: &str = "throughput";

/// Stable identifier for the resource metric family.
pub const RESOURCE_FAMILY: &str = "resource";

/// Stable identifier for the stability metric family.
pub const STABILITY_FAMILY: &str = "stability";

/// Stable identifier for the leakage metric family.
pub const LEAKAGE_FAMILY: &str = "leakage";

/// Stable identifier for the logical metric family.
pub const LOGICAL_FAMILY: &str = "logical";

/// Number of built-in metric families.
///
/// This is intentionally a constant rather than a mutable registry count.
pub const BUILTIN_METRIC_FAMILY_COUNT: usize = 10;

/// Stable ordered list of all built-in metric-family identifiers.
///
/// The order is part of the module's stable API and must not be used to infer
/// benchmark execution order. It exists only for discovery/introspection.
///
/// New metric families should be appended rather than inserted in the middle
/// if external consumers persist this ordering.
pub const BUILTIN_METRIC_FAMILIES: [&str; BUILTIN_METRIC_FAMILY_COUNT] = [
    PROBABILITY_FAMILY,
    FIDELITY_FAMILY,
    GATE_ERROR_FAMILY,
    READOUT_FAMILY,
    RUNTIME_FAMILY,
    THROUGHPUT_FAMILY,
    RESOURCE_FAMILY,
    STABILITY_FAMILY,
    LEAKAGE_FAMILY,
    LOGICAL_FAMILY,
];

/// Returns all built-in metric-family identifiers.
///
/// A static slice is returned so callers cannot mutate the registry.
///
/// This function intentionally returns identifiers rather than implementation
/// modules or function pointers. Registry ownership remains outside this
/// module.
#[inline]
pub const fn builtin_metric_families() -> &'static [&'static str] {
    &BUILTIN_METRIC_FAMILIES
}

/// Returns whether a metric-family identifier is built in to Zamani.
///
/// Comparison is exact and case-sensitive. This prevents silently accepting
/// misspelled metric-family identifiers at an API boundary.
#[inline]
pub fn is_builtin_metric_family(identifier: &str) -> bool {
    match identifier {
        PROBABILITY_FAMILY
        | FIDELITY_FAMILY
        | GATE_ERROR_FAMILY
        | READOUT_FAMILY
        | RUNTIME_FAMILY
        | THROUGHPUT_FAMILY
        | RESOURCE_FAMILY
        | STABILITY_FAMILY
        | LEAKAGE_FAMILY
        | LOGICAL_FAMILY => true,

        _ => false,
    }
}


// =============================================================================
// Controlled prelude
// =============================================================================

/// Controlled metric-module prelude.
///
/// The prelude intentionally exports modules, not every function/type from
/// those modules. This keeps API ownership explicit and prevents accidental
/// namespace collisions as the metric implementation grows.
///
/// Example:
///
/// ```ignore
/// use crate::quantum::benchmarking::metrics::prelude::*;
///
/// let _ = probability;
/// ```
pub mod prelude {
    pub use super::{
        fidelity,
        gate_error,
        leakage,
        logical,
        probability,
        readout,
        resource,
        runtime,
        stability,
        throughput,
    };
}


// =============================================================================
// Compatibility aliases
// =============================================================================
//
// These aliases make the ownership boundary explicit while avoiding any
// duplication of implementation.
//
// They are module aliases, not copies of metric APIs.

/// Compatibility namespace for probability metrics.
pub use probability as probability_metrics;

/// Compatibility namespace for fidelity metrics.
pub use fidelity as fidelity_metrics;

/// Compatibility namespace for gate-error metrics.
pub use gate_error as gate_error_metrics;

/// Compatibility namespace for readout metrics.
pub use readout as readout_metrics;

/// Compatibility namespace for runtime metrics.
pub use runtime as runtime_metrics;

/// Compatibility namespace for throughput metrics.
pub use throughput as throughput_metrics;

/// Compatibility namespace for resource metrics.
pub use resource as resource_metrics;

/// Compatibility namespace for stability metrics.
pub use stability as stability_metrics;

/// Compatibility namespace for leakage metrics.
pub use leakage as leakage_metrics;

/// Compatibility namespace for logical metrics.
pub use logical as logical_metrics;


// =============================================================================
// Architectural tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_metric_family_count_is_consistent() {
        assert_eq!(
            BUILTIN_METRIC_FAMILIES.len(),
            BUILTIN_METRIC_FAMILY_COUNT
        );
    }

    #[test]
    fn builtin_metric_families_are_unique() {
        let families = builtin_metric_families();

        for (index, family) in families.iter().enumerate() {
            assert!(
                !families[index + 1..].contains(family),
                "duplicate metric family identifier: {family}"
            );
        }
    }

    #[test]
    fn all_builtin_metric_families_are_recognized() {
        for family in builtin_metric_families() {
            assert!(
                is_builtin_metric_family(family),
                "built-in family was not recognized: {family}"
            );
        }
    }

    #[test]
    fn unknown_metric_family_is_not_builtin() {
        assert!(!is_builtin_metric_family("unknown"));
        assert!(!is_builtin_metric_family(""));
        assert!(!is_builtin_metric_family("Fidelity"));
        assert!(!is_builtin_metric_family("fidelity "));
    }

    #[test]
    fn metric_family_identifiers_are_stable() {
        assert_eq!(PROBABILITY_FAMILY, "probability");
        assert_eq!(FIDELITY_FAMILY, "fidelity");
        assert_eq!(GATE_ERROR_FAMILY, "gate_error");
        assert_eq!(READOUT_FAMILY, "readout");
        assert_eq!(RUNTIME_FAMILY, "runtime");
        assert_eq!(THROUGHPUT_FAMILY, "throughput");
        assert_eq!(RESOURCE_FAMILY, "resource");
        assert_eq!(STABILITY_FAMILY, "stability");
        assert_eq!(LEAKAGE_FAMILY, "leakage");
        assert_eq!(LOGICAL_FAMILY, "logical");
    }

    #[test]
    fn family_order_is_stable() {
        assert_eq!(
            builtin_metric_families(),
            &[
                "probability",
                "fidelity",
                "gate_error",
                "readout",
                "runtime",
                "throughput",
                "resource",
                "stability",
                "leakage",
                "logical",
            ]
        );
    }
}