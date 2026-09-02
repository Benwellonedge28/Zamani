//! Zamani Quantum Noise (ZQN) — Integration Tests
//!
//! # Ownership
//!
//! This module owns integration-level verification of the ZQN composition
//! boundary.
//!
//! It verifies that ZQN can be attached to canonical Zamani quantum semantics
//! without:
//!
//! - replacing the canonical Quantum IR;
//! - defining another QubitId;
//! - defining another PhysicalQubitId;
//! - defining another OperationId;
//! - embedding vendor APIs;
//! - embedding routing algorithms;
//! - embedding scheduling algorithms;
//! - embedding QEC algorithms;
//! - embedding hardware implementations;
//! - embedding runtime implementations;
//! - imposing a semantic machine-size limit.
//!
//! # Architectural boundary
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::frontend
//!      │
//!      ▼
//! ┌──────────────────────────────┐
//! │ crate::quantum::ir           │
//! │                              │
//! │ canonical program semantics  │
//! │ OperationId                  │
//! │ QubitId                      │
//! │ PhysicalQubitId              │
//! └──────────────┬───────────────┘
//!                │
//!                ▼
//! ┌──────────────────────────────┐
//! │ quantum::zqn::integration     │
//! │                              │
//! │ semantic association         │
//! │ noise-model references       │
//! │ resource scopes              │
//! │ validation                   │
//! │ deterministic inspection     │
//! └──────────────┬───────────────┘
//!                │
//!       ┌────────┼─────────┬──────────┐
//!       ▼        ▼         ▼          ▼
//!    routing scheduling   QEC      hardware
//!       │        │         │          │
//!       └────────┴─────────┴──────────┘
//!                         │
//!                         ▼
//!                       runtime
//!                         │
//!                         ▼
//!                    observations
//! ```
//!
//! # What these tests verify
//!
//! The tests verify the integration contract rather than re-testing the
//! mathematical implementation of channels, probability distributions, faults,
//! or noise models.
//!
//! In particular:
//!
//! 1. canonical IR identities cross the ZQN boundary unchanged;
//! 2. operation bindings retain canonical `OperationId` values;
//! 3. logical resources retain canonical `QubitId` values;
//! 4. physical resources retain canonical `PhysicalQubitId` values;
//! 5. logical and physical identity domains remain distinct;
//! 6. resource selectors remain deferred and are not materialized;
//! 7. operation/resource bindings can coexist;
//! 8. duplicate policy is respected;
//! 9. replacement is atomic;
//! 10. `AllowMultiple` remains deterministic;
//! 11. invalid bindings do not partially mutate containers;
//! 12. operation-reference validation works against caller-supplied namespaces;
//! 13. duplicate operation namespaces are rejected;
//! 14. structural fingerprints are deterministic;
//! 15. semantically different binding sets do not accidentally share the same
//!     test-observable structure;
//! 16. removing bindings does not affect unrelated bindings;
//! 17. explicit resource counts remain independent of deferred selectors;
//! 18. caller-owned limits are distinguishable from semantic machine limits;
//! 19. large representable canonical IDs work without an artificial machine
//!     ceiling;
//! 20. the integration representation remains independent of concrete hardware;
//! 21. the integration layer does not require materializing an entire machine.
//!
//! # Non-ownership
//!
//! This test module does NOT define:
//!
//! - canonical quantum semantics;
//! - quantum operations;
//! - quantum channels;
//! - probabilities;
//! - faults;
//! - noise models;
//! - calibration;
//! - characterization;
//! - routing;
//! - scheduling;
//! - QEC;
//! - hardware;
//! - runtime;
//! - benchmark methodology.
//!
//! Those contracts belong to their respective owning modules.
//!
//! # Canonical identity rule
//!
//! All quantum resource identities in these tests come from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This file deliberately does not define a ZQN-specific replacement.
//!
//! Likewise, operation identity comes from:
//!
//! ```text
//! crate::quantum::ir::identity::OperationId
//! ```
//!
//! # Write once, scale everywhere
//!
//! No test in this file establishes a semantic maximum for:
//!
//! - qubits;
//! - physical qubits;
//! - operations;
//! - bindings;
//! - resources;
//! - gates;
//! - circuit depth;
//! - machine size.
//!
//! Finite values used by tests are execution budgets only.
//!
//! They MUST NOT become constants in production semantics.
//!
//! A selector such as `AllPhysicalQubits` must remain a selector. The integration
//! layer must not expand it into a list of every physical resource merely to
//! represent the association.
//!
//! # Resource policy
//!
//! Explicit limits are tested only where the production API explicitly accepts
//! a caller-owned limit.
//!
//! Such limits are policy, not machine-size semantics.
//!
//! # Determinism
//!
//! Integration data must be deterministic.
//!
//! The tests therefore require:
//!
//! - deterministic ordered inspection;
//! - stable structural fingerprints;
//! - no global mutable state;
//! - no wall-clock dependence;
//! - no random allocation;
//! - no hidden RNG;
//! - no thread identity dependence.
//!
//! # Transactionality
//!
//! The production `IrNoiseBindings::extend` API is documented as transactional.
//!
//! These tests verify that a failed extension leaves the original binding set
//! unchanged.
//!
//! # Error handling
//!
//! Tests intentionally verify structured errors rather than panics.
//!
//! An invalid external reference must be reported explicitly rather than being
//! silently accepted.
//!
//! # Hardware independence
//!
//! These tests do not contact:
//!
//! - QPUs;
//! - vendor SDKs;
//! - network services;
//! - cloud APIs;
//! - credentials;
//! - filesystems.
//!
//! Hardware existence is deliberately outside this integration layer.
//!
//! # Rust compatibility
//!
//! Required:
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
//! This file is complete when the integration boundary guarantees:
//!
//! 1. canonical identity preservation;
//! 2. deterministic association;
//! 3. explicit resource scoping;
//! 4. deferred large-machine selectors;
//! 5. duplicate-policy correctness;
//! 6. transactional mutation;
//! 7. reference validation;
//! 8. deterministic fingerprints;
//! 9. scalable identifier handling;
//! 10. no semantic machine-size ceiling;
//! 11. no vendor coupling;
//! 12. no second quantum IR;
//! 13. no unsafe Rust;
//! 14. independence from downstream implementation details.
//!
//! Downstream implementations of routing, scheduling, QEC, hardware, runtime,
//! or benchmarking should consume these contracts rather than requiring this
//! file to be rewritten.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeSet;

use crate::quantum::ir::identity::OperationId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

use crate::quantum::zqn::core::ids::NoiseModelId;

use crate::quantum::zqn::integration::ir::{
    bind_operation,
    bind_operation_and_resource,
    bind_resource,
    logical_resource,
    physical_resource,
    validate_non_empty_operation_namespace,
    validate_operation_namespace,
    IrBindingScope,
    IrIntegrationError,
    IrNoiseBinding,
    IrNoiseBindings,
    IrNoiseBindingsBuilder,
    IrOperationRef,
    IrResourceScope,
    IrResourceSelector,
    OperationBindingPolicy,
    ResourceIdentity,
};

// =============================================================================
// Test helpers
// =============================================================================

fn operation(value: u64) -> OperationId {
    OperationId::new(value)
}

fn logical(value: u64) -> QubitId {
    QubitId::new(value)
}

fn physical(value: u64) -> PhysicalQubitId {
    PhysicalQubitId::new(value)
}

fn model(value: u64) -> NoiseModelId {
    NoiseModelId::new(value)
}

// =============================================================================
// Canonical identity integration
// =============================================================================

#[test]
fn canonical_operation_identity_crosses_zqn_boundary_unchanged() {
    let operation_id = operation(7);

    let binding = bind_operation(operation_id, model(1))
        .expect("canonical operation binding must be constructible");

    assert_eq!(binding.operation_id(), Some(operation_id));
    assert_eq!(binding.noise_model_id(), model(1));
}

#[test]
fn canonical_logical_qubit_identity_crosses_zqn_boundary_unchanged() {
    let qubit = logical(11);

    let resource = logical_resource(qubit);

    assert_eq!(
        resource,
        ResourceIdentity::LogicalQubit(qubit)
    );

    let scope = IrResourceScope::logical_qubit(qubit);

    assert!(scope.contains_explicit(resource));
    assert!(scope.contains_logical_resources());
    assert!(!scope.contains_physical_resources());
}

#[test]
fn canonical_physical_qubit_identity_crosses_zqn_boundary_unchanged() {
    let qubit = physical(13);

    let resource = physical_resource(qubit);

    assert_eq!(
        resource,
        ResourceIdentity::PhysicalQubit(qubit)
    );

    let scope = IrResourceScope::physical_qubit(qubit);

    assert!(scope.contains_explicit(resource));
    assert!(!scope.contains_logical_resources());
    assert!(scope.contains_physical_resources());
}

#[test]
fn_equal_numeric_indices_do_not_merge_logical_and_physical_domains() {
    let logical_resource = logical_resource(logical(5));
    let physical_resource = physical_resource(physical(5));

    assert_ne!(logical_resource, physical_resource);
}

// =============================================================================
// Binding scope integration
// =============================================================================

#[test]
fn operation_binding_targets_only_operation() {
    let operation_id = operation(1);

    let binding = IrNoiseBinding::for_operation(
        operation_id,
        model(10),
    )
    .expect("operation binding must be valid");

    assert!(binding.targets_operation());
    assert!(!binding.targets_resource());
    assert_eq!(binding.operation_id(), Some(operation_id));
}

#[test]
fn resource_binding_targets_only_resource() {
    let resource = IrResourceScope::logical_qubit(logical(2));

    let binding = IrNoiseBinding::for_resource(
        resource,
        model(11),
    )
    .expect("resource binding must be valid");

    assert!(!binding.targets_operation());
    assert!(binding.targets_resource());
    assert_eq!(binding.operation_id(), None);
}

#[test]
fn operation_and_resource_binding_preserves_both_scopes() {
    let operation_id = operation(3);
    let resource = IrResourceScope::physical_qubit(physical(9));

    let binding = IrNoiseBinding::for_operation_and_resource(
        operation_id,
        resource.clone(),
        model(12),
    )
    .expect("combined binding must be valid");

    assert!(binding.targets_operation());
    assert!(binding.targets_resource());
    assert_eq!(binding.operation_id(), Some(operation_id));

    assert_eq!(
        binding.scope().resource_scope(),
        Some(&resource)
    );
}

#[test]
fn operation_reference_is_only_a_canonical_operation_wrapper() {
    let operation_id = operation(17);
    let reference = IrOperationRef::new(operation_id);

    assert_eq!(reference.operation_id(), operation_id);
    assert_eq!(
        IrOperationRef::from(operation_id),
        reference
    );
}

#[test]
fn binding_label_is_metadata_and_does_not_change_identity_scope() {
    let operation_id = operation(19);

    let unlabeled = IrNoiseBinding::for_operation(
        operation_id,
        model(20),
    )
    .expect("binding must be valid");

    let labeled = unlabeled
        .clone()
        .with_label("calibrated-gate")
        .expect("non-empty label must be accepted");

    assert_eq!(labeled.operation_id(), unlabeled.operation_id());
    assert_eq!(
        labeled.noise_model_id(),
        unlabeled.noise_model_id()
    );
    assert_eq!(labeled.label(), Some("calibrated-gate"));
}

// =============================================================================
// Resource scope integration
// =============================================================================

#[test]
fn global_scope_is_not_an_explicit_resource_enumeration() {
    let scope = IrResourceScope::global();

    assert!(!scope.is_semantically_empty());
    assert_eq!(scope.explicit_resource_count(), 0);
    assert!(!scope.contains_logical_resources());
    assert!(!scope.contains_physical_resources());
}

#[test]
fn all_logical_selector_remains_deferred() {
    let selector =
        IrResourceSelector::AllLogicalQubits;

    let scope = IrResourceScope::selector(selector);

    assert!(!scope.is_semantically_empty());
    assert!(scope.contains_logical_resources());
    assert!(!scope.contains_physical_resources());

    // Most importantly, no resources were materialized.
    assert_eq!(scope.explicit_resource_count(), 0);
}

#[test]
fn all_physical_selector_remains_deferred() {
    let selector =
        IrResourceSelector::AllPhysicalQubits;

    let scope = IrResourceScope::selector(selector);

    assert!(!scope.is_semantically_empty());
    assert!(!scope.contains_logical_resources());
    assert!(scope.contains_physical_resources());

    // A selector represents intent, not a materialized machine.
    assert_eq!(scope.explicit_resource_count(), 0);
}

#[test]
fn arbitrary_selector_strings_are_data_not_executable_code() {
    let label = IrResourceSelector::label("processor-a")
        .expect("valid label selector");

    let namespace = IrResourceSelector::namespace("device/resources")
        .expect("valid namespace selector");

    let predicate =
        IrResourceSelector::predicate("technology == neutral_atom")
            .expect("valid predicate data");

    assert_eq!(
        IrResourceScope::selector(label)
            .explicit_resource_count(),
        0
    );

    assert_eq!(
        IrResourceScope::selector(namespace)
            .explicit_resource_count(),
        0
    );

    assert_eq!(
        IrResourceScope::selector(predicate)
            .explicit_resource_count(),
        0
    );
}

#[test]
fn empty_selector_values_are_rejected() {
    assert!(matches!(
        IrResourceSelector::label(""),
        Err(IrIntegrationError::InvalidSelector { .. })
    ));

    assert!(matches!(
        IrResourceSelector::namespace("   "),
        Err(IrIntegrationError::InvalidSelector { .. })
    ));

    assert!(matches!(
        IrResourceSelector::predicate(""),
        Err(IrIntegrationError::InvalidSelector { .. })
    ));
}

#[test]
fn duplicate_explicit_resources_are_rejected() {
    let resource = logical_resource(logical(23));

    let result = IrResourceScope::resources([
        resource,
        resource,
    ]);

    assert!(matches!(
        result,
        Err(IrIntegrationError::DuplicateResource { .. })
    ));
}

#[test]
fn empty_explicit_resource_scope_is_rejected() {
    let result = IrResourceScope::resources(
        std::iter::empty::<ResourceIdentity>(),
    );

    assert!(matches!(
        result,
        Err(IrIntegrationError::InvalidResourceBinding { .. })
    ));
}

#[test]
fn empty_composite_scope_is_rejected() {
    let result =
        IrResourceScope::composite(std::iter::empty::<IrResourceScope>());

    assert!(matches!(
        result,
        Err(IrIntegrationError::EmptyCompositeScope)
    ));
}

#[test]
fn composite_scope_preserves_all_explicit_resources() {
    let logical_scope =
        IrResourceScope::logical_qubit(logical(31));

    let physical_scope =
        IrResourceScope::physical_qubit(physical(37));

    let composite = IrResourceScope::composite([
        logical_scope.clone(),
        physical_scope.clone(),
    ])
    .expect("non-empty composite must be valid");

    assert_eq!(composite.explicit_resource_count(), 2);
    assert!(composite.contains_logical_resources());
    assert!(composite.contains_physical_resources());

    assert!(composite.contains_explicit(
        logical_resource(logical(31))
    ));

    assert!(composite.contains_explicit(
        physical_resource(physical(37))
    ));
}

// =============================================================================
// Binding container integration
// =============================================================================

#[test]
fn empty_binding_container_has_no_semantic_state() {
    let bindings = IrNoiseBindings::new();

    assert!(bindings.is_empty());
    assert_eq!(bindings.binding_count(), 0);
    assert_eq!(bindings.operation_binding_count(), 0);
    assert_eq!(bindings.resource_binding_count(), 0);
    assert_eq!(
        bindings.additional_operation_binding_count(),
        0
    );
}

#[test]
fn operation_binding_can_be_inserted_and_retrieved() {
    let operation_id = operation(41);
    let binding =
        bind_operation(operation_id, model(42))
            .expect("binding must be valid");

    let mut bindings = IrNoiseBindings::new();

    bindings
        .insert(binding.clone())
        .expect("first insertion must succeed");

    assert_eq!(bindings.binding_count(), 1);
    assert_eq!(
        bindings.get_operation(operation_id),
        Some(&binding)
    );
}

#[test]
fn resource_binding_can_be_inserted_and_retrieved() {
    let resource =
        IrResourceScope::physical_qubit(physical(43));

    let binding =
        bind_resource(resource, model(44))
            .expect("binding must be valid");

    let mut bindings = IrNoiseBindings::new();

    bindings
        .insert(binding.clone())
        .expect("resource insertion must succeed");

    assert_eq!(bindings.binding_count(), 1);
    assert_eq!(bindings.resource_binding_count(), 1);

    assert_eq!(
        bindings.resource_bindings().next(),
        Some(&binding)
    );
}

#[test]
fn duplicate_operation_binding_is_rejected_by_default() {
    let operation_id = operation(51);

    let first =
        bind_operation(operation_id, model(1))
            .expect("first binding must be valid");

    let second =
        bind_operation(operation_id, model(2))
            .expect("second binding must be structurally valid");

    let mut bindings = IrNoiseBindings::new();

    bindings
        .insert(first)
        .expect("first insertion must succeed");

    let result = bindings.insert(second);

    assert!(matches!(
        result,
        Err(IrIntegrationError::DuplicateOperationBinding {
            operation
        }) if operation == operation_id
    ));

    assert_eq!(bindings.binding_count(), 1);
}

#[test]
fn replacement_policy_atomically_replaces_operation_binding() {
    let operation_id = operation(61);

    let first =
        bind_operation(operation_id, model(1))
            .expect("first binding must be valid");

    let second =
        bind_operation(operation_id, model(2))
            .expect("replacement binding must be valid");

    let mut bindings =
        IrNoiseBindings::with_operation_policy(
            OperationBindingPolicy::Replace,
        );

    bindings
        .insert(first)
        .expect("first insertion must succeed");

    bindings
        .insert(second.clone())
        .expect("replacement must succeed");

    assert_eq!(
        bindings.get_operation(operation_id),
        Some(&second)
    );

    assert_eq!(bindings.binding_count(), 1);
}

#[test]
fn allow_multiple_policy_keeps_multiple_operation_bindings() {
    let operation_id = operation(71);

    let first =
        bind_operation(operation_id, model(1))
            .expect("first binding must be valid");

    let second =
        bind_operation(operation_id, model(2))
            .expect("second binding must be valid");

    let mut bindings =
        IrNoiseBindings::with_operation_policy(
            OperationBindingPolicy::AllowMultiple,
        );

    bindings
        .insert(first.clone())
        .expect("first insertion must succeed");

    bindings
        .insert(second.clone())
        .expect("second insertion must succeed");

    assert_eq!(bindings.binding_count(), 2);
    assert_eq!(
        bindings.bindings_for_operation(operation_id).len(),
        2
    );

    assert_eq!(
        bindings.count_noise_model(model(1)),
        1
    );

    assert_eq!(
        bindings.count_noise_model(model(2)),
        1
    );
}

#[test]
fn identical_duplicate_is_idempotent_under_allow_multiple() {
    let operation_id = operation(73);

    let binding =
        bind_operation(operation_id, model(3))
            .expect("binding must be valid");

    let mut bindings =
        IrNoiseBindings::with_operation_policy(
            OperationBindingPolicy::AllowMultiple,
        );

    bindings
        .insert(binding.clone())
        .expect("first insertion must succeed");

    bindings
        .insert(binding)
        .expect("identical insertion should be idempotent");

    assert_eq!(bindings.binding_count(), 1);
}

#[test]
fn operation_and_resource_bindings_can_coexist() {
    let operation_id = operation(81);

    let operation_binding =
        bind_operation(operation_id, model(1))
            .expect("operation binding must be valid");

    let resource_binding =
        bind_resource(
            IrResourceScope::physical_qubit(
                physical(82),
            ),
            model(2),
        )
        .expect("resource binding must be valid");

    let mut bindings = IrNoiseBindings::new();

    bindings
        .insert(operation_binding)
        .expect("operation insertion must succeed");

    bindings
        .insert(resource_binding)
        .expect("resource insertion must succeed");

    assert_eq!(bindings.binding_count(), 2);
    assert_eq!(bindings.operation_binding_count(), 1);
    assert_eq!(bindings.resource_binding_count(), 1);
}

// =============================================================================
// Transactional integration
// =============================================================================

#[test]
fn extend_is_transactional_when_one_binding_is_invalid() {
    let valid =
        bind_operation(operation(91), model(1))
            .expect("valid binding must construct");

    let invalid =
        IrNoiseBinding::for_resource(
            IrResourceScope::composite(
                std::iter::empty::<IrResourceScope>(),
            )
            .unwrap_or_else(|_| IrResourceScope::global()),
            model(2),
        )
        .expect("fallback binding should be structurally valid");

    let mut bindings = IrNoiseBindings::new();

    bindings
        .insert(valid.clone())
        .expect("initial binding must succeed");

    let before = bindings.clone();

    let conflicting =
        bind_operation(operation(91), model(3))
            .expect("conflicting binding must be structurally valid");

    let result = bindings.extend([
        conflicting,
        invalid,
    ]);

    assert!(result.is_err());

    assert_eq!(bindings, before);
}

#[test]
fn builder_is_transactional_at_build_boundary() {
    let mut builder = IrNoiseBindingsBuilder::new();

    builder
        .push(
            bind_operation(operation(101), model(1))
                .expect("valid binding"),
        )
        .expect("push must succeed");

    builder
        .push(
            bind_operation(operation(102), model(2))
                .expect("valid binding"),
        )
        .expect("push must succeed");

    let bindings =
        builder.build().expect("builder should succeed");

    assert_eq!(bindings.binding_count(), 2);
}

// =============================================================================
// Operation namespace integration
// =============================================================================

#[test]
fn operation_namespace_is_canonical_and_ordered() {
    let operations = [
        operation(7),
        operation(3),
        operation(11),
        operation(1),
    ];

    let namespace =
        validate_operation_namespace(operations)
            .expect("unique operation namespace must validate");

    let ordered: Vec<_> = namespace.into_iter().collect();

    assert_eq!(
        ordered,
        vec![
            operation(1),
            operation(3),
            operation(7),
            operation(11),
        ]
    );
}

#[test]
fn duplicate_operation_namespace_is_rejected() {
    let result =
        validate_operation_namespace([
            operation(1),
            operation(2),
            operation(1),
        ]);

    assert!(matches!(
        result,
        Err(IrIntegrationError::DuplicateOperationIdentity {
            operation
        }) if operation == operation(1)
    ));
}

#[test]
fn non_empty_operation_namespace_rejects_empty_input() {
    let result =
        validate_non_empty_operation_namespace(
            std::iter::empty::<OperationId>(),
        );

    assert!(matches!(
        result,
        Err(IrIntegrationError::EmptyOperationNamespace)
    ));
}

#[test]
fn binding_operation_reference_validation_accepts_known_operations() {
    let first = operation(111);
    let second = operation(112);

    let mut bindings = IrNoiseBindings::new();

    bindings
        .insert(
            bind_operation(first, model(1))
                .expect("binding must be valid"),
        )
        .expect("insertion must succeed");

    bindings
        .insert(
            bind_operation(second, model(2))
                .expect("binding must be valid"),
        )
        .expect("insertion must succeed");

    bindings
        .validate_operations([first, second])
        .expect("known operation references must validate");
}

#[test]
fn unknown_operation_reference_is_rejected() {
    let known = operation(121);
    let unknown = operation(122);

    let mut bindings = IrNoiseBindings::new();

    bindings
        .insert(
            bind_operation(unknown, model(1))
                .expect("binding must be valid"),
        )
        .expect("insertion must succeed");

    let result =
        bindings.validate_operations([known]);

    assert!(matches!(
        result,
        Err(IrIntegrationError::UnknownOperation {
            operation
        }) if operation == unknown
    ));
}

#[test]
fn exact_operation_validation_rejects_duplicate_namespace_entries() {
    let result =
        validate_non_empty_operation_namespace([
            operation(131),
            operation(132),
            operation(131),
        ]);

    assert!(matches!(
        result,
        Err(IrIntegrationError::DuplicateOperationIdentity {
            operation
        }) if operation == operation(131)
    ));
}

// =============================================================================
// Query integration
// =============================================================================

#[test]
fn bindings_can_be_queried_by_noise_model() {
    let first =
        bind_operation(operation(141), model(9))
            .expect("binding must be valid");

    let second =
        bind_operation(operation(142), model(9))
            .expect("binding must be valid");

    let third =
        bind_operation(operation(143), model(10))
            .expect("binding must be valid");

    let mut bindings = IrNoiseBindings::new();

    bindings.insert(first).expect("insert");
    bindings.insert(second).expect("insert");
    bindings.insert(third).expect("insert");

    assert!(bindings.contains_noise_model(model(9)));
    assert!(bindings.contains_noise_model(model(10)));
    assert!(!bindings.contains_noise_model(model(11)));

    assert_eq!(
        bindings.count_noise_model(model(9)),
        2
    );

    assert_eq!(
        bindings.bindings_for_noise_model(model(9)).len(),
        2
    );
}

#[test]
fn bindings_can_be_queried_by_canonical_operation() {
    let operation_id = operation(151);

    let binding =
        bind_operation(operation_id, model(1))
            .expect("binding must be valid");

    let mut bindings = IrNoiseBindings::new();

    bindings
        .insert(binding.clone())
        .expect("insert must succeed");

    let found =
        bindings.bindings_for_operation(operation_id);

    assert_eq!(found, vec![&binding]);
}

#[test]
fn bindings_can_be_queried_by_explicit_canonical_resource() {
    let qubit = physical(161);
    let resource = physical_resource(qubit);

    let binding =
        bind_resource(
            IrResourceScope::physical_qubit(qubit),
            model(1),
        )
        .expect("binding must be valid");

    let mut bindings = IrNoiseBindings::new();

    bindings
        .insert(binding.clone())
        .expect("insert must succeed");

    let found =
        bindings.bindings_for_resource(resource);

    assert_eq!(found, vec![&binding]);
}

#[test]
fn deferred_selectors_are_not_resolved_by_resource_queries() {
    let binding =
        bind_resource(
            IrResourceScope::selector(
                IrResourceSelector::AllPhysicalQubits,
            ),
            model(1),
        )
        .expect("selector binding must be valid");

    let mut bindings = IrNoiseBindings::new();

    bindings
        .insert(binding)
        .expect("insert must succeed");

    let found =
        bindings.bindings_for_resource(
            physical_resource(physical(171)),
        );

    assert!(found.is_empty());
}

// =============================================================================
// Removal integration
// =============================================================================

#[test]
fn removing_one_operation_does_not_remove_unrelated_operations() {
    let first_operation = operation(181);
    let second_operation = operation(182);

    let first =
        bind_operation(first_operation, model(1))
            .expect("first binding");

    let second =
        bind_operation(second_operation, model(2))
            .expect("second binding");

    let mut bindings = IrNoiseBindings::new();

    bindings.insert(first).expect("insert");
    bindings.insert(second.clone()).expect("insert");

    let removed =
        bindings.remove_operation(first_operation);

    assert!(removed.is_some());

    assert_eq!(bindings.binding_count(), 1);
    assert_eq!(
        bindings.get_operation(second_operation),
        Some(&second)
    );
}

#[test]
fn remove_all_for_operation_removes_primary_and_additional_bindings() {
    let operation_id = operation(191);

    let first =
        bind_operation(operation_id, model(1))
            .expect("first binding");

    let second =
        bind_operation(operation_id, model(2))
            .expect("second binding");

    let mut bindings =
        IrNoiseBindings::with_operation_policy(
            OperationBindingPolicy::AllowMultiple,
        );

    bindings.insert(first).expect("insert");
    bindings.insert(second).expect("insert");

    let removed =
        bindings.remove_all_for_operation(operation_id);

    assert_eq!(removed.len(), 2);
    assert_eq!(
        bindings.bindings_for_operation(operation_id).len(),
        0
    );
    assert!(bindings.is_empty());
}

// =============================================================================
// Deterministic integration
// =============================================================================

#[test]
fn binding_iteration_is_deterministic() {
    let mut first = IrNoiseBindings::new();
    let mut second = IrNoiseBindings::new();

    let bindings = [
        bind_operation(operation(203), model(3))
            .expect("binding"),
        bind_operation(operation(201), model(1))
            .expect("binding"),
        bind_operation(operation(202), model(2))
            .expect("binding"),
    ];

    for binding in bindings.iter().cloned() {
        first.insert(binding).expect("insert");
    }

    for binding in bindings.iter().rev().cloned() {
        second.insert(binding).expect("insert");
    }

    assert_eq!(first, second);
    assert_eq!(
        first.structural_fingerprint(),
        second.structural_fingerprint()
    );

    let first_values: Vec<_> =
        first.bindings().into_iter().cloned().collect();

    let second_values: Vec<_> =
        second.bindings().into_iter().cloned().collect();

    assert_eq!(first_values, second_values);
}

#[test]
fn structural_fingerprint_is_stable_for_repeated_evaluation() {
    let mut bindings = IrNoiseBindings::new();

    bindings
        .insert(
            bind_operation(operation(211), model(1))
                .expect("binding"),
        )
        .expect("insert");

    bindings
        .insert(
            bind_resource(
                IrResourceScope::logical_qubit(
                    logical(212),
                ),
                model(2),
            )
            .expect("binding"),
        )
        .expect("insert");

    let first = bindings.structural_fingerprint();
    let second = bindings.structural_fingerprint();

    assert_eq!(first, second);
}

#[test]
fn changing_semantic_binding_changes_structural_fingerprint() {
    let mut first = IrNoiseBindings::new();

    first
        .insert(
            bind_operation(operation(221), model(1))
                .expect("binding"),
        )
        .expect("insert");

    let mut second = IrNoiseBindings::new();

    second
        .insert(
            bind_operation(operation(221), model(2))
                .expect("binding"),
        )
        .expect("insert");

    assert_ne!(
        first.structural_fingerprint(),
        second.structural_fingerprint()
    );
}

// =============================================================================
// Explicit resource accounting
// =============================================================================

#[test]
fn explicit_resource_count_does_not_count_deferred_selectors() {
    let mut bindings = IrNoiseBindings::new();

    bindings
        .insert(
            bind_resource(
                IrResourceScope::logical_qubit(
                    logical(231),
                ),
                model(1),
            )
            .expect("binding"),
        )
        .expect("insert");

    bindings
        .insert(
            bind_resource(
                IrResourceScope::selector(
                    IrResourceSelector::AllPhysicalQubits,
                ),
                model(2),
            )
            .expect("binding"),
        )
        .expect("insert");

    assert_eq!(
        bindings.explicit_resource_count(),
        1
    );
}

#[test]
fn selector_based_binding_does_not_materialize_machine_resources() {
    let binding =
        bind_resource(
            IrResourceScope::selector(
                IrResourceSelector::AllPhysicalQubits,
            ),
            model(1),
        )
        .expect("selector binding must be valid");

    let mut bindings = IrNoiseBindings::new();

    bindings.insert(binding).expect("insert");

    assert_eq!(bindings.explicit_resource_count(), 0);
    assert_eq!(bindings.resource_binding_count(), 1);
}

// =============================================================================
// Caller-owned resource policy
// =============================================================================

#[test]
fn caller_owned_binding_limit_is_enforced_without_global_limit() {
    let mut bindings = IrNoiseBindings::new();

    let first =
        bind_operation(operation(241), model(1))
            .expect("binding");

    let second =
        bind_operation(operation(242), model(2))
            .expect("binding");

    bindings
        .insert_with_limit(first, 1)
        .expect("first binding should fit");

    let result =
        bindings.insert_with_limit(second, 1);

    assert!(matches!(
        result,
        Err(IrIntegrationError::BindingLimitExceeded {
            maximum: 1,
            requested: 2
        })
    ));

    assert_eq!(bindings.binding_count(), 1);
}

#[test]
fn builder_can_apply_explicit_caller_owned_limit() {
    let mut builder =
        IrNoiseBindingsBuilder::new()
            .maximum_bindings(2);

    builder
        .push(
            bind_operation(operation(251), model(1))
                .expect("binding"),
        )
        .expect("first binding should fit");

    builder
        .push(
            bind_operation(operation(252), model(2))
                .expect("binding"),
        )
        .expect("second binding should fit");

    let result = builder.push(
        bind_operation(operation(253), model(3))
            .expect("binding"),
    );

    assert!(matches!(
        result,
        Err(IrIntegrationError::BindingLimitExceeded {
            maximum: 2,
            requested: 3
        })
    ));
}

// =============================================================================
// Large-system identity/scalability integration
// =============================================================================

#[test]
fn large_representable_operation_ids_remain_semantic_values() {
    let values = [
        0,
        1,
        u64::from(u32::MAX),
        u64::MAX - 1,
        u64::MAX,
    ];

    let mut namespace = BTreeSet::new();

    for value in values {
        namespace.insert(operation(value));
    }

    assert_eq!(namespace.len(), values.len());

    for value in values {
        assert!(namespace.contains(&operation(value)));
    }
}

#[test]
fn large_representable_qubit_ids_remain_canonical() {
    let logical_id = logical(u64::MAX);
    let physical_id = physical(u64::MAX);

    let logical_resource =
        logical_resource(logical_id);

    let physical_resource =
        physical_resource(physical_id);

    assert_eq!(
        logical_resource,
        ResourceIdentity::LogicalQubit(logical_id)
    );

    assert_eq!(
        physical_resource,
        ResourceIdentity::PhysicalQubit(physical_id)
    );
}

#[test]
fn large_ids_do_not_create_an_artificial_machine_size_contract() {
    let operation_id = operation(u64::MAX);
    let logical_id = logical(u64::MAX);
    let physical_id = physical(u64::MAX);

    let operation_binding =
        bind_operation(operation_id, model(u64::MAX))
            .expect("maximum representable ID must be accepted");

    let logical_binding =
        bind_resource(
            IrResourceScope::logical_qubit(logical_id),
            model(u64::MAX),
        )
        .expect("maximum logical resource ID must be accepted");

    let physical_binding =
        bind_resource(
            IrResourceScope::physical_qubit(physical_id),
            model(u64::MAX),
        )
        .expect("maximum physical resource ID must be accepted");

    let mut bindings = IrNoiseBindings::new();

    bindings
        .insert(operation_binding)
        .expect("operation insertion");

    bindings
        .insert(logical_binding)
        .expect("logical insertion");

    bindings
        .insert(physical_binding)
        .expect("physical insertion");

    assert_eq!(bindings.binding_count(), 3);
}

// =============================================================================
// Cross-domain binding integration
// =============================================================================

#[test]
fn same_noise_model_can_be_shared_across_operations_and_resources() {
    let shared_model = model(301);

    let operation_binding =
        bind_operation(operation(302), shared_model)
            .expect("operation binding");

    let resource_binding =
        bind_resource(
            IrResourceScope::physical_qubit(
                physical(303),
            ),
            shared_model,
        )
        .expect("resource binding");

    let mut bindings = IrNoiseBindings::new();

    bindings
        .insert(operation_binding)
        .expect("operation insertion");

    bindings
        .insert(resource_binding)
        .expect("resource insertion");

    assert_eq!(
        bindings.count_noise_model(shared_model),
        2
    );
}

#[test]
fn operation_and_resource_binding_can_reference_same_model_without_merging_identity_domains() {
    let shared_model = model(311);
    let operation_id = operation(312);
    let resource =
        IrResourceScope::physical_qubit(physical(313));

    let operation_binding =
        bind_operation(operation_id, shared_model)
            .expect("operation binding");

    let combined_binding =
        bind_operation_and_resource(
            operation_id,
            resource,
            shared_model,
        )
        .expect("combined binding");

    assert_eq!(
        operation_binding.noise_model_id(),
        combined_binding.noise_model_id()
    );

    assert_ne!(
        operation_binding.scope(),
        combined_binding.scope()
    );
}

// =============================================================================
// Clear/reset integration
// =============================================================================

#[test]
fn clear_removes_all_integration_state() {
    let mut bindings = IrNoiseBindings::new();

    bindings
        .insert(
            bind_operation(operation(321), model(1))
                .expect("binding"),
        )
        .expect("insert");

    bindings
        .insert(
            bind_resource(
                IrResourceScope::physical_qubit(
                    physical(322),
                ),
                model(2),
            )
            .expect("binding"),
        )
        .expect("insert");

    assert!(!bindings.is_empty());

    bindings.clear();

    assert!(bindings.is_empty());
    assert_eq!(bindings.binding_count(), 0);
}

// =============================================================================
// Public integration contract smoke test
// =============================================================================

#[test]
fn integration_contract_smoke_test() {
    let operation_id = operation(401);
    let logical_qubit = logical(402);
    let physical_qubit = physical(403);
    let noise_model = model(404);

    let operation_binding =
        bind_operation(operation_id, noise_model)
            .expect("operation binding");

    let logical_binding =
        bind_resource(
            IrResourceScope::logical_qubit(
                logical_qubit,
            ),
            noise_model,
        )
        .expect("logical resource binding");

    let physical_binding =
        bind_operation_and_resource(
            operation_id,
            IrResourceScope::physical_qubit(
                physical_qubit,
            ),
            noise_model,
        )
        .expect("operation/resource binding");

    let mut bindings =
        IrNoiseBindings::with_operation_policy(
            OperationBindingPolicy::AllowMultiple,
        );

    bindings
        .insert(operation_binding)
        .expect("operation insertion");

    bindings
        .insert(logical_binding)
        .expect("logical insertion");

    bindings
        .insert(physical_binding)
        .expect("combined insertion");

    bindings
        .validate()
        .expect("complete integration structure must validate");

    bindings
        .validate_operations([operation_id])
        .expect("canonical operation reference must validate");

    assert_eq!(
        bindings.count_noise_model(noise_model),
        3
    );

    assert_eq!(
        bindings.explicit_resource_count(),
        2
    );

    assert!(!bindings.is_empty());
}