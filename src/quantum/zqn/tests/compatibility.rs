#![forbid(unsafe_code)]

//! # ZQN Compatibility Integration Tests
//!
//! Production-level compatibility tests for the Zamani Quantum Noise (ZQN)
//! serialization/versioning boundary.
//!
//! ## Ownership
//!
//! This file owns tests for the compatibility contract exposed by:
//!
//! `crate::quantum::zqn::io::compatibility`
//!
//! It verifies:
//!
//! - schema-version ordering;
//! - compatibility direction;
//! - identity compatibility;
//! - explicit migration registration;
//! - deterministic migration;
//! - multi-step migration;
//! - migration-path determinism;
//! - migration semantic policies;
//! - resource limits;
//! - migration failures;
//! - malformed migration identifiers;
//! - duplicate/invalid migration handling;
//! - migration isolation;
//! - reproducibility of equivalent compatibility executions;
//! - absence of quantum-machine-size assumptions.
//!
//! ## Does not own
//!
//! This file does not implement:
//!
//! - schema validation;
//! - serialization;
//! - deserialization;
//! - canonical encoding;
//! - migration algorithms;
//! - quantum IR;
//! - qubit identity;
//! - hardware compatibility;
//! - target capability negotiation;
//! - numerical channel compatibility.
//!
//! Those responsibilities remain in their respective production modules.
//!
//! ## Architectural boundary
//!
//! ```text
//!                         ZQN
//!                          |
//!                +---------+---------+
//!                |                   |
//!             schema            serialization
//!                |                   |
//!                +---------+---------+
//!                          |
//!                   compatibility
//!                          |
//!                    this test file
//!                          |
//!             +------------+-------------+
//!             |            |             |
//!          version      migration      limits
//! ```
//!
//! ## Critical invariant
//!
//! Compatibility is a schema concern, not a quantum-system-size concern.
//!
//! These tests deliberately never encode:
//!
//! - a maximum qubit count;
//! - a maximum number of gates;
//! - a maximum topology size;
//! - a particular backend;
//! - a vendor;
//! - a fixed gate arity;
//! - a fixed quantum representation.
//!
//! Resource limits tested here are operational safety policies only.
//!
//! ## Determinism
//!
//! The same:
//!
//! ```text
//! registry
//! source version
//! target version
//! input document
//! policy
//! limits
//! ```
//!
//! must produce the same migration result.
//!
//! Migration path selection must not depend on hash-map iteration order,
//! thread scheduling, process identity, or global mutable state.
//!
//! The production compatibility module uses ordered collections for this
//! purpose. The tests below intentionally exercise equivalent paths to ensure
//! that the observable result remains deterministic.
//!
//! ## Qubit identity
//!
//! Compatibility tests do not invent a ZQN-specific qubit identifier.
//!
//! When a document contains quantum resource identifiers, those identifiers
//! remain owned by:
//!
//! `crate::quantum::ir::qubit::{QubitId, PhysicalQubitId}`
//!
//! A schema migration must not reinterpret or renumber those identifiers merely
//! because a schema version changed.
//!
//! ## Rust
//!
//! Target language version:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//!
//! The tests use only stable standard-library functionality available to those
//! toolchains.
//!
//! ## Integration contract
//!
//! `io::compatibility` must remain responsible for:
//!
//! 1. determining whether a migration is required;
//! 2. selecting an explicit migration path;
//! 3. applying registered migrations;
//! 4. enforcing compatibility resource policies;
//! 5. rejecting unsupported migration paths;
//! 6. rejecting migrations that violate the selected semantic policy.
//!
//! `io::schema` remains responsible for validating the resulting document.
//!
//! `io::serialization` and `io::deserialization` remain responsible for bytes.
//!
//! `io::canonical` remains responsible for canonical representation.
//!
//! ## Production requirement
//!
//! A test passing here does not mean arbitrary historical schemas are
//! automatically supported. Only explicitly registered migration paths are
//! supported.
//!
//! Silent best-effort conversion is intentionally forbidden.
//!

use serde_json::{json, Value};

use crate::quantum::zqn::io::compatibility::{
    CompatibilityDirection,
    CompatibilityLimits,
    CompatibilityError,
    Migration,
    MigrationError,
    MigrationPolicy,
    MigrationRegistry,
    MigrationSemantics,
    SchemaVersion,
};

/// Builds a deterministic lossless migration from schema `1` to `2`.
fn migration_1_to_2(document: Value) -> Result<Value, MigrationError> {
    let mut document = document;

    if let Some(object) = document.as_object_mut() {
        object.insert("schema_version".to_owned(), json!(2));
        object.insert("compatibility_marker".to_owned(), json!("v2"));
    }

    Ok(document)
}

/// Builds a deterministic lossless migration from schema `2` to `3`.
fn migration_2_to_3(document: Value) -> Result<Value, MigrationError> {
    let mut document = document;

    if let Some(object) = document.as_object_mut() {
        object.insert("schema_version".to_owned(), json!(3));
        object.insert("compatibility_marker".to_owned(), json!("v3"));
    }

    Ok(document)
}

/// Builds a deterministic lossless migration from schema `3` to `4`.
fn migration_3_to_4(document: Value) -> Result<Value, MigrationError> {
    let mut document = document;

    if let Some(object) = document.as_object_mut() {
        object.insert("schema_version".to_owned(), json!(4));
        object.insert("compatibility_marker".to_owned(), json!("v4"));
    }

    Ok(document)
}

/// Migration deliberately failing for error-propagation tests.
fn failing_migration(_document: Value) -> Result<Value, MigrationError> {
    Err(MigrationError::Failed {
        message: "intentional compatibility test failure".to_owned(),
    })
}

/// Migration that expands the document substantially.
///
/// The production compatibility layer must account for output growth when
/// output-size limits are configured.
fn expanding_migration(document: Value) -> Result<Value, MigrationError> {
    let mut document = document;

    if let Some(object) = document.as_object_mut() {
        object.insert(
            "expanded_payload".to_owned(),
            Value::String("compatibility-output-growth-test".repeat(128)),
        );

        object.insert("schema_version".to_owned(), json!(2));
    }

    Ok(document)
}

/// Returns a registry containing a canonical linear migration chain.
fn linear_registry() -> MigrationRegistry {
    let mut registry = MigrationRegistry::new();

    registry
        .register(Migration::new(
            SchemaVersion::new(1),
            SchemaVersion::new(2),
            "test.v1-to-v2",
            migration_1_to_2,
        ))
        .expect("v1 -> v2 migration registration must succeed");

    registry
        .register(Migration::new(
            SchemaVersion::new(2),
            SchemaVersion::new(3),
            "test.v2-to-v3",
            migration_2_to_3,
        ))
        .expect("v2 -> v3 migration registration must succeed");

    registry
        .register(Migration::new(
            SchemaVersion::new(3),
            SchemaVersion::new(4),
            "test.v3-to-v4",
            migration_3_to_4,
        ))
        .expect("v3 -> v4 migration registration must succeed");

    registry
}

/// Returns the same logical registry with migrations registered in a different
/// order.
///
/// Compatibility must not depend on registration order.
fn reordered_linear_registry() -> MigrationRegistry {
    let mut registry = MigrationRegistry::new();

    registry
        .register(Migration::new(
            SchemaVersion::new(3),
            SchemaVersion::new(4),
            "test.v3-to-v4",
            migration_3_to_4,
        ))
        .expect("v3 -> v4 migration registration must succeed");

    registry
        .register(Migration::new(
            SchemaVersion::new(1),
            SchemaVersion::new(2),
            "test.v1-to-v2",
            migration_1_to_2,
        ))
        .expect("v1 -> v2 migration registration must succeed");

    registry
        .register(Migration::new(
            SchemaVersion::new(2),
            SchemaVersion::new(3),
            "test.v2-to-v3",
            migration_2_to_3,
        ))
        .expect("v2 -> v3 migration registration must succeed");

    registry
}

#[test]
fn schema_version_ordering_is_total_and_deterministic() {
    let v0 = SchemaVersion::new(0);
    let v1 = SchemaVersion::new(1);
    let v2 = SchemaVersion::new(2);
    let v64 = SchemaVersion::new(64);
    let vmax = SchemaVersion::new(u64::MAX);

    assert!(v0 < v1);
    assert!(v1 < v2);
    assert!(v2 < v64);
    assert!(v64 < vmax);

    assert_eq!(v0.get(), 0);
    assert_eq!(vmax.get(), u64::MAX);

    assert!(v0.is_initial());
    assert!(!v1.is_initial());
    assert!(!vmax.is_initial());

    assert_eq!(v2, SchemaVersion::new(2));
}

#[test]
fn compatibility_direction_is_correct() {
    let v1 = SchemaVersion::new(1);
    let v2 = SchemaVersion::new(2);

    assert_eq!(
        CompatibilityDirection::between(v1, v1),
        CompatibilityDirection::Identity
    );

    assert_eq!(
        CompatibilityDirection::between(v1, v2),
        CompatibilityDirection::Forward
    );

    assert_eq!(
        CompatibilityDirection::between(v2, v1),
        CompatibilityDirection::Backward
    );
}

#[test]
fn identity_migration_preserves_document_exactly() {
    let registry = MigrationRegistry::new();

    let document = json!({
        "schema_version": 7,
        "program": {
            "resources": [
                "q0",
                "q1"
            ]
        },
        "noise": {
            "model": "example"
        }
    });

    let migrated = registry
        .migrate(
            document.clone(),
            SchemaVersion::new(7),
            SchemaVersion::new(7),
            &CompatibilityLimits::unlimited(),
        )
        .expect("identity compatibility must succeed");

    assert_eq!(migrated, document);
}

#[test]
fn single_lossless_migration_is_applied() {
    let registry = {
        let mut registry = MigrationRegistry::new();

        registry
            .register(Migration::new(
                SchemaVersion::new(1),
                SchemaVersion::new(2),
                "test.v1-to-v2",
                migration_1_to_2,
            ))
            .expect("migration registration must succeed");

        registry
    };

    let document = json!({
        "schema_version": 1,
        "payload": "stable"
    });

    let migrated = registry
        .migrate(
            document,
            SchemaVersion::new(1),
            SchemaVersion::new(2),
            &CompatibilityLimits::unlimited(),
        )
        .expect("migration must succeed");

    assert_eq!(migrated["schema_version"], json!(2));
    assert_eq!(migrated["compatibility_marker"], json!("v2"));
    assert_eq!(migrated["payload"], json!("stable"));
}

#[test]
fn multi_step_migration_reaches_requested_schema() {
    let registry = linear_registry();

    let document = json!({
        "schema_version": 1,
        "payload": {
            "value": 42
        }
    });

    let migrated = registry
        .migrate(
            document,
            SchemaVersion::new(1),
            SchemaVersion::new(4),
            &CompatibilityLimits::unlimited(),
        )
        .expect("multi-step migration must succeed");

    assert_eq!(migrated["schema_version"], json!(4));
    assert_eq!(migrated["compatibility_marker"], json!("v4"));
    assert_eq!(migrated["payload"]["value"], json!(42));
}

#[test]
fn migration_is_deterministic_for_identical_input() {
    let registry = linear_registry();

    let document = json!({
        "schema_version": 1,
        "program": {
            "operations": [
                "prepare",
                "gate",
                "measure"
            ]
        }
    });

    let first = registry
        .migrate(
            document.clone(),
            SchemaVersion::new(1),
            SchemaVersion::new(4),
            &CompatibilityLimits::unlimited(),
        )
        .expect("first migration must succeed");

    let second = registry
        .migrate(
            document,
            SchemaVersion::new(1),
            SchemaVersion::new(4),
            &CompatibilityLimits::unlimited(),
        )
        .expect("second migration must succeed");

    assert_eq!(first, second);
}

#[test]
fn migration_result_is_independent_of_registration_order() {
    let first_registry = linear_registry();
    let second_registry = reordered_linear_registry();

    let document = json!({
        "schema_version": 1,
        "payload": "determinism"
    });

    let first = first_registry
        .migrate(
            document.clone(),
            SchemaVersion::new(1),
            SchemaVersion::new(4),
            &CompatibilityLimits::unlimited(),
        )
        .expect("first registry migration must succeed");

    let second = second_registry
        .migrate(
            document,
            SchemaVersion::new(1),
            SchemaVersion::new(4),
            &CompatibilityLimits::unlimited(),
        )
        .expect("second registry migration must succeed");

    assert_eq!(first, second);
}

#[test]
fn migration_does_not_mutate_original_input() {
    let registry = {
        let mut registry = MigrationRegistry::new();

        registry
            .register(Migration::new(
                SchemaVersion::new(1),
                SchemaVersion::new(2),
                "test.v1-to-v2",
                migration_1_to_2,
            ))
            .expect("migration registration must succeed");

        registry
    };

    let original = json!({
        "schema_version": 1,
        "payload": {
            "unchanged": true
        }
    });

    let original_snapshot = original.clone();

    let migrated = registry
        .migrate(
            original.clone(),
            SchemaVersion::new(1),
            SchemaVersion::new(2),
            &CompatibilityLimits::unlimited(),
        )
        .expect("migration must succeed");

    assert_eq!(original, original_snapshot);
    assert_ne!(migrated, original);
}

#[test]
fn migration_path_is_explicit() {
    let registry = MigrationRegistry::new();

    let document = json!({
        "schema_version": 1,
        "payload": "no-path"
    });

    let result = registry.migrate(
        document,
        SchemaVersion::new(1),
        SchemaVersion::new(2),
        &CompatibilityLimits::unlimited(),
    );

    assert!(
        result.is_err(),
        "compatibility must reject an absent migration path"
    );
}

#[test]
fn disconnected_schema_versions_are_not_silently_converted() {
    let registry = {
        let mut registry = MigrationRegistry::new();

        registry
            .register(Migration::new(
                SchemaVersion::new(1),
                SchemaVersion::new(2),
                "test.v1-to-v2",
                migration_1_to_2,
            ))
            .expect("migration registration must succeed");

        registry
    };

    let document = json!({
        "schema_version": 1
    });

    let result = registry.migrate(
        document,
        SchemaVersion::new(1),
        SchemaVersion::new(7),
        &CompatibilityLimits::unlimited(),
    );

    assert!(
        result.is_err(),
        "unsupported schema conversion must fail explicitly"
    );
}

#[test]
fn semantic_policy_distinguishes_lossless_and_lossy_migrations() {
    let mut registry = MigrationRegistry::new();

    registry
        .register(Migration::with_semantics(
            SchemaVersion::new(1),
            SchemaVersion::new(2),
            "test.lossy",
            MigrationSemantics::Lossy,
            migration_1_to_2,
        ))
        .expect("lossy migration registration must succeed");

    let document = json!({
        "schema_version": 1,
        "payload": "lossy"
    });

    let lossless_only = registry.migrate_with_policy(
        document.clone(),
        SchemaVersion::new(1),
        SchemaVersion::new(2),
        MigrationPolicy::LosslessOnly,
        &CompatibilityLimits::unlimited(),
    );

    assert!(
        lossless_only.is_err(),
        "lossy migration must not pass LosslessOnly policy"
    );

    let allow_lossy = registry.migrate_with_policy(
        document,
        SchemaVersion::new(1),
        SchemaVersion::new(2),
        MigrationPolicy::AllowLossy,
        &CompatibilityLimits::unlimited(),
    );

    assert!(
        allow_lossy.is_ok(),
        "explicit AllowLossy policy must permit a registered lossy migration"
    );
}

#[test]
fn semantic_change_requires_explicit_policy() {
    let mut registry = MigrationRegistry::new();

    registry
        .register(Migration::with_semantics(
            SchemaVersion::new(1),
            SchemaVersion::new(2),
            "test.semantic-change",
            MigrationSemantics::SemanticChange,
            migration_1_to_2,
        ))
        .expect("semantic-change migration registration must succeed");

    let document = json!({
        "schema_version": 1,
        "payload": "semantic-change"
    });

    let default_restrictive_result = registry.migrate_with_policy(
        document.clone(),
        SchemaVersion::new(1),
        SchemaVersion::new(2),
        MigrationPolicy::AllowLossy,
        &CompatibilityLimits::unlimited(),
    );

    assert!(
        default_restrictive_result.is_err(),
        "semantic changes must not pass AllowLossy policy"
    );

    let explicit_result = registry.migrate_with_policy(
        document,
        SchemaVersion::new(1),
        SchemaVersion::new(2),
        MigrationPolicy::AllowSemanticChange,
        &CompatibilityLimits::unlimited(),
    );

    assert!(
        explicit_result.is_ok(),
        "explicit semantic-change policy must permit the registered migration"
    );
}

#[test]
fn migration_failure_is_propagated() {
    let mut registry = MigrationRegistry::new();

    registry
        .register(Migration::new(
            SchemaVersion::new(1),
            SchemaVersion::new(2),
            "test.failing",
            failing_migration,
        ))
        .expect("migration registration must succeed");

    let document = json!({
        "schema_version": 1,
        "payload": "failure"
    });

    let result = registry.migrate(
        document,
        SchemaVersion::new(1),
        SchemaVersion::new(2),
        &CompatibilityLimits::unlimited(),
    );

    assert!(
        result.is_err(),
        "migration implementation failures must propagate"
    );
}

#[test]
fn migration_step_limit_is_enforced() {
    let registry = linear_registry();

    let limits = CompatibilityLimits {
        max_migration_steps: Some(1),
        ..CompatibilityLimits::unlimited()
    };

    let document = json!({
        "schema_version": 1,
        "payload": "steps"
    });

    let result = registry.migrate(
        document,
        SchemaVersion::new(1),
        SchemaVersion::new(4),
        &limits,
    );

    assert!(
        result.is_err(),
        "multi-step migration must fail when the step budget is exhausted"
    );
}

#[test]
fn migration_path_length_limit_is_enforced() {
    let registry = linear_registry();

    let limits = CompatibilityLimits {
        max_path_length: Some(2),
        ..CompatibilityLimits::unlimited()
    };

    let document = json!({
        "schema_version": 1
    });

    let result = registry.migrate(
        document,
        SchemaVersion::new(1),
        SchemaVersion::new(4),
        &limits,
    );

    assert!(
        result.is_err(),
        "migration must fail when the path-length budget is too small"
    );
}

#[test]
fn path_search_limit_is_enforced() {
    let registry = linear_registry();

    let limits = CompatibilityLimits {
        max_path_search_states: Some(0),
        ..CompatibilityLimits::unlimited()
    };

    let document = json!({
        "schema_version": 1
    });

    let result = registry.migrate(
        document,
        SchemaVersion::new(1),
        SchemaVersion::new(4),
        &limits,
    );

    assert!(
        result.is_err(),
        "path search must respect its explicit resource budget"
    );
}

#[test]
fn registered_migration_limit_is_enforced() {
    let mut registry = MigrationRegistry::new();

    registry
        .register(Migration::new(
            SchemaVersion::new(1),
            SchemaVersion::new(2),
            "test.v1-to-v2",
            migration_1_to_2,
        ))
        .expect("migration registration must succeed");

    let limits = CompatibilityLimits {
        max_registered_migrations: Some(0),
        ..CompatibilityLimits::unlimited()
    };

    let result = registry.migrate(
        json!({
            "schema_version": 1
        }),
        SchemaVersion::new(1),
        SchemaVersion::new(2),
        &limits,
    );

    assert!(
        result.is_err(),
        "registry size/resource policy must be enforceable"
    );
}

#[test]
fn output_growth_limit_is_enforced() {
    let mut registry = MigrationRegistry::new();

    registry
        .register(Migration::new(
            SchemaVersion::new(1),
            SchemaVersion::new(2),
            "test.expanding",
            expanding_migration,
        ))
        .expect("expanding migration registration must succeed");

    let limits = CompatibilityLimits {
        max_output_bytes: Some(16),
        ..CompatibilityLimits::unlimited()
    };

    let result = registry.migrate(
        json!({
            "schema_version": 1
        }),
        SchemaVersion::new(1),
        SchemaVersion::new(2),
        &limits,
    );

    assert!(
        result.is_err(),
        "output growth must be rejected when it exceeds the configured budget"
    );
}

#[test]
fn document_size_limit_is_enforced() {
    let mut registry = MigrationRegistry::new();

    registry
        .register(Migration::new(
            SchemaVersion::new(1),
            SchemaVersion::new(2),
            "test.v1-to-v2",
            migration_1_to_2,
        ))
        .expect("migration registration must succeed");

    let limits = CompatibilityLimits {
        max_document_bytes: Some(1),
        ..CompatibilityLimits::unlimited()
    };

    let result = registry.migrate(
        json!({
            "schema_version": 1,
            "payload": "larger than one byte"
        }),
        SchemaVersion::new(1),
        SchemaVersion::new(2),
        &limits,
    );

    assert!(
        result.is_err(),
        "input document-size limits must be enforced"
    );
}

#[test]
fn defensive_limits_are_not_quantum_machine_limits() {
    let limits = CompatibilityLimits::defensive();

    assert!(limits.max_document_bytes.is_some());
    assert!(limits.max_migration_steps.is_some());

    /*
     * The test intentionally does not interpret these values as:
     *
     *     maximum qubits
     *
     *     maximum gates
     *
     *     maximum physical resources
     *
     * They are compatibility-processing resource policies only.
     */
}

#[test]
fn unlimited_limits_disable_zqn_compatibility_resource_ceiling() {
    let limits = CompatibilityLimits::unlimited();

    assert_eq!(limits.max_migration_steps, None);
    assert_eq!(limits.max_path_search_states, None);
    assert_eq!(limits.max_document_bytes, None);
    assert_eq!(limits.max_output_bytes, None);
    assert_eq!(limits.max_metadata_bytes, None);
    assert_eq!(limits.max_document_depth, None);
    assert_eq!(limits.max_registered_migrations, None);
    assert_eq!(limits.max_path_length, None);
}

#[test]
fn invalid_migration_identifier_is_rejected() {
    let result = crate::quantum::zqn::io::compatibility::MigrationId::new("");

    assert!(
        matches!(result, Err(CompatibilityError::InvalidMigrationId)),
        "empty migration identifiers must be rejected"
    );
}

#[test]
fn whitespace_only_migration_identifier_is_rejected() {
    let result =
        crate::quantum::zqn::io::compatibility::MigrationId::new("   ");

    assert!(
        matches!(result, Err(CompatibilityError::InvalidMigrationId)),
        "whitespace-only migration identifiers must be rejected"
    );
}

#[test]
fn migration_identifier_is_stable() {
    let id = crate::quantum::zqn::io::compatibility::MigrationId::new(
        "test.stable-migration",
    )
    .expect("valid migration ID must be accepted");

    assert_eq!(id.as_str(), "test.stable-migration");
    assert_eq!(id.to_string(), "test.stable-migration");
}

#[test]
fn migration_semantics_are_classified_correctly() {
    assert!(MigrationSemantics::Lossless.is_lossless());
    assert!(!MigrationSemantics::Lossy.is_lossless());
    assert!(!MigrationSemantics::SemanticChange.is_lossless());
}

#[test]
fn migration_policy_is_strict_by_default() {
    let mut registry = MigrationRegistry::new();

    registry
        .register(Migration::with_semantics(
            SchemaVersion::new(1),
            SchemaVersion::new(2),
            "test.semantic-change",
            MigrationSemantics::SemanticChange,
            migration_1_to_2,
        ))
        .expect("registration must succeed");

    let document = json!({
        "schema_version": 1
    });

    let result = registry.migrate_with_policy(
        document,
        SchemaVersion::new(1),
        SchemaVersion::new(2),
        MigrationPolicy::LosslessOnly,
        &CompatibilityLimits::unlimited(),
    );

    assert!(
        result.is_err(),
        "strict compatibility policy must reject semantic changes"
    );
}

#[test]
fn schema_version_display_is_stable() {
    assert_eq!(SchemaVersion::new(0).to_string(), "0");
    assert_eq!(SchemaVersion::new(1).to_string(), "1");
    assert_eq!(SchemaVersion::new(u64::MAX).to_string(), u64::MAX.to_string());
}

#[test]
fn compatibility_direction_is_independent_of_quantum_resource_count() {
    /*
     * Compatibility is a schema concern. The same schema migration contract
     * must apply regardless of whether a document describes one resource,
     * thousands of resources, or a distributed/future quantum system.
     *
     * We therefore construct documents of different resource cardinalities
     * without introducing a semantic maximum.
     */
    let registry = {
        let mut registry = MigrationRegistry::new();

        registry
            .register(Migration::new(
                SchemaVersion::new(1),
                SchemaVersion::new(2),
                "test.scale-independent",
                migration_1_to_2,
            ))
            .expect("migration registration must succeed");

        registry
    };

    for resource_count in [0usize, 1, 2, 8, 32, 128] {
        let resources: Vec<Value> = (0..resource_count)
            .map(|index| {
                json!({
                    "kind": "quantum-resource",
                    "index": index
                })
            })
            .collect();

        let document = json!({
            "schema_version": 1,
            "resources": resources
        });

        let migrated = registry
            .migrate(
                document,
                SchemaVersion::new(1),
                SchemaVersion::new(2),
                &CompatibilityLimits::unlimited(),
            )
            .expect("migration must remain independent of resource cardinality");

        assert_eq!(migrated["schema_version"], json!(2));
        assert_eq!(
            migrated["resources"]
                .as_array()
                .expect("resources must remain an array")
                .len(),
            resource_count
        );
    }
}

#[test]
fn compatibility_does_not_rewrite_quantum_resource_identity() {
    /*
     * This is intentionally a document-level invariant rather than a direct
     * dependency on a particular QubitId constructor. The canonical identity
     * remains owned by quantum::ir::qubit; compatibility must preserve the
     * serialized identity unless a migration explicitly declares a semantic
     * identity transformation.
     */
    let registry = {
        let mut registry = MigrationRegistry::new();

        registry
            .register(Migration::new(
                SchemaVersion::new(1),
                SchemaVersion::new(2),
                "test.identity-preservation",
                migration_1_to_2,
            ))
            .expect("migration registration must succeed");

        registry
    };

    let document = json!({
        "schema_version": 1,
        "resources": [
            {
                "kind": "qubit",
                "id": "q0"
            },
            {
                "kind": "qubit",
                "id": "q17"
            },
            {
                "kind": "physical-qubit",
                "id": "physical-q42"
            }
        ]
    });

    let migrated = registry
        .migrate(
            document,
            SchemaVersion::new(1),
            SchemaVersion::new(2),
            &CompatibilityLimits::unlimited(),
        )
        .expect("migration must succeed");

    let resources = migrated["resources"]
        .as_array()
        .expect("resources must remain an array");

    assert_eq!(resources[0]["id"], json!("q0"));
    assert_eq!(resources[1]["id"], json!("q17"));
    assert_eq!(resources[2]["id"], json!("physical-q42"));
}

#[test]
fn migration_preserves_unowned_fields_when_the_migration_does_not_change_them() {
    let registry = {
        let mut registry = MigrationRegistry::new();

        registry
            .register(Migration::new(
                SchemaVersion::new(1),
                SchemaVersion::new(2),
                "test.preservation",
                migration_1_to_2,
            ))
            .expect("migration registration must succeed");

        registry
    };

    let document = json!({
        "schema_version": 1,
        "provenance": {
            "source": "measurement",
            "dataset": "example-dataset"
        },
        "calibration": {
            "identity": "calibration-123"
        },
        "resources": [
            "q0",
            "q1"
        ],
        "application": {
            "name": "example"
        }
    });

    let migrated = registry
        .migrate(
            document,
            SchemaVersion::new(1),
            SchemaVersion::new(2),
            &CompatibilityLimits::unlimited(),
        )
        .expect("migration must succeed");

    assert_eq!(
        migrated["provenance"]["dataset"],
        json!("example-dataset")
    );

    assert_eq!(
        migrated["calibration"]["identity"],
        json!("calibration-123")
    );

    assert_eq!(
        migrated["application"]["name"],
        json!("example")
    );
}

#[test]
fn equivalent_runs_produce_equivalent_serialized_values() {
    let registry = linear_registry();

    let document = json!({
        "schema_version": 1,
        "metadata": {
            "name": "deterministic"
        },
        "resources": [
            {
                "kind": "qubit",
                "id": "q0"
            },
            {
                "kind": "qubit",
                "id": "q1"
            }
        ]
    });

    let first = registry
        .migrate(
            document.clone(),
            SchemaVersion::new(1),
            SchemaVersion::new(4),
            &CompatibilityLimits::unlimited(),
        )
        .expect("first run must succeed");

    let second = registry
        .migrate(
            document,
            SchemaVersion::new(1),
            SchemaVersion::new(4),
            &CompatibilityLimits::unlimited(),
        )
        .expect("second run must succeed");

    assert_eq!(
        serde_json::to_string(&first).expect("first document must serialize"),
        serde_json::to_string(&second).expect("second document must serialize")
    );
}

#[test]
fn compatibility_does_not_depend_on_global_state() {
    let first_registry = linear_registry();
    let second_registry = linear_registry();

    let document = json!({
        "schema_version": 1,
        "value": "global-state-test"
    });

    let first = first_registry
        .migrate(
            document.clone(),
            SchemaVersion::new(1),
            SchemaVersion::new(4),
            &CompatibilityLimits::unlimited(),
        )
        .expect("first registry must migrate");

    let second = second_registry
        .migrate(
            document,
            SchemaVersion::new(1),
            SchemaVersion::new(4),
            &CompatibilityLimits::unlimited(),
        )
        .expect("second registry must migrate");

    assert_eq!(first, second);
}

#[test]
fn compatibility_policy_does_not_change_identity_migration() {
    let registry = MigrationRegistry::new();

    let document = json!({
        "schema_version": 5,
        "payload": "identity"
    });

    let lossless = registry
        .migrate_with_policy(
            document.clone(),
            SchemaVersion::new(5),
            SchemaVersion::new(5),
            MigrationPolicy::LosslessOnly,
            &CompatibilityLimits::unlimited(),
        )
        .expect("identity migration must be accepted");

    let allow_lossy = registry
        .migrate_with_policy(
            document.clone(),
            SchemaVersion::new(5),
            SchemaVersion::new(5),
            MigrationPolicy::AllowLossy,
            &CompatibilityLimits::unlimited(),
        )
        .expect("identity migration must be accepted");

    let allow_semantic_change = registry
        .migrate_with_policy(
            document,
            SchemaVersion::new(5),
            SchemaVersion::new(5),
            MigrationPolicy::AllowSemanticChange,
            &CompatibilityLimits::unlimited(),
        )
        .expect("identity migration must be accepted");

    assert_eq!(lossless, allow_lossy);
    assert_eq!(allow_lossy, allow_semantic_change);
}

#[test]
fn compatibility_limits_are_copyable_and_stable() {
    let original = CompatibilityLimits::defensive();
    let copied = original;

    assert_eq!(original, copied);
}

#[test]
fn migration_metadata_is_stable() {
    let migration = Migration::new(
        SchemaVersion::new(10),
        SchemaVersion::new(11),
        "test.metadata",
        migration_1_to_2,
    );

    assert_eq!(migration.source(), SchemaVersion::new(10));
    assert_eq!(migration.target(), SchemaVersion::new(11));
    assert_eq!(migration.id(), "test.metadata");
    assert_eq!(migration.semantics(), MigrationSemantics::Lossless);
}

#[test]
fn forward_and_backward_direction_are_explicit() {
    let low = SchemaVersion::new(10);
    let high = SchemaVersion::new(20);

    assert_eq!(
        CompatibilityDirection::between(low, high),
        CompatibilityDirection::Forward
    );

    assert_eq!(
        CompatibilityDirection::between(high, low),
        CompatibilityDirection::Backward
    );

    assert_ne!(
        CompatibilityDirection::between(low, high),
        CompatibilityDirection::Identity
    );

    assert_ne!(
        CompatibilityDirection::between(high, low),
        CompatibilityDirection::Identity
    );
}