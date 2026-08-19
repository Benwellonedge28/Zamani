//! Zamani Quantum Error Correction — Decoder.
//!
//! Decoder-independent interfaces and reference implementations for
//! converting stabilizer syndromes into candidate corrections.
//!
//! Mathematical Pauli/stabilizer operations live in `stabilizer.rs`.
//! This module owns decoding policy and correction selection.
//!
//! Architecture:
//!
//! ```text
//!                    stabilizer.rs
//!                         │
//!              ┌──────────┴──────────┐
//!              │                     │
//!         PauliString             Syndrome
//!              │                     │
//!              └──────────┬──────────┘
//!                         ▼
//!                    Decoder
//!                         │
//!             ┌───────────┴───────────┐
//!             ▼                       ▼
//!        Correction              DecodeResult
//! ```
//!
//! A decoder does not modify the quantum state. It interprets a measured
//! syndrome and proposes a correction.

use std::collections::BTreeSet;
use std::fmt;

use super::stabilizer::{
    Pauli,
    PauliString,
    StabilizerError,
    StabilizerGroup,
    Syndrome,
};

// -----------------------------------------------------------------------------
// Decoder identifier
// -----------------------------------------------------------------------------

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct DecoderId(pub usize);

impl DecoderId {
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for DecoderId {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "decoder-{}", self.0)
    }
}

// -----------------------------------------------------------------------------
// Correction
// -----------------------------------------------------------------------------

/// A proposed Pauli correction.
///
/// The correction is represented using the same binary-symplectic
/// `PauliString` used by the stabilizer algebra.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct Correction {
    operator: PauliString,
}

impl Correction {
    pub fn new(
        operator: PauliString,
    ) -> Self {
        Self { operator }
    }

    pub fn identity(
        num_qubits: usize,
    ) -> Self {
        Self {
            operator:
                PauliString::identity(
                    num_qubits,
                ),
        }
    }

    pub fn operator(&self) -> &PauliString {
        &self.operator
    }

    pub fn weight(&self) -> usize {
        self.operator.weight()
    }

    pub fn is_identity(&self) -> bool {
        self.operator.is_identity()
    }

    pub fn num_qubits(&self) -> usize {
        self.operator.num_qubits()
    }
}

// -----------------------------------------------------------------------------
// Decode result
// -----------------------------------------------------------------------------

/// Result returned by a decoder.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct DecodeResult {
    decoder: DecoderId,
    syndrome: Syndrome,
    correction: Correction,
}

impl DecodeResult {
    pub fn new(
        decoder: DecoderId,
        syndrome: Syndrome,
        correction: Correction,
    ) -> Self {
        Self {
            decoder,
            syndrome,
            correction,
        }
    }

    pub const fn decoder(&self) -> DecoderId {
        self.decoder
    }

    pub fn syndrome(&self) -> &Syndrome {
        &self.syndrome
    }

    pub fn correction(&self) -> &Correction {
        &self.correction
    }

    pub fn correction_weight(&self) -> usize {
        self.correction.weight()
    }

    pub fn is_trivial(&self) -> bool {
        self.syndrome.is_trivial()
    }
}

// -----------------------------------------------------------------------------
// Decoder trait
// -----------------------------------------------------------------------------

/// Common interface implemented by all Zamani QEC decoders.
pub trait Decoder {
    fn id(&self) -> DecoderId;

    fn decode(
        &self,
        syndrome: &Syndrome,
    ) -> Result<DecodeResult, DecoderError>;
}

// -----------------------------------------------------------------------------
// Stabilizer-backed decoder
// -----------------------------------------------------------------------------

/// Base decoder containing the stabilizer system.
///
/// This gives decoders a validated mathematical model without forcing every
/// decoder implementation to duplicate stabilizer validation.
#[derive(
    Debug,
    Clone,
)]
pub struct StabilizerDecoder {
    id: DecoderId,
    stabilizers: StabilizerGroup,
}

impl StabilizerDecoder {
    pub fn new(
        id: DecoderId,
        stabilizers: StabilizerGroup,
    ) -> Result<Self, DecoderError> {
        stabilizers
            .validate()
            .map_err(DecoderError::Stabilizer)?;

        Ok(Self {
            id,
            stabilizers,
        })
    }

    pub const fn id(&self) -> DecoderId {
        self.id
    }

    pub fn stabilizers(
        &self,
    ) -> &StabilizerGroup {
        &self.stabilizers
    }

    pub fn num_qubits(&self) -> usize {
        self.stabilizers.num_qubits()
    }

    pub fn generator_count(&self) -> usize {
        self.stabilizers.len()
    }

    /// Recomputes a syndrome for a candidate Pauli error.
    pub fn syndrome_for_error(
        &self,
        error: &PauliString,
    ) -> Result<Syndrome, DecoderError> {
        self.stabilizers
            .syndrome(error)
            .map_err(DecoderError::Stabilizer)
    }

    /// Verifies that a candidate correction produces the requested syndrome.
    pub fn correction_matches_syndrome(
        &self,
        correction: &Correction,
        expected: &Syndrome,
    ) -> Result<bool, DecoderError> {
        let actual =
            self.syndrome_for_error(
                correction.operator(),
            )?;

        Ok(&actual == expected)
    }
}

// -----------------------------------------------------------------------------
// Identity decoder
// -----------------------------------------------------------------------------

/// Decoder used when no correction is required.
///
/// This is useful for:
/// - no-error syndromes;
/// - pipeline testing;
/// - decoder composition;
/// - hardware integration tests.
#[derive(
    Debug,
    Clone,
    Copy,
)]
pub struct IdentityDecoder {
    id: DecoderId,
    num_qubits: usize,
}

impl IdentityDecoder {
    pub fn new(
        id: DecoderId,
        num_qubits: usize,
    ) -> Result<Self, DecoderError> {
        if num_qubits == 0 {
            return Err(
                DecoderError::InvalidQubitCount {
                    count: num_qubits,
                },
            );
        }

        Ok(Self {
            id,
            num_qubits,
        })
    }

    pub const fn id(&self) -> DecoderId {
        self.id
    }

    pub const fn num_qubits(&self) -> usize {
        self.num_qubits
    }
}

impl Decoder for IdentityDecoder {
    fn id(&self) -> DecoderId {
        self.id
    }

    fn decode(
        &self,
        syndrome: &Syndrome,
    ) -> Result<DecodeResult, DecoderError> {
        Ok(DecodeResult::new(
            self.id,
            syndrome.clone(),
            Correction::identity(
                self.num_qubits,
            ),
        ))
    }
}

// -----------------------------------------------------------------------------
// Syndrome validator
// -----------------------------------------------------------------------------

/// Validates a syndrome against a stabilizer group.
pub fn validate_syndrome(
    syndrome: &Syndrome,
    stabilizers: &StabilizerGroup,
) -> Result<(), DecoderError> {
    if syndrome.len()
        != stabilizers.len()
    {
        return Err(
            DecoderError::SyndromeLengthMismatch {
                expected: stabilizers.len(),
                actual: syndrome.len(),
            },
        );
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Syndrome classification
// -----------------------------------------------------------------------------

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum SyndromeClass {
    Trivial,
    NonTrivial,
}

impl SyndromeClass {
    pub const fn classify(
        syndrome: &Syndrome,
    ) -> Self {
        if syndrome.is_trivial() {
            Self::Trivial
        } else {
            Self::NonTrivial
        }
    }
}

// -----------------------------------------------------------------------------
// Decoder statistics
// -----------------------------------------------------------------------------

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
)]
pub struct DecoderStatistics {
    decoded: usize,
    trivial: usize,
    nontrivial: usize,
    failed: usize,
}

impl DecoderStatistics {
    pub const fn new() -> Self {
        Self {
            decoded: 0,
            trivial: 0,
            nontrivial: 0,
            failed: 0,
        }
    }

    pub const fn decoded(&self) -> usize {
        self.decoded
    }

    pub const fn trivial(&self) -> usize {
        self.trivial
    }

    pub const fn nontrivial(&self) -> usize {
        self.nontrivial
    }

    pub const fn failed(&self) -> usize {
        self.failed
    }

    pub fn record(
        &mut self,
        result: Result<
            &DecodeResult,
            &DecoderError,
        >,
    ) {
        match result {
            Ok(result) => {
                self.decoded += 1;

                if result.is_trivial() {
                    self.trivial += 1;
                } else {
                    self.nontrivial += 1;
                }
            }

            Err(_) => {
                self.failed += 1;
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Correction validation
// -----------------------------------------------------------------------------

/// Validates that a correction belongs to the same physical system.
pub fn validate_correction(
    correction: &Correction,
    num_qubits: usize,
) -> Result<(), DecoderError> {
    if correction.num_qubits()
        != num_qubits
    {
        return Err(
            DecoderError::CorrectionQubitCountMismatch {
                expected: num_qubits,
                actual: correction.num_qubits(),
            },
        );
    }

    Ok(())
}

/// Validates a correction against a measured syndrome.
///
/// A valid correction must reproduce the syndrome when measured against
/// the stabilizer generators.
pub fn validate_correction_for_syndrome(
    correction: &Correction,
    syndrome: &Syndrome,
    stabilizers: &StabilizerGroup,
) -> Result<bool, DecoderError> {
    validate_syndrome(
        syndrome,
        stabilizers,
    )?;

    validate_correction(
        correction,
        stabilizers.num_qubits(),
    )?;

    let produced =
        stabilizers
            .syndrome(
                correction.operator(),
            )
            .map_err(
                DecoderError::Stabilizer,
            )?;

    Ok(&produced == syndrome)
}

// -----------------------------------------------------------------------------
// Pauli error helpers
// -----------------------------------------------------------------------------

/// Creates a single-qubit Pauli error.
pub fn single_qubit_error(
    num_qubits: usize,
    qubit: usize,
    pauli: Pauli,
) -> Result<PauliString, DecoderError> {
    if qubit >= num_qubits {
        return Err(
            DecoderError::QubitOutOfRange {
                qubit,
                num_qubits,
            },
        );
    }

    let mut error =
        PauliString::identity(
            num_qubits,
        );

    error
        .set_pauli(
            super::stabilizer::QubitIndex::new(
                qubit,
            ),
            pauli,
        )
        .map_err(
            DecoderError::Stabilizer,
        )?;

    Ok(error)
}

/// Creates an X error on one qubit.
pub fn x_error(
    num_qubits: usize,
    qubit: usize,
) -> Result<PauliString, DecoderError> {
    single_qubit_error(
        num_qubits,
        qubit,
        Pauli::X,
    )
}

/// Creates a Y error on one qubit.
pub fn y_error(
    num_qubits: usize,
    qubit: usize,
) -> Result<PauliString, DecoderError> {
    single_qubit_error(
        num_qubits,
        qubit,
        Pauli::Y,
    )
}

/// Creates a Z error on one qubit.
pub fn z_error(
    num_qubits: usize,
    qubit: usize,
) -> Result<PauliString, DecoderError> {
    single_qubit_error(
        num_qubits,
        qubit,
        Pauli::Z,
    )
}

// -----------------------------------------------------------------------------
// Decoder registry
// -----------------------------------------------------------------------------

/// Registry for decoder implementations.
///
/// This keeps decoder selection separate from decoder mathematics and allows
/// future implementations such as:
///
/// - minimum-weight perfect matching;
/// - union-find;
/// - belief propagation;
/// - tensor-network decoding;
/// - neural decoders.
#[derive(
    Debug,
    Default,
)]
pub struct DecoderRegistry {
    ids: BTreeSet<DecoderId>,
}

impl DecoderRegistry {
    pub fn new() -> Self {
        Self {
            ids: BTreeSet::new(),
        }
    }

    pub fn register<D>(
        &mut self,
        decoder: &D,
    ) -> Result<(), DecoderError>
    where
        D: Decoder,
    {
        if !self.ids.insert(
            decoder.id(),
        ) {
            return Err(
                DecoderError::DuplicateDecoder {
                    id: decoder.id(),
                },
            );
        }

        Ok(())
    }

    pub fn contains(
        &self,
        id: DecoderId,
    ) -> bool {
        self.ids.contains(&id)
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }
}

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum DecoderError {
    Stabilizer(
        StabilizerError,
    ),

    InvalidQubitCount {
        count: usize,
    },

    QubitOutOfRange {
        qubit: usize,
        num_qubits: usize,
    },

    SyndromeLengthMismatch {
        expected: usize,
        actual: usize,
    },

    CorrectionQubitCountMismatch {
        expected: usize,
        actual: usize,
    },

    DuplicateDecoder {
        id: DecoderId,
    },
}

impl fmt::Display for DecoderError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Stabilizer(error) => {
                write!(
                    f,
                    "stabilizer error: {error}"
                )
            }

            Self::InvalidQubitCount {
                count,
            } => {
                write!(
                    f,
                    "decoder requires at least one qubit, got {count}"
                )
            }

            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => {
                write!(
                    f,
                    "qubit {qubit} is outside a {num_qubits}-qubit system"
                )
            }

            Self::SyndromeLengthMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "syndrome length mismatch: expected {expected}, got {actual}"
                )
            }

            Self::CorrectionQubitCountMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "correction qubit count mismatch: expected {expected}, got {actual}"
                )
            }

            Self::DuplicateDecoder {
                id,
            } => {
                write!(
                    f,
                    "decoder {id} is already registered"
                )
            }
        }
    }
}

impl std::error::Error for DecoderError {}

impl From<StabilizerError>
    for DecoderError
{
    fn from(
        error: StabilizerError,
    ) -> Self {
        Self::Stabilizer(error)
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_decoder_returns_identity() {
        let decoder =
            IdentityDecoder::new(
                DecoderId::new(0),
                3,
            )
            .unwrap();

        let syndrome =
            Syndrome::new(vec![
                false,
                false,
            ]);

        let result =
            decoder.decode(
                &syndrome,
            )
            .unwrap();

        assert!(
            result
                .correction()
                .is_identity()
        );

        assert_eq!(
            result
                .correction()
                .num_qubits(),
            3
        );
    }

    #[test]
    fn validates_syndrome_length() {
        let mut group =
            StabilizerGroup::new(2)
                .unwrap();

        group
            .add_generator(
                StabilizerGenerator::new(
                    0,
                    PauliString::from_paulis(
                        &[
                            Pauli::Z,
                            Pauli::Z,
                        ],
                    ),
                )
                .unwrap(),
            )
            .unwrap();

        let syndrome =
            Syndrome::new(vec![
                true,
                false,
            ]);

        assert!(matches!(
            validate_syndrome(
                &syndrome,
                &group,
            ),
            Err(
                DecoderError::SyndromeLengthMismatch {
                    expected: 1,
                    actual: 2
                }
            )
        ));
    }

    #[test]
    fn single_qubit_x_error_is_constructed() {
        let error =
            x_error(3, 1)
                .unwrap();

        assert_eq!(
            error
                .pauli_at(
                    super::super::stabilizer::QubitIndex::new(
                        0,
                    ),
                )
                .unwrap(),
            Pauli::I
        );

        assert_eq!(
            error
                .pauli_at(
                    super::super::stabilizer::QubitIndex::new(
                        1,
                    ),
                )
                .unwrap(),
            Pauli::X
        );
    }

    #[test]
    fn correction_matches_syndrome() {
        let mut group =
            StabilizerGroup::new(1)
                .unwrap();

        group
            .add_generator(
                StabilizerGenerator::new(
                    0,
                    PauliString::from_paulis(
                        &[Pauli::Z],
                    ),
                )
                .unwrap(),
            )
            .unwrap();

        let error =
            x_error(1, 0)
                .unwrap();

        let syndrome =
            group
                .syndrome(&error)
                .unwrap();

        let correction =
            Correction::new(
                error,
            );

        assert!(
            validate_correction_for_syndrome(
                &correction,
                &syndrome,
                &group,
            )
            .unwrap()
        );
    }

    #[test]
    fn registry_rejects_duplicate_decoder() {
        let decoder =
            IdentityDecoder::new(
                DecoderId::new(1),
                2,
            )
            .unwrap();

        let mut registry =
            DecoderRegistry::new();

        registry
            .register(&decoder)
            .unwrap();

        assert!(matches!(
            registry.register(
                &decoder
            ),
            Err(
                DecoderError::DuplicateDecoder {
                    id: DecoderId(1)
                }
            )
        ));
    }

    #[test]
    fn statistics_track_results() {
        let decoder =
            IdentityDecoder::new(
                DecoderId::new(2),
                2,
            )
            .unwrap();

        let syndrome =
            Syndrome::new(vec![
                false,
            ]);

        let result =
            decoder.decode(
                &syndrome,
            );

        let mut stats =
            DecoderStatistics::new();

        stats.record(
            result.as_ref()
        );

        assert_eq!(
            stats.decoded(),
            1
        );

        assert_eq!(
            stats.trivial(),
            1
        );
    }
}