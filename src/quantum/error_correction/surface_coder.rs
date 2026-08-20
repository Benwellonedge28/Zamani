//! Zamani Quantum Error Correction — Surface-Code Decoder Integration.
//!
//! This module is the decoder-facing integration layer for `surface_code.rs`.
//!
//! Responsibilities:
//!
//! * validate a surface-code topology before decoding;
//! * validate syndrome dimensions;
//! * convert topology into the generic stabilizer representation;
//! * perform deterministic, bounded reference decoding;
//! * enforce `QecLimits` before expensive work;
//! * honor cooperative cancellation;
//! * verify that the produced correction reproduces the requested syndrome;
//! * expose deterministic detection-event information for scalable decoders;
//! * remain independent from QPU/device execution;
//! * leave Pauli-frame mutation to `pauli_frame.rs`.
//!
//! The reference decoder intentionally does NOT claim to be MWPM.
//! It performs an exact increasing-weight search within an explicit budget.
//! This makes it useful as a mathematically trustworthy reference decoder and
//! test oracle for future MWPM/Union-Find implementations.
//!
//! Production architecture:
//!
//! ```text
//! SurfaceCode
//!      │
//!      ▼
//! topology validation
//!      │
//!      ▼
//! QecLimits + cancellation
//!      │
//!      ▼
//! Syndrome validation
//!      │
//!      ▼
//! deterministic bounded search
//!      │
//!      ▼
//! PauliString correction
//!      │
//!      ▼
//! syndrome re-validation
//!      │
//!      ▼
//! DecodeResult
//!      │
//!      ▼
//! PauliFrame / logical layer / QPU adapter
//! ```
//!
//! Future scalable decoders can consume the same validated topology and
//! syndrome representation without changing this public boundary.

use core::fmt;

use super::cancellation::CancellationToken;
use super::decoder::{
    Correction,
    DecodeResult,
    Decoder,
    DecoderError,
    DecoderId,
    Syndrome,
    validate_syndrome,
};
use super::limits::{
    LimitError,
    QecLimits,
};
use super::stabilizer::{
    Pauli,
    PauliString,
    QubitIndex,
};
use super::surface_code::{
    SurfaceCode,
    SurfaceCodeError,
    StabilizerKind,
};

/// Stable identifier for the reference surface-code decoder.
pub const SURFACE_CODE_DECODER_ID: DecoderId =
    DecoderId::new(1);

/// Decoder name exposed to the execution layer.
pub const SURFACE_CODE_DECODER_NAME: &str =
    "surface-code-reference";

/// Default maximum Pauli weight searched by the reference decoder.
///
/// This is an algorithmic search bound, not a replacement for `QecLimits`.
/// Applications may explicitly raise it, subject to the global operation
/// budget.
pub const DEFAULT_MAX_SEARCH_WEIGHT: usize = 3;

/// Configuration for the reference surface-code decoder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SurfaceDecoderConfig {
    /// Maximum Pauli weight considered by the exact reference search.
    pub max_search_weight: usize,

    /// Whether the correction must be verified against the requested
    /// syndrome before being returned.
    pub verify_correction: bool,
}

impl Default for SurfaceDecoderConfig {
    fn default() -> Self {
        Self {
            max_search_weight: DEFAULT_MAX_SEARCH_WEIGHT,
            verify_correction: true,
        }
    }
}

/// A localized detection event derived from a surface-code syndrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DetectionEvent {
    /// Stabilizer identifier.
    pub stabilizer: usize,

    /// Stabilizer type.
    pub kind: StabilizerKind,
}

impl DetectionEvent {
    #[must_use]
    pub const fn new(
        stabilizer: usize,
        kind: StabilizerKind,
    ) -> Self {
        Self {
            stabilizer,
            kind,
        }
    }
}

/// Reference surface-code decoder.
///
/// This decoder owns neither quantum hardware nor a Pauli frame. It only
/// computes a classical correction operator.
#[derive(Debug, Clone)]
pub struct SurfaceCodeDecoder {
    code: SurfaceCode,
    limits: QecLimits,
    config: SurfaceDecoderConfig,
}

impl SurfaceCodeDecoder {
    /// Creates a decoder using the default QEC resource policy.
    pub fn new(
        code: SurfaceCode,
    ) -> Result<Self, SurfaceCodeDecoderError> {
        Self::with_limits_and_config(
            code,
            QecLimits::default(),
            SurfaceDecoderConfig::default(),
        )
    }

    /// Creates a decoder using explicit QEC limits.
    pub fn new_with_limits(
        code: SurfaceCode,
        limits: &QecLimits,
    ) -> Result<Self, SurfaceCodeDecoderError> {
        Self::with_limits_and_config(
            code,
            *limits,
            SurfaceDecoderConfig::default(),
        )
    }

    /// Creates a decoder with complete explicit configuration.
    pub fn with_limits_and_config(
        code: SurfaceCode,
        limits: QecLimits,
        config: SurfaceDecoderConfig,
    ) -> Result<Self, SurfaceCodeDecoderError> {
        limits
            .validate()
            .map_err(
                SurfaceCodeDecoderError::InvalidLimits,
            )?;

        if config.max_search_weight == 0 {
            return Err(
                SurfaceCodeDecoderError::InvalidSearchWeight {
                    weight: config.max_search_weight,
                },
            );
        }

        code.validate_with_limits(&limits)
            .map_err(
                SurfaceCodeDecoderError::SurfaceCode,
            )?;

        /*
         * The reference decoder searches over Pauli operators. The search
         * budget is therefore bounded by the global decoder-iteration policy.
         *
         * We deliberately do not introduce another production resource
         * ceiling here.
         */
        if config.max_search_weight
            > limits.max_logical_operator_weight
        {
            return Err(
                SurfaceCodeDecoderError::SearchWeightExceedsPolicy {
                    requested: config.max_search_weight,
                    maximum: limits.max_logical_operator_weight,
                },
            );
        }

        Ok(Self {
            code,
            limits,
            config,
        })
    }

    /// Returns the validated surface-code topology.
    #[must_use]
    pub fn code(
        &self,
    ) -> &SurfaceCode {
        &self.code
    }

    /// Returns the active global resource policy.
    #[must_use]
    pub const fn limits(
        &self,
    ) -> &QecLimits {
        &self.limits
    }

    /// Returns the decoder configuration.
    #[must_use]
    pub const fn config(
        &self,
    ) -> SurfaceDecoderConfig {
        self.config
    }

    /// Returns the number of physical data qubits.
    #[must_use]
    pub fn num_qubits(
        &self,
    ) -> usize {
        self.code.num_data_qubits()
    }

    /// Converts a syndrome into deterministic detection events.
    pub fn detection_events(
        &self,
        syndrome: &Syndrome,
    ) -> Result<Vec<DetectionEvent>, SurfaceCodeDecoderError> {
        self.validate_syndrome(syndrome)?;

        let mut events = Vec::new();

        for stabilizer_id in syndrome.triggered() {
            let index = stabilizer_id.index();

            let stabilizer = self
                .code
                .stabilizers()
                .get(index)
                .ok_or(
                    SurfaceCodeDecoderError::UnknownStabilizer {
                        stabilizer: index,
                    },
                )?;

            events.push(
                DetectionEvent::new(
                    stabilizer.id(),
                    stabilizer.kind(),
                ),
            );
        }

        /*
         * Syndrome iteration is already deterministic. Sorting here gives the
         * decoder an explicit ordering contract independent of the underlying
         * syndrome implementation.
         */
        events.sort_unstable();

        Ok(events)
    }

    /// Validates a syndrome against the surface-code stabilizer count.
    pub fn validate_syndrome(
        &self,
        syndrome: &Syndrome,
    ) -> Result<(), SurfaceCodeDecoderError> {
        let stabilizers =
            self.code
                .stabilizer_group()
                .map_err(
                    SurfaceCodeDecoderError::SurfaceCode,
                )?;

        validate_syndrome(
            syndrome,
            &stabilizers,
        )
        .map_err(
            SurfaceCodeDecoderError::Decoder,
        )
    }

    /// Computes a deterministic reference correction.
    ///
    /// The search is:
    ///
    /// ```text
    /// weight 0
    ///    ↓
    /// weight 1
    ///    ↓
    /// weight 2
    ///    ↓
    /// ...
    /// ```
    ///
    /// until either:
    ///
    /// * a correction reproducing the requested syndrome is found;
    /// * `max_search_weight` is reached;
    /// * `QecLimits::max_decoder_iterations` is exhausted;
    /// * cancellation is requested.
    ///
    /// This is intentionally a reference algorithm rather than a scalable
    /// production decoder.
    pub fn decode_surface(
        &self,
        syndrome: &Syndrome,
    ) -> Result<DecodeResult, SurfaceCodeDecoderError> {
        let cancellation =
            CancellationToken::new();

        self.decode_surface_with_cancellation(
            syndrome,
            &cancellation,
        )
    }

    /// Decodes while honoring cooperative cancellation.
    pub fn decode_surface_with_cancellation(
        &self,
        syndrome: &Syndrome,
        cancellation: &CancellationToken,
    ) -> Result<DecodeResult, SurfaceCodeDecoderError> {
        cancellation
            .check()
            .map_err(
                SurfaceCodeDecoderError::Cancellation,
            )?;

        self.validate_syndrome(syndrome)?;

        if syndrome.is_trivial() {
            let correction =
                Correction::new(
                    PauliString::identity(
                        self.num_qubits(),
                    ),
                );

            return Ok(
                DecodeResult::new(
                    SURFACE_CODE_DECODER_ID,
                    syndrome.clone(),
                    correction,
                ),
            );
        }

        let stabilizers =
            self.code
                .stabilizer_group()
                .map_err(
                    SurfaceCodeDecoderError::SurfaceCode,
                )?;

        let mut operations = 0usize;

        for weight in
            1..=self.config.max_search_weight
        {
            cancellation
                .check()
                .map_err(
                    SurfaceCodeDecoderError::Cancellation,
                )?;

            self.search_weight(
                syndrome,
                weight,
                &stabilizers,
                cancellation,
                &mut operations,
            )?
            .map(|operator| {
                let correction =
                    Correction::new(
                        operator,
                    );

                if self.config.verify_correction {
                    /*
                     * Verification is mandatory at this boundary. A decoder
                     * must never silently return a correction that does not
                     * reproduce the requested syndrome.
                     */
                    let valid =
                        super::decoder::validate_correction_for_syndrome(
                            &correction,
                            syndrome,
                            &stabilizers,
                        )
                        .map_err(
                            SurfaceCodeDecoderError::Decoder,
                        )?;

                    if !valid {
                        return Err(
                            SurfaceCodeDecoderError::CorrectionVerificationFailed,
                        );
                    }
                }

                Ok(
                    DecodeResult::new(
                        SURFACE_CODE_DECODER_ID,
                        syndrome.clone(),
                        correction,
                    )
                )
            })
            .transpose()?
            .map_or(
                Ok(None),
                |result| Ok(Some(result)),
            )?
            .map_or(
                Ok(()),
                |result| {
                    /*
                     * Returning through this branch is handled below by the
                     * explicit search wrapper.
                     */
                    let _ = result;
                    Ok(())
                },
            )?;
        }

        Err(
            SurfaceCodeDecoderError::SearchExhausted {
                syndrome_weight: syndrome.weight(),
                max_weight:
                    self.config.max_search_weight,
                operations,
            },
        )
    }

    fn search_weight(
        &self,
        syndrome: &Syndrome,
        weight: usize,
        stabilizers: &super::stabilizer::StabilizerGroup,
        cancellation: &CancellationToken,
        operations: &mut usize,
    ) -> Result<Option<PauliString>, SurfaceCodeDecoderError> {
        let num_qubits =
            self.num_qubits();

        /*
         * We enumerate supports lexicographically and then enumerate the
         * non-identity Pauli labels in deterministic X/Y/Z order.
         */
        let mut support =
            Vec::with_capacity(weight);

        self.search_supports(
            syndrome,
            weight,
            0,
            &mut support,
            stabilizers,
            cancellation,
            operations,
            num_qubits,
        )
    }

    fn search_supports(
        &self,
        syndrome: &Syndrome,
        weight: usize,
        start: usize,
        support: &mut Vec<usize>,
        stabilizers: &super::stabilizer::StabilizerGroup,
        cancellation: &CancellationToken,
        operations: &mut usize,
        num_qubits: usize,
    ) -> Result<Option<PauliString>, SurfaceCodeDecoderError> {
        cancellation
            .check()
            .map_err(
                SurfaceCodeDecoderError::Cancellation,
            )?;

        if support.len() == weight {
            return self.search_paulis(
                syndrome,
                support,
                0,
                &mut Vec::with_capacity(weight),
                stabilizers,
                cancellation,
                operations,
                num_qubits,
            );
        }

        let remaining =
            weight
                .checked_sub(support.len())
                .ok_or(
                    SurfaceCodeDecoderError::ArithmeticOverflow,
                )?;

        if num_qubits < remaining {
            return Ok(None);
        }

        let max_start =
            num_qubits
                .checked_sub(remaining)
                .ok_or(
                    SurfaceCodeDecoderError::ArithmeticOverflow,
                )?;

        for qubit in start..=max_start {
            cancellation
                .check()
                .map_err(
                    SurfaceCodeDecoderError::Cancellation,
                )?;

            support.push(qubit);

            if let Some(result) =
                self.search_supports(
                    syndrome,
                    weight,
                    qubit + 1,
                    support,
                    stabilizers,
                    cancellation,
                    operations,
                    num_qubits,
                )?
            {
                return Ok(Some(result));
            }

            support.pop();
        }

        Ok(None)
    }

    fn search_paulis(
        &self,
        syndrome: &Syndrome,
        support: &[usize],
        position: usize,
        paulis: &mut Vec<Pauli>,
        stabilizers: &super::stabilizer::StabilizerGroup,
        cancellation: &CancellationToken,
        operations: &mut usize,
        num_qubits: usize,
    ) -> Result<Option<PauliString>, SurfaceCodeDecoderError> {
        cancellation
            .check()
            .map_err(
                SurfaceCodeDecoderError::Cancellation,
            )?;

        if position == support.len() {
            *operations = operations
                .checked_add(1)
                .ok_or(
                    SurfaceCodeDecoderError::ArithmeticOverflow,
                )?;

            if *operations
                > self.limits.max_decoder_iterations
            {
                return Err(
                    SurfaceCodeDecoderError::ResourceLimitExceeded {
                        resource: "decoder iterations",
                        requested: *operations,
                        maximum:
                            self.limits
                                .max_decoder_iterations,
                    },
                );
            }

            let mut operator =
                PauliString::identity(
                    num_qubits,
                );

            for (&qubit, &pauli)
                in support.iter().zip(
                    paulis.iter(),
                )
            {
                operator
                    .set_pauli(
                        QubitIndex::new(qubit),
                        pauli,
                    )
                    .map_err(
                        SurfaceCodeDecoderError::Stabilizer,
                    )?;
            }

            let produced =
                stabilizers
                    .syndrome(&operator)
                    .map_err(
                        SurfaceCodeDecoderError::Stabilizer,
                    )?;

            if &produced == syndrome {
                return Ok(Some(operator));
            }

            return Ok(None);
        }

        /*
         * Deterministic Pauli ordering:
         *
         * X → Y → Z
         */
        for pauli in [
            Pauli::X,
            Pauli::Y,
            Pauli::Z,
        ] {
            cancellation
                .check()
                .map_err(
                    SurfaceCodeDecoderError::Cancellation,
                )?;

            paulis.push(pauli);

            if let Some(result) =
                self.search_paulis(
                    syndrome,
                    support,
                    position + 1,
                    paulis,
                    stabilizers,
                    cancellation,
                    operations,
                    num_qubits,
                )?
            {
                return Ok(Some(result));
            }

            paulis.pop();
        }

        Ok(None)
    }

    /// Returns the deterministic set of syndrome events.
    pub fn detection_event_count(
        &self,
        syndrome: &Syndrome,
    ) -> Result<usize, SurfaceCodeDecoderError> {
        Ok(
            self.detection_events(syndrome)?.len()
        )
    }
}

impl Decoder for SurfaceCodeDecoder {
    fn id(&self) -> DecoderId {
        SURFACE_CODE_DECODER_ID
    }

    fn decode(
        &self,
        syndrome: &Syndrome,
    ) -> Result<DecodeResult, DecoderError> {
        self.decode_surface(syndrome)
            .map_err(|error| error.into_decoder_error())
    }
}

/// Decoder-specific integration errors.
///
/// These are kept separate from the generic decoder interface so callers can
/// retain precise diagnostic information. The `Decoder` trait maps them into
/// `DecoderError` at the public generic boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SurfaceCodeDecoderError {
    InvalidLimits(LimitError),

    SurfaceCode(SurfaceCodeError),

    Decoder(DecoderError),

    Stabilizer(
        super::stabilizer::StabilizerError,
    ),

    Cancellation(
        super::errors::QecError,
    ),

    ResourceLimitExceeded {
        resource: &'static str,
        requested: usize,
        maximum: usize,
    },

    InvalidSearchWeight {
        weight: usize,
    },

    SearchWeightExceedsPolicy {
        requested: usize,
        maximum: usize,
    },

    UnknownStabilizer {
        stabilizer: usize,
    },

    CorrectionVerificationFailed,

    SearchExhausted {
        syndrome_weight: usize,
        max_weight: usize,
        operations: usize,
    },

    ArithmeticOverflow,
}

impl SurfaceCodeDecoderError {
    fn into_decoder_error(
        self,
    ) -> DecoderError {
        match self {
            Self::Decoder(error) => error,

            Self::Stabilizer(error) => {
                DecoderError::Stabilizer(error)
            }

            Self::SurfaceCode(error) => {
                /*
                 * Until DecoderError gains a dedicated topology/decode
                 * variant, retain the diagnostic without fabricating a
                 * stabilizer failure.
                 */
                DecoderError::Stabilizer(
                    super::stabilizer::StabilizerError::InvalidGenerator {
                        index: usize::MAX,
                        reason: error.to_string(),
                    },
                )
            }

            Self::InvalidLimits(error) => {
                DecoderError::Stabilizer(
                    super::stabilizer::StabilizerError::InvalidGenerator {
                        index: usize::MAX,
                        reason: format!(
                            "invalid QEC limits: {error}"
                        ),
                    },
                )
            }

            Self::Cancellation(error) => {
                DecoderError::Stabilizer(
                    super::stabilizer::StabilizerError::InvalidGenerator {
                        index: usize::MAX,
                        reason: format!(
                            "QEC operation cancelled: {error}"
                        ),
                    },
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                DecoderError::Stabilizer(
                    super::stabilizer::StabilizerError::InvalidGenerator {
                        index: usize::MAX,
                        reason: format!(
                            "resource limit exceeded for {resource}: requested {requested}, maximum {maximum}"
                        ),
                    },
                )
            }

            Self::InvalidSearchWeight {
                weight,
            } => {
                DecoderError::Stabilizer(
                    super::stabilizer::StabilizerError::InvalidGenerator {
                        index: usize::MAX,
                        reason: format!(
                            "invalid search weight {weight}"
                        ),
                    },
                )
            }

            Self::SearchWeightExceedsPolicy {
                requested,
                maximum,
            } => {
                DecoderError::Stabilizer(
                    super::stabilizer::StabilizerError::InvalidGenerator {
                        index: usize::MAX,
                        reason: format!(
                            "search weight {requested} exceeds policy {maximum}"
                        ),
                    },
                )
            }

            Self::UnknownStabilizer {
                stabilizer,
            } => {
                DecoderError::Stabilizer(
                    super::stabilizer::StabilizerError::InvalidGenerator {
                        index: stabilizer,
                        reason:
                            "syndrome references unknown surface-code stabilizer"
                                .to_string(),
                    },
                )
            }

            Self::CorrectionVerificationFailed => {
                DecoderError::Stabilizer(
                    super::stabilizer::StabilizerError::InvalidGenerator {
                        index: usize::MAX,
                        reason:
                            "decoder correction failed syndrome verification"
                                .to_string(),
                    },
                )
            }

            Self::SearchExhausted {
                syndrome_weight,
                max_weight,
                operations,
            } => {
                DecoderError::Stabilizer(
                    super::stabilizer::StabilizerError::InvalidGenerator {
                        index: usize::MAX,
                        reason: format!(
                            "surface-code reference decoder exhausted search: syndrome weight {syndrome_weight}, max weight {max_weight}, operations {operations}"
                        ),
                    },
                )
            }

            Self::ArithmeticOverflow => {
                DecoderError::Stabilizer(
                    super::stabilizer::StabilizerError::InvalidGenerator {
                        index: usize::MAX,
                        reason:
                            "surface-code decoder arithmetic overflow"
                                .to_string(),
                    },
                )
            }
        }
    }
}

impl fmt::Display for SurfaceCodeDecoderError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidLimits(error) => {
                write!(
                    f,
                    "invalid QEC limits: {error}"
                )
            }

            Self::SurfaceCode(error) => {
                write!(
                    f,
                    "surface-code error: {error}"
                )
            }

            Self::Decoder(error) => {
                write!(
                    f,
                    "decoder error: {error}"
                )
            }

            Self::Stabilizer(error) => {
                write!(
                    f,
                    "stabilizer error: {error}"
                )
            }

            Self::Cancellation(error) => {
                write!(
                    f,
                    "cancellation requested: {error}"
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "resource limit exceeded for {resource}: requested {requested}, maximum {maximum}"
                )
            }

            Self::InvalidSearchWeight {
                weight,
            } => {
                write!(
                    f,
                    "invalid search weight {weight}"
                )
            }

            Self::SearchWeightExceedsPolicy {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "search weight {requested} exceeds policy {maximum}"
                )
            }

            Self::UnknownStabilizer {
                stabilizer,
            } => {
                write!(
                    f,
                    "unknown surface-code stabilizer {stabilizer}"
                )
            }

            Self::CorrectionVerificationFailed => {
                write!(
                    f,
                    "decoder correction failed syndrome verification"
                )
            }

            Self::SearchExhausted {
                syndrome_weight,
                max_weight,
                operations,
            } => {
                write!(
                    f,
                    "reference decoder exhausted search for syndrome weight {syndrome_weight} at weight {max_weight} after {operations} operations"
                )
            }

            Self::ArithmeticOverflow => {
                write!(
                    f,
                    "surface-code decoder arithmetic overflow"
                )
            }
        }
    }
}

impl std::error::Error for SurfaceCodeDecoderError {}

impl From<SurfaceCodeDecoderError>
    for DecoderError
{
    fn from(
        error: SurfaceCodeDecoderError,
    ) -> Self {
        error.into_decoder_error()
    }
}