//! Zamani Quantum Scheduling — QEC Scheduling Interface
//!
//! Stable, hardware-independent interface between quantum error-correction
//! planning and the generic quantum scheduler.
//!
//! # Architectural responsibility
//!
//! This module defines:
//!
//! - the information QEC must provide to the scheduler;
//! - the operations that participate in a QEC schedule;
//! - logical and optional physical qubit references;
//! - QEC round identity;
//! - syndrome identity;
//! - precedence/dependency requirements;
//! - synchronization requirements;
//! - classical-feedback requirements;
//! - QEC scheduling phases;
//! - structured validation errors;
//! - an implementation-independent provider trait.
//!
//! This module does NOT implement:
//!
//! - a scheduling algorithm;
//! - ASAP scheduling;
//! - ALAP scheduling;
//! - list scheduling;
//! - resource-constrained scheduling;
//! - routing;
//! - physical-qubit allocation;
//! - hardware discovery;
//! - hardware calibration;
//! - timing arithmetic;
//! - pulse generation;
//! - QEC decoding;
//! - surface-code geometry;
//! - a particular QEC code;
//! - simulator execution;
//! - backend execution.
//!
//! Those responsibilities belong to other subsystems.
//!
//! # Fundamental architectural boundary
//!
//! ```text
//!                 quantum::ir
//!                      │
//!                      ▼
//!             logical quantum program
//!                      │
//!                      ▼
//!                 QEC planning
//!                      │
//!                      ▼
//!        qec::interface (this module)
//!                      │
//!             QEC scheduling request
//!                      │
//!                      ▼
//!          generic scheduling engine
//!             │       │       │
//!             ▼       ▼       ▼
//!          timing  resources constraints
//!             │       │       │
//!             └───────┼───────┘
//!                     ▼
//!                   route
//!                     │
//!                     ▼
//!                 hardware
//! ```
//!
//! The QEC interface therefore describes scheduling requirements without
//! deciding when or where an operation executes.
//!
//! # Write once, scale everywhere
//!
//! Nothing in this module assumes:
//!
//! - a fixed number of qubits;
//! - a fixed number of ancillas;
//! - a fixed number of stabilizers;
//! - a fixed number of QEC rounds;
//! - a fixed code distance;
//! - a fixed stabilizer weight;
//! - a fixed topology;
//! - a fixed gate arity;
//! - a fixed number of measurement channels;
//! - a fixed number of control channels;
//! - a particular QEC code;
//! - a particular quantum technology;
//! - a particular hardware vendor.
//!
//! A concrete compilation remains finite because its input, host resources and
//! execution target are finite. "Infinity" therefore means that this module
//! introduces no artificial architectural ceiling.
//!
//! # Canonical qubit identity
//!
//! The authoritative qubit types are:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module deliberately imports those exact canonical types.
//!
//! It MUST NOT define another `QubitId`, `PhysicalQubitId`, logical-qubit
//! wrapper, or physical-qubit wrapper that could become a competing identity
//! system.
//!
//! Logical-to-physical mapping remains the responsibility of routing.
//!
//! # Dependency direction
//!
//! This module may depend on:
//!
//! - the Rust standard library;
//! - canonical quantum IR qubit identities.
//!
//! This module must not depend on:
//!
//! - scheduling algorithms;
//! - scheduling planners;
//! - hardware implementations;
//! - routing implementations;
//! - backend SDKs;
//! - runtime implementations;
//! - vendor libraries;
//! - QEC decoders.
//!
//! Downstream scheduling modules may depend on this interface.
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
//! - standard library only apart from Zamani's canonical IR dependency.
//!
//! The no-unsafe requirement is compiler-enforced.
//!
//! # Integration contract
//!
//! Future modules consume this interface as follows:
//!
//! ```text
//! qec/stabilizer.rs
//!     │
//!     ├── creates QecSchedulingRequest
//!     └── returns it to the generic scheduler
//!
//! qec/syndrome.rs
//!     │
//!     └── supplies syndrome/measurement dependencies
//!
//! qec/rounds.rs
//!     │
//!     └── supplies round structure
//!
//! scheduling/ir/operation.rs
//!     │
//!     └── converts QecOperation into scheduler operations
//!
//! scheduling/constraints/*
//!     │
//!     └── converts QEC requirements into scheduler constraints
//!
//! scheduling/planners/*
//!     │
//!     └── chooses execution times
//!
//! scheduling/verification/*
//!     │
//!     └── verifies the resulting schedule
//! ```
//!
//! None of those modules need to modify this file merely because their
//! implementations are added later.
//!
//! # Important semantic rule
//!
//! QEC planning and scheduling are different operations.
//!
//! QEC planning answers:
//!
//! > What fault-tolerance operations and dependencies are required?
//!
//! Scheduling answers:
//!
//! > When can those operations execute while satisfying all dependencies,
//! > resources, timing requirements and target constraints?
//!
//! This interface deliberately preserves that distinction.

// ============================================================================
// Safety boundary
// ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

// ============================================================================
// Stable QEC identifiers
// ============================================================================

/// Stable identifier for a QEC round.
///
/// A round identifier has meaning only inside the QEC scheduling request in
/// which it occurs. It is not a hardware identifier and does not imply a
/// particular number of rounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QecRoundId(usize);

impl QecRoundId {
    /// Creates a QEC round identifier.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the underlying identifier.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }

    /// Returns the next representable identifier without overflowing.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<usize> for QecRoundId {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl From<QecRoundId> for usize {
    fn from(value: QecRoundId) -> Self {
        value.index()
    }
}

impl fmt::Display for QecRoundId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "qec-round-{}", self.0)
    }
}

/// Stable identifier for a QEC operation.
///
/// This is distinct from `quantum::ir` operation identity. The QEC planner
/// owns this identity inside the QEC scheduling request, while the original
/// IR operation identity remains owned by the canonical IR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QecOperationId(usize);

impl QecOperationId {
    /// Creates a QEC operation identifier.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the underlying identifier.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }

    /// Returns the next representable identifier without overflowing.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<usize> for QecOperationId {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl From<QecOperationId> for usize {
    fn from(value: QecOperationId) -> Self {
        value.index()
    }
}

impl fmt::Display for QecOperationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "qec-op-{}", self.0)
    }
}

/// Stable identifier for syndrome information.
///
/// A syndrome identity is semantic QEC metadata. It does not represent a
/// classical register index, hardware measurement channel, or decoder memory
/// address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SyndromeId(usize);

impl SyndromeId {
    /// Creates a syndrome identifier.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the underlying identifier.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }

    /// Returns the next representable identifier without overflowing.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<usize> for SyndromeId {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl From<SyndromeId> for usize {
    fn from(value: SyndromeId) -> Self {
        value.index()
    }
}

impl fmt::Display for SyndromeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "syndrome-{}", self.0)
    }
}

// ============================================================================
// Qubit references
// ============================================================================

/// Explicit QEC qubit reference.
///
/// Logical identity is authoritative in the canonical IR. Physical identity
/// is supplied only when a previous mapping/routing stage has already
/// established one.
///
/// The scheduler must never infer a physical qubit merely from a logical
/// qubit's numeric value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QecQubit {
    /// Logical qubit identity from the canonical quantum IR.
    Logical(QubitId),

    /// Physical qubit identity from the canonical quantum IR vocabulary.
    ///
    /// The presence of this value does not by itself guarantee that the target
    /// hardware currently provides or permits that physical resource.
    Physical(PhysicalQubitId),
}

impl QecQubit {
    /// Creates a logical QEC qubit reference.
    #[must_use]
    pub const fn logical(id: QubitId) -> Self {
        Self::Logical(id)
    }

    /// Creates a physical QEC qubit reference.
    #[must_use]
    pub const fn physical(id: PhysicalQubitId) -> Self {
        Self::Physical(id)
    }

    /// Returns the logical identity when present.
    #[must_use]
    pub const fn logical_id(self) -> Option<QubitId> {
        match self {
            Self::Logical(id) => Some(id),
            Self::Physical(_) => None,
        }
    }

    /// Returns the physical identity when present.
    #[must_use]
    pub const fn physical_id(self) -> Option<PhysicalQubitId> {
        match self {
            Self::Logical(_) => None,
            Self::Physical(id) => Some(id),
        }
    }

    /// Returns true when the reference is logical.
    #[must_use]
    pub const fn is_logical(self) -> bool {
        matches!(self, Self::Logical(_))
    }

    /// Returns true when the reference is physical.
    #[must_use]
    pub const fn is_physical(self) -> bool {
        matches!(self, Self::Physical(_))
    }
}

impl From<QubitId> for QecQubit {
    fn from(value: QubitId) -> Self {
        Self::Logical(value)
    }
}

impl From<PhysicalQubitId> for QecQubit {
    fn from(value: PhysicalQubitId) -> Self {
        Self::Physical(value)
    }
}

impl fmt::Display for QecQubit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Logical(id) => write!(formatter, "{id}"),
            Self::Physical(id) => write!(formatter, "{id}"),
        }
    }
}

// ============================================================================
// Operation classification
// ============================================================================

/// Semantic category of an operation required by QEC.
///
/// These categories describe intent. They do not prescribe a particular gate,
/// pulse, hardware instruction or implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QecOperationKind {
    /// Prepare an ancilla or other QEC workspace.
    Preparation,

    /// Reset an ancilla/workspace resource.
    Reset,

    /// Apply an abstract syndrome-extraction interaction.
    SyndromeInteraction,

    /// Apply an abstract stabilizer interaction.
    StabilizerInteraction,

    /// Measure a QEC measurement resource.
    Measurement,

    /// Perform a semantic syndrome-transfer operation.
    SyndromeTransfer,

    /// Wait for classical information required by QEC.
    ClassicalSynchronization,

    /// Apply a QEC recovery/correction operation.
    Recovery,

    /// Perform a semantic fault-tolerance synchronization point.
    Synchronization,

    /// A QEC-specific operation supplied by an extensible QEC implementation.
    Custom,
}

/// Phase of a QEC scheduling request.
///
/// The phase is descriptive and does not force the scheduler to execute phases
/// sequentially when the dependency graph permits additional parallelism.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QecPhase {
    /// Preparation phase.
    Preparation,

    /// Syndrome extraction phase.
    SyndromeExtraction,

    /// Measurement phase.
    Measurement,

    /// Classical processing/feedback phase.
    ClassicalProcessing,

    /// Recovery phase.
    Recovery,

    /// Synchronization phase.
    Synchronization,

    /// Implementation-defined QEC phase.
    Custom,
}

// ============================================================================
// Dependency semantics
// ============================================================================

/// Why one QEC operation must precede another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QecDependencyKind {
    /// The successor semantically consumes the predecessor's quantum result.
    Quantum,

    /// The successor requires a classical result produced by the predecessor.
    Classical,

    /// The successor requires the predecessor's measurement to be complete.
    Measurement,

    /// The successor requires a QEC round to complete.
    Round,

    /// The successor depends on syndrome availability.
    Syndrome,

    /// The dependency exists because both operations share a QEC semantic
    /// resource that cannot be concurrently used.
    Resource,

    /// Explicit synchronization requested by the QEC implementation.
    Synchronization,

    /// Implementation-defined dependency semantics.
    Custom,
}

/// A directed precedence relation between two QEC operations.
///
/// `predecessor` must complete before `successor` may begin, subject to any
/// additional timing/resource constraints introduced by the generic
/// scheduler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QecDependency {
    predecessor: QecOperationId,
    successor: QecOperationId,
    kind: QecDependencyKind,
}

impl QecDependency {
    /// Creates a dependency.
    ///
    /// Self-dependencies are rejected during request validation.
    #[must_use]
    pub const fn new(
        predecessor: QecOperationId,
        successor: QecOperationId,
        kind: QecDependencyKind,
    ) -> Self {
        Self {
            predecessor,
            successor,
            kind,
        }
    }

    /// Returns the predecessor.
    #[must_use]
    pub const fn predecessor(self) -> QecOperationId {
        self.predecessor
    }

    /// Returns the successor.
    #[must_use]
    pub const fn successor(self) -> QecOperationId {
        self.successor
    }

    /// Returns the dependency kind.
    #[must_use]
    pub const fn kind(self) -> QecDependencyKind {
        self.kind
    }
}

// ============================================================================
// Syndrome requirements
// ============================================================================

/// Semantic description of a syndrome produced by QEC operations.
///
/// This deliberately does not describe how a decoder works.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyndromeRequirement {
    id: SyndromeId,
    round: QecRoundId,
    producer: QecOperationId,
    source_qubits: Vec<QecQubit>,
    required_before: Vec<QecOperationId>,
}

impl SyndromeRequirement {
    /// Creates a syndrome requirement.
    pub fn new(
        id: SyndromeId,
        round: QecRoundId,
        producer: QecOperationId,
        source_qubits: Vec<QecQubit>,
    ) -> Result<Self, QecInterfaceError> {
        if source_qubits.is_empty() {
            return Err(QecInterfaceError::EmptySyndromeSources { syndrome: id });
        }

        Ok(Self {
            id,
            round,
            producer,
            source_qubits,
            required_before: Vec::new(),
        })
    }

    /// Adds an operation that consumes this syndrome.
    ///
    /// Duplicate consumers are ignored to preserve canonical representation.
    pub fn add_consumer(&mut self, operation: QecOperationId) {
        if !self.required_before.contains(&operation) {
            self.required_before.push(operation);
        }
    }

    /// Returns the syndrome identifier.
    #[must_use]
    pub const fn id(&self) -> SyndromeId {
        self.id
    }

    /// Returns the QEC round.
    #[must_use]
    pub const fn round(&self) -> QecRoundId {
        self.round
    }

    /// Returns the producer operation.
    #[must_use]
    pub const fn producer(&self) -> QecOperationId {
        self.producer
    }

    /// Returns the source qubits.
    #[must_use]
    pub fn source_qubits(&self) -> &[QecQubit] {
        &self.source_qubits
    }

    /// Returns operations that require this syndrome.
    #[must_use]
    pub fn consumers(&self) -> &[QecOperationId] {
        &self.required_before
    }
}

// ============================================================================
// Classical feedback
// ============================================================================

/// Semantic requirement for QEC classical feedback.
///
/// Timing duration and processing latency are intentionally not represented as
/// raw numbers here. The scheduler's timing subsystem must resolve those
/// properties from the target and execution context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QecFeedbackRequirement {
    producer: QecOperationId,
    consumers: Vec<QecOperationId>,
    syndrome: Option<SyndromeId>,
}

impl QecFeedbackRequirement {
    /// Creates a classical-feedback requirement.
    #[must_use]
    pub const fn new(
        producer: QecOperationId,
        syndrome: Option<SyndromeId>,
    ) -> Self {
        Self {
            producer,
            consumers: Vec::new(),
            syndrome,
        }
    }

    /// Adds a feedback consumer.
    ///
    /// Duplicate consumers are ignored.
    pub fn add_consumer(&mut self, operation: QecOperationId) {
        if !self.consumers.contains(&operation) {
            self.consumers.push(operation);
        }
    }

    /// Returns the classical producer.
    #[must_use]
    pub const fn producer(&self) -> QecOperationId {
        self.producer
    }

    /// Returns the associated syndrome, if any.
    #[must_use]
    pub const fn syndrome(&self) -> Option<SyndromeId> {
        self.syndrome
    }

    /// Returns feedback consumers.
    #[must_use]
    pub fn consumers(&self) -> &[QecOperationId] {
        &self.consumers
    }
}

// ============================================================================
// QEC synchronization
// ============================================================================

/// Semantic synchronization requirement.
///
/// A barrier does not automatically mean that every operation in the program
/// must stop. It applies only to the explicitly named QEC operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QecSynchronization {
    id: usize,
    phase: QecPhase,
    operations: Vec<QecOperationId>,
    round: Option<QecRoundId>,
}

impl QecSynchronization {
    /// Creates a synchronization requirement.
    pub fn new(
        id: usize,
        phase: QecPhase,
        operations: Vec<QecOperationId>,
        round: Option<QecRoundId>,
    ) -> Result<Self, QecInterfaceError> {
        if operations.is_empty() {
            return Err(QecInterfaceError::EmptySynchronization {
                synchronization: id,
            });
        }

        Ok(Self {
            id,
            phase,
            operations,
            round,
        })
    }

    /// Returns the synchronization identifier.
    #[must_use]
    pub const fn id(&self) -> usize {
        self.id
    }

    /// Returns the synchronization phase.
    #[must_use]
    pub const fn phase(&self) -> QecPhase {
        self.phase
    }

    /// Returns participating operations.
    #[must_use]
    pub fn operations(&self) -> &[QecOperationId] {
        &self.operations
    }

    /// Returns the associated round.
    #[must_use]
    pub const fn round(&self) -> Option<QecRoundId> {
        self.round
    }
}

// ============================================================================
// QEC operation
// ============================================================================

/// A semantic operation required by a QEC implementation.
///
/// The operation deliberately contains no physical gate encoding and no
/// scheduling timestamp.
///
/// Hardware-specific timing/resource requirements are resolved later by the
/// generic scheduler through target capabilities and adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QecOperation {
    id: QecOperationId,
    round: QecRoundId,
    phase: QecPhase,
    kind: QecOperationKind,
    qubits: Vec<QecQubit>,
    source_operation: Option<usize>,
    syndrome: Option<SyndromeId>,
    resource_keys: Vec<String>,
}

impl QecOperation {
    /// Creates a QEC operation.
    pub fn new(
        id: QecOperationId,
        round: QecRoundId,
        phase: QecPhase,
        kind: QecOperationKind,
        qubits: Vec<QecQubit>,
    ) -> Result<Self, QecInterfaceError> {
        if qubits.is_empty() {
            return Err(QecInterfaceError::EmptyOperationQubits { operation: id });
        }

        Ok(Self {
            id,
            round,
            phase,
            kind,
            qubits,
            source_operation: None,
            syndrome: None,
            resource_keys: Vec::new(),
        })
    }

    /// Associates this QEC operation with an original canonical-IR operation
    /// identity.
    ///
    /// The value is opaque to this module. The canonical IR remains the owner
    /// of its operation identity type.
    pub fn with_source_operation(mut self, source_operation: usize) -> Self {
        self.source_operation = Some(source_operation);
        self
    }

    /// Associates this operation with a syndrome.
    pub fn with_syndrome(mut self, syndrome: SyndromeId) -> Self {
        self.syndrome = Some(syndrome);
        self
    }

    /// Adds an abstract resource requirement.
    ///
    /// The resource key is semantic only. The generic scheduling resource
    /// subsystem resolves the actual target resource and capacity.
    pub fn add_resource_key(&mut self, key: impl Into<String>) {
        let key = key.into();

        if !self.resource_keys.iter().any(|existing| existing == &key) {
            self.resource_keys.push(key);
        }
    }

    /// Returns the operation identifier.
    #[must_use]
    pub const fn id(&self) -> QecOperationId {
        self.id
    }

    /// Returns the QEC round.
    #[must_use]
    pub const fn round(&self) -> QecRoundId {
        self.round
    }

    /// Returns the QEC phase.
    #[must_use]
    pub const fn phase(&self) -> QecPhase {
        self.phase
    }

    /// Returns the semantic operation kind.
    #[must_use]
    pub const fn kind(&self) -> QecOperationKind {
        self.kind
    }

    /// Returns the participating qubits.
    #[must_use]
    pub fn qubits(&self) -> &[QecQubit] {
        &self.qubits
    }

    /// Returns the originating canonical-IR operation identity, if supplied.
    #[must_use]
    pub const fn source_operation(&self) -> Option<usize> {
        self.source_operation
    }

    /// Returns the associated syndrome.
    #[must_use]
    pub const fn syndrome(&self) -> Option<SyndromeId> {
        self.syndrome
    }

    /// Returns abstract resource requirements.
    #[must_use]
    pub fn resource_keys(&self) -> &[String] {
        &self.resource_keys
    }
}

// ============================================================================
// QEC round
// ============================================================================

/// Semantic description of a QEC round.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QecRound {
    id: QecRoundId,
    operations: Vec<QecOperationId>,
    syndromes: Vec<SyndromeId>,
}

impl QecRound {
    /// Creates an empty QEC round.
    #[must_use]
    pub const fn new(id: QecRoundId) -> Self {
        Self {
            id,
            operations: Vec::new(),
            syndromes: Vec::new(),
        }
    }

    /// Adds an operation to the round.
    pub fn add_operation(&mut self, operation: QecOperationId) {
        if !self.operations.contains(&operation) {
            self.operations.push(operation);
        }
    }

    /// Adds a syndrome to the round.
    pub fn add_syndrome(&mut self, syndrome: SyndromeId) {
        if !self.syndromes.contains(&syndrome) {
            self.syndromes.push(syndrome);
        }
    }

    /// Returns the round identifier.
    #[must_use]
    pub const fn id(&self) -> QecRoundId {
        self.id
    }

    /// Returns operations belonging to the round.
    #[must_use]
    pub fn operations(&self) -> &[QecOperationId] {
        &self.operations
    }

    /// Returns syndromes belonging to the round.
    #[must_use]
    pub fn syndromes(&self) -> &[SyndromeId] {
        &self.syndromes
    }
}

// ============================================================================
// QEC scheduling request
// ============================================================================

/// Complete semantic input supplied by a QEC implementation to the generic
/// scheduler.
///
/// This is intentionally an owned value so that scheduling can operate on an
/// immutable snapshot and does not need to retain references into a QEC
/// implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QecSchedulingRequest {
    operations: Vec<QecOperation>,
    dependencies: Vec<QecDependency>,
    rounds: Vec<QecRound>,
    syndromes: Vec<SyndromeRequirement>,
    feedback: Vec<QecFeedbackRequirement>,
    synchronizations: Vec<QecSynchronization>,
    metadata: BTreeMap<String, String>,
}

impl QecSchedulingRequest {
    /// Creates an empty request.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            operations: Vec::new(),
            dependencies: Vec::new(),
            rounds: Vec::new(),
            syndromes: Vec::new(),
            feedback: Vec::new(),
            synchronizations: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Adds an operation.
    pub fn add_operation(
        &mut self,
        operation: QecOperation,
    ) -> Result<(), QecInterfaceError> {
        if self
            .operations
            .iter()
            .any(|existing| existing.id() == operation.id())
        {
            return Err(QecInterfaceError::DuplicateOperation {
                operation: operation.id(),
            });
        }

        self.operations.push(operation);
        Ok(())
    }

    /// Adds a dependency.
    pub fn add_dependency(
        &mut self,
        dependency: QecDependency,
    ) -> Result<(), QecInterfaceError> {
        if dependency.predecessor() == dependency.successor() {
            return Err(QecInterfaceError::SelfDependency {
                operation: dependency.predecessor(),
            });
        }

        if !self.dependencies.contains(&dependency) {
            self.dependencies.push(dependency);
        }

        Ok(())
    }

    /// Adds a round.
    pub fn add_round(&mut self, round: QecRound) -> Result<(), QecInterfaceError> {
        if self.rounds.iter().any(|existing| existing.id() == round.id()) {
            return Err(QecInterfaceError::DuplicateRound { round: round.id() });
        }

        self.rounds.push(round);
        Ok(())
    }

    /// Adds a syndrome requirement.
    pub fn add_syndrome(
        &mut self,
        syndrome: SyndromeRequirement,
    ) -> Result<(), QecInterfaceError> {
        if self
            .syndromes
            .iter()
            .any(|existing| existing.id() == syndrome.id())
        {
            return Err(QecInterfaceError::DuplicateSyndrome {
                syndrome: syndrome.id(),
            });
        }

        self.syndromes.push(syndrome);
        Ok(())
    }

    /// Adds a classical-feedback requirement.
    pub fn add_feedback(&mut self, feedback: QecFeedbackRequirement) {
        self.feedback.push(feedback);
    }

    /// Adds a synchronization requirement.
    pub fn add_synchronization(
        &mut self,
        synchronization: QecSynchronization,
    ) -> Result<(), QecInterfaceError> {
        if self
            .synchronizations
            .iter()
            .any(|existing| existing.id() == synchronization.id())
        {
            return Err(QecInterfaceError::DuplicateSynchronization {
                synchronization: synchronization.id(),
            });
        }

        self.synchronizations.push(synchronization);
        Ok(())
    }

    /// Adds deterministic metadata.
    ///
    /// Metadata is descriptive and must not be used to encode hidden scheduling
    /// semantics.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Returns all QEC operations.
    #[must_use]
    pub fn operations(&self) -> &[QecOperation] {
        &self.operations
    }

    /// Returns all precedence dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &[QecDependency] {
        &self.dependencies
    }

    /// Returns all QEC rounds.
    #[must_use]
    pub fn rounds(&self) -> &[QecRound] {
        &self.rounds
    }

    /// Returns all syndrome requirements.
    #[must_use]
    pub fn syndromes(&self) -> &[SyndromeRequirement] {
        &self.syndromes
    }

    /// Returns all classical-feedback requirements.
    #[must_use]
    pub fn feedback(&self) -> &[QecFeedbackRequirement] {
        &self.feedback
    }

    /// Returns all synchronization requirements.
    #[must_use]
    pub fn synchronizations(&self) -> &[QecSynchronization] {
        &self.synchronizations
    }

    /// Returns immutable metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Validates the complete request.
    ///
    /// Validation is deliberately deterministic and does not require hardware
    /// access. Target-specific validation belongs to the generic scheduler
    /// after this request has been adapted into its target context.
    pub fn validate(&self) -> Result<(), QecInterfaceError> {
        let operation_ids: BTreeSet<QecOperationId> =
            self.operations.iter().map(QecOperation::id).collect();

        let round_ids: BTreeSet<QecRoundId> =
            self.rounds.iter().map(QecRound::id).collect();

        let syndrome_ids: BTreeSet<SyndromeId> =
            self.syndromes.iter().map(SyndromeRequirement::id).collect();

        for dependency in &self.dependencies {
            if !operation_ids.contains(&dependency.predecessor()) {
                return Err(QecInterfaceError::UnknownOperation {
                    operation: dependency.predecessor(),
                });
            }

            if !operation_ids.contains(&dependency.successor()) {
                return Err(QecInterfaceError::UnknownOperation {
                    operation: dependency.successor(),
                });
            }

            if dependency.predecessor() == dependency.successor() {
                return Err(QecInterfaceError::SelfDependency {
                    operation: dependency.predecessor(),
                });
            }
        }

        for operation in &self.operations {
            if !round_ids.contains(&operation.round()) {
                return Err(QecInterfaceError::UnknownRound {
                    round: operation.round(),
                });
            }

            if operation.qubits().is_empty() {
                return Err(QecInterfaceError::EmptyOperationQubits {
                    operation: operation.id(),
                });
            }

            let mut seen_qubits = BTreeSet::new();

            for qubit in operation.qubits() {
                if !seen_qubits.insert(*qubit) {
                    return Err(QecInterfaceError::DuplicateQubitInOperation {
                        operation: operation.id(),
                        qubit: *qubit,
                    });
                }
            }

            if let Some(syndrome) = operation.syndrome() {
                if !syndrome_ids.contains(&syndrome) {
                    return Err(QecInterfaceError::UnknownSyndrome { syndrome });
                }
            }
        }

        for round in &self.rounds {
            for operation in round.operations() {
                if !operation_ids.contains(operation) {
                    return Err(QecInterfaceError::UnknownOperation {
                        operation: *operation,
                    });
                }
            }

            for syndrome in round.syndromes() {
                if !syndrome_ids.contains(syndrome) {
                    return Err(QecInterfaceError::UnknownSyndrome {
                        syndrome: *syndrome,
                    });
                }
            }
        }

        for syndrome in &self.syndromes {
            if !operation_ids.contains(&syndrome.producer()) {
                return Err(QecInterfaceError::UnknownOperation {
                    operation: syndrome.producer(),
                });
            }

            if !round_ids.contains(&syndrome.round()) {
                return Err(QecInterfaceError::UnknownRound {
                    round: syndrome.round(),
                });
            }

            for consumer in syndrome.consumers() {
                if !operation_ids.contains(consumer) {
                    return Err(QecInterfaceError::UnknownOperation {
                        operation: *consumer,
                    });
                }
            }
        }

        for feedback in &self.feedback {
            if !operation_ids.contains(&feedback.producer()) {
                return Err(QecInterfaceError::UnknownOperation {
                    operation: feedback.producer(),
                });
            }

            if let Some(syndrome) = feedback.syndrome() {
                if !syndrome_ids.contains(&syndrome) {
                    return Err(QecInterfaceError::UnknownSyndrome { syndrome });
                }
            }

            for consumer in feedback.consumers() {
                if !operation_ids.contains(consumer) {
                    return Err(QecInterfaceError::UnknownOperation {
                        operation: *consumer,
                    });
                }
            }
        }

        for synchronization in &self.synchronizations {
            if let Some(round) = synchronization.round() {
                if !round_ids.contains(&round) {
                    return Err(QecInterfaceError::UnknownRound { round });
                }
            }

            for operation in synchronization.operations() {
                if !operation_ids.contains(operation) {
                    return Err(QecInterfaceError::UnknownOperation {
                        operation: *operation,
                    });
                }
            }
        }

        Ok(())
    }
}

impl Default for QecSchedulingRequest {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Provider trait
// ============================================================================

/// Provider of QEC scheduling requirements.
///
/// Implementations belong to QEC subsystems such as stabilizer codes,
/// subsystem codes, color codes, bosonic codes, repetition codes, or future
/// fault-tolerance systems.
///
/// The provider does not schedule anything.
///
/// It only translates its QEC semantics into the stable
/// `QecSchedulingRequest` contract.
pub trait QecSchedulingProvider {
    /// Builds an owned, immutable scheduling request.
    ///
    /// Implementations must:
    ///
    /// - preserve canonical logical qubit identities;
    /// - never invent physical mappings;
    /// - express dependencies explicitly;
    /// - express classical feedback explicitly;
    /// - avoid hardware timing assumptions;
    /// - avoid fixed machine-size assumptions;
    /// - return a request that passes `validate()`.
    fn build_scheduling_request(
        &self,
    ) -> Result<QecSchedulingRequest, QecInterfaceError>;
}

// ============================================================================
// Structured errors
// ============================================================================

/// Errors produced while constructing or validating a QEC scheduling request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QecInterfaceError {
    /// An operation was inserted more than once.
    DuplicateOperation {
        operation: QecOperationId,
    },

    /// A QEC round was inserted more than once.
    DuplicateRound {
        round: QecRoundId,
    },

    /// A syndrome was inserted more than once.
    DuplicateSyndrome {
        syndrome: SyndromeId,
    },

    /// A synchronization requirement was inserted more than once.
    DuplicateSynchronization {
        synchronization: usize,
    },

    /// An operation had no qubits.
    EmptyOperationQubits {
        operation: QecOperationId,
    },

    /// An operation contained the same qubit more than once.
    DuplicateQubitInOperation {
        operation: QecOperationId,
        qubit: QecQubit,
    },

    /// A syndrome had no source qubits.
    EmptySyndromeSources {
        syndrome: SyndromeId,
    },

    /// A synchronization contained no operations.
    EmptySynchronization {
        synchronization: usize,
    },

    /// An operation referenced an unknown round.
    UnknownRound {
        round: QecRoundId,
    },

    /// A dependency referenced an unknown operation.
    UnknownOperation {
        operation: QecOperationId,
    },

    /// A dependency referenced itself.
    SelfDependency {
        operation: QecOperationId,
    },

    /// A syndrome reference was not declared.
    UnknownSyndrome {
        syndrome: SyndromeId,
    },
}

impl fmt::Display for QecInterfaceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateOperation { operation } => {
                write!(formatter, "duplicate QEC operation: {operation}")
            }

            Self::DuplicateRound { round } => {
                write!(formatter, "duplicate QEC round: {round}")
            }

            Self::DuplicateSyndrome { syndrome } => {
                write!(formatter, "duplicate syndrome: {syndrome}")
            }

            Self::DuplicateSynchronization { synchronization } => {
                write!(
                    formatter,
                    "duplicate QEC synchronization: {synchronization}"
                )
            }

            Self::EmptyOperationQubits { operation } => {
                write!(
                    formatter,
                    "QEC operation {operation} has no participating qubits"
                )
            }

            Self::DuplicateQubitInOperation { operation, qubit } => {
                write!(
                    formatter,
                    "QEC operation {operation} contains duplicate qubit {qubit}"
                )
            }

            Self::EmptySyndromeSources { syndrome } => {
                write!(
                    formatter,
                    "syndrome {syndrome} has no source qubits"
                )
            }

            Self::EmptySynchronization { synchronization } => {
                write!(
                    formatter,
                    "QEC synchronization {synchronization} contains no operations"
                )
            }

            Self::UnknownRound { round } => {
                write!(formatter, "QEC operation references unknown round {round}")
            }

            Self::UnknownOperation { operation } => {
                write!(
                    formatter,
                    "QEC requirement references unknown operation {operation}"
                )
            }

            Self::SelfDependency { operation } => {
                write!(
                    formatter,
                    "QEC operation {operation} cannot depend on itself"
                )
            }

            Self::UnknownSyndrome { syndrome } => {
                write!(
                    formatter,
                    "QEC requirement references unknown syndrome {syndrome}"
                )
            }
        }
    }
}

impl Error for QecInterfaceError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn logical(index: usize) -> QecQubit {
        QecQubit::Logical(QubitId::new(index))
    }

    fn operation(
        id: usize,
        round: usize,
        kind: QecOperationKind,
    ) -> QecOperation {
        QecOperation::new(
            QecOperationId::new(id),
            QecRoundId::new(round),
            QecPhase::SyndromeExtraction,
            kind,
            vec![logical(id)],
        )
        .expect("test operation must be valid")
    }

    #[test]
    fn uses_canonical_logical_qubit_identity() {
        let id = QubitId::new(17);
        let qubit = QecQubit::logical(id);

        assert_eq!(qubit.logical_id(), Some(id));

        let canonical: crate::quantum::ir::qubit::QubitId = id;
        assert_eq!(canonical, id);
    }

    #[test]
    fn logical_and_physical_references_are_distinct() {
        let logical = QecQubit::logical(QubitId::new(3));
        let physical = QecQubit::physical(PhysicalQubitId::new(3));

        assert!(logical.is_logical());
        assert!(!logical.is_physical());

        assert!(physical.is_physical());
        assert!(!physical.is_logical());
    }

    #[test]
    fn request_accepts_arbitrary_number_of_operations() {
        let mut request = QecSchedulingRequest::new();

        request
            .add_round(QecRound::new(QecRoundId::new(0)))
            .expect("round must be accepted");

        for index in 0..10_000usize {
            request
                .add_operation(operation(
                    index,
                    0,
                    QecOperationKind::StabilizerInteraction,
                ))
                .expect("unique operation must be accepted");
        }

        assert_eq!(request.operations().len(), 10_000);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn duplicate_operation_is_rejected() {
        let mut request = QecSchedulingRequest::new();

        request
            .add_round(QecRound::new(QecRoundId::new(0)))
            .expect("round must be accepted");

        request
            .add_operation(operation(
                0,
                0,
                QecOperationKind::Measurement,
            ))
            .expect("first operation must be accepted");

        let result = request.add_operation(operation(
            0,
            0,
            QecOperationKind::Measurement,
        ));

        assert_eq!(
            result,
            Err(QecInterfaceError::DuplicateOperation {
                operation: QecOperationId::new(0),
            })
        );
    }

    #[test]
    fn self_dependency_is_rejected() {
        let mut request = QecSchedulingRequest::new();

        request
            .add_round(QecRound::new(QecRoundId::new(0)))
            .expect("round must be accepted");

        request
            .add_operation(operation(
                0,
                0,
                QecOperationKind::Preparation,
            ))
            .expect("operation must be accepted");

        let dependency = QecDependency::new(
            QecOperationId::new(0),
            QecOperationId::new(0),
            QecDependencyKind::Quantum,
        );

        let result = request.add_dependency(dependency);

        assert_eq!(
            result,
            Err(QecInterfaceError::SelfDependency {
                operation: QecOperationId::new(0),
            })
        );
    }

    #[test]
    fn unknown_dependency_operation_is_rejected_by_validation() {
        let mut request = QecSchedulingRequest::new();

        request
            .add_round(QecRound::new(QecRoundId::new(0)))
            .expect("round must be accepted");

        request
            .add_operation(operation(
                0,
                0,
                QecOperationKind::Preparation,
            ))
            .expect("operation must be accepted");

        request
            .add_dependency(QecDependency::new(
                QecOperationId::new(0),
                QecOperationId::new(99),
                QecDependencyKind::Quantum,
            ))
            .expect("construction should permit validation to report the unknown operation");

        assert_eq!(
            request.validate(),
            Err(QecInterfaceError::UnknownOperation {
                operation: QecOperationId::new(99),
            })
        );
    }

    #[test]
    fn physical_identity_is_not_implicitly_derived_from_logical_identity() {
        let logical = QecQubit::Logical(QubitId::new(42));

        assert_eq!(logical.logical_id(), Some(QubitId::new(42)));
        assert_eq!(logical.physical_id(), None);
    }

    #[test]
    fn syndrome_requires_source_qubits() {
        let result = SyndromeRequirement::new(
            SyndromeId::new(0),
            QecRoundId::new(0),
            QecOperationId::new(0),
            Vec::new(),
        );

        assert_eq!(
            result,
            Err(QecInterfaceError::EmptySyndromeSources {
                syndrome: SyndromeId::new(0),
            })
        );
    }

    #[test]
    fn synchronization_requires_operations() {
        let result = QecSynchronization::new(
            0,
            QecPhase::Synchronization,
            Vec::new(),
            None,
        );

        assert_eq!(
            result,
            Err(QecInterfaceError::EmptySynchronization {
                synchronization: 0,
            })
        );
    }

    #[test]
    fn request_can_represent_multiple_rounds_without_fixed_limit() {
        let mut request = QecSchedulingRequest::new();

        for index in 0..256usize {
            request
                .add_round(QecRound::new(QecRoundId::new(index)))
                .expect("round must be accepted");

            request
                .add_operation(operation(
                    index,
                    index,
                    QecOperationKind::Measurement,
                ))
                .expect("operation must be accepted");
        }

        assert_eq!(request.rounds().len(), 256);
        assert_eq!(request.operations().len(), 256);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn metadata_is_deterministically_ordered() {
        let mut request = QecSchedulingRequest::new();

        request.insert_metadata("z", "last");
        request.insert_metadata("a", "first");

        let keys: Vec<&String> = request.metadata().keys().collect();

        assert_eq!(keys, vec!["a", "z"]);
    }

    #[test]
    fn provider_contract_returns_owned_request() {
        struct TestProvider;

        impl QecSchedulingProvider for TestProvider {
            fn build_scheduling_request(
                &self,
            ) -> Result<QecSchedulingRequest, QecInterfaceError> {
                let mut request = QecSchedulingRequest::new();

                request.add_round(QecRound::new(QecRoundId::new(0)))?;

                request.add_operation(operation(
                    0,
                    0,
                    QecOperationKind::Preparation,
                ))?;

                request.validate()?;

                Ok(request)
            }
        }

        let provider = TestProvider;
        let request = provider
            .build_scheduling_request()
            .expect("provider request must validate");

        assert_eq!(request.operations().len(), 1);
    }
}