//! Zamani Quantum Benchmarking — Application Integration Tests
//!
//! Production integration/regression tests for:
//!
//! `crate::quantum::benchmarking::applications`
//!
//! # Purpose
//!
//! This file verifies the application-benchmark boundary as a whole rather
//! than testing the private implementation details of individual algorithms.
//!
//! It specifically protects the architectural contracts between:
//!
//! ```text
//! applications/mod.rs
//!        │
//!        ├── built-in application namespace
//!        │
//!        └── custom application API
//!                 │
//!                 ▼
//! generators/application.rs
//!                 │
//!                 ▼
//! core/workload.rs
//! ```
//!
//! The tests intentionally do NOT:
//!
//! - communicate with quantum hardware;
//! - select a vendor backend;
//! - require a simulator;
//! - depend on network access;
//! - depend on filesystem state;
//! - require nondeterministic randomness;
//! - require external credentials;
//! - benchmark real hardware;
//! - test performance through wall-clock timing;
//! - duplicate application implementation logic.
//!
//! # Why this file exists separately
//!
//! Individual application modules should test their algorithm-specific
//! mathematics and circuit construction locally.
//!
//! This file tests the cross-application contracts that must remain stable
//! when individual application implementations evolve.
//!
//! # Production guarantees tested here
//!
//! The suite verifies:
//!
//! 1. every built-in application has a stable identifier;
//! 2. built-in identifiers are unique;
//! 3. built-in identifiers follow the namespace contract;
//! 4. the custom benchmark namespace is available;
//! 5. custom application definitions validate;
//! 6. circuit applications advertise circuit generation;
//! 7. hybrid applications advertise both circuit and hybrid capabilities;
//! 8. non-circuit applications do not advertise circuit generation;
//! 9. verification metadata is internally consistent;
//! 10. invalid definitions are rejected;
//! 11. problem-size limits are enforced;
//! 12. request/application IDs cannot silently diverge;
//! 13. generated workloads are checked against their definition;
//! 14. deterministic generation remains reproducible;
//! 15. batch generation preserves sequence ordering;
//! 16. batch generation rejects zero-sized requests;
//! 17. batch generation is bounded;
//! 18. generator metadata remains stable;
//! 19. generated application identity cannot be spoofed;
//! 20. the tests remain backend-independent.
//!
//! # Integration contract
//!
//! This file expects the following modules to be wired by the authoritative
//! `benchmarking/tests/mod.rs`:
//!
//! ```text
//! #[cfg(test)]
//! pub mod application_tests;
//! ```
//!
//! The file is otherwise self-contained and requires no production-code
//! changes merely to understand its dependencies.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are used.
//!
//! # Important architectural rule
//!
//! These tests must use public APIs only.
//!
//! If an implementation change causes these tests to require access to private
//! fields, the abstraction boundary has probably been weakened and should be
//! reconsidered rather than exposing implementation details merely to satisfy
//! the test suite.

use super::super::applications::{
    builtin_application_benchmark_ids,
    is_builtin_application_benchmark,
    BERNSTEIN_VAZIRANI_ID,
    DEUTSCH_JOZSA_ID,
    HIDDEN_SHIFT_ID,
    QFT_ID,
    GROVER_ID,
    PHASE_ESTIMATION_ID,
    AMPLITUDE_ESTIMATION_ID,
    HHL_ID,
    HAMILTONIAN_ID,
    MONTE_CARLO_ID,
    QAOA_ID,
    MAXCUT_ID,
    VQE_ID,
    SHOR_ID,
    CUSTOM_ID,
};

use super::super::applications::custom::{
    circuit_definition,
    hybrid_definition,
    non_circuit_definition,
    request,
    request_with_parameters,
    CustomApplicationCategory,
    CustomApplicationDefinition,
    CustomApplicationExecutionModel,
    CustomApplicationGenerator,
    CustomApplicationTag,
    CustomApplicationVerification,
    MAX_CUSTOM_APPLICATION_DESCRIPTION_BYTES,
    MAX_CUSTOM_APPLICATION_NAME_BYTES,
    MAX_CUSTOM_APPLICATION_TAGS,
    MAX_REQUIRED_CAPABILITIES,
};

use super::super::core::workload::{
    ApplicationParameter,
    ApplicationParameterValue,
    WorkloadId,
};

use super::super::generators::application::{
    ApplicationBenchmarkGenerator,
    ApplicationGeneratorCapability,
    ApplicationGeneratorDescriptor,
    ApplicationGenerationRequest,
    MAX_GENERATED_INSTANCES,
    MAX_GENERATION_PARAMETERS,
    MAX_PROBLEM_SIZE,
};

use std::collections::HashSet;

// ============================================================================
// Test helpers
// ============================================================================

fn valid_circuit_definition() -> CustomApplicationDefinition {
    circuit_definition(
        "test_application",
        "Test Application",
        "1",
        "Production integration-test application.",
        CustomApplicationCategory::Algorithm,
        "test_application_generator",
        "1",
        128,
        CustomApplicationVerification::Exact,
        Some("test_exact_verifier".to_owned()),
    )
    .expect("valid circuit definition must construct")
}

fn valid_hybrid_definition() -> CustomApplicationDefinition {
    hybrid_definition(
        "test_hybrid_application",
        "Test Hybrid Application",
        "1",
        "Production integration-test hybrid application.",
        CustomApplicationCategory::Scientific,
        "test_hybrid_generator",
        "1",
        128,
        CustomApplicationVerification::ClassicalReference,
        Some("test_classical_reference".to_owned()),
    )
    .expect("valid hybrid definition must construct")
}

fn valid_non_circuit_definition() -> CustomApplicationDefinition {
    non_circuit_definition(
        "test_resource_application",
        "Test Resource Application",
        "1",
        "Production integration-test non-circuit application.",
        CustomApplicationCategory::ResourceEstimation,
        "test_resource_generator",
        "1",
        128,
        CustomApplicationVerification::None,
        None,
    )
    .expect("valid non-circuit definition must construct")
}

fn valid_workload_id(value: &str) -> WorkloadId {
    WorkloadId::new(value).expect("test workload identifier must be valid")
}

fn valid_request(
    definition: &CustomApplicationDefinition,
) -> ApplicationGenerationRequest {
    request(
        definition,
        valid_workload_id("test_instance"),
        8,
        42,
    )
    .expect("valid generation request must construct")
}

// ============================================================================
// Built-in namespace tests
// ============================================================================

#[test]
fn builtin_application_namespace_contains_all_expected_benchmarks() {
    let ids = builtin_application_benchmark_ids();

    let expected = [
        DEUTSCH_JOZSA_ID,
        BERNSTEIN_VAZIRANI_ID,
        HIDDEN_SHIFT_ID,
        QFT_ID,
        GROVER_ID,
        PHASE_ESTIMATION_ID,
        AMPLITUDE_ESTIMATION_ID,
        HHL_ID,
        HAMILTONIAN_ID,
        MONTE_CARLO_ID,
        QAOA_ID,
        MAXCUT_ID,
        VQE_ID,
        SHOR_ID,
        CUSTOM_ID,
    ];

    assert_eq!(
        ids,
        &expected,
        "the application namespace is a stable API and must remain deterministic"
    );
}

#[test]
fn builtin_application_ids_are_unique() {
    let ids = builtin_application_benchmark_ids();

    let unique: HashSet<&str> = ids.iter().copied().collect();

    assert_eq!(
        unique.len(),
        ids.len(),
        "duplicate application IDs would make registry and language integration ambiguous"
    );
}

#[test]
fn builtin_application_ids_are_valid_machine_identifiers() {
    for id in builtin_application_benchmark_ids() {
        assert!(
            !id.is_empty(),
            "application identifiers must never be empty"
        );

        assert!(
            id.as_bytes()[0].is_ascii_lowercase(),
            "application ID `{id}` must begin with a lowercase ASCII letter"
        );

        assert!(
            id.bytes().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || byte == b'_'
                    || byte == b'-'
            }),
            "application ID `{id}` contains a character outside the stable identifier grammar"
        );
    }
}

#[test]
fn builtin_application_membership_is_exact() {
    for id in builtin_application_benchmark_ids() {
        assert!(
            is_builtin_application_benchmark(id),
            "registered application `{id}` must be recognized"
        );
    }

    assert!(!is_builtin_application_benchmark(""));
    assert!(!is_builtin_application_benchmark("not_a_benchmark"));
    assert!(!is_builtin_application_benchmark("Grover"));
    assert!(!is_builtin_application_benchmark("grover "));
    assert!(!is_builtin_application_benchmark("qaoa/"));
}

#[test]
fn custom_namespace_is_part_of_application_registry_contract() {
    assert!(
        is_builtin_application_benchmark(CUSTOM_ID),
        "custom applications must have a stable namespace identifier"
    );
}

// ============================================================================
// Custom definition tests
// ============================================================================

#[test]
fn circuit_definition_has_consistent_execution_model_and_capabilities() {
    let definition = valid_circuit_definition();

    assert_eq!(
        definition.execution_model(),
        CustomApplicationExecutionModel::Circuit
    );

    assert!(definition.supports(
        ApplicationGeneratorCapability::GeneratesCircuit
    ));

    assert!(definition.supports(
        ApplicationGeneratorCapability::Deterministic
    ));

    assert!(definition.supports(
        ApplicationGeneratorCapability::Parameterized
    ));

    assert!(definition.supports(
        ApplicationGeneratorCapability::HardwareExecutable
    ));

    assert!(definition.supports(
        ApplicationGeneratorCapability::ScalableProblemSize
    ));

    assert_eq!(
        definition.verification(),
        CustomApplicationVerification::Exact
    );

    assert_eq!(
        definition.verification_id(),
        Some("test_exact_verifier")
    );

    definition
        .validate()
        .expect("valid circuit definition must remain valid");
}

#[test]
fn hybrid_definition_has_consistent_execution_model_and_capabilities() {
    let definition = valid_hybrid_definition();

    assert_eq!(
        definition.execution_model(),
        CustomApplicationExecutionModel::Hybrid
    );

    assert!(definition.execution_model().requires_circuit());

    assert!(definition.supports(
        ApplicationGeneratorCapability::GeneratesCircuit
    ));

    assert!(definition.supports(
        ApplicationGeneratorCapability::Hybrid
    ));

    assert!(definition.supports(
        ApplicationGeneratorCapability::Deterministic
    ));

    assert!(definition.supports(
        ApplicationGeneratorCapability::Parameterized
    ));

    assert_eq!(
        definition.verification(),
        CustomApplicationVerification::ClassicalReference
    );

    assert_eq!(
        definition.verification_id(),
        Some("test_classical_reference")
    );

    definition
        .validate()
        .expect("valid hybrid definition must remain valid");
}

#[test]
fn non_circuit_definition_cannot_advertise_circuit_generation() {
    let definition = valid_non_circuit_definition();

    assert_eq!(
        definition.execution_model(),
        CustomApplicationExecutionModel::NonCircuit
    );

    assert!(!definition.execution_model().requires_circuit());

    assert!(
        !definition.supports(
            ApplicationGeneratorCapability::GeneratesCircuit
        ),
        "non-circuit workloads must not claim circuit generation"
    );

    assert!(
        definition.supports(
            ApplicationGeneratorCapability::NonCircuit
        )
    );

    assert!(
        definition.supports(
            ApplicationGeneratorCapability::ResourceEstimation
        )
    );

    definition
        .validate()
        .expect("valid non-circuit definition must remain valid");
}

#[test]
fn custom_application_metadata_is_stable() {
    let definition = valid_circuit_definition();

    assert_eq!(definition.application_id(), "test_application");
    assert_eq!(definition.name(), "Test Application");
    assert_eq!(definition.version(), "1");
    assert_eq!(
        definition.description(),
        "Production integration-test application."
    );
    assert_eq!(
        definition.generator_id(),
        "test_application_generator"
    );
    assert_eq!(definition.generator_version(), "1");
    assert_eq!(definition.max_problem_size(), 128);
}

#[test]
fn duplicate_capabilities_are_deduplicated() {
    let definition = CustomApplicationDefinition::new(
        "duplicate_capability_app",
        "Duplicate Capability App",
        "1",
        "Capability normalization test.",
        CustomApplicationCategory::Algorithm,
        CustomApplicationExecutionModel::Circuit,
        CustomApplicationVerification::None,
        "duplicate_capability_generator",
        "1",
        16,
    )
    .expect("base definition must be valid")
    .with_capabilities([
        ApplicationGeneratorCapability::GeneratesCircuit,
        ApplicationGeneratorCapability::GeneratesCircuit,
        ApplicationGeneratorCapability::Deterministic,
        ApplicationGeneratorCapability::Deterministic,
    ]);

    let circuit_count = definition
        .capabilities()
        .iter()
        .filter(|capability| {
            **capability
                == ApplicationGeneratorCapability::GeneratesCircuit
        })
        .count();

    let deterministic_count = definition
        .capabilities()
        .iter()
        .filter(|capability| {
            **capability
                == ApplicationGeneratorCapability::Deterministic
        })
        .count();

    assert_eq!(circuit_count, 1);
    assert_eq!(deterministic_count, 1);
}

// ============================================================================
// Invalid definition tests
// ============================================================================

#[test]
fn invalid_application_id_is_rejected() {
    let result = CustomApplicationDefinition::new(
        "InvalidApplication",
        "Invalid Application",
        "1",
        "Invalid identifier test.",
        CustomApplicationCategory::Algorithm,
        CustomApplicationExecutionModel::Circuit,
        CustomApplicationVerification::None,
        "test_generator",
        "1",
        8,
    );

    assert!(
        result.is_err(),
        "uppercase application IDs must be rejected at the public boundary"
    );
}

#[test]
fn empty_application_id_is_rejected() {
    let result = CustomApplicationDefinition::new(
        "",
        "Invalid Application",
        "1",
        "Invalid identifier test.",
        CustomApplicationCategory::Algorithm,
        CustomApplicationExecutionModel::Circuit,
        CustomApplicationVerification::None,
        "test_generator",
        "1",
        8,
    );

    assert!(result.is_err());
}

#[test]
fn zero_problem_size_is_rejected() {
    let result = CustomApplicationDefinition::new(
        "zero_problem",
        "Zero Problem",
        "1",
        "Zero problem-size validation.",
        CustomApplicationCategory::Algorithm,
        CustomApplicationExecutionModel::Circuit,
        CustomApplicationVerification::None,
        "zero_problem_generator",
        "1",
        0,
    );

    assert!(result.is_err());
}

#[test]
fn empty_name_is_rejected() {
    let result = CustomApplicationDefinition::new(
        "empty_name",
        "",
        "1",
        "Empty name validation.",
        CustomApplicationCategory::Algorithm,
        CustomApplicationExecutionModel::Circuit,
        CustomApplicationVerification::None,
        "empty_name_generator",
        "1",
        8,
    );

    assert!(result.is_err());
}

#[test]
fn oversized_description_is_rejected() {
    let description =
        "x".repeat(MAX_CUSTOM_APPLICATION_DESCRIPTION_BYTES + 1);

    let result = CustomApplicationDefinition::new(
        "large_description",
        "Large Description",
        "1",
        description,
        CustomApplicationCategory::Algorithm,
        CustomApplicationExecutionModel::Circuit,
        CustomApplicationVerification::None,
        "large_description_generator",
        "1",
        8,
    );

    assert!(result.is_err());
}

#[test]
fn oversized_name_is_rejected() {
    let name = "x".repeat(MAX_CUSTOM_APPLICATION_NAME_BYTES + 1);

    let result = CustomApplicationDefinition::new(
        "large_name",
        name,
        "1",
        "Large-name validation.",
        CustomApplicationCategory::Algorithm,
        CustomApplicationExecutionModel::Circuit,
        CustomApplicationVerification::None,
        "large_name_generator",
        "1",
        8,
    );

    assert!(result.is_err());
}

#[test]
fn verification_identifier_requires_verification_model() {
    let result = CustomApplicationDefinition::new(
        "invalid_verification",
        "Invalid Verification",
        "1",
        "Verification consistency test.",
        CustomApplicationCategory::Algorithm,
        CustomApplicationExecutionModel::Circuit,
        CustomApplicationVerification::None,
        "invalid_verification_generator",
        "1",
        8,
    )
    .and_then(|definition| {
        definition.with_verification_id("orphan_verifier")
    });

    assert!(
        result.is_err(),
        "a verification ID without a verification model must be rejected"
    );
}

#[test]
fn verifiable_definition_requires_verification_identifier() {
    let result = CustomApplicationDefinition::new(
        "missing_verification_id",
        "Missing Verification ID",
        "1",
        "Verification consistency test.",
        CustomApplicationCategory::Algorithm,
        CustomApplicationExecutionModel::Circuit,
        CustomApplicationVerification::Exact,
        "missing_verification_id_generator",
        "1",
        8,
    );

    assert!(
        result.is_err(),
        "verifiable applications must identify their verification contract"
    );
}

#[test]
fn circuit_definition_requires_circuit_capability() {
    let result = CustomApplicationDefinition::new(
        "missing_circuit_capability",
        "Missing Circuit Capability",
        "1",
        "Capability consistency test.",
        CustomApplicationCategory::Algorithm,
        CustomApplicationExecutionModel::Circuit,
        CustomApplicationVerification::None,
        "missing_circuit_capability_generator",
        "1",
        8,
    )
    .and_then(|definition| definition.validate().map(|_| definition));

    assert!(
        result.is_err(),
        "circuit applications must advertise generates_circuit"
    );
}

#[test]
fn hybrid_definition_requires_hybrid_capability() {
    let result = CustomApplicationDefinition::new(
        "missing_hybrid_capability",
        "Missing Hybrid Capability",
        "1",
        "Capability consistency test.",
        CustomApplicationCategory::Algorithm,
        CustomApplicationExecutionModel::Hybrid,
        CustomApplicationVerification::None,
        "missing_hybrid_capability_generator",
        "1",
        8,
    )
    .and_then(|definition| {
        let definition = definition.with_capability(
            ApplicationGeneratorCapability::GeneratesCircuit,
        );

        definition.validate().map(|_| definition)
    });

    assert!(
        result.is_err(),
        "hybrid applications must advertise hybrid capability"
    );
}

#[test]
fn non_circuit_definition_rejects_circuit_capability() {
    let result = CustomApplicationDefinition::new(
        "invalid_non_circuit",
        "Invalid Non Circuit",
        "1",
        "Capability consistency test.",
        CustomApplicationCategory::ResourceEstimation,
        CustomApplicationExecutionModel::NonCircuit,
        CustomApplicationVerification::None,
        "invalid_non_circuit_generator",
        "1",
        8,
    )
    .map(|definition| {
        definition.with_capability(
            ApplicationGeneratorCapability::GeneratesCircuit,
        )
    })
    .and_then(|definition| definition.validate().map(|_| definition));

    assert!(
        result.is_err(),
        "non-circuit applications cannot advertise circuit generation"
    );
}

// ============================================================================
// Tag tests
// ============================================================================

#[test]
fn custom_application_tags_are_validated_and_deduplicated() {
    let tag = CustomApplicationTag::new("integration_test")
        .expect("valid tag must construct");

    let definition = valid_circuit_definition()
        .with_tag(tag.clone())
        .expect("first tag must be accepted")
        .with_tag(tag)
        .expect("duplicate tag must be harmless");

    assert_eq!(definition.tags().len(), 1);
    assert_eq!(definition.tags()[0].as_str(), "integration_test");
}

#[test]
fn invalid_custom_application_tag_is_rejected() {
    assert!(
        CustomApplicationTag::new("InvalidTag").is_err()
    );

    assert!(
        CustomApplicationTag::new("invalid tag").is_err()
    );

    assert!(
        CustomApplicationTag::new("").is_err()
    );
}

#[test]
fn custom_application_tag_limit_is_enforced() {
    let mut definition = valid_circuit_definition();

    for index in 0..MAX_CUSTOM_APPLICATION_TAGS {
        let tag = CustomApplicationTag::new(
            format!("tag_{index}"),
        )
        .expect("generated test tag must be valid");

        definition = definition
            .with_tag(tag)
            .expect("tag within configured limit must be accepted");
    }

    let overflow_tag =
        CustomApplicationTag::new("tag_overflow")
            .expect("overflow test tag must itself be valid");

    assert!(
        definition.with_tag(overflow_tag).is_err(),
        "the public API must reject tag collections beyond the configured limit"
    );
}

// ============================================================================
// Generator descriptor tests
// ============================================================================

#[test]
fn generator_descriptor_has_stable_identity() {
    let descriptor = ApplicationGeneratorDescriptor::new(
        "test_generator",
        "test_application",
        "1",
        "Integration-test generator.",
    )
    .expect("valid generator descriptor must construct")
    .with_capabilities([
        ApplicationGeneratorCapability::GeneratesCircuit,
        ApplicationGeneratorCapability::Deterministic,
        ApplicationGeneratorCapability::Parameterized,
    ]);

    assert_eq!(
        descriptor.generator_id(),
        "test_generator"
    );

    assert_eq!(
        descriptor.application_id(),
        "test_application"
    );

    assert_eq!(descriptor.version(), "1");

    assert_eq!(
        descriptor.description(),
        "Integration-test generator."
    );

    assert!(
        descriptor.supports(
            ApplicationGeneratorCapability::GeneratesCircuit
        )
    );

    assert!(
        descriptor.supports(
            ApplicationGeneratorCapability::Deterministic
        )
    );
}

#[test]
fn generator_descriptor_deduplicates_capabilities() {
    let descriptor = ApplicationGeneratorDescriptor::new(
        "descriptor_test",
        "descriptor_application",
        "1",
        "Descriptor test.",
    )
    .expect("descriptor must construct")
    .with_capabilities([
        ApplicationGeneratorCapability::Deterministic,
        ApplicationGeneratorCapability::Deterministic,
        ApplicationGeneratorCapability::Parameterized,
        ApplicationGeneratorCapability::Parameterized,
    ]);

    assert_eq!(
        descriptor
            .capabilities()
            .iter()
            .filter(|capability| {
                **capability
                    == ApplicationGeneratorCapability::Deterministic
            })
            .count(),
        1
    );

    assert_eq!(
        descriptor
            .capabilities()
            .iter()
            .filter(|capability| {
                **capability
                    == ApplicationGeneratorCapability::Parameterized
            })
            .count(),
        1
    );
}

// ============================================================================
// Request tests
// ============================================================================

#[test]
fn custom_request_uses_definition_application_id() {
    let definition = valid_circuit_definition();

    let generated_request = request(
        &definition,
        valid_workload_id("request_identity"),
        4,
        123,
    )
    .expect("valid request must construct");

    assert_eq!(
        generated_request.application_id(),
        definition.application_id()
    );

    assert_eq!(
        generated_request.problem_size(),
        4
    );

    assert_eq!(
        generated_request.metadata().seed(),
        123
    );
}

#[test]
fn custom_request_rejects_zero_problem_size() {
    let definition = valid_circuit_definition();

    let result = request(
        &definition,
        valid_workload_id("zero_problem_request"),
        0,
        123,
    );

    assert!(result.is_err());
}

#[test]
fn custom_request_rejects_problem_size_above_definition_limit() {
    let definition = circuit_definition(
        "bounded_application",
        "Bounded Application",
        "1",
        "Bounded application.",
        CustomApplicationCategory::Algorithm,
        "bounded_generator",
        "1",
        4,
        CustomApplicationVerification::None,
        None,
    )
    .expect("bounded definition must construct");

    let result = request(
        &definition,
        valid_workload_id("oversized_request"),
        5,
        123,
    );

    assert!(
        result.is_err(),
        "the definition limit must be enforced before generation"
    );
}

// ============================================================================
// Custom generator integration tests
// ============================================================================

#[test]
fn custom_generator_preserves_descriptor_and_definition_identity() {
    let definition = valid_circuit_definition();

    let generator = CustomApplicationGenerator::new(
        definition.clone(),
        |request: &ApplicationGenerationRequest| {
            super::super::applications::custom::
                make_custom_application_workload(request)
        },
    )
    .expect("custom generator must construct");

    assert_eq!(
        generator.definition().application_id(),
        "test_application"
    );

    assert_eq!(
        generator.descriptor().application_id(),
        "test_application"
    );

    assert_eq!(
        generator.descriptor().generator_id(),
        "test_application_generator"
    );

    assert_eq!(
        generator.descriptor().version(),
        "1"
    );
}

#[test]
fn custom_generator_rejects_request_for_different_application() {
    let definition = valid_circuit_definition();

    let generator = CustomApplicationGenerator::new(
        definition,
        |request: &ApplicationGenerationRequest| {
            super::super::applications::custom::
                make_custom_application_workload(request)
        },
    )
    .expect("custom generator must construct");

    let mismatched_request =
        ApplicationGenerationRequest::new(
            "different_application",
            valid_workload_id("mismatched_request"),
            4,
            42,
        )
        .expect("request itself should be structurally valid");

    assert!(
        generator.generate(&mismatched_request).is_err(),
        "generator and definition identities must never silently diverge"
    );
}

#[test]
fn custom_generator_rejects_problem_size_above_definition_limit() {
    let definition = circuit_definition(
        "small_application",
        "Small Application",
        "1",
        "Small application.",
        CustomApplicationCategory::Algorithm,
        "small_generator",
        "1",
        4,
        CustomApplicationVerification::None,
        None,
    )
    .expect("small definition must construct");

    let generator = CustomApplicationGenerator::new(
        definition,
        |request: &ApplicationGenerationRequest| {
            super::super::applications::custom::
                make_custom_application_workload(request)
        },
    )
    .expect("generator must construct");

    let request = ApplicationGenerationRequest::new(
        "small_application",
        valid_workload_id("oversized"),
        5,
        42,
    )
    .expect("request should be structurally valid");

    assert!(
        generator.generate(&request).is_err(),
        "definition-level maximum problem size must be enforced"
    );
}

// ============================================================================
// Determinism and batch tests
// ============================================================================

#[test]
fn custom_generator_is_reproducible_for_identical_requests() {
    let definition = valid_circuit_definition();

    let generator = CustomApplicationGenerator::new(
        definition,
        |request: &ApplicationGenerationRequest| {
            super::super::applications::custom::
                make_custom_application_workload(request)
        },
    )
    .expect("generator must construct");

    let request_a = ApplicationGenerationRequest::new(
        "test_application",
        valid_workload_id("deterministic_instance"),
        8,
        42,
    )
    .expect("request A must construct");

    let request_b = ApplicationGenerationRequest::new(
        "test_application",
        valid_workload_id("deterministic_instance"),
        8,
        42,
    )
    .expect("request B must construct");

    let generation_a = generator
        .generate(&request_a)
        .expect("generation A must succeed");

    let generation_b = generator
        .generate(&request_b)
        .expect("generation B must succeed");

    assert_eq!(
        generation_a.metadata(),
        generation_b.metadata(),
        "identical requests must produce identical generation metadata"
    );

    assert_eq!(
        generation_a.generator_id(),
        generation_b.generator_id()
    );

    assert_eq!(
        generation_a.generator_version(),
        generation_b.generator_version()
    );

    assert_eq!(
        generation_a.workload().application_id(),
        generation_b.workload().application_id()
    );

    assert_eq!(
        generation_a.workload().workload_id(),
        generation_b.workload().workload_id()
    );
}

#[test]
fn custom_generator_batch_preserves_sequence_indices() {
    let definition = valid_circuit_definition();

    let generator = CustomApplicationGenerator::new(
        definition,
        |request: &ApplicationGenerationRequest| {
            super::super::applications::custom::
                make_custom_application_workload(request)
        },
    )
    .expect("generator must construct");

    let request = valid_request(
        generator.definition()
    );

    let generated = generator
        .generate_batch(&request, 4)
        .expect("bounded batch must succeed");

    assert_eq!(generated.len(), 4);

    let first = request.metadata().sequence_index();

    for (offset, generation) in generated.iter().enumerate() {
        assert_eq!(
            generation.metadata().sequence_index(),
            first + offset as u64,
            "batch generation must assign monotonically increasing sequence indices"
        );
    }
}

#[test]
fn custom_generator_batch_rejects_zero_count() {
    let definition = valid_circuit_definition();

    let generator = CustomApplicationGenerator::new(
        definition,
        |request: &ApplicationGenerationRequest| {
            super::super::applications::custom::
                make_custom_application_workload(request)
        },
    )
    .expect("generator must construct");

    let request = valid_request(
        generator.definition()
    );

    assert!(
        generator.generate_batch(&request, 0).is_err(),
        "zero-length benchmark batches are invalid"
    );
}

#[test]
fn custom_generator_batch_rejects_count_above_global_generation_limit() {
    let definition = valid_circuit_definition();

    let generator = CustomApplicationGenerator::new(
        definition,
        |request: &ApplicationGenerationRequest| {
            super::super::applications::custom::
                make_custom_application_workload(request)
        },
    )
    .expect("generator must construct");

    let request = valid_request(
        generator.definition()
    );

    let excessive_count =
        MAX_GENERATED_INSTANCES
            .checked_add(1)
            .expect("test constant must permit one additional value");

    assert!(
        generator.generate_batch(&request, excessive_count).is_err(),
        "unbounded application generation must never be permitted"
    );
}

// ============================================================================
// Parameterized application tests
// ============================================================================

#[test]
fn parameterized_request_is_accepted_within_limits() {
    let definition = valid_circuit_definition();

    let parameters = vec![
        ApplicationParameter::new(
            "precision",
            ApplicationParameterValue::Float(0.001),
        )
        .expect("precision parameter must be valid"),
        ApplicationParameter::new(
            "iterations",
            ApplicationParameterValue::UnsignedInteger(10),
        )
        .expect("iterations parameter must be valid"),
    ];

    let generated_request = request_with_parameters(
        &definition,
        valid_workload_id("parameterized_instance"),
        8,
        42,
        parameters,
    )
    .expect("parameterized request must construct");

    assert_eq!(
        generated_request.parameters().len(),
        2
    );

    assert_eq!(
        generated_request.application_id(),
        definition.application_id()
    );
}

#[test]
fn parameter_count_above_contract_is_rejected() {
    let definition = valid_circuit_definition();

    let parameters: Vec<ApplicationParameter> =
        (0..(MAX_GENERATION_PARAMETERS + 1))
            .map(|index| {
                ApplicationParameter::new(
                    format!("parameter_{index}"),
                    ApplicationParameterValue::UnsignedInteger(
                        index as u64
                    ),
                )
                .expect("generated test parameter must be valid")
            })
            .collect();

    let result = request_with_parameters(
        &definition,
        valid_workload_id("too_many_parameters"),
        8,
        42,
        parameters,
    );

    assert!(
        result.is_err(),
        "parameter count must be bounded at the public API boundary"
    );
}

// ============================================================================
// Capability contract tests
// ============================================================================

#[test]
fn capability_identifiers_are_stable() {
    let capabilities = [
        ApplicationGeneratorCapability::GeneratesCircuit,
        ApplicationGeneratorCapability::NonCircuit,
        ApplicationGeneratorCapability::Hybrid,
        ApplicationGeneratorCapability::Deterministic,
        ApplicationGeneratorCapability::BatchGeneration,
        ApplicationGeneratorCapability::ScalableProblemSize,
        ApplicationGeneratorCapability::Parameterized,
        ApplicationGeneratorCapability::ExactSmallInstanceReference,
        ApplicationGeneratorCapability::ClassicallyVerifiable,
        ApplicationGeneratorCapability::ResourceEstimation,
        ApplicationGeneratorCapability::LogicalQubit,
        ApplicationGeneratorCapability::HardwareExecutable,
    ];

    let identifiers: Vec<&str> =
        capabilities.iter()
            .map(|capability| capability.as_str())
            .collect();

    let unique: HashSet<&str> =
        identifiers.iter().copied().collect();

    assert_eq!(
        unique.len(),
        identifiers.len(),
        "capability identifiers must be unique for serialization/registry use"
    );

    for identifier in identifiers {
        assert!(
            !identifier.is_empty(),
            "capability identifiers must never be empty"
        );

        assert!(
            identifier.bytes().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || byte == b'_'
                    || byte == b'-'
            }),
            "capability identifier `{identifier}` is outside the stable identifier grammar"
        );
    }
}

#[test]
fn execution_model_identifiers_are_stable() {
    assert_eq!(
        CustomApplicationExecutionModel::Circuit.as_str(),
        "circuit"
    );

    assert_eq!(
        CustomApplicationExecutionModel::Hybrid.as_str(),
        "hybrid"
    );

    assert_eq!(
        CustomApplicationExecutionModel::NonCircuit.as_str(),
        "non_circuit"
    );
}

#[test]
fn verification_identifiers_are_stable() {
    assert_eq!(
        CustomApplicationVerification::Exact.as_str(),
        "exact"
    );

    assert_eq!(
        CustomApplicationVerification::ClassicalReference.as_str(),
        "classical_reference"
    );

    assert_eq!(
        CustomApplicationVerification::Statistical.as_str(),
        "statistical"
    );

    assert_eq!(
        CustomApplicationVerification::None.as_str(),
        "none"
    );
}

#[test]
fn category_identifiers_are_stable() {
    let categories = [
        CustomApplicationCategory::Algorithm,
        CustomApplicationCategory::Optimization,
        CustomApplicationCategory::Simulation,
        CustomApplicationCategory::MachineLearning,
        CustomApplicationCategory::Cryptography,
        CustomApplicationCategory::Scientific,
        CustomApplicationCategory::FaultTolerant,
        CustomApplicationCategory::ResourceEstimation,
        CustomApplicationCategory::Custom,
    ];

    let identifiers: Vec<&str> =
        categories.iter()
            .map(|category| category.as_str())
            .collect();

    let unique: HashSet<&str> =
        identifiers.iter().copied().collect();

    assert_eq!(
        unique.len(),
        identifiers.len()
    );
}

// ============================================================================
// Resource-limit sanity tests
// ============================================================================

#[test]
fn production_limits_are_nonzero_and_ordered_safely() {
    assert!(MAX_GENERATED_INSTANCES > 0);
    assert!(MAX_GENERATION_PARAMETERS > 0);
    assert!(MAX_PROBLEM_SIZE > 0);
    assert!(MAX_CUSTOM_APPLICATION_TAGS > 0);
    assert!(MAX_REQUIRED_CAPABILITIES > 0);
}

#[test]
fn definition_maximum_problem_size_is_respected() {
    let maximum = 32;

    let definition = circuit_definition(
        "bounded_definition",
        "Bounded Definition",
        "1",
        "Maximum problem-size test.",
        CustomApplicationCategory::Algorithm,
        "bounded_definition_generator",
        "1",
        maximum,
        CustomApplicationVerification::None,
        None,
    )
    .expect("bounded definition must construct");

    assert_eq!(
        definition.max_problem_size(),
        maximum
    );

    let valid = request(
        &definition,
        valid_workload_id("maximum_problem"),
        maximum,
        7,
    );

    assert!(
        valid.is_ok(),
        "the configured maximum itself must remain usable"
    );

    let invalid = request(
        &definition,
        valid_workload_id("above_maximum"),
        maximum + 1,
        7,
    );

    assert!(
        invalid.is_err(),
        "one unit above the configured maximum must be rejected"
    );
}

// ============================================================================
// Generator output-integrity tests
// ============================================================================

#[test]
fn custom_generator_rejects_workload_with_wrong_application_identity() {
    let definition = valid_circuit_definition();

    let generator = CustomApplicationGenerator::new(
        definition,
        |_request: &ApplicationGenerationRequest| {
            /*
             * This deliberately attempts to construct a workload under a
             * different application identity.
             *
             * The custom-generator boundary must reject this rather than
             * allowing a generator to impersonate another application.
             */
            let wrong_request =
                ApplicationGenerationRequest::new(
                    "different_application",
                    valid_workload_id("wrong_application_output"),
                    4,
                    42,
                )?;

            super::super::applications::custom::
                make_custom_application_workload(&wrong_request)
        },
    )
    .expect("generator definition itself must be valid");

    let request = ApplicationGenerationRequest::new(
        "test_application",
        valid_workload_id("integrity_test"),
        4,
        42,
    )
    .expect("test request must be valid");

    assert!(
        generator.generate(&request).is_err(),
        "generated workload identity must match the application definition"
    );
}

#[test]
fn generated_application_keeps_generator_provenance() {
    let definition = valid_circuit_definition();

    let generator = CustomApplicationGenerator::new(
        definition,
        |request: &ApplicationGenerationRequest| {
            super::super::applications::custom::
                make_custom_application_workload(request)
        },
    )
    .expect("generator must construct");

    let request = valid_request(
        generator.definition()
    );

    let generation = generator
        .generate(&request)
        .expect("generation must succeed");

    assert_eq!(
        generation.generator_id(),
        generator.descriptor().generator_id()
    );

    assert_eq!(
        generation.generator_version(),
        generator.descriptor().version()
    );

    assert_eq!(
        generation.metadata().seed(),
        request.metadata().seed()
    );

    assert_eq!(
        generation.metadata().sequence_index(),
        request.metadata().sequence_index()
    );
}

// ============================================================================
// Application model matrix
// ============================================================================

#[test]
fn application_execution_models_have_correct_circuit_semantics() {
    assert!(
        CustomApplicationExecutionModel::Circuit
            .requires_circuit()
    );

    assert!(
        CustomApplicationExecutionModel::Hybrid
            .requires_circuit()
    );

    assert!(
        !CustomApplicationExecutionModel::NonCircuit
            .requires_circuit()
    );
}

#[test]
fn custom_definition_helpers_produce_the_expected_models() {
    assert_eq!(
        valid_circuit_definition().execution_model(),
        CustomApplicationExecutionModel::Circuit
    );

    assert_eq!(
        valid_hybrid_definition().execution_model(),
        CustomApplicationExecutionModel::Hybrid
    );

    assert_eq!(
        valid_non_circuit_definition().execution_model(),
        CustomApplicationExecutionModel::NonCircuit
    );
}

// ============================================================================
// API stability tests
// ============================================================================

#[test]
fn built_in_application_ids_are_lowercase_and_machine_stable() {
    for id in builtin_application_benchmark_ids() {
        assert_eq!(
            id,
            &id.to_ascii_lowercase(),
            "built-in IDs must remain lowercase machine identifiers"
        );
    }
}

#[test]
fn built_in_application_ids_have_no_whitespace() {
    for id in builtin_application_benchmark_ids() {
        assert!(
            !id.chars().any(char::is_whitespace),
            "application ID `{id}` contains whitespace"
        );
    }
}

#[test]
fn custom_application_id_is_not_confused_with_built_in_algorithm_ids() {
    assert!(
        !is_builtin_application_benchmark("test_application")
    );

    assert!(
        is_builtin_application_benchmark(CUSTOM_ID)
    );
}

// ============================================================================
// Documentation-level architectural regression tests
// ============================================================================

#[test]
fn application_namespace_contains_expected_application_families() {
    let ids = builtin_application_benchmark_ids();

    /*
     * These assertions intentionally test families rather than implementation
     * internals. If an application is renamed, the stable ID should be changed
     * deliberately and this test will force the namespace decision to be
     * reviewed.
     */
    assert!(ids.contains(&DEUTSCH_JOZSA_ID));
    assert!(ids.contains(&BERNSTEIN_VAZIRANI_ID));
    assert!(ids.contains(&HIDDEN_SHIFT_ID));
    assert!(ids.contains(&QFT_ID));
    assert!(ids.contains(&GROVER_ID));
    assert!(ids.contains(&PHASE_ESTIMATION_ID));
    assert!(ids.contains(&AMPLITUDE_ESTIMATION_ID));
    assert!(ids.contains(&HHL_ID));
    assert!(ids.contains(&HAMILTONIAN_ID));
    assert!(ids.contains(&MONTE_CARLO_ID));
    assert!(ids.contains(&QAOA_ID));
    assert!(ids.contains(&MAXCUT_ID));
    assert!(ids.contains(&VQE_ID));
    assert!(ids.contains(&SHOR_ID));
}

#[test]
fn application_tests_do_not_require_backend_execution() {
    /*
     * This test is intentionally structural.
     *
     * The important property is that the application contract can be
     * validated and workloads can be constructed without:
     *
     * - a hardware backend;
     * - credentials;
     * - a network connection;
     * - a simulator;
     * - a vendor SDK.
     *
     * Constructing and validating a custom definition is therefore the
     * executable assertion of backend independence.
     */
    let definition = valid_circuit_definition();

    definition
        .validate()
        .expect("application validation must be backend-independent");

    let request = valid_request(&definition);

    assert_eq!(
        request.application_id(),
        definition.application_id()
    );
}

// ============================================================================
// Regression protection for the custom API's resource boundaries
// ============================================================================

#[test]
fn custom_definition_capability_count_cannot_exceed_contract() {
    let capabilities = [
        ApplicationGeneratorCapability::GeneratesCircuit,
        ApplicationGeneratorCapability::NonCircuit,
        ApplicationGeneratorCapability::Hybrid,
        ApplicationGeneratorCapability::Deterministic,
        ApplicationGeneratorCapability::BatchGeneration,
        ApplicationGeneratorCapability::ScalableProblemSize,
        ApplicationGeneratorCapability::Parameterized,
        ApplicationGeneratorCapability::ExactSmallInstanceReference,
        ApplicationGeneratorCapability::ClassicallyVerifiable,
        ApplicationGeneratorCapability::ResourceEstimation,
        ApplicationGeneratorCapability::LogicalQubit,
        ApplicationGeneratorCapability::HardwareExecutable,
    ];

    assert!(
        capabilities.len() <= MAX_REQUIRED_CAPABILITIES,
        "the current capability vocabulary must fit within the definition contract"
    );
}