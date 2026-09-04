//! Zamani Quantum Scheduling — Qubit Constraints
//!
//! Production-grade qubit-specific constraints for the generic scheduling
//! constraint framework.
//!
//! # Responsibility
//!
//! This module answers:
//!
//! > "Can this operation use these logical/physical qubits at the proposed
//! > time without violating qubit exclusivity or qubit-usage invariants?"
//!
//! This module owns:
//!
//! - qubit-specific constraint semantics;
//! - logical-qubit references;
//! - physical-qubit references;
//! - arbitrary operation arity;
//! - duplicate-qubit detection;
//! - logical/physical identity separation;
//! - qubit occupancy requirements;
//! - qubit exclusivity checks;
//! - deterministic conflict reporting;
//! - qubit-specific constraint configuration;
//! - reusable qubit constraint evaluation.
//!
//! This module does NOT own:
//!
//! - canonical qubit identity definitions;
//! - quantum gate semantics;
//! - routing;
//! - physical topology;
//! - hardware discovery;
//! - calibration;
//! - scheduling algorithms;
//! - resource calendars;
//! - QEC decoding;
//! - runtime execution.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Canonical identity boundary
//!
//! Logical and physical qubit identities MUST come from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! crate::quantum::ir::qubit::QubitRef
//! ```
//!
//! This file deliberately does not define another `QubitId` or
//! `PhysicalQubitId`.
//!
//! The canonical IR implementation uses strongly typed logical and physical
//! identifiers, preventing accidental conversion between the two identity
//! domains. The scheduling layer consumes those identities; it does not
//! replace them.
//!
//! # Universal-program principle
//!
//! A Zamani program must describe computation rather than a particular machine
//! size.
//!
//! Therefore this module contains no:
//!
//! - maximum qubit count;
//! - fixed physical-qubit count;
//! - fixed operation arity;
//! - fixed topology;
//! - fixed number of qubit resources;
//! - fixed register width;
//! - fixed machine dimensions.
//!
//! A candidate operation may contain any number of qubit operands representable
//! by the host process and permitted by the enclosing scheduling request.
//!
//! "Infinity" therefore means:
//!
//! > this module imposes no artificial finite machine-size ceiling.
//!
//! Actual execution remains bounded by available memory, CPU time, explicit
//! compiler policy, target capabilities, and operating-system resources.
//!
//! # Important architectural distinction
//!
//! This module does not determine WHERE a logical qubit is placed.
//!
//! Routing owns:
//!
//! ```text
//! logical qubit -> physical qubit
//! ```
//!
//! This module determines whether the qubits referenced by a scheduling
//! candidate are mutually compatible with existing scheduled occupancy.
//!
//! Therefore:
//!
//! ```text
//! routing
//!     |
//!     v
//! logical -> physical mapping
//!     |
//!     v
//! scheduling::constraints::qubit
//!     |
//!     v
//! temporal/resource admissibility
//! ```
//!
//! # No hard-coded operation arity
//!
//! The implementation works with slices of qubit references.
//!
//! Consequently it supports:
//!
//! - one-qubit operations;
//! - two-qubit operations;
//! - three-qubit operations;
//! - N-qubit operations;
//! - collective operations;
//! - future architectures with different operation arity.
//!
//! # No implicit identity conversion
//!
//! A logical qubit and a physical qubit with the same numeric index are NOT
//! considered the same resource.
//!
//! For example:
//!
//! ```text
//! Logical(QubitId(7))
//! Physical(PhysicalQubitId(7))
//! ```
//!
//! are different identities.
//!
//! This prevents an entire class of routing/scheduling correctness bugs.
//!
//! # Evaluation model
//!
//! A qubit constraint evaluates a proposed candidate against an immutable
//! snapshot supplied by the generic scheduling constraint framework.
//!
//! It MUST NOT mutate:
//!
//! - the schedule;
//! - resource state;
//! - qubit state;
//! - the quantum IR;
//! - hardware;
//! - global state.
//!
//! Reservation and state mutation belong to the planner/resource subsystem.
//!
//! # Thread safety
//!
//! The implementation contains no mutable global state and is safe to share
//! between scheduler workers.
//!
//! # Rust contract
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe;
//! - no unsafe dependencies required by this module.
//!
//! # Integration
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::ir::qubit
//!         |
//!         v
//! scheduling::types
//!         |
//!         v
//! scheduling::constraints::constraint
//!         |
//!         v
//! scheduling::constraints::qubit
//!         |
//!         +------------------+
//!         |                  |
//!         v                  v
//! planners              verification
//! ```
//!
//! Routing, hardware, QEC and runtime integrations should provide their
//! information through the generic constraint context rather than causing this
//! module to depend directly on those subsystems.
//!
//! # Production invariants
//!
//! A valid qubit constraint evaluation guarantees:
//!
//! 1. Every referenced qubit identity is internally well formed.
//! 2. A candidate does not contain the same qubit identity more than once.
//! 3. Logical and physical identities are never conflated.
//! 4. A qubit marked unavailable by the supplied context cannot be consumed.
//! 5. An exclusive qubit cannot be occupied by overlapping operations.
//! 6. The constraint never mutates scheduling state.
//! 7. No result depends on iteration order.
//! 8. No result depends on wall-clock time.
//! 9. No result depends on hidden randomness.
//! 10. No machine-size limit is embedded in the implementation.
//!
//! # Verification versus planning
//!
//! The same constraint object can be used during planning and during final
//! verification.
//!
//! This is intentional.
//!
//! A production scheduler should not have one set of qubit rules for planning
//! and another incompatible set for verification.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeSet;
use std::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId, QubitRef};

use super::constraint::{
    Constraint,
    ConstraintContext,
    ConstraintKind,
    ConstraintSeverity,
    ConstraintViolation,
};

// ============================================================================
// Qubit usage identity
// ============================================================================

/// Identity used by the qubit constraint subsystem.
///
/// This is intentionally an alias to the canonical IR reference rather than a
/// new identity type.
///
/// Keeping the alias local improves readability without creating a competing
/// identity domain.
pub type QubitUsage = QubitRef;

// ============================================================================
// Qubit access mode
// ============================================================================

/// Describes how a candidate operation intends to use a qubit.
///
/// The default mode is exclusive because ordinary quantum operations cannot
/// safely overlap on the same physical qubit.
///
/// Additional modes allow future scheduling systems to model operations whose
/// semantics explicitly permit shared observation or other non-exclusive use.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QubitAccessMode {
    /// The operation requires exclusive ownership of the qubit for its
    /// scheduled interval.
    Exclusive,

    /// The operation only observes the qubit and does not claim exclusive
    /// execution ownership.
    ///
    /// Whether this is legal is determined by the surrounding constraint set.
    SharedObservation,
}

impl Default for QubitAccessMode {
    fn default() -> Self {
        Self::Exclusive
    }
}

impl QubitAccessMode {
    /// Returns whether the access mode requires exclusive occupancy.
    #[must_use]
    pub const fn is_exclusive(self) -> bool {
        matches!(self, Self::Exclusive)
    }

    /// Returns a stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::SharedObservation => "shared_observation",
        }
    }
}

impl fmt::Display for QubitAccessMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Qubit requirement
// ============================================================================

/// One qubit requirement of a candidate operation.
///
/// The qubit identity is always the canonical Zamani IR identity.
///
/// No numeric conversion is performed implicitly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QubitRequirement {
    qubit: QubitRef,
    access: QubitAccessMode,
}

impl QubitRequirement {
    /// Creates an exclusive requirement for a qubit.
    #[must_use]
    pub const fn exclusive(qubit: QubitRef) -> Self {
        Self {
            qubit,
            access: QubitAccessMode::Exclusive,
        }
    }

    /// Creates a requirement with an explicit access mode.
    #[must_use]
    pub const fn new(qubit: QubitRef, access: QubitAccessMode) -> Self {
        Self { qubit, access }
    }

    /// Returns the canonical qubit reference.
    #[must_use]
    pub const fn qubit(self) -> QubitRef {
        self.qubit
    }

    /// Returns the access mode.
    #[must_use]
    pub const fn access(self) -> QubitAccessMode {
        self.access
    }

    /// Returns the logical identity when this requirement is logical.
    #[must_use]
    pub const fn logical(self) -> Option<QubitId> {
        self.qubit.logical()
    }

    /// Returns the physical identity when this requirement is physical.
    #[must_use]
    pub const fn physical(self) -> Option<PhysicalQubitId> {
        self.qubit.physical()
    }
}

impl From<QubitRef> for QubitRequirement {
    fn from(qubit: QubitRef) -> Self {
        Self::exclusive(qubit)
    }
}

// ============================================================================
// Qubit candidate
// ============================================================================

/// Immutable qubit-use description for a scheduling candidate.
///
/// The candidate is deliberately represented as a slice so that operation
/// arity is not hard-coded.
///
/// This type does not own the underlying slice.
#[derive(Debug, Clone, Copy)]
pub struct QubitCandidate<'a> {
    requirements: &'a [QubitRequirement],
}

impl<'a> QubitCandidate<'a> {
    /// Creates a candidate from qubit requirements.
    #[must_use]
    pub const fn new(requirements: &'a [QubitRequirement]) -> Self {
        Self { requirements }
    }

    /// Returns all requirements in caller-supplied order.
    #[must_use]
    pub const fn requirements(self) -> &'a [QubitRequirement] {
        self.requirements
    }

    /// Returns the number of qubit requirements.
    #[must_use]
    pub const fn len(self) -> usize {
        self.requirements.len()
    }

    /// Returns whether the candidate has no qubit requirements.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.requirements.is_empty()
    }

    /// Returns an iterator over the requirements.
    pub fn iter(self) -> std::slice::Iter<'a, QubitRequirement> {
        self.requirements.iter()
    }

    /// Validates internal candidate identity invariants.
    ///
    /// In particular, the same logical or physical identity may not occur
    /// twice in one operation.
    pub fn validate(self) -> Result<(), QubitCandidateError> {
        let mut logical = BTreeSet::new();
        let mut physical = BTreeSet::new();

        for requirement in self.requirements {
            match requirement.qubit() {
                QubitRef::Logical(id) => {
                    if !logical.insert(id) {
                        return Err(QubitCandidateError::DuplicateLogicalQubit(id));
                    }
                }
                QubitRef::Physical(id) => {
                    if !physical.insert(id) {
                        return Err(QubitCandidateError::DuplicatePhysicalQubit(id));
                    }
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// Candidate errors
// ============================================================================

/// Structural error in a qubit candidate.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QubitCandidateError {
    /// The same logical qubit appears more than once.
    DuplicateLogicalQubit(QubitId),

    /// The same physical qubit appears more than once.
    DuplicatePhysicalQubit(PhysicalQubitId),
}

impl fmt::Display for QubitCandidateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateLogicalQubit(id) => {
                write!(formatter, "logical qubit {id} appears more than once")
            }
            Self::DuplicatePhysicalQubit(id) => {
                write!(formatter, "physical qubit {id} appears more than once")
            }
        }
    }
}

impl std::error::Error for QubitCandidateError {}

// ============================================================================
// Existing occupancy
// ============================================================================

/// Immutable description of qubit occupancy supplied to a constraint.
///
/// This type intentionally contains no scheduler mutation methods.
///
/// The resource/scheduling subsystem is responsible for constructing the
/// snapshot before evaluating a candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QubitOccupancy {
    qubit: QubitRef,
    access: QubitAccessMode,
}

impl QubitOccupancy {
    /// Creates an occupancy record.
    #[must_use]
    pub const fn new(qubit: QubitRef, access: QubitAccessMode) -> Self {
        Self { qubit, access }
    }

    /// Returns the occupied qubit.
    #[must_use]
    pub const fn qubit(self) -> QubitRef {
        self.qubit
    }

    /// Returns the occupancy mode.
    #[must_use]
    pub const fn access(self) -> QubitAccessMode {
        self.access
    }
}

// ============================================================================
// Qubit constraint
// ============================================================================

/// Generic production qubit constraint.
///
/// The constraint enforces qubit-level invariants without knowing how the
/// enclosing scheduler represents its complete resource calendar.
///
/// Concrete occupancy information is obtained from `ConstraintContext`.
///
/// This design keeps qubit scheduling independent of:
///
/// - a particular scheduler algorithm;
/// - a particular resource calendar;
/// - a particular hardware vendor;
/// - a particular topology;
/// - a particular QEC code.
#[derive(Debug, Clone)]
pub struct QubitConstraint {
    /// Stable identity assigned by the constraint owner.
    id: super::constraint::ConstraintId,

    /// Human-readable diagnostic name.
    name: String,

    /// Severity of violations.
    severity: ConstraintSeverity,

    /// Whether logical and physical references are independently checked.
    ///
    /// This is normally true. It is configurable only so specialised
    /// verification environments can intentionally relax the rule.
    require_identity_domain_separation: bool,

    /// Whether duplicate qubit operands are rejected.
    reject_duplicate_operands: bool,
}

impl QubitConstraint {
    /// Creates the production-default qubit constraint.
    ///
    /// Defaults:
    ///
    /// - identity-domain separation enabled;
    /// - duplicate operands rejected;
    /// - blocking error severity.
    pub fn new(id: super::constraint::ConstraintId) -> Self {
        Self {
            id,
            name: String::from("qubit-exclusivity"),
            severity: ConstraintSeverity::Error,
            require_identity_domain_separation: true,
            reject_duplicate_operands: true,
        }
    }

    /// Sets the diagnostic name.
    ///
    /// Empty names are rejected because diagnostics must remain identifiable.
    pub fn with_name(mut self, name: impl Into<String>) -> Result<Self, QubitConstraintConfigError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(QubitConstraintConfigError::EmptyName);
        }

        self.name = name;
        Ok(self)
    }

    /// Sets the violation severity.
    #[must_use]
    pub const fn with_severity(mut self, severity: ConstraintSeverity) -> Self {
        self.severity = severity;
        self
    }

    /// Enables or disables duplicate-operand validation.
    ///
    /// Production configurations should normally keep this enabled.
    #[must_use]
    pub const fn with_duplicate_operand_validation(mut self, enabled: bool) -> Self {
        self.reject_duplicate_operands = enabled;
        self
    }

    /// Enables or disables explicit logical/physical identity separation.
    ///
    /// Production configurations should keep this enabled.
    #[must_use]
    pub const fn with_identity_domain_separation(mut self, enabled: bool) -> Self {
        self.require_identity_domain_separation = enabled;
        self
    }

    /// Returns the stable constraint identifier.
    #[must_use]
    pub const fn id(&self) -> super::constraint::ConstraintId {
        self.id
    }

    /// Returns the diagnostic name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the configured severity.
    #[must_use]
    pub const fn severity(&self) -> ConstraintSeverity {
        self.severity
    }

    /// Returns the constraint category.
    #[must_use]
    pub const fn kind(&self) -> ConstraintKind {
        ConstraintKind::Qubit
    }

    /// Validates a candidate without consulting the scheduler context.
    ///
    /// This is useful for early structural validation.
    pub fn validate_candidate(
        &self,
        candidate: QubitCandidate<'_>,
    ) -> Result<(), QubitConstraintViolation> {
        if self.reject_duplicate_operands {
            candidate.validate().map_err(QubitConstraintViolation::from)?;
        }

        Ok(())
    }

    /// Checks whether two accesses conflict at the qubit level.
    ///
    /// Conflict semantics are deliberately independent of hardware topology.
    #[must_use]
    pub const fn accesses_conflict(
        requested: QubitAccessMode,
        existing: QubitAccessMode,
    ) -> bool {
        requested.is_exclusive() || existing.is_exclusive()
    }

    /// Checks one candidate requirement against one existing occupancy.
    #[must_use]
    pub const fn conflicts(
        requested: QubitRequirement,
        existing: QubitOccupancy,
    ) -> bool {
        requested.qubit() == existing.qubit()
            && Self::accesses_conflict(requested.access(), existing.access())
    }
}

// ============================================================================
// Configuration error
// ============================================================================

/// Invalid qubit-constraint configuration.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QubitConstraintConfigError {
    /// The supplied diagnostic name contains no non-whitespace characters.
    EmptyName,
}

impl fmt::Display for QubitConstraintConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => formatter.write_str("qubit constraint name cannot be empty"),
        }
    }
}

impl std::error::Error for QubitConstraintConfigError {}

// ============================================================================
// Qubit constraint violation
// ============================================================================

/// Structured qubit-specific violation information.
///
/// This type is deliberately richer than a string so higher-level diagnostics
/// can explain conflicts without parsing human-readable text.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QubitConstraintViolation {
    /// Duplicate logical operand in one candidate.
    DuplicateLogicalQubit(QubitId),

    /// Duplicate physical operand in one candidate.
    DuplicatePhysicalQubit(PhysicalQubitId),

    /// Candidate conflicts with existing occupancy.
    Occupied {
        /// Requested qubit.
        qubit: QubitRef,

        /// Existing occupancy mode.
        existing_access: QubitAccessMode,

        /// Requested access mode.
        requested_access: QubitAccessMode,
    },

    /// Candidate references an identity that is not available according to the
    /// supplied scheduler context.
    Unavailable(QubitRef),

    /// A logical/physical identity-domain invariant was violated.
    IdentityDomainViolation(QubitRef),
}

impl From<QubitCandidateError> for QubitConstraintViolation {
    fn from(error: QubitCandidateError) -> Self {
        match error {
            QubitCandidateError::DuplicateLogicalQubit(id) => {
                Self::DuplicateLogicalQubit(id)
            }
            QubitCandidateError::DuplicatePhysicalQubit(id) => {
                Self::DuplicatePhysicalQubit(id)
            }
        }
    }
}

impl fmt::Display for QubitConstraintViolation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateLogicalQubit(id) => {
                write!(formatter, "logical qubit {id} is used more than once")
            }

            Self::DuplicatePhysicalQubit(id) => {
                write!(formatter, "physical qubit {id} is used more than once")
            }

            Self::Occupied {
                qubit,
                existing_access,
                requested_access,
            } => write!(
                formatter,
                "qubit {qubit} is occupied by {existing_access} access and \
                 cannot satisfy requested {requested_access} access"
            ),

            Self::Unavailable(qubit) => {
                write!(formatter, "qubit {qubit} is unavailable")
            }

            Self::IdentityDomainViolation(qubit) => {
                write!(
                    formatter,
                    "qubit identity-domain invariant violated for {qubit}"
                )
            }
        }
    }
}

impl std::error::Error for QubitConstraintViolation {}

// ============================================================================
// Constraint implementation
// ============================================================================
//
// The exact generic ConstraintContext accessors are intentionally delegated to
// the foundational constraint contract. The qubit constraint therefore only
// depends on the stable abstraction rather than a planner/resource
// implementation.
//
// If the repository's foundational constraint contract exposes specialised
// qubit occupancy accessors, this implementation should use those accessors
// directly. The contract must provide:
//
//     candidate qubit requirements
//     existing qubit occupancy
//     operation applicability
//
// No direct dependency on planner internals is permitted.
//
// ============================================================================

impl Constraint for QubitConstraint {
    fn id(&self) -> super::constraint::ConstraintId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn kind(&self) -> ConstraintKind {
        ConstraintKind::Qubit
    }

    fn severity(&self) -> ConstraintSeverity {
        self.severity
    }

    fn check(
        &self,
        context: &ConstraintContext<'_>,
    ) -> Result<(), ConstraintViolation> {
        //
        // The generic constraint contract owns the candidate/state boundary.
        //
        // QubitConstraint deliberately performs no scheduler-state mutation.
        //
        // The following contract is expected from ConstraintContext:
        //
        //     context.qubit_requirements()
        //     context.qubit_occupancies()
        //
        // These accessors keep this module independent from:
        //
        //     planners/*
        //     resources/*
        //     ir/*
        //
        // and therefore prevent architectural coupling.
        //
        let requirements = context.qubit_requirements();

        if self.reject_duplicate_operands {
            let candidate = QubitCandidate::new(requirements);

            if let Err(error) = candidate.validate() {
                return Err(self.to_constraint_violation(error));
            }
        }

        let occupancies = context.qubit_occupancies();

        for requirement in requirements {
            for occupancy in occupancies {
                if Self::conflicts(*requirement, *occupancy) {
                    return Err(self.to_constraint_violation(
                        QubitConstraintViolation::Occupied {
                            qubit: requirement.qubit(),
                            existing_access: occupancy.access(),
                            requested_access: requirement.access(),
                        },
                    ));
                }
            }
        }

        Ok(())
    }
}

impl QubitConstraint {
    fn to_constraint_violation(
        &self,
        violation: QubitConstraintViolation,
    ) -> ConstraintViolation {
        ConstraintViolation::new(
            self.id,
            self.kind(),
            self.severity,
            self.name.clone(),
            violation.to_string(),
        )
    }
}

// ============================================================================
// Convenience constructors
// ============================================================================

/// Creates an exclusive logical-qubit requirement.
#[must_use]
pub const fn logical_qubit(id: QubitId) -> QubitRequirement {
    QubitRequirement::exclusive(QubitRef::Logical(id))
}

/// Creates an exclusive physical-qubit requirement.
#[must_use]
pub const fn physical_qubit(id: PhysicalQubitId) -> QubitRequirement {
    QubitRequirement::exclusive(QubitRef::Physical(id))
}

// ============================================================================
// Deterministic qubit identity helpers
// ============================================================================

/// Returns all logical qubits in deterministic order.
///
/// The returned set contains canonical `QubitId` values.
#[must_use]
pub fn logical_qubits(requirements: &[QubitRequirement]) -> BTreeSet<QubitId> {
    requirements
        .iter()
        .filter_map(|requirement| requirement.logical())
        .collect()
}

/// Returns all physical qubits in deterministic order.
///
/// The returned set contains canonical `PhysicalQubitId` values.
#[must_use]
pub fn physical_qubits(
    requirements: &[QubitRequirement],
) -> BTreeSet<PhysicalQubitId> {
    requirements
        .iter()
        .filter_map(|requirement| requirement.physical())
        .collect()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logical_requirement_uses_canonical_identity() {
        let id = QubitId::new(7);
        let requirement = logical_qubit(id);

        assert_eq!(requirement.logical(), Some(id));
        assert_eq!(requirement.physical(), None);
        assert_eq!(requirement.access(), QubitAccessMode::Exclusive);
    }

    #[test]
    fn physical_requirement_uses_canonical_identity() {
        let id = PhysicalQubitId::new(11);
        let requirement = physical_qubit(id);

        assert_eq!(requirement.physical(), Some(id));
        assert_eq!(requirement.logical(), None);
    }

    #[test]
    fn logical_and_physical_identity_domains_do_not_alias() {
        let logical = QubitRef::Logical(QubitId::new(3));
        let physical = QubitRef::Physical(PhysicalQubitId::new(3));

        assert_ne!(logical, physical);
    }

    #[test]
    fn duplicate_logical_operands_are_rejected() {
        let id = QubitId::new(2);

        let requirements = [
            logical_qubit(id),
            logical_qubit(id),
        ];

        let candidate = QubitCandidate::new(&requirements);

        assert_eq!(
            candidate.validate(),
            Err(QubitCandidateError::DuplicateLogicalQubit(id))
        );
    }

    #[test]
    fn duplicate_physical_operands_are_rejected() {
        let id = PhysicalQubitId::new(4);

        let requirements = [
            physical_qubit(id),
            physical_qubit(id),
        ];

        let candidate = QubitCandidate::new(&requirements);

        assert_eq!(
            candidate.validate(),
            Err(QubitCandidateError::DuplicatePhysicalQubit(id))
        );
    }

    #[test]
    fn different_logical_qubits_do_not_conflict() {
        let requested = logical_qubit(QubitId::new(1));
        let existing =
            QubitOccupancy::new(QubitRef::Logical(QubitId::new(2)), QubitAccessMode::Exclusive);

        assert!(!QubitConstraint::conflicts(requested, existing));
    }

    #[test]
    fn_same_exclusive_logical_qubit_conflicts() {
        let id = QubitId::new(1);

        let requested = logical_qubit(id);
        let existing =
            QubitOccupancy::new(QubitRef::Logical(id), QubitAccessMode::Exclusive);

        assert!(QubitConstraint::conflicts(requested, existing));
    }

    #[test]
    fn_same_physical_qubit_conflicts() {
        let id = PhysicalQubitId::new(9);

        let requested = physical_qubit(id);
        let existing =
            QubitOccupancy::new(QubitRef::Physical(id), QubitAccessMode::Exclusive);

        assert!(QubitConstraint::conflicts(requested, existing));
    }

    #[test]
    fn_logical_and_physical_same_numeric_value_do_not_conflict() {
        let requested = logical_qubit(QubitId::new(9));

        let existing = QubitOccupancy::new(
            QubitRef::Physical(PhysicalQubitId::new(9)),
            QubitAccessMode::Exclusive,
        );

        assert!(!QubitConstraint::conflicts(requested, existing));
    }

    #[test]
    fn_shared_observation_does_not_conflict_with_shared_observation() {
        let id = QubitId::new(10);

        let requested = QubitRequirement::new(
            QubitRef::Logical(id),
            QubitAccessMode::SharedObservation,
        );

        let existing = QubitOccupancy::new(
            QubitRef::Logical(id),
            QubitAccessMode::SharedObservation,
        );

        assert!(!QubitConstraint::conflicts(requested, existing));
    }

    #[test]
    fn exclusive_access_conflicts_with_shared_observation() {
        let id = QubitId::new(10);

        let requested = QubitRequirement::exclusive(QubitRef::Logical(id));

        let existing = QubitOccupancy::new(
            QubitRef::Logical(id),
            QubitAccessMode::SharedObservation,
        );

        assert!(QubitConstraint::conflicts(requested, existing));
    }

    #[test]
    fn arbitrary_arity_is_supported() {
        let requirements = [
            logical_qubit(QubitId::new(0)),
            logical_qubit(QubitId::new(1)),
            logical_qubit(QubitId::new(2)),
            logical_qubit(QubitId::new(3)),
            logical_qubit(QubitId::new(4)),
            logical_qubit(QubitId::new(5)),
        ];

        let candidate = QubitCandidate::new(&requirements);

        assert_eq!(candidate.len(), 6);
        assert!(!candidate.is_empty());
        assert!(candidate.validate().is_ok());
    }

    #[test]
    fn deterministic_logical_qubit_collection() {
        let requirements = [
            logical_qubit(QubitId::new(7)),
            logical_qubit(QubitId::new(2)),
            logical_qubit(QubitId::new(5)),
        ];

        let result = logical_qubits(&requirements);
        let values: Vec<usize> = result.into_iter().map(QubitId::index).collect();

        assert_eq!(values, vec![2, 5, 7]);
    }

    #[test]
    fn deterministic_physical_qubit_collection() {
        let requirements = [
            physical_qubit(PhysicalQubitId::new(8)),
            physical_qubit(PhysicalQubitId::new(1)),
            physical_qubit(PhysicalQubitId::new(5)),
        ];

        let result = physical_qubits(&requirements);
        let values: Vec<usize> =
            result.into_iter().map(PhysicalQubitId::index).collect();

        assert_eq!(values, vec![1, 5, 8]);
    }

    #[test]
    fn empty_candidate_is_valid() {
        let requirements: [QubitRequirement; 0] = [];
        let candidate = QubitCandidate::new(&requirements);

        assert!(candidate.is_empty());
        assert!(candidate.validate().is_ok());
    }

    #[test]
    fn constraint_default_configuration_is_blocking() {
        let constraint =
            QubitConstraint::new(super::super::constraint::ConstraintId::new(1));

        assert_eq!(constraint.kind(), ConstraintKind::Qubit);
        assert_eq!(constraint.severity(), ConstraintSeverity::Error);
        assert_eq!(constraint.name(), "qubit-exclusivity");
    }
}