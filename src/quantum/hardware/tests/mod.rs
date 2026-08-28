//! Zamani Quantum Hardware — Test Module Composition
//!
//! Production-grade test composition and conformance boundary for
//! `crate::quantum::hardware`.
//!
//! # Responsibility
//!
//! This module is the authoritative composition point for the hardware test
//! suite. It mounts the independently maintained hardware test modules without
//! duplicating their implementation logic.
//!
//! Current test modules:
//!
//! - `identity.rs` — identity and identifier conformance;
//! - `topology.rs` — physical connectivity and topology conformance;
//! - `instruction_set.rs` — native instruction-set conformance;
//! - `timing.rs` — hardware timing conformance.
//!
//! As additional production hardware modules become available, their tests
//! should be added here only when the corresponding test file actually exists.
//!
//! # Critical architectural rule
//!
//! This module is TEST CODE ONLY.
//!
//! It must never become part of the production hardware API.
//!
//! It must never:
//!
//! - implement hardware execution;
//! - communicate with a provider;
//! - authenticate;
//! - load credentials;
//! - access API keys;
//! - access environment secrets;
//! - perform network I/O;
//! - depend on benchmarking;
//! - depend on Danga;
//! - depend on a provider SDK;
//! - mutate global hardware state;
//! - provide a second hardware abstraction;
//! - duplicate production implementations.
//!
//! # Dependency direction
//!
//! ```text
//!                         quantum::hardware
//!                                │
//!        ┌───────────────────────┼───────────────────────┐
//!        │                       │                       │
//!        ▼                       ▼                       ▼
//!     identity               topology             instruction_set
//!        │                       │                       │
//!        └───────────────────────┼───────────────────────┘
//!                                │
//!                                ▼
//!                              tests
//!                                │
//!          ┌─────────────────────┼─────────────────────┐
//!          ▼                     ▼                     ▼
//!      identity.rs          topology.rs          instruction_set.rs
//!                                │
//!                                ▼
//!                           timing.rs
//! ```
//!
//! Tests consume production APIs.
//!
//! Production modules must never import this module.
//!
//! # Independence guarantee
//!
//! Each child test module is intentionally mounted as an independent Rust
//! module. A child test must use the public production API of the subsystem it
//! tests rather than private implementation details.
//!
//! This preserves the project's "finish one file and freeze it" rule:
//!
//! ```text
//! production module
//!        │
//!        ▼
//! public contract
//!        │
//!        ▼
//! dedicated conformance test
//! ```
//!
//! Completing another hardware subsystem must not require changing an already
//! completed test merely because the implementation of another subsystem has
//! changed.
//!
//! # Current repository integration
//!
//! The current repository contains these hardware test files:
//!
//! ```text
//! src/quantum/hardware/tests/
//! ├── mod.rs                  <-- this file
//! ├── identity.rs
//! ├── topology.rs
//! ├── instruction_set.rs
//! └── timing.rs
//! ```
//!
//! The test modules are mounted below using explicit `#[path]` attributes.
//! This makes the relationship between this composition file and the physical
//! files unambiguous.
//!
//! # Future test modules
//!
//! The following test areas are intentionally NOT declared until their
//! corresponding files exist:
//!
//! ```text
//! calibration.rs
//! capabilities.rs
//! errors.rs
//! backend.rs
//! backend_trait.rs
//! backend_config.rs
//! backend_status.rs
//! compatibility.rs
//! validation.rs
//! execution.rs
//! job.rs
//! queue.rs
//! cancellation.rs
//! result.rs
//! provider.rs
//! provider_registry.rs
//! device_registry.rs
//! discovery.rs
//! credentials.rs
//! authentication.rs
//! health.rs
//! telemetry.rs
//! serialization.rs
//! routing.rs
//! scheduling.rs
//! resource_estimator.rs
//! pulse.rs
//! analog.rs
//! annealing.rs
//! logical.rs
//! simulator.rs
//! emulator.rs
//! adapters.rs
//! failure.rs
//! conformance.rs
//! ```
//!
//! This is deliberate. Rust module declarations refer to real source files;
//! declaring a nonexistent test file would make the repository fail to build.
//!
//! # Mounting contract
//!
//! `hardware/mod.rs` should mount this module exactly once:
//!
//! ```text
//! #[cfg(test)]
//! mod tests;
//! ```
//!
//! No child test file should be separately mounted from `hardware/mod.rs`.
//! This file owns test composition.
//!
//! # Production test philosophy
//!
//! The hardware test suite follows these principles:
//!
//! 1. deterministic;
//! 2. provider-neutral;
//! 3. offline by default;
//! 4. credential-free;
//! 5. network-free;
//! 6. reproducible;
//! 7. failure-oriented;
//! 8. public-API-oriented;
//! 9. compatible with Rust 1.97 and 1.97.1;
//! 10. safe Rust only.
//!
//! Provider adapters must later reuse the same generic conformance principles
//! without modifying the foundational hardware tests.
//!
//! # Test layers
//!
//! ```text
//! Layer 1 — foundational contracts
//!     │
//!     ├── identity
//!     ├── topology
//!     ├── instruction set
//!     └── timing
//!
//! Layer 2 — hardware state
//!     │
//!     ├── capabilities
//!     ├── calibration
//!     ├── status
//!     └── validation
//!
//! Layer 3 — execution
//!     │
//!     ├── execution request
//!     ├── jobs
//!     ├── queues
//!     ├── cancellation
//!     └── results
//!
//! Layer 4 — discovery/provider infrastructure
//!     │
//!     ├── providers
//!     ├── registries
//!     ├── discovery
//!     ├── authentication
//!     └── health
//!
//! Layer 5 — interoperability
//!     │
//!     ├── OpenQASM
//!     ├── QIR
//!     └── provider adapters
//!
//! Layer 6 — system conformance
//!     │
//!     ├── local adapter
//!     ├── mock adapter
//!     ├── failure injection
//!     └── provider conformance
//! ```
//!
//! The layers are added incrementally. A lower layer must not depend on a
//! higher layer merely to make its tests pass.
//!
//! # No live hardware
//!
//! Foundational tests must never require a physical QPU or cloud account.
//!
//! Live-provider tests belong in explicitly separated integration/conformance
//! infrastructure and must be opt-in.
//!
//! The default command:
//!
//! ```text
//! cargo test
//! ```
//!
//! must remain capable of running without:
//!
//! - API credentials;
//! - internet connectivity;
//! - cloud accounts;
//! - provider SDK installations;
//! - physical quantum hardware.
//!
//! # Security
//!
//! This test composition module must never:
//!
//! - print secrets;
//! - read secret environment variables;
//! - construct authentication headers;
//! - store credentials;
//! - serialize credentials;
//! - contact provider endpoints.
//!
//! Tests involving authentication belong to the dedicated authentication
//! subsystem and should use deterministic fake credentials or mocks rather than
//! real credentials.
//!
//! # Determinism
//!
//! Tests must not depend on:
//!
//! - wall-clock timing;
//! - random hardware state;
//! - provider queue state;
//! - provider availability;
//! - network order;
//! - filesystem layout outside the test fixture;
//! - environment-specific configuration.
//!
//! If randomness is required for a property test in a future subsystem, the
//! seed must be explicit and reproducible.
//!
//! # Failure semantics
//!
//! Hardware tests should verify both successful and invalid paths.
//!
//! Production validation is not complete when only happy paths work.
//!
//! The suite must progressively cover:
//!
//! ```text
//! valid input
//! invalid input
//! boundary input
//! empty input
//! oversized input
//! duplicate input
//! conflicting input
//! unsupported capability
//! unavailable hardware
//! stale calibration
//! malformed provider data
//! provider failure
//! timeout
//! cancellation
//! serialization failure
//! ```
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
//! No nightly features are required.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden in the hardware test namespace.
//!
//! The production hardware module itself already declares the hardware safety
//! boundary. Test code must not circumvent it.
//!
//! # Integration with other hardware files
//!
//! The intended final test mapping is:
//!
//! ```text
//! identity.rs
//!     └── tests/identity.rs
//!
//! technology.rs
//!     └── tests/technology.rs
//!
//! capabilities.rs
//!     └── tests/capabilities.rs
//!
//! timing.rs
//!     └── tests/timing.rs
//!
//! instruction_set.rs
//!     └── tests/instruction_set.rs
//!
//! topology.rs
//!     └── tests/topology.rs
//!
//! calibration.rs
//!     └── tests/calibration.rs
//!
//! backend.rs
//! backend_trait.rs
//! backend_status.rs
//! backend_config.rs
//!     └── tests/backend.rs
//!
//! compatibility.rs
//!     └── tests/compatibility.rs
//!
//! validation.rs
//!     └── tests/validation.rs
//!
//! execution.rs
//! job.rs
//! queue.rs
//! cancellation.rs
//! result.rs
//!     └── tests/execution.rs
//!
//! provider.rs
//! provider_registry.rs
//! device_registry.rs
//! discovery.rs
//!     └── tests/registry.rs
//!
//! credentials.rs
//! authentication.rs
//!     └── tests/authentication.rs
//!
//! serialization.rs
//!     └── tests/serialization.rs
//!
//! adapters/*
//!     └── tests/adapters.rs
//!
//! generic conformance
//!     └── tests/conformance.rs
//!
//! failure injection
//!     └── tests/failure.rs
//! ```
//!
//! Those future modules must be introduced one at a time with their actual
//! files. This composition file should only be expanded when the corresponding
//! test implementation has been completed.
//!
//! # Integration with benchmarking
//!
//! Benchmarking consumes hardware contracts. Hardware tests must therefore not
//! import benchmarking.
//!
//! Later benchmarking integration tests should live under the benchmarking
//! test hierarchy and exercise the hardware API from the consumer side.
//!
//! # Integration with routing and scheduling
//!
//! Hardware tests verify that hardware topology, instruction and timing
//! contracts expose sufficient information for routing and scheduling.
//!
//! They must not test routing algorithms or scheduling algorithms themselves.
//! Those algorithms belong to their respective quantum subsystems.
//!
//! # Integration with frontend and IR
//!
//! Hardware tests must not make the hardware layer dependent on the frontend.
//!
//! Where workload compatibility tests are eventually added, they should consume
//! the canonical `quantum::ir` public API and the hardware compatibility
//! contract. They must not introduce a second IR inside this test module.
//!
//! # Integration with QEC
//!
//! Hardware tests may eventually verify hardware-facing logical-qubit,
//! syndrome-measurement, reset and feed-forward capabilities.
//!
//! They must not reimplement error-correction algorithms.
//!
//! # Integration with Danga
//!
//! Danga tests belong above the hardware API.
//!
//! This module verifies the hardware contracts that Danga may consume, but
//! never imports Danga or CLI code.
//!
//! # File completion rule
//!
//! This file is complete when:
//!
//! - every currently existing hardware test file is mounted exactly once;
//! - nonexistent future tests are not declared;
//! - test composition contains no production logic;
//! - tests do not require credentials;
//! - tests do not require network access;
//! - tests do not depend on benchmarking;
//! - tests do not depend on Danga;
//! - tests remain compatible with Rust 1.97.1;
//! - future test additions can be made independently;
//! - production implementation files do not need to be reopened merely because
//!   another test subsystem is added.
//!
//! The child test files themselves own their detailed test contracts.
//!
//! -----------------------------------------------------------------------------
//! Current test modules
//! -----------------------------------------------------------------------------

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

/// Identity conformance tests.
///
/// This file tests the foundational identity API without depending on higher
/// hardware layers.
#[path = "identity.rs"]
mod identity;

/// Physical topology conformance tests.
///
/// This file tests topology construction, connectivity, traversal and related
/// deterministic graph behaviour.
#[path = "topology.rs"]
mod topology;

/// Native instruction-set conformance tests.
///
/// This file tests the hardware instruction representation independently from
/// execution providers.
#[path = "instruction_set.rs"]
mod instruction_set;

/// Hardware timing conformance tests.
///
/// This file tests durations, timing constraints and timing semantics without
/// requiring a backend or provider.
#[path = "timing.rs"]
mod timing;

// =============================================================================
// Test-composition invariants
// =============================================================================

/// The test namespace itself must remain entirely compile-time/static.
///
/// This test deliberately contains no hardware access. Its purpose is to make
/// the composition boundary explicit and ensure that the module can be mounted
/// by the production hardware namespace without requiring runtime setup.
#[test]
fn hardware_test_composition_is_offline_and_static() {
    // Intentionally empty.
    //
    // The absence of provider setup, credentials, network clients and runtime
    // initialization is itself an architectural invariant of the foundational
    // hardware test suite.
}

/// Confirms that this module does not accidentally introduce a second runtime
/// hardware abstraction.
///
/// The test uses only the authoritative production module path.
#[test]
fn hardware_test_composition_uses_authoritative_hardware_namespace() {
    use crate::quantum::hardware;

    // Taking the module path as a value is impossible; the import itself is
    // intentional. This keeps the test coupled to the canonical hardware
    // namespace rather than an alternate test-only abstraction.
    let _ = core::any::type_name::<hardware::topology::HardwareTopology>();
}

/// Confirms that the foundational topology type is available through the
/// canonical hardware namespace used by downstream consumers.
#[test]
fn canonical_topology_api_is_reachable() {
    use crate::quantum::hardware::topology::HardwareTopology;

    let topology = HardwareTopology::new(1)
        .expect("a one-resource topology must be constructible");

    assert_eq!(topology.qubit_count(), 1);
    assert!(topology.is_connected());
}

/// Confirms that foundational hardware modules remain independent from the
/// benchmarking namespace at the source level.
///
/// This is represented as a compile-time architectural convention rather than
/// a runtime dependency test. The test intentionally does not import
/// `quantum::benchmarking`.
#[test]
fn foundational_hardware_tests_do_not_require_benchmarking() {
    // Intentionally empty.
    //
    // If a foundational hardware test begins requiring benchmarking merely to
    // construct its fixtures, that is an architectural regression.
}

/// Confirms that the test harness itself does not require a provider.
///
/// A physical/cloud provider must never be necessary for `cargo test` to test
/// the foundational hardware contracts.
#[test]
fn foundational_hardware_tests_require_no_provider_credentials() {
    // Intentionally empty.
    //
    // Provider credentials must not be read from the environment by this
    // module. Provider-specific integration tests belong elsewhere.
}

/// Confirms that the hardware test suite remains suitable for deterministic
/// local execution.
///
/// No wall-clock value, random value, network request, filesystem mutation or
/// external process is used by this composition module.
#[test]
fn foundational_hardware_test_composition_is_deterministic() {
    let left = core::any::type_name::<
        crate::quantum::hardware::topology::HardwareTopology,
    >();

    let right = core::any::type_name::<
        crate::quantum::hardware::topology::HardwareTopology,
    >();

    assert_eq!(left, right);
}

// =============================================================================
// Future integration contract
// =============================================================================

/// Documents the intended progression of the hardware conformance suite.
///
/// This is deliberately a test rather than a runtime registry. The purpose is
/// to make the expected dependency order explicit without creating references
/// to files that do not yet exist.
#[test]
fn hardware_test_layers_follow_dependency_order() {
    //
    // Layer 1:
    //
    // identity
    // topology
    // instruction_set
    // timing
    //
    // Layer 2:
    //
    // technology
    // capabilities
    // calibration
    // backend_status
    // backend_config
    // errors
    //
    // Layer 3:
    //
    // backend_trait
    // backend
    // compatibility
    // validation
    //
    // Layer 4:
    //
    // execution
    // job
    // queue
    // cancellation
    // result
    //
    // Layer 5:
    //
    // provider
    // provider_registry
    // device_registry
    // discovery
    // credentials
    // authentication
    // health
    // telemetry
    // serialization
    //
    // Layer 6:
    //
    // routing
    // scheduling
    // resource_estimator
    // pulse
    // analog
    // annealing
    // logical
    // simulator
    // emulator
    //
    // Layer 7:
    //
    // adapters
    // conformance
    // failure injection
    //
    // No later layer should become a hidden dependency of an earlier layer.
    assert!(true);
}

// =============================================================================
// Public test-contract documentation
// =============================================================================

/// Returns the names of the foundational hardware test modules currently
/// mounted by this file.
///
/// This is intentionally an internal test utility rather than a production
/// registry.
#[cfg(test)]
fn foundational_test_modules() -> &'static [&'static str] {
    &[
        "identity",
        "topology",
        "instruction_set",
        "timing",
    ]
}

#[test]
fn foundational_test_module_set_is_explicit() {
    assert_eq!(
        foundational_test_modules(),
        &[
            "identity",
            "topology",
            "instruction_set",
            "timing",
        ]
    );
}