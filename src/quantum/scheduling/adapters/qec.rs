//! Zamani Quantum Scheduling — QEC Adapter
//!
//! # Purpose
//!
//! This module is the scheduler-owned integration boundary between:
//!
//! ```text
//! QEC scheduling contracts
//!        │
//!        ▼
//! adapters::qec
//!        │
//!        ▼
//! scheduling::ir
//!        │
//!        ├── dependency analysis
//!        ├── resource analysis
//!        ├── timing analysis
//!        ├── constraints
//!        └── planners
//! ```
//!
//! The adapter translates QEC scheduling requirements into the generic
//! scheduler representation without implementing a QEC algorithm or a
//! scheduling algorithm.
//!
//! # Architectural responsibility
//!
//! This module is responsible for:
//!
//! - translating `QecOperation` into `SchedulingOperation`;
//! - preserving QEC operation identity;
//! - preserving QEC round identity;
//! - preserving QEC phase;
//! - preserving QEC semantic operation kind;
//! - translating QEC dependencies into scheduler-facing dependencies;
//! - preserving canonical logical qubit identity;
//! - validating that physical QEC references do not cross an incompatible
//!   logical scheduling boundary;
//! - preventing scheduler-operation-ID collisions;
//! - providing lazy operation adaptation;
//! - providing deterministic bulk adaptation;
//! - providing a stable integration contract for future QEC implementations.
//!
//! This module does NOT:
//!
//! - implement QEC;
//! - decode syndromes;
//! - route qubits;
//! - allocate physical qubits;
//! - discover hardware;
//! - calculate hardware timing;
//! - generate pulses;
//! - execute operations;
//! - choose a scheduling policy;
//! - perform ASAP scheduling;
//! - perform ALAP scheduling;
//! - perform resource-constrained scheduling;
//! - optimize a schedule;
//! - modify canonical quantum semantics.
//!
//! Those responsibilities belong to other subsystems.
//!
//! # Critical identity rule
//!
//! QEC operation identity and scheduler operation identity are different
//! namespaces.
//!
//! ```text
//! QecOperationId
//!       ≠
//! scheduling::ir::OperationId
//! ```
//!
//! Therefore this adapter NEVER performs:
//!
//! ```text
//! OperationId::new(qec_operation.id().index())
//! ```
//!
//! because doing so can collide with ordinary program operations.
//!
//! Instead, the caller supplies the scheduler `OperationId` at the integration
//! boundary. This makes the mapping explicit and allows a compiler pipeline to
//! allocate IDs from one collision-free namespace.
//!
//! # Canonical qubit identity
//!
//! Logical qubits are always represented by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! Physical qubits are always represented by:
//!
//! ```text
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module does not define another qubit identity.
//!
//! The current scheduling IR's `QubitOperand` represents canonical logical
//! `QubitId`. Consequently, a QEC operation containing a physical
//! `PhysicalQubitId` cannot safely be projected into `SchedulingOperation` by
//! this adapter.
//!
//! Such a request is rejected explicitly rather than silently changing its
//! identity.
//!
//! Physical QEC scheduling therefore belongs behind the routing/physical
//! specialization boundary, or requires a future scheduler IR representation
//! explicitly capable of carrying physical operands.
//!
//! # Write once, scale everywhere
//!
//! This module contains no:
//!
//! - maximum qubit count;
//! - maximum operation count;
//! - maximum QEC round count;
//! - fixed code distance;
//! - fixed stabilizer weight;
//! - fixed ancilla count;
//! - fixed topology;
//! - fixed number of resources;
//! - fixed gate arity;
//! - vendor-specific assumptions.
//!
//! Memory consumption is proportional to the data explicitly materialized by
//! the caller. The lazy iterator does not construct an additional
//! operation-sized collection.
//!
//! "Infinity" therefore means that the adapter introduces no artificial
//! architectural ceiling. A concrete compilation remains bounded by available
//! memory, address space, target resources, and explicitly configured limits.
//!
//! # Dependency handling
//!
//! QEC dependencies are semantic requirements. The adapter preserves their
//! identity and kind but does not decide when they execute.
//!
//! ```text
//! QEC dependency
//!       │
//!       ▼
//! adapter
//!       │
//!       ▼
//! scheduler dependency
//!       │
//!       ▼
//! dependency graph
//!       │
//!       ▼
//! planner
//! ```
//!
//! # Resource handling
//!
//! QEC-specific resource semantics are deliberately not encoded as fixed
//! scheduler resources here.
//!
//! Examples include:
//!
//! - ancillas;
//! - measurement resources;
//! - syndrome resources;
//! - control resources;
//! - classical feedback resources;
//! - communication resources.
//!
//! The QEC layer describes requirements. The generic scheduler's resource
//! subsystem determines availability and capacity.
//!
//! # Timing handling
//!
//! This adapter does not assign operation duration or start time.
//!
//! Timing comes from:
//!
//! ```text
//! hardware / target capabilities
//!        +
//! QEC timing requirements
//!        +
//! scheduler timing model
//! ```
//!
//! # Determinism
//!
//! Given:
//!
//! - the same QEC operation sequence;
//! - the same scheduler-operation-ID allocation;
//! - the same adapter;
//!
//! adaptation produces the same result.
//!
//! No randomness or global mutable state is used.
//!
//! # Thread safety
//!
//! `QecAdapter` is stateless and therefore may safely be shared by independent
//! callers. It contains no mutable global state.
//!
//! # Rust contract
//!
//! Designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! The safety requirement is compiler-enforced.
//!
//! # Integration contract
//!
//! ```text
//! QEC implementation
//!        │
//!        ▼
//! QecSchedulingRequest
//!        │
//!        ├── QecOperation
//!        ├── QecDependency
//!        ├── QecRound
//!        └── QEC requirements
//!        │
//!        ▼
//! adapters::qec
//!        │
//!        ├── AdaptedQecOperation
//!        └── AdaptedQecDependency
//!        │
//!        ▼
//! scheduling::ir
//!        │
//!        ▼
//! dependency/resource/timing analysis
//!        │
//!        ▼
//! planner
//!        │
//!        ▼
//! verification
//! ```
//!
//! No downstream implementation needs to modify this file merely because a
//! new QEC code, QEC protocol, scheduler algorithm, hardware target, or
//! resource type is added.
//!
//! # Important boundary
//!
//! QEC planning answers:
//!
//! > What fault-tolerance operations and dependencies are required?
//!
//! Routing answers:
//!
//! > Where can those operations execute?
//!
//! Scheduling answers:
//!
//! > When can they execute?
//!
//! Execution answers:
//!
//! > How are they submitted to the target?
//!
//! This adapter exists only at the first scheduler integration boundary.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;
use std::sync::Arc;

use crate::quantum::ir::qubit::QubitId;
use crate::quantum::scheduling::ir::{
    OperationClass,
    OperationId,
    QubitOperand,
    SchedulingOperation,
};
use crate::quantum::scheduling::qec::{
    QecDependency,
    QecDependencyKind,
    QecOperation,
    QecOperationId,
    QecOperationKind,
    QecPhase,
    QecQubit,
};

/// Errors produced when translating QEC scheduling data into generic
/// scheduling data.
///
/// The adapter deliberately fails closed: information that cannot be
/// represented without semantic loss is rejected rather than approximated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QecAdapterError {
    /// A QEC operation has no logical qubit representation accepted by the
    /// current scheduler IR.
    PhysicalQubitOperand {
        /// QEC operation containing the physical operand.
        operation: QecOperationId,

        /// Physical qubit identity that cannot be represented as a logical
        /// scheduler operand.
        qubit: String,
    },

    /// Two or more QEC operations were assigned the same scheduler operation
    /// identity.
    DuplicateSchedulerOperationId {
        /// The conflicting scheduler operation identity.
        operation_id: OperationId,

        /// First QEC operation using the identity.
        first: QecOperationId,

        /// Second QEC operation using the identity.
        second: QecOperationId,
    },

    /// A dependency references an operation that is absent from the supplied
    /// QEC operation set.
    MissingDependencyOperation {
        /// Referenced operation.
        operation: QecOperationId,

        /// Dependency predecessor.
        predecessor: QecOperationId,

        /// Dependency successor.
        successor: QecOperationId,
    },

    /// A dependency references itself.
    SelfDependency {
        /// Self-referencing operation.
        operation: QecOperationId,
    },

    /// A QEC operation contains the same logical qubit more than once.
    DuplicateQubitOperand {
        /// Operation containing the duplicate.
        operation: QecOperationId,

        /// Duplicated logical qubit.
        qubit: QubitId,
    },

    /// The caller supplied an empty scheduler-ID allocation for an operation
    /// set that was expected to contain entries.
    MissingSchedulerOperationId {
        /// QEC operation requiring an ID.
        operation: QecOperationId,
    },
}

impl fmt::Display for QecAdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PhysicalQubitOperand { operation, qubit } => write!(
                formatter,
                "QEC operation {operation} contains physical qubit {qubit}, \
                 but scheduling::ir::QubitOperand accepts canonical logical \
                 QubitId values only; route/physicalize at the appropriate \
                 boundary instead of silently changing identity"
            ),

            Self::DuplicateSchedulerOperationId {
                operation_id,
                first,
                second,
            } => write!(
                formatter,
                "scheduler operation ID {operation_id:?} is assigned to \
                 multiple QEC operations: {first} and {second}"
            ),

            Self::MissingDependencyOperation {
                operation,
                predecessor,
                successor,
            } => write!(
                formatter,
                "QEC dependency {predecessor} -> {successor} references \
                 missing operation {operation}"
            ),

            Self::SelfDependency { operation } => {
                write!(formatter, "QEC operation {operation} depends on itself")
            }

            Self::DuplicateQubitOperand { operation, qubit } => write!(
                formatter,
                "QEC operation {operation} contains logical qubit {qubit:?} \
                 more than once"
            ),

            Self::MissingSchedulerOperationId { operation } => write!(
                formatter,
                "QEC operation {operation} has no scheduler operation ID \
                 allocation"
            ),
        }
    }
}

impl std::error::Error for QecAdapterError {}

/// Metadata retained alongside an adapted QEC operation.
///
/// `SchedulingOperation` deliberately represents generic scheduler concerns.
/// This structure retains QEC-specific identity that must survive the adapter
/// boundary without polluting generic scheduling semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QecOperationMetadata {
    /// Original QEC operation identity.
    qec_operation_id: QecOperationId,

    /// QEC round containing the operation.
    round: crate::quantum::scheduling::qec::QecRoundId,

    /// QEC semantic phase.
    phase: QecPhase,

    /// QEC semantic operation kind.
    kind: QecOperationKind,

    /// Canonical logical qubits used by the QEC operation.
    qubits: Arc<[QubitId]>,
}

impl QecOperationMetadata {
    /// Creates QEC metadata.
    #[must_use]
    fn new(
        operation: &QecOperation,
        qubits: Arc<[QubitId]>,
    ) -> Self {
        Self {
            qec_operation_id: operation.id(),
            round: operation.round(),
            phase: operation.phase(),
            kind: operation.kind(),
            qubits,
        }
    }

    /// Returns the original QEC operation identity.
    #[must_use]
    pub const fn qec_operation_id(&self) -> QecOperationId {
        self.qec_operation_id
    }

    /// Returns the QEC round.
    #[must_use]
    pub const fn round(
        &self,
    ) -> crate::quantum::scheduling::qec::QecRoundId {
        self.round
    }

    /// Returns the QEC phase.
    #[must_use]
    pub const fn phase(&self) -> QecPhase {
        self.phase
    }

    /// Returns the QEC semantic operation kind.
    #[must_use]
    pub const fn kind(&self) -> QecOperationKind {
        self.kind
    }

    /// Returns the canonical logical qubits.
    #[must_use]
    pub fn qubits(&self) -> &[QubitId] {
        &self.qubits
    }
}

/// One successfully adapted QEC operation.
///
/// The generic scheduler consumes `scheduling_operation()`.
///
/// QEC-specific consumers can retain the metadata without requiring the
/// generic scheduler IR to understand QEC-specific concepts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaptedQecOperation {
    scheduling_operation: SchedulingOperation,
    metadata: QecOperationMetadata,
}

impl AdaptedQecOperation {
    /// Creates an adapted operation.
    #[must_use]
    fn new(
        scheduling_operation: SchedulingOperation,
        metadata: QecOperationMetadata,
    ) -> Self {
        Self {
            scheduling_operation,
            metadata,
        }
    }

    /// Returns the generic scheduler operation.
    #[must_use]
    pub fn scheduling_operation(&self) -> &SchedulingOperation {
        &self.scheduling_operation
    }

    /// Consumes this wrapper and returns the generic scheduler operation.
    #[must_use]
    pub fn into_scheduling_operation(self) -> SchedulingOperation {
        self.scheduling_operation
    }

    /// Returns QEC-specific metadata.
    #[must_use]
    pub fn metadata(&self) -> &QecOperationMetadata {
        &self.metadata
    }

    /// Returns the QEC operation identity.
    #[must_use]
    pub const fn qec_operation_id(&self) -> QecOperationId {
        self.metadata.qec_operation_id()
    }

    /// Returns the scheduler operation identity.
    #[must_use]
    pub fn scheduler_operation_id(&self) -> OperationId {
        self.scheduling_operation.operation_id()
    }
}

/// Scheduler-facing representation of a QEC dependency.
///
/// QEC IDs and scheduler IDs remain separate namespaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AdaptedQecDependency {
    qec_predecessor: QecOperationId,
    qec_successor: QecOperationId,
    predecessor: OperationId,
    successor: OperationId,
    kind: QecDependencyKind,
}

impl AdaptedQecDependency {
    /// Creates an adapted dependency.
    #[must_use]
    fn new(
        source: &QecDependency,
        predecessor: OperationId,
        successor: OperationId,
    ) -> Self {
        Self {
            qec_predecessor: source.predecessor(),
            qec_successor: source.successor(),
            predecessor,
            successor,
            kind: source.kind(),
        }
    }

    /// Returns the source QEC predecessor.
    #[must_use]
    pub const fn qec_predecessor(&self) -> QecOperationId {
        self.qec_predecessor
    }

    /// Returns the source QEC successor.
    #[must_use]
    pub const fn qec_successor(&self) -> QecOperationId {
        self.qec_successor
    }

    /// Returns the scheduler predecessor.
    #[must_use]
    pub const fn predecessor(&self) -> OperationId {
        self.predecessor
    }

    /// Returns the scheduler successor.
    #[must_use]
    pub const fn successor(&self) -> OperationId {
        self.successor
    }

    /// Returns the QEC dependency semantic kind.
    #[must_use]
    pub const fn kind(&self) -> QecDependencyKind {
        self.kind
    }
}

/// Stateless QEC-to-scheduling adapter.
///
/// The adapter owns no circuit, request, target, resource model, hardware
/// state, or scheduler state.
///
/// Consequently it is cheap to construct and safe to share across independent
/// compiler tasks.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct QecAdapter;

impl QecAdapter {
    /// Creates a QEC scheduling adapter.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Converts a QEC semantic operation kind into the generic scheduler
    /// operation class.
    ///
    /// The mapping is intentionally semantic rather than hardware-specific.
    ///
    /// Several QEC concepts legitimately map to `Quantum` because they are
    /// quantum operations whose exact hardware realization belongs to later
    /// compilation stages.
    #[must_use]
    pub const fn operation_class(
        kind: QecOperationKind,
    ) -> OperationClass {
        match kind {
            QecOperationKind::Preparation => OperationClass::Quantum,

            QecOperationKind::Reset => OperationClass::Reset,

            QecOperationKind::SyndromeInteraction => {
                OperationClass::Quantum
            }

            QecOperationKind::StabilizerInteraction => {
                OperationClass::Quantum
            }

            QecOperationKind::Measurement => OperationClass::Measurement,

            QecOperationKind::SyndromeTransfer => {
                OperationClass::Communication
            }

            QecOperationKind::ClassicalSynchronization => {
                OperationClass::ClassicalControl
            }

            QecOperationKind::Recovery => OperationClass::Quantum,

            QecOperationKind::Synchronization => {
                OperationClass::Quantum
            }

            QecOperationKind::Custom => OperationClass::Quantum,
        }
    }

    /// Converts one QEC operation into a generic scheduler operation.
    ///
    /// The caller MUST supply the scheduler operation ID.
    ///
    /// This is deliberate: QEC operation IDs and scheduler operation IDs are
    /// different namespaces and must never be conflated.
    pub fn adapt_operation(
        &self,
        operation_id: OperationId,
        operation: &QecOperation,
    ) -> Result<AdaptedQecOperation, QecAdapterError> {
        let qubits = self.adapt_qubits(operation)?;

        let operands = qubits
            .iter()
            .copied()
            .map(QubitOperand::new)
            .collect::<Vec<_>>()
            .into();

        let scheduling_operation =
            SchedulingOperation::from_canonical_ir(
                operation_id,
                Self::operation_class(operation.kind()),
                operands,
            );

        let metadata =
            QecOperationMetadata::new(operation, qubits);

        Ok(AdaptedQecOperation::new(
            scheduling_operation,
            metadata,
        ))
    }

    /// Converts the participating QEC qubits into canonical logical qubits.
    ///
    /// Physical qubits are rejected because the current scheduler IR's
    /// `QubitOperand` is explicitly based on canonical logical `QubitId`.
    ///
    /// This prevents a catastrophic class of bugs in which a physical identity
    /// is accidentally treated as a logical identity.
    fn adapt_qubits(
        &self,
        operation: &QecOperation,
    ) -> Result<Arc<[QubitId]>, QecAdapterError> {
        let mut result = Vec::with_capacity(operation.qubits().len());

        for qubit in operation.qubits() {
            match qubit {
                QecQubit::Logical(id) => {
                    if result.contains(id) {
                        return Err(
                            QecAdapterError::DuplicateQubitOperand {
                                operation: operation.id(),
                                qubit: *id,
                            },
                        );
                    }

                    result.push(*id);
                }

                QecQubit::Physical(id) => {
                    return Err(
                        QecAdapterError::PhysicalQubitOperand {
                            operation: operation.id(),
                            qubit: id.to_string(),
                        },
                    );
                }
            }
        }

        Ok(result.into())
    }

    /// Adapts an iterator of QEC operations lazily.
    ///
    /// The caller supplies the scheduler operation ID for each QEC operation.
    ///
    /// The callback receives the QEC operation and returns its scheduler
    /// operation identity.
    ///
    /// No circuit-sized intermediate collection is required.
    pub fn iter<'a, F>(
        &'a self,
        operations: &'a [QecOperation],
        mut scheduler_id: F,
    ) -> QecOperationIter<'a, F>
    where
        F: FnMut(&QecOperation) -> Option<OperationId>,
    {
        QecOperationIter {
            adapter: *self,
            operations: operations.iter(),
            scheduler_id,
            finished: false,
        }
    }

    /// Adapts all supplied QEC operations into an owned vector.
    ///
    /// This method intentionally materializes the result because the caller
    /// explicitly requested ownership.
    ///
    /// For very large workloads, prefer [`Self::iter`].
    pub fn adapt_operations<F>(
        &self,
        operations: &[QecOperation],
        mut scheduler_id: F,
    ) -> Result<Vec<AdaptedQecOperation>, QecAdapterError>
    where
        F: FnMut(&QecOperation) -> Option<OperationId>,
    {
        let mut result = Vec::with_capacity(operations.len());

        for operation in operations {
            let id = scheduler_id(operation).ok_or(
                QecAdapterError::MissingSchedulerOperationId {
                    operation: operation.id(),
                },
            )?;

            result.push(self.adapt_operation(id, operation)?);
        }

        self.validate_unique_scheduler_ids(&result)?;

        Ok(result)
    }

    /// Validates that scheduler operation IDs are unique.
    ///
    /// This uses a sorted vector rather than a hash map so validation remains
    /// deterministic and memory proportional to the supplied operation set.
    pub fn validate_unique_scheduler_ids(
        &self,
        operations: &[AdaptedQecOperation],
    ) -> Result<(), QecAdapterError> {
        if operations.len() < 2 {
            return Ok(());
        }

        let mut ids = operations
            .iter()
            .map(|operation| {
                (
                    operation.scheduler_operation_id(),
                    operation.qec_operation_id(),
                )
            })
            .collect::<Vec<_>>();

        ids.sort_unstable_by_key(|entry| entry.0);

        for pair in ids.windows(2) {
            if pair[0].0 == pair[1].0 {
                return Err(
                    QecAdapterError::DuplicateSchedulerOperationId {
                        operation_id: pair[0].0,
                        first: pair[0].1,
                        second: pair[1].1,
                    },
                );
            }
        }

        Ok(())
    }

    /// Adapts one QEC dependency after the caller has resolved both QEC
    /// operation IDs to their generic scheduler operation IDs.
    ///
    /// Keeping this mapping explicit prevents accidental namespace collision
    /// and allows the compiler pipeline to use one global operation-ID
    /// allocator.
    #[must_use]
    pub fn adapt_dependency(
        &self,
        dependency: &QecDependency,
        predecessor: OperationId,
        successor: OperationId,
    ) -> AdaptedQecDependency {
        AdaptedQecDependency::new(
            dependency,
            predecessor,
            successor,
        )
    }

    /// Validates a QEC dependency against a known set of QEC operation IDs.
    ///
    /// This performs structural validation only. It does not determine
    /// schedulability.
    pub fn validate_dependency(
        &self,
        dependency: &QecDependency,
        known_operations: &[QecOperationId],
    ) -> Result<(), QecAdapterError> {
        let predecessor = dependency.predecessor();
        let successor = dependency.successor();

        if predecessor == successor {
            return Err(QecAdapterError::SelfDependency {
                operation: predecessor,
            });
        }

        let predecessor_exists =
            known_operations.binary_search(&predecessor).is_ok();

        if !predecessor_exists {
            return Err(
                QecAdapterError::MissingDependencyOperation {
                    operation: predecessor,
                    predecessor,
                    successor,
                },
            );
        }

        let successor_exists =
            known_operations.binary_search(&successor).is_ok();

        if !successor_exists {
            return Err(
                QecAdapterError::MissingDependencyOperation {
                    operation: successor,
                    predecessor,
                    successor,
                },
            );
        }

        Ok(())
    }

    /// Validates an entire dependency collection.
    ///
    /// `known_operations` must be sorted by `QecOperationId`.
    pub fn validate_dependencies(
        &self,
        dependencies: &[QecDependency],
        known_operations: &[QecOperationId],
    ) -> Result<(), QecAdapterError> {
        for dependency in dependencies {
            self.validate_dependency(
                dependency,
                known_operations,
            )?;
        }

        Ok(())
    }

    /// Returns the number of QEC operations without adapting them.
    #[must_use]
    pub const fn operation_count(
        operations: &[QecOperation],
    ) -> usize {
        operations.len()
    }

    /// Returns whether there are no QEC operations.
    #[must_use]
    pub const fn is_empty(
        operations: &[QecOperation],
    ) -> bool {
        operations.is_empty()
    }
}

/// Lazy iterator over QEC operations.
///
/// The iterator owns no operation collection and therefore does not introduce
/// another circuit-sized allocation.
#[derive(Debug)]
pub struct QecOperationIter<'a, F>
where
    F: FnMut(&QecOperation) -> Option<OperationId>,
{
    adapter: QecAdapter,
    operations: std::slice::Iter<'a, QecOperation>,
    scheduler_id: F,
    finished: bool,
}

impl<'a, F> Iterator for QecOperationIter<'a, F>
where
    F: FnMut(&QecOperation) -> Option<OperationId>,
{
    type Item = Result<AdaptedQecOperation, QecAdapterError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let operation = match self.operations.next() {
            Some(operation) => operation,
            None => {
                self.finished = true;
                return None;
            }
        };

        let scheduler_id = match (self.scheduler_id)(operation) {
            Some(id) => id,
            None => {
                return Some(Err(
                    QecAdapterError::MissingSchedulerOperationId {
                        operation: operation.id(),
                    },
                ));
            }
        };

        Some(
            self.adapter
                .adapt_operation(scheduler_id, operation),
        )
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.operations.len();

        (remaining, Some(remaining))
    }
}

impl<F> ExactSizeIterator for QecOperationIter<'_, F>
where
    F: FnMut(&QecOperation) -> Option<OperationId>,
{
}

impl<F> std::iter::FusedIterator for QecOperationIter<'_, F>
where
    F: FnMut(&QecOperation) -> Option<OperationId>,
{
}

/// Adapt one QEC operation using the default stateless adapter.
///
/// The scheduler operation ID remains explicitly supplied.
pub fn adapt_operation(
    operation_id: OperationId,
    operation: &QecOperation,
) -> Result<AdaptedQecOperation, QecAdapterError> {
    QecAdapter::new().adapt_operation(
        operation_id,
        operation,
    )
}

/// Convert a QEC operation kind into the generic scheduler operation class.
#[must_use]
pub const fn operation_class(
    kind: QecOperationKind,
) -> OperationClass {
    QecAdapter::operation_class(kind)
}

/// Validate a QEC dependency against a sorted operation-ID collection.
pub fn validate_dependency(
    dependency: &QecDependency,
    known_operations: &[QecOperationId],
) -> Result<(), QecAdapterError> {
    QecAdapter::new().validate_dependency(
        dependency,
        known_operations,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn logical_qubit(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn qec_operation(
        id: usize,
        kind: QecOperationKind,
        qubits: Vec<QecQubit>,
    ) -> QecOperation {
        QecOperation::new(
            QecOperationId::new(id),
            crate::quantum::scheduling::qec::QecRoundId::new(0),
            QecPhase::SyndromeExtraction,
            kind,
            qubits,
        )
    }

    #[test]
    fn adapter_is_stateless() {
        let first = QecAdapter::new();
        let second = QecAdapter::new();

        assert_eq!(first, second);
    }

    #[test]
    fn adapter_is_copy() {
        let adapter = QecAdapter::new();
        let copied = adapter;

        assert_eq!(adapter, copied);
    }

    #[test]
    fn logical_qubit_identity_is_preserved() {
        let operation = qec_operation(
            0,
            QecOperationKind::SyndromeInteraction,
            vec![QecQubit::logical(logical_qubit(17))],
        );

        let adapted = adapt_operation(
            OperationId::new(9000),
            &operation,
        )
        .expect("logical QEC operation must adapt");

        assert_eq!(
            adapted.metadata().qubits(),
            &[logical_qubit(17)]
        );
    }

    #[test]
    fn scheduler_operation_id_is_not_derived_from_qec_id() {
        let operation = qec_operation(
            7,
            QecOperationKind::SyndromeInteraction,
            vec![QecQubit::logical(logical_qubit(0))],
        );

        let scheduler_id = OperationId::new(123456);

        let adapted = adapt_operation(
            scheduler_id,
            &operation,
        )
        .expect("operation should adapt");

        assert_eq!(
            adapted.scheduler_operation_id(),
            scheduler_id
        );

        assert_eq!(
            adapted.qec_operation_id(),
            QecOperationId::new(7)
        );
    }

    #[test]
    fn physical_qubit_is_rejected_without_semantic_loss() {
        let physical =
            crate::quantum::ir::qubit::PhysicalQubitId::new(4);

        let operation = qec_operation(
            0,
            QecOperationKind::SyndromeInteraction,
            vec![QecQubit::physical(physical)],
        );

        let result = adapt_operation(
            OperationId::new(0),
            &operation,
        );

        assert!(matches!(
            result,
            Err(QecAdapterError::PhysicalQubitOperand { .. })
        ));
    }

    #[test]
    fn duplicate_logical_qubits_are_rejected() {
        let qubit = logical_qubit(3);

        let operation = qec_operation(
            0,
            QecOperationKind::SyndromeInteraction,
            vec![
                QecQubit::logical(qubit),
                QecQubit::logical(qubit),
            ],
        );

        let result = adapt_operation(
            OperationId::new(0),
            &operation,
        );

        assert!(matches!(
            result,
            Err(QecAdapterError::DuplicateQubitOperand { .. })
        ));
    }

    #[test]
    fn operation_classes_are_semantic() {
        assert_eq!(
            operation_class(QecOperationKind::Measurement),
            OperationClass::Measurement
        );

        assert_eq!(
            operation_class(QecOperationKind::Reset),
            OperationClass::Reset
        );

        assert_eq!(
            operation_class(
                QecOperationKind::ClassicalSynchronization
            ),
            OperationClass::ClassicalControl
        );

        assert_eq!(
            operation_class(QecOperationKind::SyndromeTransfer),
            OperationClass::Communication
        );

        assert_eq!(
            operation_class(
                QecOperationKind::SyndromeInteraction
            ),
            OperationClass::Quantum
        );
    }

    #[test]
    fn unique_scheduler_ids_are_validated() {
        let first = qec_operation(
            0,
            QecOperationKind::Preparation,
            vec![QecQubit::logical(logical_qubit(0))],
        );

        let second = qec_operation(
            1,
            QecOperationKind::Measurement,
            vec![QecQubit::logical(logical_qubit(1))],
        );

        let operations = QecAdapter::new()
            .adapt_operations(
                &[first, second],
                |operation| {
                    Some(OperationId::new(
                        operation.id().index() + 100,
                    ))
                },
            )
            .expect("IDs should be unique");

        assert_eq!(operations.len(), 2);
        assert_ne!(
            operations[0].scheduler_operation_id(),
            operations[1].scheduler_operation_id()
        );
    }

    #[test]
    fn duplicate_scheduler_ids_are_rejected() {
        let first = qec_operation(
            0,
            QecOperationKind::Preparation,
            vec![QecQubit::logical(logical_qubit(0))],
        );

        let second = qec_operation(
            1,
            QecOperationKind::Measurement,
            vec![QecQubit::logical(logical_qubit(1))],
        );

        let result = QecAdapter::new().adapt_operations(
            &[first, second],
            |_| Some(OperationId::new(42)),
        );

        assert!(matches!(
            result,
            Err(
                QecAdapterError::DuplicateSchedulerOperationId {
                    ..
                }
            )
        ));
    }

    #[test]
    fn dependency_self_reference_is_rejected() {
        let dependency = QecDependency::new(
            QecOperationId::new(4),
            QecOperationId::new(4),
            QecDependencyKind::Quantum,
        );

        let result = validate_dependency(
            &dependency,
            &[QecOperationId::new(4)],
        );

        assert!(matches!(
            result,
            Err(QecAdapterError::SelfDependency { .. })
        ));
    }

    #[test]
    fn missing_dependency_predecessor_is_rejected() {
        let dependency = QecDependency::new(
            QecOperationId::new(10),
            QecOperationId::new(11),
            QecDependencyKind::Quantum,
        );

        let result = validate_dependency(
            &dependency,
            &[QecOperationId::new(11)],
        );

        assert!(matches!(
            result,
            Err(
                QecAdapterError::MissingDependencyOperation {
                    ..
                }
            )
        ));
    }

    #[test]
    fn empty_operation_collection_is_supported() {
        let operations: &[QecOperation] = &[];

        assert!(QecAdapter::is_empty(operations));
        assert_eq!(
            QecAdapter::operation_count(operations),
            0
        );
    }

    #[test]
    fn lazy_iterator_does_not_require_materialization() {
        let operations = [
            qec_operation(
                0,
                QecOperationKind::Preparation,
                vec![QecQubit::logical(logical_qubit(0))],
            ),
            qec_operation(
                1,
                QecOperationKind::Measurement,
                vec![QecQubit::logical(logical_qubit(1))],
            ),
        ];

        let mut iterator = QecAdapter::new().iter(
            &operations,
            |operation| {
                Some(OperationId::new(
                    operation.id().index() + 100,
                ))
            },
        );

        assert_eq!(iterator.size_hint(), (2, Some(2)));

        let first = iterator
            .next()
            .expect("first item exists")
            .expect("first adaptation succeeds");

        assert_eq!(
            first.qec_operation_id(),
            QecOperationId::new(0)
        );

        let second = iterator
            .next()
            .expect("second item exists")
            .expect("second adaptation succeeds");

        assert_eq!(
            second.qec_operation_id(),
            QecOperationId::new(1)
        );

        assert!(iterator.next().is_none());
    }
}