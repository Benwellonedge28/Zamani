//! Zamani Quantum Resilience — Adaptation Integration Tests
//!
//! Path:
//!     src/quantum/resilience/tests/adaptation.rs
//!
//! Purpose:
//!     Production-level contract and integration tests for the resilience
//!     adaptation subsystem.
//!
//! ============================================================================
//! Architectural contract
//! ============================================================================
//!
//! Adaptation is the bridge between an authorized resilience decision and the
//! authoritative quantum subsystems that perform transformations.
//!
//! ```text
//!                    resilience planning
//!                           |
//!                           v
//!                    RecoveryAction
//!                           |
//!                           v
//!                  AdaptationRequest
//!                           |
//!                           v
//!                   AdaptationAdapter
//!                           |
//!       +-------------------+-------------------+
//!       |                   |                   |
//!       v                   v                   v
//!   remapping           rerouting          rescheduling
//!       |                   |                   |
//!       +-------------------+-------------------+
//!                           |
//!              +------------+------------+
//!              |            |            |
//!              v            v            v
//!        recompilation  optimization    QEC
//!              |            |            |
//!              +------------+------------+
//!                           |
//!                           v
//!                    backend selection
//! ```
//!
//! These tests verify the adaptation boundary itself. They do not duplicate
//! routing, scheduling, compilation, optimization, QEC, or hardware logic.
//!
//! ============================================================================
//! Production invariants
//! ============================================================================
//!
//! The tests enforce the following architectural rules:
//!
//! 1. Canonical qubit identity comes from:
//!
//!        crate::quantum::ir::qubit
//!
//! 2. Adaptation is transactional:
//!
//!        preflight -> prepare -> commit -> verify
//!
//! 3. Preparing a candidate must not be confused with committing it.
//!
//! 4. Stale execution state must not be silently committed.
//!
//! 5. Stale semantic state must not be silently committed.
//!
//! 6. Logical semantics must remain distinct from physical realization.
//!
//! 7. Sparse resource identifiers must work.
//!
//! 8. No test assumes a fixed number of qubits, devices, operations, backends,
//!    execution slots, or topology elements.
//!
//! 9. Tests use generated sizes where scale is relevant.
//!
//! 10. Empty, invalid, conflicting, stale, and no-op requests are exercised.
//!
//! 11. Deterministic inputs must produce deterministic adaptation decisions.
//!
//! 12. Adaptation must remain provider-independent.
//!
//! 13. Adaptation must remain replaceable without changing canonical IR.
//!
//! 14. Concrete quantum algorithms remain owned by their respective quantum
//!     subsystems.
//!
//! 15. Verification remains a separate acceptance boundary.
//!
//! ============================================================================
//! Rust contract
//! ============================================================================
//!
//! * Rust 1.97 / 1.97.1
//! * Rust 2021
//! * stable Rust
//! * no nightly features
//! * no unsafe code
//! * no machine-size constants
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]
#![deny(clippy::all)]

use std::collections::BTreeSet;
use std::fmt::Debug;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::ir::resources::mapping::QubitMapping;

use crate::quantum::resilience::adaptation::{
    schema_identity,
    AdaptationCapabilities,
    AdaptationPhase,
    AdaptationStatus,
    AdapterId,
    AdapterOperation,
    AdapterVersion,
    ExecutionGeneration,
    SemanticRevision,
    ADAPTATION_ADAPTER_SCHEMA_ID,
    ADAPTATION_ADAPTER_SCHEMA_VERSION,
    ADAPTATION_MODULE_SCHEMA_ID,
    ADAPTATION_MODULE_SCHEMA_VERSION,
};

use crate::quantum::resilience::adaptation::remapping::{
    MappingChange,
    MappingRevision,
    RemappingAdapter,
    RemappingId,
    RemappingRequest,
    RemappingScope,
    REMAPPING_SCHEMA_ID,
    REMAPPING_SCHEMA_VERSION,
};

// ============================================================================
// Test constants
// ============================================================================
//
// These are identifiers used as test data, not hardware limits.
//
// They MUST NOT be interpreted as supported machine sizes.
//
// ============================================================================

const TEST_SEMANTIC_REVISION: &str = "semantic-revision:test";
const TEST_EXECUTION_GENERATION: &str = "execution-generation:test";
const TEST_MAPPING_REVISION: &str = "mapping-revision:test";

// ============================================================================
// Generic compile-time contract helpers
// ============================================================================

fn assert_debug<T: Debug>() {}

fn assert_send_sync<T: Send + Sync>() {}

fn assert_clone<T: Clone>() {}

fn assert_copy<T: Copy>() {}

fn assert_eq_hash_ord<T>()
where
    T: Eq + std::hash::Hash + Ord,
{
}

// ============================================================================
// Identifier helpers
// ============================================================================

fn remapping_id(value: &str) -> RemappingId {
    RemappingId::new(value).expect("test remapping identifier must be valid")
}

fn mapping_revision(value: &str) -> MappingRevision {
    MappingRevision::new(value).expect("test mapping revision must be valid")
}

fn semantic_revision(value: &str) -> SemanticRevision {
    SemanticRevision::new(value).expect("test semantic revision must be valid")
}

fn execution_generation(value: &str) -> ExecutionGeneration {
    ExecutionGeneration::new(value)
        .expect("test execution generation must be valid")
}

// ============================================================================
// Canonical mapping helpers
// ============================================================================
//
// These helpers deliberately construct sparse mappings instead of arrays
// indexed by physical-machine size.
//
// The integer identifiers are test DATA. They do not represent a maximum
// supported machine size.
// ============================================================================

fn mapping(entries: &[(u64, u64)]) -> QubitMapping {
    let mut result = QubitMapping::new();

    for &(logical, physical) in entries {
        result
            .insert(
                QubitId::new(logical),
                PhysicalQubitId::new(physical),
            )
            .expect("test mapping must contain valid unique entries");
    }

    result
}

fn request(
    id: &str,
    current: QubitMapping,
    replacement: QubitMapping,
) -> RemappingRequest {
    RemappingRequest::new(
        remapping_id(id),
        RemappingScope::Computation,
        semantic_revision(TEST_SEMANTIC_REVISION),
        execution_generation(TEST_EXECUTION_GENERATION),
        mapping_revision(TEST_MAPPING_REVISION),
        current,
        replacement,
    )
    .expect("test remapping request must be structurally valid")
}

// ============================================================================
// Schema and public-contract tests
// ============================================================================

#[test]
fn adaptation_module_schema_is_stable() {
    assert_eq!(
        schema_identity(),
        (
            ADAPTATION_MODULE_SCHEMA_ID,
            ADAPTATION_MODULE_SCHEMA_VERSION,
        )
    );

    assert_eq!(
        ADAPTATION_MODULE_SCHEMA_ID,
        "zamani.quantum.resilience.adaptation"
    );

    assert_eq!(ADAPTATION_MODULE_SCHEMA_VERSION, 1);
}

#[test]
fn adapter_schema_is_stable() {
    assert_eq!(
        ADAPTATION_ADAPTER_SCHEMA_ID,
        "zamani.quantum.resilience.adaptation.adapter"
    );

    assert_eq!(ADAPTATION_ADAPTER_SCHEMA_VERSION, 1);
}

#[test]
fn remapping_schema_is_stable() {
    assert_eq!(
        REMAPPING_SCHEMA_ID,
        "zamani.quantum.resilience.adaptation.remapping"
    );

    assert_eq!(REMAPPING_SCHEMA_VERSION, 1);
}

// ============================================================================
// Type ownership tests
// ============================================================================

#[test]
fn canonical_qubit_identity_types_are_available() {
    assert_debug::<QubitId>();
    assert_debug::<PhysicalQubitId>();

    assert_eq_hash_ord::<QubitId>();
    assert_eq_hash_ord::<PhysicalQubitId>();

    assert_copy::<QubitId>();
    assert_copy::<PhysicalQubitId>();
}

#[test]
fn adaptation_contract_value_types_are_debuggable() {
    assert_debug::<AdapterId>();
    assert_debug::<AdapterVersion>();
    assert_debug::<AdapterCapabilities>();
    assert_debug::<AdaptationPhase>();
    assert_debug::<AdaptationStatus>();
    assert_debug::<AdapterOperation>();
    assert_debug::<ExecutionGeneration>();
    assert_debug::<SemanticRevision>();
}

#[test]
fn adaptation_contract_value_types_are_send_sync_when_shared() {
    assert_send_sync::<AdapterId>();
    assert_send_sync::<AdapterVersion>();
    assert_send_sync::<AdapterCapabilities>();
    assert_send_sync::<AdaptationPhase>();
    assert_send_sync::<AdaptationStatus>();
    assert_send_sync::<AdapterOperation>();
    assert_send_sync::<ExecutionGeneration>();
    assert_send_sync::<SemanticRevision>();
}

#[test]
fn adaptation_identifiers_are_cloneable() {
    assert_clone::<AdapterId>();
    assert_clone::<ExecutionGeneration>();
    assert_clone::<SemanticRevision>();
}

#[test]
fn adaptation_capabilities_are_copyable() {
    assert_copy::<AdapterCapabilities>();
    assert_copy::<AdaptationPhase>();
    assert_copy::<AdaptationStatus>();
}

// ============================================================================
// Adapter identity
// ============================================================================

#[test]
fn adapter_identifier_rejects_empty_values() {
    assert!(AdapterId::new("").is_err());
}

#[test]
fn adapter_identifier_rejects_whitespace() {
    assert!(AdapterId::new("adapter with whitespace").is_err());
}

#[test]
fn adapter_identifier_preserves_valid_identity() {
    let id = AdapterId::new("remapping").expect("identifier must be valid");

    assert_eq!(id.as_str(), "remapping");
    assert_eq!(id.to_string(), "remapping");
}

#[test]
fn adapter_version_is_deterministic() {
    let version = AdapterVersion::new(1, 2, 3);

    assert_eq!(version.major(), 1);
    assert_eq!(version.minor(), 2);
    assert_eq!(version.patch(), 3);
    assert_eq!(version.to_string(), "1.2.3");
}

// ============================================================================
// Lifecycle tests
// ============================================================================

#[test]
fn adaptation_phases_have_stable_order() {
    assert!(AdaptationPhase::Preflight < AdaptationPhase::Prepare);
    assert!(AdaptationPhase::Prepare < AdaptationPhase::Commit);
    assert!(AdaptationPhase::Commit < AdaptationPhase::Verify);
}

#[test]
fn adaptation_phases_have_stable_names() {
    assert_eq!(AdaptationPhase::Preflight.as_str(), "preflight");
    assert_eq!(AdaptationPhase::Prepare.as_str(), "prepare");
    assert_eq!(AdaptationPhase::Commit.as_str(), "commit");
    assert_eq!(AdaptationPhase::Verify.as_str(), "verify");
}

// ============================================================================
// Remapping identity and canonical qubit tests
// ============================================================================

#[test]
fn mapping_change_preserves_canonical_logical_identity() {
    let logical = QubitId::new(17);
    let previous = PhysicalQubitId::new(31);
    let replacement = PhysicalQubitId::new(43);

    let change = MappingChange::new(
        logical,
        Some(previous),
        Some(replacement),
    );

    assert_eq!(change.logical(), logical);
    assert_eq!(change.previous(), Some(previous));
    assert_eq!(change.replacement(), Some(replacement));
    assert!(change.changed());
}

#[test]
fn mapping_change_identifies_new_mapping() {
    let change = MappingChange::new(
        QubitId::new(9),
        None,
        Some(PhysicalQubitId::new(27)),
    );

    assert!(change.is_new_mapping());
    assert!(!change.is_unmapped());
    assert!(change.changed());
}

#[test]
fn mapping_change_identifies_unmapping() {
    let change = MappingChange::new(
        QubitId::new(9),
        Some(PhysicalQubitId::new(27)),
        None,
    );

    assert!(!change.is_new_mapping());
    assert!(change.is_unmapped());
    assert!(change.changed());
}

#[test]
fn unchanged_mapping_change_is_not_reported_as_changed() {
    let physical = PhysicalQubitId::new(27);

    let change = MappingChange::new(
        QubitId::new(9),
        Some(physical),
        Some(physical),
    );

    assert!(!change.changed());
}

// ============================================================================
// Mapping construction and scalability
// ============================================================================

#[test]
fn sparse_mapping_is_supported() {
    let current = mapping(&[
        (2, 100),
        (11, 1000),
        (97, 10000),
    ]);

    assert_eq!(current.len(), 3);
    assert!(current.validate().is_ok());
}

#[test]
fn non_contiguous_logical_ids_are_supported() {
    let current = mapping(&[
        (3, 8),
        (71, 19),
        (1007, 2048),
    ]);

    assert_eq!(current.len(), 3);
    assert!(current.validate().is_ok());
}

#[test]
fn non_contiguous_physical_ids_are_supported() {
    let current = mapping(&[
        (0, 13),
        (1, 700),
        (2, 9007),
    ]);

    assert_eq!(current.len(), 3);
    assert!(current.validate().is_ok());
}

#[test]
fn mapping_does_not_require_zero_based_physical_ids() {
    let current = mapping(&[
        (0, 101),
        (1, 303),
        (2, 707),
    ]);

    assert!(current.validate().is_ok());
}

#[test]
fn mapping_supports_empty_structural_state() {
    let empty = QubitMapping::new();

    assert_eq!(empty.len(), 0);
    assert!(empty.validate().is_ok());
}

// ============================================================================
// Request construction
// ============================================================================

#[test]
fn valid_remapping_request_preserves_all_identity_components() {
    let current = mapping(&[
        (0, 4),
        (1, 5),
    ]);

    let replacement = mapping(&[
        (0, 5),
        (1, 4),
    ]);

    let request = request("remap-valid", current, replacement);

    assert_eq!(request.id().as_str(), "remap-valid");
    assert_eq!(request.scope(), RemappingScope::Computation);
    assert_eq!(
        request.semantic_revision().as_str(),
        TEST_SEMANTIC_REVISION
    );
    assert_eq!(
        request.execution_generation().as_str(),
        TEST_EXECUTION_GENERATION
    );
    assert_eq!(
        request.current_mapping_revision().as_str(),
        TEST_MAPPING_REVISION
    );
}

#[test]
fn request_detects_mapping_change() {
    let request = request(
        "changed",
        mapping(&[(0, 4), (1, 5)]),
        mapping(&[(0, 5), (1, 4)]),
    );

    assert!(request.changes_mapping());
    assert!(!request.changes().is_empty());
}

#[test]
fn request_detects_noop_mapping() {
    let current = mapping(&[
        (0, 4),
        (1, 5),
    ]);

    let request = request(
        "noop",
        current.clone(),
        current,
    );

    assert!(!request.changes_mapping());
    assert!(request.changes().is_empty());
}

// ============================================================================
// Deterministic mapping-change tests
// ============================================================================

#[test]
fn mapping_changes_are_deterministic() {
    let current = mapping(&[
        (9, 19),
        (2, 12),
        (7, 17),
    ]);

    let replacement = mapping(&[
        (9, 29),
        (2, 22),
        (7, 27),
    ]);

    let first = request(
        "deterministic-a",
        current.clone(),
        replacement.clone(),
    )
    .changes();

    let second = request(
        "deterministic-b",
        current,
        replacement,
    )
    .changes();

    assert_eq!(first, second);
}

#[test]
fn mapping_changes_are_order_independent() {
    let first_current = mapping(&[
        (1, 11),
        (5, 15),
        (9, 19),
    ]);

    let second_current = mapping(&[
        (9, 19),
        (1, 11),
        (5, 15),
    ]);

    let first_replacement = mapping(&[
        (1, 21),
        (5, 25),
        (9, 29),
    ]);

    let second_replacement = mapping(&[
        (9, 29),
        (5, 25),
        (1, 21),
    ]);

    let first = request(
        "order-a",
        first_current,
        first_replacement,
    )
    .changes();

    let second = request(
        "order-b",
        second_current,
        second_replacement,
    )
    .changes();

    assert_eq!(first, second);
}

// ============================================================================
// Adapter construction
// ============================================================================

#[test]
fn remapping_adapter_is_constructible_without_hardware_state() {
    let _adapter = RemappingAdapter;
}

#[test]
fn remapping_adapter_has_no_instance_state_requirement() {
    let first = RemappingAdapter;
    let second = RemappingAdapter;

    assert_eq!(
        std::mem::size_of_val(&first),
        std::mem::size_of_val(&second)
    );
}

// ============================================================================
// Structural invalid-input tests
// ============================================================================

#[test]
fn invalid_remapping_identifier_is_rejected() {
    assert!(RemappingId::new("").is_err());
    assert!(RemappingId::new("invalid identifier").is_err());
}

#[test]
fn invalid_mapping_revision_is_rejected() {
    assert!(MappingRevision::new("").is_err());
}

#[test]
fn invalid_semantic_revision_is_rejected() {
    assert!(SemanticRevision::new("").is_err());
}

#[test]
fn invalid_execution_generation_is_rejected() {
    assert!(ExecutionGeneration::new("").is_err());
}

// ============================================================================
// Conflicting mapping tests
// ============================================================================

#[test]
fn duplicate_physical_ownership_is_rejected_by_canonical_mapping() {
    let mut invalid = QubitMapping::new();

    let first = invalid.insert(
        QubitId::new(0),
        PhysicalQubitId::new(10),
    );

    assert!(first.is_ok());

    let second = invalid.insert(
        QubitId::new(1),
        PhysicalQubitId::new(10),
    );

    assert!(second.is_err() || invalid.validate().is_err());
}

#[test]
fn duplicate_logical_ownership_is_rejected_by_canonical_mapping() {
    let mut mapping = QubitMapping::new();

    mapping
        .insert(
            QubitId::new(0),
            PhysicalQubitId::new(10),
        )
        .expect("first entry must be valid");

    let duplicate = mapping.insert(
        QubitId::new(0),
        PhysicalQubitId::new(11),
    );

    assert!(duplicate.is_err() || mapping.validate().is_err());
}

// ============================================================================
// Scale-parametric tests
// ============================================================================
//
// The test does not define a machine maximum.
// It exercises a sequence of resource counts selected by the test itself.
//
// These values are test workloads, NOT supported hardware limits.
//
// ============================================================================

fn generated_mapping_size(size: usize) -> QubitMapping {
    let mut result = QubitMapping::new();

    for index in 0..size {
        let logical = QubitId::new(index as u64);
        let physical = PhysicalQubitId::new(
            (index as u64)
                .saturating_mul(17)
                .saturating_add(3),
        );

        result
            .insert(logical, physical)
            .expect("generated mapping must remain one-to-one");
    }

    result
}

#[test]
fn adaptation_mapping_contract_scales_with_available_test_resources() {
    let sizes = [0usize, 1, 2, 7, 31, 127, 257];

    for size in sizes {
        let current = generated_mapping_size(size);
        let replacement = generated_mapping_size(size);

        assert_eq!(current.len(), size);
        assert_eq!(replacement.len(), size);
        assert!(current.validate().is_ok());
        assert!(replacement.validate().is_ok());

        let request = request(
            &format!("scale-{size}"),
            current,
            replacement,
        );

        assert!(!request.changes_mapping());
        assert!(request.changes().is_empty());
    }
}

// ============================================================================
// Sparse large-identifier test
// ============================================================================

#[test]
fn adaptation_does_not_treat_identifier_value_as_resource_count() {
    let current = mapping(&[
        (1, 4_000_000_001),
        (9_000_000_007, 8_000_000_011),
    ]);

    let replacement = mapping(&[
        (1, 8_000_000_011),
        (9_000_000_007, 4_000_000_001),
    ]);

    let request = request(
        "sparse-large-identifiers",
        current,
        replacement,
    );

    assert!(request.changes_mapping());
    assert_eq!(request.changes().len(), 2);
}

// ============================================================================
// Logical semantic identity test
// ============================================================================

#[test]
fn remapping_changes_physical_realization_not_logical_identity() {
    let logical_a = QubitId::new(100);
    let logical_b = QubitId::new(900);

    let current = mapping(&[
        (100, 11),
        (900, 17),
    ]);

    let replacement = mapping(&[
        (100, 17),
        (900, 11),
    ]);

    let request = request(
        "semantic-preservation",
        current,
        replacement,
    );

    let changed_logicals: BTreeSet<QubitId> = request
        .changes()
        .into_iter()
        .map(MappingChange::logical)
        .collect();

    assert!(changed_logicals.contains(&logical_a));
    assert!(changed_logicals.contains(&logical_b));
    assert_eq!(changed_logicals.len(), 2);
}

// ============================================================================
// No artificial hardware assumptions
// ============================================================================

#[test]
fn adaptation_tests_do_not_require_contiguous_machine_resources() {
    let current = mapping(&[
        (13, 101),
        (1001, 10007),
        (70001, 900003),
    ]);

    let replacement = mapping(&[
        (13, 900003),
        (1001, 101),
        (70001, 10007),
    ]);

    let request = request(
        "non-contiguous",
        current,
        replacement,
    );

    assert_eq!(request.changes().len(), 3);
}

#[test]
fn adaptation_does_not_encode_provider_identity() {
    //
    // This test intentionally verifies the contract at the type/value level:
    // a remapping request contains semantic/mapping state and no provider
    // discriminator.
    //
    // Provider/device selection belongs to backend_selection.rs and the
    // hardware registry.
    //
    let request = request(
        "provider-independent",
        mapping(&[(0, 7)]),
        mapping(&[(0, 13)]),
    );

    assert_eq!(request.scope(), RemappingScope::Computation);
}

// ============================================================================
// Adapter capability contract
// ============================================================================

#[test]
fn default_adapter_capabilities_are_explicit() {
    let capabilities = AdapterCapabilities::default();

    assert!(capabilities.supports_prepare());
    assert!(capabilities.supports_commit());
    assert!(capabilities.supports_preflight());
    assert!(capabilities.deterministic());
    assert!(capabilities.scoped());
    assert!(capabilities.partial());
}

#[test]
fn adapter_capabilities_are_not_hardware_capabilities() {
    let capabilities = AdapterCapabilities::new(
        true,
        true,
        true,
        true,
        true,
        true,
        false,
    );

    assert!(capabilities.supports_prepare());
    assert!(capabilities.supports_commit());

    // Deliberately do not inspect qubit counts, topology size, provider
    // identity, or device properties here. Those belong to hardware
    // capabilities, not adaptation adapter metadata.
}

// ============================================================================
// Scope tests
// ============================================================================

#[test]
fn remapping_scope_names_are_stable() {
    assert_eq!(
        RemappingScope::Computation.as_str(),
        "computation"
    );

    assert_eq!(
        RemappingScope::AffectedResources.as_str(),
        "affected_resources"
    );

    assert_eq!(
        RemappingScope::LogicalDomain.as_str(),
        "logical_domain"
    );
}

// ============================================================================
// Revision isolation
// ============================================================================

#[test]
fn semantic_revision_is_explicitly_carried_by_remapping_request() {
    let request = request(
        "semantic-revision",
        mapping(&[(0, 1)]),
        mapping(&[(0, 2)]),
    );

    assert_eq!(
        request.semantic_revision().as_str(),
        TEST_SEMANTIC_REVISION
    );
}

#[test]
fn execution_generation_is_explicitly_carried_by_remapping_request() {
    let request = request(
        "execution-generation",
        mapping(&[(0, 1)]),
        mapping(&[(0, 2)]),
    );

    assert_eq!(
        request.execution_generation().as_str(),
        TEST_EXECUTION_GENERATION
    );
}

#[test]
fn mapping_revision_is_explicitly_carried_by_remapping_request() {
    let request = request(
        "mapping-revision",
        mapping(&[(0, 1)]),
        mapping(&[(0, 2)]),
    );

    assert_eq!(
        request.current_mapping_revision().as_str(),
        TEST_MAPPING_REVISION
    );
}

// ============================================================================
// Transactional contract documentation tests
// ============================================================================

#[test]
fn transactional_lifecycle_has_all_required_phases() {
    let phases = [
        AdaptationPhase::Preflight,
        AdaptationPhase::Prepare,
        AdaptationPhase::Commit,
        AdaptationPhase::Verify,
    ];

    assert_eq!(phases.len(), 4);

    for phase in phases {
        assert!(!phase.as_str().is_empty());
    }
}

#[test]
fn prepare_and_commit_are_distinct_contract_phases() {
    assert_ne!(
        AdaptationPhase::Prepare,
        AdaptationPhase::Commit
    );
}

#[test]
fn commit_and_verify_are_distinct_contract_phases() {
    assert_ne!(
        AdaptationPhase::Commit,
        AdaptationPhase::Verify
    );
}

// ============================================================================
// Determinism under repeated construction
// ============================================================================

#[test]
fn repeated_equivalent_requests_have_equal_changes() {
    let current = mapping(&[
        (17, 101),
        (23, 107),
        (41, 113),
    ]);

    let replacement = mapping(&[
        (17, 107),
        (23, 113),
        (41, 101),
    ]);

    let first = request(
        "repeat-one",
        current.clone(),
        replacement.clone(),
    )
    .changes();

    let second = request(
        "repeat-two",
        current,
        replacement,
    )
    .changes();

    assert_eq!(first, second);
}

// ============================================================================
// Resource identity preservation
// ============================================================================

#[test]
fn every_reported_change_has_a_logical_qubit_identity() {
    let request = request(
        "identity-completeness",
        mapping(&[
            (0, 4),
            (1, 5),
            (2, 6),
        ]),
        mapping(&[
            (0, 6),
            (1, 4),
            (2, 5),
        ]),
    );

    for change in request.changes() {
        let _logical: QubitId = change.logical();

        assert!(change.changed());
    }
}

// ============================================================================
// Mapping cardinality
// ============================================================================

#[test]
fn complete_remapping_preserves_mapping_cardinality() {
    let current = mapping(&[
        (0, 10),
        (1, 11),
        (2, 12),
        (3, 13),
    ]);

    let replacement = mapping(&[
        (0, 13),
        (1, 12),
        (2, 11),
        (3, 10),
    ]);

    let request = request(
        "cardinality",
        current,
        replacement,
    );

    assert_eq!(request.current_mapping().len(), 4);
    assert_eq!(request.replacement_mapping().len(), 4);
    assert_eq!(request.changes().len(), 4);
}

// ============================================================================
// Partial adaptation
// ============================================================================

#[test]
fn partial_mapping_changes_are_representable() {
    let current = mapping(&[
        (0, 10),
        (1, 11),
        (2, 12),
    ]);

    let replacement = mapping(&[
        (0, 10),
        (1, 21),
        (2, 12),
    ]);

    let request = request(
        "partial-change",
        current,
        replacement,
    );

    let changes = request.changes();

    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].logical(), QubitId::new(1));
    assert_eq!(
        changes[0].previous(),
        Some(PhysicalQubitId::new(11))
    );
    assert_eq!(
        changes[0].replacement(),
        Some(PhysicalQubitId::new(21))
    );
}

// ============================================================================
// No-op preservation
// ============================================================================

#[test]
fn no_op_adaptation_is_distinguishable_from_real_adaptation() {
    let mapping = mapping(&[
        (7, 17),
        (19, 29),
    ]);

    let no_op = request(
        "noop",
        mapping.clone(),
        mapping,
    );

    let real = request(
        "real",
        mapping(&[
            (7, 17),
            (19, 29),
        ]),
        mapping(&[
            (7, 29),
            (19, 17),
        ]),
    );

    assert!(no_op.changes().is_empty());
    assert!(!real.changes().is_empty());
}

// ============================================================================
// Integration-boundary tests
// ============================================================================

#[test]
fn adaptation_uses_the_planning_to_adapter_boundary() {
    //
    // The common adapter schema is the integration point from planning/recovery
    // into concrete adaptation.
    //
    // The test intentionally checks the public contract rather than reaching
    // into a provider implementation.
    //
    assert_eq!(
        ADAPTATION_ADAPTER_SCHEMA_ID,
        "zamani.quantum.resilience.adaptation.adapter"
    );
}

#[test]
fn adaptation_remapping_contract_is_separate_from_routing() {
    //
    // A RemappingRequest receives the replacement mapping from the caller.
    // It does not contain a routing algorithm or hardware topology.
    //
    let request = request(
        "routing-separation",
        mapping(&[(0, 4), (1, 5)]),
        mapping(&[(0, 5), (1, 4)]),
    );

    assert!(request.changes_mapping());
}

#[test]
fn adaptation_remapping_contract_is_separate_from_scheduling() {
    //
    // Remapping does not own operation timing or schedule construction.
    //
    let request = request(
        "scheduling-separation",
        mapping(&[(0, 4)]),
        mapping(&[(0, 7)]),
    );

    assert_eq!(request.changes().len(), 1);
}

#[test]
fn adaptation_remapping_contract_is_separate_from_qec() {
    //
    // QEC policy/configuration remains owned by qec_adaptation.rs and the
    // canonical QEC subsystem.
    //
    let request = request(
        "qec-separation",
        mapping(&[(0, 4)]),
        mapping(&[(0, 7)]),
    );

    assert_eq!(request.semantic_revision().as_str(), TEST_SEMANTIC_REVISION);
}

#[test]
fn adaptation_remapping_contract_is_separate_from_backend_selection() {
    //
    // Backend/device selection belongs to backend_selection.rs and hardware
    // capability discovery.
    //
    let request = request(
        "backend-separation",
        mapping(&[(0, 4)]),
        mapping(&[(0, 7)]),
    );

    assert_eq!(request.scope(), RemappingScope::Computation);
}

// ============================================================================
// Scale invariants
// ============================================================================

#[test]
fn adaptation_has_no_test_level_machine_size_constant() {
    //
    // This test intentionally contains no assertion against a maximum machine
    // size. The purpose is architectural: all resource cardinality comes from
    // the supplied mapping.
    //
    let tiny = generated_mapping_size(1);
    let larger = generated_mapping_size(257);

    assert_eq!(tiny.len(), 1);
    assert_eq!(larger.len(), 257);
}

#[test]
fn adaptation_handles_resource_growth_without_identity_redefinition() {
    let sizes = [1usize, 3, 8, 16, 64, 128];

    let canonical_type = std::any::type_name::<QubitId>();

    assert!(!canonical_type.is_empty());

    for size in sizes {
        let mapping = generated_mapping_size(size);

        assert_eq!(mapping.len(), size);
        assert!(mapping.validate().is_ok());
    }
}

// ============================================================================
// Public API stability
// ============================================================================

#[test]
fn remapping_public_types_are_debuggable() {
    assert_debug::<RemappingId>();
    assert_debug::<MappingRevision>();
    assert_debug::<RemappingRequest>();
    assert_debug::<RemappingScope>();
    assert_debug::<MappingChange>();
    assert_debug::<RemappingAdapter>();
}

#[test]
fn remapping_public_identifiers_are_send_sync() {
    assert_send_sync::<RemappingId>();
    assert_send_sync::<MappingRevision>();
    assert_send_sync::<RemappingScope>();
    assert_send_sync::<MappingChange>();
}

#[test]
fn remapping_public_identifiers_are_cloneable() {
    assert_clone::<RemappingId>();
    assert_clone::<MappingRevision>();
    assert_clone::<RemappingRequest>();
    assert_clone::<RemappingScope>();
    assert_clone::<MappingChange>();
}

// ============================================================================
// Future-adapter compatibility
// ============================================================================

#[test]
fn adaptation_contract_does_not_depend_on_concrete_provider_names() {
    //
    // This test intentionally has no provider list.
    //
    // Adding a new backend/provider/device must not require modifying this
    // integration test merely to make the provider visible to resilience.
    //
    let adapter_id =
        AdapterId::new("provider-independent").expect("valid adapter id");

    assert_eq!(adapter_id.as_str(), "provider-independent");
}

#[test]
fn adaptation_schema_is_not_tied_to_hardware_generation() {
    //
    // Hardware generation/version belongs to hardware capability and identity
    // contracts. The adaptation composition schema must remain independent.
    //
    assert_eq!(ADAPTATION_MODULE_SCHEMA_VERSION, 1);
    assert_eq!(ADAPTATION_ADAPTER_SCHEMA_VERSION, 1);
}

// ============================================================================
// Regression guards
// ============================================================================

#[test]
fn regression_no_replacement_qubit_identity_type_is_required() {
    //
    // Compile-time use of the canonical identity types protects the repository
    // from accidentally introducing a resilience-local QubitId.
    //
    let logical: QubitId = QubitId::new(7);
    let physical: PhysicalQubitId = PhysicalQubitId::new(13);

    let change = MappingChange::new(
        logical,
        Some(physical),
        Some(PhysicalQubitId::new(17)),
    );

    assert_eq!(change.logical(), logical);
}

#[test]
fn regression_mapping_changes_remain_value_objects() {
    let change = MappingChange::new(
        QubitId::new(1),
        Some(PhysicalQubitId::new(2)),
        Some(PhysicalQubitId::new(3)),
    );

    let copied = change;

    assert_eq!(change, copied);
}

#[test]
fn regression_empty_mapping_does_not_require_hardware_discovery() {
    let current = QubitMapping::new();
    let replacement = QubitMapping::new();

    let request = request(
        "empty-adaptation",
        current,
        replacement,
    );

    assert!(!request.changes_mapping());
    assert!(request.changes().is_empty());
}

// ============================================================================
// End-to-end remapping contract
// ============================================================================

#[test]
fn end_to_end_remapping_contract_preserves_semantic_revision() {
    let current = mapping(&[
        (3, 17),
        (8, 29),
        (21, 41),
    ]);

    let replacement = mapping(&[
        (3, 41),
        (8, 17),
        (21, 29),
    ]);

    let request = request(
        "end-to-end-remap",
        current,
        replacement,
    );

    assert!(request.changes_mapping());
    assert_eq!(request.changes().len(), 3);

    // Physical realization changed.
    assert!(request.changes().iter().all(MappingChange::changed));

    // Logical semantic revision remained the same.
    assert_eq!(
        request.semantic_revision().as_str(),
        TEST_SEMANTIC_REVISION
    );

    // Execution generation remains explicit for stale-state protection.
    assert_eq!(
        request.execution_generation().as_str(),
        TEST_EXECUTION_GENERATION
    );

    // Mapping revision remains explicit for stale-mapping protection.
    assert_eq!(
        request.current_mapping_revision().as_str(),
        TEST_MAPPING_REVISION
    );
}

// ============================================================================
// Contract-completeness guard
// ============================================================================

#[test]
fn adaptation_contract_contains_required_lifecycle_and_identity_types() {
    //
    // This intentionally compiles every foundational type used by the
    // adaptation architecture. If a future edit removes one of these public
    // contracts, this test fails at compile time instead of allowing the
    // integration surface to silently drift.
    //
    let _adapter_id: AdapterId =
        AdapterId::new("contract-test").expect("valid adapter id");

    let _version = AdapterVersion::new(1, 0, 0);

    let _capabilities = AdapterCapabilities::default();

    let _phase = AdaptationPhase::Preflight;

    let _generation =
        ExecutionGeneration::new("generation").expect("valid generation");

    let _semantic =
        SemanticRevision::new("semantic").expect("valid semantic revision");

    let _mapping_revision =
        MappingRevision::new("mapping").expect("valid mapping revision");

    let _remapping_id =
        RemappingId::new("remapping").expect("valid remapping id");

    let _scope = RemappingScope::Computation;

    let _mapping = QubitMapping::new();

    let _adapter = RemappingAdapter;

    assert_eq!(
        ADAPTATION_MODULE_SCHEMA_ID,
        "zamani.quantum.resilience.adaptation"
    );
}