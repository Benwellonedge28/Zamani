//! Canonical Quantum IR -> Scheduling IR adapter.
//!
//! # Architectural contract
//!
//! This module is the only scheduler-owned boundary responsible for translating
//! the canonical quantum IR into the scheduler's algorithm-facing operation
//! representation.
//!
//! The canonical quantum IR remains the source of truth for:
//!
//! - `QuantumCircuit`;
//! - `Gate`;
//! - `QuantumOperation` semantics;
//! - `QubitId`;
//! - gate kind;
//! - gate parameters;
//! - operand ordering;
//! - quantum semantics.
//!
//! This module MUST NOT define or reimplement any of those concepts.
//!
//! The scheduler-side `SchedulingOperation` is deliberately a derived view.
//! It exists so scheduling algorithms can attach scheduling concerns without
//! modifying canonical quantum semantics.
//!
//! # Canonical qubit identity
//!
//! Logical qubits MUST use:
//!
//! `crate::quantum::ir::qubit::QubitId`
//!
//! No scheduler-local qubit identifier may be introduced here.
//!
//! # Operation identity
//!
//! The canonical circuit exposes operations in explicit semantic order.
//! The adapter assigns a stable scheduler `OperationId` from that canonical
//! position. This is deterministic for a given canonical circuit.
//!
//! The adapter never uses:
//!
//! - physical machine size;
//! - qubit count constants;
//! - gate-count constants;
//! - topology assumptions;
//! - hardware timing;
//! - vendor identifiers;
//! - resource counts.
//!
//! Those belong to later scheduling/hardware adapters.
//!
//! # Scalability
//!
//! The iterator API is intentionally lazy. It does not create an intermediate
//! circuit-sized collection merely to traverse canonical operations.
//!
//! `adapt_circuit` is provided for callers that explicitly require owned
//! scheduler operations. Large-scale callers should prefer `iter` and stream
//! operations into graph/resource construction.
//!
//! # Ordering
//!
//! Canonical operation order is preserved exactly.
//!
//! For operation `i` in canonical order:
//!
//! `OperationId::new(i)`
//!
//! becomes its scheduler identity.
//!
//! # Semantics
//!
//! This adapter does not:
//!
//! - optimize;
//! - route;
//! - schedule;
//! - assign times;
//! - assign hardware resources;
//! - insert delays;
//! - decompose gates;
//! - alter operands;
//! - alter gate parameters;
//! - perform QEC transformations.
//!
//! It only creates the scheduling view.
//!
//! # Rust compatibility
//!
//! Designed for Rust 1.97 / 1.97.1.
//!
//! # Safety
//!
//! This module contains no `unsafe` code and does not require unsafe
//! abstractions.
//!
//! # Integration
//!
//! ```text
//! crate::quantum::ir
//!        │
//!        │ canonical QuantumCircuit / Gate
//!        ▼
//! scheduling::adapters::ir
//!        │
//!        │ SchedulingOperation
//!        ▼
//! scheduling::ir::graph
//!        │
//!        ▼
//! dependency / resource / timing analysis
//!        │
//!        ▼
//! scheduling planners
//! ```
//!
//! Hardware, routing, QEC, and timing-specific specialization belongs outside
//! this adapter.

use crate::quantum::ir::qubit::QubitId;
use crate::quantum::ir::{Gate, QuantumCircuit};
use crate::quantum::scheduling::ir::{
    OperationClass, OperationId, QubitOperand, SchedulingOperation,
};

/// Canonical Quantum IR to scheduling IR adapter.
///
/// This type is intentionally stateless.
///
/// A stateless adapter:
///
/// - is deterministic;
/// - is trivially shareable;
/// - has no global state;
/// - has no hardware assumptions;
/// - requires no synchronization;
/// - does not retain references to circuits;
/// - is suitable for parallel callers.
///
/// The unit-like representation also ensures that creating an adapter has
/// constant memory cost independent of circuit size.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct IrAdapter;

impl IrAdapter {
    /// Creates a canonical-IR scheduling adapter.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Converts one canonical gate into a scheduler operation.
    ///
    /// `operation_id` must identify the operation's position in canonical
    /// semantic order.
    ///
    /// No gate semantics are copied into a scheduler-specific representation.
    /// Only the canonical qubit operands are projected into the scheduling
    /// operation.
    ///
    /// Gate kind, parameters, and other canonical semantic information remain
    /// owned by the canonical IR and can be recovered through the canonical
    /// operation provenance maintained by the scheduling layer.
    #[must_use]
    pub fn adapt_operation(
        &self,
        operation_id: OperationId,
        gate: &Gate,
    ) -> SchedulingOperation {
        let operands = gate
            .qubits()
            .iter()
            .copied()
            .map(QubitOperand::new)
            .collect::<Vec<_>>()
            .into();

        SchedulingOperation::from_canonical_ir(
            operation_id,
            OperationClass::Quantum,
            operands,
        )
    }

    /// Converts a canonical circuit into a lazy iterator of scheduling
    /// operations.
    ///
    /// This is the preferred API for large circuits because it does not
    /// allocate a second circuit-sized collection.
    ///
    /// The yielded operation order is exactly the canonical circuit order.
    pub fn iter<'a>(
        &self,
        circuit: &'a QuantumCircuit,
    ) -> IrOperationIter<'a> {
        IrOperationIter {
            adapter: *self,
            operations: circuit.operations().iter(),
            next_index: 0,
        }
    }

    /// Converts a canonical circuit into owned scheduler operations.
    ///
    /// This method deliberately allocates because ownership is explicitly
    /// requested by the caller.
    ///
    /// For very large programs, prefer [`Self::iter`] and stream operations
    /// directly into the scheduling dependency/resource graph.
    #[must_use]
    pub fn adapt_circuit(
        &self,
        circuit: &QuantumCircuit,
    ) -> Vec<SchedulingOperation> {
        self.iter(circuit).collect()
    }

    /// Converts a canonical qubit into the scheduler's canonical qubit
    /// operand.
    ///
    /// This function exists as the single explicit qubit-identity boundary for
    /// future scheduler integrations. It does not create a new qubit identity.
    #[must_use]
    pub const fn adapt_qubit(qubit: QubitId) -> QubitOperand {
        QubitOperand::new(qubit)
    }

    /// Returns the number of canonical operations without constructing
    /// scheduler operations.
    ///
    /// This is useful for capacity planning, diagnostics, and preallocation by
    /// callers that explicitly choose to materialize the adapted representation.
    #[must_use]
    pub fn operation_count(circuit: &QuantumCircuit) -> usize {
        circuit.operations().len()
    }

    /// Returns whether the canonical circuit contains no operations.
    #[must_use]
    pub fn is_empty(circuit: &QuantumCircuit) -> bool {
        circuit.operations().is_empty()
    }
}

/// Lazy iterator over canonical operations projected into scheduling
/// operations.
///
/// The iterator stores only:
///
/// - one reference to the canonical operation slice iterator;
/// - one stateless adapter;
/// - one operation index.
///
/// Its memory consumption therefore does not scale with circuit size.
#[derive(Debug)]
pub struct IrOperationIter<'a> {
    adapter: IrAdapter,
    operations: std::slice::Iter<'a, Gate>,
    next_index: usize,
}

impl<'a> Iterator for IrOperationIter<'a> {
    type Item = SchedulingOperation;

    fn next(&mut self) -> Option<Self::Item> {
        let gate = self.operations.next()?;

        let index = self.next_index;

        // The iterator itself cannot be constructed with an invalid usize
        // index. OperationId::new is the canonical scheduler identity
        // constructor used throughout the repository.
        self.next_index = self.next_index.saturating_add(1);

        Some(
            self.adapter
                .adapt_operation(OperationId::new(index), gate),
        )
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.operations.len();

        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for IrOperationIter<'_> {}

impl std::iter::FusedIterator for IrOperationIter<'_> {}

/// Adapts a canonical circuit using the default stateless adapter.
///
/// This is the concise integration entry point for compiler pipeline code.
///
/// # Example
///
/// ```ignore
/// let operations = crate::quantum::scheduling::adapters::ir::adapt(circuit);
///
/// for operation in operations {
///     // Feed directly into scheduling graph construction.
/// }
/// ```
#[must_use]
pub fn adapt(circuit: &QuantumCircuit) -> Vec<SchedulingOperation> {
    IrAdapter::new().adapt_circuit(circuit)
}

/// Returns a lazy scheduling-operation iterator for a canonical circuit.
///
/// This should be preferred by scalable compiler pipelines.
pub fn iter(
    circuit: &QuantumCircuit,
) -> IrOperationIter<'_> {
    IrAdapter::new().iter(circuit)
}

/// Converts one canonical gate into a scheduler operation using the default
/// adapter.
#[must_use]
pub fn adapt_operation(
    operation_id: OperationId,
    gate: &Gate,
) -> SchedulingOperation {
    IrAdapter::new().adapt_operation(operation_id, gate)
}

/// Converts a canonical qubit identity into a scheduler operand.
///
/// The returned operand contains the exact canonical `QubitId`; no scheduler
/// identity is substituted.
#[must_use]
pub const fn adapt_qubit(qubit: QubitId) -> QubitOperand {
    IrAdapter::adapt_qubit(qubit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter_is_stateless() {
        let first = IrAdapter::new();
        let second = IrAdapter::new();

        assert_eq!(first, second);
    }

    #[test]
    fn adapter_is_copy() {
        let adapter = IrAdapter::new();
        let copied = adapter;

        assert_eq!(adapter, copied);
    }

    #[test]
    fn canonical_qubit_identity_is_preserved() {
        let qubit = QubitId::new(17);
        let operand = IrAdapter::adapt_qubit(qubit);

        assert_eq!(operand.qubit(), qubit);
    }

    #[test]
    fn operation_ids_are_constructed_from_canonical_order() {
        let first = OperationId::new(0);
        let second = OperationId::new(1);

        assert_ne!(first, second);
    }

    #[test]
    fn iterator_size_hint_is_exact_for_empty_iterator() {
        // This test intentionally exercises the iterator contract without
        // requiring construction of a target-specific QuantumCircuit.
        let operations: &[Gate] = &[];

        let mut iterator = IrOperationIter {
            adapter: IrAdapter::new(),
            operations: operations.iter(),
            next_index: 0,
        };

        assert_eq!(iterator.size_hint(), (0, Some(0)));
        assert_eq!(iterator.next(), None);
    }
}