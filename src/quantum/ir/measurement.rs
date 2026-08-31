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
//! - logical qubit measurement targets;
//! - logical classical result destinations;
//! - measurement basis/observable semantics;
//! - projective measurement;
//! - generalized measurement vocabulary;
//! - weak/continuous measurement vocabulary;
//! - parity/joint Pauli measurement vocabulary;
//! - destructive/non-destructive semantic intent;
//! - explicit reset-after-measurement intent;
//! - deterministic measurement grouping;
//! - measurement resource validation;
//! - logical namespace validation;
//! - canonical IR error conversion.
//!
//! It deliberately does NOT own:
//!
//! - physical readout channels;
//! - detectors;
//! - ADCs;
//! - amplifiers;
//! - resonators;
//! - laser readout;
//! - hardware measurement frequencies;
//! - calibration;
//! - measurement pulse generation;
//! - physical measurement duration;
//! - hardware scheduling;
//! - routing;
//! - QPU communication;
//! - simulator sampling;
//! - probability generation;
//! - decoding;
//! - error-correction decoding.
//!
//! Those responsibilities belong to downstream quantum compiler, hardware,
//! scheduling, backend, simulator, and QEC subsystems.
//!
//! # Universal-program principle
//!
//! A Zamani quantum program is written once at the semantic level and can be
//! lowered toward different target technologies.
//!
//! Consequently this module must not encode assumptions such as:
//!
//! - maximum 63 qubits;
//! - maximum 64 qubits;
//! - maximum 4096 qubits;
//! - one specific readout technology;
//! - one specific native measurement instruction.
//!
//! Logical identifiers are scalable `usize`-backed identifiers, while concrete
//! resource limits are supplied explicitly through `QuantumIrLimits`.
//!
//! `QuantumIrLimits` is a resource-safety policy, not the architectural
//! capacity of Zamani. A larger deployment may provide a larger explicit
//! policy or an unbounded trusted policy.
//!
//! # Measurement semantics
//!
//! The IR distinguishes:
//!
//! ```text
//! Measurement observable
//!         │
//!         ├── computational/Z
//!         ├── X
//!         ├── Y
//!         ├── Pauli
//!         ├── Pauli product / parity
//!         ├── generalized observable
//!         ├── weak measurement
//!         └── continuous measurement
//!
//! Measurement mode
//!         │
//!         ├── non-destructive
//!         └── destructive
//!
//! Post-measurement intent
//!         │
//!         └── optional explicit reset
//!
//! Result destination
//!         │
//!         └── logical classical bit
//! ```
//!
//! The IR describes semantic intent. A later compiler stage determines how a
//! particular machine implements that intent.
//!
//! # Relationship with OpenQASM and QIR
//!
//! OpenQASM 3 defines measurement as a semantic operation that measures in the
//! Z basis and writes the result into classical storage. OpenQASM also permits
//! qubits to remain available after measurement.
//!
//! QIR exposes distinct measurement forms such as computational-basis
//! measurement, result storage, and measurement-with-reset.
//!
//! Zamani intentionally provides a richer semantic vocabulary above those
//! lower-level forms so a single canonical representation can be lowered into
//! OpenQASM, QIR, hardware-native instructions, simulators, or future quantum
//! representations.
//!
//! # Atomicity
//!
//! Public mutation APIs validate before mutating whenever possible.
//!
//! A failed insertion into `MeasurementGroup` never partially inserts the
//! rejected measurement.
//!
//! # Determinism
//!
//! Measurement groups preserve insertion order.
//!
//! They do not silently sort, replace, deduplicate, or reorder measurements.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features.
//! No external dependencies.
//! No `unsafe` code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.

#![forbid(unsafe_code)]

use std::fmt;

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
// Classical bit identity
// =============================================================================
//
// This remains the canonical compatibility location for the current IR.
//
// When classical.rs is introduced, it should re-export this type rather than
// defining a second incompatible ClassicalBitId. That allows measurement.rs to
// remain frozen.

/// Logical classical-bit identifier.
///
/// This identifies a destination in the logical classical namespace.
///
/// It does not identify:
///
/// - hardware memory;
/// - a host CPU register;
/// - an ADC register;
/// - a QPU result buffer;
/// - a network location.
///
/// Physical storage is outside the canonical Quantum IR.
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
pub struct ClassicalBitId(usize);

impl ClassicalBitId {
    /// Creates a logical classical-bit identifier.
    ///
    /// This does not establish membership in a particular classical register.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the logical classical-bit index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }

    /// Returns the next identifier when representable.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<usize> for ClassicalBitId {
    fn from(index: usize) -> Self {
        Self::new(index)
    }
}

impl From<ClassicalBitId> for usize {
    fn from(bit: ClassicalBitId) -> usize {
        bit.index()
    }
}

impl fmt::Display for ClassicalBitId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "c{}", self.0)
    }
}

// =============================================================================
// Classical register
// =============================================================================

/// Logical classical-bit namespace used by measurement results.
///
/// The register stores only logical namespace capacity.
///
/// It does not represent physical hardware memory.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct ClassicalRegister {
    bits: usize,
}

impl ClassicalRegister {
    /// Creates a logical classical register without allocation.
    ///
    /// This constructor does not bypass any validation performed by a
    /// containing circuit/program.
    #[must_use]
    pub const fn new(bits: usize) -> Self {
        Self { bits }
    }

    /// Creates a register under an explicit IR resource policy.
    pub fn try_new(
        bits: usize,
        limits: &QuantumIrLimits,
    ) -> IrResult<Self> {
        limits
            .check_classical_bits(bits)
            .map_err(limit_error)?;

        Ok(Self { bits })
    }

    /// Returns the number of logical classical bits.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.bits
    }

    /// Returns whether the register contains no bits.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.bits == 0
    }

    /// Validates membership of a classical-bit identifier.
    pub fn validate(
        &self,
        bit: ClassicalBitId,
    ) -> Result<(), MeasurementError> {
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

    /// Validates membership using the canonical IR error type.
    pub fn validate_ir(
        &self,
        bit: ClassicalBitId,
    ) -> IrResult<()> {
        self.validate(bit)?;
        Ok(())
    }
}

// =============================================================================
// Measurement basis
// =============================================================================

/// Canonical measurement basis.
///
/// The first three variants cover the most common single-qubit Pauli bases.
///
/// `Pauli` allows a generic Pauli observable without coupling this file to a
/// matrix representation.
///
/// `Custom` is an explicitly named semantic observable. The name identifies
/// the observable contract; it does not contain hardware information.
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

    /// A Pauli observable.
    Pauli(PauliAxis),

    /// A named semantic observable.
    ///
    /// The name is an IR-level semantic identifier, not a device identifier.
    Custom(String),
}

impl Default for MeasurementBasis {
    fn default() -> Self {
        Self::Z
    }
}

impl MeasurementBasis {
    /// Returns whether this is a standard single-qubit Pauli basis.
    #[must_use]
    pub const fn is_standard_pauli(&self) -> bool {
        matches!(
            self,
            Self::Z | Self::X | Self::Y | Self::Pauli(_)
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
}

impl fmt::Display for MeasurementBasis {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

// =============================================================================
// Pauli axis
// =============================================================================

/// Pauli observable axis.
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
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X => formatter.write_str("X"),
            Self::Y => formatter.write_str("Y"),
            Self::Z => formatter.write_str("Z"),
        }
    }
}

// =============================================================================
// Pauli product / parity measurement
// =============================================================================

/// One factor in a Pauli-product measurement.
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
    /// Creates a Pauli factor.
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
/// This is a semantic observable. It does not dictate whether the target
/// hardware implements it directly or decomposes it into gates and a
/// computational-basis measurement.
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
    /// Creates an empty Pauli product.
    ///
    /// An empty product is not a valid measurement observable; validation will
    /// reject it.
    #[must_use]
    pub fn new() -> Self {
        Self {
            factors: Vec::new(),
        }
    }

    /// Creates a Pauli product from factors.
    pub fn from_factors(
        factors: Vec<PauliFactor>,
    ) -> Result<Self, MeasurementError> {
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

    /// Returns the factors in deterministic caller-supplied order.
    #[must_use]
    pub fn factors(&self) -> &[PauliFactor] {
        &self.factors
    }

    /// Adds a factor while preserving insertion order.
    pub fn push(
        &mut self,
        factor: PauliFactor,
    ) -> Result<(), MeasurementError> {
        if self
            .factors
            .iter()
            .any(|existing| existing.qubit() == factor.qubit())
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

    /// Validates the observable.
    pub fn validate(
        &self,
    ) -> Result<(), MeasurementError> {
        if self.factors.is_empty() {
            return Err(
                MeasurementError::EmptyPauliProduct,
            );
        }

        for (index, factor) in self.factors.iter().enumerate() {
            for previous in &self.factors[..index] {
                if previous.qubit() == factor.qubit() {
                    return Err(
                        MeasurementError::DuplicateQubit {
                            qubit: factor.qubit(),
                        },
                    );
                }
            }
        }

        Ok(())
    }

    /// Validates every qubit against a logical namespace.
    pub fn validate_qubits(
        &self,
        num_qubits: usize,
    ) -> Result<(), MeasurementError> {
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

/// Semantic class of quantum measurement.
///
/// This is intentionally broader than a single `measure` instruction so the
/// canonical IR can represent future quantum technologies without replacing
/// its measurement model.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub enum MeasurementKind {
    /// Standard projective measurement.
    Projective,

    /// Generalized measurement/POVM semantics.
    ///
    /// The string is a semantic observable/POVM identifier. Its physical
    /// realization is determined downstream.
    Generalized(String),

    /// Weak measurement.
    ///
    /// The strength is a semantic parameter in a normalized representation.
    Weak,

    /// Continuous measurement.
    Continuous,

    /// Joint Pauli-product/parity measurement.
    PauliProduct(PauliProduct),
}

impl Default for MeasurementKind {
    fn default() -> Self {
        Self::Projective
    }
}

impl MeasurementKind {
    /// Returns true for projective measurement.
    #[must_use]
    pub const fn is_projective(&self) -> bool {
        matches!(self, Self::Projective)
    }

    /// Returns true for generalized measurement.
    #[must_use]
    pub const fn is_generalized(&self) -> bool {
        matches!(self, Self::Generalized(_))
    }

    /// Returns true for weak measurement.
    #[must_use]
    pub const fn is_weak(&self) -> bool {
        matches!(self, Self::Weak)
    }

    /// Returns true for continuous measurement.
    #[must_use]
    pub const fn is_continuous(&self) -> bool {
        matches!(self, Self::Continuous)
    }

    /// Returns true for Pauli-product measurement.
    #[must_use]
    pub const fn is_pauli_product(&self) -> bool {
        matches!(self, Self::PauliProduct(_))
    }
}

// =============================================================================
// Measurement mode
// =============================================================================

/// Semantic measurement mode.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum MeasurementMode {
    /// The logical qubit remains available in the semantic program.
    ///
    /// This is the default and matches the common mid-circuit measurement
    /// model.
    NonDestructive,

    /// The measurement consumes the logical state according to the abstract
    /// semantic contract.
    ///
    /// Physical destruction is a backend concern.
    Destructive,
}

impl Default for MeasurementMode {
    fn default() -> Self {
        Self::NonDestructive
    }
}

impl MeasurementMode {
    /// Returns true if the measurement is destructive.
    #[must_use]
    pub const fn is_destructive(self) -> bool {
        matches!(self, Self::Destructive)
    }

    /// Returns true if the measurement is non-destructive.
    #[must_use]
    pub const fn is_non_destructive(self) -> bool {
        matches!(self, Self::NonDestructive)
    }
}

// =============================================================================
// Measurement error
// =============================================================================

/// Errors local to measurement construction and validation.
///
/// This type intentionally remains independent from hardware/backend errors.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum MeasurementError {
    /// Logical qubit is outside its declared namespace.
    QubitOutOfRange {
        qubit: QubitId,
        num_qubits: usize,
    },

    /// Classical destination is outside its declared namespace.
    ClassicalBitOutOfRange {
        bit: ClassicalBitId,
        num_classical_bits: usize,
    },

    /// Same logical qubit appears twice in one measurement group or observable.
    DuplicateQubit {
        qubit: QubitId,
    },

    /// Same classical destination is used twice in one measurement group.
    DuplicateClassicalTarget {
        bit: ClassicalBitId,
    },

    /// A Pauli product contains no factors.
    EmptyPauliProduct,

    /// A semantic measurement name is empty.
    EmptyObservableName,

    /// A generalized measurement identifier is empty.
    EmptyGeneralizedMeasurementName,

    /// Measurement configuration is semantically invalid.
    InvalidMeasurement {
        message: String,
    },

    /// A requested combination of semantic fields is invalid.
    InvalidConfiguration {
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
                "logical qubit {qubit} is outside range 0..{num_qubits}"
            ),

            Self::ClassicalBitOutOfRange {
                bit,
                num_classical_bits,
            } => write!(
                formatter,
                "classical bit {bit} is outside range 0..{num_classical_bits}"
            ),

            Self::DuplicateQubit { qubit } => write!(
                formatter,
                "logical qubit {qubit} occurs more than once"
            ),

            Self::DuplicateClassicalTarget { bit } => write!(
                formatter,
                "classical destination {bit} is used more than once"
            ),

            Self::EmptyPauliProduct => {
                formatter.write_str(
                    "Pauli-product measurement requires at least one factor",
                )
            }

            Self::EmptyObservableName => {
                formatter.write_str(
                    "measurement observable name must not be empty",
                )
            }

            Self::EmptyGeneralizedMeasurementName => {
                formatter.write_str(
                    "generalized measurement identifier must not be empty",
                )
            }

            Self::InvalidMeasurement { message } => {
                write!(
                    formatter,
                    "invalid measurement: {message}"
                )
            }

            Self::InvalidConfiguration { message } => {
                write!(
                    formatter,
                    "invalid measurement configuration: {message}"
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

            MeasurementError::DuplicateQubit { .. } => {
                measurement_error(
                    IrErrorCode::InvalidMeasurement,
                    message,
                )
            }

            MeasurementError::DuplicateClassicalTarget { .. } => {
                measurement_error(
                    IrErrorCode::InvalidMeasurement,
                    message,
                )
            }

            MeasurementError::EmptyPauliProduct
            | MeasurementError::EmptyObservableName
            | MeasurementError::EmptyGeneralizedMeasurementName
            | MeasurementError::InvalidMeasurement { .. }
            | MeasurementError::InvalidConfiguration { .. } => {
                measurement_error(
                    IrErrorCode::InvalidMeasurement,
                    message,
                )
            }
        }
    }
}

fn limit_error(error: LimitsError) -> IrError {
    match error {
        LimitsError::ResourceExceeded {
            resource,
            requested,
            maximum,
        } => {
            let actual = u64::try_from(requested)
                .unwrap_or(u64::MAX);

            let maximum = u64::try_from(maximum)
                .unwrap_or(u64::MAX);

            measurement_error(
                IrErrorCode::LimitExceeded,
                format!(
                    "measurement resource limit exceeded for {}: \
                     requested {}, maximum {}",
                    resource,
                    actual,
                    maximum,
                ),
            )
        }

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
                "arithmetic overflow while checking measurement resource `{}`",
                resource,
            ),
        ),

        LimitsError::ArithmeticMultiplicationOverflow {
            resource,
        } => measurement_error(
            IrErrorCode::ResourceOverflow,
            format!(
                "arithmetic multiplication overflow while checking measurement \
                 resource `{}`",
                resource,
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
                "schedule-time policy exceeded: requested {}, maximum {}",
                requested,
                maximum,
            ),
        ),
    }
}

// =============================================================================
// Measurement
// =============================================================================

/// A single logical quantum measurement operation.
///
/// This is the canonical measurement object consumed by the circuit and
/// operation layers.
///
/// It contains semantic intent only.
///
/// Hardware readout implementation is deliberately absent.
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
    basis: MeasurementBasis,
    kind: MeasurementKind,
    mode: MeasurementMode,
    reset_after: bool,
}

impl Measurement {
    /// Creates a standard computational/Z-basis projective measurement.
    #[must_use]
    pub const fn new(
        qubit: QubitId,
        classical_bit: ClassicalBitId,
    ) -> Self {
        Self {
            qubit,
            classical_bit,
            basis: MeasurementBasis::Z,
            kind: MeasurementKind::Projective,
            mode: MeasurementMode::NonDestructive,
            reset_after: false,
        }
    }

    /// Creates a standard projective measurement in a selected basis.
    #[must_use]
    pub fn in_basis(
        qubit: QubitId,
        classical_bit: ClassicalBitId,
        basis: MeasurementBasis,
    ) -> Self {
        Self {
            qubit,
            classical_bit,
            basis,
            kind: MeasurementKind::Projective,
            mode: MeasurementMode::NonDestructive,
            reset_after: false,
        }
    }

    /// Creates a generalized semantic measurement.
    pub fn generalized<S: Into<String>>(
        qubit: QubitId,
        classical_bit: ClassicalBitId,
        observable: S,
    ) -> Result<Self, MeasurementError> {
        let observable = observable.into();

        if observable.is_empty() {
            return Err(
                MeasurementError::EmptyGeneralizedMeasurementName,
            );
        }

        Ok(Self {
            qubit,
            classical_bit,
            basis: MeasurementBasis::Custom(
                observable.clone(),
            ),
            kind: MeasurementKind::Generalized(
                observable,
            ),
            mode: MeasurementMode::NonDestructive,
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
            basis,
            kind: MeasurementKind::Weak,
            mode: MeasurementMode::NonDestructive,
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
            basis,
            kind: MeasurementKind::Continuous,
            mode: MeasurementMode::NonDestructive,
            reset_after: false,
        }
    }

    /// Creates a Pauli-product/parity measurement.
    pub fn pauli_product(
        product: PauliProduct,
        classical_bit: ClassicalBitId,
    ) -> Result<Self, MeasurementError> {
        product.validate()?;

        let first_axis = product
            .factors()
            .first()
            .map(|factor| factor.axis())
            .ok_or(
                MeasurementError::EmptyPauliProduct,
            )?;

        let basis = MeasurementBasis::Pauli(first_axis);

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
            basis,
            kind: MeasurementKind::PauliProduct(product),
            mode: MeasurementMode::NonDestructive,
            reset_after: false,
        })
    }

    /// Returns the primary logical qubit.
    ///
    /// For ordinary measurements this is the only target.
    ///
    /// For a Pauli-product measurement this is the first factor and should
    /// only be used where an API explicitly accepts the distinction.
    #[must_use]
    pub const fn qubit(&self) -> QubitId {
        self.qubit
    }

    /// Returns the logical classical destination.
    #[must_use]
    pub const fn classical_bit(&self) -> ClassicalBitId {
        self.classical_bit
    }

    /// Returns the semantic measurement basis.
    #[must_use]
    pub fn basis(&self) -> &MeasurementBasis {
        &self.basis
    }

    /// Returns the semantic measurement kind.
    #[must_use]
    pub fn kind(&self) -> &MeasurementKind {
        &self.kind
    }

    /// Returns the measurement mode.
    #[must_use]
    pub const fn mode(&self) -> MeasurementMode {
        self.mode
    }

    /// Returns whether reset-after-measurement was explicitly requested.
    #[must_use]
    pub const fn reset_after(&self) -> bool {
        self.reset_after
    }

    /// Returns whether this measurement is destructive.
    #[must_use]
    pub const fn is_destructive(&self) -> bool {
        self.mode.is_destructive()
    }

    /// Returns whether this is a standard projective measurement.
    #[must_use]
    pub fn is_projective(&self) -> bool {
        self.kind.is_projective()
    }

    /// Returns whether this is a generalized measurement.
    #[must_use]
    pub fn is_generalized(&self) -> bool {
        self.kind.is_generalized()
    }

    /// Returns whether this is a weak measurement.
    #[must_use]
    pub fn is_weak(&self) -> bool {
        self.kind.is_weak()
    }

    /// Returns whether this is continuous.
    #[must_use]
    pub fn is_continuous(&self) -> bool {
        self.kind.is_continuous()
    }

    /// Returns whether this is a joint Pauli-product measurement.
    #[must_use]
    pub fn is_pauli_product(&self) -> bool {
        self.kind.is_pauli_product()
    }

    /// Returns all logical qubits semantically touched by this measurement.
    ///
    /// The returned vector is deterministic and preserves the semantic order.
    ///
    /// This method is intentionally explicit rather than pretending that a
    /// Pauli-product measurement has only one operand.
    #[must_use]
    pub fn qubits(&self) -> Vec<QubitId> {
        match &self.kind {
            MeasurementKind::PauliProduct(product) => product
                .factors()
                .iter()
                .map(|factor| factor.qubit())
                .collect(),

            _ => vec![self.qubit],
        }
    }

    /// Changes the basis while preserving all other semantic fields.
    pub fn set_basis(
        &mut self,
        basis: MeasurementBasis,
    ) {
        self.basis = basis;
    }

    /// Changes the semantic measurement kind.
    pub fn set_kind(
        &mut self,
        kind: MeasurementKind,
    ) -> Result<(), MeasurementError> {
        if let MeasurementKind::Generalized(name) = &kind {
            if name.is_empty() {
                return Err(
                    MeasurementError::EmptyGeneralizedMeasurementName,
                );
            }
        }

        if let MeasurementKind::PauliProduct(product) = &kind {
            product.validate()?;
        }

        self.kind = kind;
        Ok(())
    }

    /// Changes the measurement mode.
    pub fn set_mode(
        &mut self,
        mode: MeasurementMode,
    ) {
        self.mode = mode;
    }

    /// Sets or clears explicit reset-after-measurement intent.
    pub fn set_reset_after(
        &mut self,
        reset: bool,
    ) {
        self.reset_after = reset;
    }

    /// Returns a destructive measurement.
    #[must_use]
    pub fn destructive(mut self) -> Self {
        self.mode = MeasurementMode::Destructive;
        self
    }

    /// Returns a measurement with explicit reset intent.
    #[must_use]
    pub fn followed_by_reset(mut self) -> Self {
        self.reset_after = true;
        self
    }

    /// Clears explicit reset intent.
    #[must_use]
    pub fn without_reset(mut self) -> Self {
        self.reset_after = false;
        self
    }

    /// Validates the measurement against logical namespace sizes.
    pub fn validate(
        &self,
        num_qubits: usize,
        num_classical_bits: usize,
    ) -> Result<(), MeasurementError> {
        if self.qubit.index() >= num_qubits {
            return Err(
                MeasurementError::QubitOutOfRange {
                    qubit: self.qubit,
                    num_qubits,
                },
            );
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

        match &self.kind {
            MeasurementKind::Generalized(name) => {
                if name.is_empty() {
                    return Err(
                        MeasurementError::EmptyGeneralizedMeasurementName,
                    );
                }
            }

            MeasurementKind::PauliProduct(product) => {
                product.validate_qubits(num_qubits)?;
            }

            _ => {}
        }

        if let MeasurementBasis::Custom(name) = &self.basis {
            if name.is_empty() {
                return Err(
                    MeasurementError::EmptyObservableName,
                );
            }
        }

        Ok(())
    }

    /// Validates the measurement against logical namespaces and the explicit
    /// IR resource policy.
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

        let operand_count = self
            .qubits()
            .len();

        limits
            .check_operands(operand_count)
            .map_err(limit_error)?;

        Ok(())
    }
}

// =============================================================================
// Measurement group
// =============================================================================

/// Deterministically ordered collection of measurement operations.
///
/// The group is a semantic IR grouping, not a hardware readout batch.
///
/// A group guarantees:
///
/// - no duplicate primary logical qubits;
/// - no duplicate classical destinations;
/// - deterministic insertion order;
/// - atomic rejection of invalid insertions.
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

    /// Creates a measurement group from an ordered collection.
    pub fn from_measurements(
        measurements: Vec<Measurement>,
    ) -> Result<Self, MeasurementError> {
        let mut group = Self::new();

        for measurement in measurements {
            group.push(measurement)?;
        }

        Ok(group)
    }

    /// Creates a group under an explicit IR resource policy.
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

    /// Returns an immutable ordered view.
    #[must_use]
    pub fn measurements(&self) -> &[Measurement] {
        &self.measurements
    }

    /// Returns an item by deterministic position.
    #[must_use]
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&Measurement> {
        self.measurements.get(index)
    }

    /// Adds a measurement atomically.
    pub fn push(
        &mut self,
        measurement: Measurement,
    ) -> Result<(), MeasurementError> {
        self.ensure_unique(&measurement)?;
        self.measurements.push(measurement);
        Ok(())
    }

    /// Adds a measurement under an explicit resource policy.
    pub fn push_with_limits(
        &mut self,
        measurement: Measurement,
        limits: &QuantumIrLimits,
    ) -> IrResult<()> {
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

        measurement.validate_with_limits(
            usize::MAX,
            usize::MAX,
            limits,
        )?;

        self.measurements.push(measurement);

        Ok(())
    }

    /// Returns the first measurement whose primary qubit matches.
    #[must_use]
    pub fn for_qubit(
        &self,
        qubit: QubitId,
    ) -> Option<&Measurement> {
        self.measurements
            .iter()
            .find(|measurement| {
                measurement.qubits().contains(&qubit)
            })
    }

    /// Returns the measurement targeting a classical bit.
    #[must_use]
    pub fn for_classical_bit(
        &self,
        bit: ClassicalBitId,
    ) -> Option<&Measurement> {
        self.measurements
            .iter()
            .find(|measurement| {
                measurement.classical_bit() == bit
            })
    }

    /// Validates all group entries against logical namespaces.
    pub fn validate(
        &self,
        num_qubits: usize,
        num_classical_bits: usize,
    ) -> Result<(), MeasurementError> {
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

    /// Validates the group against namespaces and resource policy.
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
    ) -> Result<(), MeasurementError> {
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
                    .into_iter()
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
// Public construction helpers
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

/// Creates an explicitly destructive Z-basis measurement.
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

/// Creates a measurement with explicit reset-after-measurement semantics.
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
    fn qubit_namespace_uses_canonical_qubit_module() {
        let qubit =
            QubitId::new(17);

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
    fn classical_bit_identity_is_stable() {
        let bit =
            ClassicalBitId::new(9);

        assert_eq!(
            bit.index(),
            9
        );

        assert_eq!(
            bit.to_string(),
            "c9"
        );

        assert_eq!(
            usize::from(bit),
            9
        );
    }

    #[test]
    fn default_measurement_is_z_projective_and_non_destructive() {
        let measurement =
            Measurement::new(
                QubitId::new(0),
                ClassicalBitId::new(0),
            );

        assert_eq!(
            measurement.basis(),
            &MeasurementBasis::Z
        );

        assert!(
            measurement.is_projective()
        );

        assert!(
            measurement
                .mode()
                .is_non_destructive()
        );

        assert!(
            !measurement
                .reset_after()
        );
    }

    #[test]
    fn x_measurement_is_x_basis() {
        let measurement =
            measure_x(
                QubitId::new(3),
                ClassicalBitId::new(2),
            );

        assert_eq!(
            measurement.basis(),
            &MeasurementBasis::X
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
        let measurement =
            measure_y(
                QubitId::new(3),
                ClassicalBitId::new(2),
            );

        assert_eq!(
            measurement.basis(),
            &MeasurementBasis::Y
        );
    }

    #[test]
    fn destructive_measurement_is_explicit() {
        let measurement =
            measure_destructive(
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
    fn reset_after_measurement_is_explicit() {
        let measurement =
            measure_and_reset(
                QubitId::new(0),
                ClassicalBitId::new(0),
            );

        assert!(
            measurement.reset_after()
        );
    }

    #[test]
    fn measurement_namespace_validation_is_strict() {
        let measurement =
            Measurement::new(
                QubitId::new(4),
                ClassicalBitId::new(0),
            );

        let result =
            measurement.validate(
                4,
                1,
            );

        assert!(
            matches!(
                result,
                Err(
                    MeasurementError::QubitOutOfRange {
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn classical_namespace_validation_is_strict() {
        let measurement =
            Measurement::new(
                QubitId::new(0),
                ClassicalBitId::new(4),
            );

        let result =
            measurement.validate(
                1,
                4,
            );

        assert!(
            matches!(
                result,
                Err(
                    MeasurementError::ClassicalBitOutOfRange {
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn group_preserves_insertion_order() {
        let first =
            measure(
                QubitId::new(2),
                ClassicalBitId::new(0),
            );

        let second =
            measure(
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
            .push(
                measure(
                    QubitId::new(0),
                    ClassicalBitId::new(0),
                ),
            )
            .expect("first insertion");

        let result =
            group.push(
                measure(
                    QubitId::new(0),
                    ClassicalBitId::new(1),
                ),
            );

        assert!(
            result.is_err()
        );

        assert_eq!(
            group.len(),
            1
        );
    }

    #[test]
    fn group_rejects_duplicate_classical_destinations() {
        let mut group =
            MeasurementGroup::new();

        group
            .push(
                measure(
                    QubitId::new(0),
                    ClassicalBitId::new(0),
                ),
            )
            .expect("first insertion");

        let result =
            group.push(
                measure(
                    QubitId::new(1),
                    ClassicalBitId::new(0),
                ),
            );

        assert!(
            matches!(
                result,
                Err(
                    MeasurementError::DuplicateClassicalTarget {
                        ..
                    }
                )
            )
        );

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
            .push(
                PauliFactor::new(
                    QubitId::new(0),
                    PauliAxis::X,
                ),
            )
            .expect("first factor");

        let result =
            product.push(
                PauliFactor::new(
                    QubitId::new(0),
                    PauliAxis::Z,
                ),
            );

        assert!(
            matches!(
                result,
                Err(
                    MeasurementError::DuplicateQubit {
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn pauli_product_is_scalable() {
        let mut product =
            PauliProduct::new();

        for index in 0usize..128usize {
            product
                .push(
                    PauliFactor::new(
                        QubitId::new(index),
                        PauliAxis::Z,
                    ),
                )
                .expect("unique factor");
        }

        assert_eq!(
            product.len(),
            128
        );
    }

    #[test]
    fn generalized_measurement_requires_name() {
        let result =
            Measurement::generalized(
                QubitId::new(0),
                ClassicalBitId::new(0),
                "",
            );

        assert!(
            matches!(
                result,
                Err(
                    MeasurementError::EmptyGeneralizedMeasurementName
                )
            )
        );
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
    fn continuous_measurement_is_representable() {
        let measurement =
            Measurement::continuous(
                QubitId::new(0),
                ClassicalBitId::new(0),
                MeasurementBasis::Z,
            );

        assert!(
            measurement.is_continuous()
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

        let second =
            measure(
                QubitId::new(1),
                ClassicalBitId::new(1),
            );

        let result =
            MeasurementGroup::from_measurements(
                vec![first, second],
            );

        assert!(
            matches!(
                result,
                Err(
                    MeasurementError::DuplicateQubit {
                        qubit
                    }
                ) if qubit == QubitId::new(1)
            )
        );
    }

    #[test]
    fn resource_validation_uses_explicit_policy() {
        let limits =
            production_limits();

        let measurement =
            measure(
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
    fn large_identifier_is_not_an_architectural_error() {
        let qubit =
            QubitId::new(
                usize::MAX - 1,
            );

        let measurement =
            measure(
                qubit,
                ClassicalBitId::new(
                    usize::MAX - 1,
                ),
            );

        let result =
            measurement.validate(
                usize::MAX,
                usize::MAX,
            );

        assert!(
            result.is_ok()
        );
    }

    #[test]
    fn failed_group_insertion_does_not_mutate_group() {
        let mut group =
            MeasurementGroup::new();

        group
            .push(
                measure(
                    QubitId::new(0),
                    ClassicalBitId::new(0),
                ),
            )
            .expect("first insertion");

        let before =
            group.clone();

        let _ =
            group.push(
                measure(
                    QubitId::new(0),
                    ClassicalBitId::new(7),
                ),
            );

        assert_eq!(
            group,
            before
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
}