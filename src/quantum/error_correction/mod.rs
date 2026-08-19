//! Zamani Quantum Error Correction subsystem.
//!
//! Production fault-tolerance architecture:
//!
//! ```text
//! Quantum Circuit
//!       |
//!       v
//! Fault / Noise Model
//!       |
//!       v
//! Syndrome Extraction
//!       |
//!       v
//! Syndrome Stream
//!       |
//!       v
//! Detection Events
//!       |
//!       v
//! Decoding Graph
//!       |
//!   +---+-----------+
//!   |               |
//!   v               v
//!  MWPM        Union-Find
//!   |               |
//!   +-------+-------+
//!           |
//!           v
//!      Pauli Frame
//!           |
//!           v
//!   Logical Operators
//!           |
//!           v
//!    Logical Outcome
//! ```
//!
//! # Module responsibilities
//!
//! * [`decoder`] — decoder abstraction, policies, validation and results.
//! * [`decoding_graph`] — space-time decoding graph representation.
//! * [`logical`] — logical operators and logical-error classification.
//! * [`mwpm`] — minimum-weight perfect-matching decoder.
//! * [`noise`] — physical and measurement noise models.
//! * [`pauli_frame`] — deferred Pauli correction tracking.
//! * [`simulation`] — QEC simulation and experiment infrastructure.
//! * [`stabilizer`] — Pauli/stabilizer algebra.
//! * [`surface_code`] — surface-code topology and invariants.
//! * [`surface_coder`] — surface-code construction/coding operations.
//! * [`syndrome`] — repeated-round syndrome and detection-event handling.
//! * [`union_find`] — Union-Find decoding implementation.
//!
//! The QEC subsystem deliberately separates:
//!
//! ```text
//! physical fault generation
//!          !=
//! syndrome extraction
//!          !=
//! decoding
//!          !=
//! correction representation
//!          !=
//! logical-error classification
//! ```
//!
//! This prevents decoder implementations from becoming coupled to a
//! particular noise model or hardware backend.
//!
//! # Robustness requirements
//!
//! Public QEC APIs are expected to:
//!
//! * validate externally supplied dimensions and indices;
//! * reject malformed topology;
//! * avoid unchecked indexing;
//! * avoid panic-based validation;
//! * use checked arithmetic for resource-sensitive calculations;
//! * remain deterministic for deterministic inputs;
//! * enforce implementation-specific resource limits;
//! * return structured errors instead of silently accepting invalid data.
//!
//! # Test architecture
//!
//! Integration-style QEC tests live under:
//!
//! ```text
//! quantum/error_correction/tests/
//! ```
//!
//! They are intentionally kept separate from the production implementation.
//!
//! # API stability
//!
//! The QEC API version is independent of the overall Zamani version.

#![deny(unsafe_op_in_unsafe_fn)]

pub mod decoder;
pub mod decoding_graph;
pub mod logical;
pub mod mwpm;
pub mod noise;
pub mod pauli_frame;
pub mod simulation;
pub mod stabilizer;
pub mod surface_code;
pub mod surface_coder;
pub mod syndrome;
pub mod union_find;

// -----------------------------------------------------------------------------
// Test module
// -----------------------------------------------------------------------------
//
// `tests/` must contain its own `mod.rs`:
//
//     src/quantum/error_correction/tests/mod.rs
//
// This keeps test-only code out of production builds.

#[cfg(test)]
mod tests;

// -----------------------------------------------------------------------------
// Stable high-level API
// -----------------------------------------------------------------------------
//
// Only stable subsystem-level concepts should be re-exported here.
// Implementation-specific types should remain available through their
// respective modules.

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

pub use stabilizer::{
    commutes_with_stabilizer_group,
    logical_operators_anticommute,
    Pauli,
    PauliString,
    QubitIndex,
    StabilizerError,
    StabilizerGenerator,
    StabilizerGroup,
    Syndrome as StabilizerSyndrome,
};

// -----------------------------------------------------------------------------
// Subsystem metadata
// -----------------------------------------------------------------------------

/// Public QEC API version.
///
/// This version tracks compatibility of the QEC subsystem rather than the
/// complete Zamani project.
pub const QEC_API_VERSION: &str = "1.0.0";

/// Capabilities currently exposed by the QEC subsystem.
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
    pub decoding_graph: bool,
    pub mwpm: bool,
    pub union_find: bool,
    pub noise_models: bool,
    pub pauli_frame: bool,
    pub logical_operators: bool,
    pub simulation: bool,
    pub surface_code: bool,
    pub decoder_interface: bool,
}

impl QecCapabilities {
    /// Current compiled subsystem capabilities.
    pub const CURRENT: Self = Self {
        stabilizer_algebra: true,
        syndrome_generation: true,
        decoding_graph: true,
        mwpm: true,
        union_find: true,
        noise_models: true,
        pauli_frame: true,
        logical_operators: true,
        simulation: true,
        surface_code: true,
        decoder_interface: true,
    };
}

/// Returns the capabilities exposed by this QEC subsystem.
pub const fn capabilities() -> QecCapabilities {
    QecCapabilities::CURRENT
}

// -----------------------------------------------------------------------------
// Structural health check
// -----------------------------------------------------------------------------

/// Performs a lightweight deterministic QEC self-check.
///
/// This validates the most fundamental stabilizer invariant:
///
/// ```text
/// identity error
///       ↓
/// stabilizer syndrome
///       ↓
/// trivial syndrome
/// ```
///
/// It does not perform hardware access, decoding, simulation, or benchmarking.
pub fn self_check() -> Result<(), QecSelfCheckError> {
    let stabilizers =
        StabilizerGroup::new(1)
            .map_err(QecSelfCheckError::Stabilizer)?;

    stabilizers
        .validate()
        .map_err(QecSelfCheckError::Stabilizer)?;

    let identity =
        PauliString::identity(1);

    let syndrome =
        stabilizers
            .syndrome(&identity)
            .map_err(QecSelfCheckError::Stabilizer)?;

    if !syndrome.is_trivial() {
        return Err(
            QecSelfCheckError::InvalidIdentitySyndrome,
        );
    }

    Ok(())
}

/// Errors returned by [`self_check`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum QecSelfCheckError {
    Stabilizer(StabilizerError),
    InvalidIdentitySyndrome,
}

impl std::fmt::Display for QecSelfCheckError {
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

impl std::error::Error for QecSelfCheckError {}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn capabilities_are_consistent() {
        let capabilities =
            capabilities();

        assert!(
            capabilities.stabilizer_algebra
        );

        assert!(
            capabilities.syndrome_generation
        );

        assert!(
            capabilities.decoding_graph
        );

        assert!(
            capabilities.mwpm
        );

        assert!(
            capabilities.union_find
        );

        assert!(
            capabilities.noise_models
        );

        assert!(
            capabilities.pauli_frame
        );

        assert!(
            capabilities.logical_operators
        );

        assert!(
            capabilities.simulation
        );

        assert!(
            capabilities.surface_code
        );

        assert!(
            capabilities.decoder_interface
        );
    }

    #[test]
    fn self_check_passes() {
        assert!(
            self_check().is_ok()
        );
    }

    #[test]
    fn identity_has_trivial_syndrome() {
        let group =
            StabilizerGroup::new(2)
                .expect("valid stabilizer group");

        let identity =
            PauliString::identity(2);

        let syndrome =
            group
                .syndrome(&identity)
                .expect("identity syndrome");

        assert!(
            syndrome.is_trivial()
        );
    }

    #[test]
    fn correction_uses_shared_pauli_model() {
        let correction =
            Correction::identity(3);

        assert_eq!(
            correction
                .operator()
                .num_qubits(),
            3
        );

        assert!(
            correction.is_identity()
        );
    }
}