//! Zamani Quantum IR — Measurement Contract
//!
//! Hardware-independent representation of logical quantum measurements.
//!
//! # Architectural boundary
//!
//! This module owns the logical semantics of measurement:
//!
//! - logical qubit source;
//! - classical destination;
//! - measurement basis;
//! - destructive/non-destructive mode;
//! - explicit reset-after-measurement intent;
//! - deterministic measurement grouping.
//!
//! It deliberately does not own:
//!
//! - physical readout channels;
//! - detector configuration;
//! - pulse schedules;
//! - calibration;
//! - device topology;
//! - routing;
//! - QPU communication;
//! - measurement sampling/simulation.
//!
//! Those responsibilities belong to later compiler/backend stages.
//!
//! # Invariants
//!
//! 1. A `Measurement` always contains exactly one logical qubit and one
//!    classical destination.
//! 2. Identifier range is validated against the owning circuit/register before
//!    execution or lowering.
//! 3. A `MeasurementGroup` cannot contain the same logical qubit twice.
//! 4. A `MeasurementGroup` cannot target the same classical bit twice.
//! 5. Group insertion preserves caller-supplied ordering deterministically.
//! 6. Resource limits are explicit and can be checked without allocation.
//! 7. Logical measurement semantics remain independent of physical readout.
//! 8. Failed validation never partially mutates a measurement group.
//!
//! Rust compatibility target: Rust 1.97.1.

use std::fmt;

use super::errors::{
    IrError,
    IrLimitError,
    IrMeasurementError,
    IrResult,
};
use super::limits::{LimitsError, QuantumIrLimits};
use super::qubits::QubitId;

// -----------------------------------------------------------------------------
// Measurement basis
// -----------------------------------------------------------------------------

/// Basis in which a logical qubit is measured.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MeasurementBasis {
    /// Computational/Z basis.
    Z,

    /// X basis.
    X,

    /// Y basis.
    Y,
}

impl Default for MeasurementBasis {
    fn default() -> Self {
        Self::Z
    }
}

impl fmt::Display for MeasurementBasis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Z => "Z",
            Self::X => "X",
            Self::Y => "Y",
        };

        f.write_str(name)
    }
}

// -----------------------------------------------------------------------------
// Classical bit
// -----------------------------------------------------------------------------

/// Logical classical-bit identifier.
///
/// This is a logical IR identifier, not a hardware register address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassicalBitId(usize);

impl ClassicalBitId {
    /// Creates a classical-bit identifier.
    ///
    /// Register membership is established only when the identifier is
    /// validated against a circuit/classical register.
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the zero-based logical index.
    pub const fn index(self) -> usize {
        self.0
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "c{}", self.0)
    }
}

// -----------------------------------------------------------------------------
// Measurement mode
// -----------------------------------------------------------------------------

/// Logical measurement behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MeasurementMode {
    /// The measurement observes the logical state without declaring that the
    /// state is consumed by the IR operation.
    ///
    /// This is an abstract IR semantic. It does not require a backend to have
    /// a physically non-invasive detector.
    NonDestructive,

    /// The measurement consumes the logical state according to the abstract
    /// IR contract.
    ///
    /// The backend remains responsible for implementing this semantic on its
    /// actual hardware.
    Destructive,
}

impl Default for MeasurementMode {
    fn default() -> Self {
        Self::NonDestructive
    }
}

impl MeasurementMode {
    /// Returns true when this mode consumes the logical state.
    pub const fn is_destructive(self) -> bool {
        matches!(self, Self::Destructive)
    }

    /// Returns true when this mode preserves the logical state in the abstract
    /// IR model.
    pub const fn is_non_destructive(self) -> bool {
        matches!(self, Self::NonDestructive)
    }
}

// -----------------------------------------------------------------------------
// Measurement error
// -----------------------------------------------------------------------------

/// Errors produced while constructing or validating measurements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeasurementError {
    /// A measurement was required to contain a qubit but did not.
    MissingQubit,

    /// A measurement was required to contain a classical destination but did
    /// not.
    MissingClassicalTarget,

    /// A logical qubit is outside the declared circuit range.
    QubitOutOfRange {
        qubit: QubitId,
        num_qubits: usize,
    },

    /// A classical bit is outside the declared register range.
    ClassicalBitOutOfRange {
        bit: ClassicalBitId,
        num_classical_bits: usize,
    },

    /// The same logical qubit occurs more than once in one measurement group.
    DuplicateQubit {
        qubit: QubitId,
    },

    /// The same classical destination occurs more than once in one measurement
    /// group.
    DuplicateClassicalTarget {
        bit: ClassicalBitId,
    },

    /// A measurement configuration is invalid for a semantic reason that is
    /// not represented by a dedicated variant.
    InvalidMeasurement {
        message: String,
    },
}

impl fmt::Display for MeasurementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingQubit => {
                f.write_str("measurement requires a logical qubit")
            }

            Self::MissingClassicalTarget => {
                f.write_str("measurement requires a classical destination")
            }

            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => write!(
                f,
                "logical qubit {qubit} is outside range 0..{num_qubits}"
            ),

            Self::ClassicalBitOutOfRange {
                bit,
                num_classical_bits,
            } => write!(
                f,
                "classical bit {bit} is outside range 0..{num_classical_bits}"
            ),

            Self::DuplicateQubit { qubit } => {
                write!(
                    f,
                    "logical qubit {qubit} is already measured in this group"
                )
            }

            Self::DuplicateClassicalTarget { bit } => {
                write!(
                    f,
                    "classical destination {bit} is already used in this group"
                )
            }

            Self::InvalidMeasurement { message } => {
                write!(f, "invalid measurement: {message}")
            }
        }
    }
}

impl std::error::Error for MeasurementError {}

// -----------------------------------------------------------------------------
// Canonical error integration
// -----------------------------------------------------------------------------

impl From<MeasurementError> for IrError {
    fn from(error: MeasurementError) -> Self {
        match error {
            MeasurementError::MissingQubit => {
                IrMeasurementError::MissingQubit.into()
            }

            MeasurementError::MissingClassicalTarget => {
                IrMeasurementError::MissingClassicalTarget.into()
            }

            MeasurementError::QubitOutOfRange {
                qubit,
                num_qubits,
            } => IrMeasurementError::QubitOutOfRange {
                qubit: qubit.index(),
                num_qubits,
            }
            .into(),

            MeasurementError::ClassicalBitOutOfRange {
                bit,
                num_classical_bits,
            } => IrMeasurementError::ClassicalBitOutOfRange {
                bit: bit.index(),
                num_classical_bits,
            }
            .into(),

            MeasurementError::DuplicateQubit { qubit } => {
                IrMeasurementError::DuplicateQubit {
                    qubit: qubit.index(),
                }
                .into()
            }

            MeasurementError::DuplicateClassicalTarget { bit } => {
                IrMeasurementError::DuplicateClassicalTarget {
                    bit: bit.index(),
                }
                .into()
            }

            MeasurementError::InvalidMeasurement { .. } => {
                IrMeasurementError::InvalidConfiguration {
                    reason: "invalid measurement configuration",
                }
                .into()
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
        } => IrLimitError::new(resource, requested, maximum).into(),

        LimitsError::InvalidConfiguration { .. }
        | LimitsError::ArithmeticOverflow { .. }
        | LimitsError::ArithmeticMultiplicationOverflow { .. } => {
            IrLimitError::new(
                "measurement resource policy",
                usize::MAX,
                0,
            )
            .into()
        }
    }
}

// -----------------------------------------------------------------------------
// Measurement
// -----------------------------------------------------------------------------

/// A single logical quantum measurement operation.
///
/// The structure is intentionally small and immutable-by-default. Its fields
/// remain private so callers cannot create an internally inconsistent
/// representation through direct field mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Measurement {
    qubit: QubitId,
    classical_bit: ClassicalBitId,
    basis: MeasurementBasis,
    mode: MeasurementMode,
    reset_after: bool,
}

impl Measurement {
    /// Creates a computational-basis, non-destructive measurement.
    pub const fn new(
        qubit: QubitId,
        classical_bit: ClassicalBitId,
    ) -> Self {
        Self {
            qubit,
            classical_bit,
            basis: MeasurementBasis::Z,
            mode: MeasurementMode::NonDestructive,
            reset_after: false,
        }
    }

    /// Creates a non-destructive measurement in the requested basis.
    pub const fn in_basis(
        qubit: QubitId,
        classical_bit: ClassicalBitId,
        basis: MeasurementBasis,
    ) -> Self {
        Self {
            qubit,
            classical_bit,
            basis,
            mode: MeasurementMode::NonDestructive,
            reset_after: false,
        }
    }

    /// Returns the measured logical qubit.
    pub const fn qubit(&self) -> QubitId {
        self.qubit
    }

    /// Returns the logical classical destination.
    pub const fn classical_bit(&self) -> ClassicalBitId {
        self.classical_bit
    }

    /// Returns the measurement basis.
    pub const fn basis(&self) -> MeasurementBasis {
        self.basis
    }

    /// Returns the measurement mode.
    pub const fn mode(&self) -> MeasurementMode {
        self.mode
    }

    /// Returns whether reset intent follows this measurement.
    ///
    /// `true` means the logical program explicitly requests reset semantics
    /// after the measurement. The IR does not imply a particular physical reset
    /// mechanism.
    pub const fn reset_after(&self) -> bool {
        self.reset_after
    }

    /// Returns true when the measurement is destructive.
    pub const fn is_destructive(&self) -> bool {
        self.mode.is_destructive()
    }

    /// Returns true when reset-after-measurement was explicitly requested.
    pub const fn requests_reset_after(&self) -> bool {
        self.reset_after
    }

    /// Changes the measurement basis.
    pub fn set_basis(&mut self, basis: MeasurementBasis) {
        self.basis = basis;
    }

    /// Changes the measurement mode.
    pub fn set_mode(&mut self, mode: MeasurementMode) {
        self.mode = mode;
    }

    /// Sets or clears explicit reset-after-measurement intent.
    pub fn set_reset_after(&mut self, reset: bool) {
        self.reset_after = reset;
    }

    /// Returns a destructive version of this measurement.
    pub fn destructive(mut self) -> Self {
        self.mode = MeasurementMode::Destructive;
        self
    }

    /// Returns a version with explicit reset-after-measurement intent.
    pub fn followed_by_reset(mut self) -> Self {
        self.reset_after = true;
        self
    }

    /// Returns a version with reset-after-measurement intent cleared.
    pub fn without_reset(mut self) -> Self {
        self.reset_after = false;
        self
    }

    /// Validates identifier membership against a logical quantum/classical
    /// namespace.
    pub fn validate(
        &self,
        num_qubits: usize,
        num_classical_bits: usize,
    ) -> Result<(), MeasurementError> {
        if self.qubit.index() >= num_qubits {
            return Err(MeasurementError::QubitOutOfRange {
                qubit: self.qubit,
                num_qubits,
            });
        }

        if self.classical_bit.index() >= num_classical_bits {
            return Err(MeasurementError::ClassicalBitOutOfRange {
                bit: self.classical_bit,
                num_classical_bits,
            });
        }

        Ok(())
    }

    /// Validates this measurement using the canonical IR error vocabulary and
    /// resource policy.
    pub fn validate_with_limits(
        &self,
        num_qubits: usize,
        num_classical_bits: usize,
        limits: &QuantumIrLimits,
    ) -> IrResult<()> {
        self.validate(num_qubits, num_classical_bits)?;

        limits
            .check_measurements(1)
            .map_err(limit_error)?;

        limits
            .check_operands(1)
            .map_err(limit_error)?;

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Measurement group
// -----------------------------------------------------------------------------

/// Deterministically ordered collection of measurements belonging to one
/// logical measurement boundary.
///
/// A group is not a hardware readout batch. It is an IR-level grouping that
/// preserves semantic ordering and uniqueness constraints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasurementGroup {
    measurements: Vec<Measurement>,
}

impl MeasurementGroup {
    /// Creates an empty group.
    pub fn new() -> Self {
        Self {
            measurements: Vec::new(),
        }
    }

    /// Creates a group from existing measurements.
    ///
    /// Insertion is validated before each mutation, so a failed insertion does
    /// not append the invalid measurement.
    pub fn from_measurements(
        measurements: Vec<Measurement>,
    ) -> Result<Self, MeasurementError> {
        let mut group = Self::new();

        for measurement in measurements {
            group.push(measurement)?;
        }

        Ok(group)
    }

    /// Creates a group from measurements while enforcing the supplied IR
    /// resource limits.
    pub fn from_measurements_with_limits(
        measurements: Vec<Measurement>,
        limits: &QuantumIrLimits,
    ) -> IrResult<Self> {
        limits
            .check_measurements(measurements.len())
            .map_err(limit_error)?;

        let mut group = Self::new();

        for measurement in measurements {
            group.push_with_limits(measurement, limits)?;
        }

        Ok(group)
    }

    /// Returns the number of measurements in the group.
    pub fn len(&self) -> usize {
        self.measurements.len()
    }

    /// Returns true when the group contains no measurements.
    pub fn is_empty(&self) -> bool {
        self.measurements.is_empty()
    }

    /// Returns an immutable, deterministic measurement slice.
    pub fn measurements(&self) -> &[Measurement] {
        &self.measurements
    }

    /// Returns the measurement at a stable group position.
    pub fn get(&self, index: usize) -> Option<&Measurement> {
        self.measurements.get(index)
    }

    /// Adds a measurement after enforcing group uniqueness invariants.
    ///
    /// This method does not silently reorder or replace an existing entry.
    pub fn push(
        &mut self,
        measurement: Measurement,
    ) -> Result<(), MeasurementError> {
        self.ensure_unique(&measurement)?;
        self.measurements.push(measurement);
        Ok(())
    }

    /// Adds a measurement while enforcing the canonical IR measurement limit.
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
                IrLimitError::new(
                    "measurements",
                    usize::MAX,
                    limits.max_measurements(),
                )
            })?;

        limits
            .check_measurements(next_count)
            .map_err(limit_error)?;

        self.measurements.push(measurement);
        Ok(())
    }

    /// Returns the measurement for a logical qubit.
    pub fn for_qubit(
        &self,
        qubit: QubitId,
    ) -> Option<&Measurement> {
        self.measurements
            .iter()
            .find(|measurement| measurement.qubit() == qubit)
    }

    /// Returns the measurement targeting a classical bit.
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

    /// Validates all measurements and group uniqueness constraints.
    pub fn validate(
        &self,
        num_qubits: usize,
        num_classical_bits: usize,
    ) -> Result<(), MeasurementError> {
        for (index, measurement) in self.measurements.iter().enumerate() {
            measurement.validate(
                num_qubits,
                num_classical_bits,
            )?;

            for previous in self.measurements[..index].iter() {
                if previous.qubit() == measurement.qubit() {
                    return Err(
                        MeasurementError::DuplicateQubit {
                            qubit: measurement.qubit(),
                        },
                    );
                }

                if previous.classical_bit()
                    == measurement.classical_bit()
                {
                    return Err(
                        MeasurementError::DuplicateClassicalTarget {
                            bit: measurement.classical_bit(),
                        },
                    );
                }
            }
        }

        Ok(())
    }

    /// Validates the group against both namespace and resource limits.
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
        if self
            .measurements
            .iter()
            .any(|existing| {
                existing.qubit() == measurement.qubit()
            })
        {
            return Err(
                MeasurementError::DuplicateQubit {
                    qubit: measurement.qubit(),
                },
            );
        }

        if self
            .measurements
            .iter()
            .any(|existing| {
                existing.classical_bit()
                    == measurement.classical_bit()
            })
        {
            return Err(
                MeasurementError::DuplicateClassicalTarget {
                    bit: measurement.classical_bit(),
                },
            );
        }

        Ok(())
    }
}

impl Default for MeasurementGroup {
    fn default() -> Self {
        Self::new()
    }
}

// -----------------------------------------------------------------------------
// Classical register
// -----------------------------------------------------------------------------

/// Logical classical-bit namespace used by measurements.
///
/// The register stores only the number of logical bits. It does not represent
/// physical memory or hardware readout storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClassicalRegister {
    bits: usize,
}

impl ClassicalRegister {
    /// Creates a classical register without performing an allocation.
    pub const fn new(bits: usize) -> Self {
        Self { bits }
    }

    /// Creates a classical register under an explicit IR limit.
    pub fn try_new(
        bits: usize,
        limits: &QuantumIrLimits,
    ) -> IrResult<Self> {
        limits
            .check_classical_bits(bits)
            .map_err(limit_error)?;

        Ok(Self { bits })
    }

    /// Returns the number of classical bits.
    pub const fn len(&self) -> usize {
        self.bits
    }

    /// Returns true when the register is empty.
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

    /// Validates a classical identifier using the canonical IR result type.
    pub fn validate_ir(
        &self,
        bit: ClassicalBitId,
    ) -> IrResult<()> {
        self.validate(bit)?;
        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

/// Creates a standard Z-basis measurement.
pub const fn measure(
    qubit: QubitId,
    classical_bit: ClassicalBitId,
) -> Measurement {
    Measurement::new(qubit, classical_bit)
}

/// Creates an X-basis measurement.
pub const fn measure_x(
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
pub const fn measure_y(
    qubit: QubitId,
    classical_bit: ClassicalBitId,
) -> Measurement {
    Measurement::in_basis(
        qubit,
        classical_bit,
        MeasurementBasis::Y,
    )
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> QuantumIrLimits {
        QuantumIrLimits::production()
    }

    #[test]
    fn classical_bit_identity_is_deterministic() {
        let bit = ClassicalBitId::new(3);

        assert_eq!(bit.index(), 3);
        assert_eq!(bit.to_string(), "c3");
        assert_eq!(usize::from(bit), 3);
    }

    #[test]
    fn basis_display_is_stable() {
        assert_eq!(MeasurementBasis::Z.to_string(), "Z");
        assert_eq!(MeasurementBasis::X.to_string(), "X");
        assert_eq!(MeasurementBasis::Y.to_string(), "Y");
    }

    #[test]
    fn default_measurement_is_z_and_non_destructive() {
        let measurement = Measurement::new(
            QubitId::new(0),
            ClassicalBitId::new(0),
        );

        assert_eq!(
            measurement.basis(),
            MeasurementBasis::Z
        );

        assert_eq!(
            measurement.mode(),
            MeasurementMode::NonDestructive
        );

        assert!(!measurement.reset_after());
    }

    #[test]
    fn basis_helpers_are_correct() {
        assert_eq!(
            measure_x(
                QubitId::new(1),
                ClassicalBitId::new(2)
            )
            .basis(),
            MeasurementBasis::X
        );

        assert_eq!(
            measure_y(
                QubitId::new(1),
                ClassicalBitId::new(2)
            )
            .basis(),
            MeasurementBasis::Y
        );
    }

    #[test]
    fn destructive_and_reset_semantics_are_explicit() {
        let measurement = Measurement::new(
            QubitId::new(0),
            ClassicalBitId::new(0),
        )
        .destructive()
        .followed_by_reset();

        assert!(measurement.is_destructive());
        assert!(measurement.requests_reset_after());
    }

    #[test]
    fn setters_only_change_semantics() {
        let mut measurement = Measurement::new(
            QubitId::new(0),
            ClassicalBitId::new(0),
        );

        measurement.set_basis(MeasurementBasis::Y);
        measurement.set_mode(MeasurementMode::Destructive);
        measurement.set_reset_after(true);

        assert_eq!(
            measurement.basis(),
            MeasurementBasis::Y
        );

        assert_eq!(
            measurement.mode(),
            MeasurementMode::Destructive
        );

        assert!(measurement.reset_after());
    }

    #[test]
    fn valid_measurement_passes_namespace_validation() {
        let measurement = Measurement::new(
            QubitId::new(1),
            ClassicalBitId::new(2),
        );

        assert!(
            measurement.validate(4, 4).is_ok()
        );
    }

    #[test]
    fn invalid_qubit_is_rejected() {
        let measurement = Measurement::new(
            QubitId::new(4),
            ClassicalBitId::new(0),
        );

        assert_eq!(
            measurement.validate(4, 4),
            Err(
                MeasurementError::QubitOutOfRange {
                    qubit: QubitId::new(4),
                    num_qubits: 4,
                }
            )
        );
    }

    #[test]
    fn invalid_classical_bit_is_rejected() {
        let measurement = Measurement::new(
            QubitId::new(0),
            ClassicalBitId::new(4),
        );

        assert_eq!(
            measurement.validate(4, 4),
            Err(
                MeasurementError::ClassicalBitOutOfRange {
                    bit: ClassicalBitId::new(4),
                    num_classical_bits: 4,
                }
            )
        );
    }

    #[test]
    fn group_preserves_insertion_order() {
        let mut group = MeasurementGroup::new();

        group
            .push(Measurement::new(
                QubitId::new(2),
                ClassicalBitId::new(2),
            ))
            .unwrap();

        group
            .push(Measurement::new(
                QubitId::new(0),
                ClassicalBitId::new(0),
            ))
            .unwrap();

        assert_eq!(
            group.get(0).unwrap().qubit(),
            QubitId::new(2)
        );

        assert_eq!(
            group.get(1).unwrap().qubit(),
            QubitId::new(0)
        );
    }

    #[test]
    fn group_rejects_duplicate_qubit() {
        let mut group = MeasurementGroup::new();

        group
            .push(Measurement::new(
                QubitId::new(0),
                ClassicalBitId::new(0),
            ))
            .unwrap();

        let result = group.push(
            Measurement::new(
                QubitId::new(0),
                ClassicalBitId::new(1),
            ),
        );

        assert_eq!(
            result,
            Err(
                MeasurementError::DuplicateQubit {
                    qubit: QubitId::new(0),
                }
            )
        );

        assert_eq!(group.len(), 1);
    }

    #[test]
    fn group_rejects_duplicate_classical_destination() {
        let mut group = MeasurementGroup::new();

        group
            .push(Measurement::new(
                QubitId::new(0),
                ClassicalBitId::new(0),
            ))
            .unwrap();

        let result = group.push(
            Measurement::new(
                QubitId::new(1),
                ClassicalBitId::new(0),
            ),
        );

        assert_eq!(
            result,
            Err(
                MeasurementError::DuplicateClassicalTarget {
                    bit: ClassicalBitId::new(0),
                }
            )
        );

        assert_eq!(group.len(), 1);
    }

    #[test]
    fn group_validation_checks_namespace() {
        let group =
            MeasurementGroup::from_measurements(vec![
                Measurement::new(
                    QubitId::new(0),
                    ClassicalBitId::new(0),
                ),
                Measurement::new(
                    QubitId::new(1),
                    ClassicalBitId::new(1),
                ),
            ])
            .unwrap();

        assert!(group.validate(2, 2).is_ok());
        assert!(group.validate(1, 2).is_err());
    }

    #[test]
    fn limit_aware_group_rejects_excess_measurements_atomically() {
        let limits =
            QuantumIrLimits::production()
                .with_max_measurements(1);

        let mut group = MeasurementGroup::new();

        group
            .push_with_limits(
                Measurement::new(
                    QubitId::new(0),
                    ClassicalBitId::new(0),
                ),
                &limits,
            )
            .unwrap();

        let result = group.push_with_limits(
            Measurement::new(
                QubitId::new(1),
                ClassicalBitId::new(1),
            ),
            &limits,
        );

        assert!(result.is_err());
        assert_eq!(group.len(), 1);
    }

    #[test]
    fn canonical_error_conversion_preserves_category() {
        let error: IrError =
            MeasurementError::DuplicateQubit {
                qubit: QubitId::new(2),
            }
            .into();

        assert_eq!(
            error.kind(),
            super::super::errors::IrErrorKind::Measurement
        );
    }

    #[test]
    fn canonical_limit_validation_is_available() {
        let limits = limits();

        let measurement = Measurement::new(
            QubitId::new(0),
            ClassicalBitId::new(0),
        );

        assert!(
            measurement
                .validate_with_limits(
                    1,
                    1,
                    &limits
                )
                .is_ok()
        );
    }

    #[test]
    fn classical_register_does_not_allocate() {
        let register = ClassicalRegister::new(8);

        assert_eq!(register.len(), 8);
        assert!(!register.is_empty());

        assert!(
            register
                .validate(
                    ClassicalBitId::new(7)
                )
                .is_ok()
        );

        assert!(
            register
                .validate(
                    ClassicalBitId::new(8)
                )
                .is_err()
        );
    }

    #[test]
    fn classical_register_limit_is_enforced() {
        let limits =
            QuantumIrLimits::production()
                .with_max_classical_bits(2);

        assert!(
            ClassicalRegister::try_new(
                2,
                &limits
            )
            .is_ok()
        );

        assert!(
            ClassicalRegister::try_new(
                3,
                &limits
            )
            .is_err()
        );
    }
}