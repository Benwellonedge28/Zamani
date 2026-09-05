//! Zamani Quantum Scheduling — Dynamic Conditional Execution
//!
//! Production-grade, hardware-independent conditional scheduling support.
//!
//! ============================================================================
//! ARCHITECTURAL ROLE
//! ============================================================================
//!
//! This module models scheduling information for operations whose execution
//! depends on a classical condition.
//!
//! It answers:
//!
//! > "When is a conditional operation eligible to execute, and what must be
//! > known before it can execute?"
//!
//! It does NOT:
//!
//! - evaluate arbitrary classical expressions;
//! - execute classical code;
//! - execute quantum operations;
//! - perform measurements;
//! - define quantum gate semantics;
//! - perform logical-to-physical routing;
//! - discover hardware;
//! - reserve hardware resources;
//! - choose the global scheduling algorithm;
//! - decode QEC syndromes;
//! - communicate with a QPU.
//!
//! Those responsibilities remain in their owning subsystems.
//!
//! ============================================================================
//! CANONICAL OWNERSHIP
//! ============================================================================
//!
//! Canonical condition semantics are owned by:
//!
//!     crate::quantum::ir::control::condition::Condition
//!
//! Canonical quantum operation identities are owned by:
//!
//!     crate::quantum::ir::core::identity::OperationId
//!
//! Canonical logical/physical qubit identities are owned by:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! This module never defines replacement identity types.
//!
//! ============================================================================
//! CONDITIONAL EXECUTION MODEL
//! ============================================================================
//!
//! A conditional operation is represented as:
//!
//!     producer(s)
//!          |
//!          v
//!     classical value(s)
//!          |
//!          v
//!       Condition
//!          |
//!          v
//!     conditional operation
//!
//! The condition itself is semantic information.
//!
//! The dependencies represented here are scheduling information.
//!
//! Those concepts MUST remain separate.
//!
//! ============================================================================
//! STATIC AND DYNAMIC EXECUTION
//! ============================================================================
//!
//! The module supports three cases:
//!
//! 1. Unconditional:
//!
//!        operation
//!
//! 2. Compile-time conditional:
//!
//!        known condition
//!             |
//!             v
//!        operation
//!
//! 3. Runtime conditional:
//!
//!        measurement/classical producer
//!                    |
//!                    v
//!             classical value
//!                    |
//!                    v
//!                condition
//!                    |
//!                    v
//!            conditional operation
//!
//! Runtime conditions are not evaluated here. The runtime/control subsystem
//! supplies the resulting readiness state.
//!
//! ============================================================================
//! SCALABILITY
//! ============================================================================
//!
//! There are no:
//!
//! - maximum number of conditions;
//! - maximum number of branches;
//! - maximum number of classical values;
//! - maximum number of qubits;
//! - maximum number of operations;
//! - fixed controller count;
//! - fixed feedback latency;
//! - fixed hardware timing.
//!
//! Collections grow according to actual program size and available host
//! resources.
//!
//! "Infinity" therefore means that this module introduces no artificial finite
//! machine-size ceiling. A real compilation remains bounded by memory, CPU,
//! explicit compiler policies, and the target itself.
//!
//! ============================================================================
//! SAFETY
//! ============================================================================
//!
//! Rust 1.97 / Rust 1.97.1.
//! Rust 2021.
//! Stable Rust.
//! No unsafe code.
//!
//! The requirement is compiler-enforced.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! Dependency storage uses ordered collections where ordering affects public
//! iteration or diagnostics.
//!
//! No wall-clock time is consulted.
//! No hidden randomness is used.
//! No global mutable state exists.
//!
//! ============================================================================
//! INTEGRATION
//! ============================================================================
//!
//! Upstream:
//!
//!     quantum::ir
//!          |
//!          v
//!     dynamic control analysis
//!          |
//!          v
//!     ConditionalOperation
//!          |
//!          v
//!     planners / algorithms
//!
//! Related scheduler modules:
//!
//!     dynamic::classical
//!     dynamic::feedback
//!     dynamic::runtime
//!     constraints::control
//!     ir::dependency
//!     timing::*
//!     resources::*
//!     verification::*
//!
//! The planner owns schedule mutation.
//! This module only supplies conditional eligibility information.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::quantum::ir::control::condition::Condition;
use crate::quantum::ir::core::identity::{OperationId, ValueId};
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

use crate::quantum::scheduling::types::{Duration, TimePoint};

// ============================================================================
// Branch identity
// ============================================================================

/// Stable scheduler-local identity for a dynamic control branch.
///
/// This identity is intentionally separate from `OperationId`.
///
/// An operation may belong to multiple logical control-flow structures while
/// retaining one canonical IR operation identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BranchId(u64);

impl BranchId {
    /// Creates a branch identity.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric identity.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl From<u64> for BranchId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<BranchId> for u64 {
    fn from(value: BranchId) -> Self {
        value.value()
    }
}

impl fmt::Display for BranchId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "branch:{}", self.0)
    }
}

// ============================================================================
// Classical readiness identity
// ============================================================================

/// Identity of one classical value required by a condition.
///
/// This is deliberately an alias-like wrapper around the canonical IR
/// `ValueId` rather than a replacement classical value system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassicalValueId(ValueId);

impl ClassicalValueId {
    /// Creates an identifier from the canonical IR value identity.
    #[must_use]
    pub const fn new(value: ValueId) -> Self {
        Self(value)
    }

    /// Returns the canonical IR value identity.
    #[must_use]
    pub const fn value(self) -> ValueId {
        self.0
    }
}

impl From<ValueId> for ClassicalValueId {
    fn from(value: ValueId) -> Self {
        Self::new(value)
    }
}

impl From<ClassicalValueId> for ValueId {
    fn from(value: ClassicalValueId) -> Self {
        value.value()
    }
}

impl fmt::Display for ClassicalValueId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "classical-value:{}", self.0)
    }
}

// ============================================================================
// Dependency source
// ============================================================================

/// Source that makes a classical dependency available.
///
/// The source is intentionally descriptive. It does not execute or reserve
/// anything.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum DependencySource {
    /// A preceding IR operation produces the required value.
    Operation(OperationId),

    /// The value is supplied by an externally established runtime input.
    External(ClassicalValueId),

    /// The value becomes available after a runtime event.
    Runtime(ClassicalValueId),

    /// The dependency is produced by a QEC/decoder pipeline.
    Qec(ClassicalValueId),
}

impl DependencySource {
    /// Returns the associated canonical value when one exists.
    #[must_use]
    pub const fn value(self) -> Option<ClassicalValueId> {
        match self {
            Self::Operation(_) => None,
            Self::External(value)
            | Self::Runtime(value)
            | Self::Qec(value) => Some(value),
        }
    }

    /// Returns the producing operation when one exists.
    #[must_use]
    pub const fn operation(self) -> Option<OperationId> {
        match self {
            Self::Operation(operation) => Some(operation),
            Self::External(_) | Self::Runtime(_) | Self::Qec(_) => None,
        }
    }
}

// ============================================================================
// Readiness requirement
// ============================================================================

/// One prerequisite required before a condition may be resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ReadinessRequirement {
    /// Source producing or supplying the required value.
    source: DependencySource,

    /// Additional target-dependent latency between source completion and
    /// condition-readiness.
    latency: Duration,

    /// Whether this dependency must be available at runtime rather than merely
    /// known statically during compilation.
    runtime: bool,
}

impl ReadinessRequirement {
    /// Creates a requirement produced by an IR operation.
    #[must_use]
    pub const fn operation(
        operation: OperationId,
        latency: Duration,
    ) -> Self {
        Self {
            source: DependencySource::Operation(operation),
            latency,
            runtime: true,
        }
    }

    /// Creates an externally supplied value requirement.
    #[must_use]
    pub const fn external(
        value: ClassicalValueId,
        latency: Duration,
    ) -> Self {
        Self {
            source: DependencySource::External(value),
            latency,
            runtime: false,
        }
    }

    /// Creates a runtime value requirement.
    #[must_use]
    pub const fn runtime(
        value: ClassicalValueId,
        latency: Duration,
    ) -> Self {
        Self {
            source: DependencySource::Runtime(value),
            latency,
            runtime: true,
        }
    }

    /// Creates a QEC/decoder-produced value requirement.
    #[must_use]
    pub const fn qec(
        value: ClassicalValueId,
        latency: Duration,
    ) -> Self {
        Self {
            source: DependencySource::Qec(value),
            latency,
            runtime: true,
        }
    }

    /// Returns the dependency source.
    #[must_use]
    pub const fn source(self) -> DependencySource {
        self.source
    }

    /// Returns the target-supplied readiness latency.
    #[must_use]
    pub const fn latency(self) -> Duration {
        self.latency
    }

    /// Returns whether this is a runtime dependency.
    #[must_use]
    pub const fn is_runtime(self) -> bool {
        self.runtime
    }

    /// Returns the producing operation, if known.
    #[must_use]
    pub const fn producer(self) -> Option<OperationId> {
        self.source.operation()
    }

    /// Returns the required classical value, if known.
    #[must_use]
    pub const fn value(self) -> Option<ClassicalValueId> {
        self.source.value()
    }
}

// ============================================================================
// Readiness state
// ============================================================================

/// Runtime/compile-time state of one conditional dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ReadinessState {
    /// The dependency has not become available.
    Pending,

    /// The dependency is known to be available.
    Ready,

    /// The dependency cannot currently be established.
    ///
    /// This does not necessarily mean the entire program is invalid; a runtime
    /// scheduler may wait for a future event.
    Unavailable,

    /// The dependency is permanently impossible.
    Impossible,
}

impl ReadinessState {
    /// Returns whether the dependency can currently be consumed.
    #[must_use]
    pub const fn is_ready(self) -> bool {
        matches!(self, Self::Ready)
    }

    /// Returns whether execution cannot proceed now.
    #[must_use]
    pub const fn is_blocked(self) -> bool {
        !self.is_ready()
    }

    /// Returns whether the dependency is permanently impossible.
    #[must_use]
    pub const fn is_impossible(self) -> bool {
        matches!(self, Self::Impossible)
    }
}

// ============================================================================
// Dependency readiness
// ============================================================================

/// Observed readiness information for one conditional dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DependencyReadiness {
    requirement: ReadinessRequirement,
    state: ReadinessState,
    available_at: Option<TimePoint>,
}

impl DependencyReadiness {
    /// Creates a pending dependency.
    #[must_use]
    pub const fn pending(requirement: ReadinessRequirement) -> Self {
        Self {
            requirement,
            state: ReadinessState::Pending,
            available_at: None,
        }
    }

    /// Creates a ready dependency.
    #[must_use]
    pub const fn ready(
        requirement: ReadinessRequirement,
        available_at: TimePoint,
    ) -> Self {
        Self {
            requirement,
            state: ReadinessState::Ready,
            available_at: Some(available_at),
        }
    }

    /// Creates an unavailable dependency.
    #[must_use]
    pub const fn unavailable(requirement: ReadinessRequirement) -> Self {
        Self {
            requirement,
            state: ReadinessState::Unavailable,
            available_at: None,
        }
    }

    /// Creates an impossible dependency.
    #[must_use]
    pub const fn impossible(requirement: ReadinessRequirement) -> Self {
        Self {
            requirement,
            state: ReadinessState::Impossible,
            available_at: None,
        }
    }

    /// Returns the underlying requirement.
    #[must_use]
    pub const fn requirement(self) -> ReadinessRequirement {
        self.requirement
    }

    /// Returns the readiness state.
    #[must_use]
    pub const fn state(self) -> ReadinessState {
        self.state
    }

    /// Returns when the value became available, if known.
    #[must_use]
    pub const fn available_at(self) -> Option<TimePoint> {
        self.available_at
    }

    /// Returns the earliest point at which the dependency can be consumed.
    ///
    /// The caller must supply the source-completion time. This method adds the
    /// target-provided readiness latency using checked arithmetic.
    #[must_use]
    pub fn earliest_consumable_at(
        self,
        source_completion: TimePoint,
    ) -> Option<TimePoint> {
        source_completion.checked_add(self.requirement.latency())
    }
}

// ============================================================================
// Branch behavior
// ============================================================================

/// Semantic scheduling classification of a conditional operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ConditionalExecution {
    /// The operation has no condition.
    Unconditional,

    /// The condition is known during compilation.
    CompileTime,

    /// The condition requires runtime classical information.
    Runtime,

    /// The operation belongs to a branch whose execution cannot occur.
    Never,
}

impl ConditionalExecution {
    /// Returns whether the operation requires runtime information.
    #[must_use]
    pub const fn is_runtime(self) -> bool {
        matches!(self, Self::Runtime)
    }

    /// Returns whether the operation is unconditionally executable.
    #[must_use]
    pub const fn is_unconditional(self) -> bool {
        matches!(self, Self::Unconditional)
    }

    /// Returns whether the operation is impossible.
    #[must_use]
    pub const fn is_never(self) -> bool {
        matches!(self, Self::Never)
    }
}

// ============================================================================
// Conditional operation
// ============================================================================

/// Scheduler-facing description of one conditional operation.
///
/// The actual operation remains in canonical quantum IR. This structure
/// provides only the dynamic-control metadata required by scheduling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalOperation {
    operation: OperationId,
    condition: Option<Condition>,
    execution: ConditionalExecution,
    requirements: Vec<ReadinessRequirement>,
    logical_qubits: Vec<QubitId>,
    physical_qubits: Vec<PhysicalQubitId>,
    branch: Option<BranchId>,
}

impl ConditionalOperation {
    /// Creates an unconditional operation descriptor.
    #[must_use]
    pub fn unconditional(operation: OperationId) -> Self {
        Self {
            operation,
            condition: None,
            execution: ConditionalExecution::Unconditional,
            requirements: Vec::new(),
            logical_qubits: Vec::new(),
            physical_qubits: Vec::new(),
            branch: None,
        }
    }

    /// Creates a compile-time conditional operation.
    #[must_use]
    pub fn compile_time(
        operation: OperationId,
        condition: Condition,
    ) -> Self {
        Self {
            operation,
            condition: Some(condition),
            execution: ConditionalExecution::CompileTime,
            requirements: Vec::new(),
            logical_qubits: Vec::new(),
            physical_qubits: Vec::new(),
            branch: None,
        }
    }

    /// Creates a runtime conditional operation.
    #[must_use]
    pub fn runtime(
        operation: OperationId,
        condition: Condition,
    ) -> Self {
        Self {
            operation,
            condition: Some(condition),
            execution: ConditionalExecution::Runtime,
            requirements: Vec::new(),
            logical_qubits: Vec::new(),
            physical_qubits: Vec::new(),
            branch: None,
        }
    }

    /// Creates an operation known to be unreachable.
    #[must_use]
    pub fn never(
        operation: OperationId,
        condition: Option<Condition>,
    ) -> Self {
        Self {
            operation,
            condition,
            execution: ConditionalExecution::Never,
            requirements: Vec::new(),
            logical_qubits: Vec::new(),
            physical_qubits: Vec::new(),
            branch: None,
        }
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        self.operation
    }

    /// Returns the canonical condition.
    #[must_use]
    pub const fn condition(&self) -> Option<&Condition> {
        self.condition.as_ref()
    }

    /// Returns the execution classification.
    #[must_use]
    pub const fn execution(&self) -> ConditionalExecution {
        self.execution
    }

    /// Returns the readiness requirements.
    #[must_use]
    pub fn requirements(&self) -> &[ReadinessRequirement] {
        &self.requirements
    }

    /// Returns the logical qubits affected by the operation.
    ///
    /// These are canonical `quantum::ir::qubit::QubitId` values.
    #[must_use]
    pub fn logical_qubits(&self) -> &[QubitId] {
        &self.logical_qubits
    }

    /// Returns the physical qubits affected by the operation.
    ///
    /// These are canonical `quantum::ir::qubit::PhysicalQubitId` values.
    #[must_use]
    pub fn physical_qubits(&self) -> &[PhysicalQubitId] {
        &self.physical_qubits
    }

    /// Returns the enclosing branch, if any.
    #[must_use]
    pub const fn branch(&self) -> Option<BranchId> {
        self.branch
    }

    /// Adds a classical readiness requirement.
    ///
    /// Duplicate requirements are retained intentionally because provenance
    /// may matter to diagnostics and verification. Callers that need set
    /// semantics should normalize at the analysis boundary.
    pub fn add_requirement(
        &mut self,
        requirement: ReadinessRequirement,
    ) {
        self.requirements.push(requirement);
    }

    /// Associates a logical qubit with this operation.
    pub fn add_logical_qubit(&mut self, qubit: QubitId) {
        self.logical_qubits.push(qubit);
    }

    /// Associates a physical qubit with this operation.
    pub fn add_physical_qubit(&mut self, qubit: PhysicalQubitId) {
        self.physical_qubits.push(qubit);
    }

    /// Associates the operation with a dynamic branch.
    pub const fn set_branch(&mut self, branch: BranchId) {
        self.branch = Some(branch);
    }

    /// Returns whether the operation can ever execute.
    #[must_use]
    pub const fn is_executable(&self) -> bool {
        !self.execution.is_never()
    }

    /// Returns whether this operation needs runtime classical resolution.
    #[must_use]
    pub const fn requires_runtime_resolution(&self) -> bool {
        self.execution.is_runtime()
    }
}

// ============================================================================
// Eligibility
// ============================================================================

/// Result of checking whether a conditional operation is currently eligible.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConditionalEligibility {
    /// The operation may be considered by the scheduler now.
    Ready {
        /// Earliest time at which all classical requirements are consumable.
        earliest_start: TimePoint,
    },

    /// At least one dependency is still pending.
    Waiting {
        /// Dependencies preventing execution.
        pending: BTreeSet<usize>,
    },

    /// A dependency is temporarily unavailable.
    Unavailable {
        /// Dependencies that are unavailable.
        dependencies: BTreeSet<usize>,
    },

    /// The condition can never become executable.
    Impossible {
        /// Dependencies or branch information proving impossibility.
        dependencies: BTreeSet<usize>,
    },
}

impl ConditionalEligibility {
    /// Returns whether the operation is currently ready.
    #[must_use]
    pub const fn is_ready(&self) -> bool {
        matches!(self, Self::Ready { .. })
    }

    /// Returns the earliest start time when ready.
    #[must_use]
    pub const fn earliest_start(&self) -> Option<TimePoint> {
        match self {
            Self::Ready { earliest_start } => Some(*earliest_start),
            Self::Waiting { .. }
            | Self::Unavailable { .. }
            | Self::Impossible { .. } => None,
        }
    }
}

// ============================================================================
// Eligibility evaluator
// ============================================================================

/// Stateless evaluator for conditional scheduling readiness.
///
/// This type deliberately owns no mutable scheduling state. The scheduler
/// planner supplies current dependency readiness each time it evaluates an
/// operation.
#[derive(Debug, Default, Clone, Copy)]
pub struct ConditionalEligibilityEvaluator;

impl ConditionalEligibilityEvaluator {
    /// Creates an evaluator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Evaluates conditional readiness.
    ///
    /// `readiness` must correspond to the operation's requirements in the same
    /// order as `ConditionalOperation::requirements()`.
    ///
    /// `now` is the earliest scheduler-visible point at which a newly ready
    /// operation could start.
    pub fn evaluate(
        &self,
        operation: &ConditionalOperation,
        readiness: &[DependencyReadiness],
        now: TimePoint,
    ) -> ConditionalEligibility {
        if operation.execution().is_never() {
            return ConditionalEligibility::Impossible {
                dependencies: BTreeSet::new(),
            };
        }

        if operation.execution().is_unconditional()
            || operation.requirements().is_empty()
        {
            return ConditionalEligibility::Ready {
                earliest_start: now,
            };
        }

        if readiness.len() != operation.requirements().len() {
            return ConditionalEligibility::Waiting {
                pending: (readiness.len()..operation.requirements().len())
                    .collect(),
            };
        }

        let mut pending = BTreeSet::new();
        let mut unavailable = BTreeSet::new();
        let mut impossible = BTreeSet::new();
        let mut earliest = now;

        for (index, state) in readiness.iter().enumerate() {
            match state.state() {
                ReadinessState::Pending => {
                    pending.insert(index);
                }
                ReadinessState::Unavailable => {
                    unavailable.insert(index);
                }
                ReadinessState::Impossible => {
                    impossible.insert(index);
                }
                ReadinessState::Ready => {
                    if let Some(available_at) = state.available_at() {
                        if available_at > earliest {
                            earliest = available_at;
                        }
                    }
                }
            }
        }

        if !impossible.is_empty() {
            return ConditionalEligibility::Impossible {
                dependencies: impossible,
            };
        }

        if !unavailable.is_empty() {
            return ConditionalEligibility::Unavailable {
                dependencies: unavailable,
            };
        }

        if !pending.is_empty() {
            return ConditionalEligibility::Waiting { pending };
        }

        ConditionalEligibility::Ready {
            earliest_start: earliest,
        }
    }
}

// ============================================================================
// Runtime branch state
// ============================================================================

/// Runtime state of a dynamic branch.
///
/// This structure represents observations, not execution. The runtime layer
/// remains responsible for actually selecting and executing a branch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchState {
    branch: BranchId,
    entered_at: Option<TimePoint>,
    resolved: bool,
    taken: Option<bool>,
    values: BTreeMap<ClassicalValueId, bool>,
}

impl BranchState {
    /// Creates an unresolved branch.
    #[must_use]
    pub fn new(branch: BranchId) -> Self {
        Self {
            branch,
            entered_at: None,
            resolved: false,
            taken: None,
            values: BTreeMap::new(),
        }
    }

    /// Returns the branch identity.
    #[must_use]
    pub const fn branch(&self) -> BranchId {
        self.branch
    }

    /// Marks branch evaluation as having become active.
    pub const fn activate(&mut self, at: TimePoint) {
        self.entered_at = Some(at);
    }

    /// Records one resolved Boolean value.
    pub fn record_value(
        &mut self,
        value: ClassicalValueId,
        result: bool,
    ) {
        self.values.insert(value, result);
    }

    /// Marks the branch condition as resolved.
    pub const fn resolve(&mut self, taken: bool) {
        self.resolved = true;
        self.taken = Some(taken);
    }

    /// Returns whether the branch condition is resolved.
    #[must_use]
    pub const fn is_resolved(&self) -> bool {
        self.resolved
    }

    /// Returns whether this branch was selected.
    #[must_use]
    pub const fn was_taken(&self) -> Option<bool> {
        self.taken
    }

    /// Returns the activation time.
    #[must_use]
    pub const fn entered_at(&self) -> Option<TimePoint> {
        self.entered_at
    }

    /// Returns an observed classical value.
    #[must_use]
    pub fn value(&self, id: ClassicalValueId) -> Option<bool> {
        self.values.get(&id).copied()
    }

    /// Returns all observed values in deterministic order.
    #[must_use]
    pub fn values(
        &self,
    ) -> &BTreeMap<ClassicalValueId, bool> {
        &self.values
    }
}

// ============================================================================
// Conditional schedule metadata
// ============================================================================

/// Metadata emitted alongside a scheduled conditional operation.
///
/// The planner can use this structure to preserve dynamic-control provenance
/// in the final `ScheduleResult`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalScheduleMetadata {
    operation: OperationId,
    execution: ConditionalExecution,
    branch: Option<BranchId>,
    earliest_classical_ready: Option<TimePoint>,
    runtime_resolved: bool,
    dependency_count: usize,
}

impl ConditionalScheduleMetadata {
    /// Creates metadata from a conditional operation and its eligibility.
    #[must_use]
    pub fn from_operation(
        operation: &ConditionalOperation,
        eligibility: &ConditionalEligibility,
    ) -> Self {
        Self {
            operation: operation.operation(),
            execution: operation.execution(),
            branch: operation.branch(),
            earliest_classical_ready: eligibility.earliest_start(),
            runtime_resolved: operation.requires_runtime_resolution(),
            dependency_count: operation.requirements().len(),
        }
    }

    /// Returns the operation identity.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        self.operation
    }

    /// Returns execution classification.
    #[must_use]
    pub const fn execution(&self) -> ConditionalExecution {
        self.execution
    }

    /// Returns branch identity.
    #[must_use]
    pub const fn branch(&self) -> Option<BranchId> {
        self.branch
    }

    /// Returns the earliest classical readiness time.
    #[must_use]
    pub const fn earliest_classical_ready(&self) -> Option<TimePoint> {
        self.earliest_classical_ready
    }

    /// Returns whether runtime resolution is required.
    #[must_use]
    pub const fn runtime_resolved(&self) -> bool {
        self.runtime_resolved
    }

    /// Returns the number of readiness dependencies.
    #[must_use]
    pub const fn dependency_count(&self) -> usize {
        self.dependency_count
    }
}

// ============================================================================
// Validation
// ============================================================================

/// Validation failures specific to dynamic conditional scheduling.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConditionalValidationError {
    /// A runtime conditional operation has no canonical condition.
    MissingCondition {
        operation: OperationId,
    },

    /// A conditional dependency was created without an identifiable source.
    InvalidDependency,

    /// Readiness records do not correspond one-to-one with requirements.
    ReadinessCountMismatch {
        expected: usize,
        actual: usize,
    },

    /// A conditional operation has a readiness timestamp that cannot be
    /// represented safely.
    TimeOverflow {
        operation: OperationId,
    },

    /// A runtime operation has no runtime dependency and therefore cannot
    /// obtain runtime condition information from the scheduling layer.
    RuntimeConditionWithoutDependency {
        operation: OperationId,
    },
}

impl fmt::Display for ConditionalValidationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::MissingCondition { operation } => {
                write!(
                    formatter,
                    "conditional operation {operation} is missing its canonical condition"
                )
            }
            Self::InvalidDependency => {
                formatter.write_str("conditional dependency has no identifiable source")
            }
            Self::ReadinessCountMismatch { expected, actual } => {
                write!(
                    formatter,
                    "conditional readiness count mismatch: expected {expected}, got {actual}"
                )
            }
            Self::TimeOverflow { operation } => {
                write!(
                    formatter,
                    "conditional operation {operation} produced an overflowing readiness time"
                )
            }
            Self::RuntimeConditionWithoutDependency { operation } => {
                write!(
                    formatter,
                    "runtime conditional operation {operation} has no runtime readiness dependency"
                )
            }
        }
    }
}

impl std::error::Error for ConditionalValidationError {}

// ============================================================================
// Validator
// ============================================================================

/// Stateless validator for conditional scheduling descriptors.
#[derive(Debug, Default, Clone, Copy)]
pub struct ConditionalValidator;

impl ConditionalValidator {
    /// Creates a validator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Validates one conditional operation descriptor.
    pub fn validate(
        &self,
        operation: &ConditionalOperation,
    ) -> Result<(), ConditionalValidationError> {
        match operation.execution() {
            ConditionalExecution::Unconditional
            | ConditionalExecution::Never => Ok(()),

            ConditionalExecution::CompileTime => {
                if operation.condition().is_none() {
                    return Err(
                        ConditionalValidationError::MissingCondition {
                            operation: operation.operation(),
                        },
                    );
                }

                self.validate_dependencies(operation)
            }

            ConditionalExecution::Runtime => {
                if operation.condition().is_none() {
                    return Err(
                        ConditionalValidationError::MissingCondition {
                            operation: operation.operation(),
                        },
                    );
                }

                self.validate_dependencies(operation)?;

                if operation
                    .requirements()
                    .iter()
                    .all(|requirement| !requirement.is_runtime())
                {
                    return Err(
                        ConditionalValidationError::
                            RuntimeConditionWithoutDependency {
                                operation: operation.operation(),
                            },
                    );
                }

                Ok(())
            }
        }
    }

    fn validate_dependencies(
        &self,
        operation: &ConditionalOperation,
    ) -> Result<(), ConditionalValidationError> {
        for requirement in operation.requirements() {
            if requirement.source().operation().is_none()
                && requirement.source().value().is_none()
            {
                return Err(
                    ConditionalValidationError::InvalidDependency,
                );
            }
        }

        Ok(())
    }

    /// Validates an eligibility evaluation.
    pub fn validate_readiness(
        &self,
        operation: &ConditionalOperation,
        readiness: &[DependencyReadiness],
    ) -> Result<(), ConditionalValidationError> {
        if operation.requirements().len() != readiness.len() {
            return Err(
                ConditionalValidationError::ReadinessCountMismatch {
                    expected: operation.requirements().len(),
                    actual: readiness.len(),
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// Dependency index
// ============================================================================

/// Deterministic reverse index from classical values to dependent operations.
///
/// This allows dynamic scheduling to wake only operations affected by a newly
/// available value rather than rescanning every conditional operation.
///
/// The scheduler/runtime layer owns mutation of this index.
#[derive(Debug, Default, Clone)]
pub struct ConditionalDependencyIndex {
    by_value: BTreeMap<ClassicalValueId, BTreeSet<OperationId>>,
    by_operation: BTreeMap<OperationId, BTreeSet<ClassicalValueId>>,
}

impl ConditionalDependencyIndex {
    /// Creates an empty dependency index.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a conditional operation.
    pub fn register(
        &mut self,
        operation: &ConditionalOperation,
    ) {
        let operation_id = operation.operation();

        for requirement in operation.requirements() {
            if let Some(value) = requirement.value() {
                self.by_value
                    .entry(value)
                    .or_default()
                    .insert(operation_id);

                self.by_operation
                    .entry(operation_id)
                    .or_default()
                    .insert(value);
            }
        }
    }

    /// Removes an operation from the index.
    pub fn unregister(
        &mut self,
        operation: OperationId,
    ) {
        if let Some(values) = self.by_operation.remove(&operation) {
            for value in values {
                if let Some(operations) = self.by_value.get_mut(&value) {
                    operations.remove(&operation);

                    if operations.is_empty() {
                        self.by_value.remove(&value);
                    }
                }
            }
        }
    }

    /// Returns operations depending on a classical value.
    #[must_use]
    pub fn dependents(
        &self,
        value: ClassicalValueId,
    ) -> Option<&BTreeSet<OperationId>> {
        self.by_value.get(&value)
    }

    /// Returns the values required by an operation.
    #[must_use]
    pub fn dependencies(
        &self,
        operation: OperationId,
    ) -> Option<&BTreeSet<ClassicalValueId>> {
        self.by_operation.get(&operation)
    }

    /// Returns the number of indexed operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.by_operation.len()
    }

    /// Returns the number of indexed classical values.
    #[must_use]
    pub fn value_count(&self) -> usize {
        self.by_value.len()
    }

    /// Clears the index.
    pub fn clear(&mut self) {
        self.by_value.clear();
        self.by_operation.clear();
    }

    /// Returns whether no dependencies are indexed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.by_operation.is_empty()
    }
}

// ============================================================================
// Public helper
// ============================================================================

/// Computes the earliest time at which all conditional dependencies can be
/// consumed.
///
/// Returns `None` when:
///
/// - a dependency is not ready;
/// - a dependency is impossible;
/// - checked time arithmetic overflows.
///
/// The function is intentionally independent of a scheduler algorithm.
#[must_use]
pub fn earliest_conditional_start(
    readiness: &[DependencyReadiness],
    now: TimePoint,
) -> Option<TimePoint> {
    let mut earliest = now;

    for dependency in readiness {
        if !dependency.state().is_ready() {
            return None;
        }

        if let Some(available_at) = dependency.available_at() {
            if available_at > earliest {
                earliest = available_at;
            }
        }
    }

    Some(earliest)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branch_identity_is_stable() {
        let branch = BranchId::new(42);

        assert_eq!(branch.value(), 42);
        assert_eq!(BranchId::from(42_u64), branch);
    }

    #[test]
    fn classical_value_identity_wraps_canonical_value() {
        let value = ValueId::new(7);
        let wrapped = ClassicalValueId::new(value);

        assert_eq!(wrapped.value(), value);
    }

    #[test]
    fn pending_dependency_blocks_execution() {
        let operation_id = OperationId::new(1);
        let value = ClassicalValueId::new(ValueId::new(10));

        let mut operation =
            ConditionalOperation::runtime(
                operation_id,
                Condition::always(),
            );

        operation.add_requirement(
            ReadinessRequirement::runtime(
                value,
                Duration::ZERO,
            ),
        );

        let readiness = [
            DependencyReadiness::pending(
                operation.requirements()[0],
            ),
        ];

        let evaluator =
            ConditionalEligibilityEvaluator::new();

        let result = evaluator.evaluate(
            &operation,
            &readiness,
            TimePoint::ZERO,
        );

        assert!(matches!(
            result,
            ConditionalEligibility::Waiting { .. }
        ));
    }

    #[test]
    fn ready_dependencies_produce_earliest_start() {
        let operation_id = OperationId::new(2);
        let value = ClassicalValueId::new(ValueId::new(11));

        let mut operation =
            ConditionalOperation::runtime(
                operation_id,
                Condition::always(),
            );

        operation.add_requirement(
            ReadinessRequirement::runtime(
                value,
                Duration::ZERO,
            ),
        );

        let readiness = [
            DependencyReadiness::ready(
                operation.requirements()[0],
                TimePoint::new(50),
            ),
        ];

        let evaluator =
            ConditionalEligibilityEvaluator::new();

        let result = evaluator.evaluate(
            &operation,
            &readiness,
            TimePoint::new(10),
        );

        assert_eq!(
            result.earliest_start(),
            Some(TimePoint::new(50))
        );
    }

    #[test]
    fn dependency_index_is_incremental() {
        let operation_id = OperationId::new(3);
        let value = ClassicalValueId::new(ValueId::new(12));

        let mut operation =
            ConditionalOperation::runtime(
                operation_id,
                Condition::always(),
            );

        operation.add_requirement(
            ReadinessRequirement::runtime(
                value,
                Duration::ZERO,
            ),
        );

        let mut index =
            ConditionalDependencyIndex::new();

        index.register(&operation);

        assert_eq!(
            index.operation_count(),
            1
        );
        assert_eq!(
            index.value_count(),
            1
        );
        assert!(
            index
                .dependents(value)
                .is_some_and(|operations| {
                    operations.contains(&operation_id)
                })
        );

        index.unregister(operation_id);

        assert!(index.is_empty());
    }

    #[test]
    fn earliest_start_waits_for_latest_dependency() {
        let value_a =
            ClassicalValueId::new(ValueId::new(20));
        let value_b =
            ClassicalValueId::new(ValueId::new(21));

        let requirement_a =
            ReadinessRequirement::runtime(
                value_a,
                Duration::ZERO,
            );

        let requirement_b =
            ReadinessRequirement::runtime(
                value_b,
                Duration::ZERO,
            );

        let readiness = [
            DependencyReadiness::ready(
                requirement_a,
                TimePoint::new(10),
            ),
            DependencyReadiness::ready(
                requirement_b,
                TimePoint::new(30),
            ),
        ];

        assert_eq!(
            earliest_conditional_start(
                &readiness,
                TimePoint::ZERO,
            ),
            Some(TimePoint::new(30))
        );
    }
}