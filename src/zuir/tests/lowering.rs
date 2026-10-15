
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Integration-oriented conformance tests for ZUIR lowering.
//!
//! Canonical path: `src/zuir/tests/lowering.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety policy: unsafe Rust is prohibited.
//!
//! PURPOSE
//!
//! Exercise the public lowering API across authorization, backend execution,
//! mapping integrity, error propagation, determinism, and dynamically sized
//! destination sets.
//!
//! OWNERSHIP
//!
//! Production lowering semantics belong to `lowering::api`, `context`, and
//! `legality`. This file owns test fixtures and assertions only.
//!
//! GRAMMAR BOUNDARY
//!
//! Grammar and AST parsing are not tested here. The grammar owns source syntax;
//! semantic analysis owns resolved program meaning; these tests exercise the
//! subsequent lowering contract.
//!
//! SCALABILITY
//!
//! Tests use dynamically sized collections and impose no machine-capacity
//! assumptions. Finite stress cases are regression tests, not claims of
//! infinite execution or unlimited physical resources.

use std::collections::BTreeSet;
use std::io;

use crate::zuir::lowering::api::{
    lower_with, permits_direct_lowering, BackendOutput, LoweringApiError, LoweringBackend,
};
use crate::zuir::lowering::context::{
    EntityKey, EntityKind, LoweringContext, LoweringContextError, LoweringPolicy,
};
use crate::zuir::lowering::legality::{
    DestinationSemantics, LegalityCandidate, LegalityStatus, OperationSemantics,
};

/// Construct a valid test identity through the production validation API.
fn key(kind: EntityKind, namespace: &str, value: &str) -> EntityKey {
    EntityKey::new(kind, namespace, value).expect("test identity must be valid")
}

/// Construct source and destination evidence that explicitly permits a
/// simple, effect-free conversion under the current legality contract.
fn legal_candidate(source: EntityKey) -> LegalityCandidate {
    let source_semantics = OperationSemantics::new(
        "test.source",
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::new(),
        false,
    )
    .expect("source semantics must be valid");

    let destination_semantics = DestinationSemantics::new(
        "test.destination",
        BTreeSet::new(),
        BTreeSet::new(),
        false,
        true,
    )
    .expect("destination semantics must be valid");

    LegalityCandidate::new(
        source,
        Some(source_semantics),
        Some(destination_semantics),
        BTreeSet::new(),
        Vec::new(),
        Vec::new(),
        None,
        true,
    )
    .expect("legality candidate must be structurally valid")
}

/// Construct a structurally valid candidate whose semantic evidence is
/// incomplete. Incomplete evidence must not authorize direct lowering.
fn deferred_candidate(source: EntityKey) -> LegalityCandidate {
    LegalityCandidate::new(
        source,
        None,
        None,
        BTreeSet::new(),
        Vec::new(),
        Vec::new(),
        None,
        true,
    )
    .expect("candidate with missing evidence must remain representable")
}

/// Minimal test backend with observable invocation and configurable behavior.
///
/// This fixture deliberately performs no real code generation. Its purpose is
/// to verify the public orchestration contract independently of a particular
/// destination IR implementation.
#[derive(Debug)]
struct TestBackend {
    called: bool,
    destinations: Vec<EntityKey>,
    fail: bool,
}

impl TestBackend {
    fn returning(destinations: Vec<EntityKey>) -> Self {
        Self {
            called: false,
            destinations,
            fail: false,
        }
    }

    fn failing() -> Self {
        Self {
            called: false,
            destinations: Vec::new(),
            fail: true,
        }
    }
}

impl LoweringBackend for TestBackend {
    type Artifact = String;
    type Error = io::Error;

    fn lower(
        &mut self,
        _source: &EntityKey,
        _context: &LoweringContext,
    ) -> Result<BackendOutput<Self::Artifact>, Self::Error> {
        self.called = true;

        if self.fail {
            return Err(io::Error::other("intentional test backend failure"));
        }

        Ok(BackendOutput::new(
            "test-artifact".to_owned(),
            self.destinations.clone(),
        ))
    }
}

fn context() -> LoweringContext {
    LoweringContext::new(LoweringPolicy::default())
}

#[test]
fn legal_candidate_invokes_backend_and_returns_artifact() {
    let source = key(EntityKind::Operation, "zui", "source-op");
    let destination = key(EntityKind::Operation, "classical", "destination-op");

    let mut context = context();
    let mut backend = TestBackend::returning(vec![destination.clone()]);

    let outcome = lower_with(
        &mut context,
        legal_candidate(source.clone()),
        &mut backend,
    )
    .expect("authorized conversion should succeed");

    assert!(backend.called, "legal conversion must invoke the backend");
    assert!(outcome.is_lowered());
    assert_eq!(outcome.artifact.as_deref(), Some("test-artifact"));
    assert!(permits_direct_lowering(&outcome.legality));
    assert_eq!(outcome.destinations, vec![destination.clone()]);

    let mapped = context
        .mapped_entities(&source)
        .expect("successful conversion should register its mapping");

    assert_eq!(mapped.len(), 1);
    assert!(mapped.contains(&destination));
}

#[test]
fn incomplete_semantic_evidence_never_invokes_backend() {
    let source = key(EntityKind::Operation, "zui", "unresolved-op");

    let mut context = context();
    let mut backend = TestBackend::returning(Vec::new());

    let outcome = lower_with(
        &mut context,
        deferred_candidate(source.clone()),
        &mut backend,
    )
    .expect("deferred legality should be returned as a decision");

    assert!(!backend.called);
    assert!(!outcome.is_lowered());
    assert!(outcome.artifact.is_none());
    assert!(outcome.destinations.is_empty());
    assert!(!permits_direct_lowering(&outcome.legality));
    assert!(matches!(
        outcome.legality.status,
        LegalityStatus::Deferred { .. }
    ));
    assert!(context.mapped_entities(&source).is_none());
}

#[test]
fn backend_failure_is_propagated_without_registering_mappings() {
    let source = key(EntityKind::Operation, "zui", "backend-failure-op");

    let mut context = context();
    let mut backend = TestBackend::failing();

    let result = lower_with(
        &mut context,
        legal_candidate(source.clone()),
        &mut backend,
    );

    assert!(backend.called, "a legal candidate should reach the backend");
    assert!(matches!(result, Err(LoweringApiError::Backend(_))));
    assert!(
        context.mapped_entities(&source).is_none(),
        "failed conversion must not publish destination mappings"
    );
}

#[test]
fn destination_kind_mismatch_is_rejected_atomically() {
    let source = key(EntityKind::Operation, "zui", "source-op");
    let valid_destination = key(EntityKind::Operation, "target", "valid-op");
    let invalid_destination = key(EntityKind::Value, "target", "wrong-kind-value");

    // The invalid destination sorts after the valid one by EntityKind in this
    // fixture. The production API must validate all destination kinds before
    // mutating the context, regardless of their ordering.
    let mut context = context();
    let mut backend =
        TestBackend::returning(vec![valid_destination, invalid_destination]);

    let result = lower_with(
        &mut context,
        legal_candidate(source.clone()),
        &mut backend,
    );

    assert!(matches!(
        result,
        Err(LoweringApiError::Context(
            LoweringContextError::EntityKindMismatch { .. }
        ))
    ));

    assert!(
        context.mapped_entities(&source).is_none(),
        "a rejected destination set must leave no partial mapping"
    );
}

#[test]
fn destination_results_are_sorted_and_deduplicated() {
    let source = key(EntityKind::Operation, "zui", "source-op");
    let destination_a = key(EntityKind::Operation, "target", "a");
    let destination_b = key(EntityKind::Operation, "target", "b");

    let mut context = context();
    let mut backend = TestBackend::returning(vec![
        destination_b.clone(),
        destination_a.clone(),
        destination_b.clone(),
        destination_a.clone(),
    ]);

    let outcome = lower_with(
        &mut context,
        legal_candidate(source.clone()),
        &mut backend,
    )
    .expect("valid destination identities should lower");

    assert_eq!(
        outcome.destinations,
        vec![destination_a.clone(), destination_b.clone()]
    );

    let mapped = context
        .mapped_entities(&source)
        .expect("mappings should be available");

    assert_eq!(mapped.len(), 2);
    assert!(mapped.contains(&destination_a));
    assert!(mapped.contains(&destination_b));
}

#[test]
fn empty_destination_set_is_supported() {
    let source = key(EntityKind::Operation, "zui", "elided-source-op");

    let mut context = context();
    let mut backend = TestBackend::returning(Vec::new());

    let outcome = lower_with(
        &mut context,
        legal_candidate(source.clone()),
        &mut backend,
    )
    .expect("a conversion that intentionally emits no entity may succeed");

    assert!(backend.called);
    assert!(outcome.is_lowered());
    assert!(outcome.destinations.is_empty());
    assert!(context.mapped_entities(&source).is_none());
}

#[test]
fn invalid_entity_identities_are_rejected() {
    assert!(EntityKey::new(EntityKind::Operation, "", "operation").is_err());
    assert!(EntityKey::new(EntityKind::Operation, "zui", "").is_err());
    assert!(EntityKey::new(EntityKind::Operation, "zui\0namespace", "op").is_err());
    assert!(EntityKey::new(EntityKind::Operation, "zui", "op\0identity").is_err());
    assert!(
        EntityKey::new(EntityKind::Extension(String::new()), "extension", "op").is_err()
    );
}

#[test]
fn lowering_outcome_distinguishes_authorization_from_execution() {
    let source = key(EntityKind::Operation, "zui", "source-op");

    let mut context = context();
    let mut backend = TestBackend::returning(Vec::new());

    let outcome = lower_with(
        &mut context,
        legal_candidate(source),
        &mut backend,
    )
    .expect("backend should complete");

    // Legality alone is not execution success. Both facts are asserted here
    // because they represent separate parts of the public contract.
    assert!(permits_direct_lowering(&outcome.legality));
    assert!(outcome.is_lowered());
    assert_eq!(outcome.into_artifact().as_deref(), Some("test-artifact"));
}

#[test]
fn many_destinations_are_handled_without_a_hardware_capacity_assumption() {
    let source = key(EntityKind::Operation, "zui", "one-to-many-source");

    // A finite regression workload exercises dynamic collections. This value
    // is a test workload size, not a production limit or semantic maximum.
    let destinations: Vec<EntityKey> = (0..512)
        .map(|index| {
            key(
                EntityKind::Operation,
                "test-target",
                &format!("destination-{index:06}"),
            )
        })
        .collect();

    let mut context = context();
    let mut backend = TestBackend::returning(destinations.clone());

    let outcome = lower_with(
        &mut context,
        legal_candidate(source.clone()),
        &mut backend,
    )
    .expect("all valid destination identities should be processed");

    assert!(outcome.is_lowered());
    assert_eq!(outcome.destinations.len(), destinations.len());

    let mapped = context
        .mapped_entities(&source)
        .expect("one-to-many mappings should be retained");

    assert_eq!(mapped.len(), destinations.len());
    assert!(destinations.iter().all(|destination| mapped.contains(destination)));
}

#[test]
fn repeated_lowering_with_the_same_destinations_is_idempotent() {
    let source = key(EntityKind::Operation, "zui", "stable-source");
    let destination = key(EntityKind::Operation, "target", "stable-destination");

    let mut context = context();

    for _ in 0..2 {
        let mut backend = TestBackend::returning(vec![destination.clone()]);
        let outcome = lower_with(
            &mut context,
            legal_candidate(source.clone()),
            &mut backend,
        )
        .expect("repeated identical mapping should be accepted");

        assert!(outcome.is_lowered());
        assert_eq!(outcome.destinations, vec![destination.clone()]);
    }

    let mapped = context
        .mapped_entities(&source)
        .expect("the original mapping should remain available");

    assert_eq!(mapped.len(), 1);
    assert!(mapped.contains(&destination));
}
