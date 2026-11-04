
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI — Public API Integration Tests
//!
//! Path: src/ai/tests/api.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! Safety:
//!     Safe Rust only. Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Verify the externally observable contracts of the canonical Zamani AI API.
//!
//! This module tests the public API through its public interfaces rather than
//! accessing private implementation details.
//!
//! The API is tested as a consumer would use it.
//!
//! ============================================================================
//! ARCHITECTURAL AUTHORITIES
//! ============================================================================
//!
//! Public API:
//!     crate::ai::api
//!
//! AI context:
//!     crate::ai::context
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
//! Canonical AI semantic types:
//!     crate::frontend::semantic::domains::ai
//!
//! AI source grammar:
//!     grammar/ai/ai.g4
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
//! - Public API conformance tests.
//! - Public builder contract tests.
//! - Public context integration tests.
//! - Operation registration and resolution tests.
//! - Registry snapshot and immutability tests.
//! - Version compatibility boundary tests.
//! - Error propagation tests.
//! - Configuration validation tests.
//! - Open-world extensibility tests.
//! - POCO-REAF architectural boundary tests.
//!
//! This file does not own:
//!
//! - AI implementation algorithms.
//! - Model execution.
//! - Tensor storage.
//! - Semantic analysis.
//! - AST construction.
//! - Grammar parsing.
//! - ZUIR operations.
//! - Hardware discovery.
//! - Device allocation.
//! - Scheduling.
//! - Distributed execution.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! Tests must not encode language-level ceilings for:
//!
//! - Model count.
//! - Operation count.
//! - Parameter count.
//! - Tensor rank.
//! - Dataset size.
//! - Agent count.
//! - Worker count.
//! - Accelerator count.
//! - Memory capacity.
//! - Hardware topology.
//!
//! Test fixture sizes are workloads, not architectural limits.
//!
//! Literal infinite computation is not promised.
//!
//! The API must preserve computational intent independently of the
//! resources available to a particular execution realization.
//!
//! ============================================================================
//! INTEGRATION
//! ============================================================================
//!
//! These tests consume:
//!
//!     AiApi
//!     AiApiBuilder
//!     ApiError
//!     ApiResult
//!     OperationDescriptor
//!     OperationName
//!     OperationVersion
//!     OperationRegistry
//!     Context
//!     VersionSet
//!
//! They must not depend on private API implementation details.
//!
//! ============================================================================

use std::error::Error;
use std::sync::Arc;

use crate::ai::api::{
    AiApi,
    AiApiBuilder,
    AiExtensionPolicy,
    AiOperationSetVersion,
    AiApiVersion,
    AiSemanticVersion,
    AiModelSchemaVersion,
    AiArtifactFormatVersion,
    ApiError,
    ApiResult,
    Context,
    OperationDescriptor,
    OperationKind,
    OperationName,
    OperationRegistry,
    OperationVersion,
    VersionSet,
};

use crate::ai::config::AiConfig;

use crate::ai::registry::AiRegistryError;

// ============================================================================
// TEST FIXTURES
// ============================================================================

/// Construct a canonical AI operation name.
///
/// This uses the existing frontend semantic identity.
fn operation_name() -> OperationName {
    OperationName::new(
        "zamani.ai",
        "inference",
    )
    .expect("test operation name must be valid")
}

/// Construct an operation name for an independently identified operation.
fn named_operation(
    name: impl Into<String>,
) -> OperationName {
    OperationName::new(
        "zamani.ai",
        name,
    )
    .expect("test operation name must be valid")
}

/// Construct the initial operation-set version.
fn operation_version() -> OperationVersion {
    OperationVersion::new(1, 0, 0)
}

/// Construct a different operation-set version.
fn next_operation_version() -> OperationVersion {
    OperationVersion::new(2, 0, 0)
}

/// Construct a valid inference operation descriptor.
fn operation_descriptor() -> OperationDescriptor {
    OperationDescriptor::new(
        operation_name(),
        OperationKind::Inference,
        operation_version(),
    )
    .expect("test operation descriptor must be valid")
}

/// Construct an inference descriptor with a supplied name and version.
fn named_descriptor(
    name: OperationName,
    version: OperationVersion,
) -> OperationDescriptor {
    OperationDescriptor::new(
        name,
        OperationKind::Inference,
        version,
    )
    .expect("test operation descriptor must be valid")
}

/// Construct a public API using the canonical builder.
fn empty_api() -> AiApi<'static> {
    AiApi::new()
        .expect("default AI API construction must succeed")
}

/// Construct an API with one registered operation.
fn registered_api() -> AiApi<'static> {
    let mut builder = AiApiBuilder::new();

    builder
        .register_operation(operation_descriptor())
        .expect("operation registration must succeed");

    builder
        .build()
        .expect("API construction must succeed")
}

// ============================================================================
// 1. PUBLIC CONSTRUCTION CONTRACTS
// ============================================================================

#[test]
fn api_constructs_with_default_configuration() {
    let api = AiApi::new()
        .expect("default API construction");

    assert!(api.operations_are_empty());
    assert_eq!(api.operation_count(), 0);
}

#[test]
fn api_constructs_with_explicit_configuration() {
    let config = AiConfig::default();

    let api = AiApi::with_config(config.clone())
        .expect("explicit configuration");

    assert_eq!(
        api.config(),
        &config,
    );
}

#[test]
fn api_builder_constructs_default_api() {
    let api = AiApiBuilder::new()
        .build()
        .expect("builder construction");

    assert!(api.operations_are_empty());
}

#[test]
fn api_builder_default_matches_new() {
    let first = AiApi::new()
        .expect("default construction");

    let second = AiApiBuilder::default()
        .build()
        .expect("default builder construction");

    assert_eq!(
        first.versions(),
        second.versions(),
    );

    assert_eq!(
        first.operation_count(),
        second.operation_count(),
    );

    assert_eq!(
        first.config(),
        second.config(),
    );
}

#[test]
fn api_from_existing_context_preserves_dependencies() {
    let context = Context::new()
        .expect("valid AI context");

    let api = AiApi::from_context(context);

    assert!(api.operations_are_empty());

    assert_eq!(
        api.config(),
        api.context().config(),
    );

    assert_eq!(
        api.versions(),
        api.context().versions(),
    );
}

#[test]
fn api_exposes_its_context() {
    let api = empty_api();

    assert!(
        api.context().has_no_registered_operations()
    );

    assert!(
        api.context().has_no_explicit_budgets()
    );
}

// ============================================================================
// 2. CONFIGURATION VALIDATION
// ============================================================================

#[test]
fn valid_configuration_is_preserved() {
    let config = AiConfig::default();

    let api = AiApi::with_config(config.clone())
        .expect("valid configuration");

    assert_eq!(api.config(), &config);
}

#[test]
fn invalid_configuration_is_not_accepted() {
    let invalid_config = AiConfig::default()
        .with_extensions(
            AiExtensionPolicy::explicit()
                .with_extension(" "),
        );

    let result = AiApi::with_config(invalid_config);

    assert!(
        matches!(
            result,
            Err(ApiError::Context(_))
        ),
        "invalid configuration must preserve its context error"
    );
}

#[test]
fn configuration_is_preserved_by_builder_reconstruction() {
    let config = AiConfig::default();

    let original = AiApi::with_config(config.clone())
        .expect("valid API");

    let rebuilt = original
        .to_builder()
        .build()
        .expect("rebuilt API");

    assert_eq!(
        rebuilt.config(),
        &config,
    );
}

// ============================================================================
// 3. VERSION CONTRACTS
// ============================================================================

#[test]
fn initial_version_set_is_available() {
    let api = empty_api();

    assert_eq!(
        api.versions(),
        VersionSet::initial(),
    );
}

#[test]
fn custom_version_set_is_preserved() {
    let versions = VersionSet::new(
        AiApiVersion::new(2, 0, 0),
        AiSemanticVersion::new(3, 0, 0),
        AiModelSchemaVersion::new(4, 0, 0),
        AiOperationSetVersion::new(5, 0, 0),
        AiArtifactFormatVersion::new(6, 0, 0),
    );

    let api = AiApiBuilder::new()
        .versions(versions)
        .build()
        .expect("custom version API");

    assert_eq!(
        api.versions(),
        versions,
    );
}

#[test]
fn operation_version_domains_remain_explicit() {
    let version = OperationVersion::new(
        7,
        2,
        1,
    );

    let descriptor = OperationDescriptor::new(
        operation_name(),
        OperationKind::Inference,
        version,
    )
    .expect("valid descriptor");

    assert_eq!(
        descriptor.version(),
        version,
    );
}

#[test]
fn different_operation_versions_can_coexist() {
    let mut builder = AiApiBuilder::new();

    let first = named_descriptor(
        operation_name(),
        operation_version(),
    );

    let second = named_descriptor(
        operation_name(),
        next_operation_version(),
    );

    builder
        .register_operations([
            first,
            second,
        ])
        .expect("different versions must coexist");

    assert_eq!(
        builder.operation_count(),
        2,
    );
}

// ============================================================================
// 4. OPERATION REGISTRATION
// ============================================================================

#[test]
fn single_operation_registration_succeeds() {
    let mut builder = AiApiBuilder::new();

    let result = builder.register_operation(
        operation_descriptor(),
    );

    assert!(result.is_ok());

    assert_eq!(
        builder.operation_count(),
        1,
    );
}

#[test]
fn registered_operation_is_visible_after_build() {
    let api = registered_api();

    assert_eq!(
        api.operation_count(),
        1,
    );

    assert!(
        api.has_operation(
            &operation_name(),
            operation_version(),
        )
    );
}

#[test]
fn operation_descriptor_is_preserved() {
    let descriptor = operation_descriptor();

    let expected_kind = descriptor.kind().clone();

    let mut builder = AiApiBuilder::new();

    builder
        .register_operation(descriptor)
        .expect("registration");

    let api = builder
        .build()
        .expect("construction");

    let actual = api
        .resolve_operation(
            &operation_name(),
            operation_version(),
        )
        .expect("operation resolution");

    assert_eq!(
        actual.kind(),
        &expected_kind,
    );
}

#[test]
fn operation_metadata_is_preserved() {
    let descriptor = operation_descriptor()
        .with_schema_revision("schema-v1")
        .expect("valid schema revision")
        .with_description("AI inference operation")
        .expect("valid description");

    let mut builder = AiApiBuilder::new();

    builder
        .register_operation(descriptor)
        .expect("registration");

    let api = builder
        .build()
        .expect("construction");

    let actual = api
        .resolve_operation(
            &operation_name(),
            operation_version(),
        )
        .expect("resolution");

    assert_eq!(
        actual.schema_revision(),
        Some("schema-v1"),
    );

    assert_eq!(
        actual.description(),
        Some("AI inference operation"),
    );
}

#[test]
fn duplicate_exact_registration_is_rejected() {
    let mut builder = AiApiBuilder::new();

    builder
        .register_operation(operation_descriptor())
        .expect("first registration");

    let result = builder.register_operation(
        operation_descriptor(),
    );

    assert!(
        matches!(
            result,
            Err(AiRegistryError::DuplicateOperation { .. })
        )
    );

    assert_eq!(
        builder.operation_count(),
        1,
    );
}

#[test]
fn different_names_can_be_registered_under_same_version() {
    let mut builder = AiApiBuilder::new();

    let first = named_descriptor(
        named_operation("inference"),
        operation_version(),
    );

    let second = named_descriptor(
        named_operation("training"),
        operation_version(),
    );

    builder
        .register_operations([
            first,
            second,
        ])
        .expect("distinct operations");

    assert_eq!(
        builder.operation_count(),
        2,
    );
}

// ============================================================================
// 5. ATOMIC BATCH REGISTRATION
// ============================================================================

#[test]
fn batch_registration_accepts_multiple_operations() {
    let mut builder = AiApiBuilder::new();

    let descriptors = vec![
        named_descriptor(
            named_operation("inference"),
            operation_version(),
        ),
        named_descriptor(
            named_operation("training"),
            operation_version(),
        ),
        named_descriptor(
            named_operation("reasoning"),
            operation_version(),
        ),
    ];

    builder
        .register_operations(descriptors)
        .expect("batch registration");

    assert_eq!(
        builder.operation_count(),
        3,
    );
}

#[test]
fn failed_batch_registration_is_atomic() {
    let mut builder = AiApiBuilder::new();

    builder
        .register_operation(operation_descriptor())
        .expect("initial registration");

    let new_version = next_operation_version();

    let new_descriptor = named_descriptor(
        operation_name(),
        new_version,
    );

    let duplicate_descriptor = operation_descriptor();

    let result = builder.register_operations([
        new_descriptor,
        duplicate_descriptor,
    ]);

    assert!(
        matches!(
            result,
            Err(AiRegistryError::DuplicateOperation { .. })
        )
    );

    assert_eq!(
        builder.operation_count(),
        1,
    );

    assert!(
        !builder.registry_contains_for_test_only(
            &operation_name(),
            new_version,
        )
    );
}

// ============================================================================
// 6. EXACT OPERATION RESOLUTION
// ============================================================================

#[test]
fn exact_operation_resolution_succeeds() {
    let api = registered_api();

    let result = api.resolve_operation(
        &operation_name(),
        operation_version(),
    );

    assert!(result.is_ok());
}

#[test]
fn unknown_operation_returns_structured_error() {
    let api = empty_api();

    let result = api.resolve_operation(
        &operation_name(),
        operation_version(),
    );

    assert!(
        matches!(
            result,
            Err(ApiError::Registry(
                AiRegistryError::UnknownOperation { .. }
            ))
        )
    );
}

#[test]
fn unknown_version_is_not_silently_substituted() {
    let api = registered_api();

    let result = api.resolve_operation(
        &operation_name(),
        next_operation_version(),
    );

    assert!(
        matches!(
            result,
            Err(ApiError::Registry(
                AiRegistryError::UnknownOperation { .. }
            ))
        )
    );
}

#[test]
fn has_operation_checks_exact_identity_and_version() {
    let api = registered_api();

    assert!(
        api.has_operation(
            &operation_name(),
            operation_version(),
        )
    );

    assert!(
        !api.has_operation(
            &operation_name(),
            next_operation_version(),
        )
    );

    assert!(
        !api.has_operation(
            &named_operation("unknown_operation"),
            operation_version(),
        )
    );
}

// ============================================================================
// 7. REGISTRY SNAPSHOTS AND IMMUTABILITY
// ============================================================================

#[test]
fn public_operation_registry_is_read_only() {
    let api = registered_api();

    let registry: &OperationRegistry = api.operations();

    assert_eq!(
        registry.len(),
        1,
    );

    assert!(
        registry.contains(
            &operation_name(),
            operation_version(),
        )
    );
}

#[test]
fn context_clones_share_immutable_configuration() {
    let api = empty_api();

    let first = api.context().clone();
    let second = api.context().clone();

    assert!(
        Arc::ptr_eq(
            &first.shared_config(),
            &second.shared_config(),
        )
    );
}

#[test]
fn context_clones_share_immutable_registry() {
    let api = registered_api();

    let first = api.context().clone();
    let second = api.context().clone();

    assert!(
        Arc::ptr_eq(
            &first.shared_registry(),
            &second.shared_registry(),
        )
    );
}

#[test]
fn rebuilding_api_does_not_mutate_original_api() {
    let original = registered_api();

    let rebuilt = original
        .to_builder()
        .build()
        .expect("reconstruction");

    assert_eq!(
        original.operation_count(),
        1,
    );

    assert_eq!(
        rebuilt.operation_count(),
        1,
    );
}

#[test]
fn removing_operation_from_new_builder_preserves_old_api() {
    let original = registered_api();

    let mut builder = original.to_builder();

    let removed = builder.remove_operation(
        &operation_name(),
        operation_version(),
    );

    assert!(removed.is_some());

    let rebuilt = builder
        .build()
        .expect("reconstruction");

    assert_eq!(
        original.operation_count(),
        1,
    );

    assert_eq!(
        rebuilt.operation_count(),
        0,
    );
}

#[test]
fn existing_operation_snapshot_survives_registry_removal() {
    let original = registered_api();

    let snapshot = original
        .resolve_operation(
            &operation_name(),
            operation_version(),
        )
        .expect("operation snapshot");

    let mut builder = original.to_builder();

    builder.remove_operation(
        &operation_name(),
        operation_version(),
    );

    let rebuilt = builder
        .build()
        .expect("reconstruction");

    assert_eq!(
        rebuilt.operation_count(),
        0,
    );

    assert_eq!(
        snapshot.kind(),
        &OperationKind::Inference,
    );
}

// ============================================================================
// 8. CONTEXT AND DEPENDENCY INJECTION
// ============================================================================

#[test]
fn api_builder_accepts_existing_registry() {
    let mut registry = OperationRegistry::new();

    registry
        .register(operation_descriptor())
        .expect("registration");

    let api = AiApiBuilder::new()
        .registry(registry)
        .build()
        .expect("construction");

    assert_eq!(
        api.operation_count(),
        1,
    );
}

#[test]
fn context_builder_and_api_builder_share_version_contract() {
    let versions = VersionSet::initial();

    let context = Context::new()
        .expect("context");

    let api = AiApi::from_context(context);

    let rebuilt = api
        .to_builder()
        .versions(versions)
        .build()
        .expect("rebuilt API");

    assert_eq!(
        rebuilt.versions(),
        versions,
    );
}

// ============================================================================
// 9. ERROR PROPAGATION
// ============================================================================

#[test]
fn api_errors_implement_standard_error() {
    fn assert_error<E: Error>() {}

    assert_error::<ApiError>();
    assert_error::<AiRegistryError>();
}

#[test]
fn registry_errors_are_preserved_by_api_error() {
    let api = empty_api();

    let error = api
        .resolve_operation(
            &operation_name(),
            operation_version(),
        )
        .expect_err("operation must be absent");

    match error {
        ApiError::Registry(inner) => {
            assert!(
                matches!(
                    inner,
                    AiRegistryError::UnknownOperation { .. }
                )
            );
        }

        ApiError::Context(_) => {
            panic!("unexpected context error");
        }
    }
}

#[test]
fn api_error_exposes_original_error_source() {
    let api = empty_api();

    let error = api
        .resolve_operation(
            &operation_name(),
            operation_version(),
        )
        .expect_err("unknown operation");

    assert!(
        Error::source(&error).is_some()
    );
}

#[test]
fn api_result_is_a_standard_result_contract() {
    let result: ApiResult<AiApi<'static>> =
        AiApi::new();

    assert!(result.is_ok());
}

// ============================================================================
// 10. OPEN-WORLD EXTENSIBILITY
// ============================================================================

#[test]
fn custom_operation_names_are_supported() {
    let name = OperationName::new(
        "future.ai.domain",
        "future_computational_operation",
    )
    .expect("valid extensible operation identity");

    let descriptor = OperationDescriptor::new(
        name.clone(),
        OperationKind::Inference,
        operation_version(),
    )
    .expect("descriptor");

    let mut builder = AiApiBuilder::new();

    builder
        .register_operation(descriptor)
        .expect("registration");

    let api = builder
        .build()
        .expect("construction");

    assert!(
        api.has_operation(
            &name,
            operation_version(),
        )
    );
}

#[test]
fn long_operation_names_are_not_subject_to_api_capacity_limits() {
    let long_name = format!(
        "operation_{}",
        "extension".repeat(1024),
    );

    let name = OperationName::new(
        "future.ai",
        long_name,
    )
    .expect("valid long operation name");

    let descriptor = OperationDescriptor::new(
        name.clone(),
        OperationKind::Inference,
        operation_version(),
    )
    .expect("descriptor");

    let mut builder = AiApiBuilder::new();

    builder
        .register_operation(descriptor)
        .expect("registration");

    let api = builder
        .build()
        .expect("construction");

    assert!(
        api.has_operation(
            &name,
            operation_version(),
        )
    );
}

#[test]
fn custom_operation_versions_remain_independently_addressable() {
    let first_version = OperationVersion::new(
        1,
        2,
        3,
    );

    let second_version = OperationVersion::new(
        4,
        5,
        6,
    );

    let mut builder = AiApiBuilder::new();

    builder
        .register_operations([
            named_descriptor(
                operation_name(),
                first_version,
            ),
            named_descriptor(
                operation_name(),
                second_version,
            ),
        ])
        .expect("registration");

    let api = builder
        .build()
        .expect("construction");

    assert!(
        api.has_operation(
            &operation_name(),
            first_version,
        )
    );

    assert!(
        api.has_operation(
            &operation_name(),
            second_version,
        )
    );
}

// ============================================================================
// 11. PORTABILITY AND HARDWARE INDEPENDENCE
// ============================================================================

#[test]
fn api_construction_does_not_require_hardware_discovery() {
    let api = AiApi::new()
        .expect("API construction");

    assert_eq!(
        api.operation_count(),
        0,
    );
}

#[test]
fn api_configuration_can_represent_unbudgeted_operation_intent() {
    let api = AiApi::new()
        .expect("API construction");

    assert!(
        api.context().has_no_explicit_budgets()
    );
}

#[test]
fn api_does_not_claim_that_registration_means_execution_support() {
    let api = registered_api();

    assert!(
        api.has_operation(
            &operation_name(),
            operation_version(),
        )
    );

    // Registration establishes only that a declaration exists.
    //
    // It does not establish:
    //
    // - An executable implementation.
    // - A compatible target.
    // - Available memory.
    // - Available accelerators.
    // - Resource feasibility.
    // - Authorization.
    //
    // Those determinations belong to their respective authorities.
}

#[test]
fn api_does_not_require_a_specific_execution_target() {
    let api = registered_api();

    assert_eq!(
        api.operation_count(),
        1,
    );

    // No CPU, GPU, FPGA, ASIC, QPU, device or vendor is supplied.
    //
    // The API must remain independent of target realization.
}

// ============================================================================
// 12. BUILDER LIFECYCLE
// ============================================================================

#[test]
fn builder_can_register_then_remove_operation_before_build() {
    let mut builder = AiApiBuilder::new();

    builder
        .register_operation(operation_descriptor())
        .expect("registration");

    assert_eq!(
        builder.operation_count(),
        1,
    );

    let removed = builder.remove_operation(
        &operation_name(),
        operation_version(),
    );

    assert!(removed.is_some());

    assert_eq!(
        builder.operation_count(),
        0,
    );

    let api = builder
        .build()
        .expect("construction");

    assert!(api.operations_are_empty());
}

#[test]
fn empty_batch_registration_is_valid() {
    let mut builder = AiApiBuilder::new();

    builder
        .register_operations(
            std::iter::empty::<OperationDescriptor>(),
        )
        .expect("empty batch");

    assert_eq!(
        builder.operation_count(),
        0,
    );
}

#[test]
fn builder_can_be_reused_for_independent_api_construction() {
    let mut first_builder = AiApiBuilder::new();

    first_builder
        .register_operation(operation_descriptor())
        .expect("registration");

    let first = first_builder
        .build()
        .expect("first construction");

    let second = first
        .to_builder()
        .build()
        .expect("second construction");

    assert_eq!(
        first.operation_count(),
        second.operation_count(),
    );
}

// ============================================================================
// 13. DETERMINISTIC PUBLIC CONTRACTS
// ============================================================================

#[test]
fn repeated_resolution_returns_equivalent_declarations() {
    let api = registered_api();

    let first = api
        .resolve_operation(
            &operation_name(),
            operation_version(),
        )
        .expect("first resolution");

    let second = api
        .resolve_operation(
            &operation_name(),
            operation_version(),
        )
        .expect("second resolution");

    assert_eq!(
        first.as_ref(),
        second.as_ref(),
    );
}

#[test]
fn repeated_construction_preserves_initial_contracts() {
    let first = AiApi::new()
        .expect("first API");

    let second = AiApi::new()
        .expect("second API");

    assert_eq!(
        first.versions(),
        second.versions(),
    );

    assert_eq!(
        first.config(),
        second.config(),
    );

    assert_eq!(
        first.operation_count(),
        second.operation_count(),
    );
}

// ============================================================================
// 14. PUBLIC TYPE AND INTERFACE CONTRACTS
// ============================================================================

#[test]
fn public_operation_aliases_match_canonical_types() {
    let descriptor: OperationDescriptor =
        operation_descriptor();

    let name: OperationName =
        descriptor.name().clone();

    let version: OperationVersion =
        descriptor.version();

    assert_eq!(
        name,
        operation_name(),
    );

    assert_eq!(
        version,
        operation_version(),
    );
}

#[test]
fn public_context_alias_matches_canonical_context() {
    let context: Context<'static> =
        Context::new()
            .expect("context");

    let api = AiApi::from_context(context);

    assert!(api.operations_are_empty());
}

#[test]
fn public_registry_alias_matches_canonical_registry() {
    let mut registry: OperationRegistry =
        OperationRegistry::new();

    registry
        .register(operation_descriptor())
        .expect("registration");

    assert_eq!(
        registry.len(),
        1,
    );
}

// ============================================================================
// FINAL ARCHITECTURAL ASSERTIONS
// ============================================================================

#[test]
fn public_api_preserves_the_canonical_semantic_operation_kind() {
    let descriptor = operation_descriptor();

    assert_eq!(
        descriptor.kind(),
        &OperationKind::Inference,
    );
}

#[test]
fn public_api_does_not_create_a_parallel_operation_registry() {
    let api = registered_api();

    let registry: &OperationRegistry =
        api.operations();

    assert_eq!(
        registry.len(),
        api.operation_count(),
    );
}

#[test]
fn public_api_preserves_exact_versioned_operation_identity() {
    let api = registered_api();

    assert!(
        api.has_operation(
            &operation_name(),
            operation_version(),
        )
    );

    assert!(
        !api.has_operation(
            &operation_name(),
            OperationVersion::new(99, 0, 0),
        )
    );
}
