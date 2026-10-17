
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Determinism Contract Tests
//!
//! Path: src/classical/tests/determinism.rs
//! Edition: Rust 2021
//! Minimum intended Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Test the determinism contracts already exposed by the classical subsystem:
//!
//! - configuration preserves determinism and numerical policies;
//! - configuration validation is repeatable and does not mutate its input;
//! - compatibility feature collections have canonical ordering;
//! - compatibility assessment produces repeatable status and issue ordering;
//! - equivalent compatibility inputs are unaffected by feature insertion order;
//! - execution requirements preserve declared insertion order and duplicates;
//! - generated feature collections remain deterministic as their size grows.
//!
//! # Authority
//!
//! - grammar/Zamani.g4: canonical grammar composition boundary.
//! - grammar/antlr/ZamaniLexer.g4: canonical lexer boundary.
//! - grammar/specification/semantic-model.md: semantic authority.
//! - grammar/specification/scalability-model.md: resource-parametric scaling.
//! - grammar/validation/semantic-boundaries.md: semantic ownership boundaries.
//! - src/classical/config.rs: determinism and numerical policies.
//! - src/classical/compatibility.rs: ordered compatibility contracts and reports.
//! - src/classical/api.rs: public configuration and compatibility boundary.
//! - src/classical/execution/requirements.rs: ordered requirement aggregation.
//! - src/frontend/semantic/resources/model.rs: canonical resource semantics.
//! - src/frontend/semantic/capabilities/model.rs: canonical capability semantics.
//! - src/quantum/ir/classical/: canonical classical IR integration boundary.
//!
//! # Scope and limits of these tests
//!
//! These tests establish determinism only for the APIs they exercise. They do
//! not claim that the parser, semantic analyzer, IR passes, parallel runtime,
//! numerical kernels, scheduler, or every target is already deterministic.
//!
//! Those components need tests at their own canonical interfaces. In
//! particular, deterministic configuration is a request or policy, not proof
//! that a compiler pass or runtime actually enforces it.
//!
//! # Scalability
//!
//! Test workloads are generated from their input data. Their sizes are test
//! parameters, not language-level ceilings. No test establishes a universal
//! maximum number of features, requirements, operations, workers, devices,
//! or data elements.
//!
//! Finite tests cannot prove literal infinite scalability. They can detect
//! accidental order dependence, fixed semantic limits, silent deduplication,
//! and loss of determinism across increasing representative workloads.
//!
//! # Integration
//!
//! Register this file once from the classical module composition root after
//! src/classical/mod.rs has valid Rust module declarations:
//
//! #[cfg(test)]
//! #[path = "tests/determinism.rs"]
//! mod determinism_tests;
//
//! The module must use the existing configuration, compatibility, and
//! execution-requirement owners. Do not create parallel policy types or a
//! second resource/capability representation.
//!
//! # Grammar and compiler integration
//!
//! Grammar-level determinism requires separate tests proving that the same
//! source, grammar revision, lexer configuration, and parser configuration
//! yield equivalent tokens, syntax trees, source spans, and diagnostics.
//! Semantic and IR determinism requires tests at those modules' real APIs.
//! This file intentionally does not invent frontend or IR construction APIs.
//!
//! # Completion
//!
//! This file is complete when it compiles against the repository's actual
//! module tree, all assertions pass, and the repository's supported Rust
//! formatting, build, test, and lint checks pass.

use crate::classical::api::{
    assess_artifact_compatibility,
    validate_configuration,
};
use crate::classical::compatibility::{
    CompatibilityDescriptor,
    CompatibilityProfile,
    CompatibilityVersion,
};
use crate::classical::config::{
    ClassicalConfig,
    DeterminismPolicy,
    FloatSemantics,
    NumericalReproducibility,
};
use crate::classical::execution::requirements::ExecutionRequirements;
use crate::frontend::ast::node::resources::resource_kind::ResourceKind;
use crate::frontend::semantic::capabilities::model::{
    CapabilityId,
    CapabilityRequirement,
};
use crate::frontend::semantic::resources::model::{
    ResourceIntent,
    ResourceQuantity,
    ResourceRequirement,
};

// ---------------------------------------------------------------------------
// Shared deterministic fixtures
// ---------------------------------------------------------------------------

fn version(major: u64, minor: u64, patch: u64) -> CompatibilityVersion {
    CompatibilityVersion::new(major, minor, patch)
}

fn artifact(
    domain: &str,
    contract_version: CompatibilityVersion,
    semantic_version: CompatibilityVersion,
    required: impl IntoIterator<Item = String>,
    optional: impl IntoIterator<Item = String>,
) -> CompatibilityDescriptor {
    CompatibilityDescriptor::new(
        domain,
        contract_version,
        semantic_version,
        required,
        optional,
    )
    .expect("determinism test artifact must be valid")
}

fn consumer(
    domain: &str,
    contract_version: CompatibilityVersion,
    semantic_version: CompatibilityVersion,
    features: impl IntoIterator<Item = String>,
) -> CompatibilityProfile {
    CompatibilityProfile::new(
        domain,
        contract_version,
        semantic_version,
        features,
    )
    .expect("determinism test consumer must be valid")
}

fn feature_strings(features: &[&str]) -> Vec<String> {
    features
        .iter()
        .map(|feature| (*feature).to_owned())
        .collect()
}

fn resource_requirement(name: &str, quantity: &str) -> ResourceRequirement {
    let kind = ResourceKind::new("zamani.classical", name)
        .expect("test resource kind must be valid");

    let quantity = ResourceQuantity::known(quantity)
        .expect("test resource quantity must be valid");

    ResourceRequirement::new(
        kind,
        quantity,
        None,
        ResourceIntent::Required,
    )
    .expect("test resource requirement must be valid")
}

fn capability_requirement(name: &str) -> CapabilityRequirement {
    let id = CapabilityId::new("zamani.classical", name)
        .expect("test capability identifier must be valid");

    CapabilityRequirement::new(id)
}

// ---------------------------------------------------------------------------
// Configuration policy stability
// ---------------------------------------------------------------------------

#[test]
fn deterministic_configuration_policies_survive_validation_unchanged() {
    let mut config = ClassicalConfig::new();

    config.determinism = DeterminismPolicy::CrossImplementation;
    config.numerical.reproducibility = NumericalReproducibility::CrossTarget;
    config.numerical.float_semantics = FloatSemantics::Strict;
    config.numerical.allow_contraction = false;

    let before = config.clone();

    assert!(
        validate_configuration(&config).is_ok(),
        "valid determinism policies must pass configuration validation",
    );

    assert_eq!(
        config, before,
        "validation must not mutate reproducibility policy",
    );

    assert_eq!(
        config.determinism,
        DeterminismPolicy::CrossImplementation,
    );
    assert_eq!(
        config.numerical.reproducibility,
        NumericalReproducibility::CrossTarget,
    );
    assert_eq!(
        config.numerical.float_semantics,
        FloatSemantics::Strict,
    );
    assert!(!config.numerical.allow_contraction);
}

#[test]
fn repeated_validation_returns_the_same_result() {
    let config = ClassicalConfig::new();

    let first = validate_configuration(&config);
    let second = validate_configuration(&config);

    assert_eq!(
        first, second,
        "the same configuration must have the same validation result",
    );
}

#[test]
fn deterministic_policy_is_distinct_from_parallelism_policy() {
    let mut sequential = ClassicalConfig::new();
    sequential.determinism = DeterminismPolicy::Deterministic;
    sequential.parallelism =
        crate::classical::config::ParallelismPolicy::Sequential;

    let mut automatic = ClassicalConfig::new();
    automatic.determinism = DeterminismPolicy::Deterministic;
    automatic.parallelism =
        crate::classical::config::ParallelismPolicy::Auto;

    assert!(validate_configuration(&sequential).is_ok());
    assert!(validate_configuration(&automatic).is_ok());

    assert_eq!(
        sequential.determinism,
        automatic.determinism,
        "determinism policy must not be encoded as a worker-count policy",
    );

    // These configurations request the same determinism level but different
    // parallelism policies. This test does not claim that an execution engine
    // already guarantees identical results across both execution strategies.
}

// ---------------------------------------------------------------------------
// Canonical compatibility ordering
// ---------------------------------------------------------------------------

#[test]
fn required_and_optional_features_have_canonical_order() {
    let descriptor = artifact(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        feature_strings(&[
            "classical.zeta",
            "classical.alpha",
            "classical.mu",
        ]),
        feature_strings(&[
            "classical.optional.zeta",
            "classical.optional.alpha",
        ]),
    );

    let required: Vec<_> = descriptor
        .required_features()
        .iter()
        .map(String::as_str)
        .collect();

    let optional: Vec<_> = descriptor
        .optional_features()
        .iter()
        .map(String::as_str)
        .collect();

    assert_eq!(
        required,
        vec![
            "classical.alpha",
            "classical.mu",
            "classical.zeta",
        ],
    );

    assert_eq!(
        optional,
        vec![
            "classical.optional.alpha",
            "classical.optional.zeta",
        ],
    );
}

#[test]
fn feature_insertion_order_does_not_change_the_descriptor() {
    let first = artifact(
        "classical.ir",
        version(1, 2, 0),
        version(2, 1, 0),
        feature_strings(&[
            "classical.numeric.checked_arithmetic",
            "classical.resources.symbolic_requirements",
            "classical.parallel.data_parallel",
        ]),
        feature_strings(&[
            "classical.optimization.optional_fast_path",
            "classical.debug.source_mapping",
        ]),
    );

    let second = artifact(
        "classical.ir",
        version(1, 2, 0),
        version(2, 1, 0),
        feature_strings(&[
            "classical.parallel.data_parallel",
            "classical.resources.symbolic_requirements",
            "classical.numeric.checked_arithmetic",
        ]),
        feature_strings(&[
            "classical.debug.source_mapping",
            "classical.optimization.optional_fast_path",
        ]),
    );

    assert_eq!(
        first, second,
        "equivalent feature sets must yield equivalent descriptors",
    );
}

#[test]
fn compatibility_report_is_stable_for_identical_inputs() {
    let declared_artifact = artifact(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        feature_strings(&[
            "classical.numeric.arbitrary_precision",
            "classical.parallel.data_parallel",
        ]),
        feature_strings(&[
            "classical.optimization.optional_fast_path",
        ]),
    );

    let target = consumer(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        feature_strings(&[
            "classical.parallel.data_parallel",
        ]),
    );

    let first = assess_artifact_compatibility(
        &declared_artifact,
        &target,
    );

    let second = assess_artifact_compatibility(
        &declared_artifact,
        &target,
    );

    assert_eq!(first.status(), second.status());
    assert_eq!(first.is_compatible(), second.is_compatible());
    assert_eq!(
        first.has_blocking_issues(),
        second.has_blocking_issues(),
    );

    let first_codes: Vec<_> = first
        .issues()
        .iter()
        .map(|issue| issue.code().as_str())
        .collect();

    let second_codes: Vec<_> = second
        .issues()
        .iter()
        .map(|issue| issue.code().as_str())
        .collect();

    assert_eq!(
        first_codes, second_codes,
        "identical inputs must produce identically ordered issue codes",
    );
}

#[test]
fn compatibility_assessment_is_independent_of_feature_insertion_order() {
    let artifact_a = artifact(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        feature_strings(&[
            "classical.alpha.required",
            "classical.beta.required",
            "classical.gamma.required",
        ]),
        feature_strings(&[
            "classical.optional.delta",
            "classical.optional.epsilon",
        ]),
    );

    let artifact_b = artifact(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        feature_strings(&[
            "classical.gamma.required",
            "classical.alpha.required",
            "classical.beta.required",
        ]),
        feature_strings(&[
            "classical.optional.epsilon",
            "classical.optional.delta",
        ]),
    );

    let target_a = consumer(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        feature_strings(&[
            "classical.beta.required",
            "classical.optional.delta",
        ]),
    );

    let target_b = consumer(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        feature_strings(&[
            "classical.optional.delta",
            "classical.beta.required",
        ]),
    );

    let report_a = assess_artifact_compatibility(&artifact_a, &target_a);
    let report_b = assess_artifact_compatibility(&artifact_b, &target_b);

    assert_eq!(report_a.status(), report_b.status());
    assert_eq!(report_a.is_compatible(), report_b.is_compatible());
    assert_eq!(
        report_a.has_blocking_issues(),
        report_b.has_blocking_issues(),
    );

    let codes_a: Vec<_> = report_a
        .issues()
        .iter()
        .map(|issue| issue.code().as_str())
        .collect();

    let codes_b: Vec<_> = report_b
        .issues()
        .iter()
        .map(|issue| issue.code().as_str())
        .collect();

    assert_eq!(
        codes_a, codes_b,
        "permuting equivalent feature inputs must not reorder the report",
    );
}

// ---------------------------------------------------------------------------
// Deterministic requirement aggregation
// ---------------------------------------------------------------------------

#[test]
fn resource_requirements_preserve_declared_order() {
    let first = resource_requirement("memory", "1024");
    let second = resource_requirement("workers", "8");
    let third = resource_requirement("storage", "4096");

    let mut requirements = ExecutionRequirements::new();

    requirements.add_resource_requirement(first.clone());
    requirements.add_resource_requirement(second.clone());
    requirements.add_resource_requirement(third.clone());

    assert_eq!(
        requirements.resource_requirements(),
        &[first, second, third],
        "resource requirements must preserve their declared order",
    );
}

#[test]
fn duplicate_resource_requirements_are_preserved_in_order() {
    let first = resource_requirement("memory", "1024");
    let second = resource_requirement("workers", "8");
    let duplicate = resource_requirement("memory", "1024");

    let mut requirements = ExecutionRequirements::new();

    requirements.add_resource_requirement(first.clone());
    requirements.add_resource_requirement(second.clone());
    requirements.add_resource_requirement(duplicate.clone());

    assert_eq!(
        requirements.resource_requirements(),
        &[first, second, duplicate],
        "duplicate-looking declarations must not be silently removed",
    );
}

#[test]
fn capability_requirements_preserve_declared_order() {
    let first = capability_requirement("numeric.checked_arithmetic");
    let second = capability_requirement("parallel.data_parallel");
    let third = capability_requirement("future.specialized_operation");

    let mut requirements = ExecutionRequirements::new();

    requirements.add_capability_requirement(first.clone());
    requirements.add_capability_requirement(second.clone());
    requirements.add_capability_requirement(third.clone());

    let entries = requirements.capability_requirements();

    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].requirement(), &first);
    assert_eq!(entries[1].requirement(), &second);
    assert_eq!(entries[2].requirement(), &third);
}

#[test]
fn cloning_requirements_preserves_observable_order_and_contents() {
    let mut original = ExecutionRequirements::new();

    original.add_resource_requirement(
        resource_requirement("memory", "1024"),
    );
    original.add_resource_requirement(
        resource_requirement("workers", "8"),
    );
    original.add_capability_requirement(
        capability_requirement("numeric.checked_arithmetic"),
    );
    original.add_capability_requirement(
        capability_requirement("future.specialized_operation"),
    );

    let cloned = original.clone();

    assert_eq!(
        cloned.resource_requirements(),
        original.resource_requirements(),
    );

    assert_eq!(
        cloned.capability_requirements(),
        original.capability_requirements(),
    );

    assert_eq!(cloned.completeness(), original.completeness());
    assert_eq!(cloned.issue_count(), original.issue_count());
}

// ---------------------------------------------------------------------------
// Growth and extensibility
// ---------------------------------------------------------------------------

#[test]
fn generated_feature_sets_remain_canonical_as_the_workload_grows() {
    // The chosen count is a test workload, not a language-level ceiling.
    // Generate identifiers from their input index instead of enumerating a
    // closed set of hardware, devices, or built-in capabilities.
    let feature_count = 128_usize;

    let forward: Vec<String> = (0..feature_count)
        .map(|index| format!("classical.extension.feature_{index:04}"))
        .collect();

    let reverse: Vec<String> = forward.iter().rev().cloned().collect();

    let first = artifact(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        forward.clone(),
        Vec::<String>::new(),
    );

    let second = artifact(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        reverse,
        Vec::<String>::new(),
    );

    assert_eq!(first, second);
    assert_eq!(first.required_features().len(), feature_count);

    let canonical: Vec<_> = first
        .required_features()
        .iter()
        .cloned()
        .collect();

    let mut independently_sorted = forward;
    independently_sorted.sort();

    assert_eq!(
        canonical, independently_sorted,
        "feature ordering must remain canonical for a larger generated set",
    );
}
