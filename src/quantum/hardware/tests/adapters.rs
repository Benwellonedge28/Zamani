//! Zamani Quantum — Hardware Adapter Integration Tests
//!
//! Production-grade integration tests for:
//!
//! `crate::quantum::hardware::adapters`
//!
//! # Responsibility
//!
//! This module verifies that the adapter namespace is correctly composed and
//! that executable adapters integrate with the provider-neutral
//! `QuantumBackendAdapter` contract.
//!
//! This file is intentionally a TEST-ONLY consumer of the hardware adapter
//! layer.
//!
//! It does NOT:
//!
//! - implement an adapter;
//! - define backend semantics;
//! - define provider semantics;
//! - define the canonical Quantum IR;
//! - implement routing;
//! - implement scheduling;
//! - implement benchmarking;
//! - implement credentials;
//! - authenticate;
//! - perform real-provider network calls;
//! - own provider registries;
//! - modify adapter behavior to make tests pass.
//!
//! # Architectural position
//!
//! ```text
//! Zamani Quantum IR
//!        |
//!        v
//! compatibility
//!        |
//!        v
//! BackendProgram
//!        |
//!        v
//! QuantumBackendAdapter
//!        |
//!        v
//! adapters
//!        |
//!   +----+-------------------------------+
//!   |    |       |       |       |       |
//!   v    v       v       v       v       v
//! local IBM     IonQ   Braket  Rigetti  others
//!   |
//!   v
//! conformance / adapter integration tests
//! ```
//!
//! The test dependency direction is therefore:
//!
//! ```text
//! hardware implementation
//!        |
//!        v
//! tests/adapters.rs
//! ```
//!
//! Never:
//!
//! ```text
//! adapter implementation
//!        |
//!        v
//! tests/adapters.rs
//! ```
//!
//! # Why the local adapter is the reference
//!
//! Remote providers require credentials, network access, provider availability
//! and external service state. They therefore MUST NOT be required by the
//! ordinary unit/integration test suite.
//!
//! The local adapter is the deterministic reference adapter for this file.
//!
//! Provider-specific live tests belong in a separate integration-test layer
//! and must be explicitly enabled by the test environment.
//!
//! # Relationship with conformance.rs
//!
//! `tests/conformance.rs` owns the reusable behavioral conformance suite for
//! `QuantumBackendAdapter`.
//!
//! This file owns:
//!
//! - adapter namespace integrity;
//! - built-in adapter availability;
//! - local reference adapter integration;
//! - invocation of the reusable conformance suite;
//! - provider isolation checks;
//! - interoperability adapter presence;
//! - construction-side-effect checks;
//! - adapter metadata sanity;
//! - cross-adapter API-boundary checks.
//!
//! The reusable behavioral rules MUST NOT be duplicated here.
//!
//! Instead:
//!
//! ```text
//! tests/adapters.rs
//!        |
//!        +----> tests/conformance.rs
//!        |
//!        +----> adapters::local
//!        |
//!        +----> adapters namespace
//! ```
//!
//! # Provider independence
//!
//! The tests intentionally avoid requiring:
//!
//! - IBM credentials;
//! - IonQ credentials;
//! - AWS credentials;
//! - Rigetti credentials;
//! - IQM credentials;
//! - Quantinuum credentials;
//! - QuEra credentials;
//! - network connectivity.
//!
//! Their modules are checked for composition where possible, while actual live
//! provider execution remains an explicitly separate concern.
//!
//! # Security
//!
//! No credentials, tokens, passwords, API keys or private keys may appear in
//! this file.
//!
//! The tests must never read secret environment variables.
//!
//! # Determinism
//!
//! All ordinary tests must be deterministic.
//!
//! The local adapter must not require wall-clock timing, network state or
//! external service availability.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Completion contract
//!
//! This file is complete when:
//!
//! 1. the adapter namespace is present;
//! 2. the local adapter can be constructed;
//! 3. the local adapter implements `QuantumBackendAdapter`;
//! 4. the reusable conformance suite can execute against it;
//! 5. adapter metadata is exposed;
//! 6. the local adapter does not require credentials;
//! 7. construction does not require network access;
//! 8. provider modules remain isolated;
//! 9. interoperability adapters remain distinct from provider adapters;
//! 10. the tests contain no provider secrets;
//! 11. no provider-specific SDK is required merely to compile these tests;
//! 12. Rust 1.97/1.97.1 remains supported.
//!
//! # Important repository integration rule
//!
//! This file must be declared from the hardware test module under `cfg(test)`.
//!
//! It must NOT be exposed as a production runtime API.
//!
//! -----------------------------------------------------------------------------
//! Test implementation
//! -----------------------------------------------------------------------------

#![cfg(test)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::any::TypeId;

use super::adapters;
use super::adapters::local::LocalBackendAdapter;
use super::backend_trait::QuantumBackendAdapter;

/// Stable adapter namespace identifier expected by the adapter composition
/// boundary.
const EXPECTED_ADAPTER_NAMESPACE: &str = "zamani.quantum.hardware.adapters";

/// Stable local adapter family identifier.
///
/// This value is intentionally a test invariant rather than a runtime
/// configuration value.
const EXPECTED_LOCAL_ADAPTER_FAMILY: &str = "local";

/// Executes the complete adapter integration suite.
///
/// This is the preferred single entry point for running all adapter tests
/// against the repository's local reference implementation.
pub fn run_all() {
    adapter_namespace_is_present();
    adapter_schema_is_stable();
    local_adapter_is_constructible();
    local_adapter_is_object_safe();
    local_adapter_has_valid_identity();
    local_adapter_has_valid_backend();
    local_adapter_has_no_provider_dependency();
    local_adapter_is_reference_conformance_target();
    local_adapter_conformance_passes();
    interoperability_adapters_are_distinct();
    provider_modules_are_distinct();
}

/// Verifies the namespace identifier exposed by `adapters`.
#[test]
fn adapter_namespace_is_present() {
    assert_eq!(
        adapters::subsystem_id(),
        EXPECTED_ADAPTER_NAMESPACE,
        "adapter namespace identifier changed unexpectedly"
    );
}

/// Verifies that the adapter composition schema is available and valid.
#[test]
fn adapter_schema_is_stable() {
    let version = adapters::schema_version();

    assert!(
        version > 0,
        "adapter schema version must be greater than zero"
    );

    assert_eq!(
        adapters::ADAPTERS_SCHEMA_ID,
        EXPECTED_ADAPTER_NAMESPACE,
        "adapter schema ID must remain canonical"
    );
}

/// Constructs the local adapter without credentials or network access.
#[test]
fn local_adapter_is_constructible() {
    let adapter = LocalBackendAdapter::default();

    let info = adapter.adapter_info();

    assert!(
        !info.adapter_id.trim().is_empty(),
        "local adapter must expose a non-empty adapter ID"
    );

    assert!(
        !info.adapter_version.trim().is_empty(),
        "local adapter must expose a non-empty adapter version"
    );
}

/// Verifies that the local adapter satisfies the object-safe execution
/// boundary.
///
/// The test intentionally stores the adapter behind the canonical trait
/// object. This catches accidental loss of object safety in the adapter
/// contract.
#[test]
fn local_adapter_is_object_safe() {
    let adapter = LocalBackendAdapter::default();

    let adapter_ref: &dyn QuantumBackendAdapter = &adapter;

    assert!(
        !adapter_ref.adapter_info().adapter_id.trim().is_empty(),
        "QuantumBackendAdapter object must expose adapter identity"
    );
}

/// Verifies local adapter identity invariants.
#[test]
fn local_adapter_has_valid_identity() {
    let adapter = LocalBackendAdapter::default();
    let info = adapter.adapter_info();

    assert!(
        !info.adapter_id.is_empty(),
        "local adapter ID must not be empty"
    );

    assert_eq!(
        info.adapter_id.trim(),
        info.adapter_id,
        "local adapter ID must not contain leading/trailing whitespace"
    );

    assert!(
        !info.adapter_version.is_empty(),
        "local adapter version must not be empty"
    );

    assert_eq!(
        info.adapter_version.trim(),
        info.adapter_version,
        "local adapter version must not contain leading/trailing whitespace"
    );
}

/// Verifies that the local adapter exposes a valid backend descriptor.
#[test]
fn local_adapter_has_valid_backend() {
    let adapter = LocalBackendAdapter::default();
    let backend = adapter.backend();

    let backend_id = backend.id();

    assert!(
        !backend_id.trim().is_empty(),
        "local adapter backend ID must not be empty"
    );

    assert_eq!(
        backend_id.trim(),
        backend_id,
        "local adapter backend ID must not contain surrounding whitespace"
    );
}

/// The local adapter is deliberately provider-neutral.
///
/// This test prevents accidental coupling of the local reference
/// implementation to a remote-provider identity.
#[test]
fn local_adapter_has_no_provider_dependency() {
    let adapter = LocalBackendAdapter::default();
    let info = adapter.adapter_info();

    let adapter_id = info.adapter_id.to_ascii_lowercase();

    assert!(
        adapter_id.contains(EXPECTED_LOCAL_ADAPTER_FAMILY)
            || adapter_id.contains("local"),
        "local adapter identity must identify itself as a local adapter"
    );

    assert!(
        !adapter_id.contains("ibm"),
        "local adapter must not masquerade as IBM"
    );

    assert!(
        !adapter_id.contains("ionq"),
        "local adapter must not masquerade as IonQ"
    );

    assert!(
        !adapter_id.contains("braket"),
        "local adapter must not masquerade as Amazon Braket"
    );

    assert!(
        !adapter_id.contains("rigetti"),
        "local adapter must not masquerade as Rigetti"
    );

    assert!(
        !adapter_id.contains("iqm"),
        "local adapter must not masquerade as IQM"
    );

    assert!(
        !adapter_id.contains("quantinuum"),
        "local adapter must not masquerade as Quantinuum"
    );

    assert!(
        !adapter_id.contains("quera"),
        "local adapter must not masquerade as QuEra"
    );
}

/// Verifies that the local adapter is usable as the canonical conformance
/// target.
///
/// This test deliberately checks the trait boundary rather than concrete
/// methods unique to the local implementation.
#[test]
fn local_adapter_is_reference_conformance_target() {
    let adapter = LocalBackendAdapter::default();

    let adapter_ref: &dyn QuantumBackendAdapter = &adapter;

    assert!(
        !adapter_ref.adapter_info().adapter_id.is_empty(),
        "reference adapter must expose adapter identity"
    );

    assert!(
        !adapter_ref.backend().id().is_empty(),
        "reference adapter must expose backend identity"
    );
}

/// Executes the repository-wide reusable adapter conformance suite against
/// the local reference adapter.
///
/// `conformance.rs` owns the behavioral assertions. Keeping this delegation
/// here prevents this file from developing a second, divergent conformance
/// contract.
#[test]
fn local_adapter_conformance_passes() {
    let adapter = LocalBackendAdapter::default();

    super::conformance::run_all(&adapter);
}

/// OpenQASM and QIR are interoperability boundaries, not provider adapters.
///
/// This test verifies that the adapter namespace keeps those concerns
/// conceptually separate from concrete provider modules.
///
/// The actual translation semantics are tested by their respective modules.
#[test]
fn interoperability_adapters_are_distinct() {
    let openqasm_type = TypeId::of::<adapters::openqasm::OpenQasmAdapter>();
    let qir_type = TypeId::of::<adapters::qir::QirAdapter>();

    assert_ne!(
        openqasm_type, qir_type,
        "OpenQASM and QIR adapters must remain distinct adapter types"
    );
}

/// Provider adapter modules must remain separate implementations.
///
/// This test intentionally uses type identity rather than provider-specific
/// execution because provider execution requires external credentials and
/// service availability.
///
/// The provider adapters must not collapse into a single provider-specific
/// implementation.
#[test]
fn provider_modules_are_distinct() {
    let ibm_type = TypeId::of::<adapters::ibm::IbmBackendAdapter>();
    let ionq_type = TypeId::of::<adapters::ionq::IonqBackendAdapter>();
    let braket_type = TypeId::of::<adapters::aws_braket::AwsBraketBackendAdapter>();
    let rigetti_type = TypeId::of::<adapters::rigetti::RigettiBackendAdapter>();
    let iqm_type = TypeId::of::<adapters::iqm::IqmBackendAdapter>();
    let quantinuum_type = TypeId::of::<adapters::quantinuum::QuantinuumBackendAdapter>();
    let quera_type = TypeId::of::<adapters::quera::QueraBackendAdapter>();

    let provider_types = [
        ibm_type,
        ionq_type,
        braket_type,
        rigetti_type,
        iqm_type,
        quantinuum_type,
        quera_type,
    ];

    for (index, left) in provider_types.iter().enumerate() {
        for (other_index, right) in provider_types.iter().enumerate() {
            if index != other_index {
                assert_ne!(
                    left, right,
                    "provider adapters must remain distinct implementations"
                );
            }
        }
    }
}

/// Compile-time trait-bound assertion for the local reference adapter.
///
/// This is intentionally a generic helper instead of a runtime assertion.
/// If the adapter stops implementing `QuantumBackendAdapter`, compilation
/// fails immediately.
fn assert_adapter_contract<T>()
where
    T: QuantumBackendAdapter,
{
}

/// Compile-time validation that the local adapter implements the canonical
/// execution contract.
#[test]
fn local_adapter_implements_canonical_contract() {
    assert_adapter_contract::<LocalBackendAdapter>();
}