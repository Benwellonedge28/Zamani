//! Zamani Quantum IR — Measurement Semantics
//!
//! Canonical, hardware-independent representation of quantum measurement.
//!
//! # Architectural role
//!
//! This module defines the semantic meaning of measurement in the canonical
//! Zamani Quantum IR.
//!
//! It owns:
//!
//! - logical measurement targets;
//! - logical classical result destinations;
//! - measurement observable semantics;
//! - projective measurements;
//! - generalized measurements;
//! - weak measurements;
//! - continuous measurements;
//! - joint Pauli-product/parity measurements;
//! - destructive/non-destructive intent;
//! - explicit post-measurement reset intent;
//! - deterministic measurement collections;
//! - measurement-local validation;
//! - measurement resource-policy validation;
//! - canonical conversion into `IrError`.
//!
//! It deliberately does NOT own:
//!
//! - physical readout channels;
//! - ADCs/DACs;
//! - resonators;
//! - detectors;
//! - amplifiers;
//! - laser systems;
//! - hardware readout frequencies;
//! - calibration;
//! - measurement pulse generation;
//! - physical scheduling;
//! - routing;
//! - QPU communication;
//! - simulator state or sampling;
//! - probability generation;
//! - decoder implementation;
//! - QEC decoding.
//!
//! Those responsibilities belong to downstream hardware, backend, simulator,
//! scheduling, routing, or QEC subsystems.
//!
//! # Universal-program principle
//!
//! A Zamani program describes semantic intent independently of the target
//! machine. Measurement therefore must not contain:
//!
//! - a fixed maximum qubit count;
//! - a fixed maximum classical-bit count;
//! - a vendor-specific readout model;
//! - a vendor-specific result buffer;
//! - a fixed number of measurement outcomes;
//! - a fixed hardware measurement duration.
//!
//! Concrete resource limits are supplied through `QuantumIrLimits`.
//!
//! # Canonical identity boundary
//!
//! Logical quantum targets use:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! Logical classical destinations use:
//!
//! ```text
//! quantum::ir::classical::ClassicalBitId
//! ```
//!
//! This file deliberately does NOT define another `ClassicalBitId`.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration
//!
//! `qubit.rs`
//!     Supplies canonical `QubitId`.
//!
//! `classical.rs`
//!     Supplies canonical `ClassicalBitId`.
//!
//! `operation.rs`
//!     Uses `Measurement` as the canonical measurement operation body.
//!
//! `validation.rs`
//!     Performs whole-program namespace and semantic validation.
//!
//! `limits.rs`
//!     Supplies explicit resource/security limits.
//!
//! `serialization.rs`
//!     Serializes this deterministic semantic structure.
//!
//! `hash.rs`
//!     Hashes canonical measurement structure through the IR hashing layer.
//!
//! `analysis.rs`
//!     Reads measurement targets and classical dependencies.
//!
//! `control_flow.rs`
//!     Consumes measurement destinations when constructing dynamic control.
//!
//! `hardware/`
//!     Determines whether and how a target can implement the semantic
//!     measurement.
//!
//! `simulator/`
//!     Interprets the semantic measurement without becoming part of the IR.
//!
//! `qec/`
//!     May consume measurement and syndrome information without changing the
//!     canonical measurement model.
//!
//! # Important semantic rule
//!
//! Measurement is not synonymous with "Z-basis measurement of one qubit".
//!
//! The canonical model supports:
//!
//! ```text
//! projective measurement
//! X / Y / Z measurement
//! arbitrary named semantic observables
//! generalized/POVM measurement vocabulary
//! weak measurement
//! continuous measurement
//! joint Pauli-product measurement
//! destructive measurement intent
//! non-destructive measurement intent
//! explicit reset-after-measurement intent
//! ```
//!
//! Physical realization is deliberately deferred.

#![forbid(unsafe_code)]

use std::fmt;

use super::classical::ClassicalBitId;
use super::errors::{
    measurement_error,
    IrError,
    IrErrorCode,
    IrResult,
};
use super::limits::{
    LimitsError,
    QuantumIrLimits,
};
use super::qubit::QubitId;

// =============================================================================
// Result aliases
// =============================================================================

/// Result type for local measurement construction and validation.
pub type MeasurementResult<T> = Result<T, MeasurementError>;

// =============================================================================
// Classical register compatibility helper
// =============================================================================

/// Logical classical-bit namespace used for measurement destinations.
///
/// This is a lightweight semantic view and intentionally does not duplicate
/// the canonical `ClassicalBitId` type from `classical.rs`.
///
/// The actual classical-resource model remains owned by `classical.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MeasurementClassicalNamespace {
    bits: usize,
}

impl MeasurementClassicalNamespace {
    /// Creates a logical classical namespace of `bits` bits.
    ///
    /// No runtime storage is allocated.
    #[must_use]
    pub const fn new(bits: usize) -> Self {
        Self { bits }
    }

    /// Returns the number of logical classical bits.
    #[must_use]
    pub const fn len(self) -> usize {
        self.bits
    }

    /// Returns whether the namespace is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.bits == 0
    }

    /// Validates a classical destination.
    pub fn validate(
        self,
        bit: ClassicalBitId,
    ) -> MeasurementResult<()> {
        if bit.index() >= self.bits {
            return Err(
                MeasurementError::ClassicalBitOutOfRange {
                    bit,
                    num_classical_bits: self.bits,
                },
            );
        }

        Ok(())
    }
}

// =============================================================================
// Pauli axis
// =============================================================================

/// A single Pauli observable axis.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum PauliAxis {
    /// Pauli X.
    X,

    /// Pauli Y.
    Y,

    /// Pauli Z.
    Z,
}

impl fmt::Display for PauliAxis {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::X => formatter.write_str("X"),
            Self::Y => formatter.write_str("Y"),
            Self::Z => formatter.write_str("Z"),
        }
    }
}

// =============================================================================
// Measurement basis
// =============================================================================

/// Canonical single-system measurement basis/observable description.
///
/// `Custom` is a semantic observable identifier. It is not a hardware
/// channel, device name, or calibration identifier.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub enum MeasurementBasis {
    /// Computational/Z basis.
    Z,

    /// X basis.
    X,

    /// Y basis.
    Y,

    /// Explicit Pauli axis.
    Pauli(PauliAxis),

    /// Named semantic observable.
    Custom(String),
}

impl Default for MeasurementBasis {
    fn default() -> Self {
        Self::Z
    }
}

impl MeasurementBasis {
    /// Returns whether the basis is one of the standard Pauli bases.
    #[must_use]
    pub const fn is_standard_pauli(&self) -> bool {
        matches!(
            self,
            Self::Z
                | Self::X
                | Self::Y
                | Self::Pauli(_)
        )
    }

    /// Returns a stable semantic name.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Z => "Z",
            Self::X => "X",
            Self::Y => "Y",
            Self::Pauli(PauliAxis::X) => "X",
            Self::Pauli(PauliAxis::Y) => "Y",
            Self::Pauli(PauliAxis::Z) => "Z",
            Self::Custom(name) => name.as_str(),
        }
    }

    /// Validates the basis.
    pub fn validate(&self) -> MeasurementResult<()> {
        if let Self::Custom(name) = self {
            if name.trim().is_empty() {
                return Err(
                    MeasurementError::EmptyObservableName,
                );
            }
        }

        Ok(())
    }
}

impl fmt::Display for MeasurementBasis {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

// =============================================================================
// Measurement observable
// =============================================================================

/// Semantic observable selected by a measurement.
///
/// This separates "what is measured" from "how the result is stored".
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub enum MeasurementObservable {
    /// Standard basis measurement.
    Basis(MeasurementBasis),

    /// A named generalized/POVM observable.
    Generalized(String),

    /// A joint Pauli-product observable.
    PauliProduct(PauliProduct),
}

impl Default for MeasurementObservable {
    fn default() -> Self {
        Self::Basis(MeasurementBasis::Z)
    }
}

impl MeasurementObservable {
    /// Validates the observable.
    pub fn validate(&self) -> MeasurementResult<()> {
        match self {
            Self::Basis(basis) => basis.validate(),

            Self::Generalized(name) => {
                if name.trim().is_empty() {
                    return Err(
                        MeasurementError::EmptyGeneralizedMeasurementName,
                    );
                }

                Ok(())
            }

            Self::PauliProduct(product) => {
                product.validate()
            }
        }
    }

    /// Returns the logical qubits touched by this observable.
    ///
    /// The returned order is deterministic.
    pub fn qubits(&self) -> Vec<QubitId> {
        match self {
            Self::Basis(_) | Self::Generalized(_) => {
                Vec::new()
            }

            Self::PauliProduct(product) => product
                .factors()
                .iter()
                .map(|factor| factor.qubit())
                .collect(),
        }
    }

    /// Returns whether this is a Pauli-product observable.
    #[must_use]
    pub const fn is_pauli_product(&self) -> bool {
        matches!(self, Self::PauliProduct(_))
    }
}

// =============================================================================
// Pauli product
// =============================================================================

/// One factor of a joint Pauli observable.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct PauliFactor {
    qubit: QubitId,
    axis: PauliAxis,
}

impl PauliFactor {
    /// Creates one Pauli factor.
    #[must_use]
    pub const fn new(
        qubit: QubitId,
        axis: PauliAxis,
    ) -> Self {
        Self { qubit, axis }
    }

    /// Returns the logical qubit.
    #[must_use]
    pub const fn qubit(&self) -> QubitId {
        self.qubit
    }

    /// Returns the Pauli axis.
    #[must_use]
    pub const fn axis(&self) -> PauliAxis {
        self.axis
    }
}

/// A logical joint Pauli-product/parity observable.
///
/// Example:
///
/// ```text
/// X(q0) * Z(q3) * Y(q8)
/// ```
///
/// The product describes semantic intent only. A backend may implement it
/// directly or decompose it into another measurement sequence.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct PauliProduct {
    factors: Vec<PauliFactor>,
}

impl PauliProduct {
    /// Creates an empty product.
///
/// An empty product is invalid and will be rejected by `validate`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            factors: Vec::new(),
        }
    }

    /// Constructs and validates a Pauli product.
    pub fn from_factors(
        factors: Vec<PauliFactor>,
    ) -> MeasurementResult<Self> {
        let product = Self { factors };
        product.validate()?;
        Ok(product)
    }

    /// Returns the number of factors.
    #[must_use]
    pub fn len(&self) -> usize {
        self.factors.len()
    }

    /// Returns whether the product is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.factors.is_empty()
    }

    /// Returns the factors in their semantic order.
    #[must_use]
    pub fn factors(&self) -> &[PauliFactor] {
        &self.factors
    }

    /// Adds a factor without reordering existing factors.
    ///
    /// A qubit may occur at most once in a single Pauli product.
    pub fn push(
        &mut self,
        factor: PauliFactor,
    ) -> MeasurementResult<()> {
        if self
            .factors
            .iter()
            .any(|existing| {
                existing.qubit() == factor.qubit()
            })
        {
            return Err(
                MeasurementError::DuplicateQubit {
                    qubit: factor.qubit(),
                },
            );
        }

        self.factors.push(factor);
        Ok(())
    }

    /// Validates structural correctness.
    pub fn validate(&self) -> MeasurementResult<()> {
        if self.factors.is_empty() {
            return Err(
                MeasurementError::EmptyPauliProduct,
            );
        }

        for (index, factor) in
            self.factors.iter().enumerate()
        {
            if self.factors[..index]
                .iter()
                .any(|previous| {
                    previous.qubit() == factor.qubit()
                })
            {
                return Err(
                    MeasurementError::DuplicateQubit {
                        qubit: factor.qubit(),
                    },
                );
            }
        }

        Ok(())
    }

    /// Validates all logical qubit identifiers against a namespace size.
    pub fn validate_qubits(
        &self,
        num_qubits: usize,
    ) -> MeasurementResult<()> {
        self.validate()?;

        for factor in &self.factors {
            if factor.qubit().index() >= num_qubits {
                return Err(
                    MeasurementError::QubitOutOfRange {
                        qubit: factor.qubit(),
                        num_qubits,
                    },
                );
            }
        }

        Ok(())
    }
}

impl Default for PauliProduct {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Measurement kind
// =============================================================================

/// Semantic class of a measurement.
///
/// The kind describes the physical/semantic character of the measurement,
/// while `MeasurementObservable` describes what is being measured.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum MeasurementKind {
    /// Projective measurement.
    Projective,

    /// Generalized/POVM measurement.
    Generalized,

    /// Weak measurement.
    Weak,

    /// Continuous measurement.
    Continuous,
}

impl Default for MeasurementKind {
    fn default() -> Self {
        Self::Projective
    }
}

impl MeasurementKind {
    /// Returns true for projective measurement.
    #[must_use]
    pub const fn is_projective(self) -> bool {
        matches!(self, Self::Projective)
    }

    /// Returns true for generalized measurement.
    #[must_use]
    pub const fn is_generalized(self) -> bool {
        matches!(self, Self::Generalized)
    }

    /// Returns true for weak measurement.
    #[must_use]
    pub const fn is_weak(self) -> bool {
        matches!(self, Self::Weak)
    }

    /// Returns true for continuous measurement.
    #[must_use]
    pub const fn is_continuous(self) -> bool {
        matches!(self, Self::Continuous)
    }
}

// =============================================================================
// Measurement mode
// =============================================================================

/// Semantic post-measurement availability contract.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum MeasurementMode {
    /// The logical qubit remains part of the semantic program.
    NonDestructive,

    /// The semantic operation consumes/destroys the measured state.
    ///
    /// Whether a physical device actually destroys a carrier is a backend
    /// concern.
    Destructive,
}

impl Default for MeasurementMode {
    fn default() -> Self {
        Self::NonDestructive
    }
}

impl MeasurementMode {
    /// Returns whether the mode is destructive.
    #[must_use]
    pub const fn is_destructive(self) -> bool {
        matches!(self, Self::Destructive)
    }

    /// Returns whether the mode is non-destructive.
    #[must_use]
    pub const fn is_non_destructive(self) -> bool {
        matches!(self, Self::NonDestructive)
    }
}

// =============================================================================
// Result semantics
// =============================================================================

/// Semantic type of measurement output.
///
/// This does not identify a hardware buffer. It tells downstream lowering
/// what kind of classical information the measurement produces.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum MeasurementResultType {
    /// Binary measurement result.
    Bit,

    /// Finite discrete outcome represented by a logical integer value.
    Integer,

    /// Real-valued measurement result.
    Real,

    /// Complex-valued measurement result.
    Complex,

    /// Backend-defined opaque measurement result.
    Opaque,
}

impl Default for MeasurementResultType {
    fn default() -> Self {
        Self::Bit
    }
}

impl MeasurementResultType {
    /// Returns whether the result is binary.
    #[must_use]
    pub const fn is_bit(self) -> bool {
        matches!(self, Self::Bit)
    }

    /// Returns whether the result is numeric but not binary.
    #[must_use]
    pub const fn is_numeric(self) -> bool {
        matches!(
            self,
            Self::Integer
                | Self::Real
                | Self::Complex
        )
    }
}

// =============================================================================
// Measurement error
// =============================================================================

/// Local measurement construction/validation errors.
///
/// Whole-program errors can be converted into the canonical `IrError`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum MeasurementError {
    /// A logical qubit is outside its declared namespace.
    QubitOutOfRange {
        /// Invalid logical qubit.
        qubit: QubitId,

        /// Number of declared logical qubits.
        num_qubits: usize,
    },

    /// A classical destination is outside its declared namespace.
    ClassicalBitOutOfRange {
        /// Invalid destination.
        bit: ClassicalBitId,

        /// Number of declared classical bits.
        num_classical_bits: usize,
    },

    /// The same logical qubit appears more than once in one observable/group.
    DuplicateQubit {
        /// Duplicate logical qubit.
        qubit: QubitId,
    },

    /// Two measurements write to the same classical destination in one group.
    DuplicateClassicalTarget {
        /// Duplicate destination.
        bit: ClassicalBitId,
    },

    /// A Pauli product has no factors.
    EmptyPauliProduct,

    /// A named observable is empty.
    EmptyObservableName,

    /// A generalized measurement name is empty.
    EmptyGeneralizedMeasurementName,

    /// A semantic measurement configuration is invalid.
    InvalidMeasurement {
        /// Human-readable explanation.
        message: String,
    },

    /// A field combination is invalid.
    InvalidConfiguration {
        /// Human-readable explanation.
        message: String,
    },

    /// A measurement result type is incompatible with the current operation.
    InvalidResultType {
        /// Human-readable explanation.
        message: String,
    },
}

impl fmt::Display for MeasurementError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => write!(
                formatter,
                "logical qubit {qubit} is outside declared range 0..{num_qubits}"
            ),

            Self::ClassicalBitOutOfRange {
                bit,
                num_classical_bits,
            } => write!(
                formatter,
                "classical bit {bit} is outside declared range 0..{num_classical_bits}"
            ),

            Self::DuplicateQubit { qubit } => write!(
                formatter,
                "logical qubit {qubit} occurs more than once"
            ),

            Self::DuplicateClassicalTarget { bit } => write!(
                formatter,
                "classical destination {bit} is used more than once"
            ),

            Self::EmptyPauliProduct => formatter.write_str(
                "Pauli-product measurement requires at least one factor",
            ),

            Self::EmptyObservableName => formatter.write_str(
                "measurement observable name must not be empty",
            ),

            Self::EmptyGeneralizedMeasurementName => formatter.write_str(
                "generalized measurement identifier must not be empty",
            ),

            Self::InvalidMeasurement { message } => {
                write!(formatter, "invalid measurement: {message}")
            }

            Self::InvalidConfiguration { message } => {
                write!(
                    formatter,
                    "invalid measurement configuration: {message}"
                )
            }

            Self::InvalidResultType { message } => {
                write!(
                    formatter,
                    "invalid measurement result type: {message}"
                )
            }
        }
    }
}

impl std::error::Error for MeasurementError {}

impl From<MeasurementError> for IrError {
    fn from(error: MeasurementError) -> Self {
        let message = error.to_string();

        match error {
            MeasurementError::QubitOutOfRange { .. } => {
                measurement_error(
                    IrErrorCode::InvalidQubit,
                    message,
                )
            }

            MeasurementError::ClassicalBitOutOfRange { .. } => {
                measurement_error(
                    IrErrorCode::InvalidClassicalResource,
                    message,
                )
            }

            MeasurementError::DuplicateQubit { .. }
            | MeasurementError::DuplicateClassicalTarget { .. }
            | MeasurementError::EmptyPauliProduct
            | MeasurementError::EmptyObservableName
            | MeasurementError::EmptyGeneralizedMeasurementName
            | MeasurementError::InvalidMeasurement { .. }
            | MeasurementError::InvalidConfiguration { .. }
            | MeasurementError::InvalidResultType { .. } => {
                measurement_error(
                    IrErrorCode::InvalidMeasurement,
                    message,
                )
            }
        }
    }
}

// =============================================================================
// Limits conversion
// =============================================================================

fn limit_error(error: LimitsError) -> IrError {
    match error {
        LimitsError::ResourceExceeded {
            resource,
            requested,
            maximum,
        } => measurement_error(
            IrErrorCode::LimitExceeded,
            format!(
                "measurement resource limit exceeded for {resource}: \
                 requested {requested}, maximum {maximum}"
            ),
        ),

        LimitsError::InvalidConfiguration {
            field,
            value,
        } => measurement_error(
            IrErrorCode::LimitExceeded,
            format!(
                "invalid IR limit `{field}`: value {value}"
            ),
        ),

        LimitsError::ArithmeticOverflow {
            resource,
        } => measurement_error(
            IrErrorCode::ResourceOverflow,
            format!(
                "arithmetic overflow while checking measurement resource \
                 `{resource}`"
            ),
        ),

        LimitsError::ArithmeticMultiplicationOverflow {
            resource,
        } => measurement_error(
            IrErrorCode::ResourceOverflow,
            format!(
                "arithmetic multiplication overflow while checking measurement \
                 resource `{resource}`"
            ),
        ),

        LimitsError::TimeArithmeticOverflow => {
            measurement_error(
                IrErrorCode::ResourceOverflow,
                "schedule-time arithmetic overflow while validating measurement",
            )
        }

        LimitsError::ScheduleTimeExceeded {
            requested,
            maximum,
        } => measurement_error(
            IrErrorCode::LimitExceeded,
            format!(
                "schedule-time policy exceeded: requested {requested}, \
                 maximum {maximum}"
            ),
        ),
    }
}

// =============================================================================
// Measurement
// =============================================================================

/// Canonical semantic quantum measurement.
///
/// The primary target is represented explicitly by `qubit`.
///
/// Joint Pauli-product measurements additionally expose all semantic targets
/// through `qubits()`.
///
/// The classical destination remains the canonical logical
/// `classical::ClassicalBitId`, preserving compatibility with the rest of the
/// current IR while allowing future classical value/result abstractions to be
/// added in `classical.rs`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Measurement {
    qubit: QubitId,
    classical_bit: ClassicalBitId,
    observable: MeasurementObservable,
    kind: MeasurementKind,
    mode: MeasurementMode,
    result_type: MeasurementResultType,
    reset_after: bool,
}

impl Measurement {
    /// Creates a standard Z-basis projective measurement.
    #[must_use]
    pub const fn new(
        qubit: QubitId,
        classical_bit: ClassicalBitId,
    ) -> Self {
        Self {
            qubit,
            classical_bit,
            observable: MeasurementObservable::Basis(
                MeasurementBasis::Z,
            ),
            kind: MeasurementKind::Projective,
            mode: MeasurementMode::NonDestructive,
            result_type: MeasurementResultType::Bit,
            reset_after: false,
        }
    }

    /// Creates a projective measurement in a selected basis.
    #[must_use]
    pub fn in_basis(
        qubit: QubitId,
        classical_bit: ClassicalBitId,
        basis: MeasurementBasis,
    ) -> Self {
        Self {
            qubit,
            classical_bit,
            observable: MeasurementObservable::Basis(
                basis,
            ),
            kind: MeasurementKind::Projective,
            mode: MeasurementMode::NonDestructive,
            result_type: MeasurementResultType::Bit,
            reset_after: false,
        }
    }

    /// Creates a generalized semantic measurement.
    pub fn generalized<S: Into<String>>(
        qubit: QubitId,
        classical_bit: ClassicalBitId,
        observable: S,
    ) -> MeasurementResult<Self> {
        let observable = observable.into();

        if observable.trim().is_empty() {
            return Err(
                MeasurementError::EmptyGeneralizedMeasurementName,
            );
        }

        Ok(Self {
            qubit,
            classical_bit,
            observable: MeasurementObservable::Generalized(
                observable,
            ),
            kind: MeasurementKind::Generalized,
            mode: MeasurementMode::NonDestructive,
            result_type: MeasurementResultType::Bit,
            reset_after: false,
        })
    }

    /// Creates a weak measurement.
    #[must_use]
    pub fn weak(
        qubit: QubitId,
        classical_bit: ClassicalBitId,
        basis: MeasurementBasis,
    ) -> Self {
        Self {
            qubit,
            classical_bit,
            observable: MeasurementObservable::Basis(
                basis,
            ),
            kind: MeasurementKind::Weak,
            mode: MeasurementMode::NonDestructive,
            result_type: MeasurementResultType::Bit,
            reset_after: false,
        }
    }

    /// Creates a continuous measurement.
    #[must_use]
    pub fn continuous(
        qubit: QubitId,
        classical_bit: ClassicalBitId,
        basis: MeasurementBasis,
    ) -> Self {
        Self {
            qubit,
            classical_bit,
            observable: MeasurementObservable::Basis(
                basis,
            ),
            kind: MeasurementKind::Continuous,
            mode: MeasurementMode::NonDestructive,
            result_type: MeasurementResultType::Real,
            reset_after: false,
        }
    }

    /// Creates a joint Pauli-product measurement.
    pub fn pauli_product(
        product: PauliProduct,
        classical_bit: ClassicalBitId,
    ) -> MeasurementResult<Self> {
        product.validate()?;

        let first_qubit = product
            .factors()
            .first()
            .map(|factor| factor.qubit())
            .ok_or(
                MeasurementError::EmptyPauliProduct,
            )?;

        Ok(Self {
            qubit: first_qubit,
            classical_bit,
            observable: MeasurementObservable::PauliProduct(
                product,
            ),
            kind: MeasurementKind::Projective,
            mode: MeasurementMode::NonDestructive,
            result_type: MeasurementResultType::Bit,
            reset_after: false,
        })
    }

    /// Returns the primary logical qubit.
    #[must_use]
    pub const fn qubit(&self) -> QubitId {
        self.qubit
    }

    /// Returns the logical classical destination.
    #[must_use]
    pub const fn classical_bit(&self) -> ClassicalBitId {
        self.classical_bit
    }

    /// Returns the semantic observable.
    #[must_use]
    pub fn observable(&self) -> &MeasurementObservable {
        &self.observable
    }

    /// Returns the semantic basis where the observable is a basis observable.
    ///
    /// Returns `None` for generalized and Pauli-product observables.
    #[must_use]
    pub fn basis(&self) -> Option<&MeasurementBasis> {
        match &self.observable {
            MeasurementObservable::Basis(basis) => {
                Some(basis)
            }

            MeasurementObservable::Generalized(_)
            | MeasurementObservable::PauliProduct(_) => {
                None
            }
        }
    }

    /// Returns the semantic measurement kind.
    #[must_use]
    pub const fn kind(&self) -> MeasurementKind {
        self.kind
    }

    /// Returns the measurement mode.
    #[must_use]
    pub const fn mode(&self) -> MeasurementMode {
        self.mode
    }

    /// Returns the result type.
    #[must_use]
    pub const fn result_type(
        &self,
    ) -> MeasurementResultType {
        self.result_type
    }

    /// Returns whether reset-after-measurement was explicitly requested.
    #[must_use]
    pub const fn reset_after(&self) -> bool {
        self.reset_after
    }

    /// Returns whether the measurement is destructive.
    #[must_use]
    pub const fn is_destructive(&self) -> bool {
        self.mode.is_destructive()
    }

    /// Returns whether the measurement is projective.
    #[must_use]
    pub const fn is_projective(&self) -> bool {
        self.kind.is_projective()
    }

    /// Returns whether the measurement is generalized.
    #[must_use]
    pub const fn is_generalized(&self) -> bool {
        self.kind.is_generalized()
    }

    /// Returns whether the measurement is weak.
    #[must_use]
    pub const fn is_weak(&self) -> bool {
        self.kind.is_weak()
    }

    /// Returns whether the measurement is continuous.
    #[must_use]
    pub const fn is_continuous(&self) -> bool {
        self.kind.is_continuous()
    }

    /// Returns whether this is a joint Pauli-product measurement.
    #[must_use]
    pub fn is_pauli_product(&self) -> bool {
        self.observable.is_pauli_product()
    }

    /// Returns all logical qubits touched by the measurement.
    ///
    /// For ordinary measurements this contains exactly one qubit.
    ///
    /// For Pauli-product measurements it contains every factor in semantic
    /// order.
    pub fn qubits(&self) -> Vec<QubitId> {
        match &self.observable {
            MeasurementObservable::Basis(_)
            | MeasurementObservable::Generalized(_) => {
                vec![self.qubit]
            }

            MeasurementObservable::PauliProduct(
                product,
            ) => product
                .factors()
                .iter()
                .map(|factor| factor.qubit())
                .collect(),
        }
    }

    /// Returns the number of logical quantum operands.
    #[must_use]
    pub fn qubit_count(&self) -> usize {
        match &self.observable {
            MeasurementObservable::Basis(_)
            | MeasurementObservable::Generalized(_) => 1,

            MeasurementObservable::PauliProduct(
                product,
            ) => product.len(),
        }
    }

    /// Changes the basis for a basis measurement.
    pub fn set_basis(
        &mut self,
        basis: MeasurementBasis,
    ) -> MeasurementResult<()> {
        basis.validate()?;

        self.observable =
            MeasurementObservable::Basis(basis);

        Ok(())
    }

    /// Replaces the complete semantic observable.
    pub fn set_observable(
        &mut self,
        observable: MeasurementObservable,
    ) -> MeasurementResult<()> {
        observable.validate()?;

        match &observable {
            MeasurementObservable::PauliProduct(
                product,
            ) => {
                let first = product
                    .factors()
                    .first()
                    .ok_or(
                        MeasurementError::EmptyPauliProduct,
                    )?;

                self.qubit = first.qubit();
            }

            MeasurementObservable::Basis(_)
            | MeasurementObservable::Generalized(_) => {}
        }

        self.observable = observable;

        Ok(())
    }

    /// Changes the measurement kind.
    pub fn set_kind(
        &mut self,
        kind: MeasurementKind,
    ) -> MeasurementResult<()> {
        match kind {
            MeasurementKind::Projective => {}

            MeasurementKind::Generalized => {
                if !matches!(
                    self.observable,
                    MeasurementObservable::Generalized(_)
                ) {
                    return Err(
                        MeasurementError::InvalidConfiguration {
                            message:
                                "generalized measurement kind requires \
                                 a generalized observable"
                                    .to_string(),
                        },
                    );
                }
            }

            MeasurementKind::Weak
            | MeasurementKind::Continuous => {
                if !matches!(
                    self.observable,
                    MeasurementObservable::Basis(_)
                ) {
                    return Err(
                        MeasurementError::InvalidConfiguration {
                            message:
                                "weak and continuous measurement currently \
                                 require a basis observable"
                                    .to_string(),
                        },
                    );
                }
            }
        }

        self.kind = kind;
        Ok(())
    }

    /// Changes the measurement mode.
    pub const fn set_mode(
        &mut self,
        mode: MeasurementMode,
    ) {
        self.mode = mode;
    }

    /// Changes the semantic result type.
    ///
    /// Binary projective measurements normally use `Bit`.
    /// Continuous measurements may use `Real`.
    ///
    /// The method validates combinations that are intrinsically invalid at
    /// this layer without imposing hardware-specific restrictions.
    pub fn set_result_type(
        &mut self,
        result_type: MeasurementResultType,
    ) -> MeasurementResult<()> {
        if self.kind == MeasurementKind::Continuous
            && result_type == MeasurementResultType::Bit
        {
            return Err(
                MeasurementError::InvalidResultType {
                    message:
                        "continuous measurement cannot be represented as a \
                         binary-only result without an explicit downstream \
                         discretization step"
                            .to_string(),
                },
            );
        }

        self.result_type = result_type;

        Ok(())
    }

    /// Requests or clears explicit reset-after-measurement semantics.
    pub const fn set_reset_after(
        &mut self,
        reset: bool,
    ) {
        self.reset_after = reset;
    }

    /// Converts this measurement into destructive mode.
    #[must_use]
    pub const fn destructive(mut self) -> Self {
        self.mode = MeasurementMode::Destructive;
        self
    }

    /// Requests explicit reset after measurement.
    #[must_use]
    pub const fn followed_by_reset(
        mut self,
    ) -> Self {
        self.reset_after = true;
        self
    }

    /// Clears explicit reset intent.
    #[must_use]
    pub const fn without_reset(
        mut self,
    ) -> Self {
        self.reset_after = false;
        self
    }

    /// Changes the logical classical destination.
    ///
    /// This does not change the semantic measurement itself.
    pub const fn set_classical_bit(
        &mut self,
        bit: ClassicalBitId,
    ) {
        self.classical_bit = bit;
    }

    /// Performs local semantic validation.
    ///
    /// Namespace validation is separate because a `QubitId` and
    /// `ClassicalBitId` are meaningful independently of any particular
    /// program declaration.
    pub fn validate_semantics(
        &self,
    ) -> MeasurementResult<()> {
        if self
            .qubits()
            .is_empty()
        {
            return Err(
                MeasurementError::InvalidMeasurement {
                    message:
                        "measurement must reference at least one logical \
                         qubit"
                            .to_string(),
                },
            );
        }

        self.observable.validate()?;

        match self.kind {
            MeasurementKind::Projective => {}

            MeasurementKind::Generalized => {
                if !matches!(
                    self.observable,
                    MeasurementObservable::Generalized(_)
                ) {
                    return Err(
                        MeasurementError::InvalidConfiguration {
                            message:
                                "generalized measurement kind requires a \
                                 generalized observable"
                                    .to_string(),
                        },
                    );
                }
            }

            MeasurementKind::Weak => {
                if !matches!(
                    self.observable,
                    MeasurementObservable::Basis(_)
                ) {
                    return Err(
                        MeasurementError::InvalidConfiguration {
                            message:
                                "weak measurement requires a basis \
                                 observable"
                                    .to_string(),
                        },
                    );
                }
            }

            MeasurementKind::Continuous => {
                if !matches!(
                    self.observable,
                    MeasurementObservable::Basis(_)
                ) {
                    return Err(
                        MeasurementError::InvalidConfiguration {
                            message:
                                "continuous measurement requires a basis \
                                 observable"
                                    .to_string(),
                        },
                    );
                }
            }
        }

        if self.kind
            == MeasurementKind::Continuous
            && self.result_type
                == MeasurementResultType::Bit
        {
            return Err(
                MeasurementError::InvalidResultType {
                    message:
                        "continuous measurement requires a non-binary result \
                         representation"
                            .to_string(),
                },
            );
        }

        Ok(())
    }

    /// Validates logical namespaces.
    pub fn validate(
        &self,
        num_qubits: usize,
        num_classical_bits: usize,
    ) -> MeasurementResult<()> {
        self.validate_semantics()?;

        for qubit in self.qubits() {
            if qubit.index() >= num_qubits {
                return Err(
                    MeasurementError::QubitOutOfRange {
                        qubit,
                        num_qubits,
                    },
                );
            }
        }

        if self.classical_bit.index()
            >= num_classical_bits
        {
            return Err(
                MeasurementError::ClassicalBitOutOfRange {
                    bit: self.classical_bit,
                    num_classical_bits,
                },
            );
        }

        Ok(())
    }

    /// Validates semantic correctness and explicit resource policy.
    pub fn validate_with_limits(
        &self,
        num_qubits: usize,
        num_classical_bits: usize,
        limits: &QuantumIrLimits,
    ) -> IrResult<()> {
        self.validate(
            num_qubits,
            num_classical_bits,
        )?;

        limits
            .check_measurements(1)
            .map_err(limit_error)?;

        limits
            .check_operands(self.qubit_count())
            .map_err(limit_error)?;

        Ok(())
    }
}

// =============================================================================
// Measurement group
// =============================================================================

/// Deterministically ordered collection of measurements.
///
/// A group is a semantic collection, not necessarily a hardware batch.
///
/// The group guarantees:
///
/// - insertion order is preserved;
/// - no duplicate classical destination exists within the group;
/// - no overlapping quantum target exists within the group;
/// - rejected insertions do not mutate the group.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct MeasurementGroup {
    measurements: Vec<Measurement>,
}

impl MeasurementGroup {
    /// Creates an empty measurement group.
    #[must_use]
    pub fn new() -> Self {
        Self {
            measurements: Vec::new(),
        }
    }

    /// Creates and validates a group.
    pub fn from_measurements(
        measurements: Vec<Measurement>,
    ) -> MeasurementResult<Self> {
        let mut group = Self::new();

        for measurement in measurements {
            group.push(measurement)?;
        }

        Ok(group)
    }

    /// Creates a group under an explicit resource policy.
    pub fn from_measurements_with_limits(
        measurements: Vec<Measurement>,
        limits: &QuantumIrLimits,
    ) -> IrResult<Self> {
        limits
            .check_measurements(measurements.len())
            .map_err(limit_error)?;

        let mut group = Self::new();

        for measurement in measurements {
            group.push_with_limits(
                measurement,
                limits,
            )?;
        }

        Ok(group)
    }

    /// Returns the number of measurements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.measurements.len()
    }

    /// Returns whether the group is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.measurements.is_empty()
    }

    /// Returns the ordered measurements.
    #[must_use]
    pub fn measurements(&self) -> &[Measurement] {
        &self.measurements
    }

    /// Returns a measurement by insertion position.
    #[must_use]
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&Measurement> {
        self.measurements.get(index)
    }

    /// Inserts a measurement atomically.
    pub fn push(
        &mut self,
        measurement: Measurement,
    ) -> MeasurementResult<()> {
        self.ensure_unique(&measurement)?;
        self.measurements.push(measurement);
        Ok(())
    }

    /// Inserts a measurement under an explicit resource policy.
    ///
    /// Namespace validation remains the caller's responsibility because this
    /// group does not own the enclosing program's qubit/classical declarations.
    pub fn push_with_limits(
        &mut self,
        measurement: Measurement,
        limits: &QuantumIrLimits,
    ) -> IrResult<()> {
        measurement.validate_semantics()?;
        self.ensure_unique(&measurement)?;

        let next_count = self
            .measurements
            .len()
            .checked_add(1)
            .ok_or_else(|| {
                measurement_error(
                    IrErrorCode::ResourceOverflow,
                    "measurement count overflow",
                )
            })?;

        limits
            .check_measurements(next_count)
            .map_err(limit_error)?;

        limits
            .check_operands(measurement.qubit_count())
            .map_err(limit_error)?;

        self.measurements.push(measurement);

        Ok(())
    }

    /// Finds the first measurement touching a logical qubit.
    #[must_use]
    pub fn for_qubit(
        &self,
        qubit: QubitId,
    ) -> Option<&Measurement> {
        self.measurements
            .iter()
            .find(|measurement| {
                measurement
                    .qubits()
                    .contains(&qubit)
            })
    }

    /// Finds the measurement writing to a classical bit.
    #[must_use]
    pub fn for_classical_bit(
        &self,
        bit: ClassicalBitId,
    ) -> Option<&Measurement> {
        self.measurements
            .iter()
            .find(|measurement| {
                measurement.classical_bit()
                    == bit
            })
    }

    /// Validates the complete group against program namespaces.
    pub fn validate(
        &self,
        num_qubits: usize,
        num_classical_bits: usize,
    ) -> MeasurementResult<()> {
        for (index, measurement) in
            self.measurements.iter().enumerate()
        {
            measurement.validate(
                num_qubits,
                num_classical_bits,
            )?;

            for previous in
                &self.measurements[..index]
            {
                if measurement
                    .qubits()
                    .iter()
                    .any(|qubit| {
                        previous
                            .qubits()
                            .contains(qubit)
                    })
                {
                    let qubit = measurement
                        .qubits()
                        .into_iter()
                        .find(|qubit| {
                            previous
                                .qubits()
                                .contains(qubit)
                        })
                        .unwrap_or(
                            measurement.qubit(),
                        );

                    return Err(
                        MeasurementError::DuplicateQubit {
                            qubit,
                        },
                    );
                }

                if previous.classical_bit()
                    == measurement.classical_bit()
                {
                    return Err(
                        MeasurementError::DuplicateClassicalTarget {
                            bit: measurement
                                .classical_bit(),
                        },
                    );
                }
            }
        }

        Ok(())
    }

    /// Validates the complete group against namespaces and limits.
    pub fn validate_with_limits(
        &self,
        num_qubits: usize,
        num_classical_bits: usize,
        limits: &QuantumIrLimits,
    ) -> IrResult<()> {
        self.validate(
            num_qubits,
            num_classical_bits,
        )?;

        limits
            .check_measurements(self.measurements.len())
            .map_err(limit_error)?;

        for measurement in &self.measurements {
            measurement.validate_with_limits(
                num_qubits,
                num_classical_bits,
                limits,
            )?;
        }

        Ok(())
    }

    fn ensure_unique(
        &self,
        measurement: &Measurement,
    ) -> MeasurementResult<()> {
        let measurement_qubits =
            measurement.qubits();

        for existing in &self.measurements {
            if measurement_qubits
                .iter()
                .any(|qubit| {
                    existing
                        .qubits()
                        .contains(qubit)
                })
            {
                let qubit = measurement_qubits
                    .iter()
                    .copied()
                    .find(|qubit| {
                        existing
                            .qubits()
                            .contains(qubit)
                    })
                    .unwrap_or(
                        measurement.qubit(),
                    );

                return Err(
                    MeasurementError::DuplicateQubit {
                        qubit,
                    },
                );
            }

            if existing.classical_bit()
                == measurement.classical_bit()
            {
                return Err(
                    MeasurementError::DuplicateClassicalTarget {
                        bit: measurement
                            .classical_bit(),
                    },
                );
            }
        }

        Ok(())
    }
}

impl Default for MeasurementGroup {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Construction helpers
// =============================================================================

/// Creates a standard Z-basis measurement.
#[must_use]
pub const fn measure(
    qubit: QubitId,
    classical_bit: ClassicalBitId,
) -> Measurement {
    Measurement::new(
        qubit,
        classical_bit,
    )
}

/// Creates an X-basis measurement.
#[must_use]
pub fn measure_x(
    qubit: QubitId,
    classical_bit: ClassicalBitId,
) -> Measurement {
    Measurement::in_basis(
        qubit,
        classical_bit,
        MeasurementBasis::X,
    )
}

/// Creates a Y-basis measurement.
#[must_use]
pub fn measure_y(
    qubit: QubitId,
    classical_bit: ClassicalBitId,
) -> Measurement {
    Measurement::in_basis(
        qubit,
        classical_bit,
        MeasurementBasis::Y,
    )
}

/// Creates a destructive Z-basis measurement.
#[must_use]
pub fn measure_destructive(
    qubit: QubitId,
    classical_bit: ClassicalBitId,
) -> Measurement {
    Measurement::new(
        qubit,
        classical_bit,
    )
    .destructive()
}

/// Creates a measurement followed by explicit reset intent.
#[must_use]
pub fn measure_and_reset(
    qubit: QubitId,
    classical_bit: ClassicalBitId,
) -> Measurement {
    Measurement::new(
        qubit,
        classical_bit,
    )
    .followed_by_reset()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn production_limits() -> QuantumIrLimits {
        QuantumIrLimits::production()
    }

    #[test]
    fn canonical_qubit_module_is_used() {
        let qubit = QubitId::new(17);

        assert_eq!(
            qubit.index(),
            17
        );

        assert_eq!(
            qubit.to_string(),
            "q17"
        );
    }

    #[test]
    fn_canonical_classical_module_is_used() {
        let bit = ClassicalBitId::new(9);

        assert_eq!(
            bit.index(),
            9
        );

        assert_eq!(
            bit.to_string(),
            "c9"
        );
    }

    #[test]
    fn default_measurement_is_z_projective_bit_measurement() {
        let measurement = Measurement::new(
            QubitId::new(0),
            ClassicalBitId::new(0),
        );

        assert_eq!(
            measurement.basis(),
            Some(&MeasurementBasis::Z)
        );

        assert_eq!(
            measurement.kind(),
            MeasurementKind::Projective
        );

        assert_eq!(
            measurement.result_type(),
            MeasurementResultType::Bit
        );

        assert!(
            measurement
                .mode()
                .is_non_destructive()
        );
    }

    #[test]
    fn x_measurement_is_x_basis() {
        let measurement = measure_x(
            QubitId::new(3),
            ClassicalBitId::new(2),
        );

        assert_eq!(
            measurement.basis(),
            Some(&MeasurementBasis::X)
        );

        assert_eq!(
            measurement.qubit(),
            QubitId::new(3)
        );

        assert_eq!(
            measurement.classical_bit(),
            ClassicalBitId::new(2)
        );
    }

    #[test]
    fn y_measurement_is_y_basis() {
        let measurement = measure_y(
            QubitId::new(3),
            ClassicalBitId::new(2),
        );

        assert_eq!(
            measurement.basis(),
            Some(&MeasurementBasis::Y)
        );
    }

    #[test]
    fn destructive_measurement_is_explicit() {
        let measurement = measure_destructive(
            QubitId::new(0),
            ClassicalBitId::new(0),
        );

        assert!(
            measurement.is_destructive()
        );

        assert!(
            !measurement
                .mode()
                .is_non_destructive()
        );
    }

    #[test]
    fn reset_intent_is_explicit() {
        let measurement = measure_and_reset(
            QubitId::new(0),
            ClassicalBitId::new(0),
        );

        assert!(
            measurement.reset_after()
        );
    }

    #[test]
    fn namespace_validation_is_strict() {
        let measurement = Measurement::new(
            QubitId::new(4),
            ClassicalBitId::new(0),
        );

        assert!(matches!(
            measurement.validate(4, 1),
            Err(
                MeasurementError::QubitOutOfRange {
                    ..
                }
            )
        ));
    }

    #[test]
    fn classical_namespace_validation_is_strict() {
        let measurement = Measurement::new(
            QubitId::new(0),
            ClassicalBitId::new(4),
        );

        assert!(matches!(
            measurement.validate(1, 4),
            Err(
                MeasurementError::ClassicalBitOutOfRange {
                    ..
                }
            )
        ));
    }

    #[test]
    fn group_preserves_insertion_order() {
        let first = measure(
            QubitId::new(2),
            ClassicalBitId::new(0),
        );

        let second = measure(
            QubitId::new(0),
            ClassicalBitId::new(1),
        );

        let group =
            MeasurementGroup::from_measurements(
                vec![
                    first.clone(),
                    second.clone(),
                ],
            )
            .expect("valid measurement group");

        assert_eq!(
            group.get(0),
            Some(&first)
        );

        assert_eq!(
            group.get(1),
            Some(&second)
        );
    }

    #[test]
    fn group_rejects_duplicate_qubits_atomically() {
        let mut group =
            MeasurementGroup::new();

        group
            .push(measure(
                QubitId::new(0),
                ClassicalBitId::new(0),
            ))
            .expect("first insertion");

        let before = group.clone();

        let result = group.push(
            measure(
                QubitId::new(0),
                ClassicalBitId::new(1),
            ),
        );

        assert!(result.is_err());
        assert_eq!(
            group,
            before
        );
    }

    #[test]
    fn group_rejects_duplicate_classical_destinations() {
        let mut group =
            MeasurementGroup::new();

        group
            .push(measure(
                QubitId::new(0),
                ClassicalBitId::new(0),
            ))
            .expect("first insertion");

        let result = group.push(
            measure(
                QubitId::new(1),
                ClassicalBitId::new(0),
            ),
        );

        assert!(matches!(
            result,
            Err(
                MeasurementError::DuplicateClassicalTarget {
                    ..
                }
            )
        ));

        assert_eq!(
            group.len(),
            1
        );
    }

    #[test]
    fn pauli_product_rejects_duplicate_qubits() {
        let mut product =
            PauliProduct::new();

        product
            .push(PauliFactor::new(
                QubitId::new(0),
                PauliAxis::X,
            ))
            .expect("first factor");

        assert!(matches!(
            product.push(PauliFactor::new(
                QubitId::new(0),
                PauliAxis::Z,
            )),
            Err(
                MeasurementError::DuplicateQubit {
                    ..
                }
            )
        ));
    }

    #[test]
    fn pauli_product_is_scalable() {
        let mut product =
            PauliProduct::new();

        for index in 0usize..128usize {
            product
                .push(PauliFactor::new(
                    QubitId::new(index),
                    PauliAxis::Z,
                ))
                .expect("unique factor");
        }

        assert_eq!(
            product.len(),
            128
        );
    }

    #[test]
    fn generalized_measurement_requires_name() {
        assert!(matches!(
            Measurement::generalized(
                QubitId::new(0),
                ClassicalBitId::new(0),
                "",
            ),
            Err(
                MeasurementError::EmptyGeneralizedMeasurementName
            )
        ));
    }

    #[test]
    fn generalized_measurement_is_distinct_from_projective_measurement() {
        let measurement =
            Measurement::generalized(
                QubitId::new(0),
                ClassicalBitId::new(0),
                "custom_povm",
            )
            .expect("valid generalized measurement");

        assert!(
            measurement.is_generalized()
        );

        assert_eq!(
            measurement.kind(),
            MeasurementKind::Generalized
        );

        assert!(matches!(
            measurement.observable(),
            MeasurementObservable::Generalized(_)
        ));
    }

    #[test]
    fn weak_measurement_is_representable() {
        let measurement =
            Measurement::weak(
                QubitId::new(0),
                ClassicalBitId::new(0),
                MeasurementBasis::X,
            );

        assert!(
            measurement.is_weak()
        );

        assert!(
            !measurement.is_projective()
        );
    }

    #[test]
    fn continuous_measurement_uses_non_binary_result_semantics() {
        let measurement =
            Measurement::continuous(
                QubitId::new(0),
                ClassicalBitId::new(0),
                MeasurementBasis::Z,
            );

        assert!(
            measurement.is_continuous()
        );

        assert_eq!(
            measurement.result_type(),
            MeasurementResultType::Real
        );

        assert!(
            measurement
                .validate_semantics()
                .is_ok()
        );
    }

    #[test]
    fn pauli_product_measurement_is_representable() {
        let product =
            PauliProduct::from_factors(
                vec![
                    PauliFactor::new(
                        QubitId::new(0),
                        PauliAxis::X,
                    ),
                    PauliFactor::new(
                        QubitId::new(3),
                        PauliAxis::Z,
                    ),
                ],
            )
            .expect("valid product");

        let measurement =
            Measurement::pauli_product(
                product,
                ClassicalBitId::new(0),
            )
            .expect("valid measurement");

        assert!(
            measurement.is_pauli_product()
        );

        assert_eq!(
            measurement.qubits(),
            vec![
                QubitId::new(0),
                QubitId::new(3),
            ]
        );
    }

    #[test]
    fn group_rejects_overlapping_pauli_product_measurements() {
        let product =
            PauliProduct::from_factors(
                vec![
                    PauliFactor::new(
                        QubitId::new(0),
                        PauliAxis::X,
                    ),
                    PauliFactor::new(
                        QubitId::new(1),
                        PauliAxis::Z,
                    ),
                ],
            )
            .expect("valid product");

        let first =
            Measurement::pauli_product(
                product,
                ClassicalBitId::new(0),
            )
            .expect("valid measurement");

        let second = measure(
            QubitId::new(1),
            ClassicalBitId::new(1),
        );

        assert!(matches!(
            MeasurementGroup::from_measurements(
                vec![first, second],
            ),
            Err(
                MeasurementError::DuplicateQubit {
                    qubit
                }
            ) if qubit == QubitId::new(1)
        ));
    }

    #[test]
    fn resource_validation_uses_explicit_policy() {
        let limits =
            production_limits();

        let measurement = measure(
            QubitId::new(0),
            ClassicalBitId::new(0),
        );

        measurement
            .validate_with_limits(
                1,
                1,
                &limits,
            )
            .expect(
                "measurement should satisfy production policy",
            );
    }

    #[test]
    fn group_resource_validation_does_not_bypass_namespace_validation() {
        let limits =
            production_limits();

        let group =
            MeasurementGroup::from_measurements(
                vec![
                    measure(
                        QubitId::new(5),
                        ClassicalBitId::new(0),
                    ),
                ],
            )
            .expect("structurally valid group");

        assert!(matches!(
            group.validate_with_limits(
                1,
                1,
                &limits,
            ),
            Err(
                IrError {
                    ..
                }
            )
        ));
    }

    #[test]
    fn large_identifiers_are_not_architectural_limits() {
        let index =
            usize::MAX - 1;

        let measurement =
            measure(
                QubitId::new(index),
                ClassicalBitId::new(index),
            );

        assert!(
            measurement.validate(
                usize::MAX,
                usize::MAX,
            ).is_ok()
        );
    }

    #[test]
    fn canonical_error_conversion_is_measurement_domain() {
        let error =
            IrError::from(
                MeasurementError::EmptyPauliProduct,
            );

        assert_eq!(
            error.kind(),
            super::super::errors::IrErrorKind::Measurement
        );

        assert_eq!(
            error.code(),
            IrErrorCode::InvalidMeasurement
        );
    }

    #[test]
    fn classical_namespace_helper_is_allocation_free() {
        let namespace =
            MeasurementClassicalNamespace::new(
                usize::MAX,
            );

        assert!(
            namespace
                .validate(
                    ClassicalBitId::new(
                        usize::MAX - 1,
                    ),
                )
                .is_ok()
        );
    }
}