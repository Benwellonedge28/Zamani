
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZHAL Determinism and Reproducibility Tests
//!
//! Path: src/zhal/tests/determinism.rs
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Verifies deterministic behavior of the existing ZHAL contracts:
//!
//! - Immutable hardware observation snapshots.
//! - Snapshot content fingerprints.
//! - Snapshot serialization and validated deserialization.
//! - Physical hardware topology ordering and serialization.
//! - Hardware capability compatibility reports.
//! - Stable provider-scoped hardware identities.
//! - Dynamic, heterogeneous observation collections.
//!
//! ## Architectural boundaries
//!
//! These tests exercise ZHAL's physical hardware observation contracts.
//!
//! They do not:
//!
//! - Parse Zamani grammar.
//! - Interpret source-language topology declarations.
//! - Perform semantic analysis.
//! - Discover physical hardware.
//! - Register or initialize providers.
//! - Select devices.
//! - Reserve or allocate resources.
//! - Schedule hardware.
//! - Execute hardware operations.
//! - Implement quantum routing.
//! - Implement HDL realization.
//! - Implement distributed execution.
//!
//! Source-level requirements remain owned by the frontend semantic
//! authorities. Runtime allocation and orchestration remain owned by ZUR.
//! Quantum, HDL, hybrid and distributed semantics remain with their
//! respective domain authorities.
//!
//! ## Determinism contract
//!
//! A deterministic result requires identical explicit inputs and the
//! same applicable schema and policy versions.
//!
//! These tests do not use:
//!
//! - The system clock.
//! - Random number generation.
//! - Environment variables.
//! - Host-specific hardware observations.
//! - Thread scheduling.
//! - Network access.
//! - Provider iteration order.
//!
//! ## Scalability
//!
//! Test collection sizes are workloads, not supported-capacity limits.
//! No production maximum is inferred from a test fixture.
//!
//! The tests verify that ordered collections preserve their contracts
//! for dynamically generated inputs. They do not promise literal
//! infinite memory, address space or computation.
//!
//! ## Canonical authorities
//!
//! Provider identity:
//!     crate::stdlib::provider
//!
//! Device identity:
//!     crate::stdlib::io::devices
//!
//! Capability identity:
//!     crate::stdlib::capability
//!
//! Hardware identity:
//!     crate::zhal::identity
//!
//! Hardware snapshots:
//!     crate::zhal::model::snapshot
//!
//! Physical topology:
//!     crate::zhal::model::topology
//!
//! Hardware capability compatibility:
//!     crate::zhal::capabilities::compatibility
//!
//! ## Integration
//!
//! This module is registered by src/zhal/tests/mod.rs.
//!
//! The test module is registered by src/zhal/mod.rs under cfg(test).
//!
//! src/zhal/model/mod.rs must expose the existing snapshot.rs module.
//!
//! No production behavior is implemented or changed by this test file.

use std::collections::BTreeSet;

use crate::stdlib::capability::{
    CapabilityId,
    CapabilityStatus,
};

use crate::stdlib::io::devices::DeviceId;

use crate::stdlib::provider::ProviderId;

use crate::zhal::identity::{
    HardwareDeviceIdentity,
    HardwareInventoryId,
    HardwareSnapshotId,
    HardwareTopologyId,
};

use crate::zhal::capabilities::compatibility::{
    evaluate_compatibility,
    CompatibilityPolicy,
    CompatibilityScope,
    CompatibilityStatus,
};

use crate::zhal::capabilities::evidence::{
    EvidenceOrigin,
    EvidenceTimestamp,
    EvidenceVerification,
    HardwareCapabilityEvidence,
};

use crate::zhal::capabilities::requirements::{
    HardwareCapabilityRequirement,
    HardwareCapabilityRequirementSet,
};

use crate::zhal::model::snapshot::{
    HardwareSnapshot,
    SnapshotCapabilityReference,
    SnapshotConsistency,
    SnapshotProvenance,
    SnapshotReference,
    SnapshotReferenceKind,
    SnapshotRevision,
    SnapshotTimestamp,
};

use crate::zhal::model::topology::{
    HardwareTopology,
    LinkDirection,
    TopologyLink,
    TopologyLinkId,
    TopologyLinkKind,
    TopologyNode,
    TopologyNodeId,
    TopologyNodeKind,
    TopologyRevision,
};

// ============================================================================
// SHARED TEST FIXTURES
// ============================================================================

fn provider_id(value: &str) -> ProviderId {
    ProviderId::new(value.to_owned())
        .expect("test provider identity must be valid")
}

fn device_id(value: &str) -> DeviceId {
    DeviceId::new(value.to_owned())
        .expect("test device identity must be valid")
}

fn device_identity(
    provider: &str,
    device: &str,
) -> HardwareDeviceIdentity {
    HardwareDeviceIdentity::new(
        provider_id(provider),
        device_id(device),
    )
}

fn snapshot_id(value: &str) -> HardwareSnapshotId {
    HardwareSnapshotId::new(value)
        .expect("test snapshot identity must be valid")
}

fn inventory_id(value: &str) -> HardwareInventoryId {
    HardwareInventoryId::new(value)
        .expect("test inventory identity must be valid")
}

fn snapshot_revision(value: &str) -> SnapshotRevision {
    SnapshotRevision::new(value)
        .expect("test snapshot revision must be valid")
}

fn snapshot_timestamp() -> SnapshotTimestamp {
    SnapshotTimestamp::new(
        "2026-11-16T00:00:00Z",
        "test.clock.utc",
    )
    .expect("test timestamp must be valid")
}

fn snapshot_provenance() -> SnapshotProvenance {
    SnapshotProvenance::new("test.provider.observation")
        .expect("test provenance must be valid")
        .with_provider_reference("test.provider")
        .expect("test provider reference must be valid")
        .with_collection_method("determinism.fixture")
        .expect("test collection method must be valid")
}

fn capability_id(value: &str) -> CapabilityId {
    CapabilityId::new(value.to_owned())
        .expect("test capability identity must be valid")
}

fn evidence_timestamp(value: i128) -> EvidenceTimestamp {
    EvidenceTimestamp::from_unix_nanos(value)
}

// ============================================================================
// SNAPSHOT FIXTURES
// ============================================================================

fn snapshot_with_references(
    devices: impl IntoIterator<Item = HardwareDeviceIdentity>,
    capabilities: impl IntoIterator<Item = SnapshotCapabilityReference>,
    references: impl IntoIterator<Item = SnapshotReference>,
) -> HardwareSnapshot {
    HardwareSnapshot::builder(
        snapshot_id("snapshot.determinism"),
        inventory_id("inventory.determinism"),
        snapshot_revision("revision.stable"),
        snapshot_timestamp(),
        snapshot_provenance(),
    )
    .with_consistency(SnapshotConsistency::Complete)
    .with_devices(devices)
    .with_capabilities(capabilities)
    .with_references(references)
    .build()
    .expect("determinism fixture must produce a valid snapshot")
}

fn capability_reference(value: &str) -> SnapshotCapabilityReference {
    SnapshotCapabilityReference::new(
        value,
        Some(format!("observation.{value}")),
    )
    .expect("test snapshot capability reference must be valid")
}

fn external_reference(
    kind: &str,
    identity: &str,
    revision: &str,
) -> SnapshotReference {
    SnapshotReference::new(
        SnapshotReferenceKind::new(kind)
            .expect("test reference kind must be valid"),
        identity,
        Some(revision.to_owned()),
    )
    .expect("test external reference must be valid")
}

// ============================================================================
// PHYSICAL TOPOLOGY FIXTURES
// ============================================================================

fn topology_id() -> HardwareTopologyId {
    HardwareTopologyId::new("topology.determinism")
        .expect("test topology identity must be valid")
}

fn topology_revision() -> TopologyRevision {
    TopologyRevision::new("revision.physical.1")
        .expect("test topology revision must be valid")
}

fn topology_node(value: &str) -> TopologyNode {
    TopologyNode::new(
        TopologyNodeId::new(value)
            .expect("test node identity must be valid"),
        TopologyNodeKind::new("hardware.component")
            .expect("test node kind must be valid"),
    )
}

fn topology_link(
    id: &str,
    source: &str,
    target: &str,
) -> TopologyLink {
    TopologyLink::new(
        TopologyLinkId::new(id)
            .expect("test link identity must be valid"),
        TopologyLinkKind::new("hardware.interconnect")
            .expect("test link kind must be valid"),
        TopologyNodeId::new(source)
            .expect("test source identity must be valid"),
        TopologyNodeId::new(target)
            .expect("test target identity must be valid"),
        LinkDirection::Bidirectional,
    )
}

fn topology_from_order(
    node_order: &[&str],
    link_order: &[(&str, &str, &str)],
) -> HardwareTopology {
    let nodes = node_order
        .iter()
        .map(|id| topology_node(id));

    let links = link_order
        .iter()
        .map(|(id, source, target)| {
            topology_link(id, source, target)
        });

    HardwareTopology::new(
        topology_id(),
        topology_revision(),
        nodes,
        links,
    )
    .expect("test physical topology must be valid")
}

// ============================================================================
// COMPATIBILITY FIXTURES
// ============================================================================

fn requirement(value: &str) -> HardwareCapabilityRequirement {
    HardwareCapabilityRequirement::new(capability_id(value))
}

fn capability_evidence(
    capability: &str,
    status: CapabilityStatus,
    observed_at: i128,
) -> HardwareCapabilityEvidence {
    HardwareCapabilityEvidence::new(
        capability_id(capability),
        status,
        EvidenceOrigin::new(
            "test.provider.report",
            Some(format!("evidence.{capability}")),
        )
        .expect("test evidence origin must be valid"),
        evidence_timestamp(observed_at),
        "determinism test provider",
        "deterministic capability observation",
    )
    .expect("test evidence must be valid")
    .with_verification(
        EvidenceVerification::ProviderReported,
        None,
    )
    .expect("test evidence verification must be valid")
}

fn evaluate_report(
    requirements: &HardwareCapabilityRequirementSet,
    evidence: &[HardwareCapabilityEvidence],
) -> crate::zhal::capabilities::compatibility::CompatibilityReport {
    evaluate_compatibility(
        requirements,
        evidence,
        evidence_timestamp(100),
        CompatibilityScope::AllEvidence,
        CompatibilityPolicy::new(),
    )
    .expect("deterministic compatibility evaluation must succeed")
}

// ============================================================================
// SNAPSHOT DETERMINISM
// ============================================================================

#[test]
fn identical_snapshot_inputs_produce_identical_fingerprints() {
    let first = snapshot_with_references(
        [
            device_identity("provider.alpha", "device.one"),
            device_identity("provider.beta", "device.two"),
        ],
        [
            capability_reference("classical.parallel_execution"),
            capability_reference("quantum.measurement"),
        ],
        [
            external_reference(
                "hardware.observation",
                "observation.one",
                "revision.one",
            ),
        ],
    );

    let second = snapshot_with_references(
        [
            device_identity("provider.alpha", "device.one"),
            device_identity("provider.beta", "device.two"),
        ],
        [
            capability_reference("classical.parallel_execution"),
            capability_reference("quantum.measurement"),
        ],
        [
            external_reference(
                "hardware.observation",
                "observation.one",
                "revision.one",
            ),
        ],
    );

    assert_eq!(first, second);
    assert_eq!(first.fingerprint(), second.fingerprint());
    assert_eq!(
        first.fingerprint().as_str(),
        second.fingerprint().as_str(),
    );
}

#[test]
fn snapshot_fingerprint_is_independent_of_reference_insertion_order() {
    let first = snapshot_with_references(
        [
            device_identity("provider.beta", "device.two"),
            device_identity("provider.alpha", "device.one"),
        ],
        [
            capability_reference("quantum.measurement"),
            capability_reference("classical.parallel_execution"),
        ],
        [
            external_reference(
                "hardware.topology",
                "topology.two",
                "revision.two",
            ),
            external_reference(
                "hardware.observation",
                "observation.one",
                "revision.one",
            ),
        ],
    );

    let second = snapshot_with_references(
        [
            device_identity("provider.alpha", "device.one"),
            device_identity("provider.beta", "device.two"),
        ],
        [
            capability_reference("classical.parallel_execution"),
            capability_reference("quantum.measurement"),
        ],
        [
            external_reference(
                "hardware.observation",
                "observation.one",
                "revision.one",
            ),
            external_reference(
                "hardware.topology",
                "topology.two",
                "revision.two",
            ),
        ],
    );

    assert_eq!(first.fingerprint(), second.fingerprint());
    assert_eq!(
        serde_json::to_vec(&first).expect("snapshot serialization"),
        serde_json::to_vec(&second).expect("snapshot serialization"),
    );
}

#[test]
fn snapshot_iteration_order_is_stable() {
    let snapshot = snapshot_with_references(
        [
            device_identity("provider.zeta", "device.z"),
            device_identity("provider.alpha", "device.a"),
            device_identity("provider.mu", "device.m"),
        ],
        [
            capability_reference("future.photonic.compute"),
            capability_reference("classical.compute"),
            capability_reference("quantum.compute"),
        ],
        [],
    );

    let devices = snapshot
        .devices()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    let mut sorted_devices = devices.clone();
    sorted_devices.sort();

    assert_eq!(devices, sorted_devices);

    let capabilities = snapshot
        .payload()
        .capabilities()
        .map(|entry| entry.identity().to_owned())
        .collect::<Vec<_>>();

    let mut sorted_capabilities = capabilities.clone();
    sorted_capabilities.sort();

    assert_eq!(capabilities, sorted_capabilities);
}

#[test]
fn snapshot_fingerprint_changes_when_observed_content_changes() {
    let original = snapshot_with_references(
        [device_identity("provider.alpha", "device.one")],
        [capability_reference("classical.compute")],
        [],
    );

    let changed = snapshot_with_references(
        [
            device_identity("provider.alpha", "device.one"),
            device_identity("provider.alpha", "device.two"),
        ],
        [capability_reference("classical.compute")],
        [],
    );

    assert_ne!(
        original.fingerprint(),
        changed.fingerprint(),
    );
}

#[test]
fn snapshot_fingerprint_changes_when_observation_consistency_changes() {
    let complete = HardwareSnapshot::builder(
        snapshot_id("snapshot.consistency"),
        inventory_id("inventory.consistency"),
        snapshot_revision("revision.one"),
        snapshot_timestamp(),
        snapshot_provenance(),
    )
    .with_consistency(SnapshotConsistency::Complete)
    .build()
    .expect("complete snapshot must be valid");

    let partial = HardwareSnapshot::builder(
        snapshot_id("snapshot.consistency"),
        inventory_id("inventory.consistency"),
        snapshot_revision("revision.one"),
        snapshot_timestamp(),
        snapshot_provenance(),
    )
    .with_consistency(SnapshotConsistency::Partial)
    .build()
    .expect("partial snapshot must be valid");

    assert_ne!(
        complete.fingerprint(),
        partial.fingerprint(),
    );
}

#[test]
fn snapshot_serialization_is_repeatable() {
    let snapshot = snapshot_with_references(
        [
            device_identity("provider.alpha", "device.one"),
            device_identity("provider.beta", "device.two"),
        ],
        [
            capability_reference("classical.compute"),
            capability_reference("hdl.realization"),
            capability_reference("quantum.execution"),
        ],
        [
            external_reference(
                "hardware.observation",
                "observation.one",
                "revision.one",
            ),
        ],
    );

    let first = serde_json::to_vec(&snapshot)
        .expect("first snapshot serialization must succeed");

    let second = serde_json::to_vec(&snapshot)
        .expect("second snapshot serialization must succeed");

    assert_eq!(first, second);
}

#[test]
fn snapshot_round_trip_preserves_content_and_fingerprint() {
    let original = snapshot_with_references(
        [
            device_identity("provider.alpha", "device.one"),
            device_identity("provider.beta", "device.two"),
        ],
        [
            capability_reference("classical.compute"),
            capability_reference("future.photonic.compute"),
        ],
        [
            external_reference(
                "hardware.topology",
                "topology.primary",
                "revision.topology.1",
            ),
        ],
    );

    let encoded = serde_json::to_vec(&original)
        .expect("snapshot serialization must succeed");

    let restored: HardwareSnapshot =
        serde_json::from_slice(&encoded)
            .expect("snapshot deserialization must validate");

    assert_eq!(original, restored);
    assert_eq!(
        original.fingerprint(),
        restored.fingerprint(),
    );
    assert!(
        restored.verify_integrity()
            .expect("snapshot integrity check must succeed"),
    );
}

#[test]
fn snapshot_deserialization_rejects_content_tampering() {
    let original = snapshot_with_references(
        [device_identity("provider.alpha", "device.one")],
        [capability_reference("classical.compute")],
        [],
    );

    let mut encoded = serde_json::to_value(&original)
        .expect("snapshot serialization must succeed");

    encoded["payload"]["revision"] =
        serde_json::Value::String("revision.tampered".to_owned());

    let restored =
        serde_json::from_value::<HardwareSnapshot>(encoded);

    assert!(
        restored.is_err(),
        "modified content must not retain an old fingerprint",
    );
}

#[test]
fn empty_snapshot_is_not_assumed_to_be_complete() {
    let snapshot = HardwareSnapshot::builder(
        snapshot_id("snapshot.empty"),
        inventory_id("inventory.empty"),
        snapshot_revision("revision.empty"),
        snapshot_timestamp(),
        snapshot_provenance(),
    )
    .build()
    .expect("empty observation snapshot is valid");

    assert_eq!(
        snapshot.consistency(),
        SnapshotConsistency::Unknown,
    );

    assert!(!snapshot.is_declared_complete());
    assert!(!snapshot.is_partial());
}

// ============================================================================
// PHYSICAL TOPOLOGY DETERMINISM
// ============================================================================

#[test]
fn topology_node_iteration_is_independent_of_insertion_order() {
    let first = topology_from_order(
        &["node.gamma", "node.alpha", "node.beta"],
        &[],
    );

    let second = topology_from_order(
        &["node.beta", "node.gamma", "node.alpha"],
        &[],
    );

    let first_order = first
        .nodes()
        .map(|node| node.id().as_str().to_owned())
        .collect::<Vec<_>>();

    let second_order = second
        .nodes()
        .map(|node| node.id().as_str().to_owned())
        .collect::<Vec<_>>();

    assert_eq!(first_order, second_order);
    assert_eq!(
        first_order,
        vec!["node.alpha", "node.beta", "node.gamma"],
    );
}

#[test]
fn topology_link_iteration_is_independent_of_insertion_order() {
    let nodes = ["node.alpha", "node.beta", "node.gamma"];

    let first = topology_from_order(
        &nodes,
        &[
            ("link.gamma", "node.beta", "node.gamma"),
            ("link.alpha", "node.alpha", "node.beta"),
        ],
    );

    let second = topology_from_order(
        &nodes,
        &[
            ("link.alpha", "node.alpha", "node.beta"),
            ("link.gamma", "node.beta", "node.gamma"),
        ],
    );

    let first_order = first
        .links()
        .map(|link| link.id().as_str().to_owned())
        .collect::<Vec<_>>();

    let second_order = second
        .links()
        .map(|link| link.id().as_str().to_owned())
        .collect::<Vec<_>>();

    assert_eq!(first_order, second_order);
    assert_eq!(
        first_order,
        vec!["link.alpha", "link.gamma"],
    );
}

#[test]
fn equivalent_topologies_have_identical_serialization() {
    let nodes = ["node.alpha", "node.beta", "node.gamma"];

    let first = topology_from_order(
        &nodes,
        &[
            ("link.gamma", "node.beta", "node.gamma"),
            ("link.alpha", "node.alpha", "node.beta"),
        ],
    );

    let second = topology_from_order(
        &nodes,
        &[
            ("link.alpha", "node.alpha", "node.beta"),
            ("link.gamma", "node.beta", "node.gamma"),
        ],
    );

    let first_bytes = serde_json::to_vec(&first)
        .expect("first topology serialization must succeed");

    let second_bytes = serde_json::to_vec(&second)
        .expect("second topology serialization must succeed");

    assert_eq!(first_bytes, second_bytes);
}

#[test]
fn topology_round_trip_preserves_order_and_structure() {
    let original = topology_from_order(
        &["node.gamma", "node.alpha", "node.beta"],
        &[
            ("link.gamma", "node.beta", "node.gamma"),
            ("link.alpha", "node.alpha", "node.beta"),
        ],
    );

    let encoded = serde_json::to_vec(&original)
        .expect("topology serialization must succeed");

    let restored: HardwareTopology =
        serde_json::from_slice(&encoded)
            .expect("topology deserialization must validate");

    assert_eq!(original, restored);

    let original_nodes = original
        .nodes()
        .map(|node| node.id().as_str().to_owned())
        .collect::<Vec<_>>();

    let restored_nodes = restored
        .nodes()
        .map(|node| node.id().as_str().to_owned())
        .collect::<Vec<_>>();

    assert_eq!(original_nodes, restored_nodes);
}

#[test]
fn topology_preserves_directionality_deterministically() {
    let topology = topology_from_order(
        &["node.alpha", "node.beta"],
        &[("link.directed", "node.alpha", "node.beta")],
    );

    let link = topology
        .links()
        .next()
        .expect("topology must contain its link");

    assert_eq!(link.direction(), LinkDirection::Bidirectional);

    let directed = TopologyLink::new(
        TopologyLinkId::new("link.directed")
            .expect("valid link identity"),
        TopologyLinkKind::new("hardware.interconnect")
            .expect("valid link kind"),
        TopologyNodeId::new("node.alpha")
            .expect("valid source identity"),
        TopologyNodeId::new("node.beta")
            .expect("valid target identity"),
        LinkDirection::Directed,
    );

    let directed_topology = HardwareTopology::new(
        topology_id(),
        topology_revision(),
        [
            topology_node("node.alpha"),
            topology_node("node.beta"),
        ],
        [directed],
    )
    .expect("directed topology must be valid");

    let forward = directed_topology.neighbours(
        &TopologyNodeId::new("node.alpha").unwrap(),
        crate::zhal::model::topology::TraversalMode::RespectDirection,
    );

    let reverse = directed_topology.neighbours(
        &TopologyNodeId::new("node.beta").unwrap(),
        crate::zhal::model::topology::TraversalMode::RespectDirection,
    );

    assert_eq!(forward.len(), 1);
    assert!(reverse.is_empty());
}

// ============================================================================
// COMPATIBILITY REPORT DETERMINISM
// ============================================================================

#[test]
fn identical_compatibility_inputs_produce_identical_reports() {
    let requirements = HardwareCapabilityRequirementSet::from_requirements([
        requirement("classical.parallel_execution"),
        requirement("quantum.measurement"),
        requirement("hdl.realization"),
    ]);

    let evidence = vec![
        capability_evidence(
            "classical.parallel_execution",
            CapabilityStatus::Available,
            50,
        ),
        capability_evidence(
            "quantum.measurement",
            CapabilityStatus::Available,
            60,
        ),
        capability_evidence(
            "hdl.realization",
            CapabilityStatus::Available,
            70,
        ),
    ];

    let first = evaluate_report(&requirements, &evidence);
    let second = evaluate_report(&requirements, &evidence);

    assert_eq!(first, second);
    assert_eq!(first.status(), CompatibilityStatus::Compatible);
}

#[test]
fn compatibility_findings_preserve_requirement_order() {
    let requirements = HardwareCapabilityRequirementSet::from_requirements([
        requirement("quantum.measurement"),
        requirement("classical.parallel_execution"),
        requirement("hdl.realization"),
    ]);

    let evidence = vec![
        capability_evidence(
            "hdl.realization",
            CapabilityStatus::Available,
            70,
        ),
        capability_evidence(
            "classical.parallel_execution",
            CapabilityStatus::Available,
            60,
        ),
        capability_evidence(
            "quantum.measurement",
            CapabilityStatus::Available,
            50,
        ),
    ];

    let report = evaluate_report(&requirements, &evidence);

    let findings = report
        .findings()
        .iter()
        .map(|finding| finding.capability().as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        findings,
        vec![
            "quantum.measurement",
            "classical.parallel_execution",
            "hdl.realization",
        ],
    );
}

#[test]
fn compatibility_result_is_independent_of_evidence_input_order() {
    let requirements = HardwareCapabilityRequirementSet::from_requirements([
        requirement("classical.parallel_execution"),
        requirement("quantum.measurement"),
        requirement("hdl.realization"),
    ]);

    let first_evidence = vec![
        capability_evidence(
            "classical.parallel_execution",
            CapabilityStatus::Available,
            50,
        ),
        capability_evidence(
            "quantum.measurement",
            CapabilityStatus::Available,
            60,
        ),
        capability_evidence(
            "hdl.realization",
            CapabilityStatus::Available,
            70,
        ),
    ];

    let second_evidence = vec![
        capability_evidence(
            "hdl.realization",
            CapabilityStatus::Available,
            70,
        ),
        capability_evidence(
            "quantum.measurement",
            CapabilityStatus::Available,
            60,
        ),
        capability_evidence(
            "classical.parallel_execution",
            CapabilityStatus::Available,
            50,
        ),
    ];

    let first = evaluate_report(&requirements, &first_evidence);
    let second = evaluate_report(&requirements, &second_evidence);

    assert_eq!(first, second);
}

#[test]
fn compatibility_does_not_treat_unknown_evidence_as_available() {
    let requirements = HardwareCapabilityRequirementSet::from_requirements([
        requirement("future.photonic.computation"),
    ]);

    let report = evaluate_report(&requirements, &[]);

    assert_eq!(
        report.status(),
        CompatibilityStatus::Indeterminate,
    );

    assert_eq!(
        report.findings()[0].kind(),
        crate::zhal::capabilities::compatibility::CompatibilityFindingKind::Unknown,
    );
}

#[test]
fn compatibility_does_not_merge_distinct_provider_evidence_into_one_device() {
    let requirements = HardwareCapabilityRequirementSet::from_requirements([
        requirement("classical.compute"),
        requirement("quantum.measurement"),
    ]);

    let first_provider_evidence = capability_evidence(
        "classical.compute",
        CapabilityStatus::Available,
        50,
    )
    .with_provider(provider_id("provider.classical"))
    .expect("provider attribution must be valid");

    let second_provider_evidence = capability_evidence(
        "quantum.measurement",
        CapabilityStatus::Available,
        50,
    )
    .with_provider(provider_id("provider.quantum"))
    .expect("provider attribution must be valid");

    let report = evaluate_compatibility(
        &requirements,
        &[first_provider_evidence, second_provider_evidence],
        evidence_timestamp(100),
        CompatibilityScope::AllEvidence,
        CompatibilityPolicy::new(),
    )
    .expect("aggregate compatibility evaluation must succeed");

    // AllEvidence aggregates supplied evidence. It does not prove
    // that one physical device can satisfy both requirements.
    assert_eq!(report.status(), CompatibilityStatus::Compatible);
    assert_eq!(report.len(), 2);
}

// ============================================================================
// DYNAMIC AND HETEROGENEOUS COLLECTION DETERMINISM
// ============================================================================

#[test]
fn dynamically_generated_device_collections_have_stable_order() {
    // This is a repeatable stress workload, not a maximum supported
    // number of devices or a universal capacity limit.
    let workload_size = 512_usize;

    let devices = (0..workload_size)
        .rev()
        .map(|index| {
            device_identity(
                "provider.dynamic",
                &format!("device.{index:08}"),
            )
        })
        .collect::<BTreeSet<_>>();

    let ordered = devices
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    assert_eq!(ordered.len(), workload_size);

    assert!(
        ordered.windows(2).all(|pair| pair[0] < pair[1]),
        "provider-scoped device identities must have stable ordering",
    );
}

#[test]
fn heterogeneous_capability_names_remain_open_world_and_ordered() {
    let names = [
        "classical.cpu.execution",
        "quantum.qpu.execution",
        "hdl.fpga.realization",
        "accelerator.tensor.execution",
        "distributed.remote.execution",
        "embedded.sensor.access",
        "future.photonic.execution",
        "future.molecular.execution",
        "future.neuromorphic.execution",
    ];

    let capabilities = names
        .iter()
        .map(|name| capability_id(name))
        .collect::<BTreeSet<_>>();

    let ordered = capabilities
        .iter()
        .map(|capability| capability.as_str())
        .collect::<Vec<_>>();

    let mut expected = names.to_vec();
    expected.sort_unstable();

    assert_eq!(ordered, expected);
}

#[test]
fn long_valid_identity_is_not_rejected_by_an_arbitrary_test_limit() {
    // The length is an input workload, not a ZHAL maximum.
    let identifier = format!(
        "future.provider.{}",
        "x".repeat(16_384),
    );

    let identity = HardwareSnapshotId::new(identifier.clone())
        .expect("valid long identity must not hit an arbitrary ceiling");

    assert_eq!(identity.as_str(), identifier);
}

#[test]
fn duplicate_snapshot_references_are_deterministically_deduplicated() {
    let repeated_device = device_identity(
        "provider.alpha",
        "device.one",
    );

    let snapshot = snapshot_with_references(
        [
            repeated_device.clone(),
            repeated_device,
        ],
        [
            capability_reference("classical.compute"),
            capability_reference("classical.compute"),
        ],
        [
            external_reference(
                "hardware.observation",
                "observation.one",
                "revision.one",
            ),
            external_reference(
                "hardware.observation",
                "observation.one",
                "revision.one",
            ),
        ],
    );

    assert_eq!(snapshot.payload().device_count(), 1);
    assert_eq!(snapshot.payload().capability_count(), 1);
    assert_eq!(snapshot.payload().reference_count(), 1);
}
