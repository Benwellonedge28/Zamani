//! Zamani Quantum Scheduling — Channel Constraints
//!
//! Production scheduling constraints for abstract quantum channels.
//!
//! # Architectural responsibility
//!
//! This module answers:
//!
//! > "When are two channel uses compatible, conflicting, or required to be
//! > ordered in time?"
//!
//! It does NOT define the semantic channel itself.
//!
//! Canonical channel semantics are owned by:
//!
//! ```text
//! crate::quantum::ir::quantum::channel
//! ```
//!
//! Canonical qubit identities are owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! crate::quantum::ir::qubit::QubitRef
//! ```
//!
//! This module consumes those canonical definitions and adds scheduler-specific
//! conflict semantics.
//!
//! # Why this file exists
//!
//! A quantum operation can require a channel without necessarily requiring that
//! the entire channel be unavailable to every other operation.
//!
//! Examples include:
//!
//! - exclusive drive channels;
//! - shared synchronization channels;
//! - read-only acquisition channels;
//! - write-only control channels;
//! - target-specific drive channels;
//! - pairwise interaction channels;
//! - shared communication channels;
//! - dynamically constrained custom channels.
//!
//! The scheduler therefore needs a formal conflict relation rather than a
//! simplistic:
//!
//! ```text
//! channel A == channel B
//! ```
//!
//! test.
//!
//! # Separation of concerns
//!
//! ```text
//! quantum::ir::quantum::channel
//!             │
//!             │ canonical semantic channel
//!             ▼
//! scheduling::constraints::channel
//!             │
//!             │ temporal conflict semantics
//!             ▼
//! scheduling::resources
//!             │
//!             │ reservations / calendars
//!             ▼
//! scheduling planners
//! ```
//!
//! This module does not:
//!
//! - allocate physical channels;
//! - discover hardware;
//! - perform routing;
//! - schedule operations;
//! - own reservation calendars;
//! - define hardware timing;
//! - communicate with a QPU;
//! - execute pulses;
//! - perform calibration;
//! - decode QEC;
//! - define a second channel identity;
//! - define a second qubit identity.
//!
//! # Universal-program principle
//!
//! Nothing in this module assumes:
//!
//! - a fixed channel count;
//! - a fixed qubit count;
//! - a fixed gate set;
//! - a fixed topology;
//! - a fixed channel capacity;
//! - a fixed machine size;
//! - a particular quantum technology;
//! - a particular vendor.
//!
//! Concrete limits come from the target and explicit scheduling policy.
//!
//! "Infinity" means that this module introduces no artificial finite machine
//! ceiling. Concrete executions remain finite because the target, process,
//! memory and execution resources are finite.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code;
//! - standard library only.
//!
//! # Integration contract
//!
//! This file is deliberately self-contained at the scheduling-constraint
//! boundary.
//!
//! Existing/future modules consume it through these concepts:
//!
//! ```text
//! ChannelUse
//! ChannelConflict
//! ChannelConflictKind
//! ChannelConstraint
//! ChannelConstraintSet
//! ```
//!
//! `ChannelUse` may be produced by:
//!
//! - `scheduling::ir::operation`;
//! - `scheduling::adapters::ir`;
//! - `scheduling::adapters::hardware`;
//! - `scheduling::qec`;
//! - `scheduling::dynamic`;
//! - `scheduling::distributed`.
//!
//! The planner then asks this module whether two channel uses can overlap.
//!
//! The reservation/calendar layer remains responsible for finding actual
//! available time intervals.
//!
//! # Dependency direction
//!
//! ```text
//! quantum::ir::qubit
//!          │
//! quantum::ir::quantum::channel
//!          │
//!          ▼
//! scheduling::constraints::channel
//!          │
//!          ├──────────────► scheduling::resources
//!          ├──────────────► scheduling::planners
//!          ├──────────────► scheduling::verification
//!          ├──────────────► scheduling::qec
//!          └──────────────► scheduling::dynamic
//! ```
//!
//! No dependency is allowed in the opposite direction.
//!
//! # Important design rule
//!
//! This file must remain independent of concrete scheduling algorithms.
//!
//! ASAP, ALAP, critical-path, list, RCPSP, adaptive and custom planners all
//! consume the same conflict semantics.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::quantum::ir::core::identity::{ChannelId, ResourceId};
use crate::quantum::ir::qubit::QubitRef;
use crate::quantum::ir::quantum::channel::{
    Channel,
    ChannelAccess,
    ChannelDirection,
    ChannelKind,
    ChannelScope,
    ChannelTarget,
};

// =============================================================================
// Result
// =============================================================================

/// Result type for channel scheduling constraints.
pub type ChannelConstraintResult<T> = Result<T, ChannelConstraintError>;

// =============================================================================
// Access compatibility
// =============================================================================

/// Result of comparing two channel access modes.
///
/// This is intentionally scheduler-owned because the canonical IR's
/// `ChannelAccess` describes semantic access while this enum describes the
/// scheduling consequence of combining two accesses in time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChannelAccessCompatibility {
    /// Both uses may overlap.
    Compatible,

    /// The uses may overlap only when the target explicitly permits sharing.
    TargetDependent,

    /// The uses cannot overlap.
    Conflicting,
}

impl ChannelAccessCompatibility {
    /// Returns whether overlap is unconditionally compatible.
    #[must_use]
    pub const fn is_compatible(self) -> bool {
        matches!(self, Self::Compatible)
    }

    /// Returns whether target information is required.
    #[must_use]
    pub const fn is_target_dependent(self) -> bool {
        matches!(self, Self::TargetDependent)
    }

    /// Returns whether overlap is forbidden.
    #[must_use]
    pub const fn is_conflicting(self) -> bool {
        matches!(self, Self::Conflicting)
    }
}

// =============================================================================
// Conflict kind
// =============================================================================

/// Why two channel uses cannot safely overlap.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChannelConflictKind {
    /// Both uses require the same exclusive channel.
    ExclusiveAccess,

    /// A read/write access combination is incompatible.
    AccessMode,

    /// Both uses target the same exclusive semantic target.
    TargetExclusivity,

    /// Channel scopes make the uses overlap.
    ScopeOverlap,

    /// A shared channel's target-defined policy forbids the overlap.
    TargetDefinedSharing,

    /// A semantic channel requirement is incompatible with the other use.
    RequirementMismatch,

    /// An explicit scheduler constraint forbids overlap.
    ExplicitConstraint,

    /// The conflict comes from a custom target/dialect rule.
    CustomRule(String),
}

impl fmt::Display for ChannelConflictKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExclusiveAccess => formatter.write_str("exclusive-access"),
            Self::AccessMode => formatter.write_str("access-mode"),
            Self::TargetExclusivity => formatter.write_str("target-exclusivity"),
            Self::ScopeOverlap => formatter.write_str("scope-overlap"),
            Self::TargetDefinedSharing => {
                formatter.write_str("target-defined-sharing")
            }
            Self::RequirementMismatch => {
                formatter.write_str("requirement-mismatch")
            }
            Self::ExplicitConstraint => {
                formatter.write_str("explicit-constraint")
            }
            Self::CustomRule(name) => {
                write!(formatter, "custom-rule:{name}")
            }
        }
    }
}

// =============================================================================
// Channel use
// =============================================================================

/// One scheduler-visible use of an abstract channel.
///
/// This type does not contain start/end times. Time belongs to the scheduler's
/// schedule representation.
///
/// `ChannelUse` answers:
//!
//! > "What channel does this operation require and how does it intend to use
//! > it?"
///
/// The operation itself is identified through the canonical `ResourceId`
/// boundary where an operation/resource relationship has already been lowered.
///
/// A caller may also retain its own operation identity externally. This file
/// deliberately avoids creating a second operation-ID type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChannelUse {
    channel_id: ChannelId,
    resource_id: Option<ResourceId>,
    kind: ChannelKind,
    scope: ChannelScope,
    direction: ChannelDirection,
    access: ChannelAccess,
    target: ChannelTarget,
    quantity: u128,
}

impl ChannelUse {
    /// Creates a channel use directly from canonical channel semantics.
    ///
    /// The channel is validated before its semantic properties are captured.
    pub fn from_channel(
        channel: &Channel,
    ) -> ChannelConstraintResult<Self> {
        channel.validate().map_err(ChannelConstraintError::Channel)?;

        Ok(Self {
            channel_id: channel.id(),
            resource_id: None,
            kind: channel.kind().clone(),
            scope: channel.scope(),
            direction: channel.direction(),
            access: channel.access(),
            target: channel.target().clone(),
            quantity: 1,
        })
    }

    /// Creates a channel use with an explicit abstract scheduler resource
    /// identity.
    ///
    /// The resource identity is deliberately separate from `ChannelId`.
    pub fn with_resource(
        channel: &Channel,
        resource_id: ResourceId,
    ) -> ChannelConstraintResult<Self> {
        let mut usage = Self::from_channel(channel)?;
        usage.resource_id = Some(resource_id);
        Ok(usage)
    }

    /// Creates a scheduler channel use from explicit semantic properties.
    ///
    /// This constructor is useful for adapters and custom scheduling dialects
    /// that already possess canonical channel semantics but do not need to
    /// construct a second `Channel`.
    pub fn new(
        channel_id: ChannelId,
        kind: ChannelKind,
        scope: ChannelScope,
        direction: ChannelDirection,
        access: ChannelAccess,
        target: ChannelTarget,
    ) -> ChannelConstraintResult<Self> {
        Self::new_with_quantity(
            channel_id,
            kind,
            scope,
            direction,
            access,
            target,
            1,
        )
    }

    /// Creates a channel use with an explicit non-zero capacity quantity.
    pub fn new_with_quantity(
        channel_id: ChannelId,
        kind: ChannelKind,
        scope: ChannelScope,
        direction: ChannelDirection,
        access: ChannelAccess,
        target: ChannelTarget,
        quantity: u128,
    ) -> ChannelConstraintResult<Self> {
        if quantity == 0 {
            return Err(ChannelConstraintError::ZeroQuantity);
        }

        validate_scope_and_target(scope, &target)?;

        Ok(Self {
            channel_id,
            resource_id: None,
            kind,
            scope,
            direction,
            access,
            target,
            quantity,
        })
    }

    /// Returns the canonical semantic channel identity.
    #[must_use]
    pub const fn channel_id(&self) -> ChannelId {
        self.channel_id
    }

    /// Returns the scheduler resource identity, if one was assigned.
    #[must_use]
    pub const fn resource_id(&self) -> Option<ResourceId> {
        self.resource_id
    }

    /// Returns the channel semantic kind.
    #[must_use]
    pub fn kind(&self) -> &ChannelKind {
        &self.kind
    }

    /// Returns channel scope.
    #[must_use]
    pub const fn scope(&self) -> ChannelScope {
        self.scope
    }

    /// Returns channel direction.
    #[must_use]
    pub const fn direction(&self) -> ChannelDirection {
        self.direction
    }

    /// Returns channel access.
    #[must_use]
    pub const fn access(&self) -> ChannelAccess {
        self.access
    }

    /// Returns the semantic target.
    #[must_use]
    pub fn target(&self) -> &ChannelTarget {
        &self.target
    }

    /// Returns the requested channel capacity quantity.
    #[must_use]
    pub const fn quantity(&self) -> u128 {
        self.quantity
    }

    /// Associates this use with an abstract scheduler resource.
    #[must_use]
    pub const fn with_resource_id(mut self, resource_id: ResourceId) -> Self {
        self.resource_id = Some(resource_id);
        self
    }

    /// Returns the explicit qubits targeted by this channel use.
    pub fn qubits(&self) -> impl Iterator<Item = QubitRef> + '_ {
        self.target.qubits()
    }

    /// Returns whether this use explicitly targets a qubit.
    #[must_use]
    pub const fn targets_qubits(&self) -> bool {
        self.target.contains_qubit_identity()
    }

    /// Returns whether this use is global.
    #[must_use]
    pub const fn is_global(&self) -> bool {
        self.target.is_global()
    }
}

// =============================================================================
// Conflict
// =============================================================================

/// Structured description of a channel scheduling conflict.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChannelConflict {
    left_channel: ChannelId,
    right_channel: ChannelId,
    kind: ChannelConflictKind,
    reason: String,
}

impl ChannelConflict {
    /// Creates a structured conflict.
    #[must_use]
    pub fn new(
        left_channel: ChannelId,
        right_channel: ChannelId,
        kind: ChannelConflictKind,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            left_channel,
            right_channel,
            kind,
            reason: reason.into(),
        }
    }

    /// Returns the first channel identity.
    #[must_use]
    pub const fn left_channel(&self) -> ChannelId {
        self.left_channel
    }

    /// Returns the second channel identity.
    #[must_use]
    pub const fn right_channel(&self) -> ChannelId {
        self.right_channel
    }

    /// Returns the conflict category.
    #[must_use]
    pub fn kind(&self) -> &ChannelConflictKind {
        &self.kind
    }

    /// Returns the explanation.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

impl fmt::Display for ChannelConflict {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "channel conflict between {} and {}: {} ({})",
            self.left_channel,
            self.right_channel,
            self.kind,
            self.reason
        )
    }
}

// =============================================================================
// Channel constraint
// =============================================================================

/// One scheduler-level channel constraint.
///
/// This is intentionally more expressive than a Boolean "exclusive" flag.
///
/// A constraint may state that:
///
/// - a channel is exclusive;
/// - a channel has finite concurrent capacity;
/// - a channel is target-specific;
/// - particular access combinations conflict;
/// - particular channel kinds conflict;
/// - target overlap matters.
///
/// The constraint never allocates hardware.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChannelConstraint {
    channel_id: ChannelId,
    capacity: Option<u128>,
    exclusive_targets: bool,
    allow_same_target_overlap: bool,
    allow_cross_target_overlap: bool,
    access_overrides: BTreeMap<ChannelAccess, BTreeSet<ChannelAccess>>,
}

impl ChannelConstraint {
    /// Creates an unconstrained channel policy.
    ///
    /// `None` means that scalar capacity is not imposed by this constraint.
    #[must_use]
    pub fn new(channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            capacity: None,
            exclusive_targets: false,
            allow_same_target_overlap: true,
            allow_cross_target_overlap: true,
            access_overrides: BTreeMap::new(),
        }
    }

    /// Creates an exclusive channel policy.
    #[must_use]
    pub fn exclusive(channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            capacity: Some(1),
            exclusive_targets: true,
            allow_same_target_overlap: false,
            allow_cross_target_overlap: false,
            access_overrides: BTreeMap::new(),
        }
    }

    /// Creates a finite-capacity channel policy.
    ///
    /// Zero capacity is rejected when validation is requested; construction
    /// itself remains infallible so builders can be assembled incrementally.
    #[must_use]
    pub fn with_capacity(mut self, capacity: u128) -> Self {
        self.capacity = Some(capacity);
        self
    }

    /// Removes the scalar capacity restriction.
    #[must_use]
    pub fn with_unlimited_capacity(mut self) -> Self {
        self.capacity = None;
        self
    }

    /// Sets whether overlapping uses of the same target conflict.
    #[must_use]
    pub fn with_exclusive_targets(mut self, exclusive: bool) -> Self {
        self.exclusive_targets = exclusive;
        self
    }

    /// Sets whether multiple uses of the same target may overlap.
    #[must_use]
    pub fn with_same_target_overlap(mut self, allowed: bool) -> Self {
        self.allow_same_target_overlap = allowed;
        self
    }

    /// Sets whether uses on different targets may overlap.
    #[must_use]
    pub fn with_cross_target_overlap(mut self, allowed: bool) -> Self {
        self.allow_cross_target_overlap = allowed;
        self
    }

    /// Adds an explicit incompatible access pair.
    ///
    /// The relation is made symmetric so callers do not need to add both
    /// `(A, B)` and `(B, A)`.
    #[must_use]
    pub fn with_access_conflict(
        mut self,
        left: ChannelAccess,
        right: ChannelAccess,
    ) -> Self {
        self.access_overrides
            .entry(left)
            .or_default()
            .insert(right);

        self.access_overrides
            .entry(right)
            .or_default()
            .insert(left);

        self
    }

    /// Returns the channel identity governed by this constraint.
    #[must_use]
    pub const fn channel_id(&self) -> ChannelId {
        self.channel_id
    }

    /// Returns the optional concurrent capacity.
    #[must_use]
    pub const fn capacity(&self) -> Option<u128> {
        self.capacity
    }

    /// Returns whether target overlap is exclusive.
    #[must_use]
    pub const fn exclusive_targets(&self) -> bool {
        self.exclusive_targets
    }

    /// Returns whether same-target overlap is permitted.
    #[must_use]
    pub const fn allows_same_target_overlap(&self) -> bool {
        self.allow_same_target_overlap
    }

    /// Returns whether cross-target overlap is permitted.
    #[must_use]
    pub const fn allows_cross_target_overlap(&self) -> bool {
        self.allow_cross_target_overlap
    }

    /// Validates the constraint itself.
    pub fn validate(&self) -> ChannelConstraintResult<()> {
        if matches!(self.capacity, Some(0)) {
            return Err(ChannelConstraintError::ZeroCapacity {
                channel_id: self.channel_id,
            });
        }

        Ok(())
    }

    /// Checks whether one channel use is individually valid for this policy.
    pub fn validate_use(
        &self,
        usage: &ChannelUse,
    ) -> ChannelConstraintResult<()> {
        self.validate()?;

        if usage.channel_id() != self.channel_id {
            return Err(ChannelConstraintError::WrongChannel {
                expected: self.channel_id,
                actual: usage.channel_id(),
            });
        }

        if let Some(capacity) = self.capacity {
            if usage.quantity() > capacity {
                return Err(ChannelConstraintError::CapacityExceeded {
                    channel_id: self.channel_id,
                    requested: usage.quantity(),
                    capacity,
                });
            }
        }

        Ok(())
    }

    /// Compares two uses governed by this constraint.
    pub fn compare(
        &self,
        left: &ChannelUse,
        right: &ChannelUse,
    ) -> ChannelConstraintResult<ChannelAccessCompatibility> {
        self.validate()?;
        self.validate_use(left)?;
        self.validate_use(right)?;

        if left.channel_id() != right.channel_id() {
            return Ok(ChannelAccessCompatibility::Compatible);
        }

        if self.access_conflicts(left.access(), right.access()) {
            return Ok(ChannelAccessCompatibility::Conflicting);
        }

        if left.targets_qubits() && right.targets_qubits() {
            let same_target = targets_overlap(left.target(), right.target());

            if same_target && !self.allow_same_target_overlap {
                return Ok(ChannelAccessCompatibility::Conflicting);
            }

            if !same_target && !self.allow_cross_target_overlap {
                return Ok(ChannelAccessCompatibility::Conflicting);
            }

            if same_target && self.exclusive_targets {
                return Ok(ChannelAccessCompatibility::Conflicting);
            }
        }

        if self.capacity.is_some() {
            return Ok(ChannelAccessCompatibility::TargetDependent);
        }

        Ok(ChannelAccessCompatibility::Compatible)
    }

    /// Determines whether two uses conflict.
    pub fn conflicts(
        &self,
        left: &ChannelUse,
        right: &ChannelUse,
    ) -> ChannelConstraintResult<bool> {
        Ok(self.compare(left, right)?.is_conflicting())
    }

    fn access_conflicts(
        &self,
        left: ChannelAccess,
        right: ChannelAccess,
    ) -> bool {
        if self
            .access_overrides
            .get(&left)
            .is_some_and(|values| values.contains(&right))
        {
            return true;
        }

        access_compatibility(left, right).is_conflicting()
    }
}

// =============================================================================
// Constraint set
// =============================================================================

/// Deterministic collection of channel scheduling constraints.
///
/// No channel-count limit is imposed.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ChannelConstraintSet {
    constraints: BTreeMap<ChannelId, ChannelConstraint>,
}

impl ChannelConstraintSet {
    /// Creates an empty set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a channel constraint.
    ///
    /// Duplicate channel identities are rejected.
    pub fn insert(
        &mut self,
        constraint: ChannelConstraint,
    ) -> ChannelConstraintResult<()> {
        constraint.validate()?;

        let id = constraint.channel_id();

        if self.constraints.contains_key(&id) {
            return Err(ChannelConstraintError::DuplicateConstraint { id });
        }

        self.constraints.insert(id, constraint);

        Ok(())
    }

    /// Inserts or replaces a channel constraint explicitly.
    ///
    /// This operation is deterministic and is intended for target snapshots
    /// that are progressively materialized.
    pub fn insert_or_replace(
        &mut self,
        constraint: ChannelConstraint,
    ) -> ChannelConstraintResult<()> {
        constraint.validate()?;
        self.constraints
            .insert(constraint.channel_id(), constraint);
        Ok(())
    }

    /// Returns a constraint.
    #[must_use]
    pub fn get(&self, channel_id: ChannelId) -> Option<&ChannelConstraint> {
        self.constraints.get(&channel_id)
    }

    /// Returns a mutable constraint.
    pub fn get_mut(
        &mut self,
        channel_id: ChannelId,
    ) -> Option<&mut ChannelConstraint> {
        self.constraints.get_mut(&channel_id)
    }

    /// Removes a constraint.
    pub fn remove(
        &mut self,
        channel_id: ChannelId,
    ) -> Option<ChannelConstraint> {
        self.constraints.remove(&channel_id)
    }

    /// Returns whether a constraint exists.
    #[must_use]
    pub fn contains(&self, channel_id: ChannelId) -> bool {
        self.constraints.contains_key(&channel_id)
    }

    /// Returns the number of constraints.
    #[must_use]
    pub fn len(&self) -> usize {
        self.constraints.len()
    }

    /// Returns whether empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }

    /// Returns deterministic iteration.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&ChannelId, &ChannelConstraint)> {
        self.constraints.iter()
    }

    /// Validates every stored constraint.
    pub fn validate(&self) -> ChannelConstraintResult<()> {
        for constraint in self.constraints.values() {
            constraint.validate()?;
        }

        Ok(())
    }

    /// Compares two channel uses using their registered channel constraint.
    ///
    /// If no explicit constraint exists, the canonical channel access semantics
    /// are used as a conservative fallback.
    pub fn compare(
        &self,
        left: &ChannelUse,
        right: &ChannelUse,
    ) -> ChannelConstraintResult<ChannelAccessCompatibility> {
        if left.channel_id() != right.channel_id() {
            return Ok(ChannelAccessCompatibility::Compatible);
        }

        if let Some(constraint) = self.get(left.channel_id()) {
            return constraint.compare(left, right);
        }

        Ok(fallback_compatibility(left, right))
    }

    /// Returns a structured conflict, if the two uses overlap semantically.
    ///
    /// This method does not inspect time. Callers use it when they already know
    /// that two candidate intervals overlap.
    pub fn conflict(
        &self,
        left: &ChannelUse,
        right: &ChannelUse,
    ) -> ChannelConstraintResult<Option<ChannelConflict>> {
        let compatibility = self.compare(left, right)?;

        if compatibility.is_compatible() {
            return Ok(None);
        }

        if compatibility.is_target_dependent() {
            return Ok(Some(ChannelConflict::new(
                left.channel_id(),
                right.channel_id(),
                ChannelConflictKind::TargetDefinedSharing,
                "channel overlap depends on target capacity or sharing semantics",
            )));
        }

        let kind = if left.access() != right.access()
            && access_compatibility(left.access(), right.access()).is_conflicting()
        {
            ChannelConflictKind::AccessMode
        } else if targets_overlap(left.target(), right.target()) {
            ChannelConflictKind::TargetExclusivity
        } else {
            ChannelConflictKind::ExclusiveAccess
        };

        Ok(Some(ChannelConflict::new(
            left.channel_id(),
            right.channel_id(),
            kind,
            "the channel uses cannot safely overlap",
        )))
    }

    /// Finds all conflicts between a candidate use and an existing collection.
    ///
    /// Results are deterministic because the input collection is traversed in
    /// caller-defined order and the returned vector preserves that order.
    pub fn conflicts_with<'a, I>(
        &self,
        candidate: &ChannelUse,
        existing: I,
    ) -> ChannelConstraintResult<Vec<ChannelConflict>>
    where
        I: IntoIterator<Item = &'a ChannelUse>,
    {
        let mut conflicts = Vec::new();

        for usage in existing {
            if let Some(conflict) = self.conflict(candidate, usage)? {
                conflicts.push(conflict);
            }
        }

        Ok(conflicts)
    }
}

// =============================================================================
// Compatibility helpers
// =============================================================================

/// Determines the generic semantic compatibility of two channel accesses.
#[must_use]
pub const fn access_compatibility(
    left: ChannelAccess,
    right: ChannelAccess,
) -> ChannelAccessCompatibility {
    use ChannelAccess::{
        Exclusive,
        ReadOnly,
        Shared,
        TargetDefined,
        WriteOnly,
    };

    match (left, right) {
        (TargetDefined, _) | (_, TargetDefined) => {
            ChannelAccessCompatibility::TargetDependent
        }

        (Shared, Shared) => ChannelAccessCompatibility::Compatible,

        (ReadOnly, ReadOnly) => ChannelAccessCompatibility::Compatible,

        (WriteOnly, WriteOnly) => ChannelAccessCompatibility::Compatible,

        (Exclusive, _) | (_, Exclusive) => {
            ChannelAccessCompatibility::Conflicting
        }

        (ReadOnly, WriteOnly) | (WriteOnly, ReadOnly) => {
            ChannelAccessCompatibility::Conflicting
        }

        (ReadOnly, Shared)
        | (Shared, ReadOnly)
        | (WriteOnly, Shared)
        | (Shared, WriteOnly) => {
            ChannelAccessCompatibility::TargetDependent
        }
    }
}

/// Determines whether two semantic channel targets overlap.
///
/// Global targets overlap all other global targets.
///
/// An explicit resource target overlaps only the same resource.
///
/// Custom targets overlap only identical custom target names.
///
/// Qubit targets overlap when they contain at least one identical canonical
/// `QubitRef`.
#[must_use]
pub fn targets_overlap(
    left: &ChannelTarget,
    right: &ChannelTarget,
) -> bool {
    match (left, right) {
        (ChannelTarget::Global, _) | (_, ChannelTarget::Global) => true,

        (ChannelTarget::Qubit(left), ChannelTarget::Qubit(right)) => {
            left == right
        }

        (ChannelTarget::Qubit(left), ChannelTarget::Qubits(right))
        | (ChannelTarget::Qubits(right), ChannelTarget::Qubit(left)) => {
            right.iter().any(|candidate| candidate == left)
        }

        (ChannelTarget::Qubits(left), ChannelTarget::Qubits(right)) => {
            left.iter().any(|candidate| right.contains(candidate))
        }

        (ChannelTarget::Resource(left), ChannelTarget::Resource(right)) => {
            left == right
        }

        (ChannelTarget::Custom(left), ChannelTarget::Custom(right)) => {
            left == right
        }

        _ => false,
    }
}

/// Returns whether two channel uses are semantically compatible in the absence
/// of an explicit channel constraint.
#[must_use]
pub fn fallback_compatibility(
    left: &ChannelUse,
    right: &ChannelUse,
) -> ChannelAccessCompatibility {
    if left.channel_id() != right.channel_id() {
        return ChannelAccessCompatibility::Compatible;
    }

    match access_compatibility(left.access(), right.access()) {
        ChannelAccessCompatibility::Conflicting => {
            ChannelAccessCompatibility::Conflicting
        }

        ChannelAccessCompatibility::TargetDependent => {
            ChannelAccessCompatibility::TargetDependent
        }

        ChannelAccessCompatibility::Compatible => {
            if matches!(left.scope(), ChannelScope::Global)
                || matches!(right.scope(), ChannelScope::Global)
            {
                return ChannelAccessCompatibility::TargetDependent;
            }

            if targets_overlap(left.target(), right.target()) {
                if matches!(left.access(), ChannelAccess::Shared)
                    && matches!(right.access(), ChannelAccess::Shared)
                {
                    ChannelAccessCompatibility::Compatible
                } else {
                    ChannelAccessCompatibility::Conflicting
                }
            } else {
                ChannelAccessCompatibility::Compatible
            }
        }
    }
}

// =============================================================================
// Scope validation
// =============================================================================

fn validate_scope_and_target(
    scope: ChannelScope,
    target: &ChannelTarget,
) -> ChannelConstraintResult<()> {
    let target_count = target.qubit_count();

    match scope {
        ChannelScope::Global => {
            if !target.is_global() {
                return Err(ChannelConstraintError::ScopeTargetMismatch {
                    scope,
                });
            }
        }

        ChannelScope::PerTarget => {
            if target_count != 1 {
                return Err(ChannelConstraintError::ScopeTargetMismatch {
                    scope,
                });
            }
        }

        ChannelScope::MultiTarget => {
            if target_count == 0 {
                return Err(ChannelConstraintError::ScopeTargetMismatch {
                    scope,
                });
            }
        }

        ChannelScope::Pairwise => {
            if target_count != 2 {
                return Err(ChannelConstraintError::ScopeTargetMismatch {
                    scope,
                });
            }
        }

        ChannelScope::Custom => {}
    }

    Ok(())
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by channel scheduling constraint construction or checking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChannelConstraintError {
    /// Canonical channel validation failed.
    Channel(crate::quantum::ir::quantum::channel::ChannelError),

    /// A channel-use quantity was zero.
    ZeroQuantity,

    /// A channel constraint declared zero capacity.
    ZeroCapacity {
        channel_id: ChannelId,
    },

    /// A use does not belong to the constraint being applied.
    WrongChannel {
        expected: ChannelId,
        actual: ChannelId,
    },

    /// A single channel use exceeds its declared capacity.
    CapacityExceeded {
        channel_id: ChannelId,
        requested: u128,
        capacity: u128,
    },

    /// The same channel identity already has a constraint.
    DuplicateConstraint {
        id: ChannelId,
    },

    /// A scope does not match its semantic target.
    ScopeTargetMismatch {
        scope: ChannelScope,
    },
}

impl fmt::Display for ChannelConstraintError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Channel(error) => {
                write!(formatter, "invalid canonical channel: {error}")
            }

            Self::ZeroQuantity => {
                formatter.write_str("channel-use quantity must be greater than zero")
            }

            Self::ZeroCapacity { channel_id } => {
                write!(
                    formatter,
                    "channel {channel_id} cannot declare zero scheduling capacity"
                )
            }

            Self::WrongChannel { expected, actual } => {
                write!(
                    formatter,
                    "channel constraint belongs to {expected}, but use belongs to {actual}"
                )
            }

            Self::CapacityExceeded {
                channel_id,
                requested,
                capacity,
            } => {
                write!(
                    formatter,
                    "channel {channel_id} requires {requested} units but its capacity is {capacity}"
                )
            }

            Self::DuplicateConstraint { id } => {
                write!(
                    formatter,
                    "a scheduling constraint for channel {id} already exists"
                )
            }

            Self::ScopeTargetMismatch { scope } => {
                write!(
                    formatter,
                    "channel scope {scope} is incompatible with its target"
                )
            }
        }
    }
}

impl std::error::Error for ChannelConstraintError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::core::identity::ChannelId;
    use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
    use crate::quantum::ir::quantum::channel::{
        Channel,
        ChannelConstraints,
        ChannelMetadata,
        ChannelRequirement,
    };

    fn channel_id(value: u64) -> ChannelId {
        ChannelId::new(value)
    }

    fn logical(index: usize) -> QubitRef {
        QubitRef::Logical(QubitId::new(index))
    }

    fn physical(index: usize) -> QubitRef {
        QubitRef::Physical(PhysicalQubitId::new(index))
    }

    fn drive_channel(
        id: ChannelId,
        target: QubitRef,
    ) -> Channel {
        Channel::try_new(
            id,
            ChannelRequirement::new(
                ChannelKind::Drive,
                ChannelScope::PerTarget,
                ChannelDirection::Input,
                ChannelAccess::Exclusive,
            ),
            ChannelTarget::qubit(target),
            ChannelConstraints::new()
                .with_minimum_targets(1)
                .with_maximum_targets(Some(1)),
            ChannelMetadata::new(),
        )
        .expect("test drive channel must be valid")
    }

    fn shared_channel(id: ChannelId) -> Channel {
        Channel::global(
            id,
            ChannelRequirement::new(
                ChannelKind::Synchronization,
                ChannelScope::Global,
                ChannelDirection::Input,
                ChannelAccess::Shared,
            ),
        )
        .expect("test shared channel must be valid")
    }

    #[test]
    fn canonical_qubit_identity_is_used() {
        let channel = drive_channel(channel_id(1), logical(7));
        let usage =
            ChannelUse::from_channel(&channel).expect("channel use must be valid");

        assert_eq!(usage.qubit_count_for_test(), 1);
        assert!(usage.targets_qubits());
    }

    #[test]
    fn physical_qubits_are_supported() {
        let channel = drive_channel(channel_id(1), physical(9));
        let usage =
            ChannelUse::from_channel(&channel).expect("channel use must be valid");

        assert_eq!(
            usage.target(),
            &ChannelTarget::qubit(physical(9))
        );
    }

    #[test]
    fn identical_exclusive_channel_uses_conflict() {
        let channel = drive_channel(channel_id(1), logical(0));

        let left =
            ChannelUse::from_channel(&channel).expect("left use must be valid");
        let right =
            ChannelUse::from_channel(&channel).expect("right use must be valid");

        let constraints =
            ChannelConstraint::exclusive(channel_id(1));

        assert!(
            constraints
                .conflicts(&left, &right)
                .expect("comparison must succeed")
        );
    }

    #[test]
    fn_distinct_channels_do_not_conflict() {
        let left_channel = drive_channel(channel_id(1), logical(0));
        let right_channel = drive_channel(channel_id(2), logical(0));

        let left =
            ChannelUse::from_channel(&left_channel).expect("left use must be valid");
        let right =
            ChannelUse::from_channel(&right_channel).expect("right use must be valid");

        let constraints = ChannelConstraintSet::new();

        assert_eq!(
            constraints
                .compare(&left, &right)
                .expect("comparison must succeed"),
            ChannelAccessCompatibility::Compatible
        );
    }

    #[test]
    fn shared_channel_can_overlap_shared_access() {
        let channel = shared_channel(channel_id(5));

        let left =
            ChannelUse::from_channel(&channel).expect("left use must be valid");
        let right =
            ChannelUse::from_channel(&channel).expect("right use must be valid");

        assert_eq!(
            fallback_compatibility(&left, &right),
            ChannelAccessCompatibility::Compatible
        );
    }

    #[test]
    fn global_channel_is_target_dependent_without_capacity_information() {
        let channel = shared_channel(channel_id(5));

        let left =
            ChannelUse::from_channel(&channel).expect("left use must be valid");
        let right =
            ChannelUse::from_channel(&channel).expect("right use must be valid");

        let compatibility = fallback_compatibility(&left, &right);

        assert!(
            compatibility.is_compatible()
                || compatibility.is_target_dependent()
        );
    }

    #[test]
    fn overlapping_qubit_targets_are_detected() {
        let left = ChannelTarget::qubits(vec![
            logical(0),
            logical(1),
        ])
        .expect("target must be valid");

        let right = ChannelTarget::qubits(vec![
            logical(1),
            logical(2),
        ])
        .expect("target must be valid");

        assert!(targets_overlap(&left, &right));
    }

    #[test]
    fn disjoint_qubit_targets_do_not_overlap() {
        let left = ChannelTarget::qubits(vec![
            logical(0),
            logical(1),
        ])
        .expect("target must be valid");

        let right = ChannelTarget::qubits(vec![
            logical(2),
            logical(3),
        ])
        .expect("target must be valid");

        assert!(!targets_overlap(&left, &right));
    }

    #[test]
    fn identical_resource_targets_overlap() {
        let left =
            ChannelTarget::resource(ResourceId::new(10));
        let right =
            ChannelTarget::resource(ResourceId::new(10));

        assert!(targets_overlap(&left, &right));
    }

    #[test]
    fn different_resource_targets_do_not_overlap() {
        let left =
            ChannelTarget::resource(ResourceId::new(10));
        let right =
            ChannelTarget::resource(ResourceId::new(11));

        assert!(!targets_overlap(&left, &right));
    }

    #[test]
    fn custom_targets_overlap_only_when_equal() {
        let left =
            ChannelTarget::custom("target-a").expect("target must be valid");
        let right =
            ChannelTarget::custom("target-a").expect("target must be valid");
        let other =
            ChannelTarget::custom("target-b").expect("target must be valid");

        assert!(targets_overlap(&left, &right));
        assert!(!targets_overlap(&left, &other));
    }

    #[test]
    fn exclusive_constraint_rejects_excess_quantity() {
        let channel = drive_channel(channel_id(1), logical(0));

        let usage = ChannelUse::new_with_quantity(
            channel.id(),
            channel.kind().clone(),
            channel.scope(),
            channel.direction(),
            channel.access(),
            channel.target().clone(),
            2,
        )
        .expect("channel use construction must succeed");

        let constraint = ChannelConstraint::exclusive(channel.id());

        let result = constraint.validate_use(&usage);

        assert!(matches!(
            result,
            Err(ChannelConstraintError::CapacityExceeded { .. })
        ));
    }

    #[test]
    fn constraint_set_rejects_duplicate_constraints() {
        let id = channel_id(1);
        let mut set = ChannelConstraintSet::new();

        set.insert(ChannelConstraint::exclusive(id))
            .expect("first constraint must succeed");

        let result = set.insert(ChannelConstraint::exclusive(id));

        assert!(matches!(
            result,
            Err(ChannelConstraintError::DuplicateConstraint { .. })
        ));
    }

    #[test]
    fn constraint_set_can_replace_deterministically() {
        let id = channel_id(1);
        let mut set = ChannelConstraintSet::new();

        set.insert(ChannelConstraint::exclusive(id))
            .expect("first constraint must succeed");

        set.insert_or_replace(
            ChannelConstraint::new(id).with_unlimited_capacity(),
        )
        .expect("replacement must succeed");

        assert_eq!(
            set.get(id)
                .expect("constraint must exist")
                .capacity(),
            None
        );
    }

    #[test]
    fn explicit_access_conflict_is_symmetric() {
        let id = channel_id(1);
        let constraint = ChannelConstraint::new(id)
            .with_access_conflict(
                ChannelAccess::ReadOnly,
                ChannelAccess::WriteOnly,
            );

        assert!(
            constraint.access_conflicts(
                ChannelAccess::ReadOnly,
                ChannelAccess::WriteOnly,
            )
        );

        assert!(
            constraint.access_conflicts(
                ChannelAccess::WriteOnly,
                ChannelAccess::ReadOnly,
            )
        );
    }

    #[test]
    fn wrong_channel_is_rejected() {
        let channel = drive_channel(channel_id(1), logical(0));
        let usage =
            ChannelUse::from_channel(&channel).expect("usage must be valid");

        let constraint =
            ChannelConstraint::exclusive(channel_id(2));

        let result = constraint.validate_use(&usage);

        assert!(matches!(
            result,
            Err(ChannelConstraintError::WrongChannel { .. })
        ));
    }

    #[test]
    fn zero_quantity_is_rejected() {
        let result = ChannelUse::new_with_quantity(
            channel_id(1),
            ChannelKind::Drive,
            ChannelScope::PerTarget,
            ChannelDirection::Input,
            ChannelAccess::Exclusive,
            ChannelTarget::qubit(logical(0)),
            0,
        );

        assert!(matches!(
            result,
            Err(ChannelConstraintError::ZeroQuantity)
        ));
    }

    #[test]
    fn zero_capacity_is_rejected() {
        let constraint =
            ChannelConstraint::new(channel_id(1)).with_capacity(0);

        assert!(matches!(
            constraint.validate(),
            Err(ChannelConstraintError::ZeroCapacity { .. })
        ));
    }

    #[test]
    fn per_target_scope_requires_one_qubit() {
        let result = ChannelUse::new(
            channel_id(1),
            ChannelKind::Drive,
            ChannelScope::PerTarget,
            ChannelDirection::Input,
            ChannelAccess::Exclusive,
            ChannelTarget::Global,
        );

        assert!(matches!(
            result,
            Err(ChannelConstraintError::ScopeTargetMismatch {
                scope: ChannelScope::PerTarget
            })
        ));
    }

    #[test]
    fn pairwise_scope_requires_two_qubits() {
        let result = ChannelUse::new(
            channel_id(1),
            ChannelKind::Control,
            ChannelScope::Pairwise,
            ChannelDirection::Input,
            ChannelAccess::Exclusive,
            ChannelTarget::qubit(logical(0)),
        );

        assert!(matches!(
            result,
            Err(ChannelConstraintError::ScopeTargetMismatch {
                scope: ChannelScope::Pairwise
            })
        ));
    }

    #[test]
    fn conflict_contains_structured_reason() {
        let channel = drive_channel(channel_id(1), logical(0));
        let usage =
            ChannelUse::from_channel(&channel).expect("usage must be valid");

        let constraints =
            ChannelConstraintSet::new();

        let conflict = constraints
            .conflict(&usage, &usage)
            .expect("conflict check must succeed")
            .expect("exclusive channel should conflict");

        assert_eq!(conflict.left_channel(), channel.id());
        assert_eq!(conflict.right_channel(), channel.id());
        assert!(!conflict.reason().is_empty());
    }

    #[test]
    fn conflict_collection_is_deterministic() {
        let channel = drive_channel(channel_id(1), logical(0));
        let candidate =
            ChannelUse::from_channel(&channel).expect("candidate must be valid");

        let existing = vec![
            candidate.clone(),
            candidate.clone(),
            candidate.clone(),
        ];

        let constraints =
            ChannelConstraintSet::new();

        let conflicts = constraints
            .conflicts_with(&candidate, existing.iter())
            .expect("conflict collection must succeed");

        assert_eq!(conflicts.len(), 3);
    }

    // -------------------------------------------------------------------------
    // Small test-only helper.
    // -------------------------------------------------------------------------
    //
    // Kept here rather than adding API surface to ChannelUse solely for tests.
    trait ChannelUseTestExt {
        fn qubit_count_for_test(&self) -> usize;
    }

    impl ChannelUseTestExt for ChannelUse {
        fn qubit_count_for_test(&self) -> usize {
            self.target.qubit_count()
        }
    }
}