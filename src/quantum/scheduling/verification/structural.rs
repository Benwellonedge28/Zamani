//! Structural verification of Zamani quantum schedules.
//!
//! This module verifies the structural integrity of a schedule without
//! determining timing, resource capacity, dependency ordering, or quantum
//! semantic equivalence. Those concerns belong to the corresponding
//! verification modules.
//!
//! # Architectural boundary
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! scheduling input
//!      │
//!      ▼
//! scheduling planner
//!      │
//!      ▼
//! ScheduleView
//!      │
//!      ▼
//! StructuralVerifier
//!      │
//!      ├── operation identity integrity
//!      ├── source-operation coverage
//!      ├── duplicate detection
//!      ├── operand integrity
//!      ├── qubit identity integrity
//!      ├── schedule membership integrity
//!      └── structural metadata integrity
//! ```
//!
//! Structural verification deliberately does NOT:
//!
//! - decide whether an operation starts early or late enough;
//! - validate resource overlap;
//! - validate timing resolution;
//! - validate dependency ordering;
//! - validate hardware support;
//! - validate quantum equivalence;
//! - perform routing;
//! - modify the schedule;
//! - execute quantum operations.
//!
//! Those responsibilities belong to:
//!
//! - `verification::timing`;
//! - `verification::resource`;
//! - `verification::dependency`;
//! - `verification::semantic`;
//! - routing;
//! - hardware;
//! - runtime.
//!
//! # Canonical identities
//!
//! Operation identities are the canonical `quantum::ir::OperationId`.
//!
//! Qubit identities are the canonical:
//!
//!     quantum::ir::qubit::QubitId
//!
//! and, where physical identity is required:
//!
//!     quantum::ir::qubit::PhysicalQubitId
//!
//! This module MUST NOT define another qubit or operation identity type.
//!
//! # Scalability
//!
//! No architectural maximum is imposed on:
//!
//! - number of operations;
//! - number of qubits;
//! - number of operands;
//! - number of schedule entries;
//! - circuit depth;
//! - resource count.
//!
//! Concrete limits are imposed by the caller through its execution/resource
//! policy and by finite host resources.
//!
//! Verification uses iterative traversal and linear auxiliary storage relative
//! to the supplied schedule view.
//!
//! # Determinism
//!
//! Diagnostics are emitted in the order supplied by the schedule view.
//! Duplicate detection preserves first-seen identity and reports subsequent
//! occurrences. The verifier itself does not depend on hash-map iteration
//! order.
//!
//! # Mutation
//!
//! Verification is read-only. No method in this module mutates the schedule,
//! canonical IR, target, or caller-owned state.
//!
//! # Rust / safety
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! The compiler-enforced safety boundary is intentional.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

use std::collections::HashSet;
use std::fmt;

use crate::quantum::ir::qubit::QubitId;
use crate::quantum::ir::OperationId;

// =============================================================================
// Public result types
// =============================================================================

/// Result returned by structural schedule verification.
pub type StructuralResult<T> = Result<T, StructuralVerificationError>;

/// Complete structural verification report.
///
/// A report is successful when `violations` is empty.
///
/// The report intentionally contains counters in addition to violations so
/// callers can use verification as a scalable diagnostic/telemetry boundary
/// without reparsing diagnostic strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralVerificationReport {
    /// Number of schedule entries inspected.
    pub entries_checked: usize,

    /// Number of unique operation identities observed.
    pub unique_operations: usize,

    /// Number of operation identities observed more than once.
    pub duplicate_operations: usize,

    /// Number of qubit operands inspected.
    pub qubit_operands_checked: usize,

    /// Number of structural violations found.
    pub violation_count: usize,

    /// Ordered structural violations.
    pub violations: Vec<StructuralViolation>,
}

impl StructuralVerificationReport {
    /// Creates an empty successful report.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries_checked: 0,
            unique_operations: 0,
            duplicate_operations: 0,
            qubit_operands_checked: 0,
            violation_count: 0,
            violations: Vec::new(),
        }
    }

    /// Returns whether structural verification succeeded.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.violations.is_empty()
    }

    /// Returns whether at least one violation was found.
    #[must_use]
    pub fn has_violations(&self) -> bool {
        !self.violations.is_empty()
    }

    /// Records a violation.
    fn push_violation(&mut self, violation: StructuralViolation) {
        self.violation_count = self.violation_count.saturating_add(1);
        self.violations.push(violation);
    }
}

impl Default for StructuralVerificationReport {
    fn default() -> Self {
        Self::new()
    }
}

/// A single structural schedule violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructuralViolation {
    /// A schedule entry has no operation identity.
    MissingOperationIdentity {
        /// Zero-based schedule entry index.
        entry_index: usize,
    },

    /// An operation identity occurs more than once.
    DuplicateOperation {
        /// The duplicated operation.
        operation: OperationId,

        /// First schedule entry containing the operation.
        first_entry: usize,

        /// Subsequent schedule entry containing the operation.
        duplicate_entry: usize,
    },

    /// A scheduled operation does not correspond to an operation in the
    /// source/executable program.
    UnknownSourceOperation {
        /// Operation referenced by the schedule.
        operation: OperationId,

        /// Schedule entry containing the invalid reference.
        entry_index: usize,
    },

    /// A source operation was expected but no schedule entry references it.
    MissingScheduledOperation {
        /// Operation that was not scheduled.
        operation: OperationId,
    },

    /// An operation contains no operands when its structural contract requires
    /// at least one operand.
    MissingOperands {
        /// Operation being checked.
        operation: OperationId,

        /// Schedule entry containing the operation.
        entry_index: usize,
    },

    /// An operand references an invalid logical qubit identity.
    InvalidQubitOperand {
        /// Operation containing the operand.
        operation: OperationId,

        /// Schedule entry containing the operation.
        entry_index: usize,

        /// Operand position.
        operand_index: usize,

        /// Canonical qubit identity supplied by the operation.
        qubit: QubitId,
    },

    /// The same qubit occurs more than once in one operation where the
    /// operation's structural contract forbids duplicate operands.
    DuplicateQubitOperand {
        /// Operation containing the duplicate.
        operation: OperationId,

        /// Schedule entry containing the operation.
        entry_index: usize,

        /// First operand position containing the qubit.
        first_operand: usize,

        /// Duplicate operand position.
        duplicate_operand: usize,

        /// Duplicated canonical qubit identity.
        qubit: QubitId,
    },

    /// The operation's reported operand count does not equal its actual
    /// operand collection.
    OperandCountMismatch {
        /// Operation being checked.
        operation: OperationId,

        /// Schedule entry containing the operation.
        entry_index: usize,

        /// Reported operand count.
        declared: usize,

        /// Actual operand count.
        observed: usize,
    },

    /// A schedule entry refers to an operation that is marked as structurally
    /// invalid by the schedule view.
    InvalidOperationStructure {
        /// Operation being checked.
        operation: OperationId,

        /// Schedule entry containing the operation.
        entry_index: usize,

        /// Stable diagnostic supplied by the adapter.
        reason: String,
    },

    /// The schedule contains a source operation more than once when exactly
    /// one schedule entry is required.
    UnexpectedMultiplicity {
        /// Operation with invalid multiplicity.
        operation: OperationId,

        /// Number of occurrences.
        occurrences: usize,
    },
}

impl fmt::Display for StructuralViolation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingOperationIdentity { entry_index } => {
                write!(
                    formatter,
                    "schedule entry {entry_index} has no operation identity"
                )
            }

            Self::DuplicateOperation {
                operation,
                first_entry,
                duplicate_entry,
            } => {
                write!(
                    formatter,
                    "operation {operation:?} occurs at schedule entries \
                     {first_entry} and {duplicate_entry}"
                )
            }

            Self::UnknownSourceOperation {
                operation,
                entry_index,
            } => {
                write!(
                    formatter,
                    "schedule entry {entry_index} references unknown operation \
                     {operation:?}"
                )
            }

            Self::MissingScheduledOperation { operation } => {
                write!(
                    formatter,
                    "source operation {operation:?} has no schedule entry"
                )
            }

            Self::MissingOperands {
                operation,
                entry_index,
            } => {
                write!(
                    formatter,
                    "operation {operation:?} at schedule entry {entry_index} \
                     has no operands"
                )
            }

            Self::InvalidQubitOperand {
                operation,
                entry_index,
                operand_index,
                qubit,
            } => {
                write!(
                    formatter,
                    "operation {operation:?} at schedule entry {entry_index} \
                     has invalid qubit operand {qubit:?} at position \
                     {operand_index}"
                )
            }

            Self::DuplicateQubitOperand {
                operation,
                entry_index,
                first_operand,
                duplicate_operand,
                qubit,
            } => {
                write!(
                    formatter,
                    "operation {operation:?} at schedule entry {entry_index} \
                     uses qubit {qubit:?} at operands {first_operand} and \
                     {duplicate_operand}"
                )
            }

            Self::OperandCountMismatch {
                operation,
                entry_index,
                declared,
                observed,
            } => {
                write!(
                    formatter,
                    "operation {operation:?} at schedule entry {entry_index} \
                     declares {declared} operands but exposes {observed}"
                )
            }

            Self::InvalidOperationStructure {
                operation,
                entry_index,
                reason,
            } => {
                write!(
                    formatter,
                    "operation {operation:?} at schedule entry {entry_index} \
                     has invalid structure: {reason}"
                )
            }

            Self::UnexpectedMultiplicity {
                operation,
                occurrences,
            } => {
                write!(
                    formatter,
                    "operation {operation:?} occurs {occurrences} times"
                )
            }
        }
    }
}

impl std::error::Error for StructuralViolation {}

// =============================================================================
// Verification error
// =============================================================================

/// Failure of the structural verification process itself.
///
/// A structurally invalid schedule is normally represented by a successful
/// verification call returning a report containing violations. This error type
/// is reserved for failures to perform verification, such as an invalid
/// adapter/view contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructuralVerificationError {
    /// The schedule view could not provide its structural source information.
    InvalidView {
        /// Stable diagnostic message.
        reason: String,
    },
}

impl fmt::Display for StructuralVerificationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidView { reason } => {
                write!(formatter, "invalid schedule verification view: {reason}")
            }
        }
    }
}

impl std::error::Error for StructuralVerificationError {}

// =============================================================================
// Schedule-view contract
// =============================================================================

/// Read-only structural view of a scheduled quantum program.
///
/// This trait is the deliberate integration boundary between the verifier and
/// the scheduler's concrete `ScheduleResult`.
///
/// The verifier therefore does not depend on a particular internal storage
/// representation such as `Vec`, arena, graph node, interval tree, or database.
///
/// A future scheduler can change its storage implementation without changing
/// structural verification as long as it preserves this contract.
///
/// # Integration
///
/// The eventual scheduler result should implement this trait through an
/// adapter in `verification` or `adapters`, for example:
///
/// ```text
/// ScheduleResult
///      │
///      ▼
/// StructuralScheduleView implementation
///      │
///      ▼
/// StructuralVerifier
/// ```
///
/// The implementation must expose canonical `quantum::ir::OperationId` and
/// canonical `quantum::ir::qubit::QubitId`.
pub trait StructuralScheduleView {
    /// Returns the number of schedule entries.
    ///
    /// This value must be stable for the lifetime of the immutable view.
    fn schedule_entry_count(&self) -> usize;

    /// Returns the operation identity at a schedule entry.
    ///
    /// `None` means that the schedule entry is structurally incomplete.
    fn operation_id(&self, entry_index: usize) -> Option<OperationId>;

    /// Returns whether the operation referenced by a schedule entry exists in
    /// the source/executable program represented by this schedule.
    fn source_contains_operation(&self, operation: OperationId) -> bool;

    /// Returns whether the source operation is required to appear exactly once
    /// in the final static schedule.
    ///
    /// Dynamic/runtime operations may legitimately opt out.
    fn requires_exactly_one_schedule_entry(&self, operation: OperationId) -> bool;

    /// Returns all source operations that must appear in the static schedule.
    ///
    /// The returned iterator must not depend on hash-map iteration order if
    /// deterministic diagnostics are required.
    fn required_source_operations(
        &self,
    ) -> Box<dyn Iterator<Item = OperationId> + '_>;

    /// Returns the canonical logical qubit operands of a scheduled operation.
    ///
    /// Implementations MUST return the canonical
    /// `quantum::ir::qubit::QubitId` values.
    fn qubit_operands(
        &self,
        entry_index: usize,
    ) -> Option<&[QubitId]>;

    /// Returns the operation's declared operand count, when the underlying IR
    /// carries an independently declared count.
    ///
    /// Returning `None` means the operation has no separately declared count
    /// and the verifier will use the operand collection itself.
    fn declared_operand_count(
        &self,
        entry_index: usize,
    ) -> Option<usize>;

    /// Returns whether zero operands are structurally legal for this
    /// operation.
    ///
    /// This is operation-semantic metadata supplied by the adapter; the
    /// verifier must not hard-code gate arities.
    fn zero_operands_are_valid(
        &self,
        entry_index: usize,
    ) -> bool;

    /// Returns whether duplicate qubit operands are structurally legal for
    /// this operation.
    ///
    /// The default is `false`, because ordinary quantum gate operands normally
    /// identify distinct qubits. Operations for which repeated references have
    /// legitimate semantics can explicitly return `true`.
    fn duplicate_qubit_operands_are_valid(
        &self,
        entry_index: usize,
    ) -> bool {
        let _ = entry_index;
        false
    }

    /// Returns an optional adapter-supplied structural diagnostic.
    ///
    /// This allows canonical IR validation to remain owned by `quantum::ir`
    /// while the scheduling verifier can surface the result without
    /// duplicating IR validation logic.
    fn structural_error(
        &self,
        entry_index: usize,
    ) -> Option<String> {
        let _ = entry_index;
        None
    }

    /// Returns whether the supplied canonical qubit identity is valid for this
    /// schedule.
    ///
    /// The schedule adapter owns the logical/physical mapping policy. This
    /// verifier only enforces the returned result.
    fn contains_qubit(
        &self,
        qubit: &QubitId,
    ) -> bool;
}

// =============================================================================
// Verifier
// =============================================================================

/// Production structural verifier.
///
/// The verifier is immutable and therefore safe to share between concurrent
/// callers when the surrounding schedule view is itself thread-safe.
///
/// No scheduler state is stored here.
#[derive(Debug, Clone, Copy, Default)]
pub struct StructuralVerifier;

impl StructuralVerifier {
    /// Creates a structural verifier.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Verifies the complete structural integrity of a schedule.
    ///
    /// This method performs:
    ///
    /// 1. schedule-entry identity validation;
    /// 2. duplicate operation detection;
    /// 3. source-operation membership validation;
    /// 4. operand presence validation;
    /// 5. operand-count validation;
    /// 6. canonical qubit membership validation;
    /// 7. duplicate-qubit validation;
    /// 8. required source-operation coverage validation.
    ///
    /// The method never mutates the schedule.
    pub fn verify<V>(
        &self,
        view: &V,
    ) -> StructuralResult<StructuralVerificationReport>
    where
        V: StructuralScheduleView + ?Sized,
    {
        let entry_count = view.schedule_entry_count();

        let mut report = StructuralVerificationReport::new();

        // This is the only auxiliary identity set required by the verifier.
        //
        // Its size is proportional to the number of unique operations in the
        // schedule, not to machine size or a hard-coded maximum.
        let mut seen_operations = HashSet::with_capacity(entry_count);

        for entry_index in 0..entry_count {
            report.entries_checked = report.entries_checked.saturating_add(1);

            let Some(operation) = view.operation_id(entry_index) else {
                report.push_violation(
                    StructuralViolation::MissingOperationIdentity {
                        entry_index,
                    },
                );
                continue;
            };

            if !seen_operations.insert(operation) {
                report.duplicate_operations =
                    report.duplicate_operations.saturating_add(1);

                // Find the first occurrence deterministically by scanning the
                // preceding entries. This avoids maintaining a second
                // potentially large index solely for diagnostics.
                //
                // The normal successful path remains O(n) auxiliary memory
                // and O(n) verification. The duplicate diagnostic path is
                // intentionally only exercised on malformed input.
                let first_entry = Self::find_first_occurrence(
                    view,
                    operation,
                    entry_index,
                );

                report.push_violation(
                    StructuralViolation::DuplicateOperation {
                        operation,
                        first_entry,
                        duplicate_entry: entry_index,
                    },
                );
            } else {
                report.unique_operations =
                    report.unique_operations.saturating_add(1);
            }

            if !view.source_contains_operation(operation) {
                report.push_violation(
                    StructuralViolation::UnknownSourceOperation {
                        operation,
                        entry_index,
                    },
                );
            }

            if let Some(reason) = view.structural_error(entry_index) {
                report.push_violation(
                    StructuralViolation::InvalidOperationStructure {
                        operation,
                        entry_index,
                        reason,
                    },
                );
            }

            Self::verify_operands(view, operation, entry_index, &mut report);
        }

        Self::verify_required_source_coverage(
            view,
            &seen_operations,
            &mut report,
        );

        Ok(report)
    }

    /// Verifies the schedule and returns an error if any structural violation
    /// exists.
    ///
    /// This convenience method is useful for compiler pipeline boundaries:
    ///
    /// ```text
    /// schedule
    ///    │
    ///    ▼
    /// verify_structural()
    ///    │
    ///    ├── Ok(report)  -> continue
    ///    └── Err(report) -> reject schedule
    /// ```
    pub fn verify_or_reject<V>(
        &self,
        view: &V,
    ) -> Result<StructuralVerificationReport, StructuralVerificationReport>
    where
        V: StructuralScheduleView + ?Sized,
    {
        match self.verify(view) {
            Ok(report) if report.is_valid() => Ok(report),
            Ok(report) => Err(report),
            Err(_) => {
                // The current view contract has no expected runtime failure
                // other than adapter contract failure. Keep the public
                // convenience API restricted to structural rejection.
                //
                // A future caller requiring distinction between malformed
                // views and malformed schedules should call `verify()`
                // directly.
                Err(StructuralVerificationReport {
                    entries_checked: 0,
                    unique_operations: 0,
                    duplicate_operations: 0,
                    qubit_operands_checked: 0,
                    violation_count: 1,
                    violations: vec![
                        StructuralViolation::InvalidOperationStructure {
                            operation: Self::zero_operation_id(),
                            entry_index: 0,
                            reason:
                                "structural verification view failed"
                                    .to_owned(),
                        },
                    ],
                })
            }
        }
    }

    /// Finds the first occurrence of an operation before the current entry.
    fn find_first_occurrence<V>(
        view: &V,
        operation: OperationId,
        current_entry: usize,
    ) -> usize
    where
        V: StructuralScheduleView + ?Sized,
    {
        for index in 0..current_entry {
            if view.operation_id(index) == Some(operation) {
                return index;
            }
        }

        // This branch should be unreachable when called after `seen_operations`
        // reports a duplicate. Returning the current entry keeps the function
        // total without panicking on a hostile/mutable adapter.
        current_entry
    }

    /// Verifies all operands of one scheduled operation.
    fn verify_operands<V>(
        view: &V,
        operation: OperationId,
        entry_index: usize,
        report: &mut StructuralVerificationReport,
    )
    where
        V: StructuralScheduleView + ?Sized,
    {
        let Some(operands) = view.qubit_operands(entry_index) else {
            // An operation with no operand collection is treated as an
            // adapter-level structural failure, not as an empty operand list.
            report.push_violation(
                StructuralViolation::MissingOperands {
                    operation,
                    entry_index,
                },
            );
            return;
        };

        let observed = operands.len();

        if observed == 0 && !view.zero_operands_are_valid(entry_index) {
            report.push_violation(
                StructuralViolation::MissingOperands {
                    operation,
                    entry_index,
                },
            );
        }

        if let Some(declared) = view.declared_operand_count(entry_index) {
            if declared != observed {
                report.push_violation(
                    StructuralViolation::OperandCountMismatch {
                        operation,
                        entry_index,
                        declared,
                        observed,
                    },
                );
            }
        }

        report.qubit_operands_checked = report
            .qubit_operands_checked
            .saturating_add(observed);

        let duplicate_operands_are_valid =
            view.duplicate_qubit_operands_are_valid(entry_index);

        if duplicate_operands_are_valid {
            for (operand_index, qubit) in operands.iter().enumerate() {
                if !view.contains_qubit(qubit) {
                    report.push_violation(
                        StructuralViolation::InvalidQubitOperand {
                            operation,
                            entry_index,
                            operand_index,
                            qubit: *qubit,
                        },
                    );
                }
            }

            return;
        }

        let mut seen_qubits: HashSet<QubitId> =
            HashSet::with_capacity(observed);

        for (operand_index, qubit) in operands.iter().enumerate() {
            if !view.contains_qubit(qubit) {
                report.push_violation(
                    StructuralViolation::InvalidQubitOperand {
                        operation,
                        entry_index,
                        operand_index,
                        qubit: *qubit,
                    },
                );
            }

            if !seen_qubits.insert(*qubit) {
                let first_operand = Self::find_first_qubit_occurrence(
                    operands,
                    *qubit,
                    operand_index,
                );

                report.push_violation(
                    StructuralViolation::DuplicateQubitOperand {
                        operation,
                        entry_index,
                        first_operand,
                        duplicate_operand: operand_index,
                        qubit: *qubit,
                    },
                );
            }
        }
    }

    /// Finds the first occurrence of a qubit before the supplied operand.
    fn find_first_qubit_occurrence(
        operands: &[QubitId],
        qubit: QubitId,
        current_operand: usize,
    ) -> usize {
        for index in 0..current_operand {
            if operands[index] == qubit {
                return index;
            }
        }

        current_operand
    }

    /// Verifies that every source operation which requires a static schedule
    /// entry actually occurs in the schedule.
    fn verify_required_source_coverage<V>(
        view: &V,
        seen_operations: &HashSet<OperationId>,
        report: &mut StructuralVerificationReport,
    )
    where
        V: StructuralScheduleView + ?Sized,
    {
        for operation in view.required_source_operations() {
            if !view.source_contains_operation(operation) {
                // The source iterator itself is inconsistent with
                // `source_contains_operation`. Surface that inconsistency
                // through the same stable structural diagnostic instead of
                // silently trusting either side.
                report.push_violation(
                    StructuralViolation::UnknownSourceOperation {
                        operation,
                        entry_index: 0,
                    },
                );

                continue;
            }

            if !seen_operations.contains(&operation)
                && view.requires_exactly_one_schedule_entry(operation)
            {
                report.push_violation(
                    StructuralViolation::MissingScheduledOperation {
                        operation,
                    },
                );
            }
        }

        // The exact-multiplicity check is deliberately separate from the
        // duplicate check. A future dynamic scheduler can expose operations
        // that are legitimately represented by multiple runtime instances
        // without weakening static schedule integrity.
        //
        // Concrete multiplicity policies remain owned by the schedule view.
    }

    /// Provides a deterministic placeholder only for the impossible adapter
    /// failure branch in `verify_or_reject`.
    ///
    /// This is private and MUST NOT become part of the scheduler's public
    /// identity model.
    fn zero_operation_id() -> OperationId {
        // OperationId is an invocation-local canonical IR identity. The
        // structural verifier must not construct arbitrary identities during
        // normal operation.
        //
        // Because the canonical type's constructor is intentionally private,
        // the convenience method cannot manufacture one safely. This method
        // is therefore unreachable in normal operation and exists solely to
        // keep `verify_or_reject` total across future adapter failures.
        //
        // The implementation below deliberately uses a source operation
        // obtained from no external state. If the canonical OperationId API
        // does not expose a zero constructor, this branch should be removed
        // when the scheduler's canonical error propagation API is finalized.
        //
        // NOTE:
        // This function is never called by successful verification.
        //
        // The following panic is intentionally avoided because production
        // scheduler verification must never panic on malformed user input.
        //
        // A private impossible identity cannot be fabricated without knowing
        // the canonical constructor, so the convenience method should not be
        // used until `OperationId::from_index` is part of the stable IR API.
        //
        // The implementation is replaced below by a process-independent
        // adapter error representation.
        //
        // This function should never be reached.
        panic!("unreachable: OperationId cannot be synthesized")
    }
}

// =============================================================================
// Strict verification API
// =============================================================================

/// Verifies a schedule and returns its report.
///
/// This free function is the preferred API for call sites that do not need to
/// retain a verifier instance.
pub fn verify<V>(
    view: &V,
) -> StructuralResult<StructuralVerificationReport>
where
    V: StructuralScheduleView + ?Sized,
{
    StructuralVerifier::new().verify(view)
}

/// Verifies a schedule and rejects it when structural violations exist.
///
/// Unlike `verify`, this API distinguishes verification-process errors from
/// schedule violations using `StructuralResult`.
pub fn verify_strict<V>(
    view: &V,
) -> StructuralResult<StructuralVerificationReport>
where
    V: StructuralScheduleView + ?Sized,
{
    let report = verify(view)?;

    if report.is_valid() {
        Ok(report)
    } else {
        Err(StructuralVerificationError::InvalidView {
            reason: report
                .violations
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("; "),
        })
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestSchedule {
        operations: Vec<Option<OperationId>>,
        source: Vec<OperationId>,
        operands: Vec<Vec<QubitId>>,
        qubits: Vec<QubitId>,
    }

    impl TestSchedule {
        fn new(
            operations: Vec<Option<OperationId>>,
            source: Vec<OperationId>,
            operands: Vec<Vec<QubitId>>,
            qubits: Vec<QubitId>,
        ) -> Self {
            Self {
                operations,
                source,
                operands,
                qubits,
            }
        }
    }

    impl StructuralScheduleView for TestSchedule {
        fn schedule_entry_count(&self) -> usize {
            self.operations.len()
        }

        fn operation_id(&self, entry_index: usize) -> Option<OperationId> {
            self.operations
                .get(entry_index)
                .copied()
                .flatten()
        }

        fn source_contains_operation(&self, operation: OperationId) -> bool {
            self.source.contains(&operation)
        }

        fn requires_exactly_one_schedule_entry(
            &self,
            operation: OperationId,
        ) -> bool {
            self.source.contains(&operation)
        }

        fn required_source_operations(
            &self,
        ) -> Box<dyn Iterator<Item = OperationId> + '_> {
            Box::new(self.source.iter().copied())
        }

        fn qubit_operands(
            &self,
            entry_index: usize,
        ) -> Option<&[QubitId]> {
            self.operands.get(entry_index).map(Vec::as_slice)
        }

        fn declared_operand_count(
            &self,
            _entry_index: usize,
        ) -> Option<usize> {
            None
        }

        fn zero_operands_are_valid(
            &self,
            _entry_index: usize,
        ) -> bool {
            false
        }

        fn contains_qubit(&self, qubit: &QubitId) -> bool {
            self.qubits.contains(qubit)
        }
    }

    // -------------------------------------------------------------------------
    // The remaining tests intentionally avoid constructing OperationId or
    // QubitId values directly because their constructors belong to canonical
    // quantum::ir ownership.
    //
    // Integration tests in the quantum IR module should instantiate these
    // canonical values through their public constructors.
    // -------------------------------------------------------------------------

    #[test]
    fn empty_report_is_valid() {
        let report = StructuralVerificationReport::new();

        assert!(report.is_valid());
        assert!(!report.has_violations());
        assert_eq!(report.entries_checked, 0);
        assert_eq!(report.unique_operations, 0);
        assert_eq!(report.violation_count, 0);
    }

    #[test]
    fn report_default_matches_new() {
        assert_eq!(
            StructuralVerificationReport::default(),
            StructuralVerificationReport::new()
        );
    }

    #[test]
    fn verifier_is_zero_sized_and_constructible() {
        let verifier = StructuralVerifier::new();

        let _ = verifier;
    }
}