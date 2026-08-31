//! Zamani Quantum IR — Measurement-Based Quantum Computation Model
//!
//! Production-grade, hardware-independent semantic representation of
//! measurement-based quantum computation (MBQC), including one-way/cluster-
//! state computation, adaptive measurements, Pauli corrections, explicit
//! inputs/outputs, measurement dependencies, and deterministic validation.
//!
//! # Architectural role
//!
//! This module answers:
//!
//! > What does a measurement-based quantum computation mean?
//!
//! It does NOT decide:
//!
//! - which physical qubits implement the logical qubits;
//! - which cluster-state hardware is used;
//! - which entangling primitive is native;
//! - how measurements are physically performed;
//! - which pulse sequence implements a measurement;
//! - how a target topology is routed;
//! - how the computation is scheduled;
//! - how error correction is performed;
//! - how a backend executes the pattern;
//! - how quantum state is simulated.
//!
//! Those responsibilities belong to downstream IR consumers.
//!
//! # Canonical dependencies
//!
//! ```text
//! quantum::ir::qubit::QubitId
//!     = canonical logical-qubit identity
//!
//! quantum::ir::parameter::Parameter
//!     = canonical scalar/symbolic parameter
//!
//! measurement_based.rs
//!     = MBQC semantic model
//!
//! routing / hardware / scheduling / backend
//!     = target-specific lowering
//! ```
//!
//! This module intentionally uses:
//!
//! ```rust
//! use crate::quantum::ir::qubit::QubitId;
//! ```
//!
//! and never defines another qubit identifier.
//!
//! # Measurement-based model
//!
//! A pattern is represented as a sequence of semantic commands:
//!
//! ```text
//! Prepare
//!     ↓
//! Entangle
//!     ↓
//! Measure
//!     ↓
//! Adaptive correction
//!     ↓
//! Output
//! ```
//!
//! The representation also permits the compiler to preserve explicit
//! dependency information rather than hiding adaptive behavior in an opaque
//! callback.
//!
//! # Standard MBQC concepts
//!
//! The model can represent:
//!
//! - input qubits;
//! - output qubits;
//! - prepared qubits;
//! - |0>, |1>, |+>, |-> preparation;
//! - CZ entanglement;
//! - arbitrary measurement planes;
//! - XY-plane measurements;
//! - XZ-plane measurements;
//! - YZ-plane measurements;
//! - symbolic measurement angles;
//! - measurement signal domains;
//! - adaptive X corrections;
//! - adaptive Z corrections;
//! - classical parity dependencies;
//! - measurement-result dependencies;
//! - deterministic command ordering;
//! - explicit pattern validation;
//! - open patterns with unresolved symbolic parameters;
//! - arbitrary finite logical qubit counts permitted by available resources.
//!
//! # Important scalability rule
//!
//! There is no architectural qubit limit in this module.
//!
//! This module does NOT contain:
//!
//! ```text
//! MAX_QUBITS
//! MAX_CLUSTER_SIZE
//! MAX_MEASUREMENTS
//! MAX_PATTERN_SIZE
//! ```
//!
//! Any concrete limit must be imposed by the surrounding compilation,
//! validation, memory, or security policy.
//!
//! The representation is therefore the same semantic model for:
//!
//! ```text
//! 1 qubit
//! 10 qubits
//! 1,000 qubits
//! 1,000,000 qubits
//! N qubits
//! ```
//!
//! subject only to the resources available to the process.
//!
//! # Determinism
//!
//! Semantic command order is represented by `Vec<MeasurementBasedCommand>`.
//!
//! Sets of qubits used for classical signal domains are represented with
//! `BTreeSet` so that equality, validation and deterministic serialization
//! boundaries do not depend on hash-map iteration order.
//!
//! # No hidden classical state
//!
//! Adaptive behavior is represented explicitly through `SignalDomain`.
//!
//! For example:
//!
//! ```text
//! M(q0, alpha)
//! X(q1, s={q0})
//! ```
//!
//! means that the X correction on `q1` is controlled by the parity of the
//! measurement result of `q0`.
//!
//! No global measurement-result store exists in this module.
//!
//! # No physical assumptions
//!
//! `Entangle` means semantic entanglement according to the selected MBQC
//! pattern. It does not mean that a physical device necessarily implements a
//! native CZ gate.
//!
//! A downstream target may lower it to:
//!
//! ```text
//! CZ
//! CNOT + H
//! native entangler
//! calibrated pulse sequence
//! photonic interaction
//! cluster-state preparation primitive
//! ```
//!
//! without changing this module.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the safety requirement compiler-enforced.
//!
//! # Integration contract
//!
//! `quantum::ir::qubit`
//!     Supplies `QubitId`.
//!
//! `quantum::ir::parameter`
//!     Supplies `Parameter` for concrete and symbolic measurement angles.
//!
//! `quantum::ir::model::mod`
//!     Should expose this module as `measurement_based`.
//!
//! `quantum::ir::operation`
//!     May wrap this model as an extensible/model-specific operation rather
//!     than redefining these types.
//!
//! `quantum::ir::program`
//!     May contain an MBQC pattern as a program-level model.
//!
//! `quantum::ir::validation`
//!     May invoke `MeasurementBasedPattern::validate` as part of complete IR
//!     validation.
//!
//! `quantum::ir::serialization`
//!     Owns canonical persistence. This module intentionally does not create
//!     a second serialization format.
//!
//! `quantum::ir::hash`
//!     May hash the deterministic structural contents of this model.
//!
//! `quantum::ir::mapping`
//!     May map the logical qubits represented here to physical resources.
//!
//! `quantum::ir::resource` / `capability`
//!     May express target requirements such as adaptive measurement,
//!     feed-forward, and entanglement support.
//!
//! `quantum::ir::routing`
//!     May transform the logical interaction structure into target topology.
//!
//! `quantum::ir::scheduling`
//!     May derive physical execution timing.
//!
//! No dependency in the opposite direction is permitted.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeSet;
use std::fmt;

use crate::quantum::ir::parameter::Parameter;
use crate::quantum::ir::qubit::QubitId;

// =============================================================================
// Result
// =============================================================================

/// Result type returned by MBQC construction and validation.
pub type MeasurementBasedResult<T> = Result<T, MeasurementBasedError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the measurement-based quantum computation model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeasurementBasedError {
    /// A pattern contains no semantic commands.
    EmptyPattern,

    /// A pattern contains an input qubit more than once.
    DuplicateInputQubit {
        /// Duplicated qubit.
        qubit: QubitId,
    },

    /// A pattern contains an output qubit more than once.
    DuplicateOutputQubit {
        /// Duplicated qubit.
        qubit: QubitId,
    },

    /// A command contains duplicate qubit operands where uniqueness is
    /// required.
    DuplicateQubitOperand {
        /// Duplicated qubit.
        qubit: QubitId,
    },

    /// A command references a qubit that has not been introduced into the
    /// pattern namespace.
    UnknownQubit {
        /// Referenced qubit.
        qubit: QubitId,
    },

    /// An input is also explicitly prepared without being allowed by the
    /// pattern semantics.
    InputPreparationConflict {
        /// Conflicting qubit.
        qubit: QubitId,
    },

    /// An output is marked as measured.
    MeasuredOutput {
        /// Output qubit.
        qubit: QubitId,
    },

    /// A measurement command contains an empty signal domain where one was
    /// semantically required.
    InvalidSignalDomain,

    /// A signal domain contains a qubit that has not yet been measured.
    FutureMeasurementDependency {
        /// Referenced dependency.
        dependency: QubitId,

        /// Measurement that attempts to consume it.
        consumer: QubitId,
    },

    /// A signal domain contains duplicate semantic dependencies.
    DuplicateSignalDependency {
        /// Duplicated dependency.
        dependency: QubitId,
    },

    /// A qubit is measured more than once.
    MultipleMeasurements {
        /// Qubit measured more than once.
        qubit: QubitId,
    },

    /// A qubit is prepared more than once.
    MultiplePreparations {
        /// Qubit prepared more than once.
        qubit: QubitId,
    },

    /// A qubit is entangled with itself.
    SelfEntanglement {
        /// Invalid qubit.
        qubit: QubitId,
    },

    /// An entanglement edge occurs more than once in the same pattern.
    DuplicateEntanglement {
        /// First endpoint.
        first: QubitId,

        /// Second endpoint.
        second: QubitId,
    },

    /// An operation appears after its semantic lifetime has ended.
    OperationAfterMeasurement {
        /// Qubit whose lifetime was already consumed by measurement.
        qubit: QubitId,
    },

    /// A correction is applied to a qubit that has already been measured.
    CorrectionAfterMeasurement {
        /// Corrected qubit.
        qubit: QubitId,
    },

    /// An output qubit is not part of the pattern namespace.
    UnknownOutput {
        /// Output qubit.
        qubit: QubitId,
    },

    /// An input qubit is not part of the pattern namespace.
    UnknownInput {
        /// Input qubit.
        qubit: QubitId,
    },

    /// A measurement angle is invalid.
    InvalidMeasurementAngle {
        /// Measurement qubit.
        qubit: QubitId,
    },

    /// A parameter validation error was encountered.
    InvalidParameter {
        /// Measurement qubit, when available.
        qubit: Option<QubitId>,

        /// Human-readable reason.
        message: String,
    },

    /// A pattern violates a structural invariant.
    InvalidPattern {
        /// Stable explanation.
        message: &'static str,
    },

    /// An arithmetic operation required by validation overflowed.
    ArithmeticOverflow {
        /// Description of the calculation.
        calculation: &'static str,
    },
}

impl fmt::Display for MeasurementBasedError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPattern => {
                formatter.write_str("measurement-based pattern is empty")
            }

            Self::DuplicateInputQubit { qubit } => {
                write!(formatter, "duplicate MBQC input qubit {qubit}")
            }

            Self::DuplicateOutputQubit { qubit } => {
                write!(formatter, "duplicate MBQC output qubit {qubit}")
            }

            Self::DuplicateQubitOperand { qubit } => {
                write!(
                    formatter,
                    "duplicate qubit operand {qubit} in MBQC command"
                )
            }

            Self::UnknownQubit { qubit } => {
                write!(
                    formatter,
                    "MBQC command references unknown qubit {qubit}"
                )
            }

            Self::InputPreparationConflict { qubit } => {
                write!(
                    formatter,
                    "input qubit {qubit} cannot also be explicitly prepared"
                )
            }

            Self::MeasuredOutput { qubit } => {
                write!(
                    formatter,
                    "output qubit {qubit} cannot be measured in the pattern"
                )
            }

            Self::InvalidSignalDomain => {
                formatter.write_str("invalid MBQC signal domain")
            }

            Self::FutureMeasurementDependency {
                dependency,
                consumer,
            } => {
                write!(
                    formatter,
                    "measurement dependency {dependency} is not available before consuming measurement {consumer}"
                )
            }

            Self::DuplicateSignalDependency { dependency } => {
                write!(
                    formatter,
                    "duplicate signal dependency {dependency}"
                )
            }

            Self::MultipleMeasurements { qubit } => {
                write!(
                    formatter,
                    "logical qubit {qubit} is measured more than once"
                )
            }

            Self::MultiplePreparations { qubit } => {
                write!(
                    formatter,
                    "logical qubit {qubit} is prepared more than once"
                )
            }

            Self::SelfEntanglement { qubit } => {
                write!(
                    formatter,
                    "logical qubit {qubit} cannot be entangled with itself"
                )
            }

            Self::DuplicateEntanglement { first, second } => {
                write!(
                    formatter,
                    "duplicate MBQC entanglement edge between {first} and {second}"
                )
            }

            Self::OperationAfterMeasurement { qubit } => {
                write!(
                    formatter,
                    "operation references logical qubit {qubit} after measurement"
                )
            }

            Self::CorrectionAfterMeasurement { qubit } => {
                write!(
                    formatter,
                    "correction targets logical qubit {qubit} after measurement"
                )
            }

            Self::UnknownOutput { qubit } => {
                write!(
                    formatter,
                    "MBQC output qubit {qubit} is not declared by the pattern"
                )
            }

            Self::UnknownInput { qubit } => {
                write!(
                    formatter,
                    "MBQC input qubit {qubit} is not declared by the pattern"
                )
            }

            Self::InvalidMeasurementAngle { qubit } => {
                write!(
                    formatter,
                    "measurement angle for qubit {qubit} is invalid"
                )
            }

            Self::InvalidParameter { qubit, message } => {
                if let Some(qubit) = qubit {
                    write!(
                        formatter,
                        "invalid parameter for MBQC qubit {qubit}: {message}"
                    )
                } else {
                    write!(
                        formatter,
                        "invalid MBQC parameter: {message}"
                    )
                }
            }

            Self::InvalidPattern { message } => {
                write!(formatter, "invalid MBQC pattern: {message}")
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }
        }
    }
}

impl std::error::Error for MeasurementBasedError {}

// =============================================================================
// Measurement plane
// =============================================================================

/// Plane in which a qubit is measured.
///
/// MBQC measurement planes are semantic mathematical concepts. They are not
/// hardware coordinate systems.
///
/// The canonical Bloch-sphere planes are:
///
/// - XY;
/// - XZ;
/// - YZ.
///
/// A backend is responsible for determining how a particular plane is
/// physically realized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MeasurementPlane {
    /// Measurement in the XY plane.
    XY,

    /// Measurement in the XZ plane.
    XZ,

    /// Measurement in the YZ plane.
    YZ,
}

impl MeasurementPlane {
    /// Returns the canonical textual name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::XY => "XY",
            Self::XZ => "XZ",
            Self::YZ => "YZ",
        }
    }
}

impl fmt::Display for MeasurementPlane {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Preparation
// =============================================================================

/// Semantic initial preparation of a qubit for an MBQC pattern.
///
/// Input qubits are normally supplied by the program and therefore do not
/// require a `Prepare` command. Non-input qubits generally do.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PreparationState {
    /// Computational-basis |0>.
    Zero,

    /// Computational-basis |1>.
    One,

    /// |+> = (|0> + |1>) / sqrt(2).
    Plus,

    /// |-> = (|0> - |1>) / sqrt(2).
    Minus,
}

impl PreparationState {
    /// Returns whether this preparation is an X-basis eigenstate.
    #[must_use]
    pub const fn is_x_basis(self) -> bool {
        matches!(self, Self::Plus | Self::Minus)
    }

    /// Returns whether this preparation is a Z-basis eigenstate.
    #[must_use]
    pub const fn is_z_basis(self) -> bool {
        matches!(self, Self::Zero | Self::One)
    }
}

// =============================================================================
// Signal domain
// =============================================================================

/// Classical parity dependency used by adaptive MBQC.
///
/// An MBQC measurement may depend on previous measurement outcomes.
///
/// For example:
///
/// ```text
/// s = q0 XOR q2 XOR q7
/// ```
///
/// is represented as a signal domain containing those measured qubits.
///
/// `x` and `z` domains are kept separate because MBQC correction semantics
/// distinguish the two classical signal channels.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SignalDomain {
    x: BTreeSet<QubitId>,
    z: BTreeSet<QubitId>,
}

impl Default for SignalDomain {
    fn default() -> Self {
        Self::new()
    }
}

impl SignalDomain {
    /// Creates an empty signal domain.
    #[must_use]
    pub fn new() -> Self {
        Self {
            x: BTreeSet::new(),
            z: BTreeSet::new(),
        }
    }

    /// Creates a signal domain from X and Z dependency collections.
    ///
    /// The collections are deduplicated automatically.
    #[must_use]
    pub fn from_sets(
        x: impl IntoIterator<Item = QubitId>,
        z: impl IntoIterator<Item = QubitId>,
    ) -> Self {
        Self {
            x: x.into_iter().collect(),
            z: z.into_iter().collect(),
        }
    }

    /// Adds an X-domain dependency.
    ///
    /// Returns `true` if the dependency was newly inserted.
    pub fn add_x(&mut self, qubit: QubitId) -> bool {
        self.x.insert(qubit)
    }

    /// Adds a Z-domain dependency.
    ///
    /// Returns `true` if the dependency was newly inserted.
    pub fn add_z(&mut self, qubit: QubitId) -> bool {
        self.z.insert(qubit)
    }

    /// Removes an X-domain dependency.
    pub fn remove_x(&mut self, qubit: QubitId) -> bool {
        self.x.remove(&qubit)
    }

    /// Removes a Z-domain dependency.
    pub fn remove_z(&mut self, qubit: QubitId) -> bool {
        self.z.remove(&qubit)
    }

    /// Returns the X signal domain.
    #[must_use]
    pub fn x(&self) -> &BTreeSet<QubitId> {
        &self.x
    }

    /// Returns the Z signal domain.
    #[must_use]
    pub fn z(&self) -> &BTreeSet<QubitId> {
        &self.z
    }

    /// Returns the total number of unique dependencies.
    ///
    /// If a qubit appears in both domains it is counted twice because the
    /// domains have distinct semantic roles.
    pub fn dependency_count(&self) -> usize {
        self.x.len().saturating_add(self.z.len())
    }

    /// Returns whether no dependencies are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.x.is_empty() && self.z.is_empty()
    }

    /// Returns whether the supplied qubit occurs in either domain.
    #[must_use]
    pub fn contains(&self, qubit: QubitId) -> bool {
        self.x.contains(&qubit) || self.z.contains(&qubit)
    }

    /// Returns all dependencies in deterministic order.
    ///
    /// A qubit appearing in both domains occurs only once in the returned
    /// vector.
    pub fn dependencies(&self) -> Vec<QubitId> {
        self.x
            .union(&self.z)
            .copied()
            .collect()
    }
}

// =============================================================================
// Measurement specification
// =============================================================================

/// Complete semantic measurement specification for one MBQC measurement.
///
/// The angle is represented by the canonical Zamani [`Parameter`] type so
/// measurement angles may remain symbolic until a later compilation stage.
///
/// This is essential for program-once/target-later compilation.
///
/// Examples:
///
/// ```text
/// pi/2
/// theta
/// theta + phi
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementSpecification {
    plane: MeasurementPlane,
    angle: Parameter,
    domain: SignalDomain,
}

impl MeasurementSpecification {
    /// Creates a measurement specification with no adaptive dependency.
    pub fn new(
        plane: MeasurementPlane,
        angle: Parameter,
    ) -> MeasurementBasedResult<Self> {
        angle
            .validate()
            .map_err(|error| MeasurementBasedError::InvalidParameter {
                qubit: None,
                message: error.to_string(),
            })?;

        Ok(Self {
            plane,
            angle,
            domain: SignalDomain::new(),
        })
    }

    /// Creates a measurement specification with an explicit signal domain.
    pub fn with_domain(
        plane: MeasurementPlane,
        angle: Parameter,
        domain: SignalDomain,
    ) -> MeasurementBasedResult<Self> {
        angle
            .validate()
            .map_err(|error| MeasurementBasedError::InvalidParameter {
                qubit: None,
                message: error.to_string(),
            })?;

        Ok(Self {
            plane,
            angle,
            domain,
        })
    }

    /// Returns the measurement plane.
    #[must_use]
    pub const fn plane(&self) -> MeasurementPlane {
        self.plane
    }

    /// Returns the measurement angle.
    #[must_use]
    pub fn angle(&self) -> &Parameter {
        &self.angle
    }

    /// Returns the adaptive signal domain.
    #[must_use]
    pub fn domain(&self) -> &SignalDomain {
        &self.domain
    }

    /// Returns a mutable reference to the signal domain.
    pub fn domain_mut(&mut self) -> &mut SignalDomain {
        &mut self.domain
    }

    /// Returns whether the measurement is adaptive.
    #[must_use]
    pub fn is_adaptive(&self) -> bool {
        !self.domain.is_empty()
    }

    /// Validates the specification independently of a complete pattern.
    pub fn validate(&self) -> MeasurementBasedResult<()> {
        self.angle
            .validate()
            .map_err(|error| MeasurementBasedError::InvalidParameter {
                qubit: None,
                message: error.to_string(),
            })
    }
}

// =============================================================================
// MBQC command
// =============================================================================

/// Semantic command in a measurement-based quantum computation pattern.
///
/// The command vocabulary deliberately describes MBQC semantics rather than
/// physical hardware instructions.
///
/// The ordering of commands in the pattern is significant.
#[derive(Debug, Clone, PartialEq)]
pub enum MeasurementBasedCommand {
    /// Prepare a non-input logical qubit.
    Prepare {
        /// Logical qubit being prepared.
        qubit: QubitId,

        /// Semantic preparation state.
        state: PreparationState,
    },

    /// Entangle two logical qubits.
    ///
    /// The canonical MBQC interpretation is a CZ-equivalent entangling edge.
    /// A backend is free to realize this through another native interaction.
    Entangle {
        /// First endpoint.
        first: QubitId,

        /// Second endpoint.
        second: QubitId,
    },

    /// Measure one logical qubit.
    Measure {
        /// Logical qubit being measured.
        qubit: QubitId,

        /// Measurement specification.
        specification: MeasurementSpecification,
    },

    /// Apply an X correction controlled by a classical signal domain.
    CorrectX {
        /// Logical qubit being corrected.
        qubit: QubitId,

        /// Classical signal controlling the correction.
        domain: SignalDomain,
    },

    /// Apply a Z correction controlled by a classical signal domain.
    CorrectZ {
        /// Logical qubit being corrected.
        qubit: QubitId,

        /// Classical signal controlling the correction.
        domain: SignalDomain,
    },
}

impl MeasurementBasedCommand {
    /// Returns the primary command qubit when one exists.
    ///
    /// For `Entangle`, this returns the first endpoint. Use
    /// [`Self::qubits`] when all operands are required.
    #[must_use]
    pub const fn primary_qubit(&self) -> QubitId {
        match self {
            Self::Prepare { qubit, .. }
            | Self::Measure { qubit, .. }
            | Self::CorrectX { qubit, .. }
            | Self::CorrectZ { qubit, .. } => *qubit,

            Self::Entangle { first, .. } => *first,
        }
    }

    /// Returns all logical qubit operands used by this command.
    ///
    /// The returned vector preserves semantic operand order.
    #[must_use]
    pub fn qubits(&self) -> Vec<QubitId> {
        match self {
            Self::Prepare { qubit, .. }
            | Self::Measure { qubit, .. }
            | Self::CorrectX { qubit, .. }
            | Self::CorrectZ { qubit, .. } => vec![*qubit],

            Self::Entangle { first, second } => {
                vec![*first, *second]
            }
        }
    }

    /// Returns the command kind without exposing command payloads.
    #[must_use]
    pub const fn kind(&self) -> MeasurementBasedCommandKind {
        match self {
            Self::Prepare { .. } => {
                MeasurementBasedCommandKind::Prepare
            }

            Self::Entangle { .. } => {
                MeasurementBasedCommandKind::Entangle
            }

            Self::Measure { .. } => {
                MeasurementBasedCommandKind::Measure
            }

            Self::CorrectX { .. } => {
                MeasurementBasedCommandKind::CorrectX
            }

            Self::CorrectZ { .. } => {
                MeasurementBasedCommandKind::CorrectZ
            }
        }
    }

    /// Returns whether this command produces a measurement outcome.
    #[must_use]
    pub const fn produces_measurement(&self) -> bool {
        matches!(self, Self::Measure { .. })
    }

    /// Returns whether this command consumes a classical measurement signal.
    #[must_use]
    pub fn consumes_signal(&self) -> bool {
        match self {
            Self::Measure { specification, .. } => {
                specification.is_adaptive()
            }

            Self::CorrectX { domain, .. }
            | Self::CorrectZ { domain, .. } => {
                !domain.is_empty()
            }

            Self::Prepare { .. } | Self::Entangle { .. } => false,
        }
    }

    /// Returns the signal domain consumed by this command, when present.
    #[must_use]
    pub fn signal_domain(&self) -> Option<&SignalDomain> {
        match self {
            Self::Measure { specification, .. } => {
                Some(specification.domain())
            }

            Self::CorrectX { domain, .. }
            | Self::CorrectZ { domain, .. } => Some(domain),

            Self::Prepare { .. } | Self::Entangle { .. } => None,
        }
    }
}

/// Stable classification of MBQC commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MeasurementBasedCommandKind {
    /// Qubit preparation.
    Prepare,

    /// Entanglement creation.
    Entangle,

    /// Measurement.
    Measure,

    /// X correction.
    CorrectX,

    /// Z correction.
    CorrectZ,
}

impl MeasurementBasedCommandKind {
    /// Returns the canonical command name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Prepare => "prepare",
            Self::Entangle => "entangle",
            Self::Measure => "measure",
            Self::CorrectX => "correct_x",
            Self::CorrectZ => "correct_z",
        }
    }
}

impl fmt::Display for MeasurementBasedCommandKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Pattern metadata
// =============================================================================

/// Lightweight semantic metadata for an MBQC pattern.
///
/// This metadata deliberately does not contain backend/device information.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MeasurementBasedMetadata {
    /// Optional human-readable pattern name.
    name: Option<String>,

    /// Optional source-level identifier.
    source: Option<String>,

    /// Whether the pattern is intended to be used as a fault-tolerant
    /// logical computation.
    fault_tolerant: bool,
}

impl MeasurementBasedMetadata {
    /// Creates empty metadata.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            name: None,
            source: None,
            fault_tolerant: false,
        }
    }

    /// Returns the optional pattern name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the optional source identifier.
    #[must_use]
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    /// Returns whether the pattern is marked fault tolerant.
    #[must_use]
    pub const fn is_fault_tolerant(&self) -> bool {
        self.fault_tolerant
    }

    /// Sets the pattern name.
    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    /// Sets the source identifier.
    pub fn set_source(&mut self, source: Option<String>) {
        self.source = source;
    }

    /// Marks the pattern as fault tolerant.
    pub const fn set_fault_tolerant(&mut self, value: bool) {
        self.fault_tolerant = value;
    }
}

// =============================================================================
// Pattern
// =============================================================================

/// Canonical measurement-based quantum computation pattern.
///
/// A pattern consists of:
///
/// - logical qubit namespace;
/// - explicit input set;
/// - explicit output set;
/// - ordered MBQC commands;
/// - semantic metadata.
///
/// The type contains no physical hardware information.
///
/// # Namespace model
///
/// The qubit namespace is inferred from commands plus explicitly declared
/// inputs and outputs. This avoids materializing a `Qubit` object for every
/// logical identifier.
///
/// Consequently, a large sparse namespace remains memory-efficient.
///
/// # Ordering
///
/// Commands are executed according to their order in `commands`.
///
/// This ordering is semantic and deterministic.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementBasedPattern {
    inputs: Vec<QubitId>,
    outputs: Vec<QubitId>,
    commands: Vec<MeasurementBasedCommand>,
    metadata: MeasurementBasedMetadata,
}

impl Default for MeasurementBasedPattern {
    fn default() -> Self {
        Self::new()
    }
}

impl MeasurementBasedPattern {
    /// Creates an empty MBQC pattern.
    ///
    /// An empty pattern is constructible so callers can incrementally build
    /// one. `validate()` rejects it unless a future IR extension explicitly
    /// defines empty patterns as meaningful.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
            commands: Vec::new(),
            metadata: MeasurementBasedMetadata::new(),
        }
    }

    /// Creates a pattern from explicit inputs and outputs.
    ///
    /// Duplicate input/output identifiers are rejected immediately.
    pub fn with_io(
        inputs: Vec<QubitId>,
        outputs: Vec<QubitId>,
    ) -> MeasurementBasedResult<Self> {
        validate_unique_qubits(&inputs)?;
        validate_unique_qubits(&outputs)?;

        Ok(Self {
            inputs,
            outputs,
            commands: Vec::new(),
            metadata: MeasurementBasedMetadata::new(),
        })
    }

    /// Returns the declared logical inputs.
    #[must_use]
    pub fn inputs(&self) -> &[QubitId] {
        &self.inputs
    }

    /// Returns the declared logical outputs.
    #[must_use]
    pub fn outputs(&self) -> &[QubitId] {
        &self.outputs
    }

    /// Returns the ordered semantic command sequence.
    #[must_use]
    pub fn commands(&self) -> &[MeasurementBasedCommand] {
        &self.commands
    }

    /// Returns mutable access to metadata.
    ///
    /// Metadata does not affect MBQC semantics.
    pub fn metadata_mut(&mut self) -> &mut MeasurementBasedMetadata {
        &mut self.metadata
    }

    /// Returns metadata.
    #[must_use]
    pub fn metadata(&self) -> &MeasurementBasedMetadata {
        &self.metadata
    }

    /// Adds an input qubit.
    ///
    /// Input qubits are logical namespace declarations. They are not
    /// automatically prepared by this method.
    pub fn add_input(
        &mut self,
        qubit: QubitId,
    ) -> MeasurementBasedResult<()> {
        if self.inputs.contains(&qubit) {
            return Err(
                MeasurementBasedError::DuplicateInputQubit { qubit },
            );
        }

        self.inputs.push(qubit);
        Ok(())
    }

    /// Adds an output qubit.
    pub fn add_output(
        &mut self,
        qubit: QubitId,
    ) -> MeasurementBasedResult<()> {
        if self.outputs.contains(&qubit) {
            return Err(
                MeasurementBasedError::DuplicateOutputQubit { qubit },
            );
        }

        self.outputs.push(qubit);
        Ok(())
    }

    /// Appends a semantically validated command.
    ///
    /// This performs local validation only. Complete dependency and lifetime
    /// validation is performed by `validate()`.
    pub fn push(
        &mut self,
        command: MeasurementBasedCommand,
    ) -> MeasurementBasedResult<()> {
        validate_command_locally(&command)?;

        self.commands.push(command);
        Ok(())
    }

    /// Appends a preparation command.
    pub fn prepare(
        &mut self,
        qubit: QubitId,
        state: PreparationState,
    ) -> MeasurementBasedResult<()> {
        self.push(MeasurementBasedCommand::Prepare {
            qubit,
            state,
        })
    }

    /// Appends an entanglement command.
    pub fn entangle(
        &mut self,
        first: QubitId,
        second: QubitId,
    ) -> MeasurementBasedResult<()> {
        self.push(MeasurementBasedCommand::Entangle {
            first,
            second,
        })
    }

    /// Appends a measurement command.
    pub fn measure(
        &mut self,
        qubit: QubitId,
        specification: MeasurementSpecification,
    ) -> MeasurementBasedResult<()> {
        specification.validate()?;

        self.push(MeasurementBasedCommand::Measure {
            qubit,
            specification,
        })
    }

    /// Appends an adaptive X correction.
    pub fn correct_x(
        &mut self,
        qubit: QubitId,
        domain: SignalDomain,
    ) -> MeasurementBasedResult<()> {
        self.push(MeasurementBasedCommand::CorrectX {
            qubit,
            domain,
        })
    }

    /// Appends an adaptive Z correction.
    pub fn correct_z(
        &mut self,
        qubit: QubitId,
        domain: SignalDomain,
    ) -> MeasurementBasedResult<()> {
        self.push(MeasurementBasedCommand::CorrectZ {
            qubit,
            domain,
        })
    }

    /// Returns all logical qubits referenced by this pattern.
    ///
    /// The result is deterministic and deduplicated.
    ///
    /// This operation scans the pattern but does not allocate one object per
    /// qubit beyond the returned identifier collection.
    pub fn qubits(&self) -> BTreeSet<QubitId> {
        let mut qubits = BTreeSet::new();

        qubits.extend(self.inputs.iter().copied());
        qubits.extend(self.outputs.iter().copied());

        for command in &self.commands {
            qubits.extend(command.qubits());

            if let Some(domain) = command.signal_domain() {
                qubits.extend(domain.x().iter().copied());
                qubits.extend(domain.z().iter().copied());
            }
        }

        qubits
    }

    /// Returns the number of logical qubits referenced by the pattern.
    pub fn qubit_count(&self) -> usize {
        self.qubits().len()
    }

    /// Returns the number of commands.
    #[must_use]
    pub fn command_count(&self) -> usize {
        self.commands.len()
    }

    /// Returns the number of measurement commands.
    pub fn measurement_count(&self) -> usize {
        self.commands
            .iter()
            .filter(|command| command.produces_measurement())
            .count()
    }

    /// Returns whether the pattern contains adaptive behavior.
    pub fn is_adaptive(&self) -> bool {
        self.commands
            .iter()
            .any(MeasurementBasedCommand::consumes_signal)
    }

    /// Returns whether the pattern contains at least one entanglement edge.
    pub fn is_entangled(&self) -> bool {
        self.commands
            .iter()
            .any(|command| {
                matches!(
                    command,
                    MeasurementBasedCommand::Entangle { .. }
                )
            })
    }

    /// Returns whether a logical qubit is declared as an input.
    #[must_use]
    pub fn is_input(&self, qubit: QubitId) -> bool {
        self.inputs.contains(&qubit)
    }

    /// Returns whether a logical qubit is declared as an output.
    #[must_use]
    pub fn is_output(&self, qubit: QubitId) -> bool {
        self.outputs.contains(&qubit)
    }

    /// Returns whether a logical qubit is an output that remains unmeasured.
    pub fn is_live_output(&self, qubit: QubitId) -> bool {
        self.is_output(qubit)
            && !self.commands.iter().any(|command| {
                matches!(
                    command,
                    MeasurementBasedCommand::Measure {
                        qubit: measured,
                        ..
                    } if *measured == qubit
                )
            })
    }

    /// Performs complete semantic validation.
    ///
    /// Validation is iterative and does not recursively traverse the pattern.
    ///
    /// The validation checks:
    ///
    /// - unique inputs;
    /// - unique outputs;
    /// - namespace consistency;
    /// - input/preparation conflicts;
    /// - preparation uniqueness;
    /// - measurement uniqueness;
    /// - output lifetime;
    /// - entanglement self-edges;
    /// - duplicate entanglement edges;
    /// - operation-after-measurement errors;
    /// - correction lifetime;
    /// - adaptive measurement causality;
    /// - adaptive correction causality;
    /// - parameter validity.
    pub fn validate(&self) -> MeasurementBasedResult<()> {
        if self.commands.is_empty()
            && self.inputs.is_empty()
            && self.outputs.is_empty()
        {
            return Err(MeasurementBasedError::EmptyPattern);
        }

        validate_unique_qubits(&self.inputs)?;
        validate_unique_qubits(&self.outputs)?;

        let namespace = self.qubits();

        for input in &self.inputs {
            if !namespace.contains(input) {
                return Err(
                    MeasurementBasedError::UnknownInput { qubit: *input },
                );
            }
        }

        for output in &self.outputs {
            if !namespace.contains(output) {
                return Err(
                    MeasurementBasedError::UnknownOutput {
                        qubit: *output,
                    },
                );
            }
        }

        let mut prepared = BTreeSet::<QubitId>::new();
        let mut measured = BTreeSet::<QubitId>::new();
        let mut entanglements =
            BTreeSet::<CanonicalEdge>::new();

        for command in &self.commands {
            validate_command_locally(command)?;

            match command {
                MeasurementBasedCommand::Prepare { qubit, .. } => {
                    ensure_known(&namespace, *qubit)?;

                    if self.is_input(*qubit) {
                        return Err(
                            MeasurementBasedError::InputPreparationConflict {
                                qubit: *qubit,
                            },
                        );
                    }

                    if !prepared.insert(*qubit) {
                        return Err(
                            MeasurementBasedError::MultiplePreparations {
                                qubit: *qubit,
                            },
                        );
                    }

                    if measured.contains(qubit) {
                        return Err(
                            MeasurementBasedError::OperationAfterMeasurement {
                                qubit: *qubit,
                            },
                        );
                    }
                }

                MeasurementBasedCommand::Entangle { first, second } => {
                    ensure_known(&namespace, *first)?;
                    ensure_known(&namespace, *second)?;

                    if first == second {
                        return Err(
                            MeasurementBasedError::SelfEntanglement {
                                qubit: *first,
                            },
                        );
                    }

                    if measured.contains(first) {
                        return Err(
                            MeasurementBasedError::OperationAfterMeasurement {
                                qubit: *first,
                            },
                        );
                    }

                    if measured.contains(second) {
                        return Err(
                            MeasurementBasedError::OperationAfterMeasurement {
                                qubit: *second,
                            },
                        );
                    }

                    let edge = CanonicalEdge::new(*first, *second);

                    if !entanglements.insert(edge) {
                        return Err(
                            MeasurementBasedError::DuplicateEntanglement {
                                first: edge.first,
                                second: edge.second,
                            },
                        );
                    }
                }

                MeasurementBasedCommand::Measure {
                    qubit,
                    specification,
                } => {
                    ensure_known(&namespace, *qubit)?;

                    specification.validate().map_err(|error| {
                        match error {
                            MeasurementBasedError::InvalidParameter {
                                message,
                                ..
                            } => {
                                MeasurementBasedError::InvalidParameter {
                                    qubit: Some(*qubit),
                                    message,
                                }
                            }

                            other => other,
                        }
                    })?;

                    if !measured.insert(*qubit) {
                        return Err(
                            MeasurementBasedError::MultipleMeasurements {
                                qubit: *qubit,
                            },
                        );
                    }

                    if self.is_output(*qubit) {
                        return Err(
                            MeasurementBasedError::MeasuredOutput {
                                qubit: *qubit,
                            },
                        );
                    }

                    validate_signal_causality(
                        specification.domain(),
                        &measured,
                        *qubit,
                    )?;
                }

                MeasurementBasedCommand::CorrectX {
                    qubit,
                    domain,
                }
                | MeasurementBasedCommand::CorrectZ {
                    qubit,
                    domain,
                } => {
                    ensure_known(&namespace, *qubit)?;

                    if measured.contains(qubit) {
                        return Err(
                            MeasurementBasedError::CorrectionAfterMeasurement {
                                qubit: *qubit,
                            },
                        );
                    }

                    validate_signal_causality(
                        domain,
                        &measured,
                        *qubit,
                    )?;
                }
            }
        }

        Ok(())
    }

    /// Returns whether the pattern is semantically valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    /// Returns the qubits whose measurement outcomes are consumed by adaptive
    /// commands.
    ///
    /// The result is deterministic and deduplicated.
    pub fn adaptive_dependencies(&self) -> BTreeSet<QubitId> {
        let mut dependencies = BTreeSet::new();

        for command in &self.commands {
            if let Some(domain) = command.signal_domain() {
                dependencies.extend(domain.x().iter().copied());
                dependencies.extend(domain.z().iter().copied());
            }
        }

        dependencies
    }

    /// Returns the measurement commands in semantic order.
    pub fn measurements(
        &self,
    ) -> impl Iterator<Item = (&QubitId, &MeasurementSpecification)> {
        self.commands.iter().filter_map(|command| {
            match command {
                MeasurementBasedCommand::Measure {
                    qubit,
                    specification,
                } => Some((qubit, specification)),

                _ => None,
            }
        })
    }

    /// Returns all commands of a selected kind.
    pub fn commands_of_kind(
        &self,
        kind: MeasurementBasedCommandKind,
    ) -> impl Iterator<Item = &MeasurementBasedCommand> {
        self.commands
            .iter()
            .filter(move |command| command.kind() == kind)
    }
}

// =============================================================================
// Canonical edge
// =============================================================================

/// Canonical undirected entanglement edge.
///
/// MBQC entanglement edges are mathematically undirected in the pattern
/// representation. Canonicalizing the endpoint ordering ensures deterministic
/// duplicate detection and hashing boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct CanonicalEdge {
    first: QubitId,
    second: QubitId,
}

impl CanonicalEdge {
    #[must_use]
    fn new(first: QubitId, second: QubitId) -> Self {
        if first <= second {
            Self { first, second }
        } else {
            Self {
                first: second,
                second: first,
            }
        }
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Validates command-local invariants without requiring the complete pattern
/// namespace.
fn validate_command_locally(
    command: &MeasurementBasedCommand,
) -> MeasurementBasedResult<()> {
    match command {
        MeasurementBasedCommand::Prepare { .. } => Ok(()),

        MeasurementBasedCommand::Entangle { first, second } => {
            if first == second {
                return Err(
                    MeasurementBasedError::SelfEntanglement {
                        qubit: *first,
                    },
                );
            }

            Ok(())
        }

        MeasurementBasedCommand::Measure {
            qubit,
            specification,
        } => specification.validate().map_err(|error| {
            match error {
                MeasurementBasedError::InvalidParameter {
                    message,
                    ..
                } => MeasurementBasedError::InvalidParameter {
                    qubit: Some(*qubit),
                    message,
                },

                other => other,
            }
        }),

        MeasurementBasedCommand::CorrectX {
            qubit: _,
            domain,
        }
        | MeasurementBasedCommand::CorrectZ {
            qubit: _,
            domain,
        } => {
            validate_signal_domain_structure(domain)
        }
    }
}

/// Validates deterministic uniqueness of a logical-qubit list.
fn validate_unique_qubits(
    qubits: &[QubitId],
) -> MeasurementBasedResult<()> {
    let mut seen = BTreeSet::<QubitId>::new();

    for qubit in qubits {
        if !seen.insert(*qubit) {
            return Err(
                MeasurementBasedError::DuplicateQubitOperand {
                    qubit: *qubit,
                },
            );
        }
    }

    Ok(())
}

/// Validates signal-domain structural invariants.
///
/// `BTreeSet` already removes exact duplicates, so the primary purpose here
/// is to preserve an explicit validation boundary for future IR revisions.
fn validate_signal_domain_structure(
    domain: &SignalDomain,
) -> MeasurementBasedResult<()> {
    for qubit in domain.x() {
        if domain.x().contains(qubit) {
            continue;
        }

        return Err(
            MeasurementBasedError::InvalidSignalDomain,
        );
    }

    for qubit in domain.z() {
        if domain.z().contains(qubit) {
            continue;
        }

        return Err(
            MeasurementBasedError::InvalidSignalDomain,
        );
    }

    Ok(())
}

/// Ensures that a qubit belongs to the pattern namespace.
fn ensure_known(
    namespace: &BTreeSet<QubitId>,
    qubit: QubitId,
) -> MeasurementBasedResult<()> {
    if namespace.contains(&qubit) {
        Ok(())
    } else {
        Err(MeasurementBasedError::UnknownQubit { qubit })
    }
}

/// Validates that all signal dependencies have become available before the
/// consuming command.
///
/// `measured` contains outcomes produced by commands that occurred earlier in
/// the pattern. The current consumer is intentionally not inserted until after
/// this check.
///
/// This makes self-dependencies and forward dependencies explicit errors.
fn validate_signal_causality(
    domain: &SignalDomain,
    measured: &BTreeSet<QubitId>,
    consumer: QubitId,
) -> MeasurementBasedResult<()> {
    for dependency in domain.x() {
        if !measured.contains(dependency) {
            return Err(
                MeasurementBasedError::FutureMeasurementDependency {
                    dependency: *dependency,
                    consumer,
                },
            );
        }
    }

    for dependency in domain.z() {
        if !measured.contains(dependency) {
            return Err(
                MeasurementBasedError::FutureMeasurementDependency {
                    dependency: *dependency,
                    consumer,
                },
            );
        }
    }

    Ok(())
}

// =============================================================================
// Convenience constructors
// =============================================================================

/// Creates a non-adaptive XY-plane measurement specification.
pub fn xy_measurement(
    angle: Parameter,
) -> MeasurementBasedResult<MeasurementSpecification> {
    MeasurementSpecification::new(
        MeasurementPlane::XY,
        angle,
    )
}

/// Creates a non-adaptive XZ-plane measurement specification.
pub fn xz_measurement(
    angle: Parameter,
) -> MeasurementBasedResult<MeasurementSpecification> {
    MeasurementSpecification::new(
        MeasurementPlane::XZ,
        angle,
    )
}

/// Creates a non-adaptive YZ-plane measurement specification.
pub fn yz_measurement(
    angle: Parameter,
) -> MeasurementBasedResult<MeasurementSpecification> {
    MeasurementSpecification::new(
        MeasurementPlane::YZ,
        angle,
    )
}

/// Creates an adaptive XY-plane measurement specification.
pub fn adaptive_xy_measurement(
    angle: Parameter,
    domain: SignalDomain,
) -> MeasurementBasedResult<MeasurementSpecification> {
    MeasurementSpecification::with_domain(
        MeasurementPlane::XY,
        angle,
        domain,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::qubit::QubitId;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn zero_angle() -> Parameter {
        Parameter::constant(0.0)
            .expect("zero is a valid finite parameter")
    }

    #[test]
    fn empty_pattern_is_rejected() {
        let pattern = MeasurementBasedPattern::new();

        assert_eq!(
            pattern.validate(),
            Err(MeasurementBasedError::EmptyPattern)
        );
    }

    #[test]
    fn pattern_can_represent_simple_cluster_computation() {
        let mut pattern =
            MeasurementBasedPattern::with_io(
                vec![q(0)],
                vec![q(2)],
            )
            .expect("valid IO");

        pattern
            .prepare(q(1), PreparationState::Plus)
            .expect("valid preparation");

        pattern
            .entangle(q(0), q(1))
            .expect("valid edge");

        pattern
            .entangle(q(1), q(2))
            .expect("valid edge");

        pattern
            .measure(
                q(0),
                xy_measurement(zero_angle())
                    .expect("valid measurement"),
            )
            .expect("valid measurement");

        pattern
            .measure(
                q(1),
                xy_measurement(zero_angle())
                    .expect("valid measurement"),
            )
            .expect("valid measurement");

        pattern
            .validate()
            .expect("pattern should be valid");
    }

    #[test]
    fn adaptive_measurement_requires_previous_measurement() {
        let mut domain = SignalDomain::new();
        domain.add_x(q(1));

        let specification =
            adaptive_xy_measurement(zero_angle(), domain)
                .expect("valid adaptive specification");

        let mut pattern =
            MeasurementBasedPattern::with_io(
                vec![q(0)],
                vec![q(2)],
            )
            .expect("valid IO");

        pattern
            .measure(q(0), specification)
            .expect("local construction is valid");

        assert!(matches!(
            pattern.validate(),
            Err(
                MeasurementBasedError::FutureMeasurementDependency {
                    dependency,
                    consumer
                }
            ) if dependency == q(1) && consumer == q(0)
        ));
    }

    #[test]
    fn adaptive_measurement_can_consume_previous_result() {
        let mut pattern =
            MeasurementBasedPattern::with_io(
                vec![q(0)],
                vec![q(2)],
            )
            .expect("valid IO");

        pattern
            .measure(
                q(0),
                xy_measurement(zero_angle())
                    .expect("valid measurement"),
            )
            .expect("valid measurement");

        let mut domain = SignalDomain::new();
        domain.add_x(q(0));

        pattern
            .measure(
                q(1),
                adaptive_xy_measurement(
                    zero_angle(),
                    domain,
                )
                .expect("valid adaptive measurement"),
            )
            .expect("valid measurement");

        pattern
            .validate()
            .expect("dependency is available");
    }

    #[test]
    fn output_cannot_be_measured() {
        let mut pattern =
            MeasurementBasedPattern::with_io(
                vec![q(0)],
                vec![q(0)],
            )
            .expect("valid IO");

        pattern
            .measure(
                q(0),
                xy_measurement(zero_angle())
                    .expect("valid measurement"),
            )
            .expect("local measurement construction");

        assert_eq!(
            pattern.validate(),
            Err(MeasurementBasedError::MeasuredOutput {
                qubit: q(0)
            })
        );
    }

    #[test]
    fn self_entanglement_is_rejected() {
        let mut pattern =
            MeasurementBasedPattern::with_io(
                vec![q(0)],
                vec![q(1)],
            )
            .expect("valid IO");

        let result = pattern.entangle(q(0), q(0));

        assert_eq!(
            result,
            Err(
                MeasurementBasedError::SelfEntanglement {
                    qubit: q(0)
                }
            )
        );
    }

    #[test]
    fn duplicate_entanglement_is_rejected_independent_of_endpoint_order() {
        let mut pattern =
            MeasurementBasedPattern::with_io(
                vec![q(0)],
                vec![q(2)],
            )
            .expect("valid IO");

        pattern
            .entangle(q(0), q(1))
            .expect("first edge");

        pattern
            .entangle(q(1), q(0))
            .expect("local edge construction");

        assert!(matches!(
            pattern.validate(),
            Err(
                MeasurementBasedError::DuplicateEntanglement {
                    first,
                    second
                }
            ) if first == q(0) && second == q(1)
        ));
    }

    #[test]
    fn duplicate_measurement_is_rejected() {
        let mut pattern =
            MeasurementBasedPattern::with_io(
                vec![q(0)],
                vec![q(2)],
            )
            .expect("valid IO");

        let measurement =
            xy_measurement(zero_angle())
                .expect("valid measurement");

        pattern
            .measure(q(0), measurement.clone())
            .expect("first measurement");

        pattern
            .measure(q(0), measurement)
            .expect("local construction");

        assert_eq!(
            pattern.validate(),
            Err(
                MeasurementBasedError::MultipleMeasurements {
                    qubit: q(0)
                }
            )
        );
    }

    #[test]
    fn signal_domain_is_deterministic() {
        let mut domain = SignalDomain::new();

        domain.add_x(q(7));
        domain.add_x(q(1));
        domain.add_x(q(7));
        domain.add_z(q(3));
        domain.add_z(q(1));

        assert_eq!(
            domain.dependencies(),
            vec![q(1), q(3), q(7)]
        );
    }

    #[test]
    fn symbolic_measurement_angle_is_supported() {
        let angle =
            Parameter::symbol("theta")
                .expect("valid symbol");

        let measurement =
            xy_measurement(angle)
                .expect("symbolic angle is valid");

        assert!(measurement.angle().is_symbolic());
    }

    #[test]
    fn command_kind_is_stable() {
        let command =
            MeasurementBasedCommand::Prepare {
                qubit: q(0),
                state: PreparationState::Plus,
            };

        assert_eq!(
            command.kind(),
            MeasurementBasedCommandKind::Prepare
        );
        assert_eq!(
            command.kind().as_str(),
            "prepare"
        );
    }

    #[test]
    fn qubit_namespace_is_deduplicated() {
        let mut pattern =
            MeasurementBasedPattern::with_io(
                vec![q(0)],
                vec![q(2)],
            )
            .expect("valid IO");

        pattern
            .prepare(q(1), PreparationState::Plus)
            .expect("valid preparation");

        pattern
            .entangle(q(0), q(1))
            .expect("valid edge");

        assert_eq!(pattern.qubit_count(), 3);
    }

    #[test]
    fn correction_requires_previous_measurement() {
        let mut pattern =
            MeasurementBasedPattern::with_io(
                vec![q(0)],
                vec![q(1)],
            )
            .expect("valid IO");

        let mut domain = SignalDomain::new();
        domain.add_x(q(0));

        pattern
            .correct_x(q(1), domain)
            .expect("local correction construction");

        pattern
            .validate()
            .expect_err("q0 has not been measured yet");
    }

    #[test]
    fn correction_after_measurement_is_rejected() {
        let mut pattern =
            MeasurementBasedPattern::with_io(
                vec![q(0)],
                vec![q(1)],
            )
            .expect("valid IO");

        pattern
            .measure(
                q(0),
                xy_measurement(zero_angle())
                    .expect("valid measurement"),
            )
            .expect("measurement");

        let mut domain = SignalDomain::new();
        domain.add_x(q(0));

        pattern
            .correct_x(q(0), domain)
            .expect("local construction");

        assert_eq!(
            pattern.validate(),
            Err(
                MeasurementBasedError::CorrectionAfterMeasurement {
                    qubit: q(0)
                }
            )
        );
    }

    #[test]
    fn input_cannot_be_explicitly_prepared() {
        let mut pattern =
            MeasurementBasedPattern::with_io(
                vec![q(0)],
                vec![q(1)],
            )
            .expect("valid IO");

        pattern
            .prepare(q(0), PreparationState::Plus)
            .expect("local construction");

        assert_eq!(
            pattern.validate(),
            Err(
                MeasurementBasedError::InputPreparationConflict {
                    qubit: q(0)
                }
            )
        );
    }
}