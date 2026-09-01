//! Zamani Quantum Noise (ZQN) — Calibration Validation.
//!
//! This module is the validation boundary for calibration artifacts.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - policy-driven validation orchestration for calibration artifacts;
//! - validation of one or many `CalibrationSnapshot` values;
//! - cross-snapshot identity and lineage consistency checks;
//! - deterministic ordering/invariant checks that are not owned by a single
//!   calibration object;
//! - explicit resource and collection limits;
//! - a small validation-participant trait for future calibration modules;
//! - deterministic validation reports.
//!
//! # Does not own
//!
//! This file does not own:
//!
//! - calibration parameter mathematics (`parameter.rs`);
//! - gate calibration semantics (`gate.rs`);
//! - readout/measurement calibration semantics (`readout.rs`, `measurement.rs`);
//! - calibration drift laws (`drift.rs`);
//! - interpolation algorithms (`interpolation.rs`);
//! - calibration snapshot storage/identity (`snapshot.rs`);
//! - hardware existence or target capability validation;
//! - statistical characterization;
//! - noise-channel validation;
//! - serialization formats;
//! - hashing/signatures;
//! - registries or global mutable state.
//!
//! The rule is deliberately:
//!
//! ```text
//! artifact-owned invariants
//!          │
//!          ▼
//! artifact::validate()
//!          │
//!          ▼
//! calibration::validation
//!          │
//!          ├── cross-artifact invariants
//!          ├── temporal consistency
//!          ├── identity/lineage consistency
//!          └── caller resource policy
//! ```
//!
//! # Why a separate validation layer exists
//!
//! `snapshot.rs` already performs local structural validation. A central
//! validation layer is still required because some invariants cannot be
//! decided by one object in isolation. Examples are duplicate snapshot IDs,
//! resolvable lineage, cross-snapshot temporal overlap under a selected policy,
//! and aggregate resource budgets.
//!
//! Validation is therefore intentionally split into:
//!
//! 1. local validation owned by the artifact;
//! 2. cross-artifact validation owned here;
//! 3. external/environment validation owned by target/hardware layers.
//!
//! # Canonical quantum resource identity
//!
//! This file does not define a `QubitId` or `PhysicalQubitId`.
//!
//! Resource identity remains owned by:
//!
//! `crate::quantum::ir::qubit::{QubitId, PhysicalQubitId}`
//!
//! Snapshot validation delegates resource identity semantics to
//! `CalibrationResource` from `snapshot.rs`.
//!
//! # Write once, scale everywhere
//!
//! There is no semantic machine-size limit here.
//!
//! A caller can validate a single calibration or a collection containing as
//! many snapshots/resources as its available memory and explicit policy allow.
//! `None` in a limit means that this validator imposes no limit for that
//! dimension.
//!
//! The optional conservative policy is explicitly a caller-selected safety
//! policy; it is not a ZQN machine-size ceiling.
//!
//! # Determinism
//!
//! Validation is deterministic.
//!
//! It does not:
//!
//! - read the wall clock;
//! - use randomness;
//! - access global state;
//! - access a network or filesystem;
//! - depend on hash-map iteration order;
//! - mutate the supplied artifacts.
//!
//! Cross-snapshot checks use ordered collections and deterministic sorting.
//!
//! # Numerical safety
//!
//! This module does not reinterpret calibration values. Numerical validation
//! belongs to the owning artifact. When a future calibration artifact exposes
//! numerical validation through `CalibrationValidationParticipant`, this layer
//! invokes that contract rather than duplicating numerical rules.
//!
//! # Temporal semantics
//!
//! Snapshot validity is the explicit half-open interval defined by
//! `CalibrationValidity`:
//!
//! `[valid_from, valid_until)`.
//!
//! This module never invents a current time. A snapshot can therefore be
//! validated offline, in simulation, during replay, or in a distributed system
//! without changing its result.
//!
//! # Validation versus scientific truth
//!
//! Structural validity does not prove that a calibration is physically
//! accurate. A value can be well-formed and still be scientifically wrong.
//! Independent characterization/benchmarking is responsible for assessing
//! accuracy, uncertainty, drift, and fitness for a particular workload.
//!
//! This separation is important for reproducibility: calibration, execution
//! context, and validation evidence must remain distinguishable.
//!
//! # Security and resource exhaustion
//!
//! Validation may be invoked on untrusted calibration data. Every aggregate
//! operation therefore accepts explicit limits. The validator never allocates
//! a structure proportional to a declared maximum; it allocates only for the
//! actual input it receives.
//!
//! Callers handling hostile input should use a bounded policy before parsing or
//! materializing untrusted collections. Validation cannot protect against an
//! allocation that already occurred before the validator was called.
//!
//! # Serialization
//!
//! No serialization is implemented here. `zqn::io` owns wire formats.
//!
//! The validation contract is representation-independent and must remain valid
//! after serialization/deserialization. Deserializers should invoke this layer
//! before accepting a calibration bundle for execution.
//!
//! # Integration
//!
//! ```text
//! calibration::snapshot
//!       │
//!       ├── local validation
//!       ▼
//! calibration::validation
//!       │
//!       ├── parameter / gate / readout participant validation
//!       ├── lineage consistency
//!       ├── cross-snapshot consistency
//!       └── resource policy
//!       │
//!       ├───────────────┬────────────────┬─────────────────┐
//!       ▼               ▼                ▼                 ▼
//!      noise           QEC           routing          scheduling
//!       │               │                │                 │
//!       └───────────────┴────────────────┴─────────────────┘
//!                               │
//!                               ▼
//!                           execution
//! ```
//!
//! Hardware/target validation remains outside this file. A physical resource
//! ID is an identifier, not proof that the resource exists or is available.
//!
//! # External architecture considerations
//!
//! Production quantum systems commonly treat calibration as time-varying state
//! and bind execution/benchmark results to the calibration state that produced
//! them. IBM's current documentation describes dynamic backend properties and
//! repeated calibration/monitoring, while recent reproducibility work likewise
//! emphasizes retaining calibration snapshots and execution context. This
//! validator therefore treats temporal validity and explicit snapshot identity
//! as first-class data rather than as an implicit "latest calibration" lookup.
//!
//! # Rust compatibility
//!
//! - Rust 1.97 / 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust only;
//! - no nightly features;
//! - no `unsafe`.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};

use crate::quantum::zqn::calibration::parameter::CalibrationParameter;
use crate::quantum::zqn::calibration::snapshot::{
    CalibrationLineage,
    CalibrationResource,
    CalibrationSnapshot,
    CalibrationSnapshotStatus,
    SnapshotValidationLimits,
};
use crate::quantum::zqn::core::errors::{
    ZqnError,
    ZqnErrorCode,
    ZqnErrorKind,
    ZqnResult,
};
use crate::quantum::zqn::core::ids::{
    CalibrationId,
    ZqnObjectId,
};

// ============================================================================
// Schema
// ============================================================================

/// Semantic revision of the calibration-validation contract.
///
/// This is a representation/behavior revision, not a machine-size limit.
pub const CALIBRATION_VALIDATION_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Policy
// ============================================================================

/// Policy for handling temporal overlap between snapshots that describe the
/// same target, schema and exact resource scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemporalOverlapPolicy {
    /// Overlap is allowed.
    ///
    /// This is the least opinionated policy and is the default because staged
    /// calibration, concurrent revisions, and future selection policies may
    /// legitimately require overlapping records.
    Allow,

    /// Reject overlapping intervals when target, calibration schema and exact
    /// resource scope are identical and both snapshots are `Valid`.
    ///
    /// This is useful for stores that require one active calibration revision
    /// for an exact scope at any instant.
    RejectEquivalentActiveOverlap,
}

impl Default for TemporalOverlapPolicy {
    fn default() -> Self {
        Self::Allow
    }
}

/// Caller-selected policy controlling cross-artifact validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalibrationValidationPolicy {
    /// Validation limits for each individual snapshot.
    pub snapshot_limits: SnapshotValidationLimits,

    /// Aggregate limit on the number of snapshots supplied to one validation
    /// call.
    ///
    /// `None` means unlimited by this layer.
    pub max_snapshots: Option<u64>,

    /// Aggregate limit on scoped resources across all supplied snapshots.
    pub max_total_resources: Option<u64>,

    /// Aggregate limit on calibration-object references across all snapshots.
    pub max_total_calibration_objects: Option<u64>,

    /// Aggregate limit on lineage relationships.
    pub max_total_lineage_references: Option<u64>,

    /// Whether duplicate snapshot IDs are rejected.
    pub reject_duplicate_snapshot_ids: bool,

    /// Whether duplicate calibration-object references inside one snapshot
    /// are rejected.
    ///
    /// Cross-snapshot reuse remains valid.
    pub reject_duplicate_object_references: bool,

    /// Whether every lineage reference must resolve to another snapshot in the
    /// supplied validation set.
    pub require_resolvable_lineage: bool,

    /// Whether equivalent active snapshots may overlap in time.
    pub temporal_overlap: TemporalOverlapPolicy,

    /// Whether snapshots must have `Valid` lifecycle status.
    ///
    /// This does not check validity at a particular time; a time is required
    /// for that operation and is supplied separately by `validate_at`.
    pub require_usable_status: bool,
}

impl CalibrationValidationPolicy {
    /// No artificial limits and no opinionated lifecycle/overlap restrictions.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            snapshot_limits: SnapshotValidationLimits::unlimited(),
            max_snapshots: None,
            max_total_resources: None,
            max_total_calibration_objects: None,
            max_total_lineage_references: None,
            reject_duplicate_snapshot_ids: true,
            reject_duplicate_object_references: true,
            require_resolvable_lineage: false,
            temporal_overlap: TemporalOverlapPolicy::Allow,
            require_usable_status: false,
        }
    }

    /// Explicit conservative policy suitable for an untrusted ingestion
    /// boundary.
    ///
    /// These numbers are safety-policy defaults only. They do not constrain the
    /// ZQN semantic model and can be replaced by the caller.
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            snapshot_limits: SnapshotValidationLimits::conservative(),
            max_snapshots: Some(1_000_000),
            max_total_resources: Some(10_000_000),
            max_total_calibration_objects: Some(10_000_000),
            max_total_lineage_references: Some(1_000_000),
            reject_duplicate_snapshot_ids: true,
            reject_duplicate_object_references: true,
            require_resolvable_lineage: false,
            temporal_overlap: TemporalOverlapPolicy::Allow,
            require_usable_status: false,
        }
    }
}

impl Default for CalibrationValidationPolicy {
    fn default() -> Self {
        Self::conservative()
    }
}

// ============================================================================
// Validation participant contract
// ============================================================================

/// Contract implemented by calibration artifacts that own their own local
/// validation rules.
///
/// The central validator deliberately does not know the internal structure of
/// gate, readout, measurement, drift, or future calibration types. Each owning
/// module can implement this trait once and then receive the same validation
/// orchestration without modifying this file.
///
/// `object_id` is an opaque ZQN identity. It is not an authorization token and
/// does not imply that the referenced hardware resource exists.
pub trait CalibrationValidationParticipant {
    /// Returns the stable ZQN object identity of this artifact.
    fn object_id(&self) -> ZqnObjectId;

    /// Validates all invariants owned by the artifact.
    fn validate_calibration(&self) -> ZqnResult<()>;
}

impl CalibrationValidationParticipant for CalibrationParameter {
    fn object_id(&self) -> ZqnObjectId {
        self.id()
    }

    fn validate_calibration(&self) -> ZqnResult<()> {
        self.validate()
    }
}

// ============================================================================
// Report
// ============================================================================

/// Deterministic summary returned after successful collection validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CalibrationValidationReport {
    /// Number of validated snapshots.
    pub snapshots: u64,

    /// Number of explicitly scoped resources across all snapshots.
    pub resources: u64,

    /// Number of calibration-object references across all snapshots.
    pub calibration_objects: u64,

    /// Number of lineage relationships encountered.
    pub lineage_references: u64,
}

// ============================================================================
// Validator
// ============================================================================

/// Stateless calibration validator configured by an explicit policy.
#[derive(Debug, Clone, Copy)]
pub struct CalibrationValidator {
    policy: CalibrationValidationPolicy,
}

impl CalibrationValidator {
    /// Creates a validator with the supplied policy.
    #[must_use]
    pub const fn new(policy: CalibrationValidationPolicy) -> Self {
        Self { policy }
    }

    /// Returns an unrestricted validator.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self::new(CalibrationValidationPolicy::unlimited())
    }

    /// Returns a conservative untrusted-input validator.
    #[must_use]
    pub const fn conservative() -> Self {
        Self::new(CalibrationValidationPolicy::conservative())
    }

    /// Returns the policy used by this validator.
    #[must_use]
    pub const fn policy(&self) -> CalibrationValidationPolicy {
        self.policy
    }

    /// Validates one snapshot without requiring it to resolve lineage against
    /// an external collection.
    pub fn validate_snapshot(
        &self,
        snapshot: &CalibrationSnapshot,
    ) -> ZqnResult<()> {
        snapshot
            .validate(self.policy.snapshot_limits)
            .map_err(snapshot_error)?;

        validate_snapshot_ordering(snapshot)?;

        validate_object_references(
            snapshot,
            self.policy.reject_duplicate_object_references,
        )?;

        if self.policy.require_usable_status
            && !snapshot.status().is_usable()
        {
            return Err(invalid_calibration(format!(
                "calibration snapshot {:?} has status {:?}; \
                 a usable snapshot is required",
                snapshot.id(),
                snapshot.status(),
            )));
        }

        Ok(())
    }

    /// Validates one snapshot at an explicit time.
    ///
    /// This is the correct API when a caller needs to know whether a snapshot
    /// is usable for a particular execution instant.
    ///
    /// It never reads the current wall clock.
    pub fn validate_at(
        &self,
        snapshot: &CalibrationSnapshot,
        time: crate::quantum::zqn::calibration::snapshot::CalibrationTime,
    ) -> ZqnResult<()> {
        self.validate_snapshot(snapshot)?;

        if !snapshot.is_temporally_valid_at(time) {
            return Err(invalid_calibration(format!(
                "calibration snapshot {:?} is outside its explicit \
                 validity interval at {}",
                snapshot.id(),
                time,
            )));
        }

        if self.policy.require_usable_status
            && !snapshot.is_usable_at(time)
        {
            return Err(invalid_calibration(format!(
                "calibration snapshot {:?} is not usable at explicit time {}",
                snapshot.id(),
                time,
            )));
        }

        Ok(())
    }

    /// Validates a collection of snapshots with deterministic cross-object
    /// checks.
    ///
    /// The supplied slice is never mutated or reordered.
    pub fn validate_snapshots(
        &self,
        snapshots: &[CalibrationSnapshot],
    ) -> ZqnResult<CalibrationValidationReport> {
        self.check_count_limit(
            "snapshots",
            snapshots.len() as u64,
            self.policy.max_snapshots,
        )?;

        let mut report = CalibrationValidationReport::default();

        let mut ids = BTreeSet::new();

        let mut lineage =
            BTreeMap::<CalibrationId, CalibrationId>::new();

        for snapshot in snapshots {
            self.validate_snapshot(snapshot)?;

            if self.policy.reject_duplicate_snapshot_ids
                && !ids.insert(snapshot.id())
            {
                return Err(ZqnError::new(
                    ZqnErrorKind::Calibration,
                    ZqnErrorCode::DuplicateIdentifier,
                    format!(
                        "duplicate calibration snapshot identity: {:?}",
                        snapshot.id(),
                    ),
                ));
            }

            report.snapshots = checked_add(
                report.snapshots,
                1,
                "validated snapshot count",
            )?;

            report.resources = checked_add(
                report.resources,
                snapshot.resource_count() as u64,
                "total calibration resource count",
            )?;

            self.check_count_limit(
                "total calibration resources",
                report.resources,
                self.policy.max_total_resources,
            )?;

            report.calibration_objects = checked_add(
                report.calibration_objects,
                snapshot.calibration_object_count() as u64,
                "total calibration-object reference count",
            )?;

            self.check_count_limit(
                "total calibration-object references",
                report.calibration_objects,
                self.policy.max_total_calibration_objects,
            )?;

            if let Some(relation) = snapshot.lineage() {
                report.lineage_references = checked_add(
                    report.lineage_references,
                    1,
                    "calibration lineage reference count",
                )?;

                self.check_count_limit(
                    "total calibration lineage references",
                    report.lineage_references,
                    self.policy.max_total_lineage_references,
                )?;

                if let Some(parent) = lineage_parent(relation) {
                    lineage.insert(snapshot.id(), parent);
                }
            }
        }

        if self.policy.require_resolvable_lineage {
            validate_lineage_resolution(&ids, &lineage)?;
        }

        if matches!(
            self.policy.temporal_overlap,
            TemporalOverlapPolicy::RejectEquivalentActiveOverlap
        ) {
            validate_equivalent_active_overlap(snapshots)?;
        }

        Ok(report)
    }

    /// Validates all artifacts supplied through the common participant trait.
    ///
    /// This function deliberately does not impose an artifact hierarchy. A
    /// future calibration module can participate without changing this file.
    pub fn validate_participants(
        &self,
        participants: &[&dyn CalibrationValidationParticipant],
    ) -> ZqnResult<()> {
        let mut ids = BTreeSet::new();

        for participant in participants {
            participant.validate_calibration()?;

            let id = participant.object_id();

            if !ids.insert(id) {
                return Err(ZqnError::new(
                    ZqnErrorKind::Calibration,
                    ZqnErrorCode::DuplicateIdentifier,
                    format!(
                        "duplicate calibration artifact identity: {:?}",
                        id
                    ),
                ));
            }
        }

        Ok(())
    }

    fn check_count_limit(
        &self,
        label: &'static str,
        actual: u64,
        maximum: Option<u64>,
    ) -> ZqnResult<()> {
        if let Some(maximum) = maximum {
            if actual > maximum {
                return Err(ZqnError::new(
                    ZqnErrorKind::Limits,
                    ZqnErrorCode::LimitExceeded,
                    format!(
                        "calibration validation {label} count {actual} \
                         exceeds configured limit {maximum}"
                    ),
                ));
            }
        }

        Ok(())
    }
}

impl Default for CalibrationValidator {
    fn default() -> Self {
        Self::conservative()
    }
}

// ============================================================================
// Local validation helpers
// ============================================================================

fn snapshot_error(
    error: impl std::fmt::Display,
) -> ZqnError {
    invalid_calibration(error.to_string())
}

fn invalid_calibration(
    message: impl Into<String>,
) -> ZqnError {
    ZqnError::new(
        ZqnErrorKind::Calibration,
        ZqnErrorCode::InvalidCalibration,
        message.into(),
    )
}

fn validate_snapshot_ordering(
    snapshot: &CalibrationSnapshot,
) -> ZqnResult<()> {
    if snapshot
        .resources()
        .windows(2)
        .any(|window| window[0] > window[1])
    {
        return Err(invalid_calibration(format!(
            "calibration snapshot {:?} resource scope is not in \
             canonical order",
            snapshot.id(),
        )));
    }

    if snapshot
        .calibration_objects()
        .windows(2)
        .any(|window| window[0] > window[1])
    {
        return Err(invalid_calibration(format!(
            "calibration snapshot {:?} calibration-object references \
             are not in canonical order",
            snapshot.id(),
        )));
    }

    Ok(())
}

fn validate_object_references(
    snapshot: &CalibrationSnapshot,
    reject_duplicates: bool,
) -> ZqnResult<()> {
    if !reject_duplicates {
        return Ok(());
    }

    for window in snapshot.calibration_objects().windows(2) {
        if window[0] == window[1] {
            return Err(ZqnError::new(
                ZqnErrorKind::Calibration,
                ZqnErrorCode::DuplicateIdentifier,
                format!(
                    "calibration snapshot {:?} contains duplicate \
                     calibration-object reference {:?}",
                    snapshot.id(),
                    window[0],
                ),
            ));
        }
    }

    Ok(())
}

fn lineage_parent(
    lineage: CalibrationLineage,
) -> Option<CalibrationId> {
    Some(match lineage {
        CalibrationLineage::Supersedes(id)
        | CalibrationLineage::DerivedFrom(id)
        | CalibrationLineage::Replaces(id) => id,
    })
}

fn validate_lineage_resolution(
    ids: &BTreeSet<CalibrationId>,
    lineage: &BTreeMap<CalibrationId, CalibrationId>,
) -> ZqnResult<()> {
    for (child, parent) in lineage {
        if !ids.contains(parent) {
            return Err(ZqnError::new(
                ZqnErrorKind::Calibration,
                ZqnErrorCode::UnknownResource,
                format!(
                    "calibration snapshot {:?} has unresolved \
                     lineage parent {:?}",
                    child,
                    parent,
                ),
            ));
        }
    }

    // Each snapshot has at most one parent, so a cycle can be detected
    // without recursive traversal.
    //
    // The visited set also guarantees termination for malformed input.
    for start in lineage.keys() {
        let mut visited = BTreeSet::new();
        let mut current = *start;

        while let Some(parent) = lineage.get(&current) {
            if !visited.insert(current) {
                return Err(invalid_calibration(format!(
                    "calibration snapshot lineage contains a cycle \
                     involving {:?}",
                    current,
                )));
            }

            current = *parent;
        }
    }

    Ok(())
}

fn validate_equivalent_active_overlap(
    snapshots: &[CalibrationSnapshot],
) -> ZqnResult<()> {
    // Sort indices rather than the caller's data.
    //
    // This makes the operation deterministic without mutating the caller's
    // collection and avoids an O(n²) all-pairs comparison.
    let mut indices: Vec<usize> =
        (0..snapshots.len()).collect();

    indices.sort_unstable_by(|&left, &right| {
        let a = &snapshots[left];
        let b = &snapshots[right];

        a.target_id()
            .cmp(b.target_id())
            .then_with(|| {
                a.calibration_schema()
                    .cmp(b.calibration_schema())
            })
            .then_with(|| {
                a.resources().cmp(b.resources())
            })
            .then_with(|| {
                a.validity()
                    .valid_from()
                    .cmp(&b.validity().valid_from())
            })
            .then_with(|| a.id().cmp(&b.id()))
    });

    for pair in indices.windows(2) {
        let first = &snapshots[pair[0]];
        let second = &snapshots[pair[1]];

        if first.status() != CalibrationSnapshotStatus::Valid
            || second.status() != CalibrationSnapshotStatus::Valid
        {
            continue;
        }

        if first.target_id() != second.target_id()
            || first.calibration_schema()
                != second.calibration_schema()
            || first.resources() != second.resources()
        {
            continue;
        }

        if first.validity().overlaps(second.validity()) {
            return Err(ZqnError::new(
                ZqnErrorKind::Calibration,
                ZqnErrorCode::InvalidCalibration,
                format!(
                    "equivalent active calibration snapshots {:?} \
                     and {:?} have overlapping validity intervals",
                    first.id(),
                    second.id(),
                ),
            ));
        }
    }

    Ok(())
}

fn checked_add(
    current: u64,
    increment: u64,
    label: &'static str,
) -> ZqnResult<u64> {
    current.checked_add(increment).ok_or_else(|| {
        ZqnError::new(
            ZqnErrorKind::Limits,
            ZqnErrorCode::ResourceOverflow,
            format!("overflow while accumulating {label}"),
        )
    })
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::zqn::calibration::snapshot::{
        CalibrationSnapshot,
        CalibrationSnapshotStatus,
        CalibrationTime,
        CalibrationValidity,
    };

    fn calibration_id(value: u128) -> CalibrationId {
        ZqnObjectId::new("calibration", value)
    }

    fn snapshot(
        id: u128,
        from: i64,
        until: Option<i64>,
    ) -> CalibrationSnapshot {
        let start = CalibrationTime::from_seconds(from);

        let validity = match until {
            Some(end) => CalibrationValidity::finite(
                start,
                CalibrationTime::from_seconds(end),
            )
            .expect("test interval must be valid"),

            None => {
                CalibrationValidity::open_ended(start)
            }
        };

        CalibrationSnapshot::new(
            calibration_id(id),
            "test-target",
            "zqn.calibration.test",
            None,
            validity,
            CalibrationSnapshotStatus::Valid,
            Vec::<CalibrationResource>::new(),
            Vec::new(),
            None,
        )
        .expect("test snapshot must be structurally valid")
    }

    #[test]
    fn unlimited_policy_has_no_aggregate_limits() {
        let policy =
            CalibrationValidationPolicy::unlimited();

        assert!(policy.max_snapshots.is_none());
        assert!(policy.max_total_resources.is_none());
        assert!(
            policy.max_total_calibration_objects.is_none()
        );
        assert!(
            policy.max_total_lineage_references.is_none()
        );
    }

    #[test]
    fn one_snapshot_validates() {
        let validator =
            CalibrationValidator::unlimited();

        let value =
            snapshot(1, 0, Some(10));

        assert!(
            validator.validate_snapshot(&value).is_ok()
        );
    }

    #[test]
    fn collection_validation_counts_snapshots() {
        let validator =
            CalibrationValidator::unlimited();

        let values = vec![
            snapshot(1, 0, Some(10)),
            snapshot(2, 10, Some(20)),
        ];

        let report = validator
            .validate_snapshots(&values)
            .expect("collection should validate");

        assert_eq!(report.snapshots, 2);
        assert_eq!(report.resources, 0);
        assert_eq!(report.calibration_objects, 0);
        assert_eq!(report.lineage_references, 0);
    }

    #[test]
    fn duplicate_snapshot_ids_are_rejected() {
        let validator =
            CalibrationValidator::unlimited();

        let values = vec![
            snapshot(1, 0, Some(10)),
            snapshot(1, 10, Some(20)),
        ];

        assert!(
            validator.validate_snapshots(&values).is_err()
        );
    }

    #[test]
    fn explicit_time_validation_is_deterministic() {
        let validator =
            CalibrationValidator::unlimited();

        let value =
            snapshot(1, 0, Some(10));

        assert!(
            validator
                .validate_at(
                    &value,
                    CalibrationTime::from_seconds(0),
                )
                .is_ok()
        );

        assert!(
            validator
                .validate_at(
                    &value,
                    CalibrationTime::from_seconds(10),
                )
                .is_err()
        );
    }

    #[test]
    fn equivalent_active_overlap_can_be_rejected_by_policy() {
        let mut policy =
            CalibrationValidationPolicy::unlimited();

        policy.temporal_overlap =
            TemporalOverlapPolicy::RejectEquivalentActiveOverlap;

        let validator =
            CalibrationValidator::new(policy);

        let values = vec![
            snapshot(1, 0, Some(10)),
            snapshot(2, 5, Some(15)),
        ];

        assert!(
            validator.validate_snapshots(&values).is_err()
        );
    }
}