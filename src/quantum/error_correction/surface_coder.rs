//! Zamani Quantum Error Correction — Surface-Code Decoder Integration.
//!
//! # Responsibility
//!
//! `surface_coder.rs` is the decoder-facing integration layer for
//! `surface_code.rs`.
//!
//! It owns:
//!
//! - validated integration between `SurfaceCode` and the generic decoder API;
//! - deterministic reference decoding;
//! - bounded exact Pauli search;
//! - syndrome validation;
//! - detection-event extraction;
//! - cancellation;
//! - decoder-iteration accounting;
//! - correction verification;
//! - conversion to the canonical `DecodeResult`;
//! - decoder identity and configuration.
//!
//! It does NOT own:
//!
//! - surface-code topology construction;
//! - QPU execution;
//! - circuit execution;
//! - Pauli-frame mutation;
//! - distributed execution;
//! - streaming execution;
//! - MWPM;
//! - Union-Find;
//! - global QEC resource-policy definition.
//!
//! `surface_code.rs` owns the mathematical topology.
//! `decoder.rs` owns the generic decoder contract.
//! `limits.rs` owns global resource policy.
//! `cancellation.rs` owns cooperative cancellation.
//! `stabilizer.rs` owns Pauli/stabilizer mathematics.
//!
//! # Architecture
//!
//! ```text
//! SurfaceCode
//!     │
//!     ▼
//! topology validation
//!     │
//!     ▼
//! QecLimits admission
//!     │
//!     ▼
//! syndrome validation
//!     │
//!     ▼
//! deterministic bounded reference search
//!     │
//!     ▼
//! PauliString
//!     │
//!     ▼
//! syndrome verification
//!     │
//!     ▼
//! DecodeResult
//!     │
//!     ├──► PauliFrame
//!     ├──► logical-equivalence layer
//!     ├──► streaming layer
//!     └──► future scalable decoders
//! ```
//!
//! This implementation is deliberately a **reference decoder**.
//! It must never claim scalable performance comparable to MWPM or
//! Union-Find.

use core::fmt;

use super::cancellation::CancellationToken;
use super::decoder::{
    validate_syndrome,
    Correction,
    DecodeResult,
    Decoder,
    DecoderError,
    DecoderId,
    Syndrome,
};
use super::limits::{
    LimitError,
    QecLimits,
};
use super::stabilizer::{
    Pauli,
    PauliString,
    QubitIndex,
    StabilizerGroup,
};
use super::surface_code::{
    StabilizerKind,
    SurfaceCode,
    SurfaceCodeError,
};

/// Stable decoder identity for the reference surface-code decoder.
pub const SURFACE_CODE_DECODER_ID: DecoderId =
    DecoderId::new(1);

/// Human-readable decoder identity.
pub const SURFACE_CODE_DECODER_NAME: &str =
    "surface-code-reference";

/// Default maximum Pauli weight explored by the reference decoder.
///
/// This is an algorithmic bound. It is still subordinate to `QecLimits`.
pub const DEFAULT_MAX_SEARCH_WEIGHT: usize = 3;

/// Configuration of the reference surface-code decoder.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct SurfaceDecoderConfig {
    /// Maximum Pauli weight searched.
    pub max_search_weight: usize,

    /// Whether a found correction must be verified before return.
    pub verify_correction: bool,
}

impl Default for SurfaceDecoderConfig {
    fn default() -> Self {
        Self {
            max_search_weight:
                DEFAULT_MAX_SEARCH_WEIGHT,
            verify_correction: true,
        }
    }
}

impl SurfaceDecoderConfig {
    /// Creates the default reference configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_search_weight:
                DEFAULT_MAX_SEARCH_WEIGHT,
            verify_correction: true,
        }
    }

    /// Creates a configuration with a specified search weight.
    #[must_use]
    pub const fn with_max_search_weight(
        max_search_weight: usize,
    ) -> Self {
        Self {
            max_search_weight,
            verify_correction: true,
        }
    }

    /// Enables or disables final correction verification.
    #[must_use]
    pub const fn with_verification(
        mut self,
        verify_correction: bool,
    ) -> Self {
        self.verify_correction =
            verify_correction;
        self
    }

    /// Validates the algorithm configuration.
    pub fn validate(
        &self,
    ) -> Result<
        (),
        SurfaceCodeDecoderError,
    > {
        if self.max_search_weight == 0 {
            return Err(
                SurfaceCodeDecoderError::InvalidSearchWeight {
                    weight: 0,
                },
            );
        }

        Ok(())
    }
}

/// A deterministic detection event.
///
/// Detection events are derived from triggered stabilizers and contain only
/// decoder-relevant topology information.
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
pub struct DetectionEvent {
    /// Stable stabilizer identifier.
    pub stabilizer: usize,

    /// Stabilizer type.
    pub kind: StabilizerKind,
}

impl DetectionEvent {
    /// Creates a detection event.
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

/// Resource accounting produced by the reference search.
///
/// This structure is intentionally local to the decoder integration layer.
/// Global resource policy remains owned by `QecLimits`.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
)]
pub struct SurfaceDecodeUsage {
    /// Number of complete Pauli candidates evaluated.
    pub iterations: usize,

    /// Maximum Pauli weight reached.
    pub maximum_weight_reached: usize,
}

impl SurfaceDecodeUsage {
    /// Creates empty usage statistics.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            iterations: 0,
            maximum_weight_reached: 0,
        }
    }
}

/// Reference surface-code decoder.
///
/// The decoder owns a validated topology snapshot and the policy required
/// for its reference search.
///
/// It does not own hardware, execution, or a Pauli frame.
#[derive(Debug, Clone)]
pub struct SurfaceCodeDecoder {
    code: SurfaceCode,
    limits: QecLimits,
    config: SurfaceDecoderConfig,
}

impl SurfaceCodeDecoder {
    /// Creates a reference decoder with the default QEC policy.
    pub fn new(
        code: SurfaceCode,
    ) -> Result<
        Self,
        SurfaceCodeDecoderError,
    > {
        Self::with_limits_and_config(
            code,
            QecLimits::default(),
            SurfaceDecoderConfig::default(),
        )
    }

    /// Creates a decoder with an explicit QEC policy.
    pub fn new_with_limits(
        code: SurfaceCode,
        limits: &QecLimits,
    ) -> Result<
        Self,
        SurfaceCodeDecoderError,
    > {
        Self::with_limits_and_config(
            code,
            *limits,
            SurfaceDecoderConfig::default(),
        )
    }

    /// Creates a decoder with explicit policy and algorithm configuration.
    pub fn with_limits_and_config(
        code: SurfaceCode,
        limits: QecLimits,
        config: SurfaceDecoderConfig,
    ) -> Result<
        Self,
        SurfaceCodeDecoderError,
    > {
        config.validate()?;

        limits
            .validate()
            .map_err(
                SurfaceCodeDecoderError::InvalidLimits,
            )?;

        code.validate_with_limits(&limits)
            .map_err(
                SurfaceCodeDecoderError::SurfaceCode,
            )?;

        /*
         * The decoder must never introduce an independent resource policy.
         *
         * The search weight is therefore checked against the global logical
         * operator weight policy.
         */
        if config.max_search_weight
            > limits.max_logical_operator_weight
        {
            return Err(
                SurfaceCodeDecoderError::
                    SearchWeightExceedsPolicy {
                        requested:
                            config.max_search_weight,
                        maximum:
                            limits
                                .max_logical_operator_weight,
                    },
            );
        }

        /*
         * A search weight greater than the number of physical qubits can
         * never produce a distinct Pauli support. Rejecting it here also
         * prevents unnecessary recursive search.
         */
        if config.max_search_weight
            > code.num_data_qubits()
        {
            return Err(
                SurfaceCodeDecoderError::
                    SearchWeightExceedsQubitCount {
                        requested:
                            config.max_search_weight,
                        num_qubits:
                            code.num_data_qubits(),
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

    /// Returns the global QEC policy used by this decoder.
    #[must_use]
    pub const fn limits(
        &self,
    ) -> &QecLimits {
        &self.limits
    }

    /// Returns the decoder-specific algorithm configuration.
    #[must_use]
    pub const fn config(
        &self,
    ) -> SurfaceDecoderConfig {
        self.config
    }

    /// Returns the number of data qubits.
    #[must_use]
    pub fn num_qubits(
        &self,
    ) -> usize {
        self.code.num_data_qubits()
    }

    /// Returns the decoder identity.
    #[must_use]
    pub const fn decoder_id() -> DecoderId {
        SURFACE_CODE_DECODER_ID
    }

    /// Returns the decoder name.
    #[must_use]
    pub const fn decoder_name() -> &'static str {
        SURFACE_CODE_DECODER_NAME
    }

    /// Converts a validated syndrome into deterministic detection events.
    pub fn detection_events(
        &self,
        syndrome: &Syndrome,
    ) -> Result<
        Vec<DetectionEvent>,
        SurfaceCodeDecoderError,
    > {
        self.validate_syndrome(syndrome)?;

        let mut events =
            Vec::new();

        for stabilizer_id in
            syndrome.triggered()
        {
            let index =
                stabilizer_id.index();

            let stabilizer = self
                .code
                .stabilizers()
                .get(index)
                .ok_or(
                    SurfaceCodeDecoderError::
                        UnknownStabilizer {
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
         * The decoder's observable ordering contract is explicit rather than
         * relying on the implementation details of Syndrome::triggered().
         */
        events.sort_unstable();

        Ok(events)
    }

    /// Validates a syndrome against the code's stabilizer group.
    pub fn validate_syndrome(
        &self,
        syndrome: &Syndrome,
    ) -> Result<
        (),
        SurfaceCodeDecoderError,
    > {
        let stabilizers = self
            .code
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

    /// Performs reference decoding with a fresh cancellation token.
    pub fn decode_surface(
        &self,
        syndrome: &Syndrome,
    ) -> Result<
        DecodeResult,
        SurfaceCodeDecoderError,
    > {
        let cancellation =
            CancellationToken::new();

        self.decode_surface_with_cancellation(
            syndrome,
            &cancellation,
        )
    }

    /// Performs bounded reference decoding with cooperative cancellation.
    ///
    /// Search order:
    ///
    /// ```text
    /// weight 0
    ///     ↓
    /// weight 1
    ///     ↓
    /// weight 2
    ///     ↓
    /// ...
    /// ```
    ///
    /// The first correction found is therefore the first correction under
    /// the decoder's deterministic lexicographic ordering.
    pub fn decode_surface_with_cancellation(
        &self,
        syndrome: &Syndrome,
        cancellation: &CancellationToken,
    ) -> Result<
        DecodeResult,
        SurfaceCodeDecoderError,
    > {
        cancellation
            .check()
            .map_err(
                SurfaceCodeDecoderError::Cancellation,
            )?;

        self.validate_syndrome(syndrome)?;

        /*
         * The zero-syndrome case is handled without entering the search.
         */
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

        let stabilizers = self
            .code
            .stabilizer_group()
            .map_err(
                SurfaceCodeDecoderError::SurfaceCode,
            )?;

        let mut usage =
            SurfaceDecodeUsage::new();

        for weight in
            1..=self.config.max_search_weight
        {
            cancellation
                .check()
                .map_err(
                    SurfaceCodeDecoderError::Cancellation,
                )?;

            usage.maximum_weight_reached =
                weight;

            if let Some(operator) =
                self.search_weight(
                    syndrome,
                    weight,
                    &stabilizers,
                    cancellation,
                    &mut usage,
                )?
            {
                let correction =
                    Correction::new(
                        operator,
                    );

                if self.config.verify_correction {
                    self.verify_correction(
                        &correction,
                        syndrome,
                        &stabilizers,
                    )?;
                }

                let result =
                    DecodeResult::new(
                        SURFACE_CODE_DECODER_ID,
                        syndrome.clone(),
                        correction,
                    );

                /*
                 * Validate the complete result at the public boundary.
                 * This prevents a mathematically valid correction from
                 * becoming an invalid decoder result.
                 */
                super::decoder::validate_result(
                    &result,
                )
                .map_err(
                    SurfaceCodeDecoderError::Decoder,
                )?;

                return Ok(result);
            }
        }

        Err(
            SurfaceCodeDecoderError::SearchExhausted {
                syndrome_weight:
                    syndrome.weight(),
                max_weight:
                    self.config.max_search_weight,
                operations:
                    usage.iterations,
            },
        )
    }

    /// Verifies that a correction reproduces the requested syndrome.
    fn verify_correction(
        &self,
        correction: &Correction,
        syndrome: &Syndrome,
        stabilizers: &StabilizerGroup,
    ) -> Result<
        (),
        SurfaceCodeDecoderError,
    > {
        let valid =
            super::decoder::
                validate_correction_for_syndrome(
                    correction,
                    syndrome,
                    stabilizers,
                )
                .map_err(
                    SurfaceCodeDecoderError::Decoder,
                )?;

        if !valid {
            return Err(
                SurfaceCodeDecoderError::
                    CorrectionVerificationFailed,
            );
        }

        Ok(())
    }

    /// Searches all supports of one Pauli weight.
    fn search_weight(
        &self,
        syndrome: &Syndrome,
        weight: usize,
        stabilizers: &StabilizerGroup,
        cancellation: &CancellationToken,
        usage: &mut SurfaceDecodeUsage,
    ) -> Result<
        Option<PauliString>,
        SurfaceCodeDecoderError,
    > {
        if weight == 0 {
            return Ok(None);
        }

        let num_qubits =
            self.num_qubits();

        if weight > num_qubits {
            return Ok(None);
        }

        let mut support =
            Vec::with_capacity(weight);

        let mut paulis =
            Vec::with_capacity(weight);

        self.search_supports(
            syndrome,
            weight,
            0,
            &mut support,
            &mut paulis,
            stabilizers,
            cancellation,
            usage,
            num_qubits,
        )
    }

    /// Enumerates supports in lexicographic order.
    fn search_supports(
        &self,
        syndrome: &Syndrome,
        weight: usize,
        start: usize,
        support: &mut Vec<usize>,
        paulis: &mut Vec<Pauli>,
        stabilizers: &StabilizerGroup,
        cancellation: &CancellationToken,
        usage: &mut SurfaceDecodeUsage,
        num_qubits: usize,
    ) -> Result<
        Option<PauliString>,
        SurfaceCodeDecoderError,
    > {
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
                paulis,
                stabilizers,
                cancellation,
                usage,
                num_qubits,
            );
        }

        let remaining =
            weight
                .checked_sub(
                    support.len(),
                )
                .ok_or(
                    SurfaceCodeDecoderError::
                        ArithmeticOverflow,
                )?;

        if num_qubits < remaining {
            return Ok(None);
        }

        let maximum_start =
            num_qubits
                .checked_sub(remaining)
                .ok_or(
                    SurfaceCodeDecoderError::
                        ArithmeticOverflow,
                )?;

        for qubit in
            start..=maximum_start
        {
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
                    qubit
                        .checked_add(1)
                        .ok_or(
                            SurfaceCodeDecoderError::
                                ArithmeticOverflow,
                        )?,
                    support,
                    paulis,
                    stabilizers,
                    cancellation,
                    usage,
                    num_qubits,
                )?
            {
                return Ok(Some(result));
            }

            support.pop();
        }

        Ok(None)
    }

    /// Enumerates X/Y/Z assignments for a fixed support.
    fn search_paulis(
        &self,
        syndrome: &Syndrome,
        support: &[usize],
        position: usize,
        paulis: &mut Vec<Pauli>,
        stabilizers: &StabilizerGroup,
        cancellation: &CancellationToken,
        usage: &mut SurfaceDecodeUsage,
        num_qubits: usize,
    ) -> Result<
        Option<PauliString>,
        SurfaceCodeDecoderError,
    > {
        cancellation
            .check()
            .map_err(
                SurfaceCodeDecoderError::Cancellation,
            )?;

        if position == support.len() {
            /*
             * Count only complete Pauli candidates.
             */
            usage.iterations =
                usage
                    .iterations
                    .checked_add(1)
                    .ok_or(
                        SurfaceCodeDecoderError::
                            ArithmeticOverflow,
                    )?;

            if usage.iterations
                > self.limits.max_decoder_iterations
            {
                return Err(
                    SurfaceCodeDecoderError::
                        ResourceLimitExceeded {
                            resource:
                                "decoder iterations",
                            requested:
                                usage.iterations,
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
                in support
                    .iter()
                    .zip(
                        paulis.iter(),
                    )
            {
                operator
                    .set_pauli(
                        QubitIndex::new(
                            qubit,
                        ),
                        pauli,
                    )
                    .map_err(
                        |error| {
                            SurfaceCodeDecoderError::
                                Stabilizer(
                                    error,
                                )
                        },
                    )?;
            }

            let produced =
                stabilizers
                    .syndrome(
                        operator.operator(),
                    )
                    .map_err(
                        |error| {
                            SurfaceCodeDecoderError::
                                Stabilizer(
                                    error,
                                )
                        },
                    )?;

            if produced == *syndrome {
                return Ok(Some(operator));
            }

            return Ok(None);
        }

        /*
         * Deterministic Pauli ordering:
         *
         * X → Y → Z
         */
        const PAULIS: [Pauli; 3] = [
            Pauli::X,
            Pauli::Y,
            Pauli::Z,
        ];

        for pauli in PAULIS {
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
                    usage,
                    num_qubits,
                )?
            {
                return Ok(Some(result));
            }

            paulis.pop();
        }

        Ok(None)
    }
}

/* ========================================================================== */
/* Generic Decoder integration                                                */
/* ========================================================================== */

impl Decoder for SurfaceCodeDecoder {
    fn id(&self) -> DecoderId {
        SURFACE_CODE_DECODER_ID
    }

    fn name(&self) -> &'static str {
        SURFACE_CODE_DECODER_NAME
    }

    fn decode(
        &self,
        syndrome: &Syndrome,
    ) -> Result<
        DecodeResult,
        DecoderError,
    > {
        self.decode_surface(syndrome)
            .map_err(
                SurfaceCodeDecoderError::into_decoder_error,
            )
    }
}

/* ========================================================================== */
/* Errors                                                                     */
/* ========================================================================== */

/// Errors produced by the reference surface-code decoder.
///
/// These errors retain the distinction between:
///
/// - invalid input;
/// - invalid topology;
/// - resource rejection;
/// - cancellation;
/// - mathematical failure;
/// - search exhaustion.
///
/// Higher-level execution layers may convert these into `QecError`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum SurfaceCodeDecoderError {
    /// Surface-code topology is invalid.
    SurfaceCode(
        SurfaceCodeError,
    ),

    /// Generic decoder-contract failure.
    Decoder(
        DecoderError,
    ),

    /// QEC resource policy is invalid.
    InvalidLimits(
        LimitError,
    ),

    /// Search weight was zero.
    InvalidSearchWeight {
        weight: usize,
    },

    /// Search weight exceeds the configured global policy.
    SearchWeightExceedsPolicy {
        requested: usize,
        maximum: usize,
    },

    /// Search weight cannot exceed the number of physical qubits.
    SearchWeightExceedsQubitCount {
        requested: usize,
        num_qubits: usize,
    },

    /// Syndrome refers to an unknown stabilizer.
    UnknownStabilizer {
        stabilizer: usize,
    },

    /// The decoder was cancelled.
    Cancellation(
        super::cancellation::CancellationError,
    ),

    /// Arithmetic overflow occurred while constructing the search.
    ArithmeticOverflow,

    /// Decoder iteration budget was exhausted.
    ResourceLimitExceeded {
        resource: &'static str,
        requested: usize,
        maximum: usize,
    },

    /// No correction was found inside the configured search region.
    SearchExhausted {
        syndrome_weight: usize,
        max_weight: usize,
        operations: usize,
    },

    /// The candidate correction did not reproduce the syndrome.
    CorrectionVerificationFailed,

    /// Stabilizer mathematics failed.
    Stabilizer(
        super::stabilizer::StabilizerError,
    ),
}

impl SurfaceCodeDecoderError {
    /// Converts the local error into the generic decoder error boundary.
    #[must_use]
    pub fn into_decoder_error(
        self,
    ) -> DecoderError {
        match self {
            Self::SurfaceCode(error) => {
                DecoderError::Stabilizer(
                    super::stabilizer::
                        StabilizerError::InvalidGenerator {
                            message:
                                error.to_string(),
                        },
                )
            }

            Self::Decoder(error) => {
                error
            }

            Self::InvalidLimits(error) => {
                DecoderError::InvalidResult {
                    reason:
                        "invalid QEC resource limits",
                }
            }

            Self::InvalidSearchWeight {
                ..
            } => {
                DecoderError::InvalidResult {
                    reason:
                        "invalid reference-decoder search weight",
                }
            }

            Self::SearchWeightExceedsPolicy {
                ..
            } => {
                DecoderError::InvalidResult {
                    reason:
                        "reference search weight exceeds QEC policy",
                }
            }

            Self::SearchWeightExceedsQubitCount {
                ..
            } => {
                DecoderError::InvalidResult {
                    reason:
                        "reference search weight exceeds qubit count",
                }
            }

            Self::UnknownStabilizer {
                ..
            } => {
                DecoderError::InvalidResult {
                    reason:
                        "syndrome references an unknown stabilizer",
                }
            }

            Self::Cancellation(
                _,
            ) => {
                DecoderError::Cancelled
            }

            Self::ArithmeticOverflow => {
                DecoderError::InvalidResult {
                    reason:
                        "reference decoder arithmetic overflow",
                }
            }

            Self::ResourceLimitExceeded {
                ..
            } => {
                DecoderError::InvalidResult {
                    reason:
                        "reference decoder resource limit exceeded",
                }
            }

            Self::SearchExhausted {
                ..
            } => {
                DecoderError::InvalidResult {
                    reason:
                        "reference decoder search exhausted",
                }
            }

            Self::CorrectionVerificationFailed => {
                DecoderError::InvalidResult {
                    reason:
                        "decoder correction failed syndrome verification",
                }
            }

            Self::Stabilizer(error) => {
                DecoderError::Stabilizer(
                    error,
                )
            }
        }
    }
}

impl fmt::Display
    for SurfaceCodeDecoderError
{
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::SurfaceCode(error) => {
                write!(
                    formatter,
                    "surface-code error: {error}"
                )
            }

            Self::Decoder(error) => {
                write!(
                    formatter,
                    "decoder error: {error}"
                )
            }

            Self::InvalidLimits(error) => {
                write!(
                    formatter,
                    "invalid QEC limits: {error}"
                )
            }

            Self::InvalidSearchWeight {
                weight,
            } => {
                write!(
                    formatter,
                    "reference search weight must be greater than zero; got {weight}"
                )
            }

            Self::SearchWeightExceedsPolicy {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "search weight {requested} exceeds configured maximum {maximum}"
                )
            }

            Self::SearchWeightExceedsQubitCount {
                requested,
                num_qubits,
            } => {
                write!(
                    formatter,
                    "search weight {requested} exceeds {num_qubits} physical qubits"
                )
            }

            Self::UnknownStabilizer {
                stabilizer,
            } => {
                write!(
                    formatter,
                    "unknown stabilizer {stabilizer}"
                )
            }

            Self::Cancellation(error) => {
                write!(
                    formatter,
                    "decoder cancelled: {error}"
                )
            }

            Self::ArithmeticOverflow => {
                write!(
                    formatter,
                    "reference decoder arithmetic overflow"
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "{resource} limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::SearchExhausted {
                syndrome_weight,
                max_weight,
                operations,
            } => {
                write!(
                    formatter,
                    "reference search exhausted for syndrome weight {syndrome_weight}; maximum weight {max_weight}; operations {operations}"
                )
            }

            Self::CorrectionVerificationFailed => {
                write!(
                    formatter,
                    "candidate correction failed syndrome verification"
                )
            }

            Self::Stabilizer(error) => {
                write!(
                    formatter,
                    "stabilizer error: {error}"
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_is_valid() {
        let config =
            SurfaceDecoderConfig::default();

        assert!(
            config.validate().is_ok()
        );

        assert_eq!(
            config.max_search_weight,
            DEFAULT_MAX_SEARCH_WEIGHT
        );

        assert!(
            config.verify_correction
        );
    }

    #[test]
    fn zero_search_weight_is_rejected() {
        let config =
            SurfaceDecoderConfig {
                max_search_weight: 0,
                verify_correction: true,
            };

        assert!(
            matches!(
                config.validate(),
                Err(
                    SurfaceCodeDecoderError::
                        InvalidSearchWeight {
                            weight: 0
                        }
                )
            )
        );
    }

    #[test]
    fn decoder_identity_is_stable() {
        assert_eq!(
            SurfaceCodeDecoder::decoder_id(),
            SURFACE_CODE_DECODER_ID
        );

        assert_eq!(
            SurfaceCodeDecoder::decoder_name(),
            SURFACE_CODE_DECODER_NAME
        );
    }
}