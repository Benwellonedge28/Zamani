//! Zamani Quantum IR — Qubits
//!
//! Hardware-independent logical-qubit representation.
//!
//! # Architectural boundary
//!
//! This module owns the logical qubit namespace used by the quantum IR.
//! `QubitId` identifies a logical qubit and `PhysicalQubitId` identifies a
//! hardware qubit. The two types are intentionally distinct.
//!
//! This module does **not** perform:
//!
//! - physical-qubit allocation;
//! - logical-to-physical routing;
//! - hardware topology validation;
//! - calibration;
//! - scheduling;
//! - pulse generation;
//! - QPU communication.
//!
//! Those responsibilities belong to downstream compiler/backend stages.
//!
//! # Safety and invariants
//!
//! The public APIs are designed so that:
//!
//! - logical identifiers are strongly typed;
//! - logical and physical identifiers cannot be confused accidentally;
//! - register construction can be bounded before allocation;
//! - qubit collections can be validated without allocation;
//! - duplicate logical operands are rejected deterministically;
//! - out-of-range identifiers are rejected deterministically;
//! - disabled qubits cannot be used through state-transition helpers;
//! - callers cannot obtain an unrestricted mutable slice of the register;
//! - iteration order is deterministic.
//!
//! Rust compatibility target: Rust 1.97.1.

use std::fmt;

// -----------------------------------------------------------------------------
// Logical qubit identifier
// -----------------------------------------------------------------------------

/// Stable logical qubit identifier.
///
/// A `QubitId` belongs to the logical namespace of a quantum IR program.
/// It does not identify a physical hardware qubit.
///
/// `QubitId` is intentionally a distinct type from `PhysicalQubitId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QubitId(usize);

impl QubitId {
    /// Creates a logical qubit identifier from an index.
    ///
    /// This constructor does not establish that the identifier belongs to
    /// any particular register. Register membership must be validated by
    /// `QubitRegister` or the circuit validation layer.
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the zero-based logical index.
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

/// Physical hardware-qubit identifier.
///
/// This type exists in the IR vocabulary only to preserve the distinction
/// between logical and physical identity. Creating a `PhysicalQubitId` does
/// not establish a routing or hardware mapping.
///
/// Actual logical-to-physical mapping belongs to routing/backend stages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PhysicalQubitId(usize);

impl PhysicalQubitId {
    /// Creates a physical-qubit identifier.
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the physical hardware index.
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
// Logical qubit state
// -----------------------------------------------------------------------------

/// Logical IR state associated with a qubit.
///
/// This is intentionally a lightweight compiler/IR state marker. It is not a
/// simulation of a physical quantum state and must never be interpreted as
/// the amplitudes or density matrix of a quantum system.
///
/// In particular:
///
/// - `Available` means the logical qubit is usable;
/// - `Reset` records that a reset operation has established reset semantics;
/// - `Measured` records that a measurement operation has consumed/observed it;
/// - `Disabled` prevents use through the register state-transition APIs.
///
/// Circuit-level validation remains authoritative for actual operation
/// legality. This state is supplementary IR bookkeeping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QubitState {
    /// Logical qubit is available for normal IR operations.
    Available,

    /// A reset operation has established reset semantics.
    Reset,

    /// A measurement operation has been applied.
    Measured,

    /// Logical qubit has been disabled/reserved by an IR-level owner.
    Disabled,
}

impl Default for QubitState {
    fn default() -> Self {
        Self::Available
    }
}

impl QubitState {
    /// Returns whether the qubit is available.
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }

    /// Returns whether the qubit is marked measured.
    pub const fn is_measured(self) -> bool {
        matches!(self, Self::Measured)
    }

    /// Returns whether the qubit is marked reset.
    pub const fn is_reset(self) -> bool {
        matches!(self, Self::Reset)
    }

    /// Returns whether the qubit is disabled.
    pub const fn is_disabled(self) -> bool {
        matches!(self, Self::Disabled)
    }
}

// -----------------------------------------------------------------------------
// Logical qubit
// -----------------------------------------------------------------------------

/// A logical qubit owned by a `QubitRegister`.
///
/// The type deliberately contains no physical mapping information.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Qubit {
    id: QubitId,
    state: QubitState,
}

impl Qubit {
    /// Creates a new available logical qubit.
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

    /// Returns the current IR state marker.
    pub const fn state(&self) -> QubitState {
        self.state
    }

    /// Returns whether the qubit is available.
    pub const fn is_available(&self) -> bool {
        self.state.is_available()
    }

    /// Returns whether the qubit is marked measured.
    pub const fn is_measured(&self) -> bool {
        self.state.is_measured()
    }

    /// Returns whether the qubit is marked reset.
    pub const fn is_reset(&self) -> bool {
        self.state.is_reset()
    }

    /// Returns whether the qubit is disabled.
    pub const fn is_disabled(&self) -> bool {
        self.state.is_disabled()
    }

    /// Marks the qubit as reset.
    ///
    /// State transitions are performed through the register rather than
    /// exposing arbitrary mutable state to circuit consumers.
    fn mark_reset(&mut self) {
        self.state = QubitState::Reset;
    }

    /// Marks the qubit as measured.
    fn mark_measured(&mut self) {
        self.state = QubitState::Measured;
    }

    /// Marks the qubit available.
    fn mark_available(&mut self) {
        self.state = QubitState::Available;
    }

    /// Marks the qubit disabled.
    fn mark_disabled(&mut self) {
        self.state = QubitState::Disabled;
    }
}

// -----------------------------------------------------------------------------
// Qubit errors
// -----------------------------------------------------------------------------

/// Errors produced by logical-qubit namespace operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QubitError {
    /// A requested register size exceeds the permitted limit.
    CountExceedsLimit {
        count: usize,
        maximum: usize,
    },

    /// A qubit identifier does not belong to the register.
    OutOfRange {
        qubit: QubitId,
        num_qubits: usize,
    },

    /// A classical/structural operation attempted to use a disabled qubit.
    Disabled {
        qubit: QubitId,
    },

    /// A collection contains the same logical qubit more than once.
    DuplicateQubit {
        qubit: QubitId,
    },

    /// A supplied collection contains an invalid logical qubit.
    InvalidQubit {
        qubit: QubitId,
    },

    /// No available logical qubit exists.
    NoAvailableQubit,

    /// A register construction request could not be represented safely.
    InvalidCount {
        count: usize,
    },
}

impl fmt::Display for QubitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CountExceedsLimit { count, maximum } => {
                write!(
                    f,
                    "logical qubit count {count} exceeds configured maximum {maximum}"
                )
            }

            Self::OutOfRange {
                qubit,
                num_qubits,
            } => {
                write!(
                    f,
                    "logical qubit {qubit} is outside register range 0..{num_qubits}"
                )
            }

            Self::Disabled { qubit } => {
                write!(f, "logical qubit {qubit} is disabled")
            }

            Self::DuplicateQubit { qubit } => {
                write!(
                    f,
                    "logical qubit {qubit} appears more than once"
                )
            }

            Self::InvalidQubit { qubit } => {
                write!(f, "invalid logical qubit {qubit}")
            }

            Self::NoAvailableQubit => {
                write!(f, "no available logical qubit")
            }

            Self::InvalidCount { count } => {
                write!(f, "invalid logical qubit count: {count}")
            }
        }
    }
}

impl std::error::Error for QubitError {}

// -----------------------------------------------------------------------------
// Logical qubit register
// -----------------------------------------------------------------------------

/// Deterministic logical-qubit namespace for a quantum circuit.
///
/// `QubitRegister` does not allocate physical hardware resources.
///
/// Its primary responsibilities are:
///
/// - defining the logical qubit namespace;
/// - providing deterministic identifier lookup;
/// - tracking optional IR state markers;
/// - validating logical operands.
///
/// The register does not perform routing or physical allocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QubitRegister {
    qubits: Vec<Qubit>,
}

impl QubitRegister {
    /// Creates an empty logical register.
    ///
    /// This is always allocation-safe because no vector capacity is requested.
    pub fn empty() -> Self {
        Self {
            qubits: Vec::new(),
        }
    }

    /// Creates a logical register with the requested number of qubits.
    ///
    /// This constructor is retained as a compatibility convenience. For
    /// untrusted or externally supplied counts, prefer `try_new(count,
    /// maximum)` so the allocation bound is explicit.
    ///
    /// The method rejects counts that cannot be represented by the platform's
    /// vector allocation model before attempting construction.
    pub fn new(count: usize) -> Self {
        assert!(
            count <= Self::maximum_constructible_count(),
            "logical qubit count exceeds the safe construction bound"
        );

        let mut qubits = Vec::with_capacity(count);

        for index in 0..count {
            qubits.push(Qubit::new(QubitId::new(index)));
        }

        Self { qubits }
    }

    /// Creates a register with an explicit maximum qubit limit.
    ///
    /// This is the preferred constructor for compiler boundaries receiving
    /// untrusted or externally generated IR.
    ///
    /// No allocation occurs when `count` exceeds `maximum`.
    pub fn try_new(
        count: usize,
        maximum: usize,
    ) -> Result<Self, QubitError> {
        if count > maximum {
            return Err(QubitError::CountExceedsLimit {
                count,
                maximum,
            });
        }

        if count > Self::maximum_constructible_count() {
            return Err(QubitError::InvalidCount { count });
        }

        let mut qubits = Vec::with_capacity(count);

        for index in 0..count {
            qubits.push(Qubit::new(QubitId::new(index)));
        }

        Ok(Self { qubits })
    }

    /// Returns a conservative construction bound based on the platform's
    /// addressable allocation space.
    ///
    /// This is not a substitute for `QuantumIrLimits`; it only prevents
    /// obviously impossible vector allocations.
    const fn maximum_constructible_count() -> usize {
        isize::MAX as usize / std::mem::size_of::<Qubit>()
    }

    /// Returns the number of logical qubits.
    pub fn len(&self) -> usize {
        self.qubits.len()
    }

    /// Returns whether the logical register is empty.
    pub fn is_empty(&self) -> bool {
        self.qubits.is_empty()
    }

    /// Returns the first available logical qubit.
    ///
    /// The result is deterministic because register iteration is always
    /// ordered by logical identifier.
    pub fn first_available(&self) -> Option<QubitId> {
        self.qubits
            .iter()
            .find(|qubit| qubit.is_available())
            .map(Qubit::id)
    }

    /// Returns a logical qubit by identifier.
    pub fn get(&self, id: QubitId) -> Result<&Qubit, QubitError> {
        self.qubits
            .get(id.index())
            .ok_or(QubitError::OutOfRange {
                qubit: id,
                num_qubits: self.len(),
            })
    }

    /// Returns the logical qubit at an index without constructing an error.
    pub fn get_opt(&self, id: QubitId) -> Option<&Qubit> {
        self.qubits.get(id.index())
    }

    /// Returns the logical qubits as an immutable slice.
    ///
    /// No mutable slice is exposed. State transitions must go through the
    /// controlled APIs below.
    pub fn as_slice(&self) -> &[Qubit] {
        &self.qubits
    }

    /// Marks a logical qubit as measured.
    pub fn mark_measured(
        &mut self,
        id: QubitId,
    ) -> Result<(), QubitError> {
        let qubit = self.get_mut_internal(id)?;

        if qubit.is_disabled() {
            return Err(QubitError::Disabled { qubit: id });
        }

        qubit.mark_measured();

        Ok(())
    }

    /// Marks a logical qubit as reset.
    pub fn reset(
        &mut self,
        id: QubitId,
    ) -> Result<(), QubitError> {
        let qubit = self.get_mut_internal(id)?;

        if qubit.is_disabled() {
            return Err(QubitError::Disabled { qubit: id });
        }

        qubit.mark_reset();

        Ok(())
    }

    /// Marks a logical qubit as available.
    ///
    /// This operation is deliberately explicit; measurement/reset semantics
    /// should not be inferred from ordinary gate construction.
    pub fn mark_available(
        &mut self,
        id: QubitId,
    ) -> Result<(), QubitError> {
        let qubit = self.get_mut_internal(id)?;

        if qubit.is_disabled() {
            return Err(QubitError::Disabled { qubit: id });
        }

        qubit.mark_available();

        Ok(())
    }

    /// Disables a logical qubit.
    pub fn disable(
        &mut self,
        id: QubitId,
    ) -> Result<(), QubitError> {
        let qubit = self.get_mut_internal(id)?;
        qubit.mark_disabled();
        Ok(())
    }

    /// Re-enables a disabled logical qubit.
    pub fn enable(
        &mut self,
        id: QubitId,
    ) -> Result<(), QubitError> {
        let qubit = self.get_mut_internal(id)?;
        qubit.mark_available();
        Ok(())
    }

    /// Validates a logical identifier against this register.
    pub fn validate(
        &self,
        id: QubitId,
    ) -> Result<(), QubitError> {
        if id.index() >= self.len() {
            return Err(QubitError::OutOfRange {
                qubit: id,
                num_qubits: self.len(),
            });
        }

        Ok(())
    }

    /// Validates a logical identifier and verifies that it is usable.
    pub fn validate_usable(
        &self,
        id: QubitId,
    ) -> Result<(), QubitError> {
        let qubit = self.get(id)?;

        if qubit.is_disabled() {
            return Err(QubitError::Disabled { qubit: id });
        }

        Ok(())
    }

    fn get_mut_internal(
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
}

impl Default for QubitRegister {
    fn default() -> Self {
        Self::empty()
    }
}

// -----------------------------------------------------------------------------
// Deterministic iteration
// -----------------------------------------------------------------------------

impl IntoIterator for QubitRegister {
    type Item = Qubit;
    type IntoIter = std::vec::IntoIter<Qubit>;

    fn into_iter(self) -> Self::IntoIter {
        self.qubits.into_iter()
    }
}

impl<'a> IntoIterator for &'a QubitRegister {
    type Item = &'a Qubit;
    type IntoIter = std::slice::Iter<'a, Qubit>;

    fn into_iter(self) -> Self::IntoIter {
        self.qubits.iter()
    }
}

// -----------------------------------------------------------------------------
// Qubit collection validation
// -----------------------------------------------------------------------------

/// Validates that all logical qubits in a collection are unique.
///
/// Complexity is O(n²), intentionally deterministic and allocation-free.
///
/// The validation layer can impose `QuantumIrLimits::max_operands` before
/// invoking this function when processing untrusted IR.
pub fn validate_unique_qubits(
    qubits: &[QubitId],
) -> Result<(), QubitError> {
    for index in 0..qubits.len() {
        let current = qubits[index];

        if qubits[index + 1..].contains(&current) {
            return Err(QubitError::DuplicateQubit {
                qubit: current,
            });
        }
    }

    Ok(())
}

/// Validates logical qubits against a register size and rejects duplicates.
///
/// This function performs no allocation and has deterministic behavior.
pub fn validate_qubits(
    qubits: &[QubitId],
    num_qubits: usize,
) -> Result<(), QubitError> {
    validate_unique_qubits(qubits)?;

    for &qubit in qubits {
        if qubit.index() >= num_qubits {
            return Err(QubitError::OutOfRange {
                qubit,
                num_qubits,
            });
        }
    }

    Ok(())
}

/// Returns whether all qubits are unique.
///
/// This is a non-error convenience API for callers that only need a boolean.
pub fn are_unique_qubits(qubits: &[QubitId]) -> bool {
    validate_unique_qubits(qubits).is_ok()
}

/// Returns whether all qubits are valid for the supplied register size.
pub fn are_valid_qubits(
    qubits: &[QubitId],
    num_qubits: usize,
) -> bool {
    validate_qubits(qubits, num_qubits).is_ok()
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    #[test]
    fn logical_identifier_is_stable() {
        let id = QubitId::new(7);

        assert_eq!(id.index(), 7);
        assert_eq!(id.to_string(), "q7");
    }

    #[test]
    fn physical_identifier_is_distinct_in_type() {
        let logical = QubitId::new(3);
        let physical = PhysicalQubitId::new(3);

        assert_eq!(logical.index(), physical.index());
        assert_eq!(physical.to_string(), "p3");
    }

    #[test]
    fn default_qubit_is_available() {
        let qubit = Qubit::new(q(0));

        assert_eq!(qubit.state(), QubitState::Available);
        assert!(qubit.is_available());
        assert!(!qubit.is_disabled());
    }

    #[test]
    fn empty_register_is_safe() {
        let register = QubitRegister::empty();

        assert_eq!(register.len(), 0);
        assert!(register.is_empty());
        assert_eq!(register.first_available(), None);
    }

    #[test]
    fn register_contains_deterministic_ids() {
        let register = QubitRegister::new(4);

        assert_eq!(register.len(), 4);
        assert_eq!(register.get(q(0)).unwrap().id(), q(0));
        assert_eq!(register.get(q(1)).unwrap().id(), q(1));
        assert_eq!(register.get(q(2)).unwrap().id(), q(2));
        assert_eq!(register.get(q(3)).unwrap().id(), q(3));
    }

    #[test]
    fn first_available_is_deterministic() {
        let register = QubitRegister::new(4);

        assert_eq!(register.first_available(), Some(q(0)));
    }

    #[test]
    fn explicit_limit_rejects_before_allocation() {
        let result = QubitRegister::try_new(8, 4);

        assert_eq!(
            result,
            Err(QubitError::CountExceedsLimit {
                count: 8,
                maximum: 4,
            })
        );
    }

    #[test]
    fn explicit_limit_accepts_valid_count() {
        let register =
            QubitRegister::try_new(4, 4).unwrap();

        assert_eq!(register.len(), 4);
    }

    #[test]
    fn out_of_range_is_rejected() {
        let register = QubitRegister::new(2);

        assert_eq!(
            register.get(q(2)),
            Err(QubitError::OutOfRange {
                qubit: q(2),
                num_qubits: 2,
            })
        );
    }

    #[test]
    fn optional_lookup_is_allocation_free() {
        let register = QubitRegister::new(2);

        assert!(register.get_opt(q(0)).is_some());
        assert!(register.get_opt(q(2)).is_none());
    }

    #[test]
    fn measurement_transition_is_controlled() {
        let mut register = QubitRegister::new(2);

        register.mark_measured(q(1)).unwrap();

        let qubit = register.get(q(1)).unwrap();

        assert!(qubit.is_measured());
        assert_eq!(qubit.state(), QubitState::Measured);
    }

    #[test]
    fn reset_transition_is_controlled() {
        let mut register = QubitRegister::new(2);

        register.reset(q(0)).unwrap();

        assert!(register.get(q(0)).unwrap().is_reset());
    }

    #[test]
    fn disabled_qubit_cannot_be_measured() {
        let mut register = QubitRegister::new(1);

        register.disable(q(0)).unwrap();

        assert_eq!(
            register.mark_measured(q(0)),
            Err(QubitError::Disabled { qubit: q(0) })
        );
    }

    #[test]
    fn disabled_qubit_cannot_be_reset() {
        let mut register = QubitRegister::new(1);

        register.disable(q(0)).unwrap();

        assert_eq!(
            register.reset(q(0)),
            Err(QubitError::Disabled { qubit: q(0) })
        );
    }

    #[test]
    fn disabled_qubit_can_be_reenabled() {
        let mut register = QubitRegister::new(1);

        register.disable(q(0)).unwrap();
        register.enable(q(0)).unwrap();

        assert!(register.get(q(0)).unwrap().is_available());
    }

    #[test]
    fn mark_available_is_controlled() {
        let mut register = QubitRegister::new(1);

        register.mark_measured(q(0)).unwrap();
        register.mark_available(q(0)).unwrap();

        assert!(register.get(q(0)).unwrap().is_available());
    }

    #[test]
    fn duplicate_qubits_are_rejected() {
        let qubits = [q(0), q(1), q(0)];

        assert_eq!(
            validate_unique_qubits(&qubits),
            Err(QubitError::DuplicateQubit { qubit: q(0) })
        );
    }

    #[test]
    fn unique_qubits_are_accepted() {
        let qubits = [q(0), q(1), q(2)];

        assert_eq!(
            validate_unique_qubits(&qubits),
            Ok(())
        );
    }

    #[test]
    fn out_of_range_qubits_are_rejected() {
        let qubits = [q(0), q(2)];

        assert_eq!(
            validate_qubits(&qubits, 2),
            Err(QubitError::OutOfRange {
                qubit: q(2),
                num_qubits: 2,
            })
        );
    }

    #[test]
    fn valid_qubits_are_accepted() {
        let qubits = [q(0), q(1)];

        assert_eq!(
            validate_qubits(&qubits, 2),
            Ok(())
        );
    }

    #[test]
    fn boolean_unique_helper_is_correct() {
        assert!(are_unique_qubits(&[q(0), q(1)]));
        assert!(!are_unique_qubits(&[q(0), q(0)]));
    }

    #[test]
    fn boolean_validity_helper_is_correct() {
        assert!(are_valid_qubits(&[q(0), q(1)], 2));
        assert!(!are_valid_qubits(&[q(0), q(2)], 2));
    }

    #[test]
    fn immutable_slice_does_not_allow_state_mutation() {
        let register = QubitRegister::new(2);
        let slice = register.as_slice();

        assert_eq!(slice.len(), 2);
        assert_eq!(slice[0].id(), q(0));
    }

    #[test]
    fn iteration_is_in_logical_order() {
        let register = QubitRegister::new(3);

        let ids: Vec<QubitId> =
            register.iter().map(Qubit::id).collect();

        assert_eq!(ids, vec![q(0), q(1), q(2)]);
    }
}

// -----------------------------------------------------------------------------
// Public iterator convenience
// -----------------------------------------------------------------------------

impl QubitRegister {
    /// Returns an immutable deterministic iterator over logical qubits.
    pub fn iter(&self) -> std::slice::Iter<'_, Qubit> {
        self.qubits.iter()
    }
}