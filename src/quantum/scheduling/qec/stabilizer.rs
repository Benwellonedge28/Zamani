//! Zamani Quantum Scheduling — Stabilizer Scheduling Model.
//!
//! This module defines the scheduling-side representation of stabilizer
//! extraction requirements.
//!
//! # Architectural position
//!
//! `stabilizer.rs` is deliberately a constraint/model layer, not a scheduler
//! implementation and not a QEC decoder.
//!
//! The ownership boundary is:
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      ▼
//! quantum::error_correction
//!      │
//!      │ stabilizer / QEC semantics
//!      ▼
//! scheduling::qec::stabilizer
//!      │
//!      │ scheduling requirements
//!      ▼
//! scheduling::adapters::qec
//!      │
//!      ▼
//! scheduling::ir
//!      │
//!      ├── dependencies
//!      ├── resources
//!      ├── timing
//!      └── constraints
//!      │
//!      ▼
//! generic scheduler
//! ```
//!
//! # Responsibilities
//!
//! This module owns:
//!
//! - scheduling-side stabilizer descriptions;
//! - canonical qubit participation;
//! - data/ancilla role information;
//! - stabilizer basis information;
//! - extraction requirements;
//! - measurement requirements;
//! - preparation/reset requirements;
//! - stabilizer-to-stabilizer dependencies;
//! - round dependencies;
//! - classical-feedback requirements;
//! - resource-role metadata;
//! - validation of a stabilizer scheduling plan;
//! - deterministic traversal of stabilizer dependencies.
//!
//! This module does NOT own:
//!
//! - gate synthesis;
//! - gate decomposition;
//! - logical-to-physical routing;
//! - hardware topology;
//! - hardware discovery;
//! - calibration;
//! - pulse generation;
//! - absolute physical scheduling;
//! - syndrome decoding;
//! - QEC correction selection;
//! - noise modelling;
//! - QPU execution;
//! - vendor/provider SDKs;
//! - scheduler policy;
//! - ASAP/ALAP/list scheduling algorithms.
//!
//! Those responsibilities belong to the corresponding Zamani subsystems.
//!
//! # Canonical identity rule
//!
//! Qubit identity MUST come from:
//!
//! `crate::quantum::ir::qubit::QubitId`
//!
//! This module deliberately does not define another `QubitId`.
//!
//! Stabilizer identity comes from the canonical QEC subsystem:
//!
//! `crate::quantum::error_correction::syndrome::StabilizerId`
//!
//! This module therefore does not create another semantic stabilizer identity.
//!
//! # Scalability
//!
//! There are no fixed limits on:
//!
//! - qubits;
//! - stabilizers;
//! - data-qubit arity;
//! - ancillas;
//! - rounds;
//! - dependencies;
//! - extraction stages.
//!
//! A stabilizer can involve any number of canonical qubits permitted by the
//! supplied QEC model and target resources.
//!
//! The implementation uses dynamically sized collections and checked numeric
//! operations. It does not allocate a machine-sized matrix, timeline, or
//! topology.
//!
//! "Infinity" in Zamani means no artificial finite scheduler ceiling. Actual
//! execution remains bounded by available memory, address space, compilation
//! time, explicit user/compiler limits, and target resources.
//!
//! # Static and dynamic QEC
//!
//! A stabilizer may depend on:
//!
//! - another stabilizer;
//! - a previous round;
//! - a measurement result;
//! - runtime-resolved classical feedback.
//!
//! Therefore the model does not assume that all dependencies are known to be
//! ordinary static DAG edges.
//!
//! Runtime-resolved dependencies are retained as semantic metadata and are
//! converted by the dynamic scheduling adapter into the appropriate runtime
//! dependency representation.
//!
//! # Scheduling boundary
//!
//! A stabilizer description answers:
//!
//! > What must be extracted and what constraints must be preserved?
//!
//! The generic scheduler answers:
//!
//! > When can the required operations execute?
//!
//! The routing subsystem answers:
//!
//! > Where can those operations execute?
//!
//! The hardware subsystem answers:
//!
//! > Can the target actually execute those operations with its available
//! > resources?
//!
//! # Rust
//!
//! Designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only.
//!
//! # Safety
//!
//! No unsafe code is permitted.
//!
//! The module explicitly forbids unsafe code.
//!
//! # Integration contract
//!
//! `SyndromeExtractionPlan` from `qec/syndrome.rs` may contain individual
//! stabilizer checks. This module supplies the reusable stabilizer-level
//! semantic contract consumed by that plan and by `adapters/qec.rs`.
//!
//! The intended flow is:
//!
//! ```text
//! canonical QEC stabilizer
//!          │
//!          ▼
//! StabilizerSpec
//!          │
//!          ▼
//! SyndromeExtractionPlan
//!          │
//!          ▼
//! adapters::qec
//!          │
//!          ▼
//! scheduling::ir::operation
//!          │
//!          ▼
//! scheduling::ir::dependency
//!          │
//!          ▼
//! generic scheduler
//! ```
//!
//! No later scheduler implementation should need to modify this file merely
//! because a new scheduling algorithm is added.
//!
//! # Important compatibility rule
//!
//! This file intentionally uses only stable contracts from the canonical QEC
//! and IR layers. It does not depend on concrete scheduler algorithms.
//!
//! Consequently:
//!
//! - ASAP can consume it;
//! - ALAP can consume it;
//! - list scheduling can consume it;
//! - RCPSP can consume it;
//! - distributed scheduling can consume it;
//! - runtime scheduling can consume it;
//! - verification can consume it;
//! - diagnostics can consume it.
//!
//! None of those algorithms should require this file to be rewritten.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::quantum::error_correction::syndrome::StabilizerId;
use crate::quantum::ir::qubit::QubitId;

use super::syndrome::{
    AncillaPreparation,
    ClassicalDependency,
    FeedbackRequirement,
    SyndromeBasis,
    SyndromeCheckId,
    SyndromeMeasurementMode,
    SyndromeQubit,
    SyndromeQubitRole,
};

// ============================================================================
// Result / error boundary
// ============================================================================

/// Result type for stabilizer scheduling operations.
pub type StabilizerResult<T> = Result<T, StabilizerSchedulingError>;

/// Errors produced by the scheduling-side stabilizer model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StabilizerSchedulingError {
    /// A stabilizer specification has no data qubits.
    EmptyDataQubits {
        stabilizer: StabilizerId,
    },

    /// A stabilizer requiring an ancilla does not provide one.
    MissingAncilla {
        stabilizer: StabilizerId,
    },

    /// A qubit occurs more than once in the same stabilizer role set.
    DuplicateQubit {
        stabilizer: StabilizerId,
        qubit: QubitId,
    },

    /// A qubit is simultaneously classified as data and ancilla.
    DataAncillaAlias {
        stabilizer: StabilizerId,
        qubit: QubitId,
    },

    /// A stabilizer depends on itself.
    SelfDependency {
        stabilizer: StabilizerId,
    },

    /// A dependency references a stabilizer absent from the plan.
    UnknownDependency {
        stabilizer: StabilizerId,
        dependency: StabilizerId,
    },

    /// The dependency graph contains a cycle.
    DependencyCycle,

    /// A round dependency is internally inconsistent.
    InvalidRoundDependency {
        predecessor: SyndromeRound,
        successor: SyndromeRound,
    },

    /// A stabilizer is declared twice in a plan.
    DuplicateStabilizer {
        stabilizer: StabilizerId,
    },

    /// A required classical source was not declared.
    MissingFeedbackSource {
        stabilizer: StabilizerId,
    },

    /// A measurement configuration is inconsistent.
    InvalidMeasurementConfiguration {
        stabilizer: StabilizerId,
    },

    /// A numeric operation overflowed.
    NumericOverflow,

    /// A caller exceeded an explicitly configured capacity.
    CapacityExceeded {
        requested: u64,
        capacity: u64,
    },
}

impl fmt::Display for StabilizerSchedulingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDataQubits { stabilizer } => {
                write!(
                    formatter,
                    "stabilizer {stabilizer} has no data-qubit operands"
                )
            }

            Self::MissingAncilla { stabilizer } => {
                write!(
                    formatter,
                    "stabilizer {stabilizer} requires an ancilla"
                )
            }

            Self::DuplicateQubit { stabilizer, qubit } => {
                write!(
                    formatter,
                    "stabilizer {stabilizer} contains duplicate qubit {qubit:?}"
                )
            }

            Self::DataAncillaAlias { stabilizer, qubit } => {
                write!(
                    formatter,
                    "stabilizer {stabilizer} uses qubit {qubit:?} as both data and ancilla"
                )
            }

            Self::SelfDependency { stabilizer } => {
                write!(
                    formatter,
                    "stabilizer {stabilizer} depends on itself"
                )
            }

            Self::UnknownDependency {
                stabilizer,
                dependency,
            } => {
                write!(
                    formatter,
                    "stabilizer {stabilizer} depends on unknown stabilizer {dependency}"
                )
            }

            Self::DependencyCycle => {
                write!(
                    formatter,
                    "stabilizer dependency graph contains a cycle"
                )
            }

            Self::InvalidRoundDependency {
                predecessor,
                successor,
            } => {
                write!(
                    formatter,
                    "round dependency {predecessor} -> {successor} is invalid"
                )
            }

            Self::DuplicateStabilizer { stabilizer } => {
                write!(
                    formatter,
                    "stabilizer {stabilizer} occurs more than once"
                )
            }

            Self::MissingFeedbackSource { stabilizer } => {
                write!(
                    formatter,
                    "stabilizer {stabilizer} requires classical feedback but no source was declared"
                )
            }

            Self::InvalidMeasurementConfiguration { stabilizer } => {
                write!(
                    formatter,
                    "invalid measurement configuration for stabilizer {stabilizer}"
                )
            }

            Self::NumericOverflow => {
                write!(
                    formatter,
                    "numeric overflow while constructing stabilizer schedule metadata"
                )
            }

            Self::CapacityExceeded {
                requested,
                capacity,
            } => {
                write!(
                    formatter,
                    "stabilizer scheduling capacity exceeded: requested {requested}, capacity {capacity}"
                )
            }
        }
    }
}

impl std::error::Error for StabilizerSchedulingError {}

// ============================================================================
// Round identity
// ============================================================================

/// Stable QEC round identity.
///
/// This is a scheduling identity and not a hardware-specific resource count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyndromeRound(u64);

impl SyndromeRound {
    /// Creates a round identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying round number.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns the next round without wrapping.
    pub fn next(self) -> StabilizerResult<Self> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(StabilizerSchedulingError::NumericOverflow)
    }
}

impl fmt::Display for SyndromeRound {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "round {}", self.0)
    }
}

// ============================================================================
// Stabilizer extraction kind
// ============================================================================

/// Semantic type of a stabilizer extraction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StabilizerKind {
    /// Ordinary stabilizer generator.
    Generator,

    /// Gauge operator in a subsystem code.
    Gauge,

    /// Boundary/check operator.
    Boundary,

    /// Logical-operator measurement.
    Logical,

    /// Verification/checking stabilizer.
    Verification,

    /// Code-specific stabilizer class.
    Custom(String),
}

// ============================================================================
// Dependency semantics
// ============================================================================

/// Kind of dependency between stabilizer extractions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StabilizerDependencyKind {
    /// Quantum/data dependency.
    Quantum,

    /// Measurement result dependency.
    Measurement,

    /// Classical processing dependency.
    Classical,

    /// Required ordering without a data dependency.
    Ordering,

    /// Resource-induced ordering.
    Resource,

    /// Runtime feedback dependency.
    Feedback,
}

/// Dependency from one stabilizer extraction to another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StabilizerDependency {
    predecessor: StabilizerId,
    kind: StabilizerDependencyKind,
}

impl StabilizerDependency {
    /// Creates a dependency.
    #[must_use]
    pub const fn new(
        predecessor: StabilizerId,
        kind: StabilizerDependencyKind,
    ) -> Self {
        Self {
            predecessor,
            kind,
        }
    }

    /// Returns the predecessor stabilizer.
    #[must_use]
    pub const fn predecessor(self) -> StabilizerId {
        self.predecessor
    }

    /// Returns the dependency kind.
    #[must_use]
    pub const fn kind(self) -> StabilizerDependencyKind {
        self.kind
    }
}

// ============================================================================
// Classical feedback
// ============================================================================

/// Classical feedback attached to a stabilizer extraction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StabilizerFeedback {
    requirement: FeedbackRequirement,
    sources: Vec<ClassicalDependency>,
}

impl StabilizerFeedback {
    /// Creates a feedback-free specification.
    #[must_use]
    pub fn none() -> Self {
        Self {
            requirement: FeedbackRequirement::None,
            sources: Vec::new(),
        }
    }

    /// Creates a required feedback specification.
    #[must_use]
    pub fn required(sources: Vec<ClassicalDependency>) -> Self {
        Self {
            requirement: FeedbackRequirement::Required,
            sources,
        }
    }

    /// Creates a runtime-resolved feedback specification.
    #[must_use]
    pub fn runtime_resolved(sources: Vec<ClassicalDependency>) -> Self {
        Self {
            requirement: FeedbackRequirement::RuntimeResolved,
            sources,
        }
    }

    /// Returns the requirement.
    #[must_use]
    pub const fn requirement(&self) -> FeedbackRequirement {
        self.requirement
    }

    /// Returns the sources.
    #[must_use]
    pub fn sources(&self) -> &[ClassicalDependency] {
        &self.sources
    }

    /// Validates the feedback declaration.
    pub fn validate(
        &self,
        stabilizer: StabilizerId,
    ) -> StabilizerResult<()> {
        if !matches!(self.requirement, FeedbackRequirement::None)
            && self.sources.is_empty()
        {
            return Err(
                StabilizerSchedulingError::MissingFeedbackSource {
                    stabilizer,
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// Stabilizer specification
// ============================================================================

/// Complete scheduling-side description of one stabilizer extraction.
///
/// This structure contains semantic requirements but no physical timestamps.
/// The scheduler derives timestamps from target timing and resource models.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StabilizerSpec {
    /// Canonical QEC stabilizer identity.
    id: StabilizerId,

    /// Semantic stabilizer class.
    kind: StabilizerKind,

    /// Measurement basis.
    basis: SyndromeBasis,

    /// Qubits participating in this stabilizer.
    qubits: Vec<SyndromeQubit>,

    /// Ancilla preparation requirement.
    ancilla_preparation: AncillaPreparation,

    /// Measurement behavior.
    measurement_mode: SyndromeMeasurementMode,

    /// Dependencies on other stabilizers.
    dependencies: Vec<StabilizerDependency>,

    /// Optional classical feedback requirement.
    feedback: StabilizerFeedback,

    /// Round in which this extraction belongs.
    round: Option<SyndromeRound>,

    /// Whether the stabilizer extraction may execute concurrently with
    /// another extraction when all explicit resource constraints permit it.
    parallelizable: bool,

    /// Opaque code-specific metadata.
    metadata: BTreeMap<String, String>,
}

impl StabilizerSpec {
    /// Creates a stabilizer specification.
    #[must_use]
    pub fn new(
        id: StabilizerId,
        kind: StabilizerKind,
        basis: SyndromeBasis,
    ) -> Self {
        Self {
            id,
            kind,
            basis,
            qubits: Vec::new(),
            ancilla_preparation: AncillaPreparation::None,
            measurement_mode: SyndromeMeasurementMode::Destructive,
            dependencies: Vec::new(),
            feedback: StabilizerFeedback::none(),
            round: None,
            parallelizable: true,
            metadata: BTreeMap::new(),
        }
    }

    /// Returns the canonical stabilizer identifier.
    #[must_use]
    pub const fn id(&self) -> StabilizerId {
        self.id
    }

    /// Returns the stabilizer kind.
    #[must_use]
    pub const fn kind(&self) -> &StabilizerKind {
        &self.kind
    }

    /// Returns the measurement basis.
    #[must_use]
    pub const fn basis(&self) -> &SyndromeBasis {
        &self.basis
    }

    /// Adds a canonical data qubit.
    ///
    /// Duplicate qubits are rejected during validation rather than silently
    /// removed, because duplicate operands may indicate a malformed QEC model.
    pub fn add_data_qubit(&mut self, qubit: QubitId) {
        self.qubits.push(SyndromeQubit::data(qubit));
    }

    /// Adds a canonical ancilla qubit.
    pub fn add_ancilla(&mut self, qubit: QubitId) {
        self.qubits.push(SyndromeQubit::ancilla(qubit));
    }

    /// Replaces the participating qubit list.
    pub fn set_qubits(&mut self, qubits: Vec<SyndromeQubit>) {
        self.qubits = qubits;
    }

    /// Returns all participating qubits.
    #[must_use]
    pub fn qubits(&self) -> &[SyndromeQubit] {
        &self.qubits
    }

    /// Returns all data qubits.
    #[must_use]
    pub fn data_qubits(&self) -> impl Iterator<Item = QubitId> + '_ {
        self.qubits
            .iter()
            .filter(|operand| operand.role() == SyndromeQubitRole::Data)
            .map(SyndromeQubit::qubit)
    }

    /// Returns all ancilla qubits.
    #[must_use]
    pub fn ancilla_qubits(&self) -> impl Iterator<Item = QubitId> + '_ {
        self.qubits
            .iter()
            .filter(|operand| operand.role() == SyndromeQubitRole::Ancilla)
            .map(SyndromeQubit::qubit)
    }

    /// Sets the ancilla preparation requirement.
    pub fn set_ancilla_preparation(
        &mut self,
        preparation: AncillaPreparation,
    ) {
        self.ancilla_preparation = preparation;
    }

    /// Returns the ancilla preparation requirement.
    #[must_use]
    pub const fn ancilla_preparation(&self) -> AncillaPreparation {
        self.ancilla_preparation
    }

    /// Sets the measurement mode.
    pub fn set_measurement_mode(
        &mut self,
        mode: SyndromeMeasurementMode,
    ) {
        self.measurement_mode = mode;
    }

    /// Returns the measurement mode.
    #[must_use]
    pub const fn measurement_mode(&self) -> SyndromeMeasurementMode {
        self.measurement_mode
    }

    /// Adds a stabilizer dependency.
    pub fn add_dependency(
        &mut self,
        dependency: StabilizerDependency,
    ) {
        self.dependencies.push(dependency);
    }

    /// Returns stabilizer dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &[StabilizerDependency] {
        &self.dependencies
    }

    /// Sets classical feedback.
    pub fn set_feedback(&mut self, feedback: StabilizerFeedback) {
        self.feedback = feedback;
    }

    /// Returns classical feedback.
    #[must_use]
    pub const fn feedback(&self) -> &StabilizerFeedback {
        &self.feedback
    }

    /// Assigns this stabilizer to a QEC round.
    pub fn set_round(&mut self, round: SyndromeRound) {
        self.round = Some(round);
    }

    /// Returns the assigned round.
    #[must_use]
    pub const fn round(&self) -> Option<SyndromeRound> {
        self.round
    }

    /// Controls whether this extraction may run concurrently with independent
    /// extractions.
    pub fn set_parallelizable(&mut self, parallelizable: bool) {
        self.parallelizable = parallelizable;
    }

    /// Returns whether the extraction is parallelizable.
    #[must_use]
    pub const fn is_parallelizable(&self) -> bool {
        self.parallelizable
    }

    /// Adds deterministic metadata.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Returns metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Validates the stabilizer specification.
    pub fn validate(&self) -> StabilizerResult<()> {
        let data_count = self
            .qubits
            .iter()
            .filter(|q| q.role() == SyndromeQubitRole::Data)
            .count();

        if data_count == 0 {
            return Err(
                StabilizerSchedulingError::EmptyDataQubits {
                    stabilizer: self.id,
                },
            );
        }

        let mut data = BTreeSet::new();
        let mut ancilla = BTreeSet::new();

        for operand in &self.qubits {
            let qubit = operand.qubit();

            match operand.role() {
                SyndromeQubitRole::Data => {
                    if !data.insert(qubit) {
                        return Err(
                            StabilizerSchedulingError::DuplicateQubit {
                                stabilizer: self.id,
                                qubit,
                            },
                        );
                    }
                }

                SyndromeQubitRole::Ancilla => {
                    if !ancilla.insert(qubit) {
                        return Err(
                            StabilizerSchedulingError::DuplicateQubit {
                                stabilizer: self.id,
                                qubit,
                            },
                        );
                    }
                }
            }
        }

        for qubit in &data {
            if ancilla.contains(qubit) {
                return Err(
                    StabilizerSchedulingError::DataAncillaAlias {
                        stabilizer: self.id,
                        qubit: *qubit,
                    },
                );
            }
        }

        if !matches!(
            self.ancilla_preparation,
            AncillaPreparation::None
        ) && ancilla.is_empty()
        {
            return Err(
                StabilizerSchedulingError::MissingAncilla {
                    stabilizer: self.id,
                },
            );
        }

        self.feedback.validate(self.id)?;

        if matches!(
            self.measurement_mode,
            SyndromeMeasurementMode::NonDestructive
        ) && matches!(
            self.kind,
            StabilizerKind::Logical
        )
        {
            // A non-destructive logical measurement is legal on some
            // architectures, so this is deliberately not rejected.
            //
            // This branch exists as an explicit semantic boundary rather than
            // silently assuming all logical measurements are destructive.
        }

        for dependency in &self.dependencies {
            if dependency.predecessor() == self.id {
                return Err(
                    StabilizerSchedulingError::SelfDependency {
                        stabilizer: self.id,
                    },
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// Stabilizer plan
// ============================================================================

/// A collection of stabilizers that can be converted into scheduler
/// dependencies.
///
/// The plan owns semantic stabilizer descriptions but does not own timing or
/// hardware resources.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StabilizerPlan {
    stabilizers: BTreeMap<StabilizerId, StabilizerSpec>,
    round_dependencies: BTreeSet<(SyndromeRound, SyndromeRound)>,
}

impl StabilizerPlan {
    /// Creates an empty plan.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of stabilizers.
    #[must_use]
    pub fn len(&self) -> usize {
        self.stabilizers.len()
    }

    /// Returns whether the plan contains no stabilizers.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.stabilizers.is_empty()
    }

    /// Inserts a stabilizer.
    pub fn insert(
        &mut self,
        stabilizer: StabilizerSpec,
    ) -> StabilizerResult<()> {
        stabilizer.validate()?;

        if self.stabilizers.contains_key(&stabilizer.id()) {
            return Err(
                StabilizerSchedulingError::DuplicateStabilizer {
                    stabilizer: stabilizer.id(),
                },
            );
        }

        self.stabilizers.insert(stabilizer.id(), stabilizer);

        Ok(())
    }

    /// Removes a stabilizer.
    ///
    /// Dependencies pointing to the removed stabilizer are not silently
    /// deleted. Callers should normally validate the resulting plan before
    /// scheduling it.
    pub fn remove(
        &mut self,
        stabilizer: StabilizerId,
    ) -> Option<StabilizerSpec> {
        self.stabilizers.remove(&stabilizer)
    }

    /// Returns a stabilizer.
    #[must_use]
    pub fn get(
        &self,
        stabilizer: StabilizerId,
    ) -> Option<&StabilizerSpec> {
        self.stabilizers.get(&stabilizer)
    }

    /// Returns a mutable stabilizer.
    pub fn get_mut(
        &mut self,
        stabilizer: StabilizerId,
    ) -> Option<&mut StabilizerSpec> {
        self.stabilizers.get_mut(&stabilizer)
    }

    /// Returns stabilizers in deterministic identity order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&StabilizerId, &StabilizerSpec)> {
        self.stabilizers.iter()
    }

    /// Adds an ordering dependency between rounds.
    pub fn add_round_dependency(
        &mut self,
        predecessor: SyndromeRound,
        successor: SyndromeRound,
    ) -> StabilizerResult<()> {
        if predecessor >= successor {
            return Err(
                StabilizerSchedulingError::InvalidRoundDependency {
                    predecessor,
                    successor,
                },
            );
        }

        self.round_dependencies
            .insert((predecessor, successor));

        Ok(())
    }

    /// Returns declared round dependencies.
    #[must_use]
    pub fn round_dependencies(
        &self,
    ) -> impl Iterator<Item = (SyndromeRound, SyndromeRound)> + '_ {
        self.round_dependencies.iter().copied()
    }

    /// Validates the entire stabilizer plan.
    pub fn validate(&self) -> StabilizerResult<()> {
        for stabilizer in self.stabilizers.values() {
            stabilizer.validate()?;
        }

        for stabilizer in self.stabilizers.values() {
            for dependency in stabilizer.dependencies() {
                if !self
                    .stabilizers
                    .contains_key(&dependency.predecessor())
                {
                    return Err(
                        StabilizerSchedulingError::UnknownDependency {
                            stabilizer: stabilizer.id(),
                            dependency: dependency.predecessor(),
                        },
                    );
                }
            }
        }

        self.validate_dependency_graph()?;

        Ok(())
    }

    /// Returns stabilizer IDs in deterministic topological order.
    ///
    /// This implementation is iterative and therefore does not depend on
    /// call-stack depth for very large QEC plans.
    pub fn topological_order(
        &self,
    ) -> StabilizerResult<Vec<StabilizerId>> {
        self.validate()?;

        let mut indegree: BTreeMap<StabilizerId, usize> =
            self.stabilizers
                .keys()
                .copied()
                .map(|id| (id, 0))
                .collect();

        let mut successors: BTreeMap<
            StabilizerId,
            BTreeSet<StabilizerId>,
        > = BTreeMap::new();

        for stabilizer in self.stabilizers.values() {
            for dependency in stabilizer.dependencies() {
                let predecessor = dependency.predecessor();

                successors
                    .entry(predecessor)
                    .or_default()
                    .insert(stabilizer.id());

                let entry = indegree
                    .get_mut(&stabilizer.id())
                    .ok_or(
                        StabilizerSchedulingError::UnknownDependency {
                            stabilizer: stabilizer.id(),
                            dependency: predecessor,
                        },
                    )?;

                *entry = entry
                    .checked_add(1)
                    .ok_or(
                        StabilizerSchedulingError::NumericOverflow,
                    )?;
            }
        }

        let mut ready = BTreeSet::new();

        for (id, degree) in &indegree {
            if *degree == 0 {
                ready.insert(*id);
            }
        }

        let mut order = Vec::with_capacity(self.stabilizers.len());

        while let Some(id) = ready.pop_first() {
            order.push(id);

            if let Some(children) = successors.get(&id) {
                for successor in children {
                    let degree = indegree
                        .get_mut(successor)
                        .ok_or(
                            StabilizerSchedulingError::UnknownDependency {
                                stabilizer: *successor,
                                dependency: id,
                            },
                        )?;

                    *degree = degree.checked_sub(1).ok_or(
                        StabilizerSchedulingError::NumericOverflow,
                    )?;

                    if *degree == 0 {
                        ready.insert(*successor);
                    }
                }
            }
        }

        if order.len() != self.stabilizers.len() {
            return Err(StabilizerSchedulingError::DependencyCycle);
        }

        Ok(order)
    }

    /// Returns the direct predecessors of a stabilizer.
    #[must_use]
    pub fn predecessors(
        &self,
        stabilizer: StabilizerId,
    ) -> Option<Vec<StabilizerId>> {
        self.stabilizers.get(&stabilizer).map(|spec| {
            spec.dependencies()
                .iter()
                .map(StabilizerDependency::predecessor)
                .collect()
        })
    }

    /// Returns the direct successors of a stabilizer.
    #[must_use]
    pub fn successors(
        &self,
        stabilizer: StabilizerId,
    ) -> Vec<StabilizerId> {
        let mut result = Vec::new();

        for spec in self.stabilizers.values() {
            if spec
                .dependencies()
                .iter()
                .any(|dependency| dependency.predecessor() == stabilizer)
            {
                result.push(spec.id());
            }
        }

        result
    }

    /// Returns the number of dependency edges.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.stabilizers
            .values()
            .map(|stabilizer| stabilizer.dependencies().len())
            .sum()
    }

    /// Validates that the stabilizer dependency graph is acyclic.
    fn validate_dependency_graph(&self) -> StabilizerResult<()> {
        let mut indegree: BTreeMap<StabilizerId, usize> =
            self.stabilizers
                .keys()
                .copied()
                .map(|id| (id, 0))
                .collect();

        let mut successors: BTreeMap<
            StabilizerId,
            BTreeSet<StabilizerId>,
        > = BTreeMap::new();

        for stabilizer in self.stabilizers.values() {
            for dependency in stabilizer.dependencies() {
                let predecessor = dependency.predecessor();

                if predecessor == stabilizer.id() {
                    return Err(
                        StabilizerSchedulingError::SelfDependency {
                            stabilizer: stabilizer.id(),
                        },
                    );
                }

                if !self.stabilizers.contains_key(&predecessor) {
                    return Err(
                        StabilizerSchedulingError::UnknownDependency {
                            stabilizer: stabilizer.id(),
                            dependency: predecessor,
                        },
                    );
                }

                successors
                    .entry(predecessor)
                    .or_default()
                    .insert(stabilizer.id());

                let value = indegree
                    .get_mut(&stabilizer.id())
                    .ok_or(
                        StabilizerSchedulingError::UnknownDependency {
                            stabilizer: stabilizer.id(),
                            dependency: predecessor,
                        },
                    )?;

                *value = value
                    .checked_add(1)
                    .ok_or(
                        StabilizerSchedulingError::NumericOverflow,
                    )?;
            }
        }

        let mut queue = VecDeque::new();

        for (id, degree) in &indegree {
            if *degree == 0 {
                queue.push_back(*id);
            }
        }

        let mut visited = 0usize;

        while let Some(id) = queue.pop_front() {
            visited = visited
                .checked_add(1)
                .ok_or(
                    StabilizerSchedulingError::NumericOverflow,
                )?;

            if let Some(children) = successors.get(&id) {
                for child in children {
                    let degree = indegree
                        .get_mut(child)
                        .ok_or(
                            StabilizerSchedulingError::UnknownDependency {
                                stabilizer: *child,
                                dependency: id,
                            },
                        )?;

                    *degree = degree.checked_sub(1).ok_or(
                        StabilizerSchedulingError::NumericOverflow,
                    )?;

                    if *degree == 0 {
                        queue.push_back(*child);
                    }
                }
            }
        }

        if visited != self.stabilizers.len() {
            return Err(StabilizerSchedulingError::DependencyCycle);
        }

        Ok(())
    }
}

// ============================================================================
// Scheduling projection
// ============================================================================

/// Scheduling projection of a stabilizer.
///
/// This is intentionally not a concrete scheduled operation. It is the
/// information the QEC adapter needs to construct one or more scheduler IR
/// operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StabilizerSchedulingProjection {
    /// Canonical stabilizer identity.
    pub stabilizer: StabilizerId,

    /// Scheduling-side syndrome identity.
    pub syndrome_check: SyndromeCheckId,

    /// Round, if one was explicitly declared.
    pub round: Option<SyndromeRound>,

    /// Canonical qubits involved.
    pub qubits: Vec<SyndromeQubit>,

    /// Basis.
    pub basis: SyndromeBasis,

    /// Preparation requirement.
    pub ancilla_preparation: AncillaPreparation,

    /// Measurement mode.
    pub measurement_mode: SyndromeMeasurementMode,

    /// Explicit predecessor stabilizers.
    pub predecessors: Vec<StabilizerId>,

    /// Classical feedback requirement.
    pub feedback_requirement: FeedbackRequirement,

    /// Whether the extraction can participate in parallel scheduling.
    pub parallelizable: bool,
}

impl StabilizerSpec {
    /// Converts the semantic stabilizer specification into a scheduler-facing
    /// projection.
    #[must_use]
    pub fn scheduling_projection(
        &self,
    ) -> StabilizerSchedulingProjection {
        StabilizerSchedulingProjection {
            stabilizer: self.id,
            syndrome_check: SyndromeCheckId::new(self.id),
            round: self.round,
            qubits: self.qubits.clone(),
            basis: self.basis.clone(),
            ancilla_preparation: self.ancilla_preparation,
            measurement_mode: self.measurement_mode,
            predecessors: self
                .dependencies
                .iter()
                .map(StabilizerDependency::predecessor)
                .collect(),
            feedback_requirement: self.feedback.requirement(),
            parallelizable: self.parallelizable,
        }
    }
}

// ============================================================================
// Round utilities
// ============================================================================

/// Groups stabilizers by explicitly assigned QEC round.
///
/// Stabilizers without an explicit round are omitted from the returned map.
/// This is intentional: absence of a round means that the scheduler is free
/// to derive placement from dependencies and policy.
#[must_use]
pub fn group_by_round(
    plan: &StabilizerPlan,
) -> BTreeMap<SyndromeRound, Vec<StabilizerId>> {
    let mut result: BTreeMap<SyndromeRound, Vec<StabilizerId>> =
        BTreeMap::new();

    for spec in plan.stabilizers.values() {
        if let Some(round) = spec.round() {
            result.entry(round).or_default().push(spec.id());
        }
    }

    result
}

/// Returns whether two stabilizers can be considered semantically
/// independent before resource constraints are applied.
///
/// This function deliberately does NOT inspect hardware resources. Two
/// stabilizers can be semantically independent but still conflict on a
/// physical resource during scheduling.
#[must_use]
pub fn semantically_independent(
    first: &StabilizerSpec,
    second: &StabilizerSpec,
) -> bool {
    if first.id() == second.id() {
        return false;
    }

    if first
        .dependencies()
        .iter()
        .any(|dependency| dependency.predecessor() == second.id())
    {
        return false;
    }

    if second
        .dependencies()
        .iter()
        .any(|dependency| dependency.predecessor() == first.id())
    {
        return false;
    }

    if first
        .feedback()
        .sources()
        .iter()
        .any(|source| match source {
            ClassicalDependency::Measurement(check) => {
                check.stabilizer() == second.id()
            }
            ClassicalDependency::Round(_) => false,
        })
    {
        return false;
    }

    if second
        .feedback()
        .sources()
        .iter()
        .any(|source| match source {
            ClassicalDependency::Measurement(check) => {
                check.stabilizer() == first.id()
            }
            ClassicalDependency::Round(_) => false,
        })
    {
        return false;
    }

    true
}

// ============================================================================
// Resource-role extraction
// ============================================================================

/// Describes the qubit roles that the scheduler must reserve for a
/// stabilizer extraction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StabilizerResourceRequirements {
    /// Canonical data-qubit resources.
    pub data_qubits: Vec<QubitId>,

    /// Canonical ancilla resources.
    pub ancillas: Vec<QubitId>,

    /// Whether the extraction requires exclusive ownership of its qubits
    /// during the generated operations.
    pub exclusive_qubits: bool,
}

impl StabilizerSpec {
    /// Produces resource requirements without querying hardware.
    #[must_use]
    pub fn resource_requirements(
        &self,
    ) -> StabilizerResourceRequirements {
        StabilizerResourceRequirements {
            data_qubits: self.data_qubits().collect(),
            ancillas: self.ancilla_qubits().collect(),
            exclusive_qubits: true,
        }
    }
}

// ============================================================================
// Plan statistics
// ============================================================================

/// Deterministic structural statistics for a stabilizer plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StabilizerPlanStatistics {
    /// Number of stabilizers.
    pub stabilizers: usize,

    /// Number of dependency edges.
    pub dependencies: usize,

    /// Number of explicit rounds.
    pub explicit_rounds: usize,

    /// Number of canonical qubit references.
    pub qubit_references: usize,

    /// Number of data-qubit references.
    pub data_qubit_references: usize,

    /// Number of ancilla references.
    pub ancilla_references: usize,
}

impl StabilizerPlan {
    /// Calculates structural statistics without changing the plan.
    #[must_use]
    pub fn statistics(&self) -> StabilizerPlanStatistics {
        let mut explicit_rounds = BTreeSet::new();
        let mut qubit_references = 0usize;
        let mut data_qubit_references = 0usize;
        let mut ancilla_references = 0usize;
        let mut dependencies = 0usize;

        for spec in self.stabilizers.values() {
            dependencies += spec.dependencies().len();

            if let Some(round) = spec.round() {
                explicit_rounds.insert(round);
            }

            qubit_references += spec.qubits().len();

            for qubit in spec.qubits() {
                match qubit.role() {
                    SyndromeQubitRole::Data => {
                        data_qubit_references += 1;
                    }
                    SyndromeQubitRole::Ancilla => {
                        ancilla_references += 1;
                    }
                }
            }
        }

        StabilizerPlanStatistics {
            stabilizers: self.stabilizers.len(),
            dependencies,
            explicit_rounds: explicit_rounds.len(),
            qubit_references,
            data_qubit_references,
            ancilla_references,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn stabilizer_id(value: usize) -> StabilizerId {
        StabilizerId(value)
    }

    fn qubit_id(value: usize) -> QubitId {
        QubitId(value)
    }

    #[test]
    fn empty_stabilizer_is_rejected() {
        let stabilizer = StabilizerSpec::new(
            stabilizer_id(0),
            StabilizerKind::Generator,
            SyndromeBasis::Z,
        );

        assert!(matches!(
            stabilizer.validate(),
            Err(
                StabilizerSchedulingError::EmptyDataQubits { .. }
            )
        ));
    }

    #[test]
    fn duplicate_data_qubit_is_rejected() {
        let mut stabilizer = StabilizerSpec::new(
            stabilizer_id(0),
            StabilizerKind::Generator,
            SyndromeBasis::Z,
        );

        stabilizer.add_data_qubit(qubit_id(0));
        stabilizer.add_data_qubit(qubit_id(0));

        assert!(matches!(
            stabilizer.validate(),
            Err(
                StabilizerSchedulingError::DuplicateQubit { .. }
            )
        ));
    }

    #[test]
    fn data_ancilla_alias_is_rejected() {
        let mut stabilizer = StabilizerSpec::new(
            stabilizer_id(0),
            StabilizerKind::Generator,
            SyndromeBasis::Z,
        );

        stabilizer.add_data_qubit(qubit_id(0));
        stabilizer.add_ancilla(qubit_id(0));
        stabilizer.set_ancilla_preparation(
            AncillaPreparation::Reset,
        );

        assert!(matches!(
            stabilizer.validate(),
            Err(
                StabilizerSchedulingError::DataAncillaAlias { .. }
            )
        ));
    }

    #[test]
    fn missing_ancilla_is_rejected_when_required() {
        let mut stabilizer = StabilizerSpec::new(
            stabilizer_id(0),
            StabilizerKind::Generator,
            SyndromeBasis::Z,
        );

        stabilizer.add_data_qubit(qubit_id(0));
        stabilizer.set_ancilla_preparation(
            AncillaPreparation::Reset,
        );

        assert!(matches!(
            stabilizer.validate(),
            Err(
                StabilizerSchedulingError::MissingAncilla { .. }
            )
        ));
    }

    #[test]
    fn valid_stabilizer_is_accepted() {
        let mut stabilizer = StabilizerSpec::new(
            stabilizer_id(0),
            StabilizerKind::Generator,
            SyndromeBasis::Z,
        );

        stabilizer.add_data_qubit(qubit_id(0));
        stabilizer.add_data_qubit(qubit_id(1));
        stabilizer.add_ancilla(qubit_id(2));
        stabilizer.set_ancilla_preparation(
            AncillaPreparation::Reset,
        );

        assert!(stabilizer.validate().is_ok());
    }

    #[test]
    fn self_dependency_is_rejected() {
        let mut stabilizer = StabilizerSpec::new(
            stabilizer_id(0),
            StabilizerKind::Generator,
            SyndromeBasis::Z,
        );

        stabilizer.add_data_qubit(qubit_id(0));
        stabilizer.add_dependency(
            StabilizerDependency::new(
                stabilizer_id(0),
                StabilizerDependencyKind::Ordering,
            ),
        );

        assert!(matches!(
            stabilizer.validate(),
            Err(
                StabilizerSchedulingError::SelfDependency { .. }
            )
        ));
    }

    #[test]
    fn plan_orders_dependencies_deterministically() {
        let mut first = StabilizerSpec::new(
            stabilizer_id(0),
            StabilizerKind::Generator,
            SyndromeBasis::Z,
        );
        first.add_data_qubit(qubit_id(0));

        let mut second = StabilizerSpec::new(
            stabilizer_id(1),
            StabilizerKind::Generator,
            SyndromeBasis::X,
        );
        second.add_data_qubit(qubit_id(1));
        second.add_dependency(
            StabilizerDependency::new(
                stabilizer_id(0),
                StabilizerDependencyKind::Ordering,
            ),
        );

        let mut plan = StabilizerPlan::new();

        plan.insert(first).unwrap();
        plan.insert(second).unwrap();

        let order = plan.topological_order().unwrap();

        assert_eq!(
            order,
            vec![stabilizer_id(0), stabilizer_id(1)]
        );
    }

    #[test]
    fn cycle_is_rejected() {
        let mut first = StabilizerSpec::new(
            stabilizer_id(0),
            StabilizerKind::Generator,
            SyndromeBasis::Z,
        );
        first.add_data_qubit(qubit_id(0));
        first.add_dependency(
            StabilizerDependency::new(
                stabilizer_id(1),
                StabilizerDependencyKind::Ordering,
            ),
        );

        let mut second = StabilizerSpec::new(
            stabilizer_id(1),
            StabilizerKind::Generator,
            SyndromeBasis::X,
        );
        second.add_data_qubit(qubit_id(1));
        second.add_dependency(
            StabilizerDependency::new(
                stabilizer_id(0),
                StabilizerDependencyKind::Ordering,
            ),
        );

        let mut plan = StabilizerPlan::new();

        plan.insert(first).unwrap();
        plan.insert(second).unwrap();

        assert!(matches!(
            plan.validate(),
            Err(StabilizerSchedulingError::DependencyCycle)
        ));
    }

    #[test]
    fn round_ordering_must_be_forward() {
        let mut plan = StabilizerPlan::new();

        assert!(matches!(
            plan.add_round_dependency(
                SyndromeRound::new(2),
                SyndromeRound::new(1),
            ),
            Err(
                StabilizerSchedulingError::InvalidRoundDependency {
                    ..
                }
            )
        ));
    }

    #[test]
    fn statistics_are_deterministic() {
        let mut stabilizer = StabilizerSpec::new(
            stabilizer_id(0),
            StabilizerKind::Generator,
            SyndromeBasis::Z,
        );

        stabilizer.add_data_qubit(qubit_id(0));
        stabilizer.add_data_qubit(qubit_id(1));
        stabilizer.add_ancilla(qubit_id(2));
        stabilizer.set_round(SyndromeRound::new(0));

        let mut plan = StabilizerPlan::new();
        plan.insert(stabilizer).unwrap();

        let statistics = plan.statistics();

        assert_eq!(statistics.stabilizers, 1);
        assert_eq!(statistics.dependencies, 0);
        assert_eq!(statistics.explicit_rounds, 1);
        assert_eq!(statistics.qubit_references, 3);
        assert_eq!(statistics.data_qubit_references, 2);
        assert_eq!(statistics.ancilla_references, 1);
    }
}