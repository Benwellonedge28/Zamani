#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI — Determinism and Reproducibility Tests
//!
//! Path: `src/ai/tests/determinism.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! Safety: Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Verify the deterministic and reproducibility-related contracts exposed
//! by the existing Zamani AI subsystem.
//!
//! These tests exercise the actual repository authorities:
//!
//! - AI reproducibility configuration;
//! - numerical transformation policy;
//! - AI training determinism policy;
//! - AI operation registry;
//! - canonical AI semantic names and tensor shapes;
//! - AI reproducibility provenance;
//! - canonical semantic provenance records.
//!
//! The tests are written as consumer-facing integration tests within the
//! crate. They must not depend on private implementation details.
//!
//! ============================================================================
//! ARCHITECTURAL AUTHORITIES
//! ============================================================================
//!
//! AI configuration:
//!     crate::ai::config
//!
//! AI operation registry:
//!     crate::ai::registry
//!
//! AI version contracts:
//!     crate::ai::version
//!
//! AI provenance:
//!     crate::ai::provenance
//!
//! Canonical AI semantic types:
//!     crate::frontend::semantic::domains::ai
//!
//! Canonical provenance:
//!     crate::frontend::semantic::model::provenance
//!
//! AI grammar:
//!     grammar/ai/ai.g4
//!     grammar/ai/training.g4
//!     grammar/ai/inference.g4
//!     grammar/ai/distributions.g4
//!
//! AI grammar architecture:
//!     grammar/ai/README.md
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This file owns:
//!
//! - deterministic AI configuration tests;
//! - reproducibility-policy tests;
//! - deterministic AI registry ordering tests;
//! - registry snapshot and concurrency tests;
//! - canonical shape representation repeatability tests;
//! - reproducibility provenance tests;
//! - determinism-related integration contract tests.
//!
//! This file does not own:
//!
//! - random-number generation;
//! - numerical kernels;
//! - automatic differentiation;
//! - model execution;
//! - training algorithms;
//! - tensor storage;
//! - AI semantic analysis;
//! - AST construction;
//! - universal IR;
//! - ZUIR lowering;
//! - hardware discovery;
//! - accelerator allocation;
//! - distributed scheduling;
//! - hybrid execution;
//! - runtime determinism enforcement;
//! - cryptographic fingerprints.
//!
//! ============================================================================
//! DETERMINISM CONTRACT
//! ============================================================================
//!
//! Determinism is not one universal guarantee.
//!
//! The implementation must distinguish:
//!
//! 1. Deterministic declarations and configuration.
//! 2. Deterministic registry ordering.
//! 3. Deterministic metadata construction.
//! 4. Deterministic semantic representation.
//! 5. Deterministic compilation and lowering.
//! 6. Deterministic numerical execution.
//! 7. Deterministic distributed execution.
//! 8. Deterministic replay.
//!
//! Passing a test for one category does not establish the others.
//!
//! A caller-supplied seed is not, by itself, proof of reproducibility.
//!
//! Cross-target bit-for-bit identity must only be claimed where the
//! numerical, compiler, runtime and target contracts establish it.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! No test in this file defines a language-level maximum for:
//!
//! - model count;
//! - parameter count;
//! - tensor rank;
//! - tensor dimensions;
//! - dataset size;
//! - training steps;
//! - inference requests;
//! - operation count;
//! - agent count;
//! - worker count;
//! - accelerator count;
//! - distributed nodes;
//! - memory capacity.
//!
//! Test data represents individual workloads, not architectural ceilings.
//!
//! Operational budgets remain configuration and execution concerns.
//!
//! Literal infinite memory or computation is not promised.
//!
//! ============================================================================
//! FAILURE PRINCIPLE
//! ============================================================================
//!
//! If a deterministic execution contract cannot be established, the
//! responsible compiler or runtime must report that limitation.
//!
//! It must not silently:
//!
//! - claim deterministic execution;
//! - weaken the program's numerical contract;
//! - replace a requested seed;
//! - substitute an incompatible model or operation version;
//! - discard provenance;
//! - report incomplete execution as complete.
//!
//! ============================================================================

use std::sync::Arc;
use std::thread;

use crate::ai::config::{
    AiConfig,
    AiNumericalPolicy,
    AiReproducibilityPolicy,
    AiTrainingPolicy,
    ReproducibilityMode,
    TrainingDeterminismPolicy,
};

use crate::ai::provenance::{
    AiEntityKind,
    AiProvenanceRecord,
    DeterminismClaim,
    ReproducibilityMetadata,
};

use crate::ai::registry::{
    AiOperationDescriptor,
    AiOperationRegistry,
};

use crate::ai::version::AiOperationSetVersion;

use crate::frontend::semantic::domains::ai::{
    AiOperationKind,
    DimensionExpr,
    QualifiedName,
    TensorShape,
};

use crate::frontend::semantic::model::ids::ProvenanceId;

// ============================================================================
// TEST FIXTURES
// ============================================================================

/// Construct a canonical AI operation name.
///
/// QualifiedName is owned by the canonical frontend semantic subsystem.
/// This test must not introduce another AI operation-name representation.
fn operation_name(
    name: impl Into<String>,
) -> QualifiedName {
    QualifiedName::new(
        "zamani.ai",
        name,
    )
    .expect("test operation name must be valid")
}

/// Construct an operation-set version.
///
/// The operation version belongs to the AI version authority.
fn operation_version(
    major: u32,
    minor: u32,
    patch: u32,
) -> AiOperationSetVersion {
    AiOperationSetVersion::new(
        major,
        minor,
        patch,
    )
}

/// Construct a valid operation descriptor.
///
/// The descriptor is metadata, not an executable operation.
fn operation_descriptor(
    name: impl Into<String>,
    version: AiOperationSetVersion,
) -> AiOperationDescriptor {
    AiOperationDescriptor::new(
        operation_name(name),
        AiOperationKind::Inference,
        version,
    )
    .expect("test operation descriptor must be valid")
}

/// Construct a registry from explicitly supplied descriptors.
///
/// Registration order must not determine enumeration order.
fn registry_from(
    descriptors: impl IntoIterator<Item = AiOperationDescriptor>,
) -> AiOperationRegistry {
    let mut registry = AiOperationRegistry::new();

    registry
        .register_batch(descriptors)
        .expect("test registry registration must succeed");

    registry
}

/// Return the ordered identities and versions of registered operations.
///
/// This uses the public deterministic iterator.
fn registry_snapshot(
    registry: &AiOperationRegistry,
) -> Vec<(String, AiOperationSetVersion)> {
    registry
        .iter()
        .map(|descriptor| {
            let name = format!(
                "{}::{}",
                descriptor.name().namespace,
                descriptor.name().name,
            );

            (
                name,
                descriptor.version(),
            )
        })
        .collect()
}

/// Construct canonical reproducibility metadata.
///
/// The caller supplies all metadata. This helper does not generate
/// fingerprints, timestamps, random values, or hardware information.
fn reproducibility_metadata(
    details: &[(&str, &str)],
) -> ReproducibilityMetadata {
    let mut metadata = ReproducibilityMetadata::new();

    metadata.set_determinism(
        DeterminismClaim::Deterministic,
    );

    for (key, value) in details {
        metadata
            .set_detail(*key, *value)
            .expect("test reproducibility metadata must be valid");
    }

    metadata
}

/// Construct a canonical semantic provenance record.
///
/// The same explicit identity and metadata must produce equivalent
/// semantic provenance regardless of metadata insertion order.
fn semantic_provenance(
    details: &[(&str, &str)],
) -> crate::frontend::semantic::model::provenance::ProvenanceRecord {
    let metadata = reproducibility_metadata(details);

    AiProvenanceRecord::new(
        ProvenanceId::new(1)
            .expect("test provenance identity must be valid"),
        AiEntityKind::model(),
        "determinism-test-model",
    )
    .expect("test AI provenance record must be valid")
    .with_reproducibility(metadata)
    .into_semantic_record()
    .expect("canonical provenance conversion must succeed")
}

// ============================================================================
// 1. REPRODUCIBILITY CONFIGURATION
// ============================================================================

#[test]
fn default_reproducibility_policy_preserves_declared_contract() {
    let policy = AiReproducibilityPolicy::default();

    assert_eq!(
        policy.mode,
        ReproducibilityMode::PreserveDeclaredContract,
    );
}

#[test]
fn deterministic_policy_requires_deterministic_execution() {
    let policy = AiReproducibilityPolicy::deterministic();

    assert_eq!(
        policy.mode,
        ReproducibilityMode::RequireDeterministic,
    );

    assert!(
        policy.require_provenance,
        "deterministic execution must require relevant provenance",
    );

    assert!(
        policy.record_numerical_profile,
        "deterministic execution must record relevant numerical behavior",
    );
}

#[test]
fn deterministic_policy_does_not_invent_a_random_seed() {
    let policy = AiReproducibilityPolicy::deterministic();

    assert_eq!(
        policy.seed,
        None,
        "the AI configuration must not invent a seed",
    );
}

#[test]
fn caller_supplied_seed_is_preserved_exactly() {
    let seed = 0xA5A5_1234_9876_FEDC_u128;

    let policy = AiReproducibilityPolicy::deterministic()
        .with_seed(seed);

    assert_eq!(
        policy.seed,
        Some(seed),
    );
}

#[test]
fn maximum_representable_seed_is_preserved() {
    let seed = u128::MAX;

    let policy = AiReproducibilityPolicy::deterministic()
        .with_seed(seed);

    assert_eq!(
        policy.seed,
        Some(u128::MAX),
    );
}

#[test]
fn seed_does_not_change_the_declared_reproducibility_mode() {
    let policy = AiReproducibilityPolicy::preserve_declared_contract()
        .with_seed(42);

    assert_eq!(
        policy.mode,
        ReproducibilityMode::PreserveDeclaredContract,
    );

    assert_eq!(
        policy.seed,
        Some(42),
    );
}

#[test]
fn permitting_declared_nondeterminism_is_explicit() {
    let mut policy = AiReproducibilityPolicy::default();

    policy.mode =
        ReproducibilityMode::PermitDeclaredNondeterminism;

    assert_eq!(
        policy.mode,
        ReproducibilityMode::PermitDeclaredNondeterminism,
    );
}

// ============================================================================
// 2. NUMERICAL DETERMINISM POLICY
// ============================================================================

#[test]
fn strict_numerical_policy_preserves_declared_semantics() {
    let policy = AiNumericalPolicy::strict();

    assert!(
        !policy.allow_reassociation,
        "strict policy must not permit reassociation",
    );

    assert!(
        !policy.allow_precision_relaxation,
        "strict policy must not permit precision relaxation",
    );

    assert!(
        !policy.allow_alternative_rounding,
        "strict policy must not permit alternative rounding",
    );
}

#[test]
fn contract_permitted_numerical_policy_is_distinct_from_strict_policy() {
    let strict = AiNumericalPolicy::strict();

    let contract_permitted =
        AiNumericalPolicy::contract_permitted();

    assert_ne!(
        strict,
        contract_permitted,
        "numerical policies must preserve their distinct contracts",
    );
}

#[test]
fn deterministic_configuration_preserves_numerical_policy() {
    let reproducibility =
        AiReproducibilityPolicy::deterministic()
            .with_seed(u128::MAX);

    let config = AiConfig {
        reproducibility,
        numerical: AiNumericalPolicy::strict(),
        ..AiConfig::default()
    };

    config
        .validate()
        .expect("deterministic configuration must be valid");

    assert_eq!(
        config.reproducibility.mode,
        ReproducibilityMode::RequireDeterministic,
    );

    assert_eq!(
        config.reproducibility.seed,
        Some(u128::MAX),
    );

    assert_eq!(
        config.numerical,
        AiNumericalPolicy::strict(),
    );
}

// ============================================================================
// 3. TRAINING DETERMINISM POLICY
// ============================================================================

#[test]
fn default_training_policy_follows_program_contract() {
    let policy = AiTrainingPolicy::default();

    assert_eq!(
        policy.determinism,
        TrainingDeterminismPolicy::FollowProgramContract,
    );
}

#[test]
fn deterministic_training_requirement_is_explicit() {
    let policy = AiTrainingPolicy {
        determinism: TrainingDeterminismPolicy::RequireDeterministic,
        ..AiTrainingPolicy::default()
    };

    assert_eq!(
        policy.determinism,
        TrainingDeterminismPolicy::RequireDeterministic,
    );
}

#[test]
fn training_policy_does_not_silently_permit_nondeterminism() {
    let policy = AiTrainingPolicy {
        determinism: TrainingDeterminismPolicy::RequireDeterministic,
        ..AiTrainingPolicy::default()
    };

    assert_ne!(
        policy.determinism,
        TrainingDeterminismPolicy::PermitDeclaredNondeterminism,
    );
}

// ============================================================================
// 4. CONFIGURATION AND OPERATIONAL BUDGET SEPARATION
// ============================================================================

#[test]
fn default_ai_configuration_has_no_explicit_operational_budgets() {
    let config = AiConfig::default();

    assert!(
        config.budgets.has_no_explicit_budgets(),
        "default AI configuration must not impose arbitrary budgets",
    );
}

#[test]
fn deterministic_policy_does_not_introduce_resource_ceilings() {
    let config = AiConfig {
        reproducibility: AiReproducibilityPolicy::deterministic(),
        ..AiConfig::default()
    };

    config
        .validate()
        .expect("configuration must be valid");

    assert!(
        config.budgets.has_no_explicit_budgets(),
        "determinism must not introduce semantic or resource ceilings",
    );
}

#[test]
fn reproducibility_configuration_is_repeatable() {
    let first = AiConfig {
        reproducibility:
            AiReproducibilityPolicy::deterministic()
                .with_seed(u128::MAX),
        numerical: AiNumericalPolicy::strict(),
        ..AiConfig::default()
    };

    let second = AiConfig {
        reproducibility:
            AiReproducibilityPolicy::deterministic()
                .with_seed(u128::MAX),
        numerical: AiNumericalPolicy::strict(),
        ..AiConfig::default()
    };

    assert_eq!(
        first,
        second,
        "equivalent explicit configurations must compare equally",
    );
}

// ============================================================================
// 5. DETERMINISTIC OPERATION REGISTRY ORDERING
// ============================================================================

#[test]
fn registry_iteration_is_independent_of_registration_order() {
    let version = operation_version(1, 0, 0);

    let alpha = operation_descriptor("alpha", version);
    let beta = operation_descriptor("beta", version);
    let gamma = operation_descriptor("gamma", version);

    let first = registry_from([
        alpha.clone(),
        beta.clone(),
        gamma.clone(),
    ]);

    let second = registry_from([
        gamma,
        alpha,
        beta,
    ]);

    assert_eq!(
        registry_snapshot(&first),
        registry_snapshot(&second),
        "registration order must not affect registry enumeration",
    );
}

#[test]
fn registry_iteration_uses_canonical_qualified_name_order() {
    let version = operation_version(1, 0, 0);

    let registry = registry_from([
        operation_descriptor("zeta", version),
        operation_descriptor("alpha", version),
        operation_descriptor("mu", version),
    ]);

    let names: Vec<String> = registry
        .iter()
        .map(|descriptor| {
            descriptor.name().name.clone()
        })
        .collect();

    assert_eq!(
        names,
        vec![
            "alpha".to_owned(),
            "mu".to_owned(),
            "zeta".to_owned(),
        ],
    );
}

#[test]
fn multiple_operation_versions_are_preserved() {
    let name = operation_name("inference");

    let first_version = operation_version(1, 0, 0);
    let second_version = operation_version(2, 0, 0);
    let third_version = operation_version(3, 0, 0);

    let mut registry = AiOperationRegistry::new();

    registry
        .register_batch([
            AiOperationDescriptor::new(
                name.clone(),
                AiOperationKind::Inference,
                third_version,
            )
            .expect("third version"),

            AiOperationDescriptor::new(
                name.clone(),
                AiOperationKind::Inference,
                first_version,
            )
            .expect("first version"),

            AiOperationDescriptor::new(
                name.clone(),
                AiOperationKind::Inference,
                second_version,
            )
            .expect("second version"),
        ])
        .expect("all distinct versions must coexist");

    let versions: Vec<_> = registry
        .versions_for(&name)
        .collect();

    assert_eq!(
        versions,
        vec![
            first_version,
            second_version,
            third_version,
        ],
        "operation versions must be enumerated deterministically",
    );
}

#[test]
fn exact_operation_resolution_does_not_depend_on_registration_order() {
    let name = operation_name("inference");
    let version = operation_version(1, 0, 0);

    let first = registry_from([
        operation_descriptor("inference", version),
        operation_descriptor("training", version),
    ]);

    let second = registry_from([
        operation_descriptor("training", version),
        operation_descriptor("inference", version),
    ]);

    assert!(
        first.contains(&name, version),
    );

    assert!(
        second.contains(&name, version),
    );

    assert_eq!(
        first.resolve(&name, version)
            .expect("first registry resolution")
            .as_ref(),

        second.resolve(&name, version)
            .expect("second registry resolution")
            .as_ref(),
    );
}

// ============================================================================
// 6. REGISTRY ATOMICITY AND SNAPSHOTS
// ============================================================================

#[test]
fn failed_batch_registration_does_not_change_registry_state() {
    let version = operation_version(1, 0, 0);

    let mut registry = registry_from([
        operation_descriptor("existing", version),
    ]);

    let before = registry_snapshot(&registry);

    let result = registry.register_batch([
        operation_descriptor("new-operation", version),
        operation_descriptor("existing", version),
    ]);

    assert!(
        result.is_err(),
        "duplicate registration must fail",
    );

    assert_eq!(
        registry_snapshot(&registry),
        before,
        "failed batch registration must be atomic",
    );

    assert!(
        !registry.contains(
            &operation_name("new-operation"),
            version,
        ),
        "failed registration must not publish earlier staged entries",
    );
}

#[test]
fn registry_snapshot_remains_stable_after_original_mutation() {
    let version = operation_version(1, 0, 0);

    let mut original = registry_from([
        operation_descriptor("alpha", version),
        operation_descriptor("beta", version),
    ]);

    let snapshot = original.snapshot();

    let before = registry_snapshot(&snapshot);

    original
        .register(
            operation_descriptor("gamma", version),
        )
        .expect("new registration must succeed");

    assert_eq!(
        registry_snapshot(&snapshot),
        before,
        "retained snapshots must not change after original mutation",
    );

    assert_ne!(
        registry_snapshot(&original),
        before,
        "the mutated registry must expose its new registration",
    );
}

#[test]
fn removing_registration_does_not_invalidate_retained_descriptor() {
    let version = operation_version(1, 0, 0);
    let name = operation_name("inference");

    let mut registry = registry_from([
        operation_descriptor("inference", version),
    ]);

    let retained = registry
        .resolve(&name, version)
        .expect("descriptor must resolve");

    let removed = registry.remove(&name, version);

    assert!(
        removed.is_some(),
        "registered descriptor must be removable",
    );

    assert!(
        !registry.contains(&name, version),
        "removed descriptor must no longer be registered",
    );

    assert_eq!(
        retained.name(),
        &name,
        "retained immutable descriptor must remain usable",
    );
}

// ============================================================================
// 7. CONCURRENT READ DETERMINISM
// ============================================================================

#[test]
fn concurrent_registry_readers_observe_identical_order() {
    let version = operation_version(1, 0, 0);

    let registry = Arc::new(registry_from([
        operation_descriptor("alpha", version),
        operation_descriptor("beta", version),
        operation_descriptor("gamma", version),
        operation_descriptor("omega", version),
    ]));

    let expected = registry_snapshot(&registry);

    let worker_count = thread::available_parallelism()
        .map(|parallelism| parallelism.get())
        .unwrap_or(1);

    let mut workers = Vec::new();

    for _ in 0..worker_count {
        let registry = Arc::clone(&registry);

        workers.push(thread::spawn(move || {
            registry_snapshot(&registry)
        }));
    }

    for worker in workers {
        let observed = worker
            .join()
            .expect("registry reader thread must complete");

        assert_eq!(
            observed,
            expected,
            "concurrent immutable readers must observe identical ordering",
        );
    }
}

#[test]
fn concurrent_registry_reads_do_not_mutate_registry() {
    let version = operation_version(1, 0, 0);

    let registry = Arc::new(registry_from([
        operation_descriptor("alpha", version),
        operation_descriptor("beta", version),
    ]));

    let before = registry_snapshot(&registry);

    let reader = Arc::clone(&registry);

    let handle = thread::spawn(move || {
        let _ = registry_snapshot(&reader);
    });

    handle
        .join()
        .expect("reader thread must complete");

    assert_eq!(
        registry_snapshot(&registry),
        before,
        "read-only access must not mutate registry state",
    );
}

// ============================================================================
// 8. CANONICAL SEMANTIC SHAPE REPEATABILITY
// ============================================================================

#[test]
fn equivalent_canonical_tensor_shapes_are_structurally_equal() {
    let first = TensorShape::new(vec![
        DimensionExpr::Constant(2),
        DimensionExpr::Constant(3),
        DimensionExpr::Constant(5),
    ]);

    let second = TensorShape::new(vec![
        DimensionExpr::Constant(2),
        DimensionExpr::Constant(3),
        DimensionExpr::Constant(5),
    ]);

    assert_eq!(
        first,
        second,
        "equivalent semantic shape descriptions must compare equally",
    );
}

#[test]
fn symbolic_shape_expressions_preserve_structure() {
    let first = DimensionExpr::Add(
        Box::new(DimensionExpr::Constant(7)),
        Box::new(DimensionExpr::Constant(11)),
    );

    let second = DimensionExpr::Add(
        Box::new(DimensionExpr::Constant(7)),
        Box::new(DimensionExpr::Constant(11)),
    );

    assert_eq!(
        first,
        second,
        "equivalent symbolic expressions must preserve structural identity",
    );
}

#[test]
fn dimension_representation_preserves_full_u128_values() {
    let dimension = DimensionExpr::Constant(u128::MAX);

    assert_eq!(
        dimension,
        DimensionExpr::Constant(u128::MAX),
        "the semantic representation must preserve the supplied dimension",
    );
}

#[test]
fn tensor_shape_ordering_is_structural_and_repeatable() {
    let first = TensorShape::new(vec![
        DimensionExpr::Constant(2),
        DimensionExpr::Constant(4),
    ]);

    let second = TensorShape::new(vec![
        DimensionExpr::Constant(2),
        DimensionExpr::Constant(5),
    ]);

    assert!(first < second);

    assert_eq!(
        first.cmp(&second),
        first.cmp(&second),
        "canonical ordering must be repeatable",
    );
}

// ============================================================================
// 9. REPRODUCIBILITY METADATA
// ============================================================================

#[test]
fn reproducibility_metadata_preserves_determinism_claim() {
    let metadata = reproducibility_metadata(&[
        ("compiler.version", "1.97"),
        ("model.fingerprint", "model-fingerprint-test"),
        ("source.revision", "source-revision-test"),
    ]);

    assert_eq!(
        metadata.determinism(),
        DeterminismClaim::Deterministic,
    );
}

#[test]
fn reproducibility_metadata_uses_deterministic_key_order() {
    let first = reproducibility_metadata(&[
        ("source.revision", "revision"),
        ("compiler.version", "compiler"),
        ("model.fingerprint", "model"),
    ]);

    let second = reproducibility_metadata(&[
        ("model.fingerprint", "model"),
        ("source.revision", "revision"),
        ("compiler.version", "compiler"),
    ]);

    assert_eq!(
        first,
        second,
        "metadata insertion order must not affect its value",
    );

    let first_keys: Vec<&String> =
        first.details().keys().collect();

    let second_keys: Vec<&String> =
        second.details().keys().collect();

    assert_eq!(
        first_keys,
        second_keys,
        "metadata enumeration must be deterministic",
    );
}

#[test]
fn reproducibility_metadata_rejects_invalid_keys() {
    let mut metadata = ReproducibilityMetadata::new();

    assert!(
        metadata
            .set_detail("invalid key", "value")
            .is_err(),
        "invalid metadata keys must not be accepted",
    );
}

#[test]
fn reproducibility_metadata_rejects_empty_values() {
    let mut metadata = ReproducibilityMetadata::new();

    assert!(
        metadata
            .set_detail("compiler.version", "")
            .is_err(),
        "empty reproducibility values must not be accepted",
    );
}

// ============================================================================
// 10. CANONICAL PROVENANCE INTEGRATION
// ============================================================================

#[test]
fn equivalent_reproducibility_metadata_produces_equal_provenance() {
    let first = semantic_provenance(&[
        ("compiler.version", "1.97"),
        ("model.fingerprint", "model-fingerprint"),
        ("source.revision", "source-revision"),
    ]);

    let second = semantic_provenance(&[
        ("source.revision", "source-revision"),
        ("compiler.version", "1.97"),
        ("model.fingerprint", "model-fingerprint"),
    ]);

    assert_eq!(
        first.attributes(),
        second.attributes(),
        "equivalent provenance must not depend on insertion order",
    );
}

#[test]
fn provenance_preserves_determinism_classification() {
    let record = semantic_provenance(&[
        ("compiler.version", "1.97"),
        ("model.fingerprint", "model-fingerprint"),
    ]);

    assert_eq!(
        record.attributes().get(
            "ai.reproducibility.determinism",
        ),
        Some(&"deterministic".to_owned()),
    );
}

#[test]
fn provenance_preserves_reproducibility_details() {
    let record = semantic_provenance(&[
        ("compiler.version", "1.97"),
        ("model.fingerprint", "model-fingerprint"),
        ("source.revision", "source-revision"),
    ]);

    assert_eq!(
        record.attributes().get(
            "ai.reproducibility.compiler.version",
        ),
        Some(&"1.97".to_owned()),
    );

    assert_eq!(
        record.attributes().get(
            "ai.reproducibility.model.fingerprint",
        ),
        Some(&"model-fingerprint".to_owned()),
    );

    assert_eq!(
        record.attributes().get(
            "ai.reproducibility.source.revision",
        ),
        Some(&"source-revision".to_owned()),
    );
}

#[test]
fn provenance_does_not_invent_determinism_evidence() {
    let record = AiProvenanceRecord::new(
        ProvenanceId::new(2)
            .expect("test provenance identity must be valid"),
        AiEntityKind::model(),
        "unverified-model",
    )
    .expect("test provenance record must be valid")
    .into_semantic_record()
    .expect("canonical provenance conversion must succeed");

    assert!(
        !record.attributes().contains_key(
            "ai.reproducibility.determinism",
        ),
        "unknown determinism must not be promoted to deterministic",
    );
}

// ============================================================================
// 11. DETERMINISM CONTRACT BOUNDARIES
// ============================================================================

#[test]
fn deterministic_policy_is_not_a_claim_of_cross_target_bitwise_identity() {
    let policy = AiReproducibilityPolicy::deterministic();

    assert_eq!(
        policy.mode,
        ReproducibilityMode::RequireDeterministic,
    );

    // The policy expresses a requirement. It does not itself establish
    // that a particular numerical implementation or target can satisfy it.
    //
    // The compiler/runtime must establish feasibility and report failure
    // when deterministic execution cannot be guaranteed.
    assert!(
        policy.require_provenance,
    );
}

#[test]
fn provenance_determinism_claim_is_distinct_from_execution_proof() {
    let metadata = reproducibility_metadata(&[
        ("compiler.version", "1.97"),
    ]);

    assert_eq!(
        metadata.determinism(),
        DeterminismClaim::Deterministic,
    );

    // This test verifies metadata preservation only.
    // Cryptographic verification, numerical validation, execution evidence,
    // and target attestation belong to their authoritative subsystems.
    assert!(
        metadata.details().contains_key("compiler.version"),
    );
}