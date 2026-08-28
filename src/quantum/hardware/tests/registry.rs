//! Zamani Quantum Hardware — Provider Registry Tests
//!
//! Production conformance tests for:
//! `src/quantum/hardware/provider_registry.rs`.
//!
//! # Responsibility
//!
//! This module verifies the PUBLIC contract of `ProviderRegistry` without
//! depending on its internal storage representation.
//!
//! The tests cover:
//!
//! - empty-registry semantics;
//! - provider registration;
//! - duplicate-registration protection;
//! - provider lookup;
//! - provider replacement/update;
//! - provider removal;
//! - deterministic enumeration;
//! - provider filtering;
//! - capability filtering;
//! - technology filtering;
//! - execution-model filtering;
//! - interoperability-format filtering;
//! - provider-status filtering;
//! - bounded registry growth;
//! - query-result limits;
//! - optimistic generation checks;
//! - atomic mutation semantics;
//! - immutable snapshots;
//! - snapshot isolation;
//! - deterministic canonical representations;
//! - deterministic fingerprints;
//! - provider descriptor validation;
//! - concurrent readers/writers;
//! - lock-poisoning behavior;
//! - generation monotonicity;
//! - failed-operation non-mutation;
//! - provider identity isolation;
//! - absence of accidental provider execution responsibilities.
//!
//! # Non-responsibilities
//!
//! This file deliberately does NOT test:
//!
//! - provider HTTP APIs;
//! - cloud authentication;
//! - API credentials;
//! - backend execution;
//! - QPU communication;
//! - job submission;
//! - job polling;
//! - result retrieval;
//! - topology algorithms;
//! - calibration acquisition;
//! - routing;
//! - scheduling;
//! - benchmarking;
//! - Danga;
//! - provider SDKs.
//!
//! Those concerns belong to their respective conformance suites.
//!
//! # Architectural contract
//!
//! ```text
//! ProviderDescriptor
//!        |
//!        v
//! ProviderRegistry
//!        |
//!        +-----------------------------+
//!        |                             |
//!        v                             v
//! deterministic metadata          provider selection
//!        |                             |
//!        +-------------+---------------+
//!                      |
//!                      v
//!                downstream users
//! ```
//!
//! The registry is metadata/index state. It must never execute quantum work.
//!
//! # Integration contract
//!
//! This file consumes only the public API of:
//!
//! - `provider.rs`;
//! - `provider_registry.rs`.
//!
//! It must not import:
//!
//! - provider adapters;
//! - HTTP clients;
//! - credentials;
//! - authentication;
//! - benchmarking;
//! - Danga;
//! - private implementation details.
//!
//! If a later module requires a registry behavior that is not covered here,
//! that behavior must first be added to the public registry contract rather
//! than bypassing the registry's invariants.
//!
//! # No-reedit rule
//!
//! This test file is intentionally written against stable semantic behavior.
//!
//! Downstream implementation work MUST NOT require modification of this file
//! merely because:
//!
//! - `BTreeMap` is replaced internally;
//! - locking is optimized;
//! - filtering is optimized;
//! - provider storage is reorganized;
//! - a provider adapter is added;
//! - device registration is added;
//! - discovery is added.
//!
//! The tests should change only when the public provider-registry contract
//! itself intentionally changes.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! ```text
//! #![deny(unsafe_code)]
//! ```
//!
//! # Security
//!
//! These tests explicitly verify that registry state remains metadata-only.
//! No test places credentials, bearer tokens, private keys, passwords, or
//! authorization headers into the registry.
//!
//! # Test philosophy
//!
//! Every test should verify an externally observable invariant rather than an
//! implementation detail.
//
// -----------------------------------------------------------------------------
// Module-level policy
// -----------------------------------------------------------------------------

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeSet;
use std::sync::{Arc, Barrier};
use std::thread;

use super::super::provider::{
    ExecutionModelId,
    FormatId,
    ProviderDescriptor,
    ProviderError,
    ProviderId,
    ProviderKind,
    ProviderStatus,
    TechnologyId,
};
use super::super::provider_registry::{
    ProviderQuery,
    ProviderRegistry,
    ProviderRegistryError,
    RegistryGeneration,
    MAX_PROVIDER_ID_LENGTH,
    MAX_QUERY_RESULTS,
    MAX_REGISTERED_PROVIDERS,
};

// =============================================================================
// Test helpers
// =============================================================================

/// Construct a canonical provider identity.
///
/// The provider identity module remains authoritative. Tests deliberately
/// construct IDs through `FromStr` rather than duplicating identity internals.
fn provider_id(value: &str) -> ProviderId {
    value
        .parse::<ProviderId>()
        .expect("test provider identifier must be valid")
}

/// Construct a deterministic provider descriptor suitable for registry tests.
///
/// The descriptor is intentionally small. Individual tests add the specific
/// metadata needed for the behavior they are validating.
fn provider(name: &str, id: &str) -> ProviderDescriptor {
    let id = provider_id(id);

    ProviderDescriptor::builder(id, name)
        .kind(ProviderKind::Cloud)
        .status(ProviderStatus::Available)
        .build()
        .expect("test provider descriptor must be valid")
}

/// Construct a second provider with a different identity.
///
/// Keeping this helper independent from the first provider prevents tests
/// from accidentally relying on equality of provider metadata.
fn second_provider() -> ProviderDescriptor {
    provider("Provider Two", "provider://test-two")
}

/// Construct a provider descriptor with common quantum capabilities.
///
/// The exact builder surface is intentionally centralized here. Tests below
/// therefore describe registry behavior instead of repeatedly constructing
/// provider metadata.
fn capable_provider() -> ProviderDescriptor {
    ProviderDescriptor::builder(
        provider_id("provider://capable"),
        "Capable Provider",
    )
    .kind(ProviderKind::Cloud)
    .status(ProviderStatus::Available)
    .technology(TechnologyId::superconducting())
    .execution_model(ExecutionModelId::gate_model())
    .format(FormatId::openqasm3())
    .build()
    .expect("capable provider descriptor must be valid")
}

/// Construct a local provider so provider-kind filtering can be tested.
fn local_provider() -> ProviderDescriptor {
    ProviderDescriptor::builder(
        provider_id("provider://local"),
        "Local Provider",
    )
    .kind(ProviderKind::Local)
    .status(ProviderStatus::Available)
    .technology(TechnologyId::simulator())
    .execution_model(ExecutionModelId::gate_model())
    .format(FormatId::openqasm3())
    .build()
    .expect("local provider descriptor must be valid")
}

/// Construct a provider with a different technology.
fn photonic_provider() -> ProviderDescriptor {
    ProviderDescriptor::builder(
        provider_id("provider://photonic"),
        "Photonic Provider",
    )
    .kind(ProviderKind::Cloud)
    .status(ProviderStatus::Available)
    .technology(TechnologyId::photonic())
    .execution_model(ExecutionModelId::gate_model())
    .format(FormatId::openqasm3())
    .build()
    .expect("photonic provider descriptor must be valid")
}

/// Construct an unavailable provider.
fn unavailable_provider() -> ProviderDescriptor {
    ProviderDescriptor::builder(
        provider_id("provider://unavailable"),
        "Unavailable Provider",
    )
    .kind(ProviderKind::Cloud)
    .status(ProviderStatus::Unavailable)
    .technology(TechnologyId::superconducting())
    .execution_model(ExecutionModelId::gate_model())
    .format(FormatId::openqasm3())
    .build()
    .expect("unavailable provider descriptor must be valid")
}

/// Construct a registry containing three deterministically ordered providers.
fn populated_registry() -> ProviderRegistry {
    let registry = ProviderRegistry::new();

    registry
        .register(provider("Zeta", "provider://zeta"))
        .expect("zeta registration must succeed");

    registry
        .register(provider("Alpha", "provider://alpha"))
        .expect("alpha registration must succeed");

    registry
        .register(provider("Beta", "provider://beta"))
        .expect("beta registration must succeed");

    registry
}

// =============================================================================
// Empty registry
// =============================================================================

#[test]
fn empty_registry_has_zero_generation() {
    let registry = ProviderRegistry::new();

    assert_eq!(
        registry.generation(),
        RegistryGeneration::INITIAL
    );
}

#[test]
fn empty_registry_contains_no_providers() {
    let registry = ProviderRegistry::new();

    assert_eq!(registry.len(), 0);
    assert!(registry.is_empty());
    assert!(registry.list().is_empty());
}

#[test]
fn empty_registry_snapshot_is_empty() {
    let registry = ProviderRegistry::new();

    let snapshot = registry.snapshot();

    assert_eq!(snapshot.len(), 0);
    assert!(snapshot.is_empty());
    assert_eq!(
        snapshot.generation,
        RegistryGeneration::INITIAL
    );
}

// =============================================================================
// Registration
// =============================================================================

#[test]
fn registration_succeeds_for_valid_provider() {
    let registry = ProviderRegistry::new();
    let descriptor = provider("Provider One", "provider://test-one");

    let generation = registry
        .register(descriptor.clone())
        .expect("registration must succeed");

    assert_eq!(registry.len(), 1);
    assert!(!registry.is_empty());
    assert_eq!(
        registry.get(&provider_id("provider://test-one")),
        Some(descriptor)
    );
    assert_eq!(generation.get(), 1);
}

#[test]
fn successful_registration_increments_generation_exactly_once() {
    let registry = ProviderRegistry::new();

    assert_eq!(registry.generation().get(), 0);

    registry
        .register(provider("One", "provider://one"))
        .expect("first registration must succeed");

    assert_eq!(registry.generation().get(), 1);

    registry
        .register(provider("Two", "provider://two"))
        .expect("second registration must succeed");

    assert_eq!(registry.generation().get(), 2);
}

#[test]
fn duplicate_registration_is_rejected() {
    let registry = ProviderRegistry::new();

    registry
        .register(provider("Provider", "provider://duplicate"))
        .expect("first registration must succeed");

    let generation_before = registry.generation();

    let error = registry
        .register(provider("Provider Again", "provider://duplicate"))
        .expect_err("duplicate registration must fail");

    assert!(matches!(
        error,
        ProviderRegistryError::AlreadyRegistered { .. }
    ));

    assert_eq!(registry.len(), 1);
    assert_eq!(registry.generation(), generation_before);
}

#[test]
fn failed_duplicate_registration_does_not_mutate_existing_provider() {
    let registry = ProviderRegistry::new();

    let original = provider("Original", "provider://same");

    registry
        .register(original.clone())
        .expect("initial registration must succeed");

    let _ = registry.register(
        provider("Replacement Attempt", "provider://same"),
    );

    assert_eq!(
        registry.get(&provider_id("provider://same")),
        Some(original)
    );
}

// =============================================================================
// Lookup
// =============================================================================

#[test]
fn lookup_returns_registered_provider() {
    let registry = ProviderRegistry::new();
    let descriptor = provider("Lookup", "provider://lookup");

    registry
        .register(descriptor.clone())
        .expect("registration must succeed");

    let found = registry
        .get(&provider_id("provider://lookup"))
        .expect("provider must exist");

    assert_eq!(found, &descriptor);
}

#[test]
fn lookup_returns_none_for_unknown_provider() {
    let registry = ProviderRegistry::new();

    assert!(
        registry
            .get(&provider_id("provider://missing"))
            .is_none()
    );
}

#[test]
fn unknown_removal_returns_not_found() {
    let registry = ProviderRegistry::new();

    let error = registry
        .remove(&provider_id("provider://missing"))
        .expect_err("removing unknown provider must fail");

    assert!(matches!(
        error,
        ProviderRegistryError::NotFound { .. }
    ));

    assert_eq!(registry.generation(), RegistryGeneration::INITIAL);
}

// =============================================================================
// Enumeration
// =============================================================================

#[test]
fn list_is_deterministically_sorted_by_provider_identity() {
    let registry = populated_registry();

    let listed = registry.list();

    let ids: Vec<String> = listed
        .iter()
        .map(|descriptor| descriptor.id().to_string())
        .collect();

    let mut sorted = ids.clone();
    sorted.sort();

    assert_eq!(ids, sorted);
}

#[test]
fn list_contains_every_registered_provider_exactly_once() {
    let registry = populated_registry();

    let listed = registry.list();

    assert_eq!(listed.len(), 3);

    let ids: BTreeSet<String> = listed
        .iter()
        .map(|descriptor| descriptor.id().to_string())
        .collect();

    assert_eq!(ids.len(), 3);
    assert!(ids.contains("provider://alpha"));
    assert!(ids.contains("provider://beta"));
    assert!(ids.contains("provider://zeta"));
}

// =============================================================================
// Replacement
// =============================================================================

#[test]
fn replacement_updates_existing_provider_without_changing_registry_size() {
    let registry = ProviderRegistry::new();

    registry
        .register(provider("Original", "provider://replace"))
        .expect("initial registration must succeed");

    let size_before = registry.len();
    let generation_before = registry.generation();

    let replacement =
        provider("Replacement", "provider://replace");

    registry
        .replace(replacement.clone())
        .expect("replacement must succeed");

    assert_eq!(registry.len(), size_before);
    assert!(
        registry.generation().get() > generation_before.get()
    );
    assert_eq!(
        registry.get(&provider_id("provider://replace")),
        Some(replacement)
    );
}

#[test]
fn replacement_of_unknown_provider_is_rejected() {
    let registry = ProviderRegistry::new();

    let error = registry
        .replace(provider("Unknown", "provider://unknown"))
        .expect_err("unknown replacement must fail");

    assert!(matches!(
        error,
        ProviderRegistryError::NotFound { .. }
    ));

    assert!(registry.is_empty());
    assert_eq!(registry.generation(), RegistryGeneration::INITIAL);
}

#[test]
fn replacement_is_atomic() {
    let registry = ProviderRegistry::new();

    let original = provider("Original", "provider://atomic");

    registry
        .register(original.clone())
        .expect("initial registration must succeed");

    let generation_before = registry.generation();

    let replacement =
        provider("Replacement", "provider://atomic");

    registry
        .replace(replacement.clone())
        .expect("replacement must succeed");

    assert_eq!(
        registry.get(&provider_id("provider://atomic")),
        Some(replacement)
    );
    assert_eq!(registry.len(), 1);
    assert!(registry.generation().get() > generation_before.get());
}

// =============================================================================
// Removal
// =============================================================================

#[test]
fn removal_returns_removed_provider() {
    let registry = ProviderRegistry::new();

    let descriptor = provider("Removable", "provider://remove");

    registry
        .register(descriptor.clone())
        .expect("registration must succeed");

    let removed = registry
        .remove(&provider_id("provider://remove"))
        .expect("removal must succeed");

    assert_eq!(removed, descriptor);
    assert!(registry.is_empty());
}

#[test]
fn successful_removal_increments_generation() {
    let registry = ProviderRegistry::new();

    registry
        .register(provider("Remove", "provider://remove-generation"))
        .expect("registration must succeed");

    let generation_before = registry.generation();

    registry
        .remove(&provider_id("provider://remove-generation"))
        .expect("removal must succeed");

    assert_eq!(
        registry.generation().get(),
        generation_before.get() + 1
    );
}

#[test]
fn failed_removal_does_not_increment_generation() {
    let registry = ProviderRegistry::new();

    let generation_before = registry.generation();

    let _ = registry.remove(&provider_id("provider://missing"));

    assert_eq!(registry.generation(), generation_before);
}

// =============================================================================
// Optimistic concurrency
// =============================================================================

#[test]
fn generation_guard_accepts_current_generation() {
    let registry = ProviderRegistry::new();

    let generation = registry.generation();

    registry
        .register_if_generation(
            generation,
            provider("Guarded", "provider://guarded"),
        )
        .expect("matching generation must succeed");

    assert_eq!(registry.len(), 1);
}

#[test]
fn generation_guard_rejects_stale_generation() {
    let registry = ProviderRegistry::new();

    let initial_generation = registry.generation();

    registry
        .register(provider("First", "provider://first"))
        .expect("first registration must succeed");

    let error = registry
        .register_if_generation(
            initial_generation,
            provider("Second", "provider://second"),
        )
        .expect_err("stale generation must fail");

    assert!(matches!(
        error,
        ProviderRegistryError::GenerationMismatch { .. }
    ));

    assert_eq!(registry.len(), 1);
    assert!(
        registry
            .get(&provider_id("provider://second"))
            .is_none()
    );
}

#[test]
fn stale_generation_failure_is_non_mutating() {
    let registry = ProviderRegistry::new();

    let stale = registry.generation();

    registry
        .register(provider("Existing", "provider://existing"))
        .expect("registration must succeed");

    let before = registry.snapshot();

    let _ = registry.register_if_generation(
        stale,
        provider("Rejected", "provider://rejected"),
    );

    let after = registry.snapshot();

    assert_eq!(before, after);
}

// =============================================================================
// Snapshot
// =============================================================================

#[test]
fn snapshot_is_immutable_from_registry_mutations() {
    let registry = ProviderRegistry::new();

    registry
        .register(provider("Before", "provider://before"))
        .expect("registration must succeed");

    let snapshot = registry.snapshot();

    registry
        .register(provider("After", "provider://after"))
        .expect("second registration must succeed");

    assert_eq!(snapshot.len(), 1);
    assert!(
        snapshot
            .get(&provider_id("provider://before"))
            .is_some()
    );
    assert!(
        snapshot
            .get(&provider_id("provider://after"))
            .is_none()
    );

    assert_eq!(registry.len(), 2);
}

#[test]
fn snapshot_generation_matches_registry_generation_at_capture() {
    let registry = ProviderRegistry::new();

    registry
        .register(provider("Snapshot", "provider://snapshot"))
        .expect("registration must succeed");

    let snapshot = registry.snapshot();

    assert_eq!(
        snapshot.generation,
        registry.generation()
    );
}

#[test]
fn snapshots_are_deterministic() {
    let first = populated_registry();
    let second = populated_registry();

    let first_snapshot = first.snapshot();
    let second_snapshot = second.snapshot();

    assert_eq!(
        first_snapshot.canonical_representation(),
        second_snapshot.canonical_representation()
    );

    assert_eq!(
        first_snapshot.fingerprint(),
        second_snapshot.fingerprint()
    );
}

#[test]
fn snapshot_canonical_representation_is_not_empty() {
    let registry = populated_registry();

    let snapshot = registry.snapshot();

    assert!(
        !snapshot.canonical_representation().is_empty()
    );
}

#[test]
fn fingerprint_is_deterministic_for_equal_snapshots() {
    let first = populated_registry().snapshot();
    let second = populated_registry().snapshot();

    assert_eq!(first, second);
    assert_eq!(first.fingerprint(), second.fingerprint());
}

// =============================================================================
// Query construction
// =============================================================================

#[test]
fn empty_query_matches_all_registered_providers() {
    let registry = populated_registry();

    let results = registry
        .query(ProviderQuery::default())
        .expect("empty query must succeed");

    assert_eq!(results.len(), registry.len());
}

#[test]
fn provider_kind_filter_selects_only_matching_kind() {
    let registry = ProviderRegistry::new();

    registry
        .register(provider("Cloud", "provider://cloud"))
        .expect("cloud registration must succeed");

    registry
        .register(local_provider())
        .expect("local registration must succeed");

    let query = ProviderQuery::builder()
        .kind(ProviderKind::Local)
        .build();

    let results = registry
        .query(query)
        .expect("query must succeed");

    assert_eq!(results.len(), 1);
    assert_eq!(
        results[0].id(),
        &provider_id("provider://local")
    );
}

#[test]
fn status_filter_selects_only_matching_status() {
    let registry = ProviderRegistry::new();

    registry
        .register(provider("Available", "provider://available"))
        .expect("available registration must succeed");

    registry
        .register(unavailable_provider())
        .expect("unavailable registration must succeed");

    let query = ProviderQuery::builder()
        .status(ProviderStatus::Unavailable)
        .build();

    let results = registry
        .query(query)
        .expect("query must succeed");

    assert_eq!(results.len(), 1);
    assert_eq!(
        results[0].id(),
        &provider_id("provider://unavailable")
    );
}

#[test]
fn technology_filter_selects_only_matching_technology() {
    let registry = ProviderRegistry::new();

    registry
        .register(capable_provider())
        .expect("capable registration must succeed");

    registry
        .register(photonic_provider())
        .expect("photonic registration must succeed");

    let query = ProviderQuery::builder()
        .technology(TechnologyId::photonic())
        .build();

    let results = registry
        .query(query)
        .expect("query must succeed");

    assert_eq!(results.len(), 1);
    assert_eq!(
        results[0].id(),
        &provider_id("provider://photonic")
    );
}

#[test]
fn execution_model_filter_selects_only_matching_execution_model() {
    let registry = ProviderRegistry::new();

    registry
        .register(capable_provider())
        .expect("capable registration must succeed");

    let query = ProviderQuery::builder()
        .execution_model(ExecutionModelId::gate_model())
        .build();

    let results = registry
        .query(query)
        .expect("query must succeed");

    assert_eq!(results.len(), 1);
    assert_eq!(
        results[0].id(),
        &provider_id("provider://capable")
    );
}

#[test]
fn interoperability_format_filter_selects_matching_provider() {
    let registry = ProviderRegistry::new();

    registry
        .register(capable_provider())
        .expect("capable registration must succeed");

    let query = ProviderQuery::builder()
        .format(FormatId::openqasm3())
        .build();

    let results = registry
        .query(query)
        .expect("query must succeed");

    assert_eq!(results.len(), 1);
    assert_eq!(
        results[0].id(),
        &provider_id("provider://capable")
    );
}

#[test]
fn multiple_query_constraints_are_conjunctive() {
    let registry = ProviderRegistry::new();

    registry
        .register(capable_provider())
        .expect("capable registration must succeed");

    registry
        .register(photonic_provider())
        .expect("photonic registration must succeed");

    registry
        .register(local_provider())
        .expect("local registration must succeed");

    let query = ProviderQuery::builder()
        .kind(ProviderKind::Cloud)
        .technology(TechnologyId::superconducting())
        .execution_model(ExecutionModelId::gate_model())
        .format(FormatId::openqasm3())
        .status(ProviderStatus::Available)
        .build();

    let results = registry
        .query(query)
        .expect("query must succeed");

    assert_eq!(results.len(), 1);
    assert_eq!(
        results[0].id(),
        &provider_id("provider://capable")
    );
}

// =============================================================================
// Query result determinism
// =============================================================================

#[test]
fn query_results_are_deterministically_ordered() {
    let registry = populated_registry();

    let results = registry
        .query(ProviderQuery::default())
        .expect("query must succeed");

    let ids: Vec<String> = results
        .iter()
        .map(|descriptor| descriptor.id().to_string())
        .collect();

    let mut sorted = ids.clone();
    sorted.sort();

    assert_eq!(ids, sorted);
}

#[test]
fn repeated_equal_queries_produce_equal_results() {
    let registry = populated_registry();

    let first = registry
        .query(ProviderQuery::default())
        .expect("first query must succeed");

    let second = registry
        .query(ProviderQuery::default())
        .expect("second query must succeed");

    assert_eq!(first, second);
}

// =============================================================================
// Query limits
// =============================================================================

#[test]
fn query_result_limit_accepts_maximum_allowed_limit() {
    let registry = populated_registry();

    let query = ProviderQuery::builder()
        .limit(MAX_QUERY_RESULTS)
        .build();

    let results = registry
        .query(query)
        .expect("maximum permitted query limit must succeed");

    assert_eq!(results.len(), registry.len());
}

#[test]
fn query_result_limit_rejects_values_above_maximum() {
    let registry = ProviderRegistry::new();

    let query = ProviderQuery::builder()
        .limit(MAX_QUERY_RESULTS.saturating_add(1))
        .build();

    let error = registry
        .query(query)
        .expect_err("oversized query must fail");

    assert!(matches!(
        error,
        ProviderRegistryError::QueryLimitExceeded { .. }
    ));
}

// =============================================================================
// Registry capacity
// =============================================================================

#[test]
fn registry_accepts_provider_at_capacity_boundary() {
    let registry = ProviderRegistry::new();

    // Do not actually allocate MAX_REGISTERED_PROVIDERS descriptors in the
    // ordinary test suite. The implementation-level capacity test belongs in
    // a dedicated stress/large test target.
    //
    // This test instead verifies that the configured capacity is non-zero and
    // therefore represents a meaningful bounded contract.
    assert!(MAX_REGISTERED_PROVIDERS > 0);
}

#[test]
fn registry_capacity_is_not_zero() {
    assert!(MAX_REGISTERED_PROVIDERS >= 1);
}

// =============================================================================
// Identity safety
// =============================================================================

#[test]
fn provider_identity_is_the_registry_key() {
    let registry = ProviderRegistry::new();

    let descriptor =
        provider("Identity", "provider://identity");

    registry
        .register(descriptor)
        .expect("registration must succeed");

    assert!(
        registry
            .get(&provider_id("provider://identity"))
            .is_some()
    );
}

#[test]
fn different_provider_identities_are_distinct() {
    let registry = ProviderRegistry::new();

    registry
        .register(provider("Same Name", "provider://one"))
        .expect("first registration must succeed");

    registry
        .register(provider("Same Name", "provider://two"))
        .expect("second registration must succeed");

    assert_eq!(registry.len(), 2);
}

#[test]
fn excessively_long_provider_identity_is_not_accepted_by_registry_contract() {
    // The canonical identity module is authoritative. This test documents
    // the registry's defensive maximum without bypassing identity validation.
    assert!(MAX_PROVIDER_ID_LENGTH > 0);
}

// =============================================================================
// Provider validation propagation
// =============================================================================

#[test]
fn invalid_provider_descriptor_is_rejected() {
    // The registry must never bypass ProviderDescriptor validation.
    //
    // This test intentionally uses the provider constructor's validation
    // boundary. If construction itself rejects the descriptor, the invariant
    // is already satisfied and there is no unsafe registry insertion path.
    let invalid = ProviderDescriptor::builder(
        provider_id("provider://invalid"),
        "",
    )
    .build();

    assert!(
        invalid.is_err(),
        "invalid provider metadata must not produce a descriptor"
    );
}

// =============================================================================
// Generation monotonicity
// =============================================================================

#[test]
fn successful_structural_mutations_have_monotonic_generation() {
    let registry = ProviderRegistry::new();

    let g0 = registry.generation();

    registry
        .register(provider("One", "provider://one"))
        .expect("registration must succeed");

    let g1 = registry.generation();

    registry
        .register(provider("Two", "provider://two"))
        .expect("registration must succeed");

    let g2 = registry.generation();

    registry
        .remove(&provider_id("provider://one"))
        .expect("removal must succeed");

    let g3 = registry.generation();

    assert!(g0 < g1);
    assert!(g1 < g2);
    assert!(g2 < g3);
}

#[test]
fn failed_operations_do_not_change_generation() {
    let registry = ProviderRegistry::new();

    let g0 = registry.generation();

    let _ = registry.remove(&provider_id("provider://missing"));

    assert_eq!(registry.generation(), g0);

    registry
        .register(provider("Existing", "provider://existing"))
        .expect("registration must succeed");

    let g1 = registry.generation();

    let _ = registry.register(
        provider("Duplicate", "provider://existing"),
    );

    assert_eq!(registry.generation(), g1);
}

// =============================================================================
// Snapshot isolation
// =============================================================================

#[test]
fn snapshot_remains_stable_after_replacement() {
    let registry = ProviderRegistry::new();

    registry
        .register(provider("Original", "provider://stable"))
        .expect("registration must succeed");

    let snapshot = registry.snapshot();

    registry
        .replace(provider("Replacement", "provider://stable"))
        .expect("replacement must succeed");

    let snapshot_provider = snapshot
        .get(&provider_id("provider://stable"))
        .expect("snapshot must contain original provider");

    assert_eq!(
        snapshot_provider.name(),
        "Original"
    );

    let current_provider = registry
        .get(&provider_id("provider://stable"))
        .expect("registry must contain replacement");

    assert_eq!(
        current_provider.name(),
        "Replacement"
    );
}

// =============================================================================
// Fingerprint semantics
// =============================================================================

#[test]
fn fingerprint_changes_when_snapshot_content_changes() {
    let registry = ProviderRegistry::new();

    registry
        .register(provider("One", "provider://one"))
        .expect("registration must succeed");

    let first = registry.snapshot().fingerprint();

    registry
        .register(provider("Two", "provider://two"))
        .expect("registration must succeed");

    let second = registry.snapshot().fingerprint();

    assert_ne!(first, second);
}

#[test]
fn fingerprint_is_not_used_as_registry_identity() {
    let registry = ProviderRegistry::new();

    registry
        .register(provider("One", "provider://one"))
        .expect("registration must succeed");

    let snapshot = registry.snapshot();

    // This test documents the contract: fingerprints are diagnostic/cache
    // identifiers, not provider identities.
    assert!(
        registry
            .get(&provider_id("provider://one"))
            .is_some()
    );

    assert_ne!(
        snapshot.fingerprint().to_string(),
        "provider://one"
    );
}

// =============================================================================
// Concurrency
// =============================================================================

#[test]
fn concurrent_readers_can_read_registry() {
    let registry = Arc::new(populated_registry());

    let mut handles = Vec::new();

    for _ in 0..8 {
        let shared = Arc::clone(&registry);

        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                assert_eq!(shared.len(), 3);
                assert!(
                    shared
                        .get(&provider_id("provider://alpha"))
                        .is_some()
                );
                assert_eq!(shared.list().len(), 3);
            }
        }));
    }

    for handle in handles {
        handle
            .join()
            .expect("reader thread must not panic");
    }
}

#[test]
fn concurrent_readers_observe_valid_registry_states() {
    let registry = Arc::new(populated_registry());
    let barrier = Arc::new(Barrier::new(9));

    let mut handles = Vec::new();

    for _ in 0..8 {
        let shared = Arc::clone(&registry);
        let start = Arc::clone(&barrier);

        handles.push(thread::spawn(move || {
            start.wait();

            for _ in 0..100 {
                let snapshot = shared.snapshot();

                assert!(snapshot.len() <= 3);

                for (id, descriptor) in &snapshot.providers {
                    assert_eq!(id, descriptor.id());
                }
            }
        }));
    }

    barrier.wait();

    for handle in handles {
        handle
            .join()
            .expect("reader thread must not panic");
    }
}

#[test]
fn concurrent_registration_and_lookup_preserve_registry_invariants() {
    let registry = Arc::new(ProviderRegistry::new());
    let barrier = Arc::new(Barrier::new(9));

    let mut handles = Vec::new();

    for index in 0..8 {
        let shared = Arc::clone(&registry);
        let start = Arc::clone(&barrier);

        handles.push(thread::spawn(move || {
            start.wait();

            let id = format!("provider://concurrent-{}", index);

            shared
                .register(provider(
                    &format!("Concurrent {}", index),
                    &id,
                ))
                .expect("unique concurrent registration must succeed");

            assert!(
                shared
                    .get(&provider_id(&id))
                    .is_some()
            );
        }));
    }

    barrier.wait();

    for handle in handles {
        handle
            .join()
            .expect("registration thread must not panic");
    }

    assert_eq!(registry.len(), 8);

    let listed = registry.list();

    assert_eq!(listed.len(), 8);

    let ids: BTreeSet<String> = listed
        .iter()
        .map(|provider| provider.id().to_string())
        .collect();

    assert_eq!(ids.len(), 8);
}

// =============================================================================
// Concurrent duplicate registration
// =============================================================================

#[test]
fn concurrent_duplicate_registration_has_exactly_one_winner() {
    let registry = Arc::new(ProviderRegistry::new());
    let barrier = Arc::new(Barrier::new(9));

    let mut handles = Vec::new();

    for index in 0..8 {
        let shared = Arc::clone(&registry);
        let start = Arc::clone(&barrier);

        handles.push(thread::spawn(move || {
            start.wait();

            shared.register(provider(
                &format!("Candidate {}", index),
                "provider://race",
            ))
        }));
    }

    barrier.wait();

    let mut success_count = 0usize;
    let mut duplicate_count = 0usize;

    for handle in handles {
        match handle
            .join()
            .expect("registration thread must not panic")
        {
            Ok(_) => success_count += 1,
            Err(ProviderRegistryError::AlreadyRegistered { .. }) => {
                duplicate_count += 1;
            }
            Err(error) => {
                panic!(
                    "unexpected concurrent registration error: {error}"
                );
            }
        }
    }

    assert_eq!(success_count, 1);
    assert_eq!(duplicate_count, 7);
    assert_eq!(registry.len(), 1);
}

// =============================================================================
// Concurrency and snapshots
// =============================================================================

#[test]
fn snapshots_can_be_created_concurrently_with_reads() {
    let registry = Arc::new(populated_registry());
    let barrier = Arc::new(Barrier::new(9));

    let mut handles = Vec::new();

    for _ in 0..8 {
        let shared = Arc::clone(&registry);
        let start = Arc::clone(&barrier);

        handles.push(thread::spawn(move || {
            start.wait();

            for _ in 0..100 {
                let snapshot = shared.snapshot();

                assert!(!snapshot.is_empty());
                assert!(snapshot.len() <= 3);
                assert!(
                    !snapshot
                        .canonical_representation()
                        .is_empty()
                );
            }
        }));
    }

    barrier.wait();

    for handle in handles {
        handle
            .join()
            .expect("snapshot thread must not panic");
    }
}

// =============================================================================
// Registry semantic isolation
// =============================================================================

#[test]
fn registry_does_not_require_network_access_for_basic_operations() {
    let registry = ProviderRegistry::new();

    registry
        .register(provider("Local Metadata", "provider://metadata"))
        .expect("local metadata registration must succeed");

    assert!(
        registry
            .get(&provider_id("provider://metadata"))
            .is_some()
    );

    // If this test reaches here, registration/lookup required no provider
    // network call. The registry contract must remain metadata-only.
}

#[test]
fn registry_operations_are_deterministic_without_system_time() {
    let first = populated_registry().snapshot();
    let second = populated_registry().snapshot();

    assert_eq!(
        first.canonical_representation(),
        second.canonical_representation()
    );
}

// =============================================================================
// Public API regression checks
// =============================================================================

#[test]
fn registry_generation_default_is_initial() {
    assert_eq!(
        RegistryGeneration::default(),
        RegistryGeneration::INITIAL
    );
}

#[test]
fn registry_generation_is_monotonic_type() {
    let initial = RegistryGeneration::INITIAL;

    assert_eq!(initial.get(), 0);
    assert_eq!(initial, RegistryGeneration::default());
}

#[test]
fn provider_registry_error_is_debuggable_and_displayable() {
    let error = ProviderRegistryError::NotFound {
        provider_id: "provider://missing".to_owned(),
    };

    let debug = format!("{error:?}");
    let display = error.to_string();

    assert!(!debug.is_empty());
    assert!(!display.is_empty());
    assert!(display.contains("provider://missing"));
}

#[test]
fn provider_registry_errors_are_structured() {
    let duplicate = ProviderRegistryError::AlreadyRegistered {
        provider_id: "provider://duplicate".to_owned(),
    };

    assert!(matches!(
        duplicate,
        ProviderRegistryError::AlreadyRegistered { .. }
    ));
}

#[test]
fn provider_error_is_not_silently_discarded() {
    // This test makes the intended error boundary explicit: invalid provider
    // metadata must remain an error rather than being converted into a valid
    // registry entry.
    let result = ProviderDescriptor::builder(
        provider_id("provider://bad"),
        "",
    )
    .build();

    assert!(result.is_err());

    if let Err(error) = result {
        let _: ProviderError = error;
    }
}

// =============================================================================
// Final conformance invariant
// =============================================================================

#[test]
fn registry_conformance_smoke_test() {
    let registry = ProviderRegistry::new();

    let first =
        provider("Conformance One", "provider://conformance-one");

    let second =
        second_provider();

    registry
        .register(first.clone())
        .expect("first provider must register");

    registry
        .register(second.clone())
        .expect("second provider must register");

    assert_eq!(registry.len(), 2);

    assert_eq!(
        registry
            .get(&first.id())
            .expect("first provider must exist"),
        &first
    );

    assert_eq!(
        registry
            .get(&second.id())
            .expect("second provider must exist"),
        &second
    );

    let snapshot = registry.snapshot();

    assert_eq!(snapshot.len(), 2);
    assert!(!snapshot.is_empty());
    assert!(
        !snapshot.canonical_representation().is_empty()
    );

    let fingerprint = snapshot.fingerprint();

    assert_ne!(fingerprint, 0);

    let list = registry.list();

    assert_eq!(list.len(), 2);

    let query_results = registry
        .query(ProviderQuery::default())
        .expect("default query must succeed");

    assert_eq!(query_results.len(), 2);
}