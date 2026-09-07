//! Zamani Quantum Resilience — Resource Validation
//!
//! Path:
//!     src/quantum/resilience/limits/validation.rs
//!
//! Purpose:
//!     Provides the production validation boundary for resilience resource
//!     requirements.
//!
//! Architectural position:
//!
//!     quantum::ir::qubit
//!         └── owns canonical logical/physical qubit identities
//!
//!     quantum::hardware
//!         └── owns actual target capabilities and availability
//!
//!     quantum::routing
//!         └── determines physical realization requirements
//!
//!     quantum::scheduling
//!         └── determines temporal/concurrency requirements
//!
//!     quantum::optimization
//!         └── may change resource demand
//!
//!     quantum::resilience::limits::limits
//!         └── owns limit semantics
//!
//!     quantum::resilience::limits::resource
//!         └── owns resource demand/availability semantics
//!
//!     quantum::resilience::limits::validation
//!         └── owns validation orchestration and validation reports
//!
//!     quantum::resilience::planning
//!         └── consumes validation results for plan feasibility
//!
//!     quantum::resilience::recovery
//!         └── validates recovery resource requirements before execution
//!
//! This module deliberately does NOT:
//!
//!     - discover hardware;
//!     - perform routing;
//!     - perform scheduling;
//!     - perform optimization;
//!     - implement QEC;
//!     - implement error mitigation;
//!     - execute quantum operations;
//!     - reserve hardware resources;
//!     - mutate runtime state;
//!     - define a second qubit identity system;
//!     - impose fixed machine-size limits.
//!
//! -----------------------------------------------------------------------------
//! CORE INVARIANTS
//! -----------------------------------------------------------------------------
//!
//! 1. Validation must fail closed for unknown required capacity.
//! 2. `Unknown` availability is never interpreted as unlimited.
//! 3. `Limit::Unlimited` means only that this resilience layer imposes no
//!    additional upper bound.
//! 4. `Limit::Unlimited` does NOT mean that hardware is physically unlimited.
//! 5. Explicit resilience limits are checked before target availability.
//! 6. Validation is deterministic.
//! 7. Validation performs no I/O.
//! 8. Validation performs no hardware discovery.
//! 9. Validation performs no allocation.
//! 10. Validation performs no locking.
//! 11. Validation performs no random operations.
//! 12. Validation performs no wall-clock reads.
//! 13. Validation never silently truncates numeric values.
//! 14. Overflow is treated as failure by resource aggregation.
//! 15. A successful validation means only that the supplied demand is
//!     compatible with the supplied limits and known availability.
//! 16. Validation does not prove that a quantum program is semantically
//!     correct; semantic validation belongs to the IR/verification layers.
//! 17. Validation does not prove that a backend can execute an operation;
//!     capability compatibility belongs to hardware/target validation.
//! 18. This module contains no `unsafe` code.
//!
//! -----------------------------------------------------------------------------
//! SCALABILITY
//! -----------------------------------------------------------------------------
//!
//! The validation layer has no machine-size constant.
//!
//! It supports:
//!
//!     one qubit
//!     small QPUs
//!     large QPUs
//!     logical-qubit systems
//!     distributed quantum systems
//!     heterogeneous quantum systems
//!
//! by validating quantities supplied by the current execution context.
//!
//! "Infinity" means:
//!
//!     no artificial limit imposed by this layer.
//!
//! It does NOT mean:
//!
//!     infinite physical resources.
//!
//! Actual feasibility is always determined by the intersection of:
//!
//!     requested demand
//!         ∩ explicit resilience limits
//!         ∩ target availability
//!         ∩ runtime resources
//!         ∩ security constraints
//!         ∩ execution policy
//!
//! -----------------------------------------------------------------------------
//! INTEGRATION CONTRACT
//! -----------------------------------------------------------------------------
//!
//! `limits.rs`
//!     Owns `Limit`, `LimitSet`, `ResourceKind`, and `LimitViolation`.
//!
//! `resource.rs`
//!     Owns `ResourceDemand`, `ResourceAvailability`,
//!     `ResourceAssessment`, and `ResourceValidation`.
//!
//! `policy/budgets.rs`
//!     Supplies policy-derived limits.
//!
//! `hardware`
//!     Supplies target-derived availability/capability information.
//!
//! `routing`
//!     Supplies physical-qubit and connectivity-related requirements.
//!
//! `scheduling`
//!     Supplies timing/concurrency requirements.
//!
//! `optimization`
//!     May alter the final resource demand and therefore must be evaluated
//!     after the relevant optimization stage when exact resource validation
//!     is required.
//!
//! `planning/feasibility.rs`
//!     Should use `ValidationReport` or `validate_request` before ranking a
//!     recovery/adaptation plan.
//!
//! `recovery/*`
//!     Must validate the complete recovery demand before execution.
//!
//! `verification/*`
//!     May consume the validation result as part of provenance and acceptance,
//!     but resource validation alone is never sufficient for result acceptance.
//!
//! `serialization/*`
//!     May serialize validation outcomes for deterministic replay/provenance.
//!
//! -----------------------------------------------------------------------------
//! CANONICAL QUBIT IDENTITY
//! -----------------------------------------------------------------------------
//!
//! This module validates resource quantities rather than individual qubit
//! identities, so it intentionally does not import `QubitId`.
//!
//! Whenever a caller needs to identify a logical or physical qubit, it must use
//! the canonical definitions owned by:
//!
//!     crate::quantum::ir::qubit
//!
//! This module must never introduce:
//!
//!     ResilienceQubitId
//!     ResourceQubitId
//!     LogicalQubitId
//!     PhysicalQubitId
//!
//! as competing identity systems.
//!
//! -----------------------------------------------------------------------------
//! VALIDATION ORDER
//! -----------------------------------------------------------------------------
//!
//! The canonical order is the order of `ResourceKind` dimensions exposed by
//! the limits/resource architecture:
//!
//!     LogicalQubits
//!     PhysicalQubits
//!     Operations
//!     CircuitDepth
//!     Shots
//!     RecoveryAttempts
//!     MitigationExecutions
//!     Checkpoints
//!     StorageBytes
//!     CpuUnits
//!     GpuUnits
//!     NetworkBytes
//!     TelemetryEvents
//!     ConcurrentExecutions
//!     ExecutionTimeNanos
//!     CompilationTimeNanos
//!     QueueTimeNanos
//!     RecoveryTimeNanos
//!     CostUnits
//!     EnergyUnits
//!
//! Keeping the order deterministic is important for reproducible planning,
//! testing, logging, and provenance.
//!
//! -----------------------------------------------------------------------------
//! UNKNOWN CAPACITY
//! -----------------------------------------------------------------------------
//!
//! Unknown capacity is a first-class state.
//!
//! For safety-critical execution:
//!
//!     Known sufficient       -> valid
//!     Known insufficient     -> invalid
//!     Unknown                -> cannot prove feasibility
//!
//! The caller may subsequently acquire better capability information and run
//! validation again.
//!
//! Validation therefore never converts:
//!
//!     Unknown -> Unlimited
//!     Unknown -> zero
//!
//! -----------------------------------------------------------------------------
//! NO RESOURCE RESERVATION
//! -----------------------------------------------------------------------------
//!
//! Successful validation does NOT reserve resources.
//!
//! Between validation and execution, another workload may consume resources.
//!
//! Actual reservation/coordination belongs to:
//!
//!     quantum::resilience::coordination
//!     runtime/execution
//!     quantum::hardware
//!
//! A caller requiring atomic "validate + reserve" semantics must perform that
//! operation in the appropriate resource coordinator, not in this pure module.
//!
//! -----------------------------------------------------------------------------
//! RUST / SAFETY
//! -----------------------------------------------------------------------------
//!
//! Designed for:
//!
//!     Rust 2021
//!     Rust 1.97 / 1.97.1
//!
//! No `unsafe`.
//!
//! No external dependency is required by this file beyond the sibling
//! resilience resource/limit modules.
//!

use core::fmt;

use super::limits::{LimitSet, ResourceKind, ResourceViolation};
use super::resource::{
    ResourceAssessment,
    ResourceAvailability,
    ResourceDemand,
    ResourceValidation,
    ResourceValue,
    assess_resource,
    validate,
    validate_resource,
};

/// Result of validating one resource dimension.
///
/// This is intentionally richer than a boolean so callers can distinguish:
///
/// - success;
/// - an explicit policy/resilience limit;
/// - known target/resource exhaustion;
/// - insufficient information.
///
/// The underlying resource semantics remain owned by `resource.rs`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationStatus {
    /// The requested resource is permitted and known to be available.
    Satisfied,

    /// The request exceeds an explicit resilience limit.
    LimitExceeded {
        /// Resource dimension.
        resource: ResourceKind,

        /// Requested quantity.
        requested: u64,

        /// Configured limit.
        limit: u64,
    },

    /// The target/runtime is known not to have enough capacity.
    ResourceUnavailable {
        /// Resource dimension.
        resource: ResourceKind,

        /// Requested quantity.
        requested: u64,

        /// Known available quantity.
        available: u64,
    },

    /// Capacity is not sufficiently known to prove feasibility.
    ResourceUnknown {
        /// Resource dimension.
        resource: ResourceKind,

        /// Requested quantity.
        requested: u64,
    },
}

impl ValidationStatus {
    /// Returns `true` only for a fully validated resource.
    #[must_use]
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::Satisfied)
    }

    /// Returns `true` when the request exceeds an explicit limit.
    #[must_use]
    pub const fn is_limit_exceeded(self) -> bool {
        matches!(self, Self::LimitExceeded { .. })
    }

    /// Returns `true` when known capacity is insufficient.
    #[must_use]
    pub const fn is_resource_unavailable(self) -> bool {
        matches!(self, Self::ResourceUnavailable { .. })
    }

    /// Returns `true` when more information is required.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::ResourceUnknown { .. })
    }

    /// Converts the status into the canonical resource validation type.
    #[must_use]
    pub const fn as_resource_validation(self) -> ResourceValidation {
        match self {
            Self::Satisfied => ResourceValidation::Satisfied,

            Self::LimitExceeded {
                resource,
                requested,
                limit,
            } => ResourceValidation::LimitExceeded(
                super::limits::LimitViolation::new(
                    resource,
                    requested,
                    limit,
                ),
            ),

            Self::ResourceUnavailable {
                resource,
                requested,
                available,
            } => ResourceValidation::ResourceUnavailable {
                resource,
                requested,
                available,
            },

            Self::ResourceUnknown {
                resource,
                requested,
            } => ResourceValidation::ResourceUnknown {
                resource,
                requested,
            },
        }
    }
}

impl fmt::Display for ValidationStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Satisfied => formatter.write_str("satisfied"),

            Self::LimitExceeded {
                resource,
                requested,
                limit,
            } => write!(
                formatter,
                "resource '{}' requested {} but resilience limit is {}",
                resource,
                requested,
                limit
            ),

            Self::ResourceUnavailable {
                resource,
                requested,
                available,
            } => write!(
                formatter,
                "resource '{}' requested {} but only {} is available",
                resource,
                requested,
                available
            ),

            Self::ResourceUnknown {
                resource,
                requested,
            } => write!(
                formatter,
                "resource '{}' requested {} but availability is unknown",
                resource,
                requested
            ),
        }
    }
}

/// A validation issue containing its resource dimension and status.
///
/// The type is `Copy` and allocation-free so callers can inspect validation
/// failures without requiring heap storage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationIssue {
    /// Resource dimension associated with the issue.
    pub resource: ResourceKind,

    /// Requested quantity.
    pub requested: u64,

    /// Result for this dimension.
    pub status: ValidationStatus,
}

impl ValidationIssue {
    /// Returns whether this issue represents success.
    #[must_use]
    pub const fn is_satisfied(self) -> bool {
        self.status.is_satisfied()
    }

    /// Returns whether this issue is an explicit limit failure.
    #[must_use]
    pub const fn is_limit_exceeded(self) -> bool {
        self.status.is_limit_exceeded()
    }

    /// Returns whether this issue is a known resource shortage.
    #[must_use]
    pub const fn is_resource_unavailable(self) -> bool {
        self.status.is_resource_unavailable()
    }

    /// Returns whether this issue requires additional information.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        self.status.is_unknown()
    }
}

impl fmt::Display for ValidationIssue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.status.fmt(formatter)
    }
}

/// The overall validation state.
///
/// `Valid` means every requested resource dimension passed both:
///
/// 1. explicit resilience limits; and
/// 2. known resource availability.
///
/// `Invalid` means at least one dimension is definitively impossible.
///
/// `Indeterminate` means no definitive failure was found, but one or more
/// required dimensions have unknown availability.
///
/// This distinction is important for fail-closed quantum execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationState {
    /// All dimensions are proven valid.
    Valid,

    /// At least one dimension is definitively invalid.
    Invalid,

    /// No definitive violation was found, but feasibility cannot yet be
    /// proven because required availability is unknown.
    Indeterminate,
}

impl ValidationState {
    /// Returns whether validation is fully successful.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        matches!(self, Self::Valid)
    }

    /// Returns whether a definitive validation failure exists.
    #[must_use]
    pub const fn is_invalid(self) -> bool {
        matches!(self, Self::Invalid)
    }

    /// Returns whether more information is required.
    #[must_use]
    pub const fn is_indeterminate(self) -> bool {
        matches!(self, Self::Indeterminate)
    }

    /// Returns whether execution must not proceed under a fail-closed policy.
    #[must_use]
    pub const fn requires_information(self) -> bool {
        matches!(self, Self::Indeterminate)
    }
}

impl fmt::Display for ValidationState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Valid => formatter.write_str("valid"),
            Self::Invalid => formatter.write_str("invalid"),
            Self::Indeterminate => formatter.write_str("indeterminate"),
        }
    }
}

/// A complete, deterministic validation report.
///
/// The report contains only aggregate information and the first deterministic
/// issue. It does not allocate, making it appropriate for large systems where
/// the number of resource dimensions is fixed while machine size is not.
///
/// `checked_dimensions` is the number of resource dimensions evaluated before
/// completion.
///
/// `unknown_dimensions` records dimensions whose availability could not be
/// established.
///
/// `first_issue` contains the first deterministic non-success result.
///
/// A caller that needs every individual issue should use `ValidationReport::iter`
/// and process the returned iterator without requiring a heap allocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationReport {
    /// Overall validation state.
    pub state: ValidationState,

    /// Number of resource dimensions checked.
    pub checked_dimensions: u8,

    /// Number of dimensions whose availability is unknown.
    pub unknown_dimensions: u8,

    /// First deterministic issue, if any.
    pub first_issue: Option<ValidationIssue>,
}

impl ValidationReport {
    /// Creates an empty successful report.
    #[must_use]
    pub const fn valid() -> Self {
        Self {
            state: ValidationState::Valid,
            checked_dimensions: 0,
            unknown_dimensions: 0,
            first_issue: None,
        }
    }

    /// Returns whether the complete validation succeeded.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.state.is_valid()
    }

    /// Returns whether a definitive failure exists.
    #[must_use]
    pub const fn is_invalid(self) -> bool {
        self.state.is_invalid()
    }

    /// Returns whether more information is required.
    #[must_use]
    pub const fn is_indeterminate(self) -> bool {
        self.state.is_indeterminate()
    }

    /// Returns the first validation issue.
    #[must_use]
    pub const fn first_issue(self) -> Option<ValidationIssue> {
        self.first_issue
    }

    /// Returns the number of dimensions with unknown availability.
    #[must_use]
    pub const fn unknown_count(self) -> u8 {
        self.unknown_dimensions
    }

    /// Returns whether at least one required dimension is unknown.
    #[must_use]
    pub const fn has_unknown(self) -> bool {
        self.unknown_dimensions != 0
    }

    /// Returns an iterator over the validation result of every resource
    /// dimension.
    ///
    /// The iterator is allocation-free and deterministic.
    #[must_use]
    pub const fn iter(
        self,
        demand: ResourceDemand,
        availability: ResourceAvailability,
        limits: LimitSet,
    ) -> ValidationIterator {
        ValidationIterator {
            demand,
            availability,
            limits,
            index: 0,
        }
    }

    /// Returns the first definitive validation failure.
    ///
    /// Unknown capacity is not returned here because it is not a definitive
    /// failure; it is an indeterminate state.
    #[must_use]
    pub const fn definitive_failure(self) -> Option<ValidationIssue> {
        match self.first_issue {
            Some(issue) if issue.status.is_limit_exceeded()
                || issue.status.is_resource_unavailable() =>
            {
                Some(issue)
            }
            _ => None,
        }
    }

    /// Converts the report to a fail-closed result.
    ///
    /// `Ok(())` is returned only when every required resource is proven
    /// available and all explicit limits are satisfied.
    ///
    /// `Err(...)` is returned for both definitive failures and unknown
    /// availability.
    pub const fn require_valid(self) -> Result<(), ValidationIssue> {
        match self.state {
            ValidationState::Valid => Ok(()),
            ValidationState::Invalid | ValidationState::Indeterminate => {
                match self.first_issue {
                    Some(issue) => Err(issue),
                    None => Err(ValidationIssue {
                        resource: ResourceKind::Generic,
                        requested: 0,
                        status: ValidationStatus::ResourceUnknown {
                            resource: ResourceKind::Generic,
                            requested: 0,
                        },
                    }),
                }
            }
        }
    }
}

impl fmt::Display for ValidationReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.first_issue {
            Some(issue) => write!(
                formatter,
                "resource validation: state={}, checked={}, unknown={}, first_issue={}",
                self.state,
                self.checked_dimensions,
                self.unknown_dimensions,
                issue
            ),
            None => write!(
                formatter,
                "resource validation: state={}, checked={}, unknown={}",
                self.state,
                self.checked_dimensions,
                self.unknown_dimensions
            ),
        }
    }
}

/// Allocation-free deterministic iterator over every resource validation
/// dimension.
///
/// This iterator exists so callers can inspect all dimensions without creating
/// a `Vec`, fixed-size array, or machine-size-dependent collection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationIterator {
    demand: ResourceDemand,
    availability: ResourceAvailability,
    limits: LimitSet,
    index: u8,
}

impl Iterator for ValidationIterator {
    type Item = ValidationIssue;

    fn next(&mut self) -> Option<Self::Item> {
        let resource = match self.index {
            0 => ResourceKind::LogicalQubits,
            1 => ResourceKind::PhysicalQubits,
            2 => ResourceKind::Operations,
            3 => ResourceKind::CircuitDepth,
            4 => ResourceKind::Shots,
            5 => ResourceKind::RecoveryAttempts,
            6 => ResourceKind::MitigationExecutions,
            7 => ResourceKind::Checkpoints,
            8 => ResourceKind::StorageBytes,
            9 => ResourceKind::CpuUnits,
            10 => ResourceKind::GpuUnits,
            11 => ResourceKind::NetworkBytes,
            12 => ResourceKind::TelemetryEvents,
            13 => ResourceKind::ConcurrentExecutions,
            14 => ResourceKind::ExecutionTimeNanos,
            15 => ResourceKind::CompilationTimeNanos,
            16 => ResourceKind::QueueTimeNanos,
            17 => ResourceKind::RecoveryTimeNanos,
            18 => ResourceKind::CostUnits,
            19 => ResourceKind::EnergyUnits,
            _ => return None,
        };

        self.index = self.index.saturating_add(1);

        let requested = self.demand.get(resource);
        let availability = self.availability.get(resource);

        Some(ValidationIssue {
            resource,
            requested,
            status: classify_validation(
                resource,
                requested,
                availability,
                self.limits,
            ),
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = 20usize.saturating_sub(self.index as usize);
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for ValidationIterator {}

/// Performs validation for one resource dimension.
///
/// Explicit resilience limits are checked first. If the request is within the
/// configured limit, current availability is checked.
///
/// Unknown availability remains unknown.
#[must_use]
pub const fn validate_dimension(
    resource: ResourceKind,
    requested: u64,
    availability: ResourceValue,
    limits: LimitSet,
) -> ValidationStatus {
    classify_validation(resource, requested, availability, limits)
}

/// Internal classification function.
///
/// This deliberately delegates the underlying semantics to the canonical
/// `LimitSet` and resource validation primitives rather than creating a second
/// resource-validation implementation.
const fn classify_validation(
    resource: ResourceKind,
    requested: u64,
    availability: ResourceValue,
    limits: LimitSet,
) -> ValidationStatus {
    match limits.check(resource, requested) {
        Ok(()) => match assess_resource(availability, requested) {
            ResourceAssessment::Satisfied => ValidationStatus::Satisfied,

            ResourceAssessment::Insufficient {
                requested,
                available,
            } => ValidationStatus::ResourceUnavailable {
                resource,
                requested,
                available,
            },

            ResourceAssessment::Unknown => ValidationStatus::ResourceUnknown {
                resource,
                requested,
            },
        },

        Err(violation) => ValidationStatus::LimitExceeded {
            resource: violation.resource,
            requested: violation.requested,
            limit: match violation.limit {
                Some(value) => value,
                None => 0,
            },
        },
    }
}

/// Builds a complete allocation-free validation report.
///
/// The function checks all resource dimensions rather than stopping at the
/// first failure. The report stores only the first deterministic issue and
/// aggregate unknown information.
///
/// This is useful for production diagnostics while remaining independent of
/// machine size.
#[must_use]
pub fn validate_report(
    demand: ResourceDemand,
    availability: ResourceAvailability,
    limits: LimitSet,
) -> ValidationReport {
    let mut report = ValidationReport::valid();

    for issue in ValidationIterator {
        demand,
        availability,
        limits,
        index: 0,
    } {
        report.checked_dimensions =
            report.checked_dimensions.saturating_add(1);

        if issue.status.is_unknown() {
            report.unknown_dimensions =
                report.unknown_dimensions.saturating_add(1);
        }

        if !issue.is_satisfied() && report.first_issue.is_none() {
            report.first_issue = Some(issue);

            if issue.status.is_limit_exceeded()
                || issue.status.is_resource_unavailable()
            {
                report.state = ValidationState::Invalid;
            } else {
                report.state = ValidationState::Indeterminate;
            }
        }
    }

    // A later definitive failure must take precedence over an earlier
    // unknown result. The report iterator is deterministic, so we can perform
    // one final pass only when necessary.
    if matches!(report.state, ValidationState::Indeterminate) {
        for issue in ValidationIterator {
            demand,
            availability,
            limits,
            index: 0,
        } {
            if issue.status.is_limit_exceeded()
                || issue.status.is_resource_unavailable()
            {
                report.state = ValidationState::Invalid;
                report.first_issue = Some(issue);
                break;
            }
        }
    }

    report
}

/// Performs fail-closed validation.
///
/// This is the primary entry point for safety-sensitive execution paths.
///
/// It returns `Ok(())` only if:
///
/// - every explicit resilience limit is satisfied; and
/// - every required resource is known to be available.
///
/// Unknown capacity is rejected.
pub fn validate_request(
    demand: ResourceDemand,
    availability: ResourceAvailability,
    limits: LimitSet,
) -> Result<(), ValidationIssue> {
    validate_report(demand, availability, limits).require_valid()
}

/// Performs strict validation using the canonical resource-layer result.
///
/// This is useful when callers already operate with `ResourceValidation`.
#[must_use]
pub const fn validate_request_canonical(
    demand: ResourceDemand,
    availability: ResourceAvailability,
    limits: LimitSet,
) -> ResourceValidation {
    validate(demand, availability, limits)
}

/// Validates only the explicit resilience limits.
///
/// This does not inspect target availability.
///
/// It is appropriate when a planner wants to determine whether a proposed
/// action violates policy before target selection occurs.
#[must_use]
pub const fn validate_limits_only(
    demand: ResourceDemand,
    limits: LimitSet,
) -> Result<(), super::limits::LimitViolation> {
    demand.check_limits(limits)
}

/// Validates one resource against an explicit resilience limit and current
/// availability.
///
/// This is the smallest public validation primitive.
#[must_use]
pub const fn validate_one(
    resource: ResourceKind,
    requested: u64,
    availability: ResourceValue,
    limits: LimitSet,
) -> ValidationStatus {
    validate_dimension(
        resource,
        requested,
        availability,
        limits,
    )
}

/// Determines whether a demand is fully feasible.
///
/// This is deliberately equivalent to strict validation and therefore returns
/// `false` for unknown availability.
#[must_use]
pub const fn is_feasible(
    demand: ResourceDemand,
    availability: ResourceAvailability,
    limits: LimitSet,
) -> bool {
    match validate(
        demand,
        availability,
        limits,
    ) {
        ResourceValidation::Satisfied => true,
        ResourceValidation::LimitExceeded(_)
        | ResourceValidation::ResourceUnavailable { .. }
        | ResourceValidation::ResourceUnknown { .. } => false,
    }
}

/// Determines whether a demand is within resilience limits without requiring
/// target availability to be known.
///
/// This distinction is important for planning before backend selection.
#[must_use]
pub const fn fits_limits(
    demand: ResourceDemand,
    limits: LimitSet,
) -> bool {
    demand.fits_within(limits)
}

/// Determines whether the supplied availability is sufficient for the demand.
///
/// Unknown capacity returns `false`.
#[must_use]
pub const fn fits_availability(
    demand: ResourceDemand,
    availability: ResourceAvailability,
) -> bool {
    availability.can_supply(demand)
}

/// Returns the canonical resource assessment for one dimension.
#[must_use]
pub const fn assess(
    resource: ResourceKind,
    requested: u64,
    availability: ResourceAvailability,
) -> ResourceAssessment {
    assess_resource(
        availability.get(resource),
        requested,
    )
}

/// Validates that a resource reservation can be created without actually
/// reserving anything.
///
/// Actual reservation remains the responsibility of the resource coordinator.
#[must_use]
pub const fn can_reserve(
    demand: ResourceDemand,
    availability: ResourceAvailability,
    limits: LimitSet,
) -> bool {
    if !fits_limits(demand, limits) {
        return false;
    }

    availability.can_supply(demand)
}

/// Computes the availability that would remain after a successful reservation.
///
/// This is a pure planning operation. It does not mutate or reserve anything
/// externally.
///
/// `None` means either:
///
/// - capacity is insufficient; or
/// - availability arithmetic would overflow/underflow.
///
/// Unknown availability remains unknown through the canonical resource model.
pub const fn projected_after_reservation(
    demand: ResourceDemand,
    availability: ResourceAvailability,
    limits: LimitSet,
) -> Option<ResourceAvailability> {
    if !fits_limits(demand, limits) {
        return None;
    }

    availability.reserve(demand)
}

/// Computes the availability after releasing a reservation.
///
/// This does not perform actual coordination or locking.
pub const fn projected_after_release(
    demand: ResourceDemand,
    availability: ResourceAvailability,
) -> Option<ResourceAvailability> {
    availability.release(demand)
}

/// Validates that two independently supplied limit sets can be safely
/// intersected.
///
/// Every `LimitSet` is structurally valid by construction, so this function
/// exists primarily as an explicit integration point for future validation
/// layers.
///
/// It is intentionally infallible today and contains no hidden assumptions.
#[must_use]
pub const fn validate_limit_set(_limits: LimitSet) -> bool {
    true
}

/// Validates that an availability vector is structurally usable.
///
/// `Unknown` is valid state and therefore does not make the vector malformed.
///
/// A resource value of zero is also valid.
///
/// This function checks representation validity only; it does not determine
/// whether a particular workload can execute.
#[must_use]
pub const fn validate_availability(
    _availability: ResourceAvailability,
) -> bool {
    true
}

/// Validates a demand vector's representation.
///
/// All quantities are unsigned and therefore non-negative. The resource model
/// owns arithmetic overflow checks when demands are composed.
///
/// Consequently every directly constructed `ResourceDemand` is structurally
/// valid.
#[must_use]
pub const fn validate_demand(
    _demand: ResourceDemand,
) -> bool {
    true
}

/// Validates a complete request and returns its canonical resource-layer
/// result.
///
/// This function is useful as the stable adapter boundary for callers that
/// should not need to know how the resource validation implementation is
/// organized internally.
#[must_use]
pub const fn validate_complete(
    demand: ResourceDemand,
    availability: ResourceAvailability,
    limits: LimitSet,
) -> ResourceValidation {
    validate(
        demand,
        availability,
        limits,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unlimited_limits_do_not_create_artificial_ceiling() {
        let demand = ResourceDemand::single(
            ResourceKind::LogicalQubits,
            u64::MAX,
        );

        let limits = LimitSet::unlimited();

        assert!(fits_limits(demand, limits));
    }

    #[test]
    fn_zero_limit_rejects_non_zero_demand() {
        let mut limits = LimitSet::unlimited();
        limits.logical_qubits = super::super::limits::Limit::bounded(0);

        let demand = ResourceDemand::single(
            ResourceKind::LogicalQubits,
            1,
        );

        assert!(!fits_limits(demand, limits));

        let result = validate_limits_only(demand, limits);

        assert!(result.is_err());
    }

    #[test]
    fn_unknown_capacity_fails_closed() {
        let demand = ResourceDemand::single(
            ResourceKind::LogicalQubits,
            1,
        );

        let availability = ResourceAvailability::unknown();
        let limits = LimitSet::unlimited();

        assert!(!is_feasible(
            demand,
            availability,
            limits,
        ));

        let report = validate_report(
            demand,
            availability,
            limits,
        );

        assert!(report.is_indeterminate());
        assert!(report.has_unknown());
    }

    #[test]
    fn_known_insufficient_capacity_is_invalid() {
        let demand = ResourceDemand::single(
            ResourceKind::LogicalQubits,
            8,
        );

        let mut availability = ResourceAvailability::empty();
        availability.logical_qubits =
            ResourceValue::Known(4);

        let limits = LimitSet::unlimited();

        let report = validate_report(
            demand,
            availability,
            limits,
        );

        assert!(report.is_invalid());

        let issue = report.first_issue().expect(
            "expected deterministic validation issue",
        );

        assert_eq!(
            issue.resource,
            ResourceKind::LogicalQubits
        );

        assert!(issue.status.is_resource_unavailable());
    }

    #[test]
    fn explicit_limit_is_checked_before_availability() {
        let demand = ResourceDemand::single(
            ResourceKind::LogicalQubits,
            8,
        );

        let mut availability = ResourceAvailability::empty();
        availability.logical_qubits =
            ResourceValue::Known(4);

        let mut limits = LimitSet::unlimited();
        limits.logical_qubits =
            super::super::limits::Limit::bounded(2);

        let result = validate_one(
            ResourceKind::LogicalQubits,
            8,
            availability.logical_qubits,
            limits,
        );

        assert!(result.is_limit_exceeded());
    }

    #[test]
    fn exact_capacity_is_valid() {
        let demand = ResourceDemand::single(
            ResourceKind::PhysicalQubits,
            16,
        );

        let mut availability = ResourceAvailability::empty();
        availability.physical_qubits =
            ResourceValue::Known(16);

        let limits = LimitSet::unlimited();

        assert!(is_feasible(
            demand,
            availability,
            limits,
        ));
    }

    #[test]
    fn zero_demand_is_valid_when_capacity_is_known() {
        let demand = ResourceDemand::zero();

        let availability = ResourceAvailability::empty();
        let limits = LimitSet::unlimited();

        assert!(is_feasible(
            demand,
            availability,
            limits,
        ));
    }

    #[test]
    fn zero_demand_still_requires_known_dimensions() {
        let demand = ResourceDemand::zero();

        let availability = ResourceAvailability::unknown();
        let limits = LimitSet::unlimited();

        // The validation model is fail-closed for the complete resource
        // vector. Callers that know a particular dimension is irrelevant
        // should construct a scoped demand/availability model upstream.
        assert!(!is_feasible(
            demand,
            availability,
            limits,
        ));
    }

    #[test]
    fn validation_iterator_is_deterministic() {
        let demand = ResourceDemand::single(
            ResourceKind::Operations,
            10,
        );

        let availability = ResourceAvailability::unknown();
        let limits = LimitSet::unlimited();

        let first: Vec<_> = ValidationIterator {
            demand,
            availability,
            limits,
            index: 0,
        }
        .collect();

        let second: Vec<_> = ValidationIterator {
            demand,
            availability,
            limits,
            index: 0,
        }
        .collect();

        assert_eq!(first, second);
        assert_eq!(first.len(), 20);
    }

    #[test]
    fn projected_reservation_does_not_mutate_original() {
        let demand = ResourceDemand::single(
            ResourceKind::LogicalQubits,
            4,
        );

        let mut availability = ResourceAvailability::empty();
        availability.logical_qubits =
            ResourceValue::Known(8);

        let limits = LimitSet::unlimited();

        let projected = projected_after_reservation(
            demand,
            availability,
            limits,
        )
        .expect("reservation should be feasible");

        assert_eq!(
            availability.logical_qubits,
            ResourceValue::Known(8)
        );

        assert_eq!(
            projected.logical_qubits,
            ResourceValue::Known(4)
        );
    }

    #[test]
    fn overflow_is_rejected_when_combining_demands() {
        let first = ResourceDemand::single(
            ResourceKind::Operations,
            u64::MAX,
        );

        let second = ResourceDemand::single(
            ResourceKind::Operations,
            1,
        );

        assert!(first.checked_add(second).is_none());
    }

    #[test]
    fn limit_only_validation_does_not_require_hardware_information() {
        let demand = ResourceDemand::single(
            ResourceKind::LogicalQubits,
            1024,
        );

        let limits = LimitSet::unlimited();

        assert!(fits_limits(
            demand,
            limits,
        ));
    }

    #[test]
    fn strict_validation_rejects_unknown_capacity() {
        let demand = ResourceDemand::single(
            ResourceKind::PhysicalQubits,
            1,
        );

        let availability = ResourceAvailability::unknown();
        let limits = LimitSet::unlimited();

        assert!(validate_request(
            demand,
            availability,
            limits,
        ).is_err());
    }

    #[test]
    fn report_counts_all_dimensions() {
        let report = validate_report(
            ResourceDemand::zero(),
            ResourceAvailability::empty(),
            LimitSet::unlimited(),
        );

        assert_eq!(
            report.checked_dimensions,
            20
        );

        assert_eq!(
            report.unknown_dimensions,
            0
        );

        assert!(report.is_valid());
    }

    #[test]
    fn unknown_is_not_unlimited() {
        let availability =
            ResourceAvailability::unknown();

        assert!(!availability.can_supply(
            ResourceDemand::single(
                ResourceKind::LogicalQubits,
                u64::MAX,
            )
        ));
    }
}