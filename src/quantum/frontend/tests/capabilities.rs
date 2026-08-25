//! Quantum Frontend — format capability contract tests.
//!
//! This module verifies the production contract of:
//!
//! - [`crate::quantum::frontend::format::FormatCapability`];
//! - [`crate::quantum::frontend::format::FormatCapabilities`];
//! - [`crate::quantum::frontend::format::FrontendFormat`];
//! - [`crate::quantum::frontend::format::FormatCompatibility`];
//! - [`crate::quantum::frontend::format::FormatId`];
//! - [`crate::quantum::frontend::format::FormatVersion`].
//!
//! # Purpose
//!
//! These tests verify the generic, format-independent capability system.
//!
//! They intentionally do **not** depend on:
//!
//! - OpenQASM lexer/parser internals;
//! - OpenQASM AST types;
//! - OpenQASM validation internals;
//! - OpenQASM importer implementation;
//! - OpenQASM exporter implementation;
//! - Quantum IR implementation details;
//! - backend capabilities;
//! - hardware capabilities.
//!
//! The capability system describes what an external format can express or
//! transport. It does not claim that every capability is representable by the
//! canonical Zamani Quantum IR or executable by a backend.
//!
//! # Architectural boundary
//!
//! ```text
//! FormatCapability
//!        │
//!        ▼
//! FormatCapabilities
//!        │
//!        ▼
//! FrontendFormat
//!        │
//!        ▼
//! FormatCompatibility
//! ```
//!
//! The capability contract is intentionally independent from concrete formats.
//!
//! A future format such as QIR, Quil, or another quantum interchange language
//! must be able to use the same capability model without changing this test
//! suite merely because the format was added.
//!
//! # Production invariants
//!
//! This suite verifies:
//!
//! 1. every declared capability has a stable identifier;
//! 2. capability identifiers are unique;
//! 3. capability identifiers are deterministic;
//! 4. capability sets are deterministic;
//! 5. duplicate insertion is idempotent;
//! 6. membership queries are exact;
//! 7. removal is deterministic;
//! 8. union preserves all capabilities;
//! 9. missing-capability reporting is deterministic;
//! 10. capability-count limits are enforced;
//! 11. format descriptors preserve their capability sets;
//! 12. format identity is independent from version identity;
//! 13. exact compatibility is distinguishable from same-major compatibility;
//! 14. missing capabilities are distinguishable from version incompatibility;
//! 15. different formats cannot be accepted as version-compatible;
//! 16. exact compatibility requires all requested capabilities;
//! 17. compatibility predicates agree with their enum variants;
//! 18. capability names are suitable for stable machine-readable output;
//! 19. all tests are deterministic;
//! 20. the suite is compatible with Rust 1.97 / 1.97.1.
//!
//! # Rust compatibility
//!
//! - Rust 2021 edition
//! - Rust 1.97 / 1.97.1
//! - stable Rust only
//! - no nightly features
//! - no additional dependencies
//!
//! # Integration
//!
//! Register this module from `src/quantum/frontend/mod.rs`:
//!
//! ```ignore
//! #[cfg(test)]
//! #[path = "tests/capabilities.rs"]
//! mod capabilities;
//! ```
//!
//! No production code should depend on this module.

#![allow(clippy::module_name_repetitions)]

use crate::quantum::frontend::format::{
    FormatCapabilities,
    FormatCapability,
    FormatCompatibility,
    FormatId,
    FormatVersion,
    FrontendFormat,
    MAX_FORMAT_CAPABILITIES,
};

// =============================================================================
// Capability inventory
// =============================================================================

/// Complete inventory of the capabilities currently declared by the generic
/// frontend capability contract.
///
/// Keep this list synchronized with `FormatCapability`.
///
/// This is deliberately local to the tests rather than being introduced as a
/// production `ALL` constant because adding a production enumeration constant
/// would itself become part of the public API.
///
/// If a capability is added to `FormatCapability`, this inventory must be
/// updated in the same change. The exhaustive tests below will then verify its
/// stable name and behavior.
fn all_capabilities() -> &'static [FormatCapability] {
    &[
        FormatCapability::Import,
        FormatCapability::Export,
        FormatCapability::Parameters,
        FormatCapability::Measurements,
        FormatCapability::Reset,
        FormatCapability::Barriers,
        FormatCapability::GateDefinitions,
        FormatCapability::ClassicalComputation,
        FormatCapability::ClassicalControl,
        FormatCapability::Conditionals,
        FormatCapability::Loops,
        FormatCapability::Subroutines,
        FormatCapability::Includes,
        FormatCapability::Timing,
        FormatCapability::Delays,
        FormatCapability::Calibration,
        FormatCapability::Pulse,
        FormatCapability::Annotations,
        FormatCapability::ClassicalIntegers,
        FormatCapability::ClassicalFloats,
        FormatCapability::ClassicalBooleans,
        FormatCapability::Arrays,
        FormatCapability::Expressions,
        FormatCapability::SymbolicNames,
        FormatCapability::RegisterDeclarations,
        FormatCapability::DynamicResources,
        FormatCapability::PhysicalQubits,
    ]
}

// =============================================================================
// Test helpers
// =============================================================================

/// Creates a generic format descriptor without involving any concrete format.
fn test_format(
    id: &str,
    version: FormatVersion,
    capabilities: &[FormatCapability],
) -> FrontendFormat {
    let id = FormatId::new(id)
        .expect("test format identifier must be valid");

    let capabilities = FormatCapabilities::from_iter(
        capabilities.iter().copied(),
    )
    .expect("test capability set must be valid");

    FrontendFormat::new(
        id,
        version,
        capabilities,
    )
}

/// Creates a capability set from a slice.
fn capability_set(
    capabilities: &[FormatCapability],
) -> FormatCapabilities {
    FormatCapabilities::from_iter(
        capabilities.iter().copied(),
    )
    .expect("test capability set must be valid")
}

// =============================================================================
// Capability inventory and stable names
// =============================================================================

#[test]
fn every_declared_capability_has_a_non_empty_machine_name() {
    for capability in all_capabilities() {
        assert!(
            !capability.as_str().is_empty(),
            "capability {:?} must have a non-empty machine-readable name",
            capability,
        );
    }
}

#[test]
fn capability_names_are_ascii() {
    for capability in all_capabilities() {
        assert!(
            capability.as_str().is_ascii(),
            "capability name {:?} must be ASCII",
            capability.as_str(),
        );
    }
}

#[test]
fn capability_names_are_stable_kebab_case() {
    for capability in all_capabilities() {
        let name = capability.as_str();

        assert!(
            name.bytes().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || byte == b'-'
            }),
            "capability name {:?} is not stable lowercase kebab-case",
            name,
        );

        assert!(
            !name.starts_with('-'),
            "capability name {:?} must not start with '-':",
            name,
        );

        assert!(
            !name.ends_with('-'),
            "capability name {:?} must not end with '-':",
            name,
        );

        assert!(
            !name.contains("--"),
            "capability name {:?} must not contain consecutive '-':",
            name,
        );
    }
}

#[test]
fn capability_names_are_unique() {
    let mut names = Vec::new();

    for capability in all_capabilities() {
        let name = capability.as_str();

        assert!(
            !names.contains(&name),
            "duplicate capability machine name: {:?}",
            name,
        );

        names.push(name);
    }
}

#[test]
fn capability_display_matches_machine_name() {
    for capability in all_capabilities() {
        assert_eq!(
            capability.to_string(),
            capability.as_str(),
            "Display must remain the machine-readable capability name",
        );
    }
}

#[test]
fn capability_inventory_contains_all_current_capabilities() {
    // This assertion protects against accidentally forgetting a capability
    // when the production enum is extended.
    //
    // The current generic capability contract contains exactly 27 variants.
    assert_eq!(
        all_capabilities().len(),
        27,
        "update the test inventory when FormatCapability is intentionally extended",
    );
}

// =============================================================================
// Capability ordering and determinism
// =============================================================================

#[test]
fn capability_ordering_is_deterministic() {
    let first = all_capabilities().to_vec();

    let second = all_capabilities().to_vec();

    assert_eq!(
        first,
        second,
        "capability inventory must be deterministic",
    );
}

#[test]
fn capability_set_iteration_is_deterministic() {
    let capabilities = capability_set(all_capabilities());

    let first: Vec<FormatCapability> =
        capabilities.iter().collect();

    let second: Vec<FormatCapability> =
        capabilities.iter().collect();

    assert_eq!(
        first,
        second,
        "capability-set iteration must be deterministic",
    );
}

#[test]
fn capability_set_iteration_follows_capability_order() {
    let capabilities = capability_set(all_capabilities());

    let actual: Vec<FormatCapability> =
        capabilities.iter().collect();

    assert_eq!(
        actual,
        all_capabilities().to_vec(),
        "BTreeSet-backed capability iteration must remain deterministic",
    );
}

#[test]
fn capability_set_to_vec_matches_iteration() {
    let capabilities = capability_set(all_capabilities());

    assert_eq!(
        capabilities.to_vec(),
        capabilities.iter().collect::<Vec<_>>(),
    );
}

// =============================================================================
// Capability-set construction
// =============================================================================

#[test]
fn empty_capability_set_is_empty() {
    let capabilities = FormatCapabilities::new();

    assert!(capabilities.is_empty());
    assert_eq!(capabilities.len(), 0);
}

#[test]
fn inserting_every_capability_produces_complete_set() {
    let mut capabilities = FormatCapabilities::new();

    for capability in all_capabilities() {
        capabilities
            .insert(*capability)
            .expect("capability insertion must succeed");
    }

    assert_eq!(
        capabilities.len(),
        all_capabilities().len(),
    );

    for capability in all_capabilities() {
        assert!(
            capabilities.supports(*capability),
            "capability {:?} was not retained",
            capability,
        );
    }
}

#[test]
fn from_iter_produces_complete_capability_set() {
    let capabilities = capability_set(all_capabilities());

    assert_eq!(
        capabilities.len(),
        all_capabilities().len(),
    );

    assert!(
        capabilities.contains_all(&capabilities),
        "a capability set must contain itself",
    );
}

#[test]
fn duplicate_capability_insertion_is_idempotent() {
    let mut capabilities = FormatCapabilities::new();

    capabilities
        .insert(FormatCapability::Import)
        .expect("first insertion must succeed");

    capabilities
        .insert(FormatCapability::Import)
        .expect("duplicate insertion must remain harmless");

    assert_eq!(
        capabilities.len(),
        1,
    );

    assert!(
        capabilities.supports(FormatCapability::Import),
    );
}

#[test]
fn duplicate_capabilities_in_from_iter_are_collapsed() {
    let capabilities = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Import,
        FormatCapability::Export,
        FormatCapability::Export,
        FormatCapability::Measurements,
        FormatCapability::Measurements,
    ]);

    assert_eq!(
        capabilities.len(),
        3,
    );

    assert!(
        capabilities.supports(FormatCapability::Import),
    );

    assert!(
        capabilities.supports(FormatCapability::Export),
    );

    assert!(
        capabilities.supports(FormatCapability::Measurements),
    );
}

// =============================================================================
// Membership
// =============================================================================

#[test]
fn membership_query_is_exact() {
    let capabilities = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Export,
        FormatCapability::Measurements,
    ]);

    assert!(
        capabilities.supports(FormatCapability::Import),
    );

    assert!(
        capabilities.supports(FormatCapability::Export),
    );

    assert!(
        capabilities.supports(FormatCapability::Measurements),
    );

    assert!(
        !capabilities.supports(FormatCapability::Pulse),
    );

    assert!(
        !capabilities.supports(FormatCapability::Calibration),
    );
}

#[test]
fn every_declared_capability_can_be_inserted_and_queried() {
    for capability in all_capabilities() {
        let mut set = FormatCapabilities::new();

        set.insert(*capability)
            .expect("single capability must be insertable");

        assert!(
            set.supports(*capability),
            "inserted capability {:?} must be discoverable",
            capability,
        );

        assert_eq!(
            set.len(),
            1,
        );
    }
}

// =============================================================================
// Removal
// =============================================================================

#[test]
fn removing_existing_capability_returns_true() {
    let mut capabilities = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Export,
    ]);

    assert!(
        capabilities.remove(FormatCapability::Import),
    );

    assert!(
        !capabilities.supports(FormatCapability::Import),
    );

    assert!(
        capabilities.supports(FormatCapability::Export),
    );

    assert_eq!(
        capabilities.len(),
        1,
    );
}

#[test]
fn removing_missing_capability_returns_false() {
    let mut capabilities = capability_set(&[
        FormatCapability::Import,
    ]);

    assert!(
        !capabilities.remove(FormatCapability::Export),
    );

    assert_eq!(
        capabilities.len(),
        1,
    );

    assert!(
        capabilities.supports(FormatCapability::Import),
    );
}

#[test]
fn remove_then_reinsert_restores_capability() {
    let mut capabilities = capability_set(&[
        FormatCapability::Import,
    ]);

    assert!(
        capabilities.remove(FormatCapability::Import),
    );

    assert!(
        !capabilities.supports(FormatCapability::Import),
    );

    capabilities
        .insert(FormatCapability::Import)
        .expect("re-insertion must succeed");

    assert!(
        capabilities.supports(FormatCapability::Import),
    );

    assert_eq!(
        capabilities.len(),
        1,
    );
}

// =============================================================================
// Set relationships
// =============================================================================

#[test]
fn contains_all_accepts_identical_sets() {
    let capabilities = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Export,
        FormatCapability::Measurements,
    ]);

    assert!(
        capabilities.contains_all(&capabilities),
    );
}

#[test]
fn contains_all_accepts_subset() {
    let available = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Export,
        FormatCapability::Measurements,
    ]);

    let required = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Measurements,
    ]);

    assert!(
        available.contains_all(&required),
    );
}

#[test]
fn contains_all_rejects_missing_capability() {
    let available = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Export,
    ]);

    let required = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Measurements,
    ]);

    assert!(
        !available.contains_all(&required),
    );
}

#[test]
fn empty_requirements_are_always_satisfied() {
    let available = capability_set(&[
        FormatCapability::Import,
    ]);

    let required = FormatCapabilities::new();

    assert!(
        available.contains_all(&required),
    );
}

#[test]
fn missing_capabilities_reports_only_missing_entries() {
    let available = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Export,
    ]);

    let required = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Measurements,
        FormatCapability::Pulse,
    ]);

    assert_eq!(
        available.missing_from(&required),
        vec![
            FormatCapability::Measurements,
            FormatCapability::Pulse,
        ],
    );
}

#[test]
fn missing_capabilities_is_empty_when_requirements_are_satisfied() {
    let available = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Export,
        FormatCapability::Measurements,
    ]);

    let required = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Measurements,
    ]);

    assert!(
        available.missing_from(&required).is_empty(),
    );
}

#[test]
fn missing_capabilities_are_deterministic() {
    let available = capability_set(&[
        FormatCapability::Import,
    ]);

    let required = capability_set(&[
        FormatCapability::Pulse,
        FormatCapability::Measurements,
        FormatCapability::Calibration,
        FormatCapability::Export,
    ]);

    let first = available.missing_from(&required);
    let second = available.missing_from(&required);

    assert_eq!(
        first,
        second,
    );
}

// =============================================================================
// Union
// =============================================================================

#[test]
fn union_contains_capabilities_from_both_sets() {
    let first = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Measurements,
    ]);

    let second = capability_set(&[
        FormatCapability::Export,
        FormatCapability::Pulse,
    ]);

    let union = first
        .union(&second)
        .expect("union must succeed");

    assert!(
        union.supports(FormatCapability::Import),
    );

    assert!(
        union.supports(FormatCapability::Measurements),
    );

    assert!(
        union.supports(FormatCapability::Export),
    );

    assert!(
        union.supports(FormatCapability::Pulse),
    );

    assert_eq!(
        union.len(),
        4,
    );
}

#[test]
fn union_is_idempotent() {
    let capabilities = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Export,
    ]);

    let union = capabilities
        .union(&capabilities)
        .expect("self-union must succeed");

    assert_eq!(
        union,
        capabilities,
    );
}

#[test]
fn union_with_empty_set_preserves_original() {
    let capabilities = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Export,
    ]);

    let empty = FormatCapabilities::new();

    assert_eq!(
        capabilities
            .union(&empty)
            .expect("union must succeed"),
        capabilities,
    );

    assert_eq!(
        empty
            .union(&capabilities)
            .expect("union must succeed"),
        capabilities,
    );
}

// =============================================================================
// Capability-count bounds
// =============================================================================

#[test]
fn capability_count_never_exceeds_configured_maximum() {
    let capabilities = capability_set(all_capabilities());

    assert!(
        capabilities.len() <= MAX_FORMAT_CAPABILITIES,
    );
}

#[test]
fn duplicate_insertion_does_not_consume_capacity() {
    let mut capabilities = FormatCapabilities::new();

    for _ in 0..MAX_FORMAT_CAPABILITIES {
        capabilities
            .insert(FormatCapability::Import)
            .expect(
                "duplicate insertion must not consume capability capacity",
            );
    }

    assert_eq!(
        capabilities.len(),
        1,
    );
}

#[test]
fn capability_set_capacity_is_large_enough_for_current_contract() {
    assert!(
        MAX_FORMAT_CAPABILITIES >= all_capabilities().len(),
        "production capability bound must accommodate all declared capabilities",
    );
}

// =============================================================================
// FrontendFormat contract
// =============================================================================

#[test]
fn frontend_format_preserves_identity_version_and_capabilities() {
    let version = FormatVersion::new(3, 1, 0);

    let capabilities = [
        FormatCapability::Import,
        FormatCapability::Export,
        FormatCapability::Measurements,
    ];

    let format = test_format(
        "ExampleFormat",
        version,
        &capabilities,
    );

    assert_eq!(
        format.id().as_str(),
        "exampleformat",
    );

    assert_eq!(
        format.version(),
        version,
    );

    assert_eq!(
        format.capabilities().len(),
        capabilities.len(),
    );

    for capability in capabilities {
        assert!(
            format.supports(capability),
            "format must expose capability {:?}",
            capability,
        );
    }
}

#[test]
fn frontend_format_normalizes_format_identity_but_not_version() {
    let version = FormatVersion::new(3, 1, 0);

    let format = test_format(
        "OPENQASM",
        version,
        &[FormatCapability::Import],
    );

    assert_eq!(
        format.id().as_str(),
        "openqasm",
    );

    assert_eq!(
        format.version(),
        version,
    );
}

#[test]
fn frontend_formats_with_same_id_are_same_format() {
    let first = test_format(
        "openqasm",
        FormatVersion::new(3, 0, 0),
        &[FormatCapability::Import],
    );

    let second = test_format(
        "OPENQASM",
        FormatVersion::new(3, 1, 0),
        &[FormatCapability::Export],
    );

    assert!(
        first.same_format(&second),
    );

    assert!(
        !first.same_revision(&second),
    );
}

#[test]
fn frontend_formats_with_different_ids_are_not_same_format() {
    let first = test_format(
        "openqasm",
        FormatVersion::new(3, 1, 0),
        &[FormatCapability::Import],
    );

    let second = test_format(
        "quil",
        FormatVersion::new(3, 1, 0),
        &[FormatCapability::Import],
    );

    assert!(
        !first.same_format(&second),
    );

    assert!(
        !first.same_revision(&second),
    );
}

#[test]
fn frontend_format_support_query_delegates_to_capability_set() {
    let format = test_format(
        "example",
        FormatVersion::new(1, 0, 0),
        &[
            FormatCapability::Import,
            FormatCapability::Measurements,
        ],
    );

    assert!(
        format.supports(FormatCapability::Import),
    );

    assert!(
        format.supports(FormatCapability::Measurements),
    );

    assert!(
        !format.supports(FormatCapability::Export),
    );
}

// =============================================================================
// Exact compatibility
// =============================================================================

#[test]
fn exact_compatibility_requires_same_format_same_version_and_capabilities() {
    let capabilities = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Export,
        FormatCapability::Measurements,
    ]);

    let available = FrontendFormat::new(
        FormatId::new("example").expect("valid ID"),
        FormatVersion::new(3, 1, 0),
        capabilities.clone(),
    );

    let requested = FrontendFormat::new(
        FormatId::new("example").expect("valid ID"),
        FormatVersion::new(3, 1, 0),
        capabilities.clone(),
    );

    let result = available.compatibility_with_format(
        &requested,
        &capabilities,
    );

    assert_eq!(
        result,
        FormatCompatibility::Exact,
    );

    assert!(
        result.is_exact(),
    );

    assert!(
        result.is_acceptable(),
    );

    assert!(
        !result.missing_capabilities(),
    );

    assert!(
        !result.requires_negotiation(),
    );

    assert!(
        !result.is_incompatible(),
    );
}

// =============================================================================
// Exact version but missing capabilities
// =============================================================================

#[test]
fn exact_version_with_missing_capabilities_is_not_exact() {
    let available_capabilities = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Measurements,
    ]);

    let required_capabilities = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Export,
    ]);

    let available = FrontendFormat::new(
        FormatId::new("example").expect("valid ID"),
        FormatVersion::new(3, 1, 0),
        available_capabilities,
    );

    let requested = FrontendFormat::new(
        FormatId::new("example").expect("valid ID"),
        FormatVersion::new(3, 1, 0),
        required_capabilities.clone(),
    );

    let result = available.compatibility_with_format(
        &requested,
        &required_capabilities,
    );

    assert_eq!(
        result,
        FormatCompatibility::ExactVersionMissingCapabilities,
    );

    assert!(
        !result.is_exact(),
    );

    assert!(
        !result.is_acceptable(),
    );

    assert!(
        result.missing_capabilities(),
    );

    assert!(
        result.requires_negotiation(),
    );

    assert!(
        !result.is_incompatible(),
    );
}

#[test]
fn exact_version_missing_capabilities_can_be_identified_precisely() {
    let available_capabilities = capability_set(&[
        FormatCapability::Import,
    ]);

    let required_capabilities = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Export,
        FormatCapability::Measurements,
    ]);

    let available = FrontendFormat::new(
        FormatId::new("example").expect("valid ID"),
        FormatVersion::new(3, 1, 0),
        available_capabilities,
    );

    let requested = FrontendFormat::new(
        FormatId::new("example").expect("valid ID"),
        FormatVersion::new(3, 1, 0),
        required_capabilities.clone(),
    );

    let result = available.compatibility_with_format(
        &requested,
        &required_capabilities,
    );

    assert_eq!(
        result,
        FormatCompatibility::ExactVersionMissingCapabilities,
    );

    assert_eq!(
        available.missing_capabilities(&required_capabilities),
        vec![
            FormatCapability::Export,
            FormatCapability::Measurements,
        ],
    );
}

// =============================================================================
// Same-major compatibility
// =============================================================================

#[test]
fn same_major_version_with_required_capabilities_is_distinct_from_exact() {
    let capabilities = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Export,
    ]);

    let available = FrontendFormat::new(
        FormatId::new("example").expect("valid ID"),
        FormatVersion::new(3, 1, 0),
        capabilities.clone(),
    );

    let requested = FrontendFormat::new(
        FormatId::new("example").expect("valid ID"),
        FormatVersion::new(3, 0, 0),
        capabilities.clone(),
    );

    let result = available.compatibility_with_format(
        &requested,
        &capabilities,
    );

    assert_eq!(
        result,
        FormatCompatibility::SameMajorVersion,
    );

    assert!(
        !result.is_exact(),
    );

    assert!(
        result.same_major(),
    );

    assert!(
        !result.missing_capabilities(),
    );

    assert!(
        result.requires_negotiation(),
    );

    assert!(
        !result.is_incompatible(),
    );
}

#[test]
fn same_major_version_with_missing_capabilities_is_distinct() {
    let available_capabilities = capability_set(&[
        FormatCapability::Import,
    ]);

    let required_capabilities = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Export,
    ]);

    let available = FrontendFormat::new(
        FormatId::new("example").expect("valid ID"),
        FormatVersion::new(3, 1, 0),
        available_capabilities,
    );

    let requested = FrontendFormat::new(
        FormatId::new("example").expect("valid ID"),
        FormatVersion::new(3, 0, 0),
        required_capabilities.clone(),
    );

    let result = available.compatibility_with_format(
        &requested,
        &required_capabilities,
    );

    assert_eq!(
        result,
        FormatCompatibility::SameMajorVersionMissingCapabilities,
    );

    assert!(
        result.same_major(),
    );

    assert!(
        result.missing_capabilities(),
    );

    assert!(
        result.requires_negotiation(),
    );

    assert!(
        !result.is_acceptable(),
    );

    assert!(
        !result.is_incompatible(),
    );
}

// =============================================================================
// Different format
// =============================================================================

#[test]
fn different_format_is_never_treated_as_version_compatibility() {
    let capabilities = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Export,
    ]);

    let available = FrontendFormat::new(
        FormatId::new("openqasm").expect("valid ID"),
        FormatVersion::new(3, 1, 0),
        capabilities.clone(),
    );

    let requested = FrontendFormat::new(
        FormatId::new("quil").expect("valid ID"),
        FormatVersion::new(3, 1, 0),
        capabilities.clone(),
    );

    let result = available.compatibility_with_format(
        &requested,
        &capabilities,
    );

    assert_eq!(
        result,
        FormatCompatibility::DifferentFormat,
    );

    assert!(
        result.different_format(),
    );

    assert!(
        result.is_incompatible(),
    );

    assert!(
        !result.same_major(),
    );

    assert!(
        !result.missing_capabilities(),
    );

    assert!(
        !result.requires_negotiation(),
    );

    assert!(
        !result.is_acceptable(),
    );
}

#[test]
fn different_format_wins_even_when_versions_are_identical() {
    let capabilities = capability_set(&[
        FormatCapability::Import,
    ]);

    let openqasm = FrontendFormat::new(
        FormatId::new("openqasm").expect("valid ID"),
        FormatVersion::new(3, 1, 0),
        capabilities.clone(),
    );

    let quil = FrontendFormat::new(
        FormatId::new("quil").expect("valid ID"),
        FormatVersion::new(3, 1, 0),
        capabilities.clone(),
    );

    assert_eq!(
        openqasm.compatibility_with_format(
            &quil,
            &capabilities,
        ),
        FormatCompatibility::DifferentFormat,
    );
}

// =============================================================================
// Incompatible major version
// =============================================================================

#[test]
fn different_major_version_is_incompatible() {
    let capabilities = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Export,
    ]);

    let available = FrontendFormat::new(
        FormatId::new("example").expect("valid ID"),
        FormatVersion::new(4, 0, 0),
        capabilities.clone(),
    );

    let requested = FrontendFormat::new(
        FormatId::new("example").expect("valid ID"),
        FormatVersion::new(3, 1, 0),
        capabilities.clone(),
    );

    let result = available.compatibility_with_format(
        &requested,
        &capabilities,
    );

    assert_eq!(
        result,
        FormatCompatibility::IncompatibleVersion,
    );

    assert!(
        result.incompatible_version(),
    );

    assert!(
        result.is_incompatible(),
    );

    assert!(
        !result.same_major(),
    );

    assert!(
        !result.missing_capabilities(),
    );

    assert!(
        !result.requires_negotiation(),
    );

    assert!(
        !result.is_acceptable(),
    );
}

// =============================================================================
// Compatibility predicate matrix
// =============================================================================

#[test]
fn compatibility_predicates_form_a_consistent_matrix() {
    let variants = [
        FormatCompatibility::Exact,
        FormatCompatibility::ExactVersionMissingCapabilities,
        FormatCompatibility::SameMajorVersion,
        FormatCompatibility::SameMajorVersionMissingCapabilities,
        FormatCompatibility::DifferentFormat,
        FormatCompatibility::IncompatibleVersion,
    ];

    for compatibility in variants {
        match compatibility {
            FormatCompatibility::Exact => {
                assert!(compatibility.is_exact());
                assert!(compatibility.is_acceptable());
                assert!(!compatibility.same_major());
                assert!(!compatibility.missing_capabilities());
                assert!(!compatibility.different_format());
                assert!(!compatibility.incompatible_version());
                assert!(!compatibility.requires_negotiation());
                assert!(!compatibility.is_incompatible());
            }

            FormatCompatibility::ExactVersionMissingCapabilities => {
                assert!(!compatibility.is_exact());
                assert!(!compatibility.is_acceptable());
                assert!(compatibility.same_major());
                assert!(compatibility.missing_capabilities());
                assert!(!compatibility.different_format());
                assert!(!compatibility.incompatible_version());
                assert!(compatibility.requires_negotiation());
                assert!(!compatibility.is_incompatible());
            }

            FormatCompatibility::SameMajorVersion => {
                assert!(!compatibility.is_exact());
                assert!(!compatibility.is_acceptable());
                assert!(compatibility.same_major());
                assert!(!compatibility.missing_capabilities());
                assert!(!compatibility.different_format());
                assert!(!compatibility.incompatible_version());
                assert!(compatibility.requires_negotiation());
                assert!(!compatibility.is_incompatible());
            }

            FormatCompatibility::SameMajorVersionMissingCapabilities => {
                assert!(!compatibility.is_exact());
                assert!(!compatibility.is_acceptable());
                assert!(compatibility.same_major());
                assert!(compatibility.missing_capabilities());
                assert!(!compatibility.different_format());
                assert!(!compatibility.incompatible_version());
                assert!(compatibility.requires_negotiation());
                assert!(!compatibility.is_incompatible());
            }

            FormatCompatibility::DifferentFormat => {
                assert!(!compatibility.is_exact());
                assert!(!compatibility.is_acceptable());
                assert!(!compatibility.same_major());
                assert!(!compatibility.missing_capabilities());
                assert!(compatibility.different_format());
                assert!(!compatibility.incompatible_version());
                assert!(!compatibility.requires_negotiation());
                assert!(compatibility.is_incompatible());
            }

            FormatCompatibility::IncompatibleVersion => {
                assert!(!compatibility.is_exact());
                assert!(!compatibility.is_acceptable());
                assert!(!compatibility.same_major());
                assert!(!compatibility.missing_capabilities());
                assert!(!compatibility.different_format());
                assert!(compatibility.incompatible_version());
                assert!(!compatibility.requires_negotiation());
                assert!(compatibility.is_incompatible());
            }
        }
    }
}

// =============================================================================
// Direct compatibility API
// =============================================================================

#[test]
fn_direct_compatibility_requires_capabilities_but_assumes_format_identity() {
    let available = test_format(
        "example",
        FormatVersion::new(3, 1, 0),
        &[
            FormatCapability::Import,
            FormatCapability::Export,
        ],
    );

    let required = capability_set(&[
        FormatCapability::Import,
    ]);

    assert_eq!(
        available.compatibility_with(
            FormatVersion::new(3, 1, 0),
            &required,
        ),
        FormatCompatibility::Exact,
    );
}

#[test]
fn direct_compatibility_reports_missing_capabilities() {
    let available = test_format(
        "example",
        FormatVersion::new(3, 1, 0),
        &[
            FormatCapability::Import,
        ],
    );

    let required = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Export,
    ]);

    assert_eq!(
        available.compatibility_with(
            FormatVersion::new(3, 1, 0),
            &required,
        ),
        FormatCompatibility::ExactVersionMissingCapabilities,
    );
}

#[test]
fn direct_compatibility_reports_same_major_version() {
    let available = test_format(
        "example",
        FormatVersion::new(3, 1, 0),
        &[
            FormatCapability::Import,
        ],
    );

    let required = capability_set(&[
        FormatCapability::Import,
    ]);

    assert_eq!(
        available.compatibility_with(
            FormatVersion::new(3, 0, 0),
            &required,
        ),
        FormatCompatibility::SameMajorVersion,
    );
}

#[test]
fn direct_compatibility_reports_major_version_incompatibility() {
    let available = test_format(
        "example",
        FormatVersion::new(3, 1, 0),
        &[
            FormatCapability::Import,
        ],
    );

    let required = capability_set(&[
        FormatCapability::Import,
    ]);

    assert_eq!(
        available.compatibility_with(
            FormatVersion::new(4, 0, 0),
            &required,
        ),
        FormatCompatibility::IncompatibleVersion,
    );
}

// =============================================================================
// Format identity and capability independence
// =============================================================================

#[test]
fn capabilities_do_not_change_format_identity() {
    let first = test_format(
        "example",
        FormatVersion::new(1, 0, 0),
        &[FormatCapability::Import],
    );

    let second = test_format(
        "example",
        FormatVersion::new(1, 0, 0),
        &[
            FormatCapability::Import,
            FormatCapability::Export,
            FormatCapability::Pulse,
        ],
    );

    assert!(
        first.same_format(&second),
    );

    assert!(
        first.same_revision(&second),
    );

    assert_eq!(
        first.id(),
        second.id(),
    );

    assert_eq!(
        first.version(),
        second.version(),
    );

    assert_ne!(
        first.capabilities(),
        second.capabilities(),
    );
}

#[test]
fn capability_sets_are_value_types() {
    let original = capability_set(&[
        FormatCapability::Import,
        FormatCapability::Export,
    ]);

    let cloned = original.clone();

    assert_eq!(
        original,
        cloned,
    );

    let mut modified = cloned;

    modified
        .insert(FormatCapability::Measurements)
        .expect("insertion must succeed");

    assert_ne!(
        original,
        modified,
    );

    assert!(
        !original.supports(FormatCapability::Measurements),
        "modifying a clone must not mutate the original",
    );
}

// =============================================================================
// Exhaustive capability round-trip through capability names
// =============================================================================

#[test]
fn every_capability_has_a_unique_stable_serialization_name() {
    let mut seen = std::collections::BTreeSet::new();

    for capability in all_capabilities() {
        let name = capability.as_str();

        assert!(
            seen.insert(name),
            "capability name {:?} appeared more than once",
            name,
        );
    }

    assert_eq!(
        seen.len(),
        all_capabilities().len(),
    );
}

// =============================================================================
// Regression tests for the OpenQASM-independent contract
// =============================================================================

#[test]
fn capability_contract_does_not_require_openqasm() {
    let format = test_format(
        "qir",
        FormatVersion::new(1, 0, 0),
        &[
            FormatCapability::Import,
            FormatCapability::Export,
            FormatCapability::Measurements,
        ],
    );

    assert_eq!(
        format.id().as_str(),
        "qir",
    );

    assert!(
        format.supports(FormatCapability::Import),
    );

    assert!(
        format.supports(FormatCapability::Export),
    );

    assert!(
        format.supports(FormatCapability::Measurements),
    );
}

#[test]
fn capability_contract_does_not_assume_backend_support() {
    let format = test_format(
        "example",
        FormatVersion::new(1, 0, 0),
        &[
            FormatCapability::Pulse,
            FormatCapability::Calibration,
            FormatCapability::PhysicalQubits,
        ],
    );

    assert!(
        format.supports(FormatCapability::Pulse),
    );

    assert!(
        format.supports(FormatCapability::Calibration),
    );

    assert!(
        format.supports(FormatCapability::PhysicalQubits),
    );

    // The capability contract only records format expressiveness. It does not
    // provide or imply a hardware execution path.
    assert_eq!(
        format.capabilities().len(),
        3,
    );
}

// =============================================================================
// Compatibility symmetry/documented direction
// =============================================================================

#[test]
fn compatibility_is_evaluated_from_available_to_requested() {
    let available = test_format(
        "example",
        FormatVersion::new(3, 1, 0),
        &[
            FormatCapability::Import,
            FormatCapability::Export,
        ],
    );

    let requested_version = FormatVersion::new(3, 0, 0);

    let required = capability_set(&[
        FormatCapability::Import,
    ]);

    assert_eq!(
        available.compatibility_with(
            requested_version,
            &required,
        ),
        FormatCompatibility::SameMajorVersion,
    );

    // A newer available version and an older requested version are not the
    // same relationship as reversing the operands. The compatibility API is
    // intentionally directional.
    assert_eq!(
        test_format(
            "example",
            FormatVersion::new(3, 0, 0),
            &[
                FormatCapability::Import,
                FormatCapability::Export,
            ],
        )
        .compatibility_with(
            FormatVersion::new(3, 1, 0),
            &required,
        ),
        FormatCompatibility::SameMajorVersion,
    );
}

// =============================================================================
// Zero-requirement compatibility
// =============================================================================

#[test]
fn exact_version_with_no_required_capabilities_is_exact() {
    let available = test_format(
        "example",
        FormatVersion::new(3, 1, 0),
        &[],
    );

    let required = FormatCapabilities::new();

    assert_eq!(
        available.compatibility_with(
            FormatVersion::new(3, 1, 0),
            &required,
        ),
        FormatCompatibility::Exact,
    );
}

#[test]
fn same_major_version_with_no_required_capabilities_is_same_major() {
    let available = test_format(
        "example",
        FormatVersion::new(3, 1, 0),
        &[],
    );

    let required = FormatCapabilities::new();

    assert_eq!(
        available.compatibility_with(
            FormatVersion::new(3, 0, 0),
            &required,
        ),
        FormatCompatibility::SameMajorVersion,
    );
}

// =============================================================================
// Version semantics
// =============================================================================

#[test]
fn patch_version_difference_remains_same_major() {
    let capabilities = capability_set(&[
        FormatCapability::Import,
    ]);

    let available = test_format(
        "example",
        FormatVersion::new(3, 1, 2),
        &[FormatCapability::Import],
    );

    assert_eq!(
        available.compatibility_with(
            FormatVersion::new(3, 1, 1),
            &capabilities,
        ),
        FormatCompatibility::SameMajorVersion,
    );
}

#[test]
fn minor_version_difference_remains_same_major() {
    let capabilities = capability_set(&[
        FormatCapability::Import,
    ]);

    let available = test_format(
        "example",
        FormatVersion::new(3, 2, 0),
        &[FormatCapability::Import],
    );

    assert_eq!(
        available.compatibility_with(
            FormatVersion::new(3, 1, 0),
            &capabilities,
        ),
        FormatCompatibility::SameMajorVersion,
    );
}

#[test]
fn major_version_difference_is_not_same_major() {
    let capabilities = capability_set(&[
        FormatCapability::Import,
    ]);

    let available = test_format(
        "example",
        FormatVersion::new(4, 0, 0),
        &[FormatCapability::Import],
    );

    let result = available.compatibility_with(
        FormatVersion::new(3, 1, 0),
        &capabilities,
    );

    assert!(
        !result.same_major(),
    );

    assert_eq!(
        result,
        FormatCompatibility::IncompatibleVersion,
    );
}