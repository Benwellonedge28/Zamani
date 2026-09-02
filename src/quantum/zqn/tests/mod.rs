//! # Zamani Quantum Noise (ZQN) — Test Composition Root
//!
//! This module is the single composition boundary for the ZQN test suite.
//!
//! ## Purpose
//!
//! The ZQN test suite verifies the complete ZQN subsystem without making the
//! production implementation depend on test code.
//!
//! The suite is intentionally divided by responsibility:
//!
//! ```text
//! tests/
//! ├── fixtures.rs
//! ├── unit/
//! ├── property/
//! ├── differential/
//! ├── determinism/
//! ├── scaling/
//! ├── compatibility/
//! └── integration/
//! ```
//!
//! The composition root only declares and organizes those test namespaces.
//! Mathematical tests, fixtures, integration logic, and individual assertions
//! belong in their respective child modules.
//!
//! ## Ownership
//!
//! This file owns:
//!
//! - test-module composition;
//! - test-suite visibility boundaries;
//! - test-suite-wide safety policy;
//! - test-suite-wide architectural documentation;
//! - the subsystem smoke-test boundary.
//!
//! ## This file does not own
//!
//! This file does not own:
//!
//! - quantum semantics;
//! - quantum IR;
//! - noise models;
//! - channels;
//! - probability mathematics;
//! - fault semantics;
//! - calibration;
//! - characterization;
//! - simulation;
//! - propagation;
//! - target lowering;
//! - serialization;
//! - routing;
//! - scheduling;
//! - QEC;
//! - hardware integration;
//! - runtime implementation.
//!
//! Those responsibilities remain in their production modules or in the
//! dedicated child test namespaces.
//!
//! ## Canonical quantum identities
//!
//! Test fixtures and integration tests must use the canonical quantum resource
//! identities from:
//!
//! `crate::quantum::ir::qubit::{QubitId, PhysicalQubitId}`
//!
//! No ZQN test module may introduce a competing `QubitId` or
//! `PhysicalQubitId` abstraction.
//!
//! ## Scalability contract
//!
//! The test suite MUST NOT encode a semantic maximum number of qubits,
//! operations, faults, resources, shots, channels, or devices.
//!
//! Tests may select finite sizes because a concrete test execution has finite
//! resources, but those values are test inputs rather than architectural
//! limits.
//!
//! Scaling tests therefore receive or generate their resource counts instead
//! of depending on constants such as `MAX_QUBITS`.
//!
//! ## Determinism contract
//!
//! Stochastic tests must use explicit deterministic seeds and execution
//! contexts. No test may depend on a process-global RNG or on execution order
//! for correctness.
//!
//! A deterministic test must produce the same semantic result when executed:
//!
//! - serially;
//! - in a different test-thread scheduling order;
//! - repeatedly with the same seed;
//! - with the same logical input and configuration.
//!
//! Parallel implementations may change performance, but must not change the
//! deterministic semantic result unless the tested contract explicitly allows
//! statistical variation.
//!
//! ## Resource-safety contract
//!
//! Tests must not allocate resources proportional to an artificial global
//! maximum. Large-scale tests must prefer:
//!
//! - iterators;
//! - streaming;
//! - bounded batches;
//! - caller-selected sizes;
//! - explicit resource budgets;
//! - deterministic generated data.
//!
//! A test that intentionally materializes a large structure must make the
//! requested size explicit and must remain bounded by the test environment.
//!
//! ## Safety
//!
//! ZQN tests are required to contain no unsafe Rust.
//!
//! This applies recursively to all child test modules.
//!
//! ## Integration contract
//!
//! Production code MUST NOT depend on this module.
//!
//! The dependency direction is:
//!
//! ```text
//! production ZQN
//!       │
//!       ▼
//! ZQN test composition root
//!       │
//! ┌─────┼─────────┬──────────┬────────────┐
//! ▼     ▼         ▼          ▼            ▼
//! unit property differential determinism scaling ...
//! ```
//!
//! Test modules may consume public or crate-visible production APIs, but
//! production modules must never import anything from `zqn::tests`.
//!
//! ## Integration with the rest of `quantum`
//!
//! The test suite treats `quantum::ir` as the canonical semantic boundary.
//! ZQN tests must therefore verify that ZQN attaches physical-noise semantics
//! to canonical quantum resources rather than replacing the IR with a second
//! semantic representation.
//!
//! In particular:
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      ├──────────────► algorithm / optimization tests
//!      │
//!      ▼
//!     ZQN
//!      │
//!      ├──────────────► routing integration tests
//!      ├──────────────► scheduling integration tests
//!      ├──────────────► QEC integration tests
//!      ├──────────────► hardware integration tests
//!      ├──────────────► memory integration tests
//!      ├──────────────► benchmarking integration tests
//!      └──────────────► runtime integration tests
//! ```
//!
//! ## Test categories
//!
//! ### `fixtures`
//!
//! Reusable deterministic test data and generated resource plans.
//!
//! Fixtures must remain semantically lightweight and must not duplicate
//! production implementations.
//!
//! ### `unit`
//!
//! Tests one production module or abstraction at a time.
//!
//! ### `property`
//!
//! Tests mathematical and semantic invariants across generated inputs.
//!
//! ### `differential`
//!
//! Compares mathematically equivalent representations or implementations,
//! such as equivalent channel representations.
//!
//! ### `determinism`
//!
//! Verifies reproducibility of seeded stochastic behavior.
//!
//! ### `scaling`
//!
//! Verifies that algorithms operate as a function of supplied resource size
//! rather than a hard-coded machine size.
//!
//! ### `compatibility`
//!
//! Verifies schema, version, capability, representation, and approximation
//! compatibility contracts.
//!
//! ### `integration`
//!
//! Verifies boundaries between ZQN and the canonical IR, routing, scheduling,
//! QEC, hardware, memory, benchmarking, and runtime.
//!
//! ## Completion contract
//!
//! This file is considered complete when:
//!
//! 1. every test namespace has a stable ownership boundary;
//! 2. fixtures are available to all child test namespaces;
//! 3. no production module depends on tests;
//! 4. no semantic machine-size limit is introduced;
//! 5. unsafe Rust is forbidden;
//! 6. canonical quantum IR identities remain authoritative;
//! 7. deterministic testing has an explicit namespace;
//! 8. scaling has an explicit namespace;
//! 9. compatibility has an explicit namespace;
//! 10. cross-subsystem integration has an explicit namespace;
//! 11. adding implementations inside an existing test namespace does not
//!     require changing this composition root;
//! 12. adding a new production ZQN implementation does not require modifying
//!     unrelated test namespaces.
//!
//! ## Rust compatibility
//!
//! This module targets Rust 1.97 / 1.97.1 and Rust 2021.
//!
//! It intentionally uses only stable language/module facilities and does not
//! require nightly features.
//!
//! ## Safety policy
//!
//! The entire ZQN test tree is subject to the same rule.
//!
//! No unsafe code is permitted.
//!

#![forbid(unsafe_code)]

// -----------------------------------------------------------------------------
// Shared fixtures
// -----------------------------------------------------------------------------
//
// Fixtures are declared first because every other test category may consume
// them. They must not depend on the other test namespaces.
//
// `fixtures.rs` should remain dependency-light and deterministic so that
// changes to production implementations do not force fixture rewrites.
pub(crate) mod fixtures;

// -----------------------------------------------------------------------------
// Unit tests
// -----------------------------------------------------------------------------
//
// Tests individual ZQN production modules.
//
// Ownership:
// - local invariants;
// - constructor validation;
// - small deterministic examples;
// - module-level error behavior.
//
// Unit tests must not become a second integration framework.
pub(crate) mod unit;

// -----------------------------------------------------------------------------
// Property tests
// -----------------------------------------------------------------------------
//
// Tests mathematical and semantic invariants over generated inputs.
//
// Examples:
// - probabilities remain within their declared domain;
// - distributions satisfy normalization rules;
// - valid channels preserve their required invariants;
// - composition preserves declared properties;
// - canonicalization is stable;
// - generated resource counts do not change semantics.
pub(crate) mod property;

// -----------------------------------------------------------------------------
// Differential tests
// -----------------------------------------------------------------------------
//
// Compares independently implemented or mathematically equivalent
// representations.
//
// Examples:
// - Kraus ↔ Choi;
// - equivalent superoperator representations;
// - exact versus explicitly bounded approximations;
// - independent samplers under the same deterministic context.
//
// Differential tests must compare declared semantics, not implementation
// details such as allocation layout.
pub(crate) mod differential;

// -----------------------------------------------------------------------------
// Determinism tests
// -----------------------------------------------------------------------------
//
// Verifies explicit seeded stochastic execution.
//
// These tests must cover:
// - same seed => same deterministic result;
// - different seed => no accidental seed aliasing;
// - serial/parallel deterministic equivalence where promised;
// - stable derivation of operation/resource/shot sub-seeds;
// - reproducible canonical identities;
// - deterministic serialization/fingerprints where specified.
pub(crate) mod determinism;

// -----------------------------------------------------------------------------
// Scaling tests
// -----------------------------------------------------------------------------
//
// Verifies "write once, scale everywhere" behavior.
//
// These tests must not define a semantic maximum. The resource size is an
// input to the test.
//
// The test implementation should use the smallest practical workload for CI
// and permit larger externally configured workloads for stress testing.
pub(crate) mod scaling;

// -----------------------------------------------------------------------------
// Compatibility tests
// -----------------------------------------------------------------------------
//
// Verifies compatibility contracts independently of the implementation of
// individual noise models.
//
// Coverage includes:
// - ZQN semantic versions;
// - schema versions;
// - serialization compatibility;
// - deserialization compatibility;
// - capability compatibility;
// - exact versus approximate realization;
// - unsupported-feature reporting;
// - forward/backward compatibility rules where supported.
pub(crate) mod compatibility;

// -----------------------------------------------------------------------------
// Integration tests
// -----------------------------------------------------------------------------
//
// Verifies boundaries between ZQN and the rest of Zamani's quantum system.
//
// Expected integration namespaces include:
// - canonical quantum IR;
// - routing;
// - scheduling;
// - QEC;
// - hardware;
// - memory;
// - benchmarking;
// - runtime.
pub(crate) mod integration;

// -----------------------------------------------------------------------------
// Composition-root smoke test
// -----------------------------------------------------------------------------
//
// This deliberately tests only the composition boundary. Detailed behavior
// belongs in the child namespaces above.
//
// Keeping one smoke test here makes it immediately visible if the test
// composition root itself stops being valid.
#[cfg(test)]
mod composition_tests {
    #[test]
    fn zqn_test_composition_root_is_loaded() {
        // The existence of this test proves that the ZQN test composition
        // module was successfully compiled by the test harness.
        //
        // Detailed semantic testing intentionally remains in the dedicated
        // child namespaces.
    }
}