//! Zamani Quantum IR — Measurement
//!
//! Measurement is represented separately from generic gate semantics so that
//! compilation and optimization passes can reason about:
//!
//! - measured logical qubits;
//! - classical destinations;
//! - measurement basis;
//! - destructive vs non-destructive measurement;
//! - measurement ordering;
//! - reset-after-measurement;
//! - grouped measurements.
//!
//! This module is hardware-independent. Hardware-specific readout channels,
//! detector configuration, pulse schedules, and calibration belong to later
//! compilation stages.

use std::fmt;

use super::qubits::QubitId;

// -----------------------------------------------------------------------------
// Measurement basis
// -----------------------------------------------------------------------------

/// Basis in which a qubit is measured.
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

        write!(f, "{name}")
    }
}

// -----------------------------------------------------------------------------
// Classical bit
// -----------------------------------------------------------------------------

/// Logical classical-bit identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassicalBitId(usize);

impl ClassicalBitId {
    /// Creates a classical-bit identifier.
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the underlying index.
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

/// Measurement behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MeasurementMode {
    /// Measure while retaining the logical state in the abstract IR.
    NonDestructive,

    /// Measure and consume the quantum state.
    Destructive,
}

impl Default for MeasurementMode {
    fn default() -> Self {
        Self::NonDestructive
    }
}

// -----------------------------------------------------------------------------
// Measurement error
// -----------------------------------------------------------------------------

/// Errors produced while constructing or validating measurements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeasurementError {
    /// No quantum qubit was supplied.
    MissingQubit,

    /// No classical destination was supplied.
    MissingClassicalTarget,

    /// A qubit is outside the declared circuit range.
    QubitOutOfRange {
        qubit: QubitId,
        num_qubits: usize,
    },

    /// A classical bit is outside the declared register range.
    ClassicalBitOutOfRange {
        bit: ClassicalBitId,
        num_classical_bits: usize,
    },

    /// A measurement contains an invalid configuration.
    InvalidMeasurement {
        message: String,
    },
}

impl fmt::Display for MeasurementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingQubit => {
                write!(f, "measurement requires a qubit")
            }

            Self::MissingClassicalTarget => {
                write!(
                    f,
                    "measurement requires a classical destination"
                )
            }

            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => {
                write!(
                    f,
                    "qubit {qubit} is outside range 0..{num_qubits}"
                )
            }

            Self::ClassicalBitOutOfRange {
                bit,
                num_classical_bits,
            } => {
                write!(
                    f,
                    "classical bit {bit} is outside range 0..{num_classical_bits}"
                )
            }

            Self::InvalidMeasurement { message } => {
                write!(
                    f,
                    "invalid measurement: {message}"
                )
            }
        }
    }
}

impl std::error::Error for MeasurementError {}

// -----------------------------------------------------------------------------
// Measurement
// -----------------------------------------------------------------------------

/// A single quantum measurement operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Measurement {
    qubit: QubitId,
    classical_bit: ClassicalBitId,
    basis: MeasurementBasis,
    mode: MeasurementMode,
    reset_after: bool,
}

impl Measurement {
    /// Creates a computational-basis measurement.
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

    /// Creates a measurement in a specific basis.
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

    /// Returns the classical destination.
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

    /// Returns whether the qubit is reset after measurement.
    pub const fn reset_after(&self) -> bool {
        self.reset_after
    }

    /// Changes the measurement basis.
    pub fn set_basis(
        &mut self,
        basis: MeasurementBasis,
    ) {
        self.basis = basis;
    }

    /// Changes the measurement mode.
    pub fn set_mode(
        &mut self,
        mode: MeasurementMode,
    ) {
        self.mode = mode;
    }

    /// Requests reset after measurement.
    pub fn set_reset_after(
        &mut self,
        reset: bool,
    ) {
        self.reset_after = reset;
    }

    /// Converts the measurement into a destructive measurement.
    pub fn destructive(mut self) -> Self {
        self.mode =
            MeasurementMode::Destructive;
        self
    }

    /// Requests reset after measurement.
    pub fn followed_by_reset(mut self) -> Self {
        self.reset_after = true;
        self
    }

    /// Validates the measurement.
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

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Measurement group
// -----------------------------------------------------------------------------

/// A collection of measurements that conceptually belongs to one measurement
/// boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasurementGroup {
    measurements: Vec<Measurement>,
}

impl MeasurementGroup {
    /// Creates an empty measurement group.
    pub fn new() -> Self {
        Self {
            measurements: Vec::new(),
        }
    }

    /// Creates a group from existing measurements.
    pub fn from_measurements(
        measurements: Vec<Measurement>,
    ) -> Result<Self, MeasurementError> {
        let mut group = Self::new();

        for measurement in measurements {
            group.push(measurement)?;
        }

        Ok(group)
    }

    /// Number of measurements in the group.
    pub fn len(&self) -> usize {
        self.measurements.len()
    }

    /// Returns true when the group contains no measurements.
    pub fn is_empty(&self) -> bool {
        self.measurements.is_empty()
    }

    /// Returns all measurements.
    pub fn measurements(&self) -> &[Measurement] {
        &self.measurements
    }

    /// Adds a measurement to the group.
    ///
    /// A quantum qubit and classical destination may each occur only once in
    /// a group. This prevents ambiguous simultaneous destinations.
    pub fn push(
        &mut self,
        measurement: Measurement,
    ) -> Result<(), MeasurementError> {
        if self
            .measurements
            .iter()
            .any(|existing| {
                existing.qubit()
                    == measurement.qubit()
            })
        {
            return Err(
                MeasurementError::InvalidMeasurement {
                    message: format!(
                        "qubit {} is already measured in this group",
                        measurement.qubit()
                    ),
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
                MeasurementError::InvalidMeasurement {
                    message: format!(
                        "classical bit {} is already a destination in this group",
                        measurement.classical_bit()
                    ),
                },
            );
        }

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
            .find(|measurement| {
                measurement.qubit() == qubit
            })
    }

    /// Returns the measurement targeting a classical bit.
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

    /// Validates every measurement in the group.
    pub fn validate(
        &self,
        num_qubits: usize,
        num_classical_bits: usize,
    ) -> Result<(), MeasurementError> {
        for measurement in &self.measurements {
            measurement.validate(
                num_qubits,
                num_classical_bits,
            )?;
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

/// Logical classical register used by quantum measurements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassicalRegister {
    bits: usize,
}

impl ClassicalRegister {
    /// Creates a classical register.
    pub const fn new(bits: usize) -> Self {
        Self { bits }
    }

    /// Returns the number of classical bits.
    pub const fn len(&self) -> usize {
        self.bits
    }

    /// Returns true if the register contains no bits.
    pub const fn is_empty(&self) -> bool {
        self.bits == 0
    }

    /// Validates a classical bit.
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

    #[test]
    fn creates_classical_bit() {
        let bit =
            ClassicalBitId::new(3);

        assert_eq!(bit.index(), 3);
        assert_eq!(bit.to_string(), "c3");
    }

    #[test]
    fn creates_z_measurement() {
        let measurement =
            Measurement::new(
                QubitId::new(0),
                ClassicalBitId::new(0),
            );

        assert_eq!(
            measurement.qubit(),
            QubitId::new(0)
        );

        assert_eq!(
            measurement.classical_bit(),
            ClassicalBitId::new(0)
        );

        assert_eq!(
            measurement.basis(),
            MeasurementBasis::Z
        );
    }

    #[test]
    fn creates_x_measurement() {
        let measurement =
            measure_x(
                QubitId::new(1),
                ClassicalBitId::new(2),
            );

        assert_eq!(
            measurement.basis(),
            MeasurementBasis::X
        );
    }

    #[test]
    fn creates_y_measurement() {
        let measurement =
            measure_y(
                QubitId::new(1),
                ClassicalBitId::new(2),
            );

        assert_eq!(
            measurement.basis(),
            MeasurementBasis::Y
        );
    }

    #[test]
    fn destructive_measurement() {
        let measurement =
            Measurement::new(
                QubitId::new(0),
                ClassicalBitId::new(0),
            )
            .destructive();

        assert_eq!(
            measurement.mode(),
            MeasurementMode::Destructive
        );
    }

    #[test]
    fn reset_after_measurement() {
        let measurement =
            Measurement::new(
                QubitId::new(0),
                ClassicalBitId::new(0),
            )
            .followed_by_reset();

        assert!(measurement.reset_after());
    }

    #[test]
    fn valid_measurement_passes_validation() {
        let measurement =
            Measurement::new(
                QubitId::new(1),
                ClassicalBitId::new(2),
            );

        assert!(
            measurement.validate(4, 4).is_ok()
        );
    }

    #[test]
    fn invalid_qubit_is_rejected() {
        let measurement =
            Measurement::new(
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
        let measurement =
            Measurement::new(
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
    fn measurement_group_accepts_unique_measurements() {
        let mut group =
            MeasurementGroup::new();

        group
            .push(measure(
                QubitId::new(0),
                ClassicalBitId::new(0),
            ))
            .unwrap();

        group
            .push(measure(
                QubitId::new(1),
                ClassicalBitId::new(1),
            ))
            .unwrap();

        assert_eq!(group.len(), 2);
    }

    #[test]
    fn measurement_group_rejects_duplicate_qubit() {
        let mut group =
            MeasurementGroup::new();

        group
            .push(measure(
                QubitId::new(0),
                ClassicalBitId::new(0),
            ))
            .unwrap();

        let result =
            group.push(measure(
                QubitId::new(0),
                ClassicalBitId::new(1),
            ));

        assert!(matches!(
            result,
            Err(
                MeasurementError::InvalidMeasurement {
                    ..
                }
            )
        ));
    }

    #[test]
    fn measurement_group_rejects_duplicate_classical_bit() {
        let mut group =
            MeasurementGroup::new();

        group
            .push(measure(
                QubitId::new(0),
                ClassicalBitId::new(0),
            ))
            .unwrap();

        let result =
            group.push(measure(
                QubitId::new(1),
                ClassicalBitId::new(0),
            ));

        assert!(matches!(
            result,
            Err(
                MeasurementError::InvalidMeasurement {
                    ..
                }
            )
        ));
    }

    #[test]
    fn finds_measurement_by_qubit() {
        let mut group =
            MeasurementGroup::new();

        group
            .push(measure(
                QubitId::new(2),
                ClassicalBitId::new(5),
            ))
            .unwrap();

        assert!(
            group
                .for_qubit(QubitId::new(2))
                .is_some()
        );
    }

    #[test]
    fn finds_measurement_by_classical_bit() {
        let mut group =
            MeasurementGroup::new();

        group
            .push(measure(
                QubitId::new(2),
                ClassicalBitId::new(5),
            ))
            .unwrap();

        assert!(
            group
                .for_classical_bit(
                    ClassicalBitId::new(5)
                )
                .is_some()
        );
    }

    #[test]
    fn classical_register_validates_bits() {
        let register =
            ClassicalRegister::new(4);

        assert!(
            register
                .validate(ClassicalBitId::new(3))
                .is_ok()
        );

        assert!(
            register
                .validate(ClassicalBitId::new(4))
                .is_err()
        );
    }
}