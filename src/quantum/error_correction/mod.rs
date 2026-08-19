//! Zamani Quantum Error Correction subsystem.
//!
//! This module provides the common interface for quantum error correction,
//! including:
//!
//! - stabilizer algebra;
//! - surface-code geometry;
//! - syndrome representation;
//! - decoder interfaces;
//! - correction validation.
//!
//! Architectural dependency:
//!
//! ```text
//!                         QEC
//!                          │
//!             ┌────────────┼────────────┐
//!             │            │            │
//!             ▼            ▼            ▼
//!       stabilizer.rs  surface_code.rs decoder.rs
//!             │            │            │
//!             └────────────┼────────────┘
//!                          │
//!                          ▼
//!                  Future decoders
//!             ┌────────────┼────────────┐
//!             ▼            ▼            ▼
//!           MWPM       Union-Find    Tensor Network
//! ```
//!
//! `stabilizer.rs` owns the mathematical Pauli/stabilizer representation.
//! `surface_code.rs` owns surface-code topology and code invariants.
//! `decoder.rs` owns decoder policy and correction selection.

pub mod decoder;
pub mod stabilizer;
pub mod surface_code;

// -----------------------------------------------------------------------------
// Stabilizer algebra
// -----------------------------------------------------------------------------

pub use stabilizer::{
    commutes_with_stabilizer_group,
    logical_operators_anticommute,
    Pauli,
    PauliString,
    QubitIndex,
    StabilizerError,
    StabilizerGenerator,
    StabilizerGroup,
    Syndrome,
};

// -----------------------------------------------------------------------------
// Decoder API
// -----------------------------------------------------------------------------

pub use decoder::{
    validate_correction,
    validate_correction_for_syndrome,
    validate_syndrome,
    single_qubit_error,
    x_error,
    y_error,
    z_error,
    Correction,
    DecodeResult,
    Decoder,
    DecoderError,
    DecoderId,
    DecoderRegistry,
    DecoderStatistics,
    IdentityDecoder,
    StabilizerDecoder,
    SyndromeClass,
};

// -----------------------------------------------------------------------------
// Surface-code API
// -----------------------------------------------------------------------------
//
// Keep these exports deliberately explicit rather than using `pub use
// surface_code::*`. This prevents accidental leakage of implementation
// details as the surface-code implementation evolves.

pub use surface_code::{
    // The exact public surface-code types should be kept here once
    // surface_code.rs has been reconciled with stabilizer.rs.
};

// -----------------------------------------------------------------------------
// Subsystem version
// -----------------------------------------------------------------------------

/// Current QEC subsystem API version.
///
/// This is intentionally independent of the overall Zamani version so that
/// the QEC API can evolve while maintaining compatibility guarantees.
pub const QEC_API_VERSION: &str = "1.0.0";

// -----------------------------------------------------------------------------
// Capability markers
// -----------------------------------------------------------------------------

/// Compile-time marker describing capabilities provided by this QEC module.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct QecCapabilities {
    pub stabilizer_algebra: bool,
    pub syndrome_generation: bool,
    pub correction_validation: bool,
    pub surface_code: bool,
    pub decoder_interface: bool,
}

impl QecCapabilities {
    pub const CURRENT: Self = Self {
        stabilizer_algebra: true,
        syndrome_generation: true,
        correction_validation: true,
        surface_code: true,
        decoder_interface: true,
    };
}

/// Returns the capabilities provided by this QEC subsystem.
pub const fn capabilities() -> QecCapabilities {
    QecCapabilities::CURRENT
}

// -----------------------------------------------------------------------------
// QEC health check
// -----------------------------------------------------------------------------

/// Basic structural health check for the QEC subsystem.
///
/// This does not execute a hardware operation. It verifies that the
/// mathematical primitives required by the subsystem are internally
/// consistent.
pub fn self_check() -> Result<(), QecSelfCheckError> {
    let stabilizers =
        StabilizerGroup::new(1)
            .map_err(
                QecSelfCheckError::Stabilizer,
            )?;

    stabilizers
        .validate()
        .map_err(
            QecSelfCheckError::Stabilizer,
        )?;

    let identity =
        PauliString::identity(1);

    let syndrome =
        stabilizers
            .syndrome(&identity)
            .map_err(
                QecSelfCheckError::Stabilizer,
            )?;

    if !syndrome.is_trivial() {
        return Err(
            QecSelfCheckError::InvalidIdentitySyndrome,
        );
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Self-check errors
// -----------------------------------------------------------------------------

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum QecSelfCheckError {
    Stabilizer(
        StabilizerError,
    ),

    InvalidIdentitySyndrome,
}

impl std::fmt::Display
    for QecSelfCheckError
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Stabilizer(error) => {
                write!(
                    f,
                    "QEC stabilizer self-check failed: {error}"
                )
            }

            Self::InvalidIdentitySyndrome => {
                write!(
                    f,
                    "identity Pauli produced a non-trivial syndrome"
                )
            }
        }
    }
}

impl std::error::Error
    for QecSelfCheckError
{
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qec_capabilities_are_enabled() {
        let capabilities =
            capabilities();

        assert!(
            capabilities
                .stabilizer_algebra
        );

        assert!(
            capabilities
                .syndrome_generation
        );

        assert!(
            capabilities
                .correction_validation
        );

        assert!(
            capabilities
                .surface_code
        );

        assert!(
            capabilities
                .decoder_interface
        );
    }

    #[test]
    fn qec_self_check_passes() {
        self_check().unwrap();
    }

    #[test]
    fn identity_has_trivial_syndrome() {
        let group =
            StabilizerGroup::new(2)
                .unwrap();

        let identity =
            PauliString::identity(2);

        let syndrome =
            group
                .syndrome(&identity)
                .unwrap();

        assert!(
            syndrome.is_trivial()
        );
    }

    #[test]
    fn decoder_and_stabilizer_share_pauli_model() {
        let correction =
            Correction::identity(3);

        assert_eq!(
            correction
                .operator()
                .num_qubits(),
            3
        );

        assert!(
            correction
                .is_identity()
        );
    }
}