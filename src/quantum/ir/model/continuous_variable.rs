//! Zamani Quantum IR — Continuous-Variable Quantum Computing Model
//!
//! Production-grade, hardware-independent semantic representation of
//! continuous-variable (CV) quantum computation.
//!
//! # Architectural role
//!
//! This module represents the *meaning* of a continuous-variable quantum
//! program. It deliberately does not represent a particular simulator,
//! processor, photonic chip, oscillator implementation, DAC, detector,
//! vendor API, routing algorithm, scheduler, or numerical state representation.
//!
//! The canonical dependency boundary is:
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! canonical Zamani Quantum IR
//!      │
//!      ▼
//! this module: CV semantic model
//!      │
//!      ├── Gaussian operations
//!      ├── non-Gaussian operations
//!      ├── bosonic modes
//!      ├── state preparation
//!      ├── CV measurement
//!      ├── symbolic Hamiltonians
//!      ├── multimode transformations
//!      └── extensions
//!      │
//!      ▼
//! target-independent transformations
//!      │
//!      ▼
//! target capabilities / mapping / scheduling
//!      │
//!      ▼
//! simulator / hardware / backend
//! ```
//!
//! # Universal-program principle
//!
//! A CV program is written once at the semantic level and can be lowered to
//! any compatible CV target for which the required resources and capabilities
//! are available.
//!
//! This module therefore contains NO:
//!
//! - maximum mode count;
//! - maximum Fock dimension;
//! - maximum circuit depth;
//! - maximum number of operations;
//! - vendor names;
//! - hardware topology;
//! - detector implementation;
//! - fixed optical frequency;
//! - fixed sampling rate;
//! - fixed truncation dimension;
//! - simulator state vector;
//! - dense state representation;
//! - backend execution code.
//!
//! Concrete resource limits are supplied by an explicit compiler/runtime
//! policy and target capability model.
//!
//! # Continuous-variable semantics
//!
//! CV computation is broader than qubit gate computation. A CV program may
//! contain:
//!
//! - bosonic modes;
//! - quadrature observables;
//! - Gaussian state preparation;
//! - non-Gaussian state preparation;
//! - Gaussian transformations;
//! - non-Gaussian transformations;
//! - multimode transformations;
//! - Hamiltonian evolution;
//! - homodyne measurement;
//! - heterodyne measurement;
//! - photon-number measurement;
//! - parity measurement;
//! - generalized measurements;
//! - feed-forward;
//! - symbolic parameters;
//! - arbitrary future CV extensions.
//!
//! # Qubit integration
//!
//! A CV mode is NOT a qubit.
//!
//! `ModeId` is therefore the canonical identity for a bosonic mode.
//!
//! `quantum::ir::qubit::QubitId` is used only for explicitly encoded CV
//! subsystems where the semantic program genuinely refers to a logical qubit.
//!
//! New code must use:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! rather than creating another qubit identity type.
//!
//! # Numerical safety
//!
//! Floating-point constants are accepted only when finite.
//!
//! NaN and infinite values are rejected by constructors.
//!
//! This module never uses floating-point values as implicit sentinels.
//!
//! # Symbolic computation
//!
//! Numerical resolution is intentionally delayed. CV parameters use the
//! canonical Zamani `Parameter` type so expressions such as:
//!
//! ```text
//! alpha
//! theta / 2
//! sqrt(gamma)
//! kappa * t
//! ```
//!
//! can remain symbolic until a later compilation stage.
//!
//! # Scalability
//!
//! CV systems can have very large numbers of modes and arbitrarily large
//! occupation-number spaces. The IR therefore represents:
//!
//! - mode identities rather than allocated state vectors;
//! - symbolic Fock states rather than dense amplitudes;
//! - sparse multimode transformations;
//! - symbolic Hamiltonian terms;
//! - structured measurements;
//! - structured state preparations.
//!
//! Numerical truncation belongs to a simulator or target-lowering policy,
//! never to this semantic model.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code;
//! - no external dependencies.
//!
//! # Integration contract
//!
//! This file depends only on:
//!
//! - the Rust standard library;
//! - `quantum::ir::parameter::Parameter`;
//! - `quantum::ir::qubit::QubitId`.
//!
//! It does NOT depend on:
//!
//! - hardware;
//! - frontend;
//! - optimizer;
//! - routing;
//! - scheduling;
//! - simulator;
//! - QEC;
//! - backend execution.
//!
//! Downstream modules may consume these types without requiring this module
//! to be modified.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::fmt;

use crate::quantum::ir::parameter::Parameter;
use crate::quantum::ir::qubit::QubitId;

// =============================================================================
// Constants
// =============================================================================

/// Canonical semantic name for this model.
pub const MODEL_NAME: &str = "continuous_variable";

/// Canonical model namespace.
pub const MODEL_NAMESPACE: &str = "zamani.quantum.cv";

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by CV IR construction and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContinuousVariableError {
    /// An identifier or name was empty.
    EmptyName,

    /// A name contains an invalid character.
    InvalidNameCharacter {
        /// The offending byte/character position.
        position: usize,
    },

    /// A numerical value was not finite.
    NonFiniteValue {
        /// Semantic field containing the invalid value.
        field: &'static str,
    },

    /// A numerical value was outside the allowed semantic domain.
    InvalidValue {
        /// Semantic field.
        field: &'static str,

        /// Human-readable explanation.
        reason: &'static str,
    },

    /// A range has invalid bounds.
    InvalidRange {
        /// Lower bound.
        start: u64,

        /// Upper bound.
        end: u64,
    },

    /// A sparse matrix contains an invalid coordinate.
    InvalidMatrixCoordinate {
        /// Row.
        row: usize,

        /// Column.
        column: usize,
    },

    /// A sparse transformation contains conflicting entries.
    ConflictingTransformationEntry,

    /// An operation references no modes when modes are required.
    MissingModes,

    /// An operation contains an invalid mode list.
    DuplicateMode,

    /// A measurement has no observable specification.
    MissingMeasurementObservable,

    /// A Hamiltonian has no terms.
    EmptyHamiltonian,

    /// A Hamiltonian term has no operator factors.
    EmptyHamiltonianTerm,

    /// An operator factor uses an invalid exponent.
    InvalidOperatorExponent,

    /// A symbolic name is too long for the local policy.
    NameTooLong,

    /// A generalized extension has an empty namespace.
    EmptyExtensionNamespace,
}

impl fmt::Display for ContinuousVariableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => f.write_str("CV name cannot be empty"),

            Self::InvalidNameCharacter { position } => {
                write!(f, "invalid character in CV name at byte position {position}")
            }

            Self::NonFiniteValue { field } => {
                write!(f, "CV field '{field}' must be finite")
            }

            Self::InvalidValue { field, reason } => {
                write!(f, "invalid CV value for '{field}': {reason}")
            }

            Self::InvalidRange { start, end } => {
                write!(f, "invalid CV range [{start}, {end})")
            }

            Self::InvalidMatrixCoordinate { row, column } => {
                write!(
                    f,
                    "invalid CV matrix coordinate ({row}, {column})"
                )
            }

            Self::ConflictingTransformationEntry => {
                f.write_str("conflicting sparse transformation entry")
            }

            Self::MissingModes => {
                f.write_str("CV operation requires at least one mode")
            }

            Self::DuplicateMode => {
                f.write_str("CV operation contains a duplicate mode")
            }

            Self::MissingMeasurementObservable => {
                f.write_str("CV measurement requires an observable")
            }

            Self::EmptyHamiltonian => {
                f.write_str("CV Hamiltonian must contain at least one term")
            }

            Self::EmptyHamiltonianTerm => {
                f.write_str("CV Hamiltonian term must contain at least one factor")
            }

            Self::InvalidOperatorExponent => {
                f.write_str("CV operator exponent must be greater than zero")
            }

            Self::NameTooLong => {
                f.write_str("CV name exceeds the local semantic name policy")
            }

            Self::EmptyExtensionNamespace => {
                f.write_str("CV extension namespace cannot be empty")
            }
        }
    }
}

impl std::error::Error for ContinuousVariableError {}

/// Result type used by this module.
pub type CvResult<T> = Result<T, ContinuousVariableError>;

// =============================================================================
// Mode identity
// =============================================================================

/// Stable continuous-variable mode identity.
///
/// A `ModeId` identifies a semantic bosonic mode. It does not imply:
///
/// - a physical optical path;
/// - a particular frequency;
/// - a particular cavity;
/// - a particular detector;
/// - a particular hardware channel.
///
/// The target layer decides those mappings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModeId(u64);

impl ModeId {
    /// Creates a mode identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric identity.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns the next representable identifier.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<u64> for ModeId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<ModeId> for u64 {
    fn from(value: ModeId) -> Self {
        value.value()
    }
}

impl fmt::Display for ModeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "m{}", self.0)
    }
}

// =============================================================================
// Subsystem reference
// =============================================================================

/// A CV subsystem reference.
///
/// Most CV operations use `ModeId`.
///
/// `EncodedQubit` exists only when a CV resource is explicitly associated with
/// a canonical Zamani logical qubit at a higher semantic level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CvSubsystem {
    /// Native continuous-variable bosonic mode.
    Mode(ModeId),

    /// Logical qubit explicitly associated with an encoded CV subsystem.
    EncodedQubit(QubitId),
}

impl CvSubsystem {
    /// Returns the mode when this is a native CV mode.
    #[must_use]
    pub const fn mode(self) -> Option<ModeId> {
        match self {
            Self::Mode(mode) => Some(mode),
            Self::EncodedQubit(_) => None,
        }
    }

    /// Returns the qubit when this is an explicitly encoded qubit.
    #[must_use]
    pub const fn qubit(self) -> Option<QubitId> {
        match self {
            Self::Mode(_) => None,
            Self::EncodedQubit(qubit) => Some(qubit),
        }
    }
}

impl From<ModeId> for CvSubsystem {
    fn from(value: ModeId) -> Self {
        Self::Mode(value)
    }
}

impl From<QubitId> for CvSubsystem {
    fn from(value: QubitId) -> Self {
        Self::EncodedQubit(value)
    }
}

// =============================================================================
// Mode collection
// =============================================================================

/// An ordered, duplicate-free collection of CV modes/subsystems.
///
/// This type is intentionally represented by a vector rather than a fixed
/// register size. It supports arbitrary finite program sizes subject only to
/// the process/resource policy imposed by the caller.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CvModeSet {
    modes: Vec<CvSubsystem>,
}

impl CvModeSet {
    /// Creates an empty mode set.
    #[must_use]
    pub const fn new() -> Self {
        Self { modes: Vec::new() }
    }

    /// Creates a mode set from an iterator.
    pub fn try_from_iter<I>(iter: I) -> CvResult<Self>
    where
        I: IntoIterator<Item = CvSubsystem>,
    {
        let mut result = Self::new();

        for mode in iter {
            result.push(mode)?;
        }

        Ok(result)
    }

    /// Returns the number of referenced modes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.modes.len()
    }

    /// Returns whether no modes are referenced.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.modes.is_empty()
    }

    /// Returns the modes in deterministic order.
    #[must_use]
    pub fn as_slice(&self) -> &[CvSubsystem] {
        &self.modes
    }

    /// Adds a mode if it is not already present.
    pub fn push(&mut self, mode: CvSubsystem) -> CvResult<()> {
        if self.modes.contains(&mode) {
            return Err(ContinuousVariableError::DuplicateMode);
        }

        self.modes.push(mode);
        Ok(())
    }

    /// Returns whether a mode is referenced.
    #[must_use]
    pub fn contains(&self, mode: CvSubsystem) -> bool {
        self.modes.contains(&mode)
    }

    /// Returns an iterator over modes.
    pub fn iter(&self) -> std::slice::Iter<'_, CvSubsystem> {
        self.modes.iter()
    }
}

impl Default for CvModeSet {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for CvModeSet {
    type Item = CvSubsystem;
    type IntoIter = std::vec::IntoIter<CvSubsystem>;

    fn into_iter(self) -> Self::IntoIter {
        self.modes.into_iter()
    }
}

// =============================================================================
// Quadratures
// =============================================================================

/// Canonical CV quadrature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Quadrature {
    /// Position-like quadrature.
    X,

    /// Momentum-like quadrature.
    P,

    /// Arbitrary phase-space quadrature.
    Rotated,
}

/// Quadrature observable specification.
#[derive(Debug, Clone, PartialEq)]
pub struct QuadratureObservable {
    quadrature: Quadrature,
    phase: Parameter,
}

impl QuadratureObservable {
    /// Creates an X observable.
    pub fn x() -> CvResult<Self> {
        Ok(Self {
            quadrature: Quadrature::X,
            phase: Parameter::constant(0.0)
                .map_err(|_| ContinuousVariableError::NonFiniteValue {
                    field: "phase",
                })?,
        })
    }

    /// Creates a P observable.
    pub fn p() -> CvResult<Self> {
        Ok(Self {
            quadrature: Quadrature::P,
            phase: Parameter::constant(0.0)
                .map_err(|_| ContinuousVariableError::NonFiniteValue {
                    field: "phase",
                })?,
        })
    }

    /// Creates a rotated quadrature.
    pub fn rotated(phase: Parameter) -> Self {
        Self {
            quadrature: Quadrature::Rotated,
            phase,
        }
    }

    /// Returns the quadrature family.
    #[must_use]
    pub const fn quadrature(&self) -> Quadrature {
        self.quadrature
    }

    /// Returns the symbolic/numerical phase.
    #[must_use]
    pub fn phase(&self) -> &Parameter {
        &self.phase
    }
}

// =============================================================================
// Fock occupation
// =============================================================================

/// Symbolic or concrete Fock occupation number.
///
/// `Exact` is a concrete occupation.
///
/// `Symbolic` allows a compiler to preserve a parameterized occupation without
/// introducing a fixed truncation dimension.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FockOccupation {
    /// Concrete occupation number.
    Exact(u64),

    /// Symbolic occupation identifier.
    Symbol(String),
}

impl FockOccupation {
    /// Creates a concrete occupation number.
    #[must_use]
    pub const fn exact(value: u64) -> Self {
        Self::Exact(value)
    }

    /// Creates a symbolic occupation.
    pub fn symbol<S: Into<String>>(name: S) -> CvResult<Self> {
        let name = name.into();
        validate_name(&name)?;

        Ok(Self::Symbol(name))
    }
}

// =============================================================================
// State preparation
// =============================================================================

/// Continuous-variable state-preparation semantics.
#[derive(Debug, Clone, PartialEq)]
pub enum CvStatePreparation {
    /// Vacuum state.
    Vacuum,

    /// Coherent state with complex amplitude components.
    Coherent {
        /// Real part of alpha.
        alpha_real: Parameter,

        /// Imaginary part of alpha.
        alpha_imaginary: Parameter,
    },

    /// Single-mode squeezed state.
    Squeezed {
        /// Squeezing magnitude.
        magnitude: Parameter,

        /// Squeezing phase.
        phase: Parameter,
    },

    /// Displaced squeezed state.
    DisplacedSqueezed {
        /// Real displacement.
        displacement_real: Parameter,

        /// Imaginary displacement.
        displacement_imaginary: Parameter,

        /// Squeezing magnitude.
        squeezing: Parameter,

        /// Squeezing phase.
        phase: Parameter,
    },

    /// Thermal state.
    Thermal {
        /// Mean occupation.
        mean_occupation: Parameter,
    },

    /// Exact Fock state.
    Fock {
        /// Occupation.
        occupation: FockOccupation,
    },

    /// Cat-like superposition.
    Cat {
        /// Positive/negative coherent amplitude.
        amplitude: Parameter,

        /// Relative phase.
        relative_phase: Parameter,

        /// Normalization convention identifier.
        normalization: CatNormalization,
    },

    /// Generalized state specified by a named semantic family.
    ///
    /// This preserves future CV state families without forcing them into the
    /// canonical enum.
    Extension {
        /// Extension namespace.
        namespace: String,

        /// Extension operation/state name.
        name: String,

        /// String-valued semantic attributes.
        attributes: BTreeMap<String, String>,
    },
}

/// Cat-state normalization convention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CatNormalization {
    /// Canonical normalized state.
    Normalized,

    /// Backend/extension-defined normalization.
    Deferred,
}

// =============================================================================
// Gaussian operations
// =============================================================================

/// Canonical Gaussian operation family.
///
/// Gaussian operations preserve Gaussianity when applied to Gaussian states.
#[derive(Debug, Clone, PartialEq)]
pub enum GaussianOperation {
    /// Phase-space displacement.
    Displacement {
        /// Real displacement component.
        real: Parameter,

        /// Imaginary displacement component.
        imaginary: Parameter,
    },

    /// Phase-space rotation.
    Rotation {
        /// Rotation angle.
        angle: Parameter,
    },

    /// Single-mode squeezing.
    Squeezing {
        /// Squeezing magnitude.
        magnitude: Parameter,

        /// Squeezing phase.
        phase: Parameter,
    },

    /// Two-mode beam splitter.
    BeamSplitter {
        /// Mixing angle.
        angle: Parameter,

        /// Relative phase.
        phase: Parameter,
    },

    /// Two-mode controlled phase.
    ControlledPhase {
        /// Interaction strength.
        strength: Parameter,
    },

    /// Two-mode squeezing.
    TwoModeSqueezing {
        /// Squeezing magnitude.
        magnitude: Parameter,

        /// Squeezing phase.
        phase: Parameter,
    },

    /// General Gaussian transformation represented as a sparse symplectic
    /// matrix plus displacement.
    Symplectic {
        /// Sparse transformation.
        transform: SparseSymplecticTransform,

        /// Displacement vector.
        displacement: SparsePhaseSpaceVector,
    },
}

// =============================================================================
// Non-Gaussian operations
// =============================================================================

/// Non-Gaussian CV operation.
#[derive(Debug, Clone, PartialEq)]
pub enum NonGaussianOperation {
    /// Cubic phase gate.
    CubicPhase {
        /// Cubic interaction strength.
        strength: Parameter,
    },

    /// Kerr interaction.
    Kerr {
        /// Nonlinearity coefficient.
        strength: Parameter,
    },

    /// Photon-number-dependent phase.
    NumberPhase {
        /// Interaction coefficient.
        strength: Parameter,
    },

    /// Arbitrary polynomial phase operation.
    PolynomialPhase {
        /// Polynomial coefficients ordered from degree 1 upward.
        coefficients: Vec<Parameter>,
    },

    /// Hamiltonian evolution.
    HamiltonianEvolution {
        /// Hamiltonian.
        hamiltonian: CvHamiltonian,

        /// Evolution duration.
        duration: Parameter,
    },

    /// Future/extension-defined non-Gaussian operation.
    Extension {
        /// Extension namespace.
        namespace: String,

        /// Operation name.
        name: String,

        /// String-valued semantic attributes.
        attributes: BTreeMap<String, String>,
    },
}

// =============================================================================
// CV operation
// =============================================================================

/// Complete CV semantic operation.
#[derive(Debug, Clone, PartialEq)]
pub struct CvOperation {
    /// Modes/subsystems affected by the operation.
    modes: CvModeSet,

    /// Operation semantics.
    kind: CvOperationKind,

    /// Optional symbolic operation label.
    label: Option<String>,
}

/// CV operation kind.
#[derive(Debug, Clone, PartialEq)]
pub enum CvOperationKind {
    /// Gaussian operation.
    Gaussian(GaussianOperation),

    /// Non-Gaussian operation.
    NonGaussian(NonGaussianOperation),

    /// State preparation.
    Prepare(CvStatePreparation),

    /// Measurement.
    Measure(CvMeasurement),

    /// Barrier/semantic ordering constraint.
    Barrier,

    /// Explicit extension operation.
    Extension {
        /// Extension namespace.
        namespace: String,

        /// Extension operation name.
        name: String,

        /// String-valued semantic attributes.
        attributes: BTreeMap<String, String>,
    },
}

impl CvOperation {
    /// Creates an operation.
    pub fn new<I>(
        modes: I,
        kind: CvOperationKind,
    ) -> CvResult<Self>
    where
        I: IntoIterator<Item = CvSubsystem>,
    {
        let modes = CvModeSet::try_from_iter(modes)?;

        if modes.is_empty() {
            return Err(ContinuousVariableError::MissingModes);
        }

        validate_operation_kind(&kind)?;

        Ok(Self {
            modes,
            kind,
            label: None,
        })
    }

    /// Attaches a semantic label.
    pub fn with_label<S: Into<String>>(mut self, label: S) -> CvResult<Self> {
        let label = label.into();
        validate_name(&label)?;
        self.label = Some(label);
        Ok(self)
    }

    /// Returns affected modes.
    #[must_use]
    pub fn modes(&self) -> &CvModeSet {
        &self.modes
    }

    /// Returns operation kind.
    #[must_use]
    pub fn kind(&self) -> &CvOperationKind {
        &self.kind
    }

    /// Returns the optional semantic label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

// =============================================================================
// Measurements
// =============================================================================

/// Continuous-variable measurement semantics.
#[derive(Debug, Clone, PartialEq)]
pub enum CvMeasurement {
    /// Homodyne measurement of one quadrature.
    Homodyne {
        /// Observable.
        observable: QuadratureObservable,
    },

    /// Heterodyne measurement.
    Heterodyne,

    /// Photon-number measurement.
    PhotonNumber,

    /// Parity measurement.
    Parity,

    /// Generalized observable measurement.
    Observable {
        /// Observable definition.
        observable: CvObservable,
    },

    /// Extension-defined measurement.
    Extension {
        /// Extension namespace.
        namespace: String,

        /// Measurement name.
        name: String,

        /// String-valued semantic attributes.
        attributes: BTreeMap<String, String>,
    },
}

/// Generic CV observable.
#[derive(Debug, Clone, PartialEq)]
pub enum CvObservable {
    /// Polynomial in X and P.
    Polynomial {
        /// Polynomial terms.
        terms: Vec<CvObservableTerm>,
    },

    /// Number operator.
    Number,

    /// Parity operator.
    Parity,

    /// Extension-defined observable.
    Extension {
        /// Extension namespace.
        namespace: String,

        /// Observable name.
        name: String,

        /// String-valued semantic attributes.
        attributes: BTreeMap<String, String>,
    },
}

/// One observable term.
#[derive(Debug, Clone, PartialEq)]
pub struct CvObservableTerm {
    /// Scalar coefficient.
    coefficient: Parameter,

    /// Operator factors.
    factors: Vec<CvOperatorFactor>,
}

/// Operator factor.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CvOperatorFactor {
    /// Position quadrature.
    X,

    /// Momentum quadrature.
    P,

    /// Creation operator.
    Creation,

    /// Annihilation operator.
    Annihilation,

    /// Number operator.
    Number,

    /// Identity operator.
    Identity,
}

impl CvObservableTerm {
    /// Creates an observable term.
    pub fn new(
        coefficient: Parameter,
        factors: Vec<CvOperatorFactor>,
    ) -> CvResult<Self> {
        if factors.is_empty() {
            return Err(ContinuousVariableError::EmptyHamiltonianTerm);
        }

        Ok(Self {
            coefficient,
            factors,
        })
    }

    /// Returns the coefficient.
    #[must_use]
    pub fn coefficient(&self) -> &Parameter {
        &self.coefficient
    }

    /// Returns the factors.
    #[must_use]
    pub fn factors(&self) -> &[CvOperatorFactor] {
        &self.factors
    }
}

// =============================================================================
// Hamiltonian
// =============================================================================

/// Continuous-variable Hamiltonian.
///
/// The Hamiltonian is represented symbolically as a sum of terms rather than
/// as a dense matrix. This is critical for scalability.
#[derive(Debug, Clone, PartialEq)]
pub struct CvHamiltonian {
    terms: Vec<CvHamiltonianTerm>,
}

/// One symbolic Hamiltonian term.
#[derive(Debug, Clone, PartialEq)]
pub struct CvHamiltonianTerm {
    coefficient: Parameter,
    factors: Vec<CvHamiltonianFactor>,
}

/// Hamiltonian operator factor.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CvHamiltonianFactor {
    /// Position quadrature.
    X {
        /// Target mode.
        mode: ModeId,
    },

    /// Momentum quadrature.
    P {
        /// Target mode.
        mode: ModeId,
    },

    /// Creation operator.
    Creation {
        /// Target mode.
        mode: ModeId,
    },

    /// Annihilation operator.
    Annihilation {
        /// Target mode.
        mode: ModeId,
    },

    /// Number operator.
    Number {
        /// Target mode.
        mode: ModeId,
    },
}

impl CvHamiltonian {
    /// Creates an empty Hamiltonian builder.
    #[must_use]
    pub const fn new() -> Self {
        Self { terms: Vec::new() }
    }

    /// Creates a Hamiltonian from terms.
    pub fn from_terms(terms: Vec<CvHamiltonianTerm>) -> CvResult<Self> {
        if terms.is_empty() {
            return Err(ContinuousVariableError::EmptyHamiltonian);
        }

        Ok(Self { terms })
    }

    /// Adds a term.
    pub fn push_term(&mut self, term: CvHamiltonianTerm) -> CvResult<()> {
        if term.factors.is_empty() {
            return Err(ContinuousVariableError::EmptyHamiltonianTerm);
        }

        self.terms.push(term);
        Ok(())
    }

    /// Returns all terms.
    #[must_use]
    pub fn terms(&self) -> &[CvHamiltonianTerm] {
        &self.terms
    }

    /// Returns whether the Hamiltonian has no terms.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }
}

impl Default for CvHamiltonian {
    fn default() -> Self {
        Self::new()
    }
}

impl CvHamiltonianTerm {
    /// Creates a Hamiltonian term.
    pub fn new(
        coefficient: Parameter,
        factors: Vec<CvHamiltonianFactor>,
    ) -> CvResult<Self> {
        if factors.is_empty() {
            return Err(ContinuousVariableError::EmptyHamiltonianTerm);
        }

        Ok(Self {
            coefficient,
            factors,
        })
    }

    /// Returns the coefficient.
    #[must_use]
    pub fn coefficient(&self) -> &Parameter {
        &self.coefficient
    }

    /// Returns the operator factors.
    #[must_use]
    pub fn factors(&self) -> &[CvHamiltonianFactor] {
        &self.factors
    }
}

// =============================================================================
// Sparse phase-space vector
// =============================================================================

/// Sparse phase-space vector.
///
/// A dense vector is deliberately avoided so that a large multimode system
/// does not require memory proportional to every possible coordinate.
#[derive(Debug, Clone, PartialEq)]
pub struct SparsePhaseSpaceVector {
    entries: BTreeMap<usize, Parameter>,
}

impl SparsePhaseSpaceVector {
    /// Creates an empty vector.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    /// Inserts or replaces one coordinate.
    pub fn insert(
        &mut self,
        coordinate: usize,
        value: Parameter,
    ) {
        self.entries.insert(coordinate, value);
    }

    /// Returns one coordinate.
    #[must_use]
    pub fn get(&self, coordinate: usize) -> Option<&Parameter> {
        self.entries.get(&coordinate)
    }

    /// Returns all non-zero/explicit entries.
    #[must_use]
    pub fn entries(&self) -> &BTreeMap<usize, Parameter> {
        &self.entries
    }

    /// Returns whether the vector is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the number of explicit entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl Default for SparsePhaseSpaceVector {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Sparse symplectic transformation
// =============================================================================

/// Sparse representation of a phase-space transformation.
///
/// The semantic dimension is not stored as a fixed-size array. Only explicit
/// entries are represented.
///
/// This permits very large transformations to remain sparse and lets later
/// compiler stages choose a suitable numerical representation.
#[derive(Debug, Clone, PartialEq)]
pub struct SparseSymplecticTransform {
    entries: BTreeMap<(usize, usize), Parameter>,
}

impl SparseSymplecticTransform {
    /// Creates an empty sparse transformation.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    /// Inserts one matrix entry.
    pub fn insert(
        &mut self,
        row: usize,
        column: usize,
        value: Parameter,
    ) -> CvResult<()> {
        if self.entries.contains_key(&(row, column)) {
            return Err(
                ContinuousVariableError::ConflictingTransformationEntry,
            );
        }

        self.entries.insert((row, column), value);
        Ok(())
    }

    /// Replaces one matrix entry intentionally.
    pub fn set(
        &mut self,
        row: usize,
        column: usize,
        value: Parameter,
    ) {
        self.entries.insert((row, column), value);
    }

    /// Returns one explicit matrix entry.
    #[must_use]
    pub fn get(
        &self,
        row: usize,
        column: usize,
    ) -> Option<&Parameter> {
        self.entries.get(&(row, column))
    }

    /// Returns explicit matrix entries.
    #[must_use]
    pub fn entries(&self) -> &BTreeMap<(usize, usize), Parameter> {
        &self.entries
    }

    /// Returns whether no entries are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the number of explicit entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl Default for SparseSymplecticTransform {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// CV program
// =============================================================================

/// A complete continuous-variable semantic program.
///
/// This is deliberately not a simulator state.
#[derive(Debug, Clone, PartialEq)]
pub struct ContinuousVariableProgram {
    /// Program operations in semantic order.
    operations: Vec<CvOperation>,

    /// Named CV mode declarations.
    modes: BTreeMap<ModeId, CvModeDeclaration>,

    /// Optional program-level metadata.
    metadata: BTreeMap<String, String>,
}

/// Declaration of one CV mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CvModeDeclaration {
    /// Canonical mode identity.
    id: ModeId,

    /// Optional human-readable name.
    name: Option<String>,
}

impl CvModeDeclaration {
    /// Creates a mode declaration.
    pub fn new(id: ModeId) -> Self {
        Self {
            id,
            name: None,
        }
    }

    /// Assigns a human-readable name.
    pub fn with_name<S: Into<String>>(mut self, name: S) -> CvResult<Self> {
        let name = name.into();
        validate_name(&name)?;
        self.name = Some(name);
        Ok(self)
    }

    /// Returns the mode identity.
    #[must_use]
    pub const fn id(&self) -> ModeId {
        self.id
    }

    /// Returns the optional name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl ContinuousVariableProgram {
    /// Creates an empty CV program.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            operations: Vec::new(),
            modes: BTreeMap::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Declares a mode.
    pub fn declare_mode(&mut self, mode: CvModeDeclaration) -> CvResult<()> {
        if self.modes.contains_key(&mode.id()) {
            return Err(ContinuousVariableError::InvalidValue {
                field: "mode",
                reason: "mode is already declared",
            });
        }

        self.modes.insert(mode.id(), mode);
        Ok(())
    }

    /// Adds an operation.
    pub fn push(&mut self, operation: CvOperation) {
        self.operations.push(operation);
    }

    /// Adds deterministic metadata.
    pub fn set_metadata<S1, S2>(
        &mut self,
        key: S1,
        value: S2,
    ) -> CvResult<()>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let key = key.into();
        validate_name(&key)?;

        self.metadata.insert(key, value.into());
        Ok(())
    }

    /// Returns declared modes.
    #[must_use]
    pub fn modes(&self) -> &BTreeMap<ModeId, CvModeDeclaration> {
        &self.modes
    }

    /// Returns semantic operations.
    #[must_use]
    pub fn operations(&self) -> &[CvOperation] {
        &self.operations
    }

    /// Returns metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Returns whether the program contains no operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Validates the complete semantic program.
    pub fn validate(&self) -> CvResult<()> {
        for operation in &self.operations {
            validate_operation_kind(operation.kind())?;

            for subsystem in operation.modes().iter() {
                if let CvSubsystem::Mode(mode) = subsystem {
                    if !self.modes.contains_key(mode) {
                        return Err(ContinuousVariableError::InvalidValue {
                            field: "operation.mode",
                            reason: "operation references an undeclared mode",
                        });
                    }
                }
            }
        }

        Ok(())
    }
}

impl Default for ContinuousVariableProgram {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Extension model
// =============================================================================

/// Generic CV extension.
///
/// This is intentionally explicit rather than silently ignoring unknown
/// semantic constructs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CvExtension {
    /// Extension namespace.
    namespace: String,

    /// Extension name.
    name: String,

    /// Deterministic attributes.
    attributes: BTreeMap<String, String>,
}

impl CvExtension {
    /// Creates an extension.
    pub fn new<N, S>(
        namespace: N,
        name: S,
    ) -> CvResult<Self>
    where
        N: Into<String>,
        S: Into<String>,
    {
        let namespace = namespace.into();
        let name = name.into();

        if namespace.is_empty() {
            return Err(ContinuousVariableError::EmptyExtensionNamespace);
        }

        validate_name(&name)?;

        Ok(Self {
            namespace,
            name,
            attributes: BTreeMap::new(),
        })
    }

    /// Adds an extension attribute.
    pub fn set_attribute<K, V>(
        &mut self,
        key: K,
        value: V,
    ) -> CvResult<()>
    where
        K: Into<String>,
        V: Into<String>,
    {
        let key = key.into();
        validate_name(&key)?;

        self.attributes.insert(key, value.into());
        Ok(())
    }

    /// Returns namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns extension name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns deterministic attributes.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_name(name: &str) -> CvResult<()> {
    if name.is_empty() {
        return Err(ContinuousVariableError::EmptyName);
    }

    // The policy deliberately limits names to a deterministic compiler-safe
    // ASCII identifier vocabulary. Semantic values themselves are not subject
    // to this rule.
    if name.len() > 4096 {
        return Err(ContinuousVariableError::NameTooLong);
    }

    for (position, byte) in name.bytes().enumerate() {
        let valid = byte.is_ascii_alphanumeric()
            || byte == b'_'
            || byte == b'.'
            || byte == b':'
            || byte == b'-';

        if !valid {
            return Err(
                ContinuousVariableError::InvalidNameCharacter { position },
            );
        }
    }

    Ok(())
}

fn validate_operation_kind(kind: &CvOperationKind) -> CvResult<()> {
    match kind {
        CvOperationKind::Gaussian(operation) => {
            validate_gaussian_operation(operation)
        }

        CvOperationKind::NonGaussian(operation) => {
            validate_non_gaussian_operation(operation)
        }

        CvOperationKind::Prepare(preparation) => {
            validate_state_preparation(preparation)
        }

        CvOperationKind::Measure(measurement) => {
            validate_measurement(measurement)
        }

        CvOperationKind::Barrier => Ok(()),

        CvOperationKind::Extension {
            namespace,
            name,
            ..
        } => {
            if namespace.is_empty() {
                return Err(
                    ContinuousVariableError::EmptyExtensionNamespace,
                );
            }

            validate_name(name)
        }
    }
}

fn validate_gaussian_operation(
    operation: &GaussianOperation,
) -> CvResult<()> {
    match operation {
        GaussianOperation::Displacement {
            real,
            imaginary,
        } => {
            validate_parameter(real, "displacement.real")?;
            validate_parameter(imaginary, "displacement.imaginary")
        }

        GaussianOperation::Rotation { angle } => {
            validate_parameter(angle, "rotation.angle")
        }

        GaussianOperation::Squeezing {
            magnitude,
            phase,
        } => {
            validate_parameter(magnitude, "squeezing.magnitude")?;
            validate_parameter(phase, "squeezing.phase")
        }

        GaussianOperation::BeamSplitter {
            angle,
            phase,
        } => {
            validate_parameter(angle, "beam_splitter.angle")?;
            validate_parameter(phase, "beam_splitter.phase")
        }

        GaussianOperation::ControlledPhase { strength } => {
            validate_parameter(strength, "controlled_phase.strength")
        }

        GaussianOperation::TwoModeSqueezing {
            magnitude,
            phase,
        } => {
            validate_parameter(magnitude, "two_mode_squeezing.magnitude")?;
            validate_parameter(phase, "two_mode_squeezing.phase")
        }

        GaussianOperation::Symplectic {
            transform,
            displacement,
        } => {
            validate_sparse_transform(transform)?;
            validate_sparse_vector(displacement)
        }
    }
}

fn validate_non_gaussian_operation(
    operation: &NonGaussianOperation,
) -> CvResult<()> {
    match operation {
        NonGaussianOperation::CubicPhase { strength } => {
            validate_parameter(strength, "cubic_phase.strength")
        }

        NonGaussianOperation::Kerr { strength } => {
            validate_parameter(strength, "kerr.strength")
        }

        NonGaussianOperation::NumberPhase { strength } => {
            validate_parameter(strength, "number_phase.strength")
        }

        NonGaussianOperation::PolynomialPhase { coefficients } => {
            for coefficient in coefficients {
                validate_parameter(coefficient, "polynomial_phase.coefficient")?;
            }

            Ok(())
        }

        NonGaussianOperation::HamiltonianEvolution {
            hamiltonian,
            duration,
        } => {
            validate_hamiltonian(hamiltonian)?;
            validate_parameter(duration, "hamiltonian_evolution.duration")
        }

        NonGaussianOperation::Extension {
            namespace,
            name,
            ..
        } => {
            if namespace.is_empty() {
                return Err(
                    ContinuousVariableError::EmptyExtensionNamespace,
                );
            }

            validate_name(name)
        }
    }
}

fn validate_state_preparation(
    preparation: &CvStatePreparation,
) -> CvResult<()> {
    match preparation {
        CvStatePreparation::Vacuum => Ok(()),

        CvStatePreparation::Coherent {
            alpha_real,
            alpha_imaginary,
        } => {
            validate_parameter(alpha_real, "coherent.alpha_real")?;
            validate_parameter(
                alpha_imaginary,
                "coherent.alpha_imaginary",
            )
        }

        CvStatePreparation::Squeezed {
            magnitude,
            phase,
        } => {
            validate_parameter(magnitude, "squeezed.magnitude")?;
            validate_parameter(phase, "squeezed.phase")
        }

        CvStatePreparation::DisplacedSqueezed {
            displacement_real,
            displacement_imaginary,
            squeezing,
            phase,
        } => {
            validate_parameter(
                displacement_real,
                "displaced_squeezed.displacement_real",
            )?;
            validate_parameter(
                displacement_imaginary,
                "displaced_squeezed.displacement_imaginary",
            )?;
            validate_parameter(squeezing, "displaced_squeezed.squeezing")?;
            validate_parameter(phase, "displaced_squeezed.phase")
        }

        CvStatePreparation::Thermal {
            mean_occupation,
        } => {
            validate_parameter(
                mean_occupation,
                "thermal.mean_occupation",
            )
        }

        CvStatePreparation::Fock { occupation } => {
            if let FockOccupation::Symbol(name) = occupation {
                validate_name(name)?;
            }

            Ok(())
        }

        CvStatePreparation::Cat {
            amplitude,
            relative_phase,
            ..
        } => {
            validate_parameter(amplitude, "cat.amplitude")?;
            validate_parameter(relative_phase, "cat.relative_phase")
        }

        CvStatePreparation::Extension {
            namespace,
            name,
            ..
        } => {
            if namespace.is_empty() {
                return Err(
                    ContinuousVariableError::EmptyExtensionNamespace,
                );
            }

            validate_name(name)
        }
    }
}

fn validate_measurement(
    measurement: &CvMeasurement,
) -> CvResult<()> {
    match measurement {
        CvMeasurement::Homodyne { observable } => {
            validate_parameter(observable.phase(), "measurement.phase")
        }

        CvMeasurement::Heterodyne => Ok(()),

        CvMeasurement::PhotonNumber => Ok(()),

        CvMeasurement::Parity => Ok(()),

        CvMeasurement::Observable { observable } => {
            validate_observable(observable)
        }

        CvMeasurement::Extension {
            namespace,
            name,
            ..
        } => {
            if namespace.is_empty() {
                return Err(
                    ContinuousVariableError::EmptyExtensionNamespace,
                );
            }

            validate_name(name)
        }
    }
}

fn validate_observable(
    observable: &CvObservable,
) -> CvResult<()> {
    match observable {
        CvObservable::Polynomial { terms } => {
            if terms.is_empty() {
                return Err(
                    ContinuousVariableError::MissingMeasurementObservable,
                );
            }

            for term in terms {
                validate_parameter(
                    term.coefficient(),
                    "observable.coefficient",
                )?;
            }

            Ok(())
        }

        CvObservable::Number => Ok(()),

        CvObservable::Parity => Ok(()),

        CvObservable::Extension {
            namespace,
            name,
            ..
        } => {
            if namespace.is_empty() {
                return Err(
                    ContinuousVariableError::EmptyExtensionNamespace,
                );
            }

            validate_name(name)
        }
    }
}

fn validate_hamiltonian(
    hamiltonian: &CvHamiltonian,
) -> CvResult<()> {
    if hamiltonian.is_empty() {
        return Err(ContinuousVariableError::EmptyHamiltonian);
    }

    for term in hamiltonian.terms() {
        if term.factors().is_empty() {
            return Err(ContinuousVariableError::EmptyHamiltonianTerm);
        }

        validate_parameter(
            term.coefficient(),
            "hamiltonian.coefficient",
        )?;
    }

    Ok(())
}

fn validate_sparse_transform(
    transform: &SparseSymplecticTransform,
) -> CvResult<()> {
    for ((row, column), _) in transform.entries() {
        if *row == usize::MAX || *column == usize::MAX {
            return Err(
                ContinuousVariableError::InvalidMatrixCoordinate {
                    row: *row,
                    column: *column,
                },
            );
        }
    }

    Ok(())
}

fn validate_sparse_vector(
    vector: &SparsePhaseSpaceVector,
) -> CvResult<()> {
    for coordinate in vector.entries().keys() {
        if *coordinate == usize::MAX {
            return Err(
                ContinuousVariableError::InvalidMatrixCoordinate {
                    row: *coordinate,
                    column: 0,
                },
            );
        }
    }

    Ok(())
}

fn validate_parameter(
    parameter: &Parameter,
    field: &'static str,
) -> CvResult<()> {
    match parameter {
        Parameter::Constant(value) if !value.is_finite() => {
            Err(ContinuousVariableError::NonFiniteValue { field })
        }

        _ => Ok(()),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn constant(value: f64) -> Parameter {
        Parameter::constant(value).expect("finite test constant")
    }

    #[test]
    fn mode_identity_is_stable() {
        let mode = ModeId::new(42);

        assert_eq!(mode.value(), 42);
        assert_eq!(mode.to_string(), "m42");
    }

    #[test]
    fn mode_set_rejects_duplicates() {
        let mut modes = CvModeSet::new();

        modes.push(ModeId::new(0).into()).expect("first insertion");

        let result = modes.push(ModeId::new(0).into());

        assert_eq!(
            result,
            Err(ContinuousVariableError::DuplicateMode)
        );
    }

    #[test]
    fn mode_set_supports_large_identifiers() {
        let mut modes = CvModeSet::new();

        modes
            .push(ModeId::new(u64::MAX).into())
            .expect("large identifier");

        assert_eq!(modes.len(), 1);
    }

    #[test]
    fn encoded_qubit_uses_canonical_qubit_id() {
        let qubit = QubitId::new(7);
        let subsystem = CvSubsystem::EncodedQubit(qubit);

        assert_eq!(subsystem.qubit(), Some(qubit));
        assert_eq!(subsystem.mode(), None);
    }

    #[test]
    fn vacuum_preparation_is_valid() {
        let operation = CvOperation::new(
            [ModeId::new(0).into()],
            CvOperationKind::Prepare(
                CvStatePreparation::Vacuum,
            ),
        )
        .expect("valid vacuum preparation");

        assert_eq!(operation.modes().len(), 1);
    }

    #[test]
    fn coherent_state_is_symbolic() {
        let operation = CvOperation::new(
            [ModeId::new(0).into()],
            CvOperationKind::Prepare(
                CvStatePreparation::Coherent {
                    alpha_real: Parameter::symbol("alpha")
                        .expect("valid symbol"),
                    alpha_imaginary: constant(0.0),
                },
            ),
        )
        .expect("valid coherent state");

        assert!(operation.modes().contains(ModeId::new(0).into()));
    }

    #[test]
    fn gaussian_rotation_is_valid() {
        let operation = CvOperation::new(
            [ModeId::new(0).into()],
            CvOperationKind::Gaussian(
                GaussianOperation::Rotation {
                    angle: Parameter::symbol("theta")
                        .expect("valid symbol"),
                },
            ),
        )
        .expect("valid rotation");

        assert!(matches!(
            operation.kind(),
            CvOperationKind::Gaussian(
                GaussianOperation::Rotation { .. }
            )
        ));
    }

    #[test]
    fn sparse_transform_is_deterministic() {
        let mut transform = SparseSymplecticTransform::new();

        transform
            .insert(0, 0, constant(1.0))
            .expect("first entry");

        transform
            .insert(1, 1, constant(1.0))
            .expect("second entry");

        assert_eq!(transform.len(), 2);
        assert_eq!(
            transform.get(0, 0),
            Some(&constant(1.0))
        );
    }

    #[test]
    fn duplicate_sparse_entry_is_rejected() {
        let mut transform = SparseSymplecticTransform::new();

        transform
            .insert(0, 0, constant(1.0))
            .expect("first entry");

        let result =
            transform.insert(0, 0, constant(2.0));

        assert_eq!(
            result,
            Err(
                ContinuousVariableError::ConflictingTransformationEntry
            )
        );
    }

    #[test]
    fn non_gaussian_hamiltonian_is_symbolic() {
        let term = CvHamiltonianTerm::new(
            Parameter::symbol("kappa").expect("symbol"),
            vec![
                CvHamiltonianFactor::Number {
                    mode: ModeId::new(0),
                },
            ],
        )
        .expect("valid term");

        let hamiltonian =
            CvHamiltonian::from_terms(vec![term])
                .expect("valid Hamiltonian");

        let operation = CvOperation::new(
            [ModeId::new(0).into()],
            CvOperationKind::NonGaussian(
                NonGaussianOperation::HamiltonianEvolution {
                    hamiltonian,
                    duration: Parameter::symbol("t")
                        .expect("symbol"),
                },
            ),
        )
        .expect("valid evolution");

        assert_eq!(operation.modes().len(), 1);
    }

    #[test]
    fn homodyne_measurement_is_valid() {
        let observable =
            QuadratureObservable::rotated(
                Parameter::symbol("phi").expect("symbol"),
            );

        let operation = CvOperation::new(
            [ModeId::new(0).into()],
            CvOperationKind::Measure(
                CvMeasurement::Homodyne { observable },
            ),
        )
        .expect("valid measurement");

        assert!(matches!(
            operation.kind(),
            CvOperationKind::Measure(
                CvMeasurement::Homodyne { .. }
            )
        ));
    }

    #[test]
    fn program_validation_rejects_undeclared_modes() {
        let mut program =
            ContinuousVariableProgram::new();

        program.push(
            CvOperation::new(
                [ModeId::new(9).into()],
                CvOperationKind::Barrier,
            )
            .expect("valid operation"),
        );

        assert!(matches!(
            program.validate(),
            Err(
                ContinuousVariableError::InvalidValue {
                    field: "operation.mode",
                    ..
                }
            )
        ));
    }

    #[test]
    fn program_validation_accepts_declared_modes() {
        let mut program =
            ContinuousVariableProgram::new();

        program
            .declare_mode(CvModeDeclaration::new(
                ModeId::new(0),
            ))
            .expect("mode declaration");

        program.push(
            CvOperation::new(
                [ModeId::new(0).into()],
                CvOperationKind::Barrier,
            )
            .expect("valid operation"),
        );

        program.validate().expect("valid program");
    }

    #[test]
    fn non_finite_parameters_are_rejected() {
        let result = Parameter::constant(f64::NAN);

        assert!(result.is_err());
    }

    #[test]
    fn extension_namespace_is_explicit() {
        let extension =
            CvExtension::new(
                "future.vendor",
                "custom_operation",
            )
            .expect("valid extension");

        assert_eq!(
            extension.namespace(),
            "future.vendor"
        );
        assert_eq!(
            extension.name(),
            "custom_operation"
        );
    }
}