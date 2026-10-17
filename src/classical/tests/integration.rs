
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Cross-Module Integration Tests
//!
//! Path: src/classical/tests/integration.rs
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Exercise integration across the existing classical subsystem contracts:
//!
//! - public API configuration validation;
//! - target-independent operational budgets;
//! - compatibility descriptors and consumer profiles;
//! - strict compatibility acceptance and rejection;
//! - extensible resource and capability requirements;
//! - large exact resource quantities;
//! - deterministic, repeatable compatibility assessment;
//! - separation of declared requirements from target feasibility.
//!
//! # Authority
//!
//! - grammar/Zamani.g4: canonical grammar composition boundary.
//! - grammar/specification/semantic-model.md: semantic authority.
//! - grammar/specification/scalability-model.md: resource-parametric scaling.
//! - grammar/specification/poco-reaf.md: portability and realization policy.
//! - grammar/validation/semantic-boundaries.md: subsystem ownership.
//! - src/classical/api.rs: public classical API.
//! - src/classical/config.rs: classical configuration and operational budgets.
//! - src/classical/compatibility.rs: compatibility assessment.
//! - src/classical/execution/requirements.rs: classical execution requirements.
//! - src/frontend/semantic/resources/model.rs: canonical resource semantics.
//! - src/frontend/semantic/capabilities/model.rs: canonical capability semantics.
//! - src/quantum/ir/classical/: canonical classical IR in the quantum IR.
//!
//! # Ownership
//!
//! This file owns tests only. It does not define a second AST, semantic
//! model, value system, resource model, capability model, or intermediate
//! representation.
//!
//! The canonical classical IR within quantum::ir remains authoritative for
//! classical operations represented as part of quantum or hybrid computation.
//! These tests do not replace that IR or introduce a separate compiler.
//!
//! # Scalability and POCO-REAF
//!
//! Test inputs are representative workloads, not universal language limits.
//! Large exact quantities must remain exact where the canonical representation
//! supports them. Unsupported capabilities and resource infeasibility must
//! not be confused with source-language invalidity.
//!
//! No finite test proves literal infinite scalability. Actual execution remains
//! subject to representational constraints, configured budgets, available
//! resources, and target capabilities. Failures must be explicit; values and
//! requirements must not be silently truncated or narrowed.
//!
//! # Non-goals
//!
//! These tests do not claim to compile source text through the complete
//! frontend, verify every canonical IR invariant, allocate resources, invoke
//! a backend, execute foreign code, discover hardware, or prove that a
//! particular target can satisfy a requirement.
//!
//! Those behaviors require integration with their respective canonical
//! subsystem APIs. This test file deliberately does not invent those APIs.
//!
//! # Integration
//!
//! Register this file exactly once from the classical module composition root
//! after src/classical/mod.rs has been repaired to contain valid Rust module
//! declarations:
//
//! #[cfg(test)]
//! #[path = "tests/integration.rs"]
//! mod integration_tests;
//
//! The classical parent module must expose api, config, compatibility,
//! execution::requirements, and the existing canonical frontend types through
//! their actual module declarations. Do not register this file a second time
//! from another module.
//!
//! # Completion status
//!
//! Contract-first integration tests. They are complete only when the parent
//! module declarations are valid, this file compiles against the repository's
//! actual APIs, the tests pass, and the repository's formatting, build, test,
//! and lint checks pass on the supported toolchain.

use crate::classical::api::{
    assess_artifact_compatibility,
    require_artifact_compatibility,
    validate_configuration,
};
use crate::classical::compatibility::{
    CompatibilityDescriptor,
    CompatibilityProfile,
    CompatibilityStatus,
    CompatibilityVersion,
};
use crate::classical::config::{
    ClassicalConfig,
    CONFIG_SCHEMA_VERSION,
};
use crate::classical::error::ClassicalErrorKind;
use crate::classical::execution::requirements::{
    ExecutionRequirements,
    RequirementsCompleteness,
};
use crate::frontend::ast::node::resources::resource_kind::ResourceKind;
use crate::frontend::semantic::capabilities::model::{
    CapabilityId,
    CapabilityRequirement,
};
use crate::frontend::semantic::resources::model::{
    ExactQuantity,
    ResourceIntent,
    ResourceQuantity,
    ResourceRequirement,
};

// ---------------------------------------------------------------------------
// Test fixtures
// ---------------------------------------------------------------------------

fn version(major: u64, minor: u64, patch: u64) -> CompatibilityVersion {
    CompatibilityVersion::new(major, minor, patch)
}

fn artifact(
    domain: &str,
    contract_version: CompatibilityVersion,
    semantic_version: CompatibilityVersion,
    required_features: &[&str],
    optional_features: &[&str],
) -> CompatibilityDescriptor {
    CompatibilityDescriptor::new(
        domain,
        contract_version,
        semantic_version,
        required_features
            .iter()
            .map(|feature| (*feature).to_owned()),
        optional_features
            .iter()
            .map(|feature| (*feature).to_owned()),
    )
    .expect("test artifact must satisfy the descriptor contract")
}

fn consumer(
    domain: &str,
    contract_version: CompatibilityVersion,
    semantic_version: CompatibilityVersion,
    supported_features: &[&str],
) -> CompatibilityProfile {
    CompatibilityProfile::new(
        domain,
        contract_version,
        semantic_version,
        supported_features
            .iter()
            .map(|feature| (*feature).to_owned()),
    )
    .expect("test consumer must satisfy the profile contract")
}

fn resource_kind(name: &str) -> ResourceKind {
    ResourceKind::new("zamani.classical", name)
        .expect("namespaced resource identity must be valid")
}

fn known_resource_requirement(
    name: &str,
    quantity: &str,
) -> ResourceRequirement {
    let quantity = ResourceQuantity::known(quantity)
        .expect("resource quantity must be valid");

    ResourceRequirement::new(
        resource_kind(name),
        quantity,
        None,
        ResourceIntent::Required,
    )
    .expect("resource requirement must be valid")
}

fn capability_requirement(name: &str) -> CapabilityRequirement {
    let id = CapabilityId::new("zamani.classical", name)
        .expect("namespaced capability identity must be valid");

    CapabilityRequirement::new(id)
}

// ---------------------------------------------------------------------------
// Configuration and API integration
// ---------------------------------------------------------------------------

#[test]
fn default_configuration_passes_the_public_api_boundary() {
    let config = ClassicalConfig::new();

    assert_eq!(config.schema_version, CONFIG_SCHEMA_VERSION);
    assert!(
        validate_configuration(&config).is_ok(),
        "the default configuration must satisfy its canonical validator",
    );
}

#[test]
fn explicit_operational_budget_does_not_change_language_semantics() {
    let mut config = ClassicalConfig::new();
    config.budgets.max_work_units = Some(10_000);

    assert!(
        validate_configuration(&config).is_ok(),
        "a valid operational budget must be accepted",
    );

    assert_eq!(config.budgets.max_work_units, Some(10_000));
}

#[test]
fn unbounded_configuration_budget_means_no_configured_budget_ceiling() {
    let mut config = ClassicalConfig::new();
    config.budgets = crate::classical::config::ResourceBudget::unbounded();

    assert!(
        validate_configuration(&config).is_ok(),
        "an unbounded configuration budget must validate",
    );

    assert_eq!(config.budgets.max_elapsed_time_ms, None);
    assert_eq!(config.budgets.max_memory_bytes, None);
    assert_eq!(config.budgets.max_work_units, None);
    assert_eq!(config.budgets.max_evaluation_steps, None);
    assert_eq!(config.budgets.max_rewrite_steps, None);
    assert_eq!(config.budgets.max_diagnostics, None);
    assert_eq!(config.budgets.max_parallel_tasks, None);
}

#[test]
fn invalid_configuration_is_rejected_without_mutating_the_input() {
    let mut config = ClassicalConfig::new();
    config.schema_version = CONFIG_SCHEMA_VERSION.saturating_add(1);
    let before = config.clone();

    let error = validate_configuration(&config)
        .expect_err("an unsupported configuration schema must fail");

    assert_eq!(error.kind(), ClassicalErrorKind::InvalidConfiguration);
    assert_eq!(error.code(), "classical.invalid_configuration");
    assert_eq!(
        config, before,
        "validation must not mutate the caller's configuration",
    );
}

// ---------------------------------------------------------------------------
// Compatibility integration
// ---------------------------------------------------------------------------

#[test]
fn compatible_classical_contract_passes_the_strict_api_gate() {
    let contract_version = version(1, 0, 0);
    let semantic_version = version(1, 0, 0);

    let declared_artifact = artifact(
        "classical.ir",
        contract_version,
        semantic_version,
        &[
            "classical.numeric.checked_arithmetic",
            "classical.resources.symbolic_requirements",
        ],
        &[],
    );

    let target_profile = consumer(
        "classical.ir",
        contract_version,
        semantic_version,
        &[
            "classical.numeric.checked_arithmetic",
            "classical.resources.symbolic_requirements",
        ],
    );

    let report = require_artifact_compatibility(
        &declared_artifact,
        &target_profile,
    )
    .expect("the consumer supports every required feature");

    assert_eq!(report.status(), CompatibilityStatus::Compatible);
    assert!(report.is_compatible());
    assert!(!report.has_blocking_issues());
}

#[test]
fn missing_required_capability_feature_is_not_silently_accepted() {
    let contract_version = version(1, 0, 0);
    let semantic_version = version(1, 0, 0);

    let declared_artifact = artifact(
        "classical.ir",
        contract_version,
        semantic_version,
        &["classical.numeric.arbitrary_precision"],
        &[],
    );

    let target_profile = consumer(
        "classical.ir",
        contract_version,
        semantic_version,
        &[],
    );

    let report = assess_artifact_compatibility(
        &declared_artifact,
        &target_profile,
    );

    assert_eq!(report.status(), CompatibilityStatus::Incompatible);
    assert!(!report.is_compatible());
    assert!(report.has_blocking_issues());

    let error = require_artifact_compatibility(
        &declared_artifact,
        &target_profile,
    )
    .expect_err("strict compatibility must reject a missing required feature");

    assert_eq!(error.kind(), ClassicalErrorKind::VersionMismatch);
    assert_eq!(error.code(), "classical.version_mismatch");
}

#[test]
fn unavailable_optional_feature_remains_a_warning() {
    let contract_version = version(1, 0, 0);
    let semantic_version = version(1, 0, 0);

    let declared_artifact = artifact(
        "classical.ir",
        contract_version,
        semantic_version,
        &[],
        &["classical.optimization.optional_fast_path"],
    );

    let target_profile = consumer(
        "classical.ir",
        contract_version,
        semantic_version,
        &[],
    );

    let report = require_artifact_compatibility(
        &declared_artifact,
        &target_profile,
    )
    .expect("a missing optional feature must not block compatibility");

    assert_eq!(
        report.status(),
        CompatibilityStatus::CompatibleWithWarnings,
    );
    assert!(report.is_compatible());
    assert!(!report.has_blocking_issues());
    assert!(!report.issues().is_empty());
}

#[test]
fn compatibility_assessment_is_repeatable_for_identical_inputs() {
    let contract_version = version(2, 3, 0);
    let semantic_version = version(1, 4, 2);

    let declared_artifact = artifact(
        "classical.ir",
        contract_version,
        semantic_version,
        &["classical.numeric.checked_arithmetic"],
        &["classical.optimization.optional_fast_path"],
    );

    let target_profile = consumer(
        "classical.ir",
        contract_version,
        semantic_version,
        &["classical.numeric.checked_arithmetic"],
    );

    let first = assess_artifact_compatibility(
        &declared_artifact,
        &target_profile,
    );

    let second = assess_artifact_compatibility(
        &declared_artifact,
        &target_profile,
    );

    assert_eq!(first.status(), second.status());
    assert_eq!(first.is_compatible(), second.is_compatible());
    assert_eq!(first.has_blocking_issues(), second.has_blocking_issues());
    assert_eq!(first.issues().len(), second.issues().len());

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

    assert_eq!(first_codes, second_codes);
}

// ---------------------------------------------------------------------------
// Configuration, compatibility, and resource-requirement integration
// ---------------------------------------------------------------------------

#[test]
fn valid_configuration_and_compatible_contract_can_coexist_with_requirements() {
    let config = ClassicalConfig::new();
    assert!(validate_configuration(&config).is_ok());

    let contract_version = version(1, 0, 0);
    let semantic_version = version(1, 0, 0);

    let declared_artifact = artifact(
        "classical.ir",
        contract_version,
        semantic_version,
        &[
            "classical.numeric.checked_arithmetic",
            "classical.resources.symbolic_requirements",
        ],
        &[],
    );

    let target_profile = consumer(
        "classical.ir",
        contract_version,
        semantic_version,
        &[
            "classical.numeric.checked_arithmetic",
            "classical.resources.symbolic_requirements",
        ],
    );

    let report = require_artifact_compatibility(
        &declared_artifact,
        &target_profile,
    )
    .expect("declared contract must be compatible");

    assert!(report.is_compatible());

    let mut requirements = ExecutionRequirements::new();

    requirements.add_resource_requirement(
        known_resource_requirement("memory", "4096"),
    );
    requirements.add_resource_requirement(
        known_resource_requirement("workers", "8"),
    );
    requirements.add_capability_requirement(
        capability_requirement("numeric.checked_arithmetic"),
    );

    assert_eq!(
        requirements.completeness(),
        RequirementsCompleteness::Complete,
    );
    assert!(requirements.ensure_complete().is_ok());
    assert_eq!(requirements.resource_requirement_count(), 2);
    assert_eq!(requirements.capability_requirement_count(), 1);

    // Compatibility and completeness are not target-feasibility proofs.
    // Target capability discovery, allocation, placement, and scheduling
    // remain the responsibility of their respective subsystem owners.
}

#[test]
fn exact_large_resource_quantity_survives_requirement_construction() {
    // A generated test value, not a language or implementation limit.
    let decimal_quantity = format!("1{}", "0".repeat(300));

    let exact = ExactQuantity::new(&decimal_quantity)
        .expect("the canonical exact-quantity type must accept this value");

    assert_eq!(exact.as_str(), decimal_quantity);
    assert!(!exact.is_zero());

    let requirement = ResourceRequirement::new(
        resource_kind("memory"),
        ResourceQuantity::Known(exact),
        None,
        ResourceIntent::Required,
    )
    .expect("a large exact quantity must remain a valid requirement");

    let mut requirements = ExecutionRequirements::new();
    requirements.add_resource_requirement(requirement);

    assert_eq!(requirements.resource_requirement_count(), 1);
    assert!(requirements.ensure_complete().is_ok());
}

#[test]
fn repeated_resource_requirements_are_not_silently_discarded() {
    let first = known_resource_requirement("memory", "1024");
    let second = known_resource_requirement("memory", "1024");

    let mut requirements = ExecutionRequirements::new();
    requirements.add_resource_requirement(first.clone());
    requirements.add_resource_requirement(second.clone());

    assert_eq!(requirements.resource_requirement_count(), 2);
    assert_eq!(
        requirements.resource_requirements(),
        &[first, second],
        "the requirements boundary must preserve both declarations",
    );
}

#[test]
fn unknown_namespaced_capability_identifiers_remain_extensible() {
    let capability = capability_requirement(
        "future_accelerator.specialized_operation",
    );

    let mut requirements = ExecutionRequirements::new();
    requirements.add_capability_requirement(capability);

    assert_eq!(requirements.capability_requirement_count(), 1);
    assert!(requirements.ensure_complete().is_ok());

    // Acceptance of a well-formed identifier does not assert that any
    // current target implements it. Feasibility must be checked separately.
}
