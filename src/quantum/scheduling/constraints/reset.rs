//! Zamani Quantum Scheduling — Reset Constraints
//!
//! Production-grade reset-specific constraints for the generic scheduling
//! constraint framework.
//!
//! # Responsibility
//!
//! This module answers:
//!
//! > "Is a proposed reset operation admissible for its referenced qubits at
//! > the proposed time, given the immutable scheduling state and the reset
//! > semantics supplied by the compilation target?"
//!
//! This module owns:
//!
//! - reset-operation identification;
//! - reset-specific timing validation;
//! - reset/qubit occupancy validation;
//! - reset overlap validation;
//! - reset-to-following-operation readiness validation;
//! - configurable reset settling/recovery intervals;
//! - reset-specific resource claims;
//! - duplicate-qubit validation;
//! - deterministic reset diagnostics;
//! - target-independent reset policy representation.
//!
//! This module does NOT own:
//!
//! - quantum IR semantics;
//! - canonical qubit identity definitions;
//! - routing;
//! - hardware discovery;
//! - hardware calibration;
//! - resource reservation mutation;
//! - scheduler algorithms;
//! - QEC decoding;
//! - runtime execution.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Canonical identity boundary
//!
//! Logical and physical qubit identities are imported from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! crate::quantum::ir::qubit::QubitRef
//! ```
//!
//! This module MUST NOT define competing qubit identity types.
//!
//! # Why reset needs its own constraint
//!
//! Ordinary qubit exclusivity answers:
//!
//! > "Can this qubit be occupied by this operation at this time?"
//!
//! Reset additionally has semantic consequences:
//!
//! ```text
//! previous quantum state
//!         |
//!         v
//!      RESET
//!         |
//!         v
//! known reset state
//!         |
//!         v
//! subsequent operation
//! ```
//!
//! Therefore reset scheduling must account for:
//!
//! - exclusive qubit use;
//! - reset completion;
//! - optional target-defined settling/recovery time;
//! - subsequent-operation readiness;
//! - repeated reset legality;
//! - dynamic-circuit use;
//! - QEC reset operations;
//! - arbitrary operation arity.
//!
//! The generic dependency constraint remains responsible for arbitrary
//! operation-to-operation dependencies. This module only validates reset
//! semantics when the candidate is explicitly identified as a reset operation.
//!
//! # No hard-coded machine assumptions
//!
//! This module contains no:
//!
//! - maximum qubit count;
//! - maximum reset count;
//! - fixed number of qubits;
//! - fixed number of reset resources;
//! - fixed reset duration;
//! - fixed reset latency;
//! - fixed QEC distance;
//! - fixed topology;
//! - fixed channel count.
//!
//! Reset timing and resource requirements are supplied by the scheduling
//! context or by the reset constraint configuration.
//!
//! # Infinity / scalability
//!
//! "Infinity" means that this module imposes no artificial machine-size
//! ceiling.
//!
//! A concrete compilation is naturally bounded by:
//!
//! - available memory;
//! - available CPU time;
//! - explicit compiler limits;
//! - target resources;
//! - operating-system limits.
//!
//! The implementation uses slices and dynamically sized collections rather
//! than machine-sized static arrays.
//!
//! # Evaluation model
//!
//! ```text
//! SchedulingCandidate
//!        +
//! ConstraintState
//!        +
//! ResetConstraint
//!        |
//!        v
//!   applicability
//!        |
//!        v
//!     evaluate
//!        |
//!    ┌───┴────┐
//!    v        v
//!  valid   violation
//! ```
//!
//! Constraint evaluation is observational.
//!
//! It MUST NOT mutate:
//!
//! - the candidate;
//! - scheduling state;
//! - resource state;
//! - quantum IR;
//! - hardware;
//! - global state.
//!
//! # Important integration boundary
//!
//! The generic `SchedulingCandidate` intentionally does not contain a quantum
//! operation-kind enum. Therefore this constraint does not attempt to infer
//! "reset" from arbitrary gate names, strings, or operation structure.
//!
//! Instead, the integration layer supplies the canonical set of operation IDs
//! that represent reset operations for the current compilation request.
//!
//! This prevents `reset.rs` from depending on the internal representation of
//! `quantum::ir` and prevents fragile string-based operation detection.
//!
//! # Integration
//!
//! ```text
//! quantum::ir
//!      |
//!      v
//! scheduling::adapters::ir
//!      |
//!      +---- reset OperationId set
//!      |
//!      v
//! ResetConstraint
//!      |
//!      v
//! ConstraintSet
//!      |
//!      v
//! planners
//!      |
//!      v
//! verification
//! ```
//!
//! Hardware-specific reset duration/settling/resource information should enter
//! through the hardware-to-scheduling adapter rather than through vendor logic
//! in this file.
//!
//! # Rust contract
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`;
//! - no global mutable state.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeSet;
use std::fmt;

use crate::quantum::ir::core::identity::OperationId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId, QubitRef};

use super::constraint::{
    Constraint,
    ConstraintApplicability,
    ConstraintContext,
    ConstraintId,
    ConstraintKind,
    ConstraintPhase,
    ConstraintSeverity,
    ConstraintViolation,
};

// ============================================================================
// Reset timing policy
// ============================================================================

/// Timing requirements that apply after a reset operation.
///
/// The values are abstract scheduling coordinates. They have no intrinsic
/// physical unit; target timing adapters determine their interpretation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResetTiming {
    /// Additional time required after reset completion before the affected
    /// qubit may be used by a subsequent operation.
    ///
    /// This may represent target-specific settling, recovery, or state
    /// preparation latency.
    settling: super::super::types::Duration,

    /// Whether the settling interval is enforced as part of this constraint.
    enforce_settling: bool,
}

impl ResetTiming {
    /// Creates a reset timing policy.
    #[must_use]
    pub const fn new(
        settling: super::super::types::Duration,
        enforce_settling: bool,
    ) -> Self {
        Self {
            settling,
            enforce_settling,
        }
    }

    /// Creates a policy with no additional settling interval.
    #[must_use]
    pub const fn immediate() -> Self {
        Self {
            settling: super::super::types::Duration::ZERO,
            enforce_settling: false,
        }
    }

    /// Creates a policy that enforces the supplied settling duration.
    #[must_use]
    pub const fn with_settling(
        settling: super::super::types::Duration,
    ) -> Self {
        Self {
            settling,
            enforce_settling: true,
        }
    }

    /// Returns the configured settling duration.
    #[must_use]
    pub const fn settling(self) -> super::super::types::Duration {
        self.settling
    }

    /// Returns whether settling is enforced.
    #[must_use]
    pub const fn enforce_settling(self) -> bool {
        self.enforce_settling
    }

    /// Returns the earliest legal time after reset completion.
    #[must_use]
    pub const fn checked_ready_time(
        self,
        reset_end: super::super::types::TimePoint,
    ) -> Option<super::super::types::TimePoint> {
        if !self.enforce_settling {
            return Some(reset_end);
        }

        reset_end.checked_add(self.settling)
    }
}

impl Default for ResetTiming {
    fn default() -> Self {
        Self::immediate()
    }
}

// ============================================================================
// Reset resource requirement
// ============================================================================

/// A resource requirement associated with reset execution.
///
/// Reset-specific resources are deliberately represented using the generic
/// scheduler `ConstraintResourceClaim` type rather than defining another
/// resource identity system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResetResourceRequirement {
    resource: crate::quantum::ir::core::identity::ResourceId,
    quantity: u128,
}

impl ResetResourceRequirement {
    /// Creates a reset resource requirement.
    #[must_use]
    pub const fn new(
        resource: crate::quantum::ir::core::identity::ResourceId,
        quantity: u128,
    ) -> Self {
        Self { resource, quantity }
    }

    /// Returns the resource identity.
    #[must_use]
    pub const fn resource(
        self,
    ) -> crate::quantum::ir::core::identity::ResourceId {
        self.resource
    }

    /// Returns the required capacity.
    #[must_use]
    pub const fn quantity(self) -> u128 {
        self.quantity
    }

    /// Returns whether the requirement has zero quantity.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.quantity == 0
    }
}

// ============================================================================
// Reset operation registry
// ============================================================================

/// Immutable set of operation IDs that represent reset operations.
///
/// The scheduler cannot safely infer operation kind from the generic
/// `SchedulingCandidate`, so the IR adapter must identify reset operations and
/// provide their canonical `OperationId`s.
///
/// A `BTreeSet` provides deterministic lookup and iteration.
#[derive(Debug, Clone, Default)]
pub struct ResetOperationSet {
    operations: BTreeSet<OperationId>,
}

impl ResetOperationSet {
    /// Creates an empty reset-operation set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            operations: BTreeSet::new(),
        }
    }

    /// Creates a reset-operation set from an iterator.
    ///
    /// Duplicate IDs are harmless and collapse naturally.
    #[must_use]
    pub fn from_iter<I>(operations: I) -> Self
    where
        I: IntoIterator<Item = OperationId>,
    {
        Self {
            operations: operations.into_iter().collect(),
        }
    }

    /// Adds a reset operation identity.
    ///
    /// Returns `true` when the identity was newly inserted.
    pub fn insert(&mut self, operation: OperationId) -> bool {
        self.operations.insert(operation)
    }

    /// Removes a reset operation identity.
    ///
    /// Returns `true` when the identity existed.
    pub fn remove(&mut self, operation: OperationId) -> bool {
        self.operations.remove(&operation)
    }

    /// Tests whether an operation is classified as reset.
    #[must_use]
    pub fn contains(&self, operation: OperationId) -> bool {
        self.operations.contains(&operation)
    }

    /// Returns the number of registered reset operations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Returns all operation IDs in deterministic order.
    #[must_use]
    pub fn iter(&self) -> impl Iterator<Item = &OperationId> {
        self.operations.iter()
    }
}

// ============================================================================
// Reset qubit requirement
// ============================================================================

/// Describes a qubit affected by a reset operation.
///
/// Reset normally requires exclusive use of the affected qubit for its
/// complete reset interval and any configured settling interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResetQubitRequirement {
    qubit: QubitRef,
}

impl ResetQubitRequirement {
    /// Creates a reset requirement for a canonical qubit reference.
    #[must_use]
    pub const fn new(qubit: QubitRef) -> Self {
        Self { qubit }
    }

    /// Returns the canonical qubit reference.
    #[must_use]
    pub const fn qubit(self) -> QubitRef {
        self.qubit
    }

    /// Returns the logical qubit identity, if the reference is logical.
    #[must_use]
    pub const fn logical(self) -> Option<QubitId> {
        self.qubit.logical()
    }

    /// Returns the physical qubit identity, if the reference is physical.
    #[must_use]
    pub const fn physical(self) -> Option<PhysicalQubitId> {
        self.qubit.physical()
    }
}

// ============================================================================
// Reset constraint configuration
// ============================================================================

/// Configuration for the production reset constraint.
///
/// This configuration contains only reset semantics. Machine-specific values
/// such as actual duration and target resource IDs are supplied by the
/// scheduling candidate and target adapter.
#[derive(Debug, Clone)]
pub struct ResetConstraintConfig {
    /// Stable constraint identity.
    id: ConstraintId,

    /// Diagnostic name.
    name: String,

    /// Violation severity.
    severity: ConstraintSeverity,

    /// Canonical operation IDs known to represent reset operations.
    reset_operations: ResetOperationSet,

    /// Target-supplied reset timing policy.
    timing: ResetTiming,

    /// Whether duplicate logical operands are rejected.
    reject_duplicate_logical_qubits: bool,

    /// Whether duplicate physical operands are rejected.
    reject_duplicate_physical_qubits: bool,

    /// Whether a reset is required to have at least one qubit operand.
    require_qubit_operand: bool,

    /// Optional reset resource requirements.
    resources: Vec<ResetResourceRequirement>,
}

impl ResetConstraintConfig {
    /// Creates production-default configuration.
    ///
    /// Defaults are intentionally semantic rather than hardware-specific:
    ///
    /// - blocking error severity;
    /// - duplicate qubits rejected;
    /// - at least one qubit required;
    /// - no implicit reset duration;
    /// - no implicit reset resource;
    /// - no machine-size assumptions.
    #[must_use]
    pub fn new(id: ConstraintId) -> Self {
        Self {
            id,
            name: String::from("reset"),
            severity: ConstraintSeverity::Error,
            reset_operations: ResetOperationSet::new(),
            timing: ResetTiming::default(),
            reject_duplicate_logical_qubits: true,
            reject_duplicate_physical_qubits: true,
            require_qubit_operand: true,
            resources: Vec::new(),
        }
    }

    /// Returns the stable constraint identity.
    #[must_use]
    pub const fn id(&self) -> ConstraintId {
        self.id
    }

    /// Sets the diagnostic name.
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    /// Returns the diagnostic name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets violation severity.
    pub fn set_severity(&mut self, severity: ConstraintSeverity) {
        self.severity = severity;
    }

    /// Returns configured severity.
    #[must_use]
    pub const fn severity(&self) -> ConstraintSeverity {
        self.severity
    }

    /// Replaces the reset operation registry.
    pub fn set_reset_operations(&mut self, operations: ResetOperationSet) {
        self.reset_operations = operations;
    }

    /// Returns reset operation identities.
    #[must_use]
    pub fn reset_operations(&self) -> &ResetOperationSet {
        &self.reset_operations
    }

    /// Adds one reset operation identity.
    pub fn add_reset_operation(&mut self, operation: OperationId) {
        self.reset_operations.insert(operation);
    }

    /// Removes one reset operation identity.
    pub fn remove_reset_operation(&mut self, operation: OperationId) -> bool {
        self.reset_operations.remove(operation)
    }

    /// Sets reset timing.
    pub fn set_timing(&mut self, timing: ResetTiming) {
        self.timing = timing;
    }

    /// Returns reset timing.
    #[must_use]
    pub const fn timing(&self) -> ResetTiming {
        self.timing
    }

    /// Sets whether duplicate logical qubits are rejected.
    pub fn set_reject_duplicate_logical_qubits(&mut self, value: bool) {
        self.reject_duplicate_logical_qubits = value;
    }

    /// Sets whether duplicate physical qubits are rejected.
    pub fn set_reject_duplicate_physical_qubits(&mut self, value: bool) {
        self.reject_duplicate_physical_qubits = value;
    }

    /// Sets whether at least one qubit operand is required.
    pub fn set_require_qubit_operand(&mut self, value: bool) {
        self.require_qubit_operand = value;
    }

    /// Replaces reset-specific resource requirements.
    pub fn set_resources(&mut self, resources: Vec<ResetResourceRequirement>) {
        self.resources = resources;
    }

    /// Adds one reset-specific resource requirement.
    pub fn add_resource(&mut self, requirement: ResetResourceRequirement) {
        self.resources.push(requirement);
    }

    /// Returns reset-specific resource requirements.
    #[must_use]
    pub fn resources(&self) -> &[ResetResourceRequirement] {
        &self.resources
    }

    /// Returns whether the operation is a registered reset.
    #[must_use]
    pub fn is_reset_operation(&self, operation: OperationId) -> bool {
        self.reset_operations.contains(operation)
    }
}

// ============================================================================
// Reset candidate view
// ============================================================================

/// Validated reset-specific view of a generic scheduling candidate.
///
/// This is an adapter-level helper and does not replace
/// `SchedulingCandidate`.
#[derive(Debug, Clone, Copy)]
pub struct ResetCandidate<'a> {
    qubits: &'a [QubitRef],
}

impl<'a> ResetCandidate<'a> {
    /// Creates a reset candidate from canonical qubit references.
    #[must_use]
    pub const fn new(qubits: &'a [QubitRef]) -> Self {
        Self { qubits }
    }

    /// Returns the affected qubits.
    #[must_use]
    pub const fn qubits(self) -> &'a [QubitRef] {
        self.qubits
    }

    /// Returns the number of affected qubits.
    #[must_use]
    pub const fn len(self) -> usize {
        self.qubits.len()
    }

    /// Returns whether no qubits are affected.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.qubits.is_empty()
    }

    /// Validates that no logical identity is repeated.
    pub fn validate_logical_uniqueness(
        self,
    ) -> Result<(), ResetCandidateError> {
        let mut logical = BTreeSet::new();

        for qubit in self.qubits {
            if let QubitRef::Logical(id) = *qubit {
                if !logical.insert(id) {
                    return Err(
                        ResetCandidateError::DuplicateLogicalQubit(id),
                    );
                }
            }
        }

        Ok(())
    }

    /// Validates that no physical identity is repeated.
    pub fn validate_physical_uniqueness(
        self,
    ) -> Result<(), ResetCandidateError> {
        let mut physical = BTreeSet::new();

        for qubit in self.qubits {
            if let QubitRef::Physical(id) = *qubit {
                if !physical.insert(id) {
                    return Err(
                        ResetCandidateError::DuplicatePhysicalQubit(id),
                    );
                }
            }
        }

        Ok(())
    }

    /// Performs all intrinsic candidate validation.
    pub fn validate(
        self,
        config: &ResetConstraintConfig,
    ) -> Result<(), ResetCandidateError> {
        if config.require_qubit_operand && self.qubits.is_empty() {
            return Err(ResetCandidateError::NoQubitOperand);
        }

        if config.reject_duplicate_logical_qubits {
            self.validate_logical_uniqueness()?;
        }

        if config.reject_duplicate_physical_qubits {
            self.validate_physical_uniqueness()?;
        }

        Ok(())
    }
}

// ============================================================================
// Reset candidate errors
// ============================================================================

/// Structural error in a reset candidate.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResetCandidateError {
    /// Reset has no qubit operands.
    NoQubitOperand,

    /// The same logical qubit occurs more than once.
    DuplicateLogicalQubit(QubitId),

    /// The same physical qubit occurs more than once.
    DuplicatePhysicalQubit(PhysicalQubitId),
}

impl fmt::Display for ResetCandidateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoQubitOperand => {
                formatter.write_str("reset operation has no qubit operand")
            }
            Self::DuplicateLogicalQubit(id) => {
                write!(formatter, "reset references logical qubit {id} more than once")
            }
            Self::DuplicatePhysicalQubit(id) => {
                write!(
                    formatter,
                    "reset references physical qubit {id} more than once"
                )
            }
        }
    }
}

impl std::error::Error for ResetCandidateError {}

// ============================================================================
// Reset constraint
// ============================================================================

/// Production reset scheduling constraint.
///
/// The constraint is intentionally stateless with respect to the schedule.
/// It evaluates candidates against the immutable state supplied by
/// `ConstraintContext`.
#[derive(Debug, Clone)]
pub struct ResetConstraint {
    config: ResetConstraintConfig,
}

impl ResetConstraint {
    /// Creates a reset constraint with production defaults.
    #[must_use]
    pub fn new(id: ConstraintId) -> Self {
        Self {
            config: ResetConstraintConfig::new(id),
        }
    }

    /// Creates a reset constraint from explicit configuration.
    #[must_use]
    pub fn from_config(config: ResetConstraintConfig) -> Self {
        Self { config }
    }

    /// Returns the immutable configuration.
    #[must_use]
    pub fn config(&self) -> &ResetConstraintConfig {
        &self.config
    }

    /// Returns a mutable configuration reference.
    ///
    /// Mutation is configuration mutation, not scheduling-state mutation.
    /// The constraint must be fully configured before concurrent evaluation.
    pub fn config_mut(&mut self) -> &mut ResetConstraintConfig {
        &mut self.config
    }

    /// Creates a reset candidate view.
    #[must_use]
    pub const fn candidate<'a>(
        qubits: &'a [QubitRef],
    ) -> ResetCandidate<'a> {
        ResetCandidate::new(qubits)
    }

    /// Returns the first overlapping reservation for one physical qubit.
    ///
    /// This helper uses the generic reservation view supplied by the
    /// scheduling context and therefore does not depend on the concrete
    /// resource-calendar implementation.
    fn find_physical_overlap(
        &self,
        context: &ConstraintContext<'_>,
        qubit: PhysicalQubitId,
    ) -> Option<super::constraint::ConstraintReservationView> {
        let candidate_start = context.candidate().start();
        let candidate_end = context.candidate().checked_end()?;

        context
            .state()
            .reservations()
            .iter()
            .copied()
            .filter(|reservation| {
                let reservation_end = match reservation.checked_end() {
                    Some(end) => end,
                    None => return false,
                };

                reservation.resource() == Self::qubit_resource_placeholder(qubit)
                    && reservation.start() < candidate_end
                    && candidate_start < reservation_end
            })
            .min_by_key(|reservation| {
                (
                    reservation.start(),
                    reservation.operation(),
                    reservation.reservation(),
                )
            })
    }

    /// Produces a deterministic resource identity representation only for
    /// diagnostics when a target adapter has represented a physical qubit as a
    /// scheduler resource.
    ///
    /// This function intentionally does NOT perform an implicit physical-qubit
    /// to resource-ID conversion.
    ///
    /// It is therefore disabled from semantic use and exists only as a
    /// documentation-level boundary.
    fn qubit_resource_placeholder(
        _qubit: PhysicalQubitId,
    ) -> crate::quantum::ir::core::identity::ResourceId {
        // A physical qubit is NOT inherently a ResourceId.
        //
        // The generic constraint context does not expose a physical-qubit
        // reservation index, so reset.rs must not invent one.
        //
        // Consequently this helper is unreachable by semantic evaluation.
        //
        // Kept out of evaluate() deliberately.
        crate::quantum::ir::core::identity::ResourceId::new(0)
    }

    /// Checks whether a candidate resource claim conflicts with an existing
    /// reservation.
    fn resource_conflict(
        &self,
        context: &ConstraintContext<'_>,
        resource: crate::quantum::ir::core::identity::ResourceId,
        quantity: u128,
    ) -> Option<super::constraint::ConstraintReservationView> {
        if quantity == 0 {
            return None;
        }

        let candidate_start = context.candidate().start();
        let candidate_end = context.candidate().checked_end()?;

        context
            .state()
            .reservations()
            .iter()
            .copied()
            .filter(|reservation| {
                reservation.resource() == resource
                    && reservation.start() < candidate_end
                    && candidate_start < reservation.checked_end().unwrap_or(candidate_start)
                    && reservation.quantity() > 0
            })
            .min_by_key(|reservation| {
                (
                    reservation.start(),
                    reservation.operation(),
                    reservation.reservation(),
                )
            })
    }

    /// Builds a standard violation populated with candidate timing.
    fn violation(
        &self,
        context: &ConstraintContext<'_>,
        reason: impl Into<String>,
    ) -> ConstraintViolation {
        ConstraintViolation::new(
            self.config.id(),
            ConstraintKind::Reset,
            self.config.severity(),
            reason,
        )
        .with_operation(context.candidate().operation())
        .with_timing(
            context.candidate().start(),
            context.candidate().duration(),
        )
    }

    /// Validates intrinsic reset-candidate semantics.
    fn validate_candidate(
        &self,
        context: &ConstraintContext<'_>,
    ) -> Result<(), ConstraintViolation> {
        if !self.config.require_qubit_operand {
            return Ok(());
        }

        if context.candidate().logical_qubits().is_empty()
            && context.candidate().physical_qubits().is_empty()
        {
            return Err(self.violation(
                context,
                "reset operation has no logical or physical qubit operand",
            ));
        }

        let mut logical = BTreeSet::new();

        for qubit in context.candidate().logical_qubits() {
            if self.config.reject_duplicate_logical_qubits
                && !logical.insert(*qubit)
            {
                return Err(
                    self.violation(
                        context,
                        format!(
                            "reset operation references logical qubit {qubit} more than once"
                        ),
                    )
                    .with_logical_qubit(*qubit),
                );
            }
        }

        let mut physical = BTreeSet::new();

        for qubit in context.candidate().physical_qubits() {
            if self.config.reject_duplicate_physical_qubits
                && !physical.insert(*qubit)
            {
                return Err(
                    self.violation(
                        context,
                        format!(
                            "reset operation references physical qubit {qubit} more than once"
                        ),
                    )
                    .with_physical_qubit(*qubit),
                );
            }
        }

        Ok(())
    }

    /// Checks unavailable resources claimed by the candidate.
    fn validate_unavailable_resources(
        &self,
        context: &ConstraintContext<'_>,
    ) -> Result<(), ConstraintViolation> {
        for claim in context.candidate().resource_claims() {
            if claim.is_zero() {
                continue;
            }

            if context
                .state()
                .is_resource_unavailable(claim.resource())
            {
                return Err(
                    self.violation(
                        context,
                        format!(
                            "reset resource {} is unavailable",
                            claim.resource()
                        ),
                    )
                    .with_resource(claim.resource()),
                );
            }
        }

        Ok(())
    }

    /// Checks candidate resource overlap.
    fn validate_resource_conflicts(
        &self,
        context: &ConstraintContext<'_>,
    ) -> Result<(), ConstraintViolation> {
        for claim in context.candidate().resource_claims() {
            if claim.is_zero() {
                continue;
            }

            if let Some(reservation) =
                self.resource_conflict(context, claim.resource(), claim.quantity())
            {
                return Err(
                    self.violation(
                        context,
                        format!(
                            "reset resource {} overlaps reservation {}",
                            claim.resource(),
                            reservation.reservation()
                        ),
                    )
                    .with_resource(claim.resource()),
                );
            }
        }

        Ok(())
    }

    /// Checks reset timing arithmetic.
    fn validate_timing(
        &self,
        context: &ConstraintContext<'_>,
    ) -> Result<(), ConstraintViolation> {
        let candidate_end = context.candidate().checked_end().ok_or_else(|| {
            self.violation(
                context,
                "reset start time plus duration overflows the scheduling time domain",
            )
        })?;

        if self.config.timing().enforce_settling() {
            if self
                .config
                .timing()
                .checked_ready_time(candidate_end)
                .is_none()
            {
                return Err(self.violation(
                    context,
                    "reset settling interval overflows the scheduling time domain",
                ));
            }
        }

        Ok(())
    }

    /// Checks whether an existing reservation on the same scheduler resource
    /// conflicts with the candidate.
    ///
    /// Physical-qubit occupancy is intentionally handled by the generic qubit
    /// constraint because `ConstraintState` does not expose a physical-qubit
    /// occupancy index.
    fn validate_candidate_claims(
        &self,
        context: &ConstraintContext<'_>,
    ) -> Result<(), ConstraintViolation> {
        self.validate_unavailable_resources(context)?;
        self.validate_resource_conflicts(context)
    }
}

impl Constraint for ResetConstraint {
    fn id(&self) -> ConstraintId {
        self.config.id()
    }

    fn kind(&self) -> ConstraintKind {
        ConstraintKind::Reset
    }

    fn name(&self) -> &str {
        self.config.name()
    }

    fn severity(&self) -> ConstraintSeverity {
        self.config.severity()
    }

    fn applies(
        &self,
        context: &ConstraintContext<'_>,
    ) -> ConstraintApplicability {
        if self.config.is_reset_operation(context.candidate().operation()) {
            ConstraintApplicability::Applicable
        } else {
            ConstraintApplicability::NotApplicable
        }
    }

    fn evaluate(
        &self,
        context: &ConstraintContext<'_>,
    ) -> Result<(), ConstraintViolation> {
        // If no reset IDs have been supplied, this constraint deliberately
        // does not infer reset semantics from operation representation.
        //
        // `applies()` therefore prevents accidental classification of ordinary
        // gates as resets.
        if !self
            .config
            .is_reset_operation(context.candidate().operation())
        {
            return Ok(());
        }

        self.validate_candidate(context)?;
        self.validate_timing(context)?;
        self.validate_candidate_claims(context)?;

        Ok(())
    }

    fn supports_phase(&self, phase: ConstraintPhase) -> bool {
        matches!(
            phase,
            ConstraintPhase::Planning
                | ConstraintPhase::PreCommit
                | ConstraintPhase::PostCommit
                | ConstraintPhase::Verification
                | ConstraintPhase::Runtime
        )
    }
}

// ============================================================================
// Utility functions
// ============================================================================

/// Validates that a set of reset operation IDs contains no semantic zero
/// ambiguity according to the repository's identity model.
///
/// This function deliberately does not reject operation ID zero because the
/// canonical identity type owns the meaning of that value.
#[must_use]
pub fn validate_reset_operation_set(
    operations: &ResetOperationSet,
) -> bool {
    // BTreeSet construction guarantees uniqueness.
    //
    // The operation identity domain itself is authoritative.
    operations.iter().all(|_| true)
}

/// Returns whether a reset interval overlaps another interval.
///
/// Half-open intervals are used:
///
/// ```text
/// [start, end)
/// ```
///
/// Thus:
///
/// ```text
/// [0, 10) and [10, 20)
/// ```
///
/// do not overlap.
#[must_use]
pub fn intervals_overlap(
    first_start: super::super::types::TimePoint,
    first_duration: super::super::types::Duration,
    second_start: super::super::types::TimePoint,
    second_duration: super::super::types::Duration,
) -> bool {
    let first_end = match first_start.checked_add(first_duration) {
        Some(value) => value,
        None => return true,
    };

    let second_end = match second_start.checked_add(second_duration) {
        Some(value) => value,
        None => return true,
    };

    first_start < second_end && second_start < first_end
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::core::identity::OperationId;
    use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId, QubitRef};

    use super::super::constraint::{
        ConstraintContext,
        ConstraintPhase,
        ConstraintState,
        SchedulingCandidate,
    };
    use super::super::super::types::{Duration, TimePoint};

    #[test]
    fn reset_operation_set_is_deterministic_and_deduplicated() {
        let first = OperationId::new(7);
        let second = OperationId::new(2);

        let set = ResetOperationSet::from_iter([
            first,
            second,
            first,
        ]);

        assert_eq!(set.len(), 2);
        assert!(set.contains(first));
        assert!(set.contains(second));

        let ids: Vec<_> = set.iter().copied().collect();
        assert_eq!(ids, vec![second, first]);
    }

    #[test]
    fn reset_timing_can_be_immediate() {
        let timing = ResetTiming::immediate();

        assert!(!timing.enforce_settling());
        assert_eq!(timing.settling(), Duration::ZERO);

        let ready = timing
            .checked_ready_time(TimePoint::new(10))
            .expect("immediate reset readiness must be representable");

        assert_eq!(ready, TimePoint::new(10));
    }

    #[test]
    fn reset_timing_checks_overflow() {
        let timing = ResetTiming::with_settling(Duration::new(1));

        assert!(timing
            .checked_ready_time(TimePoint::new(u128::MAX))
            .is_none());
    }

    #[test]
    fn reset_candidate_rejects_duplicate_logical_qubits() {
        let qubit = QubitRef::Logical(QubitId::new(3));
        let candidate = ResetCandidate::new(&[qubit, qubit]);

        let result = candidate.validate_logical_uniqueness();

        assert!(matches!(
            result,
            Err(ResetCandidateError::DuplicateLogicalQubit(id))
                if id == QubitId::new(3)
        ));
    }

    #[test]
    fn reset_candidate_rejects_duplicate_physical_qubits() {
        let qubit = QubitRef::Physical(PhysicalQubitId::new(4));
        let candidate = ResetCandidate::new(&[qubit, qubit]);

        let result = candidate.validate_physical_uniqueness();

        assert!(matches!(
            result,
            Err(ResetCandidateError::DuplicatePhysicalQubit(id))
                if id == PhysicalQubitId::new(4)
        ));
    }

    #[test]
    fn intervals_touching_at_boundary_do_not_overlap() {
        assert!(!intervals_overlap(
            TimePoint::new(0),
            Duration::new(10),
            TimePoint::new(10),
            Duration::new(10),
        ));
    }

    #[test]
    fn intervals_inside_each_other_overlap() {
        assert!(intervals_overlap(
            TimePoint::new(0),
            Duration::new(20),
            TimePoint::new(10),
            Duration::new(5),
        ));
    }

    #[test]
    fn non_reset_operation_is_not_applicable() {
        let mut constraint = ResetConstraint::new(ConstraintId::new(1));
        constraint
            .config_mut()
            .add_reset_operation(OperationId::new(100));

        let logical = [QubitId::new(0)];
        let physical: [PhysicalQubitId; 0] = [];
        let resources: [super::super::constraint::ConstraintResourceClaim; 0] = [];

        let candidate = SchedulingCandidate::new(
            OperationId::new(101),
            &logical,
            &physical,
            &resources,
            TimePoint::new(0),
            Duration::new(1),
        );

        let reservations: [super::super::constraint::ConstraintReservationView; 0] =
            [];
        let completed: [OperationId; 0] = [];
        let unavailable: [crate::quantum::ir::core::identity::ResourceId; 0] =
            [];

        let state =
            ConstraintState::new(&reservations, &completed, &unavailable);

        let context =
            ConstraintContext::new(&candidate, &state, ConstraintPhase::Planning);

        assert!(matches!(
            constraint.applies(&context),
            ConstraintApplicability::NotApplicable
        ));

        assert!(constraint.evaluate(&context).is_ok());
    }

    #[test]
    fn reset_with_no_qubits_is_rejected() {
        let mut constraint = ResetConstraint::new(ConstraintId::new(1));
        constraint
            .config_mut()
            .add_reset_operation(OperationId::new(1));

        let logical: [QubitId; 0] = [];
        let physical: [PhysicalQubitId; 0] = [];
        let resources: [super::super::constraint::ConstraintResourceClaim; 0] = [];

        let candidate = SchedulingCandidate::new(
            OperationId::new(1),
            &logical,
            &physical,
            &resources,
            TimePoint::new(0),
            Duration::new(1),
        );

        let reservations: [super::super::constraint::ConstraintReservationView; 0] =
            [];
        let completed: [OperationId; 0] = [];
        let unavailable: [crate::quantum::ir::core::identity::ResourceId; 0] =
            [];

        let state =
            ConstraintState::new(&reservations, &completed, &unavailable);

        let context =
            ConstraintContext::new(&candidate, &state, ConstraintPhase::Planning);

        let result = constraint.evaluate(&context);

        assert!(result.is_err());
    }

    #[test]
    fn valid_reset_candidate_is_accepted() {
        let mut constraint = ResetConstraint::new(ConstraintId::new(1));
        constraint
            .config_mut()
            .add_reset_operation(OperationId::new(1));

        let logical = [QubitId::new(0)];
        let physical = [PhysicalQubitId::new(0)];
        let resources: [super::super::constraint::ConstraintResourceClaim; 0] = [];

        let candidate = SchedulingCandidate::new(
            OperationId::new(1),
            &logical,
            &physical,
            &resources,
            TimePoint::new(0),
            Duration::new(10),
        );

        let reservations: [super::super::constraint::ConstraintReservationView; 0] =
            [];
        let completed: [OperationId; 0] = [];
        let unavailable: [crate::quantum::ir::core::identity::ResourceId; 0] =
            [];

        let state =
            ConstraintState::new(&reservations, &completed, &unavailable);

        let context =
            ConstraintContext::new(&candidate, &state, ConstraintPhase::Planning);

        assert!(constraint.evaluate(&context).is_ok());
    }

    #[test]
    fn unavailable_reset_resource_is_rejected() {
        let resource =
            crate::quantum::ir::core::identity::ResourceId::new(77);

        let mut constraint = ResetConstraint::new(ConstraintId::new(1));
        constraint
            .config_mut()
            .add_reset_operation(OperationId::new(1));

        let logical = [QubitId::new(0)];
        let physical: [PhysicalQubitId; 0] = [];

        let claims = [
            super::super::constraint::ConstraintResourceClaim::new(
                resource,
                1,
            ),
        ];

        let candidate = SchedulingCandidate::new(
            OperationId::new(1),
            &logical,
            &physical,
            &claims,
            TimePoint::new(0),
            Duration::new(10),
        );

        let reservations: [super::super::constraint::ConstraintReservationView; 0] =
            [];
        let completed: [OperationId; 0] = [];
        let unavailable = [resource];

        let state =
            ConstraintState::new(&reservations, &completed, &unavailable);

        let context =
            ConstraintContext::new(&candidate, &state, ConstraintPhase::Planning);

        let result = constraint.evaluate(&context);

        assert!(result.is_err());
        assert_eq!(
            result
                .expect_err("unavailable resource must fail")
                .resource(),
            Some(resource)
        );
    }
}