//! Zamani Quantum Benchmarking — Validation Boundary
//!
//! Production validation facade for the complete quantum benchmarking
//! subsystem.
//!
//! # Purpose
//!
//! This module is the authoritative module boundary for benchmark validation.
//! It organizes the independent validation domains used throughout
//! `quantum::benchmarking`:
//!
//! - [`input`] — untrusted benchmark configuration/input validation;
//! - [`statistical`] — statistical-domain validation;
//! - [`physical`] — physical-domain validation;
//! - [`reproducibility`] — reproducibility/provenance identity validation.
//!
//! The individual modules own their validation rules. This file owns only:
//!
//! 1. module declaration;
//! 2. public API exposure;
//! 3. validation-domain organization;
//! 4. the stable validation prelude;
//! 5. architectural dependency boundaries.
//!
//! It deliberately contains no duplicated validation algorithm.
//!
//! # Architectural position
//!
//! The intended production dependency direction is:
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                    benchmark configuration
//!                              │
//!                              ▼
//!                     validation::input
//!                              │
//!                              ▼
//!                       benchmark core
//!                              │
//!            ┌─────────────────┼─────────────────┐
//!            │                 │                 │
//!            ▼                 ▼                 ▼
//!        generator         execution          protocol
//!            │                 │                 │
//!            └─────────────────┼─────────────────┘
//!                              ▼
//!                         observations
//!                              │
//!              ┌───────────────┴───────────────┐
//!              ▼                               ▼
//!     validation::physical          validation::statistical
//!              │                               │
//!              └───────────────┬───────────────┘
//!                              ▼
//!                          metrics
//!                              │
//!                              ▼
//!                           result
//!                              │
//!                              ▼
//!                 validation::reproducibility
//!                              │
//!                              ▼
//!                         reporting
//! ```
//!
//! # Validation domains
//!
//! ## Input validation
//!
//! [`input`] is the first validation boundary. It validates externally
//! supplied benchmark configuration before the configuration can authorize
//! generation, execution, hardware access, or expensive statistical work.
//!
//! It is responsible for:
//!
//! - benchmark identity;
//! - benchmark configuration;
//! - dimensions;
//! - circuit counts;
//! - shot counts;
//! - aggregate workload limits;
//! - timeout limits;
//! - configured resource ceilings;
//! - execution policy;
//! - backend identifier syntax;
//! - metadata bounds;
//! - finite configuration values;
//! - production resource envelopes.
//!
//! It must remain independent of concrete hardware implementations.
//!
//! ## Statistical validation
//!
//! [`statistical`] validates numerical and statistical data without performing
//! the statistical calculation itself.
//!
//! It is responsible for validating:
//!
//! - finite values;
//! - probabilities;
//! - confidence levels;
//! - sample counts;
//! - binomial counts;
//! - distributions;
//! - weights;
//! - paired observations;
//! - regression inputs and outputs;
//! - residuals;
//! - variance;
//! - standard error;
//! - uncertainty;
//! - confidence bounds;
//! - bootstrap workloads;
//! - hypothesis-test configuration;
//! - numerical bounds.
//!
//! Statistical estimators remain in `statistics::*`.
//!
//! ## Physical validation
//!
//! [`physical`] validates physical-domain invariants without making claims
//! about statistical significance or hardware correctness.
//!
//! It is responsible for validating:
//!
//! - probabilities;
//! - error rates;
//! - fidelities;
//! - counts;
//! - shots;
//! - qubit counts;
//! - depths;
//! - durations;
//! - frequencies;
//! - energies;
//! - readout/stochastic matrices;
//! - density-matrix trace constraints;
//! - leakage/erasure rates;
//! - physical/logical resource relationships;
//! - qubit index bounds;
//! - duplicate qubit operands.
//!
//! It must remain technology-neutral.
//!
//! ## Reproducibility validation
//!
//! [`reproducibility`] validates the identity and metadata required to make a
//! benchmark definition reproducible.
//!
//! It is responsible for validating:
//!
//! - reproducibility schema version;
//! - fingerprint algorithm;
//! - seeds;
//! - configuration fingerprints;
//! - circuit fingerprints;
//! - result fingerprints;
//! - experiment identity;
//! - generator descriptors;
//! - canonical byte bounds;
//! - circuit fingerprint uniqueness;
//! - fingerprint consistency;
//! - reproducibility metadata completeness.
//!
//! It must never generate missing reproducibility information itself.
//!
//! # What this module does NOT do
//!
//! This module must never:
//!
//! - execute quantum circuits;
//! - communicate with QPUs;
//! - access credentials;
//! - select a hardware provider;
//! - generate benchmark circuits;
//! - compile Quantum IR;
//! - perform routing;
//! - perform scheduling;
//! - calculate benchmark metrics;
//! - calculate confidence intervals;
//! - fit statistical models;
//! - calculate p-values;
//! - perform bootstrap resampling;
//! - determine protocol-specific benchmark success;
//! - mutate calibration data;
//! - access process-global mutable state;
//! - access the system clock;
//! - perform filesystem I/O;
//! - perform network I/O;
//! - print diagnostics.
//!
//! Those responsibilities belong to the appropriate benchmarking layers.
//!
//! # Dependency rules
//!
//! The validation facade is intentionally dependency-light.
//!
//! ```text
//! validation
//!     │
//!     ├── core
//!     │
//!     └── standard library / explicitly required serialization support
//! ```
//!
//! Validation MUST NOT depend on:
//!
//! ```text
//! validation
//!     ├──► protocols
//!     ├──► execution implementations
//!     ├──► hardware implementations
//!     ├──► runtime
//!     ├──► frontend
//!     └──► algorithms
//! ```
//!
//! Protocols, metrics, execution, QEC benchmarking, and reporting may depend
//! on validation. The reverse dependency would create architectural cycles.
//!
//! # Quantum IR boundary
//!
//! Benchmark validation must not replace Quantum IR validation.
//!
//! The canonical Quantum IR remains responsible for validating the logical
//! quantum program. The benchmarking layer validates benchmark-specific
//! inputs and observations.
//!
//! Therefore:
//!
//! ```text
//! quantum::ir::validation
//!         │
//!         ▼
//! logical quantum program validity
//!
//! benchmarking::validation
//!         │
//!         ▼
//! benchmark experiment validity
//! ```
//!
//! A benchmark validator may request that a protocol validate a generated
//! circuit through the canonical Quantum IR validation API, but this facade
//! must not duplicate the IR validator.
//!
//! # Validation ordering
//!
//! Production execution should follow this ordering:
//!
//! ```text
//! 1. input validation
//!        │
//!        ▼
//! 2. core configuration validation
//!        │
//!        ▼
//! 3. backend capability validation
//!        │
//!        ▼
//! 4. workload/circuit validation
//!        │
//!        ▼
//! 5. execution
//!        │
//!        ▼
//! 6. physical observation validation
//!        │
//!        ▼
//! 7. statistical observation validation
//!        │
//!        ▼
//! 8. metric/protocol analysis
//!        │
//!        ▼
//! 9. reproducibility/result validation
//!        │
//!        ▼
//! 10. reporting/publication
//! ```
//!
//! This ordering prevents expensive or externally visible work from occurring
//! before untrusted inputs have crossed the appropriate validation boundary.
//!
//! # Fail-closed principle
//!
//! Validation modules are fail-closed by default.
//!
//! Invalid data must not be silently:
//!
//! - clamped;
//! - normalized;
//! - repaired;
//! - discarded;
//! - replaced with defaults;
//! - converted into warnings when an error is required.
//!
//! An explicit caller may choose a less restrictive validation policy where
//! that policy is supported by the individual validator, but production
//! defaults remain strict and bounded.
//!
//! # Resource-safety principle
//!
//! Validation itself is an attack surface.
//!
//! A malformed benchmark request must not be able to force unbounded:
//!
//! - allocation;
//! - iteration;
//! - matrix inspection;
//! - statistical resampling;
//! - fingerprint processing;
//! - dimension expansion;
//! - result processing.
//!
//! Resource bounds therefore remain part of the individual validation
//! contracts rather than being duplicated here.
//!
//! # Stable public API
//!
//! Consumers should normally use one of the following paths:
//!
//! ```text
//! quantum::benchmarking::validation::input
//! quantum::benchmarking::validation::statistical
//! quantum::benchmarking::validation::physical
//! quantum::benchmarking::validation::reproducibility
//! ```
//!
//! Or, for common validator types:
//!
//! ```text
//! quantum::benchmarking::validation::prelude
//! ```
//!
//! Individual modules remain public so that protocol-specific code can select
//! the exact validation domain it requires.
//!
//! # Compatibility contract
//!
//! This module is deliberately designed so that new validation domains can be
//! added later without changing existing validation modules.
//!
//! For example, future additions may include:
//!
//! ```text
//! validation::capability
//! validation::execution
//! validation::result
//! validation::schema
//! validation::security
//! validation::qec
//! ```
//!
//! Such modules must be added as independent validation domains and must not
//! cause the existing validators to become coupled to one another.
//!
//! # Current validation modules
//!
//! The four production validation domains currently established in the
//! benchmarking architecture are:
//!
//! - [`input`]
//! - [`statistical`]
//! - [`physical`]
//! - [`reproducibility`]
//!
//! # Integration with future files
//!
//! The module is intentionally complete with respect to the current four-file
//! validation architecture.
//!
//! Future files should consume these validators rather than modifying this
//! facade merely to add helper logic.
//!
//! Expected integrations:
//!
//! ```text
//! core/config.rs
//!       │
//!       ▼
//! validation::input
//!
//! core/observation.rs
//!       │
//!       ├──────────────► validation::physical
//!       │
//!       └──────────────► validation::statistical
//!
//! core/provenance.rs
//!       │
//!       ▼
//! validation::reproducibility
//!
//! protocols/*
//!       │
//!       ├──────────────► validation::input
//!       ├──────────────► validation::physical
//!       └──────────────► validation::statistical
//!
//! qec/*
//!       │
//!       ├──────────────► validation::physical
//!       └──────────────► validation::statistical
//!
//! reporting/*
//!       │
//!       ▼
//! validation::reproducibility
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
//! This module intentionally contains no unstable APIs.
//!
//! # Testing contract
//!
//! This facade should remain simple enough that the substantive tests belong
//! beside the individual validation implementations.
//!
//! Integration tests for the validation boundary should verify:
//!
//! - all four domains are publicly reachable;
//! - the prelude exports only stable validator APIs;
//! - validation domains remain independent;
//! - no validation module performs execution or I/O;
//! - production constructors are deterministic;
//! - invalid input fails closed.
//!
//! Protocol-specific tests belong under `benchmarking/tests/`.
//!
//! # Security contract
//!
//! This module provides organization and API exposure only. Security-sensitive
//! validation remains in the individual validators.
//!
//! In particular, adding a new benchmark protocol must NOT require weakening
//! this facade or bypassing input validation.
//!
//! # Serialization contract
//!
//! Serialization belongs to the individual validation policies and error
//! types. This facade does not impose a second serialization format.
//!
//! # Versioning
//!
//! Individual validation contracts have their own versions. The facade does
//! not invent a second protocol version for them.
//!
//! If the meaning of a validation domain changes incompatibly, its individual
//! contract version must change there.
//!
//! # No global state
//!
//! This module declares no mutable statics, registries, caches, environment
//! reads, clocks, threads, or global configuration.
//!
//! Validation remains explicitly parameter-driven and deterministic.
//!
//! # Completion criterion
//!
//! This file is complete when:
//!
//! - all current validation modules are declared here;
//! - their public validator APIs are reachable;
//! - no validation implementation is duplicated here;
//! - no downstream protocol is required to modify this file merely to use an
//!   existing validator;
//! - the public prelude remains stable;
//! - Rust 1.97.1 compilation succeeds;
//! - validation-domain dependency direction remains acyclic.
//!
//! This file therefore acts as the permanent validation namespace rather than
//! as another validation algorithm.

#![deny(unsafe_code)]
#![deny(unused_must_use)]

/// Validation of complete benchmark input/configuration.
///
/// This is the first validation boundary for externally supplied benchmark
/// requests.
pub mod input;

/// Validation of statistical-domain values and workloads.
///
/// This module validates statistical inputs and invariants but does not
/// implement statistical estimators.
pub mod statistical;

/// Validation of physical-domain quantities and invariants.
///
/// This module is technology-neutral and does not perform hardware access.
pub mod physical;

/// Validation of reproducibility metadata and fingerprints.
///
/// This module validates reproducibility identity without generating or
/// modifying fingerprints.
pub mod reproducibility;

// =============================================================================
// Stable validator exports
// =============================================================================
//
// The individual modules remain authoritative. These exports provide a stable
// convenience surface for common callers without copying or wrapping their
// implementation logic.

pub use input::{
    InputValidationPolicy,
    InputValidator,
};

pub use statistical::{
    StatisticalValidationPolicy,
    StatisticalValidator,
};

pub use physical::{
    PhysicalValidationError,
    PhysicalValidationPolicy,
    PhysicalValidationResult,
};

pub use reproducibility::{
    ReproducibilityValidationError,
    ReproducibilityValidationPolicy,
};

// =============================================================================
// Stable validation prelude
// =============================================================================

/// Stable collection of the primary benchmark-validation APIs.
///
/// This prelude intentionally exposes validator types and their policy/error
/// contracts rather than every helper function from every validation domain.
///
/// Protocols and services should prefer explicit domain imports when they need
/// specialized validation functionality.
///
/// Example:
///
/// ```ignore
/// use crate::quantum::benchmarking::validation::prelude::{
///     InputValidator,
///     StatisticalValidator,
/// };
///
/// let input_validator = InputValidator::production();
/// let statistical_validator = StatisticalValidator::production();
/// ```
pub mod prelude {
    pub use super::{
        InputValidationPolicy,
        InputValidator,
        PhysicalValidationError,
        PhysicalValidationPolicy,
        PhysicalValidationResult,
        ReproducibilityValidationError,
        ReproducibilityValidationPolicy,
        StatisticalValidationPolicy,
        StatisticalValidator,
    };
}

// =============================================================================
// Compile-time API boundary tests
// =============================================================================
//
// These tests intentionally test only the facade's API surface. Detailed
// validation behavior belongs to the corresponding validation module tests.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_input_validator_is_constructible() {
        let validator = InputValidator::production();
        assert_eq!(
            validator.policy(),
            &InputValidationPolicy::production()
        );
    }

    #[test]
    fn production_statistical_validator_is_constructible() {
        let validator = StatisticalValidator::production();
        assert_eq!(
            validator.policy(),
            &StatisticalValidationPolicy::production()
        );
    }

    #[test]
    fn production_physical_policy_is_constructible() {
        let policy = PhysicalValidationPolicy::default();

        assert!(policy.tolerance.is_finite());
        assert!(policy.max_elements > 0);
        assert!(policy.max_matrix_dimension > 0);
    }

    #[test]
    fn production_reproducibility_policy_is_constructible() {
        let policy = ReproducibilityValidationPolicy::production();

        assert!(policy.max_canonical_bytes > 0);
        assert!(policy.max_circuit_fingerprints > 0);
        assert!(policy.max_identifier_bytes > 0);
        assert!(policy.max_warnings > 0);
    }

    #[test]
    fn production_policies_are_valid() {
        assert!(
            InputValidationPolicy::production()
                .validate()
                .is_ok()
        );

        assert!(
            StatisticalValidationPolicy::production()
                .validate()
                .is_ok()
        );

        assert!(
            ReproducibilityValidationPolicy::production()
                .validate()
                .is_ok()
        );
    }
}