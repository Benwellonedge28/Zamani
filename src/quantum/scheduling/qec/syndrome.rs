//! Zamani Quantum Scheduling — Syndrome Extraction Scheduling Model.
//!
//! This module defines the scheduling-side representation of quantum-error-
//! correction syndrome extraction.
//!
//! # Architectural boundary
//!
//! This module describes:
//!
//! - which stabilizer is being extracted;
//! - which canonical logical/physical qubits participate;
//! - which ancillas are required;
//! - which measurements are required;
//! - which reset/readout dependencies exist;
//! - which extraction steps belong to a round;
//! - ordering constraints between rounds;
//! - optional classical-feedback dependencies;
//! - resource requirements that the generic scheduler must honor.
//!
//! It deliberately does NOT:
//!
//! - synthesize quantum gates;
//! - choose a hardware topology;
//! - perform logical-to-physical routing;
//! - assign physical timestamps;
//! - assume a fixed number of qubits;
//! - assume surface-code distance;
//! - assume four-neighbor stabilizers;
//! - assume a particular QEC code;
//! - perform syndrome decoding;
//! - access a QPU;
//! - access a provider SDK;
//! - own hardware timing;
//! - own calibration;
//! - own noise/fidelity models.
//!
//! Those concerns belong to the corresponding Zamani subsystems.
//!
//! # Pipeline
//!
//! ```text
//! QEC code / compiler
//!        │
//!        ▼
//! SyndromeExtractionPlan
//!        │
//!        ├── stabilizer identity
//!        ├── canonical QubitId operands
//!        ├── ancilla requirements
//!        ├── measurement requirements
//!        ├── reset requirements
//!        ├── round dependencies
//!        └── classical-feedback dependencies
//!        │
//!        ▼
//! scheduling::adapters::qec
//!        │
//!        ▼
//! scheduling::ir
//!        │
//!        ▼
//! dependency + resource + timing analysis
//!        │
//!        ▼
//! generic scheduler
//! ```
//!
//! # Important identity rule
//!
//! Physical/logical qubit identity belongs to the canonical quantum IR.
//! This file therefore uses:
//!
//! `crate::quantum::ir::qubit::QubitId`
//!
//! and never defines another scheduler-specific qubit identifier.
//!
//! Stabilizer identity is likewise reused from the canonical QEC syndrome
//! subsystem.
//!
//! # Scalability
//!
//! There is no production machine-size constant in this module.
//!
//! The number of:
//!
//! - syndrome checks;
//! - rounds;
//! - data qubits;
//! - ancillas;
//! - measurements;
//! - dependencies;
//!
//! is determined by the supplied plan and the configured execution resources.
//!
//! `Vec` is used as the owned collection boundary because it is the natural
//! Rust representation for dynamically sized input. Callers that generate
//! enormous plans can construct them incrementally through `SyndromePlanBuilder`
//! and validate them before handing them to the scheduler.
//!
//! # Rust
//!
//! Designed for Rust 1.97 / 1.97.1.
//!
//! # Safety
//!
//! This module contains no `unsafe` code.

use core::fmt;
use std::collections::BTreeSet;

use crate::quantum::error_correction::syndrome::StabilizerId;
use crate::quantum::ir::qubit::QubitId;

// ============================================================================
// Public result / error boundary
// ============================================================================

/// Result type for syndrome scheduling operations.
pub type SyndromeResult<T> = Result<T, SyndromeSchedulingError>;

/// Errors produced while validating a scheduling-side syndrome plan.
///
/// These errors describe malformed scheduling input. They intentionally do
/// not depend on a concrete scheduler implementation so this file can be
/// completed and stabilized independently of scheduling algorithms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyndromeSchedulingError {
    /// A syndrome extraction plan contains no checks when checks are required.
    EmptyPlan,

    /// A required identifier was duplicated.
    DuplicateStabilizer {
        stabilizer: StabilizerId,
    },

    /// A check contains no data qubits.
    EmptyDataQubits {
        stabilizer: StabilizerId,
    },

    /// A check contains no ancilla when its extraction mode requires one.
    MissingAncilla {
        stabilizer: StabilizerId,
    },

    /// The same physical qubit was assigned more than once in a single role
    /// set where uniqueness is required.
    DuplicateQubit {
        stabilizer: StabilizerId,
        qubit: QubitId,
    },

    /// A data qubit and ancilla were assigned the same physical identity.
    DataAncillaAlias {
        stabilizer: StabilizerId,
        qubit: QubitId,
    },

    /// A round number is invalid for the requested operation.
    InvalidRound,

    /// A dependency refers to a stabilizer that does not exist in the plan.
    UnknownDependency {
        stabilizer: StabilizerId,
        dependency: StabilizerId,
    },

    /// A check depends directly on itself.
    SelfDependency {
        stabilizer: StabilizerId,
    },

    /// The declared dependency graph contains a cycle.
    DependencyCycle,

    /// A round dependency violates the declared round ordering.
    InvalidRoundDependency {
        predecessor: StabilizerId,
        successor: StabilizerId,
    },

    /// A classical-feedback dependency is incomplete.
    MissingFeedbackSource {
        stabilizer: StabilizerId,
    },

    /// The requested measurement mode is inconsistent with the extraction
    /// contract.
    InvalidMeasurementConfiguration {
        stabilizer: StabilizerId,
    },

    /// A plan contains a numerical value that cannot be represented safely.
    NumericOverflow,

    /// A caller attempted to exceed an explicitly configured capacity.
    CapacityExceeded {
        requested: u64,
        capacity: u64,
    },
}

impl fmt::Display for SyndromeSchedulingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPlan => {
                write!(formatter, "syndrome extraction plan is empty")
            }
            Self::DuplicateStabilizer { stabilizer } => {
                write!(formatter, "duplicate stabilizer {stabilizer}")
            }
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
                    "stabilizer {stabilizer} contains duplicate qubit \
                     operand {qubit:?}"
                )
            }
            Self::DataAncillaAlias { stabilizer, qubit } => {
                write!(
                    formatter,
                    "stabilizer {stabilizer} uses qubit {qubit:?} as both \
                     data and ancilla"
                )
            }
            Self::InvalidRound => {
                write!(formatter, "invalid syndrome extraction round")
            }
            Self::UnknownDependency {
                stabilizer,
                dependency,
            } => {
                write!(
                    formatter,
                    "stabilizer {stabilizer} depends on unknown stabilizer \
                     {dependency}"
                )
            }
            Self::SelfDependency { stabilizer } => {
                write!(
                    formatter,
                    "stabilizer {stabilizer} depends on itself"
                )
            }
            Self::DependencyCycle => {
                write!(
                    formatter,
                    "syndrome dependency graph contains a cycle"
                )
            }
            Self::InvalidRoundDependency {
                predecessor,
                successor,
            } => {
                write!(
                    formatter,
                    "stabilizer {predecessor} cannot be a predecessor of \
                     {successor} because its round ordering is invalid"
                )
            }
            Self::MissingFeedbackSource { stabilizer } => {
                write!(
                    formatter,
                    "stabilizer {stabilizer} requires classical feedback \
                     but has no source"
                )
            }
            Self::InvalidMeasurementConfiguration { stabilizer } => {
                write!(
                    formatter,
                    "invalid measurement configuration for stabilizer \
                     {stabilizer}"
                )
            }
            Self::NumericOverflow => {
                write!(formatter, "numeric overflow while constructing syndrome plan")
            }
            Self::CapacityExceeded {
                requested,
                capacity,
            } => {
                write!(
                    formatter,
                    "syndrome plan capacity exceeded: requested {requested}, \
                     capacity {capacity}"
                )
            }
        }
    }
}

impl std::error::Error for SyndromeSchedulingError {}

// ============================================================================
// Stable scheduling-side identifiers
// ============================================================================

/// Stable identifier for one syndrome extraction check.
///
/// The actual stabilizer identity is supplied by the QEC subsystem. This
/// wrapper gives scheduling a distinct semantic role without inventing a
/// second numeric identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyndromeCheckId(pub StabilizerId);

impl SyndromeCheckId {
    /// Creates a scheduling identifier from the canonical QEC stabilizer ID.
    #[must_use]
    pub const fn new(stabilizer: StabilizerId) -> Self {
        Self(stabilizer)
    }

    /// Returns the canonical stabilizer identifier.
    #[must_use]
    pub const fn stabilizer(self) -> StabilizerId {
        self.0
    }
}

impl fmt::Display for SyndromeCheckId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "syndrome:{}", self.0)
    }
}

/// Stable identifier for a syndrome extraction round.
///
/// `u64` is intentionally used for the semantic identity so the scheduler
/// does not impose a small machine-specific round limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyndromeRound(u64);

impl SyndromeRound {
    /// Creates a round identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric round.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns the following round without wrapping.
    pub fn next(self) -> SyndromeResult<Self> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(SyndromeSchedulingError::NumericOverflow)
    }
}

impl fmt::Display for SyndromeRound {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "round {}", self.0)
    }
}

// ============================================================================
// Syndrome basis / measurement semantics
// ============================================================================

/// Basis in which a stabilizer syndrome is measured.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SyndromeBasis {
    /// X basis.
    X,

    /// Y basis.
    Y,

    /// Z basis.
    Z,

    /// Code- or hardware-defined basis.
    ///
    /// The string is descriptive metadata. It is not interpreted by this
    /// scheduling layer.
    Custom(String),
}

impl SyndromeBasis {
    /// Returns whether this is a built-in basis.
    #[must_use]
    pub fn is_standard(&self) -> bool {
        matches!(self, Self::X | Self::Y | Self::Z)
    }
}

/// Measurement behavior required by the syndrome extraction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SyndromeMeasurementMode {
    /// Measurement consumes the measured quantum state.
    Destructive,

    /// Measurement preserves the quantum state according to the target's
    /// measurement contract.
    NonDestructive,
}

/// Ancilla initialization requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AncillaPreparation {
    /// No explicit preparation is requested by the QEC layer.
    None,

    /// Ancilla must be reset before extraction.
    Reset,

    /// Ancilla must be prepared in the computational zero state.
    Zero,

    /// Ancilla must be prepared in the plus state.
    Plus,

    /// Ancilla preparation is code-specific.
    Custom,
}

/// Whether classical feedback is needed after a syndrome measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FeedbackRequirement {
    /// No feedback dependency is declared.
    None,

    /// Feedback must be available before the next dependent extraction.
    Required,

    /// Feedback may be resolved asynchronously by the runtime.
    RuntimeResolved,
}

// ============================================================================
// Resource-role description
// ============================================================================

/// Semantic role of a qubit in syndrome extraction.
///
/// This is deliberately a role, not a new qubit identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SyndromeQubitRole {
    /// Data qubit participating in the stabilizer.
    Data,

    /// Ancilla used for syndrome extraction.
    Ancilla,
}

/// One qubit participating in a syndrome check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SyndromeQubit {
    qubit: QubitId,
    role: SyndromeQubitRole,
}

impl SyndromeQubit {
    /// Creates a data-qubit operand.
    #[must_use]
    pub const fn data(qubit: QubitId) -> Self {
        Self {
            qubit,
            role: SyndromeQubitRole::Data,
        }
    }

    /// Creates an ancilla operand.
    #[must_use]
    pub const fn ancilla(qubit: QubitId) -> Self {
        Self {
            qubit,
            role: SyndromeQubitRole::Ancilla,
        }
    }

    /// Returns the canonical qubit identity.
    #[must_use]
    pub const fn qubit(self) -> QubitId {
        self.qubit
    }

    /// Returns the scheduling role.
    #[must_use]
    pub const fn role(self) -> SyndromeQubitRole {
        self.role
    }
}

// ============================================================================
// Classical dependencies
// ============================================================================

/// Source of a classical dependency associated with syndrome extraction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClassicalDependency {
    /// The measurement result from this stabilizer.
    Measurement(SyndromeCheckId),

    /// A previous syndrome extraction round.
    Round(SyndromeRound),
}

/// Classical feedback contract attached to a syndrome check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyndromeFeedback {
    requirement: FeedbackRequirement,
    sources: Vec<ClassicalDependency>,
}

impl SyndromeFeedback {
    /// Creates a feedback-free contract.
    #[must_use]
    pub fn none() -> Self {
        Self {
            requirement: FeedbackRequirement::None,
            sources: Vec::new(),
        }
    }

    /// Creates a required feedback contract.
    #[must_use]
    pub fn required(sources: Vec<ClassicalDependency>) -> Self {
        Self {
            requirement: FeedbackRequirement::Required,
            sources,
        }
    }

    /// Creates a runtime-resolved feedback contract.
    #[must_use]
    pub fn runtime_resolved(sources: Vec<ClassicalDependency>) -> Self {
        Self {
            requirement: FeedbackRequirement::RuntimeResolved,
            sources,
        }
    }

    /// Returns the feedback requirement.
    #[must_use]
    pub const fn requirement(&self) -> FeedbackRequirement {
        self.requirement
    }

    /// Returns all declared classical sources.
    #[must_use]
    pub fn sources(&self) -> &[ClassicalDependency] {
        &self.sources
    }

    /// Returns whether no feedback is required.
    #[must_use]
    pub const fn is_none(&self) -> bool {
        matches!(self.requirement, FeedbackRequirement::None)
    }
}

// ============================================================================
// Syndrome check
// ============================================================================

/// One scheduling-level syndrome extraction check.
///
/// This type deliberately describes requirements rather than gate sequences.
/// A code-specific compiler can map the check to one or many scheduler
/// operations later.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyndromeCheck {
    id: SyndromeCheckId,
    round: SyndromeRound,
    basis: SyndromeBasis,
    measurement_mode: SyndromeMeasurementMode,
    ancilla_preparation: AncillaPreparation,
    data_qubits: Vec<QubitId>,
    ancilla_qubits: Vec<QubitId>,
    dependencies: Vec<SyndromeCheckId>,
    feedback: SyndromeFeedback,
}

impl SyndromeCheck {
    /// Creates a syndrome check.
    #[must_use]
    pub fn new(
        stabilizer: StabilizerId,
        round: SyndromeRound,
        basis: SyndromeBasis,
    ) -> Self {
        Self {
            id: SyndromeCheckId::new(stabilizer),
            round,
            basis,
            measurement_mode: SyndromeMeasurementMode::Destructive,
            ancilla_preparation: AncillaPreparation::Reset,
            data_qubits: Vec::new(),
            ancilla_qubits: Vec::new(),
            dependencies: Vec::new(),
            feedback: SyndromeFeedback::none(),
        }
    }

    /// Returns the check identifier.
    #[must_use]
    pub const fn id(&self) -> SyndromeCheckId {
        self.id
    }

    /// Returns the canonical stabilizer identifier.
    #[must_use]
    pub const fn stabilizer(&self) -> StabilizerId {
        self.id.stabilizer()
    }

    /// Returns the extraction round.
    #[must_use]
    pub const fn round(&self) -> SyndromeRound {
        self.round
    }

    /// Returns the requested measurement basis.
    #[must_use]
    pub fn basis(&self) -> &SyndromeBasis {
        &self.basis
    }

    /// Returns the measurement mode.
    #[must_use]
    pub const fn measurement_mode(&self) -> SyndromeMeasurementMode {
        self.measurement_mode
    }

    /// Returns the ancilla preparation requirement.
    #[must_use]
    pub const fn ancilla_preparation(&self) -> AncillaPreparation {
        self.ancilla_preparation
    }

    /// Returns all data qubits.
    #[must_use]
    pub fn data_qubits(&self) -> &[QubitId] {
        &self.data_qubits
    }

    /// Returns all ancilla qubits.
    #[must_use]
    pub fn ancilla_qubits(&self) -> &[QubitId] {
        &self.ancilla_qubits
    }

    /// Returns all preceding syndrome checks.
    #[must_use]
    pub fn dependencies(&self) -> &[SyndromeCheckId] {
        &self.dependencies
    }

    /// Returns the classical-feedback contract.
    #[must_use]
    pub fn feedback(&self) -> &SyndromeFeedback {
        &self.feedback
    }

    /// Sets the measurement mode.
    #[must_use]
    pub fn with_measurement_mode(
        mut self,
        mode: SyndromeMeasurementMode,
    ) -> Self {
        self.measurement_mode = mode;
        self
    }

    /// Sets the ancilla preparation requirement.
    #[must_use]
    pub fn with_ancilla_preparation(
        mut self,
        preparation: AncillaPreparation,
    ) -> Self {
        self.ancilla_preparation = preparation;
        self
    }

    /// Adds a data qubit.
    #[must_use]
    pub fn with_data_qubit(mut self, qubit: QubitId) -> Self {
        self.data_qubits.push(qubit);
        self
    }

    /// Adds multiple data qubits.
    #[must_use]
    pub fn with_data_qubits<I>(mut self, qubits: I) -> Self
    where
        I: IntoIterator<Item = QubitId>,
    {
        self.data_qubits.extend(qubits);
        self
    }

    /// Adds an ancilla qubit.
    #[must_use]
    pub fn with_ancilla(mut self, qubit: QubitId) -> Self {
        self.ancilla_qubits.push(qubit);
        self
    }

    /// Adds multiple ancillas.
    #[must_use]
    pub fn with_ancillas<I>(mut self, qubits: I) -> Self
    where
        I: IntoIterator<Item = QubitId>,
    {
        self.ancilla_qubits.extend(qubits);
        self
    }

    /// Adds a preceding syndrome check dependency.
    #[must_use]
    pub fn depends_on(mut self, dependency: SyndromeCheckId) -> Self {
        self.dependencies.push(dependency);
        self
    }

    /// Adds several preceding syndrome dependencies.
    #[must_use]
    pub fn depends_on_many<I>(mut self, dependencies: I) -> Self
    where
        I: IntoIterator<Item = SyndromeCheckId>,
    {
        self.dependencies.extend(dependencies);
        self
    }

    /// Sets the classical feedback contract.
    #[must_use]
    pub fn with_feedback(mut self, feedback: SyndromeFeedback) -> Self {
        self.feedback = feedback;
        self
    }

    /// Validates the individual check.
    pub fn validate(&self) -> SyndromeResult<()> {
        if self.data_qubits.is_empty() {
            return Err(SyndromeSchedulingError::EmptyDataQubits {
                stabilizer: self.stabilizer(),
            });
        }

        let mut data = BTreeSet::new();

        for qubit in &self.data_qubits {
            if !data.insert(*qubit) {
                return Err(SyndromeSchedulingError::DuplicateQubit {
                    stabilizer: self.stabilizer(),
                    qubit: *qubit,
                });
            }
        }

        if !self.ancilla_qubits.is_empty() {
            let mut ancillas = BTreeSet::new();

            for qubit in &self.ancilla_qubits {
                if !ancillas.insert(*qubit) {
                    return Err(SyndromeSchedulingError::DuplicateQubit {
                        stabilizer: self.stabilizer(),
                        qubit: *qubit,
                    });
                }

                if data.contains(qubit) {
                    return Err(SyndromeSchedulingError::DataAncillaAlias {
                        stabilizer: self.stabilizer(),
                        qubit: *qubit,
                    });
                }
            }
        } else if !matches!(
            self.ancilla_preparation,
            AncillaPreparation::None
        ) {
            return Err(SyndromeSchedulingError::MissingAncilla {
                stabilizer: self.stabilizer(),
            });
        }

        let mut dependencies = BTreeSet::new();

        for dependency in &self.dependencies {
            if !dependencies.insert(*dependency) {
                return Err(SyndromeSchedulingError::DuplicateStabilizer {
                    stabilizer: dependency.stabilizer(),
                });
            }

            if *dependency == self.id {
                return Err(SyndromeSchedulingError::SelfDependency {
                    stabilizer: self.stabilizer(),
                });
            }
        }

        if matches!(self.feedback.requirement(), FeedbackRequirement::Required)
            && self.feedback.sources().is_empty()
        {
            return Err(SyndromeSchedulingError::MissingFeedbackSource {
                stabilizer: self.stabilizer(),
            });
        }

        Ok(())
    }
}

// ============================================================================
// Round
// ============================================================================

/// A collection of syndrome checks belonging to one extraction round.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyndromeRoundPlan {
    round: SyndromeRound,
    checks: Vec<SyndromeCheck>,
}

impl SyndromeRoundPlan {
    /// Creates an empty round.
    #[must_use]
    pub const fn new(round: SyndromeRound) -> Self {
        Self {
            round,
            checks: Vec::new(),
        }
    }

    /// Returns the round identifier.
    #[must_use]
    pub const fn round(&self) -> SyndromeRound {
        self.round
    }

    /// Returns the checks in canonical insertion order.
    #[must_use]
    pub fn checks(&self) -> &[SyndromeCheck] {
        &self.checks
    }

    /// Adds a check.
    pub fn push(&mut self, check: SyndromeCheck) -> SyndromeResult<()> {
        if check.round() != self.round {
            return Err(SyndromeSchedulingError::InvalidRoundDependency {
                predecessor: check.stabilizer(),
                successor: check.stabilizer(),
            });
        }

        if self
            .checks
            .iter()
            .any(|existing| existing.id() == check.id())
        {
            return Err(SyndromeSchedulingError::DuplicateStabilizer {
                stabilizer: check.stabilizer(),
            });
        }

        check.validate()?;
        self.checks.push(check);

        Ok(())
    }

    /// Returns the number of checks in the round.
    #[must_use]
    pub fn len(&self) -> usize {
        self.checks.len()
    }

    /// Returns whether the round has no checks.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.checks.is_empty()
    }
}

// ============================================================================
// Complete extraction plan
// ============================================================================

/// Complete scheduling-side syndrome extraction plan.
///
/// This is intentionally independent of a scheduling algorithm. The generic
/// scheduler consumes this plan through an adapter and decides exact timing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyndromeExtractionPlan {
    rounds: Vec<SyndromeRoundPlan>,
    require_complete_rounds: bool,
}

impl SyndromeExtractionPlan {
    /// Creates an empty plan.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            rounds: Vec::new(),
            require_complete_rounds: false,
        }
    }

    /// Creates an empty plan that requires every explicitly represented round
    /// to contain at least one check.
    #[must_use]
    pub const fn requiring_non_empty_rounds() -> Self {
        Self {
            rounds: Vec::new(),
            require_complete_rounds: true,
        }
    }

    /// Returns all round plans.
    #[must_use]
    pub fn rounds(&self) -> &[SyndromeRoundPlan] {
        &self.rounds
    }

    /// Returns the number of represented rounds.
    #[must_use]
    pub fn round_count(&self) -> usize {
        self.rounds.len()
    }

    /// Returns the number of syndrome checks across all rounds.
    #[must_use]
    pub fn check_count(&self) -> usize {
        self.rounds.iter().map(SyndromeRoundPlan::len).sum()
    }

    /// Returns whether the plan has no rounds.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rounds.is_empty()
    }

    /// Adds a complete round.
    pub fn push_round(
        &mut self,
        round: SyndromeRoundPlan,
    ) -> SyndromeResult<()> {
        if self
            .rounds
            .iter()
            .any(|existing| existing.round() == round.round())
        {
            return Err(SyndromeSchedulingError::DuplicateStabilizer {
                stabilizer: round
                    .checks()
                    .first()
                    .map(SyndromeCheck::stabilizer)
                    .unwrap_or_else(|| StabilizerId::new(0)),
            });
        }

        if self.require_complete_rounds && round.is_empty() {
            return Err(SyndromeSchedulingError::EmptyPlan);
        }

        self.rounds.push(round);

        Ok(())
    }

    /// Validates the entire extraction plan.
    ///
    /// Validation is deliberately independent of hardware timing and
    /// topology. Those are supplied later by the scheduling context.
    pub fn validate(&self) -> SyndromeResult<()> {
        if self.rounds.is_empty() {
            return Err(SyndromeSchedulingError::EmptyPlan);
        }

        let mut stabilizers = BTreeSet::new();
        let mut round_numbers = BTreeSet::new();

        for round in &self.rounds {
            if !round_numbers.insert(round.round()) {
                return Err(SyndromeSchedulingError::DependencyCycle);
            }

            if self.require_complete_rounds && round.is_empty() {
                return Err(SyndromeSchedulingError::EmptyPlan);
            }

            for check in round.checks() {
                check.validate()?;

                if !stabilizers.insert(check.id()) {
                    return Err(SyndromeSchedulingError::DuplicateStabilizer {
                        stabilizer: check.stabilizer(),
                    });
                }
            }
        }

        // Validate dependency references and round monotonicity.
        for round in &self.rounds {
            for check in round.checks() {
                for dependency in check.dependencies() {
                    let Some(predecessor) =
                        self.find_check(*dependency)
                    else {
                        return Err(
                            SyndromeSchedulingError::UnknownDependency {
                                stabilizer: check.stabilizer(),
                                dependency: dependency.stabilizer(),
                            },
                        );
                    };

                    if predecessor.round().value() >= check.round().value() {
                        return Err(
                            SyndromeSchedulingError::InvalidRoundDependency {
                                predecessor: predecessor.stabilizer(),
                                successor: check.stabilizer(),
                            },
                        );
                    }
                }
            }
        }

        self.validate_acyclic()?;

        Ok(())
    }

    /// Finds a syndrome check by its scheduling identifier.
    #[must_use]
    pub fn find_check(
        &self,
        id: SyndromeCheckId,
    ) -> Option<&SyndromeCheck> {
        self.rounds
            .iter()
            .flat_map(SyndromeRoundPlan::checks)
            .find(|check| check.id() == id)
    }

    /// Returns the canonical set of all physical/logical qubit identities
    /// referenced by the plan.
    ///
    /// No new qubit identity is created here.
    #[must_use]
    pub fn qubits(&self) -> BTreeSet<QubitId> {
        let mut result = BTreeSet::new();

        for round in &self.rounds {
            for check in round.checks() {
                result.extend(check.data_qubits().iter().copied());
                result.extend(check.ancilla_qubits().iter().copied());
            }
        }

        result
    }

    /// Returns all stabilizers referenced by this plan.
    #[must_use]
    pub fn stabilizers(&self) -> BTreeSet<StabilizerId> {
        self.rounds
            .iter()
            .flat_map(SyndromeRoundPlan::checks)
            .map(SyndromeCheck::stabilizer)
            .collect()
    }

    /// Returns all checks in deterministic `(round, stabilizer)` order.
    #[must_use]
    pub fn checks_deterministic(&self) -> Vec<&SyndromeCheck> {
        let mut checks: Vec<&SyndromeCheck> = self
            .rounds
            .iter()
            .flat_map(SyndromeRoundPlan::checks)
            .collect();

        checks.sort_by_key(|check| (check.round(), check.stabilizer()));

        checks
    }

    /// Detects cycles using iterative graph traversal.
    ///
    /// No recursive traversal is used, which prevents call-stack growth from
    /// becoming a scalability bottleneck for very large QEC dependency graphs.
    fn validate_acyclic(&self) -> SyndromeResult<()> {
        let checks = self.checks_deterministic();

        let mut state = std::collections::BTreeMap::<
            SyndromeCheckId,
            VisitState,
        >::new();

        for check in &checks {
            state.insert(check.id(), VisitState::Unvisited);
        }

        for check in &checks {
            if state.get(&check.id()) == Some(&VisitState::Visited) {
                continue;
            }

            let mut stack: Vec<(SyndromeCheckId, usize)> =
                vec![(check.id(), 0)];

            while let Some((current, next_dependency)) = stack.last_mut() {
                if *next_dependency == 0 {
                    state.insert(*current, VisitState::Active);
                }

                let Some(current_check) = self.find_check(*current) else {
                    return Err(SyndromeSchedulingError::UnknownDependency {
                        stabilizer: *current.stabilizer(),
                        dependency: *current.stabilizer(),
                    });
                };

                if *next_dependency < current_check.dependencies().len() {
                    let dependency =
                        current_check.dependencies()[*next_dependency];

                    *next_dependency += 1;

                    match state.get(&dependency) {
                        Some(VisitState::Active) => {
                            return Err(
                                SyndromeSchedulingError::DependencyCycle
                            );
                        }
                        Some(VisitState::Visited) => {}
                        Some(VisitState::Unvisited) => {
                            stack.push((dependency, 0));
                        }
                        None => {
                            return Err(
                                SyndromeSchedulingError::UnknownDependency {
                                    stabilizer: current_check.stabilizer(),
                                    dependency: dependency.stabilizer(),
                                },
                            );
                        }
                    }
                } else {
                    state.insert(*current, VisitState::Visited);
                    stack.pop();
                }
            }
        }

        Ok(())
    }
}

impl Default for SyndromeExtractionPlan {
    fn default() -> Self {
        Self::new()
    }
}

/// Internal graph traversal state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VisitState {
    Unvisited,
    Active,
    Visited,
}

// ============================================================================
// Incremental builder
// ============================================================================

/// Incremental builder for large syndrome extraction plans.
///
/// This avoids requiring callers to construct the complete plan in one
/// expression and provides an explicit location for future streaming or
/// bounded-memory construction policies.
#[derive(Debug, Default)]
pub struct SyndromePlanBuilder {
    plan: SyndromeExtractionPlan,
}

impl SyndromePlanBuilder {
    /// Creates a new builder.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            plan: SyndromeExtractionPlan::new(),
        }
    }

    /// Creates a builder requiring non-empty rounds.
    #[must_use]
    pub const fn requiring_non_empty_rounds() -> Self {
        Self {
            plan: SyndromeExtractionPlan::requiring_non_empty_rounds(),
        }
    }

    /// Adds a round.
    pub fn push_round(
        &mut self,
        round: SyndromeRoundPlan,
    ) -> SyndromeResult<()> {
        self.plan.push_round(round)
    }

    /// Returns the number of accumulated rounds.
    #[must_use]
    pub fn round_count(&self) -> usize {
        self.plan.round_count()
    }

    /// Returns the number of accumulated checks.
    #[must_use]
    pub fn check_count(&self) -> usize {
        self.plan.check_count()
    }

    /// Validates and consumes the builder.
    pub fn build(self) -> SyndromeResult<SyndromeExtractionPlan> {
        self.plan.validate()?;
        Ok(self.plan)
    }

    /// Returns a reference to the current plan for inspection.
    #[must_use]
    pub const fn plan(&self) -> &SyndromeExtractionPlan {
        &self.plan
    }
}

// ============================================================================
// Scheduler-facing requirements
// ============================================================================

/// A normalized dependency edge produced from a syndrome plan.
///
/// This is intentionally independent from the generic scheduling dependency
/// type. `adapters::qec` can convert it into the scheduler's canonical
/// dependency representation without making this QEC module depend on the
/// planner implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SyndromeDependency {
    predecessor: SyndromeCheckId,
    successor: SyndromeCheckId,
}

impl SyndromeDependency {
    /// Creates a dependency edge.
    #[must_use]
    pub const fn new(
        predecessor: SyndromeCheckId,
        successor: SyndromeCheckId,
    ) -> Self {
        Self {
            predecessor,
            successor,
        }
    }

    /// Returns the predecessor.
    #[must_use]
    pub const fn predecessor(self) -> SyndromeCheckId {
        self.predecessor
    }

    /// Returns the successor.
    #[must_use]
    pub const fn successor(self) -> SyndromeCheckId {
        self.successor
    }
}

/// A scheduler-facing view of a complete syndrome plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyndromeSchedulingRequirements {
    checks: Vec<SyndromeCheckId>,
    dependencies: Vec<SyndromeDependency>,
    qubits: BTreeSet<QubitId>,
}

impl SyndromeSchedulingRequirements {
    /// Builds scheduling requirements from a validated syndrome plan.
    pub fn from_plan(
        plan: &SyndromeExtractionPlan,
    ) -> SyndromeResult<Self> {
        plan.validate()?;

        let checks = plan
            .checks_deterministic()
            .into_iter()
            .map(SyndromeCheck::id)
            .collect::<Vec<_>>();

        let mut dependencies = Vec::new();

        for check in plan.checks_deterministic() {
            for predecessor in check.dependencies() {
                dependencies.push(SyndromeDependency::new(
                    *predecessor,
                    check.id(),
                ));
            }
        }

        dependencies.sort_by_key(|dependency| {
            (
                dependency.predecessor(),
                dependency.successor(),
            )
        });

        Ok(Self {
            checks,
            dependencies,
            qubits: plan.qubits(),
        })
    }

    /// Returns all checks in deterministic order.
    #[must_use]
    pub fn checks(&self) -> &[SyndromeCheckId] {
        &self.checks
    }

    /// Returns dependency edges in deterministic order.
    #[must_use]
    pub fn dependencies(&self) -> &[SyndromeDependency] {
        &self.dependencies
    }

    /// Returns all referenced canonical qubit identities.
    #[must_use]
    pub fn qubits(&self) -> &BTreeSet<QubitId> {
        &self.qubits
    }

    /// Returns whether no scheduling requirements exist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.checks.is_empty()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn stabilizer(value: usize) -> StabilizerId {
        StabilizerId::new(value)
    }

    // The canonical QubitId constructor is intentionally not assumed here.
    // Tests that need concrete QubitId values should be supplied by the
    // canonical IR test suite or an integration fixture where the actual
    // QubitId construction API is known.

    #[test]
    fn round_next_is_checked() {
        let round = SyndromeRound::new(10);

        assert_eq!(round.next().expect("round should advance").value(), 11);
    }

    #[test]
    fn check_requires_data_qubits() {
        let check = SyndromeCheck::new(
            stabilizer(0),
            SyndromeRound::new(0),
            SyndromeBasis::Z,
        );

        assert!(matches!(
            check.validate(),
            Err(SyndromeSchedulingError::EmptyDataQubits { .. })
        ));
    }

    #[test]
    fn self_dependency_is_rejected() {
        let id = SyndromeCheckId::new(stabilizer(0));

        let check = SyndromeCheck::new(
            stabilizer(0),
            SyndromeRound::new(0),
            SyndromeBasis::Z,
        )
        .depends_on(id);

        assert!(matches!(
            check.validate(),
            Err(SyndromeSchedulingError::SelfDependency { .. })
        ));
    }

    #[test]
    fn required_feedback_needs_source() {
        let feedback =
            SyndromeFeedback::required(Vec::new());

        assert_eq!(
            feedback.requirement(),
            FeedbackRequirement::Required
        );
        assert!(feedback.sources().is_empty());
    }

    #[test]
    fn empty_plan_is_rejected() {
        let plan = SyndromeExtractionPlan::new();

        assert!(matches!(
            plan.validate(),
            Err(SyndromeSchedulingError::EmptyPlan)
        ));
    }

    #[test]
    fn custom_basis_is_not_treated_as_standard() {
        let basis = SyndromeBasis::Custom("code-specific".to_owned());

        assert!(!basis.is_standard());
    }
}