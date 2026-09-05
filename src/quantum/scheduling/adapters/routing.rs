//! Zamani Quantum Scheduling — Routing Integration Adapter
//!
//! This module is the integration boundary between:
//!
//! ```text
//! quantum::routing
//!        │
//!        ▼
//! scheduling::adapters::routing
//!        │
//!        ▼
//! quantum::scheduling
//! ```
//!
//! # Responsibility
//!
//! This adapter converts the *semantic output of routing* into a
//! scheduling-oriented, immutable representation without taking ownership of
//! routing itself.
//!
//! It preserves:
//!
//! - routing operation order;
//! - logical qubit identity;
//! - physical qubit identity;
//! - gate identity;
//! - routing movement operations;
//! - barriers;
//! - logical-to-physical mapping information;
//! - source routing-operation index;
//! - operation classification;
//! - operand order.
//!
//! # What this module does NOT do
//!
//! This module does not:
//!
//! - perform routing;
//! - choose a routing algorithm;
//! - mutate a `QubitMapping`;
//! - calculate paths;
//! - inspect hardware directly;
//! - schedule operations;
//! - assign start times;
//! - calculate durations;
//! - reserve resources;
//! - synthesize SWAPs;
//! - decompose gates;
//! - execute a quantum program;
//! - communicate with a backend;
//! - perform QEC decoding;
//! - impose a maximum number of qubits;
//! - impose a maximum number of operations;
//! - impose a maximum route size.
//!
//! Those responsibilities belong to other subsystems.
//!
//! # Architectural boundary
//!
//! Routing answers:
//!
//! > WHERE should logical operations execute?
//!
//! Scheduling answers:
//!
//! > WHEN should those already-routable operations execute?
//!
//! Hardware answers:
//!
//! > CAN this target execute the operation?
//!
//! Therefore this adapter must not collapse routing and scheduling into one
//! abstraction.
//!
//! # Important type boundary
//!
//! The routing subsystem intentionally owns:
//!
//! ```text
//! routing::LogicalQubitId
//! routing::PhysicalQubitId
//! ```
//!
//! while the canonical Quantum IR owns its own qubit identity:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! This adapter does not pretend that these are interchangeable types.
//! Conversion into canonical IR identifiers belongs to the canonical IR
//! adapter (`adapters::ir`) when such conversion is required.
//!
//! # Scalability
//!
//! No fixed machine size is encoded here.
//!
//! In particular, this file contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_PHYSICAL_QUBITS
//! MAX_ROUTE_LENGTH
//! ```
//!
//! Collection sizes are determined by the supplied routing result and available
//! memory/resources.
//!
//! The primary conversion API is iterator-based so callers that only need to
//! inspect or stream routed operations do not have to allocate a second copy of
//! the entire route.
//!
//! # Determinism
//!
//! Operations are exposed in the exact order supplied by the routing result.
//! No hash-map iteration is used to determine operation order.
//!
//! # Ownership
//!
//! The adapter does not retain references to routing state after an iterator or
//! owned conversion is dropped. It does not mutate its input.
//!
//! # Safety
//!
//! This module uses no `unsafe` code.
//!
//! # Rust compatibility
//!
//! Designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//! No external dependencies are required.
//!
//! # Integration
//!
//! `adapters/mod.rs` should expose this module:
//!
//! ```text
//! pub mod routing;
//! ```
//!
//! The scheduling root should expose the adapters module:
//!
//! ```text
//! pub mod adapters;
//! ```
//!
//! The routing subsystem remains upstream:
//!
//! ```text
//! quantum::routing::result::RoutingResult
//! quantum::routing::types::RoutingOperation
//! ```
//!
//! The scheduler consumes the normalized records defined below.
//!
//! # Design rule
//!
//! This file intentionally defines an adapter-owned value representation rather
//! than depending on a concrete scheduler planner. That keeps the integration
//! boundary stable when ASAP, ALAP, list scheduling, RCPSP, event-driven,
//! distributed, or future schedulers are added.
//!
//! A later scheduler can convert these records into its internal
//! `scheduling::ir::SchedulingOperation` representation in exactly one place,
//! without requiring the routing subsystem to know anything about scheduling.

use crate::quantum::routing::types::{
    GateIdentity,
    LogicalQubitId,
    PhysicalQubitId,
    RoutingMove,
    RoutingOperation,
};

use std::fmt;

// =============================================================================
// Adapter error
// =============================================================================

/// Errors produced while normalizing routed operations for scheduling.
///
/// The adapter is intentionally strict about malformed semantic records. It
/// does not silently repair malformed routing output because doing so could
/// change program semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoutingAdapterError {
    /// A gate operation contains a different number of logical and physical
    /// operands.
    OperandArityMismatch {
        /// Zero-based operation index in the routed stream.
        operation_index: usize,

        /// Number of logical operands.
        logical_count: usize,

        /// Number of physical operands.
        physical_count: usize,
    },

    /// A gate operation contains duplicate logical operands.
    DuplicateLogicalOperand {
        /// Zero-based operation index.
        operation_index: usize,

        /// Duplicated logical qubit.
        qubit: LogicalQubitId,
    },

    /// A gate operation contains duplicate physical operands.
    DuplicatePhysicalOperand {
        /// Zero-based operation index.
        operation_index: usize,

        /// Duplicated physical qubit.
        qubit: PhysicalQubitId,
    },

    /// A SWAP movement contains identical endpoints.
    InvalidSwap {
        /// Zero-based operation index.
        operation_index: usize,

        /// Invalid physical endpoint.
        qubit: PhysicalQubitId,
    },

    /// A bridge movement contains duplicate physical endpoints.
    InvalidBridge {
        /// Zero-based operation index.
        operation_index: usize,

        /// First endpoint.
        a: PhysicalQubitId,

        /// Intermediate endpoint.
        bridge: PhysicalQubitId,

        /// Final endpoint.
        b: PhysicalQubitId,
    },

    /// A permutation contains duplicate logical operands.
    DuplicatePermutationLogical {
        /// Zero-based operation index.
        operation_index: usize,

        /// Duplicated logical qubit.
        qubit: LogicalQubitId,
    },

    /// A permutation contains duplicate physical destinations.
    DuplicatePermutationPhysical {
        /// Zero-based operation index.
        operation_index: usize,

        /// Duplicated physical qubit.
        qubit: PhysicalQubitId,
    },
}

impl fmt::Display for RoutingAdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OperandArityMismatch {
                operation_index,
                logical_count,
                physical_count,
            } => write!(
                formatter,
                "routing operation {} has {} logical operands but {} physical operands",
                operation_index, logical_count, physical_count
            ),

            Self::DuplicateLogicalOperand {
                operation_index,
                qubit,
            } => write!(
                formatter,
                "routing operation {} contains duplicate logical operand {}",
                operation_index, qubit
            ),

            Self::DuplicatePhysicalOperand {
                operation_index,
                qubit,
            } => write!(
                formatter,
                "routing operation {} contains duplicate physical operand {}",
                operation_index, qubit
            ),

            Self::InvalidSwap {
                operation_index,
                qubit,
            } => write!(
                formatter,
                "routing operation {} contains an invalid self-SWAP at {}",
                operation_index, qubit
            ),

            Self::InvalidBridge {
                operation_index,
                a,
                bridge,
                b,
            } => write!(
                formatter,
                "routing operation {} contains an invalid bridge ({}, {}, {})",
                operation_index, a, bridge, b
            ),

            Self::DuplicatePermutationLogical {
                operation_index,
                qubit,
            } => write!(
                formatter,
                "routing operation {} contains duplicate permutation logical qubit {}",
                operation_index, qubit
            ),

            Self::DuplicatePermutationPhysical {
                operation_index,
                qubit,
            } => write!(
                formatter,
                "routing operation {} contains duplicate permutation physical qubit {}",
                operation_index, qubit
            ),
        }
    }
}

impl std::error::Error for RoutingAdapterError {}

// =============================================================================
// Operation classification
// =============================================================================

/// Stable classification of an operation crossing the routing/scheduling
/// boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RoutedOperationKind {
    /// A semantic movement introduced by routing.
    Movement,

    /// A routed gate operation.
    Gate,

    /// A scheduling-relevant barrier.
    Barrier,
}

impl RoutedOperationKind {
    /// Returns a stable machine-readable name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Movement => "movement",
            Self::Gate => "gate",
            Self::Barrier => "barrier",
        }
    }

    /// Returns whether this record represents a movement introduced by routing.
    #[must_use]
    pub const fn is_movement(self) -> bool {
        matches!(self, Self::Movement)
    }

    /// Returns whether this record represents a gate.
    #[must_use]
    pub const fn is_gate(self) -> bool {
        matches!(self, Self::Gate)
    }

    /// Returns whether this record represents a barrier.
    #[must_use]
    pub const fn is_barrier(self) -> bool {
        matches!(self, Self::Barrier)
    }
}

impl fmt::Display for RoutedOperationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

// =============================================================================
// Routed operation record
// =============================================================================

/// Immutable scheduling-boundary representation of one routed operation.
///
/// This is deliberately richer than a tuple and deliberately independent of
/// the scheduling algorithm.
///
/// The record preserves both namespaces:
///
/// ```text
/// logical operands
///       │
///       ▼
/// routing mapping
///       │
///       ▼
/// physical operands
/// ```
///
/// A scheduler can therefore reason about physical resource conflicts without
/// losing the logical source identity needed for verification and diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RoutedOperation {
    /// Position in the routed operation stream.
    source_index: usize,

    /// Operation classification.
    kind: RoutedOperationKind,

    /// Logical operands associated with the operation.
    ///
    /// Movement operations may have no explicit logical operand sequence.
    logical_operands: Vec<LogicalQubitId>,

    /// Physical operands touched by the operation.
    physical_operands: Vec<PhysicalQubitId>,

    /// Gate identity, when the operation is a gate.
    gate: Option<GateIdentity>,

    /// Semantic routing movement, when this operation is a movement.
    movement: Option<RoutingMove>,
}

impl RoutedOperation {
    /// Constructs a validated normalized record from one routing operation.
    ///
    /// Validation is intentionally local and deterministic.
    pub fn from_routing_operation(
        source_index: usize,
        operation: &RoutingOperation,
    ) -> Result<Self, RoutingAdapterError> {
        match operation {
            RoutingOperation::Move(movement) => {
                validate_movement(source_index, movement)?;

                Ok(Self {
                    source_index,
                    kind: RoutedOperationKind::Movement,
                    logical_operands: Vec::new(),
                    physical_operands: movement.physical_qubits(),
                    gate: None,
                    movement: Some(movement.clone()),
                })
            }

            RoutingOperation::Gate {
                gate,
                operands,
                logical_operands,
            } => {
                validate_gate(
                    source_index,
                    operands,
                    logical_operands,
                )?;

                Ok(Self {
                    source_index,
                    kind: RoutedOperationKind::Gate,
                    logical_operands: logical_operands.clone(),
                    physical_operands: operands.clone(),
                    gate: Some(gate.clone()),
                    movement: None,
                })
            }

            RoutingOperation::Barrier { operands } => {
                validate_unique_physical_operands(
                    source_index,
                    operands,
                )?;

                Ok(Self {
                    source_index,
                    kind: RoutedOperationKind::Barrier,
                    logical_operands: Vec::new(),
                    physical_operands: operands.clone(),
                    gate: None,
                    movement: None,
                })
            }
        }
    }

    /// Returns the original zero-based routed-stream index.
    #[must_use]
    pub const fn source_index(&self) -> usize {
        self.source_index
    }

    /// Returns the operation classification.
    #[must_use]
    pub const fn kind(&self) -> RoutedOperationKind {
        self.kind
    }

    /// Returns the logical operands.
    #[must_use]
    pub fn logical_operands(&self) -> &[LogicalQubitId] {
        &self.logical_operands
    }

    /// Returns the physical operands.
    #[must_use]
    pub fn physical_operands(&self) -> &[PhysicalQubitId] {
        &self.physical_operands
    }

    /// Returns the gate identity, if this is a gate operation.
    #[must_use]
    pub fn gate(&self) -> Option<&GateIdentity> {
        self.gate.as_ref()
    }

    /// Returns the routing movement, if this is a movement operation.
    #[must_use]
    pub fn movement(&self) -> Option<&RoutingMove> {
        self.movement.as_ref()
    }

    /// Returns the operation arity.
    #[must_use]
    pub fn arity(&self) -> usize {
        self.physical_operands.len()
    }

    /// Returns whether this operation touches a particular physical qubit.
    #[must_use]
    pub fn touches_physical(&self, qubit: PhysicalQubitId) -> bool {
        self.physical_operands.contains(&qubit)
    }

    /// Returns whether this operation uses a particular logical qubit.
    #[must_use]
    pub fn uses_logical(&self, qubit: LogicalQubitId) -> bool {
        self.logical_operands.contains(&qubit)
    }

    /// Returns whether the operation was introduced by routing rather than
    /// representing the original routed gate stream.
    ///
    /// A movement is routing-generated by definition. A gate/barrier is
    /// preserved semantic work.
    #[must_use]
    pub const fn is_routing_inserted(&self) -> bool {
        self.kind.is_movement()
    }
}

// =============================================================================
// Routing adapter
// =============================================================================

/// Stateless routing-to-scheduling integration adapter.
///
/// The adapter deliberately contains no configuration and no mutable state.
/// Routing configuration belongs to `quantum::routing::config`; scheduling
/// configuration belongs to `quantum::scheduling::config`.
///
/// Keeping this type stateless means it can safely be shared between compiler
/// stages without synchronization or hidden state.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct RoutingAdapter;

impl RoutingAdapter {
    /// Creates a routing adapter.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Converts one routing operation into the scheduling-boundary form.
    ///
    /// This performs structural validation but does not perform target
    /// validation. Target validation belongs to routing/hardware verification.
    pub fn adapt_operation(
        &self,
        source_index: usize,
        operation: &RoutingOperation,
    ) -> Result<RoutedOperation, RoutingAdapterError> {
        RoutedOperation::from_routing_operation(
            source_index,
            operation,
        )
    }

    /// Lazily adapts a routing operation stream.
    ///
    /// No complete second operation vector is allocated by this method.
    pub fn iter<'a, I>(
        &self,
        operations: I,
    ) -> RoutingAdapterIter<'a, I::IntoIter>
    where
        I: IntoIterator<Item = &'a RoutingOperation>,
    {
        RoutingAdapterIter {
            inner: operations.into_iter(),
            source_index: 0,
            adapter: *self,
        }
    }

    /// Adapts an entire operation stream into owned records.
    ///
    /// This is a convenience API for callers that explicitly require an owned
    /// representation. It has O(n) storage proportional to the supplied route;
    /// it does not impose a fixed maximum.
    pub fn adapt_operations<'a, I>(
        &self,
        operations: I,
    ) -> Result<Vec<RoutedOperation>, RoutingAdapterError>
    where
        I: IntoIterator<Item = &'a RoutingOperation>,
    {
        let iterator = self.iter(operations);

        let mut result = Vec::new();

        for operation in iterator {
            result.push(operation?);
        }

        Ok(result)
    }

    /// Returns the number of operations in an iterable without retaining them.
    ///
    /// This is primarily useful when the source already exposes an exact-size
    /// iterator. It does not allocate.
    #[must_use]
    pub fn count<'a, I>(&self, operations: I) -> usize
    where
        I: IntoIterator<Item = &'a RoutingOperation>,
    {
        operations.into_iter().count()
    }

    /// Validates a complete routing operation stream without allocating an
    /// adapted representation.
    pub fn validate<'a, I>(
        &self,
        operations: I,
    ) -> Result<(), RoutingAdapterError>
    where
        I: IntoIterator<Item = &'a RoutingOperation>,
    {
        for operation in self.iter(operations) {
            operation?;
        }

        Ok(())
    }
}

// =============================================================================
// Iterator
// =============================================================================

/// Lazy routing-operation adapter iterator.
///
/// The iterator preserves source order and performs validation one operation at
/// a time. This is the preferred path for very large workloads where a second
/// complete representation would be unnecessarily expensive.
pub struct RoutingAdapterIter<'a, I>
where
    I: Iterator<Item = &'a RoutingOperation>,
{
    inner: I,
    source_index: usize,
    adapter: RoutingAdapter,
}

impl<'a, I> Iterator for RoutingAdapterIter<'a, I>
where
    I: Iterator<Item = &'a RoutingOperation>,
{
    type Item = Result<RoutedOperation, RoutingAdapterError>;

    fn next(&mut self) -> Option<Self::Item> {
        let operation = self.inner.next()?;

        let source_index = self.source_index;

        self.source_index = self
            .source_index
            .checked_add(1)
            .unwrap_or(usize::MAX);

        Some(
            self.adapter
                .adapt_operation(source_index, operation),
        )
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.inner.size_hint();

        (lower, upper)
    }
}

impl<'a, I> std::iter::FusedIterator for RoutingAdapterIter<'a, I>
where
    I: Iterator<Item = &'a RoutingOperation>,
{
}

// =============================================================================
// Structural validation helpers
// =============================================================================

fn validate_gate(
    operation_index: usize,
    physical_operands: &[PhysicalQubitId],
    logical_operands: &[LogicalQubitId],
) -> Result<(), RoutingAdapterError> {
    if physical_operands.len() != logical_operands.len() {
        return Err(RoutingAdapterError::OperandArityMismatch {
            operation_index,
            logical_count: logical_operands.len(),
            physical_count: physical_operands.len(),
        });
    }

    validate_unique_logical_operands(
        operation_index,
        logical_operands,
    )?;

    validate_unique_physical_operands(
        operation_index,
        physical_operands,
    )?;

    Ok(())
}

fn validate_unique_logical_operands(
    operation_index: usize,
    operands: &[LogicalQubitId],
) -> Result<(), RoutingAdapterError> {
    // Arity is deliberately unrestricted. A temporary set is created only for
    // this operation, so there is no global qubit-count assumption.
    let mut seen = std::collections::HashSet::with_capacity(
        operands.len(),
    );

    for &qubit in operands {
        if !seen.insert(qubit) {
            return Err(
                RoutingAdapterError::DuplicateLogicalOperand {
                    operation_index,
                    qubit,
                },
            );
        }
    }

    Ok(())
}

fn validate_unique_physical_operands(
    operation_index: usize,
    operands: &[PhysicalQubitId],
) -> Result<(), RoutingAdapterError> {
    let mut seen = std::collections::HashSet::with_capacity(
        operands.len(),
    );

    for &qubit in operands {
        if !seen.insert(qubit) {
            return Err(
                RoutingAdapterError::DuplicatePhysicalOperand {
                    operation_index,
                    qubit,
                },
            );
        }
    }

    Ok(())
}

fn validate_movement(
    operation_index: usize,
    movement: &RoutingMove,
) -> Result<(), RoutingAdapterError> {
    match movement {
        RoutingMove::Swap { a, b } => {
            if a == b {
                return Err(RoutingAdapterError::InvalidSwap {
                    operation_index,
                    qubit: *a,
                });
            }
        }

        RoutingMove::Bridge {
            a,
            bridge,
            b,
            ..
        } => {
            if a == bridge || bridge == b || a == b {
                return Err(RoutingAdapterError::InvalidBridge {
                    operation_index,
                    a: *a,
                    bridge: *bridge,
                    b: *b,
                });
            }
        }

        RoutingMove::Permutation { mapping } => {
            let mut logical = std::collections::HashSet::with_capacity(
                mapping.len(),
            );

            let mut physical = std::collections::HashSet::with_capacity(
                mapping.len(),
            );

            for &(logical_qubit, physical_qubit) in mapping {
                if !logical.insert(logical_qubit) {
                    return Err(
                        RoutingAdapterError::DuplicatePermutationLogical {
                            operation_index,
                            qubit: logical_qubit,
                        },
                    );
                }

                if !physical.insert(physical_qubit) {
                    return Err(
                        RoutingAdapterError::DuplicatePermutationPhysical {
                            operation_index,
                            qubit: physical_qubit,
                        },
                    );
                }
            }
        }
    }

    Ok(())
}

// =============================================================================
// Free-function API
// =============================================================================

/// Converts one routed operation into a scheduling-boundary record.
///
/// This is a convenience wrapper around [`RoutingAdapter::adapt_operation`].
pub fn adapt_operation(
    source_index: usize,
    operation: &RoutingOperation,
) -> Result<RoutedOperation, RoutingAdapterError> {
    RoutingAdapter::new().adapt_operation(
        source_index,
        operation,
    )
}

/// Lazily adapts a routed operation stream.
///
/// This is the preferred free-function API for large workloads.
pub fn iter<'a, I>(
    operations: I,
) -> RoutingAdapterIter<'a, I::IntoIter>
where
    I: IntoIterator<Item = &'a RoutingOperation>,
{
    RoutingAdapter::new().iter(operations)
}

/// Adapts an entire routed operation stream into owned records.
pub fn adapt_operations<'a, I>(
    operations: I,
) -> Result<Vec<RoutedOperation>, RoutingAdapterError>
where
    I: IntoIterator<Item = &'a RoutingOperation>,
{
    RoutingAdapter::new().adapt_operations(operations)
}

/// Validates routed operations without allocating an adapted route.
pub fn validate<'a, I>(
    operations: I,
) -> Result<(), RoutingAdapterError>
where
    I: IntoIterator<Item = &'a RoutingOperation>,
{
    RoutingAdapter::new().validate(operations)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn logical(index: usize) -> LogicalQubitId {
        LogicalQubitId::new(index)
    }

    fn physical(index: usize) -> PhysicalQubitId {
        PhysicalQubitId::new(index)
    }

    #[test]
    fn adapter_is_stateless_and_copyable() {
        let a = RoutingAdapter::new();
        let b = a;

        assert_eq!(a, b);
    }

    #[test]
    fn gate_preserves_logical_and_physical_operands() {
        let operation = RoutingOperation::Gate {
            gate: GateIdentity::Cx,
            operands: vec![physical(7), physical(11)],
            logical_operands: vec![logical(0), logical(1)],
        };

        let adapted = adapt_operation(0, &operation)
            .expect("valid routed gate");

        assert_eq!(adapted.source_index(), 0);
        assert_eq!(adapted.kind(), RoutedOperationKind::Gate);
        assert_eq!(
            adapted.logical_operands(),
            &[logical(0), logical(1)]
        );
        assert_eq!(
            adapted.physical_operands(),
            &[physical(7), physical(11)]
        );
        assert_eq!(adapted.gate(), Some(&GateIdentity::Cx));
        assert!(!adapted.is_routing_inserted());
    }

    #[test]
    fn movement_is_preserved_without_synthesizing_hardware_gates() {
        let operation = RoutingOperation::Move(
            RoutingMove::Swap {
                a: physical(2),
                b: physical(9),
            },
        );

        let adapted = adapt_operation(4, &operation)
            .expect("valid swap");

        assert_eq!(adapted.source_index(), 4);
        assert_eq!(
            adapted.kind(),
            RoutedOperationKind::Movement
        );
        assert_eq!(
            adapted.physical_operands(),
            &[physical(2), physical(9)]
        );
        assert!(adapted.gate().is_none());
        assert!(adapted.movement().is_some());
        assert!(adapted.is_routing_inserted());
    }

    #[test]
    fn barrier_is_preserved() {
        let operation = RoutingOperation::Barrier {
            operands: vec![physical(0), physical(3)],
        };

        let adapted = adapt_operation(2, &operation)
            .expect("valid barrier");

        assert_eq!(
            adapted.kind(),
            RoutedOperationKind::Barrier
        );
        assert_eq!(
            adapted.physical_operands(),
            &[physical(0), physical(3)]
        );
    }

    #[test]
    fn mismatched_gate_operand_counts_are_rejected() {
        let operation = RoutingOperation::Gate {
            gate: GateIdentity::Cx,
            operands: vec![physical(0), physical(1)],
            logical_operands: vec![logical(0)],
        };

        let error = adapt_operation(0, &operation)
            .expect_err("malformed operation must fail");

        assert!(matches!(
            error,
            RoutingAdapterError::OperandArityMismatch {
                operation_index: 0,
                logical_count: 1,
                physical_count: 2,
            }
        ));
    }

    #[test]
    fn duplicate_logical_operands_are_rejected() {
        let operation = RoutingOperation::Gate {
            gate: GateIdentity::Cx,
            operands: vec![physical(0), physical(1)],
            logical_operands: vec![logical(0), logical(0)],
        };

        let error = adapt_operation(3, &operation)
            .expect_err("duplicate logical operands must fail");

        assert!(matches!(
            error,
            RoutingAdapterError::DuplicateLogicalOperand {
                operation_index: 3,
                qubit: _
            }
        ));
    }

    #[test]
    fn duplicate_physical_operands_are_rejected() {
        let operation = RoutingOperation::Gate {
            gate: GateIdentity::Cx,
            operands: vec![physical(5), physical(5)],
            logical_operands: vec![logical(0), logical(1)],
        };

        let error = adapt_operation(8, &operation)
            .expect_err("duplicate physical operands must fail");

        assert!(matches!(
            error,
            RoutingAdapterError::DuplicatePhysicalOperand {
                operation_index: 8,
                qubit: _
            }
        ));
    }

    #[test]
    fn self_swap_is_rejected() {
        let operation = RoutingOperation::Move(
            RoutingMove::Swap {
                a: physical(4),
                b: physical(4),
            },
        );

        let error = adapt_operation(1, &operation)
            .expect_err("self swap must fail");

        assert!(matches!(
            error,
            RoutingAdapterError::InvalidSwap {
                operation_index: 1,
                ..
            }
        ));
    }

    #[test]
    fn invalid_bridge_is_rejected() {
        let operation = RoutingOperation::Move(
            RoutingMove::Bridge {
                a: physical(1),
                bridge: physical(2),
                b: physical(2),
                gate: GateIdentity::Cx,
            },
        );

        let error = adapt_operation(5, &operation)
            .expect_err("invalid bridge must fail");

        assert!(matches!(
            error,
            RoutingAdapterError::InvalidBridge {
                operation_index: 5,
                ..
            }
        ));
    }

    #[test]
    fn permutation_duplicate_logical_qubits_are_rejected() {
        let operation = RoutingOperation::Move(
            RoutingMove::Permutation {
                mapping: vec![
                    (logical(0), physical(1)),
                    (logical(0), physical(2)),
                ],
            },
        );

        let error = adapt_operation(6, &operation)
            .expect_err("duplicate logical permutation must fail");

        assert!(matches!(
            error,
            RoutingAdapterError::DuplicatePermutationLogical {
                operation_index: 6,
                ..
            }
        ));
    }

    #[test]
    fn permutation_duplicate_physical_qubits_are_rejected() {
        let operation = RoutingOperation::Move(
            RoutingMove::Permutation {
                mapping: vec![
                    (logical(0), physical(3)),
                    (logical(1), physical(3)),
                ],
            },
        );

        let error = adapt_operation(7, &operation)
            .expect_err("duplicate physical permutation must fail");

        assert!(matches!(
            error,
            RoutingAdapterError::DuplicatePermutationPhysical {
                operation_index: 7,
                ..
            }
        ));
    }

    #[test]
    fn iterator_preserves_source_order() {
        let operations = vec![
            RoutingOperation::Gate {
                gate: GateIdentity::H,
                operands: vec![physical(0)],
                logical_operands: vec![logical(0)],
            },
            RoutingOperation::Move(
                RoutingMove::Swap {
                    a: physical(0),
                    b: physical(1),
                },
            ),
            RoutingOperation::Gate {
                gate: GateIdentity::Cx,
                operands: vec![physical(1), physical(2)],
                logical_operands: vec![logical(0), logical(1)],
            },
        ];

        let adapted = adapt_operations(&operations)
            .expect("valid operation stream");

        assert_eq!(adapted.len(), 3);
        assert_eq!(adapted[0].source_index(), 0);
        assert_eq!(adapted[1].source_index(), 1);
        assert_eq!(adapted[2].source_index(), 2);
    }

    #[test]
    fn validation_does_not_require_owned_output() {
        let operations = vec![
            RoutingOperation::Gate {
                gate: GateIdentity::X,
                operands: vec![physical(0)],
                logical_operands: vec![logical(0)],
            },
        ];

        validate(&operations)
            .expect("valid route should validate");
    }

    #[test]
    fn empty_stream_is_valid() {
        let operations: Vec<RoutingOperation> = Vec::new();

        let result = adapt_operations(&operations)
            .expect("empty routing result is valid");

        assert!(result.is_empty());
    }

    #[test]
    fn custom_gate_identity_is_preserved() {
        let gate = GateIdentity::Custom(
            "future_native_operation".to_owned(),
        );

        let operation = RoutingOperation::Gate {
            gate: gate.clone(),
            operands: vec![physical(12)],
            logical_operands: vec![logical(4)],
        };

        let adapted = adapt_operation(0, &operation)
            .expect("custom gates must be supported");

        assert_eq!(adapted.gate(), Some(&gate));
    }
}