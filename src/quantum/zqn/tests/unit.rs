//! Zamani Quantum Noise (ZQN) — Unit Tests
//!
//! # Purpose
//!
//! This module contains deterministic, dependency-conscious unit tests for the
//! ZQN public foundation.
//!
//! The tests intentionally validate:
//!
//! - canonical quantum-resource identity integration;
//! - ZQN-owned typed identity semantics;
//! - conversion round-trips;
//! - checked identifier arithmetic;
//! - deterministic formatting;
//! - identifier-domain separation;
//! - value ordering;
//! - absence of hidden/global identity allocation;
//! - large representable identifier values;
//! - boundary behavior;
//! - basic Send/Sync contracts;
//! - stable public module exposure;
//! - write-once/scale-everywhere architectural invariants.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                              │
//!                              │ canonical resource identity
//!                              ▼
//!                    quantum::ir::qubit
//!                              │
//!                              ▼
//!                       ┌─────────────┐
//!                       │     ZQN     │
//!                       │             │
//!                       │ core::ids   │
//!                       └──────┬──────┘
//!                              │
//!             ┌────────────────┼─────────────────┐
//!             ▼                ▼                 ▼
//!        probability       channels            faults
//!             │                │                 │
//!             └────────────────┼─────────────────┘
//!                              ▼
//!                            noise
//!                              │
//!             ┌────────────────┼────────────────┐
//!             ▼                ▼                ▼
//!       calibration    characterization    simulation
//!             │                │                │
//!             └────────────────┼────────────────┘
//!                              ▼
//!                       propagation/target
//!                              │
//!                              ▼
//!                         integration
//! ```
//!
//! # Ownership
//!
//! This file owns only unit-test coverage.
//!
//! It does not define:
//!
//! - quantum semantics;
//! - quantum channels;
//! - noise models;
//! - fault semantics;
//! - calibration semantics;
//! - simulation semantics;
//! - target capabilities;
//! - hardware behavior;
//! - runtime behavior;
//! - identifiers.
//!
//! Production definitions remain in their owning modules.
//!
//! # Canonical identity rule
//!
//! ZQN must never define another `QubitId` or `PhysicalQubitId`.
//!
//! The canonical identities are owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The tests therefore import those identities through the ZQN identity
//! boundary only to verify that they remain the canonical types.
//!
//! # Scalability rule
//!
//! These tests intentionally do not define or assert an architectural maximum
//! for:
//!
//! - qubits;
//! - physical qubits;
//! - operations;
//! - channels;
//! - faults;
//! - circuit depth;
//! - machine size.
//!
//! A finite test value is used only because a test execution itself has finite
//! resources. It must never become a semantic limit in production code.
//!
//! # Determinism
//!
//! Tests use no random number generator and no wall-clock time.
//!
//! The results must be deterministic across repeated executions and independent
//! of execution order.
//!
//! # Safety
//!
//! This file contains no unsafe code.
//!
//! The test module explicitly forbids unsafe Rust.
//!
//! # Rust compatibility
//!
//! Required:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # Integration
//!
//! The file is intended to be compiled from:
//!
//! ```text
//! src/quantum/zqn/tests/mod.rs
//! ```
//!
//! with:
//!
//! ```text
//! pub mod unit;
//! ```
//!
//! The parent ZQN module should expose the test module only under `cfg(test)`:
//!
//! ```text
//! #[cfg(test)]
//! mod tests;
//! ```
//!
//! This prevents test-only code from becoming part of the production API.
//!
//! # Dependency policy
//!
//! This file intentionally uses only:
//!
//! - the Rust standard library;
//! - the canonical Quantum IR qubit types;
//! - the completed ZQN core identity API.
//!
//! It does not depend on vendor SDKs, network access, files, external services,
//! simulator backends, or QPU credentials.
//!
//! # Completion invariant
//!
//! This file is considered complete when:
//!
//! 1. all tests compile against the stable ZQN core identity contract;
//! 2. canonical IR qubit identities remain canonical;
//! 3. typed ZQN IDs cannot accidentally be mixed;
//! 4. identifier conversion is lossless;
//! 5. checked arithmetic handles the maximum value without wrapping;
//! 6. formatting is deterministic;
//! 7. ordering is deterministic;
//! 8. no global mutable allocator is required;
//! 9. very large representable identifier values remain valid;
//! 10. no test introduces a hardware-size assumption;
//! 11. no unsafe Rust exists;
//! 12. tests remain valid when downstream ZQN subsystems are implemented;
//! 13. adding a new ZQN subsystem does not require changing these foundational
//!     tests unless its public contract itself changes.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use crate::quantum::ir::qubit::{
    PhysicalQubitId,
    QubitId,
};

use crate::quantum::zqn::core::ids::{
    CalibrationId,
    ChannelId,
    CharacterizationId,
    CorrelationId,
    DistributionId,
    ErrorBudgetId,
    ExperimentId,
    FaultId,
    NoiseApplicationId,
    NoiseModelId,
    NoiseParameterId,
    NoiseProfileId,
    NoiseRealizationId,
    NoiseSnapshotId,
    ObservationId,
    QubitId as ZqnQubitId,
    PhysicalQubitId as ZqnPhysicalQubitId,
    ZqnIdKind,
    ZqnIdValue,
    ZqnObjectId,
};

// =============================================================================
// Test helpers
// =============================================================================

/// Asserts that an identifier type has ordinary immutable value semantics.
///
/// This helper deliberately uses only operations guaranteed by the public ID
/// contract: construction, extraction, equality, hashing, ordering, and
/// formatting.
fn assert_value_identity<T>(first: T, second: T, expected_value: ZqnIdValue)
where
    T: Copy
        + Eq
        + Ord
        + Hash
        + Debug
        + std::fmt::Display
        + From<ZqnIdValue>
        + Into<ZqnIdValue>,
{
    assert_eq!(first, second);
    assert_eq!(first.into(), expected_value);
    assert_eq!(second.into(), expected_value);
    assert_eq!(first.cmp(&second), std::cmp::Ordering::Equal);

    let first_debug = format!("{first:?}");
    let second_debug = format!("{second:?}");

    assert_eq!(first_debug, second_debug);
}

/// Verifies a standard hash/equality contract.
///
/// Equal values must hash to the same bucket key in standard hash maps/sets.
fn assert_hash_equality<T>(value: T)
where
    T: Copy + Eq + Hash,
{
    let mut set = HashSet::new();
    set.insert(value);

    assert!(set.contains(&value));
    assert_eq!(set.len(), 1);

    let mut map = HashMap::new();
    map.insert(value, true);

    assert_eq!(map.get(&value), Some(&true));
}

/// Verifies ordering and uniqueness in an ordered collection.
fn assert_ordering<T>(low: T, high: T)
where
    T: Copy + Ord,
{
    assert!(low < high);
    assert!(high > low);
    assert_ne!(low, high);

    let mut set = BTreeSet::new();
    set.insert(low);
    set.insert(high);

    assert_eq!(set.len(), 2);

    let values: Vec<T> = set.into_iter().collect();

    assert_eq!(values[0], low);
    assert_eq!(values[1], high);
}

// =============================================================================
// Canonical Quantum IR identity integration
// =============================================================================

#[test]
fn canonical_logical_qubit_identity_is_preserved() {
    let canonical = QubitId::new(7);

    // `zqn::core::ids::QubitId` is an alias/re-export, not a new type.
    let through_zqn: ZqnQubitId = canonical;

    assert_eq!(through_zqn, canonical);
    assert_eq!(through_zqn.index(), 7);
}

#[test]
fn canonical_physical_qubit_identity_is_preserved() {
    let canonical = PhysicalQubitId::new(11);

    // `zqn::core::ids::PhysicalQubitId` is an alias/re-export, not a new type.
    let through_zqn: ZqnPhysicalQubitId = canonical;

    assert_eq!(through_zqn, canonical);
    assert_eq!(through_zqn.index(), 11);
}

#[test]
fn_logical_and_physical_qubit_domains_remain_distinct() {
    let logical = QubitId::new(3);
    let physical = PhysicalQubitId::new(3);

    // Equal numeric indices do not make the semantic domains interchangeable.
    assert_eq!(logical.index(), physical.index());

    // Compile-time type separation is the important invariant here.
    //
    // The following would intentionally fail to compile and therefore is not
    // written as executable test code:
    //
    // let _: PhysicalQubitId = logical;
    //
    // Likewise:
    //
    // let _: QubitId = physical;
}

#[test]
fn zqn_does_not_introduce_a_competing_qubit_identifier() {
    fn accepts_canonical_logical(_: QubitId) {}
    fn accepts_canonical_physical(_: PhysicalQubitId) {}

    let logical = ZqnQubitId::new(17);
    let physical = ZqnPhysicalQubitId::new(19);

    accepts_canonical_logical(logical);
    accepts_canonical_physical(physical);
}

// =============================================================================
// Generic ZQN object identity
// =============================================================================

#[test]
fn_generic_object_id_round_trip_is_lossless() {
    let original = ZqnObjectId::new(42);
    let raw: ZqnIdValue = original.into();
    let reconstructed = ZqnObjectId::from(raw);

    assert_eq!(original, reconstructed);
    assert_eq!(reconstructed.value(), 42);
}

#[test]
fn_generic_object_id_display_is_deterministic() {
    let id = ZqnObjectId::new(42);

    assert_eq!(id.to_string(), "zqn:42");
    assert_eq!(id.to_string(), id.to_string());
}

#[test]
fn_generic_object_id_is_orderable() {
    let low = ZqnObjectId::new(1);
    let high = ZqnObjectId::new(2);

    assert_ordering(low, high);
}

#[test]
fn_generic_object_id_hashes_consistently() {
    assert_hash_equality(ZqnObjectId::new(123));
}

// =============================================================================
// Typed ZQN identifier semantics
// =============================================================================

#[test]
fn noise_model_id_has_stable_value_semantics() {
    let id = NoiseModelId::new(10);

    assert_value_identity(id, NoiseModelId::new(10), 10);
    assert_eq!(NoiseModelId::prefix(), "noise-model");
    assert_eq!(id.to_string(), "noise-model:10");
}

#[test]
fn channel_id_has_stable_value_semantics() {
    let id = ChannelId::new(20);

    assert_value_identity(id, ChannelId::new(20), 20);
    assert_eq!(ChannelId::prefix(), "channel");
    assert_eq!(id.to_string(), "channel:20");
}

#[test]
fn fault_id_has_stable_value_semantics() {
    let id = FaultId::new(30);

    assert_value_identity(id, FaultId::new(30), 30);
    assert_eq!(FaultId::prefix(), "fault");
    assert_eq!(id.to_string(), "fault:30");
}

#[test]
fn noise_application_id_has_stable_value_semantics() {
    let id = NoiseApplicationId::new(40);

    assert_value_identity(id, NoiseApplicationId::new(40), 40);
    assert_eq!(NoiseApplicationId::prefix(), "noise-application");
    assert_eq!(id.to_string(), "noise-application:40");
}

#[test]
fn noise_snapshot_id_has_stable_value_semantics() {
    let id = NoiseSnapshotId::new(50);

    assert_value_identity(id, NoiseSnapshotId::new(50), 50);
    assert_eq!(NoiseSnapshotId::prefix(), "noise-snapshot");
    assert_eq!(id.to_string(), "noise-snapshot:50");
}

#[test]
fn calibration_id_has_stable_value_semantics() {
    let id = CalibrationId::new(60);

    assert_value_identity(id, CalibrationId::new(60), 60);
    assert_eq!(CalibrationId::prefix(), "calibration");
    assert_eq!(id.to_string(), "calibration:60");
}

#[test]
fn characterization_id_has_stable_value_semantics() {
    let id = CharacterizationId::new(70);

    assert_value_identity(id, CharacterizationId::new(70), 70);
    assert_eq!(
        CharacterizationId::prefix(),
        "characterization"
    );
    assert_eq!(id.to_string(), "characterization:70");
}

#[test]
fn experiment_id_has_stable_value_semantics() {
    let id = ExperimentId::new(80);

    assert_value_identity(id, ExperimentId::new(80), 80);
    assert_eq!(ExperimentId::prefix(), "experiment");
    assert_eq!(id.to_string(), "experiment:80");
}

#[test]
fn observation_id_has_stable_value_semantics() {
    let id = ObservationId::new(90);

    assert_value_identity(id, ObservationId::new(90), 90);
    assert_eq!(ObservationId::prefix(), "observation");
    assert_eq!(id.to_string(), "observation:90");
}

#[test]
fn noise_realization_id_has_stable_value_semantics() {
    let id = NoiseRealizationId::new(100);

    assert_value_identity(id, NoiseRealizationId::new(100), 100);
    assert_eq!(
        NoiseRealizationId::prefix(),
        "noise-realization"
    );
    assert_eq!(id.to_string(), "noise-realization:100");
}

#[test]
fn correlation_id_has_stable_value_semantics() {
    let id = CorrelationId::new(110);

    assert_value_identity(id, CorrelationId::new(110), 110);
    assert_eq!(CorrelationId::prefix(), "correlation");
    assert_eq!(id.to_string(), "correlation:110");
}

#[test]
fn noise_parameter_id_has_stable_value_semantics() {
    let id = NoiseParameterId::new(120);

    assert_value_identity(id, NoiseParameterId::new(120), 120);
    assert_eq!(
        NoiseParameterId::prefix(),
        "noise-parameter"
    );
    assert_eq!(id.to_string(), "noise-parameter:120");
}

#[test]
fn distribution_id_has_stable_value_semantics() {
    let id = DistributionId::new(130);

    assert_value_identity(id, DistributionId::new(130), 130);
    assert_eq!(DistributionId::prefix(), "distribution");
    assert_eq!(id.to_string(), "distribution:130");
}

#[test]
fn error_budget_id_has_stable_value_semantics() {
    let id = ErrorBudgetId::new(140);

    assert_value_identity(id, ErrorBudgetId::new(140), 140);
    assert_eq!(ErrorBudgetId::prefix(), "error-budget");
    assert_eq!(id.to_string(), "error-budget:140");
}

#[test]
fn noise_profile_id_has_stable_value_semantics() {
    let id = NoiseProfileId::new(150);

    assert_value_identity(id, NoiseProfileId::new(150), 150);
    assert_eq!(NoiseProfileId::prefix(), "noise-profile");
    assert_eq!(id.to_string(), "noise-profile:150");
}

// =============================================================================
// Identifier domain separation
// =============================================================================

#[test]
fn typed_identifier_domains_are_not_interchangeable() {
    let noise_model = NoiseModelId::new(1);
    let channel = ChannelId::new(1);
    let fault = FaultId::new(1);

    // Same underlying numeric value does not imply semantic equality because
    // these are distinct Rust types.
    assert_eq!(noise_model.value(), channel.value());
    assert_eq!(channel.value(), fault.value());

    // The values can coexist safely in collections because the Rust types are
    // distinct.
    assert_eq!(noise_model.to_string(), "noise-model:1");
    assert_eq!(channel.to_string(), "channel:1");
    assert_eq!(fault.to_string(), "fault:1");
}

#[test]
fn typed_identifier_domains_can_be_used_as_independent_map_keys() {
    let noise_model = NoiseModelId::new(1);
    let channel = ChannelId::new(1);
    let fault = FaultId::new(1);

    let mut noise_models = BTreeMap::new();
    noise_models.insert(noise_model, "model");

    let mut channels = BTreeMap::new();
    channels.insert(channel, "channel");

    let mut faults = BTreeMap::new();
    faults.insert(fault, "fault");

    assert_eq!(noise_models.get(&noise_model), Some(&"model"));
    assert_eq!(channels.get(&channel), Some(&"channel"));
    assert_eq!(faults.get(&fault), Some(&"fault"));
}

// =============================================================================
// Checked identifier arithmetic
// =============================================================================

#[test]
fn checked_next_increments_without_wrapping() {
    let id = NoiseModelId::new(41);

    let next = id
        .checked_next()
        .expect("41 must have a representable successor");

    assert_eq!(next.value(), 42);
}

#[test]
fn checked_next_handles_maximum_without_overflow() {
    let id = NoiseModelId::new(ZqnIdValue::MAX);

    assert_eq!(id.checked_next(), None);
}

#[test]
fn checked_next_does_not_modify_the_original_identifier() {
    let id = ChannelId::new(100);

    let next = id
        .checked_next()
        .expect("100 must have a representable successor");

    assert_eq!(id.value(), 100);
    assert_eq!(next.value(), 101);
}

// =============================================================================
// Large-value / scalability tests
// =============================================================================

#[test]
fn maximum_representable_zqn_identifier_is_a_valid_identifier_value() {
    let id = NoiseModelId::new(ZqnIdValue::MAX);

    assert_eq!(id.value(), ZqnIdValue::MAX);
    assert_eq!(id.to_string(), format!("noise-model:{}", ZqnIdValue::MAX));
}

#[test]
fn large_identifiers_do_not_imply_machine_size_limits() {
    let logical = QubitId::new(usize::MAX);
    let physical = PhysicalQubitId::new(usize::MAX);

    assert_eq!(logical.index(), usize::MAX);
    assert_eq!(physical.index(), usize::MAX);

    // The identifiers are valid values even though no test attempts to
    // allocate a machine/register of that size.
    //
    // This distinction is fundamental:
    //
    // identifier domain != resource allocation.
}

#[test]
fn identifier_values_remain_value_based_at_large_scale() {
    let values = [
        0_u64,
        1_u64,
        7_u64,
        1_000_u64,
        1_000_000_u64,
        1_000_000_000_u64,
        ZqnIdValue::MAX,
    ];

    for value in values {
        let id = NoiseSnapshotId::new(value);

        assert_eq!(id.value(), value);
        assert_eq!(
            NoiseSnapshotId::from(value).value(),
            value
        );
    }
}

// =============================================================================
// Conversion contracts
// =============================================================================

#[test]
fn all_typed_ids_support_lossless_raw_value_conversion() {
    macro_rules! round_trip {
        ($type:ty, $value:expr) => {{
            let original = <$type>::new($value);
            let raw: ZqnIdValue = original.into();
            let reconstructed = <$type>::from(raw);

            assert_eq!(raw, $value);
            assert_eq!(reconstructed.value(), $value);
        }};
    }

    round_trip!(NoiseModelId, 1);
    round_trip!(ChannelId, 2);
    round_trip!(FaultId, 3);
    round_trip!(NoiseApplicationId, 4);
    round_trip!(NoiseSnapshotId, 5);
    round_trip!(CalibrationId, 6);
    round_trip!(CharacterizationId, 7);
    round_trip!(ExperimentId, 8);
    round_trip!(ObservationId, 9);
    round_trip!(NoiseRealizationId, 10);
    round_trip!(CorrelationId, 11);
    round_trip!(NoiseParameterId, 12);
    round_trip!(DistributionId, 13);
    round_trip!(ErrorBudgetId, 14);
    round_trip!(NoiseProfileId, 15);
}

#[test]
fn object_id_conversion_is_lossless_at_boundary_values() {
    let values = [
        ZqnIdValue::MIN,
        1,
        42,
        ZqnIdValue::MAX,
    ];

    for value in values {
        let object = ZqnObjectId::new(value);
        let raw: ZqnIdValue = object.into();
        let reconstructed = ZqnObjectId::from(raw);

        assert_eq!(raw, value);
        assert_eq!(reconstructed.value(), value);
    }
}

// =============================================================================
// Deterministic formatting
// =============================================================================

#[test]
fn typed_id_formatting_is_stable() {
    assert_eq!(
        NoiseModelId::new(1).to_string(),
        "noise-model:1"
    );

    assert_eq!(
        ChannelId::new(2).to_string(),
        "channel:2"
    );

    assert_eq!(
        FaultId::new(3).to_string(),
        "fault:3"
    );

    assert_eq!(
        CalibrationId::new(4).to_string(),
        "calibration:4"
    );

    assert_eq!(
        ObservationId::new(5).to_string(),
        "observation:5"
    );
}

#[test]
fn typed_id_debug_output_is_repeatable() {
    let ids = [
        format!("{:?}", NoiseModelId::new(1)),
        format!("{:?}", ChannelId::new(2)),
        format!("{:?}", FaultId::new(3)),
        format!("{:?}", CalibrationId::new(4)),
    ];

    let repeated = [
        format!("{:?}", NoiseModelId::new(1)),
        format!("{:?}", ChannelId::new(2)),
        format!("{:?}", FaultId::new(3)),
        format!("{:?}", CalibrationId::new(4)),
    ];

    assert_eq!(ids, repeated);
}

// =============================================================================
// Prefix contracts
// =============================================================================

#[test]
fn typed_id_prefixes_are_non_empty() {
    let prefixes = [
        NoiseModelId::prefix(),
        ChannelId::prefix(),
        FaultId::prefix(),
        NoiseApplicationId::prefix(),
        NoiseSnapshotId::prefix(),
        CalibrationId::prefix(),
        CharacterizationId::prefix(),
        ExperimentId::prefix(),
        ObservationId::prefix(),
        NoiseRealizationId::prefix(),
        CorrelationId::prefix(),
        NoiseParameterId::prefix(),
        DistributionId::prefix(),
        ErrorBudgetId::prefix(),
        NoiseProfileId::prefix(),
    ];

    for prefix in prefixes {
        assert!(!prefix.is_empty());
    }
}

#[test]
fn typed_id_prefixes_are_stable_constants() {
    assert_eq!(NoiseModelId::prefix(), "noise-model");
    assert_eq!(ChannelId::prefix(), "channel");
    assert_eq!(FaultId::prefix(), "fault");
    assert_eq!(
        NoiseApplicationId::prefix(),
        "noise-application"
    );
    assert_eq!(
        NoiseSnapshotId::prefix(),
        "noise-snapshot"
    );
    assert_eq!(CalibrationId::prefix(), "calibration");
    assert_eq!(
        CharacterizationId::prefix(),
        "characterization"
    );
    assert_eq!(ExperimentId::prefix(), "experiment");
    assert_eq!(ObservationId::prefix(), "observation");
    assert_eq!(
        NoiseRealizationId::prefix(),
        "noise-realization"
    );
    assert_eq!(CorrelationId::prefix(), "correlation");
    assert_eq!(
        NoiseParameterId::prefix(),
        "noise-parameter"
    );
    assert_eq!(DistributionId::prefix(), "distribution");
    assert_eq!(ErrorBudgetId::prefix(), "error-budget");
    assert_eq!(NoiseProfileId::prefix(), "noise-profile");
}

// =============================================================================
// Ordering contracts
// =============================================================================

#[test]
fn typed_ids_have_deterministic_ordering() {
    assert_ordering(
        NoiseModelId::new(1),
        NoiseModelId::new(2),
    );

    assert_ordering(
        ChannelId::new(10),
        ChannelId::new(20),
    );

    assert_ordering(
        FaultId::new(100),
        FaultId::new(200),
    );
}

#[test]
fn equal_ids_compare_equal_in_ordered_collections() {
    let a = CalibrationId::new(123);
    let b = CalibrationId::new(123);

    assert_eq!(a.cmp(&b), std::cmp::Ordering::Equal);

    let mut set = BTreeSet::new();
    set.insert(a);
    set.insert(b);

    assert_eq!(set.len(), 1);
}

// =============================================================================
// Hash contracts
// =============================================================================

#[test]
fn typed_ids_have_consistent_hash_behavior() {
    assert_hash_equality(NoiseModelId::new(1));
    assert_hash_equality(ChannelId::new(2));
    assert_hash_equality(FaultId::new(3));
    assert_hash_equality(CalibrationId::new(4));
    assert_hash_equality(ObservationId::new(5));
}

#[test]
fn different_values_are_distinguishable_in_hash_sets() {
    let mut set = HashSet::new();

    set.insert(NoiseModelId::new(1));
    set.insert(NoiseModelId::new(2));
    set.insert(NoiseModelId::new(3));

    assert_eq!(set.len(), 3);
}

// =============================================================================
// Identity-kind vocabulary
// =============================================================================

#[test]
fn generic_object_kind_exists() {
    assert_eq!(ZqnIdKind::Object, ZqnIdKind::Object);
}

#[test]
fn core_zqn_id_kinds_are_distinct() {
    assert_ne!(ZqnIdKind::NoiseModel, ZqnIdKind::Channel);
    assert_ne!(ZqnIdKind::Channel, ZqnIdKind::Fault);
    assert_ne!(
        ZqnIdKind::NoiseModel,
        ZqnIdKind::Fault
    );
}

#[test]
fn id_kind_values_are_orderable() {
    let mut kinds = BTreeSet::new();

    kinds.insert(ZqnIdKind::Object);
    kinds.insert(ZqnIdKind::NoiseModel);
    kinds.insert(ZqnIdKind::Channel);
    kinds.insert(ZqnIdKind::Fault);

    assert_eq!(kinds.len(), 4);
}

// =============================================================================
// Send / Sync contracts
// =============================================================================

fn assert_send_sync<T>()
where
    T: Send + Sync,
{
}

#[test]
fn zqn_identifier_types_are_send_and_sync() {
    assert_send_sync::<ZqnObjectId>();

    assert_send_sync::<NoiseModelId>();
    assert_send_sync::<ChannelId>();
    assert_send_sync::<FaultId>();
    assert_send_sync::<NoiseApplicationId>();
    assert_send_sync::<NoiseSnapshotId>();
    assert_send_sync::<CalibrationId>();
    assert_send_sync::<CharacterizationId>();
    assert_send_sync::<ExperimentId>();
    assert_send_sync::<ObservationId>();
    assert_send_sync::<NoiseRealizationId>();
    assert_send_sync::<CorrelationId>();
    assert_send_sync::<NoiseParameterId>();
    assert_send_sync::<DistributionId>();
    assert_send_sync::<ErrorBudgetId>();
    assert_send_sync::<NoiseProfileId>();
}

#[test]
fn canonical_qubit_ids_are_send_and_sync() {
    assert_send_sync::<QubitId>();
    assert_send_sync::<PhysicalQubitId>();
}

// =============================================================================
// No hidden allocation / registry semantics
// =============================================================================

#[test]
fn constructing_identifiers_is_pure_value_construction() {
    let a = NoiseModelId::new(1);
    let b = NoiseModelId::new(1);

    assert_eq!(a, b);

    // Constructing the same value twice does not imply that two global objects
    // were registered. The identity type is deliberately only a value.
    //
    // This test therefore checks equality rather than expecting allocation or
    // registry side effects.
    assert_eq!(a.value(), b.value());
}

#[test]
fn identifiers_do_not_depend_on_creation_order() {
    let first = ChannelId::new(100);
    let second = ChannelId::new(50);

    assert_eq!(first.value(), 100);
    assert_eq!(second.value(), 50);

    let reconstructed_first = ChannelId::new(100);

    assert_eq!(first, reconstructed_first);
}

// =============================================================================
// Independence from hardware size
// =============================================================================

#[test]
fn identifiers_do_not_encode_vendor_information() {
    let id = NoiseModelId::new(1);

    let text = id.to_string();

    assert!(!text.contains("ibm"));
    assert!(!text.contains("ionq"));
    assert!(!text.contains("rigetti"));
    assert!(!text.contains("quantinuum"));
    assert!(!text.contains("google"));
    assert!(!text.contains("aws"));
}

#[test]
fn identifiers_do_not_encode_a_fixed_machine_size() {
    let values = [
        1_u64,
        2_u64,
        16_u64,
        32_u64,
        64_u64,
        128_u64,
        1024_u64,
        1_000_000_u64,
        ZqnIdValue::MAX,
    ];

    for value in values {
        let id = NoiseModelId::new(value);

        assert_eq!(id.value(), value);
    }

    // The test intentionally never interprets any value as a maximum number
    // of qubits. An identifier is not a capacity.
}

// =============================================================================
// Immutability / Copy semantics
// =============================================================================

#[test]
fn identifier_copy_semantics_preserve_value() {
    let original = NoiseSnapshotId::new(777);
    let copied = original;

    assert_eq!(original, copied);
    assert_eq!(original.value(), copied.value());
}

#[test]
fn identifier_copy_does_not_consume_semantic_identity() {
    let original = CalibrationId::new(888);

    let first = original;
    let second = original;

    assert_eq!(first, second);
    assert_eq!(first.value(), 888);
    assert_eq!(second.value(), 888);
}

// =============================================================================
// Boundary-value ordering
// =============================================================================

#[test]
fn boundary_values_have_correct_ordering() {
    let minimum = NoiseModelId::new(ZqnIdValue::MIN);
    let maximum = NoiseModelId::new(ZqnIdValue::MAX);

    assert!(minimum < maximum);
    assert!(maximum > minimum);
    assert_ne!(minimum, maximum);
}

#[test]
fn adjacent_boundary_values_are_distinct() {
    let maximum = NoiseModelId::new(ZqnIdValue::MAX);
    let predecessor = NoiseModelId::new(ZqnIdValue::MAX - 1);

    assert_ne!(predecessor, maximum);
    assert!(predecessor < maximum);

    assert_eq!(
        predecessor
            .checked_next()
            .expect("MAX - 1 must have MAX as its successor"),
        maximum
    );

    assert_eq!(maximum.checked_next(), None);
}

// =============================================================================
// Reproducibility-oriented identity behavior
// =============================================================================

#[test]
fn same_identity_value_produces_same_text_across_repeated_construction() {
    for value in [
        0_u64,
        1_u64,
        42_u64,
        1_000_000_u64,
        ZqnIdValue::MAX,
    ] {
        let first = NoiseRealizationId::new(value);
        let second = NoiseRealizationId::new(value);

        assert_eq!(first, second);
        assert_eq!(first.to_string(), second.to_string());
        assert_eq!(
            format!("{first:?}"),
            format!("{second:?}")
        );
    }
}

// =============================================================================
// Collection scalability
// =============================================================================

#[test]
fn ordered_identity_collections_preserve_deterministic_order() {
    let ids = [
        NoiseModelId::new(9),
        NoiseModelId::new(1),
        NoiseModelId::new(7),
        NoiseModelId::new(3),
        NoiseModelId::new(5),
    ];

    let ordered: BTreeSet<_> = ids.into_iter().collect();

    let values: Vec<_> =
        ordered.into_iter().map(NoiseModelId::value).collect();

    assert_eq!(values, vec![1, 3, 5, 7, 9]);
}

#[test]
fn identity_values_can_be_used_in_hash_collections_without_global_state() {
    let mut map = HashMap::new();

    for value in 0_u64..=8_u64 {
        map.insert(ObservationId::new(value), value);
    }

    assert_eq!(map.len(), 9);

    for value in 0_u64..=8_u64 {
        assert_eq!(
            map.get(&ObservationId::new(value)),
            Some(&value)
        );
    }
}

// =============================================================================
// API surface / module-boundary tests
// =============================================================================

#[test]
fn canonical_zqn_identity_types_are_constructible_through_the_public_api() {
    let _ = ZqnObjectId::new(0);

    let _ = NoiseModelId::new(0);
    let _ = ChannelId::new(0);
    let _ = FaultId::new(0);
    let _ = NoiseApplicationId::new(0);
    let _ = NoiseSnapshotId::new(0);
    let _ = CalibrationId::new(0);
    let _ = CharacterizationId::new(0);
    let _ = ExperimentId::new(0);
    let _ = ObservationId::new(0);
    let _ = NoiseRealizationId::new(0);
    let _ = CorrelationId::new(0);
    let _ = NoiseParameterId::new(0);
    let _ = DistributionId::new(0);
    let _ = ErrorBudgetId::new(0);
    let _ = NoiseProfileId::new(0);
}

#[test]
fn canonical_ir_qubit_api_is_available_to_zqn_consumers() {
    let logical = crate::quantum::ir::qubit::QubitId::new(0);
    let physical =
        crate::quantum::ir::qubit::PhysicalQubitId::new(0);

    assert_eq!(logical.index(), 0);
    assert_eq!(physical.index(), 0);
}

// =============================================================================
// Architecture invariants
// =============================================================================

#[test]
fn zqn_identity_values_are_not_resource_counts() {
    let id = NoiseModelId::new(128);

    // This assertion intentionally tests only identity semantics.
    assert_eq!(id.value(), 128);

    // There is deliberately no API such as:
    //
    //     id.qubit_count()
    //
    // because a ZQN identity must not silently acquire machine-capacity
    // semantics.
}

#[test]
fn zqn_identity_namespace_is_separate_from_canonical_qubit_namespace() {
    let zqn = NoiseModelId::new(7);
    let logical = QubitId::new(7);

    assert_eq!(zqn.value() as usize, logical.index());

    // Numeric equality is irrelevant to semantic type identity.
    //
    // `NoiseModelId` identifies a ZQN object.
    // `QubitId` identifies a logical quantum resource.
    //
    // The Rust type system prevents them from being accidentally substituted.
}

#[test]
fn no_test_depends_on_a_fixed_number_of_qubits() {
    // This is intentionally a structural regression test.
    //
    // If the ZQN identity API later introduces a machine-size constant, this
    // test should NOT be changed to accommodate it. The correct response is to
    // keep machine-size policy outside the identity semantics.
    //
    // The test therefore succeeds simply by exercising an arbitrarily chosen
    // identity value.
    let id = NoiseProfileId::new(4096);

    assert_eq!(id.value(), 4096);
}

// =============================================================================
// Regression tests for known architectural failure modes
// =============================================================================

#[test]
fn regression_no_wrapping_identifier_increment() {
    let maximum = ObservationId::new(ZqnIdValue::MAX);

    assert_eq!(maximum.checked_next(), None);
}

#[test]
fn regression_no_silent_identifier_normalization() {
    let id = FaultId::new(ZqnIdValue::MAX);

    // The value must survive exactly as supplied.
    assert_eq!(id.value(), ZqnIdValue::MAX);
}

#[test]
fn regression_no_hidden_global_identifier_counter_required() {
    let first = ExperimentId::new(0);
    let second = ExperimentId::new(0);

    assert_eq!(first, second);

    // If identity creation ever begins requiring a hidden global allocator,
    // this foundational contract has changed and the owning ID API must be
    // reconsidered explicitly rather than silently changing these tests.
}

#[test]
fn regression_identity_serialization_text_is_not_address_based() {
    let first = CharacterizationId::new(42);
    let second = CharacterizationId::new(42);

    assert_eq!(first.to_string(), second.to_string());
    assert!(!first.to_string().contains("0x"));
}

#[test]
fn regression_identity_debug_is_not_process_address_based() {
    let first = DistributionId::new(42);
    let second = DistributionId::new(42);

    assert_eq!(
        format!("{first:?}"),
        format!("{second:?}")
    );
}

// =============================================================================
// End-to-end foundational identity contract
// =============================================================================

#[test]
fn complete_foundational_identity_contract() {
    let raw = ZqnIdValue::MAX - 1;

    let model = NoiseModelId::new(raw);
    let reconstructed = NoiseModelId::from(model.value());

    assert_eq!(model, reconstructed);
    assert_eq!(model.value(), raw);
    assert_eq!(
        model.to_string(),
        format!("noise-model:{raw}")
    );

    let next = model
        .checked_next()
        .expect("MAX - 1 must have a representable successor");

    assert_eq!(next.value(), ZqnIdValue::MAX);
    assert_eq!(next.checked_next(), None);

    let logical = QubitId::new(usize::MAX);
    let physical = PhysicalQubitId::new(usize::MAX);

    assert_eq!(logical.index(), usize::MAX);
    assert_eq!(physical.index(), usize::MAX);
}