//! Zamani Quantum IR — Qubits
//!
//! Logical-qubit representation and allocation.
//!
//! This module deliberately separates logical qubits from physical hardware
//! qubits. Mapping logical qubits to physical qubits belongs to the routing
//! and backend-lowering stages.

use std::fmt;

// -----------------------------------------------------------------------------
// Qubit identifier
// -----------------------------------------------------------------------------

/// Stable logical qubit identifier.
///
/// A `QubitId` identifies a logical qubit inside a quantum program. It does
/// not identify a physical hardware qubit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QubitId(usize);

impl QubitId {
    /// Creates a logical qubit identifier.
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the underlying index.
    pub const fn index(self) -> usize {
        self.0
    }
}

impl From<usize> for QubitId {
    fn from(index: usize) -> Self {
        Self::new(index)
    }
}

impl From<QubitId> for usize {
    fn from(qubit: QubitId) -> usize {
        qubit.index()
    }
}

impl fmt::Display for QubitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "q{}", self.0)
    }
}

// -----------------------------------------------------------------------------
// Physical qubit identifier
// -----------------------------------------------------------------------------

/// Physical qubit identifier.
///
/// This is intentionally distinct from `QubitId` so that a compiler cannot
/// accidentally confuse logical and physical qubits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PhysicalQubitId(usize);

impl PhysicalQubitId {
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    pub const fn index(self) -> usize {
        self.0
    }
}

impl From<usize> for PhysicalQubitId {
    fn from(index: usize) -> Self {
        Self::new(index)
    }
}

impl From<PhysicalQubitId> for usize {
    fn from(qubit: PhysicalQubitId) -> usize {
        qubit.index()
    }
}

impl fmt::Display for PhysicalQubitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "p{}", self.0)
    }
}

// -----------------------------------------------------------------------------
// Qubit state
// -----------------------------------------------------------------------------

/// Compile-time logical state of a qubit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QubitState {
    /// Normal usable logical qubit.
    Available,

    /// Qubit has been explicitly reset.
    Reset,

    /// Qubit has been measured.
    Measured,

    /// Qubit has been disabled/reserved.
    Disabled,
}

impl Default for QubitState {
    fn default() -> Self {
        Self::Available
    }
}

// -----------------------------------------------------------------------------
// Logical qubit
// -----------------------------------------------------------------------------

/// A logical qubit tracked by the Zamani IR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Qubit {
    id: QubitId,
    state: QubitState,
}

impl Qubit {
    /// Creates a new logical qubit.
    pub const fn new(id: QubitId) -> Self {
        Self {
            id,
            state: QubitState::Available,
        }
    }

    /// Returns the logical identifier.
    pub const fn id(&self) -> QubitId {
        self.id
    }

    /// Returns the current logical state.
    pub const fn state(&self) -> QubitState {
        self.state
    }

    /// Marks the qubit as reset.
    pub fn reset(&mut self) {
        self.state = QubitState::Reset;
    }

    /// Marks the qubit as measured.
    pub fn mark_measured(&mut self) {
        self.state = QubitState::Measured;
    }

    /// Marks the qubit as available.
    pub fn mark_available(&mut self) {
        self.state = QubitState::Available;
    }

    /// Disables the qubit.
    pub fn disable(&mut self) {
        self.state = QubitState::Disabled;
    }

    pub fn is_available(&self) -> bool {
        self.state == QubitState::Available
    }

    pub fn is_measured(&self) -> bool {
        self.state == QubitState::Measured
    }

    pub fn is_disabled(&self) -> bool {
        self.state == QubitState::Disabled
    }
}

// -----------------------------------------------------------------------------
// Qubit allocation errors
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QubitError {
    InvalidCount {
        count: usize,
    },

    OutOfRange {
        qubit: QubitId,
        num_qubits: usize,
    },

    AlreadyAllocated {
        qubit: QubitId,
    },

    NotAllocated {
        qubit: QubitId,
    },

    Disabled {
        qubit: QubitId,
    },

    NoAvailableQubit,

    DuplicateQubit {
        qubit: QubitId,
    },
}

impl fmt::Display for QubitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCount { count } => {
                write!(
                    f,
                    "invalid qubit count: {count}"
                )
            }

            Self::OutOfRange {
                qubit,
                num_qubits,
            } => {
                write!(
                    f,
                    "qubit {qubit} is outside range 0..{num_qubits}"
                )
            }

            Self::AlreadyAllocated { qubit } => {
                write!(
                    f,
                    "qubit {qubit} is already allocated"
                )
            }

            Self::NotAllocated { qubit } => {
                write!(
                    f,
                    "qubit {qubit} is not allocated"
                )
            }

            Self::Disabled { qubit } => {
                write!(
                    f,
                    "qubit {qubit} is disabled"
                )
            }

            Self::NoAvailableQubit => {
                write!(f, "no available logical qubit")
            }

            Self::DuplicateQubit { qubit } => {
                write!(
                    f,
                    "qubit {qubit} appears more than once"
                )
            }
        }
    }
}

impl std::error::Error for QubitError {}

// -----------------------------------------------------------------------------
// Qubit register
// -----------------------------------------------------------------------------

/// Logical qubit register.
///
/// The register owns the logical qubit namespace for a circuit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QubitRegister {
    qubits: Vec<Qubit>,
}

impl QubitRegister {
    /// Creates `count` logical qubits.
    pub fn new(count: usize) -> Self {
        let qubits = (0..count)
            .map(|index| Qubit::new(QubitId::new(index)))
            .collect();

        Self { qubits }
    }

    /// Number of logical qubits.
    pub fn len(&self) -> usize {
        self.qubits.len()
    }

    /// Returns true if no qubits exist.
    pub fn is_empty(&self) -> bool {
        self.qubits.is_empty()
    }

    /// Returns a qubit by identifier.
    pub fn get(
        &self,
        id: QubitId,
    ) -> Result<&Qubit, QubitError> {
        self.qubits
            .get(id.index())
            .ok_or(QubitError::OutOfRange {
                qubit: id,
                num_qubits: self.len(),
            })
    }

    /// Returns mutable access to a qubit.
    pub fn get_mut(
        &mut self,
        id: QubitId,
    ) -> Result<&mut Qubit, QubitError> {
        let len = self.len();

        self.qubits
            .get_mut(id.index())
            .ok_or(QubitError::OutOfRange {
                qubit: id,
                num_qubits: len,
            })
    }

    /// Returns all logical qubits.
    pub fn as_slice(&self) -> &[Qubit] {
        &self.qubits
    }

    /// Returns all logical qubits mutably.
    pub fn as_mut_slice(&mut self) -> &mut [Qubit] {
        &mut self.qubits
    }

    /// Returns the first available qubit.
    pub fn first_available(
        &self,
    ) -> Option<QubitId> {
        self.qubits
            .iter()
            .find(|q| q.is_available())
            .map(Qubit::id)
    }

    /// Marks a qubit as measured.
    pub fn mark_measured(
        &mut self,
        id: QubitId,
    ) -> Result<(), QubitError> {
        let qubit = self.get_mut(id)?;

        if qubit.is_disabled() {
            return Err(QubitError::Disabled {
                qubit: id,
            });
        }

        qubit.mark_measured();

        Ok(())
    }

    /// Resets a qubit.
    pub fn reset(
        &mut self,
        id: QubitId,
    ) -> Result<(), QubitError> {
        let qubit = self.get_mut(id)?;

        if qubit.is_disabled() {
            return Err(QubitError::Disabled {
                qubit: id,
            });
        }

        qubit.reset();

        Ok(())
    }

    /// Disables a logical qubit.
    pub fn disable(
        &mut self,
        id: QubitId,
    ) -> Result<(), QubitError> {
        let qubit = self.get_mut(id)?;
        qubit.disable();
        Ok(())
    }

    /// Enables a disabled qubit.
    pub fn enable(
        &mut self,
        id: QubitId,
    ) -> Result<(), QubitError> {
        let qubit = self.get_mut(id)?;

        qubit.mark_available();

        Ok(())
    }
}

impl IntoIterator for QubitRegister {
    type Item = Qubit;
    type IntoIter =
        std::vec::IntoIter<Qubit>;

    fn into_iter(self) -> Self::IntoIter {
        self.qubits.into_iter()
    }
}

impl<'a> IntoIterator for &'a QubitRegister {
    type Item = &'a Qubit;
    type IntoIter =
        std::slice::Iter<'a, Qubit>;

    fn into_iter(self) -> Self::IntoIter {
        self.qubits.iter()
    }
}

// -----------------------------------------------------------------------------
// Qubit utilities
// -----------------------------------------------------------------------------

/// Validates that a collection of qubits contains no duplicates.
pub fn validate_unique_qubits(
    qubits: &[QubitId],
) -> Result<(), QubitError> {
    for (index, qubit) in qubits.iter().enumerate() {
        if qubits[index + 1..].contains(qubit) {
            return Err(
                QubitError::DuplicateQubit {
                    qubit: *qubit,
                },
            );
        }
    }

    Ok(())
}

/// Validates a collection against a register size.
pub fn validate_qubits(
    qubits: &[QubitId],
    num_qubits: usize,
) -> Result<(), QubitError> {
    validate_unique_qubits(qubits)?;

    for qubit in qubits {
        if qubit.index() >= num_qubits {
            return Err(
                QubitError::OutOfRange {
                    qubit: *qubit,
                    num_qubits,
                },
            );
        }
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_qubit_id() {
        let q = QubitId::new(7);

        assert_eq!(q.index(), 7);
        assert_eq!(q.to_string(), "q7");
    }

    #[test]
    fn physical_and_logical_ids_are_distinct() {
        let logical =
            QubitId::new(3);

        let physical =
            PhysicalQubitId::new(3);

        assert_eq!(
            logical.index(),
            physical.index()
        );
    }

    #[test]
    fn creates_register() {
        let register =
            QubitRegister::new(8);

        assert_eq!(register.len(), 8);
        assert!(!register.is_empty());
    }

    #[test]
    fn first_available_qubit() {
        let register =
            QubitRegister::new(4);

        assert_eq!(
            register.first_available(),
            Some(QubitId::new(0))
        );
    }

    #[test]
    fn measurement_updates_state() {
        let mut register =
            QubitRegister::new(2);

        register
            .mark_measured(QubitId::new(1))
            .expect("measurement should succeed");

        assert!(
            register
                .get(QubitId::new(1))
                .unwrap()
                .is_measured()
        );
    }

    #[test]
    fn reset_updates_state() {
        let mut register =
            QubitRegister::new(2);

        register
            .reset(QubitId::new(0))
            .expect("reset should succeed");

        assert_eq!(
            register
                .get(QubitId::new(0))
                .unwrap()
                .state(),
            QubitState::Reset
        );
    }

    #[test]
    fn disabled_qubit_cannot_be_measured() {
        let mut register =
            QubitRegister::new(2);

        register
            .disable(QubitId::new(0))
            .unwrap();

        let result =
            register.mark_measured(
                QubitId::new(0)
            );

        assert_eq!(
            result,
            Err(QubitError::Disabled {
                qubit: QubitId::new(0)
            })
        );
    }

    #[test]
    fn enable_restores_qubit() {
        let mut register =
            QubitRegister::new(1);

        register
            .disable(QubitId::new(0))
            .unwrap();

        register
            .enable(QubitId::new(0))
            .unwrap();

        assert!(
            register
                .get(QubitId::new(0))
                .unwrap()
                .is_available()
        );
    }

    #[test]
    fn out_of_range_is_rejected() {
        let register =
            QubitRegister::new(2);

        let result =
            register.get(QubitId::new(2));

        assert_eq!(
            result,
            Err(QubitError::OutOfRange {
                qubit: QubitId::new(2),
                num_qubits: 2
            })
        );
    }

    #[test]
    fn duplicate_qubits_are_rejected() {
        let qubits = [
            QubitId::new(0),
            QubitId::new(1),
            QubitId::new(0),
        ];

        let result =
            validate_unique_qubits(&qubits);

        assert_eq!(
            result,
            Err(QubitError::DuplicateQubit {
                qubit: QubitId::new(0)
            })
        );
    }

    #[test]
    fn qubit_range_is_validated() {
        let qubits = [
            QubitId::new(0),
            QubitId::new(3),
        ];

        let result =
            validate_qubits(&qubits, 3);

        assert_eq!(
            result,
            Err(QubitError::OutOfRange {
                qubit: QubitId::new(3),
                num_qubits: 3
            })
        );
    }

    #[test]
    fn register_iteration_works() {
        let register =
            QubitRegister::new(3);

        let ids: Vec<_> =
            register
                .into_iter()
                .map(|q| q.id().index())
                .collect();

        assert_eq!(ids, vec![0, 1, 2]);
    }
}