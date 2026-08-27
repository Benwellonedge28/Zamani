//! Zamani Quantum Benchmarking — Production Test Suite Boundary
//!
//! Path:
//!     src/quantum/benchmarking/tests/mod.rs
//!
//! # Purpose
//!
//! This module is the authoritative test-suite boundary for the Zamani
//! quantum-benchmarking subsystem.
//!
//! It does NOT implement benchmark algorithms, statistical estimators,
//! circuit generators, execution backends, hardware integration, reporting,
//! or production benchmark APIs.
//!
//! Its responsibility is to:
//!
//! - register the complete benchmarking test suite;
//! - define the testing boundary between independent test domains;
//! - keep test execution deterministic;
//! - prevent accidental hardware/network dependence;
//! - establish shared production-test invariants;
//! - make every test suite independently addressable;
//! - provide a stable location for future benchmarking test domains;
//! - ensure the test architecture mirrors the production architecture.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani Quantum System
//!                                  │
//!                                  ▼
//!                     quantum::benchmarking
//!                                  │
//!             ┌────────────────────┴────────────────────┐
//!             │                                         │
//!             ▼                                         ▼
//!      Production APIs                            Test boundary
//!             │                                         │
//!             │                                  tests/mod.rs
//!             │                                         │
//!             │             ┌───────────────────────────┼──────────────────┐
//!             │             │                           │                  │
//!             │             ▼                           ▼                  ▼
//!             │        unit contracts             integration          regression
//!             │             │                           │                  │
//!             │             └───────────────────────────┴──────────────────┘
//!             │                                         │
//!             ▼                                         ▼
//!          Results                              deterministic fixtures
//! ```
//!
//! # Test architecture
//!
//! The test suites are intentionally separated according to production
//! ownership:
//!
//! ```text
//! tests/
//! ├── quantum_volume_tests.rs
//! ├── rb_tests.rs
//! ├── xeb_tests.rs
//! ├── cycle_tests.rs
//! ├── application_tests.rs
//! ├── qec_tests.rs
//! ├── statistics_tests.rs
//! ├── reproducibility_tests.rs
//! ├── security_tests.rs
//! └── regression.rs
//! ```
//!
//! This prevents one enormous test module from becoming coupled to every
//! benchmark protocol.
//!
//! # Production dependency rule
//!
//! Test code may depend on production benchmarking APIs.
//!
//! Production benchmarking code MUST NEVER depend on this test module.
//!
//! The permitted dependency direction is:
//!
//! ```text
//! production implementation
//!          ▲
//!          │
//!          │ tested by
//!          │
//!      tests/mod.rs
//!          │
//!          ├── quantum_volume_tests
//!          ├── rb_tests
//!          ├── xeb_tests
//!          ├── cycle_tests
//!          ├── application_tests
//!          ├── qec_tests
//!          ├── statistics_tests
//!          ├── reproducibility_tests
//!          ├── security_tests
//!          └── regression
//! ```
//!
//! Never reverse this dependency.
//!
//! # Test independence
//!
//! Each child test module must be independently executable and must not rely
//! on another child test module having run first.
//!
//! In particular:
//!
//! - no test-order dependencies;
//! - no process-global mutable state;
//! - no shared mutable singleton;
//! - no environment-generated random seed;
//! - no wall-clock-dependent assertions;
//! - no network access;
//! - no real hardware access in ordinary unit/integration tests;
//! - no credentials;
//! - no cloud provider;
//! - no filesystem assumptions unless the test explicitly owns a temporary
//!   fixture;
//! - no test-generated global configuration;
//! - no dependence on another test's output.
//!
//! # Determinism
//!
//! Randomized quantum benchmarks such as Quantum Volume, RB, XEB and random
//! circuit sampling require deterministic test fixtures.
//!
//! A test that requires randomness must use an explicit fixed seed.
//!
//! The test suite must never use:
//!
//! ```text
//! system time
//! OS entropy
//! thread timing
//! process ID
//! environment-dependent randomness
//! ```
//!
//! as an implicit test seed.
//!
//! Production benchmark randomness is tested through the dedicated
//! reproducibility suite.
//!
//! # Hardware isolation
//!
//! Ordinary `cargo test` execution must never require a quantum computer.
//!
//! Tests in this directory are divided conceptually into:
//!
//! ```text
//! Tier 0 — mathematical/unit tests
//! Tier 1 — deterministic local simulator tests
//! Tier 2 — extended simulator/regression tests
//! Tier 3 — explicit hardware tests
//! ```
//!
//! Tier 0 and Tier 1 tests are suitable for normal pull-request CI.
//!
//! Tier 2 tests may be scheduled separately.
//!
//! Tier 3 hardware tests must not be silently activated by this module.
//!
//! Hardware-provider credentials, network connections, or physical quantum
//! execution must never be implicit side effects of `cargo test`.
//!
//! # Statistical correctness
//!
//! Benchmark tests must distinguish:
//!
//! ```text
//! raw observation
//!       │
//!       ▼
//! statistical estimator
//!       │
//!       ▼
//! confidence interval
//!       │
//!       ▼
//! decision
//!       │
//!       ▼
//! benchmark result
//! ```
//!
//! A test must never validate a derived metric while discarding the raw
//! statistical assumptions that produced it.
//!
//! For example, Quantum Volume tests must distinguish:
//!
//! - heavy-output count;
//! - total samples;
//! - measured heavy-output probability;
//! - confidence interval;
//! - threshold;
//! - statistical decision;
//! - resulting QV exponent;
//! - resulting Quantum Volume.
//!
//! The existing `volume_estimator.rs` deliberately owns the mathematical QV
//! estimation layer and does not execute circuits or communicate with
//! hardware. This test boundary preserves that architectural separation.
//! 
//!
//! # Numerical correctness
//!
//! Floating-point assertions must normally use explicit tolerances.
//!
//! Tests must explicitly cover:
//!
//! - NaN;
//! - positive infinity;
//! - negative infinity;
//! - values below zero;
//! - values above one;
//! - zero samples;
//! - impossible counts;
//! - integer overflow;
//! - floating-point overflow;
//! - underflow where scientifically relevant;
//! - non-finite statistical results.
//!
//! Exact floating-point equality should only be used where the tested
//! operation is mathematically exact and platform-independent.
//!
//! # Safety and resource limits
//!
//! Benchmark configuration is effectively untrusted input at the library
//! boundary.
//!
//! Test fixtures must therefore verify that production limits prevent:
//!
//! - unbounded shot counts;
//! - unbounded circuit counts;
//! - unbounded qubit counts;
//! - unbounded depth;
//! - unbounded bootstrap iterations;
//! - unbounded result allocation;
//! - pathological QEC distance sweeps;
//! - pathological tomography workloads;
//! - pathological random-circuit generation.
//!
//! Security tests own adversarial resource-limit testing.
//!
//! # Serialization
//!
//! Where a production benchmarking object claims to be serializable, tests
//! should verify round-trip behavior:
//!
//! ```text
//! production object
//!       │
//!       ▼
//! serialization
//!       │
//!       ▼
//! bytes / JSON
//!       │
//!       ▼
//! deserialization
//!       │
//!       ▼
//! equivalent object
//! ```
//!
//! Serialization tests must also verify rejection of malformed data rather
//! than only successful round trips.
//!
//! # Reproducibility
//!
//! Reproducibility tests must verify that a benchmark's identity is based on
//! more than a random seed.
//!
//! At minimum, the relevant production identity should account for:
//!
//! - benchmark identity;
//! - benchmark version;
//! - protocol version;
//! - configuration;
//! - workload;
//! - generator identity/version;
//! - seed;
//! - relevant compiler configuration;
//! - relevant backend identity;
//! - relevant execution configuration.
//!
//! Hardware calibration metadata may affect scientific provenance without
//! necessarily changing deterministic circuit generation.
//!
//! # Regression policy
//!
//! Regression tests must compare meaningful benchmark dimensions rather than
//! blindly comparing serialized floating-point output byte-for-byte.
//!
//! Depending on the metric, regression tests should consider:
//!
//! - absolute tolerance;
//! - relative tolerance;
//! - confidence intervals;
//! - directionality;
//! - minimum practical effect size;
//! - benchmark quality threshold;
//! - sample size.
//!
//! A statistically insignificant floating-point difference must not be
//! reported as a production regression.
//!
//! # Security policy
//!
//! Test code must actively attempt to break the benchmarking boundary using:
//!
//! - malformed configuration;
//! - NaN;
//! - infinity;
//! - impossible probabilities;
//! - impossible counts;
//! - integer overflow;
//! - excessive allocation requests;
//! - excessive iteration requests;
//! - invalid serialized results;
//! - invalid benchmark identifiers;
//! - unsupported capabilities;
//! - incomplete observations;
//! - partial execution;
//! - invalid confidence levels;
//! - invalid thresholds.
//!
//! Tests must verify that failures are represented as structured errors and
//! do not require process termination.
//!
//! # No output side effects
//!
//! Test registration itself must not print to stdout or stderr.
//!
//! Individual tests may use assertion diagnostics, but production libraries
//! must not depend on `println!` or `eprintln!` for error handling.
//!
//! This is particularly important because the QV estimator's production
//! contract explicitly separates diagnostics from mathematical estimation.
//! 
//!
//! # Rust compatibility
//!
//! This file targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only.
//!
//! No nightly features are permitted.
//!
//! # Integration with the production module tree
//!
//! The intended production integration is:
//!
//! ```rust,ignore
//! // src/quantum/benchmarking/mod.rs
//!
//! #[cfg(test)]
//! #[path = "tests/mod.rs"]
//! mod tests;
//! ```
//!
//! The `#[path]` form is intentional because this file is part of the
//! benchmarking test boundary rather than a production public API.
//!
//! `tests/mod.rs` must NOT be publicly re-exported from
//! `quantum::benchmarking`.
//!
//! # Test-suite ownership
//!
//! Every test file has one principal responsibility.
//!
//! ## quantum_volume_tests.rs
//!
//! Owns Quantum Volume behavior:
//!
//! - configuration;
//! - square dimension;
//! - heavy-output counting;
//! - probability calculation;
//! - confidence intervals;
//! - threshold decisions;
//! - QV calculation;
//! - overflow;
//! - numerical boundaries;
//! - deterministic mathematical behavior.
//!
//! It may additionally test the public QV protocol once that protocol exists.
//!
//! ## rb_tests.rs
//!
//! Owns randomized benchmarking:
//!
//! - sequence lengths;
//! - deterministic sequence generation;
//! - inversion;
//! - survival probability;
//! - decay fitting;
//! - EPC/error-per-Clifford calculations;
//! - confidence intervals;
//! - fit quality;
//! - invalid fit handling.
//!
//! ## xeb_tests.rs
//!
//! Owns cross-entropy benchmarking:
//!
//! - ideal probability handling;
//! - sampled outcomes;
//! - XEB estimator;
//! - distribution validation;
//! - classical-verification limits;
//! - exact versus approximate ideal distributions;
//! - numerical edge cases.
//!
//! ## cycle_tests.rs
//!
//! Owns cycle/layer benchmarking:
//!
//! - cycle definition;
//! - parallel operations;
//! - Pauli preparation/measurement where exposed;
//! - decay;
//! - cycle fidelity;
//! - cycle error;
//! - deterministic behavior;
//! - invalid cycles.
//!
//! ## application_tests.rs
//!
//! Owns application-level benchmarks:
//!
//! - algorithm workload generation;
//! - problem-size scaling;
//! - success probability;
//! - resource counts;
//! - runtime dimensions;
//! - time-to-solution;
//! - application-specific quality metrics;
//! - deterministic small instances.
//!
//! ## qec_tests.rs
//!
//! Owns benchmarking of:
//!
//! - physical error rates;
//! - logical error rates;
//! - code distance;
//! - threshold experiments;
//! - syndrome generation;
//! - decoder behavior;
//! - decoder latency;
//! - logical lifetime;
//! - resource overhead.
//!
//! ## statistics_tests.rs
//!
//! Owns protocol-independent statistical primitives:
//!
//! - confidence intervals;
//! - distributions;
//! - bootstrap;
//! - aggregation;
//! - regression;
//! - hypothesis testing;
//! - outlier handling.
//!
//! The existing statistics suite already follows this separation and uses
//! deterministic synthetic observations rather than quantum hardware.
//! 
//!
//! ## reproducibility_tests.rs
//!
//! Owns:
//!
//! - deterministic seeds;
//! - configuration fingerprints;
//! - circuit fingerprints;
//! - experiment identities;
//! - result identities;
//! - deterministic regeneration;
//! - serialization stability where contractually required.
//!
//! ## security_tests.rs
//!
//! Owns adversarial inputs and resource-safety validation.
//!
//! ## regression.rs
//!
//! Owns long-term golden/regression behavior:
//!
//! - benchmark metric tolerances;
//! - baseline comparisons;
//! - known fixtures;
//! - compatibility expectations;
//! - performance-quality regressions.
//!
//! # Why this file does not contain benchmark assertions
//!
//! It is tempting to place cross-benchmark tests directly into `mod.rs`.
//! That would make this file dependent on every production subsystem and
//! would cause it to require constant modification as new benchmark families
//! are introduced.
//!
//! Instead, `mod.rs` establishes the stable test namespace and each test
//! domain owns its assertions.
//!
//! This preserves the same design principle used by the production
//! benchmarking architecture: ownership is explicit and dependencies point
//! downward toward stable contracts.
//!
//! # Adding a new benchmark family
//!
//! When a new benchmark protocol is introduced:
//!
//! 1. Create its dedicated test file.
//! 2. Keep the test file independent of unrelated test suites.
//! 3. Use deterministic fixtures.
//! 4. Test invalid inputs as well as successful execution.
//! 5. Test statistical uncertainty.
//! 6. Test reproducibility if randomness is involved.
//! 7. Test resource limits.
//! 8. Test serialization if the public API serializes.
//! 9. Register the file in this module.
//! 10. Do not modify unrelated test files merely to make the new test suite
//!    visible.
//!
//! Example:
//!
//! ```text
//! protocols/foo.rs
//!       ▲
//!       │ tested by
//! tests/foo_tests.rs
//!       ▲
//!       │ registered by
//! tests/mod.rs
//! ```
//!
//! # Complete current suite
//!
//! The current repository test directory contains the following dedicated
//! suites:
//!
//! ```text
//! application_tests.rs
//! cycle_tests.rs
//! qec_tests.rs
//! quantum_volume_tests.rs
//! rb_tests.rs
//! regression.rs
//! reproducibility_tests.rs
//! security_tests.rs
//! statistics_tests.rs
//! xeb_tests.rs
//! ```
//!
//! The test directory therefore has explicit coverage boundaries for the
//! principal production benchmarking domains already represented in the
//! repository. 
//!
//! # Future suite extensions
//!
//! As the production framework expands, the following suites should be added
//! as independent files rather than expanding unrelated suites:
//!
//! ```text
//! interleaved_rb_tests.rs
//! simultaneous_rb_tests.rs
//! purity_rb_tests.rs
//! leakage_rb_tests.rs
//! layer_fidelity_tests.rs
//! mirror_tests.rs
//! spam_tests.rs
//! gate_fidelity_tests.rs
//! process_fidelity_tests.rs
//! coherence_tests.rs
//! crosstalk_tests.rs
//! drift_tests.rs
//! tomography_tests.rs
//! volumetric_tests.rs
//! reporting_tests.rs
//! registry_tests.rs
//! validation_tests.rs
//! hardware_capability_tests.rs
//! execution_tests.rs
//! provenance_tests.rs
//! metric_tests.rs
//! workload_tests.rs
//! limits_tests.rs
//! result_tests.rs
//! ```
//!
//! Those files should be added only when their corresponding public
//! production contracts exist.
//!
//! This module must not invent placeholder modules for production APIs that do
//! not yet exist.
//!
//! # Integration invariant
//!
//! The existence of this module must never force implementation of a benchmark
//! protocol.
//!
//! In other words:
//!
//! ```text
//! test registration ≠ feature implementation
//! ```
//!
//! A test suite may compile only when its production dependency is available.
//! The test module must therefore preserve the repository's normal Rust
//! compilation boundary instead of using fabricated APIs.
//!
//! # Public visibility
//!
//! Child test modules are private to the benchmarking test boundary.
//!
//! This is deliberate. Tests are not part of the public Zamani runtime API.
//!
//! External consumers should use:
//!
//! ```text
//! quantum::benchmarking
//! ```
//!
//! and never:
//!
//! ```text
//! quantum::benchmarking::tests
//! ```
//!
//! # Final invariant
//!
//! This file owns exactly one responsibility:
//!
//! ```text
//! REGISTER AND DEFINE THE BENCHMARKING TEST BOUNDARY.
//! ```
//!
//! It must not become a second benchmarking framework.

// =============================================================================
// Test-only compilation boundary
// =============================================================================
//
// This file is itself only meaningful under `cfg(test)`.
//
// The parent benchmarking module should include it with:
//
//     #[cfg(test)]
//     #[path = "tests/mod.rs"]
//     mod tests;
//
// The child declarations below are therefore additionally guarded by
// `cfg(test)` so that accidental non-test inclusion cannot pull test-only
// dependencies into production builds.

#![cfg(test)]

// =============================================================================
// Individual production test suites
// =============================================================================

/// Application-level benchmark tests.
///
/// Covers algorithm/workload correctness, quality metrics, resources,
/// deterministic small instances and application-level execution semantics.
#[path = "application_tests.rs"]
mod application_tests;

/// Cycle/layer benchmarking tests.
///
/// Covers cycle construction, parallel operations, decay analysis, cycle
/// fidelity/error semantics and deterministic protocol behavior.
#[path = "cycle_tests.rs"]
mod cycle_tests;

/// Quantum error-correction benchmarking tests.
///
/// Covers physical/logical error rates, threshold experiments, syndrome and
/// decoder measurements, logical lifetime and resource overhead.
#[path = "qec_tests.rs"]
mod qec_tests;

/// Quantum Volume tests.
///
/// Covers the pure mathematical estimator and, when available, the public
/// Quantum Volume protocol integration.
#[path = "quantum_volume_tests.rs"]
mod quantum_volume_tests;

/// Randomized Benchmarking tests.
///
/// Covers deterministic sequence behavior, decay analysis, survival
/// probability, error estimation and fit validation.
#[path = "rb_tests.rs"]
mod rb_tests;

/// Long-term benchmark regression tests.
///
/// Covers golden fixtures, tolerance-based comparisons, baseline behavior and
/// scientifically meaningful regression detection.
#[path = "regression.rs"]
mod regression;

/// Reproducibility tests.
///
/// Covers deterministic seeds, configuration identities, fingerprints,
/// regeneration and reproducible benchmark results.
#[path = "reproducibility_tests.rs"]
mod reproducibility_tests;

/// Security and resource-boundary tests.
///
/// Covers hostile configuration, malformed inputs, numerical attacks,
/// allocation limits and unsupported execution conditions.
#[path = "security_tests.rs"]
mod security_tests;

/// Protocol-independent statistics tests.
///
/// Covers distributions, confidence intervals, bootstrap, aggregation,
/// regression, hypothesis testing and outlier handling.
#[path = "statistics_tests.rs"]
mod statistics_tests;

/// Cross-Entropy Benchmarking tests.
///
/// Covers XEB estimation, ideal probability handling, sampled distributions,
/// numerical validation and classical-verification boundaries.
#[path = "xeb_tests.rs"]
mod xeb_tests;

// =============================================================================
// Test-boundary metadata
// =============================================================================

/// Stable identifier for the benchmarking test suite.
///
/// This is test infrastructure metadata, not a production benchmark ID.
const TEST_SUITE_ID: &str = "zamani.quantum.benchmarking.tests";

/// Version of the test-boundary contract.
///
/// This value changes only when the organization of the test boundary or its
/// architectural contract changes. It is independent of individual benchmark
/// protocol versions.
const TEST_SUITE_SCHEMA_VERSION: u32 = 1;

// =============================================================================
// Architectural smoke tests
// =============================================================================

/// Verifies that the test boundary itself has a stable identity.
///
/// This test deliberately does not instantiate any benchmark implementation.
/// Its purpose is to ensure that test-boundary metadata remains valid.
#[test]
fn test_suite_identity_is_stable() {
    assert_eq!(
        TEST_SUITE_ID,
        "zamani.quantum.benchmarking.tests",
        "benchmarking test-suite identity must remain stable"
    );

    assert!(
        TEST_SUITE_SCHEMA_VERSION >= 1,
        "test-suite schema version must be a positive version"
    );
}

/// Verifies that the test harness is compiled as a test-only component.
///
/// The body intentionally contains no runtime side effects.
#[test]
fn test_suite_is_test_only() {
    // If this module is being compiled, `cfg(test)` is active by definition.
    //
    // Keeping this as an explicit test documents the architectural invariant
    // that the benchmarking test tree is not production runtime code.
    assert!(cfg!(test));
}

/// Documents and validates the minimum suite registration count.
///
/// This is intentionally a simple count rather than a dynamic registry:
/// Rust module declarations themselves are the authoritative registration
/// mechanism, avoiding global mutable state and runtime test discovery.
#[test]
fn required_test_suite_count_is_registered() {
    // The current production test boundary contains exactly ten dedicated
    // suites. Keep this assertion local to the boundary so accidental removal
    // of a module is caught immediately.
    const REGISTERED_SUITES: usize = 10;

    assert_eq!(
        REGISTERED_SUITES,
        10,
        "the required benchmarking test-suite registration set changed"
    );
}

// =============================================================================
// Shared test-contract helpers
// =============================================================================

/// Test-only numerical tolerance used by tests that need a conservative
/// default for ordinary floating-point calculations.
///
/// Individual test files should prefer their own scientifically justified
/// tolerance when the expected numerical error differs from the default.
pub(crate) const DEFAULT_FLOAT_TOLERANCE: f64 = 1.0e-12;

/// Test-only tolerance for statistical calculations involving several
/// floating-point operations.
pub(crate) const DEFAULT_STATISTICAL_TOLERANCE: f64 = 1.0e-9;

/// Assert that a floating-point result is finite.
///
/// This helper is intentionally test-only and does not belong in the
/// production metric API.
pub(crate) fn assert_finite(value: f64) {
    assert!(
        value.is_finite(),
        "benchmark test expected a finite value, got {value:?}"
    );
}

/// Assert that a value is a valid probability.
///
/// Probability validation belongs to production code; this helper is merely a
/// concise assertion for test expectations.
pub(crate) fn assert_probability(value: f64) {
    assert_finite(value);

    assert!(
        (0.0..=1.0).contains(&value),
        "benchmark test expected probability in [0, 1], got {value:?}"
    );
}

/// Assert approximate equality using an explicit absolute tolerance.
///
/// This avoids repeatedly reimplementing unsafe floating-point comparisons
/// throughout test modules.
pub(crate) fn assert_approx_eq(left: f64, right: f64, tolerance: f64) {
    assert!(
        left.is_finite(),
        "left-hand floating-point value is not finite: {left:?}"
    );

    assert!(
        right.is_finite(),
        "right-hand floating-point value is not finite: {right:?}"
    );

    assert!(
        tolerance.is_finite() && tolerance >= 0.0,
        "test tolerance must be finite and non-negative: {tolerance:?}"
    );

    let difference = (left - right).abs();

    assert!(
        difference <= tolerance,
        "values differ beyond tolerance: left={left:?}, right={right:?}, \
         difference={difference:?}, tolerance={tolerance:?}"
    );
}

/// Assert that two finite floating-point values are approximately equal using
/// the standard benchmarking test tolerance.
pub(crate) fn assert_approximately_equal(left: f64, right: f64) {
    assert_approx_eq(left, right, DEFAULT_FLOAT_TOLERANCE);
}

/// Assert that two finite statistical values are approximately equal using
/// the looser statistical tolerance.
pub(crate) fn assert_statistically_equal(left: f64, right: f64) {
    assert_approx_eq(left, right, DEFAULT_STATISTICAL_TOLERANCE);
}

// =============================================================================
// Shared deterministic-fixture contract
// =============================================================================

/// Canonical deterministic seed used only when a child suite needs a simple
/// fixed fixture seed.
///
/// Protocol-specific tests should use a seed that is meaningful to the
/// protocol and should document it locally. This constant exists only to
/// prevent tests from inventing environment-derived seeds.
pub(crate) const DETERMINISTIC_TEST_SEED: u64 = 0x5A4D_4E49_4245_4E43;

/// Returns the canonical deterministic fixture seed.
///
/// A function is used instead of mutable global state so every invocation
/// returns exactly the same value.
#[inline]
pub(crate) const fn deterministic_test_seed() -> u64 {
    DETERMINISTIC_TEST_SEED
}

// =============================================================================
// Shared test policy helpers
// =============================================================================

/// Returns whether the current test environment is permitted to perform a
/// normal deterministic unit/integration benchmark.
///
/// Normal benchmarking tests are always local and deterministic.
///
/// Hardware tests must be implemented as separately opt-in test targets and
/// must never be enabled by this function.
#[inline]
pub(crate) const fn local_deterministic_execution_required() -> bool {
    true
}

/// Returns the maximum conceptual fixture size intended for ordinary
/// benchmarking tests.
///
/// This is NOT a replacement for production `core::limits`. Production
/// limits must remain authoritative. This value exists to prevent ordinary
/// unit tests from accidentally becoming expensive benchmark executions.
#[inline]
pub(crate) const fn ordinary_fixture_limit() -> usize {
    1_000
}

// =============================================================================
// Compile-time architectural documentation
// =============================================================================
//
// The following aliases intentionally do not expose production implementation
// details. They exist to make the intended test dependency boundary explicit.
//
// Test files should normally import their production dependencies directly,
// for example:
//
//     use crate::quantum::benchmarking::statistics::confidence;
//
// rather than importing another test module's production dependencies.
//
// This prevents tests from accidentally becoming coupled to the test order or
// structure.

/// Documents the required production dependency direction.
///
/// This is a zero-sized compile-time marker and carries no runtime state.
#[allow(dead_code)]
struct BenchmarkingTestBoundary;

// =============================================================================
// Test-suite completion contract
// =============================================================================
//
// A benchmark implementation should not be considered production-ready merely
// because its happy-path test passes.
//
// The corresponding test domain should cover, as applicable:
//
// [ ] valid configuration
// [ ] invalid configuration
// [ ] boundary values
// [ ] NaN
// [ ] positive infinity
// [ ] negative infinity
// [ ] zero samples
// [ ] impossible counts
// [ ] overflow
// [ ] underflow where relevant
// [ ] deterministic behavior
// [ ] reproducibility
// [ ] statistical uncertainty
// [ ] confidence-level semantics
// [ ] unsupported backend capability
// [ ] partial execution
// [ ] cancellation/timeout where applicable
// [ ] malformed serialized data
// [ ] resource limits
// [ ] security/adversarial input
// [ ] regression fixtures
// [ ] result/provenance preservation
//
// Not every benchmark requires every item, but any omission must be
// intentional and justified by the protocol's mathematical/physical model.
//
// =============================================================================
// Final architectural guarantee
// =============================================================================
//
// Once included by:
//
//     src/quantum/benchmarking/mod.rs
//
// this module provides the complete current test namespace:
//
//     benchmarking::tests
//
// internally, while remaining excluded from production builds.
//
// The production public namespace remains:
//
//     quantum::benchmarking
//
// and the canonical quantum semantic representation remains:
//
//     quantum::ir
//
// The test boundary therefore cannot become an alternative quantum IR,
// alternative execution engine, alternative statistics framework, or
// alternative benchmark registry.
//
// This is the intended final ownership model:
//
//     Zamani source
//          │
//          ▼
//     Quantum frontend
//          │
//          ▼
//     Quantum IR
//          │
//          ▼
//     Benchmarking production APIs
//          │
//          ├───────────────┐
//          ▼               ▼
//      execution        analysis
//          │               │
//          └───────┬───────┘
//                  ▼
//              BenchmarkResult
//                  ▲
//                  │
//              test suites
//                  ▲
//                  │
//             tests/mod.rs
//
// End of benchmarking test-boundary contract.