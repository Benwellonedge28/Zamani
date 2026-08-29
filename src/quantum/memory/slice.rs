//! Zamani Quantum Memory — Safe Quantum-Memory Slices
//!
//! This module defines the representation-independent contract for selecting
//! subsets of logical qubits from quantum memory.
//!
//! # Architectural responsibility
//!
//! `memory::slice` owns:
//!
//! - validated qubit selections;
//! - ordered subsets of logical qubits;
//! - complement/discarded-qubit calculation;
//! - explicit slice operation semantics;
//! - view/copy/projection/partial-trace planning;
//! - logical-to-storage-position resolution through `MemoryLayout`;
//! - safe basis-bit projection metadata;
//! - deterministic slice descriptions;
//! - serialization-compatible slice descriptors.
//!
//! It does NOT own:
//!
//! - quantum amplitudes;
//! - state-vector storage;
//! - density-matrix storage;
//! - stabilizer/tableau storage;
//! - sparse-state storage;
//! - tensor-network storage;
//! - allocation;
//! - copying of actual state data;
//! - measurement;
//! - measurement collapse;
//! - partial-trace mathematics;
//! - projection mathematics;
//! - routing;
//! - scheduling;
//! - hardware communication;
//! - GPU APIs;
//! - distributed communication.
//!
//! Those responsibilities remain with their owning modules.
//!
//! # Critical semantic distinction
//!
//! A quantum-memory "slice" is NOT automatically a smaller quantum state.
//!
//! For an entangled state, selecting qubits can mean several fundamentally
//! different things:
//!
//! ```text
//! View
//!     non-owning selection of the original memory/state
//!
//! Copy
//!     independent materialized data selected from the source
//!
//! Projection
//!     projection onto explicitly supplied computational-basis assignments
//!
//! PartialTrace
//!     mathematical reduction obtained by tracing out discarded qubits
//! ```
//!
//! These operations must never be silently conflated.
//!
//! `slice.rs` therefore describes the operation to be performed but does not
//! perform the quantum mathematics itself.
//!
//! # Example
//!
//! Given:
//!
//! ```text
//! q0 q1 q2 q3
//! ```
//!
//! selecting:
//!
//! ```text
//! q3 q1
//! ```
//!
//! creates an ordered selection:
//!
//! ```text
//! retained = [q3, q1]
//! discarded = [q0, q2]
//! ```
//!
//! The order is significant. State/tensor implementations may use it when
//! determining the order of the resulting subsystem.
//!
//! # Hardware neutrality
//!
//! This module is intentionally independent of the physical execution model.
//! The same slice contract can therefore be consumed by:
//!
//! - CPU simulators;
//! - GPU simulators;
//! - distributed simulators;
//! - superconducting QPUs;
//! - trapped-ion QPUs;
//! - neutral-atom systems;
//! - photonic systems;
//! - spin/semiconductor systems;
//! - annealing-oriented backends;
//! - remote/cloud QPUs;
//! - future hardware not known today.
//!
//! A QPU implementation decides whether a particular slice operation is
//! supported. This module merely supplies a validated, provider-neutral plan.
//!
//! # Dependency direction
//!
//! ```text
//! quantum::ir::QubitId
//!          │
//!          ▼
//! memory::types ──────────────┐
//!          │                  │
//!          ▼                  │
//! memory::slice ◄── memory::layout
//!          │                  │
//!          ├──► state_vector  │
//!          ├──► density_matrix│
//!          ├──► stabilizer    │
//!          ├──► sparse        │
//!          ├──► tensor_network│
//!          ├──► view          │
//!          ├──► permutation   │
//!          ├──► serialization │
//!          └──► hardware      │
//! ```
//!
//! `slice.rs` does not depend on routing, scheduling, benchmarking, or a
//! particular backend.
//!
//! # Canonical identity rule
//!
//! The canonical logical qubit identifier remains:
//!
//! ```text
//! crate::quantum::ir::QubitId
//! ```
//!
//! `slice.rs` does not define a replacement qubit identifier.
//!
//! # No unsafe
//!
//! This module intentionally contains no `unsafe` code and exposes no raw
//! pointers, references into unmanaged storage, device pointers, or backend
//! handles.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! The completed API is designed so later state representations do not need to
//! reopen this file merely because they are implemented.
//!
//! `state_vector.rs` may use `SlicePlan` to select basis-state dimensions.
//!
//! `density_matrix.rs` may use `SlicePlan::PartialTrace` or
//! `SlicePlan::Projection` to select retained/discarded subsystems.
//!
//! `stabilizer.rs` may use the same logical selection without assuming dense
//! storage.
//!
//! `sparse.rs` may use the selected logical basis positions when filtering
//! sparse basis states.
//!
//! `tensor_network.rs` may use the ordered retained-qubit list as tensor-leg
//! selection metadata.
//!
//! `view.rs` may use `SlicePlan::View` without copying state.
//!
//! `serialization.rs`, `snapshot.rs`, and `checkpoint.rs` may persist the
//! descriptor because all semantic information is explicit and deterministic.
//!
//! `routing.rs` may use `storage_positions()` to translate the logical
//! selection into the current memory layout, without making `slice.rs` depend
//! on routing.
//!
//! Hardware adapters can inspect the operation kind and selected physical
//! positions after layout resolution without requiring any vendor-specific
//! types here.
//!
//! The actual quantum operation remains the responsibility of the consuming
//! state/backend implementation.
//!
//! # Important invariants
//!
//! 1. Every selected qubit is within the source register.
//! 2. No selected qubit occurs twice.
//! 3. Selection order is preserved exactly.
//! 4. The complement contains every non-selected qubit exactly once.
//! 5. A projection contains explicit assignments for every discarded qubit.
//! 6. Projection assignments cannot target retained qubits.
//! 7. Partial trace does not silently become projection.
//! 8. View does not imply ownership.
//! 9. Copy does not imply a mathematical partial trace.
//! 10. No allocation or state mutation occurs here except construction of the
//!     small descriptor itself.
//! 11. Layout resolution is checked.
//! 12. No raw memory address is exposed.
//! 13. No hardware-specific assumption is made.
//! 14. All public fallible operations use `MemoryError`.
//!
//! # Design rationale
//!
//! Mature quantum simulators need to distinguish logical-qubit selection from
//! the underlying representation. A sparse simulator, state-vector simulator,
//! density-matrix simulator, and tensor-network simulator cannot all implement
//! "slice" as the same physical memory operation. The common contract must
//! therefore describe *what the caller means*, not *how a backend stores it*.
//!
//! This module follows that rule.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use serde::{Deserialize, Serialize};

use crate::quantum::ir::QubitId;

use super::errors::MemoryError;
use super::layout::MemoryLayout;
use super::types::QubitCount;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for quantum-memory slice descriptors.
pub const MEMORY_SLICE_SCHEMA_ID: &str = "zamani.quantum.memory.slice";

/// Current semantic schema version.
pub const MEMORY_SLICE_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Slice operation
// =============================================================================

/// Semantic operation requested for a selected quantum subsystem.
///
/// These variants deliberately describe different mathematical/ownership
/// semantics. A backend must not silently substitute one for another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SliceOperation {
    /// Create a non-owning logical view of selected qubits.
    ///
    /// No quantum state is mathematically reduced by this descriptor.
    View,

    /// Materialize selected data into independent storage.
    ///
    /// This describes copying/extraction semantics. It does not mean
    /// "partial trace" and does not perform a mathematical reduction.
    Copy,

    /// Project discarded qubits onto explicitly supplied computational-basis
    /// values and retain the selected subsystem.
    ///
    /// A projection may change normalization and therefore the state
    /// implementation must define whether the resulting object is normalized,
    /// subnormalized, or returned with the projection probability.
    Projection,

    /// Trace out all discarded qubits and retain the selected subsystem.
    ///
    /// This is a mathematical reduced-state operation and is representation
    /// dependent.
    PartialTrace,
}

impl SliceOperation {
    /// Returns whether this operation requires independent owned output.
    #[must_use]
    pub const fn requires_owned_output(self) -> bool {
        matches!(self, Self::Copy | Self::Projection | Self::PartialTrace)
    }

    /// Returns whether this operation is non-owning.
    #[must_use]
    pub const fn is_view(self) -> bool {
        matches!(self, Self::View)
    }

    /// Returns whether this operation is a mathematical state reduction.
    #[must_use]
    pub const fn is_reduction(self) -> bool {
        matches!(self, Self::Projection | Self::PartialTrace)
    }

    /// Returns whether this operation requires explicit basis assignments for
    /// discarded qubits.
    #[must_use]
    pub const fn requires_projection_assignments(self) -> bool {
        matches!(self, Self::Projection)
    }
}

// =============================================================================
// Computational-basis assignment
// =============================================================================

/// A computational-basis assignment for one logical qubit.
///
/// `value == false` represents `|0⟩`.
///
/// `value == true` represents `|1⟩`.
///
/// This type is intentionally representation independent. It does not imply
/// that a backend stores states in a computational basis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BasisAssignment {
    qubit: QubitId,
    value: bool,
}

impl BasisAssignment {
    /// Creates an explicit computational-basis assignment.
    #[must_use]
    pub const fn new(qubit: QubitId, value: bool) -> Self {
        Self { qubit, value }
    }

    /// Creates a `|0⟩` assignment.
    #[must_use]
    pub const fn zero(qubit: QubitId) -> Self {
        Self::new(qubit, false)
    }

    /// Creates a `|1⟩` assignment.
    #[must_use]
    pub const fn one(qubit: QubitId) -> Self {
        Self::new(qubit, true)
    }

    /// Returns the logical qubit being assigned.
    #[must_use]
    pub const fn qubit(self) -> QubitId {
        self.qubit
    }

    /// Returns the assigned computational-basis value.
    #[must_use]
    pub const fn value(self) -> bool {
        self.value
    }
}

// =============================================================================
// Qubit selection
// =============================================================================

/// An ordered, validated subset of logical qubits.
///
/// The order is part of the semantic contract.
///
/// For example:
///
/// ```text
/// [q3, q1]
/// ```
///
/// is not interchangeable with:
///
/// ```text
/// [q1, q3]
/// ```
///
/// even though both contain the same set of qubits.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QubitSlice {
    source_qubits: QubitCount,
    selected: Vec<QubitId>,
}

impl QubitSlice {
    /// Creates a selection containing every qubit in canonical order.
    pub fn all(source_qubits: QubitCount) -> Result<Self, MemoryError> {
        let count = source_qubits.get();

        let selected = (0..count).map(QubitId::new).collect();

        Ok(Self {
            source_qubits,
            selected,
        })
    }

    /// Creates an empty selection.
    ///
    /// An empty selection is valid and is useful for operations such as:
    ///
    /// - tracing out an entire state;
    /// - constructing zero-dimensional subsystem descriptors;
    /// - validating complete projection/discard plans.
    pub const fn empty(source_qubits: QubitCount) -> Self {
        Self {
            source_qubits,
            selected: Vec::new(),
        }
    }

    /// Creates a validated ordered selection from logical qubit identifiers.
    ///
    /// The supplied order is preserved.
    pub fn try_new(
        source_qubits: QubitCount,
        selected: Vec<QubitId>,
    ) -> Result<Self, MemoryError> {
        validate_selection(source_qubits, &selected)?;

        Ok(Self {
            source_qubits,
            selected,
        })
    }

    /// Creates a validated selection from a slice of logical qubit IDs.
    pub fn from_ids(
        source_qubits: QubitCount,
        selected: &[QubitId],
    ) -> Result<Self, MemoryError> {
        Self::try_new(source_qubits, selected.to_vec())
    }

    /// Creates a selection from an inclusive/exclusive Rust-style range.
    ///
    /// For example:
    ///
    /// ```text
    /// 1..4 -> q1, q2, q3
    /// ```
    pub fn range(
        source_qubits: QubitCount,
        range: std::ops::Range<usize>,
    ) -> Result<Self, MemoryError> {
        if range.start > range.end {
            return Err(invalid_slice(
                "range start must not be greater than range end",
            ));
        }

        if range.end > source_qubits.get() {
            return Err(invalid_slice(format!(
                "range {}..{} exceeds source qubit count {}",
                range.start,
                range.end,
                source_qubits.get()
            )));
        }

        let selected = range.map(QubitId::new).collect::<Vec<_>>();

        Self::try_new(source_qubits, selected)
    }

    /// Creates a single-qubit selection.
    pub fn single(
        source_qubits: QubitCount,
        qubit: QubitId,
    ) -> Result<Self, MemoryError> {
        Self::try_new(source_qubits, vec![qubit])
    }

    /// Returns the source qubit count.
    #[must_use]
    pub const fn source_qubits(&self) -> QubitCount {
        self.source_qubits
    }

    /// Returns the number of selected qubits.
    #[must_use]
    pub fn len(&self) -> usize {
        self.selected.len()
    }

    /// Returns whether no qubits are selected.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.selected.is_empty()
    }

    /// Returns the selected qubits in their semantic order.
    #[must_use]
    pub fn as_slice(&self) -> &[QubitId] {
        &self.selected
    }

    /// Returns the selected qubit at a subsystem position.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<QubitId> {
        self.selected.get(index).copied()
    }

    /// Returns the subsystem position of a selected logical qubit.
    #[must_use]
    pub fn position_of(&self, qubit: QubitId) -> Option<usize> {
        self.selected.iter().position(|candidate| *candidate == qubit)
    }

    /// Returns whether the logical qubit is selected.
    #[must_use]
    pub fn contains(&self, qubit: QubitId) -> bool {
        self.position_of(qubit).is_some()
    }

    /// Returns the complement of this selection.
    ///
    /// The complement is always returned in canonical logical-qubit order:
    ///
    /// ```text
    /// q0, q1, ..., q(n-1)
    /// ```
    ///
    /// with selected qubits removed.
    pub fn complement(&self) -> Result<Self, MemoryError> {
        let mut selected = Vec::with_capacity(
            self.source_qubits
                .get()
                .saturating_sub(self.selected.len()),
        );

        let mut selected_bitmap = vec![false; self.source_qubits.get()];

        for &qubit in &self.selected {
            selected_bitmap[qubit.index()] = true;
        }

        for index in 0..self.source_qubits.get() {
            if !selected_bitmap[index] {
                selected.push(QubitId::new(index));
            }
        }

        Self::try_new(self.source_qubits, selected)
    }

    /// Returns the selected qubits as logical indices.
    ///
    /// The order is preserved.
    #[must_use]
    pub fn indices(&self) -> Vec<usize> {
        self.selected
            .iter()
            .map(|qubit| qubit.index())
            .collect()
    }

    /// Returns the storage positions occupied by the selected logical qubits
    /// under the supplied immutable memory layout.
    ///
    /// This does not perform a copy or allocation of quantum state. It only
    /// resolves logical identities to storage positions.
    pub fn storage_positions(
        &self,
        layout: &MemoryLayout,
    ) -> Result<Vec<usize>, MemoryError> {
        if layout.num_qubits() != self.source_qubits.get() {
            return Err(invalid_slice(format!(
                "layout contains {} qubits but slice source contains {} qubits",
                layout.num_qubits(),
                self.source_qubits.get()
            )));
        }

        self.selected
            .iter()
            .map(|&qubit| {
                layout.position_of(qubit).map_err(|error| {
                    invalid_slice(format!(
                        "failed to resolve logical qubit {} in memory layout: {}",
                        qubit.index(),
                        error
                    ))
                })
            })
            .collect()
    }

    /// Returns the basis-index bit positions corresponding to the selected
    /// qubits under the supplied layout.
    pub fn bit_positions(
        &self,
        layout: &MemoryLayout,
    ) -> Result<Vec<usize>, MemoryError> {
        if layout.num_qubits() != self.source_qubits.get() {
            return Err(invalid_slice(format!(
                "layout contains {} qubits but slice source contains {} qubits",
                layout.num_qubits(),
                self.source_qubits.get()
            )));
        }

        self.selected
            .iter()
            .map(|&qubit| {
                layout.bit_position(qubit).map_err(|error| {
                    invalid_slice(format!(
                        "failed to resolve basis bit for logical qubit {}: {}",
                        qubit.index(),
                        error
                    ))
                })
            })
            .collect()
    }

    /// Returns a new selection containing the selected qubits in the supplied
    /// order.
    ///
    /// This is useful when a tensor/state representation needs an explicit
    /// output ordering.
    pub fn reorder(
        &self,
        order: &[QubitId],
    ) -> Result<Self, MemoryError> {
        if order.len() != self.selected.len() {
            return Err(invalid_slice(format!(
                "reorder length {} does not match selected-qubit count {}",
                order.len(),
                self.selected.len()
            )));
        }

        let mut result = Vec::with_capacity(order.len());

        for &qubit in order {
            if !self.contains(qubit) {
                return Err(invalid_slice(format!(
                    "qubit {} is not part of the original selection",
                    qubit.index()
                )));
            }

            if result.contains(&qubit) {
                return Err(invalid_slice(format!(
                    "qubit {} occurs more than once in reorder operation",
                    qubit.index()
                )));
            }

            result.push(qubit);
        }

        Self::try_new(self.source_qubits, result)
    }

    /// Returns whether this selection contains all source qubits.
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.selected.len() == self.source_qubits.get()
    }

    /// Returns whether this selection is a strict subset of the source.
    #[must_use]
    pub fn is_partial(&self) -> bool {
        !self.is_full() && !self.is_empty()
    }

    /// Returns whether this selection is empty.
    #[must_use]
    pub fn selects_none(&self) -> bool {
        self.is_empty()
    }

    /// Returns whether the selection is the canonical identity order.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.selected
            .iter()
            .enumerate()
            .all(|(index, qubit)| qubit.index() == index)
    }
}

// =============================================================================
// Slice plan
// =============================================================================

/// Fully validated semantic plan for operating on a quantum-memory slice.
///
/// `SlicePlan` is the primary integration type intended for state
/// representations and backend adapters.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SlicePlan {
    selection: QubitSlice,
    operation: SliceOperation,
    projection: Vec<BasisAssignment>,
}

impl SlicePlan {
    /// Creates a view plan.
    pub fn view(selection: QubitSlice) -> Result<Self, MemoryError> {
        Self::new(selection, SliceOperation::View, Vec::new())
    }

    /// Creates a copy plan.
    pub fn copy(selection: QubitSlice) -> Result<Self, MemoryError> {
        Self::new(selection, SliceOperation::Copy, Vec::new())
    }

    /// Creates a partial-trace plan.
    pub fn partial_trace(selection: QubitSlice) -> Result<Self, MemoryError> {
        Self::new(
            selection,
            SliceOperation::PartialTrace,
            Vec::new(),
        )
    }

    /// Creates a projection plan.
    ///
    /// Every discarded qubit must have exactly one computational-basis
    /// assignment.
    pub fn projection(
        selection: QubitSlice,
        assignments: Vec<BasisAssignment>,
    ) -> Result<Self, MemoryError> {
        Self::new(
            selection,
            SliceOperation::Projection,
            assignments,
        )
    }

    /// Creates a fully validated plan.
    pub fn new(
        selection: QubitSlice,
        operation: SliceOperation,
        projection: Vec<BasisAssignment>,
    ) -> Result<Self, MemoryError> {
        validate_operation(
            &selection,
            operation,
            &projection,
        )?;

        Ok(Self {
            selection,
            operation,
            projection,
        })
    }

    /// Returns the selected subsystem.
    #[must_use]
    pub const fn selection(&self) -> &QubitSlice {
        &self.selection
    }

    /// Returns the semantic operation.
    #[must_use]
    pub const fn operation(&self) -> SliceOperation {
        self.operation
    }

    /// Returns the projection assignments.
    ///
    /// For non-projection operations this slice is empty.
    #[must_use]
    pub fn projection_assignments(&self) -> &[BasisAssignment] {
        &self.projection
    }

    /// Returns the discarded subsystem.
    pub fn discarded(&self) -> Result<QubitSlice, MemoryError> {
        self.selection.complement()
    }

    /// Returns the number of retained qubits.
    #[must_use]
    pub fn retained_qubits(&self) -> QubitCount {
        QubitCount::new(self.selection.len())
    }

    /// Returns the number of discarded qubits.
    #[must_use]
    pub fn discarded_qubits(&self) -> QubitCount {
        QubitCount::new(
            self.selection
                .source_qubits()
                .get()
                .saturating_sub(self.selection.len()),
        )
    }

    /// Returns whether the operation retains every source qubit.
    #[must_use]
    pub fn retains_all_qubits(&self) -> bool {
        self.selection.is_full()
    }

    /// Returns whether the operation discards at least one qubit.
    #[must_use]
    pub fn discards_qubits(&self) -> bool {
        !self.selection.is_full()
    }

    /// Returns the source-to-storage positions for retained qubits.
    pub fn retained_storage_positions(
        &self,
        layout: &MemoryLayout,
    ) -> Result<Vec<usize>, MemoryError> {
        self.selection.storage_positions(layout)
    }

    /// Returns the source-to-basis-bit positions for retained qubits.
    pub fn retained_bit_positions(
        &self,
        layout: &MemoryLayout,
    ) -> Result<Vec<usize>, MemoryError> {
        self.selection.bit_positions(layout)
    }

    /// Returns the source-to-storage positions for discarded qubits.
    pub fn discarded_storage_positions(
        &self,
        layout: &MemoryLayout,
    ) -> Result<Vec<usize>, MemoryError> {
        self.discarded()?.storage_positions(layout)
    }

    /// Returns the source-to-basis-bit positions for discarded qubits.
    pub fn discarded_bit_positions(
        &self,
        layout: &MemoryLayout,
    ) -> Result<Vec<usize>, MemoryError> {
        self.discarded()?.bit_positions(layout)
    }

    /// Returns the assignment for a discarded qubit, if this is a projection
    /// plan.
    #[must_use]
    pub fn projection_value(&self, qubit: QubitId) -> Option<bool> {
        self.projection
            .iter()
            .find(|assignment| assignment.qubit() == qubit)
            .map(BasisAssignment::value)
    }

    /// Returns the assignment for a discarded qubit as a typed object.
    #[must_use]
    pub fn projection_assignment(
        &self,
        qubit: QubitId,
    ) -> Option<BasisAssignment> {
        self.projection
            .iter()
            .find(|assignment| assignment.qubit() == qubit)
            .copied()
    }

    /// Returns whether the plan specifies a projection value for the qubit.
    #[must_use]
    pub fn has_projection_value(&self, qubit: QubitId) -> bool {
        self.projection_value(qubit).is_some()
    }

    /// Validates the plan against a memory layout.
    ///
    /// This checks that the layout describes the same logical source
    /// namespace. It does not inspect or modify quantum state.
    pub fn validate_layout(
        &self,
        layout: &MemoryLayout,
    ) -> Result<(), MemoryError> {
        if layout.num_qubits() != self.selection.source_qubits().get() {
            return Err(invalid_slice(format!(
                "slice source has {} qubits but layout has {}",
                self.selection.source_qubits().get(),
                layout.num_qubits()
            )));
        }

        // Force complete checked validation of every selected/discarded qubit.
        let _ = self.retained_storage_positions(layout)?;
        let _ = self.discarded_storage_positions(layout)?;

        Ok(())
    }
}

// =============================================================================
// Validation
// =============================================================================

/// Validates a raw ordered selection.
fn validate_selection(
    source_qubits: QubitCount,
    selected: &[QubitId],
) -> Result<(), MemoryError> {
    let source_count = source_qubits.get();

    for &qubit in selected {
        if qubit.index() >= source_count {
            return Err(invalid_slice(format!(
                "logical qubit {} is outside source range 0..{}",
                qubit.index(),
                source_count
            )));
        }
    }

    // `Vec` is appropriate here because the descriptor is normally small
    // compared with the quantum state it describes. More importantly, this
    // avoids relying on HashSet iteration order and therefore keeps validation
    // deterministic.
    for (position, &qubit) in selected.iter().enumerate() {
        if selected[..position].contains(&qubit) {
            return Err(invalid_slice(format!(
                "logical qubit {} occurs more than once in selection",
                qubit.index()
            )));
        }
    }

    Ok(())
}

/// Validates operation-specific semantics.
fn validate_operation(
    selection: &QubitSlice,
    operation: SliceOperation,
    assignments: &[BasisAssignment],
) -> Result<(), MemoryError> {
    let discarded = selection.complement()?;

    match operation {
        SliceOperation::View | SliceOperation::Copy | SliceOperation::PartialTrace => {
            if !assignments.is_empty() {
                return Err(invalid_slice(
                    "projection assignments are only valid for Projection operations",
                ));
            }
        }

        SliceOperation::Projection => {
            // A projection onto a subsystem is only unambiguous when every
            // discarded qubit receives exactly one computational-basis value.
            if assignments.len() != discarded.len() {
                return Err(invalid_slice(format!(
                    "projection requires exactly {} discarded-qubit assignments, got {}",
                    discarded.len(),
                    assignments.len()
                )));
            }

            for (position, assignment) in assignments.iter().enumerate() {
                let qubit = assignment.qubit();

                if selection.contains(qubit) {
                    return Err(invalid_slice(format!(
                        "projection assignment for retained qubit {} is invalid; \
                         projection assignments must target discarded qubits",
                        qubit.index()
                    )));
                }

                if qubit.index() >= selection.source_qubits().get() {
                    return Err(invalid_slice(format!(
                        "projection assignment targets qubit {} outside source \
                         range 0..{}",
                        qubit.index(),
                        selection.source_qubits().get()
                    )));
                }

                if assignments[..position]
                    .iter()
                    .any(|previous| previous.qubit() == qubit)
                {
                    return Err(invalid_slice(format!(
                        "projection contains duplicate assignment for qubit {}",
                        qubit.index()
                    )));
                }

                if !discarded.contains(qubit) {
                    return Err(invalid_slice(format!(
                        "projection assignment for qubit {} is not part of the \
                         discarded subsystem",
                        qubit.index()
                    )));
                }
            }

            // Every discarded qubit must have an assignment.
            for &qubit in discarded.as_slice() {
                if !assignments
                    .iter()
                    .any(|assignment| assignment.qubit() == qubit)
                {
                    return Err(invalid_slice(format!(
                        "projection is missing an assignment for discarded qubit {}",
                        qubit.index()
                    )));
                }
            }
        }
    }

    Ok(())
}

// =============================================================================
// Error helper
// =============================================================================

/// Creates the canonical memory error for invalid slice descriptors.
fn invalid_slice(reason: impl Into<String>) -> MemoryError {
    MemoryError::InvalidSlice {
        reason: reason.into(),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::memory::layout::{
        BitOrder,
        MemoryLayout,
        QubitOrder,
        StorageLayout,
    };

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn layout(num_qubits: usize) -> MemoryLayout {
        let order = QubitOrder::identity(num_qubits)
            .expect("identity order must be valid");

        MemoryLayout::try_new(
            num_qubits,
            BitOrder::LittleEndian,
            StorageLayout::Contiguous,
            order,
        )
        .expect("identity layout must be valid")
    }

    #[test]
    fn empty_selection_is_valid() {
        let slice =
            QubitSlice::empty(QubitCount::new(4));

        assert!(slice.is_empty());
        assert_eq!(slice.source_qubits().get(), 4);
        assert_eq!(slice.len(), 0);
    }

    #[test]
    fn all_selection_is_canonical() {
        let slice =
            QubitSlice::all(QubitCount::new(4))
                .expect("all selection must succeed");

        assert!(slice.is_full());
        assert!(slice.is_identity());
        assert_eq!(
            slice.as_slice(),
            &[q(0), q(1), q(2), q(3)]
        );
    }

    #[test]
    fn ordered_selection_is_preserved() {
        let slice = QubitSlice::from_ids(
            QubitCount::new(4),
            &[q(3), q(1)],
        )
        .expect("selection must be valid");

        assert_eq!(
            slice.as_slice(),
            &[q(3), q(1)]
        );

        assert_eq!(slice.position_of(q(3)), Some(0));
        assert_eq!(slice.position_of(q(1)), Some(1));
    }

    #[test]
    fn duplicate_selection_is_rejected() {
        let result = QubitSlice::from_ids(
            QubitCount::new(4),
            &[q(1), q(1)],
        );

        assert!(matches!(
            result,
            Err(MemoryError::InvalidSlice { .. })
        ));
    }

    #[test]
    fn out_of_range_selection_is_rejected() {
        let result = QubitSlice::from_ids(
            QubitCount::new(4),
            &[q(4)],
        );

        assert!(matches!(
            result,
            Err(MemoryError::InvalidSlice { .. })
        ));
    }

    #[test]
    fn range_selection_is_correct() {
        let slice = QubitSlice::range(
            QubitCount::new(5),
            1..4,
        )
        .expect("range must be valid");

        assert_eq!(
            slice.as_slice(),
            &[q(1), q(2), q(3)]
        );
    }

    #[test]
    fn invalid_range_is_rejected() {
        let result = QubitSlice::range(
            QubitCount::new(5),
            4..7,
        );

        assert!(matches!(
            result,
            Err(MemoryError::InvalidSlice { .. })
        ));
    }

    #[test]
    fn complement_is_canonical() {
        let slice = QubitSlice::from_ids(
            QubitCount::new(5),
            &[q(4), q(1)],
        )
        .expect("selection must be valid");

        let complement =
            slice.complement()
                .expect("complement must succeed");

        assert_eq!(
            complement.as_slice(),
            &[q(0), q(2), q(3)]
        );
    }

    #[test]
    fn storage_positions_follow_layout() {
        let order = QubitOrder::try_from_logical_order(
            4,
            vec![q(2), q(0), q(3), q(1)],
        )
        .expect("order must be valid");

        let layout = MemoryLayout::try_new(
            4,
            BitOrder::LittleEndian,
            StorageLayout::Permuted,
            order,
        )
        .expect("layout must be valid");

        let slice = QubitSlice::from_ids(
            QubitCount::new(4),
            &[q(0), q(3), q(1)],
        )
        .expect("selection must be valid");

        let positions = slice
            .storage_positions(&layout)
            .expect("positions must resolve");

        // Storage:
        // position 0 -> q2
        // position 1 -> q0
        // position 2 -> q3
        // position 3 -> q1
        assert_eq!(positions, vec![1, 2, 3]);
    }

    #[test]
    fn bit_positions_follow_layout() {
        let layout = layout(4);

        let slice = QubitSlice::from_ids(
            QubitCount::new(4),
            &[q(0), q(2), q(3)],
        )
        .expect("selection must be valid");

        let bits = slice
            .bit_positions(&layout)
            .expect("bit positions must resolve");

        assert_eq!(bits, vec![0, 2, 3]);
    }

    #[test]
    fn reorder_preserves_membership_and_changes_order() {
        let slice = QubitSlice::from_ids(
            QubitCount::new(4),
            &[q(3), q(1), q(0)],
        )
        .expect("selection must be valid");

        let reordered = slice
            .reorder(&[q(0), q(3), q(1)])
            .expect("reorder must be valid");

        assert_eq!(
            reordered.as_slice(),
            &[q(0), q(3), q(1)]
        );
    }

    #[test]
    fn reorder_rejects_missing_qubit() {
        let slice = QubitSlice::from_ids(
            QubitCount::new(4),
            &[q(0), q(1)],
        )
        .expect("selection must be valid");

        let result = slice.reorder(&[q(0), q(2)]);

        assert!(matches!(
            result,
            Err(MemoryError::InvalidSlice { .. })
        ));
    }

    #[test]
    fn view_plan_has_no_projection_assignments() {
        let slice = QubitSlice::from_ids(
            QubitCount::new(4),
            &[q(0), q(2)],
        )
        .expect("selection must be valid");

        let plan =
            SlicePlan::view(slice)
                .expect("view plan must be valid");

        assert_eq!(
            plan.operation(),
            SliceOperation::View
        );
        assert!(plan.projection_assignments().is_empty());
        assert_eq!(plan.retained_qubits().get(), 2);
        assert_eq!(plan.discarded_qubits().get(), 2);
    }

    #[test]
    fn copy_plan_is_not_partial_trace() {
        let slice = QubitSlice::from_ids(
            QubitCount::new(4),
            &[q(0), q(2)],
        )
        .expect("selection must be valid");

        let copy =
            SlicePlan::copy(slice.clone())
                .expect("copy plan must be valid");

        let trace =
            SlicePlan::partial_trace(slice)
                .expect("trace plan must be valid");

        assert_eq!(
            copy.operation(),
            SliceOperation::Copy
        );

        assert_eq!(
            trace.operation(),
            SliceOperation::PartialTrace
        );

        assert_ne!(
            copy.operation(),
            trace.operation()
        );
    }

    #[test]
    fn projection_requires_all_discarded_qubits() {
        let slice = QubitSlice::from_ids(
            QubitCount::new(3),
            &[q(0)],
        )
        .expect("selection must be valid");

        let result = SlicePlan::projection(
            slice,
            vec![BasisAssignment::zero(q(1))],
        );

        assert!(matches!(
            result,
            Err(MemoryError::InvalidSlice { .. })
        ));
    }

    #[test]
    fn projection_requires_discarded_qubits_only() {
        let slice = QubitSlice::from_ids(
            QubitCount::new(3),
            &[q(0)],
        )
        .expect("selection must be valid");

        let result = SlicePlan::projection(
            slice,
            vec![
                BasisAssignment::zero(q(0)),
                BasisAssignment::one(q(1)),
                BasisAssignment::zero(q(2)),
            ],
        );

        assert!(matches!(
            result,
            Err(MemoryError::InvalidSlice { .. })
        ));
    }

    #[test]
    fn projection_accepts_complete_assignment() {
        let slice = QubitSlice::from_ids(
            QubitCount::new(3),
            &[q(0)],
        )
        .expect("selection must be valid");

        let plan = SlicePlan::projection(
            slice,
            vec![
                BasisAssignment::one(q(1)),
                BasisAssignment::zero(q(2)),
            ],
        )
        .expect("complete projection must be valid");

        assert_eq!(
            plan.projection_value(q(1)),
            Some(true)
        );

        assert_eq!(
            plan.projection_value(q(2)),
            Some(false)
        );

        assert_eq!(
            plan.projection_value(q(0)),
            None
        );
    }

    #[test]
    fn projection_assignments_are_deterministic() {
        let slice = QubitSlice::from_ids(
            QubitCount::new(4),
            &[q(0), q(3)],
        )
        .expect("selection must be valid");

        let plan = SlicePlan::projection(
            slice,
            vec![
                BasisAssignment::one(q(1)),
                BasisAssignment::zero(q(2)),
            ],
        )
        .expect("projection must be valid");

        assert_eq!(
            plan.projection_assignments(),
            &[
                BasisAssignment::one(q(1)),
                BasisAssignment::zero(q(2))
            ]
        );
    }

    #[test]
    fn partial_trace_has_no_projection_assignments() {
        let slice = QubitSlice::from_ids(
            QubitCount::new(4),
            &[q(1), q(3)],
        )
        .expect("selection must be valid");

        let plan =
            SlicePlan::partial_trace(slice)
                .expect("partial trace must be valid");

        assert_eq!(
            plan.operation(),
            SliceOperation::PartialTrace
        );

        assert!(plan.projection_assignments().is_empty());
    }

    #[test]
    fn layout_validation_rejects_wrong_source_size() {
        let slice = QubitSlice::from_ids(
            QubitCount::new(4),
            &[q(0), q(2)],
        )
        .expect("selection must be valid");

        let plan =
            SlicePlan::view(slice)
                .expect("plan must be valid");

        let wrong_layout = layout(3);

        let result =
            plan.validate_layout(&wrong_layout);

        assert!(matches!(
            result,
            Err(MemoryError::InvalidSlice { .. })
        ));
    }

    #[test]
    fn full_selection_has_empty_complement() {
        let slice =
            QubitSlice::all(QubitCount::new(4))
                .expect("all selection must succeed");

        let complement =
            slice.complement()
                .expect("complement must succeed");

        assert!(complement.is_empty());
    }

    #[test]
    fn empty_selection_has_full_complement() {
        let slice =
            QubitSlice::empty(QubitCount::new(4));

        let complement =
            slice.complement()
                .expect("complement must succeed");

        assert_eq!(
            complement.as_slice(),
            &[q(0), q(1), q(2), q(3)]
        );
    }

    #[test]
    fn operation_properties_are_correct() {
        assert!(SliceOperation::View.is_view());
        assert!(!SliceOperation::View.requires_owned_output());
        assert!(!SliceOperation::View.is_reduction());

        assert!(!SliceOperation::Copy.is_view());
        assert!(SliceOperation::Copy.requires_owned_output());
        assert!(!SliceOperation::Copy.is_reduction());

        assert!(SliceOperation::Projection.is_reduction());
        assert!(SliceOperation::Projection.requires_owned_output());
        assert!(
            SliceOperation::Projection
                .requires_projection_assignments()
        );

        assert!(SliceOperation::PartialTrace.is_reduction());
        assert!(SliceOperation::PartialTrace.requires_owned_output());
        assert!(
            !SliceOperation::PartialTrace
                .requires_projection_assignments()
        );
    }
}