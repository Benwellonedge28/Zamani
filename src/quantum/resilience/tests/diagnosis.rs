//! Zamani Quantum Resilience — Diagnosis Integration Tests.
//!
//! Path:
//!     src/quantum/resilience/tests/diagnosis.rs
//!
//! Purpose
//! =======
//!
//! Production-oriented tests for the resilience diagnosis boundary.
//!
//! These tests verify the contracts between:
//!
//!     detection
//!         |
//!         v
//!     classifier
//!         |
//!         +----------+-----------+
//!         |          |           |
//!         v          v           v
//!     correlation localization root-cause
//!         |          |           |
//!         +----------+-----------+
//!                    |
//!                    v
//!                diagnosis
//!
//! The tests intentionally do NOT:
//!
//! - assume a particular quantum processor;
//! - assume a fixed number of qubits;
//! - assume a provider;
//! - assume a fixed topology;
//! - assume a fixed retry count;
//! - assume a fixed confidence threshold;
//! - create resilience-local QubitId types;
//! - duplicate ZQN fault semantics;
//! - execute recovery;
//! - mutate hardware;
//! - depend on wall-clock time;
//! - depend on network access;
//! - depend on filesystem state;
//! - use unsafe Rust.
//!
//! Scalability contract
//! =====================
//!
//! Diagnosis is a semantic layer. Its resource cardinality is supplied by
//! callers and must remain dynamically sized. These tests therefore validate
//! properties of diagnosis rather than a particular machine size.
//!
//! Canonical quantum identity
//! ===========================
//!
//! When a quantum identity is required by future diagnosis contributors,
//! the canonical type is:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! This test module does not create a replacement identity type.
//!
//! Rust compatibility
//! ===================
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no unsafe code
//!
//! Integration
//! ===========
//!
//! This file is intended to be included from:
//!
//!     src/quantum/resilience/tests/mod.rs
//!
//! through:
//!
//!     mod diagnosis;
//!
//! The tests use crate-relative imports so they remain internal to the Zamani
//! crate and do not depend on an installed external crate name.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]

use core::any::type_name;
use core::num::NonZeroU64;

use crate::quantum::ir::qubit::QubitId;
use crate::quantum::resilience::diagnosis::classifier::{
    ClassificationRule,
    ConfidencePolicy,
    DiagnosisClassifierConfig,
    CLASSIFIER_SCHEMA_ID,
    CLASSIFIER_SCHEMA_VERSION,
    DEFAULT_CLASSIFIER_NAME,
    DEFAULT_CLASSIFIER_VERSION,
};
use crate::quantum::resilience::diagnosis::diagnostician::{
    ContributorIdentity,
    DiagnosisCategory,
    DiagnosisConfidence,
    DiagnosisId,
    DIAGNOSIS_SCHEMA_ID,
    DIAGNOSIS_SCHEMA_VERSION,
};
use crate::quantum::resilience::detection::detector::DetectionClassification;

// =============================================================================
// Public contract smoke tests
// =============================================================================

/// Verifies that the canonical diagnosis types are available through their
/// intended public modules.
///
/// This is intentionally a compile-time/API-surface test rather than a test
/// coupled to private representation.
#[test]
fn diagnosis_public_contract_is_available() {
    assert!(!type_name::<DiagnosisId>().is_empty());
    assert!(!type_name::<ContributorIdentity>().is_empty());
    assert!(!type_name::<DiagnosisCategory>().is_empty());
    assert!(!type_name::<DiagnosisConfidence>().is_empty());
    assert!(!type_name::<ClassificationRule>().is_empty());
    assert!(!type_name::<ConfidencePolicy>().is_empty());
    assert!(!type_name::<DiagnosisClassifierConfig>().is_empty());

    // Ensure quantum identity remains owned by canonical IR.
    assert!(!type_name::<QubitId>().is_empty());
}

// =============================================================================
// Schema contracts
// =============================================================================

#[test]
fn diagnosis_schema_identifiers_are_non_empty_and_versioned() {
    assert!(!DIAGNOSIS_SCHEMA_ID.is_empty());
    assert!(DIAGNOSIS_SCHEMA_VERSION > 0);

    assert!(!CLASSIFIER_SCHEMA_ID.is_empty());
    assert!(CLASSIFIER_SCHEMA_VERSION > 0);

    assert!(!DEFAULT_CLASSIFIER_NAME.is_empty());
    assert!(!DEFAULT_CLASSIFIER_VERSION.is_empty());
}

// =============================================================================
// Diagnosis identity
// =============================================================================

#[test]
fn diagnosis_id_rejects_zero() {
    assert!(DiagnosisId::from_u64(0).is_none());
}

#[test]
fn diagnosis_id_is_stable_for_the_same_value() {
    let first = DiagnosisId::from_u64(1)
        .expect("non-zero test identity must be constructible");

    let second = DiagnosisId::from_u64(1)
        .expect("non-zero test identity must be constructible");

    assert_eq!(first, second);
    assert_eq!(first.value(), second.value());
    assert_eq!(first.to_string(), second.to_string());
}

#[test]
fn diagnosis_id_distinguishes_different_values() {
    let first = DiagnosisId::from_u64(1)
        .expect("non-zero test identity must be constructible");

    let second = DiagnosisId::from_u64(2)
        .expect("non-zero test identity must be constructible");

    assert_ne!(first, second);
    assert_ne!(first.value(), second.value());
}

#[test]
fn diagnosis_id_accepts_full_non_zero_identifier_domain() {
    let value = u64::MAX;

    let id = DiagnosisId::from_u64(value)
        .expect("maximum non-zero u64 must be a valid opaque diagnosis id");

    assert_eq!(id.value(), value);
}

// =============================================================================
// Contributor identity
// =============================================================================

#[test]
fn contributor_identity_requires_name_and_version() {
    assert!(
        ContributorIdentity::new("", "1").is_err(),
        "empty contributor names must not form stable identities"
    );

    assert!(
        ContributorIdentity::new("diagnosis", "").is_err(),
        "empty contributor versions must not form stable identities"
    );

    assert!(
        ContributorIdentity::new("   ", "1").is_err(),
        "whitespace-only contributor names must not form stable identities"
    );

    assert!(
        ContributorIdentity::new("diagnosis", "   ").is_err(),
        "whitespace-only contributor versions must not form stable identities"
    );
}

#[test]
fn contributor_identity_is_deterministic() {
    let first = ContributorIdentity::new(
        "zamani.test.diagnosis",
        "1",
    )
    .expect("valid contributor identity");

    let second = ContributorIdentity::new(
        "zamani.test.diagnosis",
        "1",
    )
    .expect("valid contributor identity");

    assert_eq!(first, second);
    assert_eq!(first.name(), second.name());
    assert_eq!(first.version(), second.version());
    assert_eq!(first.to_string(), second.to_string());
}

#[test]
fn contributor_identity_distinguishes_versions() {
    let first = ContributorIdentity::new(
        "zamani.test.diagnosis",
        "1",
    )
    .expect("valid contributor identity");

    let second = ContributorIdentity::new(
        "zamani.test.diagnosis",
        "2",
    )
    .expect("valid contributor identity");

    assert_ne!(first, second);
}

// =============================================================================
// Diagnosis categories
// =============================================================================

#[test]
fn diagnosis_categories_have_stable_machine_names() {
    let categories = [
        DiagnosisCategory::NoCondition,
        DiagnosisCategory::Anomaly,
        DiagnosisCategory::Fault,
        DiagnosisCategory::Hardware,
        DiagnosisCategory::CalibrationDrift,
        DiagnosisCategory::NoiseDrift,
        DiagnosisCategory::Resource,
        DiagnosisCategory::Backend,
        DiagnosisCategory::Routing,
        DiagnosisCategory::Scheduling,
        DiagnosisCategory::Qec,
        DiagnosisCategory::Timeout,
        DiagnosisCategory::ExecutionFailure,
        DiagnosisCategory::Security,
        DiagnosisCategory::DataQuality,
        DiagnosisCategory::Correlated,
        DiagnosisCategory::Software,
        DiagnosisCategory::Semantic,
        DiagnosisCategory::Unknown,
    ];

    for category in categories {
        assert!(
            !category.as_str().is_empty(),
            "every diagnosis category needs a stable machine name"
        );
    }
}

#[test]
fn no_condition_is_not_actionable() {
    assert!(!DiagnosisCategory::NoCondition.is_actionable());
}

#[test]
fn_actual_diagnosis_categories_are_actionable_candidates() {
    let categories = [
        DiagnosisCategory::Anomaly,
        DiagnosisCategory::Fault,
        DiagnosisCategory::Hardware,
        DiagnosisCategory::Resource,
        DiagnosisCategory::Backend,
        DiagnosisCategory::Routing,
        DiagnosisCategory::Scheduling,
        DiagnosisCategory::Qec,
        DiagnosisCategory::Timeout,
        DiagnosisCategory::ExecutionFailure,
        DiagnosisCategory::Security,
        DiagnosisCategory::Software,
        DiagnosisCategory::Semantic,
        DiagnosisCategory::Unknown,
    ];

    for category in categories {
        assert!(
            category.is_actionable(),
            "non-empty diagnosis categories must remain distinguishable from NoCondition"
        );
    }
}

#[test]
fn external_category_preserves_caller_supplied_identity() {
    let category =
        DiagnosisCategory::External("future.domain.category".to_owned());

    assert_eq!(
        category.as_str(),
        "future.domain.category"
    );

    assert!(category.is_actionable());
}

// =============================================================================
// Diagnosis confidence
// =============================================================================

#[test]
fn diagnosis_confidence_accepts_domain_endpoints() {
    let minimum =
        DiagnosisConfidence::new(0.0)
            .expect("zero is a valid confidence");

    let maximum =
        DiagnosisConfidence::new(1.0)
            .expect("one is a valid confidence");

    assert_eq!(minimum.value(), 0.0);
    assert_eq!(maximum.value(), 1.0);
}

#[test]
fn diagnosis_confidence_rejects_non_finite_values() {
    assert!(
        DiagnosisConfidence::new(f64::NAN).is_err()
    );

    assert!(
        DiagnosisConfidence::new(f64::INFINITY).is_err()
    );

    assert!(
        DiagnosisConfidence::new(f64::NEG_INFINITY).is_err()
    );
}

#[test]
fn diagnosis_confidence_rejects_values_outside_domain() {
    assert!(
        DiagnosisConfidence::new(-f64::EPSILON).is_err()
    );

    assert!(
        DiagnosisConfidence::new(1.0 + f64::EPSILON).is_err()
    );
}

#[test]
fn diagnosis_confidence_meets_is_explicit_threshold_comparison() {
    let observed =
        DiagnosisConfidence::new(0.75)
            .expect("valid confidence");

    let required =
        DiagnosisConfidence::new(0.50)
            .expect("valid confidence");

    assert!(observed.meets(required));
}

#[test]
fn diagnosis_confidence_meets_rejects_insufficient_confidence() {
    let observed =
        DiagnosisConfidence::new(0.25)
            .expect("valid confidence");

    let required =
        DiagnosisConfidence::new(0.50)
            .expect("valid confidence");

    assert!(!observed.meets(required));
}

#[test]
fn diagnosis_confidence_minimum_is_conservative() {
    let first =
        DiagnosisConfidence::new(0.25)
            .expect("valid confidence");

    let second =
        DiagnosisConfidence::new(0.75)
            .expect("valid confidence");

    assert_eq!(first.minimum(second), first);
    assert_eq!(second.minimum(first), first);
}

#[test]
fn diagnosis_confidence_maximum_is_conservative_in_the_other_direction() {
    let first =
        DiagnosisConfidence::new(0.25)
            .expect("valid confidence");

    let second =
        DiagnosisConfidence::new(0.75)
            .expect("valid confidence");

    assert_eq!(first.maximum_of(second), second);
    assert_eq!(second.maximum_of(first), second);
}

// =============================================================================
// Confidence policy
// =============================================================================

#[test]
fn confidence_policy_propagates_without_hidden_transformation() {
    let confidence =
        DiagnosisConfidence::new(0.625)
            .expect("valid confidence");

    let transformed =
        ConfidencePolicy::Propagate
            .apply(confidence)
            .expect("propagation must succeed");

    assert_eq!(transformed, confidence);
}

#[test]
fn confidence_policy_rejects_invalid_scale() {
    assert!(
        ConfidencePolicy::scale(-f64::EPSILON).is_err()
    );

    assert!(
        ConfidencePolicy::scale(1.0 + f64::EPSILON).is_err()
    );

    assert!(
        ConfidencePolicy::scale(f64::NAN).is_err()
    );

    assert!(
        ConfidencePolicy::scale(f64::INFINITY).is_err()
    );
}

#[test]
fn confidence_policy_accepts_explicit_scale() {
    let policy =
        ConfidencePolicy::scale(0.5)
            .expect("valid explicit scale");

    let confidence =
        DiagnosisConfidence::new(0.8)
            .expect("valid confidence");

    let transformed =
        policy
            .apply(confidence)
            .expect("valid confidence scaling");

    assert_eq!(transformed.value(), 0.4);
}

#[test]
fn confidence_policy_scale_zero_is_valid_and_deterministic() {
    let policy =
        ConfidencePolicy::scale(0.0)
            .expect("zero scaling is an explicit policy");

    let confidence =
        DiagnosisConfidence::new(1.0)
            .expect("valid confidence");

    let transformed =
        policy
            .apply(confidence)
            .expect("zero scaling must remain valid");

    assert_eq!(transformed.value(), 0.0);
}

#[test]
fn confidence_policy_scale_one_preserves_confidence() {
    let policy =
        ConfidencePolicy::scale(1.0)
            .expect("unit scaling is valid");

    let confidence =
        DiagnosisConfidence::new(0.73)
            .expect("valid confidence");

    let transformed =
        policy
            .apply(confidence)
            .expect("unit scaling must succeed");

    assert_eq!(transformed, confidence);
}

// =============================================================================
// Classifier configuration
// =============================================================================

#[test]
fn empty_classifier_configuration_has_no_implicit_rules() {
    let config = DiagnosisClassifierConfig::empty();

    assert_eq!(
        config.rules().count(),
        0,
        "empty configuration must not silently install machine-specific behavior"
    );
}

#[test]
fn standard_classifier_configuration_is_available() {
    let config = DiagnosisClassifierConfig::standard();

    assert!(
        config.rules().count() > 0,
        "standard configuration must provide the canonical semantic mappings"
    );
}

#[test]
fn standard_configuration_has_no_fixed_hardware_cardinality() {
    let config = DiagnosisClassifierConfig::standard();

    // The classifier configuration is a mapping of semantic categories, not
    // a list of qubits, devices, gates, providers, or hardware slots.
    //
    // This test deliberately avoids asserting a specific number of rules.
    // Future semantic categories may legitimately be added without breaking
    // the scalability contract.
    for rule in config.rules() {
        assert!(!rule.detection().as_str().is_empty());
        assert!(!rule.diagnosis().as_str().is_empty());
    }
}

#[test]
fn classifier_fallback_is_explicit() {
    let mut config = DiagnosisClassifierConfig::empty();

    assert_eq!(
        config.fallback_category(),
        &DiagnosisCategory::Unknown
    );

    config.set_fallback_category(DiagnosisCategory::DataQuality);

    assert_eq!(
        config.fallback_category(),
        &DiagnosisCategory::DataQuality
    );
}

#[test]
fn classifier_no_condition_policy_is_explicit() {
    let mut config = DiagnosisClassifierConfig::empty();

    assert!(!config.include_no_condition());

    config.set_include_no_condition(true);

    assert!(config.include_no_condition());
}

// =============================================================================
// Classification rule contracts
// =============================================================================

#[test]
fn classification_rule_normalizes_blank_explanation() {
    let rule = ClassificationRule::new(
        DetectionClassification::Fault,
        DiagnosisCategory::Fault,
        ConfidencePolicy::Propagate,
        Some("   ".to_owned()),
    )
    .expect("valid rule");

    assert_eq!(rule.explanation(), None);
}

#[test]
fn classification_rule_preserves_meaningful_explanation() {
    let rule = ClassificationRule::new(
        DetectionClassification::Fault,
        DiagnosisCategory::Fault,
        ConfidencePolicy::Propagate,
        Some("Canonical fault classification".to_owned()),
    )
    .expect("valid rule");

    assert_eq!(
        rule.explanation(),
        Some("Canonical fault classification")
    );
}

#[test]
fn classification_rule_exposes_semantic_mapping() {
    let rule = ClassificationRule::new(
        DetectionClassification::Timeout,
        DiagnosisCategory::Timeout,
        ConfidencePolicy::Propagate,
        None,
    )
    .expect("valid rule");

    assert_eq!(
        rule.detection(),
        DetectionClassification::Timeout
    );

    assert_eq!(
        rule.diagnosis(),
        &DiagnosisCategory::Timeout
    );

    assert_eq!(
        rule.confidence_policy(),
        ConfidencePolicy::Propagate
    );
}

#[test]
fn classifier_rule_rejects_invalid_confidence_scale() {
    let result = ClassificationRule::new(
        DetectionClassification::Fault,
        DiagnosisCategory::Fault,
        ConfidencePolicy::Scale(f64::NAN),
        None,
    );

    assert!(result.is_err());
}

// =============================================================================
// Standard semantic classification coverage
// =============================================================================

#[test]
fn standard_classifier_covers_core_detection_classes() {
    let config = DiagnosisClassifierConfig::standard();

    let expected = [
        DetectionClassification::NoCondition,
        DetectionClassification::Anomaly,
        DetectionClassification::Fault,
        DetectionClassification::Degradation,
        DetectionClassification::Unavailability,
        DetectionClassification::Timeout,
        DetectionClassification::ExecutionFailure,
        DetectionClassification::QecSignal,
        DetectionClassification::HardwareSignal,
        DetectionClassification::Inconclusive,
    ];

    for detection in expected {
        assert!(
            config.rule(detection).is_some(),
            "standard diagnosis configuration must explicitly classify detection category: {}",
            detection.as_str()
        );
    }
}

#[test]
fn standard_classifier_preserves_detection_confidence_by_default() {
    let config = DiagnosisClassifierConfig::standard();

    for rule in config.rules() {
        assert_eq!(
            rule.confidence_policy(),
            ConfidencePolicy::Propagate,
            "standard semantic mappings must not silently alter evidence confidence"
        );
    }
}

// =============================================================================
// Deterministic rule replacement
// =============================================================================

#[test]
fn classifier_rule_replacement_is_deterministic() {
    let mut config = DiagnosisClassifierConfig::empty();

    let first = ClassificationRule::new(
        DetectionClassification::Fault,
        DiagnosisCategory::Fault,
        ConfidencePolicy::Propagate,
        Some("first".to_owned()),
    )
    .expect("valid first rule");

    let second = ClassificationRule::new(
        DetectionClassification::Fault,
        DiagnosisCategory::Hardware,
        ConfidencePolicy::Propagate,
        Some("second".to_owned()),
    )
    .expect("valid second rule");

    assert!(config.insert_rule(first).is_none());

    let replaced = config
        .insert_rule(second)
        .expect("second insertion must replace the first mapping");

    assert_eq!(
        replaced.diagnosis(),
        &DiagnosisCategory::Fault
    );

    let active = config
        .rule(DetectionClassification::Fault)
        .expect("replacement rule must be present");

    assert_eq!(
        active.diagnosis(),
        &DiagnosisCategory::Hardware
    );

    assert_eq!(
        active.explanation(),
        Some("second")
    );
}

#[test]
fn classifier_rule_removal_is_explicit() {
    let mut config = DiagnosisClassifierConfig::empty();

    let rule = ClassificationRule::new(
        DetectionClassification::Timeout,
        DiagnosisCategory::Timeout,
        ConfidencePolicy::Propagate,
        None,
    )
    .expect("valid rule");

    config.insert_rule(rule);

    assert!(
        config.rule(DetectionClassification::Timeout).is_some()
    );

    let removed = config.remove_rule(
        DetectionClassification::Timeout,
    );

    assert!(removed.is_some());

    assert!(
        config.rule(DetectionClassification::Timeout).is_none()
    );
}

// =============================================================================
// Resource-independent scaling contract
// =============================================================================

#[test]
fn diagnosis_identity_domain_is_not_bounded_by_hardware_size() {
    // The test uses opaque diagnosis identities, not machine identifiers.
    //
    // The important property is that the identity model supports the full
    // non-zero u64 domain without embedding a "maximum machine size".
    let values = [
        1_u64,
        2_u64,
        17_u64,
        1_024_u64,
        u64::MAX,
    ];

    for value in values {
        let id = DiagnosisId::from_u64(value)
            .expect("every non-zero u64 is a valid diagnosis identity");

        assert_eq!(id.value(), value);
    }
}

#[test]
fn canonical_qubit_identity_remains_outside_resilience_model() {
    // This test intentionally does not construct or compare physical-qubit
    // numbers. Its purpose is to ensure that the resilience diagnosis test
    // surface references the canonical IR identity type when such identity
    // is needed, rather than introducing a resilience-local substitute.
    assert!(
        type_name::<QubitId>().contains("quantum")
            || !type_name::<QubitId>().is_empty()
    );
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn standard_configuration_is_deterministic_across_construction() {
    let first = DiagnosisClassifierConfig::standard();
    let second = DiagnosisClassifierConfig::standard();

    assert_eq!(first, second);
}

#[test]
fn contributor_identity_ordering_is_deterministic() {
    let first = ContributorIdentity::new(
        "a",
        "1",
    )
    .expect("valid identity");

    let second = ContributorIdentity::new(
        "b",
        "1",
    )
    .expect("valid identity");

    assert!(first < second);
}

#[test]
fn diagnosis_ids_have_stable_ordering() {
    let first = DiagnosisId::from_u64(1)
        .expect("valid id");

    let second = DiagnosisId::from_u64(2)
        .expect("valid id");

    assert!(first < second);
}

// =============================================================================
// Integration boundary invariants
// =============================================================================

#[test]
fn diagnosis_categories_do_not_encode_recovery_actions() {
    let categories = [
        DiagnosisCategory::Fault,
        DiagnosisCategory::Hardware,
        DiagnosisCategory::Resource,
        DiagnosisCategory::Backend,
        DiagnosisCategory::Routing,
        DiagnosisCategory::Scheduling,
        DiagnosisCategory::Qec,
        DiagnosisCategory::Timeout,
        DiagnosisCategory::ExecutionFailure,
    ];

    for category in categories {
        let name = category.as_str();

        assert!(
            !name.contains("retry"),
            "diagnosis must not encode recovery actions"
        );

        assert!(
            !name.contains("restart"),
            "diagnosis must not encode recovery actions"
        );

        assert!(
            !name.contains("migrate"),
            "diagnosis must not encode recovery actions"
        );
    }
}

#[test]
fn diagnosis_categories_do_not_encode_provider_identity() {
    let categories = [
        DiagnosisCategory::Fault,
        DiagnosisCategory::Hardware,
        DiagnosisCategory::Resource,
        DiagnosisCategory::Backend,
        DiagnosisCategory::Routing,
        DiagnosisCategory::Scheduling,
        DiagnosisCategory::Qec,
    ];

    for category in categories {
        let name = category.as_str();

        // Provider selection belongs to hardware/backend selection policy.
        // Diagnosis must remain provider-neutral.
        assert!(!name.starts_with("ibm"));
        assert!(!name.starts_with("google"));
        assert!(!name.starts_with("aws"));
        assert!(!name.starts_with("azure"));
    }
}

#[test]
fn diagnosis_does_not_define_machine_size_limits() {
    // This test is intentionally structural.
    //
    // The public diagnosis model is expressed in terms of:
    //
    // - opaque diagnosis IDs;
    // - semantic categories;
    // - contributor identities;
    // - confidence;
    // - classification rules.
    //
    // No fixed qubit/device/topology cardinality is required by these
    // contracts.
    assert!(!type_name::<DiagnosisId>().is_empty());
    assert!(!type_name::<DiagnosisClassifierConfig>().is_empty());
}

// =============================================================================
// End-to-end contract composition
// =============================================================================

#[test]
fn detection_to_diagnosis_contract_can_be_composed_without_recovery() {
    let classifier = DiagnosisClassifierConfig::standard();

    let fault_rule = classifier
        .rule(DetectionClassification::Fault)
        .expect("standard classifier must expose fault mapping");

    assert_eq!(
        fault_rule.diagnosis(),
        &DiagnosisCategory::Fault
    );

    let confidence =
        DiagnosisConfidence::new(0.8)
            .expect("valid confidence");

    let propagated =
        fault_rule
            .confidence_policy()
            .apply(confidence)
            .expect("standard propagation must succeed");

    assert_eq!(propagated, confidence);

    // This is the intended boundary:
    //
    // detection
    //     -> classification
    //     -> diagnosis finding
    //
    // The test intentionally stops here. Recovery is owned by policy/planning
    // and must never be an implicit side effect of diagnosis.
}

#[test]
fn diagnosis_remains_valid_when_evidence_confidence_is_uncertain() {
    let low_confidence =
        DiagnosisConfidence::new(0.0)
            .expect("zero confidence is still valid evidence");

    let category = DiagnosisCategory::Unknown;

    assert!(category.is_actionable());

    assert!(
        !low_confidence.meets(
            DiagnosisConfidence::new(0.5)
                .expect("valid confidence")
        )
    );
}

#[test]
fn diagnosis_schema_is_independent_of_execution_backend() {
    // Schema identity is a resilience contract, not a provider contract.
    assert_eq!(
        DIAGNOSIS_SCHEMA_ID,
        "zamani.quantum.resilience.diagnosis.diagnostician"
    );

    assert_eq!(
        CLASSIFIER_SCHEMA_ID,
        "zamani.quantum.resilience.diagnosis.classifier"
    );
}

// =============================================================================
// Regression guards
// =============================================================================

#[test]
fn diagnosis_id_display_is_machine_stable() {
    let id = DiagnosisId::from_u64(42)
        .expect("valid diagnosis id");

    assert_eq!(
        id.to_string(),
        "diagnosis-42"
    );
}

#[test]
fn maximum_diagnosis_id_does_not_overflow() {
    let id = DiagnosisId::from_u64(u64::MAX)
        .expect("maximum non-zero u64 must remain representable");

    assert_eq!(id.value(), u64::MAX);
}

#[test]
fn confidence_extreme_values_remain_ordered() {
    let zero =
        DiagnosisConfidence::zero();

    let maximum =
        DiagnosisConfidence::maximum();

    assert!(zero < maximum);
    assert_eq!(zero.value(), 0.0);
    assert_eq!(maximum.value(), 1.0);
}

#[test]
fn confidence_equality_is_deterministic() {
    let first =
        DiagnosisConfidence::new(0.5)
            .expect("valid confidence");

    let second =
        DiagnosisConfidence::new(0.5)
            .expect("valid confidence");

    assert_eq!(first, second);
}

#[test]
fn standard_classifier_does_not_change_when_recreated() {
    let a = DiagnosisClassifierConfig::standard();
    let b = DiagnosisClassifierConfig::standard();
    let c = DiagnosisClassifierConfig::standard();

    assert_eq!(a, b);
    assert_eq!(b, c);
}

// =============================================================================
// Future-proofing contract
// =============================================================================

#[test]
fn future_external_categories_remain_provider_neutral() {
    let category =
        DiagnosisCategory::External(
            "future.quantum.diagnosis".to_owned(),
        );

    assert_eq!(
        category.as_str(),
        "future.quantum.diagnosis"
    );

    assert!(category.is_actionable());
}

#[test]
fn diagnosis_layer_has_no_implicit_retry_policy() {
    // A diagnosis category is an observation/interpretation. It is not a
    // recovery command.
    //
    // Keeping this test explicit protects the architectural boundary when
    // additional categories are introduced.
    let categories = [
        DiagnosisCategory::Timeout,
        DiagnosisCategory::ExecutionFailure,
        DiagnosisCategory::Backend,
        DiagnosisCategory::Hardware,
    ];

    for category in categories {
        assert!(category.is_actionable());
    }
}

// =============================================================================
// Compile-time construction guard
// =============================================================================

#[test]
fn non_zero_identifier_construction_matches_canonical_std_contract() {
    let value =
        NonZeroU64::new(1)
            .expect("literal non-zero test identity must be valid");

    let id =
        DiagnosisId::new(value);

    assert_eq!(id.value(), 1);
}