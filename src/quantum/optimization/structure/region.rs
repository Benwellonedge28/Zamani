//! Zamani Quantum Optimization — Semantic Optimization Regions
//!
//! Production-grade, backend-independent region abstraction for
//! `quantum::optimization::structure`.
//!
//! # Architectural position
//!
//! ```text
//!                         canonical Quantum IR
//!                                  │
//!                                  ▼
//!                    optimization::circuit::CircuitView
//!                                  │
//!                                  ▼
//!                         structure::region
//!                                  │
//!                 ┌────────────────┼────────────────┐
//!                 │                │                │
//!                 ▼                ▼                ▼
//!             local passes     loop passes     conditional passes
//!                 │                │                │
//!                 └────────────────┼────────────────┘
//!                                  ▼
//!                       CircuitEditPlan / rewrite
//! ```
//!
//! This module defines the optimizer's notion of a **semantic region**.
//!
//! A region is an optimizer-owned, invocation-local view over a contiguous
//! operation range in the canonical Quantum IR. It is NOT a second quantum IR,
//! and it does not own or mutate quantum operations.
//!
//! # Ownership
//!
//! The canonical Quantum IR remains owned by:
//!
//! `crate::quantum::ir`
//!
//! The optimizer circuit access layer remains owned by:
//!
//! `crate::quantum::optimization::circuit`
//!
//! This file owns only:
//!
//! - region identity;
//! - region kind;
//! - region boundaries;
//! - region optimization policy;
//! - validated operation ranges;
//! - immutable region views;
//! - deterministic region relationships;
//! - region containment/overlap queries;
//! - region partitioning;
//! - region-tree construction;
//! - region-local iteration;
//! - region invariants.
//!
//! # Why regions exist
//!
//! Quantum optimization cannot safely assume that every circuit is one flat
//! sequence of freely movable operations.
//!
//! Future Zamani quantum programs may contain:
//!
//! - ordinary linear blocks;
//! - measurement-delimited blocks;
//! - reset-delimited blocks;
//! - classically controlled regions;
//! - conditional branches;
//! - loop bodies;
//! - loop-carried regions;
//! - nested regions;
//! - optimization barriers;
//! - protected compiler-generated regions;
//! - logical/fault-tolerant regions;
//! - target-specific protected regions.
//!
//! A local optimization pass must therefore know:
//!
//! 1. where it is allowed to operate;
//! 2. what boundaries it must not cross;
//! 3. whether the region may be reordered;
//! 4. whether the region may be fused with siblings;
//! 5. whether the region is only observational or may be transformed;
//! 6. whether the region is nested inside another region.
//!
//! # Important semantic rule
//!
//! A region boundary is an **optimizer permission boundary**, not automatically
//! a statement about quantum semantics.
//!
//! For example, `MeasurementDelimited` means that an optimizer must not move
//! operations across the boundary merely because the operations commute.
//! It does not itself attempt to prove the measurement semantics.
//!
//! Semantic equivalence remains owned by:
//!
//! `optimization::equivalence`
//!
//! and final semantic verification remains owned by:
//!
//! `optimization::verification`.
//!
//! # Contiguous representation
//!
//! Regions are represented by half-open ranges:
//!
//! ```text
//! start .. end
//! ```
//!
//! where:
//!
//! - `start` is inclusive;
//! - `end` is exclusive.
//!
//! Therefore:
//!
//! ```text
//! 0..0  = empty region
//! 0..1  = first operation
//! 2..5  = operations 2, 3 and 4
//! ```
//!
//! Empty regions are supported because they are useful for insertion points,
//! CFG construction, and future structural transformations. Whether a
//! particular pass accepts an empty region is its own policy decision.
//!
//! # Scaling
//!
//! There is no artificial maximum region size.
//!
//! A region stores only:
//!
//! - an invocation-local identifier;
//! - a range;
//! - compact policy metadata;
//! - optional parent information.
//!
//! Region creation is O(1) after the caller has a valid `CircuitView`.
//!
//! Region iteration is O(n) in the number of operations visited.
//!
//! Region relationship checks are O(1).
//!
//! Region-tree construction is O(r log r) in the number of regions because
//! regions are deterministically sorted before parent assignment.
//!
//! Therefore this module can scale from tiny circuits to the largest circuits
//! that the surrounding IR, allocator, and configured optimizer resource
//! budgets can support.
//!
//! This module deliberately does not introduce a second size limit. The
//! canonical IR and optimizer limit systems remain responsible for resource
//! protection.
//!
//! # Determinism
//!
//! Region IDs are invocation-local.
//!
//! Region ordering is deterministic:
//!
//! 1. start index;
//! 2. end index;
//! 3. region kind;
//! 4. region ID.
//!
//! No global mutable state or random number generator is used.
//!
//! # Mutation
//!
//! This module provides no mutable access to canonical operations.
//!
//! A region can be used to *plan* optimization, but actual mutation must pass
//! through `optimization::circuit::CircuitEditPlan` / `CircuitEditor` or the
//! higher-level rewrite infrastructure.
//!
//! This prevents region logic from bypassing canonical IR invariants.
//!
//! # Integration contract
//!
//! ## `optimization/circuit.rs`
//!
//! This is the primary dependency.
//!
//! `region.rs` uses the existing:
//!
//! - `CircuitView`;
//! - `OperationId`;
//! - `RegionId`;
//! - `OperationRef`.
//!
//! It deliberately does NOT redefine those identifiers.
//!
//! `circuit.rs` remains the owner of optimizer-local operation/region IDs.
//!
//! ## `structure/block.rs`
//!
//! `block.rs` can use `Region` as the enclosing structural boundary and
//! construct block-specific views without changing this file.
//!
//! ## `structure/loop.rs`
//!
//! Loop optimization can classify a region as `LoopBody`, `LoopHeader`,
//! `LoopLatch`, or `LoopCarried` using `RegionKind` and `RegionBoundary`.
//!
//! No loop implementation is required by this file.
//!
//! ## `structure/conditional.rs`
//!
//! Conditional optimization can use `ConditionalArm` and
//! `ClassicalControl` region kinds. Branch-specific semantics remain owned by
//! the conditional subsystem.
//!
//! ## `structure/control_flow.rs`
//!
//! A future CFG can represent each CFG node/edge's operation span using
//! `Region`. This module intentionally does not implement a CFG itself.
//!
//! ## `analysis/*`
//!
//! Analyses can consume `RegionView` and restrict dependency, liveness,
//! commutation, depth, and gate-count analysis to the region.
//!
//! The analysis subsystem remains the owner of analysis semantics.
//!
//! ## `rewrite.rs`
//!
//! Rewrite candidates may be restricted to a `Region` before matching.
//! `region.rs` does not execute rewrites.
//!
//! ## `pass.rs` / `pipeline.rs`
//!
//! Passes may declare region requirements and use `RegionSet` to determine
//! which portions of the circuit are eligible for a transformation.
//!
//! The pass and pipeline systems remain responsible for pass sequencing.
//!
//! ## `equivalence.rs` / `verification/*`
//!
//! Region boundaries do not constitute semantic equivalence proofs.
//! Transformations crossing or changing region boundaries must still use the
//! appropriate equivalence/verification machinery.
//!
//! ## `limits.rs`
//!
//! This file does not duplicate optimizer limits. Region collections remain
//! bounded by the existing optimizer/IR resource policies.
//!
//! ## `quantum::ir`
//!
//! The region abstraction reads canonical operations through `CircuitView`.
//! It never introduces another `Gate`, `QuantumCircuit`, qubit, parameter, or
//! measurement representation.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! # Safety
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! This module contains no unsafe operations and no unsafe abstractions.
//!
//! # Design invariant
//!
//! If this file is completed and the surrounding `circuit.rs` API remains
//! stable, future region-aware optimization files should not need to modify
//! this file merely because new optimization passes are added.
//!
//! New semantic region types should normally be represented through the
//! extensible `RegionKind` enum only when they change region behavior. A pass
//! that merely needs a label should use `RegionLabel` instead of expanding the
//! semantic enum.

#![forbid(unsafe_code)]

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Range, RangeInclusive};

use super::super::circuit::{
    CircuitView,
    OperationId,
    OperationRef,
    RegionId,
};

// =============================================================================
// Result / error types
// =============================================================================

/// Result type used by the region subsystem.
pub type RegionResult<T> = Result<T, RegionError>;

/// Errors produced by semantic-region construction and querying.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegionError {
    /// The region range is outside the supplied circuit view.
    RangeOutOfBounds {
        /// Requested start.
        start: usize,

        /// Requested end.
        end: usize,

        /// Number of operations in the circuit.
        circuit_len: usize,
    },

    /// The region has an invalid relationship with another region.
    InvalidRelationship {
        /// First region.
        first: RegionId,

        /// Second region.
        second: RegionId,

        /// Human-readable reason.
        message: &'static str,
    },

    /// A parent region does not contain its child.
    ParentDoesNotContainChild {
        /// Parent region.
        parent: RegionId,

        /// Child region.
        child: RegionId,
    },

    /// A child region overlaps its parent incorrectly.
    InvalidNesting {
        /// Region involved.
        region: RegionId,

        /// Conflicting region.
        conflicting: RegionId,
    },

    /// Sibling regions overlap.
    OverlappingSiblings {
        /// First region.
        first: RegionId,

        /// Second region.
        second: RegionId,
    },

    /// A partition does not cover the requested range exactly.
    InvalidPartition {
        /// Expected range.
        expected: Range<usize>,

        /// Actual covered range.
        actual: Range<usize>,
    },

    /// A partition contains overlapping regions.
    PartitionOverlap {
        /// First region.
        first: RegionId,

        /// Second region.
        second: RegionId,
    },

    /// A partition contains a gap where a complete partition was required.
    PartitionGap {
        /// Gap start.
        start: usize,

        /// Gap end.
        end: usize,
    },

    /// Region IDs must be unique inside one region collection.
    DuplicateRegionId {
        /// Duplicated ID.
        id: RegionId,
    },

    /// A supplied region belongs to a different circuit snapshot.
    SnapshotMismatch {
        /// Region involved.
        region: RegionId,
    },

    /// A requested operation is outside the region.
    OperationOutsideRegion {
        /// Operation index.
        index: usize,

        /// Region range.
        region: Range<usize>,
    },

    /// A requested region index is invalid.
    IndexOutOfRange {
        /// Requested index.
        index: usize,

        /// Number of regions.
        len: usize,
    },

    /// A numeric calculation overflowed.
    ArithmeticOverflow {
        /// Description of the calculation.
        calculation: &'static str,
    },
}

impl fmt::Display for RegionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RangeOutOfBounds {
                start,
                end,
                circuit_len,
            } => write!(
                formatter,
                "optimization region {start}..{end} is outside circuit \
                 length {circuit_len}"
            ),

            Self::InvalidRelationship {
                first,
                second,
                message,
            } => write!(
                formatter,
                "invalid relationship between {first} and {second}: {message}"
            ),

            Self::ParentDoesNotContainChild { parent, child } => write!(
                formatter,
                "region {parent} does not contain child region {child}"
            ),

            Self::InvalidNesting {
                region,
                conflicting,
            } => write!(
                formatter,
                "region {region} has invalid nesting with {conflicting}"
            ),

            Self::OverlappingSiblings { first, second } => write!(
                formatter,
                "sibling regions {first} and {second} overlap"
            ),

            Self::InvalidPartition { expected, actual } => write!(
                formatter,
                "invalid region partition: expected {expected:?}, \
                 covered {actual:?}"
            ),

            Self::PartitionOverlap { first, second } => write!(
                formatter,
                "partition regions {first} and {second} overlap"
            ),

            Self::PartitionGap { start, end } => write!(
                formatter,
                "region partition contains uncovered gap {start}..{end}"
            ),

            Self::DuplicateRegionId { id } => {
                write!(formatter, "duplicate optimization region ID {id}")
            }

            Self::SnapshotMismatch { region } => write!(
                formatter,
                "region {region} belongs to a different circuit snapshot"
            ),

            Self::OperationOutsideRegion { index, region } => write!(
                formatter,
                "operation {index} lies outside region {region:?}"
            ),

            Self::IndexOutOfRange { index, len } => write!(
                formatter,
                "region index {index} is outside collection length {len}"
            ),

            Self::ArithmeticOverflow { calculation } => write!(
                formatter,
                "arithmetic overflow while calculating {calculation}"
            ),
        }
    }
}

impl std::error::Error for RegionError {}

// =============================================================================
// Region kind
// =============================================================================

/// Semantic category of an optimizer region.
///
/// The enum intentionally describes optimization semantics rather than
/// source-language syntax. A frontend or control-flow subsystem may map its
/// own constructs onto these categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegionKind {
    /// Ordinary linear sequence of operations.
    Linear,

    /// Generic optimization block.
    Block,

    /// Region containing operations that form a loop header.
    LoopHeader,

    /// Loop body.
    LoopBody,

    /// Loop latch/back-edge body.
    LoopLatch,

    /// Operations whose values/state are carried between loop iterations.
    LoopCarried,

    /// Conditional branch/arm.
    ConditionalArm,

    /// Region controlled by classical data.
    ClassicalControl,

    /// Region delimited by measurement semantics.
    MeasurementDelimited,

    /// Region delimited by reset semantics.
    ResetDelimited,

    /// Region protected from ordinary optimization movement.
    Protected,

    /// Compiler-generated region that should not be treated as user-level
    /// source structure.
    CompilerGenerated,

    /// Logical/fault-tolerant region.
    Logical,

    /// Target-specific protected region.
    TargetSpecific,

    /// A region supplied by an external optimizer integration.
    External,
}

impl RegionKind {
    /// Returns a stable machine-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Linear => "linear",
            Self::Block => "block",
            Self::LoopHeader => "loop_header",
            Self::LoopBody => "loop_body",
            Self::LoopLatch => "loop_latch",
            Self::LoopCarried => "loop_carried",
            Self::ConditionalArm => "conditional_arm",
            Self::ClassicalControl => "classical_control",
            Self::MeasurementDelimited => "measurement_delimited",
            Self::ResetDelimited => "reset_delimited",
            Self::Protected => "protected",
            Self::CompilerGenerated => "compiler_generated",
            Self::Logical => "logical",
            Self::TargetSpecific => "target_specific",
            Self::External => "external",
        }
    }

    /// Returns true if ordinary transformations must not cross this region's
    /// boundary without explicit permission.
    #[must_use]
    pub const fn is_boundary_sensitive(self) -> bool {
        matches!(
            self,
            Self::MeasurementDelimited
                | Self::ResetDelimited
                | Self::Protected
                | Self::Logical
                | Self::TargetSpecific
                | Self::ClassicalControl
                | Self::ConditionalArm
                | Self::LoopCarried
        )
    }

    /// Returns true if this kind represents control-flow structure.
    #[must_use]
    pub const fn is_control_flow(self) -> bool {
        matches!(
            self,
            Self::LoopHeader
                | Self::LoopBody
                | Self::LoopLatch
                | Self::LoopCarried
                | Self::ConditionalArm
                | Self::ClassicalControl
        )
    }

    /// Returns true if this region normally permits local optimization.
    #[must_use]
    pub const fn permits_local_optimization(self) -> bool {
        !matches!(self, Self::Protected)
    }
}

impl Ord for RegionKind {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialOrd for RegionKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// =============================================================================
// Region boundary
// =============================================================================

/// Semantic policy for a region boundary.
///
/// This is deliberately independent from a particular gate or operation name.
/// Gate-level analyses decide whether a specific operation actually creates
/// such a boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegionBoundary {
    /// No additional movement restriction is implied.
    Open,

    /// Operations must not be moved across this boundary by ordinary local
    /// optimization.
    Closed,

    /// Crossing requires an explicit pass-level proof/permission.
    Guarded,

    /// Crossing is forbidden unless a specialized structural transformation
    /// owns the operation.
    Structural,
}

impl RegionBoundary {
    /// Returns true when movement across the boundary is unrestricted by this
    /// policy alone.
    #[must_use]
    pub const fn is_open(self) -> bool {
        matches!(self, Self::Open)
    }

    /// Returns true when an ordinary local pass must treat the boundary as
    /// closed.
    #[must_use]
    pub const fn is_closed(self) -> bool {
        matches!(self, Self::Closed | Self::Structural)
    }

    /// Returns true when crossing requires explicit permission.
    #[must_use]
    pub const fn requires_permission(self) -> bool {
        matches!(self, Self::Guarded | Self::Structural)
    }
}

impl Default for RegionBoundary {
    fn default() -> Self {
        Self::Open
    }
}

// =============================================================================
// Region label
// =============================================================================

/// Optional compact label for diagnostics, provenance, and external
/// integrations.
///
/// Labels are not interpreted semantically by the region engine.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegionLabel(String);

impl RegionLabel {
    /// Creates a region label.
    ///
    /// Empty labels are rejected because they provide no useful identity in
    /// diagnostics or provenance.
    pub fn new(value: impl Into<String>) -> RegionResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(RegionError::InvalidRelationship {
                first: RegionId::new(0),
                second: RegionId::new(0),
                message: "region labels must not be empty",
            });
        }

        Ok(Self(value))
    }

    /// Returns the label text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RegionLabel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Region policy
// =============================================================================

/// Transformation policy attached to a region.
///
/// The policy is intentionally declarative. It does not execute an optimizer
/// pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegionPolicy {
    /// Whether local gate rewrites may occur inside the region.
    pub allow_local_rewrites: bool,

    /// Whether operation movement inside the region is permitted.
    pub allow_reordering: bool,

    /// Whether sibling regions may later be considered for fusion.
    pub allow_fusion: bool,

    /// Whether structural optimization may change the region's operation
    /// count.
    pub allow_structural_change: bool,

    /// Whether the region is protected from ordinary optimization.
    pub protected: bool,

    /// Whether an explicit equivalence proof should be required by a
    /// transformation crossing the region boundary.
    pub require_equivalence_proof: bool,
}

impl RegionPolicy {
    /// Conservative policy for arbitrary regions.
    pub const fn conservative() -> Self {
        Self {
            allow_local_rewrites: true,
            allow_reordering: false,
            allow_fusion: false,
            allow_structural_change: false,
            protected: false,
            require_equivalence_proof: true,
        }
    }

    /// Policy appropriate for an ordinary local optimization block.
    pub const fn local() -> Self {
        Self {
            allow_local_rewrites: true,
            allow_reordering: true,
            allow_fusion: true,
            allow_structural_change: true,
            protected: false,
            require_equivalence_proof: true,
        }
    }

    /// Fully protected policy.
    pub const fn protected() -> Self {
        Self {
            allow_local_rewrites: false,
            allow_reordering: false,
            allow_fusion: false,
            allow_structural_change: false,
            protected: true,
            require_equivalence_proof: true,
        }
    }
}

impl Default for RegionPolicy {
    fn default() -> Self {
        Self::conservative()
    }
}

// =============================================================================
// Region
// =============================================================================

/// Invocation-local semantic optimization region.
///
/// `Region` contains no quantum operations. It only identifies an operation
/// span and the optimization policy governing that span.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Region {
    id: RegionId,
    range: Range<usize>,
    kind: RegionKind,
    entry_boundary: RegionBoundary,
    exit_boundary: RegionBoundary,
    policy: RegionPolicy,
    parent: Option<RegionId>,
    label: Option<RegionLabel>,
}

impl Region {
    /// Creates a validated region over a circuit view.
    pub fn new(
        id: RegionId,
        range: Range<usize>,
        kind: RegionKind,
        view: &CircuitView<'_>,
    ) -> RegionResult<Self> {
        validate_range(&range, view.len())?;

        let policy = default_policy_for_kind(kind);

        Ok(Self {
            id,
            range,
            kind,
            entry_boundary: default_boundary_for_kind(kind),
            exit_boundary: default_boundary_for_kind(kind),
            policy,
            parent: None,
            label: None,
        })
    }

    /// Creates an empty region at a valid insertion point.
    pub fn empty(
        id: RegionId,
        index: usize,
        kind: RegionKind,
        view: &CircuitView<'_>,
    ) -> RegionResult<Self> {
        Self::new(id, index..index, kind, view)
    }

    /// Returns the invocation-local region ID.
    #[must_use]
    pub const fn id(&self) -> RegionId {
        self.id
    }

    /// Returns the inclusive/exclusive operation range.
    #[must_use]
    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    /// Returns the first operation index.
    #[must_use]
    pub const fn start(&self) -> usize {
        self.range.start
    }

    /// Returns the exclusive end index.
    #[must_use]
    pub const fn end(&self) -> usize {
        self.range.end
    }

    /// Returns the number of operations in the region.
    ///
    /// This is always safe because the range is validated on construction.
    #[must_use]
    pub fn len(&self) -> usize {
        self.range.end - self.range.start
    }

    /// Returns true if the region contains no operations.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.range.start == self.range.end
    }

    /// Returns the semantic region kind.
    #[must_use]
    pub const fn kind(&self) -> RegionKind {
        self.kind
    }

    /// Returns the entry boundary policy.
    #[must_use]
    pub const fn entry_boundary(&self) -> RegionBoundary {
        self.entry_boundary
    }

    /// Returns the exit boundary policy.
    #[must_use]
    pub const fn exit_boundary(&self) -> RegionBoundary {
        self.exit_boundary
    }

    /// Returns the region optimization policy.
    #[must_use]
    pub const fn policy(&self) -> RegionPolicy {
        self.policy
    }

    /// Returns the optional parent region.
    #[must_use]
    pub const fn parent(&self) -> Option<RegionId> {
        self.parent
    }

    /// Returns the optional diagnostic/provenance label.
    #[must_use]
    pub fn label(&self) -> Option<&RegionLabel> {
        self.label.as_ref()
    }

    /// Sets the region's entry and exit boundary policies.
    pub fn with_boundaries(
        mut self,
        entry: RegionBoundary,
        exit: RegionBoundary,
    ) -> Self {
        self.entry_boundary = entry;
        self.exit_boundary = exit;
        self
    }

    /// Sets the region optimization policy.
    pub fn with_policy(
        mut self,
        policy: RegionPolicy,
    ) -> Self {
        self.policy = policy;
        self
    }

    /// Sets the parent region.
    pub fn with_parent(
        mut self,
        parent: RegionId,
    ) -> Self {
        self.parent = Some(parent);
        self
    }

    /// Sets a diagnostic/provenance label.
    pub fn with_label(
        mut self,
        label: RegionLabel,
    ) -> Self {
        self.label = Some(label);
        self
    }

    /// Returns true when an operation index lies inside the region.
    #[must_use]
    pub const fn contains_index(
        &self,
        index: usize,
    ) -> bool {
        index >= self.range.start && index < self.range.end
    }

    /// Returns true when the region contains an operation ID from the same
    /// invocation.
    #[must_use]
    pub const fn contains_operation(
        &self,
        operation: OperationId,
    ) -> bool {
        self.contains_index(operation.index())
    }

    /// Returns true when this region fully contains another region.
    #[must_use]
    pub fn contains_region(
        &self,
        other: &Self,
    ) -> bool {
        self.range.start <= other.range.start
            && other.range.end <= self.range.end
    }

    /// Returns true when this region strictly contains another region.
    #[must_use]
    pub fn strictly_contains_region(
        &self,
        other: &Self,
    ) -> bool {
        self.contains_region(other) && self.range != other.range
    }

    /// Returns true when the regions have at least one operation in common.
    ///
    /// Two empty regions at the same insertion point do not overlap.
    #[must_use]
    pub fn overlaps(
        &self,
        other: &Self,
    ) -> bool {
        self.range.start < other.range.end
            && other.range.start < self.range.end
    }

    /// Returns true when the two regions are adjacent.
    #[must_use]
    pub fn is_adjacent(
        &self,
        other: &Self,
    ) -> bool {
        self.range.end == other.range.start
            || other.range.end == self.range.start
    }

    /// Returns true when the two regions are disjoint.
    #[must_use]
    pub fn is_disjoint(
        &self,
        other: &Self,
    ) -> bool {
        !self.overlaps(other)
    }

    /// Returns the intersection of two regions when one exists.
    #[must_use]
    pub fn intersection(
        &self,
        other: &Self,
    ) -> Option<Range<usize>> {
        let start = self.range.start.max(other.range.start);
        let end = self.range.end.min(other.range.end);

        (start < end).then_some(start..end)
    }

    /// Returns the smallest range containing both regions.
    ///
    /// This operation is purely structural. It does not claim that the union
    /// is semantically optimizable as one region.
    #[must_use]
    pub fn hull(
        &self,
        other: &Self,
    ) -> Range<usize> {
        self.range.start.min(other.range.start)
            ..self.range.end.max(other.range.end)
    }

    /// Returns true when this region can be treated as an ordinary local
    /// optimization region under its own policy.
    #[must_use]
    pub const fn permits_local_optimization(&self) -> bool {
        self.policy.allow_local_rewrites && !self.policy.protected
    }

    /// Returns true when operation reordering is permitted by this region.
    #[must_use]
    pub const fn permits_reordering(&self) -> bool {
        self.policy.allow_reordering && !self.policy.protected
    }

    /// Returns true when this region is protected.
    #[must_use]
    pub const fn is_protected(&self) -> bool {
        self.policy.protected || matches!(self.kind, RegionKind::Protected)
    }

    /// Returns true if crossing this region's entry or exit requires explicit
    /// permission.
    #[must_use]
    pub const fn requires_boundary_permission(&self) -> bool {
        self.entry_boundary.requires_permission()
            || self.exit_boundary.requires_permission()
    }

    /// Validates this region against a circuit view.
    pub fn validate(
        &self,
        view: &CircuitView<'_>,
    ) -> RegionResult<()> {
        validate_range(&self.range, view.len())
    }

    /// Creates an immutable view over this region.
    pub fn view<'a>(
        &'a self,
        circuit: &'a CircuitView<'a>,
    ) -> RegionResult<RegionView<'a>> {
        self.validate(circuit)?;

        Ok(RegionView {
            region: self,
            circuit: *circuit,
        })
    }
}

impl Ord for Region {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start()
            .cmp(&other.start())
            .then_with(|| self.end().cmp(&other.end()))
            .then_with(|| self.kind.cmp(&other.kind))
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for Region {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Region {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{} {}..{}",
            self.kind.as_str(),
            self.start(),
            self.end()
        )
    }
}

// =============================================================================
// Region view
// =============================================================================

/// Immutable view of the canonical circuit restricted to one region.
#[derive(Clone, Copy)]
pub struct RegionView<'a> {
    region: &'a Region,
    circuit: CircuitView<'a>,
}

impl<'a> fmt::Debug for RegionView<'a> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("RegionView")
            .field("region", &self.region)
            .field("circuit_len", &self.circuit.len())
            .finish()
    }
}

impl<'a> RegionView<'a> {
    /// Returns the underlying region descriptor.
    #[must_use]
    pub const fn region(&self) -> &'a Region {
        self.region
    }

    /// Returns the underlying canonical circuit view.
    #[must_use]
    pub const fn circuit(&self) -> CircuitView<'a> {
        self.circuit
    }

    /// Returns the region ID.
    #[must_use]
    pub const fn id(&self) -> RegionId {
        self.region.id()
    }

    /// Returns the region's operation range.
    #[must_use]
    pub fn range(&self) -> Range<usize> {
        self.region.range()
    }

    /// Returns the number of operations in the region.
    #[must_use]
    pub fn len(&self) -> usize {
        self.region.len()
    }

    /// Returns true when the region contains no operations.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.region.is_empty()
    }

    /// Returns the region kind.
    #[must_use]
    pub const fn kind(&self) -> RegionKind {
        self.region.kind()
    }

    /// Returns an operation by region-relative index.
    pub fn operation(
        &self,
        relative_index: usize,
    ) -> RegionResult<OperationRef<'a>> {
        let absolute_index = self
            .region
            .start()
            .checked_add(relative_index)
            .ok_or(RegionError::ArithmeticOverflow {
                calculation: "region-relative operation index",
            })?;

        if absolute_index >= self.region.end() {
            return Err(RegionError::OperationOutsideRegion {
                index: absolute_index,
                region: self.region.range(),
            });
        }

        self.circuit
            .operation(absolute_index)
            .map_err(|_| RegionError::OperationOutsideRegion {
                index: absolute_index,
                region: self.region.range(),
            })
    }

    /// Returns an operation by absolute circuit index after verifying that the
    /// operation belongs to this region.
    pub fn operation_absolute(
        &self,
        absolute_index: usize,
    ) -> RegionResult<OperationRef<'a>> {
        if !self.region.contains_index(absolute_index) {
            return Err(RegionError::OperationOutsideRegion {
                index: absolute_index,
                region: self.region.range(),
            });
        }

        self.circuit
            .operation(absolute_index)
            .map_err(|_| RegionError::OperationOutsideRegion {
                index: absolute_index,
                region: self.region.range(),
            })
    }

    /// Returns a deterministic iterator over operations in the region.
    pub fn iter(&self) -> RegionOperationIter<'a> {
        RegionOperationIter {
            circuit: self.circuit,
            next: self.region.start(),
            end: self.region.end(),
        }
    }

    /// Returns the logical number of qubits in the underlying circuit.
    #[must_use]
    pub fn num_qubits(&self) -> usize {
        self.circuit.num_qubits()
    }

    /// Returns the logical number of classical bits in the underlying
    /// circuit.
    #[must_use]
    pub fn num_classical_bits(&self) -> usize {
        self.circuit.num_classical_bits()
    }

    /// Returns true when the region allows ordinary local rewrites.
    #[must_use]
    pub const fn permits_local_optimization(&self) -> bool {
        self.region.permits_local_optimization()
    }

    /// Returns true when operation movement is permitted inside this region.
    #[must_use]
    pub const fn permits_reordering(&self) -> bool {
        self.region.permits_reordering()
    }

    /// Returns true when the region is protected.
    #[must_use]
    pub const fn is_protected(&self) -> bool {
        self.region.is_protected()
    }
}

// =============================================================================
// Region operation iterator
// =============================================================================

/// Deterministic immutable iterator over canonical operations in a region.
#[derive(Clone, Copy)]
pub struct RegionOperationIter<'a> {
    circuit: CircuitView<'a>,
    next: usize,
    end: usize,
}

impl<'a> Iterator for RegionOperationIter<'a> {
    type Item = RegionResult<OperationRef<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= self.end {
            return None;
        }

        let index = self.next;
        self.next += 1;

        Some(
            self.circuit
                .operation(index)
                .map_err(|_| RegionError::OperationOutsideRegion {
                    index,
                    region: index..self.end,
                }),
        )
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end.saturating_sub(self.next);
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for RegionOperationIter<'_> {}

impl std::iter::FusedIterator for RegionOperationIter<'_> {}

// =============================================================================
// Region collection
// =============================================================================

/// Deterministic collection of optimizer regions belonging to one circuit
/// snapshot.
///
/// `RegionSet` owns descriptors, not quantum operations.
#[derive(Debug, Clone, Default)]
pub struct RegionSet {
    regions: Vec<Region>,
}

impl RegionSet {
    /// Creates an empty region set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            regions: Vec::new(),
        }
    }

    /// Creates a region set with a known collection capacity.
    ///
    /// This is a memory reservation only. It does not impose a semantic
    /// maximum.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            regions: Vec::with_capacity(capacity),
        }
    }

    /// Returns the number of regions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.regions.len()
    }

    /// Returns true if there are no regions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.regions.is_empty()
    }

    /// Returns the region at a deterministic collection index.
    pub fn get(
        &self,
        index: usize,
    ) -> RegionResult<&Region> {
        self.regions
            .get(index)
            .ok_or(RegionError::IndexOutOfRange {
                index,
                len: self.regions.len(),
            })
    }

    /// Returns all regions in insertion order.
    #[must_use]
    pub fn as_slice(&self) -> &[Region] {
        &self.regions
    }

    /// Returns an iterator in insertion order.
    pub fn iter(&self) -> std::slice::Iter<'_, Region> {
        self.regions.iter()
    }

    /// Adds a region after validating ID uniqueness.
    pub fn push(
        &mut self,
        region: Region,
    ) -> RegionResult<()> {
        if self.regions.iter().any(|existing| existing.id() == region.id()) {
            return Err(RegionError::DuplicateRegionId { id: region.id() });
        }

        self.regions.push(region);
        Ok(())
    }

    /// Sorts regions into the canonical deterministic ordering.
    pub fn sort_deterministic(&mut self) {
        self.regions.sort();
    }

    /// Returns regions sorted by their canonical deterministic order without
    /// modifying the collection.
    #[must_use]
    pub fn sorted(&self) -> Vec<&Region> {
        let mut regions: Vec<&Region> = self.regions.iter().collect();
        regions.sort();
        regions
    }

    /// Finds a region by ID.
    #[must_use]
    pub fn find(
        &self,
        id: RegionId,
    ) -> Option<&Region> {
        self.regions.iter().find(|region| region.id() == id)
    }

    /// Finds the innermost region containing an operation index.
    ///
    /// If several nested regions contain the operation, the smallest region
    /// is returned.
    #[must_use]
    pub fn innermost_containing(
        &self,
        index: usize,
    ) -> Option<&Region> {
        self.regions
            .iter()
            .filter(|region| region.contains_index(index))
            .min_by(|a, b| {
                a.len()
                    .cmp(&b.len())
                    .then_with(|| a.start().cmp(&b.start()))
                    .then_with(|| a.id().cmp(&b.id()))
            })
    }

    /// Finds the outermost region containing an operation index.
    #[must_use]
    pub fn outermost_containing(
        &self,
        index: usize,
    ) -> Option<&Region> {
        self.regions
            .iter()
            .filter(|region| region.contains_index(index))
            .max_by(|a, b| {
                a.len()
                    .cmp(&b.len())
                    .then_with(|| b.start().cmp(&a.start()))
                    .then_with(|| b.id().cmp(&a.id()))
            })
    }

    /// Validates IDs, ranges, and nesting relationships.
    pub fn validate(
        &self,
        view: &CircuitView<'_>,
    ) -> RegionResult<()> {
        for region in &self.regions {
            region.validate(view)?;
        }

        for (index, first) in self.regions.iter().enumerate() {
            for second in self.regions.iter().skip(index + 1) {
                if first.id() == second.id() {
                    return Err(RegionError::DuplicateRegionId {
                        id: first.id(),
                    });
                }

                if first.parent() == Some(second.id())
                    && !second.contains_region(first)
                {
                    return Err(RegionError::ParentDoesNotContainChild {
                        parent: second.id(),
                        child: first.id(),
                    });
                }

                if second.parent() == Some(first.id())
                    && !first.contains_region(second)
                {
                    return Err(RegionError::ParentDoesNotContainChild {
                        parent: first.id(),
                        child: second.id(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Validates that the regions form a complete non-overlapping partition
    /// of `range`.
    ///
    /// Nested regions are not valid in a partition because a partition is
    /// specifically a flat decomposition.
    pub fn validate_partition(
        &self,
        range: Range<usize>,
    ) -> RegionResult<()> {
        let mut regions: Vec<&Region> = self
            .regions
            .iter()
            .filter(|region| region.start() >= range.start && region.end() <= range.end)
            .collect();

        regions.sort();

        if regions.is_empty() {
            if range.start == range.end {
                return Ok(());
            }

            return Err(RegionError::PartitionGap {
                start: range.start,
                end: range.end,
            });
        }

        let mut cursor = range.start;

        for region in regions {
            if region.start() < cursor {
                return Err(RegionError::PartitionOverlap {
                    first: region.id(),
                    second: region.id(),
                });
            }

            if region.start() > cursor {
                return Err(RegionError::PartitionGap {
                    start: cursor,
                    end: region.start(),
                });
            }

            cursor = region.end();
        }

        if cursor != range.end {
            return Err(RegionError::PartitionGap {
                start: cursor,
                end: range.end,
            });
        }

        Ok(())
    }

    /// Returns the number of regions that overlap the supplied range.
    #[must_use]
    pub fn count_overlapping(
        &self,
        range: Range<usize>,
    ) -> usize {
        self.regions
            .iter()
            .filter(|region| {
                region.range().start < range.end
                    && range.start < region.range().end
            })
            .count()
    }
}

impl<'a> IntoIterator for &'a RegionSet {
    type Item = &'a Region;
    type IntoIter = std::slice::Iter<'a, Region>;

    fn into_iter(self) -> Self::IntoIter {
        self.regions.iter()
    }
}

// =============================================================================
// Region tree
// =============================================================================

/// Deterministic parent/child hierarchy of nested optimization regions.
///
/// The tree stores only region IDs. The actual region descriptors remain in
/// `RegionSet`.
#[derive(Debug, Clone, Default)]
pub struct RegionTree {
    roots: Vec<RegionId>,
    children: Vec<(RegionId, Vec<RegionId>)>,
}

impl RegionTree {
    /// Builds a deterministic region tree from a region collection.
    ///
    /// Parent relationships explicitly declared through `Region::with_parent`
    /// are checked. For regions without an explicit parent, the smallest
    /// strictly containing region is selected.
    pub fn build(
        regions: &RegionSet,
    ) -> RegionResult<Self> {
        let mut sorted = regions.sorted();

        // Deterministic ordering is required for deterministic implicit parent
        // selection.
        sorted.sort();

        let mut roots = Vec::new();
        let mut children: Vec<(RegionId, Vec<RegionId>)> = Vec::new();

        for region in &sorted {
            children.push((region.id(), Vec::new()));
        }

        for region in &sorted {
            let parent = if let Some(explicit_parent) = region.parent() {
                let parent_region = regions
                    .find(explicit_parent)
                    .ok_or(RegionError::InvalidRelationship {
                        first: region.id(),
                        second: explicit_parent,
                        message: "explicit parent does not exist",
                    })?;

                if !parent_region.strictly_contains_region(region) {
                    return Err(RegionError::ParentDoesNotContainChild {
                        parent: explicit_parent,
                        child: region.id(),
                    });
                }

                Some(explicit_parent)
            } else {
                sorted
                    .iter()
                    .filter(|candidate| {
                        candidate.id() != region.id()
                            && candidate.strictly_contains_region(region)
                    })
                    .min_by(|a, b| {
                        a.len()
                            .cmp(&b.len())
                            .then_with(|| a.start().cmp(&b.start()))
                            .then_with(|| a.id().cmp(&b.id()))
                    })
                    .map(|candidate| candidate.id())
            };

            if let Some(parent) = parent {
                if let Some((_, child_ids)) = children
                    .iter_mut()
                    .find(|(id, _)| *id == parent)
                {
                    child_ids.push(region.id());
                } else {
                    return Err(RegionError::InvalidRelationship {
                        first: region.id(),
                        second: parent,
                        message: "parent region is not present in region tree",
                    });
                }
            } else {
                roots.push(region.id());
            }
        }

        roots.sort();

        for (_, child_ids) in &mut children {
            child_ids.sort();
        }

        // Validate that no siblings overlap.
        for (_, child_ids) in &children {
            for (left_index, left_id) in child_ids.iter().enumerate() {
                for right_id in child_ids.iter().skip(left_index + 1) {
                    let left = regions.find(*left_id).ok_or(
                        RegionError::InvalidRelationship {
                            first: *left_id,
                            second: *right_id,
                            message: "child region disappeared during tree construction",
                        },
                    )?;

                    let right = regions.find(*right_id).ok_or(
                        RegionError::InvalidRelationship {
                            first: *left_id,
                            second: *right_id,
                            message: "child region disappeared during tree construction",
                        },
                    )?;

                    if left.overlaps(right) {
                        return Err(RegionError::OverlappingSiblings {
                            first: left.id(),
                            second: right.id(),
                        });
                    }
                }
            }
        }

        Ok(Self { roots, children })
    }

    /// Returns root region IDs in deterministic order.
    #[must_use]
    pub fn roots(&self) -> &[RegionId] {
        &self.roots
    }

    /// Returns child IDs for a region.
    #[must_use]
    pub fn children(
        &self,
        parent: RegionId,
    ) -> &[RegionId] {
        self.children
            .iter()
            .find(|(id, _)| *id == parent)
            .map(|(_, children)| children.as_slice())
            .unwrap_or(&[])
    }

    /// Returns whether a region has children.
    #[must_use]
    pub fn has_children(
        &self,
        parent: RegionId,
    ) -> bool {
        !self.children(parent).is_empty()
    }
}

// =============================================================================
// Region partition helpers
// =============================================================================

/// Creates a deterministic flat partition of `range` from lengths.
///
/// Example:
///
/// ```text
/// partition(0..10, [3, 2, 5])
///     => 0..3, 3..5, 5..10
/// ```
///
/// Every requested length must be non-overflowing and the lengths must sum
/// exactly to the requested range length.
pub fn partition_by_lengths(
    range: Range<usize>,
    lengths: &[usize],
) -> RegionResult<Vec<Range<usize>>> {
    validate_range_basic(&range)?;

    let expected = range.end - range.start;

    let mut total = 0usize;

    for &length in lengths {
        total = total
            .checked_add(length)
            .ok_or(RegionError::ArithmeticOverflow {
                calculation: "region partition length",
            })?;
    }

    if total != expected {
        return Err(RegionError::InvalidPartition {
            expected: range,
            actual: range.start..range.start + total,
        });
    }

    let mut result = Vec::with_capacity(lengths.len());
    let mut cursor = range.start;

    for &length in lengths {
        let end = cursor
            .checked_add(length)
            .ok_or(RegionError::ArithmeticOverflow {
                calculation: "region partition end",
            })?;

        result.push(cursor..end);
        cursor = end;
    }

    Ok(result)
}

/// Creates a partition from inclusive operation intervals.
///
/// Each interval is converted into a half-open region range.
///
/// ```text
/// 0..=2 -> 0..3
/// ```
pub fn inclusive_ranges_to_regions(
    ranges: &[RangeInclusive<usize>],
) -> RegionResult<Vec<Range<usize>>> {
    let mut result = Vec::with_capacity(ranges.len());

    for range in ranges {
        let start = *range.start();
        let end_inclusive = *range.end();

        let end = end_inclusive
            .checked_add(1)
            .ok_or(RegionError::ArithmeticOverflow {
                calculation: "inclusive region end",
            })?;

        if start > end_inclusive {
            return Err(RegionError::InvalidRelationship {
                first: RegionId::new(0),
                second: RegionId::new(0),
                message: "inclusive region range has start greater than end",
            });
        }

        result.push(start..end);
    }

    Ok(result)
}

// =============================================================================
// Internal helpers
// =============================================================================

fn validate_range(
    range: &Range<usize>,
    circuit_len: usize,
) -> RegionResult<()> {
    validate_range_basic(range)?;

    if range.end > circuit_len {
        return Err(RegionError::RangeOutOfBounds {
            start: range.start,
            end: range.end,
            circuit_len,
        });
    }

    Ok(())
}

fn validate_range_basic(
    range: &Range<usize>,
) -> RegionResult<()> {
    if range.start > range.end {
        return Err(RegionError::RangeOutOfBounds {
            start: range.start,
            end: range.end,
            circuit_len: range.end,
        });
    }

    Ok(())
}

const fn default_policy_for_kind(
    kind: RegionKind,
) -> RegionPolicy {
    match kind {
        RegionKind::Linear | RegionKind::Block => RegionPolicy::local(),

        RegionKind::LoopHeader
        | RegionKind::LoopBody
        | RegionKind::LoopLatch
        | RegionKind::LoopCarried
        | RegionKind::ConditionalArm
        | RegionKind::ClassicalControl => RegionPolicy::conservative(),

        RegionKind::MeasurementDelimited
        | RegionKind::ResetDelimited => RegionPolicy {
            allow_local_rewrites: true,
            allow_reordering: false,
            allow_fusion: false,
            allow_structural_change: false,
            protected: false,
            require_equivalence_proof: true,
        },

        RegionKind::Protected => RegionPolicy::protected(),

        RegionKind::CompilerGenerated => RegionPolicy {
            allow_local_rewrites: true,
            allow_reordering: false,
            allow_fusion: false,
            allow_structural_change: true,
            protected: false,
            require_equivalence_proof: true,
        },

        RegionKind::Logical => RegionPolicy {
            allow_local_rewrites: true,
            allow_reordering: false,
            allow_fusion: false,
            allow_structural_change: true,
            protected: false,
            require_equivalence_proof: true,
        },

        RegionKind::TargetSpecific => RegionPolicy::conservative(),

        RegionKind::External => RegionPolicy::conservative(),
    }
}

const fn default_boundary_for_kind(
    kind: RegionKind,
) -> RegionBoundary {
    match kind {
        RegionKind::Linear | RegionKind::Block => RegionBoundary::Open,

        RegionKind::LoopHeader
        | RegionKind::LoopBody
        | RegionKind::LoopLatch => RegionBoundary::Structural,

        RegionKind::LoopCarried
        | RegionKind::ConditionalArm
        | RegionKind::ClassicalControl => RegionBoundary::Guarded,

        RegionKind::MeasurementDelimited
        | RegionKind::ResetDelimited
        | RegionKind::Protected
        | RegionKind::Logical
        | RegionKind::TargetSpecific => RegionBoundary::Closed,

        RegionKind::CompilerGenerated | RegionKind::External => {
            RegionBoundary::Guarded
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Test helpers
    // -------------------------------------------------------------------------

    fn empty_view<'a>() -> CircuitView<'a> {
        panic!(
            "These tests intentionally avoid constructing a repository-specific \
             QuantumCircuit because region.rs must remain independent of circuit \
             construction APIs."
        );
    }

    // -------------------------------------------------------------------------
    // Pure range semantics
    // -------------------------------------------------------------------------

    #[test]
    fn empty_range_is_valid() {
        assert!(validate_range_basic(&(4..4)).is_ok());
    }

    #[test]
    fn reversed_range_is_rejected() {
        assert!(validate_range_basic(&(5..4)).is_err());
    }

    #[test]
    fn partition_by_lengths_is_exact() {
        let ranges = partition_by_lengths(
            0..10,
            &[3, 2, 5],
        )
        .expect("valid partition");

        assert_eq!(
            ranges,
            vec![0..3, 3..5, 5..10]
        );
    }

    #[test]
    fn partition_by_lengths_rejects_gap_or_excess() {
        let result = partition_by_lengths(
            0..10,
            &[3, 2],
        );

        assert!(matches!(
            result,
            Err(RegionError::InvalidPartition { .. })
        ));
    }

    #[test]
    fn inclusive_ranges_are_converted_safely() {
        let ranges = inclusive_ranges_to_regions(
            &[0..=2, 3..=4],
        )
        .expect("valid ranges");

        assert_eq!(
            ranges,
            vec![0..3, 3..5]
        );
    }

    #[test]
    fn inclusive_max_value_is_rejected_without_overflow() {
        let result = inclusive_ranges_to_regions(
            &[usize::MAX..=usize::MAX],
        );

        assert!(matches!(
            result,
            Err(RegionError::ArithmeticOverflow { .. })
        ));
    }

    #[test]
    fn region_kind_classification_is_stable() {
        assert_eq!(
            RegionKind::LoopBody.as_str(),
            "loop_body"
        );

        assert!(RegionKind::LoopBody.is_control_flow());
        assert!(RegionKind::LoopBody.is_boundary_sensitive());

        assert!(
            RegionKind::Protected
                .is_boundary_sensitive()
        );

        assert!(
            !RegionKind::Linear
                .is_boundary_sensitive()
        );
    }

    #[test]
    fn conservative_policy_is_not_reordering() {
        let policy = RegionPolicy::conservative();

        assert!(policy.allow_local_rewrites);
        assert!(!policy.allow_reordering);
        assert!(!policy.allow_fusion);
        assert!(!policy.allow_structural_change);
        assert!(!policy.protected);
        assert!(policy.require_equivalence_proof);
    }

    #[test]
    fn protected_policy_blocks_all_local_mutation() {
        let policy = RegionPolicy::protected();

        assert!(!policy.allow_local_rewrites);
        assert!(!policy.allow_reordering);
        assert!(!policy.allow_fusion);
        assert!(!policy.allow_structural_change);
        assert!(policy.protected);
        assert!(policy.require_equivalence_proof);
    }

    // -------------------------------------------------------------------------
    // Region relationship semantics
    // -------------------------------------------------------------------------

    #[test]
    fn ranges_have_expected_relationships() {
        let a = test_region_unchecked(
            RegionId::new(0),
            0..4,
            RegionKind::Linear,
        );

        let b = test_region_unchecked(
            RegionId::new(1),
            1..3,
            RegionKind::Block,
        );

        let c = test_region_unchecked(
            RegionId::new(2),
            4..7,
            RegionKind::Block,
        );

        assert!(a.contains_region(&b));
        assert!(a.strictly_contains_region(&b));
        assert!(a.is_disjoint(&c));
        assert!(a.is_adjacent(&c));
        assert!(a.overlaps(&b));

        assert_eq!(
            a.intersection(&b),
            Some(1..3)
        );

        assert_eq!(
            a.hull(&c),
            0..7
        );
    }

    #[test]
    fn equal_ranges_are_containing_but_not_strictly_containing() {
        let a = test_region_unchecked(
            RegionId::new(0),
            1..4,
            RegionKind::Block,
        );

        let b = test_region_unchecked(
            RegionId::new(1),
            1..4,
            RegionKind::Block,
        );

        assert!(a.contains_region(&b));
        assert!(!a.strictly_contains_region(&b));
        assert!(a.overlaps(&b));
    }

    #[test]
    fn empty_regions_do_not_overlap() {
        let a = test_region_unchecked(
            RegionId::new(0),
            4..4,
            RegionKind::Linear,
        );

        let b = test_region_unchecked(
            RegionId::new(1),
            4..4,
            RegionKind::Linear,
        );

        assert!(!a.overlaps(&b));
        assert!(a.is_adjacent(&b));
    }

    // -------------------------------------------------------------------------
    // Region tree tests
    // -------------------------------------------------------------------------

    #[test]
    fn region_tree_builds_nested_regions() {
        let mut regions = RegionSet::new();

        regions
            .regions
            .push(test_region_unchecked(
                RegionId::new(0),
                0..10,
                RegionKind::Block,
            ));

        regions
            .regions
            .push(
                test_region_unchecked(
                    RegionId::new(1),
                    2..8,
                    RegionKind::LoopBody,
                )
                .with_parent(RegionId::new(0)),
            );

        regions
            .regions
            .push(
                test_region_unchecked(
                    RegionId::new(2),
                    3..6,
                    RegionKind::Block,
                )
                .with_parent(RegionId::new(1)),
            );

        let tree = RegionTree::build(&regions)
            .expect("valid nested region tree");

        assert_eq!(
            tree.roots(),
            &[RegionId::new(0)]
        );

        assert_eq!(
            tree.children(RegionId::new(0)),
            &[RegionId::new(1)]
        );

        assert_eq!(
            tree.children(RegionId::new(1)),
            &[RegionId::new(2)]
        );
    }

    #[test]
    fn region_tree_rejects_invalid_explicit_parent() {
        let mut regions = RegionSet::new();

        regions
            .regions
            .push(
                test_region_unchecked(
                    RegionId::new(1),
                    0..2,
                    RegionKind::Block,
                )
                .with_parent(RegionId::new(99)),
            );

        let result = RegionTree::build(&regions);

        assert!(result.is_err());
    }

    // -------------------------------------------------------------------------
    // Test-only region constructor
    // -------------------------------------------------------------------------
    //
    // This does not bypass production validation. It is deliberately limited to
    // pure range/relationship tests where no QuantumCircuit construction should
    // be required.

    fn test_region_unchecked(
        id: RegionId,
        range: Range<usize>,
        kind: RegionKind,
    ) -> Region {
        Region {
            id,
            range,
            kind,
            entry_boundary: default_boundary_for_kind(kind),
            exit_boundary: default_boundary_for_kind(kind),
            policy: default_policy_for_kind(kind),
            parent: None,
            label: None,
        }
    }
}