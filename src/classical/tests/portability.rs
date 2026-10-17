
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Portability Contract Tests
//!
//! Path: src/classical/tests/portability.rs
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Test the existing, target-independent portability contracts:
//!
//! - required features must be supported before compatibility is accepted;
//! - optional features may be unavailable without blocking compatibility;
//! - compatibility domains and versions must not be silently weakened;
//! - compatibility reports must be stable for identical inputs;
//! - extensible feature identifiers must not be restricted to a closed list;
//! - numerical reproducibility and determinism policies must remain explicit;
//! - operational budgets must remain separate from language semantics;
//! - configuration validation must not mutate the supplied configuration.
//!
//! # Authority
//!
//! - grammar/antlr/ZamaniLexer.g4: canonical lexical composition boundary.
//! - grammar/antlr/ZamaniParser.g4: canonical parser composition boundary.
//! - grammar/specification/poco-reaf.md: normative POCO-REAF objective.
//! - grammar/specification/scalability-model.md: resource-parametric scaling.
//! - grammar/specification/semantic-model.md: semantic ownership.
//! - grammar/validation/semantic-boundaries.md: subsystem boundaries.
//! - src/classical/api.rs: public configuration and compatibility API.
//! - src/classical/config.rs: numerical policies and operational budgets.
//! - src/classical/compatibility.rs: compatibility descriptors and reports.
//! - src/classical/execution/requirements.rs: canonical execution requirements.
//! - src/frontend/semantic/: canonical semantic resource and capability models.
//! - src/quantum/ir/classical/: existing classical IR integration boundary.
//! - src/quantum/ir/: canonical quantum/hybrid IR boundary.
//!
//! # Ownership and non-goals
//!
//! This file owns tests only. It does not define a target model, capability
//! registry, resource model, AST, semantic model, IR, compiler, or runtime.
//!
//! The current compatibility profile describes a contract domain, versions,
//! and supported feature identifiers. It is not a hardware-discovery result.
//! Therefore these tests verify compatibility decisions, not actual hardware
//! execution or end-to-end cross-target output equivalence.
//!
//! The grammar specifies source syntax and composition. It does not establish
//! that a grammar production has been implemented by the compiler or that a
//! target can realize the resulting program.
//!
//! # POCO-REAF and scalability
//!
//! A portable program must not acquire a different meaning merely because a
//! target has fewer optional optimizations or a different execution scale.
//! Missing required features must be reported as incompatibility; missing
//! optional features may produce warnings. Neither case may silently weaken
//! a declared requirement.
//!
//! Test fixture sizes are test inputs, not language-level limits. No finite
//! test proves literal infinite scalability. Actual execution remains subject
//! to representational limits, explicit operational budgets, target
//! capabilities, implementation support, and physical resources.
//!
//! # Integration
//!
//! Register this file exactly once from the classical module composition root
//! when that root contains valid Rust module declarations:
//
//! #[cfg(test)]
//! #[path = "tests/portability.rs"]
//! mod portability_tests;
//
//! Use the existing `api`, `config`, and `compatibility` owners. Do not add a
//! parallel compatibility or target-capability representation.
//!
//! End-to-end portability tests should later be added at the real compiler,
//! canonical IR verification, lowering, and target conformance interfaces.
//! They must compare specified observable behavior, not merely successful
//! compilation.
//!
//! # Completion status
//!
//! The test source is written against the repository APIs inspected for this
//! task. It is not considered build-verified until the classical module tree
//! is valid and the tests pass on the repository's supported Rust toolchain.

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
    DeterminismPolicy,
    FloatSemantics,
    NumericalReproducibility,
    ParallelismPolicy,
    ResourceBudget,
};

// ---------------------------------------------------------------------------
// Shared fixtures
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
    .expect("portability test artifact must satisfy the descriptor contract")
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
    .expect("portability test consumer must satisfy the profile contract")
}

fn base_versions() -> (CompatibilityVersion, CompatibilityVersion) {
    (version(1, 0, 0), version(1, 0, 0))
}

// ---------------------------------------------------------------------------
// Required and optional feature portability
// ---------------------------------------------------------------------------

#[test]
fn required_contract_is_accepted_by_each_consumer_that_supports_it() {
    let (contract_version, semantic_version) = base_versions();

    let program_contract = artifact(
        "classical.ir",
        contract_version,
        semantic_version,
        &[
            "classical.numeric.checked_arithmetic",
            "classical.resources.symbolic_requirements",
        ],
        &[],
    );

    // These are abstract consumer profiles, not claims about any particular
    // physical device. Both support the same required semantic contract.
    let smaller_consumer = consumer(
        "classical.ir",
        contract_version,
        semantic_version,
        &[
            "classical.numeric.checked_arithmetic",
            "classical.resources.symbolic_requirements",
        ],
    );

    let larger_consumer = consumer(
        "classical.ir",
        contract_version,
        semantic_version,
        &[
            "classical.numeric.checked_arithmetic",
            "classical.resources.symbolic_requirements",
            "classical.optimization.vectorization",
            "classical.parallel.data_parallel",
        ],
    );

    let smaller_report =
        require_artifact_compatibility(&program_contract, &smaller_consumer)
            .expect("all required features are supported");

    let larger_report =
        require_artifact_compatibility(&program_contract, &larger_consumer)
            .expect("all required features are supported");

    assert_eq!(smaller_report.status(), CompatibilityStatus::Compatible);
    assert_eq!(larger_report.status(), CompatibilityStatus::Compatible);
    assert!(!smaller_report.has_blocking_issues());
    assert!(!larger_report.has_blocking_issues());
}

#[test]
fn missing_required_feature_is_reported_as_incompatible() {
    let (contract_version, semantic_version) = base_versions();

    let program_contract = artifact(
        "classical.ir",
        contract_version,
        semantic_version,
        &["classical.numeric.checked_arithmetic"],
        &[],
    );

    let consumer_without_feature = consumer(
        "classical.ir",
        contract_version,
        semantic_version,
        &[],
    );

    let report =
        assess_artifact_compatibility(&program_contract, &consumer_without_feature);

    assert_eq!(report.status(), CompatibilityStatus::Incompatible);
    assert!(!report.is_compatible());
    assert!(report.has_blocking_issues());

    let required_feature_issue_exists = report.issues().iter().any(|issue| {
        issue.code().as_str()
            == "classical.compatibility.missing_required_feature"
    });

    assert!(
        required_feature_issue_exists,
        "the report must identify the missing required feature",
    );

    assert!(
        require_artifact_compatibility(&program_contract, &consumer_without_feature)
            .is_err(),
        "the strict API must not accept an unsatisfied required feature",
    );
}

#[test]
fn missing_optional_feature_is_a_warning_not_a_blocking_requirement() {
    let (contract_version, semantic_version) = base_versions();

    let program_contract = artifact(
        "classical.ir",
        contract_version,
        semantic_version,
        &["classical.numeric.checked_arithmetic"],
        &["classical.optimization.optional_fast_path"],
    );

    let consumer_without_optional_feature = consumer(
        "classical.ir",
        contract_version,
        semantic_version,
        &["classical.numeric.checked_arithmetic"],
    );

    let report = require_artifact_compatibility(
        &program_contract,
        &consumer_without_optional_feature,
    )
    .expect("missing optional functionality must not block compatibility");

    assert_eq!(
        report.status(),
        CompatibilityStatus::CompatibleWithWarnings,
    );
    assert!(report.is_compatible());
    assert!(!report.has_blocking_issues());

    assert!(
        report.issues().iter().any(|issue| {
            issue.code().as_str()
                == "classical.compatibility.missing_optional_feature"
        }),
        "the optional feature omission must remain visible to callers",
    );
}

// ---------------------------------------------------------------------------
// Domain and version boundaries
// ---------------------------------------------------------------------------

#[test]
fn different_compatibility_domains_are_not_treated_as_portable_equivalents() {
    let (contract_version, semantic_version) = base_versions();

    let program_contract = artifact(
        "classical.ir",
        contract_version,
        semantic_version,
        &[],
        &[],
    );

    let wrong_domain_consumer = consumer(
        "classical.experimental_ir",
        contract_version,
        semantic_version,
        &[],
    );

    let report =
        assess_artifact_compatibility(&program_contract, &wrong_domain_consumer);

    assert_eq!(report.status(), CompatibilityStatus::Incompatible);
    assert!(report.has_blocking_issues());

    assert!(
        report.issues().iter().any(|issue| {
            issue.code().as_str() == "classical.compatibility.domain_mismatch"
        }),
        "a domain mismatch must have an explicit diagnostic code",
    );
}

#[test]
fn newer_incompatible_semantic_version_is_not_silently_downgraded() {
    let contract_version = version(1, 0, 0);
    let newer_semantics = version(2, 0, 0);
    let older_consumer_semantics = version(1, 0, 0);

    let program_contract = artifact(
        "classical.ir",
        contract_version,
        newer_semantics,
        &[],
        &[],
    );

    let older_consumer = consumer(
        "classical.ir",
        contract_version,
        older_consumer_semantics,
        &[],
    );

    let report = assess_artifact_compatibility(&program_contract, &older_consumer);

    assert_eq!(report.status(), CompatibilityStatus::Incompatible);
    assert!(report.has_blocking_issues());

    assert!(
        report.issues().iter().any(|issue| {
            issue.code().as_str()
                == "classical.compatibility.semantic_major_mismatch"
        }),
        "an unsupported semantic major version must be diagnosed",
    );
}

// ---------------------------------------------------------------------------
// Extensibility and deterministic portability decisions
// ---------------------------------------------------------------------------

#[test]
fn future_namespaced_features_are_not_restricted_to_a_closed_enum() {
    let (contract_version, semantic_version) = base_versions();

    // Generate feature identifiers as test data. The count is not a language
    // limit or a promise about any target's available capacity.
    let generated_features: Vec<String> = (0..128)
        .map(|index| format!("classical.future_domain.feature_{index}"))
        .collect();

    let program_contract = CompatibilityDescriptor::new(
        "classical.ir",
        contract_version,
        semantic_version,
        generated_features.clone(),
        Vec::<String>::new(),
    )
    .expect("well-formed future feature identifiers must be representable");

    let supporting_consumer = CompatibilityProfile::new(
        "classical.ir",
        contract_version,
        semantic_version,
        generated_features,
    )
    .expect("the profile must accept extensible feature identifiers");

    let report =
        require_artifact_compatibility(&program_contract, &supporting_consumer)
            .expect("the consumer declares all required features");

    assert_eq!(report.status(), CompatibilityStatus::Compatible);
    assert!(report.issues().is_empty());
}

#[test]
fn repeated_assessment_produces_the_same_status_and_issue_order() {
    let artifact_contract = artifact(
        "classical.ir",
        version(2, 0, 0),
        version(2, 0, 0),
        &[
            "classical.numeric.arbitrary_precision",
            "classical.numeric.checked_arithmetic",
        ],
        &["classical.optimization.optional_fast_path"],
    );

    let consumer_profile = consumer(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
    );

    let first = assess_artifact_compatibility(&artifact_contract, &consumer_profile);
    let second = assess_artifact_compatibility(&artifact_contract, &consumer_profile);

    assert_eq!(first.status(), second.status());
    assert_eq!(first.is_compatible(), second.is_compatible());
    assert_eq!(first.has_blocking_issues(), second.has_blocking_issues());

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
// Numerical semantics and reproducibility policy
// ---------------------------------------------------------------------------

#[test]
fn cross_target_numerical_requirements_remain_explicit_in_configuration() {
    let mut config = ClassicalConfig::new();

    config.determinism = DeterminismPolicy::CrossImplementation;
    config.numerical.float_semantics = FloatSemantics::Strict;
    config.numerical.reproducibility = NumericalReproducibility::CrossTarget;

    let before_validation = config.clone();

    assert!(
        validate_configuration(&config).is_ok(),
        "the cross-target policy combination must pass configuration validation",
    );

    assert_eq!(config, before_validation);
    assert_eq!(config.determinism, DeterminismPolicy::CrossImplementation);
    assert_eq!(config.numerical.float_semantics, FloatSemantics::Strict);
    assert_eq!(
        config.numerical.reproducibility,
        NumericalReproducibility::CrossTarget,
    );
}

#[test]
fn automatic_parallelism_is_a_policy_not_a_fixed_worker_count() {
    let mut config = ClassicalConfig::new();
    config.parallelism = ParallelismPolicy::Auto;

    assert!(validate_configuration(&config).is_ok());
    assert_eq!(config.parallelism, ParallelismPolicy::Auto);
}

// ---------------------------------------------------------------------------
// Operational budgets are not universal semantic limits
// ---------------------------------------------------------------------------

#[test]
fn explicit_budget_is_accepted_without_becoming_a_language_limit() {
    let mut config = ClassicalConfig::new();
    config.budgets.max_memory_bytes = Some(4096);
    config.budgets.max_work_units = Some(20_000);

    assert!(validate_configuration(&config).is_ok());

    assert_eq!(config.budgets.max_memory_bytes, Some(4096));
    assert_eq!(config.budgets.max_work_units, Some(20_000));
}

#[test]
fn absent_operational_budgets_do_not_impose_configuration_level_ceilings() {
    let mut config = ClassicalConfig::new();
    config.budgets = ResourceBudget::unbounded();

    assert!(validate_configuration(&config).is_ok());

    assert_eq!(config.budgets.max_elapsed_time_ms, None);
    assert_eq!(config.budgets.max_memory_bytes, None);
    assert_eq!(config.budgets.max_work_units, None);
    assert_eq!(config.budgets.max_evaluation_steps, None);
    assert_eq!(config.budgets.max_rewrite_steps, None);
    assert_eq!(config.budgets.max_diagnostics, None);
    assert_eq!(config.budgets.max_parallel_tasks, None);
}

#[test]
fn invalid_budget_is_rejected_without_mutating_configuration() {
    let mut config = ClassicalConfig::new();
    config.budgets.max_work_units = Some(0);
    let before_validation = config.clone();

    assert!(
        validate_configuration(&config).is_err(),
        "zero work budget must be rejected by the canonical validator",
    );

    assert_eq!(
        config, before_validation,
        "configuration validation must not mutate caller-owned input",
    );
}
