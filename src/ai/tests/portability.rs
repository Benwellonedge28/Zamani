#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani AI Portability Conformance Tests
//!
//! Path: `src/ai/tests/portability.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Test the AI subsystem's POCO-REAF portability contracts:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! These tests verify that:
//!
//! - AI computational meaning is independent of consumer identity;
//! - compatibility is evaluated using explicit version contracts;
//! - required features are checked rather than silently assumed;
//! - unsupported future versions are rejected;
//! - conversions use explicitly registered, typed adapters;
//! - conversions preserve their declared semantic fidelity;
//! - representation versions are not silently substituted;
//! - dynamic collections are not assigned language-level capacity ceilings;
//! - deterministic registry behavior does not depend on registration order.
//!
//! ## Architectural authorities
//!
//! These tests exercise existing repository implementations:
//!
//! - `crate::ai::compatibility`
//! - `crate::ai::interoperability`
//! - `crate::ai::version`
//! - `crate::zuir::version`
//!
//! Source-language portability is specified by:
//!
//! - `grammar/ai/README.md`
//! - `grammar/ai/ai.g4`
//! - `grammar/ai/tensors.g4`
//! - `grammar/ai/pipelines.g4`
//!
//! ## Ownership boundary
//!
//! This file owns tests only.
//!
//! It does not implement:
//!
//! - a second AI compatibility system;
//! - a second AI representation registry;
//! - a second tensor representation;
//! - a second AI intermediate representation;
//! - hardware discovery;
//! - resource allocation;
//! - device placement;
//! - scheduling;
//! - model execution;
//! - distributed execution;
//! - quantum execution;
//! - hybrid execution.
//!
//! The adapters and representations declared here are test fixtures.
//! They are not production hardware backends or production domain adapters.
//!
//! ## Scalability contract
//!
//! A finite test workload is a sample, not a semantic capacity limit.
//!
//! These tests must never introduce `MAX_*` constants or assert that
//! a particular model size, tensor rank, operation count, device count,
//! worker count, or memory capacity is a universal language limit.
//!
//! Actual feasibility belongs to the compiler, target capability,
//! resource planning, and execution infrastructure.
//!
//! ## Important limitation
//!
//! Passing these tests establishes the tested portability contracts.
//! It does not establish that every Zamani backend or computational
//! domain has implemented a production-ready realization adapter.

use std::collections::BTreeSet;

use crate::ai::compatibility::{
    evaluate_compatibility,
    AiCompatibilityDescriptor,
    AiCompatibilityPolicies,
    AiCompatibilityProfile,
    AiFeatureId,
};

use crate::ai::interoperability::{
    AiAdapterDescriptor,
    AiAdapterId,
    AiConversionContext,
    AiConversionFidelity,
    AiConverted,
    AiFidelityClass,
    AiInteroperabilityRegistry,
    AiRepresentationAdapter,
    AiRepresentationId,
    AiRepresentationRef,
};

use crate::ai::version::{
    exact_ai_policy,
    AiApiVersion,
    AiArtifactFormatVersion,
    AiModelSchemaVersion,
    AiOperationSetVersion,
    AiSemanticVersion,
};

use crate::zuir::version::VersionTriple;

// ============================================================================
// TEST FIXTURE: CANONICAL LOGICAL COMPUTATION
// ============================================================================

/// A test-only description of logical AI computation.
///
/// This fixture deliberately contains no hardware identifiers,
/// physical device references, memory addresses, or placement decisions.
///
/// It is not a replacement for Zamani's canonical AST, semantic model,
/// or ZUIR representation.
#[derive(Clone, Debug, Eq, PartialEq)]
struct LogicalAiProgram {
    semantic_contract: String,
    operations: Vec<String>,
}

impl LogicalAiProgram {
    /// Construct a logical AI program fixture.
    fn new(
        semantic_contract: impl Into<String>,
        operations: impl IntoIterator<Item = String>,
    ) -> Self {
        Self {
            semantic_contract: semantic_contract.into(),
            operations: operations.into_iter().collect(),
        }
    }

    /// Validate the fixture's basic invariants.
    fn validate(&self) -> Result<(), &'static str> {
        if self.semantic_contract.trim().is_empty() {
            return Err("semantic contract must not be empty");
        }

        if self.operations.iter().any(|operation| {
            operation.trim().is_empty()
        }) {
            return Err("operation identities must not be empty");
        }

        Ok(())
    }
}

// ============================================================================
// TEST FIXTURE: CONSUMER REPRESENTATION
// ============================================================================

/// Test-only representation of a consumer's logical input.
///
/// A consumer here means a compatibility-test participant.
/// It does not represent an actual physical device or execution backend.
#[derive(Clone, Debug, Eq, PartialEq)]
struct ConsumerProgramView {
    consumer_identity: String,
    semantic_contract: String,
    operations: Vec<String>,
}

impl ConsumerProgramView {
    /// Validate the consumer representation.
    fn validate(&self) -> Result<(), &'static str> {
        if self.consumer_identity.trim().is_empty() {
            return Err("consumer identity must not be empty");
        }

        if self.semantic_contract.trim().is_empty() {
            return Err("semantic contract must not be empty");
        }

        if self.operations.iter().any(|operation| {
            operation.trim().is_empty()
        }) {
            return Err("operation identities must not be empty");
        }

        Ok(())
    }
}

// ============================================================================
// VERSION AND COMPATIBILITY FIXTURES
// ============================================================================

/// Construct the five independent AI version policies.
///
/// Each version domain uses the existing AI version authority.
fn exact_compatibility_policies() -> AiCompatibilityPolicies {
    AiCompatibilityPolicies::new(
        exact_ai_policy(AiApiVersion::new(1, 0, 0)),
        exact_ai_policy(AiSemanticVersion::new(1, 0, 0)),
        exact_ai_policy(AiModelSchemaVersion::new(1, 0, 0)),
        exact_ai_policy(AiOperationSetVersion::new(1, 0, 0)),
        exact_ai_policy(AiArtifactFormatVersion::new(1, 0, 0)),
    )
}

/// Construct an AI compatibility descriptor.
fn compatibility_descriptor() -> AiCompatibilityDescriptor {
    AiCompatibilityDescriptor::new(
        AiApiVersion::new(1, 0, 0),
        AiSemanticVersion::new(1, 0, 0),
        AiModelSchemaVersion::new(1, 0, 0),
        AiOperationSetVersion::new(1, 0, 0),
        AiArtifactFormatVersion::new(1, 0, 0),
    )
}

/// Construct a consumer compatibility profile.
fn consumer_profile(
    identity: &str,
    supported_features: impl IntoIterator<Item = AiFeatureId>,
) -> AiCompatibilityProfile {
    let mut profile = AiCompatibilityProfile::new(
        identity,
        exact_compatibility_policies(),
    )
    .expect("the test compatibility policy is valid");

    profile.supported_features =
        supported_features.into_iter().collect();

    profile
}

/// Construct a validated AI feature identity.
fn feature(
    namespace: &str,
    name: &str,
) -> AiFeatureId {
    AiFeatureId::new(namespace, name)
        .expect("the test feature identity is valid")
}

// ============================================================================
// INTEROPERABILITY FIXTURES
// ============================================================================

fn logical_program_representation() -> AiRepresentationRef {
    AiRepresentationRef::new(
        AiRepresentationId::new(
            "zamani.ai",
            "logical_program",
        )
        .expect("valid source representation"),
        VersionTriple::new(1, 0, 0),
    )
}

fn consumer_view_representation() -> AiRepresentationRef {
    AiRepresentationRef::new(
        AiRepresentationId::new(
            "zamani.ai.tests",
            "consumer_program_view",
        )
        .expect("valid destination representation"),
        VersionTriple::new(1, 0, 0),
    )
}

/// Test adapter that preserves the logical computation description.
///
/// This adapter deliberately performs no hardware lowering.
/// It tests the production interoperability contract using test-owned
/// source and destination fixtures.
struct LogicalProgramTestAdapter {
    descriptor: AiAdapterDescriptor,
    consumer_identity: String,
}

impl LogicalProgramTestAdapter {
    fn new(
        adapter_name: &str,
        consumer_identity: &str,
    ) -> Self {
        let descriptor = AiAdapterDescriptor::new(
            AiAdapterId::new(
                "zamani.ai.tests",
                adapter_name,
            )
            .expect("valid test adapter identity"),
            logical_program_representation(),
            consumer_view_representation(),
            VersionTriple::new(1, 0, 0),
            [AiFidelityClass::Exact],
        )
        .expect("valid test adapter descriptor");

        Self {
            descriptor,
            consumer_identity: consumer_identity.to_owned(),
        }
    }
}

impl AiRepresentationAdapter for LogicalProgramTestAdapter {
    type Source = LogicalAiProgram;
    type Destination = ConsumerProgramView;

    fn descriptor(&self) -> &AiAdapterDescriptor {
        &self.descriptor
    }

    fn convert(
        &self,
        source: &Self::Source,
        _context: &AiConversionContext,
    ) -> crate::ai::error::AiResult<(
        Self::Destination,
        crate::ai::interoperability::AiConversionReport,
    )> {
        use crate::ai::interoperability::AiConversionReport;

        source.validate().map_err(|message| {
            crate::ai::error::AiError::new(
                crate::ai::error::AiErrorKind::InvalidInput,
                message,
            )
        })?;

        let destination = ConsumerProgramView {
            consumer_identity: self.consumer_identity.clone(),
            semantic_contract: source.semantic_contract.clone(),
            operations: source.operations.clone(),
        };

        let report = AiConversionReport::new(
            self.descriptor.identity().clone(),
            self.descriptor.source().clone(),
            self.descriptor.destination().clone(),
            AiConversionFidelity::Exact,
        )?;

        Ok((destination, report))
    }
}

/// Register a test-only logical-preservation adapter.
fn test_registry(
    adapter_name: &str,
    consumer_identity: &str,
) -> AiInteroperabilityRegistry {
    let mut registry = AiInteroperabilityRegistry::new();

    registry
        .register(LogicalProgramTestAdapter::new(
            adapter_name,
            consumer_identity,
        ))
        .expect("test adapter registration succeeds");

    registry
}

/// Perform a test-only logical conversion through the actual registry.
fn convert_for_consumer(
    registry: &AiInteroperabilityRegistry,
    adapter_name: &str,
    source: &LogicalAiProgram,
) -> AiConverted<ConsumerProgramView> {
    let adapter_identity = AiAdapterId::new(
        "zamani.ai.tests",
        adapter_name,
    )
    .expect("valid test adapter identity");

    registry
        .convert(
            &adapter_identity,
            &logical_program_representation(),
            &consumer_view_representation(),
            source,
            &AiConversionContext::exact_only(),
        )
        .expect("exact logical conversion succeeds")
}

// ============================================================================
// TESTS: LOGICAL PORTABILITY
// ============================================================================

/// A logical AI program must not acquire hardware-specific meaning
/// merely because a different consumer is evaluating compatibility.
#[test]
fn logical_program_compatibility_is_independent_of_consumer_identity() {
    let required_features = [
        feature("ai.tensor", "symbolic"),
        feature("ai.learning", "training"),
        feature("ai.reasoning", "probabilistic"),
    ];

    let descriptor = compatibility_descriptor()
        .with_required_features(required_features.clone());

    let first_consumer = consumer_profile(
        "consumer.small",
        required_features.clone(),
    );

    let second_consumer = consumer_profile(
        "consumer.large",
        required_features,
    );

    let first = evaluate_compatibility(
        &descriptor,
        &first_consumer,
    )
    .expect("first compatibility evaluation succeeds");

    let second = evaluate_compatibility(
        &descriptor,
        &second_consumer,
    )
    .expect("second compatibility evaluation succeeds");

    assert_eq!(
        first.versions,
        second.versions,
        "the same logical contract must have the same version decisions"
    );

    assert_eq!(
        first.features,
        second.features,
        "the same required features must have the same feature decisions"
    );

    assert!(
        first.is_compatible_without_migration()
    );

    assert!(
        second.is_compatible_without_migration()
    );
}

/// A consumer must not silently claim compatibility with an
/// unsupported required AI feature.
#[test]
fn unsupported_required_features_are_not_portable_by_assumption() {
    let required = feature(
        "ai.learning",
        "federated",
    );

    let descriptor = compatibility_descriptor()
        .with_required_feature(required.clone());

    let consumer = consumer_profile(
        "consumer.without.federated",
        [],
    );

    let report = evaluate_compatibility(
        &descriptor,
        &consumer,
    )
    .expect("compatibility evaluation succeeds");

    assert!(report.must_reject());

    assert!(
        report.features.unsupported.contains(&required),
        "the missing feature must be reported explicitly"
    );
}

/// Future semantic contracts must not be accepted as though they were
/// already understood by an older consumer.
#[test]
fn future_semantic_version_is_rejected() {
    let mut descriptor = compatibility_descriptor();

    descriptor.semantic_version =
        AiSemanticVersion::new(2, 0, 0);

    let consumer = consumer_profile(
        "consumer.current",
        [],
    );

    let report = evaluate_compatibility(
        &descriptor,
        &consumer,
    )
    .expect("compatibility evaluation succeeds");

    assert!(report.must_reject());

    assert_eq!(
        report.versions.semantics.supplied(),
        AiSemanticVersion::new(2, 0, 0)
    );

    assert_eq!(
        report.versions.semantics.current(),
        AiSemanticVersion::new(1, 0, 0)
    );
}

/// Compatibility must preserve the distinction between migration
/// requirements and outright incompatibility.
#[test]
fn compatibility_reports_migration_without_silently_migrating() {
    use crate::ai::version::{
        AiVersionPolicy,
        VersionRange,
    };

    let current = AiModelSchemaVersion::new(2, 0, 0);

    let supported = VersionRange::new(
        AiModelSchemaVersion::new(1, 0, 0),
        current,
    )
    .expect("valid historical version range");

    let model_policy = AiVersionPolicy::new(
        current,
        supported,
        false,
    )
    .expect("valid model schema policy");

    let mut policies = exact_compatibility_policies();
    policies.model_schema = model_policy;

    let mut descriptor = compatibility_descriptor();

    descriptor.model_schema_version =
        AiModelSchemaVersion::new(1, 5, 0);

    let consumer = AiCompatibilityProfile::new(
        "consumer.requires.migration",
        policies,
    )
    .expect("valid consumer profile");

    let report = evaluate_compatibility(
        &descriptor,
        &consumer,
    )
    .expect("compatibility evaluation succeeds");

    assert!(report.requires_migration());

    assert!(!report.is_compatible_without_migration());

    assert!(!report.must_reject());
}

// ============================================================================
// TESTS: SEMANTIC PRESERVATION THROUGH ADAPTERS
// ============================================================================

/// Exact conversion must preserve the logical operation sequence
/// and semantic contract.
#[test]
fn exact_conversion_preserves_logical_program_meaning() {
    let source = LogicalAiProgram::new(
        "zamani.ai.semantics.v1",
        [
            "tensor.matmul".to_owned(),
            "neural.activation".to_owned(),
            "model.inference".to_owned(),
        ],
    );

    let registry = test_registry(
        "logical_preservation",
        "consumer.test",
    );

    let converted = convert_for_consumer(
        &registry,
        "logical_preservation",
        &source,
    );

    assert_eq!(
        converted.value().semantic_contract,
        source.semantic_contract
    );

    assert_eq!(
        converted.value().operations,
        source.operations
    );

    assert_eq!(
        converted.report().fidelity(),
        &AiConversionFidelity::Exact
    );

    assert_eq!(
        converted.report().adapter().name(),
        "logical_preservation"
    );

    converted
        .value()
        .validate()
        .expect("converted representation remains valid");
}

/// Different consumer identities must not alter the logical
/// computation preserved by the adapter.
#[test]
fn different_consumers_receive_the_same_logical_computation() {
    let source = LogicalAiProgram::new(
        "zamani.ai.semantics.v1",
        [
            "tensor.contract".to_owned(),
            "learning.update".to_owned(),
            "reasoning.infer".to_owned(),
        ],
    );

    let first_registry = test_registry(
        "consumer_a_adapter",
        "consumer.a",
    );

    let second_registry = test_registry(
        "consumer_b_adapter",
        "consumer.b",
    );

    let first = convert_for_consumer(
        &first_registry,
        "consumer_a_adapter",
        &source,
    );

    let second = convert_for_consumer(
        &second_registry,
        "consumer_b_adapter",
        &source,
    );

    assert_eq!(
        first.value().semantic_contract,
        second.value().semantic_contract
    );

    assert_eq!(
        first.value().operations,
        second.value().operations
    );

    assert_ne!(
        first.value().consumer_identity,
        second.value().consumer_identity,
        "consumer identity is metadata, not computational meaning"
    );
}

/// Exact-only portability must not silently accept an approximate
/// conversion.
#[test]
fn exact_portability_policy_does_not_accept_approximation() {
    let approximation = AiConversionFidelity::Approximate {
        contract: "zamani.ai.numerical.approximation.v1".to_owned(),
    };

    assert!(
        !AiConversionContext::exact_only()
            .policy()
            .permits(&approximation)
    );
}

/// An explicitly contractual conversion is not automatically exact.
#[test]
fn contractual_fidelity_remains_distinct_from_exact_fidelity() {
    let contractual = AiConversionFidelity::Contractual {
        contract: "zamani.ai.numerical.contract.v1".to_owned(),
    };

    assert_ne!(
        contractual.class(),
        AiFidelityClass::Exact
    );

    assert!(
        !AiConversionContext::exact_only()
            .policy()
            .permits(&contractual)
    );
}

// ============================================================================
// TESTS: VERSIONED REPRESENTATION PORTABILITY
// ============================================================================

/// A representation version mismatch must fail instead of silently
/// substituting a different version.
#[test]
fn representation_version_mismatch_is_rejected() {
    let source = LogicalAiProgram::new(
        "zamani.ai.semantics.v1",
        ["model.inference".to_owned()],
    );

    let registry = test_registry(
        "versioned_adapter",
        "consumer.versioned",
    );

    let adapter_identity = AiAdapterId::new(
        "zamani.ai.tests",
        "versioned_adapter",
    )
    .expect("valid adapter identity");

    let incompatible_source = AiRepresentationRef::new(
        AiRepresentationId::new(
            "zamani.ai",
            "logical_program",
        )
        .expect("valid representation identity"),
        VersionTriple::new(2, 0, 0),
    );

    let result = registry.convert::<
        LogicalAiProgram,
        ConsumerProgramView,
    >(
        &adapter_identity,
        &incompatible_source,
        &consumer_view_representation(),
        &source,
        &AiConversionContext::exact_only(),
    );

    assert!(
        result.is_err(),
        "an adapter must not silently substitute its registered source version"
    );
}

/// A missing adapter must fail explicitly rather than selecting an
/// unrelated registered conversion.
#[test]
fn missing_adapter_is_not_replaced_by_an_implicit_fallback() {
    let source = LogicalAiProgram::new(
        "zamani.ai.semantics.v1",
        ["model.inference".to_owned()],
    );

    let registry = test_registry(
        "registered_adapter",
        "consumer.test",
    );

    let missing_identity = AiAdapterId::new(
        "zamani.ai.tests",
        "unregistered_adapter",
    )
    .expect("valid adapter identity");

    let result = registry.convert::<
        LogicalAiProgram,
        ConsumerProgramView,
    >(
        &missing_identity,
        &logical_program_representation(),
        &consumer_view_representation(),
        &source,
        &AiConversionContext::exact_only(),
    );

    assert!(result.is_err());
}

// ============================================================================
// TESTS: DYNAMIC WORKLOADS AND SCALABILITY
// ============================================================================

/// Dynamic operation collections must preserve all supplied operations.
///
/// The tested collection sizes are test samples only. They do not define
/// a maximum model size, operation count, graph depth, or language limit.
#[test]
fn dynamically_sized_logical_programs_preserve_all_operations() {
    let operation_counts = [
        0_usize,
        1,
        2,
        17,
        257,
        4097,
    ];

    for operation_count in operation_counts {
        let operations = (0..operation_count)
            .map(|index| format!("ai.operation.{index}"));

        let source = LogicalAiProgram::new(
            "zamani.ai.semantics.v1",
            operations,
        );

        source
            .validate()
            .expect("generated logical program is valid");

        let registry = test_registry(
            "dynamic_program_adapter",
            "consumer.dynamic",
        );

        let converted = convert_for_consumer(
            &registry,
            "dynamic_program_adapter",
            &source,
        );

        assert_eq!(
            converted.value().operations.len(),
            operation_count
        );

        assert_eq!(
            converted.value().operations,
            source.operations
        );
    }
}

/// The test fixture must not confuse an empty workload with an invalid
/// workload. Whether an empty program is semantically valid is determined
/// by the actual language and semantic contracts; this fixture only checks
/// that its representation does not impose an artificial collection limit.
#[test]
fn empty_operation_collection_is_representable() {
    let source = LogicalAiProgram::new(
        "zamani.ai.semantics.v1",
        std::iter::empty(),
    );

    assert!(source.validate().is_ok());

    assert!(source.operations.is_empty());
}

/// Large dynamic operation names must be preserved without truncation.
///
/// This tests representation behavior, not a promise that every target
/// can execute a workload of arbitrary size.
#[test]
fn operation_identity_is_not_truncated_by_the_portability_fixture() {
    let operation_name = format!(
        "ai.operation.{}",
        "x".repeat(16_384)
    );

    let source = LogicalAiProgram::new(
        "zamani.ai.semantics.v1",
        [operation_name.clone()],
    );

    let registry = test_registry(
        "long_identity_adapter",
        "consumer.long_identity",
    );

    let converted = convert_for_consumer(
        &registry,
        "long_identity_adapter",
        &source,
    );

    assert_eq!(
        converted.value().operations,
        vec![operation_name]
    );
}

// ============================================================================
// TESTS: DETERMINISM
// ============================================================================

/// Compatibility evaluation must be deterministic for identical inputs.
#[test]
fn compatibility_evaluation_is_deterministic() {
    let required_features = [
        feature("ai.tensor", "symbolic"),
        feature("ai.learning", "training"),
        feature("ai.reasoning", "probabilistic"),
    ];

    let descriptor = compatibility_descriptor()
        .with_required_features(required_features.clone());

    let consumer = consumer_profile(
        "consumer.deterministic",
        required_features,
    );

    let first = evaluate_compatibility(
        &descriptor,
        &consumer,
    )
    .expect("first evaluation succeeds");

    let second = evaluate_compatibility(
        &descriptor,
        &consumer,
    )
    .expect("second evaluation succeeds");

    assert_eq!(first, second);
}

/// Adapter enumeration must remain deterministic because the production
/// registry owns deterministic identity ordering.
#[test]
fn adapter_enumeration_is_deterministic() {
    let mut registry = AiInteroperabilityRegistry::new();

    let adapters = [
        ("adapter_c", "consumer.c"),
        ("adapter_a", "consumer.a"),
        ("adapter_b", "consumer.b"),
    ];

    for (adapter_name, consumer_identity) in adapters {
        registry
            .register(LogicalProgramTestAdapter::new(
                adapter_name,
                consumer_identity,
            ))
            .expect("unique test adapter registration succeeds");
    }

    let first: Vec<String> = registry
        .iter()
        .map(|descriptor| {
            descriptor.identity().qualified_name()
        })
        .collect();

    let second: Vec<String> = registry
        .iter()
        .map(|descriptor| {
            descriptor.identity().qualified_name()
        })
        .collect();

    assert_eq!(first, second);

    assert_eq!(
        first,
        vec![
            "zamani.ai.tests::adapter_a",
            "zamani.ai.tests::adapter_b",
            "zamani.ai.tests::adapter_c",
        ]
    );
}

/// Duplicate feature declarations must not create nondeterministic
/// compatibility results.
#[test]
fn duplicate_required_features_are_normalized_deterministically() {
    let repeated = feature("ai.tensor", "symbolic");

    let descriptor = compatibility_descriptor()
        .with_required_features([
            repeated.clone(),
            repeated.clone(),
            repeated.clone(),
        ]);

    let consumer = consumer_profile(
        "consumer.feature_set",
        [repeated.clone()],
    );

    let report = evaluate_compatibility(
        &descriptor,
        &consumer,
    )
    .expect("compatibility evaluation succeeds");

    assert!(report.features.is_compatible());

    assert_eq!(
        report.features.supported,
        BTreeSet::from([repeated])
    );
}

// ============================================================================
// TESTS: VALIDATION AND FAILURE TRANSPARENCY
// ============================================================================

/// Invalid logical input must be rejected by the adapter rather than
/// converted into a successful consumer representation.
#[test]
fn invalid_logical_program_is_rejected_by_adapter() {
    let source = LogicalAiProgram::new(
        "zamani.ai.semantics.v1",
        ["".to_owned()],
    );

    let registry = test_registry(
        "invalid_program_adapter",
        "consumer.invalid",
    );

    let adapter_identity = AiAdapterId::new(
        "zamani.ai.tests",
        "invalid_program_adapter",
    )
    .expect("valid adapter identity");

    let result = registry.convert::<
        LogicalAiProgram,
        ConsumerProgramView,
    >(
        &adapter_identity,
        &logical_program_representation(),
        &consumer_view_representation(),
        &source,
        &AiConversionContext::exact_only(),
    );

    assert!(
        result.is_err(),
        "invalid input must not be reported as a successful conversion"
    );
}

/// A valid consumer representation must retain its own validation
/// invariants after conversion.
#[test]
fn converted_consumer_representation_is_validated() {
    let source = LogicalAiProgram::new(
        "zamani.ai.semantics.v1",
        [
            "tensor.matmul".to_owned(),
            "model.inference".to_owned(),
        ],
    );

    let registry = test_registry(
        "validated_consumer_adapter",
        "consumer.validated",
    );

    let converted = convert_for_consumer(
        &registry,
        "validated_consumer_adapter",
        &source,
    );

    assert!(
        converted.value().validate().is_ok()
    );
}