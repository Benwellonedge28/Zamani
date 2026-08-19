//! Production-grade Pauli-frame tracking for Zamani QEC.
//!
//! A Pauli frame records pending Pauli corrections classically instead of
//! requiring every correction to be physically applied immediately.
//!
//! Global phase is intentionally ignored, matching the stabilizer algebra.
//!
//! Invariants:
//! - the frame always has a fixed qubit count;
//! - every operation is bounds checked;
//! - incompatible Pauli strings are rejected;
//! - construction is bounded against accidental enormous allocations;
//! - updates are deterministic;
//! - this module never performs a physical quantum operation.

use std::fmt;

use super::stabilizer::{
    Pauli,
    PauliString,
    QubitIndex,
    StabilizerError,
};

/// Maximum number of qubits accepted by the safe frame constructor.
///
/// This is a resource-safety boundary for untrusted configuration. Larger
/// systems should be governed by an application-level resource policy rather
/// than silently permitting arbitrary allocation.
pub const MAX_FRAME_QUBITS: usize = 1_000_000;

/// Classical Pauli frame.
///
/// The frame represents the pending Pauli correction that should be tracked
/// alongside the physical quantum state.
///
/// For an n-qubit system:
///
/// ```text
/// frame = P_0 ⊗ P_1 ⊗ ... ⊗ P_(n-1)
/// ```
///
/// where each `P_i` is one of `I`, `X`, `Y`, or `Z`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct PauliFrame {
    operator: PauliString,
}

impl PauliFrame {
    /// Creates an identity Pauli frame.
    pub fn new(
        num_qubits: usize,
    ) -> Result<Self, PauliFrameError> {
        if num_qubits == 0 {
            return Err(
                PauliFrameError::ZeroQubits,
            );
        }

        if num_qubits > MAX_FRAME_QUBITS {
            return Err(
                PauliFrameError::QubitLimitExceeded {
                    requested: num_qubits,
                    maximum: MAX_FRAME_QUBITS,
                },
            );
        }

        Ok(Self {
            operator:
                PauliString::identity(
                    num_qubits,
                ),
        })
    }

    /// Creates a frame from an existing Pauli operator.
    ///
    /// The operator must contain at least one qubit and must not exceed the
    /// frame resource limit.
    pub fn from_operator(
        operator: PauliString,
    ) -> Result<Self, PauliFrameError> {
        let num_qubits =
            operator.num_qubits();

        if num_qubits == 0 {
            return Err(
                PauliFrameError::ZeroQubits,
            );
        }

        if num_qubits > MAX_FRAME_QUBITS {
            return Err(
                PauliFrameError::QubitLimitExceeded {
                    requested: num_qubits,
                    maximum: MAX_FRAME_QUBITS,
                },
            );
        }

        Ok(Self { operator })
    }

    /// Number of physical qubits represented by this frame.
    pub const fn num_qubits(
        &self,
    ) -> usize {
        self.operator.num_qubits()
    }

    /// Returns the complete Pauli operator represented by the frame.
    pub fn operator(
        &self,
    ) -> &PauliString {
        &self.operator
    }

    /// Returns true when no correction is pending.
    pub fn is_identity(
        &self,
    ) -> bool {
        self.operator.is_identity()
    }

    /// Returns the number of qubits carrying a non-identity correction.
    pub fn weight(
        &self,
    ) -> usize {
        self.operator.weight()
    }

    /// Returns the Pauli currently tracked for one qubit.
    pub fn pauli_at(
        &self,
        qubit: usize,
    ) -> Result<Pauli, PauliFrameError> {
        self.check_qubit(qubit)?;

        self.operator
            .pauli_at(
                QubitIndex::new(qubit),
            )
            .map_err(
                PauliFrameError::Stabilizer,
            )
    }

    /// Sets the pending Pauli correction on one qubit.
    ///
    /// This replaces the existing Pauli at that qubit.
    pub fn set(
        &mut self,
        qubit: usize,
        pauli: Pauli,
    ) -> Result<(), PauliFrameError> {
        self.check_qubit(qubit)?;

        self.operator
            .set_pauli(
                QubitIndex::new(qubit),
                pauli,
            )
            .map_err(
                PauliFrameError::Stabilizer,
            )
    }

    /// Composes another Pauli correction into the frame.
    ///
    /// Pauli multiplication is performed modulo global phase. In the
    /// binary-symplectic representation this is XOR of the X/Z components.
    pub fn compose(
        &mut self,
        correction: &PauliString,
    ) -> Result<(), PauliFrameError> {
        self.check_compatible(
            correction,
        )?;

        self.operator =
            self.operator
                .multiply(correction)
                .map_err(
                    PauliFrameError::Stabilizer,
                )?;

        Ok(())
    }

    /// Returns a new frame produced by composing `correction`.
    ///
    /// The original frame is not modified.
    pub fn composed(
        &self,
        correction: &PauliString,
    ) -> Result<Self, PauliFrameError> {
        let mut result =
            self.clone();

        result.compose(correction)?;

        Ok(result)
    }

    /// Clears all pending corrections.
    ///
    /// The physical-qubit count remains unchanged.
    pub fn clear(
        &mut self,
    ) {
        self.operator =
            PauliString::identity(
                self.num_qubits(),
            );
    }

    /// Tests whether the frame commutes with an observable.
    pub fn commutes_with(
        &self,
        observable: &PauliString,
    ) -> Result<bool, PauliFrameError> {
        self.check_compatible(
            observable,
        )?;

        self.operator
            .commutes_with(observable)
            .map_err(
                PauliFrameError::Stabilizer,
            )
    }

    /// Tests whether the frame anticommutes with an observable.
    pub fn anticommutes_with(
        &self,
        observable: &PauliString,
    ) -> Result<bool, PauliFrameError> {
        self.check_compatible(
            observable,
        )?;

        self.operator
            .anticommutes_with(observable)
            .map_err(
                PauliFrameError::Stabilizer,
            )
    }

    /// Returns the qubits carrying non-identity corrections.
    ///
    /// `PauliString::support()` returns qubits in deterministic ascending
    /// order, making this suitable for reproducible decoding and testing.
    pub fn support(
        &self,
    ) -> Vec<QubitIndex> {
        self.operator.support()
    }

    fn check_qubit(
        &self,
        qubit: usize,
    ) -> Result<(), PauliFrameError> {
        if qubit >= self.num_qubits() {
            return Err(
                PauliFrameError::QubitOutOfRange {
                    qubit,
                    num_qubits:
                        self.num_qubits(),
                },
            );
        }

        Ok(())
    }

    fn check_compatible(
        &self,
        other: &PauliString,
    ) -> Result<(), PauliFrameError> {
        if other.num_qubits()
            != self.num_qubits()
        {
            return Err(
                PauliFrameError::QubitCountMismatch {
                    expected:
                        self.num_qubits(),
                    actual:
                        other.num_qubits(),
                },
            );
        }

        Ok(())
    }
}

impl fmt::Display for PauliFrame {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "{}",
            self.operator
        )
    }
}

/// Errors produced by safe Pauli-frame operations.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum PauliFrameError {
    /// A frame cannot represent a zero-qubit system.
    ZeroQubits,

    /// The requested frame exceeds the safety allocation limit.
    QubitLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// A qubit index lies outside the frame.
    QubitOutOfRange {
        qubit: usize,
        num_qubits: usize,
    },

    /// A correction/observable belongs to a different-sized system.
    QubitCountMismatch {
        expected: usize,
        actual: usize,
    },

    /// Error originating in the shared stabilizer algebra.
    Stabilizer(StabilizerError),
}

impl fmt::Display
    for PauliFrameError
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::ZeroQubits => {
                write!(
                    f,
                    "a Pauli frame requires at least one qubit"
                )
            }

            Self::QubitLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "Pauli frame size {requested} exceeds maximum {maximum}"
                )
            }

            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => {
                write!(
                    f,
                    "qubit {qubit} is outside a {num_qubits}-qubit frame"
                )
            }

            Self::QubitCountMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Pauli frame requires {expected} qubits, got {actual}"
                )
            }

            Self::Stabilizer(error) => {
                write!(
                    f,
                    "stabilizer error: {error}"
                )
            }
        }
    }
}

impl std::error::Error
    for PauliFrameError
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_frame_is_empty() {
        let frame =
            PauliFrame::new(3)
                .unwrap();

        assert!(
            frame.is_identity()
        );

        assert_eq!(
            frame.weight(),
            0
        );
    }

    #[test]
    fn set_tracks_pauli() {
        let mut frame =
            PauliFrame::new(3)
                .unwrap();

        frame
            .set(1, Pauli::X)
            .unwrap();

        assert_eq!(
            frame.pauli_at(1)
                .unwrap(),
            Pauli::X
        );

        assert_eq!(
            frame.weight(),
            1
        );
    }

    #[test]
    fn setting_identity_clears_qubit() {
        let mut frame =
            PauliFrame::new(2)
                .unwrap();

        frame
            .set(0, Pauli::Y)
            .unwrap();

        frame
            .set(0, Pauli::I)
            .unwrap();

        assert_eq!(
            frame.pauli_at(0)
                .unwrap(),
            Pauli::I
        );

        assert!(
            frame.is_identity()
        );
    }

    #[test]
    fn composition_is_modulo_global_phase() {
        let mut frame =
            PauliFrame::new(2)
                .unwrap();

        frame
            .set(0, Pauli::X)
            .unwrap();

        frame
            .set(1, Pauli::Z)
            .unwrap();

        let correction =
            PauliString::from_paulis(
                &[Pauli::X, Pauli::Z],
            );

        frame
            .compose(&correction)
            .unwrap();

        assert!(
            frame.is_identity()
        );
    }

    #[test]
    fn x_composed_with_z_becomes_y() {
        let mut frame =
            PauliFrame::new(1)
                .unwrap();

        frame
            .set(0, Pauli::X)
            .unwrap();

        let correction =
            PauliString::from_paulis(
                &[Pauli::Z],
            );

        frame
            .compose(&correction)
            .unwrap();

        assert_eq!(
            frame.pauli_at(0)
                .unwrap(),
            Pauli::Y
        );
    }

    #[test]
    fn composed_does_not_mutate_original() {
        let mut frame =
            PauliFrame::new(1)
                .unwrap();

        frame
            .set(0, Pauli::X)
            .unwrap();

        let correction =
            PauliString::from_paulis(
                &[Pauli::Z],
            );

        let result =
            frame
                .composed(&correction)
                .unwrap();

        assert_eq!(
            frame.pauli_at(0)
                .unwrap(),
            Pauli::X
        );

        assert_eq!(
            result.pauli_at(0)
                .unwrap(),
            Pauli::Y
        );
    }

    #[test]
    fn clear_restores_identity() {
        let mut frame =
            PauliFrame::new(4)
                .unwrap();

        frame
            .set(0, Pauli::X)
            .unwrap();

        frame
            .set(2, Pauli::Y)
            .unwrap();

        frame.clear();

        assert!(
            frame.is_identity()
        );

        assert_eq!(
            frame.weight(),
            0
        );

        assert_eq!(
            frame.num_qubits(),
            4
        );
    }

    #[test]
    fn support_is_deterministic() {
        let mut frame =
            PauliFrame::new(4)
                .unwrap();

        frame
            .set(3, Pauli::Y)
            .unwrap();

        frame
            .set(1, Pauli::X)
            .unwrap();

        assert_eq!(
            frame.support(),
            vec![
                QubitIndex::new(1),
                QubitIndex::new(3),
            ]
        );
    }

    #[test]
    fn out_of_range_is_rejected() {
        let mut frame =
            PauliFrame::new(2)
                .unwrap();

        assert!(matches!(
            frame.set(
                2,
                Pauli::X,
            ),
            Err(
                PauliFrameError::
                    QubitOutOfRange { .. }
            )
        ));

        assert!(matches!(
            frame.pauli_at(2),
            Err(
                PauliFrameError::
                    QubitOutOfRange { .. }
            )
        ));
    }

    #[test]
    fn mismatched_operator_is_rejected() {
        let mut frame =
            PauliFrame::new(2)
                .unwrap();

        let correction =
            PauliString::identity(3);

        assert!(matches!(
            frame.compose(
                &correction
            ),
            Err(
                PauliFrameError::
                    QubitCountMismatch { .. }
            )
        ));
    }

    #[test]
    fn mismatched_observable_is_rejected() {
        let frame =
            PauliFrame::new(2)
                .unwrap();

        let observable =
            PauliString::identity(3);

        assert!(matches!(
            frame.commutes_with(
                &observable
            ),
            Err(
                PauliFrameError::
                    QubitCountMismatch { .. }
            )
        ));
    }

    #[test]
    fn zero_qubit_frame_is_rejected() {
        assert!(matches!(
            PauliFrame::new(0),
            Err(
                PauliFrameError::
                    ZeroQubits
            )
        ));
    }

    #[test]
    fn oversized_frame_is_rejected() {
        assert!(matches!(
            PauliFrame::new(
                MAX_FRAME_QUBITS + 1
            ),
            Err(
                PauliFrameError::
                    QubitLimitExceeded { .. }
            )
        ));
    }
}