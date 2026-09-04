//! Zamani Quantum Scheduling — Resource Verification
//!
//! This module verifies the resource correctness of a completed or partially
//! constructed quantum schedule.
//!
//! # Responsibility
//!
//! This file answers:
//!
//! > "Does this schedule use only declared resources, with legal quantities,
//! > without violating the declared temporal resource-sharing semantics?"
//!
//! It verifies:
//!
//! - reservation identity uniqueness;
//! - reservation self-consistency;
//! - referenced-resource existence;
//! - resource capacity compatibility;
//! - resource sharing semantics;
//! - temporal resource conflicts;
//! - concurrent capacity usage;
//! - consumable-resource accounting constraints that can be established from
//!   the supplied static resource model;
//! - deterministic verification results;
//! - overflow-safe aggregate capacity accounting.
//!
//! It deliberately does NOT verify:
//!
//! - quantum operation semantics;
//! - gate correctness;
//! - dependency ordering;
//! - routing correctness;
//! - timing alignment;
//! - calibration windows;
//! - hardware discovery;
//! - QEC semantics;
//! - classical control semantics;
//! - execution correctness.
//!
//! Those concerns belong to the corresponding verification layers.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir::core::identity
//!          │
//!          ├── OperationId
//!          └── ResourceId
//!
//! quantum::ir::qubit
//!          │
//!          └── canonical QubitId / PhysicalQubitId
//!
//! scheduling::resources::resource
//!          │
//!          └── Resource / ResourceCapacity / ResourceSharing
//!
//! scheduling::resources::reservation
//!          │
//!          └── Reservation
//!
//!                 ▼
//!      verification::resource
//!                 │
//!       ┌─────────┼──────────┐
//!       ▼         ▼          ▼
//!   structural dependency  timing
//!       │         │          │
//!       └─────────┼──────────┘
//!                 ▼
//!       verification::verifier
//! ```
//!
//! # Fundamental invariant
//!
//! A resource-valid schedule must satisfy, for every resource:
//!
//! ```text
//! actual simultaneous usage <= declared capacity
//! ```
//!
//! and, for exclusive resources:
//!
//! ```text
//! no positive-duration reservations overlap
//! ```
//!
//! Intervals use the reservation subsystem's half-open semantics:
//!
//! ```text
//! [start, end)
//! ```
//!
//! Therefore:
//!
//! ```text
//! [0, 10) and [10, 20)
//! ```
//!
//! are adjacent but do not conflict.
//!
//! # Scalability
//!
//! This implementation intentionally does not use:
//!
//! - a fixed number of qubits;
//! - a fixed number of resources;
//! - a fixed number of reservations;
//! - a fixed schedule depth;
//! - a fixed time horizon;
//! - a fixed number of channels;
//! - a fixed machine size;
//! - a fixed resource capacity.
//!
//! Verification uses resource-local event sweeps rather than a global
//! time-slot matrix.
//!
//! For `R` reservations distributed across resources, the expected complexity
//! is:
//!
//! ```text
//! O(R log R)
//! ```
//!
//! for sorting resource-local events, with linear scanning thereafter.
//!
//! Memory consumption is:
//!
//! ```text
//! O(R + M)
//! ```
//!
//! where `M` is the number of declared resources.
//!
//! There is no artificial finite machine-size ceiling.
//!
//! "Infinity" means that this module introduces no semantic maximum. A real
//! verification run remains bounded by available address space, execution
//! policy, and the size of the supplied schedule.
//!
//! # Why this is not O(n²)
//!
//! A naive verifier could compare every reservation with every other
//! reservation:
//!
//! ```text
//! reservation A × reservation B
//! ```
//!
//! That becomes unacceptable for large machines.
//!
//! This implementation instead:
//!
//! ```text
//! reservations
//!      │
//!      ▼
//! group by resource
//!      │
//!      ▼
//! sort by interval start
//!      │
//!      ▼
//! event sweep
//! ```
//!
//! Only reservations that can actually overlap are considered simultaneously.
//!
//! # Capacity semantics
//!
//! `Resource` distinguishes:
//!
//! - capacity;
//! - sharing mode;
//! - resource state.
//!
//! `Reservation` independently carries:
//!
//! - resource identity;
//! - operation identity;
//! - interval;
//! - reservation mode;
//! - quantity.
//!
//! This verifier deliberately does not redefine either model.
//!
//! # Static versus temporal availability
//!
//! A resource being currently marked `Busy`, `Maintenance`, `Disabled`, or
//! `Unknown` does not automatically make every future reservation invalid.
//!
//! Temporal availability belongs to `resources::availability` and
//! `resources::calendar`.
//!
//! Consequently this verifier checks static resource semantics by default and
//! does not reinterpret the current coarse resource state as a future
//! availability calendar.
//!
//! # Consumable resources
//!
//! A `Consumable` resource has a finite amount that can be consumed by
//! execution. A reservation interval still describes when the consumption
//! occurs from the scheduler's perspective.
//!
//! This verifier checks that an individual reservation does not request more
//! than the declared capacity and that concurrent capacity usage is legal when
//! the resource model supports concurrent consumption.
//!
//! Lifetime accounting beyond the supplied reservation set belongs to the
//! resource-pool/execution layer because a schedule may represent only one
//! execution epoch.
//!
//! # Hierarchical resources
//!
//! Hierarchical resource semantics cannot be inferred safely from a flat
//! `Resource` record alone.
//!
//! Therefore hierarchical resources are treated conservatively:
//!
//! - their declared scalar capacity is still checked;
//! - unsupported semantic relationships are reported only when the verifier is
//!   explicitly configured to reject them;
//! - no fake hierarchy is invented here.
//!
//! # Determinism
//!
//! Verification is deterministic:
//!
//! - resource IDs are used as stable grouping keys;
//! - reservations are ordered by start/end/resource/operation/reservation ID;
//! - diagnostics are emitted in deterministic order;
//! - hash-map iteration order is never exposed directly in the result.
//!
//! # Thread safety
//!
//! The verifier owns no global mutable state and contains no locks,
//! interior-mutability primitives, raw pointers, or unsafe code.
//!
//! Independent verification calls may therefore safely be executed
//! concurrently by the caller.
//!
//! # Integration contract
//!
//! The aggregate verifier should call:
//!
//! ```text
//! ResourceVerifier::verify
//! ```
//!
//! with:
//!
//! ```text
//! &[Resource]
//! &[Reservation]
//! ```
//!
//! Planners should call verification only after reservations have been
//! constructed.
//!
//! `resources::reservation` remains responsible for reservation construction
//! and local reservation invariants.
//!
//! `resources::resource` remains responsible for resource semantics.
//!
//! `resources::calendar` remains responsible for dynamic availability.
//!
//! `verification::dependency` remains responsible for dependency ordering.
//!
//! `verification::timing` remains responsible for timing constraints.
//!
//! `verification::semantic` remains responsible for semantic equivalence.
//!
//! `verification::verifier` should aggregate this report with those reports.
//!
//! # Finish-once rule
//!
//! This file intentionally depends only on foundational resource,
//! reservation, identity, and scheduling type contracts.
//!
//! It does not depend on:
//!
//! - planners;
//! - algorithms;
//! - policies;
//! - routing;
//! - hardware providers;
//! - QEC;
//! - runtime;
//! - serialization;
//! - diagnostics;
//! - optimization.
//!
//! Adding those components therefore does not require reopening this file
//! merely to integrate them.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::quantum::ir::core::identity::{OperationId, ResourceId};

use super::super::resources::reservation::{Reservation, ReservationMode};
use super::super::resources::resource::{
    Resource,
    ResourceCapacity,
    ResourceSharing,
};
use super::super::types::{ReservationId, TimePoint};

// =============================================================================
// Verification severity
// =============================================================================

/// Severity of a resource verification diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceVerificationSeverity {
    /// Informational condition that does not invalidate the schedule.
    Info,

    /// Condition that may require attention but does not necessarily make
    /// execution unsafe.
    Warning,

    /// A definite resource correctness violation.
    Error,
}

impl ResourceVerificationSeverity {
    /// Returns whether the diagnostic invalidates a production schedule.
    #[must_use]
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error)
    }
}

impl fmt::Display for ResourceVerificationSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => formatter.write_str("info"),
            Self::Warning => formatter.write_str("warning"),
            Self::Error => formatter.write_str("error"),
        }
    }
}

// =============================================================================
// Verification issue kind
// =============================================================================

/// Machine-readable category of a resource verification issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ResourceVerificationIssueKind {
    /// Two reservations use the same reservation identity.
    DuplicateReservationId,

    /// A reservation's internal mode/quantity representation is inconsistent.
    InvalidReservation,

    /// A reservation references a resource that is not in the supplied target
    /// resource inventory.
    UnknownResource,

    /// A reservation requests more capacity than the resource can provide.
    CapacityExceededBySingleReservation,

    /// Multiple reservations exceed finite concurrent resource capacity.
    ConcurrentCapacityExceeded,

    /// Exclusive resource reservations overlap.
    ExclusiveOverlap,

    /// A reservation mode is incompatible with the resource's sharing mode.
    IncompatibleSharingMode,

    /// A resource has zero capacity while a positive quantity is requested.
    ZeroCapacityResource,

    /// Capacity accounting would overflow the verifier's integer domain.
    CapacityArithmeticOverflow,

    /// The verifier was asked to enforce a semantic rule that cannot be
    /// established from the supplied flat resource model.
    UnsupportedResourceSemantics,
}

impl fmt::Display for ResourceVerificationIssueKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::DuplicateReservationId => "duplicate-reservation-id",
            Self::InvalidReservation => "invalid-reservation",
            Self::UnknownResource => "unknown-resource",
            Self::CapacityExceededBySingleReservation => {
                "single-reservation-capacity-exceeded"
            }
            Self::ConcurrentCapacityExceeded => {
                "concurrent-capacity-exceeded"
            }
            Self::ExclusiveOverlap => "exclusive-resource-overlap",
            Self::IncompatibleSharingMode => "incompatible-sharing-mode",
            Self::ZeroCapacityResource => "zero-capacity-resource",
            Self::CapacityArithmeticOverflow => {
                "capacity-arithmetic-overflow"
            }
            Self::UnsupportedResourceSemantics => {
                "unsupported-resource-semantics"
            }
        };

        formatter.write_str(text)
    }
}

// =============================================================================
// Verification issue
// =============================================================================

/// Structured resource verification diagnostic.
///
/// Indices refer to the caller-supplied reservation/resource slices.
///
/// Keeping indices rather than references makes the report independent from
/// the lifetime of the schedule and suitable for serialization or aggregation
/// by the higher-level verifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceVerificationIssue {
    /// Severity of the issue.
    pub severity: ResourceVerificationSeverity,

    /// Machine-readable issue category.
    pub kind: ResourceVerificationIssueKind,

    /// Index of the primary reservation, when applicable.
    pub reservation_index: Option<usize>,

    /// Index of a second conflicting reservation, when applicable.
    pub conflicting_reservation_index: Option<usize>,

    /// Reservation identity involved in the issue, when applicable.
    pub reservation_id: Option<ReservationId>,

    /// Operation identity involved in the issue, when applicable.
    pub operation_id: Option<OperationId>,

    /// Resource identity involved in the issue, when applicable.
    pub resource_id: Option<ResourceId>,

    /// Start time involved in the issue, when applicable.
    pub start: Option<TimePoint>,

    /// End time involved in the issue, when applicable.
    pub end: Option<TimePoint>,

    /// Requested quantity, when applicable.
    pub requested_quantity: Option<u128>,

    /// Resource capacity, when applicable.
    pub resource_capacity: Option<ResourceCapacity>,

    /// Human-readable explanation.
    pub message: String,
}

impl ResourceVerificationIssue {
    /// Creates a new issue.
    #[must_use]
    pub fn new(
        severity: ResourceVerificationSeverity,
        kind: ResourceVerificationIssueKind,
        message: String,
    ) -> Self {
        Self {
            severity,
            kind,
            reservation_index: None,
            conflicting_reservation_index: None,
            reservation_id: None,
            operation_id: None,
            resource_id: None,
            start: None,
            end: None,
            requested_quantity: None,
            resource_capacity: None,
            message,
        }
    }

    /// Returns whether this issue is an error.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        self.severity.is_error()
    }
}

impl fmt::Display for ResourceVerificationIssue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "[{}] {}: {}",
            self.severity,
            self.kind,
            self.message
        )
    }
}

// =============================================================================
// Verification configuration
// =============================================================================

/// Configuration controlling resource verification behavior.
///
/// Defaults are intentionally suitable for production verification while
/// avoiding assumptions about future hardware models.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceVerificationConfig {
    /// Reject resource-sharing combinations that are provably incompatible.
    pub enforce_sharing_mode: bool,

    /// Reject hierarchical resource semantics that cannot be resolved from the
    /// supplied flat resource inventory.
    pub reject_unsupported_hierarchy: bool,

    /// Continue checking after the first issue.
    ///
    /// `true` is recommended for compiler diagnostics because it allows users
    /// to repair several independent scheduling errors in one compilation.
    pub collect_all_issues: bool,
}

impl Default for ResourceVerificationConfig {
    fn default() -> Self {
        Self {
            enforce_sharing_mode: true,
            reject_unsupported_hierarchy: false,
            collect_all_issues: true,
        }
    }
}

// =============================================================================
// Verification statistics
// =============================================================================

/// Deterministic statistics describing a verification pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ResourceVerificationStatistics {
    /// Number of reservations inspected.
    pub reservations_checked: usize,

    /// Number of distinct resources referenced by reservations.
    pub referenced_resources: usize,

    /// Number of declared resources supplied to the verifier.
    pub resources_checked: usize,

    /// Number of positive-duration reservations.
    pub positive_duration_reservations: usize,

    /// Number of zero-duration reservations.
    pub zero_duration_reservations: usize,

    /// Number of finite-capacity resources encountered.
    pub finite_capacity_resources: usize,

    /// Number of unlimited-capacity resources encountered.
    pub unlimited_capacity_resources: usize,

    /// Number of diagnostics generated.
    pub issues: usize,

    /// Number of error diagnostics generated.
    pub errors: usize,

    /// Number of warning diagnostics generated.
    pub warnings: usize,
}

// =============================================================================
// Verification report
// =============================================================================

/// Complete result of resource verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceVerificationReport {
    valid: bool,
    issues: Vec<ResourceVerificationIssue>,
    statistics: ResourceVerificationStatistics,
}

impl ResourceVerificationReport {
    /// Creates an empty successful report.
    #[must_use]
    pub fn success(statistics: ResourceVerificationStatistics) -> Self {
        Self {
            valid: true,
            issues: Vec::new(),
            statistics,
        }
    }

    /// Returns whether the schedule satisfies all enforced resource rules.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.valid
    }

    /// Returns all diagnostics in deterministic order.
    #[must_use]
    pub fn issues(&self) -> &[ResourceVerificationIssue] {
        &self.issues
    }

    /// Returns verification statistics.
    #[must_use]
    pub const fn statistics(&self) -> ResourceVerificationStatistics {
        self.statistics
    }

    /// Returns whether at least one error exists.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(ResourceVerificationIssue::is_error)
    }

    /// Returns the number of errors.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.is_error())
            .count()
    }

    /// Returns the number of warnings.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| {
                issue.severity == ResourceVerificationSeverity::Warning
            })
            .count()
    }
}

// =============================================================================
// Internal sweep event
// =============================================================================

/// Event used by the resource-local sweep.
///
/// Events are generated only for positive-duration reservations because a
/// zero-duration half-open interval occupies no temporal region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SweepEvent {
    Start {
        time: TimePoint,
        reservation_index: usize,
    },
    End {
        time: TimePoint,
        reservation_index: usize,
    },
}

impl SweepEvent {
    /// Returns the event time.
    #[must_use]
    const fn time(self) -> TimePoint {
        match self {
            Self::Start { time, .. } | Self::End { time, .. } => time,
        }
    }

    /// Returns the reservation index.
    #[must_use]
    const fn reservation_index(self) -> usize {
        match self {
            Self::Start {
                reservation_index,
                ..
            }
            | Self::End {
                reservation_index,
                ..
            } => reservation_index,
        }
    }

    /// Returns whether this is an end event.
    ///
    /// End events sort before start events at the same timestamp. This is
    /// essential for half-open intervals:
    ///
    /// [0, 10) and [10, 20)
    ///
    /// do not overlap.
    #[must_use]
    const fn is_end(self) -> bool {
        matches!(self, Self::End { .. })
    }
}

// =============================================================================
// Resource verifier
// =============================================================================

/// Production resource verifier.
///
/// The verifier is stateless. All target and schedule information is supplied
/// to `verify`, making the verifier reusable and safe to invoke concurrently.
#[derive(Debug, Clone, Copy)]
pub struct ResourceVerifier {
    config: ResourceVerificationConfig,
}

impl ResourceVerifier {
    /// Creates a verifier with the supplied configuration.
    #[must_use]
    pub const fn new(config: ResourceVerificationConfig) -> Self {
        Self { config }
    }

    /// Returns the verifier configuration.
    #[must_use]
    pub const fn config(self) -> ResourceVerificationConfig {
        self.config
    }

    /// Creates a verifier using production defaults.
    #[must_use]
    pub const fn production() -> Self {
        Self::new(ResourceVerificationConfig {
            enforce_sharing_mode: true,
            reject_unsupported_hierarchy: false,
            collect_all_issues: true,
        })
    }

    /// Verifies resource correctness of a schedule.
    ///
    /// # Inputs
    ///
    /// `resources` is the complete resource inventory visible to this
    /// scheduling context.
    ///
    /// `reservations` is the set of reservations belonging to the schedule.
    ///
    /// # Guarantees
    ///
    /// The method:
    ///
    /// - never mutates either input;
    /// - never performs hardware I/O;
    /// - never accesses global state;
    /// - never uses unsafe code;
    /// - never imposes a machine-size limit;
    /// - returns deterministic diagnostics.
    ///
    /// # Complexity
    ///
    /// Expected:
    ///
    /// ```text
    /// O(M + R log R)
    /// ```
    ///
    /// where `M` is the number of resources and `R` is the number of
    /// reservations.
    #[must_use]
    pub fn verify(
        &self,
        resources: &[Resource],
        reservations: &[Reservation],
    ) -> ResourceVerificationReport {
        let mut statistics = ResourceVerificationStatistics {
            reservations_checked: reservations.len(),
            resources_checked: resources.len(),
            ..ResourceVerificationStatistics::default()
        };

        let resource_map = self.build_resource_map(resources, &mut statistics);

        let mut issues = Vec::new();

        let mut seen_reservation_ids: HashMap<ReservationId, usize> =
            HashMap::with_capacity(reservations.len());

        let mut reservations_by_resource: HashMap<
            ResourceId,
            Vec<usize>,
        > = HashMap::new();

        let mut referenced_resources: HashSet<ResourceId> =
            HashSet::new();

        for (reservation_index, reservation) in reservations.iter().enumerate() {
            if !self.config.collect_all_issues && !issues.is_empty() {
                break;
            }

            self.record_duration_statistics(reservation, &mut statistics);

            if let Some(previous_index) =
                seen_reservation_ids.insert(reservation.id(), reservation_index)
            {
                let mut issue = ResourceVerificationIssue::new(
                    ResourceVerificationSeverity::Error,
                    ResourceVerificationIssueKind::DuplicateReservationId,
                    format!(
                        "reservation `{}` appears at indices {} and {}",
                        reservation.id(),
                        previous_index,
                        reservation_index
                    ),
                );

                issue.reservation_index = Some(reservation_index);
                issue.conflicting_reservation_index = Some(previous_index);
                issue.reservation_id = Some(reservation.id());
                issue.operation_id = Some(reservation.operation_id());
                issue.resource_id = Some(reservation.resource_id());

                issues.push(issue);

                if !self.config.collect_all_issues {
                    break;
                }
            }

            if let Err(error) = reservation.validate() {
                let mut issue = ResourceVerificationIssue::new(
                    ResourceVerificationSeverity::Error,
                    ResourceVerificationIssueKind::InvalidReservation,
                    format!(
                        "reservation `{}` is internally inconsistent: {}",
                        reservation.id(),
                        error
                    ),
                );

                issue.reservation_index = Some(reservation_index);
                issue.reservation_id = Some(reservation.id());
                issue.operation_id = Some(reservation.operation_id());
                issue.resource_id = Some(reservation.resource_id());
                issue.start = Some(reservation.start());
                issue.end = Some(reservation.end());
                issue.requested_quantity = Some(reservation.quantity());

                issues.push(issue);

                if !self.config.collect_all_issues {
                    break;
                }
            }

            let resource_id = reservation.resource_id();
            referenced_resources.insert(resource_id);

            let resource = match resource_map.get(&resource_id) {
                Some(resource) => *resource,
                None => {
                    let mut issue = ResourceVerificationIssue::new(
                        ResourceVerificationSeverity::Error,
                        ResourceVerificationIssueKind::UnknownResource,
                        format!(
                            "reservation `{}` references undeclared resource `{}`",
                            reservation.id(),
                            resource_id
                        ),
                    );

                    issue.reservation_index = Some(reservation_index);
                    issue.reservation_id = Some(reservation.id());
                    issue.operation_id = Some(reservation.operation_id());
                    issue.resource_id = Some(resource_id);
                    issue.start = Some(reservation.start());
                    issue.end = Some(reservation.end());
                    issue.requested_quantity =
                        Some(reservation.quantity());

                    issues.push(issue);

                    if !self.config.collect_all_issues {
                        break;
                    }

                    continue;
                }
            };

            if !resource.can_satisfy(self.requirement_for(reservation)) {
                let kind =
                    if resource.capacity().is_zero()
                        && reservation.quantity() > 0
                    {
                        ResourceVerificationIssueKind::ZeroCapacityResource
                    } else {
                        ResourceVerificationIssueKind::
                            CapacityExceededBySingleReservation
                    };

                let mut issue = ResourceVerificationIssue::new(
                    ResourceVerificationSeverity::Error,
                    kind,
                    format!(
                        "reservation `{}` requests quantity {} from resource `{}` \
                         with capacity {}",
                        reservation.id(),
                        reservation.quantity(),
                        resource.id(),
                        resource.capacity()
                    ),
                );

                issue.reservation_index = Some(reservation_index);
                issue.reservation_id = Some(reservation.id());
                issue.operation_id = Some(reservation.operation_id());
                issue.resource_id = Some(resource.id());
                issue.start = Some(reservation.start());
                issue.end = Some(reservation.end());
                issue.requested_quantity = Some(reservation.quantity());
                issue.resource_capacity = Some(resource.capacity());

                issues.push(issue);

                if !self.config.collect_all_issues {
                    break;
                }
            }

            if self.config.enforce_sharing_mode {
                if let Some(issue) =
                    self.check_sharing_compatibility(reservation, resource)
                {
                    issues.push(issue);

                    if !self.config.collect_all_issues {
                        break;
                    }
                }
            }

            if self.config.reject_unsupported_hierarchy
                && resource.sharing() == ResourceSharing::Hierarchical
            {
                let mut issue = ResourceVerificationIssue::new(
                    ResourceVerificationSeverity::Error,
                    ResourceVerificationIssueKind::UnsupportedResourceSemantics,
                    format!(
                        "hierarchical resource `{}` requires hierarchy-aware \
                         capacity semantics that are not represented by the \
                         supplied flat resource inventory",
                        resource.id()
                    ),
                );

                issue.reservation_index = Some(reservation_index);
                issue.reservation_id = Some(reservation.id());
                issue.operation_id = Some(reservation.operation_id());
                issue.resource_id = Some(resource.id());

                issues.push(issue);

                if !self.config.collect_all_issues {
                    break;
                }
            }

            reservations_by_resource
                .entry(resource_id)
                .or_default()
                .push(reservation_index);
        }

        statistics.referenced_resources = referenced_resources.len();

        if self.config.collect_all_issues || issues.is_empty() {
            self.verify_resource_groups(
                resources,
                reservations,
                &reservations_by_resource,
                &mut issues,
            );
        }

        issues.sort_by(Self::issue_ordering);

        statistics.issues = issues.len();
        statistics.errors = issues
            .iter()
            .filter(|issue| issue.severity.is_error())
            .count();
        statistics.warnings = issues
            .iter()
            .filter(|issue| {
                issue.severity == ResourceVerificationSeverity::Warning
            })
            .count();

        ResourceVerificationReport {
            valid: statistics.errors == 0,
            issues,
            statistics,
        }
    }

    // =========================================================================
    // Resource inventory
    // =========================================================================

    fn build_resource_map<'a>(
        &self,
        resources: &'a [Resource],
        statistics: &mut ResourceVerificationStatistics,
    ) -> HashMap<ResourceId, &'a Resource> {
        let mut map = HashMap::with_capacity(resources.len());

        for resource in resources {
            match map.insert(resource.id(), resource) {
                Some(previous) => {
                    // Duplicate resource identities are not represented as an
                    // issue here because the resource inventory itself is not
                    // a reservation. The last insertion would otherwise make
                    // verification order-dependent.
                    //
                    // Keep the first declaration authoritative. The aggregate
                    // resource verifier can separately report duplicate
                    // inventory identities if desired.
                    let _ = map.insert(previous.id(), previous);
                }
                None => {}
            }

            if resource.capacity().is_finite() {
                statistics.finite_capacity_resources =
                    statistics.finite_capacity_resources.saturating_add(1);
            } else {
                statistics.unlimited_capacity_resources =
                    statistics.unlimited_capacity_resources.saturating_add(1);
            }
        }

        map
    }

    // =========================================================================
    // Reservation statistics
    // =========================================================================

    fn record_duration_statistics(
        &self,
        reservation: &Reservation,
        statistics: &mut ResourceVerificationStatistics,
    ) {
        if reservation.start() < reservation.end() {
            statistics.positive_duration_reservations =
                statistics.positive_duration_reservations.saturating_add(1);
        } else {
            statistics.zero_duration_reservations =
                statistics.zero_duration_reservations.saturating_add(1);
        }
    }

    // =========================================================================
    // Resource requirements
    // =========================================================================

    /// Converts reservation semantics to the existing resource requirement
    /// vocabulary without creating a second resource model.
    fn requirement_for(
        &self,
        reservation: &Reservation,
    ) -> super::super::resources::resource::ResourceRequirement {
        use super::super::resources::resource::ResourceRequirement;

        match reservation.mode() {
            ReservationMode::Exclusive => {
                ResourceRequirement::exclusive_units(
                    reservation.resource(),
                    reservation.quantity().into(),
                )
            }

            ReservationMode::Shared => ResourceRequirement::shared(
                reservation.resource(),
                reservation.quantity().into(),
            ),

            ReservationMode::Capacity { .. }
            | ReservationMode::Reusable { .. } => {
                ResourceRequirement::shared(
                    reservation.resource(),
                    reservation.quantity().into(),
                )
            }

            ReservationMode::Consumable { .. } => {
                ResourceRequirement::consumable(
                    reservation.resource(),
                    reservation.quantity().into(),
                )
            }
        }
    }

    // =========================================================================
    // Sharing compatibility
    // =========================================================================

    fn check_sharing_compatibility(
        &self,
        reservation: &Reservation,
        resource: &Resource,
    ) -> Option<ResourceVerificationIssue> {
        let compatible = match (resource.sharing(), reservation.mode()) {
            // Exclusive resource + exclusive reservation is valid.
            (ResourceSharing::Exclusive, ReservationMode::Exclusive) => true,

            // An exclusive resource cannot safely accept a reservation that
            // explicitly declares sharing semantics.
            (ResourceSharing::Exclusive, ReservationMode::Shared) => false,

            // Capacity/reusable reservations can be checked by capacity.
            (ResourceSharing::Exclusive, ReservationMode::Capacity { .. }) => {
                true
            }

            (ResourceSharing::Exclusive, ReservationMode::Reusable { .. }) => {
                true
            }

            (ResourceSharing::Exclusive, ReservationMode::Consumable { .. }) => {
                true
            }

            // Shared resources support explicit shared and capacity use.
            (ResourceSharing::Shared, ReservationMode::Exclusive) => true,
            (ResourceSharing::Shared, ReservationMode::Shared) => true,
            (ResourceSharing::Shared, ReservationMode::Capacity { .. }) => true,
            (ResourceSharing::Shared, ReservationMode::Reusable { .. }) => true,
            (ResourceSharing::Shared, ReservationMode::Consumable { .. }) => {
                true
            }

            // A reusable resource is explicitly designed to be occupied and
            // reused after release.
            (ResourceSharing::Reusable, ReservationMode::Exclusive) => true,
            (ResourceSharing::Reusable, ReservationMode::Reusable { .. }) => {
                true
            }
            (ResourceSharing::Reusable, ReservationMode::Shared) => false,
            (ResourceSharing::Reusable, ReservationMode::Capacity { .. }) => {
                false
            }
            (ResourceSharing::Reusable, ReservationMode::Consumable { .. }) => {
                false
            }

            // Consumable resources should not be interpreted as reusable
            // shared pools by this verifier.
            (ResourceSharing::Consumable, ReservationMode::Consumable { .. }) => {
                true
            }
            (ResourceSharing::Consumable, ReservationMode::Exclusive) => true,
            (ResourceSharing::Consumable, ReservationMode::Shared) => false,
            (ResourceSharing::Consumable, ReservationMode::Capacity { .. }) => {
                false
            }
            (ResourceSharing::Consumable, ReservationMode::Reusable { .. }) => {
                false
            }

            // Hierarchical semantics require higher-level resource
            // interpretation. Do not reject by default.
            (ResourceSharing::Hierarchical, _) => true,
        };

        if compatible {
            return None;
        }

        let mut issue = ResourceVerificationIssue::new(
            ResourceVerificationSeverity::Error,
            ResourceVerificationIssueKind::IncompatibleSharingMode,
            format!(
                "reservation `{}` uses mode `{}` on resource `{}` whose \
                 sharing mode is `{}`",
                reservation.id(),
                reservation.mode(),
                resource.id(),
                resource.sharing()
            ),
        );

        issue.reservation_id = Some(reservation.id());
        issue.operation_id = Some(reservation.operation_id());
        issue.resource_id = Some(resource.id());
        issue.start = Some(reservation.start());
        issue.end = Some(reservation.end());
        issue.requested_quantity = Some(reservation.quantity());
        issue.resource_capacity = Some(resource.capacity());

        Some(issue)
    }

    // =========================================================================
    // Resource-local verification
    // =========================================================================

    fn verify_resource_groups(
        &self,
        resources: &[Resource],
        reservations: &[Reservation],
        reservations_by_resource: &HashMap<ResourceId, Vec<usize>>,
        issues: &mut Vec<ResourceVerificationIssue>,
    ) {
        // Iterate over the authoritative resource inventory rather than the
        // HashMap so diagnostics are deterministic.
        for resource in resources {
            let reservation_indices =
                match reservations_by_resource.get(&resource.id()) {
                    Some(indices) => indices,
                    None => continue,
                };

            self.verify_one_resource(
                resource,
                reservations,
                reservation_indices,
                issues,
            );

            if !self.config.collect_all_issues && !issues.is_empty() {
                return;
            }
        }
    }

    fn verify_one_resource(
        &self,
        resource: &Resource,
        reservations: &[Reservation],
        reservation_indices: &[usize],
        issues: &mut Vec<ResourceVerificationIssue>,
    ) {
        let mut events = Vec::with_capacity(
            reservation_indices.len().saturating_mul(2),
        );

        for &reservation_index in reservation_indices {
            let reservation = &reservations[reservation_index];

            // Zero-duration reservations do not occupy any half-open temporal
            // interval and therefore cannot cause an overlap.
            if reservation.start() == reservation.end() {
                continue;
            }

            events.push(SweepEvent::Start {
                time: reservation.start(),
                reservation_index,
            });

            events.push(SweepEvent::End {
                time: reservation.end(),
                reservation_index,
            });
        }

        events.sort_by(Self::event_ordering);

        match resource.sharing() {
            ResourceSharing::Exclusive | ResourceSharing::Reusable => {
                self.verify_exclusive_like_resource(
                    resource,
                    reservations,
                    &events,
                    issues,
                );
            }

            ResourceSharing::Shared
            | ResourceSharing::Consumable
            | ResourceSharing::Hierarchical => {
                self.verify_capacity_resource(
                    resource,
                    reservations,
                    &events,
                    issues,
                );
            }
        }
    }

    // =========================================================================
    // Exclusive verification
    // =========================================================================

    fn verify_exclusive_like_resource(
        &self,
        resource: &Resource,
        reservations: &[Reservation],
        events: &[SweepEvent],
        issues: &mut Vec<ResourceVerificationIssue>,
    ) {
        let mut active: Option<usize> = None;

        for event in events {
            match *event {
                SweepEvent::End {
                    reservation_index,
                    ..
                } => {
                    if active == Some(reservation_index) {
                        active = None;
                    }
                }

                SweepEvent::Start {
                    reservation_index,
                    ..
                } => {
                    if let Some(active_index) = active {
                        let current = &reservations[reservation_index];
                        let previous = &reservations[active_index];

                        if current.overlaps(*previous) {
                            let mut issue = ResourceVerificationIssue::new(
                                ResourceVerificationSeverity::Error,
                                ResourceVerificationIssueKind::
                                    ExclusiveOverlap,
                                format!(
                                    "reservations `{}` and `{}` overlap on \
                                     exclusive resource `{}`",
                                    previous.id(),
                                    current.id(),
                                    resource.id()
                                ),
                            );

                            issue.reservation_index =
                                Some(reservation_index);
                            issue.conflicting_reservation_index =
                                Some(active_index);
                            issue.reservation_id = Some(current.id());
                            issue.operation_id =
                                Some(current.operation_id());
                            issue.resource_id = Some(resource.id());
                            issue.start = Some(current.start());
                            issue.end = Some(current.end());
                            issue.requested_quantity =
                                Some(current.quantity());
                            issue.resource_capacity =
                                Some(resource.capacity());

                            issues.push(issue);

                            if !self.config.collect_all_issues {
                                return;
                            }
                        }
                    }

                    active = Some(reservation_index);
                }
            }
        }
    }

    // =========================================================================
    // Capacity verification
    // =========================================================================

    fn verify_capacity_resource(
        &self,
        resource: &Resource,
        reservations: &[Reservation],
        events: &[SweepEvent],
        issues: &mut Vec<ResourceVerificationIssue>,
    ) {
        let Some(capacity) = resource.capacity().finite_value() else {
            // Unlimited capacity cannot overflow at the resource level.
            // Individual reservation legality has already been checked.
            return;
        };

        let mut active: HashMap<usize, u128> = HashMap::new();
        let mut usage: u128 = 0;

        let mut index = 0;

        while index < events.len() {
            let current_time = events[index].time();

            // Half-open interval semantics require every end event at T to be
            // processed before every start event at T.
            //
            // We therefore process all events at the same time in two passes.
            let mut end_cursor = index;

            while end_cursor < events.len()
                && events[end_cursor].time() == current_time
                && events[end_cursor].is_end()
            {
                let reservation_index =
                    events[end_cursor].reservation_index();

                if let Some(quantity) = active.remove(&reservation_index) {
                    usage = usage.saturating_sub(quantity);
                }

                end_cursor += 1;
            }

            let mut start_cursor = end_cursor;

            while start_cursor < events.len()
                && events[start_cursor].time() == current_time
            {
                let event = events[start_cursor];

                if !event.is_end() {
                    let reservation_index = event.reservation_index();
                    let reservation = &reservations[reservation_index];
                    let quantity = reservation.quantity();

                    let next_usage = match usage.checked_add(quantity) {
                        Some(value) => value,
                        None => {
                            let mut issue = ResourceVerificationIssue::new(
                                ResourceVerificationSeverity::Error,
                                ResourceVerificationIssueKind::
                                    CapacityArithmeticOverflow,
                                format!(
                                    "capacity usage for resource `{}` \
                                     overflowed while processing reservation `{}`",
                                    resource.id(),
                                    reservation.id()
                                ),
                            );

                            issue.reservation_index =
                                Some(reservation_index);
                            issue.reservation_id =
                                Some(reservation.id());
                            issue.operation_id =
                                Some(reservation.operation_id());
                            issue.resource_id = Some(resource.id());
                            issue.start = Some(reservation.start());
                            issue.end = Some(reservation.end());
                            issue.requested_quantity = Some(quantity);
                            issue.resource_capacity =
                                Some(resource.capacity());

                            issues.push(issue);

                            if !self.config.collect_all_issues {
                                return;
                            }

                            // Saturate only for continued diagnostic
                            // collection. This state must never be treated as
                            // valid.
                            u128::MAX
                        }
                    };

                    usage = next_usage;
                    active.insert(reservation_index, quantity);

                    if usage > capacity {
                        let mut issue = ResourceVerificationIssue::new(
                            ResourceVerificationSeverity::Error,
                            ResourceVerificationIssueKind::
                                ConcurrentCapacityExceeded,
                            format!(
                                "resource `{}` exceeds capacity {} at time \
                                 {}: concurrent usage is {}",
                                resource.id(),
                                capacity,
                                current_time,
                                usage
                            ),
                        );

                        issue.reservation_index =
                            Some(reservation_index);
                        issue.reservation_id =
                            Some(reservation.id());
                        issue.operation_id =
                            Some(reservation.operation_id());
                        issue.resource_id = Some(resource.id());
                        issue.start = Some(reservation.start());
                        issue.end = Some(reservation.end());
                        issue.requested_quantity = Some(quantity);
                        issue.resource_capacity =
                            Some(resource.capacity());

                        issues.push(issue);

                        if !self.config.collect_all_issues {
                            return;
                        }
                    }
                }

                start_cursor += 1;
            }

            index = start_cursor;
        }
    }

    // =========================================================================
    // Deterministic ordering
    // =========================================================================

    fn event_ordering(left: &SweepEvent, right: &SweepEvent) -> std::cmp::Ordering {
        left.time()
            .cmp(&right.time())
            .then_with(|| {
                // End before start at equal time because reservations are
                // half-open.
                right.is_end().cmp(&left.is_end())
            })
            .then_with(|| {
                left.reservation_index()
                    .cmp(&right.reservation_index())
            })
    }

    fn issue_ordering(
        left: &ResourceVerificationIssue,
        right: &ResourceVerificationIssue,
    ) -> std::cmp::Ordering {
        left.resource_id
            .cmp(&right.resource_id)
            .then_with(|| {
                left.reservation_index
                    .cmp(&right.reservation_index)
            })
            .then_with(|| {
                left.conflicting_reservation_index
                    .cmp(&right.conflicting_reservation_index)
            })
            .then_with(|| left.kind.cmp(&right.kind))
            .then_with(|| left.message.cmp(&right.message))
    }
}

// =============================================================================
// Convenience API
// =============================================================================

/// Verifies resources using production defaults.
///
/// This is the stable one-call API for higher-level verification code that
/// does not need custom configuration.
#[must_use]
pub fn verify_resources(
    resources: &[Resource],
    reservations: &[Reservation],
) -> ResourceVerificationReport {
    ResourceVerifier::production().verify(resources, reservations)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::core::identity::{
        OperationId,
        ResourceId,
    };

    use super::super::super::resources::reservation::Reservation;
    use super::super::super::resources::resource::{
        Resource,
        ResourceKind,
        ResourceOwnership,
        ResourceScope,
        ResourceState,
    };
    use super::super::super::types::{
        OperationRef,
        ResourceRef,
        ReservationId,
    };

    fn resource(id: u64, capacity: u128, sharing: ResourceSharing) -> Resource {
        Resource::new(
            ResourceId::new(id),
            ResourceKind::Compute,
            ResourceCapacity::finite(capacity),
            sharing,
            ResourceState::Available,
            ResourceScope::Device,
            super::super::super::resources::resource::ResourceAffinity::None,
            ResourceOwnership::Local,
        )
    }

    fn reservation(
        reservation_id: u64,
        operation_id: u64,
        resource_id: u64,
        start: u128,
        end: u128,
        quantity: u128,
        mode: ReservationMode,
    ) -> Reservation {
        Reservation::new(
            ReservationId::new(reservation_id),
            OperationRef::new(OperationId::new(operation_id)),
            ResourceRef::new(ResourceId::new(resource_id)),
            super::super::super::types::TimeInterval::new(
                TimePoint::new(start),
                TimePoint::new(end),
            )
            .expect("test interval must be valid"),
            mode,
            quantity,
        )
    }

    #[test]
    fn empty_schedule_is_valid() {
        let report = verify_resources(&[], &[]);

        assert!(report.is_valid());
        assert_eq!(report.statistics().reservations_checked, 0);
    }

    #[test]
    fn adjacent_exclusive_reservations_do_not_conflict() {
        let resources = [resource(1, 1, ResourceSharing::Exclusive)];

        let reservations = [
            reservation(
                1,
                1,
                1,
                0,
                10,
                1,
                ReservationMode::Exclusive,
            ),
            reservation(
                2,
                2,
                1,
                10,
                20,
                1,
                ReservationMode::Exclusive,
            ),
        ];

        let report = verify_resources(&resources, &reservations);

        assert!(report.is_valid());
    }

    #[test]
    fn overlapping_exclusive_reservations_are_rejected() {
        let resources = [resource(1, 1, ResourceSharing::Exclusive)];

        let reservations = [
            reservation(
                1,
                1,
                1,
                0,
                10,
                1,
                ReservationMode::Exclusive,
            ),
            reservation(
                2,
                2,
                1,
                5,
                20,
                1,
                ReservationMode::Exclusive,
            ),
        ];

        let report = verify_resources(&resources, &reservations);

        assert!(!report.is_valid());
        assert!(report.issues().iter().any(|issue| {
            issue.kind == ResourceVerificationIssueKind::ExclusiveOverlap
        }));
    }

    #[test]
    fn shared_capacity_allows_legal_parallel_usage() {
        let resources = [resource(1, 2, ResourceSharing::Shared)];

        let reservations = [
            reservation(
                1,
                1,
                1,
                0,
                10,
                1,
                ReservationMode::Shared,
            ),
            reservation(
                2,
                2,
                1,
                0,
                10,
                1,
                ReservationMode::Shared,
            ),
        ];

        let report = verify_resources(&resources, &reservations);

        assert!(report.is_valid());
    }

    #[test]
    fn shared_capacity_rejects_excess_parallel_usage() {
        let resources = [resource(1, 2, ResourceSharing::Shared)];

        let reservations = [
            reservation(
                1,
                1,
                1,
                0,
                10,
                1,
                ReservationMode::Shared,
            ),
            reservation(
                2,
                2,
                1,
                0,
                10,
                1,
                ReservationMode::Shared,
            ),
            reservation(
                3,
                3,
                1,
                0,
                10,
                1,
                ReservationMode::Shared,
            ),
        ];

        let report = verify_resources(&resources, &reservations);

        assert!(!report.is_valid());
        assert!(report.issues().iter().any(|issue| {
            issue.kind
                == ResourceVerificationIssueKind::
                    ConcurrentCapacityExceeded
        }));
    }

    #[test]
    fn unknown_resource_is_rejected() {
        let reservations = [reservation(
            1,
            1,
            99,
            0,
            10,
            1,
            ReservationMode::Exclusive,
        )];

        let report = verify_resources(&[], &reservations);

        assert!(!report.is_valid());
        assert!(report.issues().iter().any(|issue| {
            issue.kind == ResourceVerificationIssueKind::UnknownResource
        }));
    }

    #[test]
    fn single_reservation_cannot_exceed_capacity() {
        let resources = [resource(1, 1, ResourceSharing::Shared)];

        let reservations = [reservation(
            1,
            1,
            1,
            0,
            10,
            2,
            ReservationMode::Shared,
        )];

        let report = verify_resources(&resources, &reservations);

        assert!(!report.is_valid());
        assert!(report.issues().iter().any(|issue| {
            issue.kind
                == ResourceVerificationIssueKind::
                    CapacityExceededBySingleReservation
        }));
    }

    #[test]
    fn duplicate_reservation_identity_is_rejected() {
        let resources = [resource(1, 2, ResourceSharing::Shared)];

        let reservations = [
            reservation(
                1,
                1,
                1,
                0,
                10,
                1,
                ReservationMode::Shared,
            ),
            reservation(
                1,
                2,
                1,
                10,
                20,
                1,
                ReservationMode::Shared,
            ),
        ];

        let report = verify_resources(&resources, &reservations);

        assert!(!report.is_valid());
        assert!(report.issues().iter().any(|issue| {
            issue.kind == ResourceVerificationIssueKind::DuplicateReservationId
        }));
    }

    #[test]
    fn zero_duration_reservation_does_not_create_overlap() {
        let resources = [resource(1, 1, ResourceSharing::Exclusive)];

        let reservations = [
            reservation(
                1,
                1,
                1,
                10,
                10,
                1,
                ReservationMode::Exclusive,
            ),
            reservation(
                2,
                2,
                1,
                10,
                20,
                1,
                ReservationMode::Exclusive,
            ),
        ];

        let report = verify_resources(&resources, &reservations);

        assert!(report.is_valid());
        assert_eq!(
            report.statistics().zero_duration_reservations,
            1
        );
    }
}