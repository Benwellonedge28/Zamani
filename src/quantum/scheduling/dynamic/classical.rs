//! Zamani Quantum Scheduling — Dynamic Classical Scheduling
//!
//! Production-ready classical dependency and readiness modelling for dynamic
//! quantum circuits.
//!
//! # Architectural role
//!
//! This module models the scheduler-visible portion of classical computation
//! that can affect the timing of quantum execution.
//!
//! It answers questions such as:
//!
//! - When is a measurement result available?
//! - When may a classical condition be evaluated?
//! - Which quantum operation depends on which classical result?
//! - What classical values must be available before a dynamic operation may
//!   start?
//! - How much target-supplied classical processing latency must be respected?
//! - Which runtime event releases a waiting operation?
//!
//! It does NOT:
//!
//! - parse Zamani source;
//! - define Zamani's classical language;
//! - define quantum measurement semantics;
//! - define another `QubitId`;
//! - define another `PhysicalQubitId`;
//! - define another `QuantumOperation`;
//! - define another `QuantumCircuit`;
//! - execute classical programs;
//! - execute quantum programs;
//! - discover hardware;
//! - communicate with a QPU;
//! - own a resource calendar;
//! - perform routing;
//! - perform QEC decoding;
//! - choose a scheduling algorithm.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Canonical quantum identity
//!
//! When classical events are associated with quantum measurements or qubit
//! state, canonical quantum identities MUST be imported from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This file deliberately does not define replacement qubit identifiers.
//!
//! # Dynamic scheduling model
//!
//! A dynamic quantum program is not necessarily a static DAG.
//!
//! Its execution can contain:
//!
//! ```text
//! quantum operation
//!       |
//!       v
//! measurement
//!       |
//!       v
//! classical result
//!       |
//!       v
//! classical computation
//!       |
//!       v
//! condition
//!       |
//!       v
//! quantum operation
//! ```
//!
//! Therefore classical dependencies are represented explicitly as scheduler
//! events and predicates.
//!
//! # Target independence
//!
//! No hardware timing value is hard-coded here.
//!
//! Classical latency is supplied by the target through the scheduler context.
//!
//! The module can therefore represent:
//!
//! - zero-latency classical decisions;
//! - deterministic processing latency;
//! - target-dependent processing latency;
//! - symbolic/unresolved latency;
//! - runtime-observed readiness;
//! - distributed classical communication.
//!
//! # Scalability
//!
//! There is no fixed maximum for:
//!
//! - classical signals;
//! - registers;
//! - predicates;
//! - dependencies;
//! - events;
//! - operands;
//! - qubits;
//! - operations;
//! - scheduling depth.
//!
//! "Infinity" means that this implementation introduces no artificial
//! machine-size ceiling. A concrete compilation remains bounded by available
//! memory, CPU time, explicit compiler limits, operating-system resources,
//! and target resources.
//!
//! # Determinism
//!
//! All public collections preserve insertion order unless an API explicitly
//! documents set semantics.
//!
//! No wall-clock time is consulted.
//! No global mutable state is used.
//! No implicit randomness is used.
//!
//! # Thread safety
//!
//! All state containers in this module are ordinary owned values.
//!
//! No interior mutability is required.
//!
//! The core data structures are therefore suitable for use from `Send + Sync`
//! scheduler components when their containing scheduler is itself thread-safe.
//!
//! # Safety
//!
//! Rust 1.97 / Rust 1.97.1.
//! Rust 2021.
//! Stable Rust.
//! No nightly features.
//! No unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

// ============================================================================
// Classical identity types
// ============================================================================

/// Stable identity for a scheduler-visible classical signal.
///
/// A signal is a semantic identity, not a physical memory address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassicalSignalId(u64);

impl ClassicalSignalId {
    /// Creates an identifier from an explicitly supplied value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns whether this identifier is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl From<u64> for ClassicalSignalId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<ClassicalSignalId> for u64 {
    fn from(value: ClassicalSignalId) -> Self {
        value.value()
    }
}

impl fmt::Display for ClassicalSignalId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "classical-signal:{}", self.0)
    }
}

/// Stable identity for a classical computation node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassicalNodeId(u64);

impl ClassicalNodeId {
    /// Creates a node identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl From<u64> for ClassicalNodeId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<ClassicalNodeId> for u64 {
    fn from(value: ClassicalNodeId) -> Self {
        value.value()
    }
}

impl fmt::Display for ClassicalNodeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "classical-node:{}", self.0)
    }
}

/// Stable identity for a classical predicate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PredicateId(u64);

impl PredicateId {
    /// Creates a predicate identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl From<u64> for PredicateId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<PredicateId> for u64 {
    fn from(value: PredicateId) -> Self {
        value.value()
    }
}

impl fmt::Display for PredicateId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "predicate:{}", self.0)
    }
}

/// Stable identity for a classical execution event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassicalEventId(u64);

impl ClassicalEventId {
    /// Creates an event identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl From<u64> for ClassicalEventId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<ClassicalEventId> for u64 {
    fn from(value: ClassicalEventId) -> Self {
        value.value()
    }
}

impl fmt::Display for ClassicalEventId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "classical-event:{}", self.0)
    }
}

// ============================================================================
// Classical values
// ============================================================================

/// A scheduler-visible classical value.
///
/// This is deliberately an execution value model rather than a programming
/// language AST. It allows dynamic scheduling to determine readiness without
/// owning the classical language implementation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ClassicalValue {
    /// A single Boolean value.
    Bool(bool),

    /// An unsigned integer.
    Unsigned(u128),

    /// A signed integer.
    Signed(i128),

    /// An opaque bit-vector represented in canonical big-endian byte order.
    Bits(Vec<u8>),

    /// A symbolic value whose concrete value is not yet known.
    Symbolic(String),
}

impl ClassicalValue {
    /// Returns whether the value is concretely known.
    #[must_use]
    pub fn is_known(&self) -> bool {
        !matches!(self, Self::Symbolic(_))
    }

    /// Returns whether the value is symbolic.
    #[must_use]
    pub fn is_symbolic(&self) -> bool {
        matches!(self, Self::Symbolic(_))
    }

    /// Returns a Boolean value when this is a Boolean.
    #[must_use]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns an unsigned integer when this is an unsigned integer.
    #[must_use]
    pub fn as_unsigned(&self) -> Option<u128> {
        match self {
            Self::Unsigned(value) => Some(*value),
            _ => None,
        }
    }
}

// ============================================================================
// Classical latency
// ============================================================================

/// Target-supplied classical readiness latency.
///
/// This type deliberately does not assume a particular unit.
///
/// The scheduler timing adapter is responsible for converting target timing
/// information into its canonical scheduling duration type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ClassicalLatency {
    /// Readiness occurs at the same scheduling instant as the producing event.
    Immediate,

    /// Readiness occurs after a target-supplied duration represented by an
    /// opaque scheduler duration.
    Duration(u128),

    /// The latency is known only at runtime.
    Runtime,

    /// The latency is symbolic and must be resolved by a later compilation or
    /// execution stage.
    Symbolic(String),
}

impl ClassicalLatency {
    /// Returns whether the latency is immediately available.
    #[must_use]
    pub const fn is_immediate(&self) -> bool {
        matches!(self, Self::Immediate)
    }

    /// Returns whether the latency must be resolved at runtime.
    #[must_use]
    pub const fn is_runtime(&self) -> bool {
        matches!(self, Self::Runtime)
    }

    /// Returns whether the latency is symbolic.
    #[must_use]
    pub const fn is_symbolic(&self) -> bool {
        matches!(self, Self::Symbolic(_))
    }
}

// ============================================================================
// Classical signal source
// ============================================================================

/// Describes where a classical signal originated.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ClassicalSignalSource {
    /// Result produced by a quantum measurement.
    Measurement {
        /// Logical qubits associated with the measurement.
        logical_qubits: Vec<QubitId>,

        /// Physical qubits associated with the measurement when known.
        physical_qubits: Vec<PhysicalQubitId>,
    },

    /// Result produced by a classical computation.
    Computation {
        /// Producing computation node.
        node: ClassicalNodeId,
    },

    /// Result supplied externally by the runtime.
    Runtime,

    /// Result supplied by a distributed classical communication event.
    Communication,

    /// User/plugin-defined source.
    Custom(String),
}

impl ClassicalSignalSource {
    /// Validates source identity consistency.
    pub fn validate(&self) -> Result<(), ClassicalModelError> {
        match self {
            Self::Measurement {
                logical_qubits,
                physical_qubits,
            } => {
                if logical_qubits.is_empty() {
                    return Err(ClassicalModelError::MeasurementWithoutLogicalQubit);
                }

                if has_duplicate_qubits(logical_qubits) {
                    return Err(ClassicalModelError::DuplicateLogicalQubit);
                }

                if has_duplicate_physical_qubits(physical_qubits) {
                    return Err(ClassicalModelError::DuplicatePhysicalQubit);
                }

                Ok(())
            }

            Self::Computation { .. }
            | Self::Runtime
            | Self::Communication
            | Self::Custom(_) => Ok(()),
        }
    }
}

// ============================================================================
// Classical signal
// ============================================================================

/// Scheduler-visible classical signal.
///
/// A signal represents availability of a classical value, not the semantics
/// of the program that produced it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassicalSignal {
    id: ClassicalSignalId,
    source: ClassicalSignalSource,
    latency: ClassicalLatency,
    value: Option<ClassicalValue>,
}

impl ClassicalSignal {
    /// Creates a classical signal.
    #[must_use]
    pub fn new(
        id: ClassicalSignalId,
        source: ClassicalSignalSource,
        latency: ClassicalLatency,
        value: Option<ClassicalValue>,
    ) -> Self {
        Self {
            id,
            source,
            latency,
            value,
        }
    }

    /// Returns the signal identifier.
    #[must_use]
    pub const fn id(&self) -> ClassicalSignalId {
        self.id
    }

    /// Returns the source.
    #[must_use]
    pub fn source(&self) -> &ClassicalSignalSource {
        &self.source
    }

    /// Returns the latency model.
    #[must_use]
    pub fn latency(&self) -> &ClassicalLatency {
        &self.latency
    }

    /// Returns the current value, if known.
    #[must_use]
    pub fn value(&self) -> Option<&ClassicalValue> {
        self.value.as_ref()
    }

    /// Returns whether a concrete value is currently available.
    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.value.as_ref().is_some_and(ClassicalValue::is_known)
    }

    /// Updates the value when a runtime event produces it.
    pub fn set_value(&mut self, value: ClassicalValue) {
        self.value = Some(value);
    }

    /// Validates the signal.
    pub fn validate(&self) -> Result<(), ClassicalModelError> {
        if self.id.is_zero() {
            return Err(ClassicalModelError::ZeroSignalId);
        }

        self.source.validate()
    }
}

// ============================================================================
// Classical operations
// ============================================================================

/// Scheduler-visible classical operation kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ClassicalOperationKind {
    /// Boolean conjunction.
    And,

    /// Boolean disjunction.
    Or,

    /// Boolean exclusive-or.
    Xor,

    /// Boolean negation.
    Not,

    /// Equality comparison.
    Equal,

    /// Inequality comparison.
    NotEqual,

    /// Unsigned less-than comparison.
    LessThan,

    /// Unsigned less-than-or-equal comparison.
    LessOrEqual,

    /// Unsigned greater-than comparison.
    GreaterThan,

    /// Unsigned greater-than-or-equal comparison.
    GreaterOrEqual,

    /// Generic reduction.
    Reduce,

    /// User/plugin-defined operation.
    Custom,
}

impl ClassicalOperationKind {
    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::And => "and",
            Self::Or => "or",
            Self::Xor => "xor",
            Self::Not => "not",
            Self::Equal => "equal",
            Self::NotEqual => "not_equal",
            Self::LessThan => "less_than",
            Self::LessOrEqual => "less_or_equal",
            Self::GreaterThan => "greater_than",
            Self::GreaterOrEqual => "greater_or_equal",
            Self::Reduce => "reduce",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for ClassicalOperationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Scheduler-visible classical computation node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassicalOperation {
    id: ClassicalNodeId,
    kind: ClassicalOperationKind,
    inputs: Vec<ClassicalSignalId>,
    outputs: Vec<ClassicalSignalId>,
    latency: ClassicalLatency,
}

impl ClassicalOperation {
    /// Creates a classical operation.
    #[must_use]
    pub fn new(
        id: ClassicalNodeId,
        kind: ClassicalOperationKind,
        inputs: Vec<ClassicalSignalId>,
        outputs: Vec<ClassicalSignalId>,
        latency: ClassicalLatency,
    ) -> Self {
        Self {
            id,
            kind,
            inputs,
            outputs,
            latency,
        }
    }

    /// Returns the operation identifier.
    #[must_use]
    pub const fn id(&self) -> ClassicalNodeId {
        self.id
    }

    /// Returns the operation kind.
    #[must_use]
    pub const fn kind(&self) -> ClassicalOperationKind {
        self.kind
    }

    /// Returns input signals.
    #[must_use]
    pub fn inputs(&self) -> &[ClassicalSignalId] {
        &self.inputs
    }

    /// Returns output signals.
    #[must_use]
    pub fn outputs(&self) -> &[ClassicalSignalId] {
        &self.outputs
    }

    /// Returns operation latency.
    #[must_use]
    pub fn latency(&self) -> &ClassicalLatency {
        &self.latency
    }

    /// Validates the operation.
    pub fn validate(&self) -> Result<(), ClassicalModelError> {
        if self.id.value() == 0 {
            return Err(ClassicalModelError::ZeroNodeId);
        }

        if self.outputs.is_empty() {
            return Err(ClassicalModelError::OperationWithoutOutput);
        }

        if has_duplicate_ids(&self.inputs) {
            return Err(ClassicalModelError::DuplicateInputSignal);
        }

        if has_duplicate_ids(&self.outputs) {
            return Err(ClassicalModelError::DuplicateOutputSignal);
        }

        Ok(())
    }
}

// ============================================================================
// Predicate
// ============================================================================

/// Predicate comparison operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum PredicateOperator {
    /// Equality.
    Equal,

    /// Inequality.
    NotEqual,

    /// Less than.
    LessThan,

    /// Less than or equal.
    LessOrEqual,

    /// Greater than.
    GreaterThan,

    /// Greater than or equal.
    GreaterOrEqual,

    /// Boolean conjunction.
    And,

    /// Boolean disjunction.
    Or,

    /// Boolean exclusive-or.
    Xor,

    /// Boolean negation.
    Not,
}

/// Operand used by a predicate.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PredicateOperand {
    /// Classical signal.
    Signal(ClassicalSignalId),

    /// Literal value.
    Value(ClassicalValue),

    /// Nested predicate.
    Predicate(PredicateId),
}

/// Scheduler-visible dynamic predicate.
///
/// It describes when a dynamic quantum operation becomes eligible. It does
/// not execute user code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Predicate {
    id: PredicateId,
    operator: PredicateOperator,
    operands: Vec<PredicateOperand>,
}

impl Predicate {
    /// Creates a predicate.
    #[must_use]
    pub fn new(
        id: PredicateId,
        operator: PredicateOperator,
        operands: Vec<PredicateOperand>,
    ) -> Self {
        Self {
            id,
            operator,
            operands,
        }
    }

    /// Returns the predicate identifier.
    #[must_use]
    pub const fn id(&self) -> PredicateId {
        self.id
    }

    /// Returns the predicate operator.
    #[must_use]
    pub const fn operator(&self) -> PredicateOperator {
        self.operator
    }

    /// Returns predicate operands.
    #[must_use]
    pub fn operands(&self) -> &[PredicateOperand] {
        &self.operands
    }

    /// Returns all direct classical signal dependencies.
    #[must_use]
    pub fn signal_dependencies(&self) -> Vec<ClassicalSignalId> {
        let mut result = BTreeSet::new();

        for operand in &self.operands {
            if let PredicateOperand::Signal(signal) = operand {
                result.insert(*signal);
            }
        }

        result.into_iter().collect()
    }

    /// Validates the predicate shape.
    pub fn validate(&self) -> Result<(), ClassicalModelError> {
        if self.id.value() == 0 {
            return Err(ClassicalModelError::ZeroPredicateId);
        }

        if self.operands.is_empty() {
            return Err(ClassicalModelError::PredicateWithoutOperands);
        }

        Ok(())
    }
}

// ============================================================================
// Dynamic readiness state
// ============================================================================

/// Readiness state for a dynamic operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ClassicalReadiness {
    /// Required classical information is not available.
    Waiting,

    /// Required classical information is available but the target-supplied
    /// processing latency has not elapsed.
    Processing,

    /// The operation is eligible from the classical dependency perspective.
    Ready,

    /// A predicate evaluated to false.
    PredicateFalse,

    /// A runtime condition prevents static determination.
    RuntimeDependent,
}

impl ClassicalReadiness {
    /// Returns whether the classical dependency is currently ready.
    #[must_use]
    pub const fn is_ready(self) -> bool {
        matches!(self, Self::Ready)
    }

    /// Returns whether the operation is permanently blocked by a false
    /// predicate.
    #[must_use]
    pub const fn is_predicate_false(self) -> bool {
        matches!(self, Self::PredicateFalse)
    }
}

// ============================================================================
// Classical dependency
// ============================================================================

/// Dependency from a classical signal to a scheduler-visible consumer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassicalDependency {
    signal: ClassicalSignalId,
    predicate: Option<PredicateId>,
    required: bool,
}

impl ClassicalDependency {
    /// Creates a dependency.
    #[must_use]
    pub fn new(
        signal: ClassicalSignalId,
        predicate: Option<PredicateId>,
        required: bool,
    ) -> Self {
        Self {
            signal,
            predicate,
            required,
        }
    }

    /// Returns the source signal.
    #[must_use]
    pub const fn signal(&self) -> ClassicalSignalId {
        self.signal
    }

    /// Returns the optional predicate.
    #[must_use]
    pub const fn predicate(&self) -> Option<PredicateId> {
        self.predicate
    }

    /// Returns whether the signal is required.
    #[must_use]
    pub const fn required(&self) -> bool {
        self.required
    }
}

// ============================================================================
// Classical readiness request
// ============================================================================

/// A request from the dynamic scheduler to determine whether an operation can
/// proceed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassicalReadinessRequest {
    dependencies: Vec<ClassicalDependency>,
    predicate: Option<PredicateId>,
}

impl ClassicalReadinessRequest {
    /// Creates a readiness request.
    #[must_use]
    pub fn new(
        dependencies: Vec<ClassicalDependency>,
        predicate: Option<PredicateId>,
    ) -> Self {
        Self {
            dependencies,
            predicate,
        }
    }

    /// Returns dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &[ClassicalDependency] {
        &self.dependencies
    }

    /// Returns the top-level predicate.
    #[must_use]
    pub const fn predicate(&self) -> Option<PredicateId> {
        self.predicate
    }

    /// Returns all signal IDs required by the request.
    #[must_use]
    pub fn signal_ids(&self) -> Vec<ClassicalSignalId> {
        let mut ids = BTreeSet::new();

        for dependency in &self.dependencies {
            ids.insert(dependency.signal());
        }

        ids.into_iter().collect()
    }

    /// Validates the request.
    pub fn validate(&self) -> Result<(), ClassicalModelError> {
        let mut signals = BTreeSet::new();

        for dependency in &self.dependencies {
            if !signals.insert(dependency.signal()) {
                return Err(ClassicalModelError::DuplicateDependency);
            }
        }

        Ok(())
    }
}

// ============================================================================
// Classical state
// ============================================================================

/// Immutable-by-convention view of scheduler-visible classical state.
///
/// The state is owned by the dynamic scheduler and can be cloned to create
/// snapshots for speculative evaluation.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ClassicalState {
    signals: BTreeMap<ClassicalSignalId, ClassicalSignal>,
    operations: BTreeMap<ClassicalNodeId, ClassicalOperation>,
    predicates: BTreeMap<PredicateId, Predicate>,
    events: BTreeMap<ClassicalEventId, ClassicalEvent>,
}

impl ClassicalState {
    /// Creates an empty classical state.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a classical signal.
    pub fn insert_signal(
        &mut self,
        signal: ClassicalSignal,
    ) -> Result<(), ClassicalModelError> {
        signal.validate()?;

        if self.signals.contains_key(&signal.id()) {
            return Err(ClassicalModelError::DuplicateSignal {
                id: signal.id(),
            });
        }

        self.signals.insert(signal.id(), signal);
        Ok(())
    }

    /// Inserts a classical operation.
    pub fn insert_operation(
        &mut self,
        operation: ClassicalOperation,
    ) -> Result<(), ClassicalModelError> {
        operation.validate()?;

        if self.operations.contains_key(&operation.id()) {
            return Err(ClassicalModelError::DuplicateOperation {
                id: operation.id(),
            });
        }

        self.operations.insert(operation.id(), operation);
        Ok(())
    }

    /// Inserts a predicate.
    pub fn insert_predicate(
        &mut self,
        predicate: Predicate,
    ) -> Result<(), ClassicalModelError> {
        predicate.validate()?;

        if self.predicates.contains_key(&predicate.id()) {
            return Err(ClassicalModelError::DuplicatePredicate {
                id: predicate.id(),
            });
        }

        self.predicates.insert(predicate.id(), predicate);
        Ok(())
    }

    /// Inserts a runtime classical event.
    pub fn insert_event(
        &mut self,
        event: ClassicalEvent,
    ) -> Result<(), ClassicalModelError> {
        event.validate()?;

        if self.events.contains_key(&event.id()) {
            return Err(ClassicalModelError::DuplicateEvent {
                id: event.id(),
            });
        }

        self.events.insert(event.id(), event);
        Ok(())
    }

    /// Returns a signal.
    #[must_use]
    pub fn signal(&self, id: ClassicalSignalId) -> Option<&ClassicalSignal> {
        self.signals.get(&id)
    }

    /// Returns a mutable signal.
    pub fn signal_mut(
        &mut self,
        id: ClassicalSignalId,
    ) -> Option<&mut ClassicalSignal> {
        self.signals.get_mut(&id)
    }

    /// Returns a classical operation.
    #[must_use]
    pub fn operation(&self, id: ClassicalNodeId) -> Option<&ClassicalOperation> {
        self.operations.get(&id)
    }

    /// Returns a predicate.
    #[must_use]
    pub fn predicate(&self, id: PredicateId) -> Option<&Predicate> {
        self.predicates.get(&id)
    }

    /// Returns an event.
    #[must_use]
    pub fn event(&self, id: ClassicalEventId) -> Option<&ClassicalEvent> {
        self.events.get(&id)
    }

    /// Returns all signals in deterministic order.
    #[must_use]
    pub fn signals(&self) -> impl Iterator<Item = &ClassicalSignal> {
        self.signals.values()
    }

    /// Returns all predicates in deterministic order.
    #[must_use]
    pub fn predicates(&self) -> impl Iterator<Item = &Predicate> {
        self.predicates.values()
    }

    /// Marks a signal as produced by assigning its runtime value.
    pub fn publish(
        &mut self,
        id: ClassicalSignalId,
        value: ClassicalValue,
    ) -> Result<(), ClassicalModelError> {
        let signal = self
            .signal_mut(id)
            .ok_or(ClassicalModelError::UnknownSignal { id })?;

        signal.set_value(value);
        Ok(())
    }

    /// Evaluates classical readiness for a request.
    #[must_use]
    pub fn readiness(
        &self,
        request: &ClassicalReadinessRequest,
    ) -> ClassicalReadiness {
        if request.validate().is_err() {
            return ClassicalReadiness::Waiting;
        }

        for dependency in request.dependencies() {
            match self.signal(dependency.signal()) {
                Some(signal) if signal.is_ready() => {}
                Some(_) if dependency.required() => {
                    return ClassicalReadiness::Waiting;
                }
                Some(_) => {}
                None => return ClassicalReadiness::Waiting,
            }
        }

        if let Some(predicate_id) = request.predicate() {
            let Some(predicate) = self.predicate(predicate_id) else {
                return ClassicalReadiness::Waiting;
            };

            match self.evaluate_predicate(predicate) {
                PredicateEvaluation::True => ClassicalReadiness::Ready,
                PredicateEvaluation::False => ClassicalReadiness::PredicateFalse,
                PredicateEvaluation::Unknown => ClassicalReadiness::Waiting,
                PredicateEvaluation::RuntimeDependent => {
                    ClassicalReadiness::RuntimeDependent
                }
            }
        } else {
            ClassicalReadiness::Ready
        }
    }

    /// Evaluates a predicate using currently available concrete values.
    #[must_use]
    pub fn evaluate_predicate(
        &self,
        predicate: &Predicate,
    ) -> PredicateEvaluation {
        evaluate_predicate(predicate, self)
    }

    /// Returns the number of signals currently represented.
    #[must_use]
    pub fn signal_count(&self) -> usize {
        self.signals.len()
    }

    /// Returns the number of classical computation nodes.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Returns the number of predicates.
    #[must_use]
    pub fn predicate_count(&self) -> usize {
        self.predicates.len()
    }

    /// Returns the number of runtime events.
    #[must_use]
    pub fn event_count(&self) -> usize {
        self.events.len()
    }
}

// ============================================================================
// Predicate evaluation
// ============================================================================

/// Result of scheduler-level predicate evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PredicateEvaluation {
    /// Predicate is true.
    True,

    /// Predicate is false.
    False,

    /// Required values are not yet available.
    Unknown,

    /// Evaluation depends on runtime state that cannot be statically
    /// determined.
    RuntimeDependent,
}

// ============================================================================
// Runtime events
// ============================================================================

/// Kind of runtime event that can affect classical readiness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ClassicalEventKind {
    /// Measurement result became available.
    MeasurementResult,

    /// Classical computation completed.
    ComputationComplete,

    /// Distributed classical message arrived.
    CommunicationComplete,

    /// External runtime input became available.
    ExternalInput,

    /// Custom event.
    Custom,
}

/// Scheduler-visible classical event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassicalEvent {
    id: ClassicalEventId,
    kind: ClassicalEventKind,
    produced_signals: Vec<ClassicalSignalId>,
}

impl ClassicalEvent {
    /// Creates a classical event.
    #[must_use]
    pub fn new(
        id: ClassicalEventId,
        kind: ClassicalEventKind,
        produced_signals: Vec<ClassicalSignalId>,
    ) -> Self {
        Self {
            id,
            kind,
            produced_signals,
        }
    }

    /// Returns the event identifier.
    #[must_use]
    pub const fn id(&self) -> ClassicalEventId {
        self.id
    }

    /// Returns the event kind.
    #[must_use]
    pub const fn kind(&self) -> ClassicalEventKind {
        self.kind
    }

    /// Returns produced signals.
    #[must_use]
    pub fn produced_signals(&self) -> &[ClassicalSignalId] {
        &self.produced_signals
    }

    /// Validates the event.
    pub fn validate(&self) -> Result<(), ClassicalModelError> {
        if self.id.value() == 0 {
            return Err(ClassicalModelError::ZeroEventId);
        }

        if has_duplicate_ids(&self.produced_signals) {
            return Err(ClassicalModelError::DuplicateEventSignal);
        }

        Ok(())
    }
}

// ============================================================================
// Evaluation implementation
// ============================================================================

fn evaluate_predicate(
    predicate: &Predicate,
    state: &ClassicalState,
) -> PredicateEvaluation {
    match predicate.operator() {
        PredicateOperator::Not => {
            if predicate.operands().len() != 1 {
                return PredicateEvaluation::Unknown;
            }

            match resolve_bool_operand(&predicate.operands()[0], state) {
                Some(value) => {
                    if !value {
                        PredicateEvaluation::True
                    } else {
                        PredicateEvaluation::False
                    }
                }
                None => PredicateEvaluation::Unknown,
            }
        }

        PredicateOperator::And => {
            let mut unknown = false;

            for operand in predicate.operands() {
                match resolve_bool_operand(operand, state) {
                    Some(false) => return PredicateEvaluation::False,
                    Some(true) => {}
                    None => unknown = true,
                }
            }

            if unknown {
                PredicateEvaluation::Unknown
            } else {
                PredicateEvaluation::True
            }
        }

        PredicateOperator::Or => {
            let mut unknown = false;

            for operand in predicate.operands() {
                match resolve_bool_operand(operand, state) {
                    Some(true) => return PredicateEvaluation::True,
                    Some(false) => {}
                    None => unknown = true,
                }
            }

            if unknown {
                PredicateEvaluation::Unknown
            } else {
                PredicateEvaluation::False
            }
        }

        PredicateOperator::Xor => {
            let mut value = false;

            for operand in predicate.operands() {
                match resolve_bool_operand(operand, state) {
                    Some(next) => value ^= next,
                    None => return PredicateEvaluation::Unknown,
                }
            }

            if value {
                PredicateEvaluation::True
            } else {
                PredicateEvaluation::False
            }
        }

        PredicateOperator::Equal
        | PredicateOperator::NotEqual
        | PredicateOperator::LessThan
        | PredicateOperator::LessOrEqual
        | PredicateOperator::GreaterThan
        | PredicateOperator::GreaterOrEqual => {
            if predicate.operands().len() != 2 {
                return PredicateEvaluation::Unknown;
            }

            let left = resolve_value_operand(&predicate.operands()[0], state);
            let right = resolve_value_operand(&predicate.operands()[1], state);

            let (Some(left), Some(right)) = (left, right) else {
                return PredicateEvaluation::Unknown;
            };

            let ordering = compare_values(left, right);

            let result = match predicate.operator() {
                PredicateOperator::Equal => ordering == Some(std::cmp::Ordering::Equal),

                PredicateOperator::NotEqual => {
                    ordering != Some(std::cmp::Ordering::Equal)
                }

                PredicateOperator::LessThan => {
                    ordering == Some(std::cmp::Ordering::Less)
                }

                PredicateOperator::LessOrEqual => {
                    matches!(
                        ordering,
                        Some(
                            std::cmp::Ordering::Less
                                | std::cmp::Ordering::Equal
                        )
                    )
                }

                PredicateOperator::GreaterThan => {
                    ordering == Some(std::cmp::Ordering::Greater)
                }

                PredicateOperator::GreaterOrEqual => {
                    matches!(
                        ordering,
                        Some(
                            std::cmp::Ordering::Greater
                                | std::cmp::Ordering::Equal
                        )
                    )
                }

                _ => return PredicateEvaluation::Unknown,
            };

            if result {
                PredicateEvaluation::True
            } else {
                PredicateEvaluation::False
            }
        }
    }
}

fn resolve_bool_operand(
    operand: &PredicateOperand,
    state: &ClassicalState,
) -> Option<bool> {
    match operand {
        PredicateOperand::Value(ClassicalValue::Bool(value)) => Some(*value),

        PredicateOperand::Signal(id) => state.signal(*id)?.value()?.as_bool(),

        PredicateOperand::Predicate(id) => {
            match state.predicate(*id).map(|predicate| {
                state.evaluate_predicate(predicate)
            }) {
                Some(PredicateEvaluation::True) => Some(true),
                Some(PredicateEvaluation::False) => Some(false),
                _ => None,
            }
        }

        PredicateOperand::Value(_) => None,
    }
}

fn resolve_value_operand<'a>(
    operand: &'a PredicateOperand,
    state: &'a ClassicalState,
) -> Option<&'a ClassicalValue> {
    match operand {
        PredicateOperand::Value(value) => Some(value),

        PredicateOperand::Signal(id) => state.signal(*id)?.value(),

        PredicateOperand::Predicate(_) => None,
    }
}

fn compare_values(
    left: &ClassicalValue,
    right: &ClassicalValue,
) -> Option<std::cmp::Ordering> {
    match (left, right) {
        (ClassicalValue::Bool(a), ClassicalValue::Bool(b)) => Some(a.cmp(b)),

        (ClassicalValue::Unsigned(a), ClassicalValue::Unsigned(b)) => {
            Some(a.cmp(b))
        }

        (ClassicalValue::Signed(a), ClassicalValue::Signed(b)) => {
            Some(a.cmp(b))
        }

        (ClassicalValue::Bits(a), ClassicalValue::Bits(b)) => {
            Some(a.cmp(b))
        }

        (ClassicalValue::Symbolic(_), _)
        | (_, ClassicalValue::Symbolic(_)) => None,

        _ => None,
    }
}

// ============================================================================
// Validation helpers
// ============================================================================

fn has_duplicate_ids(ids: &[ClassicalSignalId]) -> bool {
    let mut seen = BTreeSet::new();

    ids.iter().any(|id| !seen.insert(*id))
}

fn has_duplicate_qubits(qubits: &[QubitId]) -> bool {
    let mut seen = BTreeSet::new();

    qubits.iter().any(|qubit| !seen.insert(*qubit))
}

fn has_duplicate_physical_qubits(qubits: &[PhysicalQubitId]) -> bool {
    let mut seen = BTreeSet::new();

    qubits.iter().any(|qubit| !seen.insert(*qubit))
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by the dynamic classical scheduler model.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ClassicalModelError {
    /// A signal identifier was zero.
    ZeroSignalId,

    /// A classical node identifier was zero.
    ZeroNodeId,

    /// A predicate identifier was zero.
    ZeroPredicateId,

    /// A runtime event identifier was zero.
    ZeroEventId,

    /// A measurement source had no logical qubits.
    MeasurementWithoutLogicalQubit,

    /// A logical qubit appeared more than once.
    DuplicateLogicalQubit,

    /// A physical qubit appeared more than once.
    DuplicatePhysicalQubit,

    /// A signal appeared more than once.
    DuplicateSignal {
        /// Duplicated signal.
        id: ClassicalSignalId,
    },

    /// A computation operation appeared more than once.
    DuplicateOperation {
        /// Duplicated operation.
        id: ClassicalNodeId,
    },

    /// A predicate appeared more than once.
    DuplicatePredicate {
        /// Duplicated predicate.
        id: PredicateId,
    },

    /// A runtime event appeared more than once.
    DuplicateEvent {
        /// Duplicated event.
        id: ClassicalEventId,
    },

    /// A classical operation has no output.
    OperationWithoutOutput,

    /// A classical operation repeats an input signal.
    DuplicateInputSignal,

    /// A classical operation repeats an output signal.
    DuplicateOutputSignal,

    /// A predicate has no operands.
    PredicateWithoutOperands,

    /// A readiness request repeats a dependency.
    DuplicateDependency,

    /// A runtime event repeats a produced signal.
    DuplicateEventSignal,

    /// A referenced signal does not exist.
    UnknownSignal {
        /// Unknown signal.
        id: ClassicalSignalId,
    },
}

impl fmt::Display for ClassicalModelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroSignalId => {
                formatter.write_str("classical signal identifier cannot be zero")
            }

            Self::ZeroNodeId => {
                formatter.write_str("classical node identifier cannot be zero")
            }

            Self::ZeroPredicateId => {
                formatter.write_str("predicate identifier cannot be zero")
            }

            Self::ZeroEventId => {
                formatter.write_str("classical event identifier cannot be zero")
            }

            Self::MeasurementWithoutLogicalQubit => formatter.write_str(
                "measurement classical source must reference at least one logical qubit",
            ),

            Self::DuplicateLogicalQubit => {
                formatter.write_str("measurement source contains a duplicate logical qubit")
            }

            Self::DuplicatePhysicalQubit => {
                formatter.write_str("measurement source contains a duplicate physical qubit")
            }

            Self::DuplicateSignal { id } => {
                write!(formatter, "classical signal {id} already exists")
            }

            Self::DuplicateOperation { id } => {
                write!(formatter, "classical operation {id} already exists")
            }

            Self::DuplicatePredicate { id } => {
                write!(formatter, "predicate {id} already exists")
            }

            Self::DuplicateEvent { id } => {
                write!(formatter, "classical event {id} already exists")
            }

            Self::OperationWithoutOutput => {
                formatter.write_str("classical operation must produce at least one output")
            }

            Self::DuplicateInputSignal => {
                formatter.write_str("classical operation contains a duplicate input signal")
            }

            Self::DuplicateOutputSignal => {
                formatter.write_str("classical operation contains a duplicate output signal")
            }

            Self::PredicateWithoutOperands => {
                formatter.write_str("classical predicate must contain at least one operand")
            }

            Self::DuplicateDependency => {
                formatter.write_str("readiness request contains a duplicate dependency")
            }

            Self::DuplicateEventSignal => {
                formatter.write_str("classical event contains a duplicate produced signal")
            }

            Self::UnknownSignal { id } => {
                write!(formatter, "unknown classical signal {id}")
            }
        }
    }
}

impl std::error::Error for ClassicalModelError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn signal(id: u64, value: Option<ClassicalValue>) -> ClassicalSignal {
        ClassicalSignal::new(
            ClassicalSignalId::new(id),
            ClassicalSignalSource::Runtime,
            ClassicalLatency::Immediate,
            value,
        )
    }

    #[test]
    fn signal_identity_is_stable() {
        let id = ClassicalSignalId::new(42);

        assert_eq!(id.value(), 42);
        assert!(!id.is_zero());
        assert_eq!(id.to_string(), "classical-signal:42");
    }

    #[test]
    fn boolean_predicate_becomes_ready() {
        let mut state = ClassicalState::new();

        state
            .insert_signal(signal(1, Some(ClassicalValue::Bool(true))))
            .expect("signal insertion must succeed");

        state
            .insert_predicate(Predicate::new(
                PredicateId::new(1),
                PredicateOperator::Equal,
                vec![
                    PredicateOperand::Signal(ClassicalSignalId::new(1)),
                    PredicateOperand::Value(ClassicalValue::Bool(true)),
                ],
            ))
            .expect("predicate insertion must succeed");

        let request = ClassicalReadinessRequest::new(
            vec![ClassicalDependency::new(
                ClassicalSignalId::new(1),
                Some(PredicateId::new(1)),
                true,
            )],
            Some(PredicateId::new(1)),
        );

        assert_eq!(
            state.readiness(&request),
            ClassicalReadiness::Ready
        );
    }

    #[test]
    fn unavailable_signal_blocks_readiness() {
        let state = ClassicalState::new();

        let request = ClassicalReadinessRequest::new(
            vec![ClassicalDependency::new(
                ClassicalSignalId::new(99),
                None,
                true,
            )],
            None,
        );

        assert_eq!(
            state.readiness(&request),
            ClassicalReadiness::Waiting
        );
    }

    #[test]
    fn false_predicate_blocks_dynamic_operation() {
        let mut state = ClassicalState::new();

        state
            .insert_signal(signal(1, Some(ClassicalValue::Bool(false))))
            .expect("signal insertion must succeed");

        state
            .insert_predicate(Predicate::new(
                PredicateId::new(1),
                PredicateOperator::Equal,
                vec![
                    PredicateOperand::Signal(ClassicalSignalId::new(1)),
                    PredicateOperand::Value(ClassicalValue::Bool(true)),
                ],
            ))
            .expect("predicate insertion must succeed");

        let request = ClassicalReadinessRequest::new(
            vec![ClassicalDependency::new(
                ClassicalSignalId::new(1),
                Some(PredicateId::new(1)),
                true,
            )],
            Some(PredicateId::new(1)),
        );

        assert_eq!(
            state.readiness(&request),
            ClassicalReadiness::PredicateFalse
        );
    }

    #[test]
    fn publishing_signal_releases_waiting_dependency() {
        let mut state = ClassicalState::new();

        state
            .insert_signal(signal(1, None))
            .expect("signal insertion must succeed");

        let request = ClassicalReadinessRequest::new(
            vec![ClassicalDependency::new(
                ClassicalSignalId::new(1),
                None,
                true,
            )],
            None,
        );

        assert_eq!(
            state.readiness(&request),
            ClassicalReadiness::Waiting
        );

        state
            .publish(ClassicalSignalId::new(1), ClassicalValue::Bool(true))
            .expect("publishing must succeed");

        assert_eq!(
            state.readiness(&request),
            ClassicalReadiness::Ready
        );
    }

    #[test]
    fn conjunction_requires_all_inputs() {
        let mut state = ClassicalState::new();

        state
            .insert_signal(signal(1, Some(ClassicalValue::Bool(true))))
            .expect("signal insertion must succeed");

        state
            .insert_signal(signal(2, Some(ClassicalValue::Bool(false))))
            .expect("signal insertion must succeed");

        state
            .insert_predicate(Predicate::new(
                PredicateId::new(1),
                PredicateOperator::And,
                vec![
                    PredicateOperand::Signal(ClassicalSignalId::new(1)),
                    PredicateOperand::Signal(ClassicalSignalId::new(2)),
                ],
            ))
            .expect("predicate insertion must succeed");

        assert_eq!(
            state.evaluate_predicate(
                state
                    .predicate(PredicateId::new(1))
                    .expect("predicate must exist")
            ),
            PredicateEvaluation::False
        );
    }

    #[test]
    fn symbolic_values_remain_unknown() {
        let mut state = ClassicalState::new();

        state
            .insert_signal(signal(
                1,
                Some(ClassicalValue::Symbolic("runtime_value".into())),
            ))
            .expect("signal insertion must succeed");

        state
            .insert_predicate(Predicate::new(
                PredicateId::new(1),
                PredicateOperator::Equal,
                vec![
                    PredicateOperand::Signal(ClassicalSignalId::new(1)),
                    PredicateOperand::Value(ClassicalValue::Unsigned(1)),
                ],
            ))
            .expect("predicate insertion must succeed");

        assert_eq!(
            state.evaluate_predicate(
                state
                    .predicate(PredicateId::new(1))
                    .expect("predicate must exist")
            ),
            PredicateEvaluation::Unknown
        );
    }

    #[test]
    fn duplicate_signal_is_rejected() {
        let mut state = ClassicalState::new();

        state
            .insert_signal(signal(1, None))
            .expect("first insertion must succeed");

        let error = state
            .insert_signal(signal(1, None))
            .expect_err("duplicate must fail");

        assert!(matches!(
            error,
            ClassicalModelError::DuplicateSignal { .. }
        ));
    }

    #[test]
    fn measurement_source_uses_canonical_qubit_identity() {
        let source = ClassicalSignalSource::Measurement {
            logical_qubits: vec![QubitId::new(1)],
            physical_qubits: vec![PhysicalQubitId::new(7)],
        };

        assert!(source.validate().is_ok());
    }

    #[test]
    fn event_validation_rejects_duplicate_outputs() {
        let event = ClassicalEvent::new(
            ClassicalEventId::new(1),
            ClassicalEventKind::MeasurementResult,
            vec![
                ClassicalSignalId::new(2),
                ClassicalSignalId::new(2),
            ],
        );

        assert!(matches!(
            event.validate(),
            Err(ClassicalModelError::DuplicateEventSignal)
        ));
    }

    #[test]
    fn state_is_deterministically_ordered() {
        let mut state = ClassicalState::new();

        state
            .insert_signal(signal(3, None))
            .expect("insertion must succeed");

        state
            .insert_signal(signal(1, None))
            .expect("insertion must succeed");

        state
            .insert_signal(signal(2, None))
            .expect("insertion must succeed");

        let ids: Vec<_> = state.signals().map(|signal| signal.id().value()).collect();

        assert_eq!(ids, vec![1, 2, 3]);
    }

    #[test]
    fn state_counts_scale_without_fixed_capacity() {
        let mut state = ClassicalState::new();

        for id in 1..=1024 {
            state
                .insert_signal(signal(id, None))
                .expect("signal insertion must succeed");
        }

        assert_eq!(state.signal_count(), 1024);
    }
}