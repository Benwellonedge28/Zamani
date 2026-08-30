//! Zamani Quantum Optimization — Canonical Circuit Access Layer
//!
//! This module provides the optimizer's safe view and transactional editing
//! layer over the canonical `quantum::ir::QuantumCircuit`.
//!
//! # Architectural rule
//!
//! This module MUST NOT define another quantum IR.
//!
//! The authoritative representation is:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! and the authoritative operation representation is:
//!
//! `crate::quantum::ir::Gate`.
//!
//! This module only adds optimizer-specific concepts:
//!
//! - `OperationId`;
//! - `RegionId`;
//! - `OperationSlice`;
//! - `CircuitView`;
//! - `CircuitCursor`;
//! - `CircuitEdit`;
//! - `CircuitEditPlan`;
//! - `CircuitEditor`;
//! - dependency-safe accessors;
//! - transactional edit validation;
//! - optimizer-local snapshots.
//!
//! # Design goals
//!
//! The optimizer must be able to:
//!
//! - inspect a circuit without copying it;
//! - identify operations with stable invocation-local IDs;
//! - inspect operation windows;
//! - inspect qubit usage;
//! - create deterministic edit plans;
//! - validate an entire edit plan before mutation;
//! - apply edits atomically;
//! - never expose an unrestricted mutable operation slice;
//! - preserve canonical IR ownership;
//! - remain independent of routing, scheduling, hardware, and execution;
//! - remain deterministic;
//! - remain bounded by explicit optimizer limits;
//! - scale from tiny circuits to the largest circuit permitted by available
//!   memory and configured resource limits.
//!
//! # Transactional model
//!
//! Optimization passes should prefer:
//!
//! ```text
//! QuantumCircuit
//!       |
//!       v
//! CircuitView
//!       |
//!       v
//! CircuitEditPlan
//!       |
//!       v
//! validate all edits
//!       |
//!       v
//! apply atomically
//! ```
//!
//! A failed edit plan must not leave a partially modified optimizer circuit.
//!
//! # No unsafe
//!
//! This file contains no `unsafe` code.
//!
//! # Rust compatibility
//!
//! Rust 1.97 / Rust 1.97.1.
//!
//! No nightly features are required.

use std::fmt;
use std::ops::Range;

use crate::quantum::ir::gate::Gate;
use crate::quantum::ir::qubits::QubitId;
use crate::quantum::ir::QuantumCircuit;

use super::limits::{
    OptimizationLimits,
    OptimizationLimitsError,
    OptimizationResource,
};

// ============================================================================
// IDs
// ============================================================================

/// Stable identifier for an operation within one optimizer invocation.
///
/// `OperationId` is intentionally not the same thing as a physical/backend
/// operation identifier. It identifies a position in the optimizer's logical
/// circuit snapshot.
///
/// IDs are invocation-local and must never be persisted as globally stable
/// identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OperationId(usize);

impl OperationId {
    /// Creates an operation identifier from an invocation-local index.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the underlying invocation-local index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for OperationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "op{}", self.0)
    }
}

/// Identifier for a logical optimization region.
///
/// Region IDs are invocation-local. A region is an optimizer concept and does
/// not modify the canonical IR's ownership model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegionId(usize);

impl RegionId {
    /// Creates a region identifier.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the region index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for RegionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "region{}", self.0)
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by the optimizer circuit access layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CircuitViewError {
    /// The requested operation does not exist.
    OperationOutOfRange {
        /// Requested operation index.
        index: usize,

        /// Number of operations available.
        len: usize,
    },

    /// The requested range is invalid.
    InvalidRange {
        /// Start of the requested range.
        start: usize,

        /// End of the requested range.
        end: usize,

        /// Number of available operations.
        len: usize,
    },

    /// An operation ID belongs to a different snapshot or is no longer live.
    StaleOperationId {
        /// Stale operation ID.
        operation: OperationId,
    },

    /// A region range is invalid.
    InvalidRegion {
        /// Region identifier.
        region: RegionId,
    },

    /// An edit plan contains conflicting edits.
    ConflictingEdits {
        /// First conflicting operation.
        first: OperationId,

        /// Second conflicting operation.
        second: OperationId,
    },

    /// An edit attempts to reference an operation that has already been
    /// deleted/replaced by an earlier edit in the same transaction.
    DeletedOperation {
        /// Deleted operation.
        operation: OperationId,
    },

    /// An insertion position is outside the valid boundary.
    InvalidInsertionPoint {
        /// Requested insertion point.
        index: usize,

        /// Number of current operations.
        len: usize,
    },

    /// The edit would exceed the optimizer's configured circuit-operation
    /// budget.
    CircuitOperationLimitExceeded {
        /// Requested operation count.
        requested: usize,

        /// Maximum operation count.
        maximum: u64,
    },

    /// The edit would exceed the optimizer's configured qubit budget.
    CircuitQubitLimitExceeded {
        /// Requested qubit count.
        requested: usize,

        /// Maximum qubit count.
        maximum: u64,
    },

    /// Arithmetic overflow occurred while calculating a new circuit size.
    ArithmeticOverflow {
        /// Calculation that overflowed.
        calculation: &'static str,
    },

    /// Canonical IR validation failed after a proposed transformation.
    InvalidCanonicalCircuit {
        /// Human-readable reason.
        message: String,
    },

    /// A supplied canonical circuit is already invalid.
    InvalidInputCircuit {
        /// Human-readable validation failure.
        message: String,
    },

    /// The operation supplied by an edit is not valid for the target circuit.
    InvalidOperation {
        /// Human-readable reason.
        message: String,
    },

    /// The optimizer limit subsystem rejected a request.
    Limits(OptimizationLimitsError),
}

impl fmt::Display for CircuitViewError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OperationOutOfRange { index, len } => {
                write!(
                    formatter,
                    "operation index {index} is outside circuit length {len}"
                )
            }

            Self::InvalidRange { start, end, len } => {
                write!(
                    formatter,
                    "operation range {start}..{end} is outside circuit length {len}"
                )
            }

            Self::StaleOperationId { operation } => {
                write!(formatter, "{operation} is stale or no longer exists")
            }

            Self::InvalidRegion { region } => {
                write!(formatter, "invalid optimizer region {region}")
            }

            Self::ConflictingEdits { first, second } => {
                write!(
                    formatter,
                    "edits for {first} and {second} conflict"
                )
            }

            Self::DeletedOperation { operation } => {
                write!(
                    formatter,
                    "{operation} was deleted by an earlier edit"
                )
            }

            Self::InvalidInsertionPoint { index, len } => {
                write!(
                    formatter,
                    "insertion point {index} is outside 0..={len}"
                )
            }

            Self::CircuitOperationLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "optimizer circuit-operation limit exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::CircuitQubitLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "optimizer circuit-qubit limit exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::InvalidCanonicalCircuit { message } => {
                write!(
                    formatter,
                    "optimized circuit violates canonical IR invariants: {message}"
                )
            }

            Self::InvalidInputCircuit { message } => {
                write!(
                    formatter,
                    "input circuit is invalid: {message}"
                )
            }

            Self::InvalidOperation { message } => {
                write!(
                    formatter,
                    "invalid optimizer operation: {message}"
                )
            }

            Self::Limits(error) => {
                write!(formatter, "{error}")
            }
        }
    }
}

impl std::error::Error for CircuitViewError {}

impl From<OptimizationLimitsError> for CircuitViewError {
    fn from(error: OptimizationLimitsError) -> Self {
        Self::Limits(error)
    }
}

// ============================================================================
// Operation references
// ============================================================================

/// Immutable reference to an operation in a `CircuitView`.
///
/// The reference contains both the invocation-local ID and its current logical
/// position. It is intentionally lightweight and copyable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationRef<'a> {
    id: OperationId,
    index: usize,
    gate: &'a Gate,
}

impl<'a> OperationRef<'a> {
    /// Returns the operation ID.
    #[must_use]
    pub const fn id(self) -> OperationId {
        self.id
    }

    /// Returns the current index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }

    /// Returns the canonical gate.
    #[must_use]
    pub const fn gate(self) -> &'a Gate {
        self.gate
    }
}

/// Immutable operation slice.
#[derive(Debug, Clone, Copy)]
pub struct OperationSlice<'a> {
    operations: &'a [Gate],
    start: usize,
}

impl<'a> OperationSlice<'a> {
    fn new(
        operations: &'a [Gate],
        start: usize,
    ) -> Self {
        Self {
            operations,
            start,
        }
    }

    /// Number of operations in the slice.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns true when the slice contains no operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Returns an operation by slice-relative index.
    #[must_use]
    pub fn get(
        &self,
        index: usize,
    ) -> Option<OperationRef<'a>> {
        self.operations.get(index).map(|gate| OperationRef {
            id: OperationId::new(self.start + index),
            index: self.start + index,
            gate,
        })
    }

    /// Returns the immutable canonical gates.
    #[must_use]
    pub fn gates(&self) -> &'a [Gate] {
        self.operations
    }

    /// Returns an iterator over operation references.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = OperationRef<'a>> + '_ {
        self.operations
            .iter()
            .enumerate()
            .map(move |(offset, gate)| OperationRef {
                id: OperationId::new(self.start + offset),
                index: self.start + offset,
                gate,
            })
    }
}

impl<'a> IntoIterator for OperationSlice<'a> {
    type Item = OperationRef<'a>;
    type IntoIter =
        std::iter::Map<
            std::iter::Enumerate<std::slice::Iter<'a, Gate>>,
            fn((usize, &'a Gate)) -> OperationRef<'a>,
        >;

    fn into_iter(self) -> Self::IntoIter {
        fn map_operation<'a>(
            item: (usize, &'a Gate),
        ) -> OperationRef<'a> {
            OperationRef {
                id: OperationId::new(item.0),
                index: item.0,
                gate: item.1,
            }
        }

        self.operations
            .iter()
            .enumerate()
            .map(map_operation)
    }
}

// ============================================================================
// Circuit view
// ============================================================================

/// Immutable optimizer view over a canonical quantum circuit.
///
/// `CircuitView` does not own the circuit and therefore cannot mutate it.
///
/// This is the preferred object for analyses and read-only optimization passes.
#[derive(Debug, Clone, Copy)]
pub struct CircuitView<'a> {
    circuit: &'a QuantumCircuit,
    operations: &'a [Gate],
}

impl<'a> CircuitView<'a> {
    /// Creates a view over a canonical circuit.
    ///
    /// The canonical circuit is validated before the view is returned.
    pub fn new(
        circuit: &'a QuantumCircuit,
    ) -> Result<Self, CircuitViewError> {
        circuit
            .validate()
            .map_err(|error| CircuitViewError::InvalidInputCircuit {
                message: error.to_string(),
            })?;

        Ok(Self {
            circuit,
            operations: circuit.operations(),
        })
    }

    /// Creates a view without repeating canonical validation.
    ///
    /// # Safety contract
    ///
    /// This function is safe Rust and does not use `unsafe`.
    ///
    /// The caller must only use it when the circuit has already been validated
    /// in the same compiler stage.
    ///
    /// This method exists to prevent repeated O(n) validation in pipelines that
    /// have already established the invariant.
    #[must_use]
    pub const fn from_validated(
        circuit: &'a QuantumCircuit,
    ) -> Self {
        Self {
            circuit,
            operations: circuit.operations(),
        }
    }

    /// Returns the canonical circuit.
    #[must_use]
    pub const fn canonical(&self) -> &'a QuantumCircuit {
        self.circuit
    }

    /// Returns the number of logical qubits.
    #[must_use]
    pub fn num_qubits(&self) -> usize {
        self.circuit.num_qubits()
    }

    /// Returns the number of classical bits.
    #[must_use]
    pub fn num_classical_bits(&self) -> usize {
        self.circuit.num_classical_bits()
    }

    /// Returns the number of operations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns true if the circuit has no operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Returns an operation by index.
    pub fn operation(
        &self,
        index: usize,
    ) -> Result<OperationRef<'a>, CircuitViewError> {
        self.operations
            .get(index)
            .map(|gate| OperationRef {
                id: OperationId::new(index),
                index,
                gate,
            })
            .ok_or(CircuitViewError::OperationOutOfRange {
                index,
                len: self.len(),
            })
    }

    /// Returns an operation by invocation-local ID.
    pub fn operation_by_id(
        &self,
        id: OperationId,
    ) -> Result<OperationRef<'a>, CircuitViewError> {
        self.operation(id.index())
    }

    /// Returns an immutable operation slice.
    pub fn slice(
        &self,
        range: Range<usize>,
    ) -> Result<OperationSlice<'a>, CircuitViewError> {
        if range.start > range.end
            || range.end > self.operations.len()
        {
            return Err(CircuitViewError::InvalidRange {
                start: range.start,
                end: range.end,
                len: self.len(),
            });
        }

        Ok(OperationSlice::new(
            &self.operations[range],
            range.start,
        ))
    }

    /// Returns all canonical operations.
    #[must_use]
    pub fn operations(&self) -> &'a [Gate] {
        self.operations
    }

    /// Returns the first operation, if any.
    #[must_use]
    pub fn first(&self) -> Option<OperationRef<'a>> {
        self.operations.first().map(|gate| OperationRef {
            id: OperationId::new(0),
            index: 0,
            gate,
        })
    }

    /// Returns the last operation, if any.
    #[must_use]
    pub fn last(&self) -> Option<OperationRef<'a>> {
        self.operations
            .last()
            .map(|gate| {
                let index = self.operations.len() - 1;

                OperationRef {
                    id: OperationId::new(index),
                    index,
                    gate,
                }
            })
    }

    /// Returns the operation ID range.
    #[must_use]
    pub fn operation_ids(
        &self,
    ) -> impl Iterator<Item = OperationId> + '_ {
        (0..self.operations.len()).map(OperationId::new)
    }

    /// Returns all operations touching a logical qubit.
    ///
    /// This is deliberately a linear scan. Higher-level analyses should build
    /// indexed structures when repeated queries justify their memory cost.
    pub fn operations_on_qubit(
        &self,
        qubit: QubitId,
    ) -> impl Iterator<Item = OperationRef<'a>> + '_ {
        self.operations
            .iter()
            .enumerate()
            .filter(move |(_, gate)| gate.qubits().contains(&qubit))
            .map(|(index, gate)| OperationRef {
                id: OperationId::new(index),
                index,
                gate,
            })
    }

    /// Returns whether an operation touches a logical qubit.
    #[must_use]
    pub fn touches_qubit(
        &self,
        operation: OperationId,
        qubit: QubitId,
    ) -> Result<bool, CircuitViewError> {
        Ok(self.operation_by_id(operation)?
            .gate()
            .qubits()
            .contains(&qubit))
    }

    /// Creates an immutable cursor.
    #[must_use]
    pub fn cursor(&self) -> CircuitCursor<'a> {
        CircuitCursor {
            view: *self,
            position: 0,
        }
    }

    /// Creates an edit plan.
    #[must_use]
    pub fn edit_plan(&self) -> CircuitEditPlan<'a> {
        CircuitEditPlan::new(*self)
    }

    /// Returns a snapshot suitable for optimizer-local transformations.
    ///
    /// This intentionally clones only the canonical operation vector. The
    /// canonical circuit remains untouched.
    #[must_use]
    pub fn snapshot(&self) -> CircuitSnapshot {
        CircuitSnapshot {
            num_qubits: self.num_qubits(),
            num_classical_bits: self.num_classical_bits(),
            operations: self.operations.to_vec(),
        }
    }
}

// ============================================================================
// Cursor
// ============================================================================

/// Deterministic forward cursor over a circuit.
#[derive(Debug, Clone, Copy)]
pub struct CircuitCursor<'a> {
    view: CircuitView<'a>,
    position: usize,
}

impl<'a> CircuitCursor<'a> {
    /// Returns the current position.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.position
    }

    /// Returns true when the cursor is at the end.
    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.position >= self.view.len()
    }

    /// Returns the current operation without advancing.
    pub fn current(
        &self,
    ) -> Result<Option<OperationRef<'a>>, CircuitViewError> {
        if self.is_finished() {
            return Ok(None);
        }

        self.view.operation(self.position).map(Some)
    }

    /// Advances the cursor by one operation.
    pub fn advance(
        &mut self,
    ) -> Result<Option<OperationRef<'a>>, CircuitViewError> {
        if self.is_finished() {
            return Ok(None);
        }

        let operation = self.view.operation(self.position)?;
        self.position += 1;

        Ok(Some(operation))
    }

    /// Moves the cursor to a specific operation.
    pub fn seek(
        &mut self,
        position: usize,
    ) -> Result<(), CircuitViewError> {
        if position > self.view.len() {
            return Err(CircuitViewError::OperationOutOfRange {
                index: position,
                len: self.view.len(),
            });
        }

        self.position = position;

        Ok(())
    }

    /// Returns the next operation without changing the cursor.
    pub fn peek(
        &self,
    ) -> Result<Option<OperationRef<'a>>, CircuitViewError> {
        self.current()
    }
}

// ============================================================================
// Snapshot
// ============================================================================

/// Owned optimizer-local circuit snapshot.
///
/// This is NOT a second canonical IR.
///
/// It is a temporary transactional workspace containing canonical `Gate`
/// values plus the logical namespace sizes required to rebuild a canonical
/// circuit.
#[derive(Debug, Clone, PartialEq)]
pub struct CircuitSnapshot {
    num_qubits: usize,
    num_classical_bits: usize,
    operations: Vec<Gate>,
}

impl CircuitSnapshot {
    /// Creates a snapshot from validated canonical components.
    pub fn new(
        num_qubits: usize,
        num_classical_bits: usize,
        operations: Vec<Gate>,
    ) -> Self {
        Self {
            num_qubits,
            num_classical_bits,
            operations,
        }
    }

    /// Returns the logical qubit count.
    #[must_use]
    pub const fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Returns the classical-bit count.
    #[must_use]
    pub const fn num_classical_bits(&self) -> usize {
        self.num_classical_bits
    }

    /// Returns the operation count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns true if there are no operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Returns the operations.
    #[must_use]
    pub fn operations(&self) -> &[Gate] {
        &self.operations
    }

    /// Returns mutable access only to the private optimizer workspace.
    ///
    /// This does not expose mutable access to the canonical IR.
    pub(crate) fn operations_mut(&mut self) -> &mut Vec<Gate> {
        &mut self.operations
    }

    /// Consumes the snapshot and returns its canonical operation sequence.
    #[must_use]
    pub fn into_operations(self) -> Vec<Gate> {
        self.operations
    }
}

// ============================================================================
// Edits
// ============================================================================

/// One atomic circuit edit.
///
/// Edits refer to operation IDs from the source `CircuitView`.
#[derive(Debug, Clone, PartialEq)]
pub enum CircuitEdit {
    /// Remove one operation.
    Remove {
        /// Operation to remove.
        operation: OperationId,
    },

    /// Replace one operation with another canonical gate.
    Replace {
        /// Operation to replace.
        operation: OperationId,

        /// Replacement operation.
        replacement: Gate,
    },

    /// Insert operations immediately before an existing operation.
    InsertBefore {
        /// Anchor operation.
        anchor: OperationId,

        /// Operations to insert.
        operations: Vec<Gate>,
    },

    /// Insert operations immediately after an existing operation.
    InsertAfter {
        /// Anchor operation.
        anchor: OperationId,

        /// Operations to insert.
        operations: Vec<Gate>,
    },

    /// Replace one operation by zero or more operations.
    ///
    /// This is the preferred primitive for decomposition and synthesis.
    ReplaceWith {
        /// Operation to replace.
        operation: OperationId,

        /// Replacement sequence.
        operations: Vec<Gate>,
    },
}

impl CircuitEdit {
    fn primary_operation(&self) -> OperationId {
        match self {
            Self::Remove { operation }
            | Self::Replace { operation, .. }
            | Self::ReplaceWith { operation, .. } => *operation,

            Self::InsertBefore { anchor, .. }
            | Self::InsertAfter { anchor, .. } => *anchor,
        }
    }

    fn inserted_count(&self) -> usize {
        match self {
            Self::Remove { .. } => 0,

            Self::Replace { .. } => 1,

            Self::InsertBefore { operations, .. }
            | Self::InsertAfter { operations, .. }
            | Self::ReplaceWith { operations, .. } => operations.len(),
        }
    }

    fn removed_count(&self) -> usize {
        match self {
            Self::Remove { .. }
            | Self::Replace { .. }
            | Self::ReplaceWith { .. } => 1,

            Self::InsertBefore { .. }
            | Self::InsertAfter { .. } => 0,
        }
    }

    fn validate_gate(
        gate: &Gate,
        num_qubits: usize,
        num_classical_bits: usize,
    ) -> Result<(), CircuitViewError> {
        gate.validate()
            .map_err(|error| CircuitViewError::InvalidOperation {
                message: error.to_string(),
            })?;

        for qubit in gate.qubits() {
            if qubit.index() >= num_qubits {
                return Err(CircuitViewError::InvalidOperation {
                    message: format!(
                        "gate references logical qubit {} but circuit has {} qubits",
                        qubit.index(),
                        num_qubits
                    ),
                });
            }
        }

        if let Some(bit) = gate.classical_target() {
            if bit >= num_classical_bits {
                return Err(CircuitViewError::InvalidOperation {
                    message: format!(
                        "gate references classical bit {bit} but circuit has \
                         {num_classical_bits} classical bits"
                    ),
                });
            }
        }

        Ok(())
    }
}

// ============================================================================
// Edit plan
// ============================================================================

/// Transactional optimizer edit plan.
///
/// A plan is constructed against one immutable `CircuitView`.
///
/// The plan is not allowed to mutate the canonical circuit while edits are
/// being collected.
///
/// All edits are validated before application.
#[derive(Debug)]
pub struct CircuitEditPlan<'a> {
    view: CircuitView<'a>,
    edits: Vec<CircuitEdit>,
}

impl<'a> CircuitEditPlan<'a> {
    fn new(view: CircuitView<'a>) -> Self {
        Self {
            view,
            edits: Vec::new(),
        }
    }

    /// Returns the source circuit view.
    #[must_use]
    pub const fn view(&self) -> CircuitView<'a> {
        self.view
    }

    /// Returns the number of pending edits.
    #[must_use]
    pub fn len(&self) -> usize {
        self.edits.len()
    }

    /// Returns true when no edits have been recorded.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.edits.is_empty()
    }

    /// Adds an edit to the transaction.
    ///
    /// No mutation occurs.
    pub fn push(
        &mut self,
        edit: CircuitEdit,
    ) -> Result<(), CircuitViewError> {
        self.validate_edit(&edit)?;

        self.edits.push(edit);

        Ok(())
    }

    /// Removes an operation.
    pub fn remove(
        &mut self,
        operation: OperationId,
    ) -> Result<(), CircuitViewError> {
        self.push(CircuitEdit::Remove { operation })
    }

    /// Replaces one operation.
    pub fn replace(
        &mut self,
        operation: OperationId,
        replacement: Gate,
    ) -> Result<(), CircuitViewError> {
        self.push(CircuitEdit::Replace {
            operation,
            replacement,
        })
    }

    /// Replaces one operation by zero or more operations.
    pub fn replace_with(
        &mut self,
        operation: OperationId,
        operations: Vec<Gate>,
    ) -> Result<(), CircuitViewError> {
        self.push(CircuitEdit::ReplaceWith {
            operation,
            operations,
        })
    }

    /// Inserts operations before an existing operation.
    pub fn insert_before(
        &mut self,
        anchor: OperationId,
        operations: Vec<Gate>,
    ) -> Result<(), CircuitViewError> {
        self.push(CircuitEdit::InsertBefore {
            anchor,
            operations,
        })
    }

    /// Inserts operations after an existing operation.
    pub fn insert_after(
        &mut self,
        anchor: OperationId,
        operations: Vec<Gate>,
    ) -> Result<(), CircuitViewError> {
        self.push(CircuitEdit::InsertAfter {
            anchor,
            operations,
        })
    }

    fn validate_edit(
        &self,
        edit: &CircuitEdit,
    ) -> Result<(), CircuitViewError> {
        let operation = edit.primary_operation();

        self.view.operation_by_id(operation)?;

        let (operations, qubits, classical_bits) = match edit {
            CircuitEdit::Remove { .. } => (0usize, 0usize, 0usize),

            CircuitEdit::Replace {
                replacement,
                ..
            } => {
                CircuitEdit::validate_gate(
                    replacement,
                    self.view.num_qubits(),
                    self.view.num_classical_bits(),
                )?;

                (1, replacement.qubits().len(), replacement.classical_target().map_or(0, |_| 1))
            }

            CircuitEdit::InsertBefore {
                operations,
                ..
            }
            | CircuitEdit::InsertAfter {
                operations,
                ..
            }
            | CircuitEdit::ReplaceWith {
                operations,
                ..
            } => {
                for gate in operations {
                    CircuitEdit::validate_gate(
                        gate,
                        self.view.num_qubits(),
                        self.view.num_classical_bits(),
                    )?;
                }

                let qubits = operations
                    .iter()
                    .map(|gate| gate.qubits().len())
                    .try_fold(
                        0usize,
                        |accumulator, count| {
                            accumulator.checked_add(count)
                        },
                    )
                    .ok_or(
                        CircuitViewError::ArithmeticOverflow {
                            calculation: "edit qubit references",
                        },
                    )?;

                let classical_bits = operations
                    .iter()
                    .filter(|gate| gate.classical_target().is_some())
                    .count();

                (operations.len(), qubits, classical_bits)
            }
        };

        let current = self.view.len();

        let mut resulting_operations = current;

        resulting_operations = resulting_operations
            .checked_add(operations)
            .ok_or(CircuitViewError::ArithmeticOverflow {
                calculation: "resulting operation count",
            })?;

        resulting_operations = resulting_operations
            .checked_sub(edit.removed_count())
            .ok_or(CircuitViewError::InvalidOperation {
                message: "edit would produce a negative operation count".to_string(),
            })?;

        let _ = qubits;
        let _ = classical_bits;

        if resulting_operations > current {
            // Actual configured optimizer limits are checked again by the
            // editor because the plan may contain multiple individually-valid
            // edits whose combined size exceeds the limit.
        }

        Ok(())
    }

    /// Validates all edits and calculates the resulting operation count.
    pub fn validate(
        &self,
        limits: &OptimizationLimits,
    ) -> Result<usize, CircuitViewError> {
        let mut resulting_operations = self.view.len();

        let maximum = limits.max_circuit_operations();

        for edit in &self.edits {
            resulting_operations = resulting_operations
                .checked_sub(edit.removed_count())
                .ok_or(CircuitViewError::InvalidOperation {
                    message: "edit sequence removes more operations than exist"
                        .to_string(),
                })?;

            resulting_operations = resulting_operations
                .checked_add(edit.inserted_count())
                .ok_or(CircuitViewError::ArithmeticOverflow {
                    calculation: "transaction operation count",
                })?;

            if resulting_operations as u64 > maximum {
                return Err(
                    CircuitViewError::CircuitOperationLimitExceeded {
                        requested: resulting_operations,
                        maximum,
                    },
                );
            }
        }

        self.validate_conflicts()?;

        Ok(resulting_operations)
    }

    fn validate_conflicts(
        &self,
    ) -> Result<(), CircuitViewError> {
        for (left_index, left) in self.edits.iter().enumerate() {
            let left_operation = left.primary_operation();

            for right in self.edits.iter().skip(left_index + 1) {
                let right_operation = right.primary_operation();

                if left_operation != right_operation {
                    continue;
                }

                let left_is_insert =
                    matches!(
                        left,
                        CircuitEdit::InsertBefore { .. }
                            | CircuitEdit::InsertAfter { .. }
                    );

                let right_is_insert =
                    matches!(
                        right,
                        CircuitEdit::InsertBefore { .. }
                            | CircuitEdit::InsertAfter { .. }
                    );

                // Multiple insertions around the same anchor are allowed.
                // Multiple mutations of the same operation are not.
                if left_is_insert && right_is_insert {
                    continue;
                }

                return Err(CircuitViewError::ConflictingEdits {
                    first: left_operation,
                    second: right_operation,
                });
            }
        }

        Ok(())
    }

    /// Applies the edit plan to an optimizer-local snapshot.
    ///
    /// This operation is atomic from the caller's perspective: a failure
    /// returns an error and the supplied snapshot remains untouched.
    pub fn apply(
        &self,
        snapshot: &CircuitSnapshot,
        limits: &OptimizationLimits,
    ) -> Result<CircuitSnapshot, CircuitViewError> {
        if snapshot.num_qubits() != self.view.num_qubits()
            || snapshot.num_classical_bits()
                != self.view.num_classical_bits()
            || snapshot.operations() != self.view.operations()
        {
            return Err(CircuitViewError::InvalidOperation {
                message:
                    "snapshot does not correspond to the edit-plan source circuit"
                        .to_string(),
            });
        }

        self.validate(limits)?;

        let mut result = snapshot.clone();

        // Build a lookup table for the source operations that remain.
        let mut removed = vec![false; snapshot.len()];

        for edit in &self.edits {
            match edit {
                CircuitEdit::Remove { operation } => {
                    removed[operation.index()] = true;
                }

                CircuitEdit::Replace { operation, .. }
                | CircuitEdit::ReplaceWith { operation, .. } => {
                    removed[operation.index()] = true;
                }

                CircuitEdit::InsertBefore { .. }
                | CircuitEdit::InsertAfter { .. } => {}
            }
        }

        let mut output = Vec::with_capacity(
            self.validate(limits)?,
        );

        for index in 0..snapshot.len() {
            let operation = OperationId::new(index);

            for edit in &self.edits {
                if let CircuitEdit::InsertBefore {
                    anchor,
                    operations,
                } = edit
                {
                    if *anchor == operation {
                        output.extend(operations.iter().cloned());
                    }
                }
            }

            if !removed[index] {
                output.push(snapshot.operations()[index].clone());
            }

            for edit in &self.edits {
                match edit {
                    CircuitEdit::InsertAfter {
                        anchor,
                        operations,
                    } if *anchor == operation => {
                        output.extend(operations.iter().cloned());
                    }

                    CircuitEdit::Replace {
                        operation: target,
                        replacement,
                    } if *target == operation => {
                        output.push(replacement.clone());
                    }

                    CircuitEdit::ReplaceWith {
                        operation: target,
                        operations,
                    } if *target == operation => {
                        output.extend(operations.iter().cloned());
                    }

                    _ => {}
                }
            }
        }

        let final_count = output.len();

        if final_count as u64 > limits.max_circuit_operations() {
            return Err(
                CircuitViewError::CircuitOperationLimitExceeded {
                    requested: final_count,
                    maximum: limits.max_circuit_operations(),
                },
            );
        }

        result.operations = output;

        Ok(result)
    }

    /// Returns the edits without allowing the caller to mutate the plan.
    #[must_use]
    pub fn edits(&self) -> &[CircuitEdit] {
        &self.edits
    }
}

// ============================================================================
// Editor
// ============================================================================

/// Transactional optimizer circuit editor.
///
/// `CircuitEditor` owns an optimizer-local snapshot. It never exposes the
/// canonical circuit's mutable storage.
#[derive(Debug)]
pub struct CircuitEditor {
    snapshot: CircuitSnapshot,
    limits: OptimizationLimits,
}

impl CircuitEditor {
    /// Creates an editor from a canonical circuit.
    pub fn new(
        circuit: &QuantumCircuit,
        limits: OptimizationLimits,
    ) -> Result<Self, CircuitViewError> {
        let view = CircuitView::new(circuit)?;

        Self::from_view(view, limits)
    }

    /// Creates an editor from an already validated view.
    pub fn from_view(
        view: CircuitView<'_>,
        limits: OptimizationLimits,
    ) -> Result<Self, CircuitViewError> {
        limits.validate()?;

        let snapshot = view.snapshot();

        Self::validate_snapshot(&snapshot, &limits)?;

        Ok(Self {
            snapshot,
            limits,
        })
    }

    /// Returns the optimizer-local snapshot.
    #[must_use]
    pub fn snapshot(&self) -> &CircuitSnapshot {
        &self.snapshot
    }

    /// Returns a mutable optimizer-local snapshot.
    ///
    /// This is crate-private deliberately. Public passes should use edit plans
    /// so transformations remain transactional.
    pub(crate) fn snapshot_mut(&mut self) -> &mut CircuitSnapshot {
        &mut self.snapshot
    }

    /// Returns the active optimizer limits.
    #[must_use]
    pub const fn limits(&self) -> &OptimizationLimits {
        &self.limits
    }

    /// Applies an edit plan atomically.
    pub fn apply(
        &mut self,
        plan: &CircuitEditPlan<'_>,
    ) -> Result<(), CircuitViewError> {
        let candidate =
            plan.apply(&self.snapshot, &self.limits)?;

        Self::validate_snapshot(
            &candidate,
            &self.limits,
        )?;

        self.snapshot = candidate;

        Ok(())
    }

    /// Returns the current number of operations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.snapshot.len()
    }

    /// Returns true if the current optimizer circuit is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.snapshot.is_empty()
    }

    /// Returns the current immutable operation sequence.
    #[must_use]
    pub fn operations(&self) -> &[Gate] {
        self.snapshot.operations()
    }

    /// Consumes the editor and returns the resulting operation sequence.
    #[must_use]
    pub fn into_operations(self) -> Vec<Gate> {
        self.snapshot.into_operations()
    }

    fn validate_snapshot(
        snapshot: &CircuitSnapshot,
        limits: &OptimizationLimits,
    ) -> Result<(), CircuitViewError> {
        if snapshot.num_qubits() as u64
            > limits.max_circuit_qubits()
        {
            return Err(
                CircuitViewError::CircuitQubitLimitExceeded {
                    requested: snapshot.num_qubits(),
                    maximum: limits.max_circuit_qubits(),
                },
            );
        }

        if snapshot.len() as u64
            > limits.max_circuit_operations()
        {
            return Err(
                CircuitViewError::CircuitOperationLimitExceeded {
                    requested: snapshot.len(),
                    maximum: limits.max_circuit_operations(),
                },
            );
        }

        for gate in snapshot.operations() {
            CircuitEdit::validate_gate(
                gate,
                snapshot.num_qubits(),
                snapshot.num_classical_bits(),
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// Utility functions
// ============================================================================

/// Returns the logical qubits touched by an operation.
///
/// The result is deterministic and preserves the gate's operand order.
#[must_use]
pub fn touched_qubits(gate: &Gate) -> &[QubitId] {
    gate.qubits()
}

/// Returns whether two operations share at least one logical qubit.
#[must_use]
pub fn operations_overlap(
    first: &Gate,
    second: &Gate,
) -> bool {
    first
        .qubits()
        .iter()
        .any(|qubit| second.qubits().contains(qubit))
}

/// Returns whether two operations are disjoint in their logical qubit
/// operands.
#[must_use]
pub fn operations_disjoint(
    first: &Gate,
    second: &Gate,
) -> bool {
    !operations_overlap(first, second)
}

/// Returns the number of distinct logical qubits touched by a sequence.
pub fn distinct_qubit_count(
    operations: &[Gate],
) -> Result<usize, CircuitViewError> {
    let mut qubits = Vec::<QubitId>::new();

    for gate in operations {
        for qubit in gate.qubits() {
            if !qubits.contains(qubit) {
                qubits
                    .try_reserve(1)
                    .map_err(|_| CircuitViewError::ArithmeticOverflow {
                        calculation: "distinct qubit storage",
                    })?;

                qubits.push(*qubit);
            }
        }
    }

    Ok(qubits.len())
}

/// Returns the operation indices touching a qubit.
///
/// The returned vector is deterministic and sorted by circuit order.
pub fn operation_indices_on_qubit(
    operations: &[Gate],
    qubit: QubitId,
) -> Result<Vec<OperationId>, CircuitViewError> {
    let mut result = Vec::new();

    for (index, gate) in operations.iter().enumerate() {
        if gate.qubits().contains(&qubit) {
            result
                .try_reserve(1)
                .map_err(|_| CircuitViewError::ArithmeticOverflow {
                    calculation: "qubit operation index storage",
                })?;

            result.push(OperationId::new(index));
        }
    }

    Ok(result)
}

/// Returns the inclusive operation interval covering all uses of a qubit.
///
/// `None` means that the qubit is unused.
pub fn qubit_live_interval(
    operations: &[Gate],
    qubit: QubitId,
) -> Option<Range<usize>> {
    let mut first = None;
    let mut last = None;

    for (index, gate) in operations.iter().enumerate() {
        if gate.qubits().contains(&qubit) {
            if first.is_none() {
                first = Some(index);
            }

            last = Some(index);
        }
    }

    match (first, last) {
        (Some(first), Some(last)) => {
            last.checked_add(1).map(|end| first..end)
        }

        _ => None,
    }
}

/// Returns whether an operation is a semantic boundary that should normally
/// prevent local optimizer movement across it.
///
/// This function is deliberately conservative.
///
/// More sophisticated semantic classification belongs in `operation.rs` and
/// analysis modules.
#[must_use]
pub fn is_hard_boundary(gate: &Gate) -> bool {
    gate.is_measurement()
        || gate.is_reset()
        || gate.is_barrier()
}

/// Returns whether an operation can be considered unitary at the canonical
/// logical level.
#[must_use]
pub fn is_unitary(gate: &Gate) -> bool {
    gate.is_unitary()
}

/// Validates a canonical circuit and creates a view.
///
/// This is the preferred public convenience function for optimizer entry
/// points.
pub fn view(
    circuit: &QuantumCircuit,
) -> Result<CircuitView<'_>, CircuitViewError> {
    CircuitView::new(circuit)
}

/// Validates a canonical circuit and creates a transactional editor.
pub fn editor(
    circuit: &QuantumCircuit,
    limits: OptimizationLimits,
) -> Result<CircuitEditor, CircuitViewError> {
    CircuitEditor::new(circuit, limits)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::gate::{
        Gate,
        GateKind,
    };
    use crate::quantum::ir::qubits::QubitId;
    use crate::quantum::ir::parameter::Parameter;
    use crate::quantum::ir::QuantumCircuit;

    fn x(
        qubit: usize,
    ) -> Gate {
        Gate::new(
            GateKind::X,
            vec![QubitId::new(qubit)],
            Vec::new(),
            None,
            None,
        )
        .expect("X gate should be valid")
    }

    fn h(
        qubit: usize,
    ) -> Gate {
        Gate::new(
            GateKind::H,
            vec![QubitId::new(qubit)],
            Vec::new(),
            None,
            None,
        )
        .expect("H gate should be valid")
    }

    #[test]
    fn operation_ids_are_deterministic() {
        let circuit =
            QuantumCircuit::from_operations(
                2,
                0,
                vec![x(0), h(1), x(0)],
            )
            .expect("circuit should be valid");

        let view =
            CircuitView::new(&circuit)
                .expect("view should be valid");

        let ids: Vec<_> =
            view.operation_ids().collect();

        assert_eq!(
            ids,
            vec![
                OperationId::new(0),
                OperationId::new(1),
                OperationId::new(2),
            ]
        );
    }

    #[test]
    fn slice_preserves_absolute_operation_ids() {
        let circuit =
            QuantumCircuit::from_operations(
                2,
                0,
                vec![x(0), h(1), x(0)],
            )
            .expect("circuit should be valid");

        let view =
            CircuitView::new(&circuit)
                .expect("view should be valid");

        let slice =
            view.slice(1..3)
                .expect("slice should be valid");

        assert_eq!(
            slice.get(0).expect("operation").id(),
            OperationId::new(1)
        );

        assert_eq!(
            slice.get(1).expect("operation").id(),
            OperationId::new(2)
        );
    }

    #[test]
    fn cursor_is_deterministic() {
        let circuit =
            QuantumCircuit::from_operations(
                2,
                0,
                vec![x(0), h(1)],
            )
            .expect("circuit should be valid");

        let view =
            CircuitView::new(&circuit)
                .expect("view should be valid");

        let mut cursor = view.cursor();

        assert_eq!(
            cursor.position(),
            0
        );

        assert_eq!(
            cursor
                .advance()
                .expect("advance")
                .expect("operation")
                .id(),
            OperationId::new(0)
        );

        assert_eq!(
            cursor
                .advance()
                .expect("advance")
                .expect("operation")
                .id(),
            OperationId::new(1)
        );

        assert!(
            cursor
                .advance()
                .expect("advance")
                .is_none()
        );
    }

    #[test]
    fn disjoint_operations_are_detected() {
        let first = x(0);
        let second = h(1);

        assert!(
            operations_disjoint(
                &first,
                &second
            )
        );
    }

    #[test]
    fn overlapping_operations_are_detected() {
        let first = x(0);
        let second = h(0);

        assert!(
            operations_overlap(
                &first,
                &second
            )
        );
    }

    #[test]
    fn qubit_live_interval_is_correct() {
        let operations =
            vec![x(0), h(1), h(0), x(0)];

        assert_eq!(
            qubit_live_interval(
                &operations,
                QubitId::new(0)
            ),
            Some(0..4)
        );

        assert_eq!(
            qubit_live_interval(
                &operations,
                QubitId::new(1)
            ),
            Some(1..2)
        );
    }

    #[test]
    fn edit_plan_is_atomic() {
        let circuit =
            QuantumCircuit::from_operations(
                2,
                0,
                vec![x(0), h(1), x(0)],
            )
            .expect("circuit should be valid");

        let view =
            CircuitView::new(&circuit)
                .expect("view should be valid");

        let mut plan =
            view.edit_plan();

        plan.remove(OperationId::new(1))
            .expect("remove should succeed");

        let snapshot =
            view.snapshot();

        let limits =
            OptimizationLimits::production();

        let result =
            plan.apply(
                &snapshot,
                &limits,
            )
            .expect("plan should apply");

        assert_eq!(
            result.operations().len(),
            2
        );

        // Original canonical circuit is untouched.
        assert_eq!(
            view.operations().len(),
            3
        );
    }

    #[test]
    fn replacement_is_atomic() {
        let circuit =
            QuantumCircuit::from_operations(
                1,
                0,
                vec![x(0)],
            )
            .expect("circuit should be valid");

        let view =
            CircuitView::new(&circuit)
                .expect("view should be valid");

        let mut plan =
            view.edit_plan();

        plan.replace(
            OperationId::new(0),
            h(0),
        )
        .expect("replacement should succeed");

        let result =
            plan.apply(
                &view.snapshot(),
                &OptimizationLimits::production(),
            )
            .expect("plan should apply");

        assert_eq!(
            result.operations()[0],
            h(0)
        );

        assert_eq!(
            view.operations()[0],
            x(0)
        );
    }

    #[test]
    fn conflicting_mutations_are_rejected() {
        let circuit =
            QuantumCircuit::from_operations(
                1,
                0,
                vec![x(0)],
            )
            .expect("circuit should be valid");

        let view =
            CircuitView::new(&circuit)
                .expect("view should be valid");

        let mut plan =
            view.edit_plan();

        plan.remove(OperationId::new(0))
            .expect("first edit should succeed");

        let result =
            plan.replace(
                OperationId::new(0),
                h(0),
            );

        assert!(
            matches!(
                result,
                Err(CircuitViewError::ConflictingEdits { .. })
            )
        );
    }

    #[test]
    fn multiple_insertions_at_same_anchor_are_allowed() {
        let circuit =
            QuantumCircuit::from_operations(
                1,
                0,
                vec![x(0)],
            )
            .expect("circuit should be valid");

        let view =
            CircuitView::new(&circuit)
                .expect("view should be valid");

        let mut plan =
            view.edit_plan();

        plan.insert_before(
            OperationId::new(0),
            vec![h(0)],
        )
        .expect("first insertion should succeed");

        plan.insert_before(
            OperationId::new(0),
            vec![x(0)],
        )
        .expect("second insertion should succeed");

        let result =
            plan.apply(
                &view.snapshot(),
                &OptimizationLimits::production(),
            )
            .expect("plan should apply");

        assert_eq!(
            result.len(),
            3
        );
    }

    #[test]
    fn snapshot_is_independent_from_canonical_circuit() {
        let circuit =
            QuantumCircuit::from_operations(
                1,
                0,
                vec![x(0)],
            )
            .expect("circuit should be valid");

        let view =
            CircuitView::new(&circuit)
                .expect("view should be valid");

        let mut snapshot =
            view.snapshot();

        snapshot
            .operations_mut()
            .clear();

        assert_eq!(
            view.len(),
            1
        );

        assert!(
            snapshot.is_empty()
        );
    }

    #[test]
    fn unitary_and_boundary_classification_is_conservative() {
        let unitary = x(0);

        assert!(
            is_unitary(&unitary)
        );

        assert!(
            !is_hard_boundary(&unitary)
        );
    }

    #[test]
    fn operation_overlap_is_symmetric() {
        let first = x(0);
        let second = h(0);

        assert_eq!(
            operations_overlap(
                &first,
                &second
            ),
            operations_overlap(
                &second,
                &first
            )
        );
    }

    #[test]
    fn distinct_qubit_count_is_deterministic() {
        let operations =
            vec![x(0), h(1), x(0)];

        assert_eq!(
            distinct_qubit_count(&operations)
                .expect("count should succeed"),
            2
        );
    }

    #[test]
    fn invalid_operation_id_is_rejected() {
        let circuit =
            QuantumCircuit::from_operations(
                1,
                0,
                vec![x(0)],
            )
            .expect("circuit should be valid");

        let view =
            CircuitView::new(&circuit)
                .expect("view should be valid");

        let result =
            view.operation_by_id(
                OperationId::new(99)
            );

        assert!(
            matches!(
                result,
                Err(
                    CircuitViewError::OperationOutOfRange {
                        ..
                    }
                )
            )
        );
    }
}